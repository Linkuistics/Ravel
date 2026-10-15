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
