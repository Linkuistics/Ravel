//! The incumbent runner judged by the repaired instruments' own claims.
//!
//! Compiled only into the frozen incumbent subject (`scripts/native-incumbent.sh`),
//! as a `cfg(test)` module of its `keyed-launch` crate. A port of the E05 and E07
//! sections of the repaired `run/native_tests.rs`: the same fixture roles, the same
//! controller, the same facts and the same named claims, with the repaired
//! settlement internals it does not have left out. Row identities come from the
//! frozen manifest, never from a list here, and each verdict is filed under the
//! subject `baseline` through the same write-once recorder.
//!
//! A hook here observes or places a call at a point of the incumbent's own flow;
//! none adds a mechanism it lacks. The one place a report is substituted is the
//! observation error E07's runner-wait-error boundary names, and only after the
//! physical `waitid` has run. A fact the instrument could not observe (a barrier
//! never reached, a controller that did not settle) fails the test and files no
//! record: it is an instrument fault, never the incumbent failing a claim.
use super::*;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicU32, AtomicU8};

static HANDLER_RUNS: AtomicU32 = AtomicU32::new(0);
static SEEN_RUNS: AtomicU32 = AtomicU32::new(0);
static STAGE: AtomicU8 = AtomicU8::new(0);
static ARM_OBSERVE_EIO: AtomicBool = AtomicBool::new(false);
static ERROR_VARIANT: AtomicBool = AtomicBool::new(false);
static ACTIVE: AtomicBool = AtomicBool::new(false);
static FIRST_WINS: AtomicBool = AtomicBool::new(false);

/// Async-signal-safe: one atomic increment, no allocation.
pub(super) fn handler_ran() {
    HANDLER_RUNS.fetch_add(1, Ordering::SeqCst);
}

/// The harness control that gives the incumbent the repaired first-wins latch.
pub(super) fn first_wins() -> bool {
    FIRST_WINS.load(Ordering::SeqCst)
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

// The reports a row substitutes at the two operations the incumbent performs
// to learn that its child ended: the `waitid(WNOWAIT)` observation and the
// reap. A substituted observation report follows the physical `waitid`; a
// substituted reap report *omits* the physical wait, so the reservation stays
// with the fixture, which reaps it afterwards (never the product).
static OBSERVE_ERRNO: AtomicI32 = AtomicI32::new(0);
static OBSERVE_ONCE: AtomicBool = AtomicBool::new(false);
static REAP_ERRNO: AtomicI32 = AtomicI32::new(0);
static REAP_ONCE: AtomicBool = AtomicBool::new(false);
/// A reported ECHILD says the reservation is lost; any numeric signal the
/// incumbent still sends after that is counted.
static REPORTED_ECHILD: AtomicBool = AtomicBool::new(false);
static SIGNALS_AFTER_REPORT: AtomicU32 = AtomicU32::new(0);
static PHYSICAL_REAPS: AtomicU32 = AtomicU32::new(0);
static OMITTED_REAP_PID: AtomicI32 = AtomicI32::new(0);

/// The runner's observation result. E07's error variant arms one substituted
/// EIO once the first signal's consumption is acknowledged; S02 and S04 name
/// theirs. The physical `waitid` has already run.
pub(super) fn observed(result: libc::c_int) -> libc::c_int {
    if ARM_OBSERVE_EIO.swap(false, Ordering::SeqCst) {
        set_errno(libc::EIO);
        return -1;
    }
    let errno = OBSERVE_ERRNO.load(Ordering::SeqCst);
    if errno != 0 {
        if OBSERVE_ONCE.load(Ordering::SeqCst) {
            OBSERVE_ERRNO.store(0, Ordering::SeqCst);
        }
        if errno == libc::ECHILD {
            REPORTED_ECHILD.store(true, Ordering::SeqCst);
        }
        set_errno(errno);
        return -1;
    }
    result
}

/// The reap's substituted report, if one is armed. The physical wait is not
/// performed: the pid is remembered for the fixture's own reap.
pub(super) fn reap_report(pid: libc::pid_t) -> Option<std::io::Error> {
    let errno = REAP_ERRNO.load(Ordering::SeqCst);
    if errno == 0 {
        return None;
    }
    if REAP_ONCE.load(Ordering::SeqCst) {
        REAP_ERRNO.store(0, Ordering::SeqCst);
    }
    if errno == libc::ECHILD {
        REPORTED_ECHILD.store(true, Ordering::SeqCst);
    }
    OMITTED_REAP_PID.store(pid, Ordering::SeqCst);
    Some(std::io::Error::from_raw_os_error(errno))
}

pub(super) fn physical_reap(result: &std::io::Result<ExitStatus>) {
    if result.is_ok() {
        PHYSICAL_REAPS.fetch_add(1, Ordering::SeqCst);
    }
}
pub(super) fn physical_reaps() -> u32 {
    PHYSICAL_REAPS.load(Ordering::SeqCst)
}
pub(super) fn signal_attempt() {
    if REPORTED_ECHILD.load(Ordering::SeqCst) {
        SIGNALS_AFTER_REPORT.fetch_add(1, Ordering::SeqCst);
    }
}

/// S04's unknown status, reached the incumbent's own way: observation fails
/// once, SIGKILL is sent, and the reap then fails once.
pub(super) fn unknown_status(on: bool) {
    let errno = if on { libc::EIO } else { 0 };
    OBSERVE_ERRNO.store(errno, Ordering::SeqCst);
    OBSERVE_ONCE.store(true, Ordering::SeqCst);
    REAP_ERRNO.store(errno, Ordering::SeqCst);
    REAP_ONCE.store(true, Ordering::SeqCst);
}

/// The fixture's own bounded reap of a child whose wait the product never
/// performed. `Ok` when there was none.
pub(super) fn fixture_reap() -> std::io::Result<()> {
    let pid = OMITTED_REAP_PID.swap(0, Ordering::SeqCst);
    if pid == 0 {
        return Ok(());
    }
    let by = Instant::now() + Duration::from_secs(2);
    loop {
        let mut status = 0;
        // SAFETY: waitpid on a child of this process the product never waited.
        let result = unsafe { libc::waitpid(pid, &mut status, libc::WNOHANG) };
        if result == pid {
            return Ok(());
        }
        if result == -1 {
            return Err(std::io::Error::last_os_error());
        }
        if Instant::now() >= by {
            return Err(std::io::Error::new(
                std::io::ErrorKind::TimedOut,
                "fixture reap unconfirmed",
            ));
        }
        std::thread::sleep(Duration::from_millis(1));
    }
}

fn field(facts: &str, name: &str) -> Option<i32> {
    facts
        .split_whitespace()
        .find_map(|word| word.strip_prefix(&format!("{name}=")))
        .and_then(|value| value.parse().ok())
}

/// Every manifest row identity of one family, in manifest order.
fn rows(family: &str) -> Vec<String> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/native-entry-cases.json");
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let prefix = format!("{family}__");
    text.split("\"id\": \"")
        .skip(1)
        .filter_map(|rest| rest.split('"').next())
        .filter(|id| id.starts_with(&prefix))
        .map(str::to_owned)
        .collect()
}

