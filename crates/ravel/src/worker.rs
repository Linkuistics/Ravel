//! Evaluate the compiled policy worker through a bounded private socket.

use std::ffi::OsString;
use std::fs;
use std::io::{self, ErrorKind, Read, Write};
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd, RawFd};
use std::os::unix::fs::PermissionsExt as _;
use std::os::unix::net::UnixStream;
use std::os::unix::process::CommandExt as _;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use serde_json::{json, Value};

use crate::cancellation;
use crate::context::Measured;
use crate::frame::{read_frame, write_frame, FrameError};
use crate::limits::{Bound, Limits, SELECTION_MAX_MS};
use crate::refusal::{
    Diagnostics, Failure, Refusal, Stage, EXIT_REFUSED, EXIT_TIMEOUT, EXIT_WORKER,
};

pub const PROTOCOL: u64 = 1;
pub const PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");
/// The digest of the worker source this front was built beside (see build.rs).
pub const BUILD_ID: &str = env!("RAVEL_WORKER_BUILD_ID");

/// The installed layout, relative to the directory holding the real front
/// executable. Installation and archives must keep the same relative layout.
const WORKER_FROM_BIN: &str = "../libexec/ravel/ravel-policy";

/// The descriptor the worker's channel occupies, fixed on both sides.
const CHANNEL_FD: RawFd = 3;

/// The directory listing this process's open descriptors.
#[cfg(target_os = "linux")]
const DESCRIPTORS: &str = "/proc/self/fd";
#[cfg(not(target_os = "linux"))]
const DESCRIPTORS: &str = "/dev/fd";

/// How long to keep collecting diagnostics after the worker is reaped. A
/// descendant the policy left holding its stdout could otherwise keep a drain
/// open forever; detached policy children are outside the contract, so their
/// late output is not waited for. Both streams share the one grace.
const DRAIN_GRACE: Duration = Duration::from_secs(1);

/// How long a worker has to exit, after TERM at the deadline or by itself
/// after its result, before it is killed.
const CLEANUP_GRACE: Duration = Duration::from_secs(1);

/// The longest a channel read or write waits before it looks again at the
/// diagnostics bound, so that a policy printing without end is stopped for
/// that, promptly, rather than at the deadline.
const POLL: Duration = Duration::from_millis(50);

/// What a context frame carries beyond the delivered context the worker
/// measured: its type, and the `measured` key it moves out of the context.
const CONTEXT_ENVELOPE: usize = 1024;

const REBUILD: &str = "reinstall ravel so that its front and worker come from one build; \
     in a source checkout, run `task worker`";

#[derive(Debug)]
pub struct WorkerIdentity {
    pub path: PathBuf,
    pub package_version: String,
    pub build_id: String,
    pub bun_version: String,
}

#[derive(Debug)]
pub enum Outcome {
    /// The serializable snapshot of the entry's `policy` export.
    Policy(Value),
    /// The entry, or something it imports, failed to load.
    LoadFailed { name: String, message: String },
    /// Loading awaited a promise that nothing was left to settle.
    LoadUnsettled,
    /// The export is not a policy-shaped object at all.
    Invalid { location: String, message: String },
    /// The snapshot would have exceeded the protocol message bound.
    Breach(Breach),
}

/// What the policy's `select` produced, as the worker reported it. Every value
/// is data for the judge, abstention included.
#[derive(Debug)]
pub enum Produced {
    /// The value it returned or resolved to, with `undefined` as `null`.
    Result(Value),
    /// What it threw, or the reason its promise rejected with.
    Threw { name: String, message: String },
    /// The value it produced cannot be serialized.
    Unserializable { name: String, message: String },
    /// Its promise was still pending when nothing was left to settle it.
    Unsettled,
    /// Its result would have exceeded the protocol message bound.
    Breach(Breach),
}

/// What the worker assembled as the context, or why it could not.
#[derive(Debug)]
pub enum Assembled {
    Context {
        context: Value,
        measured: Vec<Measured>,
    },
    /// `loadContext` threw, or its promise rejected.
    Threw { name: String, message: String },
    /// `loadContext` failed because an SDK read of `source` did.
    SourceUnreadable { source: String, message: String },
    /// Its promise was still pending when nothing was left to settle it.
    Unsettled,
    /// Its result cannot be serialized.
    Unserializable { name: String, message: String },
    /// A bound was exceeded, whether or not the policy caught the error.
    Breach(Breach),
}

/// A bound the worker saw exceeded, as it reported it. Only the facts the
/// front cannot know come from the worker: which bound, how much, and for a
/// read its source and any `maxBytes` the policy passed. The limits
/// themselves are the front's own.
#[derive(Debug)]
pub struct Breach {
    /// `context`, `source`, `sources` or `message`.
    pub bound: String,
    /// The size or count reached, or at least reached.
    pub actual: Option<u64>,
    /// The source a read or count bound was exceeded at.
    pub source: Option<String>,
    /// The `maxBytes` a read passed, if it passed one.
    pub max_bytes: Option<u64>,
}

#[derive(Debug)]
pub struct Evaluation<T> {
    pub worker: WorkerIdentity,
    /// What the judge made of the worker's reports.
    pub decided: T,
    pub diagnostics: Diagnostics,
    pub operational: Vec<String>,
    /// From starting the worker to the judge's decision.
    pub elapsed: Duration,
}

/// The worker after it has reported on the entry, waiting to learn whether the
/// judge needs a context or a selection from it.
pub struct Loaded<'a> {
    channel: Bounded<'a>,
    worker: &'a Path,
    limits: &'a Limits,
    /// The caller's `--context` document, as the front measured it.
    caller: Option<&'a Measured>,
    assembled: bool,
    selected: bool,
}

