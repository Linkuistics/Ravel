//! Spawning one child directly and supervising it until it ends.

use std::ffi::OsStr;
use std::fs::File;
use std::io::{Read as _, Write as _};
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd, RawFd};
use std::os::unix::process::CommandExt as _;
use std::path::Path;
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::atomic::{AtomicI32, AtomicU8, Ordering};
use std::time::{Duration, Instant};

use crate::channel::Channel;
use crate::error::LaunchError;

/// How long the supervisor waits between checks of the child's liveness and the
/// completion channel.
///
/// Not a knob. The interval only bounds how late an escalation starts, and the
/// escalation's own graces are measured in seconds — a caller tuning this would
/// be tuning latency it cannot observe.
const POLL_INTERVAL: Duration = Duration::from_millis(500);

/// The two waits of the kill escalation.
///
/// **The escalation exists because an interactive child is never reaped on its
/// own.** A child that returns to a prompt after finishing its work has not
/// exited and will not: it sits waiting for input that is not coming. The channel
/// is the only evidence it is done, and ending it is therefore the launcher's
/// job — which is a job only the launcher can do, since it is the child's own
/// parent process, outside whatever sandbox the child runs under. A child asked
/// to end itself may simply be denied (macOS Seatbelt refuses a same-sandbox
/// process signalling its own session), and denied silently.
///
/// `grace` runs from the channel's appearance to SIGTERM, so a child that
/// signalled mid-operation gets to finish that operation and let its own call
/// return. `kill_grace` runs from SIGTERM to SIGKILL, for a child that installs
/// a handler and declines to die.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Escalation {
    pub grace: Duration,
    pub kill_grace: Duration,
}

/// Everything one launch is.
///
/// Every field is the caller's: this crate supplies no default program, no
/// default environment, and no default variable name. What it supplies is that
/// the `argv` is spawned **whole and directly** — no shell, no appended
/// argument, no reordering — and that the child's environment is the caller's
/// own minus `scrub`, plus `grant`, plus the one channel path.
pub struct Launch<'a> {
    /// The program and arguments, built by the caller with
    /// [`Argv::new`](crate::Argv::new).
    pub argv: &'a crate::Argv,
    /// Optional exit channel and the variable publishing its path. A supervisor
    /// launch with none ends only on its child's exit or its own cancellation.
    pub channel: Option<(&'a Channel, &'a str)>,
    /// Variable names removed from the child's inherited environment.
    ///
    /// **Scrubbing is the caller's obligation and this is where it is
    /// discharged.** An environment is inherited, not addressed: a launcher
    /// that merely declines to *set* its own control variables still hands the
    /// child whatever its own environment carried — including, for a nested
    /// launcher, a live channel path belonging to somebody else's launch, which
    /// is authority to end a session nobody meant to grant.
    pub scrub: &'a [&'a OsStr],
    /// Values set after the scrub, each replacing an inherited one: the
    /// caller's own control variables, which only it and the child agree on.
    pub grant: &'a [(&'a OsStr, &'a OsStr)],
    /// `Some` for a **transparent** caller: a wrapper whose own caller should
    /// see the child as though it had been started directly. The child then
    /// receives the signal mask and every disposition that survives exec as
    /// [`EntrySignals`] records them, in place of this crate's defaults, and
    /// none of the launcher's own handlers. Its pending signals stay the
    /// launcher's, as for any spawned child.
    ///
    /// A transparent launcher is also cancelled by SIGINT when it has a
    /// terminal. A typed Ctrl-C reaches the child's group and never the
    /// launcher, so an interrupt that does reach a transparent launcher was
    /// sent to it, and means what TERM or HUP would.
    pub transparent: Option<&'a EntrySignals>,
    /// The child's working directory. `None` inherits the launcher's, which is
    /// rarely what a launcher wants: it is wherever a human happened to be
    /// standing.
    pub cwd: Option<&'a Path>,
    pub escalation: Escalation,
}

/// A noninteractive supervisor launch. It carries no completion channel:
/// only the child's exit or the caller's cancellation can end it.
/// Both output streams go to the regular file passed to `run_noninteractive`.
pub struct NoninteractiveLaunch<'a> {
    pub argv: &'a crate::Argv,
    pub scrub: &'a [&'a OsStr],
    pub cwd: Option<&'a Path>,
    pub escalation: Escalation,
}

struct Job<'a> {
    argv: &'a crate::Argv,
    completion: Option<(&'a Channel, &'a str)>,
    scrub: &'a [&'a OsStr],
    grant: &'a [(&'a OsStr, &'a OsStr)],
    transparent: Option<&'a EntrySignals>,
    cwd: Option<&'a Path>,
    escalation: Escalation,
}

impl<'a> From<Launch<'a>> for Job<'a> {
    fn from(launch: Launch<'a>) -> Self {
        Self {
            argv: launch.argv,
            completion: launch.channel,
            scrub: launch.scrub,
            grant: launch.grant,
            transparent: launch.transparent,
            cwd: launch.cwd,
            escalation: launch.escalation,
        }
    }
}

/// How a launch ended.
///
/// `Escalated` and `Exited` both describe a child that is gone; they differ in
/// *who ended it*, which is what a caller needs to distinguish a launch that
/// completed its work from one that fell over. `signalled` is
/// orthogonal to all three: a child that signals and then exits before the
/// grace elapses ends `Exited`, signalled, and is a perfectly ordinary
/// completion.
#[derive(Debug)]
pub struct Ended {
    pub end: End,
    pub status: ExitStatus,
    pub elapsed: Duration,
    /// Whether the channel existed once the child was reaped, whatever it
    /// held: its appearance is the whole signal. Looked for after the reap, so
    /// a child that signals and exits at once has still signalled.
    pub signalled: bool,
    /// Whether the child's process group was confirmed gone after the reap.
    ///
    /// **A caller acts on the ending only beside [`Group::Gone`].** The child
    /// is reaped and `status` is its own whatever this says. But a member of its
    /// group that survived two SIGKILLs is still running: it can hold a lock,
    /// still write an output, or still hold shared epoch admission. A caller
    /// that relaunched, published or reported success beside it would be
    /// acting beside it.
    pub group: Group,
}

