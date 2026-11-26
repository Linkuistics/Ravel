//! The tracer and its trace session: `docs/specs/tracer.md`, section "The
//! interface of the core".
//!
//! A trace session holds a grammar without errors, a sequence and the number
//! of steps read. The state on view is the state after the steps read. The
//! session reads one step at a time, so that another program can drive it step
//! by step. Only the symbols of the steps decide acceptance:
//! `docs/adr/symbols-decide-acceptance.md`.
//!
//! The readings and their trees come with the shared forest of
//! `docs/adr/readings-from-a-shared-forest.md`.

use std::{fmt, sync::Arc};

use crate::{
    automaton::{self, Automata},
    checked::{Diagnostic, Severity, Span},
    checked_grammar,
    earley::Earley,
    forest::{self, Forest, ForestTables, Label, Token},
    marks::{self, Element, Marks, MarksError, ViewFacts},
    trace_file,
};

/// The version of the trace file. A change to the form that an older reader
/// cannot read raises the version.
pub const TRACE_FORMAT_VERSION: u32 = 1;

/// The reading cap: the most readings that the tracer lists for one state.
pub const READING_CAP: u32 = 10;

/// A grammar without errors, compiled for the tracer. A clone shares the
/// compiled form, so many sessions can trace one grammar.
#[derive(Debug, Clone)]
pub struct TraceGrammar {
    inner: Arc<Compiled>,
}

#[derive(Debug)]
struct Compiled {
    automata: Automata,
    tables: ForestTables,
    /// The name of each rule, in the order of the text.
    rule_names: Vec<String>,
    grammar_sha256: String,
    /// The facts of the drawing that the marks need.
    facts: ViewFacts,
}

impl TraceGrammar {
    /// Compiles a grammar text. A grammar with an error has no traces, so the
    /// result is then its diagnostics with severity error.
    pub fn new(text: &str) -> Result<Self, Vec<Diagnostic>> {
        let grammar = checked_grammar(text);
        if grammar.has_errors() {
            return Err(grammar.diagnostics.into_iter().filter(|d| d.severity == Severity::Error).collect());
        }
        let automata = automaton::compile(&grammar);
        let tables = ForestTables::new(&automata);
        let rule_names = grammar.rules.iter().map(|r| r.name.clone()).collect();
        let facts = crate::diagram::view_facts(&grammar);
        Ok(Self {
            inner: Arc::new(Compiled { automata, tables, rule_names, grammar_sha256: grammar.grammar_sha256, facts }),
        })
    }

    /// The SHA-256 hash of the grammar text, in lower-case hex.
    pub fn grammar_sha256(&self) -> &str {
        &self.inner.grammar_sha256
    }

    pub(crate) fn automata(&self) -> &Automata {
        &self.inner.automata
    }

    fn marks_grammar(&self) -> marks::Grammar<'_> {
        marks::Grammar { automata: &self.inner.automata, tables: &self.inner.tables, facts: &self.inner.facts }
    }

    /// Checks an open path, and returns its rules.
    fn resolve_open_path(&self, open_path: &[String]) -> Result<Vec<usize>, MarksError> {
        let inner = &self.inner;
        marks::resolve_open_path(&inner.facts.references, inner.automata.start, &inner.rule_names, open_path)
    }

    fn rule_names(&self, rules: &[usize]) -> Vec<String> {
        rules.iter().map(|&r| self.inner.rule_names[r].clone()).collect()
    }

    /// The chains of references from the start rule, with 1 to `max` rules.
    /// The property tests of the marks take their open paths from them.
    #[doc(hidden)]
    pub fn reference_chains(&self, max: usize) -> Vec<Vec<String>> {
        marks::chains(&self.inner.facts.references, self.automata().start, max)
            .iter()
            .map(|chain| self.rule_names(chain))
            .collect()
    }
}

/// One step of a sequence: a symbol and its attributes.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize, tsify::Tsify), serde(rename_all = "camelCase"))]
pub struct Step {
    pub symbol: String,
    /// The attributes, in the order of the trace file. No two have one key.
    pub attributes: Vec<Attribute>,
}

impl Step {
    /// A step without attributes.
    pub fn new(symbol: impl Into<String>) -> Self {
        Self { symbol: symbol.into(), attributes: Vec::new() }
    }

