//! Single-site reported faults for the F rows. Test builds only.
//!
//! One fixed site is selected by the running case; every other site is the
//! product's own code. A *mutation* site either omits the physical operation
//! (a failed mutation that changed nothing) or performs it and then reports
//! the substituted error; a *query* site always performs the physical query
//! and substitutes only the report. An irreversible operation is never
//! simulated by rewriting its report. The one broken subject ignores the
//! selected failure at its site, so the product proceeds as if it had not
//! happened.
use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Mode {
    Report,
    Omit,
    Perform,
}
impl Mode {
    fn name(self) -> &'static str {
        match self {
            Self::Report => "report",
            Self::Omit => "omit",
            Self::Perform => "perform",
        }
    }
}
thread_local! {
    static SELECTED: RefCell<Option<(String, Mode)>> = const { RefCell::new(None) };
}

/// Select the row's one site from its case, before anything else runs.
pub(super) fn select(case: &Value) {
    let (Some(site), Some(mode)) = (case["site"].as_str(), case["mode"].as_str()) else {
        return;
    };
    let mode = match mode {
        "report" => Mode::Report,
        "omit" => Mode::Omit,
        "perform" => Mode::Perform,
        other => panic!("unknown fixed fault mode {other}"),
    };
    SELECTED.with(|selected| *selected.borrow_mut() = Some((site.to_owned(), mode)));
}
fn selected(site: &str) -> Option<Mode> {
    if !active() {
        return None;
    }
    SELECTED.with(|selected| {
        selected
            .borrow()
            .as_ref()
            .filter(|(name, _)| name == site)
            .map(|&(_, mode)| mode)
    })
}
fn ignored() -> bool {
    BROKEN.get() == Broken::IgnoreFault
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
fn errno() -> i32 {
    io::Error::last_os_error().raw_os_error().unwrap_or(0)
}
/// The actual state after the site: who owns each handled disposition and what
/// the calling thread has blocked. Read-only.
fn after() -> Value {
    let owner = |signal| crate::cancellation::handler_owner(signal);
    let mut chld = unsafe { std::mem::zeroed::<libc::sigaction>() };
    let chld_queried = unsafe { libc::sigaction(libc::SIGCHLD, std::ptr::null(), &mut chld) };
    json!({"INT":owner(libc::SIGINT),"TERM":owner(libc::SIGTERM),"HUP":owner(libc::SIGHUP),
        "blocked":blocked_bits(),"pending":pending_bits(),
        "sigchld":{"queried":chld_queried,"ignored":chld.sa_sigaction == libc::SIG_IGN,
            "noChildWait":chld.sa_flags & libc::SA_NOCLDWAIT != 0}})
}
fn note(site: &str, mode: Mode, performed: bool, actual: i32, actual_errno: Option<i32>) {
    receipt(
        "fault-receipt.json",
        json!({"site":site,"mode":mode.name(),"operationPerformed":performed,
            "actualResult":actual,"actualErrno":actual_errno,"reported":"EIO",
            "failureIgnored":ignored(),"after":after()}),
    );
}

/// A `sigaction`-style mutation: `-1` with errno on failure.
pub(crate) fn mutation_c(site: &str, real: impl FnOnce() -> i32) -> i32 {
    let Some(mode) = selected(site) else {
        return real();
    };
    let performed = mode != Mode::Omit;
    let actual = if performed { real() } else { 0 };
    let physical = (actual == -1).then(errno);
    note(site, mode, performed, actual, physical);
    if ignored() {
        return actual;
    }
    set_errno(libc::EIO);
    -1
}
/// A `pthread_sigmask`-style mutation: the error number itself.
pub(crate) fn mutation_r(site: &str, real: impl FnOnce() -> i32) -> i32 {
    let Some(mode) = selected(site) else {
        return real();
    };
    let performed = mode != Mode::Omit;
    let actual = if performed { real() } else { 0 };
    note(
        site,
        mode,
        performed,
        actual,
        (actual != 0).then_some(actual),
    );
    if ignored() {
        return actual;
    }
    libc::EIO
}

/// What a selected query site does with the physical result it just produced.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Query {
    /// Not selected: use the physical result.
    Real,
    /// The report is replaced by EIO (errno set); the physical query ran.
    Substituted,
    /// The broken subject: the failure is ignored and the answer is empty.
    Ignored,
}
pub(crate) fn query(site: &str, actual: i32) -> Query {
    let Some(mode) = selected(site) else {
        return Query::Real;
    };
    let physical = (actual == -1).then(errno);
    note(site, mode, true, actual, physical);
    if ignored() {
        return Query::Ignored;
    }
    set_errno(libc::EIO);
    Query::Substituted
}
