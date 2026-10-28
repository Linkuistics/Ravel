//! Optional owner settings: selection bound, context budget, policy environment
//! grants, and the harness entries and budget of a Rhai configuration script.

use std::ffi::OsStr;
use std::fs;
use std::io::ErrorKind;
use std::ops::RangeInclusive;
use std::path::Path;

use serde_json::Value;

use crate::environment;
use crate::harness_entries::{self, Harnesses, Problem};
use crate::limits::{
    CONTEXT_MAX_BYTES, CONTEXT_MIN_BYTES, HARNESS_RUNS_MAX, SELECTION_MAX_MS, SELECTION_MIN_MS,
};
use crate::refusal::{EXIT_MALFORMED, Refusal, Stage};

/// The settings file, relative to HOME, beside the personal configuration script.
pub const SETTINGS_FILE: &str = ".config/ravel/settings.json";

/// Where a value the file set is reported as coming from.
pub const ORIGIN: &str = "settings.json";

#[derive(Debug, Default)]
pub struct Settings {
    pub timeout_ms: Option<u64>,
    pub context_bytes: Option<u64>,
    pub config_env: Vec<String>,
    pub harnesses: Harnesses,
    pub budgets: BudgetSettings,
}

/// The owner's choices for the budgets, each within the range of the spec table.
/// A budget the owner leaves out keeps its default.
#[derive(Debug, Default)]
pub struct BudgetSettings {
    pub harness_runs: Option<u64>,
}

/// What `ravel::harnesses()` reports on: the owner's entries, checked and by
/// name.
#[derive(Clone, Debug, Default)]
pub struct Directory {
    pub harnesses: Harnesses,
}

impl Settings {
    /// The owner's settings, checked whole. A missing file sets nothing.
    pub fn read(home: Option<&OsStr>) -> Result<Settings, Refusal> {
        let Some(home) = home.map(Path::new).filter(|home| home.is_absolute()) else {
            return Ok(Settings::default());
        };
        let path = home.join(SETTINGS_FILE);
        let file = path.to_string_lossy().into_owned();
        // A regular file only, as for the configuration entry: a FIFO there would
        // hold every command before any bound applies.
        match fs::metadata(&path) {
            Err(error) if error.kind() == ErrorKind::NotFound => return Ok(Settings::default()),
            Err(error) => return Err(invalid(&file, format!("cannot be read: {error}"))),
            Ok(metadata) if !metadata.is_file() => {
                return Err(invalid(&file, "is not a regular file"));
            }
            Ok(_) => {}
        }
        let bytes =
            fs::read(&path).map_err(|error| invalid(&file, format!("cannot be read: {error}")))?;
        let document: Value = serde_json::from_slice(&bytes)
            .map_err(|error| invalid(&file, format!("is not JSON: {error}")))?;
        let Value::Object(document) = document else {
            return Err(invalid(&file, "is not a JSON object"));
        };

        let mut settings = Settings::default();
        for (key, value) in &document {
            match key.as_str() {
                "timeoutMs" => {
                    let range = SELECTION_MIN_MS..=SELECTION_MAX_MS;
                    settings.timeout_ms = Some(bound(&file, key, value, "milliseconds", range)?);
                }
                "contextBytes" => {
                    let range = CONTEXT_MIN_BYTES..=CONTEXT_MAX_BYTES;
                    settings.context_bytes = Some(bound(&file, key, value, "bytes", range)?);
                }
                "configEnv" => {
                    let names = value.as_array().ok_or_else(|| {
                        invalid(
                            &file,
                            format!("holds a configEnv, {value}, that is not an array"),
                        )
                        .location(key)
                    })?;
                    for name in names {
                        let name = name.as_str().ok_or_else(|| {
                            invalid(
                                &file,
                                format!("holds {name} in configEnv, which is not a string"),
                            )
                            .location(key)
                        })?;
                        environment::grantable(name, &environment::Named::Setting { file: &file })?;
                        settings.config_env.push(name.to_owned());
                    }
                }
                "harnesses" => {
                    settings.harnesses =
                        harness_entries::parse_harnesses(value).map_err(|p| problem(&file, p))?;
                }
                "budgets" => settings.budgets = budgets(&file, value)?,
                _ => {
                    return Err(
                        invalid(&file, format!("holds the unknown key {key:?}")).location(key)
                    );
                }
            }
        }
        Ok(settings)
    }

    /// The entries that discovery reports on.
    pub fn directory(&self) -> Directory {
        Directory {
            harnesses: self.harnesses.clone(),
        }
    }
}

