//! Private executable-entry fixtures: real policy worker, fake managed process.
use std::cell::{Cell, RefCell};
use std::fs::{self, File};
use std::io::{self, Write as _};
use std::os::fd::AsRawFd as _;
use std::os::unix::process::{CommandExt as _, ExitStatusExt as _};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use clap::Parser as _;
use keyed_launch::{Ended, EntrySignals, LaunchEvent};
use serde_json::{json, Value};

const ROLE: &str = "native_entry_tests::native_entry_role";
const RESULT: &str = "native 🪶\nresult";
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Broken {
    None,
    Capture,
    Reinstatement,
    BlockedPending,
    WorkerWatching,
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
        json!({"rawStatus":ending.status.into_raw(),"group":format!("{:?}",ending.group),"elapsedNanos":ending.elapsed.as_nanos().to_string(),"operationReports":ending.operation_reports}),
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
    let selected = json!({"status":"selected","provider":"fixture","model":"native","effort":"fixture","reason":"native entry",
        "program":program,"args":["--exact",ROLE,"--ignored","--nocapture"]});
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

fn case(case: Value, broken: &str) -> bool {
    let at = Instant::now();
    let outer = at + Duration::from_secs(30);
    let case_by = at + Duration::from_secs(25);
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
    // No allocation, JSON access or formatting in the child.
    unsafe {
        command.pre_exec(move || raw_entry(signal, ignored, blocked, pending, chld));
    }
    let mut child = command.spawn().unwrap();
    let mut aborted = None;
    let mut killed = false;
    let status = loop {
        if let Some(status) = child.try_wait().unwrap() {
            break status;
        }
        let now = Instant::now();
        if aborted.is_none() && now >= case_by {
            assert_eq!(unsafe { libc::kill(child.id() as i32, libc::SIGTERM) }, 0);
            aborted = Some(now);
        }
        if !killed && aborted.is_some_and(|at| now.duration_since(at) >= Duration::from_secs(1)) {
            child.kill().unwrap();
            killed = true;
        }
        assert!(
            now < outer,
            "native fixture cleanup unconfirmed after outer deadline"
        );
        std::thread::sleep(
            Duration::from_millis(1).min(outer.saturating_duration_since(Instant::now())),
        );
    };
    let launcher = read_json(directory.path().join("launcher-entry.json")).unwrap();
    let managed = read_json(directory.path().join("child-entry.json"));
    let worker = read_json(directory.path().join("worker-wait.json"));
    let ending = read_json(directory.path().join("managed-ending.json"));
    let outcome = read_json(directory.path().join("outcome.json"));
    let result = fs::read(directory.path().join("result-output")).unwrap();
    let log = fs::read_to_string(stderr).unwrap();
    eprintln!("native entry case={case} broken={broken} raw_wait={} elapsed={:?} launcher={launcher} worker={worker:?} managed={managed:?} ending={ending:?} outcome={outcome:?} result={result:?}\n{log}", status.into_raw(), at.elapsed());
    assert!(
        aborted.is_none(),
        "native case deadline expired; fixture tree settlement is unconfirmed"
    );
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
