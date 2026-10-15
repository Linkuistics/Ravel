use std::io::{self, Write as _};
use std::ptr;
use std::sync::atomic::{AtomicI32, Ordering};

use crate::refusal::{Refusal, Stage};
use crate::signal_state;

const HANDLED: [(libc::c_int, &str); 3] = [
    (libc::SIGINT, "SIGINT"),
    (libc::SIGTERM, "SIGTERM"),
    (libc::SIGHUP, "SIGHUP"),
];
static RECEIVED: AtomicI32 = AtomicI32::new(0);
#[cfg(test)]
static PENDING_FAULT: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
#[cfg(test)]
static BLOCK_FAULT: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
/// A signal number raised on the finalising thread after the entry mask is
/// restored and immediately before the final block: the deterministic boundary
/// at which a runner handler can run with nothing yet blocked.
#[cfg(test)]
static RAISE_BEFORE_FINAL_BLOCK: AtomicI32 = AtomicI32::new(0);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Signal {
    number: libc::c_int,
    name: &'static str,
}
impl Signal {
    pub fn of(number: i32) -> Option<Self> {
        HANDLED
            .iter()
            .find(|&&(n, _)| n == number)
            .map(|&(number, name)| Self { number, name })
    }
    pub fn name(self) -> &'static str {
        self.name
    }
    pub fn exit(self) -> u8 {
        u8::try_from(128 + self.number).unwrap_or(u8::MAX)
    }
}
pub fn received() -> Option<Signal> {
    #[cfg(test)]
    if crate::native_entry_tests::broken_join() == crate::native_entry_tests::Join::Front {
        // Private break: the front latch is never joined.
        return None;
    }
    Signal::of(RECEIVED.load(Ordering::SeqCst))
}

/// Which owner's handler is installed for `number`, by comparing the installed
/// action's address with the front's own handler. Read-only; test builds only.
#[cfg(test)]
pub(crate) fn handler_owner(number: libc::c_int) -> &'static str {
    let mut action = unsafe { std::mem::zeroed::<libc::sigaction>() };
    if unsafe { libc::sigaction(number, ptr::null(), &mut action) } == -1 {
        return "unknown";
    }
    let front = on_signal as extern "C" fn(libc::c_int) as libc::sighandler_t;
    match action.sa_sigaction {
        libc::SIG_DFL => "default",
        libc::SIG_IGN => "ignore",
        address if address == front => "front",
        _ => "other",
    }
}
extern "C" fn on_signal(signal: libc::c_int) {
    let _ = RECEIVED.compare_exchange(0, signal, Ordering::SeqCst, Ordering::SeqCst);
}

fn checked(result: i32) -> io::Result<()> {
    if result == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}
fn mask_result(result: i32) -> io::Result<()> {
    if result == 0 {
        Ok(())
    } else {
        Err(io::Error::from_raw_os_error(result))
    }
}
fn handled_set() -> io::Result<libc::sigset_t> {
    let mut set = unsafe { std::mem::zeroed::<libc::sigset_t>() };
    checked(unsafe { libc::sigemptyset(&mut set) })?;
    for (signal, _) in HANDLED {
        checked(unsafe { libc::sigaddset(&mut set, signal) })?;
    }
    Ok(set)
}

/// Front-owned protection, retained even after partial installation. No Drop
/// performs a hidden transition: all release reports belong to finalisation.
pub struct Handlers {
    entry: Vec<(libc::c_int, libc::sigaction)>,
    blocked: bool,
    installed_complete: bool,
}
impl Handlers {
    pub fn new() -> Self {
        Self {
            entry: Vec::new(),
            blocked: false,
            installed_complete: false,
        }
    }

    pub fn begin(&mut self) -> io::Result<()> {
        let set = handled_set()?;
        let mut previous = unsafe { std::mem::zeroed::<libc::sigset_t>() };
        mask_result(unsafe { libc::pthread_sigmask(libc::SIG_BLOCK, &set, &mut previous) })?;
        self.blocked = true;
        RECEIVED.store(0, Ordering::SeqCst);
        for (signal, _) in HANDLED {
            let mut original = unsafe { std::mem::zeroed::<libc::sigaction>() };
            checked(unsafe { libc::sigaction(signal, ptr::null(), &mut original) })?;
            if original.sa_sigaction == libc::SIG_IGN {
                continue;
            }
            let mut action = unsafe { std::mem::zeroed::<libc::sigaction>() };
            action.sa_sigaction = on_signal as extern "C" fn(libc::c_int) as libc::sighandler_t;
            action.sa_flags = libc::SA_RESTART;
            checked(unsafe { libc::sigemptyset(&mut action.sa_mask) })?;
            // Own the original before any attempted mutation, including a
            // physical mutation whose reported result is later substituted.
            self.entry.push((signal, original));
            checked(unsafe { libc::sigaction(signal, &action, ptr::null_mut()) })?;
        }
        self.installed_complete = true;
        mask_result(unsafe {
            libc::pthread_sigmask(libc::SIG_SETMASK, &previous, ptr::null_mut())
        })?;
        self.blocked = false;
        Ok(())
    }

