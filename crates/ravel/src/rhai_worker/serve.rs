//! The Tokio current-thread runtime that serves the worker while it evaluates
//! (`docs/specs/policy-evaluation.md`, *The bridge between the script and the
//! CLI*).
//!
//! The runtime exists for the evaluation phase only. [`front`](super::front)
//! builds it after the handshake, runs [`serve`] to the worker's result and
//! drops it before the supervision stops the worker, so no task outlives the
//! evaluation and none exists when the managed process launches.
//!
//! The thread that runs the runtime makes no filesystem call. While a host call
//! runs on the reader thread, [`serve`] keeps watching four things: the
//! deadline, the cancellation latch, the bound on the policy's output and the
//! channel itself. A worker that sends a frame during a host call broke the
//! protocol, and one whose end closed has died.

use std::io::{self, ErrorKind};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};

use serde_json::{Value, json};
use tokio::io::{AsyncReadExt as _, AsyncWriteExt as _};
use tokio::net::UnixStream;
use tokio::time::{Instant, interval, sleep_until};

use super::harness::RunRecord;
use super::host::{Answer, Host};
use super::{Bounds, Module, Read, Reply, ScriptFailure};
use crate::cancellation;
use crate::context::{Delivered, Measured};
use crate::frame::{FRONT_FRAME_BYTES, FrameError};
use crate::refusal::{EXIT_WORKER, Refusal, Stage};
use crate::worker::{Conversation, POLL, broken_for};

/// What a frame carries beyond the value it names.
const ENVELOPE: usize = 1024;

/// What the front hands to judge a loader's context: the value that
/// `load_context` returned, and the sources it is measured from.
pub type Judge<'a> = dyn FnMut(Value, Vec<Measured>) -> Result<Delivered, Refusal> + 'a;

/// What a served evaluation leaves behind.
pub(super) struct Served {
    pub reply: Reply,
    pub delivered: Option<Delivered>,
    pub modules: Vec<Module>,
    pub reads: Vec<Read>,
    pub runs: Vec<RunRecord>,
}

/// What stops the evaluation, whichever comes first.
enum Stop {
    Deadline,
    Signal,
    Output,
}

impl Stop {
    fn conversation(self) -> Conversation {
        match self {
            Stop::Deadline => Conversation::Expired { handed_over: true },
            Stop::Signal => Conversation::Ended(FrameError::Io(io::Error::other(
                "a signal cancelled the selection",
            ))),
            Stop::Output => Conversation::Ended(FrameError::Io(io::Error::other(
                "the policy's output passed its bound",
            ))),
        }
    }
}

struct Watch<'a> {
    deadline: Instant,
    overflow: &'a AtomicBool,
}

impl Watch<'_> {
    /// Resolve when the evaluation must stop. The latch and the flag are atomics
    /// with no waker, so they are looked at every `POLL`.
    async fn stopped(&self) -> Stop {
        let mut tick = interval(POLL);
        loop {
            if self.overflow.load(Ordering::SeqCst) {
                return Stop::Output;
            }
            if cancellation::received().is_some() {
                return Stop::Signal;
            }
            tokio::select! {
                () = sleep_until(self.deadline) => return Stop::Deadline,
                _ = tick.tick() => {}
            }
        }
    }

    /// Await `work`, unless the evaluation stops first. A stop drops `work`,
    /// which abandons a read in progress.
    async fn until<T>(&self, work: impl Future<Output = T>) -> Result<T, Conversation> {
        tokio::select! {
            biased;
            stop = self.stopped() => Err(stop.conversation()),
            done = work => Ok(done),
        }
    }
}

/// What the evaluation is of, and how to describe the worker if it breaks.
pub(super) struct Setup<'a> {
    pub program: &'a Path,
    pub rebuild: &'a str,
    /// The canonical path of the entry, which the reader thread reads.
    pub entry: &'a str,
    /// The refusal for an entry that cannot be read, from what is wrong with it.
    pub unreadable: &'a dyn Fn(&str) -> Refusal,
    pub request: &'a Value,
    pub select_context: Option<&'a Value>,
    pub bounds: &'a Bounds,
}