/// What remained of the child's process group once the launch ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Group {
    /// The system answered that no such process group exists — the only answer
    /// that confirms it.
    Gone,
    /// The group still answered a second after it was killed, or the query
    /// failed in some other way. `pgid` names it for a diagnostic. It was the
    /// child's pid, so once the child is reaped it may be reused: name it, but
    /// never signal it.
    Present { pgid: i32 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum End {
    /// The child exited of its own accord — whether or not it signalled on the
    /// way out.
    Exited,
    /// The escalation ended the child: its channel appeared, the grace elapsed
    /// with the child still running, and it was sent SIGTERM.
    ///
    /// This is deliberately *narrower* than "the channel appeared", which
    /// [`Ended::signalled`] already reports. A child that signals and then
    /// exits inside its own grace was never touched, and comes back `Exited`
    /// and signalled.
    Escalated,
    /// The *launcher's* process was sent SIGTERM or SIGHUP — or SIGINT, for a
    /// launch with no terminal — **during this launch**. An interactive or
    /// noninteractive child's group was sent the same signal and, after the
    /// kill-grace, SIGKILL; a confined child's group was killed at once. Either
    /// way it was reaped, never left orphaned onto the terminal. The channel
    /// cannot express this case — an interrupt normally leaves no channel file.
    ///
    /// **The signal is carried rather than merely noted** so a launcher can
    /// report it onward. A process that catches a termination signal, tidies up
    /// and then exits 0 tells its own parent it finished its work; the only way
    /// to say what actually happened is to die of the same signal, and that
    /// needs its number. [`reraise`] is that ending, and this field is its
    /// argument.
    ///
    /// A signal arriving *between* launches is not this: `run` discards it, and
    /// [`take_interrupt`] is where a looping launcher collects it.
    Interrupted { signal: i32 },
}

/// The supervisor's state machine: idle until the channel appears, then timed
/// toward SIGTERM and finally SIGKILL, after which only the exit is awaited.
enum Watch {
    Running,
    Signalled(Instant),
    Terminated(Instant),
    /// SIGKILL has been sent; only the exit is awaited.
    Killed,
}

/// The signal [`on_terminate`] last received, or `0`, read by [`run`]'s poll
/// loop.
///
/// Process-global because a signal disposition is process-global, and latched
/// because the launch on which the child finally exits still has to report it.
/// The *number* rather than a flag, because the ending is reported onward and a
/// launcher that re-raises SIGTERM for a SIGHUP has told its parent the wrong
/// thing.
///
/// **A latch that outlives its launch is a loaded gun**, and this one is scoped
/// to exactly one launch at both ends. [`run`] clears it immediately before
/// spawning, so a signal that arrived while no child existed can never be
/// spent on a fresh child that has not signalled and has done nothing wrong;
/// [`take_interrupt`] lets a launcher consume it between launches, which is
/// where such a signal actually belongs. Without the clear, a driver signalled
/// in the gap between two iterations starts the next session and SIGTERMs it on
/// its first poll.
static INTERRUPTED_BY: AtomicI32 = AtomicI32::new(0);

/// Which signal, if any, was sent to this process outside a launch — clearing
/// the latch.
///
/// For a launcher that runs launches in a loop: [`run`] reports a signal that
/// arrives *during* a launch as [`End::Interrupted`], but one arriving between
/// two launches has no launch to be reported against, and `run` deliberately
/// discards it rather than spending it on the next child. Call this at the top
/// of the loop to honour it instead.
///
/// It answers `None` before this process's first [`run`], because nothing has
/// installed a handler yet and the signal took its default disposition.
#[must_use]
pub fn take_interrupt() -> Option<i32> {
    match INTERRUPTED_BY.swap(0, Ordering::Relaxed) {
        0 => None,
        signal => Some(signal),
    }
}

/// Die of the signal that ended this launcher, so **its** parent sees the
/// conventional `128 + N` in the wait status.
///
/// A process that catches SIGTERM, cleans up and exits 0 has told a systemd
/// unit, a `timeout(1)` or a shell `wait` that it finished its work. An exit
/// *code* cannot express "was signalled" at all — only a wait status can, and
/// the only way to produce one is to actually die of the signal. So the
/// disposition this crate installed is put back to the default, the signal is
/// unblocked in case it is still masked from the handler that ran, and it is
/// raised.
///
/// **This crate owns the call because this crate installed the handler.** A
/// consumer undoing it would be reaching for a disposition it did not set and
/// cannot see, and would get it wrong for a signal `run` starts catching later.
/// What stays the consumer's is *whether* to re-raise, which is a statement
/// about that process's own exit status.
pub fn reraise(signal: i32) -> ! {
    // Buffered output is lost by a signal death the way it is lost by
    // `process::exit`, and the last diagnostic before a termination is the one
    // a reader most wants.
    let _ = std::io::stdout().flush();
    let _ = std::io::stderr().flush();

    // SAFETY: restoring the default disposition of a signal this crate set a
    // handler for, unblocking that one signal, and raising it against this
    // process. `sigset_t` is initialised by `sigemptyset` before use.
    unsafe {
        libc::signal(signal, libc::SIG_DFL);
        let mut unblock: libc::sigset_t = std::mem::zeroed();
        libc::sigemptyset(&mut unblock);
        libc::sigaddset(&mut unblock, signal);
        libc::sigprocmask(libc::SIG_UNBLOCK, &unblock, std::ptr::null_mut());
        libc::raise(signal);
    }

    // Unreachable for any signal whose default action terminates, which is
    // every signal a launcher is interrupted by. If a caller passes one that
    // does not — SIGCHLD, SIGURG — saying so in the exit code is still better
    // than falling through to whatever the caller does after an infallible
    // call it believed diverged.
    std::process::exit(128 + signal)
}

/// A single store is the *only* work done here, because it is the only work
/// that is async-signal-safe. Signalling and reaping the child happen one poll
/// tick later, on [`run`]'s ordinary stack.
extern "C" fn on_terminate(signal: libc::c_int) {
    INTERRUPTED_BY.store(signal, Ordering::Relaxed);
}

/// Catch the signals that cancel this launch, so a launcher can forward
/// termination to its child and reap it rather than orphan it.
///
/// SIGTERM and SIGHUP always; SIGINT for a launch with **no terminal** or a
/// [transparent](Launch::transparent) launcher. A launch with a terminal has
/// handed it to the child, so a typed Ctrl-C reaches the child's group and not
/// the launcher's. What any other launcher does about a SIGINT it receives
/// anyway is its own policy, not this crate's. A launch with no terminal has no
/// other route by which an interrupt could reach its child, and a transparent
/// launcher stands in for its child, so an interrupt sent to it is one sent to
/// the child.
///
/// **A disposition the launcher ignores is left ignored.** Ignoring a signal is
/// a statement the launcher made, a caller's ignored SIGINT for one,
/// and a handler installed over it would turn a signal the launcher chose to
/// survive into a cancellation. Checked on every launch: once this crate
/// installs its handler the disposition is no longer an ignore, so a repeat
/// call finds its own handler and re-installs it, which is idempotent.
///
/// Installed by [`run`] rather than exported, because [`End::Interrupted`] is a
/// promise this crate makes and a caller cannot be relied on to have enabled
/// it.
fn install_termination_handler(catch_interrupt: bool) {
    // Through the function *pointer* rather than casting the function item
    // straight to an integer, which rustc warns about: a function item is
    // zero-sized and the cast reads as a value conversion rather than the
    // address-taking it is.
    let handler = on_terminate as extern "C" fn(libc::c_int) as usize;
    let cancelling: &[libc::c_int] = if catch_interrupt {
        &[libc::SIGTERM, libc::SIGHUP, libc::SIGINT]
    } else {
        &[libc::SIGTERM, libc::SIGHUP]
    };
    for &signal in cancelling {
        if disposition(signal) != libc::SIG_IGN {
            // SAFETY: `signal(2)` with a handler that performs one relaxed
            // atomic store.
            unsafe { libc::signal(signal, handler as libc::sighandler_t) };
        }
    }
}

/// The current disposition of `signal`, read without changing it.
fn disposition(signal: libc::c_int) -> libc::sighandler_t {
    // SAFETY: `sigaction(2)` with a null new action only reads the current one
    // into an initialised struct.
    unsafe {
        let mut current: libc::sigaction = std::mem::zeroed();
        libc::sigaction(signal, std::ptr::null(), &mut current);
        current.sa_sigaction
    }
}

/// Whether this process inherited SIGCHLD ignored: `0` not yet read, `1` yes,
/// `2` no.
///
/// Latched on the first launch, because that launch changes the disposition
/// and every later launch would otherwise read back its own repair.
static SIGCHLD_IGNORED_AT_ENTRY: AtomicU8 = AtomicU8::new(0);

/// **An ignored SIGCHLD makes the kernel reap children unwatched.** The child's
/// exit then leaves no zombie. So there is nothing for `waitid` to observe, and
/// nothing reserves the group's ID while the rest of the group is killed. A
/// launcher that inherited the ignore therefore gets the default back for
/// itself, with no flags, and **never a handler**: a handler would turn every
/// child's exit into EINTR on whatever the launcher is doing.
///
/// Answers whether the entry disposition was the ignore, so the spawn can hand
/// the child the disposition it would have inherited. The repair is the
/// launcher's, not the child's.
pub(crate) fn restore_child_watching() -> bool {
    let ignored = match SIGCHLD_IGNORED_AT_ENTRY.load(Ordering::Relaxed) {
        0 => {
            let ignored = disposition(libc::SIGCHLD) == libc::SIG_IGN;
            SIGCHLD_IGNORED_AT_ENTRY.store(if ignored { 1 } else { 2 }, Ordering::Relaxed);
            ignored
        }
        latched => latched == 1,
    };
    if ignored {
        // SAFETY: `sigaction(2)` installing the default disposition with an
        // empty mask and no flags, which clears SA_NOCLDWAIT with it.
        unsafe {
            let mut default: libc::sigaction = std::mem::zeroed();
            default.sa_sigaction = libc::SIG_DFL;
            libc::sigemptyset(&mut default.sa_mask);
            libc::sigaction(libc::SIGCHLD, &default, std::ptr::null_mut());
        }
    }
    ignored
}

/// The signals the child is handed back at their **default** disposition.
///
/// **Only an *ignored* disposition survives `execve`.** POSIX resets a caught
/// handler to the default across an exec and leaves an ignore in place, and
/// `std::process::Command` restores exactly one thing on top of that — SIGPIPE,
/// which Rust ignores process-wide at start-up. So this list is not about
/// handlers, which take care of themselves. It is about a launcher that
/// *ignores* one of these for its own reasons and would otherwise hand the
/// ignore to its child, to that child's children, and to every wrapper in
/// between: a login shell or an `ssh` hop that inherits an ignored SIGINT keeps
/// ignoring it *and propagates it onward*, so an interactive session under one
/// cannot be interrupted at all and nothing in it can tell why.
///
/// A launcher that wanted a child to inherit an ignore has to say so some other
/// way. That is the right default for this crate, whose child owns a terminal:
/// a terminal-generated signal the human types must reach the process the
/// human is looking at.
const DEFAULT_DISPOSITION_IN_CHILD: [libc::c_int; 7] = [
    libc::SIGINT,
    libc::SIGQUIT,
    libc::SIGTERM,
    libc::SIGHUP,
    libc::SIGTSTP,
    libc::SIGTTIN,
    libc::SIGTTOU,
];

/// The signal state a transparent launcher inherited, to hand its child in
/// place of [`DEFAULT_DISPOSITION_IN_CHILD`]: which signals were ignored and
/// which were blocked when the launcher started.
///
/// **Recording it is the caller's**, because only the caller can do it early
/// enough: a Rust runtime changes SIGPIPE's disposition before `main`, so the
/// launcher has to have read its state before that. This crate only hands on
/// what it is given.
///
/// In the child every signal is set to the ignore or the default this records,
/// and then the mask is set to this one. The mask comes last, so a signal the
/// launcher had blocked across the spawn is delivered to the child, if at all,
/// only under its entry disposition. SIGKILL and SIGSTOP cannot be changed, and
/// a number the system does not have is left alone.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct EntrySignals {
    /// Bit `N - 1` for signal `N`, for every signal number 1 to 64.
    ignored: u64,
    blocked: u64,
}

