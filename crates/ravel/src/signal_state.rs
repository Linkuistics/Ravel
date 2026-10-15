//! The signal state this process inherited, recorded before the Rust runtime
//! changes it, and handed to the harness
//! (`docs/specs/session-execution.md`, *Execution and authority*).
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
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicU64, Ordering};

use crate::refusal::{Refusal, Stage, EXIT_WORKER};

/// Target signal domain: 1..=31 on Darwin, 1..=64 on Linux, excluding glibc's
/// reserved 32/33. Required queries inside this domain must succeed; Darwin's
/// unchangeable KILL/STOP dispositions have an explicit query exclusion below.
fn signals() -> impl Iterator<Item = libc::c_int> {
    #[cfg(target_vendor = "apple")]
    let domain = 1..=31;
    #[cfg(not(target_vendor = "apple"))]
    let domain = 1..=64;
    domain.filter(|&n| !cfg!(target_env = "gnu") || (n != 32 && n != 33))
}

/// Whether the initializer ran, and what it found, one bit per signal (bit
/// `N - 1` for signal `N`): which were ignored, and which were blocked. Written once before `main`, while this
/// process has one thread, and only read afterwards.
static RECORDED: AtomicBool = AtomicBool::new(false);
static IGNORED: AtomicU64 = AtomicU64::new(0);
static BLOCKED: AtomicU64 = AtomicU64::new(0);
static ERROR_OPERATION: AtomicI32 = AtomicI32::new(0);
static ERROR_SIGNAL: AtomicI32 = AtomicI32::new(0);
static ERROR_NUMBER: AtomicI32 = AtomicI32::new(0);
#[cfg(test)]
static RAW_CHLD_FLAGS: AtomicI32 = AtomicI32::new(0);
#[cfg(test)]
static RAW_PENDING: AtomicU64 = AtomicU64::new(0);
#[cfg(test)]
static RAW_PENDING_ERROR: AtomicI32 = AtomicI32::new(0);
#[cfg(test)]
static CAPTURING_THREAD: AtomicU64 = AtomicU64::new(0);

fn capture_error(operation: i32, signal: i32, number: i32) {
    ERROR_SIGNAL.store(signal, Ordering::Relaxed);
    ERROR_NUMBER.store(number, Ordering::Relaxed);
    ERROR_OPERATION.store(operation, Ordering::Release);
}

fn errno() -> i32 {
    #[cfg(target_vendor = "apple")]
    unsafe {
        *libc::__error()
    }
    #[cfg(target_os = "linux")]
    unsafe {
        *libc::__errno_location()
    }
}

