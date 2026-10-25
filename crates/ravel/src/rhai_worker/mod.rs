//! The Rhai policy worker: Ravel's own binary in an internal role, the strict
//! Rhai engine behind a private socket (`docs/specs/policy-evaluation.md`,
//! *The policy worker*).
//!
//! The front side (`front`) launches the role through `worker::supervise`. The
//! role (`role`) is chosen by `arg0`, because it accepts no command-line input.
//! It reads one source from the evaluate frame, because it reads no file.
//!
//! What the removal increment does with the code beside this module:
//!
//! - It keeps this whole module, and in `worker.rs` the supervision (`supervise`,
//!   `Launch`, `Supervised`, `Conversation`, `Bounded` and everything they own).
//! - It removes the Bun protocol in `worker.rs` (`evaluate`, `locate`,
//!   `converse`, `verify_hello`, `Loaded` and the frame readers), `policy.rs`,
//!   and the build script's digest of the TypeScript sources.
//!
//! The wire protocol is length-prefixed JSON frames (`frame.rs`):
//!
//! | Frame | From | Fields |
//! |---|---|---|
//! | `hello` | worker | `protocol`, `packageVersion`, `buildId`, `rhaiVersion` |
//! | `evaluate` | front | `protocol`, `entry`, `source`, `request`, `bounds`, and `selectContext` when the front has a context to hand `select` |
//! | `selection` | worker | `version`: the policy's `VERSION`. `result`: the JSON form of what `select` returned |
//! | `failure` | worker | `failure`: a [`ScriptFailure`] |

pub mod convert;
pub mod engine;
pub mod front;
mod locate;
pub mod role;

use serde_json::{Value, json};

use crate::limits::Limits;

/// The `arg0` that marks the role. The front sets it, and `main` compares the
/// whole of `argv[0]` to it, so a path that ends in this name is no role.
pub const ARG0: &str = "ravel-rhai-policy-worker";

/// The version of this protocol. The Bun protocol has its own.
pub const PROTOCOL: u64 = 1;

/// The Rhai release the worker is built with, which `Cargo.toml` pins.
pub const RHAI_VERSION: &str = "1.27.0";

/// The digest of the crate sources this binary was built from (see build.rs).
pub const BUILD_ID: &str = env!("RAVEL_BUILD_ID");

/// The exit status of a role that refuses to start: input on the command line,
/// no socket at the channel descriptor, or a handshake that fails.
pub const EXIT_ROLE_REFUSED: u8 = 2;

/// The effective bounds the policy reads in `request.limits`. The worker keeps
/// its own copy, so nothing the policy does can raise one. The LLM and harness
/// budgets hold the defaults of the spec until the leaves that enforce them
/// read the owner settings.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Bounds {
    pub selection_ms: u64,
    pub context_bytes: u64,
    pub source_bytes: u64,
    pub sources: u64,
    pub message_bytes: u64,
    pub diagnostics_bytes: u64,
    pub llm_calls: u64,
    pub llm_output_tokens: u64,
    pub harness_runs: u64,
}

pub const LLM_CALLS_DEFAULT: u64 = 8;
pub const LLM_OUTPUT_TOKENS_DEFAULT: u64 = 32_768;
pub const HARNESS_RUNS_DEFAULT: u64 = 0;

/// The names of the bounds as the policy and the wire know them, in the order
/// of [`Bounds::values`].
const BOUND_NAMES: [&str; 9] = [
    "selectionMs",
    "contextBytes",
    "sourceBytes",
    "sources",
    "messageBytes",
    "diagnosticsBytes",
    "llmCalls",
    "llmOutputTokens",
    "harnessRuns",
];

impl Bounds {
    pub fn from_limits(limits: &Limits) -> Self {
        Bounds {
            selection_ms: limits.selection.value,
            context_bytes: limits.context.value,
            source_bytes: limits.source.value,
            sources: limits.sources.value,
            message_bytes: limits.message.value,
            diagnostics_bytes: limits.diagnostics.value,
            llm_calls: LLM_CALLS_DEFAULT,
            llm_output_tokens: LLM_OUTPUT_TOKENS_DEFAULT,
            harness_runs: HARNESS_RUNS_DEFAULT,
        }
    }

    fn values(self) -> [u64; 9] {
        [
            self.selection_ms,
            self.context_bytes,
            self.source_bytes,
            self.sources,
            self.message_bytes,
            self.diagnostics_bytes,
            self.llm_calls,
            self.llm_output_tokens,
            self.harness_runs,
        ]
    }

    /// The `limits` map of the request, keyed by the JSON names of the 1.x
    /// contract.
    pub fn to_json(self) -> Value {
        let mut map = serde_json::Map::new();
        for (name, value) in BOUND_NAMES.into_iter().zip(self.values()) {
            map.insert(name.to_owned(), json!(value));
        }
        Value::Object(map)
    }

    pub fn from_json(value: &Value) -> Result<Self, String> {
        let object = value.as_object().ok_or("the bounds are not an object")?;
        if object.len() != BOUND_NAMES.len() {
            return Err(format!("the bounds must hold exactly {BOUND_NAMES:?}"));
        }
        let mut read = [0u64; 9];
        for (slot, name) in read.iter_mut().zip(BOUND_NAMES) {
            *slot = object
                .get(name)
                .and_then(Value::as_u64)
                .ok_or_else(|| format!("the bound {name} is not a nonnegative integer"))?;
        }
        let [
            selection_ms,
            context_bytes,
            source_bytes,
            sources,
            message_bytes,
            diagnostics_bytes,
            llm_calls,
            llm_output_tokens,
            harness_runs,
        ] = read;
        Ok(Bounds {
            selection_ms,
            context_bytes,
            source_bytes,
            sources,
            message_bytes,
            diagnostics_bytes,
            llm_calls,
            llm_output_tokens,
            harness_runs,
        })
    }
}