/// The signal numbers [`EntrySignals`] can hold: 1 to 31 on macOS, to 64 on
/// Linux.
const SIGNAL_NUMBERS: std::ops::RangeInclusive<libc::c_int> = 1..=64;

impl EntrySignals {
    /// The entry state from the signals that were ignored and those that were
    /// blocked. A number outside 1 to 64 is no signal, and is left out.
    #[must_use]
    pub fn new(
        ignored: impl IntoIterator<Item = libc::c_int>,
        blocked: impl IntoIterator<Item = libc::c_int>,
    ) -> Self {
        let bits = |signals: &mut dyn Iterator<Item = libc::c_int>| {
            signals
                .filter(|signal| SIGNAL_NUMBERS.contains(signal))
                .fold(0_u64, |bits, signal| bits | 1 << (signal - 1))
        };
        Self {
            ignored: bits(&mut ignored.into_iter()),
            blocked: bits(&mut blocked.into_iter()),
        }
    }

    /// Set every changeable signal's disposition, then the mask. **Runs
    /// between `fork` and `exec`**, so it makes only `sigaction`,
    /// `sigemptyset`, `sigaddset` and `pthread_sigmask` calls, all
    /// async-signal-safe, and allocates nothing. A disposition the system
    /// refuses to change belongs to a number it does not have, or reserves for
    /// itself, and is left as it is.
    fn reinstate(&self) -> std::io::Result<()> {
        // SAFETY: each action and the set are initialised before use, and the
        // handler is SIG_IGN or SIG_DFL.
        unsafe {
            let mut mask: libc::sigset_t = std::mem::zeroed();
            libc::sigemptyset(&mut mask);
            for signal in SIGNAL_NUMBERS {
                let bit = 1_u64 << (signal - 1);
                if self.blocked & bit != 0 {
                    libc::sigaddset(&mut mask, signal);
                }
                if signal == libc::SIGKILL || signal == libc::SIGSTOP {
                    continue;
                }
                let mut action: libc::sigaction = std::mem::zeroed();
                action.sa_sigaction = if self.ignored & bit != 0 {
                    libc::SIG_IGN
                } else {
                    libc::SIG_DFL
                };
                libc::sigemptyset(&mut action.sa_mask);
                libc::sigaction(signal, &action, std::ptr::null_mut());
            }
            match libc::pthread_sigmask(libc::SIG_SETMASK, &mask, std::ptr::null_mut()) {
                0 => Ok(()),
                error => Err(std::io::Error::from_raw_os_error(error)),
            }
        }
    }
}

/// The launcher's controlling terminal, open for as long as a launch needs to
/// hand it back and forth.
struct Terminal(OwnedFd);

impl Terminal {
    /// `/dev/tty` rather than stdin, and the difference is the gate.
    ///
    /// `/dev/tty` *is* the controlling terminal by definition, so a launcher
    /// whose stdin was redirected still hands over the right device — and a
    /// launcher that has no controlling terminal at all (a test runner, a CI
    /// job, a daemon) simply fails to open it and gets no job control. There is
    /// no flag to set and nothing for a caller to configure wrongly.
    fn open() -> Option<Self> {
        // SAFETY: `open(2)` against a constant NUL-terminated path. The
        // returned descriptor is owned from here on and closed by `OwnedFd`.
        let fd = unsafe {
            libc::open(
                c"/dev/tty".as_ptr(),
                libc::O_RDWR | libc::O_NOCTTY | libc::O_CLOEXEC,
            )
        };
        if fd < 0 {
            return None;
        }
        // SAFETY: a fresh descriptor this process just opened and has not
        // handed to anything else.
        Some(Self(unsafe { OwnedFd::from_raw_fd(fd) }))
    }

    fn fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }

    /// Which process group currently owns the terminal, or `-1`.
    fn foreground(&self) -> libc::pid_t {
        // SAFETY: `tcgetpgrp(3)` on a descriptor this struct owns.
        unsafe { libc::tcgetpgrp(self.fd()) }
    }

    /// Make `pgid` the terminal's foreground process group.
    ///
    /// **`tcsetpgrp` from a group that is not already the foreground one raises
    /// SIGTTOU at the caller**, whose default action stops it — so a launcher
    /// reclaiming the terminal from its child would stop itself in the act of
    /// taking it back. Ignoring SIGTTOU across the call and restoring the
    /// previous disposition afterwards is the standard job-control dance, and
    /// it is why this is a method rather than a bare call at three sites.
    fn hand_to(&self, pgid: libc::pid_t) -> Result<(), LaunchError> {
        self.with_ttou_ignored("terminal handback", || {
            // SAFETY: this lease owns the descriptor and the admitted holder policy.
            unsafe { libc::tcsetpgrp(self.fd(), pgid) }
        })
    }

    fn with_ttou_ignored(
        &self,
        operation: &'static str,
        action: impl FnOnce() -> libc::c_int,
    ) -> Result<(), LaunchError> {
        // SAFETY: exclusive launcher disposition authority; restore on every path.
        let previous = unsafe { libc::signal(libc::SIGTTOU, libc::SIG_IGN) };
        if previous == libc::SIG_ERR {
            return Err(LaunchError::new(format!(
                "cannot suppress SIGTTOU for {operation}: {}",
                std::io::Error::last_os_error()
            )));
        }
        let mut reports = None;
        recovery_report(&mut reports, operation, os_result(action()));
        // SAFETY: restore the exact disposition returned above, even after action failure.
        let restored = unsafe { libc::signal(libc::SIGTTOU, previous) };
        recovery_report(
            &mut reports,
            "temporary SIGTTOU restoration",
            if restored == libc::SIG_ERR {
                Err(std::io::Error::last_os_error())
            } else {
                Ok(())
            },
        );
        reports.map_or(Ok(()), Err)
    }

    /// The terminal's current attributes, or `None` if they cannot be read.
    fn attributes(&self) -> Option<libc::termios> {
        // SAFETY: `tcgetattr(3)` filling an initialised struct from a
        // descriptor this struct owns.
        unsafe {
            let mut attributes: libc::termios = std::mem::zeroed();
            (libc::tcgetattr(self.fd(), &mut attributes) == 0).then_some(attributes)
        }
    }

    /// Put back attributes saved earlier: cooked mode and echo, for a child
    /// that set the terminal raw and was killed before it could undo that.
    ///
    /// Made with SIGTTOU ignored for the same reason as [`Terminal::hand_to`].
    /// It is called only from the foreground, but the guard costs nothing.
    fn restore(&self, attributes: &libc::termios) -> Result<(), LaunchError> {
        self.with_ttou_ignored("terminal mode restoration", || {
            // SAFETY: saved tcgetattr output and this lease's live descriptor.
            unsafe { libc::tcsetattr(self.fd(), libc::TCSANOW, attributes) }
        })
    }
}