impl Loaded<'_> {
    pub fn context(&mut self) -> Result<Assembled, Halt> {
        assert!(!self.assembled, "the context is assembled at most once");
        self.assembled = true;
        write_frame(
            &mut self.channel,
            &json!({ "type": "context", "protocol": PROTOCOL }),
        )
        .map_err(|error| Halt(Conversation::failed(error, true)))?;
        let max =
            usize::try_from(self.limits.context.value).unwrap_or(usize::MAX) + CONTEXT_ENVELOPE;
        let frame = receive(&mut self.channel, self.worker, true, max).map_err(Halt)?;
        let assembled = assembled(&frame).ok_or_else(|| {
            Halt(broken(
                self.worker,
                format!("unexpected context frame: {}", shown(&frame)),
            ))
        })?;
        if let Assembled::Context { measured, .. } = &assembled {
            let from_caller = measured
                .iter()
                .filter(|source| source.via == crate::context::VIA_CALLER)
                .count();
            let consistent = match self.caller {
                Some(caller) => measured.first() == Some(caller) && from_caller == 1,
                None => from_caller == 0,
            };
            if !consistent {
                return Err(Halt(broken(
                    self.worker,
                    "the worker's measured sources do not begin with the --context document \
                     exactly as the front measured it"
                        .to_owned(),
                )));
            }
        }
        Ok(assembled)
    }

    /// Ask the worker, once, to call the policy's `select`, and wait, within
    /// what is left of the deadline, for what it produced.
    pub fn select(&mut self) -> Result<Produced, Halt> {
        assert!(!self.selected, "select is called at most once");
        self.selected = true;
        write_frame(
            &mut self.channel,
            &json!({ "type": "select", "protocol": PROTOCOL }),
        )
        .map_err(|error| Halt(Conversation::failed(error, true)))?;
        let max = message_max(self.limits);
        let frame = receive(&mut self.channel, self.worker, true, max).map_err(Halt)?;
        let produced = produced(&frame).ok_or_else(|| {
            Halt(broken(
                self.worker,
                format!("unexpected selection frame: {}", shown(&frame)),
            ))
        })?;
        Ok(produced)
    }
}

/// The bound on a snapshot or result frame, the fixed protocol message bound.
fn message_max(limits: &Limits) -> usize {
    usize::try_from(limits.message.value).unwrap_or(usize::MAX)
}

/// Why a conversation stopped before the judge decided: a refusal of the
/// judge's own, or the worker's failure to answer.
pub struct Halt(Conversation);

impl From<Refusal> for Halt {
    fn from(refusal: Refusal) -> Self {
        Halt(Conversation::Refused(refusal))
    }
}

/// The worker beside the real front executable, following any symlinks to it.
/// No PATH, cwd or environment input takes part.
pub fn locate() -> Result<PathBuf, Refusal> {
    // Without the front's own path the worker's has no directory either, so
    // the source is the layout the worker would be found at.
    let unlocatable = |detail: String, source: String| {
        Refusal::new(
            "worker_missing",
            Stage::Worker,
            EXIT_WORKER,
            format!("cannot locate the policy worker: {detail}"),
            REBUILD,
        )
        .source(source)
    };
    let exe = std::env::current_exe()
        .and_then(fs::canonicalize)
        .map_err(|error| {
            unlocatable(
                format!("the front executable's path is unknown ({error})"),
                format!("<the front executable's directory>/{WORKER_FROM_BIN}"),
            )
        })?;
    let bin = exe.parent().ok_or_else(|| {
        unlocatable(
            format!("{} has no parent directory", exe.display()),
            exe.to_string_lossy().into_owned(),
        )
    })?;
    let worker = normalize(&bin.join(WORKER_FROM_BIN));
    match fs::metadata(&worker) {
        Ok(metadata) if metadata.is_file() => Ok(worker),
        Ok(_) => Err(Refusal::new(
            "worker_missing",
            Stage::Worker,
            EXIT_WORKER,
            format!(
                "the policy worker {} is not a regular file",
                worker.display()
            ),
            REBUILD,
        )
        .source(worker.to_string_lossy())),
        Err(error) => Err(Refusal::new(
            "worker_missing",
            Stage::Worker,
            EXIT_WORKER,
            format!(
                "the policy worker is not installed at {} ({error})",
                worker.display()
            ),
            REBUILD,
        )
        .source(worker.to_string_lossy())),
    }
}

/// Drop `..` lexically. The input is already canonical up to the layout suffix,
/// so this only makes the reported path readable.
fn normalize(path: &Path) -> PathBuf {
    let mut normal = PathBuf::new();
    for component in path.components() {
        match component {
            std::path::Component::ParentDir => {
                normal.pop();
            }
            other => normal.push(other),
        }
    }
    normal
}

