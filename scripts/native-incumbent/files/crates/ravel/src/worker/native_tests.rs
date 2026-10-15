//! Reader receipts for the incumbent's worker owner. Test builds only.
//!
//! A port of the observation half of the repaired `worker/native_tests.rs`: it
//! records the creating thread's mask around reader creation, each reader's own
//! mask taken on its thread, which readers reached the end of their stream, and
//! the bounded collection. It injects nothing; the repaired file's fault
//! injection acts on mechanisms the incumbent does not have.
use super::*;
use std::cell::RefCell;

#[derive(Clone, Debug)]
pub(super) struct MaskReceipt {
    raw: Vec<u8>,
    handled: u8,
}
type ReaderSink = Arc<Mutex<Vec<(bool, MaskReceipt)>>>;
type FinishedSink = Arc<Mutex<Vec<bool>>>;

#[derive(Default)]
struct Control {
    observing: bool,
    reader_masks: ReaderSink,
    creator_before: Option<MaskReceipt>,
    creator_after: Option<MaskReceipt>,
    finished: FinishedSink,
    collected: Option<(usize, usize, Duration, Duration)>,
}
thread_local! { static CONTROL: RefCell<Control> = RefCell::new(Control::default()); }

fn mask_receipt() -> MaskReceipt {
    let mut mask = unsafe { std::mem::zeroed::<libc::sigset_t>() };
    assert_eq!(
        unsafe { libc::pthread_sigmask(libc::SIG_BLOCK, std::ptr::null(), &mut mask) },
        0
    );
    let mut handled = 0;
    for (index, signal) in [libc::SIGINT, libc::SIGTERM, libc::SIGHUP]
        .into_iter()
        .enumerate()
    {
        let member = unsafe { libc::sigismember(&mask, signal) };
        assert!(member == 0 || member == 1);
        handled |= (member as u8) << index;
    }
    let raw = unsafe {
        std::slice::from_raw_parts(
            (&mask as *const libc::sigset_t).cast::<u8>(),
            std::mem::size_of_val(&mask),
        )
    }
    .to_vec();
    MaskReceipt { raw, handled }
}
fn active() -> bool {
    CONTROL.with(|c| c.borrow().observing)
}
pub(super) fn creator_before() {
    if active() {
        CONTROL.with(|c| c.borrow_mut().creator_before = Some(mask_receipt()));
    }
}
pub(super) fn creator_after() {
    if active() {
        CONTROL.with(|c| c.borrow_mut().creator_after = Some(mask_receipt()));
    }
}
pub(super) fn reader_sink() -> Option<ReaderSink> {
    active().then(|| CONTROL.with(|c| Arc::clone(&c.borrow().reader_masks)))
}
pub(super) fn record_reader(sink: Option<ReaderSink>, stdout: bool) {
    if let Some(sink) = sink {
        sink.lock().unwrap().push((stdout, mask_receipt()));
    }
}
pub(super) fn finished_sink() -> Option<FinishedSink> {
    active().then(|| CONTROL.with(|c| Arc::clone(&c.borrow().finished)))
}
/// A reader reached the end of its stream. Recorded before it reports done.
pub(super) fn record_finished(sink: Option<FinishedSink>, stdout: bool) {
    if let Some(sink) = sink {
        sink.lock().unwrap().push(stdout);
    }
}
/// The bounded collection's own receipt: how many readers it waited for, how
/// many completed within its window, the window and the time it actually took.
pub(super) fn collected(streams: usize, completed: usize, window: Duration, took: Duration) {
    if active() {
        CONTROL.with(|c| c.borrow_mut().collected = Some((streams, completed, window, took)));
    }
}
fn mask_json(mask: &MaskReceipt) -> serde_json::Value {
    json!({"handled": mask.handled, "raw": mask.raw})
}
/// Everything E08 reads at a boundary. Read-only.
pub(crate) fn reader_receipt() -> serde_json::Value {
    CONTROL.with(|c| {
        let c = c.borrow();
        let readers: Vec<_> = c
            .reader_masks
            .lock()
            .unwrap()
            .iter()
            .map(|(stdout, mask)| json!({"stdout": stdout, "mask": mask_json(mask)}))
            .collect();
        let finished: Vec<bool> = c.finished.lock().unwrap().clone();
        json!({
            "creatorBefore": c.creator_before.as_ref().map(mask_json),
            "creatorAfter": c.creator_after.as_ref().map(mask_json),
            "readers": readers,
            "finished": finished,
            "collect": c.collected.map(|(streams, completed, window, took)| json!({
                "streams": streams, "completed": completed,
                "windowMs": window.as_millis() as u64, "tookMs": took.as_millis() as u64,
            })),
        })
    })
}
/// Observation for the private real-main fixture, injecting nothing.
pub(crate) fn observe() {
    CONTROL.with(|c| {
        *c.borrow_mut() = Control {
            observing: true,
            ..Control::default()
        }
    });
}