#[cfg(test)]
static CAPTURE_FAULT: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);

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
        let queried = libc::pthread_sigmask(libc::SIG_BLOCK, ptr::null(), &mut mask);
        #[cfg(test)]
        let queried = if CAPTURE_FAULT.load(Ordering::Relaxed) == 1 {
            libc::EIO
        } else {
            queried
        };
        if queried != 0 {
            capture_error(1, 0, queried);
            return;
        }
        let (mut ignored, mut blocked) = (0, 0);
        for signal in signals() {
            let mut action: libc::sigaction = std::mem::zeroed();
            // Darwin rejects even a query of KILL/STOP. This is an explicit
            // target exclusion, never a blanket EINVAL exemption:
            // https://github.com/apple-oss-distributions/xnu/blob/f6217f891ac0bb64f3d375211650a4c1ff8ca1ea/bsd/kern/kern_sig.c#L425
            let queried = if cfg!(target_vendor = "apple")
                && matches!(signal, libc::SIGKILL | libc::SIGSTOP)
            {
                0 // These dispositions cannot be ignored or changed.
            } else {
                libc::sigaction(signal, ptr::null(), &mut action)
            };
            #[cfg(test)]
            let queried = if signal == libc::SIGTERM && CAPTURE_FAULT.load(Ordering::Relaxed) == 2 {
                -1
            } else {
                queried
            };
            if queried != 0 {
                let number = errno();
                #[cfg(test)]
                let number = if CAPTURE_FAULT.load(Ordering::Relaxed) == 2 {
                    libc::EIO
                } else {
                    number
                };
                capture_error(2, signal, number);
                return;
            }
            if action.sa_sigaction == libc::SIG_IGN {
                ignored |= bit(signal);
            }
            #[cfg(test)]
            if signal == libc::SIGCHLD {
                RAW_CHLD_FLAGS.store(action.sa_flags, Ordering::Relaxed);
            }
            let member = libc::sigismember(&mask, signal);
            #[cfg(test)]
            let member = if signal == libc::SIGTERM && CAPTURE_FAULT.load(Ordering::Relaxed) == 3 {
                -1
            } else {
                member
            };
            match member {
                1 => blocked |= bit(signal),
                0 => {}
                _ => {
                    let number = errno();
                    #[cfg(test)]
                    let number = if CAPTURE_FAULT.load(Ordering::Relaxed) == 3 {
                        libc::EIO
                    } else {
                        number
                    };
                    capture_error(3, signal, number);
                    return;
                }
            }
        }
        IGNORED.store(ignored, Ordering::Relaxed);
        BLOCKED.store(blocked, Ordering::Relaxed);
        #[cfg(test)]
        {
            let mut pending: libc::sigset_t = std::mem::zeroed();
            if libc::sigpending(&mut pending) == 0 {
                let mut bits = 0;
                for signal in signals() {
                    match libc::sigismember(&pending, signal) {
                        1 => bits |= bit(signal),
                        0 => {}
                        _ => {
                            RAW_PENDING_ERROR.store(errno(), Ordering::Relaxed);
                        }
                    }
                }
                RAW_PENDING.store(bits, Ordering::Relaxed);
            } else {
                RAW_PENDING_ERROR.store(errno(), Ordering::Relaxed);
            }
            CAPTURING_THREAD.store(libc::pthread_self() as usize as u64, Ordering::Relaxed);
        }
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
        format!("executable-entry signal capture unavailable (operation {}, signal {}, errno {}); nothing was evaluated",
            ERROR_OPERATION.load(Ordering::Acquire), ERROR_SIGNAL.load(Ordering::Relaxed), ERROR_NUMBER.load(Ordering::Relaxed)),
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

pub fn ignored_at_entry(signal: libc::c_int) -> bool {
    IGNORED.load(Ordering::Relaxed) & bit(signal) != 0
}

/// The entry mask, rebuilt from its record.
fn entry_mask() -> io::Result<libc::sigset_t> {
    let blocked = BLOCKED.load(Ordering::Relaxed);
    // SAFETY: sigemptyset initializes the set before any member is added.
    unsafe {
        let mut mask: libc::sigset_t = std::mem::zeroed();
        if libc::sigemptyset(&mut mask) != 0 {
            return Err(io::Error::last_os_error());
        }
        for signal in signals().filter(|&signal| blocked & bit(signal) != 0) {
            if libc::sigaddset(&mut mask, signal) != 0 {
                return Err(io::Error::last_os_error());
            }
        }
        Ok(mask)
    }
}

/// Set this thread's mask back to the caller's. A signal that was blocked here
/// and not by the caller, and is pending, is delivered as this returns.
pub fn restore_mask() -> io::Result<()> {
    let mask = entry_mask()?;
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
        signals().filter(|&signal| ignored & bit(signal) != 0),
        signals().filter(|&signal| blocked & bit(signal) != 0),
    )
}