/// Start the worker, verify it, have it evaluate `entry`, and let `judge`
/// decide on what it reports, asking it for a context or a selection if the
/// judge needs one, all within the selection bound, which counts from the
/// worker's start, and every other bound in `limits`. `caller` is the
/// `--context` document as the front measured it, the first measured source.
/// `environment` is the worker's whole environment (`environment::Grants`).
pub fn evaluate<T>(
    worker: &Path,
    entry: &str,
    request: Value,
    limits: &Limits,
    caller: Option<&Measured>,
    environment: Vec<(OsString, OsString)>,
    judge: impl FnOnce(Outcome, Loaded<'_>) -> Result<T, Halt>,
) -> Result<Evaluation<T>, Failure> {
    let bound = limits.selection;
    let failed = |message: String| {
        Refusal::new(
            "worker_failed",
            Stage::Worker,
            EXIT_WORKER,
            message,
            REBUILD,
        )
        .source(worker.to_string_lossy())
    };

    child_watching()
        .map_err(|error| failed(format!("cannot establish worker watching: {error}")))?;

    // bunfig autoloading, and dotenv autoloading for the worker's first VM,
    // read the directory a process starts in, so the worker starts in one
    // that is the front's own and empty, though it moves to `/` before it
    // loads any policy code. That directory is no control for a VM started
    // later, such as a native `Worker` a policy starts, whose dotenv files
    // are read from wherever the process then is (`worker/src/main.ts`). The
    // mode is stated because `tempfile` otherwise makes a directory
    // `0o777 & !umask`, which a permissive umask leaves open to other users:
    // https://docs.rs/tempfile/3.27.0/tempfile/struct.Builder.html#method.permissions
    #[cfg(test)]
    let parent = crate::directory_control::parent("worker", std::env::temp_dir());
    let private_dir = tempfile::Builder::new()
        .prefix("ravel-worker.")
        .permissions(fs::Permissions::from_mode(0o700))
        .tempdir_in({
            #[cfg(test)]
            {
                parent
            }
            #[cfg(not(test))]
            {
                std::env::temp_dir()
            }
        })
        .map_err(|error| {
            failed(format!(
                "cannot create the worker's private directory: {error}"
            ))
        })?;
    let mut private_dir = crate::private_directory::PrivateDirectory::new(private_dir, "worker");
    let prepared = (|| -> Result<_, Refusal> {
        private_dir
            .record_identity()
            .map_err(|error| failed(format!("cannot record worker directory identity: {error}")))?;
        let (front_end, pair_end) = UnixStream::pair()
            .map_err(|error| failed(format!("cannot create the protocol channel: {error}")))?;
        #[cfg(test)]
        let (front_end, pair_end) = crate::directory_control::operation(
            "worker",
            "pair",
            private_dir.path(),
            Ok((front_end, pair_end)),
        )
        .map_err(|e| failed(format!("cannot create the protocol channel: {e}")))?;
        let worker_end = above_stdio(&pair_end)
            .map_err(|error| failed(format!("cannot prepare the protocol channel: {error}")))?;
        #[cfg(test)]
        let worker_end = crate::directory_control::operation(
            "worker",
            "duplicate",
            private_dir.path(),
            Ok(worker_end),
        )
        .map_err(|e| failed(format!("cannot prepare the protocol channel: {e}")))?;
        // Only the duplicate crosses into the worker. Any other copy of its end
        // held here would keep the channel open after the worker dies, and the
        // read below would wait for an EOF that never comes.
        drop(pair_end);

        let mut command = Command::new(worker);
        command
            .current_dir(private_dir.path())
            .env_clear()
            .envs(environment)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let channel = worker_end.as_raw_fd();
        let inherited = held_descriptors().map_err(|error| {
            Refusal::new(
                "worker_failed",
                Stage::Worker,
                EXIT_WORKER,
                format!(
                    "cannot list this process's open descriptors in {DESCRIPTORS}, so the policy \
                 worker could inherit one it must not: {error}"
                ),
                format!("run ravel where {DESCRIPTORS} lists the process's descriptors"),
            )
            .source(DESCRIPTORS)
        })?;
        #[cfg(test)]
        let inherited = crate::directory_control::operation(
            "worker",
            "descriptors",
            private_dir.path(),
            Ok(inherited),
        )
        .map_err(|e| {
            Refusal::new(
                "worker_failed",
                Stage::Worker,
                EXIT_WORKER,
                format!("cannot list descriptors: {e}"),
                format!("run ravel where {DESCRIPTORS} lists the process descriptors"),
            )
            .source(DESCRIPTORS)
        })?;
        #[cfg(test)]
        crate::directory_control::before_spawn(worker);
        #[cfg(test)]
        let native_ignore_term = native_tests::active();
        // SAFETY: the closure runs between fork and exec, so it calls only
        // async-signal-safe functions (`dup2`, `fcntl`) on descriptors computed
        // before the fork, and touches no Rust runtime state or allocator.
        unsafe {
            command.pre_exec(move || {
                #[cfg(test)]
                if native_ignore_term && libc::signal(libc::SIGTERM, libc::SIG_IGN) == libc::SIG_ERR
                {
                    return Err(io::Error::last_os_error());
                }
                // `dup2` onto itself would leave close-on-exec set, and
                // `above_stdio` guarantees the source is at least 3.
                if channel == CHANNEL_FD {
                    let flags = libc::fcntl(channel, libc::F_GETFD);
                    if flags == -1
                        || libc::fcntl(channel, libc::F_SETFD, flags & !libc::FD_CLOEXEC) == -1
                    {
                        return Err(std::io::Error::last_os_error());
                    }
                } else if libc::dup2(channel, CHANNEL_FD) == -1 {
                    return Err(std::io::Error::last_os_error());
                }
                // Everything above the channel closes at exec, so the worker holds
                // descriptors 0-3 and nothing a caller happened to leave open.
                for &fd in inherited.iter().filter(|&&fd| fd > CHANNEL_FD) {
                    if libc::fcntl(fd, libc::F_SETFD, libc::FD_CLOEXEC) == -1
                        && std::io::Error::last_os_error().raw_os_error() != Some(libc::EBADF)
                    {
                        return Err(std::io::Error::last_os_error());
                    }
                }
                Ok(())
            });
        }

        let started = Instant::now();
        let deadline = started + bound.duration();
        let child = command.spawn().map_err(|error| {
            if error.kind() == ErrorKind::NotFound {
                Refusal::new(
                    "worker_missing",
                    Stage::Worker,
                    EXIT_WORKER,
                    format!(
                        "the policy worker {} cannot be started: {error}",
                        worker.display()
                    ),
                    REBUILD,
                )
                .source(worker.to_string_lossy())
            } else {
                failed(format!(
                    "the policy worker {} cannot be started: {error}",
                    worker.display()
                ))
            }
        })?;
        Ok((front_end, worker_end, child, started, deadline))
    })();
    let (front_end, worker_end, mut child, started, deadline) = match prepared {
        Ok(prepared) => prepared,
        Err(original) => {
            let cleanup = private_dir.cleanup();
            return Err(match cleanup {
                Ok(()) => original,
                Err(error) => {
                    original.report(&error.to_string(), crate::private_directory::CLEANUP_ADVICE)
                }
            }
            .into());
        }
    };
    // The front keeps only its own end, so the worker's exit reads as EOF.
    drop(worker_end);
    #[cfg(test)]
    native_tests::child_started(&child, &front_end);
    let (capture, reader_reports) = Capture::start(
        child.stdout.take(),
        child.stderr.take(),
        limits.diagnostics.value,
    );
    // F03's private break: the reader failure is ignored, so admission goes on.
    #[cfg(test)]
    let reader_reports = native_tests::reader_reports(reader_reports);

    let channel = Bounded {
        channel: &front_end,
        deadline,
        overflow: &capture.overflow,
    };
    let asked = Asked {
        worker,
        entry,
        request,
        limits,
        caller,
    };
    // Retain the conversation even when cancellation supersedes it: its
    // original cause and metadata still belong in the failure report.
    let result = if reader_reports.is_empty() {
        converse(channel, asked, started, judge)
    } else {
        Err(Conversation::Broken(failed(format!(
            "cannot admit diagnostic readers for worker pid {}: {}",
            child.id(),
            reader_reports.join("; ")
        ))))
    };
    drop(front_end);
    let stopped = match &result {
        _ if !reader_reports.is_empty() => stop(&mut child, Some(libc::SIGTERM)),
        _ if cancellation::received().is_some() => stop(&mut child, Some(libc::SIGTERM)),
        _ if capture.overflowed() => kill_and_wait(&mut child),
        Ok(_) | Err(Conversation::Refused(_)) => stop(&mut child, None),
        Err(Conversation::Expired { .. }) => stop(&mut child, Some(libc::SIGTERM)),
        Err(Conversation::Broken(_) | Conversation::Ended(_)) => kill_and_wait(&mut child),
    };
    let (diagnostics, overflowed) =
        capture.collect((Instant::now() + DRAIN_GRACE).min(stopped.drain_by));
    let directory = private_dir.path().to_owned();
    let removed = private_dir.cleanup();
    #[allow(unused_mut)]
    let mut teardown = Teardown {
        pid: child.id(),
        status: stopped.status,
        reports: {
            #[cfg(test)]
            let dropped = native_tests::drop_wait_reports();
            #[cfg(not(test))]
            let dropped = false;
            if dropped {
                Vec::new()
            } else {
                stopped.reports
            }
        },
        stop_failed: stopped.stop_failed,
        directory,
        removed,
    };
    #[cfg(test)]
    crate::directory_control::receipt(format!(
        "actual worker wait pid={} result={:?} operations={:?}",
        teardown.pid, teardown.status, teardown.reports
    ));
    #[cfg(test)]
    crate::native_entry_tests::worker_wait(teardown.pid, &teardown.status);
    #[cfg(test)]
    native_tests::fixture_cleanup(&mut child, &teardown);
    #[cfg(test)]
    admission_tests::reported_teardown(&mut teardown, &mut child);
    #[cfg(test)]
    crate::directory_control::worker_reports(&mut teardown.status, &mut teardown.reports);
    let admitted = teardown.status.is_ok() && teardown.removed.is_ok() && !teardown.stop_failed;
    let operational = teardown.describe(worker);
    let primary = overflowed.then(|| output_limit(entry, limits));
    let (success, original) = match result {
        Ok(success) => (Some(success), None),
        Err(Conversation::Refused(refusal) | Conversation::Broken(refusal)) => {
            (None, Some(refusal))
        }
        Err(Conversation::Expired { handed_over }) => {
            (None, Some(expired(worker, entry, bound, handed_over)))
        }
        Err(Conversation::Ended(error)) => {
            let status = teardown.status.as_ref().map_or_else(
                |error| format!("unconfirmed reap (wait: {error})"),
                |status| format!("confirmed reap: {status}"),
            );
            (
                None,
                Some(failed(format!(
                    "the policy worker ended without a result ({error}; {status}); \
                 a policy that calls process.exit ends it the same way"
                ))),
            )
        }
    };
    let refusal = match (primary, original) {
        (Some(primary), Some(original)) => Some(primary.report(
            &format!(
                "original conversation failure:\n{}",
                Failure::from(original).to_text()
            ),
            "retain both conversation reports when retrying",
        )),
        (primary, original) => primary.or(original),
    }
    .or_else(|| {
        (!admitted).then(|| {
            failed("policy conversation completed, but required worker teardown failed".into())
        })
    });
    if let Some(refusal) = refusal {
        let refusal = if operational.is_empty() {
            refusal
        } else {
            refusal.report(&operational.join("\n"), TEARDOWN_REMEDY)
        };
        let failure = Failure::with_diagnostics(refusal, diagnostics);
        return Err(match cancellation::received() {
            Some(signal) => failure.cancelled_by(signal, entry),
            None => failure,
        });
    }
    if let Some(signal) = cancellation::received() {
        let refusal = cancellation::refusal(signal, entry);
        let refusal = if operational.is_empty() {
            refusal
        } else {
            refusal.report(&operational.join("\n"), TEARDOWN_REMEDY)
        };
        return Err(Failure::with_diagnostics(refusal, diagnostics));
    }
    // Every conversation failure was converted above; only a judged success
    // with confirmed reap and successful required removal reaches this branch.
    match success {
        Some((worker, decided, elapsed)) => Ok(Evaluation {
            worker,
            decided,
            diagnostics,
            operational,
            elapsed,
        }),
        None => Err(Failure::with_diagnostics(
            failed("worker supplied no judgment".into()),
            diagnostics,
        )),
    }
}

// Local operation reports stay separate from the bounded policy byte streams.
struct Teardown {
    pid: u32,
    status: io::Result<ExitStatus>,
    reports: Vec<String>,
    stop_failed: bool,
    directory: PathBuf,
    removed: io::Result<()>,
}

const TEARDOWN_REMEDY: &str = "inspect the named worker/OS operation before retrying; do not signal a PID that may have been reused; if private-directory removal failed, verify the owned path and remove it manually before retrying";

impl Teardown {
    fn describe(&self, worker: &Path) -> Vec<String> {
        let identity = format!("policy worker {} (pid {})", worker.display(), self.pid);
        let status = match &self.status {
            Ok(status) => format!("confirmed reap: {status}"),
            Err(error) => format!("wait: {error}; worker reap unconfirmed"),
        };
        let mut reports = self
            .reports
            .iter()
            .map(|report| format!("{identity}: {report}; {status}"))
            .collect::<Vec<_>>();
        if self.status.is_err() {
            reports.push(format!("{identity}: {status}"));
        }
        if let Err(error) = &self.removed {
            reports.push(format!(
                "{identity}: remove private directory {}: {error}",
                self.directory.display()
            ));
        }
        reports
    }
}

enum Conversation {
    /// The judge refused what the worker reported.
    Refused(Refusal),
    /// The worker broke the protocol, or is not this front's pair.
    Broken(Refusal),
    /// The channel failed or closed: the worker died, or exited on its own.
    Ended(FrameError),
    /// The deadline passed first, after the entry was handed over or before.
    Expired { handed_over: bool },
}

impl Conversation {
    fn failed(error: FrameError, handed_over: bool) -> Self {
        match error {
            FrameError::Io(error) if error.kind() == ErrorKind::TimedOut => {
                Conversation::Expired { handed_over }
            }
            other => Conversation::Ended(other),
        }
    }
}

/// What the worker is asked to evaluate, and within what.
struct Asked<'a> {
    worker: &'a Path,
    entry: &'a str,
    request: Value,
    limits: &'a Limits,
    caller: Option<&'a Measured>,
}