/// E08's private worker: the real protocol, with a deliberately escaped
/// pipe-holder. Runs from the real-main fixture binary, never installed.
///
/// With a holder socket it first hands its stdout to the *controller*, which
/// spawns and owns a holder process on that pipe and acknowledges. The worker
/// then finishes the conversation and exits, so the pipe's last writer is the
/// controller's holder, not anything the product owns.
pub(crate) fn e08_worker() -> ! {
    let selection: Value =
        serde_json::from_slice(&fs::read(std::env::var_os("K28_E08_SELECTION").unwrap()).unwrap())
            .unwrap();
    let mut channel = unsafe { UnixStream::from_raw_fd(CHANNEL_FD) };
    write_frame(
        &mut channel,
        &json!({"type":"hello","protocol":1,"packageVersion":PACKAGE_VERSION,"buildId":BUILD_ID,"bunVersion":"1.4.2"}),
    )
    .unwrap();
    read_frame(&mut channel, 1_048_576).unwrap();
    println!("owner stdout");
    eprintln!("owner stderr");
    if let Some(socket) = std::env::var_os("K28_E08_HOLDER") {
        if let Err(error) = crate::native_barrier::send_descriptor(Path::new(&socket), 1) {
            eprintln!("e08 worker: holder handoff failed: {error}");
            std::process::exit(70);
        }
    }
    write_frame(
        &mut channel,
        &json!({"type":"policy","policy":{"schemaVersion":2,"version":"native-entry","select":{"$ravel":"function"}}}),
    )
    .unwrap();
    read_frame(&mut channel, 1_048_576).unwrap();
    write_frame(
        &mut channel,
        &json!({"type":"selection","result":selection}),
    )
    .unwrap();
    // The front closes the protocol after reading the selection.
    let mut byte = [0];
    let _ = channel.read(&mut byte);
    std::process::exit(0);
}

// ---- F03: the shield's block, the reader creations and the creator's restore --
//
// The incumbent performs exactly these four operations around its readers:
// `shielded` blocks the handled signals on the creating thread, the two
// `thread::spawn` calls create the readers, and `shielded` puts the creator's
// mask back. It reads neither mask result, and `thread::spawn` has no failure
// return (std panics). The fixture places the row's one fault at the operation
// the incumbent performs and lets the incumbent do what it does; it adds no
// check. A reader's or the creator's mask is observed, never changed.

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum F03 {
    #[default]
    None,
    BlockOmitted,
    BlockPerformed,
    FirstCreate,
    SecondCreate,
    RestoreOmitted,
    RestorePerformed,
}
thread_local! { static F03_ARMED: std::cell::Cell<F03> = const { std::cell::Cell::new(F03::None) }; }

fn f03() -> F03 {
    F03_ARMED.get()
}
/// Whether the armed row is one where the incumbent returns from the front
/// (a mask fault it never reads), rather than dying in `thread::spawn`.
pub(crate) fn returns_normally() -> bool {
    matches!(
        f03(),
        F03::BlockOmitted | F03::BlockPerformed | F03::RestoreOmitted | F03::RestorePerformed
    )
}
/// The shield's block, with the row's substituted report (a `pthread_sigmask`
/// result is the error number itself).
pub(crate) fn shield_block(real: impl FnOnce() -> libc::c_int) -> libc::c_int {
    match f03() {
        F03::BlockOmitted => {
            eprintln!("native baseline F03 block: performed=false reported=EIO");
            libc::EIO
        }
        F03::BlockPerformed => {
            let actual = real();
            eprintln!("native baseline F03 block: performed=true actual={actual} reported=EIO");
            libc::EIO
        }
        _ => real(),
    }
}
/// The creator's mask restore, with the row's substituted report.
pub(crate) fn creator_restore(real: impl FnOnce() -> libc::c_int) -> libc::c_int {
    match f03() {
        F03::RestoreOmitted => {
            eprintln!("native baseline F03 restore: performed=false reported=EIO");
            libc::EIO
        }
        F03::RestorePerformed => {
            let actual = real();
            eprintln!("native baseline F03 restore: performed=true actual={actual} reported=EIO");
            libc::EIO
        }
        _ => real(),
    }
}
/// What the incumbent's readers and creator looked like, as one line.
pub(super) fn f03_receipts() -> String {
    CONTROL.with(|c| {
        let c = c.borrow();
        let readers = c.reader_masks.lock().unwrap();
        format!(
            "creator_before_handled={:?} creator_after_handled={:?} readers_handled={:?}",
            c.creator_before.as_ref().map(|m| m.handled),
            c.creator_after.as_ref().map(|m| m.handled),
            readers.iter().map(|(_, m)| m.handled).collect::<Vec<_>>(),
        )
    })
}

