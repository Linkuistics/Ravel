//! The deadline and the cancellation of one resolution
//! (docs/specs/skill-discovery.md, "Settings", "Bounds" and "Failures").
//!
//! A resolution looks at its watch between operations: at each entry of a
//! source, each block of a copy and each entry of a check or a seal. A
//! consumed INT, TERM or HUP comes first, and then the deadline. The watch
//! cannot interrupt one system call that storage holds, so a stalled call
//! delays the refusal until it returns.

use std::cell::RefCell;
use std::fmt;
use std::io;
#[cfg(test)]
use std::time::Duration;
use std::time::Instant;

use super::Located;
use super::settings::SkillSettings;
use crate::cancellation::Signal;
use crate::limits::{Bound, Origin};
use crate::refusal::{EXIT_TIMEOUT, Refusal, Stage};

/// What a resolution does at present, for the report of a stop.
#[derive(Clone, Debug)]
struct Active {
    operation: &'static str,
    /// The ID and the settings location of the active source.
    source: Option<(String, String)>,
}

/// The deadline of one resolution and its active operation.
pub struct Watch {
    started: Instant,
    deadline: Bound,
    active: RefCell<Active>,
}

impl Watch {
    /// A watch that starts now, with the effective deadline of `skills`.
    pub fn start(skills: &SkillSettings) -> Watch {
        Watch {
            started: Instant::now(),
            deadline: Bound {
                name: "resolutionTimeoutMs",
                unit: "ms",
                value: skills.resolution_timeout_ms,
                origin: if skills.resolution_timeout_set {
                    Origin::Set("settings.json")
                } else {
                    Origin::Default
                },
            },
            active: RefCell::new(Active {
                operation: "the start of the resolution",
                source: None,
            }),
        }
    }

    /// A watch with the largest deadline, for unit tests of one operation.
    #[cfg(test)]
    pub fn unbounded() -> Watch {
        Watch::start(&SkillSettings {
            resolution_timeout_ms: super::settings::RESOLUTION_MAX_MS,
            ..SkillSettings::default()
        })
    }

    /// Name the operation that runs from now on, and its source if it has
    /// one.
    pub fn enter(&self, operation: &'static str, source: Option<&Located>) {
        *self.active.borrow_mut() = Active {
            operation,
            source: source.map(|source| (source.id.clone(), source.location.clone())),
        };
    }

    /// Refuse when a signal was consumed or the deadline passed.
    pub fn check(&self) -> Result<(), Refusal> {
        if let Some(signal) = received() {
            return Err(self.cancelled(signal));
        }
        if self.started.elapsed() >= self.deadline.duration() {
            return Err(self.timeout());
        }
        Ok(())
    }

    /// [`Watch::check`] for code that reports I/O errors. The refusal rides
    /// in the error, and [`stopped`] takes it out again.
    pub fn tick(&self) -> io::Result<()> {
        self.check()
            .map_err(|refusal| io::Error::other(Stopped(refusal)))
    }

    /// A consumed signal wins over `refusal`, which the cancellation then
    /// keeps as its original failure. A cancellation stays as it is.
    pub fn overrule(&self, refusal: Refusal) -> Refusal {
        if refusal.signal.is_some() {
            return refusal;
        }
        match received() {
            None => refusal,
            Some(signal) => {
                let original = crate::refusal::Failure::from(refusal).to_text();
                self.cancelled(signal).report(
                    &format!("original failure:\n{original}"),
                    "retain the original failure details when retrying",
                )
            }
        }
    }

    fn during(&self) -> (String, Option<String>) {
        let active = self.active.borrow();
        match &active.source {
            Some((id, location)) => (
                format!("{} of the source {id:?}", active.operation),
                Some(location.clone()),
            ),
            None => (active.operation.to_owned(), None),
        }
    }

    fn timeout(&self) -> Refusal {
        let (during, location) = self.during();
        let refusal = Refusal::new(
            "skill_resolution_timeout",
            Stage::Skills,
            EXIT_TIMEOUT,
            format!(
                "the skill resolution passed its deadline of {} during {during}",
                self.deadline.to_text()
            ),
            "raise skills.resolutionTimeoutMs in the settings file, up to 3600000, or address the \
             latency of the storage or the transport of the source",
        )
        .bound(self.deadline);
        match location {
            Some(location) => refusal.location(location),
            None => refusal,
        }
    }