fn converse<T>(
    mut channel: Bounded<'_>,
    asked: Asked<'_>,
    started: Instant,
    judge: impl FnOnce(Outcome, Loaded<'_>) -> Result<T, Halt>,
) -> Result<(WorkerIdentity, T, Duration), Conversation> {
    let Asked {
        worker,
        entry,
        request,
        limits,
        caller,
    } = asked;
    let max = message_max(limits);
    let hello = receive(&mut channel, worker, false, max)?;
    let identity = verify_hello(&hello, worker).map_err(Conversation::Broken)?;

    write_frame(
        &mut channel,
        &json!({
            "type": "evaluate",
            "protocol": PROTOCOL,
            // Exactly the admitted file: `authority` refuses any path that
            // no string names exactly.
            "entry": entry,
            "request": request,
            // The worker's own copies, which nothing the policy does can
            // raise; the policy sees the same values in `request.limits`.
            "bounds": {
                "contextBytes": limits.context.value,
                "sourceBytes": limits.source.value,
                "sources": limits.sources.value,
                "messageBytes": limits.message.value,
            },
            "measured": caller.map(Measured::to_json).into_iter().collect::<Vec<_>>(),
        }),
    )
    .map_err(|error| Conversation::failed(error, false))?;

    let reported = receive(&mut channel, worker, true, max)?;
    let outcome = outcome(&reported).ok_or_else(|| {
        broken(
            worker,
            format!("unexpected result frame: {}", shown(&reported)),
        )
    })?;
    let loaded = Loaded {
        channel,
        worker,
        limits,
        caller,
        assembled: false,
        selected: false,
    };
    let decided = judge(outcome, loaded).map_err(|Halt(halted)| halted)?;
    Ok((identity, decided, started.elapsed()))
}

/// The next frame, or why there is none: a malformed frame is the worker
/// breaking the protocol, and anything else ends the conversation or its time.
fn receive(
    channel: &mut Bounded<'_>,
    worker: &Path,
    handed_over: bool,
    max: usize,
) -> Result<Value, Conversation> {
    match read_frame(channel, max) {
        Err(error @ (FrameError::TooLarge(..) | FrameError::NotJson(_))) => Err(broken(
            worker,
            format!("the policy worker sent a malformed frame: {error}"),
        )),
        other => other.map_err(|error| Conversation::failed(error, handed_over)),
    }
}

fn broken(worker: &Path, message: String) -> Conversation {
    Conversation::Broken(
        Refusal::new(
            "protocol_error",
            Stage::Worker,
            EXIT_WORKER,
            message,
            REBUILD,
        )
        .source(worker.to_string_lossy()),
    )
}

fn text(frame: &Value, field: &str) -> Option<String> {
    frame.get(field).and_then(Value::as_str).map(str::to_owned)
}

/// A frame, abbreviated for an error message: a context frame can be
/// megabytes long, and the message only needs to show what it is.
fn shown(frame: &Value) -> String {
    let text = frame.to_string();
    match text.char_indices().nth(400) {
        Some((at, _)) => format!("{}… ({} bytes)", &text[..at], text.len()),
        None => text,
    }
}

/// How a phase failed, as the worker reported it in a `failure` frame. Each
/// phase admits its own subset.
enum Fault {
    Breach(Breach),
    Unsettled,
    Unserializable { name: String, message: String },
    SourceUnreadable { source: String, message: String },
    Threw { name: String, message: String },
}

/// The failure a `failure` frame reports for `stage`, if it is one.
fn fault(frame: &Value, stage: &str) -> Option<Fault> {
    if text(frame, "type").as_deref() != Some("failure")
        || text(frame, "stage").as_deref() != Some(stage)
    {
        return None;
    }
    let flag = |field: &str| frame.get(field) == Some(&Value::Bool(true));
    if let Some(bound) = frame.get("bound") {
        let number = |field: &str| bound.get(field).and_then(Value::as_u64);
        return Some(Fault::Breach(Breach {
            bound: text(bound, "name")?,
            actual: number("actual"),
            source: text(bound, "source"),
            max_bytes: number("maxBytes"),
        }));
    }
    if flag("unsettled") {
        return Some(Fault::Unsettled);
    }
    if let Some(unreadable) = frame.get("sourceUnreadable") {
        return Some(Fault::SourceUnreadable {
            source: text(unreadable, "source")?,
            message: text(unreadable, "message")?,
        });
    }
    let message = text(frame, "message")?;
    let name = text(frame, "name").unwrap_or_else(|| "Error".to_owned());
    Some(if flag("unserializable") {
        Fault::Unserializable { name, message }
    } else {
        Fault::Threw { name, message }
    })
}

/// The worker's report on the entry, if the frame is one.
fn outcome(frame: &Value) -> Option<Outcome> {
    if text(frame, "type").as_deref() == Some("policy") {
        return frame.get("policy").cloned().map(Outcome::Policy);
    }
    if text(frame, "type").as_deref() == Some("failure")
        && text(frame, "stage").as_deref() == Some("validation")
    {
        return text(frame, "message").map(|message| Outcome::Invalid {
            location: text(frame, "location").unwrap_or_else(|| "policy".to_owned()),
            message,
        });
    }
    match fault(frame, "load")? {
        Fault::Breach(breach) => Some(Outcome::Breach(breach)),
        Fault::Unsettled => Some(Outcome::LoadUnsettled),
        Fault::Threw { name, message } => Some(Outcome::LoadFailed { name, message }),
        _ => None,
    }
}

/// The worker's report on the context, if the frame is one.
fn assembled(frame: &Value) -> Option<Assembled> {
    if text(frame, "type").as_deref() == Some("context") {
        return Some(Assembled::Context {
            context: frame.get("context")?.clone(),
            measured: measured(frame.get("measured")?)?,
        });
    }
    Some(match fault(frame, "context")? {
        Fault::Breach(breach) => Assembled::Breach(breach),
        Fault::Unsettled => Assembled::Unsettled,
        Fault::Unserializable { name, message } => Assembled::Unserializable { name, message },
        Fault::SourceUnreadable { source, message } => {
            Assembled::SourceUnreadable { source, message }
        }
        Fault::Threw { name, message } => Assembled::Threw { name, message },
    })
}

/// The measured sources the worker lists: each exactly a canonical name, how
/// it was read, its bytes and their SHA-256.
fn measured(list: &Value) -> Option<Vec<Measured>> {
    list.as_array()?
        .iter()
        .map(|entry| {
            let fields = entry.as_object()?;
            let name = fields
                .get("name")?
                .as_str()
                .filter(|name| !name.is_empty())?;
            let via = fields
                .get("via")?
                .as_str()
                .filter(|via| [crate::context::VIA_CALLER, "readText", "readJson"].contains(via))?;
            let sha256 = fields.get("sha256")?.as_str().filter(|digest| {
                digest.len() == 64
                    && digest
                        .bytes()
                        .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
            })?;
            (fields.len() == 4).then_some(())?;
            Some(Measured {
                name: name.to_owned(),
                via: via.to_owned(),
                bytes: fields.get("bytes")?.as_u64()?,
                sha256: sha256.to_owned(),
            })
        })
        .collect()
}

/// The worker's report on `select`, if the frame is one.
fn produced(frame: &Value) -> Option<Produced> {
    if text(frame, "type").as_deref() == Some("selection") {
        return frame.get("result").cloned().map(Produced::Result);
    }
    Some(match fault(frame, "select")? {
        Fault::Breach(breach) => Produced::Breach(breach),
        Fault::Unsettled => Produced::Unsettled,
        Fault::Unserializable { name, message } => Produced::Unserializable { name, message },
        Fault::Threw { name, message } => Produced::Threw { name, message },
        Fault::SourceUnreadable { .. } => return None,
    })
}

/// Output past the diagnostics bound requests worker teardown; captured bytes
/// stay bounded even when teardown cannot be confirmed.
fn output_limit(entry: &str, limits: &Limits) -> Refusal {
    let bound = limits.diagnostics.value;
    Refusal::new(
        "output_limit",
        Stage::Evaluation,
        EXIT_REFUSED,
        format!(
            "the policy {entry} printed more than {bound} bytes on stdout and stderr together, \
             so worker teardown was attempted and nothing was launched; its first {bound} bytes are \
             kept, and the rest was read and discarded"
        ),
        "print less while selecting: keep diagnostics to short notes, and write anything longer \
         to a file of your own; this bound is fixed",
    )
    .source(entry)
    .bound(limits.diagnostics)
}

/// The refusal when the deadline passes: after the entry was handed over, the
/// policy held evaluation; before, the worker never became ready for it.
fn expired(worker: &Path, entry: &str, bound: Bound, handed_over: bool) -> Refusal {
    let limit = bound.to_text();
    let raise = format!(
        "or allow more time with timeoutMs in the owner settings or --timeout-ms, up to \
         {SELECTION_MAX_MS}"
    );
    let refusal = if handed_over {
        Refusal::new(
            "selection_timeout",
            Stage::Evaluation,
            EXIT_TIMEOUT,
            format!(
                "the policy entry {entry} did not return a result within the selection bound \
                 of {limit}, so worker teardown was attempted and nothing was launched"
            ),
            format!(
                "make the policy return sooner: a loop, or an await on work that never \
                 settles, holds its evaluation; {raise}"
            ),
        )
        .source(entry)
    } else {
        Refusal::new(
            "selection_timeout",
            Stage::Evaluation,
            EXIT_TIMEOUT,
            format!(
                "the policy worker {} was not ready for the policy within the selection bound \
                 of {limit}, so worker teardown was attempted and nothing was launched",
                worker.display()
            ),
            format!("{REBUILD}; on a heavily loaded machine, {raise}"),
        )
        .source(worker.to_string_lossy())
    };
    let refusal = refusal.bound(bound);
    match bound.set_by() {
        Some(flag) => refusal.input(flag),
        None => refusal,
    }
}

/// Reap the worker. It has `CLEANUP_GRACE` to exit, after `signal` when one is
/// given, and is then killed. Only the worker is signalled: it shares the
/// caller's process group, so a group signal would reach this process and its
/// caller too.
struct Stopped {
    status: io::Result<ExitStatus>,
    reports: Vec<String>,
    drain_by: Instant,
    stop_failed: bool,
}

fn child_watching() -> io::Result<()> {
    #[cfg(test)]
    if crate::native_entry_tests::skip_worker_repair() {
        return Ok(());
    }
    let mut action = unsafe { std::mem::zeroed::<libc::sigaction>() };
    let queried = unsafe { libc::sigaction(libc::SIGCHLD, std::ptr::null(), &mut action) };
    // F02: the physical query ran; only its report is substituted.
    #[cfg(test)]
    let queried = {
        use crate::native_entry_tests::faults::{query, Query};
        match query("watch-action-query", queried) {
            Query::Real => queried,
            Query::Substituted => -1,
            Query::Ignored => {
                action = unsafe { std::mem::zeroed::<libc::sigaction>() };
                0
            }
        }
    };
    if queried == -1 {
        return Err(io::Error::last_os_error());
    }
    if action.sa_sigaction == libc::SIG_IGN || action.sa_flags & libc::SA_NOCLDWAIT != 0 {
        let mut default = unsafe { std::mem::zeroed::<libc::sigaction>() };
        default.sa_sigaction = libc::SIG_DFL;
        if unsafe { libc::sigemptyset(&mut default.sa_mask) } == -1 {
            return Err(io::Error::last_os_error());
        }
        let repair = || unsafe { libc::sigaction(libc::SIGCHLD, &default, std::ptr::null_mut()) };
        #[cfg(test)]
        let repaired =
            crate::native_entry_tests::faults::mutation_c("default-action-repair", repair);
        #[cfg(not(test))]
        let repaired = repair();
        if repaired == -1 {
            return Err(io::Error::last_os_error());
        }
    }
    Ok(())
}

fn kill_and_wait(child: &mut Child) -> Stopped {
    bounded_stop(child, None, false)
}
fn stop(child: &mut Child, signal: Option<libc::c_int>) -> Stopped {
    bounded_stop(child, signal, true)
}

fn bounded_stop(child: &mut Child, signal: Option<libc::c_int>, graceful: bool) -> Stopped {
    let began = Instant::now();
    let by = began + Duration::from_secs(2);
    let grace = if graceful {
        (began + CLEANUP_GRACE).min(by)
    } else {
        began
    };
    let drain_by = by + DRAIN_GRACE;
    // S03's private deadline break lets the poll loop outlive the 2-second bound.
    #[cfg(test)]
    let by_loop = by + native_tests::deadline_slack();
    #[cfg(not(test))]
    let by_loop = by;
    let mut reports = Vec::new();
    let mut stop_failed = false;
    let pid = child.id() as libc::pid_t;
    if let Some(signal) = signal {
        #[cfg(not(test))]
        let stopped = unsafe { libc::kill(pid, signal) };
        #[cfg(test)]
        let stopped = native_tests::stop(pid, signal);
        if stopped == -1 {
            stop_failed = true;
            reports.push(format!(
                "stop pid {pid} signal {signal}: {}",
                io::Error::last_os_error()
            ));
        }
    }
    let mut killed = false;
    let mut skip_grace = false;
    let mut first_interrupted = None;
    let mut interrupts = 0u64;
    loop {
        if Instant::now() >= by_loop {
            #[cfg(test)]
            native_tests::teardown_took(began.elapsed());
            if interrupts > 0 {
                reports.push(format!(
                    "try_wait first errno {first_interrupted:?}, EINTR retries {interrupts}"
                ));
            }
            return Stopped {
                status: Err(io::Error::new(ErrorKind::TimedOut, "worker teardown deadline expired; child ownership released, actual reap unconfirmed")),
                reports, drain_by, stop_failed,
            };
        }
        #[cfg(not(test))]
        let observed = child.try_wait();
        #[cfg(test)]
        let observed = native_tests::poll(child);
        match observed {
            Ok(Some(status)) => {
                if interrupts > 0 {
                    reports.push(format!("try_wait first errno {first_interrupted:?}, EINTR retries {interrupts}; actual reap confirmed"));
                }
                return Stopped {
                    status: Ok(status),
                    reports,
                    drain_by,
                    stop_failed,
                };
            }
            Ok(None) => {}
            Err(error) if error.kind() == ErrorKind::Interrupted => {
                first_interrupted.get_or_insert(error.raw_os_error());
                interrupts = interrupts.saturating_add(1);
            }
            Err(error) => {
                // S03's private identity-guard break keeps polling after ECHILD.
                #[cfg(test)]
                let lost = error.raw_os_error() == Some(libc::ECHILD)
                    && !native_tests::identity_guard_broken();
                #[cfg(not(test))]
                let lost = error.raw_os_error() == Some(libc::ECHILD);
                reports.push(format!("try_wait pid {pid}: {error}"));
                if lost {
                    return Stopped {
                        status: Err(error),
                        reports,
                        drain_by,
                        stop_failed,
                    };
                }
                skip_grace = true;
            }
        }
        if !killed && (skip_grace || Instant::now() >= grace) {
            // No try_wait reported reap/ECHILD. This owner alone may signal
            // this PID; the worker shares our group, which must not be killed.
            #[cfg(not(test))]
            let killed_result = child.kill();
            #[cfg(test)]
            let killed_result = if native_tests::stop(pid, libc::SIGKILL) == -1 {
                Err(io::Error::last_os_error())
            } else {
                Ok(())
            };
            if let Err(error) = killed_result {
                stop_failed = true;
                reports.push(format!("kill pid {pid}: {error}"));
            }
            killed = true;
        }
        thread::sleep(
            Duration::from_millis(10).min(by_loop.saturating_duration_since(Instant::now())),
        );
    }
}

/// The protocol channel with the deadline applied: each read or write waits
/// only for the time left, and once none is left it fails with `TimedOut`. It
/// waits in slices of at most `POLL`, and between slices it fails at once if
/// the policy's output has passed its bound or a handled signal was received.
#[derive(Clone, Copy)]
struct Bounded<'a> {
    channel: &'a UnixStream,
    deadline: Instant,
    overflow: &'a AtomicBool,
}

