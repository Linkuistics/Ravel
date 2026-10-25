//! The worker side: Ravel's own binary in the internal role.
//!
//! `main` enters here when `argv[0]` is [`ARG0`](super::ARG0). The role accepts
//! no command-line input and talks only over the socket at descriptor 3. Input
//! beyond `arg0`, a descriptor 3 that is no socket, or a handshake that fails
//! all exit with status 2. After it sends its one result, it exits.

use std::io::Write as _;
use std::mem::MaybeUninit;
use std::os::fd::FromRawFd as _;
use std::os::unix::net::UnixStream;
use std::process::ExitCode;
use std::thread;

use serde_json::{Value, json};

use super::engine::{self, Entry};
use super::{BUILD_ID, Bounds, EXIT_ROLE_REFUSED, PROTOCOL, RHAI_VERSION, Reply, ScriptFailure};
use crate::frame::{FRONT_FRAME_BYTES, read_frame, write_frame};
use crate::worker::CHANNEL_FD;

/// The stack of the thread that evaluates the script.
const STACK_BYTES: usize = 64 * 1024 * 1024;

/// Why the role stopped, and with which status.
struct Stop {
    status: u8,
    reason: String,
}

impl Stop {
    fn refused(reason: impl Into<String>) -> Self {
        Stop {
            status: EXIT_ROLE_REFUSED,
            reason: reason.into(),
        }
    }
}

/// Run the role, with the process's own command line and descriptors.
pub fn run() -> ExitCode {
    let outcome = if std::env::args_os().count() == 1 {
        serve()
    } else {
        Err(Stop::refused("it accepts no command-line input"))
    };
    match outcome {
        Ok(()) => ExitCode::SUCCESS,
        Err(Stop { status, reason }) => {
            let _ = writeln!(std::io::stderr(), "ravel: internal role stopped: {reason}");
            ExitCode::from(status)
        }
    }
}

/// The role proper, once its command line is known to be empty.
fn serve() -> Result<(), Stop> {
    let channel =
        take_channel().ok_or_else(|| Stop::refused("descriptor 3 is not the protocol socket"))?;
    let hello = json!({
        "type": "hello",
        "protocol": PROTOCOL,
        "packageVersion": env!("CARGO_PKG_VERSION"),
        "buildId": BUILD_ID,
        "rhaiVersion": RHAI_VERSION,
    });
    write_frame(&channel, &hello)
        .map_err(|error| Stop::refused(format!("the hello could not be sent: {error}")))?;
    let evaluate = read_frame(&channel, FRONT_FRAME_BYTES)
        .map_err(|error| Stop::refused(format!("no evaluate frame arrived: {error}")))?;
    let evaluate = Evaluate::from_frame(&evaluate)
        .ok_or_else(|| Stop::refused("the first frame is not a well-formed evaluate frame"))?;
    let message_bytes = evaluate.bounds.message_bytes;

    let evaluated = thread::Builder::new()
        .name("rhai-policy".into())
        .stack_size(STACK_BYTES)
        .spawn(move || {
            let entry = Entry {
                name: &evaluate.entry,
                source: &evaluate.source,
            };
            engine::evaluate(
                &entry,
                &evaluate.request,
                evaluate.select_context.as_ref(),
                &evaluate.bounds,
            )
        })
        .map_err(|error| Stop::refused(format!("the evaluation thread did not start: {error}")))
        .and_then(|thread| {
            thread
                .join()
                .map_err(|_| Stop::refused("the evaluation thread panicked"))
        });
    let reply = evaluated?.map_err(Stop::refused)?;
    write_frame(&channel, &bounded(reply, message_bytes)).map_err(|error| Stop {
        status: 1,
        reason: format!("the result could not be sent: {error}"),
    })
}

/// The reply as a frame, or the failure that its size makes of it: the front
/// reads no frame past the message bound.
fn bounded(reply: Reply, message_bytes: u64) -> Value {
    let frame = reply.to_frame();
    let size = serde_json::to_vec(&frame).map_or(u64::MAX, |bytes| bytes.len() as u64);
    if size <= message_bytes {
        return frame;
    }
    let mut failure = ScriptFailure::new(
        "message_too_large",
        "selection",
        format!(
            "the result of select encodes to {size} bytes, over the message bound of \
             {message_bytes}"
        ),
    );
    failure.actual = Some(size);
    Reply::Failure(failure).to_frame()
}

/// Descriptor 3 as the protocol channel, if it is a socket.
fn take_channel() -> Option<UnixStream> {
    let mut stat = MaybeUninit::<libc::stat>::zeroed();
    // SAFETY: `fstat` writes a `stat` through the pointer when it returns 0,
    // and does nothing to the descriptor.
    let known = unsafe { libc::fstat(CHANNEL_FD, stat.as_mut_ptr()) } == 0;
    // SAFETY: the structure is initialised exactly when `fstat` returned 0.
    if !known || unsafe { stat.assume_init() }.st_mode & libc::S_IFMT != libc::S_IFSOCK {
        return None;
    }
    // SAFETY: the front passes its end of the socket pair as descriptor 3 and
    // nothing in this process owns it, so this stream becomes its only owner.
    Some(unsafe { UnixStream::from_raw_fd(CHANNEL_FD) })
}

/// The decoded evaluate frame.
struct Evaluate {
    entry: String,
    source: String,
    request: Value,
    /// What `select` gets as its second argument, when it is not `request.context`.
    select_context: Option<Value>,
    bounds: Bounds,
}

impl Evaluate {
    fn from_frame(frame: &Value) -> Option<Self> {
        let text = |field: &str| frame.get(field).and_then(Value::as_str).map(str::to_owned);
        if text("type")? != "evaluate" || frame.get("protocol")?.as_u64()? != PROTOCOL {
            return None;
        }
        Some(Evaluate {
            entry: text("entry")?,
            source: text("source")?,
            request: frame.get("request")?.clone(),
            select_context: frame.get("selectContext").filter(|v| !v.is_null()).cloned(),
            bounds: Bounds::from_json(frame.get("bounds")?).ok()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// What a unit test supervises: the test binary runs this as the role,
    /// behind a script that execs it, because a test binary cannot take `arg0`
    /// for its role. The role code is the same as the real binary's.
    #[test]
    #[ignore = "runs as a child of a supervising test"]
    fn role_fixture() {
        if let Err(Stop { status, reason }) = serve() {
            eprintln!("role fixture stopped: {reason}");
            std::process::exit(i32::from(status));
        }
    }

    #[test]
    fn a_result_over_the_message_bound_becomes_a_failure() {
        let big = Reply::Selection {
            version: "1".to_owned(),
            result: json!({ "text": "x".repeat(200) }),
        };
        let frame = bounded(big, 100);
        assert_eq!(frame["type"], "failure");
        assert_eq!(frame["failure"]["code"], "message_too_large");
        assert!(frame["failure"]["actual"].as_u64().unwrap() > 100);
        let small = Reply::Selection {
            version: "1".to_owned(),
            result: json!(1),
        };
        assert_eq!(bounded(small.clone(), 100), small.to_frame());
    }
}