#[cfg(test)]
pub(crate) fn native_receipt() -> serde_json::Value {
    serde_json::json!({"recorded": RECORDED.load(Ordering::Acquire),
        "ignored": IGNORED.load(Ordering::Relaxed), "blocked": BLOCKED.load(Ordering::Relaxed),
        "pending": RAW_PENDING.load(Ordering::Relaxed), "pendingQueryError": RAW_PENDING_ERROR.load(Ordering::Relaxed),
        "capturingThread": CAPTURING_THREAD.load(Ordering::Relaxed), "currentThread": unsafe { libc::pthread_self() as usize as u64 },
        "sigchldFlags": RAW_CHLD_FLAGS.load(Ordering::Relaxed),
        "pid": unsafe { libc::getpid() }, "sid": unsafe { libc::getsid(0) }, "pgid": unsafe { libc::getpgrp() }})
}

#[cfg(test)]
pub(crate) fn native_capture_report(operation: i32, broken: bool) {
    capture_error(
        operation,
        if operation == 1 { 0 } else { libc::SIGTERM },
        libc::EIO,
    );
    RECORDED.store(broken, Ordering::Release);
}

#[cfg(test)]
pub(crate) fn native_wrong_capture(signal: i32) -> keyed_launch::EntrySignals {
    let ignored = IGNORED.load(Ordering::Relaxed) ^ bit(signal);
    let blocked = BLOCKED.load(Ordering::Relaxed) ^ bit(signal);
    keyed_launch::EntrySignals::new(
        signals().filter(|&s| ignored & bit(s) != 0),
        signals().filter(|&s| blocked & bit(s) != 0),
    )
}

#[cfg(test)]
mod capture_tests {
    use super::*;

    #[test]
    fn required_query_errors_never_publish_partial_entry_state() {
        // Break: treating a failed required query as default or not blocked.
        // Fork isolation keeps the immutable executable record in the parent.
        for fault in [1, 2, 3] {
            let pid = unsafe { libc::fork() };
            assert!(pid >= 0);
            if pid == 0 {
                CAPTURE_FAULT.store(fault, Ordering::Relaxed);
                RECORDED.store(false, Ordering::Relaxed);
                record();
                let published = RECORDED.load(Ordering::Acquire);
                let operation = ERROR_OPERATION.load(Ordering::Acquire);
                let signal = ERROR_SIGNAL.load(Ordering::Relaxed);
                let errno = ERROR_NUMBER.load(Ordering::Relaxed);
                let expected_signal = if fault == 1 { 0 } else { libc::SIGTERM };
                let valid = !published
                    && operation == i32::from(fault)
                    && signal == expected_signal
                    && errno == libc::EIO;
                unsafe {
                    libc::_exit(i32::from(!valid));
                }
            }
            let at = std::time::Instant::now();
            let by = at + std::time::Duration::from_secs(25);
            let outer = at + std::time::Duration::from_secs(30);
            let mut status = 0;
            let mut timed_out = false;
            loop {
                let result = unsafe { libc::waitpid(pid, &mut status, libc::WNOHANG) };
                if result == pid {
                    break;
                }
                if result == -1 {
                    let error = io::Error::last_os_error();
                    if error.raw_os_error() == Some(libc::EINTR) {
                        assert!(std::time::Instant::now() < outer);
                        continue;
                    }
                    // In particular ECHILD cannot authorise a numeric kill.
                    panic!("capture fixture wait lost confirmation: {error}");
                }
                if !timed_out && std::time::Instant::now() >= by {
                    // This unreaped direct child still reserves its identity.
                    let killed = unsafe { libc::kill(pid, libc::SIGKILL) };
                    assert!(
                        killed == 0
                            || io::Error::last_os_error().raw_os_error() == Some(libc::ESRCH)
                    );
                    timed_out = true;
                }
                assert!(
                    std::time::Instant::now() < outer,
                    "capture fixture reap unconfirmed at cleanup deadline"
                );
                std::thread::sleep(std::time::Duration::from_millis(1));
            }
            assert!(
                !timed_out,
                "capture fixture exceeded its work deadline; actual cleanup reap confirmed"
            );
            assert!(libc::WIFEXITED(status));
            assert_eq!(
                libc::WEXITSTATUS(status),
                0,
                "capture fault {fault} published incomplete state"
            );
        }
    }
}