/// Read the entry, hand it to the worker with the request, and serve the worker
/// until it sends its result.
pub(super) async fn serve(
    stream: std::os::unix::net::UnixStream,
    (deadline, overflow): (std::time::Instant, &AtomicBool),
    setup: Setup<'_>,
    mut host: Host,
    judge: &mut Judge<'_>,
) -> Result<Served, Conversation> {
    let Setup {
        program,
        rebuild,
        entry,
        unreadable,
        request,
        select_context,
        bounds,
    } = setup;
    let mut stream = UnixStream::from_std(stream).map_err(|error| {
        Conversation::Ended(FrameError::Io(io::Error::other(format!(
            "the channel could not join the runtime: {error}"
        ))))
    })?;
    let watch = Watch {
        deadline: Instant::from_std(deadline),
        overflow,
    };
    // The deadline began when the worker started, so it covers this read.
    let source = match watch.until(host.read_entry(entry)).await? {
        Ok(source) => source,
        Err(what) => return Err(Conversation::Refused(unreadable(&what))),
    };
    let mut evaluate = json!({
        "type": "evaluate",
        "protocol": super::PROTOCOL,
        // Exactly the admitted file.
        "entry": entry,
        "source": source,
        "request": request,
        "bounds": bounds.to_json(),
    });
    if let Some(context) = select_context {
        evaluate["selectContext"] = context.clone();
    }
    let body = serde_json::to_vec(&evaluate).expect("a JSON value always encodes");
    watch
        .until(write_body(&mut stream, &body))
        .await?
        .map_err(|error| Conversation::failed(error, false))?;

    let message = usize::try_from(bounds.message_bytes).unwrap_or(usize::MAX);
    let context = usize::try_from(bounds.context_bytes).unwrap_or(usize::MAX);
    let largest = message.max(context).saturating_add(ENVELOPE);
    let broken = |why: String| broken_for(program, rebuild, why);
    let mut delivered = None;
    loop {
        let (frame, length) = watch
            .until(read_frame(&mut stream, largest))
            .await?
            .map_err(|error| match error {
                error @ (FrameError::TooLarge(..) | FrameError::NotJson(_)) => {
                    broken(format!("the policy worker sent a malformed frame: {error}"))
                }
                other => Conversation::failed(other, true),
            })?;
        match frame.get("type").and_then(Value::as_str) {
            Some("host") => {
                if length > message.saturating_add(ENVELOPE) {
                    return Err(broken("a host frame passed the message bound".to_owned()));
                }
                let called = watch.until(during_call(&stream, host.call(&frame))).await;
                // A stop drops the call, and a harness run that the call was
                // waiting for goes on until it is asked to stop.
                let answer = match called {
                    Ok(Ok(answer)) => answer,
                    Ok(Err(stopped)) | Err(stopped) => return Err(host.settle(stopped).await),
                }
                .map_err(|why| broken(format!("a malformed host frame: {why}")))?;
                match answer {
                    Answer::Reply(body) => watch
                        .until(write_body(&mut stream, &body))
                        .await?
                        .map_err(|error| Conversation::failed(error, true))?,
                    Answer::End(failure) => return finish(Reply::Failure(*failure), None, host),
                }
            }
            Some("context") => {
                if delivered.is_some() {
                    return Err(broken("the worker sent a second context".to_owned()));
                }
                if let Some(size) = frame.get("oversize").and_then(Value::as_u64) {
                    let mut failure = ScriptFailure::new(
                        "context_too_large",
                        "context",
                        format!("load_context returned a context of {size} bytes"),
                    );
                    failure.actual = Some(size);
                    return finish(Reply::Failure(failure), None, host);
                }
                let value = frame
                    .get("value")
                    .cloned()
                    .ok_or_else(|| broken("a context frame has no value".to_owned()))?;
                let judged = judge(value, host.context_sources()).map_err(Conversation::Refused)?;
                let reply = json!({ "type": "delivered", "context": judged.value });
                let body = serde_json::to_vec(&reply).expect("a JSON value always encodes");
                watch
                    .until(write_body(&mut stream, &body))
                    .await?
                    .map_err(|error| Conversation::failed(error, true))?;
                delivered = Some(judged);
            }
            _ => {
                if length > message.saturating_add(ENVELOPE) {
                    return Err(broken("a result frame passed the message bound".to_owned()));
                }
                let reply = Reply::from_frame(&frame).ok_or_else(|| {
                    broken(format!(
                        "an unexpected frame: {}",
                        super::front::shown(&frame)
                    ))
                })?;
                return finish(reply, delivered, host);
            }
        }
    }
}

fn finish(reply: Reply, delivered: Option<Delivered>, host: Host) -> Result<Served, Conversation> {
    let (modules, reads, runs) = host.into_record();
    Ok(Served {
        reply,
        delivered,
        modules,
        reads,
        runs,
    })
}

/// Await a host call, and fail if the worker speaks or goes away meanwhile.
/// The script is synchronous, so a worker with a call in progress sends nothing.
async fn during_call<T>(
    stream: &UnixStream,
    call: impl Future<Output = T>,
) -> Result<T, Conversation> {
    tokio::select! {
        biased;
        interrupted = unexpected(stream) => Err(interrupted),
        done = call => Ok(done),
    }
}

/// Resolve when the channel has something to read or has closed, neither of
/// which a worker waiting for a reply does.
async fn unexpected(stream: &UnixStream) -> Conversation {
    loop {
        if let Err(error) = stream.readable().await {
            return Conversation::Ended(FrameError::Io(error));
        }
        match stream.try_read(&mut [0u8; 1]) {
            Ok(0) => return Conversation::Ended(FrameError::Closed),
            Ok(_) => {
                return Conversation::Broken(Refusal::new(
                    "worker_failed",
                    Stage::Worker,
                    EXIT_WORKER,
                    "the policy worker sent a message while a host call was in progress, \
                         which a synchronous script cannot do"
                        .to_owned(),
                    "reinstall ravel, and start the session again; the policy worker is the \
                         ravel binary itself, so it differs only when that file was replaced \
                         while ravel ran",
                ));
            }
            Err(error) if error.kind() == ErrorKind::WouldBlock => {}
            Err(error) => return Conversation::Ended(FrameError::Io(error)),
        }
    }
}