// ---- S03: second reader creation fails, with a secondary fault armed -------
//
// The repaired worker creates its readers with a checked `Builder::spawn`, and
// turns a failed second creation into a teardown whose stop, kill and wait the
// row then faults. The incumbent creates them with `thread::spawn`, which has
// no failure return: std documents a panic if the OS fails to create a thread
// (https://doc.rust-lang.org/std/thread/fn.spawn.html), and nothing in the
// front catches it. The fixture injects the primary where the incumbent
// performs it and lets the incumbent do what std does, then arms the row's
// secondary fault at the stop, kill and poll operations of the incumbent's own
// teardown, so that "never reached" is observed, not assumed: a reached
// secondary writes `native baseline S03 secondary reached` to stderr.

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum S03 {
    #[default]
    None,
    StopOmitted,
    StopPerformed,
    KillOmitted,
    KillPerformed,
    PollEio,
    PollEintr,
    PollLost,
    Running,
}
thread_local! { static S03_ARMED: std::cell::Cell<S03> = const { std::cell::Cell::new(S03::None) }; }

pub(super) fn arm_s03(fault: &str) {
    // F03 rows, and the paired F03 row, which is S03's wait-report case. A
    // fixture that arms any row also observes its readers, injecting nothing.
    F03_ARMED.set(match fault {
        "block-omitted" => F03::BlockOmitted,
        "block-performed" => F03::BlockPerformed,
        "first-create" => F03::FirstCreate,
        "second-create" => F03::SecondCreate,
        "restore-omitted" => F03::RestoreOmitted,
        "restore-performed" => F03::RestorePerformed,
        _ => F03::None,
    });
    if f03() != F03::None {
        // Observation only: readers' and creator's masks, nothing injected.
        CONTROL.with(|c| {
            *c.borrow_mut() = Control {
                observing: true,
                ..Control::default()
            }
        });
        return;
    }
    S03_ARMED.set(match fault {
        "stop-omitted" => S03::StopOmitted,
        "stop-performed" => S03::StopPerformed,
        "kill-omitted" => S03::KillOmitted,
        "kill-performed" => S03::KillPerformed,
        "poll-EIO" => S03::PollEio,
        "poll-EINTR" => S03::PollEintr,
        "poll-ECHILD" => S03::PollLost,
        "always-running" => S03::Running,
        other => panic!("unknown fixed S03 case {other}"),
    });
}
fn armed() -> S03 {
    S03_ARMED.get()
}
fn reached(operation: &str) {
    eprintln!("native baseline S03 secondary reached: {operation}");
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

/// Called where the incumbent creates a reader, before `thread::spawn`. The
/// second reader (stderr) fails the way std reports an OS refusal: its
/// `thread::spawn` is `Builder::spawn(..).expect("failed to spawn thread")`.
pub(super) fn before_reader_create(stdout: bool) {
    // F03's creations: the first reader (stdout) or the second (stderr).
    match (f03(), stdout) {
        (F03::FirstCreate, true) | (F03::SecondCreate, false) => {
            eprintln!(
                "native baseline F03: the {} reader creation is refused (EAGAIN)",
                if stdout { "first" } else { "second" }
            );
            Err::<(), io::Error>(io::Error::from_raw_os_error(libc::EAGAIN))
                .expect("failed to spawn thread");
        }
        _ => {}
    }
    if armed() == S03::None || stdout {
        return;
    }
    // The harness control that leaves the secondary armed without the primary.
    if std::env::var_os("C2_S03_PRIMARY").is_some_and(|mode| mode == "off") {
        return;
    }
    eprintln!("native baseline S03: the second reader creation is refused (EAGAIN)");
    Err::<(), io::Error>(io::Error::from_raw_os_error(libc::EAGAIN))
        .expect("failed to spawn thread");
}
pub(super) fn stop(pid: i32, signal: i32) -> i32 {
    let mode = match (armed(), signal) {
        (S03::StopOmitted, libc::SIGTERM) => Some(false),
        (S03::StopPerformed, libc::SIGTERM) => Some(true),
        _ => None,
    };
    let Some(performed) = mode else {
        return unsafe { libc::kill(pid, signal) };
    };
    reached("PID stop");
    if performed {
        unsafe { libc::kill(pid, signal) };
    }
    set_errno(libc::EPERM);
    -1
}
pub(super) fn kill(child: &mut Child) -> io::Result<()> {
    match armed() {
        mode @ (S03::KillOmitted | S03::KillPerformed) => {
            reached("PID kill");
            if mode == S03::KillPerformed {
                let _ = child.kill();
            }
            Err(io::Error::from_raw_os_error(libc::EPERM))
        }
        _ => child.kill(),
    }
}
pub(super) fn poll(child: &mut Child) -> io::Result<Option<ExitStatus>> {
    match armed() {
        S03::PollEio => {
            reached("try_wait");
            Err(io::Error::from_raw_os_error(libc::EIO))
        }
        S03::PollEintr => {
            reached("try_wait");
            Err(io::Error::from_raw_os_error(libc::EINTR))
        }
        S03::PollLost => {
            reached("try_wait");
            Err(io::Error::from_raw_os_error(libc::ECHILD))
        }
        S03::Running => {
            reached("try_wait");
            Ok(None)
        }
        _ => child.try_wait(),
    }
}

/// What one S03 role run left behind.
#[derive(Debug)]
struct S03Run {
    status: std::process::ExitStatus,
    log: String,
    started: bool,
    /// What the role's redirected result descriptor holds. The libtest harness
    /// prints its own failure text there when the role panics, so the product's
    /// result is judged by the absence of any selection output.
    result: String,
    elapsed: Duration,
    /// Whether the incumbent's worker was still alive after the role had died.
    orphan_survived: bool,
}

fn s03_run(fault: &str, primary: bool) -> S03Run {
    let exe = std::env::current_exe().unwrap();
    let worker = exe
        .ancestors()
        .nth(3)
        .unwrap()
        .join("libexec/ravel/ravel-policy");
    assert!(worker.is_file(), "run task worker: {}", worker.display());
    let root = tempfile::tempdir().unwrap();
    let bin = root.path().join("bin");
    let libexec = root.path().join("libexec/ravel");
    fs::create_dir_all(&bin).unwrap();
    fs::create_dir_all(&libexec).unwrap();
    let fixture = bin.join("fixture");
    fs::copy(&exe, &fixture).unwrap();
    fs::copy(&worker, libexec.join("ravel-policy")).unwrap();
    fs::create_dir(root.path().join("home")).unwrap();
    fs::create_dir(root.path().join("tmp")).unwrap();
    let started = Instant::now();
    let mut command = Command::new(&fixture);
    command
        .args([
            "--exact",
            "worker::admission_tests::worker_reap_admission_fixture",
            "--ignored",
            "--nocapture",
        ])
        .env_clear()
        .env("PATH", "/usr/bin:/bin")
        .env("HOME", root.path().join("home"))
        .env("TMPDIR", root.path().join("tmp"))
        .env("C2_SCENARIO", "native-reader")
        .env("C2_FAULT", fault)
        .env("C2_ROUTE", "front")
        .env("C2_S03_PRIMARY", if primary { "on" } else { "off" })
        .env("C2_FRONT", exe.ancestors().nth(2).unwrap().join("ravel"))
        .current_dir(root.path())
        .process_group(0)
        .stdout(Stdio::from(
            fs::File::create(root.path().join("fixture-stdout")).unwrap(),
        ))
        .stderr(Stdio::from(
            fs::File::create(root.path().join("fixture-stderr")).unwrap(),
        ));
    let mut child = command.spawn().unwrap();
    let pid = libc::pid_t::try_from(child.id()).unwrap();
    let by = Instant::now() + Duration::from_secs(8);
    let status = loop {
        match child.try_wait().unwrap() {
            Some(status) => break status,
            None if Instant::now() >= by => {
                // The fixture is still unreaped and reserves its group's ID.
                unsafe { libc::kill(-pid, libc::SIGKILL) };
                let _ = child.wait();
                panic!("S03 fixture exceeded its bound ({fault})");
            }
            None => std::thread::sleep(Duration::from_millis(5)),
        }
    };
    let elapsed = started.elapsed();
    // The worker is a child of the role and shares its group. The role is
    // reaped, so only a worker the incumbent did not end can keep the group
    // present: observe that, then end it, bounded, so no row leaves a process.
    let present = || {
        // SAFETY: signal 0 only probes the group the fixture created.
        unsafe { libc::kill(-pid, 0) == 0 }
    };
    std::thread::sleep(Duration::from_millis(300));
    let orphan_survived = present();
    if orphan_survived {
        // SAFETY: the group is the fixture's own and is still reserved by a member.
        unsafe { libc::kill(-pid, libc::SIGKILL) };
        let by = Instant::now() + Duration::from_secs(2);
        while present() {
            assert!(
                Instant::now() < by,
                "S03 fixture could not end the incumbent's orphaned worker ({fault})"
            );
            std::thread::sleep(Duration::from_millis(5));
        }
    }
    S03Run {
        status,
        log: fs::read_to_string(root.path().join("fixture-stderr")).unwrap_or_default(),
        started: root.path().join("started").exists(),
        result: fs::read_to_string(root.path().join("result-output")).unwrap_or_default(),
        elapsed,
        orphan_survived,
    }
}

/// The fixed role case a manifest row names.
fn s03_fault(id: &str) -> &'static str {
    let part = |name: &str| {
        id.split("__")
            .find_map(|part| part.strip_prefix(&format!("{name}=")))
            .unwrap_or("")
            .to_owned()
    };
    match (part("operation").as_str(), part("mode").as_str()) {
        ("PID-stop-EPERM", "omit") => "stop-omitted",
        ("PID-stop-EPERM", "perform-report") => "stop-performed",
        ("PID-kill-EPERM", "omit") => "kill-omitted",
        ("PID-kill-EPERM", "perform-report") => "kill-performed",
        ("try-wait-EIO", "report") => "poll-EIO",
        ("EINTR-until-deadline", "report") => "poll-EINTR",
        ("ECHILD", "report") => "poll-ECHILD",
        ("always-running", "report") => "always-running",
        other => panic!("{id}: {other:?} is not a fixed S03 case"),
    }
}

