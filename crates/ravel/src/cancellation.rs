use std::io::{self, Write as _};
use std::ptr;
use std::sync::atomic::{AtomicI32, Ordering};

use crate::refusal::{Refusal, Stage};
use crate::signal_state;

/// The handled signals, by number and name.
const HANDLED: [(libc::c_int, &str); 3] = [
    (libc::SIGINT, "SIGINT"),
    (libc::SIGTERM, "SIGTERM"),
    (libc::SIGHUP, "SIGHUP"),
];

/// The first handled signal received, or 0. Process-global, because a
/// disposition is.
static RECEIVED: AtomicI32 = AtomicI32::new(0);

/// One of the handled signals.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Signal {
    number: libc::c_int,
    name: &'static str,
}

impl Signal {
    pub fn of(number: i32) -> Option<Self> {
        HANDLED
            .iter()
            .find(|&&(handled, _)| handled == number)
            .map(|&(number, name)| Self { number, name })
    }
    pub fn name(self) -> &'static str {
        self.name
    }

    /// The status a shell reports for a process this signal ended: `128 + N`.
    pub fn exit(self) -> u8 {
        u8::try_from(128 + self.number).unwrap_or(u8::MAX)
    }
}

/// The handled signal received since the handlers were installed, if any.
pub fn received() -> Option<Signal> {
    let number = RECEIVED.load(Ordering::SeqCst);
    HANDLED
        .iter()
        .find(|&&(handled, _)| handled == number)
        .map(|&(number, name)| Signal { number, name })
}

/// Only an atomic store is async-signal-safe, so that is all the handler
/// does. The first signal is kept: it is the one that cancelled.
extern "C" fn on_signal(signal: libc::c_int) {
    let _ = RECEIVED.compare_exchange(0, signal, Ordering::SeqCst, Ordering::SeqCst);
}

/// The handlers installed for one evaluation. Dropping them restores each
/// signal's entry disposition.
pub struct Handlers {
    entry: Vec<(libc::c_int, libc::sigaction)>,
}

impl Handlers {
    /// Catch each handled signal that is not ignored, keeping its entry
    /// disposition to restore.
    pub fn install() -> io::Result<Handlers> {
        let mut handlers = Handlers { entry: Vec::new() };
        for (signal, _) in HANDLED {
            // SAFETY: a zeroed sigaction is a valid value, and with a null new
            // action sigaction only writes the current one into it.
            let mut entry: libc::sigaction = unsafe { std::mem::zeroed() };
            if unsafe { libc::sigaction(signal, ptr::null(), &mut entry) } == -1 {
                return Err(io::Error::last_os_error());
            }
            if entry.sa_sigaction == libc::SIG_IGN {
                continue;
            }
            // SAFETY: the action's mask is emptied before use, and the handler
            // performs one atomic operation.
            let installed = unsafe {
                let mut action: libc::sigaction = std::mem::zeroed();
                action.sa_sigaction = on_signal as extern "C" fn(libc::c_int) as libc::sighandler_t;
                action.sa_flags = libc::SA_RESTART;
                libc::sigemptyset(&mut action.sa_mask);
                libc::sigaction(signal, &action, ptr::null_mut())
            };
            if installed == -1 {
                return Err(io::Error::last_os_error());
            }
            handlers.entry.push((signal, entry));
        }
        Ok(handlers)
    }

    /// The linearization point. Block the handled signals in this thread; the
    /// threads that drain the worker have them blocked already, so from here
    /// on none of them is taken by any thread, and one that arrives stays
    /// pending. Then look once more: a signal the handler noted, or one now
    /// pending that the caller had not blocked, cancels. A signal the caller
    /// blocked was never a cancellation, and stays pending for the harness.
    ///
    /// The signals stay blocked whatever the answer: until [`reraise`]
    /// reports a cancellation, or the pre-exec hook reinstates the caller's
    /// mask.
    pub fn block_and_check(&self) -> Option<Signal> {
        // SAFETY: the sets are initialised by sigemptyset and sigpending
        // before use.
        let pending = unsafe {
            let mut handled: libc::sigset_t = std::mem::zeroed();
            libc::sigemptyset(&mut handled);
            for (signal, _) in &self.entry {
                libc::sigaddset(&mut handled, *signal);
            }
            libc::pthread_sigmask(libc::SIG_BLOCK, &handled, ptr::null_mut());
            let mut pending: libc::sigset_t = std::mem::zeroed();
            libc::sigemptyset(&mut pending);
            libc::sigpending(&mut pending);
            pending
        };
        received().or_else(|| {
            HANDLED
                .iter()
                .filter(|&&(signal, _)| self.entry.iter().any(|(handled, _)| *handled == signal))
                .filter(|&&(signal, _)| !signal_state::blocked_at_entry(signal))
                // SAFETY: the set was filled by sigpending.
                .find(|&&(signal, _)| unsafe { libc::sigismember(&pending, signal) } == 1)
                .map(|&(number, name)| Signal { number, name })
        })
    }
}

