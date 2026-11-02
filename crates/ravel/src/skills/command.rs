//! `ravel skills resolve`: source acquisition with no configuration script,
//! result channel or harness
//! (docs/specs/skill-discovery.md#ravel-skills-resolve).

use std::io::Write as _;
use std::path::PathBuf;

use serde_json::json;

use super::{Resolution, resolve};
use crate::cancellation::{Handlers, Signal};
use crate::refusal::{EXIT_WORKER, Failure, Refusal, Stage};
use crate::settings::Settings;

/// Resolve the configured sources and write the result to standard output.
///
/// The handlers of INT, TERM and HUP stay installed through the resolution,
/// so that a signal stops it at its next look at the watch, and its staging
/// is removed. Ravel then reports `skill_resolution_cancelled` and raises
/// the same signal again.
pub fn run(json: bool) -> Result<(), Failure> {
    crate::signal_state::recorded()?;
    let mut handlers = Handlers::new();
    let output = handlers
        .begin()
        .map_err(|error| unavailable("cancellation_unavailable", &error.to_string()).into())
        .and_then(|()| output(json));
    let (signal, reports) = handlers.finalise(None);
    let output = settle(output, signal, &reports)?;
    let mut stdout = std::io::stdout().lock();
    stdout
        .write_all(output.as_bytes())
        .and_then(|()| stdout.flush())
        .map_err(|error| {
            Refusal::new(
                "output_unavailable",
                Stage::Skills,
                EXIT_WORKER,
                format!("Ravel cannot write the resolution to standard output: {error}"),
                "give the command a writable standard output, then resolve again",
            )
            .into()
        })
}

fn output(json: bool) -> Result<String, Failure> {
    let home = std::env::var_os("HOME").map(PathBuf::from);
    let settings = Settings::read(home.as_deref().map(std::path::Path::as_os_str))?;
    Ok(match resolve(home.as_deref(), &settings)? {
        Resolution::Disabled if json => {
            format!("{}\n", json!({"schemaVersion": 1, "status": "disabled"}))
        }
        Resolution::Disabled => String::new(),
        Resolution::Resolved(resolved) if json => format!("{}\n", resolved.to_json()),
        Resolution::Resolved(resolved) => format!("{}\n", resolved.instructions),
    })
}

/// A signal that the final look found and that the resolution did not see
/// wins over its output and over its refusal, which the report keeps. A
/// report of the signal state joins a refusal, or replaces the output.
fn settle(
    output: Result<String, Failure>,
    signal: Option<Signal>,
    reports: &[std::io::Error],
) -> Result<String, Failure> {
    let output = match (output, signal) {
        (Err(failure), _) if failure.refusal.signal.is_some() => Err(failure),
        (output, Some(signal)) => Err(late(signal, output.err()).into()),
        (output, None) => output,
    };
    let mut reports = reports.iter().map(ToString::to_string);
    match output {
        Ok(output) => match reports.next() {
            None => Ok(output),
            Some(first) => Err(reports
                .fold(
                    unavailable("signal_state_unavailable", &first),
                    |refusal, report| {
                        refusal.report(&report, "inspect the signal state of the process")
                    },
                )
                .into()),
        },
        Err(mut failure) => {
            for report in reports {
                failure.refusal = failure
                    .refusal
                    .report(&report, "inspect the signal state of the process");
            }
            Err(failure)
        }
    }
}

/// The cancellation for a signal that arrived after the last look of the
/// watch, with the original failure when there was one.
fn late(signal: Signal, original: Option<Failure>) -> Refusal {
    let name = signal.name();
    let refusal = Refusal::new(
        "skill_resolution_cancelled",
        Stage::Skills,
        signal.exit(),
        format!("the skill resolution was cancelled by {name} at its end"),
        format!(
            "nothing needs fixing if the signal was meant; otherwise run the same command again. \
             ravel ends by re-raising {name}, so its caller sees that signal"
        ),
    )
    .signal(signal);
    match original {
        None => refusal,
        Some(failure) => refusal.report(
            &format!("original failure:\n{}", failure.to_text()),
            "retain the original failure details when retrying",
        ),
    }
}

fn unavailable(code: &'static str, message: &str) -> Refusal {
    Refusal::new(
        code,
        Stage::Skills,
        EXIT_WORKER,
        format!("Ravel cannot manage the signals of the resolution: {message}"),
        "run the command again; if the failure stays, inspect the signal state of the process",
    )
}