fn s03_manifest_ids() -> Vec<String> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/native-entry-cases.json");
    let manifest: serde_json::Value =
        serde_json::from_slice(&fs::read(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display())))
            .unwrap();
    manifest["rows"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|row| row["family"] == "S03")
        .map(|row| row["id"].as_str().unwrap().to_owned())
        .collect()
}

#[test]
#[ignore = "private real-process fixture; run native-incumbent.sh test"]
fn native_baseline_s03_second_reader_creation_failure_with_a_secondary_fault() {
    let ids = s03_manifest_ids();
    assert_eq!(ids.len(), 8);
    for id in ids {
        let fault = s03_fault(&id);
        let seen = s03_run(fault, true);
        // Instrument integrity: the primary must have been injected where the
        // incumbent creates its second reader, and the process must have ended
        // there, or the row did not exercise its premise.
        assert!(
            seen.log.contains("the second reader creation is refused")
                && seen.log.contains("failed to spawn thread"),
            "{id}: the primary was not injected: {seen:?}"
        );
        // Whether the teardown the secondary fault lives in ran at all: the
        // line the incumbent's teardown prints once it has stopped and reaped
        // the worker, and whether any armed secondary was reached.
        let teardown_ran = seen.log.contains("physical worker pid=");
        let secondary = seen.log.contains("native baseline S03 secondary reached");
        // The named claims read the failure the front reports. The incumbent
        // reported none: the process ended by the panic, so each report claim
        // has no text to be found in, and the one the row names is unattempted.
        let reported = !seen.status.success() && teardown_ran;
        let mut unmet = vec![
            "reader-admission-failure-reported",
            "worker-identity-reported",
            "original-creation-failure-retained",
        ];
        unmet.push(match fault {
            "stop-omitted" | "stop-performed" => "stop-report-retained",
            "kill-omitted" | "kill-performed" => "kill-report-retained",
            "poll-EIO" => "wait-report-retained",
            "poll-EINTR" | "always-running" => "absolute-deadline",
            "poll-ECHILD" => "ownership-loss-reported",
            _ => unreachable!(),
        });
        let held = ["no-managed-start", "result-stdout-empty"];
        // No managed start, and no selection result on stdout: the only text
        // there is the libtest harness reporting the role's own panic.
        assert!(
            !seen.started && !seen.result.contains("\"status\""),
            "{id}: {seen:?}"
        );
        let conforms = reported && unmet.is_empty();
        crate::native_barrier::row(
            &id,
            "baseline",
            conforms,
            &format!(
                "unmet {unmet:?}; held {held:?}; exit {}; teardown_ran={teardown_ran}; \
                 secondary_reached={secondary}; orphaned_worker_survived={}; the incumbent's \
                 thread::spawn panicked at the second reader and no failure was reported; \
                 elapsed {:?}",
                seen.status, seen.orphan_survived, seen.elapsed
            ),
        );
    }
}

