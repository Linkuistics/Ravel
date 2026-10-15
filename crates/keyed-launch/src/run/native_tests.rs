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