impl Bounded<'_> {
    /// The next slice to wait, which is never zero: std refuses a zero socket
    /// timeout, and rounds a sub-microsecond one up to a microsecond.
    fn slice(&self) -> io::Result<Duration> {
        if self.overflow.load(Ordering::SeqCst) {
            return Err(io::Error::other("the policy's output passed its bound"));
        }
        if cancellation::received().is_some() {
            return Err(io::Error::other("a signal cancelled the selection"));
        }
        let left = self.deadline.saturating_duration_since(Instant::now());
        if left.is_zero() {
            Err(ErrorKind::TimedOut.into())
        } else {
            Ok(left.min(POLL))
        }
    }
}

/// Whether an error is a socket timeout expiring: `WouldBlock` (`EAGAIN`) on
/// Unix, though std documents either kind.
fn expired_slice(error: &io::Error) -> bool {
    matches!(error.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut)
}

impl Read for Bounded<'_> {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        loop {
            // macOS refuses a timeout on a socket whose peer has closed, with
            // `EINVAL`, while frames the peer wrote are still unread. A read
            // from such a socket cannot block, so it needs no timeout.
            match self.channel.set_read_timeout(Some(self.slice()?)) {
                Err(error) if error.raw_os_error() != Some(libc::EINVAL) => return Err(error),
                _ => {}
            }
            let mut channel = self.channel;
            match channel.read(buffer) {
                Err(error) if expired_slice(&error) => {}
                other => return other,
            }
        }
    }
}