    #[cfg(test)]
    pub fn install() -> io::Result<Self> {
        let mut scope = Self::new();
        scope.begin()?;
        Ok(scope)
    }
    #[cfg(test)]
    pub fn block_and_check(&mut self) -> Option<Signal> {
        self.try_block_and_check().expect("checked signal snapshot")
    }

    pub fn try_block_and_check(&mut self) -> io::Result<Option<Signal>> {
        self.blocked = false;
        #[cfg(test)]
        if BLOCK_FAULT.load(Ordering::Relaxed) {
            return Err(io::Error::from_raw_os_error(libc::EIO));
        }
        let set = handled_set()?;
        mask_result(unsafe { libc::pthread_sigmask(libc::SIG_BLOCK, &set, ptr::null_mut()) })?;
        self.blocked = true;
        let mut pending = unsafe { std::mem::zeroed::<libc::sigset_t>() };
        let queried = unsafe { libc::sigpending(&mut pending) };
        #[cfg(test)]
        let queried = if PENDING_FAULT.load(Ordering::Relaxed) {
            return Err(io::Error::from_raw_os_error(libc::EIO));
        } else {
            queried
        };
        checked(queried)?;
        let mut eligible = None;
        for (number, name) in HANDLED {
            let member = unsafe { libc::sigismember(&pending, number) };
            if member == -1 {
                return Err(io::Error::last_os_error());
            }
            if member == 1
                && eligible.is_none()
                && {
                    #[cfg(not(test))]
                    {
                        !signal_state::blocked_at_entry(number)
                    }
                    #[cfg(test)]
                    {
                        !signal_state::blocked_at_entry(number)
                            || crate::native_entry_tests::broken_blocked_pending()
                    }
                }
                && !signal_state::ignored_at_entry(number)
            {
                eligible = Some(Signal { number, name });
            }
        }
        Ok(received().or(eligible))
    }

    pub fn finalise(&mut self, outcome: Option<Signal>) -> (Option<Signal>, Vec<io::Error>) {
        #[cfg(test)]
        let outcome = if crate::native_entry_tests::broken_join()
            == crate::native_entry_tests::Join::Outcome
        {
            None
        } else {
            outcome
        };
        let front = received();
        let mut reports = Vec::new();
        if let Err(error) = self.restore_entry_mask() {
            reports.push(io::Error::new(
                error.kind(),
                format!("entry mask restore: {error}"),
            ));
        }
        // Handlers are still present while entry restoration delivers eligible
        // pending cancellation. Readers retain their independently inherited mask.
        let front = front.or(received());
        #[cfg(test)]
        match RAISE_BEFORE_FINAL_BLOCK.swap(0, Ordering::SeqCst) {
            0 => {}
            number => assert_eq!(unsafe { libc::raise(number) }, 0),
        }
        let pending = match self.try_block_and_check() {
            Ok(signal) => signal,
            Err(error) => {
                reports.push(io::Error::new(
                    error.kind(),
                    format!("final signal snapshot: {error}"),
                ));
                None
            }
        };
        // The runner's handlers outlive supervision and write only their own
        // latch. Consume it after the block attempt: a handler that ran before
        // it has stored by now, and a later signal stays pending. A read
        // before the block lets a signal land between the read and the block,
        // consumed and so absent from the pending set.
        let late = keyed_launch::take_interrupt().and_then(Signal::of);
        #[cfg(test)]
        let late =
            if crate::native_entry_tests::broken_join() == crate::native_entry_tests::Join::Late {
                None
            } else {
                late
            };
        let signal = front.or(outcome).or(late).or(received()).or(pending);
        let mut released = self.blocked;
        if self.blocked {
            for (number, action) in &self.entry {
                if let Err(error) =
                    checked(unsafe { libc::sigaction(*number, action, ptr::null_mut()) })
                {
                    reports.push(io::Error::new(
                        error.kind(),
                        format!("signal {number} disposition release: {error}"),
                    ));
                    released = false;
                }
            }
        } else {
            reports.push(io::Error::other(
                "main mask not confirmed blocked; handlers retained",
            ));
        }
        if released && signal.is_none() {
            if let Err(error) = signal_state::restore_mask() {
                reports.push(io::Error::new(
                    error.kind(),
                    format!("final mask release: {error}"),
                ));
            } else {
                self.blocked = false;
            }
        }
        (signal, reports)
    }