    /// Checks the rules of the section "Steps": a symbol that is not empty and
    /// holds no line break, and no two attributes with one key.
    pub fn check(&self) -> Result<(), StepError> {
        if self.symbol.is_empty() {
            return Err(StepError { code: TraceCode::EmptySymbol, message: "The symbol of a step is empty.".into() });
        }
        if self.symbol.chars().any(trace_file::is_line_break) {
            return Err(StepError {
                code: TraceCode::LineBreakInSymbol,
                message: format!("The symbol {:?} holds a line break.", self.symbol),
            });
        }
        for (i, attribute) in self.attributes.iter().enumerate() {
            if self.attributes[..i].iter().any(|a| a.key == attribute.key) {
                return Err(StepError {
                    code: TraceCode::DuplicateAttribute,
                    message: format!("The step has two attributes with the key {:?}.", attribute.key),
                });
            }
        }
        Ok(())
    }
}

/// A step that breaks the rules of the section "Steps".
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize, tsify::Tsify), serde(rename_all = "camelCase"))]
pub struct StepError {
    pub code: TraceCode,
    pub message: String,
}

impl fmt::Display for StepError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for StepError {}

/// An attribute: one property of a step in the trace file. The key, the value
/// and the type annotation stay as the writer wrote them. The core can write a
/// number in another notation, such as `16` for `0x10`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize, tsify::Tsify), serde(rename_all = "camelCase"))]
pub struct Attribute {
    pub key: String,
    pub value: AttributeValue,
    /// The type annotation, such as `url` in `(url)"https://example.org"`.
    pub annotation: Option<String>,
}

/// The value of an attribute: a KDL 2.0 value.
///
/// At the WebAssembly boundary, a number crosses as its decimal text, so no
/// value loses precision: `docs/specs/tracer.md`, section "The WebAssembly
/// boundary". The module `boundary` writes that form.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "wasm",
    derive(tsify::Tsify),
    tsify(type = "{ type: \"string\"; value: string } | { type: \"integer\"; value: string } \
        | { type: \"float\"; value: string } | { type: \"bool\"; value: boolean } | { type: \"null\" }")
)]
pub enum AttributeValue {
    String(String),
    Integer(i128),
    Float(Float),
    Bool(bool),
    Null,
}

/// A KDL 2.0 float. Two floats are equal when their bits are equal, so `#nan`
/// equals itself and `-0.0` differs from `0.0`, as their KDL text does.
#[derive(Debug, Clone, Copy)]
pub struct Float(pub f64);

impl PartialEq for Float {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == other.0.to_bits()
    }
}

impl Eq for Float {}

/// A next option: a symbol that can come next, with the symbol occurrences
/// that offer it.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize, tsify::Tsify), serde(rename_all = "camelCase"))]
pub struct NextOption {
    pub symbol: String,
    /// The spans of the occurrences, in the order of the text.
    pub occurrences: Vec<Span>,
}

/// The state on view, without readings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceState {
    /// The whole sequence. Step n of the sequence is `steps[n - 1]`.
    pub steps: Vec<Step>,
    pub steps_read: usize,
    /// The number of the rejected step, after a read that failed.
    pub failure_point: Option<usize>,
    /// Whether the sequence can end after the steps read.
    pub can_end: bool,
    /// The next options, in the order of the symbols in the checked grammar.
    pub next_options: Vec<NextOption>,
}

/// The result of a read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize, tsify::Tsify), serde(tag = "type", rename_all = "kebab-case"))]
pub enum ReadOutcome {
    /// The tracer accepted the step.
    Accepted,
    /// No item accepts the step. The state stays the state before it, and
    /// `step` is the failure point.
    Rejected { step: usize },
    /// The tracer read every step of the sequence.
    EndOfSequence,
}