impl Drop for Handlers {
    fn drop(&mut self) {
        for (signal, entry) in &self.entry {
            // SAFETY: reinstating a disposition sigaction itself reported.
            unsafe { libc::sigaction(*signal, entry, ptr::null_mut()) };
        }
    }
}

/// Refuse with the cancellation if a handled signal has been received.
pub fn check(source: &str) -> Result<(), Refusal> {
    match received() {
        Some(signal) => Err(refusal(signal, source)),
        None => Ok(()),
    }
}

/// The refusal a cancelled selection reports before it re-raises `signal`.
pub fn refusal(signal: Signal, source: &str) -> Refusal {
    let name = signal.name();
    Refusal::new(
        "selection_cancelled",
        Stage::Evaluation,
        signal.exit(),
        format!(
            "selection with the policy {source} was cancelled by {name}: its worker was stopped \
             and reaped, and nothing was launched"
        ),
        format!(
            "nothing needs fixing if the signal was meant; otherwise run the same command again. \
             ravel ends by re-raising {name}, so its caller sees that signal"
        ),
    )
    .source(source)
    .signal(signal)
}

/// End this process with `signal`, under the disposition it had at entry,
/// which the evaluation's handlers restored when they were dropped. The
/// caller's mask comes back first, since the linearization point blocked the
/// handled signals: a cancelling signal still pending is delivered then.
pub fn reraise(signal: Signal) -> ! {
    // A signal death loses buffered output, and the refusal is the last thing
    // the caller reads.
    let _ = io::stdout().flush();
    let _ = io::stderr().flush();
    let _ = signal_state::restore_mask();
    // SAFETY: raise only sends a signal to this thread.
    unsafe { libc::raise(signal.number) };
    // Every handled signal terminates by default, a caught one could not have
    // been inherited, and one the caller blocked was never received, so this
    // is reached only if something else in the process has since changed that
    // disposition or blocked the signal.
    std::process::exit(i32::from(signal.exit()))
}

/// Run `start` with the handled signals blocked in this thread, so that a
/// thread it starts inherits them blocked and never takes one.
pub fn shielded<T>(start: impl FnOnce() -> T) -> T {
    // SAFETY: the sets are initialised by sigemptyset and pthread_sigmask
    // before use, and the calling thread's own mask is put back as it was.
    unsafe {
        let mut blocked: libc::sigset_t = std::mem::zeroed();
        libc::sigemptyset(&mut blocked);
        for (signal, _) in HANDLED {
            libc::sigaddset(&mut blocked, signal);
        }
        let mut previous: libc::sigset_t = std::mem::zeroed();
        libc::pthread_sigmask(libc::SIG_BLOCK, &blocked, &mut previous);
        let started = start();
        libc::pthread_sigmask(libc::SIG_SETMASK, &previous, ptr::null_mut());
        started
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The path the command seam cannot reach: a handled signal that arrives
    /// between the block and the look, which only `sigpending` can see, since
    /// the handler never runs. Here it is raised in the looking thread once
    /// the block is in place, so it is pending and nothing has noted it.
    #[test]
    fn a_handled_signal_pending_behind_the_block_cancels() {
        // SAFETY: the sets are initialised before use; the thread's mask and
        // the signal's disposition are put back before the test ends.
        unsafe {
            let mut mask: libc::sigset_t = std::mem::zeroed();
            libc::pthread_sigmask(libc::SIG_BLOCK, ptr::null(), &mut mask);
            let handlers = Handlers::install().unwrap();
            let &(signal, name) = HANDLED
                .iter()
                .find(|&&(signal, _)| {
                    handlers.entry.iter().any(|(handled, _)| *handled == signal)
                        && !signal_state::blocked_at_entry(signal)
                })
                .expect("a handled signal this test process neither ignores nor blocks");

            // The control: nothing pending and nothing noted.
            assert_eq!(handlers.block_and_check(), None);
            libc::raise(signal);
            let seen = handlers.block_and_check();
            assert_eq!(received(), None, "the handler ran, so the block did not");

            // Setting SIG_IGN discards the pending signal.
            libc::signal(signal, libc::SIG_IGN);
            drop(handlers);
            libc::pthread_sigmask(libc::SIG_SETMASK, &mask, ptr::null_mut());
            assert_eq!(seen.map(Signal::name), Some(name));
        }
    }
}
