//! Private executable-entry fixtures: real policy worker, fake managed process.
use std::cell::{Cell, RefCell};
use std::fs::{self, File};
use std::io::{self, Write as _};
use std::os::fd::AsRawFd as _;
use std::os::unix::process::{CommandExt as _, ExitStatusExt as _};
use std::path::PathBuf;
use std::process::{Command, Stdio};

use clap::Parser as _;
use keyed_launch::{Ended, EntrySignals, LaunchEvent};

use crate::native_barrier::{Controller, Limits, Settlement};
use serde_json::{json, Value};
use std::time::{Duration, Instant};

const ROLE: &str = "native_entry_tests::native_entry_role";
const RESULT: &str = "native 🪶\nresult";
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Broken {
    None,
    Capture,
    Reinstatement,
    BlockedPending,
    WorkerWatching,
    Join(Join),
}
/// The cancellation joins E06 removes one at a time. Fixed enumeration only.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Join {
    None,
    Selection,
    Front,
    Outcome,
    Late,
}
thread_local! {
    static CASE: RefCell<Option<Value>> = const { RefCell::new(None) };
    static BROKEN: Cell<Broken> = const { Cell::new(Broken::None) };
    static LAST_RAW_CHLD_FLAGS: Cell<i32> = const { Cell::new(0) };
}
fn active() -> bool {
    CASE.with(|c| c.borrow().is_some())
}
fn family() -> String {
    CASE.with(|c| {
        c.borrow().as_ref().unwrap()["family"]
            .as_str()
            .unwrap()
            .to_owned()
    })
}
fn root() -> PathBuf {
    PathBuf::from(std::env::var_os("K28_ENTRY_ROOT").unwrap())
}
fn receipt(name: &str, value: Value) {
    if active() {
        fs::write(root().join(name), value.to_string()).unwrap();
    }
}
pub(crate) fn broken_join() -> Join {
    match BROKEN.get() {
        Broken::Join(join) => join,
        _ => Join::None,
    }
}
pub(crate) fn broken_blocked_pending() -> bool {
    BROKEN.get() == Broken::BlockedPending
}
pub(crate) fn broken_reinstatement() -> bool {
    BROKEN.get() == Broken::Reinstatement
}
pub(crate) fn skip_worker_repair() -> bool {
    BROKEN.get() == Broken::WorkerWatching
}
pub(crate) fn entry_override(real: EntrySignals) -> EntrySignals {
    if BROKEN.get() == Broken::Capture {
        let signal = CASE.with(|c| c.borrow().as_ref().unwrap()["signal"].as_i64().unwrap() as i32);
        crate::signal_state::native_wrong_capture(signal)
    } else {
        real
    }
}
fn private_no_child_wait() {
    let mut action = unsafe { std::mem::zeroed::<libc::sigaction>() };
    action.sa_sigaction = libc::SIG_DFL;
    action.sa_flags = libc::SA_NOCLDWAIT;
    assert_eq!(unsafe { libc::sigemptyset(&mut action.sa_mask) }, 0);
    assert_eq!(
        unsafe { libc::sigaction(libc::SIGCHLD, &action, std::ptr::null_mut()) },
        0
    );
    let mut actual = unsafe { std::mem::zeroed::<libc::sigaction>() };
    assert_eq!(
        unsafe { libc::sigaction(libc::SIGCHLD, std::ptr::null(), &mut actual) },
        0
    );
    receipt(
        "watching-precondition.json",
        json!({"privatePostEntry":true,"action":actual.sa_sigaction,"flags":actual.sa_flags}),
    );
}
pub(crate) fn before_managed() {
    if active() && family() == "E04-managed" {
        private_no_child_wait();
    }
}
pub(crate) fn managed_event(event: LaunchEvent) {
    if !active() {
        return;
    }
    receipt(
        match event {
            LaunchEvent::Started => "managed-started.json",
            LaunchEvent::Reaped => "managed-reaped.json",
        },
        json!({"actualEvent":format!("{event:?}")}),
    );
}
pub(crate) fn managed_ending(ending: &Ended) {
    receipt(
        "managed-ending.json",
        json!({"rawStatus":ending.status.into_raw(),"end":format!("{:?}",ending.end),"group":format!("{:?}",ending.group),"elapsedNanos":ending.elapsed.as_nanos().to_string(),"operationReports":ending.operation_reports}),
    );
}
pub(crate) fn worker_wait(pid: u32, status: &io::Result<std::process::ExitStatus>) {
    receipt(
        "worker-wait.json",
        match status {
            Ok(status) => {
                json!({"pid":pid,"actualConfirmedReap":true,"rawStatus":status.into_raw()})
            }
            Err(error) => {
                json!({"pid":pid,"actualConfirmedReap":false,"error":error.to_string(),"errno":error.raw_os_error()})
            }
        },
    );
}