/// The stable code of a diagnostic of a trace file. Tests and other programs
/// match on the code, never on the message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize, tsify::Tsify), serde(rename_all = "kebab-case"))]
pub enum TraceCode {
    /// Text that is not KDL 2.0.
    NotKdl,
    /// A document whose root is not one `trace` node.
    NotATrace,
    /// A format version that the core does not know.
    UnknownFormatVersion,
    /// A node that the format does not know at its place.
    UnknownNode,
    /// A property that the format does not know on its node.
    UnknownProperty,
    /// A property that a node needs and lacks.
    MissingProperty,
    /// A child node that a node needs and lacks, such as the `sequence`.
    MissingNode,
    /// A property with a value of the wrong type.
    WrongType,
    /// A node that takes no argument with an argument.
    UnexpectedArgument,
    /// One property twice on a node other than a step.
    DuplicateProperty,
    /// A node out of order, such as a second `sequence`.
    NodeOutOfOrder,
    /// A step without exactly one argument, or an argument that is not a string.
    StepArgument,
    /// A step whose symbol is empty.
    EmptySymbol,
    /// A step whose symbol holds a line break.
    LineBreakInSymbol,
    /// A step with children.
    StepChildren,
    /// Two attributes with one key on one step.
    DuplicateAttribute,
    /// A derived node that breaks the format in another way.
    MalformedDerivedNode,
    /// A grammar hash that differs from the hash of the grammar text.
    GrammarHashDiffers,
}

impl TraceCode {
    /// Every code.
    pub const ALL: &'static [TraceCode] = &[
        Self::NotKdl,
        Self::NotATrace,
        Self::UnknownFormatVersion,
        Self::UnknownNode,
        Self::UnknownProperty,
        Self::MissingProperty,
        Self::MissingNode,
        Self::WrongType,
        Self::UnexpectedArgument,
        Self::DuplicateProperty,
        Self::NodeOutOfOrder,
        Self::StepArgument,
        Self::EmptySymbol,
        Self::LineBreakInSymbol,
        Self::StepChildren,
        Self::DuplicateAttribute,
        Self::MalformedDerivedNode,
        Self::GrammarHashDiffers,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::NotKdl => "not-kdl",
            Self::NotATrace => "not-a-trace",
            Self::UnknownFormatVersion => "unknown-format-version",
            Self::UnknownNode => "unknown-node",
            Self::UnknownProperty => "unknown-property",
            Self::MissingProperty => "missing-property",
            Self::MissingNode => "missing-node",
            Self::WrongType => "wrong-type",
            Self::UnexpectedArgument => "unexpected-argument",
            Self::DuplicateProperty => "duplicate-property",
            Self::NodeOutOfOrder => "node-out-of-order",
            Self::StepArgument => "step-argument",
            Self::EmptySymbol => "empty-symbol",
            Self::LineBreakInSymbol => "line-break-in-symbol",
            Self::StepChildren => "step-children",
            Self::DuplicateAttribute => "duplicate-attribute",
            Self::MalformedDerivedNode => "malformed-derived-node",
            Self::GrammarHashDiffers => "grammar-hash-differs",
        }
    }

    pub fn parse(text: &str) -> Option<Self> {
        Self::ALL.iter().copied().find(|c| c.as_str() == text)
    }

    /// The severity of the diagnostic. Only a differing hash is a warning.
    pub fn severity(self) -> Severity {
        if self == Self::GrammarHashDiffers { Severity::Warning } else { Severity::Error }
    }
}

impl fmt::Display for TraceCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A diagnostic of a trace file. The span is a byte range in the text of the
/// trace file.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize, tsify::Tsify), serde(rename_all = "camelCase"))]
pub struct TraceDiagnostic {
    pub severity: Severity,
    pub code: TraceCode,
    pub message: String,
    pub span: Span,
}

impl TraceDiagnostic {
    pub(crate) fn new(code: TraceCode, span: Span, message: impl Into<String>) -> Self {
        Self { severity: code.severity(), code, message: message.into(), span }
    }
}

/// Why a session did not open.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpenError {
    /// The grammar has errors. These are its diagnostics with severity error.
    Grammar(Vec<Diagnostic>),
    /// The trace file has errors. These are all its diagnostics.
    TraceFile(Vec<TraceDiagnostic>),
}

/// A session that opened, with the warnings of its trace file.
#[derive(Debug, Clone)]
pub struct Opened {
    pub session: Session,
    pub warnings: Vec<TraceDiagnostic>,
}

/// A trace session.
#[derive(Debug, Clone)]
pub struct Session {
    grammar: TraceGrammar,
    steps: Vec<Step>,
    earley: Earley,
    failure_point: Option<usize>,
}

impl Session {
    /// Opens a session on a grammar text, and optionally on the text of a
    /// trace file. With a trace file, the session reads as many steps as it can.
    pub fn open(grammar_text: &str, trace_text: Option<&str>) -> Result<Opened, OpenError> {
        let grammar = TraceGrammar::new(grammar_text).map_err(OpenError::Grammar)?;
        match trace_text {
            None => Ok(Opened { session: Self::new(&grammar), warnings: Vec::new() }),
            Some(text) => Self::from_trace_file(&grammar, text),
        }
    }

