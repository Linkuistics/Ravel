//! The signal state this process inherited, recorded before the Rust runtime
//! changes it, and handed to the harness
//! (`docs/specs/harness-selection-and-execution.md`, *Execution and authority*).
//!
//! The harness is to receive the signal mask, and every disposition that
//! survives exec, exactly as this process inherited them. Two facts of Rust's
//! standard library stand in the way, and both hold in 1.85.0, the workspace's
//! minimum, and in 1.98.1:
//!
//! - Before `main`, `lang_start` runs `sys::init`, whose `reset_sigpipe` sets
//!   SIGPIPE to ignored unless the unstable `-Zon-broken-pipe` is used
//!   (`library/std/src/sys/pal/unix/mod.rs`). Nothing in `main` can see the
//!   caller's SIGPIPE.
//! - A spawn with a `pre_exec` closure forks and runs `do_exec` in the child.
//!   It keeps the calling thread's mask, sets SIGPIPE to default, and only
//!   then runs the `pre_exec` closures before `execvp` (1.85.0:
//!   `library/std/src/sys/pal/unix/process/process_unix.rs`; 1.98.1:
//!   `library/std/src/sys/process/unix/unix.rs`). Nothing before those
//!   closures can restore SIGPIPE.
//!
//! So [`record`] runs from the executable's initializer section, which the
//! loader runs before the C `main` that calls `lang_start`, and [`entry`] hands
//! what it found to the runner as a transparent launch's entry state, which the
//! runner sets in its `pre_exec` closure. Exec keeps an ignored disposition and
//! resets every other one to default, so the ignored set and the mask are the
//! whole of what survives it. Every signal is handed on, not only SIGPIPE and
//! the three that cancel, so the harness receives its caller's state whatever
//! else in this process changed a disposition.

use std::io;
use std::ptr;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use crate::refusal::{Refusal, Stage, EXIT_WORKER};

/// Every signal number either platform uses: 1 to 31 on macOS, to 64 on
/// Linux. The kernel refuses a query of a number it does not have, and that
/// signal is then left alone.
const SIGNALS: std::ops::RangeInclusive<libc::c_int> = 1..=64;

/// Whether the initializer ran, and what it found, one bit per signal (bit
/// `N - 1` for signal `N`): which were ignored, and which were blocked. Written once before `main`, while this
/// process has one thread, and only read afterwards.
static RECORDED: AtomicBool = AtomicBool::new(false);
static IGNORED: AtomicU64 = AtomicU64::new(0);
static BLOCKED: AtomicU64 = AtomicU64::new(0);

fn bit(signal: libc::c_int) -> u64 {
    1 << (signal - 1)
}

/// The initializer. The ELF loader runs `.init_array` and dyld runs a Mach-O
/// executable's `__mod_init_func` before `main`: the section names std itself
/// uses for its glibc `ARGV_INIT_ARRAY`, and the `ctor` crate for Apple
/// targets (`__DATA,__mod_init_func,mod_init_funcs`).
#[used]
#[cfg_attr(
    target_vendor = "apple",
    unsafe(link_section = "__DATA,__mod_init_func,mod_init_funcs")
)]
#[cfg_attr(not(target_vendor = "apple"), unsafe(link_section = ".init_array"))]
static RECORD: extern "C" fn() = record;

/// Record the mask and the ignored set. It runs before the Rust runtime is
/// initialized, so it uses nothing but system calls and atomics.
extern "C" fn record() {
    // SAFETY: the set and each action are written by the calls that take
    // them, from zeroed values; a null new action or set only queries.
    unsafe {
        let mut mask: libc::sigset_t = std::mem::zeroed();
        if libc::pthread_sigmask(libc::SIG_BLOCK, ptr::null(), &mut mask) != 0 {
            return;
        }
        let (mut ignored, mut blocked) = (0, 0);
        for signal in SIGNALS {
            let mut action: libc::sigaction = std::mem::zeroed();
            if libc::sigaction(signal, ptr::null(), &mut action) == 0
                && action.sa_sigaction == libc::SIG_IGN
            {
                ignored |= bit(signal);
            }
            if libc::sigismember(&mask, signal) == 1 {
                blocked |= bit(signal);
            }
        }
        IGNORED.store(ignored, Ordering::Relaxed);
        BLOCKED.store(blocked, Ordering::Relaxed);
        RECORDED.store(true, Ordering::Release);
    }
}

/// Refuse a handoff that could not be transparent: the initializer did not
/// run, so this build cannot know its caller's SIGPIPE.
pub fn recorded() -> Result<(), Refusal> {
    if RECORDED.load(Ordering::Acquire) {
        return Ok(());
    }
    Err(Refusal::new(
        "signal_state_unavailable",
        Stage::Exec,
        EXIT_WORKER,
        "this build of ravel did not record its caller's signal state before the Rust \
         runtime changed it, so it cannot hand that state to a harness; nothing was evaluated",
        "reinstall ravel from a release archive or the Homebrew formula; a build for \
         this platform must run its initializer before main",
    )
    .source(
        std::env::current_exe()
            .map(|path| path.to_string_lossy().into_owned())
            .unwrap_or_else(|_| "ravel".to_owned()),
    ))
}

/// Whether the caller had `signal` blocked.
pub fn blocked_at_entry(signal: libc::c_int) -> bool {
    BLOCKED.load(Ordering::Relaxed) & bit(signal) != 0
}

/// The entry mask, rebuilt from its record.
fn entry_mask() -> libc::sigset_t {
    let blocked = BLOCKED.load(Ordering::Relaxed);
    // SAFETY: sigemptyset initializes the set before any member is added.
    unsafe {
        let mut mask: libc::sigset_t = std::mem::zeroed();
        libc::sigemptyset(&mut mask);
        for signal in SIGNALS.filter(|&signal| blocked & bit(signal) != 0) {
            libc::sigaddset(&mut mask, signal);
        }
        mask
    }
}

/// Set this thread's mask back to the caller's. A signal that was blocked here
/// and not by the caller, and is pending, is delivered as this returns.
pub fn restore_mask() -> io::Result<()> {
    let mask = entry_mask();
    // SAFETY: the set is initialized; pthread_sigmask returns its error.
    match unsafe { libc::pthread_sigmask(libc::SIG_SETMASK, &mask, ptr::null_mut()) } {
        0 => Ok(()),
        error => Err(io::Error::from_raw_os_error(error)),
    }
}

/// The entry state as the runner takes it: the ignored set and the mask, for
/// the harness to receive in place of anything this process has since set.
pub fn entry() -> keyed_launch::EntrySignals {
    let (ignored, blocked) = (
        IGNORED.load(Ordering::Relaxed),
        BLOCKED.load(Ordering::Relaxed),
    );
    keyed_launch::EntrySignals::new(
        SIGNALS.filter(|&signal| ignored & bit(signal) != 0),
        SIGNALS.filter(|&signal| blocked & bit(signal) != 0),
    )
}