fn blocked_bits() -> u64 {
    let mut current = unsafe { std::mem::zeroed::<libc::sigset_t>() };
    unsafe { libc::pthread_sigmask(libc::SIG_BLOCK, std::ptr::null(), &mut current) };
    set_bits(&current)
}
fn pending_bits() -> u64 {
    let mut pending = unsafe { std::mem::zeroed::<libc::sigset_t>() };
    unsafe { libc::sigpending(&mut pending) };
    set_bits(&pending)
}
fn set_bits(set: &libc::sigset_t) -> u64 {
    (1..=31)
        .filter(|&signal| unsafe { libc::sigismember(set, signal) } == 1)
        .fold(0, |bits, signal| bits | bit(signal))
}
/// Current owner/mask/pending/consumed facts. Read-only; nothing here changes
/// signal state, so a probe never becomes the thing it observes.
fn facts(boundary: &str) -> String {
    let at = root();
    let owner = |signal| crate::cancellation::handler_owner(signal);
    json!({"boundary":boundary,"pid":unsafe{libc::getpid()},"tid":unsafe{libc::pthread_self() as usize as u64},
        "blocked":blocked_bits(),"pending":pending_bits(),
        "frontLatch":crate::cancellation::received().map(|s| s.name()),
        "owners":{"INT":owner(libc::SIGINT),"TERM":owner(libc::SIGTERM),"HUP":owner(libc::SIGHUP)},
        "managedStarted":at.join("managed-started.json").is_file(),
        "managedReaped":at.join("managed-reaped.json").is_file(),
        "workerWaited":at.join("worker-wait.json").is_file(),
        "managedEnded":at.join("managed-ending.json").is_file()})
    .to_string()
}
/// A named boundary acts only when the running case selects it. A barrier that
/// cannot complete is recorded and fails the case; it never silently passes.
pub(crate) fn barrier(boundary: &str) {
    if !active() {
        return;
    }
    let selected = CASE.with(|c| {
        c.borrow().as_ref().unwrap()["boundary"]
            .as_str()
            .is_some_and(|name| name == boundary)
    });
    if !selected {
        return;
    }
    if let Err(error) = crate::native_barrier::exchange(boundary, &mut || facts(boundary)) {
        receipt(
            "barrier-failure.json",
            json!({"boundary":boundary,"error":error}),
        );
    }
}

#[test]
#[ignore = "private executable-entry process"]
fn native_entry_role() {
    run_role();
}