impl Write for Bounded<'_> {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        loop {
            self.channel.set_write_timeout(Some(self.slice()?))?;
            let mut channel = self.channel;
            match channel.write(buffer) {
                Err(error) if expired_slice(&error) => {}
                other => return other,
            }
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn verify_hello(hello: &Value, worker: &Path) -> Result<WorkerIdentity, Refusal> {
    let text = |field: &str| hello.get(field).and_then(Value::as_str).unwrap_or("absent");
    if hello.get("type").and_then(Value::as_str) != Some("hello") {
        return Err(Refusal::new(
            "protocol_error",
            Stage::Worker,
            EXIT_WORKER,
            format!("the policy worker's first frame is not a hello: {hello}"),
            REBUILD,
        )
        .source(worker.to_string_lossy()));
    }
    let protocol = hello.get("protocol").and_then(Value::as_u64);
    if protocol != Some(PROTOCOL)
        || text("packageVersion") != PACKAGE_VERSION
        || text("buildId") != BUILD_ID
    {
        let protocol = protocol.map_or_else(|| "absent".to_owned(), |p| p.to_string());
        return Err(Refusal::new(
            "worker_identity_mismatch",
            Stage::Worker,
            EXIT_WORKER,
            format!(
                "the policy worker {} is not this front's pair: it reports protocol {protocol}, \
                 package {}, build {}; this front needs protocol {PROTOCOL}, package \
                 {PACKAGE_VERSION}, build {BUILD_ID}",
                worker.display(),
                text("packageVersion"),
                text("buildId"),
            ),
            REBUILD,
        )
        .source(worker.to_string_lossy()));
    }
    Ok(WorkerIdentity {
        path: worker.to_owned(),
        package_version: PACKAGE_VERSION.to_owned(),
        build_id: BUILD_ID.to_owned(),
        bun_version: text("bunVersion").to_owned(),
    })
}

