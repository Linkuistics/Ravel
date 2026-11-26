//! The WebAssembly bindings of the core, for the workbench page.
//!
//! The page calls the core through typed objects:
//! `docs/adr/typed-objects-at-the-webassembly-boundary.md`. The exports follow
//! the section "The interface of the core" of `docs/specs/tracer.md`.
//!
//! A domain failure, such as a grammar with errors, crosses as a tagged value.
//! An `Err` of an export throws in JavaScript
//! (https://wasm-bindgen.github.io/wasm-bindgen/reference/types/result.html),
//! so a `JsError` stands only for a fault of the conversion. `Ts<T>` converts
//! inside the function, so such a fault cannot leak memory
//! (https://github.com/madonoharu/tsify, section "Why are the `wasm_abi`
//! attributes deprecated?").

use serde::Serialize;
use tsify::{Ts, Tsify};
use wasm_bindgen::prelude::*;
use workbench_core::{
    CheckedGrammar, Diagrams, Session, TraceGrammar,
    checked::Diagnostic,
    marks::{Marks, MarksError},
    trace::{NextOption, OpenError, PathNode, ReadOutcome, Readings, Step, StepError, TraceDiagnostic, TreeNode},
};

/// Parses a grammar text and runs the rule checks. The result holds the
/// diagnostics, the rules and the badges of the recursive groups.
#[wasm_bindgen(js_name = checkGrammar)]
pub fn check_grammar(text: &str) -> Result<Ts<CheckedGrammar>, JsError> {
    Ok(workbench_core::checked_grammar(text).into_ts()?)
}

/// Lays out the diagram of each rule of a grammar text: the operation
/// Diagrams of the section "The interface of the core" of
/// `docs/specs/rule-diagrams.md`. A grammar with errors has no diagrams.
#[wasm_bindgen]
pub fn diagrams(text: &str) -> Result<Ts<DiagramsOutcome>, JsError> {
    let outcome = match workbench_core::diagrams(text) {
        Ok(diagrams) => DiagramsOutcome::Diagrams { diagrams },
        Err(diagnostics) => DiagramsOutcome::GrammarErrors { diagnostics },
    };
    Ok(outcome.into_ts()?)
}

/// The outcome of the operation Diagrams.
#[derive(Debug, Serialize, Tsify)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum DiagramsOutcome {
    /// The diagram of each rule, in the order of the text.
    Diagrams { diagrams: Diagrams },
    /// The grammar has errors, so it has no diagrams. These are its
    /// diagnostics with severity error.
    GrammarErrors { diagnostics: Vec<Diagnostic> },
}

/// Opens a trace session on a grammar text, and optionally on the text of a
/// trace file. With a trace file, the session reads as many steps as it can.
#[wasm_bindgen(js_name = openSession)]
pub fn open_session(grammar: &str, trace_file: Option<String>) -> OpenOutcome {
    match Session::open(grammar, trace_file.as_deref()) {
        Ok(opened) => OpenOutcome::opened(opened.session, opened.warnings, 0),
        Err(error) => OpenOutcome::refused(error),
    }
}

/// Opens a trace session on a grammar text, the text of a trace file and the
/// symbols of an edit of its sequence. The session reads as many steps as it
/// can.
///
/// A step of the file keeps its attributes when the edit changes neither that
/// step nor any step before it. A step also keeps them when the edit changes
/// neither that step nor any step after it. Each other symbol becomes a step
/// without attributes. The report counts the steps of the file that lost
/// their attributes.
#[wasm_bindgen(js_name = openEditedTraceFile)]
pub fn open_edited_trace_file(grammar: &str, trace_file: &str, symbols: Vec<String>) -> OpenOutcome {
    let opened = match Session::open(grammar, Some(trace_file)) {
        Ok(opened) => opened,
        Err(error) => return OpenOutcome::refused(error),
    };
    let (steps, attributes_dropped) = edited_steps(opened.session.steps(), symbols);
    match Session::with_steps(opened.session.grammar(), steps) {
        Ok(session) => OpenOutcome::opened(session, opened.warnings, attributes_dropped),
        Err(error) => OpenOutcome::failed(OpenReport::InvalidStep { error }),
    }
}

