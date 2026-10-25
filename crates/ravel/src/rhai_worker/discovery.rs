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

use crate::endpoints::{Endpoints, Harnesses};
use crate::program;

/// `ravel::endpoints()`: one map for each endpoint, in name order. The
/// credential is read to learn whether it is set, and it is never reported.
/// `environment` is the lookup into the caller's environment. It makes no
/// filesystem call.
pub(super) fn endpoints(
    endpoints: &Endpoints,
    environment: impl Fn(&str) -> Option<std::ffi::OsString>,
) -> Value {
    endpoints
        .iter()
        .map(|(name, endpoint)| {
            // An endpoint with no credential variable needs none.
            let set = endpoint.api_key_env.as_deref().is_none_or(|variable| {
                environment(variable).is_some_and(|value| !value.is_empty())
            });
            json!({
                "name": name,
                "type": endpoint.api.name(),
                "available": set,
                "reason": if set { Value::Null } else { json!("credential_unset") },
                "models": endpoint.models,
                "structuredOutput": endpoint.structured_output(),
            })
        })
        .collect()
}

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
