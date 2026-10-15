//! Fixed owner-local settlement controls. Never compiled into installed builds.
use super::*;
use std::cell::RefCell;
use std::rc::Rc;

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