// A harness control: with the primary absent the same armed secondary fault is
// reached by the incumbent's own teardown, so "never reached" above is a
// property of the primary and not of a hook that cannot fire. It files no row.
#[test]
#[ignore = "private real-process fixture; run native-incumbent.sh test"]
fn native_baseline_control_s03_an_armed_secondary_is_reached_without_the_primary() {
    for (fault, operation) in [("poll-EIO", "try_wait"), ("always-running", "try_wait")] {
        let seen = s03_run(fault, false);
        assert!(
            !seen.log.contains("the second reader creation is refused"),
            "{fault}: the primary was injected: {seen:?}"
        );
        assert!(
            seen.log.contains("physical worker pid="),
            "{fault}: the teardown did not run: {seen:?}"
        );
        assert!(
            seen.log
                .contains(&format!("native baseline S03 secondary reached: {operation}")),
            "{fault}: the armed secondary was not reached by the incumbent's teardown: {seen:?}"
        );
    }
}

// ---- F03 rows and the paired F03 row ----------------------------------------
//
// The same private role as S03's, with the row's one fault placed at the
// operation the incumbent performs: the shield's block, a reader creation, or
// the creator's restore. The claims are the repaired `native-reader` scenario's
// own, named one by one; a claim the incumbent has no text for is unmet, never
// a thing the instrument invented.

