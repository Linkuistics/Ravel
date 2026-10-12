use std::fmt::Write as _;

use serde_json::{json, Map, Value};

use crate::cancellation::Signal;
use crate::limits::Bound;

/// Exit results before exec.
pub const EXIT_MALFORMED: u8 = 2;
pub const EXIT_REFUSED: u8 = 3;
pub const EXIT_WORKER: u8 = 5;
pub const EXIT_TIMEOUT: u8 = 124;
pub const EXIT_UNEXECUTABLE: u8 = 126;
pub const EXIT_NOT_FOUND: u8 = 127;

/// Where in the invocation a refusal arose. The names are part of the JSON
/// contract, so they are stable strings rather than `Debug` output.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    /// The command line itself, and the owner settings its flags replace.
    Cli,
    /// Choosing and opening the policy entry.
    Authority,
    /// Finding, starting and verifying the compiled worker, and its protocol.
    Worker,
    /// The worker's evaluation as a whole, from its start to its result. A
    /// timeout names this stage, because the front cannot see which part of
    /// the evaluation was running when the bound ran out.
    Evaluation,
    /// Importing the policy entry.
    Load,
    /// Reading the caller's context document, or assembling, validating and
    /// measuring the context delivered to selection.
    Context,
    /// The policy's exported shape.
    Validation,
    /// Calling a valid policy's `select` and judging what it returned.
    Selection,
    /// Finding the selected command's program.
    Resolution,
    /// Starting the harness, from the linearization point on.
    Exec,
}

impl Stage {
    pub fn as_str(self) -> &'static str {
        match self {
            Stage::Cli => "cli",
            Stage::Authority => "authority",
            Stage::Worker => "worker",
            Stage::Evaluation => "evaluation",
            Stage::Load => "load",
            Stage::Context => "context",
            Stage::Validation => "validation",
            Stage::Selection => "selection",
            Stage::Resolution => "resolution",
            Stage::Exec => "exec",
        }
    }
}

/// A refusal, boxed so that every `Result` carrying one stays a pointer wide
/// on its success path. Its fields read through `Deref`.
#[derive(Debug)]
pub struct Refusal(Box<Details>);

#[derive(Debug)]
pub struct Details {
    pub code: &'static str,
    pub stage: Stage,
    pub message: String,
    pub remedy: String,
    pub exit: u8,
    /// The caller input involved, as the caller spelled it (`--kind impl`).
    pub input: Option<String>,
    /// The file the refusal is about, usually the resolved policy entry.
    pub source: Option<String>,
    /// Where inside `source` the problem is (`policy.select`, `result.args[2]`).
    pub location: Option<String>,
    /// A policy's own code for a refusal it returned, reported beside the
    /// stable `policy_refused` rather than in its place.
    pub policy_code: Option<String>,
    /// The bound whose exhaustion this refusal reports.
    pub bound: Option<Bound>,
    pub signal: Option<Signal>,
}

impl std::ops::Deref for Refusal {
    type Target = Details;

    fn deref(&self) -> &Details {
        &self.0
    }
}

impl Refusal {
    pub fn new(
        code: &'static str,
        stage: Stage,
        exit: u8,
        message: impl Into<String>,
        remedy: impl Into<String>,
    ) -> Self {
        Refusal(Box::new(Details {
            code,
            stage,
            message: message.into(),
            remedy: remedy.into(),
            exit,
            input: None,
            source: None,
            location: None,
            policy_code: None,
            bound: None,
            signal: None,
        }))
    }

    pub fn input(mut self, input: impl Into<String>) -> Self {
        self.0.input = Some(input.into());
        self
    }

    pub fn source(mut self, source: impl Into<String>) -> Self {
        self.0.source = Some(source.into());
        self
    }

    pub fn location(mut self, location: impl Into<String>) -> Self {
        self.0.location = Some(location.into());
        self
    }

    pub fn policy_code(mut self, code: impl Into<String>) -> Self {
        self.0.policy_code = Some(code.into());
        self
    }

