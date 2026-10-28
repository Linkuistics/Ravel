//! The harness entries of the owner settings
//! (`docs/specs/policy-evaluation.md`, *Harness entries*).
//!
//! The owner names a harness outside the script, so a configuration script can pick an entry
//! but never a program path of its own. This module checks the shape of one
//! entry and holds what the checked entry says. It runs no program:
//! `ravel::harnesses()` resolves the program from the front, at the time of the
//! call.

use std::collections::BTreeMap;

use serde_json::{Map, Value};

/// A name has 1 to 64 lowercase letters, digits and dashes, and its first
/// character is a letter or a digit.
const NAME_MAX: usize = 64;

/// The owner's harness entries, by name.
pub type Harnesses = BTreeMap<String, Harness>;

/// What is wrong with one entry of the settings, and where. The caller turns it
/// into the refusal and names the file.
#[derive(Debug, PartialEq, Eq)]
pub struct Problem {
    /// A dotted path such as `harnesses.claude.program`.
    pub location: String,
    pub what: String,
}

impl Problem {
    fn new(location: impl Into<String>, what: impl Into<String>) -> Self {
        Problem {
            location: location.into(),
            what: what.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Harness {
    pub program: String,
    pub capabilities: Vec<String>,
}

/// The map under `harnesses` of the settings, entry by entry.
pub fn parse_harnesses(value: &Value) -> Result<Harnesses, Problem> {
    let entries = object(value, "harnesses")?;
    entries
        .iter()
        .map(|(name, entry)| Ok((name.clone(), harness(name, entry)?)))
        .collect()
}

fn object<'a>(value: &'a Value, location: &str) -> Result<&'a Map<String, Value>, Problem> {
    value
        .as_object()
        .ok_or_else(|| Problem::new(location, format!("holds {value}, which is not an object")))
}

/// Why `name` is no harness name, if it is not.
fn name_problem(name: &str, location: &str) -> Option<Problem> {
    let first_ok = name
        .chars()
        .next()
        .is_some_and(|first| first.is_ascii_lowercase() || first.is_ascii_digit());
    let rest_ok = name
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-');
    (name.len() > NAME_MAX || !first_ok || !rest_ok).then(|| {
        Problem::new(
            location,
            format!(
                "holds the name {name:?}, which is not 1 to {NAME_MAX} lowercase letters, digits \
                 and dashes that begin with a letter or a digit"
            ),
        )
    })
}

/// An entry's fields, refusing a field that is not in `known`.
fn fields<'a>(
    entry: &'a Value,
    location: &str,
    known: &[&str],
) -> Result<&'a Map<String, Value>, Problem> {
    let map = object(entry, location)?;
    match map.keys().find(|key| !known.contains(&key.as_str())) {
        Some(key) => Err(Problem::new(
            format!("{location}.{key}"),
            format!(
                "is an unknown field, and {location} holds only {}",
                known.join(", ")
            ),
        )),
        None => Ok(map),
    }
}

fn string<'a>(
    map: &'a Map<String, Value>,
    location: &str,
    key: &str,
) -> Result<Option<&'a str>, Problem> {
    match map.get(key) {
        None => Ok(None),
        Some(Value::String(text)) if !text.is_empty() && !text.contains('\0') => {
            Ok(Some(text.as_str()))
        }
        Some(other) => Err(Problem::new(
            format!("{location}.{key}"),
            format!("holds {other}, which is not a nonempty string"),
        )),
    }
}

/// An array of strings, each nonempty. Absent is empty.
fn strings(map: &Map<String, Value>, location: &str, key: &str) -> Result<Vec<String>, Problem> {
    let here = format!("{location}.{key}");
    let Some(value) = map.get(key) else {
        return Ok(Vec::new());
    };
    let items = value
        .as_array()
        .ok_or_else(|| Problem::new(&here, format!("holds {value}, which is not an array")))?;
    items
        .iter()
        .map(|item| match item {
            Value::String(text) if !text.is_empty() => Ok(text.clone()),
            other => Err(Problem::new(
                &here,
                format!("holds {other}, which is not a nonempty string"),
            )),
        })
        .collect()
}

fn harness(name: &str, entry: &Value) -> Result<Harness, Problem> {
    let location = format!("harnesses.{name}");
    if let Some(problem) = name_problem(name, "harnesses") {
        return Err(problem);
    }
    let map = fields(entry, &location, &["program", "capabilities"])?;
    let program = string(map, &location, "program")?
        .ok_or_else(|| Problem::new(format!("{location}.program"), "is required"))?
        .to_owned();
    Ok(Harness {
        program,
        capabilities: strings(map, &location, "capabilities")?,
    })
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn a_harness_entry_has_a_program_and_optional_capability_labels() {
        let parsed = parse_harnesses(&json!({
            "claude": { "program": "claude", "capabilities": ["edit", "shell"] },
            "bare": { "program": "/opt/bin/agent" },
        }))
        .unwrap();
        assert_eq!(parsed["claude"].program, "claude");
        assert_eq!(parsed["claude"].capabilities, ["edit", "shell"]);
        assert!(parsed["bare"].capabilities.is_empty());
        for (value, location) in [
            (json!(3), "harnesses"),
            (json!({ "A": { "program": "x" } }), "harnesses"),
            (json!({ "a": "x" }), "harnesses.a"),
            (json!({ "a": {} }), "harnesses.a.program"),
            (json!({ "a": { "program": "" } }), "harnesses.a.program"),
            (json!({ "a": { "program": 4 } }), "harnesses.a.program"),
            (
                json!({ "a": { "program": "x", "capabilities": "edit" } }),
                "harnesses.a.capabilities",
            ),
            (
                json!({ "a": { "program": "x", "capabilities": [1] } }),
                "harnesses.a.capabilities",
            ),
            (
                json!({ "a": { "program": "x", "args": [] } }),
                "harnesses.a.args",
            ),
        ] {
            assert_eq!(
                parse_harnesses(&value).unwrap_err().location,
                location,
                "{value}"
            );
        }
    }

    #[test]
    fn a_name_has_one_to_sixty_four_lowercase_letters_digits_and_dashes_and_does_not_begin_with_a_dash()
     {
        let entry = json!({ "program": "x" });
        let long = "a".repeat(64);
        for good in ["a", "0", "main-1", "9-lives", long.as_str()] {
            let map = json!({ good: entry.clone() });
            assert!(parse_harnesses(&map).is_ok(), "{good}");
        }
        let too_long = "a".repeat(65);
        for bad in ["", "-a", "A", "a_b", "a b", "é", "a.b", too_long.as_str()] {
            let map = json!({ bad: entry.clone() });
            assert_eq!(
                parse_harnesses(&map).unwrap_err().location,
                "harnesses",
                "{bad:?}"
            );
        }
    }
}