fn problem(file: &str, problem: Problem) -> Refusal {
    invalid(
        file,
        format!("is invalid at {}: it {}", problem.location, problem.what),
    )
    .location(problem.location)
}

/// The `budgets` key: an object of whole numbers within the ranges of the spec
/// table, with no key besides `harnessRuns`.
fn budgets(file: &str, value: &Value) -> Result<BudgetSettings, Refusal> {
    let fields = value.as_object().ok_or_else(|| {
        invalid(
            file,
            format!("holds budgets, {value}, that is not an object"),
        )
        .location("budgets")
    })?;
    let mut budgets = BudgetSettings::default();
    for (key, value) in fields {
        let (slot, max) = match key.as_str() {
            "harnessRuns" => (&mut budgets.harness_runs, HARNESS_RUNS_MAX),
            _ => {
                return Err(invalid(
                    file,
                    format!(
                        "holds budgets.{key}, an unknown budget: the only budget is harnessRuns"
                    ),
                )
                .location(format!("budgets.{key}")));
            }
        };
        let at = format!("budgets.{key}");
        *slot = Some(value.as_u64().filter(|n| *n <= max).ok_or_else(|| {
            invalid(
                file,
                format!("holds {at}, {value}, that is not a whole number from 0 to {max}"),
            )
            .location(&at)
        })?);
    }
    Ok(budgets)
}

fn invalid(file: &str, what: impl std::fmt::Display) -> Refusal {
    Refusal::new(
        "settings_invalid",
        Stage::Cli,
        EXIT_MALFORMED,
        format!("the owner settings file {file} {what}"),
        "make it one JSON object holding any of timeoutMs, contextBytes, configEnv (an array of variable names), harnesses and budgets, or remove it to set nothing",
    )
    .source(file)
}

