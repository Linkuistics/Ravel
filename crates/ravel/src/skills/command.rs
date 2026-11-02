//! `ravel skills resolve`: source acquisition with no configuration script,
//! result channel or harness
//! (docs/specs/skill-discovery.md#ravel-skills-resolve).

use std::io::Write as _;
use std::path::PathBuf;

use serde_json::json;

use super::{Resolution, resolve};
use crate::refusal::{EXIT_WORKER, Failure, Refusal, Stage};
use crate::settings::Settings;

/// Resolve the configured sources and write the result to standard output.
pub fn run(json: bool) -> Result<(), Failure> {
    let home = std::env::var_os("HOME").map(PathBuf::from);
    let settings = Settings::read(home.as_deref().map(std::path::Path::as_os_str))?;
    let output = match resolve(home.as_deref(), &settings)? {
        Resolution::Disabled if json => {
            format!("{}\n", json!({"schemaVersion": 1, "status": "disabled"}))
        }
        Resolution::Disabled => String::new(),
        Resolution::Resolved(resolved) if json => format!("{}\n", resolved.to_json()),
        Resolution::Resolved(resolved) => format!("{}\n", resolved.instructions),
    };
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