/// The value of `name` in a row identity such as `E07__signals=A__boundary=B`.
fn parameter<'a>(id: &'a str, name: &str) -> &'a str {
    id.split("__")
        .find_map(|part| part.strip_prefix(&format!("{name}=")))
        .unwrap_or_else(|| panic!("{id} names no {name}"))
}

fn signal(name: &str) -> i32 {
    match name {
        "INT" => libc::SIGINT,
        "TERM" => libc::SIGTERM,
        "HUP" => libc::SIGHUP,
        other => panic!("manifest signal {other} is not one the instruments name"),
    }
}

fn baseline(id: &str, conforms: bool, note: &str) {
    crate::native_barrier::row(id, "baseline", conforms, note);
}

// ---- E07: first consumed cancellation survives later signals ---------------
//
// The controller acknowledges each stage before the next signal is sent, so no
// row claims an arrival order between simultaneously pending standard signals.

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
/// taken. The first signal's latch is acknowledged, then a second is sent.
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
        // reports an error so the runner ends through its error path.
        ARM_OBSERVE_EIO.store(true, Ordering::SeqCst);
    }
}

const E07_ROLE: &str = "run::baseline_tests::native_baseline_role_e07";

#[test]
#[ignore = "private E07 runner role; run native-incumbent.sh test"]
fn native_baseline_role_e07() {
    let Some(variant) = std::env::var_os("K45_E07_VARIANT") else {
        return;
    };
    ERROR_VARIANT.store(variant == "error", Ordering::SeqCst);
    FIRST_WINS.store(
        std::env::var_os("K45_E07_LATCH").is_some_and(|mode| mode == "first-wins"),
        Ordering::SeqCst,
    );
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
    let path = std::path::PathBuf::from(std::env::var_os("K45_E07_OUT").unwrap());
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
    /// The repaired instrument's own claim, unchanged.
    fn conforms(&self, first: i32, error: bool) -> bool {
        let shape = if error { "error " } else { "ended " };
        self.clean
            && self.first_ack_latch == Some(first)
            && self.second_ack_latch == Some(first)
            && self.outcome.starts_with(shape)
            && self.named() == Some(first)
    }
}