/// The next frame and its length in bytes, if it is at most `max`.
async fn read_frame(stream: &mut UnixStream, max: usize) -> Result<(Value, usize), FrameError> {
    let mut header = [0u8; 4];
    read_exact(stream, &mut header).await?;
    let length = u32::from_be_bytes(header) as usize;
    if length > max {
        return Err(FrameError::TooLarge(length, max));
    }
    let mut body = vec![0u8; length];
    read_exact(stream, &mut body).await?;
    serde_json::from_slice(&body)
        .map(|frame| (frame, length))
        .map_err(FrameError::NotJson)
}

async fn read_exact(stream: &mut UnixStream, buffer: &mut [u8]) -> Result<(), FrameError> {
    stream.read_exact(buffer).await.map(drop).map_err(|error| {
        if error.kind() == ErrorKind::UnexpectedEof {
            FrameError::Closed
        } else {
            FrameError::Io(error)
        }
    })
}

/// Send a frame whose JSON body is already encoded.
async fn write_body(stream: &mut UnixStream, body: &[u8]) -> Result<(), FrameError> {
    let too_large = || FrameError::TooLarge(body.len(), FRONT_FRAME_BYTES);
    if body.len() > FRONT_FRAME_BYTES {
        return Err(too_large());
    }
    let length = u32::try_from(body.len()).map_err(|_| too_large())?;
    stream
        .write_all(&length.to_be_bytes())
        .await
        .map_err(FrameError::Io)?;
    stream.write_all(body).await.map_err(FrameError::Io)
}

#[cfg(test)]
mod tests {
    use std::io::Write as _;
    use std::os::unix::net::UnixStream as StdStream;
    use std::time::{Duration, Instant as StdInstant};

    use super::*;
    use crate::rhai_worker::reader::Reader;

    fn runtime() -> tokio::runtime::Runtime {
        tokio::runtime::Builder::new_current_thread()
            .enable_io()
            .enable_time()
            .build()
            .unwrap()
    }

    fn pair() -> (StdStream, StdStream) {
        let (ours, theirs) = StdStream::pair().unwrap();
        ours.set_nonblocking(true).unwrap();
        (ours, theirs)
    }

    #[test]
    fn a_byte_from_the_worker_during_a_host_call_breaks_the_protocol() {
        let (ours, mut theirs) = pair();
        let outcome = runtime().block_on(async {
            let stream = UnixStream::from_std(ours).unwrap();
            theirs.write_all(&[0]).unwrap();
            during_call(&stream, std::future::pending::<()>()).await
        });
        match outcome {
            Err(Conversation::Broken(refusal)) => assert_eq!(refusal.code, "worker_failed"),
            _ => panic!("a message during a host call must break the protocol"),
        }
    }

    #[test]
    fn a_worker_that_closes_during_a_host_call_has_ended() {
        let (ours, theirs) = pair();
        let outcome = runtime().block_on(async {
            let stream = UnixStream::from_std(ours).unwrap();
            drop(theirs);
            during_call(&stream, std::future::pending::<()>()).await
        });
        assert!(matches!(
            outcome,
            Err(Conversation::Ended(FrameError::Closed))
        ));
    }

    #[test]
    fn a_silent_worker_lets_the_call_finish() {
        let (ours, _theirs) = pair();
        let outcome = runtime().block_on(async {
            let stream = UnixStream::from_std(ours).unwrap();
            during_call(&stream, async { 7 }).await
        });
        assert!(matches!(outcome, Ok(7)));
    }

    /// The mechanism, not the seam: the spec says that no test at the seam
    /// proves that a stop never waits for a read in progress. Here the read is a
    /// closure on the reader thread, and the deadline passes while it runs.
    #[test]
    fn a_deadline_does_not_wait_for_the_call_the_reader_is_in() {
        let reader = Reader::start().unwrap();
        let overflow = AtomicBool::new(false);
        let began = StdInstant::now();
        let outcome = runtime().block_on(async {
            let watch = Watch {
                deadline: Instant::now() + Duration::from_millis(100),
                overflow: &overflow,
            };
            let slow = reader.run(|| std::thread::sleep(Duration::from_secs(3)));
            watch.until(slow).await
        });
        assert!(matches!(
            outcome,
            Err(Conversation::Expired { handed_over: true })
        ));
        assert!(
            began.elapsed() < Duration::from_secs(2),
            "the stop waited {:?}",
            began.elapsed()
        );
    }

    #[test]
    fn the_output_bound_stops_the_wait() {
        let overflow = AtomicBool::new(true);
        let outcome = runtime().block_on(async {
            let watch = Watch {
                deadline: Instant::now() + Duration::from_secs(30),
                overflow: &overflow,
            };
            watch.until(std::future::pending::<()>()).await
        });
        assert!(matches!(outcome, Err(Conversation::Ended(_))));
    }
}
