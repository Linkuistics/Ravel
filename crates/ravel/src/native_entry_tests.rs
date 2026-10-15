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

pub(crate) mod faults;

const ROLE: &str = "native_entry_tests::native_entry_role";
const RESULT: &str = "native 🪶\nresult";
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Broken {
    None,
    Capture,
    Reinstatement,
    BlockedPending,
    WorkerWatching,
    Shielding,
    /// The selected F-row failure is ignored at its site.
    IgnoreFault,
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
/// E08's private break: the reader shield reports success without blocking.
pub(crate) fn broken_shielding() -> bool {
    BROKEN.get() == Broken::Shielding
}
/// F06's sites run in the forked child of the real runner, which this fixture
/// links without test cfg. The runner's off-by-default `native-faults` feature
/// supplies them; the child writes its fixed two-byte record to a file this
/// row owns, never formatted in the child.
#[cfg(feature = "native-faults")]
fn select_runner_fault(case: &Value) {
    let record = File::create(root().join("child-fault-record.bin")).unwrap();
    // The descriptor stays open for the process's life: the forked child needs
    // it, and exec closes it (CLOEXEC) for any child that gets that far.
    let fd = record.as_raw_fd();
    std::mem::forget(record);
    keyed_launch::native_fault_select(
        case["site"].as_str().unwrap(),
        case["mode"].as_str().unwrap(),
        std::env::var("K28_ENTRY_BROKEN").is_ok_and(|mode| mode == "ignore-fault"),
        fd,
    );
}
#[cfg(not(feature = "native-faults"))]
fn select_runner_fault(_: &Value) {
    panic!("F06 needs the fixture built with --features native-faults (task test:native-entry)");
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
    // The parent's known handoff block holds here: the admission block has
    // been taken and the mask is restored only by the Started check.
    barrier("handoff-block-held");
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
/// Whether this process still has any child at all, read without consuming
/// one: `waitid` with WNOWAIT leaves an exited child waitable, so a probe never
/// takes a reap the product owns. ECHILD is the only clean answer.
fn stray_child() -> i64 {
    // SAFETY: a zeroed siginfo is a valid output buffer for waitid.
    let mut info = unsafe { std::mem::zeroed::<libc::siginfo_t>() };
    // SAFETY: nonblocking, non-consuming query of this process's own children.
    let result = unsafe {
        libc::waitid(
            libc::P_ALL,
            0,
            &mut info,
            libc::WEXITED | libc::WNOHANG | libc::WNOWAIT,
        )
    };
    if result == -1 && io::Error::last_os_error().raw_os_error() == Some(libc::ECHILD) {
        0
    } else {
        1
    }
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
        "handlerThread":crate::cancellation::HANDLER_THREAD.load(std::sync::atomic::Ordering::SeqCst),
        "readers":crate::worker::native_tests::reader_receipt(),
        "foreground":unsafe{libc::tcgetpgrp(0)},"pgrp":unsafe{libc::getpgrp()},
        "strayChild":stray_child(),
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
    if std::env::var_os("K28_E08_WORKER").is_some() {
        crate::worker::native_tests::e08_worker();
    }
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
    faults::select(&case);
    if family() == "F06" {
        select_runner_fault(&case);
    }
    BROKEN.set(match std::env::var("K28_ENTRY_BROKEN").unwrap().as_str() {
        "none" => Broken::None,
        "capture" => Broken::Capture,
        "reinstatement" => Broken::Reinstatement,
        "blocked-pending" => Broken::BlockedPending,
        "worker-watching" => Broken::WorkerWatching,
        "shielding" => Broken::Shielding,
        "ignore-fault" => Broken::IgnoreFault,
        "join-selection" => Broken::Join(Join::Selection),
        "join-front" => Broken::Join(Join::Front),
        "join-outcome" => Broken::Join(Join::Outcome),
        "join-late" => Broken::Join(Join::Late),
        other => panic!("unknown fixed broken control {other}"),
    });
    if family() == "E08" {
        // Record reader and creator masks and the bounded collection; inject
        // nothing.
        crate::worker::native_tests::observe();
    }
    receipt("launcher-entry.json", crate::signal_state::native_receipt());
    let output = File::create(directory.join("result-output")).unwrap();
    assert_eq!(unsafe { libc::dup2(output.as_raw_fd(), 1) }, 1);
    if family() == "E04-raw" {
        // This is only the executable flag reachability observation.
        unsafe {
            libc::_exit(0);
        }
    }
    if family() == "E04-worker" || family() == "F02" {
        // F02's precondition: private default plus no-child-wait (E03
        // independently covers the executable-entry ignore).
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
    let spawn_enoent = case["outcome"] == "spawn-ENOENT";
    match case["boundary"].as_str() {
        Some("selection-failure-before-finalise") => {
            selected = json!({"status":"refused","code":"owner_declined","message":"native entry refusal","remedy":"none"});
        }
        Some("no-ending-error-before-finalise") => {
            spawn_failure(&directory, &mut selected);
        }
        _ if spawn_enoent => spawn_failure(&directory, &mut selected),
        _ => {}
    }
    if case["result"] == "policy-refusal" || case["outcome"] == "policy-refusal" {
        selected = json!({"status":"refused","code":"owner_declined","message":"native entry refusal","remedy":"none"});
    }
    // E08's private worker returns this verbatim; the real worker reads policy.ts.
    fs::write(directory.join("e08-selection.json"), selected.to_string()).unwrap();
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

/// Resolves at selection, then fails at exec: a missing interpreter is ENOENT
/// from the spawn, not a selection failure.
fn spawn_failure(directory: &std::path::Path, selected: &mut Value) {
    let script = directory.join("missing-interpreter");
    fs::write(&script, "#!/nonexistent/k28-interpreter\n").unwrap();
    fs::set_permissions(&script, std::os::unix::fs::PermissionsExt::from_mode(0o755)).unwrap();
    selected["program"] = json!(script);
    selected["args"] = json!([]);
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
            // The injection is read back before exec: a later "cleared at entry"
            // reading is only an exec observation if the state was really set.
            let mut set = std::mem::zeroed::<libc::sigaction>();
            if libc::sigaction(libc::SIGCHLD, std::ptr::null(), &mut set) == -1 {
                return Err(io::Error::last_os_error());
            }
            if !chld_injection_confirmed(chld, set.sa_sigaction, set.sa_flags) {
                return Err(io::Error::from_raw_os_error(libc::EIO));
            }
        }
        if pending && libc::raise(signal) != 0 {
            return Err(io::Error::last_os_error());
        }
    }
    Ok(())
}
/// Whether the raw SIGCHLD state read back after injecting `chld` (1 ignore,
/// 2 default with no-child-wait) is the state that was meant. Fixed data only:
/// it runs between fork and exec.
fn chld_injection_confirmed(chld: i32, handler: libc::sighandler_t, flags: i32) -> bool {
    match chld {
        1 => handler == libc::SIG_IGN,
        2 => handler == libc::SIG_DFL && flags & libc::SA_NOCLDWAIT != 0,
        _ => true,
    }
}
/// Short manifest label of a handled or tracked signal.
fn label(signal: i32) -> &'static str {
    match signal {
        libc::SIGINT => "INT",
        libc::SIGTERM => "TERM",
        libc::SIGHUP => "HUP",
        libc::SIGPIPE => "PIPE",
        other => panic!("signal {other} has no manifest label"),
    }
}
/// File one judged (row, subject) under the campaign's directory, if any.
fn record(id: &str, subject: &str, conforms: bool, note: &str) {
    crate::native_barrier::row(id, subject, conforms, note);
}
/// The manifest's spelling of a fault mode.
fn mode_label(mode: &str) -> &'static str {
    match mode {
        "report" => "report-EIO",
        "omit" => "omit-EIO",
        "perform" => "perform-report-EIO",
        other => panic!("mode {other} has no manifest name"),
    }
}
fn panic_text(payload: &(dyn std::any::Any + Send)) -> String {
    payload
        .downcast_ref::<String>()
        .cloned()
        .or_else(|| {
            payload
                .downcast_ref::<&str>()
                .map(|text| (*text).to_owned())
        })
        .unwrap_or_default()
}
/// Run the claims a row's assertions make of one subject and file the verdict
/// before any failure propagates: a violated claim is a recorded verdict.
fn judged<R>(id: &str, subject: &str, claims: impl FnOnce() -> R) -> R {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(claims)) {
        Ok(value) => {
            record(id, subject, true, "");
            value
        }
        Err(payload) => {
            record(id, subject, false, &panic_text(&*payload));
            std::panic::resume_unwind(payload)
        }
    }
}
/// The same for a named broken subject: its assertions state the *violation*, so
/// their holding means the defect was detected and the subject did not conform.
fn detected<R>(id: &str, subject: &str, violation: impl FnOnce() -> R) -> R {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(violation)) {
        Ok(value) => {
            record(id, subject, false, "violation detected");
            value
        }
        Err(payload) => {
            record(id, subject, true, &panic_text(&*payload));
            std::panic::resume_unwind(payload)
        }
    }
}
fn bit(signal: i32) -> u64 {
    1u64 << (signal - 1)
}
fn read_json(path: PathBuf) -> Option<Value> {
    fs::read(path)
        .ok()
        .map(|bytes| serde_json::from_slice(&bytes).unwrap())
}