pub(crate) fn run_role() {
    let directory = root();
    if std::env::var_os("K28_ENTRY_JOB").is_some() {
        fs::write(
            directory.join("child-entry.json"),
            crate::signal_state::native_receipt().to_string(),
        )
        .unwrap();
        crate::completion::end_session(RESULT).unwrap();
        unsafe {
            libc::_exit(0);
        }
    }
    let case: Value = serde_json::from_str(&std::env::var("K28_ENTRY_CASE").unwrap()).unwrap();
    CASE.with(|c| *c.borrow_mut() = Some(case.clone()));
    BROKEN.set(match std::env::var("K28_ENTRY_BROKEN").unwrap().as_str() {
        "none" => Broken::None,
        "capture" => Broken::Capture,
        "reinstatement" => Broken::Reinstatement,
        "blocked-pending" => Broken::BlockedPending,
        "worker-watching" => Broken::WorkerWatching,
        "join-selection" => Broken::Join(Join::Selection),
        "join-front" => Broken::Join(Join::Front),
        "join-outcome" => Broken::Join(Join::Outcome),
        "join-late" => Broken::Join(Join::Late),
        other => panic!("unknown fixed broken control {other}"),
    });
    receipt("launcher-entry.json", crate::signal_state::native_receipt());
    let output = File::create(directory.join("result-output")).unwrap();
    assert_eq!(unsafe { libc::dup2(output.as_raw_fd(), 1) }, 1);
    if family() == "E04-raw" {
        // This is only the executable flag reachability observation.
        unsafe {
            libc::_exit(0);
        }
    }
    if family() == "E04-worker" {
        private_no_child_wait();
    }
    if family() == "F01" {
        crate::signal_state::native_capture_report(
            case["operation"].as_i64().unwrap() as i32,
            BROKEN.get() == Broken::Capture,
        );
    }
    let program = std::env::current_exe().unwrap();
    let mut selected = json!({"status":"selected","provider":"fixture","model":"native","effort":"fixture","reason":"native entry",
        "program":program,"args":["--exact",ROLE,"--ignored","--nocapture"]});
    match case["boundary"].as_str() {
        Some("selection-failure-before-finalise") => {
            selected = json!({"status":"refused","code":"owner_declined","message":"native entry refusal","remedy":"none"});
        }
        Some("no-ending-error-before-finalise") => {
            // Resolves at selection, then fails at exec: a missing interpreter
            // is ENOENT from the spawn, not a selection failure.
            let script = directory.join("missing-interpreter");
            fs::write(&script, "#!/nonexistent/k28-interpreter\n").unwrap();
            fs::set_permissions(&script, std::os::unix::fs::PermissionsExt::from_mode(0o755))
                .unwrap();
            selected["program"] = json!(script);
            selected["args"] = json!([]);
        }
        _ => {}
    }
    let policy = directory.join("policy.ts");
    fs::write(&policy, format!("export const policy={{schemaVersion:2,version:'native-entry',select(){{return {selected};}}}};" )).unwrap();
    // Only the fake managed process receives this marker. Worker environment
    // scrubbing remains the product's real implementation.
    std::env::set_var("K28_ENTRY_JOB", "1");
    let cli = crate::cli::Cli::try_parse_from([
        "ravel",
        "start-session",
        "--kind",
        "native",
        "--prompt",
        "native entry",
        "--config",
        policy.to_str().unwrap(),
        "--json",
    ])
    .unwrap();
    let crate::cli::Command::StartSession(args) = cli.command else {
        unreachable!()
    };
    let outcome = crate::run::run(&args);
    io::stdout().flush().unwrap();
    match outcome {
        Ok(()) => {
            receipt("outcome.json", json!({"success":true}));
            unsafe {
                libc::_exit(0);
            }
        }
        Err(failure) => {
            receipt("outcome.json", failure.to_json());
            eprintln!("{}", failure.to_json());
            if let Some(signal) = failure.refusal.signal {
                crate::cancellation::reraise(signal);
            }
            unsafe {
                libc::_exit(i32::from(failure.refusal.exit));
            }
        }
    }
}

fn raw_entry(
    signal: i32,
    ignored: bool,
    blocked: bool,
    pending: bool,
    chld: i32,
) -> io::Result<()> {
    // This function runs only between fork and exec: fixed data and child-safe
    // signal operations. All JSON interpretation happened before this closure.
    unsafe {
        let mut mask = std::mem::zeroed::<libc::sigset_t>();
        if libc::sigemptyset(&mut mask) == -1 {
            return Err(io::Error::last_os_error());
        }
        if blocked && libc::sigaddset(&mut mask, signal) == -1 {
            return Err(io::Error::last_os_error());
        }
        let set = libc::pthread_sigmask(libc::SIG_SETMASK, &mask, std::ptr::null_mut());
        if set != 0 {
            return Err(io::Error::from_raw_os_error(set));
        }
        let mut action = std::mem::zeroed::<libc::sigaction>();
        if libc::sigemptyset(&mut action.sa_mask) == -1 {
            return Err(io::Error::last_os_error());
        }
        action.sa_sigaction = if ignored {
            libc::SIG_IGN
        } else {
            libc::SIG_DFL
        };
        if libc::sigaction(signal, &action, std::ptr::null_mut()) == -1 {
            return Err(io::Error::last_os_error());
        }
        if chld != 0 {
            action.sa_sigaction = if chld == 1 {
                libc::SIG_IGN
            } else {
                libc::SIG_DFL
            };
            action.sa_flags = if chld == 2 { libc::SA_NOCLDWAIT } else { 0 };
            if libc::sigaction(libc::SIGCHLD, &action, std::ptr::null_mut()) == -1 {
                return Err(io::Error::last_os_error());
            }
        }
        if pending && libc::raise(signal) != 0 {
            return Err(io::Error::last_os_error());
        }
    }
    Ok(())
}
fn bit(signal: i32) -> u64 {
    1u64 << (signal - 1)
}
fn read_json(path: PathBuf) -> Option<Value> {
    fs::read(path)
        .ok()
        .map(|bytes| serde_json::from_slice(&bytes).unwrap())
}

