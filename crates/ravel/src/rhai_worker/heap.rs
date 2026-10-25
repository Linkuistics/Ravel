//! The counting allocator behind the heap bound of the worker
//! (`docs/specs/policy-evaluation.md`, *The policy worker*).
//!
//! The binary installs [`Counting`] as its global allocator and counts every
//! live byte, in the front as in the role. The count stays harmless until the
//! role calls [`arm`]. Once armed, an allocation that would take the live bytes
//! past the bound reports the breach and ends the process, because a script
//! cannot catch an allocation failure and no later code can be trusted with a
//! heap that has no room.
//!
//! The report cannot allocate, so [`arm`] encodes the whole `memory_limit`
//! frame ahead of time. The breach writes those bytes to the protocol socket
//! with `write(2)` and calls `_exit(2)`.

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicBool, AtomicPtr, AtomicUsize, Ordering::Relaxed};

use serde_json::json;

use super::{Limit, ScriptFailure};
use crate::worker::CHANNEL_FD;

/// The exit status of a worker that ended at the heap bound. The failure frame
/// is what the front reads, so the status is for a person reading a log.
pub const EXIT_HEAP_LIMIT: i32 = 4;

/// Wraps the system allocator and counts the bytes that are live.
pub struct Counting;

#[global_allocator]
static ALLOCATOR: Counting = Counting;

/// Live bytes of the whole process, from the first allocation.
static LIVE: AtomicUsize = AtomicUsize::new(0);
/// The bound, or `usize::MAX` while no role has armed it.
static BOUND: AtomicUsize = AtomicUsize::new(usize::MAX);
/// The encoded failure frame, leaked on purpose: the breach reads it after the
/// heap is no longer safe to touch.
static REPORT: AtomicPtr<u8> = AtomicPtr::new(std::ptr::null_mut());
static REPORT_LEN: AtomicUsize = AtomicUsize::new(0);
static BREACHED: AtomicBool = AtomicBool::new(false);
/// The most live bytes since the last [`reset_peak`]. Test builds only, because
/// the worker needs no peak and an extra atomic on every allocation is a cost.
#[cfg(test)]
static PEAK: AtomicUsize = AtomicUsize::new(0);

/// Bound the live bytes of this process to `bytes` from now on. The role calls
/// it once, before it runs any policy code.
pub fn arm(bytes: usize) {
    let frame = Box::leak(report(bytes).into_boxed_slice());
    REPORT_LEN.store(frame.len(), Relaxed);
    REPORT.store(frame.as_mut_ptr(), Relaxed);
    BOUND.store(bytes, Relaxed);
}

/// The `memory_limit` failure as a whole frame: the four-byte big-endian length
/// and the JSON body that `frame::read_frame` reads.
fn report(bytes: usize) -> Vec<u8> {
    let failure = ScriptFailure::new(
        "memory_limit",
        "evaluation",
        format!("the policy worker's heap reached the bound of {bytes} bytes"),
    )
    .bound(Limit::Heap);
    let body = serde_json::to_vec(&json!({ "type": "failure", "failure": failure.to_json() }))
        .expect("a failure frame encodes");
    let length = u32::try_from(body.len()).expect("a short frame");
    let mut frame = Vec::with_capacity(4 + body.len());
    frame.extend_from_slice(&length.to_be_bytes());
    frame.extend_from_slice(&body);
    frame
}

/// The live bytes of the whole process now. Test builds only.
#[cfg(test)]
pub(crate) fn live() -> usize {
    LIVE.load(Relaxed)
}

/// Start a new peak from the live bytes now. Test builds only.
#[cfg(test)]
pub(crate) fn reset_peak() {
    PEAK.store(LIVE.load(Relaxed), Relaxed);
}

/// The most live bytes of the whole process since [`reset_peak`]. Test builds
/// only. The count covers every thread, so a test that reads it must run alone
/// in its process, as nextest runs it.
#[cfg(test)]
pub(crate) fn peak() -> usize {
    PEAK.load(Relaxed)
}

/// Take `bytes` more live bytes, or end the process when that passes the bound.
fn reserve(bytes: usize) {
    let after = LIVE.fetch_add(bytes, Relaxed).saturating_add(bytes);
    #[cfg(test)]
    PEAK.fetch_max(after, Relaxed);
    if after > BOUND.load(Relaxed) {
        breach();
    }
}

fn release(bytes: usize) {
    LIVE.fetch_sub(bytes, Relaxed);
}

/// Report the breach to the front and end the process. Only the first thread
/// to get here writes; any other waits for that thread to end the process.
fn breach() -> ! {
    if BREACHED.swap(true, Relaxed) {
        loop {
            // SAFETY: `sleep` has no preconditions. The reporting thread ends
            // the process while this one waits.
            unsafe { libc::sleep(60) };
        }
    }
    let frame = REPORT.load(Relaxed);
    let mut left = REPORT_LEN.load(Relaxed);
    let mut at = frame.cast_const();
    while left > 0 && !at.is_null() {
        // SAFETY: `at` and `left` describe the rest of the leaked frame, which
        // lives for the whole process.
        let written = unsafe { libc::write(CHANNEL_FD, at.cast(), left) };
        if written <= 0 {
            break;
        }
        let written = written as usize;
        // SAFETY: `written` is at most `left`, so `at` stays inside the frame
        // or one past its end.
        at = unsafe { at.add(written) };
        left -= written;
    }
    // SAFETY: `_exit` ends the process without running any destructor, which
    // is the point: the heap has no room to run one.
    unsafe { libc::_exit(EXIT_HEAP_LIMIT) }
}

// SAFETY: every method defers to `System` for the memory itself and only adds
// the count, which uses atomics and allocates nothing.
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        reserve(layout.size());
        // SAFETY: the caller upholds `GlobalAlloc::alloc`'s contract for `layout`.
        let pointer = unsafe { System.alloc(layout) };
        if pointer.is_null() {
            release(layout.size());
        }
        pointer
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        reserve(layout.size());
        // SAFETY: the caller upholds `GlobalAlloc::alloc_zeroed`'s contract.
        let pointer = unsafe { System.alloc_zeroed(layout) };
        if pointer.is_null() {
            release(layout.size());
        }
        pointer
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        // SAFETY: the caller upholds `GlobalAlloc::dealloc`'s contract.
        unsafe { System.dealloc(pointer, layout) };
        release(layout.size());
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        if new_size > layout.size() {
            reserve(new_size - layout.size());
        }
        // SAFETY: the caller upholds `GlobalAlloc::realloc`'s contract.
        let moved = unsafe { System.realloc(pointer, layout, new_size) };
        if moved.is_null() {
            if new_size > layout.size() {
                release(new_size - layout.size());
            }
        } else if new_size < layout.size() {
            release(layout.size() - new_size);
        }
        moved
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame::read_frame;
    use crate::rhai_worker::Reply;

    #[test]
    fn the_prepared_report_is_a_frame_the_front_reads_as_a_memory_limit_failure() {
        let frame = report(1 << 30);
        let value = read_frame(frame.as_slice(), frame.len()).unwrap();
        match Reply::from_frame(&value) {
            Some(Reply::Failure(failure)) => {
                assert_eq!(failure.code, "memory_limit");
                assert_eq!(failure.stage, "evaluation");
                assert_eq!(failure.bound.as_deref(), Some("heap"));
            }
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn nothing_is_bounded_until_a_role_arms_the_allocator() {
        assert_eq!(BOUND.load(Relaxed), usize::MAX);
    }
}