    fn cancelled(&self, signal: Signal) -> Refusal {
        let (during, location) = self.during();
        let name = signal.name();
        let refusal = Refusal::new(
            "skill_resolution_cancelled",
            Stage::Skills,
            signal.exit(),
            format!("the skill resolution was cancelled by {name} during {during}"),
            format!(
                "nothing needs fixing if the signal was meant; otherwise run the same command \
                 again. ravel ends by re-raising {name}, so its caller sees that signal"
            ),
        )
        .signal(signal);
        match location {
            Some(location) => refusal.location(location),
            None => refusal,
        }
    }
}

/// A refusal of the watch inside an I/O error.
#[derive(Debug)]
struct Stopped(Refusal);

impl fmt::Display for Stopped {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0.message)
    }
}

impl std::error::Error for Stopped {}

/// The refusal of the watch that `error` carries, or else `error` itself.
pub fn stopped(error: io::Error) -> Result<Refusal, io::Error> {
    if error
        .get_ref()
        .is_some_and(|inner| inner.downcast_ref::<Stopped>().is_some())
    {
        let inner = error.into_inner().expect("the error has an inner value");
        let Ok(stopped) = inner.downcast::<Stopped>() else {
            unreachable!("the inner value is a stop");
        };
        return Ok(stopped.0);
    }
    Err(error)
}

/// The consumed signal. The unit tests read only their own thread's fault
/// seam, because the latch of the process is shared by every test thread.
fn received() -> Option<Signal> {
    #[cfg(test)]
    return super::faults::signal();
    #[cfg(not(test))]
    crate::cancellation::received()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::skills::faults::{Hooks, install};

    fn settings(ms: u64, set: bool) -> SkillSettings {
        SkillSettings {
            resolution_timeout_ms: ms,
            resolution_timeout_set: set,
            ..SkillSettings::default()
        }
    }

    #[test]
    fn the_deadline_reports_its_value_and_origin() {
        let watch = Watch::start(&settings(
            super::super::settings::RESOLUTION_DEFAULT_MS,
            false,
        ));
        assert_eq!(watch.deadline.value, 120_000);
        assert_eq!(watch.deadline.to_text(), "120000 ms (the default)");
        let watch = Watch::start(&settings(1_000, true));
        assert_eq!(watch.deadline.to_text(), "1000 ms (settings.json)");
        assert!(watch.check().is_ok());
    }

    #[test]
    fn a_passed_deadline_names_the_active_source_and_operation() {
        let mut watch = Watch::start(&settings(1_000, true));
        watch.started -= Duration::from_millis(1_000);
        watch.enter(
            "capture",
            Some(&Located {
                id: "a".to_owned(),
                location: "skills.sources[0]".to_owned(),
            }),
        );
        let refusal = watch.check().unwrap_err();
        assert_eq!(refusal.code, "skill_resolution_timeout");
        assert_eq!(refusal.exit, 124);
        assert_eq!(refusal.location.as_deref(), Some("skills.sources[0]"));
        assert!(
            refusal
                .message
                .ends_with("1000 ms (settings.json) during capture of the source \"a\""),
            "{}",
            refusal.message
        );
        // The refusal rides through an I/O error and comes out whole.
        let error = watch.tick().unwrap_err();
        assert_eq!(stopped(error).unwrap().code, "skill_resolution_timeout");
        let plain = io::Error::other("plain");
        assert_eq!(stopped(plain).unwrap_err().to_string(), "plain");
    }

    #[test]
    fn a_consumed_signal_comes_before_the_deadline_and_wins_over_a_refusal() {
        let signal = Signal::of(libc::SIGHUP).unwrap();
        install(Hooks {
            signal: Some(signal),
            ..Hooks::default()
        });
        let mut watch = Watch::start(&settings(1_000, true));
        watch.started -= Duration::from_millis(1_000);
        let refusal = watch.check().unwrap_err();
        assert_eq!(refusal.code, "skill_resolution_cancelled");
        assert_eq!(refusal.exit, 129);
        assert_eq!(refusal.signal, Some(signal));

        let original = super::super::refuse("skill_source_missing", "gone", "restore it");
        let overruled = watch.overrule(original);
        assert_eq!(overruled.code, "skill_resolution_cancelled");
        assert!(
            overruled.message.contains("original failure:"),
            "{}",
            overruled.message
        );
        assert!(
            overruled.message.contains("skill_source_missing"),
            "{}",
            overruled.message
        );
        // A cancellation is not overruled again.
        assert_eq!(
            watch.overrule(refusal).message.matches("original").count(),
            0
        );
        install(Hooks::default());
        let kept = watch.overrule(super::super::refuse("skill_source_missing", "gone", "x"));
        assert_eq!(kept.code, "skill_source_missing");
    }
}