/// Spawn one private real-main fixture under a controller that owns it. The
/// fixture is its own process-group leader; the controller owns settlement.
fn launch(case: &Value, broken: &str) -> (tempfile::TempDir, Controller, PathBuf) {
    let directory = tempfile::tempdir().unwrap();
    for path in ["bin", "libexec/ravel", "home", "tmp"] {
        fs::create_dir_all(directory.path().join(path)).unwrap();
    }
    let exe = std::env::current_exe().unwrap();
    let worker = exe
        .ancestors()
        .nth(3)
        .unwrap()
        .join("libexec/ravel/ravel-policy");
    let fixture = directory.path().join("bin/fixture");
    let private = exe
        .ancestors()
        .nth(3)
        .unwrap()
        .join("native-entry-fixture/debug/ravel");
    assert!(
        private.is_file(),
        "build the private main-thread executable with task test:native-entry"
    );
    fs::copy(&private, &fixture).unwrap();
    fs::copy(&worker, directory.path().join("libexec/ravel/ravel-policy")).unwrap();
    let stdout = directory.path().join("fixture-stdout");
    let stderr = directory.path().join("fixture-stderr");
    let signal = case["signal"].as_i64().unwrap_or(i64::from(libc::SIGTERM)) as i32;
    let ignored = case["ignored"].as_bool().unwrap_or(false);
    let blocked = case["blocked"].as_bool().unwrap_or(false);
    let pending = case["pending"].as_bool().unwrap_or(false);
    let chld = case["chld"].as_i64().unwrap_or(0) as i32;
    // The barrier channels live beside, not inside, the fixture's own tree.
    let channels = directory.path().join("barrier");
    fs::create_dir(&channels).unwrap();
    let mut controller = Controller::new(&channels, Limits::FROZEN).unwrap();
    let mut command = Command::new(fixture);
    command
        .args(["--exact", ROLE, "--ignored", "--nocapture"])
        .env_clear()
        .env("HOME", directory.path().join("home"))
        .env("TMPDIR", directory.path().join("tmp"))
        .env("PATH", "/usr/bin:/bin")
        .env("K28_ENTRY_ROOT", directory.path())
        .env("K28_NATIVE_ENTRY_MAIN", "1")
        .env("K28_ENTRY_CASE", case.to_string())
        .env("K28_ENTRY_BROKEN", broken)
        .current_dir(directory.path())
        .process_group(0)
        .stdout(File::create(&stdout).unwrap())
        .stderr(File::create(&stderr).unwrap())
        .stdin(Stdio::null());
    controller.configure(&mut command);
    // No allocation, JSON access or formatting in the child.
    unsafe {
        command.pre_exec(move || raw_entry(signal, ignored, blocked, pending, chld));
    }
    controller.adopt(command.spawn().unwrap());
    (directory, controller, stderr)
}

fn case(case: Value, broken: &str) -> bool {
    let (directory, mut controller, stderr) = launch(&case, broken);
    let settlement = controller.finish();
    let at = settlement.elapsed;
    let signal = case["signal"].as_i64().unwrap_or(i64::from(libc::SIGTERM)) as i32;
    let ignored = case["ignored"].as_bool().unwrap_or(false);
    let blocked = case["blocked"].as_bool().unwrap_or(false);
    let pending = case["pending"].as_bool().unwrap_or(false);
    assert!(
        settlement.clean(),
        "native fixture settlement unconfirmed: {settlement:?}"
    );
    let status = settlement.status.expect("clean settlement has a status");
    let launcher = read_json(directory.path().join("launcher-entry.json")).unwrap();
    let managed = read_json(directory.path().join("child-entry.json"));
    let worker = read_json(directory.path().join("worker-wait.json"));
    let ending = read_json(directory.path().join("managed-ending.json"));
    let outcome = read_json(directory.path().join("outcome.json"));
    let result = fs::read(directory.path().join("result-output")).unwrap();
    let log = fs::read_to_string(stderr).unwrap();
    eprintln!("native entry case={case} broken={broken} raw_wait={} elapsed={:?} launcher={launcher} worker={worker:?} managed={managed:?} ending={ending:?} outcome={outcome:?} result={result:?}\n{log}", status.into_raw(), at);
    assert_eq!(launcher["recorded"], true);
    assert_eq!(
        launcher["capturingThread"], launcher["currentThread"],
        "fixture moved the front away from the executable-entry thread"
    );
    assert_eq!(launcher["pendingQueryError"], 0);
    LAST_RAW_CHLD_FLAGS.set(launcher["sigchldFlags"].as_i64().unwrap() as i32);
    let family = case["family"].as_str().unwrap();
    if family == "E04-raw" {
        let flags = launcher["sigchldFlags"].as_i64().unwrap() as i32;
        eprintln!(
            "Darwin executable SA_NOCLDWAIT reachability: raw_flags={flags}; inherited={}",
            flags & libc::SA_NOCLDWAIT != 0
        );
        return status.success() && result.is_empty() && managed.is_none() && worker.is_none();
    }
    if family == "F01" {
        return !status.success()
            && result.is_empty()
            && managed.is_none()
            && worker.is_none()
            && outcome
                .as_ref()
                .is_some_and(|v| v["error"]["code"] == "signal_state_unavailable");
    }
    let parent_ignore = launcher["ignored"].as_u64().unwrap();
    let parent_block = launcher["blocked"].as_u64().unwrap();
    if family == "E01" || family == "E02" {
        assert_eq!(parent_ignore & bit(signal) != 0, ignored);
        assert_eq!(parent_block & bit(signal) != 0, blocked);
        if family == "E02" {
            assert_eq!(
                launcher["pending"].as_u64().unwrap() & bit(signal) != 0,
                pending
            );
        }
    }
    let Some(managed) = managed else {
        return false;
    };
    status.success()
        && result == RESULT.as_bytes()
        && managed["ignored"] == launcher["ignored"]
        && managed["blocked"] == launcher["blocked"]
        && worker
            .as_ref()
            .is_some_and(|v| v["actualConfirmedReap"] == true)
        && directory.path().join("managed-started.json").is_file()
        && directory.path().join("managed-reaped.json").is_file()
        && ending.as_ref().is_some_and(|v| v["group"] == "Gone")
}

