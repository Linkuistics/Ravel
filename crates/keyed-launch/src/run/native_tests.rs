//! Fixed owner-local settlement controls. Never compiled into installed builds.
use super::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU8};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum Fault {
    #[default]
    None,
    Stop {
        attempt: u32,
        performed: bool,
    },
    ObserveEio,
    ObserveEintr,
    ObserveLost,
    WaitEio,
    WaitEintr,
    WaitLost,
    Running,
}

#[derive(Default)]
pub(super) struct Control {
    fault: Fault,
    observed_once: bool,
    waited_once: bool,
    broken_report: bool,
    physical: Vec<String>,
}

fn set_errno(number: i32) {
    #[cfg(target_vendor = "apple")]
    unsafe {
        *libc::__error() = number;
    }
    #[cfg(target_os = "linux")]
    unsafe {
        *libc::__errno_location() = number;
    }
}
impl Control {
    pub(super) fn stop(&mut self, attempt: u32, target: i32, signal: i32) -> i32 {
        let injected = matches!(self.fault, Fault::Stop { attempt: n, .. } if n == attempt);
        let performed =
            !matches!(self.fault, Fault::Stop { attempt: n, performed: false } if n == attempt);
        let actual = if performed {
            unsafe { libc::kill(target, signal) }
        } else {
            0
        };
        let actual_errno = (actual == -1).then(|| std::io::Error::last_os_error().raw_os_error());
        self.physical.push(format!("stop attempt={attempt} target={target} signal={signal} performed={performed} raw={actual} errno={actual_errno:?}"));
        if injected {
            if self.broken_report {
                return 0;
            }
            set_errno(libc::EPERM);
            return -1;
        }
        actual
    }
    pub(super) fn observed(&mut self, actual: i32, info: &mut libc::siginfo_t) -> i32 {
        // E07's runner-error variant arms this once the first signal's
        // consumption is acknowledged: the next observation reports EIO. The
        // physical WNOWAIT has already run, so only the report is substituted.
        if ARM_OBSERVE_EIO.swap(false, Ordering::SeqCst) {
            self.physical
                .push("WNOWAIT performed=true reported_errno=EIO".to_owned());
            set_errno(libc::EIO);
            return -1;
        }
        let physical_errno = (actual == -1).then(|| std::io::Error::last_os_error().raw_os_error());
        self.physical.push(format!(
            "WNOWAIT performed=true raw={actual} pid={} code={} errno={physical_errno:?}",
            unsafe { info.si_pid() },
            info.si_code
        ));
        // A substituted result never restores a real lost reservation.
        if physical_errno == Some(Some(libc::ECHILD)) {
            return actual;
        }
        let fault = match self.fault {
            Fault::ObserveEio if !self.observed_once => Some(libc::EIO),
            Fault::ObserveEintr => Some(libc::EINTR),
            Fault::ObserveLost => Some(libc::ECHILD),
            _ => None,
        };
        self.observed_once = true;
        if let Some(error) = fault {
            if self.broken_report {
                return actual;
            }
            set_errno(error);
            return -1;
        }
        if self.fault == Fault::Running {
            *info = unsafe { std::mem::zeroed() };
        }
        actual
    }
    pub(super) fn wait(&mut self, pid: i32, status: &mut i32) -> i32 {
        let fault = match self.fault {
            Fault::WaitEio if !self.waited_once => Some(libc::EIO),
            Fault::WaitEintr => Some(libc::EINTR),
            Fault::WaitLost => Some(libc::ECHILD),
            _ => None,
        };
        self.waited_once = true;
        if let Some(error) = fault {
            // Deliberately retain the physical wait reservation while modelling
            // an unavailable wait. The independent fixture owner reaps below.
            self.physical.push(format!(
                "waitpid physical_reap_omitted=true reported_errno={error}"
            ));
            if !self.broken_report {
                set_errno(error);
                return -1;
            }
        }
        let actual = unsafe { libc::waitpid(pid, status, libc::WNOHANG) };
        self.physical.push(format!(
            "waitpid performed=true raw={actual} status={status}"
        ));
        actual
    }
}

struct Shared(Rc<RefCell<OwnedChild>>);
impl Process for Shared {
    fn exited(&mut self) -> std::io::Result<bool> {
        self.0.borrow_mut().exited()
    }
    fn reap(&mut self) -> std::io::Result<ExitStatus> {
        self.0.borrow_mut().reap()
    }
    fn signal(&mut self, signal: i32) {
        self.0.borrow_mut().signal(signal);
    }
    fn kill_group(&mut self) {
        self.0.borrow_mut().kill_group();
    }
    fn confirm_gone(&mut self) -> Group {
        self.0.borrow_mut().confirm_gone()
    }
    fn bound(&mut self, by: Instant) {
        self.0.borrow_mut().bound(by);
    }
    fn reserved(&self) -> bool {
        self.0.borrow().reserved()
    }
    fn actual_reap(&self) -> Option<ExitStatus> {
        self.0.borrow().actual_reap()
    }
    fn stop_reports(&mut self) -> Vec<StopReport> {
        self.0.borrow_mut().stop_reports()
    }
    fn release(&mut self) {
        self.0.borrow_mut().release();
    }
}