    pub fn bound(mut self, bound: Bound) -> Self {
        self.0.bound = Some(bound);
        self
    }

    pub fn signal(mut self, signal: Signal) -> Self {
        self.0.signal = Some(signal);
        self
    }
}

/// What the policy printed while the worker evaluated it. Captured from the
/// worker's own stdout and stderr, which carry no protocol, and reported
/// separately from any result.
#[derive(Clone, Debug, Default)]
pub struct Diagnostics {
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

impl Diagnostics {
    pub fn to_json(&self) -> Value {
        json!({
            "stdout": String::from_utf8_lossy(&self.stdout),
            "stderr": String::from_utf8_lossy(&self.stderr),
        })
    }

    /// Each captured line, prefixed with the stream it came from, for text mode.
    pub fn to_text(&self) -> String {
        let mut text = String::new();
        for (stream, bytes) in [("stdout", &self.stdout), ("stderr", &self.stderr)] {
            let captured = String::from_utf8_lossy(bytes);
            if captured.is_empty() {
                continue;
            }
            for line in captured.strip_suffix('\n').unwrap_or(&captured).split('\n') {
                let _ = writeln!(text, "policy {stream}: {line}");
            }
        }
        text
    }
}

#[derive(Debug)]
pub struct Failure {
    pub refusal: Refusal,
    pub diagnostics: Option<Diagnostics>,
}

impl From<Refusal> for Failure {
    fn from(refusal: Refusal) -> Self {
        Failure {
            refusal,
            diagnostics: None,
        }
    }
}

impl Failure {
    pub fn with_diagnostics(refusal: Refusal, diagnostics: Diagnostics) -> Self {
        Failure {
            refusal,
            diagnostics: Some(diagnostics),
        }
    }

    pub fn to_json(&self) -> Value {
        let refusal = &self.refusal;
        let mut error = Map::new();
        error.insert("code".into(), refusal.code.into());
        error.insert("stage".into(), refusal.stage.as_str().into());
        error.insert("message".into(), refusal.message.clone().into());
        for (name, value) in [
            ("input", &refusal.input),
            ("source", &refusal.source),
            ("location", &refusal.location),
            ("policyCode", &refusal.policy_code),
        ] {
            if let Some(value) = value {
                error.insert(name.into(), value.clone().into());
            }
        }
        if let Some(bound) = &refusal.bound {
            let mut named = bound.to_json();
            named["name"] = bound.name.into();
            error.insert("bound".into(), named);
        }
        if let Some(signal) = refusal.signal {
            error.insert("signal".into(), signal.name().into());
        }
        error.insert("remedy".into(), refusal.remedy.clone().into());
        error.insert("exit".into(), refusal.exit.into());
        let mut document = Map::new();
        document.insert("schemaVersion".into(), 1.into());
        document.insert("error".into(), Value::Object(error));
        if let Some(diagnostics) = &self.diagnostics {
            document.insert("diagnostics".into(), diagnostics.to_json());
        }
        Value::Object(document)
    }

    pub fn to_text(&self) -> String {
        let refusal = &self.refusal;
        let mut text = String::new();
        if let Some(diagnostics) = &self.diagnostics {
            text.push_str(&diagnostics.to_text());
        }
        let _ = writeln!(
            text,
            "ravel: refused ({}, stage {}): {}",
            refusal.code,
            refusal.stage.as_str(),
            refusal.message
        );
        for (label, value) in [
            ("input", &refusal.input),
            ("source", &refusal.source),
            ("location", &refusal.location),
            ("policy code", &refusal.policy_code),
        ] {
            if let Some(value) = value {
                let _ = writeln!(text, "  {label}: {value}");
            }
        }
        if let Some(bound) = &refusal.bound {
            let _ = writeln!(text, "  bound: {} {}", bound.name, bound.to_text());
        }
        if let Some(signal) = refusal.signal {
            let _ = writeln!(
                text,
                "  signal: {}, re-raised once this is reported",
                signal.name()
            );
        }
        let _ = writeln!(text, "  remedy: {}", refusal.remedy);
        text
    }
}