#[test]
#[ignore = "private real-main fixture; run task test:native-entry"]
fn native_e01_entry_state_and_independent_capture_reinstatement_controls() {
    for signal in [libc::SIGINT, libc::SIGTERM, libc::SIGHUP, libc::SIGPIPE] {
        for ignored in [false, true] {
            for blocked in [false, true] {
                let row =
                    json!({"family":"E01","signal":signal,"ignored":ignored,"blocked":blocked});
                assert!(case(row.clone(), "none"));
                assert!(!case(row.clone(), "capture"));
                assert!(!case(row, "reinstatement"));
            }
        }
    }
}
#[test]
#[ignore = "private real-main fixture; run task test:native-entry"]
fn native_e02_entry_blocked_pending_is_not_cancellation() {
    for signal in [libc::SIGINT, libc::SIGTERM, libc::SIGHUP] {
        for pending in [false, true] {
            let row = json!({"family":"E02","signal":signal,"blocked":true,"pending":pending});
            assert!(case(row.clone(), "none"));
            let mutant = case(row, "blocked-pending");
            assert_eq!(
                mutant, !pending,
                "the no-pending row is a negative control, not an invented baseline failure"
            );
        }
    }
}
#[test]
#[ignore = "private real-main fixture; run task test:native-entry"]
fn native_e03_both_real_waits_survive_ignored_sigchld_entry() {
    for chld in [0, 1] {
        let row = json!({"family":"E03","chld":chld});
        assert!(case(row.clone(), "none"));
        let mutant = case(row, "worker-watching");
        // Darwin can preserve the ignore while clearing no-child-wait across
        // exec. Record that already-waitable baseline honestly. The private
        // E04 worker row supplies the failing repair discriminator in that domain.
        let already_waitable = LAST_RAW_CHLD_FLAGS.get() & libc::SA_NOCLDWAIT == 0;
        assert_eq!(mutant, already_waitable);
        eprintln!("E03 skip-repair mutant conformance={mutant}; raw no-child-wait present={}; private E04 is the independent repair control", !already_waitable);
    }
}
#[test]
#[ignore = "private real-main fixture; run task test:native-entry"]
fn native_e04_private_flag_boundaries_and_separate_exec_reachability() {
    assert!(case(json!({"family":"E04-worker"}), "none"));
    assert!(!case(json!({"family":"E04-worker"}), "worker-watching"));
    assert!(case(json!({"family":"E04-managed"}), "none"));
    assert!(case(json!({"family":"E04-raw","chld":2}), "none"));
}
#[test]
#[ignore = "private real-main fixture; run task test:native-entry"]
fn native_f01_initializer_result_failures_admit_no_children() {
    for operation in 1..=3 {
        let row = json!({"family":"F01","operation":operation});
        assert!(case(row.clone(), "none"));
        assert!(!case(row, "capture"));
    }
}