/// A PTY the row owns for a fixture that must hold a terminal lease. The
/// fixture is its session and group leader with the slave as controlling
/// terminal. A process exiting onto an undrained terminal can block in tty close
/// beyond SIGKILL, so the row drains its own master on a thread it joins on
/// every path.
struct Pty {
    master: Option<File>,
    stop: std::sync::Arc<std::sync::atomic::AtomicBool>,
    drain: Option<std::thread::JoinHandle<()>>,
}
impl Pty {
    fn open() -> (Self, File) {
        use std::os::fd::FromRawFd as _;
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
        let (master, slave) = unsafe { (File::from_raw_fd(master), File::from_raw_fd(slave)) };
        for fd in [master.as_raw_fd(), slave.as_raw_fd()] {
            // SAFETY: close-on-exec on a descriptor this row owns.
            assert_eq!(
                unsafe { libc::fcntl(fd, libc::F_SETFD, libc::FD_CLOEXEC) },
                0
            );
        }
        // SAFETY: O_NONBLOCK on a descriptor this row owns.
        unsafe {
            let flags = libc::fcntl(master.as_raw_fd(), libc::F_GETFL);
            assert_eq!(
                libc::fcntl(master.as_raw_fd(), libc::F_SETFL, flags | libc::O_NONBLOCK),
                0
            );
        }
        let stop = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let drain = {
            let mut reader = master.try_clone().unwrap();
            let stop = std::sync::Arc::clone(&stop);
            std::thread::spawn(move || {
                use std::io::Read as _;
                let mut sink = [0u8; 4096];
                while !stop.load(std::sync::atomic::Ordering::SeqCst) {
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
        (
            Self {
                master: Some(master),
                stop,
                drain: Some(drain),
            },
            slave,
        )
    }
    /// Stop draining and close the master, after the fixture was settled.
    fn finish(mut self) {
        self.stop.store(true, std::sync::atomic::Ordering::SeqCst);
        if let Some(drain) = self.drain.take() {
            drain.join().unwrap();
        }
        drop(self.master.take());
    }
}

/// Spawn one private real-main fixture under a controller that owns it. The
/// fixture is its own process-group leader; the controller owns settlement.
fn launch(case: &Value, broken: &str) -> (tempfile::TempDir, Controller, PathBuf, Option<Pty>) {
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
    let installed = directory.path().join("libexec/ravel/ravel-policy");
    if case["family"] == "E08" {
        // The private worker is this same executable in its worker role. Its
        // settings are baked into the script: nothing is inherited.
        let holder = if case["readers"] == "one-escaped-holder" {
            format!(
                "K28_E08_HOLDER='{}' ",
                directory.path().join("holder.sock").display()
            )
        } else {
            String::new()
        };
        fs::write(
            &installed,
            format!(
                "#!/bin/sh\nK28_NATIVE_ENTRY_MAIN=1 K28_E08_WORKER=1 K28_E08_SELECTION='{}' {holder}exec '{}'\n",
                directory.path().join("e08-selection.json").display(),
                fixture.display()
            ),
        )
        .unwrap();
        fs::set_permissions(
            &installed,
            std::os::unix::fs::PermissionsExt::from_mode(0o755),
        )
        .unwrap();
    } else {
        fs::copy(&worker, &installed).unwrap();
    }
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
    if case["family"] == "E08" && case["readers"] == "one-escaped-holder" {
        controller
            .serve_holder(&directory.path().join("holder.sock"))
            .unwrap();
    }
    let (pty, slave) = match case["pty"].as_bool().unwrap_or(false).then(Pty::open) {
        Some((pty, slave)) => (Some(pty), Some(slave)),
        None => (None, None),
    };
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
        .stdout(File::create(&stdout).unwrap())
        .stderr(File::create(&stderr).unwrap());
    match &pty {
        Some(_) => {
            // The slave is stdin and becomes the controlling terminal below.
            command.stdin(slave.expect("slave of an open pty"));
        }
        None => {
            command.process_group(0).stdin(Stdio::null());
        }
    }
    controller.configure(&mut command);
    // No allocation, JSON access or formatting in the child.
    unsafe {
        if pty.is_some() {
            // setsid makes the fixture its own session and group leader;
            // TIOCSCTTY gives it the slave as its controlling terminal.
            command.pre_exec(|| {
                if libc::setsid() == -1 || libc::ioctl(0, libc::TIOCSCTTY as _, 0) == -1 {
                    return Err(io::Error::last_os_error());
                }
                Ok(())
            });
        }
        command.pre_exec(move || raw_entry(signal, ignored, blocked, pending, chld));
    }
    controller.adopt(command.spawn().unwrap());
    drop(command);
    (directory, controller, stderr, pty)
}

fn case(case: Value, broken: &str) -> bool {
    let (directory, mut controller, stderr, pty) = launch(&case, broken);
    let settlement = controller.finish();
    pty.into_iter().for_each(Pty::finish);
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
        let inherited = flags & libc::SA_NOCLDWAIT != 0;
        eprintln!(
            "executable-entry SA_NOCLDWAIT reachability: raw_flags={flags}; inherited={inherited}"
        );
        let ran = status.success() && result.is_empty() && managed.is_none() && worker.is_none();
        // Darwin: an observation only, whichever the answer. Linux: exec clears
        // every handler's flags (flush_signal_handlers sets sa_flags to 0 for
        // each signal, ignored ones included), so a set flag at entry would
        // contradict the source and is a failed row, never an inheritance claim.
        // https://github.com/torvalds/linux/blob/v6.18/kernel/signal.c#L489
        return ran && (!cfg!(target_os = "linux") || !inherited);
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
                let id = format!(
                    "E01__signal={}__disposition={}__mask={}",
                    label(signal),
                    if ignored { "ignore" } else { "default" },
                    if blocked { "blocked" } else { "clear" }
                );
                let repaired = case(row.clone(), "none");
                record(&id, "repaired", repaired, "");
                assert!(repaired);
                let capture = case(row.clone(), "capture");
                record(&id, "broken:capture", capture, "");
                assert!(!capture);
                let reinstatement = case(row, "reinstatement");
                record(&id, "broken:reinstatement", reinstatement, "");
                assert!(!reinstatement);
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
            let id = format!(
                "E02__signal={}__pending={}",
                label(signal),
                if pending { "yes" } else { "no" }
            );
            let repaired = case(row.clone(), "none");
            record(&id, "repaired", repaired, "");
            assert!(repaired);
            let mutant = case(row, "blocked-pending");
            record(&id, "broken:blocked-pending", mutant, "");
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
        let id = format!(
            "E03__disposition={}",
            if chld == 0 { "default" } else { "ignore" }
        );
        let repaired = case(row.clone(), "none");
        record(&id, "repaired", repaired, "");
        assert!(repaired);
        let mutant = case(row, "worker-watching");
        record(
            &id,
            "broken:worker-watching",
            mutant,
            &format!("raw flags {}", LAST_RAW_CHLD_FLAGS.get()),
        );
        // Darwin can preserve the ignore while clearing no-child-wait across
        // exec. Record that already-waitable baseline honestly. The private
        // E04 worker row supplies the failing repair discriminator in that domain.
        // On Linux the ignored disposition is itself the no-zombie state (wait
        // fails with ECHILD once the children are gone) and exec preserves it,
        // so the ignore row is a failing discriminator of its own and only the
        // default row is already waitable.
        // https://man7.org/linux/man-pages/man2/sigaction.2.html (Notes: "POSIX.1-2001
        // specifies that if the disposition of SIGCHLD is set to SIG_IGN ...")
        let already_waitable = if cfg!(target_os = "linux") {
            chld == 0
        } else {
            LAST_RAW_CHLD_FLAGS.get() & libc::SA_NOCLDWAIT == 0
        };
        assert_eq!(mutant, already_waitable);
        eprintln!("E03 skip-repair mutant conformance={mutant}; raw no-child-wait present={}; private E04 is the independent repair control", !already_waitable);
    }
}
#[test]
#[ignore = "private real-main fixture; run task test:native-entry"]
fn native_e04_private_flag_boundaries_and_separate_exec_reachability() {
    const WORKER: &str = "E04__boundary=worker__state=private-default-SA_NOCLDWAIT";
    const MANAGED: &str = "E04__boundary=managed__state=private-default-SA_NOCLDWAIT";
    const RAW: &str = "E04__boundary=raw-executable-entry__state=Darwin-flag-reachability";
    let repaired = case(json!({"family":"E04-worker"}), "none");
    record(WORKER, "repaired", repaired, "");
    assert!(repaired);
    let mutant = case(json!({"family":"E04-worker"}), "worker-watching");
    record(WORKER, "broken:worker-watching", mutant, "");
    assert!(!mutant);
    // The end-to-end positive; the runner-level role owns the failing mutant.
    let managed = case(json!({"family":"E04-managed"}), "none");
    record(MANAGED, "repaired:front", managed, "");
    assert!(managed);
    // An observation, never a repair claim: whether the flag reached the entry.
    let raw = case(json!({"family":"E04-raw","chld":2}), "none");
    record(
        RAW,
        "repaired",
        raw,
        &format!(
            "raw SIGCHLD flags {}; SA_NOCLDWAIT reached the executable entry: {}",
            LAST_RAW_CHLD_FLAGS.get(),
            LAST_RAW_CHLD_FLAGS.get() & libc::SA_NOCLDWAIT != 0
        ),
    );
    assert!(raw);
}
#[test]
#[ignore = "private real-main fixture; run task test:native-entry"]
fn native_f01_initializer_result_failures_admit_no_children() {
    for operation in 1..=3 {
        let row = json!({"family":"F01","operation":operation});
        let id = format!(
            "F01__operation={}__mode=report-EIO",
            ["mask-query", "TERM-action-query", "TERM-membership"][operation - 1]
        );
        let repaired = case(row.clone(), "none");
        record(&id, "repaired", repaired, "");
        assert!(repaired);
        let ignored = case(row, "capture");
        record(&id, "broken:ignore-fault", ignored, "");
        assert!(!ignored);
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
    worker_started: bool,
    managed_end: Option<String>,
    managed_group: Option<String>,
    worker_confirmed: bool,
    outcome: Option<Value>,
    barrier_failure: Option<Value>,
    /// The selected F-row site: physical operation versus substituted report,
    /// and the state after it. Absent when no fault site was reached.
    fault: Option<Value>,
    /// The runner child's own fixed two-byte record (F06): site, then bit 0
    /// performed. Written before the child exited; never formatted there.
    child_record: Option<Vec<u8>>,
    diagnostics: String,
}

fn boundary_row(boundary: &str, signal: i32, broken: &str) -> Boundary {
    drive_row(
        json!({"family":"E06","boundary":boundary,"signal":signal}),
        signal,
        broken,
    )
}

/// Drive one acknowledged exchange: ready, the actual signal, a probe that
/// reads the facts fresh, and an acknowledged release.
fn drive_row(row: Value, signal: i32, broken: &str) -> Boundary {
    drive(row, Some(signal), broken)
}

/// One whole case with no exchange at all: the fixture runs to its end under
/// the controller, which owns and settles it.
fn run_row(row: Value, broken: &str) -> Boundary {
    drive(row, None, broken)
}

/// `Some(signal)` drives the exchange above. `None` on a row with a named
/// boundary reads facts fresh and releases without delivering anything; on a
/// row with no boundary the fixture is only observed to its end.
fn drive(row: Value, action: Option<i32>, broken: &str) -> Boundary {
    let boundary = row["boundary"].as_str().unwrap_or("").to_owned();
    let boundary = boundary.as_str();
    let signal = action.unwrap_or(0);
    let (directory, mut controller, stderr, pty) = launch(&row, broken);
    let named = row["boundary"].is_string();
    let ready = if named {
        controller.ready()
    } else {
        Err("no boundary".to_owned())
    };
    let mut ack = None;
    let mut delivered = if named {
        ready.clone().map(|_| ())
    } else {
        Ok(())
    };
    if ready.is_ok() {
        // The controller acts only between acknowledged exchanges: the ready
        // record, the signal, an acknowledgement that reads the facts fresh,
        // and then an acknowledged release.
        delivered = match action {
            Some(signal) => controller.signal(signal),
            None => Ok(()),
        }
        .and_then(|()| controller.probe().map(|facts| ack = Some(facts)))
        .and_then(|()| controller.release());
    }
    let settlement = if delivered.is_ok() {
        controller.finish()
    } else {
        controller.abort()
    };
    pty.into_iter().for_each(Pty::finish);
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
        worker_started: at.join("worker-wait.json").is_file(),
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
        fault: read_json(at.join("fault-receipt.json")),
        child_record: fs::read(at.join("child-fault-record.bin")).ok(),
        diagnostics: log.clone(),
        settlement,
    };
    eprintln!(
        "native {} boundary={boundary} signal={signal} broken={broken}\n  {observed:?}\n{log}",
        row["family"].as_str().unwrap()
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
            let id = format!(
                "E06__boundary={}__signal={}",
                match boundary {
                    "selection-to-runner" => "selection-to-runner",
                    "started-before-mask-restore" => "Started-before-mask-restore",
                    "after-reap-before-front" => "reaped-before-front",
                    "selection-failure-before-finalise" => "refusal-before-finalise",
                    "no-ending-error-before-finalise" => "ENOENT-before-finalise",
                    other => panic!("boundary {other} has no manifest name"),
                },
                label(signal)
            );
            let seen = boundary_row(boundary, signal, "none");
            let axes = judge(boundary, signal, &seen);
            record(&id, "repaired", axes.conforms(), &format!("{axes:?}"));
            assert!(axes.conforms(), "{boundary} signal {signal}: {axes:?}");

            // Removing only the corresponding join must fail a named axis.
            let broken = boundary_row(boundary, signal, mutant);
            let axes = judge(boundary, signal, &broken);
            let detected = axes.controller
                && if boundary == "selection-to-runner" {
                    axes.cancelled && !axes.boundary && broken.managed_started
                } else {
                    !axes.cancelled
                };
            record(
                &id,
                &format!("broken:{mutant}"),
                !detected,
                &format!("{axes:?}"),
            );
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

// ---- F rows: single-site reported faults ------------------------------------
//
// Each row selects one fixed site and one mode. The repaired product must fail
// closed there; the broken subject ignores that one failure and must therefore
// fail a named axis. Physical operation and substituted report are separate
// fields of the fault receipt, never one field.

/// What an admission-stage failure must look like: refused before any worker
/// or managed child exists, no result bytes, the site's own receipt.
#[derive(Debug)]
struct Refused {
    controller: bool,
    failed: bool,
    zero_result: bool,
    code: bool,
    text: bool,
    no_worker: bool,
    no_managed: bool,
    site: bool,
}
impl Refused {
    fn conforms(&self) -> bool {
        self.controller
            && self.failed
            && self.zero_result
            && self.code
            && self.text
            && self.no_worker
            && self.no_managed
            && self.site
    }
}
fn fault_site_matches(seen: &Boundary, site: &str, mode: &str, broken: bool) -> bool {
    seen.fault.as_ref().is_some_and(|fault| {
        fault["site"] == site
            && fault["mode"] == mode
            && fault["operationPerformed"] == (mode != "omit")
            && fault["reported"] == "EIO"
            && fault["failureIgnored"] == broken
    })
}
fn refused(
    seen: &Boundary,
    code: &str,
    text: &str,
    site: &str,
    mode: &str,
    broken: bool,
) -> Refused {
    Refused {
        controller: seen.settlement.clean()
            && seen.delivered.is_ok()
            && seen.barrier_failure.is_none(),
        failed: seen
            .settlement
            .status
            .is_some_and(|status| !status.success()),
        zero_result: seen.result_empty,
        code: seen
            .outcome
            .as_ref()
            .is_some_and(|v| v["error"]["code"] == code),
        text: seen.outcome.as_ref().is_some_and(|v| {
            v["error"]["message"]
                .as_str()
                .is_some_and(|m| m.contains(text))
        }),
        no_worker: !seen.worker_confirmed && !seen.worker_started,
        no_managed: !seen.managed_started,
        site: fault_site_matches(seen, site, mode, broken),
    }
}

#[test]
#[ignore = "private real-main fixture; run task test:native-entry"]
fn native_f02_worker_watching_faults_refuse_before_any_worker() {
    for (site, mode) in [
        ("watch-action-query", "report"),
        ("default-action-repair", "omit"),
        ("default-action-repair", "perform"),
    ] {
        let id = format!(
            "F02__boundary=worker__operation={site}__mode={}",
            match mode {
                "report" => "report-EIO",
                "omit" => "omit-EIO",
                _ => "perform-report-EIO",
            }
        );
        let row = json!({"family":"F02","owner":"worker","site":site,"mode":mode});
        let seen = run_row(row.clone(), "none");
        let axes = refused(
            &seen,
            "worker_failed",
            "cannot establish worker watching",
            site,
            mode,
            false,
        );
        let fault = seen.fault.as_ref();
        // The physical state after the site, separately from the report: the
        // omitted repair left no-child-wait set, the performed one cleared it.
        let state = site != "default-action-repair"
            || fault.is_some_and(|f| f["after"]["sigchld"]["noChildWait"] == (mode == "omit"));
        record(
            &id,
            "repaired",
            axes.conforms() && state,
            &format!("{axes:?} state_ok={state}"),
        );
        assert!(axes.conforms(), "{site}/{mode}: {axes:?}");
        let fault = seen.fault.as_ref().unwrap();
        if site == "default-action-repair" {
            assert_eq!(
                fault["after"]["sigchld"]["noChildWait"],
                mode == "omit",
                "{site}/{mode}: actual SIGCHLD state"
            );
        }

        // The broken subject ignores that one failure and admits the worker.
        let broken = run_row(row, "ignore-fault");
        let axes = refused(
            &broken,
            "worker_failed",
            "cannot establish worker watching",
            site,
            mode,
            true,
        );
        // Detected: the worker was admitted although watching was never
        // established, and its own later failure is not the watching refusal.
        let detected =
            axes.controller && axes.site && !axes.conforms() && !axes.no_worker && !axes.text;
        record(&id, "broken:ignore-fault", !detected, &format!("{axes:?}"));
        assert!(axes.controller && axes.site, "{site}/{mode}: {axes:?}");
        // The worker was admitted although watching was never established; its
        // own later failure is a different report, not the watching refusal.
        assert!(
            !axes.conforms() && !axes.no_worker && !axes.text,
            "{site}/{mode}: ignoring the failure still refused: {axes:?}"
        );
    }
}

/// F04's selection-owned handlers. Install and query faults fail admission
/// before any worker or child; the receipt at the finalisation boundary names
/// which handlers the front still owns, and the mask it still holds.
#[test]
#[ignore = "private real-main fixture; run task test:native-entry"]
fn native_f04_selection_handler_faults_fail_admission_with_the_partial_subset_owned() {
    // (site, mode, owners of INT/TERM/HUP after the site)
    let rows = [
        ("action-query", "report", ["default", "default", "default"]),
        ("INT-install", "omit", ["default", "default", "default"]),
        ("INT-install", "perform", ["front", "default", "default"]),
        ("TERM-install", "omit", ["front", "default", "default"]),
        ("TERM-install", "perform", ["front", "front", "default"]),
    ];
    for (site, mode, owners) in rows {
        let id = format!(
            "F04__owner=selection__operation={site}__mode={}",
            mode_label(mode)
        );
        let row = json!({"family":"F04","owner":"selection","site":site,"mode":mode,
            "boundary":"before-finalise"});
        let seen = run_row(row.clone(), "none");
        let axes = refused(
            &seen,
            "cancellation_unavailable",
            "Input/output error",
            site,
            mode,
            false,
        );
        judged(&id, "repaired", || {
            assert!(axes.conforms(), "{site}/{mode}: {axes:?}");
            // The subset the front owns when it reaches finalisation, and that
            // the handled signals are still blocked: nothing released early.
            let facts = seen.ready.as_ref().expect("finalisation facts");
            let found = ["INT", "TERM", "HUP"]
                .map(|name| facts["owners"][name].as_str().unwrap().to_owned());
            assert_eq!(found, owners, "{site}/{mode}: partial installed subset");
            let handled = bit(libc::SIGINT) | bit(libc::SIGTERM) | bit(libc::SIGHUP);
            assert_eq!(
                facts["blocked"].as_u64().unwrap() & handled,
                handled,
                "{site}/{mode}: handled signals unblocked before finalisation"
            );
        });

        let broken = run_row(row, "ignore-fault");
        let axes = refused(
            &broken,
            "cancellation_unavailable",
            "Input/output error",
            site,
            mode,
            true,
        );
        assert!(axes.controller && axes.site, "{site}/{mode}: {axes:?}");
        // Admission proceeded: the worker ran and the failure left no refusal.
        detected(&id, "broken:ignore-fault", || {
            assert!(
                !axes.conforms() && !axes.no_worker && !axes.code,
                "{site}/{mode}: ignoring the failure still failed admission: {axes:?}"
            );
        });
    }
}

/// Rows whose failure lands at the common finalisation, after the work they
/// would otherwise report: the original outcome must survive beside a secondary
/// label, nothing may be written, and the failed site's physical state is the
/// operation's own, never the report's.
#[derive(Debug)]
struct Finalised {
    controller: bool,
    failed: bool,
    zero_result: bool,
    /// The original outcome (refusal / spawn failure) is still what was reported.
    original: bool,
    /// The secondary failure's label is in the diagnostics.
    secondary: bool,
    site: bool,
    /// What actually happened to the managed child before finalisation.
    child: bool,
}
impl Finalised {
    fn conforms(&self) -> bool {
        self.controller
            && self.failed
            && self.zero_result
            && self.original
            && self.secondary
            && self.site
            && self.child
    }
}
#[allow(clippy::too_many_arguments)]
fn finalised(
    seen: &Boundary,
    original: &str,
    secondary: &str,
    site: &str,
    mode: &str,
    broken: bool,
    child_ran: bool,
) -> Finalised {
    Finalised {
        controller: seen.settlement.clean()
            && seen.delivered.is_ok()
            && seen.barrier_failure.is_none(),
        failed: seen
            .settlement
            .status
            .is_some_and(|status| !status.success()),
        zero_result: seen.result_empty,
        original: original.is_empty() || seen.diagnostics.contains(original),
        secondary: seen.diagnostics.contains(secondary),
        site: fault_site_matches(seen, site, mode, broken),
        child: if child_ran {
            seen.managed_started
                && seen.worker_confirmed
                && seen.managed_group.as_deref() == Some("Gone")
        } else {
            !seen.managed_started
        },
    }
}

/// F04's final restore. The same front operation, at two finalisation
/// boundaries: only the front's handlers installed (policy refusal) and the
/// runner's handlers installed over them (a completed launch).
#[test]
#[ignore = "private real-main fixture; run task test:native-entry"]
fn native_f04_int_final_restore_failure_retains_ownership_and_blocks_the_result() {
    for (owner, result) in [("selection", "policy-refusal"), ("runner", "success")] {
        for mode in ["omit", "perform"] {
            let id = format!(
                "F04__owner={owner}__operation=INT-final-restore__mode={}",
                mode_label(mode)
            );
            let row = json!({"family":"F04","owner":owner,"site":"INT-final-restore",
                "mode":mode,"result":result,"boundary":"before-finalise"});
            let seen = run_row(row.clone(), "none");
            let child_ran = owner == "runner";
            let original = if owner == "selection" {
                "owner_declined"
            } else {
                ""
            };
            let axes = finalised(
                &seen,
                original,
                "disposition release",
                "INT-final-restore",
                mode,
                false,
                child_ran,
            );
            judged(&id, "repaired", || {
                assert!(axes.conforms(), "{owner}/{mode}: {axes:?}");
                // Who owns INT afterwards is the operation's, not the report's:
                // omitted left the installed owner, performed restored the entry
                // action. Either way the failed release keeps the mask blocked.
                let after = &seen.fault.as_ref().unwrap()["after"];
                let installed = if owner == "selection" {
                    "front"
                } else {
                    "other"
                };
                let expected = if mode == "omit" { installed } else { "default" };
                assert_eq!(after["INT"], expected, "{owner}/{mode}: actual INT owner");
                let handled = bit(libc::SIGINT) | bit(libc::SIGTERM) | bit(libc::SIGHUP);
                assert_eq!(after["blocked"].as_u64().unwrap() & handled, handled);
            });

            let broken = run_row(row, "ignore-fault");
            let axes = finalised(
                &broken,
                original,
                "disposition release",
                "INT-final-restore",
                mode,
                true,
                child_ran,
            );
            assert!(axes.controller && axes.site, "{owner}/{mode}: {axes:?}");
            detected(&id, "broken:ignore-fault", || {
                assert!(
                    !axes.conforms() && !axes.secondary,
                    "{owner}/{mode}: ignoring the failed release still reported it: {axes:?}"
                );
            });
        }
    }
}

/// F05's finalisation rows: three outcomes by the common finalisation's own
/// operations. Rows that query the pending set queue an actual TERM under the
/// block first, so uncertainty could only become "no cancellation" by a defect.
const F05_OUTCOMES: [(&str, &str, bool); 3] = [
    // outcome, text the original failure must keep, did the child run
    ("selection-success", "", true),
    ("policy-refusal", "owner_declined", false),
    ("spawn-ENOENT", "No such file or directory", false),
];
const F05_OPERATIONS: [(&str, &str, &str); 6] = [
    ("finalisation-block", "omit", "final signal snapshot"),
    ("finalisation-block", "perform", "final signal snapshot"),
    ("pending-query", "report", "final signal snapshot"),
    ("TERM-membership", "report", "final signal snapshot"),
    ("final-mask-restore", "omit", "final mask release"),
    ("final-mask-restore", "perform", "final mask release"),
];

fn f05_row(outcome: &str, site: &str, mode: &str, broken: &str) -> Boundary {
    let queued = matches!(site, "pending-query" | "TERM-membership");
    let mut row = json!({"family":"F05","outcome":outcome,"site":site,"mode":mode});
    if queued {
        row["boundary"] = json!("final-block-held");
        return drive(row, Some(libc::SIGTERM), broken);
    }
    run_row(row, broken)
}

#[test]
#[ignore = "private real-main fixture; run task test:native-entry"]
fn native_f05_finalisation_failures_keep_the_original_and_never_write_a_result() {
    let handled = bit(libc::SIGINT) | bit(libc::SIGTERM) | bit(libc::SIGHUP);
    for (outcome, original, child_ran) in F05_OUTCOMES {
        for (site, mode, secondary) in F05_OPERATIONS {
            let label = format!("{outcome}/{site}/{mode}");
            let id = format!(
                "F05__outcome={outcome}__operation={site}__mode={}",
                mode_label(mode)
            );
            let queued = matches!(site, "pending-query" | "TERM-membership");
            let seen = f05_row(outcome, site, mode, "none");
            let axes = finalised(&seen, original, secondary, site, mode, false, child_ran);
            judged(&id, "repaired", || {
                if queued {
                    // The signal was an actual pending TERM at the failed query;
                    // its only legitimate reports are a failure or the signal's
                    // own death. A success carrying a result is the defect.
                    let facts = seen.ack.as_ref().expect("acknowledged facts");
                    assert!(
                        facts["pending"].as_u64().unwrap() & bit(libc::SIGTERM) != 0,
                        "{label}: TERM was not actually pending at the query: {facts}"
                    );
                    // Unknown pending state keeps the handled signals blocked
                    // until the diagnostics are written. The incumbent restored
                    // the mask, so the queued TERM killed the launcher under its
                    // default action before the original or any secondary report
                    // existed.
                    assert!(axes.conforms(), "{label}: {axes:?}");
                    assert_eq!(seen.died_by, None, "{label}: killed before reporting");
                } else {
                    assert!(axes.conforms(), "{label}: {axes:?}");
                }
                // The physical state is the operation's own, never the report's.
                let after = &seen.fault.as_ref().unwrap()["after"];
                let blocked = after["blocked"].as_u64().unwrap() & handled;
                match (site, mode) {
                    ("finalisation-block", "omit") => assert_eq!(blocked, 0, "{label}"),
                    ("finalisation-block", "perform") => assert_eq!(blocked, handled, "{label}"),
                    ("final-mask-restore", "omit") => assert_eq!(blocked, handled, "{label}"),
                    ("final-mask-restore", "perform") => assert_eq!(blocked, 0, "{label}"),
                    _ => {}
                }
            });

            let broken = f05_row(outcome, site, mode, "ignore-fault");
            let axes = finalised(&broken, original, secondary, site, mode, true, child_ran);
            assert!(axes.controller && axes.site, "{label}: broken {axes:?}");
            detected(&id, "broken:ignore-fault", || {
                assert!(
                    !axes.conforms() && !axes.secondary,
                    "{label}: ignoring the failure still reported it: {axes:?}"
                );
                if queued {
                    // The ignored query lets the mask restore deliver the actual
                    // TERM: death by the signal with no report at all.
                    assert_eq!(broken.died_by, Some(libc::SIGTERM), "{label}");
                }
            });
        }
    }
}

/// The paired failure: the original policy refusal and the secondary final
/// mask restore failure. The refusal's own code and exit status stay the
/// reported failure; the restore failure is retained beside it, not instead.
#[test]
#[ignore = "private real-main fixture; run task test:native-entry"]
fn native_paired_f05_refusal_keeps_its_exit_beside_the_final_restore_failure() {
    const ID: &str =
        "paired__case=F05-refusal-final-restore-original__mode=omit-primary-report-secondary";
    let seen = f05_row("policy-refusal", "final-mask-restore", "omit", "none");
    judged(ID, "repaired", || {
        let outcome = seen.outcome.as_ref().expect("a failure outcome");
        assert!(seen.settlement.clean() && seen.result_empty, "{seen:?}");
        assert_eq!(outcome["error"]["code"], "policy_refused", "{outcome}");
        assert_eq!(outcome["error"]["exit"], 3, "{outcome}");
        assert!(
            seen.settlement.status.is_some_and(|s| s.code() == Some(3)),
            "{seen:?}"
        );
        assert!(
            seen.diagnostics
                .contains("final mask release: Input/output error"),
            "secondary failure lost: {}",
            seen.diagnostics
        );
        assert!(fault_site_matches(
            &seen,
            "final-mask-restore",
            "omit",
            false
        ));
        assert!(!seen.managed_started);
    });

    // The ignored restore failure leaves the refusal alone, with no trace of
    // the secondary: the pair assertion above cannot hold.
    let broken = f05_row(
        "policy-refusal",
        "final-mask-restore",
        "omit",
        "ignore-fault",
    );
    assert!(broken.settlement.clean(), "{broken:?}");
    detected(ID, "broken:ignore-fault", || {
        assert!(
            !broken.diagnostics.contains("final mask release"),
            "{broken:?}"
        );
    });
}

/// F05's Started parent-mask restore: a live real child, its group and a PTY
/// terminal lease. The failed check must enter bounded real stop, wait and
/// terminal recovery before returning, never lend the terminal again, and the
/// front must still finalise and write nothing.
#[test]
#[ignore = "private real-main fixture; run task test:native-entry"]
fn native_f05_started_restore_failure_cleans_up_the_live_child_and_terminal() {
    let handled = bit(libc::SIGINT) | bit(libc::SIGTERM) | bit(libc::SIGHUP);
    for mode in ["omit", "perform"] {
        let id = format!(
            "F05__outcome=selection-success__operation=Started-parent-restore__mode={}",
            mode_label(mode)
        );
        let row = json!({"family":"F05","outcome":"selection-success","site":"Started-parent-restore",
            "mode":mode,"pty":true,"boundary":"before-finalise"});
        let seen = run_row(row.clone(), "none");
        judged(&id, "repaired", || {
            let facts = seen.ready.as_ref().expect("finalisation facts");
            let at = |key: &str| facts[key].as_i64().unwrap();
            let outcome = seen.outcome.as_ref().expect("a failure outcome");
            assert!(
                seen.settlement.clean() && seen.delivered.is_ok(),
                "{mode}: {seen:?}"
            );
            assert!(
                seen.settlement.status.is_some_and(|s| !s.success()),
                "{mode}"
            );
            assert!(seen.result_empty, "{mode}: bytes were written");
            assert_eq!(
                outcome["error"]["code"], "supervision_failed",
                "{mode}: {outcome}"
            );
            // The live child was started once and the runner confirmed its actual
            // reap during its own error cleanup; no ending was ever produced.
            assert!(seen.managed_started, "{mode}");
            assert!(seen.worker_confirmed, "{mode}");
            assert!(
                facts["managedReaped"] == true,
                "{mode}: no confirmed reap: {facts}"
            );
            assert!(seen.managed_end.is_none(), "{mode}");
            // The terminal is the launcher's again, and nothing is left unreaped.
            assert_eq!(
                at("foreground"),
                at("pgrp"),
                "{mode}: terminal not returned"
            );
            assert_eq!(at("strayChild"), 0, "{mode}: a child remains");
            assert!(
                fault_site_matches(&seen, "Started-parent-restore", mode, false),
                "{mode}"
            );
            // The restore's own physical state, not its report.
            let blocked = seen.fault.as_ref().unwrap()["after"]["blocked"]
                .as_u64()
                .unwrap()
                & handled;
            assert_eq!(blocked, if mode == "omit" { handled } else { 0 }, "{mode}");
        });

        // Ignoring the failed restore leaves a launch that simply completes.
        let broken = run_row(row, "ignore-fault");
        assert!(broken.settlement.clean(), "{mode}: {broken:?}");
        detected(&id, "broken:ignore-fault", || {
            assert!(
                !broken.result_empty && broken.settlement.status.is_some_and(|s| s.success()),
                "{mode}: ignoring the failure still failed the launch: {broken:?}"
            );
        });
    }
}

/// The queued-TERM variant of the Started restore. TERM is delivered while the
/// parent's handoff block still holds (so it is pending, never consumed), the
/// restore is then omitted and reported failed, and the common finalisation
/// must join the pending signal with the original failure retained under it.
#[test]
#[ignore = "private real-main fixture; run task test:native-entry"]
fn native_f05_started_restore_failure_with_queued_term_joins_at_finalisation() {
    let handled = bit(libc::SIGINT) | bit(libc::SIGTERM) | bit(libc::SIGHUP);
    let row = json!({"family":"F05","outcome":"selection-success","site":"Started-parent-restore",
        "mode":"omit","pty":true,"boundary":"started-before-mask-restore"});
    const ID: &str =
        "cancellation__case=F05-Started-restore__mode=omit-EIO__signal=parent-blocked-pending-TERM";
    let seen = drive_row(row.clone(), libc::SIGTERM, "none");
    judged(ID, "repaired", || {
        assert!(
            seen.settlement.clean() && seen.delivered.is_ok(),
            "{seen:?}"
        );
        // The controller's TERM was an actual pending signal under the block: the
        // handler never ran, so nothing consumed it before finalisation.
        let ready = seen.ready.as_ref().expect("ready facts");
        let ack = seen.ack.as_ref().expect("acknowledged facts");
        assert_eq!(
            ready["blocked"].as_u64().unwrap() & handled,
            handled,
            "{ready}"
        );
        assert_ne!(
            ack["pending"].as_u64().unwrap() & bit(libc::SIGTERM),
            0,
            "{ack}"
        );
        assert!(
            ack["frontLatch"].is_null() && ack["managedStarted"] == true,
            "{ack}"
        );
        // Joined at finalisation: death by the selected signal, no bytes, the
        // original failure still reported, and the live child cleaned up first.
        assert_eq!(seen.died_by, Some(libc::SIGTERM), "{seen:?}");
        assert!(seen.result_empty);
        assert!(
            seen.outcome
                .as_ref()
                .is_some_and(|v| v["error"]["signal"] == "SIGTERM"),
            "{seen:?}"
        );
        assert!(
            seen.diagnostics
                .contains("Started parent transition failed"),
            "original failure lost under cancellation: {}",
            seen.diagnostics
        );
        assert!(seen.managed_started && seen.managed_end.is_none());
        assert!(fault_site_matches(
            &seen,
            "Started-parent-restore",
            "omit",
            false
        ));
    });

    // Ignoring the failed restore lets the child run to its end, and the
    // original failure never exists to be retained.
    let broken = drive_row(row, libc::SIGTERM, "ignore-fault");
    assert!(broken.settlement.clean(), "{broken:?}");
    detected(ID, "broken:ignore-fault", || {
        assert!(
            !broken
                .diagnostics
                .contains("Started parent transition failed"),
            "{broken:?}"
        );
    });
}

/// The queued-TERM variant of F06's action failure: TERM delivered while the
/// parent's handoff block holds, the child's TERM action restore omitted and
/// reported failed, and the spawn failure joined with the pending signal at
/// the front's finalisation. Runs the real runner's child path.
#[test]
#[ignore = "private real-main fixture; run task test:native-entry"]
fn native_f06_action_failure_with_queued_term_joins_at_finalisation() {
    let handled = bit(libc::SIGINT) | bit(libc::SIGTERM) | bit(libc::SIGHUP);
    let row = json!({"family":"F06","site":"TERM-action-restore","mode":"omit",
        "boundary":"handoff-block-held"});
    const ID: &str =
        "cancellation__case=F06-TERM-action__mode=omit-EIO__signal=parent-blocked-pending-TERM";
    let seen = drive_row(row.clone(), libc::SIGTERM, "none");
    judged(ID, "repaired", || {
        assert!(
            seen.settlement.clean() && seen.delivered.is_ok(),
            "{seen:?}"
        );
        let ready = seen.ready.as_ref().expect("ready facts");
        let ack = seen.ack.as_ref().expect("acknowledged facts");
        assert_eq!(
            ready["blocked"].as_u64().unwrap() & handled,
            handled,
            "{ready}"
        );
        assert_ne!(
            ack["pending"].as_u64().unwrap() & bit(libc::SIGTERM),
            0,
            "{ack}"
        );
        assert!(ack["frontLatch"].is_null(), "{ack}");
        // The failed spawn consumed nothing; finalisation joins the pending TERM.
        assert_eq!(seen.died_by, Some(libc::SIGTERM), "{seen:?}");
        assert!(seen.result_empty);
        assert!(
            seen.outcome
                .as_ref()
                .is_some_and(|v| v["error"]["signal"] == "SIGTERM"),
            "{seen:?}"
        );
        assert!(
            seen.diagnostics.contains("cannot spawn")
                && seen.diagnostics.contains("Input/output error"),
            "original spawn failure lost under cancellation: {}",
            seen.diagnostics
        );
        assert!(!seen.managed_started && seen.managed_end.is_none());
        assert!(seen.worker_confirmed, "worker teardown was not confirmed");
        // The child's own record, written before exit: operation omitted.
        let record = seen.child_record.clone().unwrap_or_default();
        assert_eq!(
            record.len(),
            2,
            "the child never reached the site: {record:?}"
        );
        assert_eq!(record[1] & 1, 0, "the omitted restore was performed");
    });

    // Ignoring the action failure runs the child, and the original spawn
    // failure never exists to be retained under the cancellation.
    let broken = drive_row(row, libc::SIGTERM, "ignore-fault");
    assert!(broken.settlement.clean(), "{broken:?}");
    detected(ID, "broken:ignore-fault", || {
        assert!(
            !broken.diagnostics.contains("cannot spawn"),
            "ignoring the failure still failed the spawn: {broken:?}"
        );
    });
}

// ---- E08: readers, creator masks and an escaped pipe-holder -----------------
//
// The private worker (this executable in its worker role) speaks the real
// protocol. For the escaped-holder rows it first hands its stdout to the
// controller, which spawns and owns a holder on that pipe. A process-directed
// TERM is then delivered at the common finalisation barrier, after the readers
// were collected, while the shielded readers may still be alive.

fn reader_row(readers: &str, result: &str, broken: &str) -> Boundary {
    drive_row(
        json!({"family":"E08","boundary":"before-finalise","signal":libc::SIGTERM,
               "readers":readers,"result":result}),
        libc::SIGTERM,
        broken,
    )
}

#[derive(Debug, PartialEq, Eq)]
struct ReaderAxes {
    controller: bool,
    cancelled: bool,
    zero_result: bool,
    /// Both readers' own masks, taken on their threads, blocked every handled signal.
    shielded: bool,
    /// The creating thread regained exactly the mask it entered with.
    creator_restored: bool,
    /// Completed versus still-blocked readers and the bounded collection.
    reader_state: bool,
    /// The front's handler ran on the main thread (front-owned rows only).
    main_owns: bool,
}
impl ReaderAxes {
    fn conforms(&self) -> bool {
        self.controller
            && self.cancelled
            && self.zero_result
            && self.shielded
            && self.creator_restored
            && self.reader_state
            && self.main_owns
    }
}

fn judge_readers(readers: &str, result: &str, seen: &Boundary) -> ReaderAxes {
    let escaped = readers == "one-escaped-holder";
    let ack = seen.ack.as_ref();
    let facts = ack.map(|v| &v["readers"]);
    let masks = facts
        .and_then(|v| v["readers"].as_array())
        .cloned()
        .unwrap_or_default();
    let raw = |which: &str| facts.map(|v| v[which]["raw"].clone());
    let finished: Vec<bool> = facts
        .and_then(|v| v["finished"].as_array())
        .map(|list| list.iter().filter_map(Value::as_bool).collect())
        .unwrap_or_default();
    let collect = facts.map(|v| &v["collect"]);
    let number = |name: &str| collect.and_then(|v| v[name].as_u64());
    // The holder holds the worker's stdout only: stderr's reader completes and
    // stdout's remains blocked until its owner kills the holder.
    let state = if escaped {
        finished == [false]
            && number("streams") == Some(2)
            && number("completed") == Some(1)
            // Bounded by the existing one-second drain window, never a join.
            && number("windowMs").is_some_and(|ms| ms <= 1000)
            && number("tookMs")
                .zip(number("windowMs"))
                .is_some_and(|(took, window)| took + 50 >= window && took <= window + 500)
    } else {
        let mut both = finished.clone();
        both.sort_unstable();
        both == [false, true]
            && number("streams") == Some(2)
            && number("completed") == Some(2)
            && number("tookMs").is_some_and(|ms| ms < 1000)
    };
    let holders_ok = if escaped {
        seen.settlement.holders.len() == 1
            && seen.settlement.holders[0].1
            && seen.settlement.stopped_by_controller
    } else {
        seen.settlement.holders.is_empty()
    };
    // A refusal keeps the front's handlers; a selected launch leaves the
    // runner's installed, so only the former has a front handler to observe.
    let owner = ack.and_then(|v| v["owners"]["TERM"].as_str());
    let main_owns = if result == "policy-refusal" {
        owner == Some("front")
            && ack.is_some_and(|v| {
                v["frontLatch"] == "SIGTERM"
                    && v["handlerThread"].as_u64() != Some(0)
                    && v["handlerThread"] == v["tid"]
            })
    } else {
        owner == Some("other") && ack.is_some_and(|v| v["frontLatch"].is_null())
    };
    ReaderAxes {
        controller: seen.settlement.clean()
            && seen.delivered.is_ok()
            && seen.barrier_failure.is_none()
            && holders_ok,
        cancelled: seen.died_by == Some(libc::SIGTERM)
            && seen
                .outcome
                .as_ref()
                .is_some_and(|v| v["error"]["signal"] == "SIGTERM"),
        zero_result: seen.result_empty,
        shielded: masks.len() == 2 && masks.iter().all(|m| m["mask"]["handled"] == 7),
        creator_restored: raw("creatorBefore").is_some_and(|b| !b.is_null())
            && raw("creatorBefore") == raw("creatorAfter"),
        reader_state: state,
        main_owns,
    }
}

#[test]
#[ignore = "private real-main fixture; run task test:native-entry"]
fn native_e08_reader_shield_collection_bound_and_escaped_holder_ownership() {
    for readers in ["both-complete", "one-escaped-holder"] {
        for result in ["selection-success", "policy-refusal"] {
            let id = format!(
                "E08__readers={}__result={result}",
                match readers {
                    "one-escaped-holder" => "one-escaped-holder",
                    other => other,
                }
            );
            let seen = reader_row(readers, result, "none");
            let axes = judge_readers(readers, result, &seen);
            record(&id, "repaired", axes.conforms(), &format!("{axes:?}"));
            assert!(axes.conforms(), "{readers}/{result}: {axes:?}");

            // Breaking only the shield must fail the eligibility receipt and
            // nothing the controller owns.
            let broken = reader_row(readers, result, "shielding");
            let axes = judge_readers(readers, result, &broken);
            // Detected: only the eligibility receipt fails; everything the
            // controller owns, the cancellation and the result stay intact.
            let caught = axes.controller && axes.cancelled && axes.zero_result && !axes.shielded;
            record(&id, "broken:shielding", !caught, &format!("{axes:?}"));
            assert!(axes.controller, "{readers}/{result}: {axes:?}");
            assert!(
                axes.cancelled && axes.zero_result,
                "{readers}/{result}: {axes:?}"
            );
            assert!(
                !axes.shielded,
                "{readers}/{result}: broken shielding passed the eligibility receipt"
            );
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
