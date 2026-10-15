//! Fixed reader/worker-owner controls, reachable only from the private fixture.
use super::*;
use std::cell::RefCell;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Mutation {
    Omitted,
    Performed,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum Fault {
    #[default]
    None,
    Block(Mutation),
    FirstCreate,
    SecondCreate,
    CreatorRestore(Mutation),
    Stop(Mutation),
    Kill(Mutation),
    PollEio,
    PollEintr,
    PollLost,
    Running,
    /// Record reader/creator receipts only; inject nothing (E08).
    Observe,
}
#[derive(Default)]
struct Control {
    fault: Fault,
    poll_once: bool,
    protocol: Option<UnixStream>,
    physical: Vec<String>,
    product_polls_after_lost: usize,
    reported_lost: bool,
    reader_masks: Arc<Mutex<Vec<(bool, MaskReceipt)>>>,
    creator_before: Option<MaskReceipt>,
    creator_after: Option<MaskReceipt>,
    finished: Arc<Mutex<Vec<bool>>>,
    collected: Option<(usize, usize, Duration, Duration)>,
}

pub(super) type ReaderSink = Arc<Mutex<Vec<(bool, MaskReceipt)>>>;
pub(super) type FinishedSink = Arc<Mutex<Vec<bool>>>;

#[derive(Clone, Debug)]
pub(super) struct MaskReceipt {
    raw: Vec<u8>,
    handled: u8,
}
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
/// Everything E08 reads at a boundary: the creator's masks around reader
/// creation, each reader's own mask taken on its thread, which readers reached
/// the end of their stream, and the bounded collection. Read-only.
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
thread_local! { static CONTROL: RefCell<Control> = RefCell::new(Control::default()); }

pub(super) fn configure(case: &str) {
    let fault = match case {
        "observe" => Fault::Observe,
        "block-omitted" => Fault::Block(Mutation::Omitted),
        "block-performed" => Fault::Block(Mutation::Performed),
        "first-create" => Fault::FirstCreate,
        "second-create" => Fault::SecondCreate,
        "restore-omitted" => Fault::CreatorRestore(Mutation::Omitted),
        "restore-performed" => Fault::CreatorRestore(Mutation::Performed),
        "stop-omitted" => Fault::Stop(Mutation::Omitted),
        "stop-performed" => Fault::Stop(Mutation::Performed),
        "kill-omitted" => Fault::Kill(Mutation::Omitted),
        "kill-performed" => Fault::Kill(Mutation::Performed),
        "poll-EIO" => Fault::PollEio,
        "poll-EINTR" => Fault::PollEintr,
        "poll-ECHILD" => Fault::PollLost,
        "always-running" => Fault::Running,
        _ => panic!("unknown fixed reader/worker case {case}"),
    };
    CONTROL.with(|c| {
        *c.borrow_mut() = Control {
            fault,
            ..Control::default()
        }
    });
}
pub(super) fn active() -> bool {
    CONTROL.with(|c| c.borrow().fault != Fault::None)
}
/// A fault was configured, as opposed to observation only.
fn injected() -> bool {
    CONTROL.with(|c| !matches!(c.borrow().fault, Fault::None | Fault::Observe))
}
/// Observation for the private real-main fixture, injecting nothing.
pub(crate) fn observe() {
    configure("observe");
}
pub(crate) fn block_fault() -> Option<Mutation> {
    CONTROL.with(|c| match c.borrow().fault {
        Fault::Block(mode) => Some(mode),
        _ => None,
    })
}
pub(crate) fn restore_fault() -> Option<Mutation> {
    CONTROL.with(|c| match c.borrow().fault {
        Fault::CreatorRestore(mode) => Some(mode),
        _ => None,
    })
}
pub(super) fn omit_create(stdout: bool) -> bool {
    CONTROL.with(|c| {
        matches!(
            (c.borrow().fault, stdout),
            (Fault::FirstCreate, true) | (Fault::SecondCreate, false)
        ) || (!stdout
            && matches!(
                c.borrow().fault,
                Fault::Stop(_)
                    | Fault::Kill(_)
                    | Fault::PollEio
                    | Fault::PollEintr
                    | Fault::PollLost
                    | Fault::Running
            ))
    })
}
pub(super) fn child_started(child: &Child, protocol: &UnixStream) {
    if !injected() {
        return;
    }
    CONTROL.with(|c| {
        let mut c = c.borrow_mut();
        c.protocol = Some(protocol.try_clone().unwrap());
        c.physical.push(format!(
            "worker pid={} protocol fixture clone retained; TERM ignore applied before exec",
            child.id()
        ));
    });
}
pub(super) fn stop(pid: i32, signal: i32) -> i32 {
    CONTROL.with(|c| {
        let mut c = c.borrow_mut();
        assert!(!c.reported_lost, "product signalled after reported ECHILD");
        let mode = match (c.fault, signal) {
            (Fault::Stop(mode), libc::SIGTERM) | (Fault::Kill(mode), libc::SIGKILL) => Some(mode),
            _ => None,
        };
        let performed = mode != Some(Mutation::Omitted);
        let result = if performed { unsafe { libc::kill(pid, signal) } } else { 0 };
        let errno = (result == -1).then(|| io::Error::last_os_error().raw_os_error());
        c.physical.push(format!("stop pid={pid} signal={signal} performed={performed} result={result} errno={errno:?} reported_fault={mode:?}"));
        if mode.is_some() {
            #[cfg(target_vendor = "apple")]
            unsafe { *libc::__error() = libc::EPERM; }
            #[cfg(target_os = "linux")]
            unsafe { *libc::__errno_location() = libc::EPERM; }
            -1
        } else { result }
    })
}
pub(super) fn poll(child: &mut Child) -> io::Result<Option<ExitStatus>> {
    CONTROL.with(|c| {
        let mut c = c.borrow_mut();
        if c.reported_lost {
            c.product_polls_after_lost += 1;
            panic!("product polled after reported ECHILD");
        }
        let fault = match c.fault {
            Fault::PollEio if !c.poll_once => Some(libc::EIO),
            Fault::PollEintr => Some(libc::EINTR),
            Fault::PollLost => Some(libc::ECHILD),
            _ => None,
        };
        c.poll_once = true;
        if let Some(error) = fault {
            c.physical.push(format!(
                "try_wait omitted to retain physical reservation; substituted errno={error}"
            ));
            if error == libc::ECHILD {
                c.reported_lost = true;
            }
            return Err(io::Error::from_raw_os_error(error));
        }
        if c.fault == Fault::Running {
            c.physical.push(
                "try_wait omitted to retain physical reservation; substituted running".into(),
            );
            return Ok(None);
        }
        let actual = child.try_wait();
        c.physical.push(format!(
            "actual try_wait pid={} result={actual:?}",
            child.id()
        ));
        actual
    })
}

pub(super) fn fixture_cleanup(child: &mut Child, product: &Teardown) {
    if !injected() {
        return;
    }
    eprintln!("native worker product observation before fixture cleanup: pid={} status={:?} stops_failed={} reports={:?}", product.pid, product.status, product.stop_failed, product.reports);
    // The fixture alone retains a physical Child handle; substituted ECHILD
    // never released that actual reservation. No product ownership is restored.
    let until = Instant::now() + Duration::from_secs(2);
    let mut killed = false;
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                eprintln!(
                    "native fixture actual reap pid={} status={status}",
                    child.id()
                );
                break;
            }
            Err(error) => panic!("native physical fixture ownership/reap failed: {error}"),
            Ok(None) => {}
        }
        if !killed {
            child.kill().unwrap();
            killed = true;
        }
        assert!(
            Instant::now() < until,
            "native physical fixture cleanup deadline expired"
        );
        thread::sleep(Duration::from_millis(1));
    }
    CONTROL.with(|c| {
        let mut c = c.borrow_mut();
        c.protocol.take();
        eprintln!("native worker physical receipts={:#?}", c.physical);
        let readers = c.reader_masks.lock().unwrap();
        eprintln!(
            "native creator before={:?} after={:?} readers={readers:?}",
            c.creator_before, c.creator_after
        );
        let expected = match c.fault {
            Fault::Block(_) | Fault::FirstCreate => 0,
            Fault::CreatorRestore(_) => 2,
            _ => 1,
        };
        assert_eq!(readers.len(), expected, "partial reader ownership was lost");
        for (_, mask) in readers.iter() {
            assert_eq!(mask.handled, 7, "reader became signal-eligible");
            assert!(!mask.raw.is_empty());
        }
        let before = c.creator_before.as_ref().unwrap();
        let after = c.creator_after.as_ref().unwrap();
        assert_eq!(
            before.handled, 0,
            "fixed reader fixture must enter unblocked"
        );
        match c.fault {
            Fault::Block(Mutation::Performed) | Fault::CreatorRestore(Mutation::Omitted) => {
                assert_eq!(after.handled, 7)
            }
            _ => assert_eq!(
                after.raw, before.raw,
                "creator did not regain its actual previous mask"
            ),
        }
        assert_eq!(c.product_polls_after_lost, 0);
    });
}

macro_rules! reader_case {
    ($name:ident, $fault:literal) => {
        #[test]
        fn $name() {
            admission_tests::cases(&["native-reader"], &[$fault], &["front"]);
        }
    };
}
reader_case!(native_f03_block_omitted, "block-omitted");
reader_case!(native_f03_block_performed, "block-performed");
reader_case!(native_f03_first_create, "first-create");
reader_case!(native_f03_second_create, "second-create");
reader_case!(native_f03_restore_omitted, "restore-omitted");
reader_case!(native_f03_restore_performed, "restore-performed");
reader_case!(native_s03_stop_omitted, "stop-omitted");
reader_case!(native_s03_stop_performed, "stop-performed");
reader_case!(native_s03_kill_omitted, "kill-omitted");
reader_case!(native_s03_kill_performed, "kill-performed");
reader_case!(native_s03_poll_eio, "poll-EIO");
reader_case!(native_s03_poll_eintr, "poll-EINTR");
reader_case!(native_s03_poll_echild, "poll-ECHILD");
reader_case!(native_s03_running, "always-running");

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