// ---- E06: acknowledged boundary joins -------------------------------------

/// One E06 row's observed facts. Physical observations only: what the fixture
/// reported at each acknowledgement and what the controller saw end.
#[derive(Debug)]
struct Boundary {
    ready: Option<Value>,
    ack: Option<Value>,
    delivered: Result<(), String>,
    settlement: Settlement,
    died_by: Option<i32>,
    result_empty: bool,
    managed_started: bool,
    managed_end: Option<String>,
    managed_group: Option<String>,
    worker_confirmed: bool,
    outcome: Option<Value>,
    barrier_failure: Option<Value>,
    diagnostics: String,
}

fn boundary_row(boundary: &str, signal: i32, broken: &str) -> Boundary {
    let row = json!({"family":"E06","boundary":boundary,"signal":signal});
    let (directory, mut controller, stderr) = launch(&row, broken);
    let ready = controller.ready();
    let mut ack = None;
    let mut delivered = ready.clone().map(|_| ());
    if ready.is_ok() {
        // The controller acts only between acknowledged exchanges: the ready
        // record, the signal, an acknowledgement that reads the facts fresh,
        // and then an acknowledged release.
        delivered = controller
            .signal(signal)
            .and_then(|()| controller.probe().map(|facts| ack = Some(facts)))
            .and_then(|()| controller.release());
    }
    let settlement = if delivered.is_ok() {
        controller.finish()
    } else {
        controller.abort()
    };
    let parse = |text: String| -> Value {
        serde_json::from_str(text.split_once(' ').map_or(&text, |(_, facts)| facts)).unwrap()
    };
    let at = directory.path();
    let result = fs::read(at.join("result-output")).unwrap_or_default();
    let ending = read_json(at.join("managed-ending.json"));
    let log = fs::read_to_string(stderr).unwrap_or_default();
    let observed = Boundary {
        ready: ready.ok().map(parse),
        ack: ack.map(parse),
        delivered,
        died_by: settlement.status.and_then(|s| s.signal()),
        result_empty: result.is_empty(),
        managed_started: at.join("managed-started.json").is_file(),
        managed_end: ending
            .as_ref()
            .and_then(|v| v["end"].as_str().map(str::to_owned)),
        managed_group: ending
            .as_ref()
            .and_then(|v| v["group"].as_str().map(str::to_owned)),
        worker_confirmed: read_json(at.join("worker-wait.json"))
            .is_some_and(|v| v["actualConfirmedReap"] == true),
        outcome: read_json(at.join("outcome.json")),
        barrier_failure: read_json(at.join("barrier-failure.json")),
        diagnostics: log.clone(),
        settlement,
    };
    eprintln!(
        "native E06 boundary={boundary} signal={signal} broken={broken}\n  {observed:?}\n{log}"
    );
    observed
}

fn signal_name(signal: i32) -> &'static str {
    match signal {
        libc::SIGINT => "SIGINT",
        libc::SIGTERM => "SIGTERM",
        _ => "SIGHUP",
    }
}

/// Whether the row met the contract, as its own per-axis answers. Each axis is a
/// physical or reported fact, so a broken control names the axis that failed.
#[derive(Debug, PartialEq, Eq)]
struct Axes {
    controller: bool,
    cancelled: bool,
    zero_result: bool,
    boundary: bool,
}
impl Axes {
    fn conforms(&self) -> bool {
        self.controller && self.cancelled && self.zero_result && self.boundary
    }
}