/// Active product ownership ends before this independently retained fixture
/// handle may act. A confirmed raw reap disarms it permanently.
struct Fixture(Rc<RefCell<OwnedChild>>);
impl Fixture {
    fn cleanup(&self) -> std::io::Result<()> {
        let mut real = self.0.borrow_mut();
        if real.reaped.is_some() {
            return Ok(());
        }
        let by = Instant::now() + Duration::from_secs(2);
        loop {
            match real.child.try_wait() {
                Ok(Some(status)) => {
                    real.reaped = Some(status);
                    return Ok(());
                }
                Err(error) => return Err(error),
                Ok(None) => {}
            }
            // Every healthy row already physically attempts the other SIGKILLs.
            // Do not signal a product-released numeric identity during cleanup.
            if Instant::now() >= by {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::TimedOut,
                    "fixture reap unconfirmed",
                ));
            }
            std::thread::sleep(Duration::from_millis(1));
        }
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        if let Err(error) = self.cleanup() {
            eprintln!("native fixture cleanup FAILED: {error}");
        }
    }
}

fn settlement_case(
    fault: Fault,
    broken: bool,
) -> (String, Vec<LaunchEvent>, Duration, Vec<String>) {
    let child = Command::new("/bin/sleep")
        .arg("20")
        .process_group(0)
        .spawn()
        .unwrap();
    let mut child = OwnedChild::new(child);
    child.control.fault = fault;
    child.control.broken_report = broken;
    let state = Rc::new(RefCell::new(child));
    let fixture = Fixture(Rc::clone(&state));
    let mut events = Vec::new();
    let at = Instant::now();
    let error = supervise_started(
        Shared(Rc::clone(&state)),
        None,
        Escalation {
            grace: Duration::ZERO,
            kill_grace: Duration::ZERO,
        },
        Mode::Interactive,
        &mut |event| events.push(event),
        &mut || Err(std::io::Error::from_raw_os_error(libc::EIO)),
        || panic!("failed Started lent again"),
        |_| Ok(()),
    )
    .unwrap_err();
    let elapsed = at.elapsed();
    let released = !state.borrow().reserved;
    let physical = state.borrow().control.physical.clone();
    let actual_reap = state.borrow().reaped;
    let cleanup = fixture.cleanup();
    eprintln!("native settlement fault={fault:?} broken={broken} elapsed={elapsed:?} product_released={released} actual_reap={actual_reap:?} events={events:?} fixture_cleanup={cleanup:?} error={error}\nphysical={physical:#?}");
    assert!(released);
    cleanup.unwrap();
    assert_eq!(error.raw_os_error(), None);
    assert_eq!(
        std::error::Error::source(&error)
            .unwrap()
            .downcast_ref::<std::io::Error>()
            .unwrap()
            .raw_os_error(),
        Some(libc::EIO)
    );
    assert_eq!(
        events.iter().filter(|&&e| e == LaunchEvent::Reaped).count(),
        usize::from(actual_reap.is_some())
    );
    assert!(elapsed < Duration::from_millis(2500));
    (error.to_string(), events, elapsed, physical)
}

#[test]
fn native_s01_retains_each_stop_report_and_distinguishes_physical_operations() {
    for attempt in 1..=4 {
        for performed in [false, true] {
            let fault = Fault::Stop { attempt, performed };
            let (message, events, _, physical) = settlement_case(fault, false);
            assert!(message.contains(&format!("stop attempt {attempt} target")));
            assert_eq!(events, [LaunchEvent::Started, LaunchEvent::Reaped]);
            assert_eq!(
                physical
                    .iter()
                    .filter(|s| s.starts_with("stop attempt="))
                    .count(),
                4
            );
            // A fixed lost-report mutant performs the same physical operations.
            // Its absence must fail the independently named report expectation.
            let (broken, _, _, _) = settlement_case(fault, true);
            assert!(
                !broken.contains(&format!("stop attempt {attempt} target")),
                "broken report control was not detected"
            );
        }
    }
}

#[test]
fn native_s02_wait_uncertainty_never_invents_reap_and_releases_ownership() {
    for fault in [
        Fault::ObserveEio,
        Fault::ObserveEintr,
        Fault::ObserveLost,
        Fault::WaitEio,
        Fault::WaitEintr,
        Fault::WaitLost,
        Fault::Running,
    ] {
        let (message, events, elapsed, _) = settlement_case(fault, false);
        match fault {
            Fault::ObserveEio => assert!(message.contains("settlement observation:")),
            Fault::WaitEio => assert!(message.contains("wait recovered after errno")),
            Fault::ObserveLost | Fault::WaitLost => {
                assert!(
                    message.contains("reap unconfirmed") || message.contains("no confirmed reap")
                );
                assert_eq!(events, [LaunchEvent::Started]);
            }
            _ => {
                assert!(message.contains("deadline"));
                assert!(elapsed >= ERROR_SETTLEMENT);
                assert_eq!(events, [LaunchEvent::Started]);
            }
        }
    }
}