    /// A session with an empty sequence.
    pub fn new(grammar: &TraceGrammar) -> Self {
        let earley = Earley::new(grammar.automata());
        Self { grammar: grammar.clone(), steps: Vec::new(), earley, failure_point: None }
    }

    /// A session with a sequence, which reads as many steps as it can. Each
    /// step must keep the rules of the section "Steps", as for Insert. It
    /// returns the error of the first step that breaks them.
    pub fn with_steps(grammar: &TraceGrammar, steps: Vec<Step>) -> Result<Self, StepError> {
        for (i, step) in steps.iter().enumerate() {
            step.check().map_err(|e| StepError { code: e.code, message: format!("Step {}: {}", i + 1, e.message) })?;
        }
        Ok(Self::with_checked_steps(grammar, steps))
    }

    /// A session with a sequence whose steps keep the rules of the section
    /// "Steps".
    fn with_checked_steps(grammar: &TraceGrammar, steps: Vec<Step>) -> Self {
        let mut session = Self::new(grammar);
        session.steps = steps;
        session.read_all();
        session
    }

    /// A session on the sequence of a trace file, which reads as many steps as
    /// it can. The core never takes a derived node from the file.
    pub fn from_trace_file(grammar: &TraceGrammar, text: &str) -> Result<Opened, OpenError> {
        let file = trace_file::read(text);
        if file.diagnostics.iter().any(|d| d.severity == Severity::Error) {
            return Err(OpenError::TraceFile(file.diagnostics));
        }
        let mut warnings = file.diagnostics;
        if let Some((hash, span)) = &file.grammar_sha256
            && hash != grammar.grammar_sha256()
        {
            warnings.push(TraceDiagnostic::new(
                TraceCode::GrammarHashDiffers,
                *span,
                format!(
                    "The trace file names the grammar hash {hash}, but the grammar text has the hash {}. \
                     The core traces the sequence against this grammar text, and it derives every other node again.",
                    grammar.grammar_sha256()
                ),
            ));
        }
        // The reader checks each step by the same rules as `Step::check`.
        Ok(Opened { session: Self::with_checked_steps(grammar, file.steps), warnings })
    }

    pub fn grammar(&self) -> &TraceGrammar {
        &self.grammar
    }

    pub fn steps(&self) -> &[Step] {
        &self.steps
    }

    pub fn steps_read(&self) -> usize {
        self.earley.steps_read()
    }

    pub fn failure_point(&self) -> Option<usize> {
        self.failure_point
    }

    /// Reads the next step of the sequence.
    pub fn read(&mut self) -> ReadOutcome {
        let k = self.steps_read();
        let Some(step) = self.steps.get(k) else {
            return ReadOutcome::EndOfSequence;
        };
        if self.earley.read(self.grammar.automata(), &step.symbol) {
            self.failure_point = None;
            ReadOutcome::Accepted
        } else {
            self.failure_point = Some(k + 1);
            ReadOutcome::Rejected { step: k + 1 }
        }
    }

    /// Reads steps until the end of the sequence or the first rejected step.
    pub fn read_all(&mut self) -> ReadOutcome {
        loop {
            match self.read() {
                ReadOutcome::Accepted => {}
                outcome => return outcome,
            }
        }
    }

    /// Drops the last step read. The step stays in the sequence. A step back
    /// clears the failure point, because the next step is then a step that
    /// the tracer accepted. It returns false before the first step.
    pub fn back(&mut self) -> bool {
        let dropped = self.earley.back();
        if dropped {
            self.failure_point = None;
        }
        dropped
    }

    /// Inserts a step right after the steps read, and does not read it. At the
    /// end of the sequence, this appends the step. An insert clears the
    /// failure point, because the tracer has not read the steps after it.
    pub fn insert(&mut self, step: Step) -> Result<(), StepError> {
        step.check()?;
        self.steps.insert(self.steps_read(), step);
        self.failure_point = None;
        Ok(())
    }

    /// Whether the sequence can end after k steps, for k up to the steps read.
    pub fn can_end_after(&self, k: usize) -> bool {
        self.earley.can_end(self.grammar.automata(), k)
    }