fn judge(boundary: &str, signal: i32, seen: &Boundary) -> Axes {
    let bit = bit(signal);
    let name = signal_name(signal);
    let ready = seen.ready.as_ref();
    let ack = seen.ack.as_ref();
    let flag = |facts: Option<&Value>, key: &str| facts.is_some_and(|v| v[key] == true);
    let owner = |facts: Option<&Value>, which: &str| {
        facts.and_then(|v| v["owners"][which].as_str().map(str::to_owned))
    };
    let sig = match signal {
        libc::SIGINT => "INT",
        libc::SIGTERM => "TERM",
        _ => "HUP",
    };
    let boundary_facts = match boundary {
        // The front's own handler owns the signal and consumes it.
        "selection-to-runner" => {
            owner(ready, sig).as_deref() == Some("front")
                && ack.is_some_and(|v| v["frontLatch"] == name)
                && !flag(ack, "managedStarted")
                && !seen.managed_started
        }
        // The original refusal survives beside the cancellation that wins.
        "selection-failure-before-finalise" => {
            owner(ready, sig).as_deref() == Some("front")
                && ack.is_some_and(|v| v["frontLatch"] == name)
                && !seen.managed_started
                && seen.diagnostics.contains("policy code: owner_declined")
        }
        // Handled signals are still blocked, so the signal can only be pending;
        // the runner's handler then consumes it when the mask is restored.
        "started-before-mask-restore" => {
            ready.is_some_and(|v| v["blocked"].as_u64().unwrap() & bit != 0)
                && ack.is_some_and(|v| {
                    v["pending"].as_u64().unwrap() & bit != 0 && v["frontLatch"].is_null()
                })
                && flag(ack, "managedStarted")
                && seen.managed_end.as_deref()
                    == Some(format!("Interrupted {{ signal: {signal} }}").as_str())
                && seen.managed_group.as_deref() == Some("Gone")
        }
        // The child is reaped and the runner's handler, not the front's, owns it.
        "after-reap-before-front" => {
            flag(ready, "managedReaped")
                && owner(ready, sig).as_deref() == Some("other")
                && ack.is_some_and(|v| v["frontLatch"].is_null())
                && seen.managed_group.as_deref() == Some("Gone")
        }
        // No ending exists; the worker was torn down by the front's own wait.
        // The runner's handlers outlive the failed spawn and own the signal;
        // the original spawn failure survives beside the cancellation.
        "no-ending-error-before-finalise" => {
            owner(ready, sig).as_deref() == Some("other")
                && ack.is_some_and(|v| v["frontLatch"].is_null())
                && seen.managed_end.is_none()
                && !seen.managed_started
                && seen.worker_confirmed
                && seen.diagnostics.contains("supervision_failed")
                && seen.diagnostics.contains("No such file or directory")
        }
        other => panic!("boundary {other} is not defined"),
    };
    Axes {
        controller: seen.settlement.clean()
            && seen.delivered.is_ok()
            && seen.barrier_failure.is_none(),
        cancelled: seen.died_by == Some(signal)
            && seen
                .outcome
                .as_ref()
                .is_some_and(|v| v["error"]["signal"] == name),
        zero_result: seen.result_empty,
        boundary: boundary_facts,
    }
}

const BOUNDARIES: [(&str, &str); 5] = [
    ("selection-to-runner", "join-selection"),
    ("started-before-mask-restore", "join-outcome"),
    ("after-reap-before-front", "join-late"),
    ("selection-failure-before-finalise", "join-front"),
    ("no-ending-error-before-finalise", "join-late"),
];

#[test]
#[ignore = "private real-main fixture; run task test:native-entry"]
fn native_e06_each_boundary_joins_cancellation_and_each_join_is_load_bearing() {
    for (boundary, mutant) in BOUNDARIES {
        for signal in [libc::SIGINT, libc::SIGTERM, libc::SIGHUP] {
            let seen = boundary_row(boundary, signal, "none");
            let axes = judge(boundary, signal, &seen);
            assert!(axes.conforms(), "{boundary} signal {signal}: {axes:?}");

            // Removing only the corresponding join must fail a named axis.
            let broken = boundary_row(boundary, signal, mutant);
            let axes = judge(boundary, signal, &broken);
            assert!(axes.controller, "{boundary} {signal}: controller {axes:?}");
            if boundary == "selection-to-runner" {
                // The final join still reports the signal, but the runner was
                // admitted after cancellation: only admission can tell.
                assert!(
                    axes.cancelled && !axes.boundary,
                    "{boundary} {signal}: {axes:?}"
                );
                assert!(broken.managed_started);
            } else {
                assert!(!axes.cancelled, "{boundary} {signal}: {axes:?}");
            }
        }
    }
}

// ---- controller qualification ---------------------------------------------
//
// These run the controller against stub fixtures that misbehave on purpose.
// A passing campaign receipt is only trusted because each of these is seen to
// fail the way the controller is meant to detect and settle.

fn stub(script: &str, limits: Limits) -> (tempfile::TempDir, Controller) {
    let directory = tempfile::tempdir().unwrap();
    let mut controller = Controller::new(directory.path(), limits).unwrap();
    let mut command = Command::new("/bin/sh");
    command
        .args(["-c", script])
        .env_clear()
        .env("PATH", "/usr/bin:/bin")
        .process_group(0)
        .stdin(Stdio::null());
    controller.configure(&mut command);
    controller.adopt(command.spawn().unwrap());
    (directory, controller)
}