    pub fn restore_entry_mask(&mut self) -> io::Result<()> {
        if !self.installed_complete {
            return Err(io::Error::other("partial signal installation; entry-mask release deferred until the blocked final snapshot"));
        }
        self.blocked = false;
        signal_state::restore_mask()
    }
}

pub fn check(source: &str) -> Result<(), Refusal> {
    match received() {
        Some(signal) => Err(refusal(signal, source)),
        None => Ok(()),
    }
}
pub fn refusal(signal: Signal, source: &str) -> Refusal {
    let name = signal.name();
    Refusal::new("selection_cancelled", Stage::Evaluation, signal.exit(),
        format!("selection with the policy {source} was cancelled by {name}: worker teardown was attempted and nothing was launched"),
        format!("nothing needs fixing if the signal was meant; otherwise run the same command again. ravel ends by re-raising {name}, so its caller sees that signal"))
        .source(source).signal(signal)
}

/// A reader inherits this block permanently; only its creator is restored.
pub struct Shield {
    previous: libc::sigset_t,
}
impl Shield {
    pub fn block() -> io::Result<Self> {
        #[cfg(test)]
        if crate::worker::native_tests::block_fault()
            == Some(crate::worker::native_tests::Mutation::Omitted)
        {
            return Err(io::Error::from_raw_os_error(libc::EIO));
        }
        let set = handled_set()?;
        let mut previous = unsafe { std::mem::zeroed::<libc::sigset_t>() };
        mask_result(unsafe { libc::pthread_sigmask(libc::SIG_BLOCK, &set, &mut previous) })?;
        #[cfg(test)]
        if crate::worker::native_tests::block_fault().is_some() {
            return Err(io::Error::from_raw_os_error(libc::EIO));
        }
        Ok(Self { previous })
    }
    pub fn restore(self) -> io::Result<()> {
        #[cfg(test)]
        if crate::worker::native_tests::restore_fault()
            == Some(crate::worker::native_tests::Mutation::Omitted)
        {
            return Err(io::Error::from_raw_os_error(libc::EIO));
        }
        mask_result(unsafe {
            libc::pthread_sigmask(libc::SIG_SETMASK, &self.previous, ptr::null_mut())
        })?;
        #[cfg(test)]
        if crate::worker::native_tests::restore_fault().is_some() {
            return Err(io::Error::from_raw_os_error(libc::EIO));
        }
        Ok(())
    }
}

pub fn reraise(signal: Signal) -> ! {
    let _ = io::stdout().flush();
    let _ = io::stderr().flush();
    let propagate = (|| -> io::Result<()> {
        let handled = handled_set()?;
        mask_result(unsafe { libc::pthread_sigmask(libc::SIG_BLOCK, &handled, ptr::null_mut()) })?;
        let mut action = unsafe { std::mem::zeroed::<libc::sigaction>() };
        action.sa_sigaction = libc::SIG_DFL;
        checked(unsafe { libc::sigemptyset(&mut action.sa_mask) })?;
        #[cfg(not(test))]
        let defaulted = unsafe { libc::sigaction(signal.number, &action, ptr::null_mut()) };
        #[cfg(test)]
        let defaulted = propagation_tests::default_action(signal.number, &action);
        checked(defaulted).map_err(|error| {
            io::Error::new(error.kind(), format!("selected default action: {error}"))
        })?;
        let mut selected = unsafe { std::mem::zeroed::<libc::sigset_t>() };
        checked(unsafe { libc::sigemptyset(&mut selected) })?;
        checked(unsafe { libc::sigaddset(&mut selected, signal.number) })?;
        #[cfg(not(test))]
        let unblocked =
            unsafe { libc::pthread_sigmask(libc::SIG_UNBLOCK, &selected, ptr::null_mut()) };
        #[cfg(test)]
        let unblocked = propagation_tests::unblock(&selected);
        mask_result(unblocked).map_err(|error| {
            io::Error::new(error.kind(), format!("selected signal unblock: {error}"))
        })?;
        #[cfg(not(test))]
        let raised = unsafe { libc::raise(signal.number) };
        #[cfg(test)]
        let raised = propagation_tests::raise(signal.number);
        mask_result(raised).map_err(|error| {
            io::Error::new(error.kind(), format!("selected signal raise: {error}"))
        })
    })();
    if let Err(error) = propagate {
        eprintln!("ravel: cannot propagate {}: {error}", signal.name());
    }
    std::process::exit(i32::from(signal.exit()))
}

