//! The discovery calls of the `ravel` namespace (`docs/specs/policy-evaluation.md`,
//! *Discovery*): what the owner configured, whether it is available now, and what
//! the evaluation still has.
//!
//! Discovery uses no network and runs no program. It reports configuration and
//! availability, and it never recommends. Each function builds the `value` of an
//! `ok` reply, a JSON document that the worker converts to Rhai values.

use std::ffi::OsStr;
use std::path::Path;

use serde_json::{Value, json};

use crate::harness_entries::Harnesses;
use crate::program;

/// `ravel::harnesses()`: one map for each entry, in name order. The program
/// resolves as a selected program resolves, and nothing runs. This looks at the
/// filesystem, so the reader thread calls it.
pub(super) fn harnesses(harnesses: &Harnesses, cwd: &Path, path: Option<&OsStr>) -> Value {
    harnesses
        .iter()
        .map(|(name, harness)| {
            let found = program::resolve(
                &harness.program,
                &format!("the harness entry {name}"),
                cwd,
                path,
            );
            let (resolved, reason) = match found {
                Ok(executable) => (json!(executable.path), Value::Null),
                Err(refusal) if refusal.code == "program_not_found" => {
                    (Value::Null, json!("not_found"))
                }
                Err(_) => (Value::Null, json!("not_executable")),
            };
            json!({
                "name": name,
                "program": harness.program,
                "available": resolved.is_string(),
                "resolved": resolved,
                "reason": reason,
                "capabilities": harness.capabilities,
            })
        })
        .collect()
}