fn os_result(result: libc::c_int) -> std::io::Result<()> {
    if result == -1 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(())
    }
}

// Reports are secondary to the spawn/watch cause. Continue required recovery
// attempts after a failed report, retaining each operation's diagnostic.
fn recovery_report(
    reports: &mut Option<LaunchError>,
    operation: &'static str,
    result: std::io::Result<()>,
) {
    #[cfg(test)]
    let result = terminal_tests::reported(operation, result);
    if let Err(error) = result {
        append_recovery(
            reports,
            LaunchError::new(format!("{operation} failed: {error}")),
        );
    }
}

fn append_recovery(reports: &mut Option<LaunchError>, error: LaunchError) {
    *reports = Some(match reports.take() {
        Some(primary) => primary.with_diagnostic(error),
        None => error,
    });
}

/// Private fixed-size attempted-group record, prepared only for an owned lease.
/// No descriptor survives exec; no group is signalled using this record.
struct AttemptGroup {
    reader: File,
    writer: OwnedFd,
}

impl AttemptGroup {
    fn prepare() -> Result<Self, LaunchError> {
        let make = || -> std::io::Result<Self> {
            let mut fds = [-1; 2];
            // SAFETY: valid output storage. Ownership is established immediately.
            os_result(unsafe { libc::pipe(fds.as_mut_ptr()) })?;
            let ends = unsafe { [OwnedFd::from_raw_fd(fds[0]), OwnedFd::from_raw_fd(fds[1])] };
            // Keep the private ends above stdio even when a caller closed 0/1/2.
            let duplicate = |fd| -> std::io::Result<OwnedFd> {
                let copy = unsafe { libc::fcntl(fd, libc::F_DUPFD_CLOEXEC, 3) };
                if copy == -1 {
                    return Err(std::io::Error::last_os_error());
                }
                Ok(unsafe { OwnedFd::from_raw_fd(copy) })
            };
            let reader = duplicate(ends[0].as_raw_fd())?;
            let writer = duplicate(ends[1].as_raw_fd())?;
            // A missing/partial record must never hang recovery, even if some
            // unrelated fork retained an endpoint. Only one four-byte write exists.
            let flags = unsafe { libc::fcntl(reader.as_raw_fd(), libc::F_GETFL) };
            if flags == -1 {
                return Err(std::io::Error::last_os_error());
            }
            os_result(unsafe {
                libc::fcntl(reader.as_raw_fd(), libc::F_SETFL, flags | libc::O_NONBLOCK)
            })?;
            Ok(Self {
                reader: File::from(reader),
                writer,
            })
        };
        make().map_err(|error| {
            LaunchError::new(format!(
                "cannot prepare terminal attempted-group pipe: {error}"
            ))
        })
    }

    fn receive(mut self) -> Result<libc::pid_t, LaunchError> {
        drop(self.writer); // Parent's writer closes on both spawn outcomes.
        let mut record = [0u8; 5];
        let count = loop {
            match self.reader.read(&mut record) {
                Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
                other => break other,
            }
        }
        .map_err(|error| {
            LaunchError::new(format!(
                "terminal attempted-group record unavailable: {error}"
            ))
        })?;
        if count != 4 {
            return Err(LaunchError::new(
                "terminal attempted-group record missing or malformed; recovery unconfirmed",
            ));
        }
        let group = i32::from_ne_bytes([record[0], record[1], record[2], record[3]]);
        if group <= 0 {
            return Err(LaunchError::new(
                "terminal attempted-group record has invalid group; recovery unconfirmed",
            ));
        }
        Ok(group)
    }
}

/// Runs only in the restricted pre-exec child: fixed data and child-safe syscalls.
fn record_attempt(fd: RawFd, group: libc::pid_t) -> std::io::Result<()> {
    let record = group.to_ne_bytes();
    loop {
        let written = unsafe { libc::write(fd, record.as_ptr().cast(), record.len()) };
        if written == record.len() as isize {
            return Ok(());
        }
        if written == -1 {
            let error = std::io::Error::last_os_error();
            if error.kind() == std::io::ErrorKind::Interrupted {
                continue;
            }
            return Err(error);
        }
        return Err(std::io::Error::from_raw_os_error(libc::EIO));
    }
}

/// This process's own process group.
fn own_group() -> libc::pid_t {
    // SAFETY: `getpgrp(2)` takes no argument and cannot fail.
    unsafe { libc::getpgrp() }
}

/// One launch's share of the terminal: when it was held, what it looked like
/// then, and how it is given back.
///
/// **"Held the foreground" is the gate for everything on the way back.** A
/// launcher in the foreground lends the terminal to its child and owes the
/// human a terminal back. One that never held it during the launch lent
/// nothing, so it takes nothing and restores nothing: anything else would
/// steal from whichever job does hold it.
struct Lease {
    terminal: Option<Terminal>,
    child: libc::pid_t,
    /// Whether the launch has held the foreground at any moment so far.
    held: bool,
    /// The attributes at the moment the launch first held it, when they
    /// could be read.
    saved: Option<libc::termios>,
    reports: Option<LaunchError>,
}

impl Lease {
    /// Before the spawn: whether this launcher holds the foreground now, and
    /// if so the descriptor the child hands itself the terminal through.
    fn hold_before_spawn(&mut self) -> Result<Option<RawFd>, LaunchError> {
        let Some(terminal) = self.terminal.as_ref() else {
            return Ok(None);
        };
        let holder = terminal.foreground();
        if holder == -1 {
            return Err(LaunchError::new(format!(
                "cannot query terminal foreground before spawn: {}",
                std::io::Error::last_os_error()
            )));
        }
        if holder != own_group() {
            return Ok(None);
        }
        self.saved = Some(terminal.attributes().ok_or_else(|| {
            LaunchError::new(format!(
                "cannot save terminal modes before spawn: {}",
                std::io::Error::last_os_error()
            ))
        })?);
        self.held = true;
        Ok(Some(terminal.fd()))
    }

    /// Called on every poll tick, and once before the spawn. Re-checked each
    /// time, so a launcher started in the background and later brought
    /// forward (`launcher &`, then `fg`) hands the terminal on to the job that is
    /// actually running under it, and only then saves what it will restore.
    fn lend(&mut self) {
        let Some(terminal) = &self.terminal else {
            return;
        };
        if self.reports.is_some() {
            return;
        }
        let holder = terminal.foreground();
        if holder == -1 {
            append_recovery(
                &mut self.reports,
                LaunchError::new(format!(
                    "terminal foreground query while lending failed: {}",
                    std::io::Error::last_os_error()
                )),
            );
            return;
        }
        if holder != own_group() {
            return;
        }
        if !self.held {
            let Some(saved) = terminal.attributes() else {
                append_recovery(
                    &mut self.reports,
                    LaunchError::new(format!(
                        "cannot save terminal modes before lending: {}",
                        std::io::Error::last_os_error()
                    )),
                );
                return;
            };
            self.held = true;
            self.saved = Some(saved);
        }
        if let Err(error) = terminal.hand_to(self.child) {
            append_recovery(&mut self.reports, error);
        }
    }