// ---- E07: first consumed cancellation survives later signals ---------------
//
// The runner's first latched signal and its first consumed signal both outlive
// any later delivery. The controller acknowledges each stage before the next
// signal is sent, so no row claims an arrival order between simultaneously
// pending standard signals.

static HANDLER_RUNS: AtomicU32 = AtomicU32::new(0);
static SEEN_RUNS: AtomicU32 = AtomicU32::new(0);
static STAGE: AtomicU8 = AtomicU8::new(0);
static ARM_OBSERVE_EIO: AtomicBool = AtomicBool::new(false);
static BREAK: AtomicU8 = AtomicU8::new(0);
static ERROR_VARIANT: AtomicBool = AtomicBool::new(false);
static ACTIVE: AtomicBool = AtomicBool::new(false);

const BREAKS: [&str; 5] = ["none", "latch", "guard", "at-reap", "error"];

/// Async-signal-safe: one atomic increment, no allocation.
pub(super) fn handler_ran() {
    HANDLER_RUNS.fetch_add(1, Ordering::SeqCst);
}
pub(super) fn first_latch_broken(which: &str) -> bool {
    ACTIVE.load(Ordering::SeqCst)
        && BREAKS
            .iter()
            .position(|&name| name == which)
            .is_some_and(|index| BREAK.load(Ordering::SeqCst) as usize == index)
}

/// Facts at one exchange. A probe first waits (bounded, under the controller's
/// acknowledgement) for the handler run that the signal just sent produced, so
/// the acknowledgement is of consumption by the handler and never of a sleep.
fn facts(stage: &str, wait_for_handler: bool) -> String {
    if wait_for_handler {
        let by = Instant::now() + Duration::from_secs(3);
        while HANDLER_RUNS.load(Ordering::SeqCst) <= SEEN_RUNS.load(Ordering::SeqCst)
            && Instant::now() < by
        {
            std::thread::sleep(Duration::from_millis(1));
        }
    }
    let runs = HANDLER_RUNS.load(Ordering::SeqCst);
    let advanced = runs > SEEN_RUNS.swap(runs, Ordering::SeqCst);
    format!(
        "stage={stage} latch={} runs={runs} advanced={advanced}",
        INTERRUPTED_BY.load(Ordering::SeqCst)
    )
}