fn gone(pid: u32) -> bool {
    // The identity is reaped by its owner before this asks; ESRCH confirms.
    let result = unsafe { libc::kill(pid as i32, 0) };
    result == -1 && io::Error::last_os_error().raw_os_error() == Some(libc::ESRCH)
}

#[test]
#[ignore = "private controller qualification; run task test:native-entry"]
fn native_controller_watchdog_expiry_is_a_failed_receipt_that_stops_the_fixture() {
    // A fixture that never reaches its barrier and never exits.
    let (_directory, mut controller) = stub("exec sleep 600", Limits::FROZEN);
    let pid = controller.pid().unwrap();
    let ready = controller.ready();
    assert!(ready.is_err(), "a silent fixture produced a ready record");
    let settlement = controller.finish();
    eprintln!("controller watchdog: {settlement:?} ready={ready:?}");
    assert!(settlement.watchdog_expired);
    assert!(settlement.stopped_by_controller);
    assert!(!settlement.clean(), "an expired watchdog read as a pass");
    assert_eq!(
        settlement.status.and_then(|s| s.signal()),
        Some(libc::SIGKILL)
    );
    assert!(settlement.unresolved.is_empty());
    assert!(gone(pid));
    // Frozen envelope: work stops at 25 seconds, all of it inside 30.
    assert!(
        settlement.elapsed >= Duration::from_secs(25),
        "{settlement:?}"
    );
    assert!(settlement.elapsed <= Limits::FROZEN.outer, "{settlement:?}");
}

#[test]
#[ignore = "private controller qualification; run task test:native-entry"]
fn native_controller_blocked_acknowledgement_and_failed_release_are_failed_receipts() {
    // The fixture announces itself and then never reads another command.
    let (_directory, mut controller) = stub(
        r#"printf 'READY stub {}\n' > "$K28_BARRIER_DIR/ready.fifo"; exec sleep 600"#,
        Limits::FROZEN,
    );
    let pid = controller.pid().unwrap();
    assert!(controller.ready().is_ok());
    let at = Instant::now();
    assert!(
        controller.probe().is_err(),
        "a blocked fixture acknowledged"
    );
    let probed = at.elapsed();
    let at = Instant::now();
    assert!(
        controller.release().is_err(),
        "an unread release was confirmed"
    );
    let released = at.elapsed();
    let settlement = controller.abort();
    eprintln!("controller blocked-ack: probe={probed:?} release={released:?} {settlement:?}");
    // Each acknowledgement is bounded by its own five seconds.
    for took in [probed, released] {
        assert!(took >= Limits::FROZEN.ack && took < Limits::FROZEN.ack + Duration::from_secs(1));
    }
    assert!(settlement.faults.len() >= 2, "{settlement:?}");
    assert!(!settlement.clean());
    assert_eq!(
        settlement.status.and_then(|s| s.signal()),
        Some(libc::SIGKILL)
    );
    assert!(gone(pid));
}

#[test]
#[ignore = "private controller qualification; run task test:native-entry"]
fn native_controller_escaped_holder_is_stopped_and_reaped_by_its_owner() {
    // The holder is deliberately escaped: its own session, not the fixture's.
    for natural in [false, true] {
        let (directory, mut controller) = stub(
            if natural { "exit 0" } else { "exec sleep 600" },
            Limits::FROZEN,
        );
        let holder = Command::new("/bin/sleep")
            .arg("600")
            .process_group(0)
            .stdin(Stdio::null())
            .spawn()
            .unwrap();
        let holder_pid = holder.id();
        controller.adopt_holder(holder);
        let settlement = if natural {
            controller.finish()
        } else {
            controller.abort()
        };
        drop(directory);
        eprintln!("controller holder natural={natural}: {settlement:?}");
        assert_eq!(settlement.holders, vec![(holder_pid, true)]);
        assert!(settlement.stopped_by_controller);
        assert!(settlement.unresolved.is_empty());
        assert!(gone(holder_pid));
        assert_eq!(settlement.clean(), natural);
    }
}

#[test]
#[ignore = "private controller qualification; run task test:native-entry"]
fn native_controller_never_signals_a_reaped_fixture() {
    let (_directory, mut controller) = stub("exit 0", Limits::FROZEN);
    let settlement = controller.finish();
    assert!(settlement.clean(), "{settlement:?}");
    let refused = controller.signal(libc::SIGTERM).unwrap_err();
    assert!(refused.contains("already ended"), "{refused}");
    // A second settlement neither signals nor invents a new status.
    let again = controller.finish();
    assert_eq!(again.status, settlement.status);
    assert!(!again.stopped_by_controller);
}