    /// Take the terminal back once the child is reaped, and restore it.
    ///
    /// From the child's group after an ordinary exit. That group is gone by
    /// now, but the terminal still names it until it is given to another.
    /// **After a death by signal, from whichever group then holds it**,
    /// unless that is the launcher's own or the session leader's. A child
    /// that is itself a supervisor and was killed can leave its own child's
    /// group holding the terminal, an orphan in the foreground reading the
    /// human's input, and only this rule takes it back. After an ordinary
    /// exit the rule stays narrow, because a group other than the child's that
    /// holds the terminal then took it on purpose. A job-control shell nested
    /// in this session is one example.
    ///
    /// The attributes go back whenever the launcher ends in the foreground,
    /// so a raw-mode child killed by the escalation still leaves cooked mode
    /// behind.
    fn reclaim(
        &mut self,
        status: Option<ExitStatus>,
        failed_spawn: bool,
    ) -> Result<(), LaunchError> {
        use std::os::unix::process::ExitStatusExt as _;
        let mut reports = self.reports.take();
        let Some(terminal) = self.terminal.as_ref().filter(|_| self.held) else {
            return reports.map_or(Ok(()), Err);
        };
        let holder = terminal.foreground();
        let query = if holder == -1 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(())
        };
        let mut query_report = None;
        recovery_report(&mut query_report, "terminal foreground query", query);
        if let Some(error) = query_report {
            append_recovery(&mut reports, error);
            return reports.map_or(Ok(()), Err);
        }
        let own = own_group();
        // A record is conditional holder evidence, never a reservation after
        // std's failed-child wait. The caller excludes competing transfers and
        // unrelated group-ID reuse from this observation through handback.
        let take = if !failed_spawn && status.and_then(|s| s.signal()).is_some() {
            let leader = unsafe { libc::getsid(0) };
            holder > 0 && holder != own && holder != leader
        } else {
            self.child > 0 && holder == self.child
        };
        if failed_spawn && self.child <= 0 {
            append_recovery(
                &mut reports,
                LaunchError::new("terminal attempt ownership missing; recovery unconfirmed"),
            );
        }
        if take {
            if let Err(error) = terminal.hand_to(own) {
                append_recovery(&mut reports, error);
            }
        } else if failed_spawn && holder != own {
            append_recovery(&mut reports, LaunchError::new(format!("terminal held by unrelated foreground group {holder}; preserved; recovery unconfirmed")));
        }
        let after = terminal.foreground();
        let query = if after == -1 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(())
        };
        recovery_report(&mut reports, "terminal foreground confirmation", query);
        if after == own {
            if let Some(saved) = &self.saved {
                if let Err(error) = terminal.restore(saved) {
                    append_recovery(&mut reports, error);
                }
            } else {
                append_recovery(
                    &mut reports,
                    LaunchError::new("terminal saved modes missing; recovery unconfirmed"),
                );
            }
        } else if take && after != -1 {
            append_recovery(
                &mut reports,
                LaunchError::new("terminal handback unconfirmed; saved modes not restored"),
            );
        }
        reports.map_or(Ok(()), Err)
    }
}

/// Spawn `launch`'s argv directly and supervise the child until it ends.
///
/// The child's environment is the launcher's, minus [`Launch::scrub`], plus
/// [`Launch::grant`], plus the optional channel path under its chosen variable.
/// Nothing else is added: no argument, no flag, no variable. A child that needs
/// one is given it by the caller, in the argv or the grant the caller built.
///
/// **The child is a job, not just a process.** It is put in a process group of
/// its own and — when this launcher owns a controlling terminal and is the
/// foreground group of it — handed that terminal, exactly as a shell does for a
/// foreground job. Two things follow, and both are the point. A terminal signal
/// the human types reaches *the child's* group rather than the launcher's, so
/// the launcher survives a Ctrl-C it never has to catch; and the escalation can
/// signal the whole group, so a grandchild the child spawned is reaped with it
/// rather than left running and attached to the terminal. The child's group is
/// *not* a new session: `setsid` leaves it with no controlling terminal, so
/// the handover fails silently at both ends — the return is ignored — and an
/// interactive child reads on unstopped while the launcher keeps the Ctrl-C.
///
/// The child's signal dispositions are the defaults, whatever the launcher's
/// are — see [`DEFAULT_DISPOSITION_IN_CHILD`] — unless the launcher is
/// [transparent](Launch::transparent), when they and the mask are its entry
/// state. The launcher's own are respected:
/// no handler goes over a signal it ignores, and an ignored SIGCHLD is repaired
/// for the launcher alone, so that its child is not reaped unwatched.
///
/// **The group ends with the launch, whatever ended the child.** Once the child
/// has exited (a stop is not an exit), its group is killed while the unreaped
/// child still reserves the group's ID. Then the child is reaped, the terminal
/// is taken back with the attributes saved at the handover, and the group is
/// confirmed gone. A group that is still present is [`Ended::group`].
///
/// Supervision polls three things, and they are the only three ways a launch
/// ends: the child exits, the channel appears, or the launcher itself is
/// signalled. **A child that finishes its work and never signals reaches none
/// of them** — an interactive one returns to its prompt instead of exiting, so
/// the launch *stalls* rather than ending. That is a real failure mode with no
/// cheap fix here: nothing this crate can observe distinguishes a child that
/// forgot to signal from one still working, so a second completion observable
/// would only trade a stall for a wrong kill. It is the caller's to close, at
/// the layer that instructs the child.
pub fn run(launch: Launch<'_>) -> Result<Ended, LaunchError> {
    run_observed(launch, &mut |_| {})
}

/// Parent-side evidence about one launched child, independent of its completion signal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LaunchEvent {
    /// Spawn reported success; exec is not confirmed. The child may already have exited.
    Started,
    /// A wait confirmed reap, even if supervision returns an error afterwards.
    Reaped,
}

/// Run a launch with synchronous, infallible parent-side notifications.
///
/// Started occurs exactly once after successful spawn; failed spawn emits no
/// events. Reaped occurs exactly once on confirmed reap, before terminal recovery.
/// Channel appearance alone or an unsuccessful wait is not reap.
/// The callback must return promptly and must not panic; observation failures
/// must be handled within it. No child acknowledgement or outcome override is
/// involved. All other behavior is the same as [`run`].
pub fn run_observed(
    launch: Launch<'_>,
    observer: &mut dyn FnMut(LaunchEvent),
) -> Result<Ended, LaunchError> {
    run_with_output(launch.into(), observer, None, None)
}

/// Run a noninteractive child in a new POSIX session, without a controlling
/// terminal or inherited input. Both output streams go to a caller-owned regular
/// file. Cancellation forwards the launcher's signal to the job and, after the
/// kill-grace, kills it: the child may itself be a supervisor, and needs that
/// long to end its own child. Remaining members of the child's process group
/// are killed when it exits, as in every launch. This isolates process control, not filesystem access or processes that
/// deliberately leave the child's process group.
pub fn run_noninteractive(
    launch: NoninteractiveLaunch<'_>,
    output: File,
) -> Result<Ended, LaunchError> {
    let regular = output
        .metadata()
        .map_err(|error| LaunchError::new(format!("cannot inspect the launch log: {error}")))?;
    if !regular.is_file() {
        return Err(LaunchError::new(
            "the noninteractive launch log must be a regular file",
        ));
    }
    run_with_output(
        Job {
            argv: launch.argv,
            completion: None,
            scrub: launch.scrub,
            grant: &[],
            transparent: None,
            cwd: launch.cwd,
            escalation: launch.escalation,
        },
        &mut |_| {},
        Some(output),
        None,
    )
}