/// Why the worker could not produce a selection: a stable code, with where it
/// happened. The seam leaf joins it to the refusal the spec names.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScriptFailure {
    /// A code of the spec's table: `policy_parse_failed`, `policy_import_failed`,
    /// `policy_invalid`, `unsupported_version`, `selection_threw`,
    /// `selection_malformed` or `message_too_large`.
    pub code: String,
    /// The stage of that code.
    pub stage: String,
    pub message: String,
    /// The entry the line and column are in.
    pub file: Option<String>,
    pub line: Option<u64>,
    pub column: Option<u64>,
    /// Where in a converted value the problem is, such as `result.args[2]`.
    pub path: Option<String>,
    /// The size a bound was exceeded at.
    pub actual: Option<u64>,
}

impl ScriptFailure {
    pub fn new(code: &str, stage: &str, message: impl Into<String>) -> Self {
        ScriptFailure {
            code: code.to_owned(),
            stage: stage.to_owned(),
            message: message.into(),
            file: None,
            line: None,
            column: None,
            path: None,
            actual: None,
        }
    }

    pub fn at(mut self, file: &str, line: Option<u64>, column: Option<u64>) -> Self {
        self.file = Some(file.to_owned());
        self.line = line;
        self.column = column;
        self
    }

    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }

    pub fn to_json(&self) -> Value {
        let mut map = serde_json::Map::new();
        map.insert("code".into(), json!(self.code));
        map.insert("stage".into(), json!(self.stage));
        map.insert("message".into(), json!(self.message));
        for (name, value) in [("file", &self.file), ("path", &self.path)] {
            if let Some(value) = value {
                map.insert(name.into(), json!(value));
            }
        }
        for (name, value) in [
            ("line", self.line),
            ("column", self.column),
            ("actual", self.actual),
        ] {
            if let Some(value) = value {
                map.insert(name.into(), json!(value));
            }
        }
        Value::Object(map)
    }

    pub fn from_json(value: &Value) -> Option<Self> {
        let text = |field: &str| value.get(field).and_then(Value::as_str).map(str::to_owned);
        let number = |field: &str| value.get(field).and_then(Value::as_u64);
        Some(ScriptFailure {
            code: text("code")?,
            stage: text("stage")?,
            message: text("message")?,
            file: text("file"),
            line: number("line"),
            column: number("column"),
            path: text("path"),
            actual: number("actual"),
        })
    }
}

/// What the worker reports once it has evaluated the entry.
#[derive(Clone, Debug, PartialEq)]
pub enum Reply {
    /// What `select` returned, with the `VERSION` the policy declared. The
    /// result is JSON, `null` for `()`. Judging it is the front's.
    Selection {
        version: String,
        result: Value,
    },
    Failure(ScriptFailure),
}

impl Reply {
    pub fn to_frame(&self) -> Value {
        match self {
            Reply::Selection { version, result } => {
                json!({ "type": "selection", "version": version, "result": result })
            }
            Reply::Failure(failure) => json!({ "type": "failure", "failure": failure.to_json() }),
        }
    }

    pub fn from_frame(frame: &Value) -> Option<Self> {
        match frame.get("type").and_then(Value::as_str)? {
            "selection" => Some(Reply::Selection {
                version: frame.get("version")?.as_str()?.to_owned(),
                result: frame.get("result")?.clone(),
            }),
            "failure" => ScriptFailure::from_json(frame.get("failure")?).map(Reply::Failure),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_pinned_rhai_release_is_the_one_the_worker_reports() {
        let manifest = include_str!("../../Cargo.toml");
        assert!(
            manifest.contains(&format!("rhai = {{ version = \"={RHAI_VERSION}\"")),
            "Cargo.toml must pin rhai =={RHAI_VERSION} with the compact_str feature"
        );
        assert!(!manifest.contains("unchecked"));
    }

    #[test]
    fn bounds_round_trip_through_their_wire_form() {
        let bounds = Bounds {
            selection_ms: 30_000,
            context_bytes: 262_144,
            source_bytes: 65_536,
            sources: 256,
            message_bytes: 1_048_576,
            diagnostics_bytes: 262_144,
            llm_calls: LLM_CALLS_DEFAULT,
            llm_output_tokens: LLM_OUTPUT_TOKENS_DEFAULT,
            harness_runs: HARNESS_RUNS_DEFAULT,
        };
        assert_eq!(Bounds::from_json(&bounds.to_json()), Ok(bounds));
        assert!(Bounds::from_json(&json!({})).is_err());
    }

    #[test]
    fn a_reply_round_trips_through_its_frame() {
        let selection = Reply::Selection {
            version: "1".to_owned(),
            result: json!({ "status": "selected" }),
        };
        assert_eq!(Reply::from_frame(&selection.to_frame()), Some(selection));
        let failure = Reply::Failure(
            ScriptFailure::new("selection_threw", "selection", "boom").at("p.rhai", Some(3), None),
        );
        assert_eq!(Reply::from_frame(&failure.to_frame()), Some(failure));
        assert_eq!(Reply::from_frame(&json!({ "type": "other" })), None);
    }
}
