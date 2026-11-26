// Compiled as a private run-module test, not as an integration target.
use super::*;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::os::unix::process::ExitStatusExt as _;
use std::rc::Rc;

type Trace = Rc<RefCell<Vec<&'static str>>>;
struct FakeProcess {
    polls: VecDeque<std::io::Result<bool>>,
    reaps: VecDeque<std::io::Result<ExitStatus>>,
    group: Group,
    trace: Trace,
    signal_at_reap: Option<i32>,
    stops: Stops,
}
/// The fake group's ID, the PID of its leader.
const FAKE_PGID: i32 = 4242;
/// Which numeric target refuses each signal, and with which errno.
#[derive(Clone, Copy)]
enum Refusal {
    Group(i32),
    Pid(i32),
    /// Only these attempts, each with its errno.
    Attempts(&'static [(u32, i32)]),
}
/// Stop reports in the form `OwnedChild::stop_target` makes them.
#[derive(Default)]
struct Stops {
    refusal: Option<Refusal>,
    attempts: u32,
    reports: Vec<StopReport>,
}
impl Stops {
    fn attempt(&mut self, target: i32, signal: i32) {
        self.attempts += 1;
        let errno = match self.refusal {
            Some(Refusal::Group(errno)) if target < 0 => errno,
            Some(Refusal::Pid(errno)) if target > 0 => errno,
            Some(Refusal::Attempts(refused)) => {
                match refused.iter().find(|(n, _)| *n == self.attempts) {
                    Some(&(_, errno)) => errno,
                    None => return,
                }
            }
            _ => return,
        };
        self.reports.push(StopReport {
            attempt: self.attempts,
            target,
            signal,
            error: std::io::Error::from_raw_os_error(errno),
        });
    }
}
impl Process for FakeProcess {
    fn exited(&mut self) -> std::io::Result<bool> {
        self.trace.borrow_mut().push("poll");
        self.polls.pop_front().expect("unexpected poll")
    }
    fn reap(&mut self) -> std::io::Result<ExitStatus> {
        self.trace.borrow_mut().push("wait");
        let status = self.reaps.pop_front().expect("unexpected wait");
        if let Some(signal) = self.signal_at_reap {
            INTERRUPTED_BY.store(signal, Ordering::Relaxed);
        }
        status
    }
    fn signal(&mut self, signal: i32) {
        self.trace.borrow_mut().push(match signal {
            libc::SIGTERM => "term",
            libc::SIGKILL => "kill",
            _ => "forward",
        });
        self.stops.attempt(-FAKE_PGID, signal);
        self.stops.attempt(FAKE_PGID, signal);
    }
    fn kill_group(&mut self) {
        self.trace.borrow_mut().push("kill-group");
        self.stops.attempt(-FAKE_PGID, libc::SIGKILL);
        self.stops.attempt(-FAKE_PGID, libc::SIGKILL);
    }
    fn confirm_gone(&mut self) -> Group {
        self.trace.borrow_mut().push("confirm");
        self.group
    }
    fn stop_reports(&mut self) -> Vec<StopReport> {
        std::mem::take(&mut self.stops.reports)
    }
}
fn failed<T>() -> std::io::Result<T> {
    Err(std::io::Error::other("injected wait failure"))
}
/// The interrupt latch is process-global and these tests run on parallel
/// threads, so a latch one test sets could be read by another's supervision.
/// Every test here that supervises holds this.
static LATCH: std::sync::Mutex<()> = std::sync::Mutex::new(());
fn serial() -> std::sync::MutexGuard<'static, ()> {
    LATCH
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

const NO_WAIT: Escalation = Escalation {
    grace: Duration::ZERO,
    kill_grace: Duration::ZERO,
};

#[test]
fn the_first_consumed_signal_survives_a_second_handler_call() {
    let _serial = serial();
    INTERRUPTED_BY.store(0, Ordering::Relaxed);
    on_terminate(libc::SIGHUP);
    on_terminate(libc::SIGTERM);
    assert_eq!(take_interrupt(), Some(libc::SIGHUP));
}

/// Every path to a reaped child runs the end in one order: the exit is observed
/// unreaped, the group is killed, the child is reaped and reported, the
/// terminal is taken back, and only then is the group confirmed gone. A wait failure skips the group steps, which need a child
/// known to be a zombie, and still recovers the terminal.
#[test]
fn confirmed_reap_precedes_recovery_on_every_wait_path() {
    let _serial = serial();
    for path in ["poll", "recovery", "escalation"] {
        for confirmed in [false, true] {
            if path == "poll" && !confirmed {
                continue;
            }
            let dir = tempfile::TempDir::new().unwrap();
            let channel = Channel::allocate(dir.path()).unwrap();
            let trace = Trace::default();
            let status = ExitStatus::from_raw(0);
            let polls = match path {
                "poll" => vec![Ok(true)],
                "recovery" => vec![failed()],
                _ => {
                    std::fs::write(channel.path(), "before").unwrap();
                    vec![Ok(false), Ok(false), Ok(false), Ok(true)]
                }
            };
            let child = FakeProcess {
                polls: polls.into(),
                reaps: vec![if confirmed { Ok(status) } else { failed() }].into(),
                group: Group::Gone,
                trace: trace.clone(),
                signal_at_reap: None,
                stops: Stops::default(),
            };
            let recovered = RefCell::new(None);
            let outcome = supervise(
                child,
                Some(&channel),
                NO_WAIT,
                Mode::Interactive,
                &mut |event| {
                    assert_eq!(event, LaunchEvent::Reaped);
                    trace.borrow_mut().push("reaped");
                    std::fs::write(channel.path(), "after").unwrap();
                },
                || {},
                |reaped| {
                    trace.borrow_mut().push("recover");
                    *recovered.borrow_mut() = Some(reaped);
                },
            );
            let expected: &[&str] = match (path, confirmed) {
                ("poll", _) => &["poll", "kill-group", "wait", "reaped", "recover", "confirm"],
                ("recovery", true) => &["poll", "kill", "wait", "reaped", "recover"],
                ("recovery", false) => &["poll", "kill", "wait", "recover"],
                (_, true) => &[
                    "poll",
                    "poll",
                    "term",
                    "poll",
                    "kill",
                    "poll",
                    "kill-group",
                    "wait",
                    "reaped",
                    "recover",
                    "confirm",
                ],
                (_, false) => &[
                    "poll",
                    "poll",
                    "term",
                    "poll",
                    "kill",
                    "poll",
                    "kill-group",
                    "wait",
                    "recover",
                ],
            };
            assert_eq!(&*trace.borrow(), expected, "{path}, confirmed={confirmed}");
            assert_eq!(
                recovered.into_inner(),
                Some(confirmed.then_some(status)),
                "the terminal is recovered once, told the status only of a reaped child"
            );
            if path == "recovery" || !confirmed {
                let error = outcome.unwrap_err().to_string();
                assert!(error.contains("injected wait failure"));
                if path == "recovery" {
                    assert!(error.contains(if confirmed {
                        "and reaped"
                    } else {
                        "could not be reaped"
                    }));
                }
            } else {
                let ended = outcome.unwrap();
                assert!(ended.signalled);
                assert_eq!(ended.group, Group::Gone);
            }
        }
    }
}

/// A group that still answers after the reap is reported beside the child's
/// own status, never in place of it.
#[test]
fn a_surviving_group_is_reported_beside_the_childs_status() {
    let _serial = serial();
    let dir = tempfile::TempDir::new().unwrap();
    let channel = Channel::allocate(dir.path()).unwrap();
    let status = ExitStatus::from_raw(3 << 8);
    let child = FakeProcess {
        polls: vec![Ok(true)].into(),
        reaps: vec![Ok(status)].into(),
        group: Group::Present { pgid: 4242 },
        trace: Trace::default(),
        signal_at_reap: None,
        stops: Stops::default(),
    };
    let ended = supervise(
        child,
        Some(&channel),
        NO_WAIT,
        Mode::Interactive,
        &mut |_| {},
        || {},
        |_| {},
    )
    .unwrap();
    assert_eq!(ended.group, Group::Present { pgid: 4242 });
    assert_eq!(ended.status.code(), Some(3));
    assert_eq!(ended.end, End::Exited);
}

/// Run the escalation of a sent result against a child that is still running
/// at the TERM and has exited at the next observation. `refusal` names the
/// numeric target that refuses each stop, and `group` what the absence query
/// answers after the reap.
fn escalate_against(
    refusal: Refusal,
    group: Group,
) -> (Result<Ended, LaunchError>, Vec<&'static str>) {
    escalate_with(refusal, group, vec![Ok(false), Ok(false), Ok(true)])
}

/// As [`escalate_against`], with the observations given.
fn escalate_with(
    refusal: Refusal,
    group: Group,
    polls: Vec<std::io::Result<bool>>,
) -> (Result<Ended, LaunchError>, Vec<&'static str>) {
    let dir = tempfile::TempDir::new().unwrap();
    let channel = Channel::allocate(dir.path()).unwrap();
    std::fs::write(channel.path(), "staging-ok").unwrap();
    let trace = Trace::default();
    let child = FakeProcess {
        polls: polls.into(),
        reaps: vec![Ok(ExitStatus::from_raw(0))].into(),
        group,
        trace: trace.clone(),
        signal_at_reap: None,
        stops: Stops {
            refusal: Some(refusal),
            ..Stops::default()
        },
    };
    let outcome = supervise(
        child,
        Some(&channel),
        NO_WAIT,
        Mode::Noninteractive,
        &mut |_| trace.borrow_mut().push("reaped"),
        || {},
        |_| trace.borrow_mut().push("recover"),
    );
    let trace = trace.borrow().clone();
    (outcome, trace)
}

/// The race of ravel-stop-eperm-k302, measured on macOS 27.0.1: the result's
/// TERM reaches a group whose leader is inside its exit teardown. Darwin then
/// refuses the group signal with EPERM, although `waitid` reports no exit yet
/// and the leader's own PID takes the signal. Supervision keeps watching, and
/// the report is judged by the approved rule after the observed exit: retained
/// only beside the reap, terminal recovery and confirmed absence.
#[test]
fn a_group_eperm_at_escalation_waits_for_the_exit_and_confirmed_absence() {
    let _serial = serial();
    let (outcome, trace) = escalate_against(Refusal::Group(libc::EPERM), Group::Gone);
    assert_eq!(
        trace,
        [
            "poll",
            "poll",
            "term",
            "poll",
            "kill-group",
            "wait",
            "reaped",
            "recover",
            "confirm"
        ]
    );
    let ended = outcome.expect("a reaped child with a confirmed absent group");
    assert_eq!(ended.end, End::Escalated);
    assert_eq!(ended.group, Group::Gone);
    assert_eq!(
        ended.operation_reports,
        [
            "stop attempt 1 target -4242 signal 15: Operation not permitted (os error 1)",
            "stop attempt 3 target -4242 signal 9: Operation not permitted (os error 1)",
            "stop attempt 4 target -4242 signal 9: Operation not permitted (os error 1)",
        ],
        "the escalation's report is retained in order, beside the reap's"
    );
}

/// The retained escalation report is no confirmation by itself: a group that
/// still answers after the reap fails the launch with every report.
#[test]
fn a_group_eperm_at_escalation_fails_beside_a_present_group() {
    let _serial = serial();
    let (outcome, trace) = escalate_against(
        Refusal::Group(libc::EPERM),
        Group::Present { pgid: FAKE_PGID },
    );
    assert_eq!(trace.last(), Some(&"confirm"));
    let error = outcome.unwrap_err().to_string();
    assert!(error.starts_with("required group stop failed"), "{error}");
    assert!(
        error.contains("stop attempt 1 target -4242 signal 15"),
        "{error}"
    );
}

/// A refusal that the exit race cannot explain still ends supervision at once:
/// EPERM from the direct child, which takes a signal even as a zombie, and any
/// other errno from the group.
#[test]
fn other_stop_refusals_at_escalation_fail_at_once() {
    let _serial = serial();
    for (refusal, report) in [
        (
            Refusal::Pid(libc::EPERM),
            "stop attempt 2 target 4242 signal 15",
        ),
        (
            Refusal::Group(libc::EINVAL),
            "stop attempt 1 target -4242 signal 15",
        ),
    ] {
        let (outcome, trace) = escalate_against(refusal, Group::Gone);
        assert_eq!(
            trace,
            ["poll", "poll", "term", "kill", "wait", "reaped", "recover"],
            "no observation after the refused TERM"
        );
        let error = outcome.unwrap_err().to_string();
        assert!(error.starts_with("required job stop failed"), "{error}");
        assert!(error.contains(report), "{error}");
        assert!(error.contains("and reaped"), "{error}");
    }
}

/// A group EPERM does not excuse a refused stop to the direct child in the same
/// tick, as for a child that executed a program it may not signal.
#[test]
fn a_child_refusal_beside_a_group_eperm_fails_at_once() {
    let _serial = serial();
    let (outcome, trace) = escalate_against(
        Refusal::Attempts(&[(1, libc::EPERM), (2, libc::EPERM)]),
        Group::Gone,
    );
    assert_eq!(
        trace,
        ["poll", "poll", "term", "kill", "wait", "reaped", "recover"]
    );
    let error = outcome.unwrap_err().to_string();
    assert!(error.starts_with("required job stop failed"), "{error}");
    assert!(
        error.contains("stop attempt 1 target -4242 signal 15"),
        "{error}"
    );
    assert!(
        error.contains("stop attempt 2 target 4242 signal 15"),
        "{error}"
    );
}

/// A kept group report goes with each failure that ends the watch later: a
/// fatal refusal at KILL, a failed observation, and the post-KILL deadline.
#[test]
fn a_kept_group_report_goes_with_each_later_failure() {
    let _serial = serial();
    // The TERM's group EPERM is kept. The KILL's refusal by the child is fatal.
    let (outcome, trace) = escalate_with(
        Refusal::Attempts(&[(1, libc::EPERM), (4, libc::EPERM)]),
        Group::Gone,
        vec![Ok(false), Ok(false), Ok(false)],
    );
    assert_eq!(
        trace,
        [
            "poll", "poll", "term", "poll", "kill", "kill", "wait", "reaped", "recover"
        ]
    );
    let error = outcome.unwrap_err().to_string();
    assert!(error.starts_with("required job stop failed"), "{error}");
    assert!(
        error.contains("stop attempt 1 target -4242 signal 15"),
        "{error}"
    );
    assert!(
        error.contains("stop attempt 4 target 4242 signal 9"),
        "{error}"
    );

    // The TERM's group EPERM is kept, then the observation fails.
    let (outcome, _) = escalate_with(
        Refusal::Group(libc::EPERM),
        Group::Gone,
        vec![Ok(false), Ok(false), failed()],
    );
    let error = outcome.unwrap_err().to_string();
    assert!(
        error.contains("cannot wait on the launched child"),
        "{error}"
    );
    assert!(error.contains("injected wait failure"), "{error}");
    assert!(
        error.contains("stop attempt 1 target -4242 signal 15"),
        "{error}"
    );

    // A child that never exits: the kept reports do not extend the bound of
    // two seconds from the first KILL.
    let polls = (0..2000).map(|_| Ok(false)).collect();
    let started = Instant::now();
    let (outcome, _) = escalate_with(Refusal::Group(libc::EPERM), Group::Gone, polls);
    let elapsed = started.elapsed();
    let error = outcome.unwrap_err().to_string();
    assert!(
        error.starts_with("post-SIGKILL deadline expired"),
        "{error}"
    );
    assert!(
        error.contains("stop attempt 1 target -4242 signal 15"),
        "{error}"
    );
    assert!(
        error.contains("stop attempt 3 target -4242 signal 9"),
        "{error}"
    );
    assert!(
        elapsed < ERROR_SETTLEMENT + Duration::from_secs(10),
        "the post-KILL bound held: {elapsed:?}"
    );
}

/// An interactive or noninteractive child's group is forwarded the launcher's
/// own signal and given the kill-grace.
#[test]
fn cancellation_forwards_or_kills_by_mode() {
    let _serial = serial();
    for (mode, expected) in [
        (
            Mode::Interactive,
            &[
                "forward",
                "poll",
                "kill",
                "poll",
                "kill-group",
                "wait",
                "confirm",
            ][..],
        ),
        (
            Mode::Noninteractive,
            &[
                "forward",
                "poll",
                "kill",
                "poll",
                "kill-group",
                "wait",
                "confirm",
            ][..],
        ),
    ] {
        let dir = tempfile::TempDir::new().unwrap();
        let channel = Channel::allocate(dir.path()).unwrap();
        let trace = Trace::default();
        let child = FakeProcess {
            polls: vec![Ok(false), Ok(true)].into(),
            reaps: vec![Ok(ExitStatus::from_raw(libc::SIGKILL))].into(),
            group: Group::Gone,
            trace: trace.clone(),
            signal_at_reap: None,
            stops: Stops::default(),
        };
        // SIGHUP, so the forwarded signal is told apart from the escalation's
        // TERM in the trace. The latch the handler would set; supervision reads it on its first
        // tick. Set directly rather than raised: a real signal would reach
        // whatever else this test process is running.
        INTERRUPTED_BY.store(libc::SIGHUP, Ordering::Relaxed);
        let ended = supervise(
            child,
            Some(&channel),
            NO_WAIT,
            mode,
            &mut |_| {},
            || {},
            |_| {},
        )
        .unwrap();
        assert_eq!(&*trace.borrow(), expected, "{mode:?}");
        assert_eq!(
            ended.end,
            End::Interrupted {
                signal: libc::SIGHUP
            }
        );
    }
}

/// A signal once the terminal is reclaimed belongs to the caller, not to the
/// reaped launch. Moving the last latch read past reclaim breaks this boundary.
#[test]
fn a_signal_after_the_reap_does_not_cancel_the_reaped_launch() {
    let _serial = serial();
    for signalled in [false, true] {
        let dir = tempfile::TempDir::new().unwrap();
        let channel = Channel::allocate(dir.path()).unwrap();
        if signalled {
            std::fs::write(channel.path(), "").unwrap();
        }
        let child = FakeProcess {
            polls: vec![Ok(true)].into(),
            reaps: vec![Ok(ExitStatus::from_raw(0))].into(),
            group: Group::Gone,
            trace: Trace::default(),
            signal_at_reap: None,
            stops: Stops::default(),
        };
        let ended = supervise(
            child,
            Some(&channel),
            NO_WAIT,
            Mode::Interactive,
            &mut |_| {},
            || {},
            |_| {
                INTERRUPTED_BY.store(libc::SIGTERM, Ordering::Relaxed);
            },
        )
        .unwrap();
        let late = take_interrupt();
        assert_eq!(ended.end, End::Exited);
        assert_eq!(ended.signalled, signalled);
        assert_eq!(late, Some(libc::SIGTERM));
    }
}

/// An observer and terminal recovery can be slow without lengthening the
/// measured lifetime of a child already reaped.
#[test]
fn duration_stops_at_the_reap_before_observation_and_recovery() {
    let _serial = serial();
    let dir = tempfile::TempDir::new().unwrap();
    let channel = Channel::allocate(dir.path()).unwrap();
    let child = FakeProcess {
        polls: vec![Ok(true)].into(),
        reaps: vec![Ok(ExitStatus::from_raw(0))].into(),
        group: Group::Gone,
        trace: Trace::default(),
        signal_at_reap: None,
        stops: Stops::default(),
    };
    let delay = Duration::from_millis(150);
    let wall = Instant::now();
    let ended = supervise(
        child,
        Some(&channel),
        NO_WAIT,
        Mode::Interactive,
        &mut |_| std::thread::sleep(delay),
        || {},
        |_| std::thread::sleep(delay),
    )
    .unwrap();
    assert!(wall.elapsed() >= delay * 2);
    assert!(
        ended.elapsed < delay,
        "duration included post-reap work: {:?}",
        ended.elapsed
    );
}

/// The first cancellation remains the run's; another one at the final reap
/// sample is consumed rather than leaking into the caller's late-signal path.
#[test]
fn the_reap_sample_drains_a_second_cancellation() {
    let _serial = serial();
    let dir = tempfile::TempDir::new().unwrap();
    let channel = Channel::allocate(dir.path()).unwrap();
    let child = FakeProcess {
        polls: vec![Ok(true)].into(),
        reaps: vec![Ok(ExitStatus::from_raw(0))].into(),
        group: Group::Gone,
        trace: Trace::default(),
        signal_at_reap: Some(libc::SIGTERM),
        stops: Stops::default(),
    };
    INTERRUPTED_BY.store(libc::SIGHUP, Ordering::Relaxed);
    let ended = supervise(
        child,
        Some(&channel),
        NO_WAIT,
        Mode::Interactive,
        &mut |_| {},
        || {},
        |_| {},
    )
    .unwrap();
    let late = take_interrupt();
    assert_eq!(
        ended.end,
        End::Interrupted {
            signal: libc::SIGHUP
        }
    );
    assert_eq!(late, None);
}

// Dropping the failure observation, skipping its final sample, or emitting a
// Reaped/group fact without a wait receipt violates these fixed expectations.
#[test]
fn cancellation_survives_observation_failure_with_confirmed_reap() {
    check_wait_failure(true, false);
}

#[test]
fn cancellation_survives_observation_failure_with_failed_reap() {
    check_wait_failure(true, true);
}

#[test]
fn cancellation_survives_final_reap_failure() {
    check_wait_failure(false, true);
}

fn check_wait_failure(poll_failed: bool, reap_failed: bool) {
    let _serial = serial();
    for (early, at_reap, expected) in [
        (Some(libc::SIGHUP), None, Some(libc::SIGHUP)),
        (Some(libc::SIGHUP), Some(libc::SIGTERM), Some(libc::SIGHUP)),
        (None, Some(libc::SIGTERM), Some(libc::SIGTERM)),
        (None, None, None),
    ] {
        let trace = Trace::default();
        let status = ExitStatus::from_raw(0);
        let child = FakeProcess {
            polls: vec![if poll_failed {
                Err(std::io::Error::other("original observation failure"))
            } else {
                Ok(true)
            }]
            .into(),
            reaps: vec![if reap_failed {
                Err(std::io::Error::other("distinct emergency wait failure"))
            } else {
                Ok(status)
            }]
            .into(),
            group: Group::Gone,
            trace: trace.clone(),
            signal_at_reap: at_reap,
            stops: Stops::default(),
        };
        INTERRUPTED_BY.store(early.unwrap_or(0), Ordering::Relaxed);
        let recovered = RefCell::new(None);
        let error = supervise(
            child,
            None,
            NO_WAIT,
            Mode::Interactive,
            &mut |_| {
                assert_eq!(take_interrupt(), None, "sample before observer");
                trace.borrow_mut().push("reaped");
            },
            || {},
            |receipt| {
                assert_eq!(take_interrupt(), None, "sample before recovery");
                trace.borrow_mut().push("recover");
                *recovered.borrow_mut() = Some(receipt);
            },
        )
        .unwrap_err();
        let late = take_interrupt(); // Keep the latch clean even on assertion failure.
        assert_eq!(
            error.consumed_signal(),
            expected,
            "poll={poll_failed}, reap={reap_failed}"
        );
        assert_eq!(late, None);
        let message = error.to_string();
        if poll_failed {
            assert!(message.contains("original observation failure"));
        }
        if reap_failed {
            assert!(message.contains("distinct emergency wait failure"));
        }
        assert_eq!(
            recovered.into_inner(),
            Some((!reap_failed).then_some(status))
        );
        let mut want = Vec::new();
        if early.is_some() {
            want.push("forward");
        }
        want.extend([
            "poll",
            if poll_failed { "kill" } else { "kill-group" },
            "wait",
        ]);
        if !reap_failed {
            want.push("reaped");
        }
        want.push("recover");
        assert_eq!(*trace.borrow(), want);
        assert_eq!(error.raw_os_error(), None);
        eprintln!(
            "checked poll_failed={poll_failed} reap_failed={reap_failed} early={early:?} at_reap={at_reap:?} retained={expected:?}; diagnostics, receipt, ordering and drained latch asserted"
        );
    }
}

#[test]
fn cancellation_survives_reported_recovery_failure_and_keeps_late_signal_separate() {
    let _serial = serial();
    for (failed_watch, failed_reap) in [(false, false), (true, false), (true, true), (false, true)]
    {
        for early in [None, Some(libc::SIGHUP)] {
            let trace = Trace::default();
            let child = FakeProcess {
                polls: vec![if failed_watch { failed() } else { Ok(true) }].into(),
                reaps: vec![if failed_reap {
                    failed()
                } else {
                    Ok(ExitStatus::from_raw(0))
                }]
                .into(),
                group: Group::Gone,
                trace: trace.clone(),
                signal_at_reap: None,
                stops: Stops::default(),
            };
            INTERRUPTED_BY.store(early.unwrap_or(0), Ordering::Relaxed);
            let error = supervise_reported(
                child,
                None,
                NO_WAIT,
                Mode::Interactive,
                &mut |_| trace.borrow_mut().push("reaped"),
                || {},
                |status| {
                    assert_eq!(status.is_some(), !failed_reap);
                    trace.borrow_mut().push("recover");
                    INTERRUPTED_BY.store(libc::SIGTERM, Ordering::Relaxed);
                    Err(LaunchError::new("reported terminal recovery failure"))
                },
            )
            .unwrap_err();
            let late = take_interrupt();
            assert_eq!(error.consumed_signal(), early);
            assert_eq!(late, Some(libc::SIGTERM));
            assert!(
                error
                    .to_string()
                    .contains("reported terminal recovery failure")
            );
            if failed_watch {
                assert!(error.to_string().contains("injected wait failure"));
            }
            assert!(!trace.borrow().contains(&"confirm"));
            assert_eq!(trace.borrow().last(), Some(&"recover"));
            assert_eq!(trace.borrow().contains(&"reaped"), !failed_reap);
            eprintln!(
                "checked reported recovery error: observation_failed={failed_watch} reap_failed={failed_reap} early={early:?}; retained observation and late TERM asserted"
            );
        }
    }
}

#[test]
fn error_annotations_preserve_first_cancellation_primary_errno_and_diagnostics() {
    let error = LaunchError::new("original spawn failure")
        .with_errno(Some(libc::ENOENT))
        .with_consumed_signal(Some(libc::SIGHUP))
        .with_diagnostic("secondary recovery failure")
        .with_consumed_signal(Some(libc::SIGTERM));
    assert_eq!(error.consumed_signal(), Some(libc::SIGHUP));
    assert_eq!(error.raw_os_error(), Some(libc::ENOENT));
    assert!(error.to_string().contains("original spawn failure"));
    assert!(format!("{error:?}").contains("secondary recovery failure"));
    let unrelated = LaunchError::new("ordinary failure").with_errno(Some(libc::EACCES));
    assert_eq!(unrelated.consumed_signal(), None);
}