fn f03_manifest_ids(family: &str) -> Vec<String> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/native-entry-cases.json");
    let manifest: serde_json::Value =
        serde_json::from_slice(&fs::read(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display())))
            .unwrap();
    manifest["rows"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|row| row["family"] == family)
        .map(|row| row["id"].as_str().unwrap().to_owned())
        .collect()
}

fn id_part(id: &str, name: &str) -> String {
    id.split("__")
        .find_map(|part| part.strip_prefix(&format!("{name}=")))
        .unwrap_or_else(|| panic!("{id} names no {name}"))
        .to_owned()
}

fn f03_fault(id: &str) -> &'static str {
    match (id_part(id, "operation").as_str(), id_part(id, "mode").as_str()) {
        ("reader-block", "omit-EIO") => "block-omitted",
        ("reader-block", "perform-report-EIO") => "block-performed",
        ("first-create", "omit-EAGAIN") => "first-create",
        ("second-create", "omit-EAGAIN") => "second-create",
        ("creator-restore", "omit-EIO") => "restore-omitted",
        ("creator-restore", "perform-report-EIO") => "restore-performed",
        other => panic!("{id}: {other:?} is not a fixed F03 case"),
    }
}

#[test]
#[ignore = "private real-process fixture; run native-incumbent.sh test"]
fn native_baseline_f03_reader_shield_creation_and_restore_faults() {
    let ids = f03_manifest_ids("F03");
    assert_eq!(ids.len(), 6);
    for id in ids {
        let fault = f03_fault(&id);
        let seen = s03_run(fault, true);
        let create = matches!(fault, "first-create" | "second-create");
        // Instrument integrity: the named operation was faulted where the
        // incumbent performs it, and the role ended as the incumbent's own code
        // makes it end, or this file no record.
        if create {
            let which = if fault == "first-create" { "first" } else { "second" };
            assert!(
                seen.log.contains(&format!("the {which} reader creation is refused"))
                    && seen.log.contains("failed to spawn thread"),
                "{id}: the creation was not refused: {seen:?}"
            );
        } else {
            let marker = if fault.starts_with("block") { "F03 block: performed=" } else { "F03 restore: performed=" };
            assert!(seen.log.contains(marker), "{id}: the site was not reached: {seen:?}");
            assert!(
                seen.log.contains("native baseline F03 outcome:"),
                "{id}: the incumbent did not return from the front: {seen:?}"
            );
        }
        // The claims. A reader-creation panic prints no failure at all, so every
        // report claim has no text to be found in; a mask fault the incumbent
        // never reads prints none either.
        let failure_text = seen
            .log
            .lines()
            .filter(|l| l.contains("cannot admit diagnostic readers") || l.starts_with("native baseline F03 failure:"))
            .collect::<Vec<_>>()
            .join("\n");
        let admission_reported = seen.log.contains("cannot admit diagnostic readers");
        let identity_reported = failure_text.contains("reader") && failure_text.contains("pid");
        let retained = if create {
            failure_text.contains("os error 35") || failure_text.contains("os error 11")
        } else {
            failure_text.contains("os error 5")
        };
        let claims = [
            ("reader-admission-failure-reported", admission_reported),
            ("worker-identity-reported", identity_reported),
            (if create { "creation-failure-retained" } else { "mask-failure-retained" }, retained),
            ("no-managed-start", !seen.started),
            ("no-result-written", !seen.result.contains("\"status\"") && !seen.result.contains("native")),
        ];
        let unmet: Vec<_> = claims.iter().filter(|(_, held)| !held).map(|(n, _)| *n).collect();
        let held: Vec<_> = claims.iter().filter(|(_, held)| *held).map(|(n, _)| *n).collect();
        let receipts = seen
            .log
            .lines()
            .filter(|l| l.starts_with("native baseline"))
            .collect::<Vec<_>>()
            .join(" | ");
        crate::native_barrier::row(
            &id,
            "baseline",
            unmet.is_empty(),
            &format!(
                "unmet {unmet:?}; held {held:?}; exit {}; managed_started={}; orphaned_worker_survived={}; \
                 elapsed {:?}; {receipts}",
                seen.status, seen.started, seen.orphan_survived, seen.elapsed
            ),
        );
    }
}