/// The steps of an edit of a sequence, and the number of steps of the file
/// with attributes that the edit changed or removed. The steps that the edit
/// leaves unchanged at the start and at the end keep their attributes.
fn edited_steps(file: &[Step], symbols: Vec<String>) -> (Vec<Step>, usize) {
    let same = |(step, symbol): &(&Step, &String)| step.symbol == **symbol;
    let prefix = file.iter().zip(&symbols).take_while(same).count();
    let suffix = file[prefix..].iter().rev().zip(symbols[prefix..].iter().rev()).take_while(same).count();
    let kept_end = file.len() - suffix;
    let attributes_dropped = file[prefix..kept_end].iter().filter(|step| !step.attributes.is_empty()).count();
    let typed_end = symbols.len() - suffix;
    let mut steps = file[..prefix].to_vec();
    steps.extend(symbols.into_iter().take(typed_end).skip(prefix).map(Step::new));
    steps.extend_from_slice(&file[kept_end..]);
    (steps, attributes_dropped)
}

/// Opens a trace session on a grammar text and a sequence of symbols. The
/// steps have no attributes. The session reads as many steps as it can.
#[wasm_bindgen(js_name = openSequence)]
pub fn open_sequence(grammar: &str, symbols: Vec<String>) -> OpenOutcome {
    let grammar = match TraceGrammar::new(grammar) {
        Ok(grammar) => grammar,
        Err(diagnostics) => return OpenOutcome::failed(OpenReport::GrammarErrors { diagnostics }),
    };
    match Session::with_steps(&grammar, symbols.into_iter().map(Step::new).collect()) {
        Ok(session) => OpenOutcome::opened(session, Vec::new(), 0),
        Err(error) => OpenOutcome::failed(OpenReport::InvalidStep { error }),
    }
}

/// The report of an attempt to open a session.
#[derive(Debug, Serialize, Tsify)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum OpenReport {
    /// The session opened. A trace file can add warnings, such as a grammar
    /// hash that differs.
    Opened {
        warnings: Vec<TraceDiagnostic>,
        /// The number of steps of the trace file with attributes that an edit
        /// changed or removed. Without an edit, it is 0.
        #[serde(rename = "attributesDropped")]
        attributes_dropped: usize,
    },
    /// The grammar has errors, so it has no traces. These are its diagnostics
    /// with severity error.
    GrammarErrors { diagnostics: Vec<Diagnostic> },
    /// The trace file has errors. These are all its diagnostics.
    TraceFileErrors { diagnostics: Vec<TraceDiagnostic> },
    /// A symbol breaks the rules of a step, as an empty symbol does.
    InvalidStep { error: StepError },
}

/// The outcome of an attempt to open a session: a report, and the session
/// when it opened.
#[wasm_bindgen]
pub struct OpenOutcome {
    report: OpenReport,
    session: Option<TraceSession>,
}

impl OpenOutcome {
    fn opened(session: Session, warnings: Vec<TraceDiagnostic>, attributes_dropped: usize) -> Self {
        Self {
            report: OpenReport::Opened { warnings, attributes_dropped },
            session: Some(TraceSession { session, readings: None }),
        }
    }

    fn refused(error: OpenError) -> Self {
        match error {
            OpenError::Grammar(diagnostics) => Self::failed(OpenReport::GrammarErrors { diagnostics }),
            OpenError::TraceFile(diagnostics) => Self::failed(OpenReport::TraceFileErrors { diagnostics }),
        }
    }

    fn failed(report: OpenReport) -> Self {
        Self { report, session: None }
    }
}

#[wasm_bindgen]
impl OpenOutcome {
    /// The report.
    pub fn report(&self) -> Result<Ts<OpenReport>, JsError> {
        Ok(self.report.into_ts()?)
    }

    /// The session, when it opened. A second call returns nothing.
    #[wasm_bindgen(js_name = takeSession)]
    pub fn take_session(&mut self) -> Option<TraceSession> {
        self.session.take()
    }
}

/// A trace session: a grammar without errors, a sequence and the number of
/// steps read. The state on view is the state after the steps read.
#[wasm_bindgen]
pub struct TraceSession {
    session: Session,
    /// The readings of the state on view, from the first call that needs them.
    readings: Option<Readings>,
}

impl TraceSession {
    fn readings(&mut self) -> &Readings {
        let k = self.session.steps_read();
        self.readings.get_or_insert_with(|| self.session.readings_after(k))
    }
}

#[wasm_bindgen]
impl TraceSession {
    /// Reads the next step of the sequence.
    pub fn read(&mut self) -> Result<Ts<ReadOutcome>, JsError> {
        self.readings = None;
        Ok(self.session.read().into_ts()?)
    }

    /// Drops the last step read. The step stays in the sequence. It returns
    /// false before the first step.
    pub fn back(&mut self) -> bool {
        self.readings = None;
        self.session.back()
    }