/// Confine a noninteractive job with EOF stdin and inherited stdout/stderr.
/// Start a new POSIX session, close inherited descriptors above stderr on
/// exec, and clear the environment before applying Launch's explicit grants.
/// The channel is granted last. Observations have `run_observed`'s ordering;
/// cancellation kills the group immediately, so a nested supervisor finishes
/// within its enclosing supervisor's grace.
pub fn run_confined_observed(
    launch: Launch<'_>,
    policy: &crate::FilesystemGrants<'_>,
    observer: &mut dyn FnMut(LaunchEvent),
) -> Result<Ended, LaunchError> {
    let mut command = crate::confinement::command_with_grants(launch.argv, policy)?;
    command.env_clear();
    run_with_output(launch.into(), observer, None, Some(command))
}

fn run_with_output(
    launch: Job<'_>,
    observer: &mut dyn FnMut(LaunchEvent),
    output: Option<File>,
    confined_command: Option<Command>,
) -> Result<Ended, LaunchError> {
    let mode = match (&output, &confined_command) {
        (_, Some(_)) => Mode::Confined,
        (Some(_), None) => Mode::Noninteractive,
        (None, None) => Mode::Interactive,
    };
    let detached = mode != Mode::Interactive;
    // A detached child is in a session of its own and could never be handed
    // the terminal, so only an interactive launch opens it.
    let terminal = if detached { None } else { Terminal::open() };
    install_termination_handler(terminal.is_none() || launch.transparent.is_some());
    let transparent = launch.transparent.copied();
    let sigchld_ignored = restore_child_watching();
    let descriptor_limit = if detached { descriptor_limit()? } else { 3 };

    let mut command = confined_command.unwrap_or_else(|| {
        let mut command = Command::new(launch.argv.program());
        command.arg0(launch.argv.arg0()).args(launch.argv.args());
        command
    });
    if detached {
        command.stdin(Stdio::null());
    }
    if let Some(output) = output {
        let stderr = output.try_clone().map_err(|error| {
            LaunchError::new(format!("cannot duplicate the launch log: {error}"))
        })?;
        command.stdin(Stdio::null()).stdout(output).stderr(stderr);
    }
    if let Some(cwd) = launch.cwd {
        command.current_dir(cwd);
    }
    // Scrub first, grant second, and the order is load-bearing rather than
    // stylistic: a caller whose scrub list *contains* its own `channel_var` is
    // the expected shape, not a mistake — the list names the launch-control
    // variables a nested launcher must not inherit, and the channel variable is
    // the first of them. Granting before scrubbing would remove the path this
    // launch just published and leave the child unable to signal, which reads
    // as a session that hung. `tests/launch.rs` pins it. The channel is set
    // last, so no grant can name another path under its variable.
    for name in launch.scrub {
        command.env_remove(name);
    }
    for (name, value) in launch.grant {
        command.env(name, value);
    }
    if let Some((channel, name)) = launch.completion {
        command.env(name, channel.path());
    }

    // Hand the terminal over from *inside* the child as well as from the parent
    // below, which cannot be early enough on its own: the parent's handover
    // waits on `spawn` returning, nothing orders the child's first read after
    // that, and a read from a background group is what SIGTTIN stops. Only when
    // this launcher is the terminal's current owner — handing over a terminal
    // owned by somebody else's job is theft, not job control. Holding it now is
    // also when its attributes are saved, before a child can set it raw.
    let mut lease = Lease {
        terminal,
        child: 0,
        held: false,
        saved: None,
        reports: None,
    };
    let handover_fd = lease.hold_before_spawn()?;
    let attempt = handover_fd.map(|_| AttemptGroup::prepare()).transpose()?;
    #[cfg(test)]
    terminal_tests::prepared(&mut command, attempt.as_ref());
    let attempt_fd = attempt.as_ref().map(|pipe| pipe.writer.as_raw_fd());
    #[cfg(test)]
    let probe = terminal_tests::probe();

    // The group, through `std`'s own checked path rather than a `setpgid` of our
    // own: it runs it before the `pre_exec` callbacks below and reports a
    // failure as a failed spawn, which a raw call in the closure could only do
    // by hand.
    if !detached {
        command.process_group(0);
    }

    // SAFETY: the closure runs between `fork` and `exec`, so it may call only
    // async-signal-safe functions. `signal`, `getpid` and `tcsetpgrp` (an
    // `ioctl`) are all on POSIX's list, as is everything
    // `EntrySignals::reinstate` calls; nothing here allocates, locks, or
    // touches Rust runtime state. Child memory is not parent communication:
    // https://doc.rust-lang.org/std/os/unix/process/trait.CommandExt.html#tymethod.pre_exec
    // `std` itself resets only SIGPIPE across a
    // spawn and inherits the signal mask, so everything below is work nothing
    // else is doing.
    unsafe {
        command.pre_exec(move || {
            if detached {
                // Close on exec rather than close here: std's error-reporting
                // pipe must survive until exec succeeds (it is CLOEXEC too).
                // fcntl is async-signal-safe; all allocation happened in parent.
                for fd in 3..descriptor_limit {
                    if libc::fcntl(fd, libc::F_SETFD, libc::FD_CLOEXEC) == -1 {
                        let error = std::io::Error::last_os_error();
                        if error.raw_os_error() != Some(libc::EBADF) {
                            return Err(error);
                        }
                    }
                }
            }
            // setsid must precede becoming a process-group leader. It creates
            // both the session and the group, so setpgid is omitted in this mode.
            if detached && libc::setsid() == -1 {
                return Err(std::io::Error::last_os_error());
            }
            // Until the `tcsetpgrp` below returns, this process is a background
            // group touching the terminal, which is precisely what SIGTTOU is
            // raised for. The loop that follows puts the disposition back.
            let previous = libc::signal(libc::SIGTTOU, libc::SIG_IGN);
            if previous == libc::SIG_ERR {
                return Err(std::io::Error::last_os_error());
            }
            #[cfg(test)]
            terminal_tests::before_handoff(probe)?;
            if let (Some(fd), Some(record_fd)) = (handover_fd, attempt_fd) {
                #[cfg(test)]
                let record_fd = terminal_tests::record_fd(probe, record_fd);
                let group = libc::getpid();
                #[cfg(test)]
                let group = terminal_tests::record_group(probe, group);
                record_attempt(record_fd, group)?;
                #[cfg(test)]
                let fd = terminal_tests::handoff_fd(probe, fd);
                let handoff = loop {
                    let result = libc::tcsetpgrp(fd, libc::getpid());
                    if result == 0 {
                        break Ok(());
                    }
                    let error = std::io::Error::last_os_error();
                    if error.kind() != std::io::ErrorKind::Interrupted {
                        break Err(error);
                    }
                };
                let restored = libc::signal(libc::SIGTTOU, previous);
                handoff?;
                if restored == libc::SIG_ERR {
                    return Err(std::io::Error::last_os_error());
                }
            }
            #[cfg(test)]
            terminal_tests::after_handoff(probe, handover_fd)?;
            // A transparent launcher's child gets its launcher's entry state
            // whole, SIGTTOU's and SIGCHLD's included, and the mask last.
            if let Some(entry) = &transparent {
                return entry.reinstate();
            }
            for signal in DEFAULT_DISPOSITION_IN_CHILD {
                libc::signal(signal, libc::SIG_DFL);
            }
            // The launcher repaired an inherited ignored SIGCHLD for itself
            // alone; the child gets the disposition it would have inherited.
            if sigchld_ignored {
                libc::signal(libc::SIGCHLD, libc::SIG_IGN);
            }
            Ok(())
        });
    }

    // Clear the latch *before* the spawn, never after: see `INTERRUPTED_BY`. A
    // signal that arrived while no child existed is the launcher's to handle
    // through `take_interrupt`, and is not evidence about the child below.
    INTERRUPTED_BY.store(0, Ordering::Relaxed);

    // Rust 1.99 Unix spawn waits on ordinary exec errors, but EOF can report
    // success after pre-exec signal death. Keep actual supervision on Ok:
    // https://github.com/rust-lang/rust/blob/1.99.0/library/std/src/sys/process/unix/unix.rs
    let spawned = command.spawn();
    #[cfg(test)]
    terminal_tests::after_spawn();
    let attempted = attempt.map(AttemptGroup::receive).transpose();
    let child = match spawned {
        Ok(child) => {
            if let Err(error) = attempted {
                append_recovery(&mut lease.reports, error);
            }
            child
        }
        Err(cause) => {
            let mut error = LaunchError::new(format!(
                "cannot spawn {:?}: {cause}; check that the program exists and is executable",
                launch.argv.program()
            ))
            .with_errno(cause.raw_os_error())
            .with_consumed_signal(take_interrupt());
            match attempted {
                Ok(Some(group)) => lease.child = group,
                Ok(None) => {}
                Err(report) => error = error.with_diagnostic(report),
            }
            if let Err(report) = lease.reclaim(None, true) {
                error = error.with_diagnostic(report);
            }
            return Err(error.with_consumed_signal(take_interrupt()));
        }
    };

    observer(LaunchEvent::Started);

    // The parent's half of the same `setpgid` — insurance, not a race. The
    // `pre_exec` above takes `std` off `posix_spawn` onto fork-and-exec, and
    // successful spawn ordinarily follows exec; signal death before exec can also
    // report success. On the ordinary path this call is
    // measured to fail EACCES. Kept: that ordering is undocumented, not a rule.
    let pgid = child.id() as libc::pid_t;
    // SAFETY: `setpgid(2)` naming this process's own child.
    if !detached {
        unsafe { libc::setpgid(pgid, pgid) };
    }

    lease.child = pgid;
    let lease = std::cell::RefCell::new(lease);

    supervise_reported(
        child,
        launch.completion.map(|(channel, _)| channel),
        launch.escalation,
        mode,
        observer,
        || lease.borrow_mut().lend(),
        |status| lease.borrow_mut().reclaim(status, false),
    )
}