    /// The next options after k steps, for k up to the steps read.
    pub fn next_options_after(&self, k: usize) -> Vec<NextOption> {
        let automata = self.grammar.automata();
        self.earley
            .next_options(automata, k)
            .into_iter()
            .map(|(symbol, occurrences)| NextOption { symbol: automata.symbols[symbol].clone(), occurrences })
            .collect()
    }

    /// The state on view.
    pub fn state(&self) -> TraceState {
        let k = self.steps_read();
        TraceState {
            steps: self.steps.clone(),
            steps_read: k,
            failure_point: self.failure_point,
            can_end: self.can_end_after(k),
            next_options: self.next_options_after(k),
        }
    }

    /// The trace file of the session, as KDL 2.0 text.
    pub fn save(&self) -> String {
        trace_file::write(self)
    }
}

pub use crate::forest::LoopKind;

impl LoopKind {
    /// The name of the kind in a trace file.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Repetition => "repetition",
            Self::TailReferences => "tail-references",
            Self::HeadReferences => "head-references",
        }
    }
}

/// The kind and the label of a tree node.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize, tsify::Tsify), serde(tag = "type", rename_all = "kebab-case"))]
pub enum TreeKind {
    Rule {
        name: String,
    },
    /// A loop node. A loop through rules names its pass rule.
    Loop {
        kind: LoopKind,
        #[cfg_attr(feature = "wasm", serde(rename = "passRule"))]
        pass_rule: Option<String>,
    },
    /// A pass, by its number from 1.
    Pass {
        number: usize,
    },
    /// A step, by its number from 1, with its symbol.
    Step {
        number: usize,
        symbol: String,
    },
}

/// A node of the tree of a reading.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize, tsify::Tsify), serde(rename_all = "camelCase"))]
pub struct TreeNode {
    pub kind: TreeKind,
    /// The spans of the grammar elements that the node came from, in the
    /// order of the text.
    pub origins: Vec<Span>,
    /// The children, in the order of their steps.
    pub children: Vec<TreeNode>,
}

/// A node on the position of a reading: its kind and its origins.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize, tsify::Tsify), serde(rename_all = "camelCase"))]
pub struct PathNode {
    pub kind: TreeKind,
    pub origins: Vec<Span>,
}

/// One listed reading of a state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reading {
    /// The number from 1 of the reading after the step before that this
    /// reading extends, when that reading is listed.
    pub parent: Option<usize>,
    /// Whether the sequence can end after the steps read in this reading.
    pub can_end: bool,
    /// The next options that this reading allows, with their occurrences in
    /// this reading.
    pub next_options: Vec<NextOption>,
    /// The path of the tree from the root to the last step, with the origins
    /// of each node. Before the first step, it holds the root alone.
    pub position: Vec<PathNode>,
    /// The follow path: the rules of the rule nodes on the position, without
    /// a last rule for one step: `docs/specs/rule-diagrams.md`, section
    /// "Following a reading".
    pub follow_path: Vec<String>,
    /// The tree of the reading.
    pub tree: TreeNode,
}

/// The readings of a state, up to the reading cap.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Readings {
    pub readings: Vec<Reading>,
    /// Whether more readings exist than the list holds.
    pub cap_reached: bool,
}