/// F03's paired failure is S03's wait-report case: the omitted second reader
/// creation as the primary and the worker's own `try_wait` EIO as the
/// secondary. The incumbent has one path for it, so the same fixture is run and
/// the verdict filed under the paired identity.
#[test]
#[ignore = "private real-process fixture; run native-incumbent.sh test"]
fn native_baseline_paired_f03_second_create_keeps_the_worker_wait_report() {
    let ids: Vec<_> = f03_manifest_ids("paired")
        .into_iter()
        .filter(|id| id_part(id, "case").starts_with("F03-"))
        .collect();
    assert_eq!(ids.len(), 1);
    let id = &ids[0];
    let seen = s03_run("poll-EIO", true);
    assert!(
        seen.log.contains("the second reader creation is refused")
            && seen.log.contains("failed to spawn thread"),
        "{id}: the primary was not injected: {seen:?}"
    );
    let teardown_ran = seen.log.contains("physical worker pid=");
    let secondary = seen.log.contains("native baseline S03 secondary reached");
    // The claims read the failure the front *reports*: a process ended by a
    // panic reports nothing, and the panic's own text is not a report.
    let reported = seen.log.contains("cannot admit diagnostic readers");
    let creation_retained =
        reported && (seen.log.contains("os error 35") || seen.log.contains("os error 11"));
    let text_has_both = reported && creation_retained && seen.log.contains("try_wait pid");
    let claims = [
        ("creation-failure-retained", creation_retained),
        ("worker-wait-report-retained", text_has_both && secondary),
        ("no-managed-start", !seen.started),
    ];
    let unmet: Vec<_> = claims.iter().filter(|(_, held)| !held).map(|(n, _)| *n).collect();
    crate::native_barrier::row(
        id,
        "baseline",
        unmet.is_empty(),
        &format!(
            "unmet {unmet:?}; exit {}; teardown_ran={teardown_ran}; secondary_reached={secondary}; \
             orphaned_worker_survived={}; the incumbent's thread::spawn panicked at the second reader, \
             so neither the creation failure nor the worker's wait report was written; elapsed {:?}",
            seen.status, seen.orphan_survived, seen.elapsed
        ),
    );
}