/// A close-on-exec duplicate of `stream` numbered 3 or higher. A caller that
/// started this process with a standard descriptor closed would otherwise let
/// the socket land on 0-2, where the child's stdio setup overwrites it.
fn above_stdio(stream: &UnixStream) -> std::io::Result<OwnedFd> {
    // SAFETY: F_DUPFD_CLOEXEC returns a new descriptor this function owns.
    let fd = unsafe { libc::fcntl(stream.as_raw_fd(), libc::F_DUPFD_CLOEXEC, CHANNEL_FD) };
    if fd == -1 {
        return Err(std::io::Error::last_os_error());
    }
    // SAFETY: `fd` is a fresh descriptor owned by nothing else.
    Ok(unsafe { OwnedFd::from_raw_fd(fd) })
}

/// Every descriptor this process holds, as its descriptor directory lists
/// them. The descriptor limit is no bound: a caller can leave one open above
/// 65,536, or lower its soft limit below one it already holds. Every
/// descriptor this process opens itself is close-on-exec, so the ones the
/// sweep must mark were all inherited, and all are listed here before the
/// worker's fork. The listing includes its own directory's descriptor, closed
/// again by then, which the sweep passes over as `EBADF`.
fn held_descriptors() -> io::Result<Vec<RawFd>> {
    let mut held = Vec::new();
    for entry in fs::read_dir(DESCRIPTORS)? {
        let name = entry?.file_name();
        let fd = name
            .to_str()
            .and_then(|name| name.parse().ok())
            .ok_or_else(|| {
                io::Error::other(format!(
                    "{DESCRIPTORS} lists {name:?}, which is not a descriptor"
                ))
            })?;
        held.push(fd);
    }
    Ok(held)
}