impl Session {
    fn forest(&self) -> Forest<'_> {
        self.forest_with_cap(READING_CAP as usize + 1)
    }

    fn forest_with_cap(&self, list_cap: usize) -> Forest<'_> {
        let automata = self.grammar.automata();
        let steps = self.steps[..self.steps_read()]
            .iter()
            .map(|s| automata.symbol(&s.symbol).expect("a step that the tracer read"))
            .collect();
        Forest::new(automata, &self.grammar.inner.tables, &self.earley, steps, list_cap)
    }

    /// The number of readings after k steps, for k up to the steps read, and
    /// whether the tracer reached the reading cap.
    pub fn reading_count_after(&self, k: usize) -> (usize, bool) {
        let found = self.forest().readings(k).len();
        (found.min(READING_CAP as usize), found > READING_CAP as usize)
    }

    /// The readings after k steps, for k up to the steps read.
    pub fn readings_after(&self, k: usize) -> Readings {
        let mut forest = self.forest();
        self.readings_with(&mut forest, k)
    }

    /// The counts of readings after each count of steps read, and the
    /// readings of the state on view, from one evaluation of the forest.
    /// The readings after k steps from lists that keep the first `list_cap`
    /// projections of each split, not one more than the reading cap. The
    /// property tests compare them with the readings of the tracer.
    #[doc(hidden)]
    pub fn readings_with_list_cap(&self, k: usize, list_cap: usize) -> Readings {
        let mut forest = self.forest_with_cap(list_cap);
        self.readings_with(&mut forest, k)
    }

    /// The readings after each count of steps read, from 0 to the steps read,
    /// from one evaluation of the forest.
    pub fn readings_of_each_prefix(&self) -> Vec<Readings> {
        let mut forest = self.forest();
        (0..=self.steps_read()).map(|k| self.readings_with(&mut forest, k)).collect()
    }

    pub(crate) fn all_readings(&self) -> (Vec<(usize, bool)>, Readings) {
        let mut forest = self.forest();
        let n = self.steps_read();
        let counts = (0..=n)
            .map(|k| {
                let found = forest.readings(k).len();
                (found.min(READING_CAP as usize), found > READING_CAP as usize)
            })
            .collect();
        let readings = self.readings_with(&mut forest, n);
        (counts, readings)
    }

    fn readings_with(&self, forest: &mut Forest<'_>, k: usize) -> Readings {
        let cap = READING_CAP as usize;
        let mut found = forest.readings(k);
        let cap_reached = found.len() > cap;
        found.truncate(cap);
        let parents = if k > 0 {
            let mut before = forest.readings(k - 1);
            before.truncate(cap);
            before
        } else {
            Vec::new()
        };
        let automata = self.grammar.automata();
        let readings = found
            .iter()
            .map(|proj| {
                let stripped = forest::without_last_step(&proj.tokens, k, automata.start);
                let parent = (k > 0)
                    .then(|| parents.iter().position(|p| forest::same_tree(&p.tokens, &stripped)).map(|i| i + 1))
                    .flatten();
                let next_options = forest
                    .next_options(proj)
                    .into_iter()
                    .map(|(symbol, occurrences)| NextOption { symbol: automata.symbols[symbol].clone(), occurrences })
                    .collect();
                let follow_path = marks::follow_path(&proj.tokens, &self.grammar.inner.facts.one_step);
                Reading {
                    parent,
                    can_end: proj.can_end,
                    next_options,
                    position: self.position(&proj.tokens),
                    follow_path: self.grammar.rule_names(&follow_path),
                    tree: self.tree(&proj.tokens),
                }
            })
            .collect();
        Readings { readings, cap_reached }
    }

    fn kind(&self, token: &Token, pass: usize) -> TreeKind {
        let rules = &self.grammar.inner.rule_names;
        match token.label {
            Label::Rule(r) => TreeKind::Rule { name: rules[r].clone() },
            Label::Loop(kind, rule) => TreeKind::Loop { kind, pass_rule: rule.map(|r| rules[r].clone()) },
            Label::Pass => TreeKind::Pass { number: pass },
            Label::Step(n) => TreeKind::Step { number: n, symbol: self.steps[n - 1].symbol.clone() },
        }
    }

    /// The tree of a list of tokens in pre-order.
    fn tree(&self, tokens: &[Token]) -> TreeNode {
        fn build(session: &Session, tokens: &[Token], at: &mut usize, pass: usize) -> TreeNode {
            let token = &tokens[*at];
            *at += 1;
            let mut children = Vec::new();
            let mut passes = 0;
            while *at < tokens.len() && tokens[*at].depth > token.depth {
                if tokens[*at].label == Label::Pass {
                    passes += 1;
                }
                children.push(build(session, tokens, at, passes));
            }
            TreeNode { kind: session.kind(token, pass), origins: token.origins.as_slice().to_vec(), children }
        }
        build(self, tokens, &mut 0, 0)
    }

    /// The path from the root to the last step, or the root alone.
    fn position(&self, tokens: &[Token]) -> Vec<PathNode> {
        let last = tokens.iter().rposition(|t| matches!(t.label, Label::Step(_))).unwrap_or(0);
        let mut path = Vec::new();
        let mut depth = tokens[last].depth + 1;
        for i in (0..=last).rev() {
            if tokens[i].depth < depth {
                depth = tokens[i].depth;
                let pass = if tokens[i].label == Label::Pass {
                    let parent = tokens[..i].iter().rposition(|t| t.depth < depth).unwrap_or(0);
                    tokens[parent + 1..=i].iter().filter(|t| t.depth == depth).count()
                } else {
                    0
                };
                path.push(PathNode {
                    kind: self.kind(&tokens[i], pass),
                    origins: tokens[i].origins.as_slice().to_vec(),
                });
            }
        }
        path.reverse();
        path
    }
}