fn e07_row(first: i32, second: i32, error: bool, latch: &str) -> E07 {
    use crate::native_barrier::{Controller, Limits};
    let directory = tempfile::tempdir().unwrap();
    let channels = directory.path().join("barrier");
    std::fs::create_dir(&channels).unwrap();
    let out = directory.path().join("outcome");
    let mut controller = Controller::new(&channels, Limits::FROZEN).unwrap();
    let mut command = Command::new(std::env::current_exe().unwrap());
    command
        .args(["--exact", E07_ROLE, "--ignored", "--nocapture"])
        .env("K45_E07_VARIANT", if error { "error" } else { "active" })
        .env("K45_E07_LATCH", latch)
        .env("K45_E07_OUT", &out)
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
    eprintln!("native baseline E07 first={first} second={second} error={error} latch={latch}: {row:?} {settlement:?}");
    row
}

#[test]
#[ignore = "private real-process fixture; run native-incumbent.sh test"]
fn native_baseline_e07_first_consumed_cancellation_survives_every_later_delivery() {
    let rows = rows("E07");
    assert_eq!(rows.len(), 4);
    for id in rows {
        let (first, second) = match parameter(&id, "signals").split_once("-then-") {
            Some((first, second)) => (signal(first), signal(second)),
            None => panic!("{id}: signals are not an ordered pair"),
        };
        let error = match parameter(&id, "boundary") {
            "runner-active" => false,
            "runner-wait-error" => true,
            other => panic!("E07 boundary {other} has no instrument boundary"),
        };
        let seen = e07_row(first, second, error, "incumbent");
        // Instrument integrity is not a claim: a controller that did not
        // settle, or a barrier that was not reached, files no record.
        assert!(
            seen.clean && seen.reported.is_empty(),
            "{id}: the instrument did not observe the boundary: {seen:?}"
        );
        assert!(
            seen.consumed_ack_latch.is_some(),
            "{id}: the later delivery was not acknowledged: {seen:?}"
        );
        // The repaired claim, unchanged: the first signal's latch survives the
        // acknowledged second delivery, and the launch names the first.
        let conforms = seen.conforms(first, error) && seen.consumed_ack_latch == Some(second);
        baseline(&id, conforms, &format!("{seen:?}"));
    }
}

// ---- E05: direct INT versus terminal VINTR, before and after handoff --------
//
// The role is a launcher on a real PTY of its own (session and group leader
// with the slave as its controlling terminal). The controller owns it, the
// master and the acknowledgements. A direct INT is `kill(launcher)`; a terminal
// INT is the VINTR byte written to the master, which the line discipline
// delivers to whichever group is then foreground.

static E05_ACTIVE: AtomicBool = AtomicBool::new(false);
static E05_BEFORE: AtomicBool = AtomicBool::new(false);
static E05_INVENT: AtomicBool = AtomicBool::new(false);
static E05_CHANNEL: std::sync::OnceLock<std::path::PathBuf> = std::sync::OnceLock::new();
static E05_CHILD_PID: std::sync::OnceLock<std::path::PathBuf> = std::sync::OnceLock::new();

/// The harness control that reports the child's own terminal-INT death as the
/// launcher's cancellation.
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
         published={} latch={} runs={}",
        i32::from(published),
        INTERRUPTED_BY.load(Ordering::SeqCst),
        HANDLER_RUNS.load(Ordering::SeqCst)
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

const E05_ROLE: &str = "run::baseline_tests::native_baseline_role_e05";

