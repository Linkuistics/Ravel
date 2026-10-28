//! Supervise the internal Rhai worker through a bounded private socket.

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
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::{Duration, Instant};

use serde_json::Value;
#[cfg(test)]
use serde_json::json;

use crate::cancellation;
#[cfg(test)]
use crate::frame::write_frame;
use crate::frame::{FrameError, read_frame};
use crate::limits::{Bound, Limits, SELECTION_MAX_MS};
use crate::refusal::{
    Diagnostics, EXIT_REFUSED, EXIT_TIMEOUT, EXIT_WORKER, Failure, Refusal, Stage,
};

#[cfg(test)]
pub const PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// The descriptor the worker's channel occupies, fixed on both sides.
pub(crate) const CHANNEL_FD: RawFd = 3;

/// The directory listing this process's open descriptors.
#[cfg(target_os = "linux")]
const DESCRIPTORS: &str = "/proc/self/fd";
#[cfg(not(target_os = "linux"))]
const DESCRIPTORS: &str = "/dev/fd";

/// How long to keep collecting diagnostics after the worker is reaped. A
/// descendant the configuration script left holding its stdout could otherwise keep a drain
/// open forever; detached policy children are outside the contract, so their
/// late output is not waited for. Both streams share the one grace.
const DRAIN_GRACE: Duration = Duration::from_secs(1);

/// How long a worker has to exit, after TERM at the deadline or by itself
/// after its result, before it is killed.
const CLEANUP_GRACE: Duration = Duration::from_secs(1);

/// The longest a channel read or write waits before it looks again at the
/// diagnostics bound, so that a configuration script printing without end is stopped for
/// that, promptly, rather than at the deadline.
pub(crate) const POLL: Duration = Duration::from_millis(50);

/// How to start one worker process. The supervision owns everything about the
/// child except what it is: the program and its `arg0`, its whole environment,
/// and the remedy that a failure to start it points at.
pub(crate) struct Launch<'a> {
    pub program: &'a Path,
    /// The name the child sees as its own, when it must differ from the path.
    pub arg0: Option<&'a std::ffi::OsStr>,
    /// The child's whole environment.
    pub environment: Vec<(OsString, OsString)>,
    /// The remedy for a worker that is missing, broken or not ready in time.
    pub rebuild: &'static str,
}

/// What a supervised worker left behind when its conversation succeeded and
/// its teardown was confirmed.
pub(crate) struct Supervised<T> {
    /// What the conversation decided.
    pub decided: T,
    pub diagnostics: Diagnostics,
    pub operational: Vec<String>,
}