impl Session {
    /// The marks of the open diagram of `open_path` for the state on view:
    /// `docs/specs/rule-diagrams.md`, section "The interface of the core".
    /// `focus` is the number of the step in focus. An open path that is not a
    /// chain of references from the start rule, or from a rule that the start
    /// rule does not reach, is an error.
    pub fn marks(&self, open_path: &[String], focus: Option<usize>) -> Result<Marks, MarksError> {
        let automata = self.grammar.automata();
        let rules = self.grammar.resolve_open_path(open_path)?;
        let n = self.steps_read();
        let cap = READING_CAP as usize;
        let mut forest = self.forest();
        let mut listed = forest.readings(n);
        listed.truncate(cap);
        let mut allowed: Vec<String> = Vec::new();
        for proj in &listed {
            for (symbol, _) in forest.next_options(proj) {
                allowed.push(automata.symbols[symbol].clone());
            }
        }
        let unlisted_options =
            self.next_options_after(n).into_iter().map(|o| o.symbol).filter(|s| !allowed.contains(s)).collect();
        // No reading stands in a diagram whose open path starts at a rule that
        // the start rule does not reach, and no next position enters it.
        let readings = if rules[0] == automata.start {
            marks::reading_marks(&self.grammar.marks_grammar(), &mut forest, n, &listed, &rules, focus)
        } else {
            Vec::new()
        };
        Ok(Marks { open_path: self.grammar.rule_names(&rules), readings, unlisted_options })
    }

    /// The marks as KDL 2.0 text, for the tests only. The trace file does not
    /// hold them, because they describe a view of a run, not the run.
    pub fn marks_kdl(&self, marks: &Marks) -> String {
        marks::write(marks, self.grammar.automata(), &self.grammar.inner.rule_names)
    }

    /// The successors of each listed reading through `symbol`, from lists
    /// without a cap: the readings after the step whose parent is that
    /// reading. It returns `None` when the step gives `max` readings or more.
    #[doc(hidden)]
    pub fn uncapped_step(&self, symbol: &str, max: usize) -> Option<UncappedStep> {
        let n = self.steps_read();
        let automata = self.grammar.automata();
        let mut listed = self.forest().readings(n);
        listed.truncate(READING_CAP as usize);
        let readings: Vec<Vec<Token>> = listed.into_iter().map(|p| p.tokens).collect();
        let mut steps = self.steps[..n].to_vec();
        steps.push(Step::new(symbol));
        let next = Session::with_checked_steps(&self.grammar, steps);
        if next.steps_read() != n + 1 {
            let successors = vec![Vec::new(); readings.len()];
            return Some(UncappedStep { n, readings, successors });
        }
        let after = next.forest_with_cap(max).readings(n + 1);
        if after.len() >= max {
            return None;
        }
        let successors = readings
            .iter()
            .map(|reading| {
                after
                    .iter()
                    .filter(|p| {
                        forest::same_tree(&forest::without_last_step(&p.tokens, n + 1, automata.start), reading)
                    })
                    .cloned()
                    .collect()
            })
            .collect();
        Some(UncappedStep { n, readings, successors })
    }

    /// The marks of the successors of one listed reading from an uncapped
    /// step, by the four rules, with the end event as an element. The open
    /// path must start at the start rule.
    #[doc(hidden)]
    pub fn uncapped_marks(&self, step: &UncappedStep, reading: usize, open_path: &[String]) -> Vec<Element> {
        let rules = self.grammar.resolve_open_path(open_path).expect("a valid open path");
        let g = self.grammar.marks_grammar();
        marks::successor_elements(&g, step.n, &step.readings[reading], &step.successors[reading], &rules)
    }
}

/// The successors of each listed reading through one symbol, from an uncapped
/// step. The property tests of the marks compare with them.
#[doc(hidden)]
pub struct UncappedStep {
    n: usize,
    readings: Vec<Vec<Token>>,
    successors: Vec<Vec<forest::Proj>>,
}

impl UncappedStep {
    /// The count of successors of each listed reading.
    pub fn successor_counts(&self) -> Vec<usize> {
        self.successors.iter().map(Vec::len).collect()
    }
}