/// Both diagnostic streams, each read to its end on a thread of its own and
/// kept within one shared bound. Past the bound the threads go on reading,
/// so that the worker never blocks on a full pipe, but keep nothing more, and
/// set the flag the channel watches. The threads never take a handled signal,
/// which is left to the thread that converses.
struct Capture {
    captured: Arc<Mutex<Diagnostics>>,
    overflow: Arc<AtomicBool>,
    done: mpsc::Receiver<()>,
    streams: usize,
}

impl Capture {
    fn start(
        stdout: Option<impl Read + Send + 'static>,
        stderr: Option<impl Read + Send + 'static>,
        limit: u64,
    ) -> (Capture, Vec<String>) {
        #[cfg(test)]
        native_tests::creator_before();
        let captured = Arc::new(Mutex::new(Diagnostics::default()));
        let overflow = Arc::new(AtomicBool::new(false));
        let (sender, done) = mpsc::channel();
        let limit = usize::try_from(limit).unwrap_or(usize::MAX);
        let mut capture = Capture {
            captured,
            overflow,
            done,
            streams: 0,
        };
        let mut reports = Vec::new();
        let shield = match cancellation::Shield::block() {
            Ok(shield) => shield,
            Err(error) => {
                #[cfg(test)]
                native_tests::creator_after();
                return (capture, vec![format!("reader mask block: {error}")]);
            }
        };
        let mut spawn = |stream: Option<Box<dyn Read + Send>>, stdout: bool| -> io::Result<()> {
            let Some(mut stream) = stream else {
                return Ok(());
            };
            #[cfg(test)]
            if native_tests::omit_create(stdout) {
                return Err(io::Error::from_raw_os_error(libc::EAGAIN));
            }
            let (captured, overflow, sender) = (
                Arc::clone(&capture.captured),
                Arc::clone(&capture.overflow),
                sender.clone(),
            );
            #[cfg(test)]
            let reader_sink = native_tests::reader_sink();
            #[cfg(test)]
            let finished_sink = native_tests::finished_sink();
            thread::Builder::new()
                .name(
                    if stdout {
                        "policy-stdout"
                    } else {
                        "policy-stderr"
                    }
                    .into(),
                )
                .spawn(move || {
                    #[cfg(test)]
                    native_tests::record_reader(reader_sink, stdout);
                    // The reader retains its inherited shield until thread exit.
                    let mut chunk = [0; 8192];
                    loop {
                        match stream.read(&mut chunk) {
                            Ok(0) => break,
                            Ok(read) => {
                                let mut captured = captured
                                    .lock()
                                    .unwrap_or_else(|poisoned| poisoned.into_inner());
                                let room = limit
                                    .saturating_sub(captured.stdout.len() + captured.stderr.len());
                                let kept = read.min(room);
                                let buffer = if stdout {
                                    &mut captured.stdout
                                } else {
                                    &mut captured.stderr
                                };
                                buffer.extend_from_slice(&chunk[..kept]);
                                if kept < read {
                                    overflow.store(true, Ordering::SeqCst);
                                }
                            }
                            Err(error) if error.kind() == ErrorKind::Interrupted => {}
                            Err(_) => break,
                        }
                    }
                    #[cfg(test)]
                    native_tests::record_finished(finished_sink, stdout);
                    let _ = sender.send(());
                })?;
            capture.streams += 1;
            Ok(())
        };
        let created = spawn(stdout.map(|s| Box::new(s) as Box<dyn Read + Send>), true)
            .and_then(|()| spawn(stderr.map(|s| Box::new(s) as Box<dyn Read + Send>), false));
        if let Err(error) = created {
            reports.push(format!("reader thread create: {error}"));
        }
        if let Err(error) = shield.restore() {
            reports.push(format!("reader creator mask restore: {error}"));
        }
        #[cfg(test)]
        native_tests::creator_after();
        (capture, reports)
    }

    fn overflowed(&self) -> bool {
        self.overflow.load(Ordering::SeqCst)
    }

    /// Everything kept so far, once both streams end or `by` passes, and
    /// whether any output went past the bound.
    fn collect(self, by: Instant) -> (Diagnostics, bool) {
        #[cfg(test)]
        let began = Instant::now();
        #[cfg(test)]
        let mut completed = 0;
        for _ in 0..self.streams {
            if self
                .done
                .recv_timeout(by.saturating_duration_since(Instant::now()))
                .is_err()
            {
                break;
            }
            #[cfg(test)]
            {
                completed += 1;
            }
        }
        #[cfg(test)]
        native_tests::collected(
            self.streams,
            completed,
            by.saturating_duration_since(began),
            began.elapsed(),
        );
        let captured = self
            .captured
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clone();
        (captured, self.overflowed())
    }
}

#[cfg(test)]
#[path = "worker_admission_tests.rs"]
mod admission_tests;

#[cfg(test)]
pub(crate) mod native_tests;

#[cfg(test)]
mod tests {
    use super::*;

    // A worker exits as soon as it has written its last frame, and macOS
    // refuses a timeout on a socket whose peer has closed.
    #[test]
    fn a_frame_written_before_the_worker_exited_is_still_read() {
        let (front, mut worker) = UnixStream::pair().unwrap();
        write_frame(&mut worker, &json!({"type": "result"})).unwrap();
        drop(worker);
        let overflow = AtomicBool::new(false);
        let channel = Bounded {
            channel: &front,
            deadline: Instant::now() + Duration::from_secs(5),
            overflow: &overflow,
        };
        assert_eq!(
            read_frame(channel, 1024).unwrap(),
            json!({"type": "result"})
        );
    }
}