    /// Inserts a step without attributes right after the steps read, and does
    /// not read it. At the end of the sequence, this appends the step. It
    /// returns the error of a symbol that breaks the rules of a step.
    pub fn insert(&mut self, symbol: String) -> Result<Option<Ts<StepError>>, JsError> {
        self.readings = None;
        match self.session.insert(Step::new(symbol)) {
            Ok(()) => Ok(None),
            Err(error) => Ok(Some(error.into_ts()?)),
        }
    }

    /// The state on view, with each listed reading but without trees.
    pub fn state(&mut self) -> Result<Ts<SessionState>, JsError> {
        let state = self.session.state();
        let readings = self.readings();
        let summary = SessionState {
            steps: state.steps,
            steps_read: state.steps_read,
            failure_point: state.failure_point,
            can_end: state.can_end,
            next_options: state.next_options,
            cap_reached: readings.cap_reached,
            readings: readings
                .readings
                .iter()
                .map(|r| ReadingSummary {
                    parent: r.parent,
                    can_end: r.can_end,
                    next_options: r.next_options.clone(),
                    position: r.position.clone(),
                    follow_path: r.follow_path.clone(),
                })
                .collect(),
        };
        Ok(summary.into_ts()?)
    }

    /// The tree of one listed reading, by its number from 1. A number outside
    /// the list returns nothing.
    pub fn tree(&mut self, reading: usize) -> Result<Option<Ts<TreeNode>>, JsError> {
        let readings = self.readings();
        match reading.checked_sub(1).and_then(|i| readings.readings.get(i)) {
            Some(r) => Ok(Some(r.tree.into_ts()?)),
            None => Ok(None),
        }
    }

    /// The marks of the open diagram of `open_path` for the state on view:
    /// the operation Marks of the section "The interface of the core" of
    /// `docs/specs/rule-diagrams.md`. `focus` is the number of the step in
    /// focus. An open path that is not a chain of references from the start
    /// rule, or from a rule that the start rule does not reach, returns its
    /// error as a value.
    pub fn marks(&self, open_path: Vec<String>, focus: Option<usize>) -> Result<Ts<MarksOutcome>, JsError> {
        let outcome = match self.session.marks(&open_path, focus) {
            Ok(marks) => MarksOutcome::Marks { marks },
            Err(error) => MarksOutcome::InvalidOpenPath { error },
        };
        Ok(outcome.into_ts()?)
    }

    /// The trace file of the session, as KDL 2.0 text.
    pub fn save(&self) -> String {
        self.session.save()
    }
}

/// The outcome of the operation Marks.
#[derive(Debug, Serialize, Tsify)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum MarksOutcome {
    /// The marks of the open diagram.
    Marks { marks: Marks },
    /// The open path is not a chain of references from the start rule, or
    /// from a rule that the start rule does not reach.
    InvalidOpenPath { error: MarksError },
}

/// The state on view: the section "The interface of the core" of the tracer
/// spec. The number of readings is the length of `readings`.
#[derive(Debug, Serialize, Tsify)]
#[serde(rename_all = "camelCase")]
pub struct SessionState {
    /// The whole sequence. Step n of the sequence is `steps[n - 1]`.
    pub steps: Vec<Step>,
    pub steps_read: usize,
    /// The number of the rejected step, after a read that failed.
    pub failure_point: Option<usize>,
    /// Whether the sequence can end after the steps read.
    pub can_end: bool,
    /// The next options, in the order of the symbols in the checked grammar.
    pub next_options: Vec<NextOption>,
    /// Whether more readings exist than the list holds.
    pub cap_reached: bool,
    /// The listed readings, up to the reading cap.
    pub readings: Vec<ReadingSummary>,
}

/// One listed reading of the state on view, without its tree.
#[derive(Debug, Serialize, Tsify)]
#[serde(rename_all = "camelCase")]
pub struct ReadingSummary {
    /// The number from 1 of the reading after the step before that this
    /// reading extends, when that reading is listed.
    pub parent: Option<usize>,
    /// Whether the sequence can end after the steps read in this reading.
    pub can_end: bool,
    /// The next options that this reading allows, with their occurrences.
    pub next_options: Vec<NextOption>,
    /// The path of the tree from the root to the last step, with the origins
    /// of each node.
    pub position: Vec<PathNode>,
    /// The follow path: the rules of the rule nodes on the position, without
    /// a last rule for one step, unless it is the only one. After each
    /// operation on the trace, the open path becomes the follow path of the
    /// reading on view: `docs/specs/rule-diagrams.md`, section "Following a
    /// reading".
    pub follow_path: Vec<String>,
}