/// Runner loop top, first iteration only: the signals are latched, not yet
/// taken. The first signal's latch is acknowledged, then a second is sent and
/// must not replace it.
pub(super) fn runner_active() {
    if !ACTIVE.load(Ordering::SeqCst)
        || STAGE
            .compare_exchange(0, 1, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
    {
        return;
    }
    let mut first = true;
    let result = crate::native_barrier::exchange("runner-active", &mut || {
        // The ready record precedes any signal: do not wait for a handler.
        let wait = !std::mem::replace(&mut first, false);
        facts("runner-active", wait)
    });
    if let Err(error) = result {
        eprintln!("E07 barrier failure at runner-active: {error}");
        std::process::exit(70);
    }
}

/// Immediately after the runner took (consumed) its first signal.
pub(super) fn first_consumed(signal: i32) {
    if !ACTIVE.load(Ordering::SeqCst)
        || STAGE
            .compare_exchange(1, 2, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
    {
        return;
    }
    let mut first = true;
    let result = crate::native_barrier::exchange("first-consumed", &mut || {
        let wait = !std::mem::replace(&mut first, false);
        format!("{} consumed={signal}", facts("first-consumed", wait))
    });
    if let Err(error) = result {
        eprintln!("E07 barrier failure at first-consumed: {error}");
        std::process::exit(70);
    }
    if ERROR_VARIANT.load(Ordering::SeqCst) {
        // The first consumption is acknowledged; the next wait observation
        // reports an error so the runner settles through its error path.
        ARM_OBSERVE_EIO.store(true, Ordering::SeqCst);
    }
}

const ROLE: &str = "run::native_tests::native_e07_role";

#[test]
#[ignore = "private E07 runner role; run task test:native-entry"]
fn native_e07_role() {
    let Some(variant) = std::env::var_os("K28_E07_VARIANT") else {
        return;
    };
    let broken = std::env::var("K28_E07_BREAK").unwrap();
    BREAK.store(
        BREAKS.iter().position(|&b| b == broken).unwrap() as u8,
        Ordering::SeqCst,
    );
    ERROR_VARIANT.store(variant == "error", Ordering::SeqCst);
    ACTIVE.store(true, Ordering::SeqCst);
    let argv = crate::Argv::new("/bin/sleep".into(), vec!["30".into()]);
    let log = tempfile::tempfile().unwrap();
    let outcome = crate::run_noninteractive(
        crate::NoninteractiveLaunch {
            argv: &argv,
            scrub: &[],
            cwd: None,
            escalation: Escalation {
                grace: Duration::from_secs(1),
                kill_grace: Duration::from_secs(1),
            },
        },
        log,
    );
    let line = match outcome {
        Ok(ended) => match ended.end {
            End::Interrupted { signal } => format!("ended interrupted={signal}"),
            other => format!("ended other={other:?}"),
        },
        Err(error) => format!(
            "error consumed={:?} message={error}",
            error.consumed_signal()
        ),
    };
    let path = std::path::PathBuf::from(std::env::var_os("K28_E07_OUT").unwrap());
    std::fs::write(path, line).unwrap();
}

#[derive(Debug)]
struct E07 {
    first_ack_latch: Option<i32>,
    second_ack_latch: Option<i32>,
    consumed_ack_latch: Option<i32>,
    outcome: String,
    clean: bool,
    reported: Vec<String>,
}
impl E07 {
    /// Which signal the launch's own result named: the Ended interruption or
    /// the error's consumed signal. `None` when it named none.
    fn named(&self) -> Option<i32> {
        self.outcome
            .split_whitespace()
            .find_map(|word| {
                word.strip_prefix("interrupted=")
                    .or_else(|| word.strip_prefix("consumed=Some("))
            })
            .and_then(|value| value.trim_end_matches(')').parse().ok())
    }
    fn conforms(&self, first: i32, error: bool) -> bool {
        let shape = if error { "error " } else { "ended " };
        self.clean
            && self.first_ack_latch == Some(first)
            && self.second_ack_latch == Some(first)
            && self.outcome.starts_with(shape)
            && self.named() == Some(first)
    }
}

fn field(facts: &str, name: &str) -> Option<i32> {
    facts
        .split_whitespace()
        .find_map(|word| word.strip_prefix(&format!("{name}=")))
        .and_then(|value| value.parse().ok())
}

fn e07_row(first: i32, second: i32, error: bool, broken: &str) -> E07 {
    use crate::native_barrier::{Controller, Limits};
    use std::os::unix::process::CommandExt as _;
    let directory = tempfile::tempdir().unwrap();
    let channels = directory.path().join("barrier");
    std::fs::create_dir(&channels).unwrap();
    let out = directory.path().join("outcome");
    let mut controller = Controller::new(&channels, Limits::FROZEN).unwrap();
    let mut command = Command::new(std::env::current_exe().unwrap());
    command
        .args(["--exact", ROLE, "--ignored", "--nocapture"])
        .env("K28_E07_VARIANT", if error { "error" } else { "active" })
        .env("K28_E07_BREAK", broken)
        .env("K28_E07_OUT", &out)
        .process_group(0)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    controller.configure(&mut command);
    controller.adopt(command.spawn().unwrap());

    let mut row = E07 {
        first_ack_latch: None,
        second_ack_latch: None,
        consumed_ack_latch: None,
        outcome: String::new(),
        clean: false,
        reported: Vec::new(),
    };
    // READY -> first -> ack -> second -> ack -> release, then the second
    // exchange after the runner took the first.
    let flow = (|| -> Result<(), String> {
        controller.ready()?;
        controller.signal(first)?;
        row.first_ack_latch = field(&controller.probe()?, "latch");
        controller.signal(second)?;
        row.second_ack_latch = field(&controller.probe()?, "latch");
        controller.release()?;
        controller.ready()?;
        controller.signal(second)?;
        row.consumed_ack_latch = field(&controller.probe()?, "latch");
        controller.release()
    })();
    let settlement = if flow.is_ok() {
        controller.finish()
    } else {
        controller.abort()
    };
    row.clean =
        flow.is_ok() && settlement.clean() && settlement.status.is_some_and(|s| s.success());
    if let Err(error) = flow {
        row.reported.push(error);
    }
    row.outcome = std::fs::read_to_string(&out).unwrap_or_default();
    eprintln!("native E07 first={first} second={second} error={error} broken={broken}: {row:?} {settlement:?}");
    row
}

#[test]
fn native_e07_first_consumed_cancellation_survives_every_later_delivery() {
    let rows = [(libc::SIGTERM, libc::SIGHUP), (libc::SIGHUP, libc::SIGINT)];
    for (first, second) in rows {
        for error in [false, true] {
            let baseline = e07_row(first, second, error, "none");
            assert!(baseline.conforms(first, error), "{baseline:?}");
            // The acknowledged later delivery landed after the first's
            // consumption, so the runner's own result is the evidence.
            assert_eq!(baseline.consumed_ack_latch, Some(second), "{baseline:?}");
            // Each guard is load-bearing: break the latch store, then the
            // consumption guard / reap join (active) or the error composition
            // (runner-error). A named break lets the *later* signal win.
            let breaks: &[&str] = if error {
                &["latch", "error"]
            } else {
                &["latch", "guard", "at-reap"]
            };
            for broken in breaks {
                let seen = e07_row(first, second, error, broken);
                assert!(
                    seen.clean,
                    "{broken}: controller settlement must stay clean: {seen:?}"
                );
                assert!(
                    seen.outcome
                        .starts_with(if error { "error " } else { "ended " }),
                    "{broken}: launch shape changed: {seen:?}"
                );
                assert_eq!(
                    seen.named(),
                    Some(second),
                    "{broken} did not let the later signal win for {first}->{second} error={error}: {seen:?}"
                );
                // The break changes only what it names: the first latch's
                // acknowledgement is intact unless the latch itself is broken.
                if *broken != "latch" {
                    assert_eq!(seen.first_ack_latch, Some(first), "{seen:?}");
                    assert_eq!(seen.second_ack_latch, Some(first), "{seen:?}");
                }
            }
        }
    }
}

// ---- E04: managed-boundary no-child-wait repair ------------------------------
//
// The runner is the managed boundary. A private default-plus-SA_NOCLDWAIT
// action is installed after entry, the runner's own query records the flags it
// saw immediately before its checked watching, and a real child's exit and wait
// are the receipt. The ignore-only mutant repairs SIG_IGN and nothing else, so
// the kernel reaps the child unwatched and the launch cannot confirm it.

static IGNORE_ONLY: AtomicBool = AtomicBool::new(false);
static WATCHING_FLAGS: AtomicI32 = AtomicI32::new(-1);

pub(super) fn ignore_only_repair() -> bool {
    IGNORE_ONLY.load(Ordering::SeqCst)
}
/// The flags the runner's own query saw immediately before it repaired.
pub(super) fn watching_queried(flags: i32) {
    WATCHING_FLAGS.store(flags, Ordering::SeqCst);
}

fn sigchld_action() -> libc::sigaction {
    let mut action = unsafe { std::mem::zeroed::<libc::sigaction>() };
    assert_eq!(
        unsafe { libc::sigaction(libc::SIGCHLD, std::ptr::null(), &mut action) },
        0
    );
    action
}

const E04_ROLE: &str = "run::native_tests::native_e04_role";

#[test]
#[ignore = "private E04 managed-boundary role; run task test:native-entry"]
fn native_e04_role() {
    let Some(mode) = std::env::var_os("K28_E04_BREAK") else {
        return;
    };
    IGNORE_ONLY.store(mode == "ignore-only", Ordering::SeqCst);
    // The private post-entry action: default disposition plus no-child-wait.
    let mut action = unsafe { std::mem::zeroed::<libc::sigaction>() };
    action.sa_sigaction = libc::SIG_DFL;
    action.sa_flags = libc::SA_NOCLDWAIT;
    assert_eq!(unsafe { libc::sigemptyset(&mut action.sa_mask) }, 0);
    assert_eq!(
        unsafe { libc::sigaction(libc::SIGCHLD, &action, std::ptr::null_mut()) },
        0
    );
    let private = sigchld_action();
    let argv = crate::Argv::new("/bin/sh".into(), vec!["-c".into(), "exit 7".into()]);
    let mut events = Vec::new();
    let result = run_observed(
        Launch {
            argv: &argv,
            channel: None,
            scrub: &[],
            grant: &[],
            transparent: None,
            cwd: None,
            escalation: Escalation {
                grace: Duration::from_secs(1),
                kill_grace: Duration::from_secs(1),
            },
        },
        &mut |event| events.push(event),
    );
    let after = sigchld_action();
    let ended = match &result {
        Ok(ended) => format!(
            "end={:?} code={:?} group={:?}",
            ended.end,
            ended.status.code(),
            ended.group
        ),
        Err(error) => format!("error={error}"),
    };
    let line = format!(
        "private_flags={} runner_saw_flags={} events={events:?} {ended} after_flags={}",
        private.sa_flags,
        WATCHING_FLAGS.load(Ordering::SeqCst),
        after.sa_flags
    );
    std::fs::write(std::env::var_os("K28_E04_OUT").unwrap(), line).unwrap();
}

#[derive(Debug)]
struct E04 {
    line: String,
    clean: bool,
}
impl E04 {
    fn number(&self, name: &str) -> Option<i32> {
        self.line
            .split_whitespace()
            .find_map(|word| word.strip_prefix(&format!("{name}=")))
            .and_then(|value| value.parse().ok())
    }
    fn no_child_wait_seen(&self) -> bool {
        self.number("runner_saw_flags")
            .is_some_and(|flags| flags & libc::SA_NOCLDWAIT != 0)
    }
    fn repaired(&self) -> bool {
        self.number("after_flags")
            .is_some_and(|flags| flags & libc::SA_NOCLDWAIT == 0)
    }
    /// The real child's exit and wait were confirmed with its own status.
    fn confirmed(&self) -> bool {
        self.line.contains("events=[Started, Reaped]")
            && self.line.contains("end=Exited code=Some(7) group=Gone")
    }
}

fn e04_row(broken: &str) -> E04 {
    use crate::native_barrier::{Controller, Limits};
    let directory = tempfile::tempdir().unwrap();
    let channels = directory.path().join("barrier");
    std::fs::create_dir(&channels).unwrap();
    let out = directory.path().join("outcome");
    let mut controller = Controller::new(&channels, Limits::FROZEN).unwrap();
    let mut command = Command::new(std::env::current_exe().unwrap());
    command
        .args(["--exact", E04_ROLE, "--ignored", "--nocapture"])
        .env("K28_E04_BREAK", broken)
        .env("K28_E04_OUT", &out)
        .process_group(0)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    controller.configure(&mut command);
    controller.adopt(command.spawn().unwrap());
    let settlement = controller.finish();
    let row = E04 {
        line: std::fs::read_to_string(&out).unwrap_or_default(),
        clean: settlement.clean() && settlement.status.is_some_and(|s| s.success()),
    };
    eprintln!("native E04 managed broken={broken}: {row:?} {settlement:?}");
    row
}

#[test]
fn native_e04_managed_default_no_child_wait_is_repaired_and_ignore_only_fails() {
    let repaired = e04_row("none");
    assert!(repaired.clean, "{repaired:?}");
    assert!(
        repaired.no_child_wait_seen(),
        "the runner did not see the private flag: {repaired:?}"
    );
    assert!(repaired.repaired(), "{repaired:?}");
    assert!(repaired.confirmed(), "{repaired:?}");

    // The independent mutant repairs the ignore and omits the flag. The
    // instrument's own receipt proves it left the flag in place; the real child
    // is then reaped by the kernel and no wait can be confirmed.
    let mutant = e04_row("ignore-only");
    assert!(mutant.clean, "controller settlement: {mutant:?}");
    assert!(mutant.no_child_wait_seen(), "{mutant:?}");
    assert!(
        !mutant.repaired(),
        "ignore-only mutant repaired the flag after all: {mutant:?}"
    );
    assert!(
        !mutant.confirmed(),
        "ignore-only repair still confirmed the child's wait: {mutant:?}"
    );
}

// ---- E05: direct INT versus terminal VINTR, before and after handoff --------
//
// The role is a launcher on a real PTY of its own (session and group leader
// with the slave as its controlling terminal). The controller owns it, the
// master and the acknowledgements. A direct INT is `kill(launcher)`; a terminal
// INT is the VINTR byte written to the master, which the line discipline
// delivers to whichever group is then foreground. Before the child is spawned
// that group is the launcher's own; once the child has exec'd and taken the
// terminal it is the child's. The facts record SID, PGIDs and `tcgetpgrp`.

static E05_ACTIVE: AtomicBool = AtomicBool::new(false);
static E05_BEFORE: AtomicBool = AtomicBool::new(false);
static E05_INVENT: AtomicBool = AtomicBool::new(false);
static E05_CHANNEL: std::sync::OnceLock<std::path::PathBuf> = std::sync::OnceLock::new();
static E05_CHILD_PID: std::sync::OnceLock<std::path::PathBuf> = std::sync::OnceLock::new();

pub(super) fn invented_cancellation(status: ExitStatus) -> Option<i32> {
    use std::os::unix::process::ExitStatusExt as _;
    (E05_INVENT.load(Ordering::SeqCst) && status.signal() == Some(libc::SIGINT))
        .then_some(libc::SIGINT)
}

fn e05_facts(stage: &str) -> String {
    let foreground = Terminal::open().map_or(-2, |terminal| terminal.foreground());
    let published = E05_CHANNEL.get().is_some_and(|path| path.exists());
    let child = E05_CHILD_PID
        .get()
        .and_then(|path| std::fs::read_to_string(path).ok())
        .and_then(|text| text.trim().parse::<i32>().ok())
        .unwrap_or(0);
    // SAFETY: argument-free queries of this process's own identity.
    let (pid, sid, group) = unsafe { (libc::getpid(), libc::getsid(0), libc::getpgrp()) };
    format!(
        "stage={stage} pid={pid} sid={sid} pgrp={group} fg={foreground} child={child} \
         published={} latch={}",
        i32::from(published),
        INTERRUPTED_BY.load(Ordering::SeqCst)
    )
}

fn e05_exchange(stage: &str) {
    if let Err(error) = crate::native_barrier::exchange(stage, &mut || e05_facts(stage)) {
        eprintln!("E05 barrier failure at {stage}: {error}");
        std::process::exit(70);
    }
}

/// The parent's last act before spawning: the launcher still owns the terminal.
pub(super) fn before_spawn() {
    if E05_ACTIVE.load(Ordering::SeqCst) && E05_BEFORE.load(Ordering::SeqCst) {
        e05_exchange("before-handoff");
    }
}

const E05_ROLE: &str = "run::native_tests::native_e05_role";

#[test]
#[ignore = "private E05 launcher role on its own PTY; run task test:native-entry"]
fn native_e05_role() {
    let Some(boundary) = std::env::var_os("K28_E05_BOUNDARY") else {
        return;
    };
    let before = boundary == "before";
    E05_BEFORE.store(before, Ordering::SeqCst);
    E05_INVENT.store(
        std::env::var_os("K28_E05_BREAK").is_some_and(|mode| mode == "invent"),
        Ordering::SeqCst,
    );
    E05_ACTIVE.store(true, Ordering::SeqCst);
    let directory = std::path::PathBuf::from(std::env::var_os("K28_E05_DIR").unwrap());
    let channel = crate::Channel::allocate(&directory).unwrap();
    let _ = E05_CHANNEL.set(channel.path().to_owned());
    let _ = E05_CHILD_PID.set(directory.join("child.pid"));
    let terminal = Terminal::open().expect("the role must have a controlling terminal");
    let own = own_group();
    // The child records its identity, publishes through the channel and then
    // waits with the default INT disposition it was handed.
    let script = r#"echo $$ > "$K28_E05_DIR/child.pid"; printf done > "$K28_E05_CHANNEL"; exec /bin/sleep 600"#;
    let argv = crate::Argv::new("/bin/sh".into(), vec!["-c".into(), script.into()]);
    let entry = EntrySignals::new([], []);
    let grant = [(OsStr::new("K28_E05_DIR"), directory.as_os_str())];
    let mut events = Vec::new();
    let result = run_observed_with_start_check(
        Launch {
            argv: &argv,
            channel: Some((&channel, "K28_E05_CHANNEL")),
            scrub: &[],
            grant: &grant,
            transparent: Some(&entry),
            cwd: None,
            escalation: Escalation {
                grace: Duration::from_secs(10),
                kill_grace: Duration::from_secs(5),
            },
        },
        &mut |event| events.push(event),
        &mut || {
            if !before {
                e05_exchange("after-handoff");
            }
            Ok(())
        },
    );
    use std::os::unix::process::ExitStatusExt as _;
    let ended = match &result {
        Ok(ended) => format!(
            "end={:?} status_signal={:?} signalled={} group={:?}",
            ended.end,
            ended.status.signal(),
            ended.signalled,
            ended.group
        ),
        Err(error) => format!(
            "error consumed={:?} message={error}",
            error.consumed_signal()
        ),
    };
    let line = format!(
        "events={events:?} {ended} foreground_restored={}",
        terminal.foreground() == own
    );
    std::fs::write(std::env::var_os("K28_E05_OUT").unwrap(), line).unwrap();
}

#[derive(Debug)]
struct E05 {
    ready: Option<String>,
    ack: Option<String>,
    outcome: String,
    clean: bool,
    faults: Vec<String>,
}
fn facts_number(facts: Option<&String>, name: &str) -> Option<i64> {
    facts?
        .split_whitespace()
        .find_map(|word| word.strip_prefix(&format!("{name}=")))
        .and_then(|value| value.parse().ok())
}
impl E05 {
    fn ready(&self, name: &str) -> Option<i64> {
        facts_number(self.ready.as_ref(), name)
    }
    fn ack(&self, name: &str) -> Option<i64> {
        facts_number(self.ack.as_ref(), name)
    }
    fn interrupted_by_launcher(&self) -> bool {
        self.outcome.contains("end=Interrupted { signal: 2 }")
            || self.outcome.contains("consumed=Some(2)")
    }
    /// What the contract requires of one row, as separate named axes.
    fn axes(&self, delivery: &str, boundary: &str) -> E05Axes {
        let (pgrp, foreground) = (self.ready("pgrp"), self.ready("fg"));
        let leader = self.ready("pid") == pgrp && self.ready("sid") == pgrp;
        let handed = self.ready("child").is_some_and(|child| child > 0)
            && foreground == self.ready("child")
            && foreground != pgrp
            && self.ack("published") == Some(1);
        let target = if boundary == "before" {
            // Nothing has been handed over: the launcher's own group is
            // foreground, so a terminal INT can only be the launcher's.
            leader && foreground == pgrp && self.ready("child") == Some(0)
        } else {
            leader && handed
        };
        let launcher_received = self.ack("latch") == Some(i64::from(libc::SIGINT));
        let child_status = self
            .outcome
            .contains("end=Exited status_signal=Some(2) signalled=true");
        let by_launcher = delivery == "direct" || boundary == "before";
        E05Axes {
            controller: self.clean && self.faults.is_empty(),
            target,
            // The launcher's latch saw the INT iff it was the launcher's.
            routed: launcher_received == by_launcher,
            attributed: if by_launcher {
                self.interrupted_by_launcher()
            } else {
                // Publish-then-terminal-INT: the child's own status, with no
                // launcher cancellation invented from it.
                child_status && !self.interrupted_by_launcher()
            },
            reaped: self.outcome.contains("group=Gone") || self.outcome.contains("error consumed"),
            foreground_restored: self.outcome.contains("foreground_restored=true"),
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
struct E05Axes {
    controller: bool,
    target: bool,
    routed: bool,
    attributed: bool,
    reaped: bool,
    foreground_restored: bool,
}
impl E05Axes {
    fn conforms(&self) -> bool {
        self.controller
            && self.target
            && self.routed
            && self.attributed
            && self.reaped
            && self.foreground_restored
    }
}

fn e05_row(delivery: &str, boundary: &str, broken: &str) -> E05 {
    use crate::native_barrier::{Controller, Limits};
    use std::io::Write as _;
    use std::os::fd::{AsRawFd as _, FromRawFd as _};
    use std::os::unix::process::CommandExt as _;
    let directory = tempfile::tempdir().unwrap();
    let channels = directory.path().join("barrier");
    std::fs::create_dir(&channels).unwrap();
    let out = directory.path().join("outcome");
    let (mut master, mut slave) = (-1, -1);
    // SAFETY: openpty fills two descriptors that are owned immediately below.
    assert_eq!(
        unsafe {
            libc::openpty(
                &mut master,
                &mut slave,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            )
        },
        0
    );
    // SAFETY: fresh descriptors from openpty.
    let (mut master, slave) = unsafe {
        (
            std::fs::File::from_raw_fd(master),
            std::fs::File::from_raw_fd(slave),
        )
    };
    for fd in [master.as_raw_fd(), slave.as_raw_fd()] {
        // SAFETY: setting close-on-exec on an owned descriptor.
        assert_eq!(
            unsafe { libc::fcntl(fd, libc::F_SETFD, libc::FD_CLOEXEC) },
            0
        );
    }
    let mut controller = Controller::new(&channels, Limits::FROZEN).unwrap();
    let mut command = Command::new(std::env::current_exe().unwrap());
    command
        .args([
            "--exact",
            E05_ROLE,
            "--ignored",
            "--nocapture",
            "--test-threads=1",
        ])
        .env("K28_E05_BOUNDARY", boundary)
        .env("K28_E05_BREAK", broken)
        .env("K28_E05_DIR", directory.path())
        .env("K28_E05_OUT", &out)
        .stdin(slave.try_clone().unwrap())
        .stdout(slave.try_clone().unwrap())
        .stderr(slave);
    controller.configure(&mut command);
    // SAFETY: only async-signal-safe calls between fork and exec. setsid makes
    // the role its own session and group leader; TIOCSCTTY gives it the slave.
    unsafe {
        command.pre_exec(|| {
            if libc::setsid() == -1 || libc::ioctl(0, libc::TIOCSCTTY as _, 0) == -1 {
                return Err(std::io::Error::last_os_error());
            }
            Ok(())
        });
    }
    controller.adopt(command.spawn().unwrap());
    drop(command);

    // A process exiting onto a terminal nobody reads can block in tty close,
    // beyond even SIGKILL, so the row drains its own master on a thread it
    // owns and stops: nonblocking, polled, and joined on every path.
    // SAFETY: setting O_NONBLOCK on a descriptor this row owns.
    unsafe {
        let flags = libc::fcntl(master.as_raw_fd(), libc::F_GETFL);
        assert_eq!(
            libc::fcntl(master.as_raw_fd(), libc::F_SETFL, flags | libc::O_NONBLOCK),
            0
        );
    }
    let stop = std::sync::Arc::new(AtomicBool::new(false));
    let drain = {
        let mut reader = master.try_clone().unwrap();
        let stop = std::sync::Arc::clone(&stop);
        std::thread::spawn(move || {
            use std::io::Read as _;
            let mut sink = [0u8; 4096];
            while !stop.load(Ordering::SeqCst) {
                let mut poll = libc::pollfd {
                    fd: reader.as_raw_fd(),
                    events: libc::POLLIN,
                    revents: 0,
                };
                // SAFETY: one initialised pollfd, bounded wait.
                unsafe { libc::poll(&mut poll, 1, 20) };
                while matches!(reader.read(&mut sink), Ok(count) if count > 0) {}
            }
        })
    };

    let mut row = E05 {
        ready: None,
        ack: None,
        outcome: String::new(),
        clean: false,
        faults: Vec::new(),
    };
    let flow = (|| -> Result<(), String> {
        row.ready = Some(controller.ready()?);
        if boundary == "after" {
            // The child's publication is the acknowledgement that it ran and
            // that the signal now follows it. Each probe is an acknowledged
            // exchange bounded by its own five seconds.
            let by = Instant::now() + Duration::from_secs(10);
            loop {
                let facts = controller.probe()?;
                if facts_number(Some(&facts), "published") == Some(1) {
                    row.ready = Some(facts);
                    break;
                }
                if Instant::now() >= by {
                    return Err("the child never published".to_owned());
                }
            }
        }
        if delivery == "direct" {
            controller.signal(libc::SIGINT)?;
        } else {
            // The master's VINTR byte; the line discipline picks the group.
            master
                .write_all(&[3])
                .map_err(|error| format!("VINTR write: {error}"))?;
        }
        row.ack = Some(controller.probe()?);
        controller.release()
    })();
    let settlement = if flow.is_ok() {
        controller.finish()
    } else {
        controller.abort()
    };
    if let Err(fault) = flow {
        row.faults.push(fault);
    }
    // The master stays open until the role has been settled.
    stop.store(true, Ordering::SeqCst);
    drain.join().unwrap();
    drop(master);
    row.clean = settlement.clean() && settlement.status.is_some_and(|s| s.success());
    row.outcome = std::fs::read_to_string(&out).unwrap_or_default();
    eprintln!("native E05 {delivery}/{boundary} broken={broken}: {row:?} {settlement:?}");
    row
}

#[test]
fn native_e05_direct_and_terminal_int_before_and_after_handoff() {
    for delivery in ["direct", "pty"] {
        for boundary in ["before", "after"] {
            let seen = e05_row(delivery, boundary, "none");
            let axes = seen.axes(delivery, boundary);
            assert!(axes.conforms(), "{delivery}/{boundary}: {axes:?} {seen:?}");
        }
    }
    // Invented cancellation: the child's own terminal-INT death reported as the
    // launcher's cancellation. Only the publish-then-terminal-INT row can tell.
    let invented = e05_row("pty", "after", "invent");
    let axes = invented.axes("pty", "after");
    assert!(
        axes.controller && axes.target && axes.routed,
        "{axes:?} {invented:?}"
    );
    assert!(
        !axes.attributed && invented.interrupted_by_launcher(),
        "the invented-cancellation mutant was not detected: {axes:?} {invented:?}"
    );
}