#[cfg(test)]
mod propagation_tests;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn final_block_failure_retains_handlers_even_after_a_previous_block() {
        let mut handlers = Handlers::install().unwrap();
        assert!(handlers.try_block_and_check().unwrap().is_none());
        signal_state::restore_mask().unwrap();
        BLOCK_FAULT.store(true, Ordering::Relaxed);
        let (_, reports) = handlers.finalise(None);
        BLOCK_FAULT.store(false, Ordering::Relaxed);
        let mut action = unsafe { std::mem::zeroed::<libc::sigaction>() };
        assert_eq!(
            unsafe { libc::sigaction(libc::SIGTERM, ptr::null(), &mut action) },
            0
        );
        let kept =
            action.sa_sigaction == on_signal as extern "C" fn(libc::c_int) as libc::sighandler_t;
        let _ = handlers.finalise(None);
        assert!(!reports.is_empty());
        assert!(kept, "stale mask knowledge authorised disposition release");
    }

    /// A real launch leaves the runner's handlers installed over the front's,
    /// so a signal after the entry restore sets only the runner latch.
    fn leave_runner_handlers_installed() {
        let argv = keyed_launch::Argv::new("/usr/bin/true".into(), Vec::new());
        let ended = keyed_launch::run_noninteractive(
            keyed_launch::NoninteractiveLaunch {
                argv: &argv,
                scrub: &[],
                cwd: None,
                escalation: keyed_launch::Escalation {
                    grace: std::time::Duration::from_secs(1),
                    kill_grace: std::time::Duration::from_secs(1),
                },
            },
            tempfile::tempfile().unwrap(),
        )
        .expect("the trivial launch ends");
        assert!(matches!(ended.end, keyed_launch::End::Exited));
        assert_eq!(keyed_launch::take_interrupt(), None);
    }

    #[test]
    fn a_runner_signal_between_the_entry_restore_and_the_final_block_cancels() {
        // Break: read the runner latch before the final block. TERM then runs
        // the runner handler after that read, is consumed, and is never pending.
        let mut handlers = Handlers::install().unwrap();
        leave_runner_handlers_installed();
        RAISE_BEFORE_FINAL_BLOCK.store(libc::SIGTERM, Ordering::SeqCst);
        let (signal, reports) = handlers.finalise(None);
        assert_eq!(
            RAISE_BEFORE_FINAL_BLOCK.load(Ordering::SeqCst),
            0,
            "the boundary was never reached"
        );
        assert_eq!(received(), None, "the front handler ran; wrong boundary");
        let _ = signal_state::restore_mask();
        assert!(reports.is_empty(), "{reports:?}");
        assert_eq!(signal.map(Signal::name), Some("SIGTERM"));
    }

    #[test]
    fn a_runner_signal_survives_a_failed_final_block() {
        // Break: drop the runner latch when the final block fails.
        let mut handlers = Handlers::install().unwrap();
        leave_runner_handlers_installed();
        RAISE_BEFORE_FINAL_BLOCK.store(libc::SIGTERM, Ordering::SeqCst);
        BLOCK_FAULT.store(true, Ordering::Relaxed);
        let (signal, reports) = handlers.finalise(None);
        BLOCK_FAULT.store(false, Ordering::Relaxed);
        let _ = signal_state::restore_mask();
        assert!(!reports.is_empty(), "the failed block was not reported");
        assert_eq!(signal.map(Signal::name), Some("SIGTERM"));
    }

    #[test]
    fn an_unknown_pending_set_never_confirms_absence() {
        // Break: ignore sigpending failure and accept an empty zeroed set.
        let mut handlers = Handlers::install().unwrap();
        PENDING_FAULT.store(true, Ordering::Relaxed);
        let observed = handlers.try_block_and_check();
        PENDING_FAULT.store(false, Ordering::Relaxed);
        assert!(observed.is_err(), "unknown pending state became absence");
    }

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
            let mut handlers = Handlers::install().unwrap();
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