/// Run one worker process, bounded, through `converse`, and tear it down.
///
/// The worker gets a private mode-0700 empty directory, null stdin, the
/// environment of `launch` and the private socket at descriptor 3, and
/// inherits no other descriptor. `converse` talks to it over the bounded
/// channel and gets the instant the worker started, from which the selection
/// bound counts. Whatever `converse` returns, the worker is stopped and
/// reaped, its directory removed and its output collected before a success
/// is returned, and a cancellation that arrived meanwhile wins over every
/// other outcome. `entry` names the configuration script in the refusals about it.
pub(crate) fn supervise<T>(
    launch: Launch<'_>,
    entry: &str,
    limits: &Limits,
    converse: impl FnOnce(Bounded<'_>, Instant) -> Result<T, Conversation>,
) -> Result<Supervised<T>, Failure> {
    let Launch {
        program: worker,
        arg0,
        environment,
        rebuild,
    } = launch;
    let bound = limits.selection;
    let failed = |message: String| {
        Refusal::new(
            "worker_failed",
            Stage::Worker,
            EXIT_WORKER,
            message,
            rebuild,
        )
        .source(worker.to_string_lossy())
    };

    child_watching()
        .map_err(|error| failed(format!("cannot establish worker watching: {error}")))?;

    // The worker starts in an owned empty directory. The
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
        if let Some(arg0) = arg0 {
            command.arg0(arg0);
        }
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
                    "cannot list this process's open descriptors in {DESCRIPTORS}, so the Rhai \
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
                        "the Rhai worker {} cannot be started: {error}",
                        worker.display()
                    ),
                    rebuild,
                )
                .source(worker.to_string_lossy())
            } else {
                failed(format!(
                    "the Rhai worker {} cannot be started: {error}",
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
    // Retain the conversation even when cancellation supersedes it: its
    // original cause and metadata still belong in the failure report.
    let result = if reader_reports.is_empty() {
        converse(channel, started)
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
            if dropped { Vec::new() } else { stopped.reports }
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
        Err(Conversation::Expired { handed_over }) => (
            None,
            Some(expired(worker, entry, bound, handed_over, rebuild)),
        ),
        Err(Conversation::Ended(error)) => {
            let status = teardown.status.as_ref().map_or_else(
                |error| format!("unconfirmed reap (wait: {error})"),
                |status| format!("confirmed reap: {status}"),
            );
            (
                None,
                Some(failed(format!(
                    "the Rhai worker ended without a result ({error}; {status})"
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
            failed("configuration evaluation completed, but required worker teardown failed".into())
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
        Some(decided) => Ok(Supervised {
            decided,
            diagnostics,
            operational,
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
        let identity = format!("Rhai worker {} (pid {})", worker.display(), self.pid);
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

pub(crate) enum Conversation {
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
    pub(crate) fn failed(error: FrameError, handed_over: bool) -> Self {
        match error {
            FrameError::Io(error) if error.kind() == ErrorKind::TimedOut => {
                Conversation::Expired { handed_over }
            }
            other => Conversation::Ended(other),
        }
    }
}

pub(crate) fn receive_for(
    channel: &mut Bounded<'_>,
    worker: &Path,
    rebuild: &str,
    handed_over: bool,
    max: usize,
) -> Result<Value, Conversation> {
    match read_frame(channel, max) {
        Err(error @ (FrameError::TooLarge(..) | FrameError::NotJson(_))) => Err(broken_for(
            worker,
            rebuild,
            format!("the Rhai worker sent a malformed frame: {error}"),
        )),
        other => other.map_err(|error| Conversation::failed(error, handed_over)),
    }
}

pub(crate) fn broken_for(worker: &Path, rebuild: &str, message: String) -> Conversation {
    Conversation::Broken(
        Refusal::new(
            "protocol_error",
            Stage::Worker,
            EXIT_WORKER,
            message,
            rebuild,
        )
        .source(worker.to_string_lossy()),
    )
}

fn output_limit(entry: &str, limits: &Limits) -> Refusal {
    let bound = limits.diagnostics.value;
    Refusal::new(
        "output_limit",
        Stage::Evaluation,
        EXIT_REFUSED,
        format!(
            "the configuration script {entry} printed more than {bound} bytes on stdout and stderr together, \
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
fn expired(worker: &Path, entry: &str, bound: Bound, handed_over: bool, rebuild: &str) -> Refusal {
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
                "the configuration entry {entry} did not return a result within the selection bound \
                 of {limit}, so worker teardown was attempted and nothing was launched"
            ),
            format!(
                "make the configuration script return sooner: a loop, or an await on work that never \
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
                "the Rhai worker {} was not ready for the configuration script within the selection bound \
                 of {limit}, so worker teardown was attempted and nothing was launched",
                worker.display()
            ),
            format!("{rebuild}; on a heavily loaded machine, {raise}"),
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
        use crate::native_entry_tests::faults::{Query, query};
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
                status: Err(io::Error::new(
                    ErrorKind::TimedOut,
                    "worker teardown deadline expired; child ownership released, actual reap unconfirmed",
                )),
                reports,
                drain_by,
                stop_failed,
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
/// the configuration script's output has passed its bound or a handled signal was received.
#[derive(Clone, Copy)]
pub(crate) struct Bounded<'a> {
    channel: &'a UnixStream,
    deadline: Instant,
    overflow: &'a AtomicBool,
}

impl<'a> Bounded<'a> {
    /// The parts a runtime needs to take over the conversation: the channel,
    /// the instant the evaluation ends, and the flag that the configuration script's output
    /// passed its bound. The handshake used this channel with timeouts, and
    /// the runtime keeps the same stops.
    pub(crate) fn parts(&self) -> (&'a UnixStream, Instant, &'a AtomicBool) {
        (self.channel, self.deadline, self.overflow)
    }
}

impl Bounded<'_> {
    /// The next slice to wait, which is never zero: std refuses a zero socket
    /// timeout, and rounds a sub-microsecond one up to a microsecond.
    fn slice(&self) -> io::Result<Duration> {
        if self.overflow.load(Ordering::SeqCst) {
            return Err(io::Error::other(
                "the configuration script's output passed its bound",
            ));
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
