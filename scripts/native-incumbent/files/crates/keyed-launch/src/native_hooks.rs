//! Single-site reported faults for the runner-owned F rows (F02 managed, F04
//! runner handlers, F06 child restoration). Compiled only into unit-test builds
//! and into the isolated private fixture built with the off-by-default
//! `native-faults` feature; never into an installed build.
//!
//! The selection is two atomics, so the forked child can read it without
//! allocating or locking. A mutation site omits its operation or performs it
//! and then reports EIO; a query site always performs the query and
//! substitutes only the report. Each site that is reached appends one fixed
//! two-byte record to a private pipe (`write` is async-signal-safe), so the
//! child's physical operation and substituted report are observable by the
//! controller without any formatting in the child. The one broken subject
//! ignores the selected failure.
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicU8, Ordering};

pub(crate) const WATCH_QUERY: u8 = 1;
pub(crate) const WATCH_REPAIR: u8 = 2;
pub(crate) const HANDLER_QUERY: u8 = 3;
pub(crate) const INT_INSTALL: u8 = 4;
pub(crate) const TERM_INSTALL: u8 = 5;
pub(crate) const TERM_RESTORE: u8 = 6;
pub(crate) const MASK_BUILD: u8 = 7;
pub(crate) const MASK_RESTORE: u8 = 8;
/// The terminal handback's report in the runner's recovery (the paired F05
/// row's secondary): the physical handback runs, only its report is replaced.
pub(crate) const HANDBACK_REPORT: u8 = 9;

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) const REPORT: u8 = 1;
pub(crate) const OMIT: u8 = 2;
pub(crate) const PERFORM: u8 = 3;

static SITE: AtomicU8 = AtomicU8::new(0);
static MODE: AtomicU8 = AtomicU8::new(0);
static IGNORE: AtomicBool = AtomicBool::new(false);
static RECORD_FD: AtomicI32 = AtomicI32::new(-1);

pub(crate) fn select(site: u8, mode: u8, ignore: bool, record_fd: i32) {
    RECORD_FD.store(record_fd, Ordering::SeqCst);
    IGNORE.store(ignore, Ordering::SeqCst);
    MODE.store(mode, Ordering::SeqCst);
    SITE.store(site, Ordering::SeqCst);
}
fn selected(site: u8) -> Option<u8> {
    (SITE.load(Ordering::SeqCst) == site).then(|| MODE.load(Ordering::SeqCst))
}
/// Byte 0: the site. Byte 1: bit 0 operation performed, bit 1 the physical
/// result was a failure, bit 2 the calling process's own group was the
/// terminal's foreground group at that moment (the lease had been handed over).
/// Fixed data, async-signal-safe queries and one `write`.
fn record(site: u8, performed: bool, failed: bool) {
    let fd = RECORD_FD.load(Ordering::SeqCst);
    if fd >= 0 {
        // SAFETY: argument-free or descriptor-only queries; both fail harmlessly
        // when there is no terminal.
        let foreground = unsafe { libc::tcgetpgrp(0) == libc::getpgrp() };
        let bytes = [
            site,
            u8::from(performed) | (u8::from(failed) << 1) | (u8::from(foreground) << 2),
        ];
        // SAFETY: a fixed two-byte buffer to a descriptor the role owns.
        unsafe { libc::write(fd, bytes.as_ptr().cast(), 2) };
    }
}
fn set_errno(number: i32) {
    #[cfg(target_vendor = "apple")]
    // SAFETY: the calling thread's own errno location.
    unsafe {
        *libc::__error() = number;
    }
    #[cfg(target_os = "linux")]
    // SAFETY: the calling thread's own errno location.
    unsafe {
        *libc::__errno_location() = number;
    }
}

/// What a query site does with the physical result it just produced.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Query {
    Real,
    Substituted,
    Ignored,
}
pub(crate) fn query(site: u8, actual: i32) -> Query {
    if selected(site).is_none() {
        return Query::Real;
    }
    record(site, true, actual != 0);
    if IGNORE.load(Ordering::SeqCst) {
        return Query::Ignored;
    }
    set_errno(libc::EIO);
    Query::Substituted
}
/// A `sigaction`-style mutation: `-1` with errno on failure.
pub(crate) fn mutation_c(site: u8, real: impl FnOnce() -> i32) -> i32 {
    let Some(mode) = selected(site) else {
        return real();
    };
    let performed = mode != OMIT;
    let actual = if performed { real() } else { 0 };
    record(site, performed, actual != 0);
    if IGNORE.load(Ordering::SeqCst) {
        return actual;
    }
    set_errno(libc::EIO);
    -1
}
/// A `pthread_sigmask`-style mutation: the error number itself.
pub(crate) fn mutation_r(site: u8, real: impl FnOnce() -> i32) -> i32 {
    let Some(mode) = selected(site) else {
        return real();
    };
    let performed = mode != OMIT;
    let actual = if performed { real() } else { 0 };
    record(site, performed, actual != 0);
    if IGNORE.load(Ordering::SeqCst) {
        return actual;
    }
    libc::EIO
}

/// The recovery report of one runner operation. Only the terminal handback is a
/// site: the physical handback has already run, and a selected row replaces its
/// report by a failure the product then retains or discards as it does for a
/// real one. The text is the one the runner's own unit-test seam produces.
pub(crate) fn recovery_report(
    operation: &str,
    result: std::io::Result<()>,
) -> std::io::Result<()> {
    if operation != "terminal handback" || selected(HANDBACK_REPORT).is_none() {
        return result;
    }
    record(HANDBACK_REPORT, true, result.is_err());
    if IGNORE.load(Ordering::SeqCst) {
        return result;
    }
    result.and(Err(std::io::Error::other(format!(
        "C4 substituted {operation} report after physical operation"
    ))))
}

/// The private fixture's one entry point: select a fixed site by name. The
/// string form keeps the site numbers private. Any unknown name is a fixture
/// defect and stops it.
#[cfg(feature = "native-faults")]
pub fn native_fault_select(site: &str, mode: &str, ignore: bool, record_fd: i32) {
    let site = match site {
        "TERM-action-restore" => TERM_RESTORE,
        "mask-set-build" => MASK_BUILD,
        "mask-restore" => MASK_RESTORE,
        "terminal-handback-report" => HANDBACK_REPORT,
        other => panic!("unknown fixed runner fault site {other}"),
    };
    let mode = match mode {
        "report" => REPORT,
        "omit" => OMIT,
        "perform" => PERFORM,
        other => panic!("unknown fixed runner fault mode {other}"),
    };
    select(site, mode, ignore, record_fd);
}