/// Include existing descriptors even if the caller lowered its soft limit
/// after opening them. The remaining range covers descriptors std opens while
/// preparing the child. Noninteractive launches must not run concurrently with
/// a thread changing the process descriptor limit or signal dispositions.
fn descriptor_limit() -> Result<RawFd, LaunchError> {
    // SAFETY: getrlimit writes the initialized rlimit structure only.
    let mut limit: libc::rlimit = unsafe { std::mem::zeroed() };
    if unsafe { libc::getrlimit(libc::RLIMIT_NOFILE, &mut limit) } != 0 {
        return Err(LaunchError::new(format!(
            "cannot inspect descriptor limit: {}",
            std::io::Error::last_os_error()
        )));
    }
    let mut maximum = i32::try_from(limit.rlim_cur).map_err(|_| {
        LaunchError::new(
            "cannot isolate an unbounded descriptor table; set a finite open-file limit",
        )
    })?;
    let directory = if cfg!(target_os = "linux") {
        "/proc/self/fd"
    } else {
        "/dev/fd"
    };
    for entry in std::fs::read_dir(directory).map_err(|error| {
        LaunchError::new(format!("cannot inspect inherited descriptors: {error}"))
    })? {
        let entry = entry.map_err(|error| LaunchError::new(error.to_string()))?;
        if let Some(fd) = entry
            .file_name()
            .to_str()
            .and_then(|name| name.parse::<i32>().ok())
        {
            maximum = maximum.max(fd.saturating_add(1));
        }
    }
    Ok(maximum)
}

/// How long the runner waits between the two SIGKILLs it sends what remains of
/// the child's group.
///
/// **Two kills, because one is measured to miss.** On macOS a member that is
/// forking while the group's SIGKILL lands can leave a new process that the
/// kill never reached. In a tight fork loop that happened in about two runs in
/// three; Linux restarts the fork instead. The second kill, sent while the
/// unreaped child still reserves the group's ID, reaches that process. The
/// pause only has to outlast one fork.
const SECOND_KILL_PAUSE: Duration = Duration::from_millis(20);

/// How long, after the reap, the runner waits for the system to answer that
/// the child's group is gone.
///
/// Every member has been sent SIGKILL twice by then, so the wait only covers
/// what follows a kill: members dying, and their parent or `init` reaping
/// them. A group of zombies still answers. A member still present after this
/// bound is reported, not waited for.
const GROUP_CONFIRMATION: Duration = Duration::from_secs(1);

/// How often the supervisor checks for the child's exit after the
/// escalation's SIGKILL. The child is dying, not deciding, so this is much
/// shorter than [`POLL_INTERVAL`].
const KILLED_POLL_INTERVAL: Duration = Duration::from_millis(10);

/// Which of the three launch functions this is, as far as supervision cares:
/// how a cancellation reaches the child.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Mode {
    Interactive,
    Noninteractive,
    Confined,
}

/// The launched child as supervision sees it: a process that can be watched,
/// signalled and reaped, and the group it leads.
///
/// A private seam, so tests can force wait errors and trace the order of the
/// end without faking launch events.
trait Process {
    /// Whether the child has exited, **leaving it unreaped**. A stop is not an
    /// exit.
    fn exited(&mut self) -> std::io::Result<bool>;
    /// Reap the child, blocking, and answer its status.
    fn reap(&mut self) -> std::io::Result<ExitStatus>;
    /// Signal the whole group, then the child itself.
    fn signal(&mut self, signal: i32);
    /// SIGKILL what remains of the group, twice with a pause, while the
    /// unreaped child still reserves its ID.
    fn kill_group(&mut self);
    /// After the reap, and only querying: whether the group is gone.
    fn confirm_gone(&mut self) -> Group;
}

impl Process for Child {
    fn exited(&mut self) -> std::io::Result<bool> {
        // WNOWAIT observes the exit without releasing the child's PID, which
        // is also its group's ID. The rest of the group is killed before the
        // reap, so the ID cannot have been reused by the time the signal lands.
        // SAFETY: initialized siginfo, this process's own child, no reap.
        let mut info: libc::siginfo_t = unsafe { std::mem::zeroed() };
        let result = unsafe {
            libc::waitid(
                libc::P_PID,
                self.id(),
                &mut info,
                libc::WEXITED | libc::WNOHANG | libc::WNOWAIT,
            )
        };
        if result != 0 {
            return Err(std::io::Error::last_os_error());
        }
        // SAFETY: `waitid` filled `info`; `si_pid` is zero when nothing was
        // reported.
        if unsafe { info.si_pid() } == 0 {
            return Ok(false);
        }
        // macOS reports a stopped child here despite WEXITED alone, and goes on
        // reporting it on every poll until it is continued. A stopped child is
        // not an exit: it must be neither reaped nor have its group killed.
        Ok(matches!(
            info.si_code,
            libc::CLD_EXITED | libc::CLD_KILLED | libc::CLD_DUMPED
        ))
    }

    fn reap(&mut self) -> std::io::Result<ExitStatus> {
        Child::wait(self)
    }

    fn signal(&mut self, signal: i32) {
        kill(self.id() as libc::pid_t, signal);
    }

    fn kill_group(&mut self) {
        let pgid = self.id() as libc::pid_t;
        // SAFETY: `kill(2)` on the group the unreaped child still leads. A
        // failure is ignored: ESRCH means nothing is left, and EPERM a group
        // holding only zombies.
        unsafe { libc::kill(-pgid, libc::SIGKILL) };
        std::thread::sleep(SECOND_KILL_PAUSE);
        // SAFETY: as above; the child is still unreaped.
        unsafe { libc::kill(-pgid, libc::SIGKILL) };
    }

    fn confirm_gone(&mut self) -> Group {
        confirm_gone(self.id() as libc::pid_t)
    }
}