#[test]
#[ignore = "private E05 launcher role on its own PTY; run native-incumbent.sh test"]
fn native_baseline_role_e05() {
    let Some(boundary) = std::env::var_os("K45_E05_BOUNDARY") else {
        return;
    };
    let before = boundary == "before";
    E05_BEFORE.store(before, Ordering::SeqCst);
    E05_INVENT.store(
        std::env::var_os("K45_E05_CONTROL").is_some_and(|mode| mode == "invent"),
        Ordering::SeqCst,
    );
    E05_ACTIVE.store(true, Ordering::SeqCst);
    let directory = std::path::PathBuf::from(std::env::var_os("K45_E05_DIR").unwrap());
    let channel = crate::Channel::allocate(&directory).unwrap();
    let _ = E05_CHANNEL.set(channel.path().to_owned());
    let _ = E05_CHILD_PID.set(directory.join("child.pid"));
    let terminal = Terminal::open().expect("the role must have a controlling terminal");
    let own = own_group();
    // The child records its identity, publishes through the channel and then
    // waits with the default INT disposition it was handed.
    let script = r#"echo $$ > "$K45_E05_DIR/child.pid"; printf done > "$K45_E05_CHANNEL"; exec /bin/sleep 600"#;
    let argv = crate::Argv::new("/bin/sh".into(), vec!["-c".into(), script.into()]);
    let entry = EntrySignals::new([], []);
    let grant = [(OsStr::new("K45_E05_DIR"), directory.as_os_str())];
    let mut events = Vec::new();
    // The repaired role holds its barrier in the start check, which runs right
    // after Started. The incumbent's own post-spawn point is the Started
    // notification, so the barrier is held there: same place in the flow.
    let result = run_observed(
        Launch {
            argv: &argv,
            channel: Some((&channel, "K45_E05_CHANNEL")),
            scrub: &[],
            grant: &grant,
            transparent: Some(&entry),
            cwd: None,
            escalation: Escalation {
                grace: Duration::from_secs(10),
                kill_grace: Duration::from_secs(5),
            },
        },
        &mut |event| {
            events.push(event);
            if event == LaunchEvent::Started && !before {
                e05_exchange("after-handoff");
            }
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
    std::fs::write(std::env::var_os("K45_E05_OUT").unwrap(), line).unwrap();
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
    /// What the contract requires of one row, as separate named axes: the
    /// repaired instrument's own judgement, unchanged.
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

fn e05_row(delivery: &str, boundary: &str, control: &str) -> E05 {
    use crate::native_barrier::{Controller, Limits};
    use std::io::Write as _;
    use std::os::fd::{AsRawFd as _, FromRawFd as _};
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
        .env("K45_E05_BOUNDARY", boundary)
        .env("K45_E05_CONTROL", control)
        .env("K45_E05_DIR", directory.path())
        .env("K45_E05_OUT", &out)
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
        // The delivery is asynchronous. Where the launcher is the target its
        // handler's own run counter is the acknowledgement, polled through
        // acknowledged probes (never a sleep) until it is seen or the bounded
        // window ends; an absence is then a fact, not an early read.
        let by_launcher = delivery == "direct" || boundary == "before";
        let window = Instant::now() + Duration::from_secs(3);
        loop {
            let facts = controller.probe()?;
            let seen = facts_number(Some(&facts), "runs").is_some_and(|runs| runs > 0);
            row.ack = Some(facts);
            if !by_launcher || seen || Instant::now() >= window {
                break;
            }
        }
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
    eprintln!("native baseline E05 {delivery}/{boundary} control={control}: {row:?} {settlement:?}");
    row
}

fn e05_parameters(id: &str) -> (&'static str, &'static str) {
    let delivery = match parameter(id, "delivery") {
        "direct-INT" => "direct",
        "PTY-VINTR" => "pty",
        other => panic!("E05 delivery {other} has no instrument delivery"),
    };
    let boundary = match parameter(id, "boundary") {
        "before-handoff" => "before",
        "after-handoff" => "after",
        other => panic!("E05 boundary {other} has no instrument boundary"),
    };
    (delivery, boundary)
}

#[test]
#[ignore = "private real-PTY fixture; run native-incumbent.sh test"]
fn native_baseline_e05_direct_and_terminal_int_before_and_after_handoff() {
    let rows = rows("E05");
    assert_eq!(rows.len(), 4);
    for id in rows {
        let (delivery, boundary) = e05_parameters(&id);
        let seen = e05_row(delivery, boundary, "none");
        let axes = seen.axes(delivery, boundary);
        // The controller axis is the instrument's own integrity, not a claim.
        assert!(
            axes.controller && axes.target,
            "{id}: the boundary was not observed: {axes:?} {seen:?}"
        );
        baseline(&id, axes.conforms(), &format!("{axes:?} {seen:?}"));
    }
}

// ---- harness controls -------------------------------------------------------
//
// A baseline reading is trustworthy only if the same instrument is seen to read
// the other way when the incumbent is made to differ in the one way the row
// names. These file no row record.

#[test]
#[ignore = "private real-process fixture; run native-incumbent.sh test"]
fn native_baseline_control_e07_a_first_wins_latch_is_seen_to_hold() {
    for (first, second, error) in [
        (libc::SIGTERM, libc::SIGHUP, false),
        (libc::SIGHUP, libc::SIGINT, true),
    ] {
        let incumbent = e07_row(first, second, error, "incumbent");
        assert!(incumbent.clean, "{incumbent:?}");
        assert!(
            !incumbent.conforms(first, error),
            "the incumbent's latch was expected to be overwritten: {incumbent:?}"
        );
        let held = e07_row(first, second, error, "first-wins");
        assert!(held.clean, "{held:?}");
        assert!(
            held.conforms(first, error) && held.consumed_ack_latch == Some(second),
            "the same instrument must read 'holds' when the first signal wins: {held:?}"
        );
    }
}

#[test]
#[ignore = "private real-PTY fixture; run native-incumbent.sh test"]
fn native_baseline_control_e05_an_invented_cancellation_is_detected() {
    let honest = e05_row("pty", "after", "none").axes("pty", "after");
    assert!(honest.controller && honest.target, "{honest:?}");
    let invented = e05_row("pty", "after", "invent");
    let axes = invented.axes("pty", "after");
    assert!(
        axes.controller && axes.target && axes.routed,
        "{axes:?} {invented:?}"
    );
    assert!(
        !axes.attributed && invented.interrupted_by_launcher() && !axes.conforms(),
        "the instrument must fail a launcher that reports the child's death as its own cancellation: {axes:?} {invented:?}"
    );
}

// ---- S02: observation and wait uncertainty -----------------------------------
//
// The repaired rows reach these faults through a failed Started check and its
// bounded settlement. The incumbent has neither, but it performs both
// operations: it observes its child's exit with `waitid(WNOWAIT)` and reaps it
// with a blocking wait, and each has an error path (an observation failure
// sends SIGKILL and reaps; a reap failure ends the launch with no status).
// The fixture substitutes the report at those operations and judges what the
// incumbent then does by the repaired row's named claims, as follows.
//
// Claims whose fact the incumbent exposes are judged as the repaired row
// states them, with a wording predicate restated by meaning where the repaired
// text is its own (the failed operation and its errno are named). Not judged,
// because the fact cannot exist: `start-check-source-retained` (no start check)
// and `ownership-released` (the incumbent has no reservation state to read).
// The always-running row is not hosted at all: no error path is entered, so
// there is no settlement to observe (see the plan's reason).

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum S02 {
    ObserveEio,
    ObserveEintr,
    ObserveLost,
    WaitEio,
    WaitEintr,
    WaitLost,
}

#[derive(Debug)]
struct Settled {
    text: String,
    events: Vec<LaunchEvent>,
    elapsed: Duration,
    physical_reaps: u32,
    signals_after_report: u32,
    spawn_errno: Option<i32>,
}

fn s02_settle(kind: S02) -> Settled {
    let (observe, reap, once, child) = match kind {
        S02::ObserveEio => (libc::EIO, 0, true, ("/bin/sleep", "20")),
        S02::ObserveEintr => (libc::EINTR, 0, false, ("/bin/sleep", "20")),
        S02::ObserveLost => (libc::ECHILD, 0, true, ("/bin/sleep", "20")),
        S02::WaitEio => (0, libc::EIO, true, ("/usr/bin/true", "")),
        S02::WaitEintr => (0, libc::EINTR, false, ("/usr/bin/true", "")),
        S02::WaitLost => (0, libc::ECHILD, true, ("/usr/bin/true", "")),
    };
    OBSERVE_ERRNO.store(observe, Ordering::SeqCst);
    OBSERVE_ONCE.store(once, Ordering::SeqCst);
    REAP_ERRNO.store(reap, Ordering::SeqCst);
    REAP_ONCE.store(once, Ordering::SeqCst);
    let args = if child.1.is_empty() {
        vec![]
    } else {
        vec![child.1.into()]
    };
    let argv = crate::Argv::new(child.0.into(), args);
    let mut events = Vec::new();
    let at = Instant::now();
    let result = run_observed(
        Launch {
            argv: &argv,
            channel: None,
            scrub: &[],
            grant: &[],
            transparent: None,
            cwd: None,
            escalation: Escalation {
                grace: Duration::ZERO,
                kill_grace: Duration::ZERO,
            },
        },
        &mut |event| events.push(event),
    );
    let elapsed = at.elapsed();
    OBSERVE_ERRNO.store(0, Ordering::SeqCst);
    REAP_ERRNO.store(0, Ordering::SeqCst);
    // The fixture alone holds a reservation the product's reported failure
    // left: reap it before judging, so no row leaves a child behind.
    fixture_reap().expect("the fixture's own reap must settle on every row");
    let (text, spawn_errno) = match &result {
        Ok(ended) => (format!("ended {:?}", ended.end), None),
        Err(error) => (error.to_string(), error.raw_os_error()),
    };
    Settled {
        text,
        events,
        elapsed,
        physical_reaps: physical_reaps(),
        signals_after_report: SIGNALS_AFTER_REPORT.load(Ordering::SeqCst),
        spawn_errno,
    }
}

impl Settled {
    /// Every claim this row does not meet, named as the repaired row names it.
    fn violations(&self, kind: S02) -> Vec<String> {
        let mut named = Vec::new();
        let mut claim = |held: bool, name: &str| {
            if !held {
                named.push(name.to_owned());
            }
        };
        claim(
            self.spawn_errno.is_none(),
            "spawn-errno-accessor-spawn-only",
        );
        let reaped = self
            .events
            .iter()
            .filter(|&&event| event == LaunchEvent::Reaped)
            .count();
        claim(
            reaped == self.physical_reaps as usize,
            "reaped-only-on-actual-reap",
        );
        claim(self.signals_after_report == 0, "no-signal-after-release");
        let started_reaped = self.events == [LaunchEvent::Started, LaunchEvent::Reaped];
        let bounded = self.elapsed >= Duration::from_secs(2)
            && self.elapsed < Duration::from_millis(2500);
        match kind {
            S02::ObserveEio => {
                claim(
                    self.text.contains("cannot wait on the launched child")
                        && self.text.contains("Input/output error"),
                    "observation-error-reported",
                );
                claim(started_reaped, "reaped-after-recovered-error");
            }
            S02::WaitEio => {
                claim(
                    self.text.contains("cannot reap the exited child")
                        && self.text.contains("Input/output error"),
                    "wait-error-reported",
                );
                claim(started_reaped, "reaped-after-recovered-error");
            }
            S02::ObserveEintr | S02::WaitEintr => {
                claim(bounded, "absolute-deadline");
                claim(self.text.contains("deadline"), "deadline-expiry-reported");
                claim(self.text.contains("EINTR retries"), "retries-reported");
            }
            S02::ObserveLost | S02::WaitLost => {
                claim(
                    self.text.contains("No child processes"),
                    "ownership-loss-errno-reported",
                );
                claim(
                    self.elapsed < Duration::from_secs(1),
                    "loss-does-not-wait-out-deadline",
                );
            }
        }
        named
    }
}

fn s02_row(id: &str, kind: S02) {
    assert!(
        rows("S02").iter().any(|row| row == id),
        "{id} is not a frozen manifest row"
    );
    let seen = s02_settle(kind);
    // Instrument integrity: the substituted report must have been the one the
    // row names, or the row did not exercise its operation.
    let reported = match kind {
        S02::ObserveEio | S02::ObserveEintr | S02::ObserveLost => {
            seen.text.contains("cannot wait on the launched child")
        }
        S02::WaitEio | S02::WaitEintr | S02::WaitLost => {
            seen.text.contains("cannot reap the exited child")
        }
    };
    assert!(
        reported,
        "{id}: the substituted report did not reach the incumbent's error path: {seen:?}"
    );
    let violated = seen.violations(kind);
    baseline(
        id,
        violated.is_empty(),
        &format!("violated {violated:?}; {seen:?}"),
    );
}

macro_rules! s02_test {
    ($name:ident, $id:literal, $kind:expr) => {
        #[test]
        #[ignore = "private real-process fixture; run native-incumbent.sh test"]
        fn $name() {
            s02_row(concat!("S02__operation=", $id), $kind);
        }
    };
}
s02_test!(native_baseline_s02_observation_eio_once, "observation-EIO-once", S02::ObserveEio);
s02_test!(
    native_baseline_s02_observation_eintr_until_deadline,
    "observation-EINTR-until-deadline",
    S02::ObserveEintr
);
s02_test!(native_baseline_s02_observation_echild, "observation-ECHILD", S02::ObserveLost);
s02_test!(native_baseline_s02_wait_eio_once, "wait-EIO-once", S02::WaitEio);
s02_test!(
    native_baseline_s02_wait_eintr_until_deadline,
    "wait-EINTR-until-deadline",
    S02::WaitEintr
);
s02_test!(native_baseline_s02_wait_echild, "wait-ECHILD", S02::WaitLost);