/// A bound's setting, held to its flag's rule: a whole number within `range`.
fn bound(
    file: &str,
    key: &str,
    value: &Value,
    unit: &str,
    range: RangeInclusive<u64>,
) -> Result<u64, Refusal> {
    value
        .as_u64()
        .filter(|value| range.contains(value))
        .ok_or_else(|| {
            Refusal::new(
                "malformed_input",
                Stage::Cli,
                EXIT_MALFORMED,
                format!(
                    "{key} in the owner settings file {file} is {value}, not a whole number of \
                     {unit} from {} to {}",
                    range.start(),
                    range.end()
                ),
                format!("set {key} to a whole number in that range, or remove it for the default"),
            )
            .source(file)
            .location(key)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read(contents: &str) -> Result<Settings, Refusal> {
        let home = tempfile::tempdir().unwrap();
        let path = home.path().join(SETTINGS_FILE);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, contents).unwrap();
        Settings::read(Some(home.path().as_os_str()))
    }

    #[test]
    fn a_missing_file_or_a_home_that_cannot_place_one_sets_nothing() {
        let home = tempfile::tempdir().unwrap();
        for home in [
            Some(home.path().as_os_str()),
            Some(OsStr::new("relative")),
            None,
        ] {
            let settings = Settings::read(home).unwrap();
            assert_eq!(settings.timeout_ms, None);
            assert_eq!(settings.context_bytes, None);
            assert!(settings.config_env.is_empty());
        }
    }

    #[test]
    fn every_key_is_optional_and_each_is_read() {
        assert_eq!(read("{}").unwrap().timeout_ms, None);
        let settings =
            read(r#"{ "timeoutMs": 600000, "contextBytes": 1, "configEnv": ["A_TOKEN"] }"#)
                .unwrap();
        assert_eq!(settings.timeout_ms, Some(600_000));
        assert_eq!(settings.context_bytes, Some(1));
        assert_eq!(settings.config_env, ["A_TOKEN"]);
    }

    #[test]
    fn a_value_is_held_to_its_flags_rule_and_the_file_to_its_shape() {
        for (contents, code, key) in [
            (
                r#"{ "timeoutMs": 999 }"#,
                "malformed_input",
                Some("timeoutMs"),
            ),
            (
                r#"{ "timeoutMs": 600001 }"#,
                "malformed_input",
                Some("timeoutMs"),
            ),
            (
                r#"{ "timeoutMs": "5000" }"#,
                "malformed_input",
                Some("timeoutMs"),
            ),
            (
                r#"{ "timeoutMs": 5000.5 }"#,
                "malformed_input",
                Some("timeoutMs"),
            ),
            (
                r#"{ "contextBytes": 0 }"#,
                "malformed_input",
                Some("contextBytes"),
            ),
            (
                r#"{ "contextBytes": 8388609 }"#,
                "malformed_input",
                Some("contextBytes"),
            ),
            (
                r#"{ "configEnv": ["LD_PRELOAD"] }"#,
                "excluded_grant",
                Some("configEnv"),
            ),
            (
                r#"{ "configEnv": ["A=B"] }"#,
                "malformed_input",
                Some("configEnv"),
            ),
            (
                r#"{ "configEnv": [""] }"#,
                "malformed_input",
                Some("configEnv"),
            ),
            (
                r#"{ "configEnv": [7] }"#,
                "settings_invalid",
                Some("configEnv"),
            ),
            (
                r#"{ "configEnv": "A_TOKEN" }"#,
                "settings_invalid",
                Some("configEnv"),
            ),
            (
                r#"{ "stateDir": "records" }"#,
                "settings_invalid",
                Some("stateDir"),
            ),
            (r#"{ "stateDir": 7 }"#, "settings_invalid", Some("stateDir")),
            (
                r#"{ "config": "/p/policy.rhai" }"#,
                "settings_invalid",
                Some("config"),
            ),
            ("[]", "settings_invalid", None),
            ("{", "settings_invalid", None),
            ("", "settings_invalid", None),
        ] {
            let refusal = read(contents).unwrap_err();
            assert_eq!(refusal.code, code, "{contents}");
            assert_eq!(refusal.exit, EXIT_MALFORMED, "{contents}");
            assert_eq!(refusal.location.as_deref(), key, "{contents}");
            let file = refusal.source.as_deref().expect("the file is named");
            assert!(file.ends_with(SETTINGS_FILE), "{contents}: {file}");
            assert!(
                refusal.message.contains(file),
                "{contents}: {}",
                refusal.message
            );
        }
    }

    #[test]
    fn the_two_keys_of_a_rhai_policy_are_read_and_default_to_nothing() {
        let none = read("{}").unwrap();
        assert!(none.harnesses.is_empty());
        assert_eq!(none.budgets.harness_runs, None);
        let settings = read(
            r#"{
                "harnesses": { "claude": { "program": "claude", "capabilities": ["edit"] } },
                "budgets": { "harnessRuns": 16 }
            }"#,
        )
        .unwrap();
        assert_eq!(settings.harnesses["claude"].program, "claude");
        assert_eq!(settings.budgets.harness_runs, Some(16));
        let zero = read(r#"{ "budgets": { "harnessRuns": 0 } }"#).unwrap();
        assert_eq!(zero.budgets.harness_runs, Some(0));
    }

    #[test]
    fn a_problem_in_a_new_key_is_settings_invalid_at_its_location_whatever_the_key_order() {
        for (contents, location) in [
            // The `endpoints` key is gone, so it is an unknown key.
            (r#"{ "endpoints": {} }"#, "endpoints"),
            (r#"{ "harnesses": { "a": {} } }"#, "harnesses.a.program"),
            (r#"{ "budgets": 5 }"#, "budgets"),
            // The two LLM budgets are gone, so their keys are unknown budgets.
            (r#"{ "budgets": { "llmCalls": 8 } }"#, "budgets.llmCalls"),
            (
                r#"{ "budgets": { "llmOutputTokens": 4096 } }"#,
                "budgets.llmOutputTokens",
            ),
            (
                r#"{ "budgets": { "harnessRuns": -1 } }"#,
                "budgets.harnessRuns",
            ),
            (
                r#"{ "budgets": { "harnessRuns": 1.5 } }"#,
                "budgets.harnessRuns",
            ),
            (
                r#"{ "budgets": { "harnessRuns": "8" } }"#,
                "budgets.harnessRuns",
            ),
            (
                r#"{ "budgets": { "harnessRuns": 17 } }"#,
                "budgets.harnessRuns",
            ),
            (r#"{ "budgets": { "ms": 5 } }"#, "budgets.ms"),
        ] {
            let refusal = read(contents).unwrap_err();
            assert_eq!(refusal.code, "settings_invalid", "{contents}");
            assert_eq!(refusal.exit, EXIT_MALFORMED, "{contents}");
            assert_eq!(refusal.location.as_deref(), Some(location), "{contents}");
        }
    }

    #[test]
    fn a_settings_path_that_is_not_a_regular_file_refuses() {
        let home = tempfile::tempdir().unwrap();
        fs::create_dir_all(home.path().join(SETTINGS_FILE)).unwrap();
        let refusal = Settings::read(Some(home.path().as_os_str())).unwrap_err();
        assert_eq!(refusal.code, "settings_invalid");
    }
}