/// Query, without signalling, until the system answers that `pgid` names no
/// process group, for at most [`GROUP_CONFIRMATION`].
///
/// **Only ESRCH confirms.** EPERM answers for a member the launcher cannot
/// signal, and macOS also gives it for a group of zombies, so it is "present"
/// like a success is. A query that hits a reused ID after the reap reads as
/// present too, which fails safe.
fn confirm_gone(pgid: libc::pid_t) -> Group {
    let deadline = Instant::now() + GROUP_CONFIRMATION;
    loop {
        // SAFETY: `kill(2)` with signal 0, the existence probe, which sends
        // nothing.
        if unsafe { libc::kill(-pgid, 0) } == -1
            && std::io::Error::last_os_error().raw_os_error() == Some(libc::ESRCH)
        {
            return Group::Gone;
        }
        if Instant::now() >= deadline {
            return Group::Present { pgid };
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}

/// What watching produced: a reaped child, and why it ended.
struct Watched {
    status: ExitStatus,
    interrupted: Option<i32>,
    escalated: bool,
    elapsed: Duration,
}

/// A supervision that failed, with the child's status if it was reaped anyway.
struct Failed {
    error: LaunchError,
    status: Option<ExitStatus>,
}

/// Watch the child to its end, then end its group, take back the terminal,
/// and confirm the group gone. The order is the contract:
///
/// 1. observe the exit, never a stop, without reaping;
/// 2. kill what remains of the group, twice, while the zombie child still
///    reserves its ID;
/// 3. reap, which is when [`LaunchEvent::Reaped`] is emitted;
/// 4. take the terminal back and restore it;
/// 5. confirm, querying only, that the group is gone.
///
/// The child's status is fixed at step 1, so nothing after it can change what
/// the launch reports about the child. Step 5 can only add whether anything
/// survived it.
#[cfg(test)]
fn supervise(
    child: impl Process,
    channel: Option<&Channel>,
    escalation: Escalation,
    mode: Mode,
    observer: &mut dyn FnMut(LaunchEvent),
    lend: impl FnMut(),
    reclaim: impl FnOnce(Option<ExitStatus>),
) -> Result<Ended, LaunchError> {
    supervise_reported(child, channel, escalation, mode, observer, lend, |status| {
        reclaim(status);
        Ok(())
    })
}

// Keep the original watch outcome and cancellation when checked recovery fails.
fn supervise_reported(
    mut child: impl Process,
    channel: Option<&Channel>,
    escalation: Escalation,
    mode: Mode,
    observer: &mut dyn FnMut(LaunchEvent),
    mut lend: impl FnMut(),
    reclaim: impl FnOnce(Option<ExitStatus>) -> Result<(), LaunchError>,
) -> Result<Ended, LaunchError> {
    match watch(&mut child, channel, escalation, mode, observer, &mut lend) {
        Ok(Watched {
            status,
            interrupted,
            escalated,
            elapsed,
        }) => {
            reclaim(Some(status)).map_err(|error| error.with_consumed_signal(interrupted))?;
            let group = child.confirm_gone();
            Ok(Ended {
                end: match (interrupted, escalated) {
                    (Some(signal), _) => End::Interrupted { signal },
                    (None, true) => End::Escalated,
                    (None, false) => End::Exited,
                },
                status,
                elapsed,
                signalled: channel.is_some_and(Channel::appeared),
                group,
            })
        }
        Err(Failed { error, status }) => {
            let recovery = reclaim(status);
            Err(match recovery {
                Ok(()) => error,
                Err(recovery) => error.with_diagnostic(recovery),
            })
        }
    }
}

fn watch(
    child: &mut impl Process,
    channel: Option<&Channel>,
    escalation: Escalation,
    mode: Mode,
    observer: &mut dyn FnMut(LaunchEvent),
    lend: &mut dyn FnMut(),
) -> Result<Watched, Failed> {
    let started = Instant::now();
    let mut watch = Watch::Running;
    let mut interrupted: Option<i32> = None;
    let mut escalated = false;

    loop {
        lend();

        // Check cancellation before accepting even an already-exited child.
        // In nested launches the outer supervisor may have only a short grace
        // left, which is why a confined child is killed at once.
        if interrupted.is_none() {
            if let Some(signal) = take_interrupt() {
                interrupted = Some(signal);
                if mode == Mode::Confined {
                    child.signal(libc::SIGKILL);
                    watch = Watch::Killed;
                } else {
                    child.signal(signal);
                    if matches!(watch, Watch::Running | Watch::Signalled(_)) {
                        watch = Watch::Terminated(Instant::now());
                    }
                }
            }
        }

        match child.exited() {
            Ok(true) => break,
            Ok(false) => {}
            Err(error) => {
                // The child's state is now unknown, and returning here would
                // leave an interactive one holding the terminal with nothing
                // left to reap it. Ending it is the last thing this launch can
                // still do correctly, so it does that before reporting.
                child.signal(libc::SIGKILL);
                let reaped = child.reap();
                let at_reap = take_interrupt();
                let interrupted = interrupted.or(at_reap);
                let (status, report) = match reaped {
                    Ok(status) => (Some(status), "reaped".to_owned()),
                    Err(error) => (
                        None,
                        format!("could not be reaped — check for an orphaned process: {error}"),
                    ),
                };
                if status.is_some() {
                    observer(LaunchEvent::Reaped);
                }
                return Err(Failed {
                    error: LaunchError::new(format!(
                        "cannot wait on the launched child: {error}; it has been sent SIGKILL and {}",
                        report
                    )).with_consumed_signal(interrupted),
                    status,
                });
            }
        }

        watch = match watch {
            Watch::Running if channel.is_some_and(Channel::appeared) => {
                Watch::Signalled(Instant::now())
            }
            Watch::Signalled(at) if at.elapsed() >= escalation.grace => {
                // `escalated` is latched *here*, where the escalation actually
                // runs, and not where the channel appeared. `End` would
                // otherwise be telling the caller only what `signalled` already
                // tells it, while claiming something stronger: that this launch
                // had to be ended. A child that signals and then exits inside
                // its own grace was never touched, and says so.
                escalated = true;
                child.signal(libc::SIGTERM);
                Watch::Terminated(Instant::now())
            }
            Watch::Terminated(at) if at.elapsed() >= escalation.kill_grace => {
                child.signal(libc::SIGKILL);
                Watch::Killed
            }
            other => other,
        };

        std::thread::sleep(if matches!(watch, Watch::Killed) {
            KILLED_POLL_INTERVAL
        } else {
            POLL_INTERVAL
        });
    }

    // A child ended by the escalation exits non-zero, or by signal. That is
    // the normal completion path, not a failure: the caller judges the work;
    // the runner reports channel appearance and process status separately.
    child.kill_group();
    let reaped = child.reap();
    // Freeze the launch at the reap, before an observer, terminal recovery or
    // group confirmation can delay it. Later signals belong to the caller.
    let at_reap = take_interrupt();
    let elapsed = started.elapsed();
    let interrupted = interrupted.or(at_reap);
    let status = reaped.map_err(|error| Failed {
        error: LaunchError::new(format!("cannot reap the exited child: {error}"))
            .with_consumed_signal(interrupted),
        status: None,
    })?;
    observer(LaunchEvent::Reaped);
    Ok(Watched {
        status,
        interrupted,
        escalated,
        elapsed,
    })
}

/// Signal the job this process launched — **the whole process group, then the
/// child itself**.
///
/// A grandchild the child spawned — a tool subprocess, a language server, an
/// agent's own in-flight command — is a member of that group and is reaped with
/// its parent rather than surviving it. That matters beyond tidiness: such a
/// grandchild can hold a lock its launcher's caller is about to wait on, and
/// then the escalation's SIGKILL buys a stall rather than a teardown.
///
/// `pgid` is the child's pid, made a group leader by the `process_group(0)`
/// [`run`] sets before the spawn, or by the `setsid` a detached launch makes —
/// a failure there is a failed spawn, and no child. A group with that id can
/// only have been created by that process, so `-pgid` cannot name an unrelated
/// job even in the impossible case where the group was never created; the
/// direct `kill` behind it covers that case. Every call is made before the
/// child is reaped, while its pid still reserves the id.
///
/// A failure is ignored on purpose — ESRCH means the process exited between the
/// poll and the signal, which the next poll reports anyway. This is the shell's
/// `kill … 2>/dev/null`, written down.
fn kill(pgid: libc::pid_t, signal: libc::c_int) {
    // SAFETY: `kill(2)` on the process group of, and then the pid of, a child
    // of this process.
    unsafe {
        libc::kill(-pgid, signal);
        libc::kill(pgid, signal);
    }
}

#[cfg(test)]
#[path = "../tests/internal/wait_events.rs"]
mod wait_events;

#[cfg(test)]
mod terminal_tests;
