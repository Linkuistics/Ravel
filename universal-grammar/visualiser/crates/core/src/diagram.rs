//! The layout of the rule diagrams: `docs/specs/rule-diagrams.md`, sections
//! "The diagram of a rule", "The drawing of each construct", "Labels and the
//! font", "The layout" and "The diagram KDL".
//!
//! The layout is a fold over the expression of each rule, as in the railroad
//! library of Tab Atkins. First each element measures its children. Then it
//! places them in the space that it measured. An element has a width, an
//! extent above the main line and an extent below it. Its flow enters at the
//! left end of the main line and leaves at the right end.
//!
//! The module `diagram_file` writes the result as the diagram KDL.

use std::collections::{HashMap, HashSet};

use crate::{
    automaton::{self, PositionKind},
    cells_table,
    checked::{CheckedGrammar, Diagnostic, Expr, Rule, Severity, Span},
    diagram_file,
    kdl_map::MappingError,
    loops::{Links, LoopGraphs},
};

/// The version of the diagram KDL. A change to the form that an older reader
/// cannot read raises the version.
pub const DIAGRAM_FORMAT_VERSION: u32 = 1;

/// The size of the label font, in px.
pub const FONT_SIZE: u32 = 13;
/// The width of one cell of the label font, in px: 0.6 em at 13 px.
pub const CELL_WIDTH: f64 = 7.8;
/// The height of one line of a label, in px.
pub const LINE_HEIGHT: u32 = 16;
/// The version of Unicode by which the core counts the cells of a label.
pub use crate::cells_table::UNICODE_VERSION;

// The constants of the layout, in px, from the table of the spec. The cell
// width 7.8 px is 39/5, so each width is a whole number of fifths.
const CELL_FIFTHS: u32 = 39;
const MAX_CELLS: u32 = 18;
const PAD_X: u32 = 10;
const PAD_Y: u32 = 8;
const MIN_W: u32 = 64;
const MIN_H: u32 = 36;
const BAND: u32 = 22;
const DRILL: u32 = 12;
const GATEWAY: u32 = 24;
const EVENT: u32 = 24;
const FLOW: u32 = 24;
const PAIR: u32 = 10;
const SKIP: u32 = 12;
const BACK: u32 = 18;
const BELOW_MARKER: u32 = 9;
const BELOW_PLAIN: u32 = 2;
const ALT_GAP: u32 = 14;
const LANE: u32 = 24;
const LANE_LABEL: u32 = 8;
const MARGIN: u32 = 16;
const HALF: u32 = GATEWAY / 2;

/// The diagrams of a grammar without errors, one for each rule.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize, tsify::Tsify), serde(rename_all = "camelCase"))]
pub struct Diagrams {
    pub format_version: u32,
    /// The SHA-256 hash of the grammar text, in lower-case hex.
    pub grammar_sha256: String,
    pub font_size: u32,
    pub cell_width: f64,
    pub line_height: u32,
    /// One diagram for each rule, in the order of the text.
    pub diagrams: Vec<Diagram>,
}

/// The diagram of one rule.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize, tsify::Tsify), serde(rename_all = "camelCase"))]
pub struct Diagram {
    pub rule: String,
    pub width: u32,
    pub height: u32,
    /// The shapes, in the order in which the layout places them.
    pub shapes: Vec<Shape>,
    /// The flows, in the order in which the layout places them.
    pub flows: Vec<Flow>,
}

/// A shape of a diagram. The point `x`, `y` is its upper left corner.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "wasm",
    derive(serde::Serialize, tsify::Tsify),
    serde(tag = "type", rename_all = "kebab-case", rename_all_fields = "camelCase")
)]
pub enum Shape {
    /// The thin circle at the left end of the main line.
    StartEvent { x: u32, y: u32, size: u32 },
    /// The thick circle at the right end of the main line.
    EndEvent { x: u32, y: u32, size: u32 },
    /// The box of a symbol occurrence. The span is that of the occurrence.
    Task { span: Span, symbol: String, x: u32, y: u32, width: u32, height: u32, label: Vec<String> },
    /// The box of a reference. The span is that of the reference.
    Reference {
        span: Span,
        rule: String,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        /// Whether the rule is for one step. Such a box has no drill-down
        /// marker.
        one_step: bool,
        /// The kind of loop through rules that the reference links, if any.
        loop_link: Option<LoopLink>,
        label: Vec<String>,
    },
    /// An exclusive gateway. The span is that of its construct, or that of
    /// the rule name for a `back-edge-join`.
    Gateway { role: GatewayRole, span: Span, x: u32, y: u32, size: u32 },
}

/// The kind of loop through rules that a reference links.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize, tsify::Tsify), serde(rename_all = "kebab-case"))]
pub enum LoopLink {
    TailReferences,
    HeadReferences,
}

impl LoopLink {
    pub const ALL: [Self; 2] = [Self::TailReferences, Self::HeadReferences];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::TailReferences => "tail-references",
            Self::HeadReferences => "head-references",
        }
    }

    pub fn parse(text: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|v| v.as_str() == text)
    }
}

/// The role of a gateway in its construct.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize, tsify::Tsify), serde(rename_all = "kebab-case"))]
pub enum GatewayRole {
    ChoiceSplit,
    ChoiceJoin,
    OptionSplit,
    OptionJoin,
    SkipSplit,
    LoopJoin,
    LoopSplit,
    SkipJoin,
    BackEdgeJoin,
}

impl GatewayRole {
    pub const ALL: [Self; 9] = [
        Self::ChoiceSplit,
        Self::ChoiceJoin,
        Self::OptionSplit,
        Self::OptionJoin,
        Self::SkipSplit,
        Self::LoopJoin,
        Self::LoopSplit,
        Self::SkipJoin,
        Self::BackEdgeJoin,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::ChoiceSplit => "choice-split",
            Self::ChoiceJoin => "choice-join",
            Self::OptionSplit => "option-split",
            Self::OptionJoin => "option-join",
            Self::SkipSplit => "skip-split",
            Self::LoopJoin => "loop-join",
            Self::LoopSplit => "loop-split",
            Self::SkipJoin => "skip-join",
            Self::BackEdgeJoin => "back-edge-join",
        }
    }

    pub fn parse(text: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|v| v.as_str() == text)
    }
}

/// A sequence flow: a line through its points, with an arrowhead at the last
/// point. Its segments are horizontal or vertical.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize, tsify::Tsify), serde(rename_all = "camelCase"))]
pub struct Flow {
    pub role: FlowRole,
    /// For a skip flow or a back flow, the span of its optional part or
    /// repetition. For a back edge, the span of its reference.
    pub span: Option<Span>,
    /// The loop marker of a back flow, or the label of a back edge.
    pub label: Option<FlowLabel>,
    /// The corners of the flow, from its source to its target.
    pub points: Vec<Point>,
}

/// The role of a flow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize, tsify::Tsify), serde(rename_all = "kebab-case"))]
pub enum FlowRole {
    Sequence,
    Skip,
    Back,
    BackEdge,
}

impl FlowRole {
    pub const ALL: [Self; 4] = [Self::Sequence, Self::Skip, Self::Back, Self::BackEdge];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Sequence => "sequence",
            Self::Skip => "skip",
            Self::Back => "back",
            Self::BackEdge => "back-edge",
        }
    }

    pub fn parse(text: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|v| v.as_str() == text)
    }
}

/// The text of a label on a flow, and its centre.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize, tsify::Tsify), serde(rename_all = "camelCase"))]
pub struct FlowLabel {
    pub text: String,
    pub x: u32,
    pub y: u32,
}

/// A point of a diagram, in whole pixels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize, tsify::Tsify), serde(rename_all = "camelCase"))]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Diagrams {
    /// Writes the diagrams as the diagram KDL, a KDL 2.0 document.
    pub fn to_kdl(&self) -> String {
        diagram_file::write(self)
    }

    /// Reads the diagram KDL. The reader is strict, as the reader of the
    /// checked grammar is.
    pub fn from_kdl(text: &str) -> Result<Self, MappingError> {
        diagram_file::read(text)
    }
}

/// Lays out the diagram of each rule of a grammar text. A grammar with an
/// error has no diagrams, so the result is then its diagnostics with severity
/// error.
pub fn diagrams(text: &str) -> Result<Diagrams, Vec<Diagnostic>> {
    let grammar = crate::checked_grammar(text);
    if grammar.has_errors() {
        return Err(grammar.diagnostics.into_iter().filter(|d| d.severity == Severity::Error).collect());
    }
    Ok(lay_out(&grammar))
}

/// The facts of the drawing that the marks of a trace need: the references
/// that draw as back edges, and whether each rule is for one step, in the
/// order of the text.
pub(crate) fn view_facts(grammar: &CheckedGrammar) -> crate::marks::ViewFacts {
    let facts = Facts::new(grammar);
    let one_step = grammar.rules.iter().map(|rule| facts.one_step(&rule.name)).collect();
    let back_edges = grammar
        .rules
        .iter()
        .filter_map(|rule| lay_out_rule(&facts, rule))
        .flat_map(|diagram| diagram.flows)
        .filter(|flow| flow.role == FlowRole::BackEdge)
        .filter_map(|flow| flow.span)
        .collect();
    crate::marks::ViewFacts { back_edges, one_step, references: crate::marks::rule_references(grammar) }
}

/// Lays out the diagram of each rule of a checked grammar without errors.
fn lay_out(grammar: &CheckedGrammar) -> Diagrams {
    let facts = Facts::new(grammar);
    Diagrams {
        format_version: DIAGRAM_FORMAT_VERSION,
        grammar_sha256: grammar.grammar_sha256.clone(),
        font_size: FONT_SIZE,
        cell_width: CELL_WIDTH,
        line_height: LINE_HEIGHT,
        diagrams: grammar.rules.iter().filter_map(|rule| lay_out_rule(&facts, rule)).collect(),
    }
}

// Labels.

/// The count of cells of a text in the label font. A combining mark takes no
/// cell, and a wide East Asian character takes two cells.
pub fn cells(text: &str) -> u32 {
    text.chars().map(char_cells).sum()
}

fn char_cells(c: char) -> u32 {
    let c = c as u32;
    let table = cells_table::RANGES;
    match table.binary_search_by(|&(lo, hi, _)| {
        if hi < c {
            std::cmp::Ordering::Less
        } else if lo > c {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    }) {
        Ok(i) => u32::from(table[i].2),
        Err(_) => 1,
    }
}

/// The places where a label can break. A piece ends after a space, a `_` or
/// a `-` when the next character is not a space and the piece holds a
/// character that is not a space. So no break falls inside a run of spaces,
/// inside the spaces at the start of the label or at its end. Each piece
/// holds a character that is not a space.
fn pieces(label: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut start = 0;
    let mut chars = label.char_indices().peekable();
    while let Some((i, c)) = chars.next() {
        let end = i + c.len_utf8();
        let next_is_text = chars.peek().is_some_and(|&(_, n)| n != ' ');
        if matches!(c, ' ' | '_' | '-') && next_is_text && label[start..end].contains(|p| p != ' ') {
            out.push(&label[start..end]);
            start = end;
        }
    }
    out.push(&label[start..]);
    out
}

/// A line without the spaces at its end: a break after spaces drops them all.
fn trimmed(line: &str) -> &str {
    line.trim_end_matches(' ')
}

/// A line as it is drawn. A line before a break drops the spaces at its end.
/// The last line of the label keeps them.
fn drawn(line: &str, last: bool) -> &str {
    if last { line } else { trimmed(line) }
}

/// Fills each line with as many pieces as fit in `limit` cells. A piece that
/// does not fit on an empty line takes the line alone.
fn fill(pieces: &[&str], limit: u32) -> Vec<String> {
    let mut lines = Vec::new();
    let mut line = String::new();
    let last = pieces.len() - 1;
    for (k, piece) in pieces.iter().enumerate() {
        let longer = format!("{line}{piece}");
        if !line.is_empty() && cells(drawn(&longer, k == last)) > limit {
            lines.push(trimmed(&line).to_string());
            line = (*piece).to_string();
        } else {
            line = longer;
        }
    }
    lines.push(line);
    lines
}

/// The lines of a label. A label takes the fewest lines that the limit of 18
/// cells allows. With that count, its box is as narrow as possible. Among the
/// sets of breaks with that count and that width, each line holds as much as
/// it can, from the first line.
///
/// Filling each line greedily gives the fewest lines for a limit. So the
/// narrowest limit that keeps the count of lines gives the narrowest box, and
/// filling at that limit makes each line hold as much as it can.
pub fn label_lines(label: &str) -> Vec<String> {
    let pieces = pieces(label);
    let lines = fill(&pieces, MAX_CELLS);
    if lines.len() == 1 {
        return lines;
    }
    let last = pieces.len() - 1;
    let widest_piece = pieces.iter().enumerate().map(|(k, p)| cells(drawn(p, k == last))).max().unwrap_or(0);
    for limit in widest_piece..MAX_CELLS {
        let trial = fill(&pieces, limit);
        if trial.len() == lines.len() {
            return trial;
        }
    }
    lines
}

/// The smallest even number that is not less than `fifths / 5`.
fn even_up(fifths: u32) -> u32 {
    let whole = fifths.div_ceil(5);
    whole + whole % 2
}

// The facts about the grammar that the drawing needs.

struct Facts<'g> {
    rules: HashMap<&'g str, &'g Rule>,
    /// The reference spans that link a loop through rules, with its kind.
    links: HashMap<Span, LoopLink>,
    /// The reference spans of the self tail calls.
    self_tail_calls: HashSet<Span>,
}

impl<'g> Facts<'g> {
    fn new(grammar: &'g CheckedGrammar) -> Self {
        let automata = automaton::compile(grammar);
        let graphs = LoopGraphs::new(&automata);
        let mut links = HashMap::new();
        let mut self_tail_calls = HashSet::new();
        for (r, rule) in automata.rules.iter().enumerate() {
            for (p, position) in rule.positions.iter().enumerate() {
                let PositionKind::Reference(target) = position.kind else { continue };
                match graphs.link(&automata, r, p) {
                    Some(Links::Head) => {
                        links.insert(position.span, LoopLink::HeadReferences);
                    }
                    Some(Links::Tail) => {
                        links.insert(position.span, LoopLink::TailReferences);
                        if target == r {
                            self_tail_calls.insert(position.span);
                        }
                    }
                    None => {}
                }
            }
        }
        let rules = grammar.rules.iter().map(|r| (r.name.as_str(), r)).collect();
        Self { rules, links, self_tail_calls }
    }

    /// Whether a rule is for one step: each node that it can make holds
    /// exactly one step and nothing else. A reference and a repetition make
    /// nodes of their own, and an empty match makes no node.
    fn one_step(&self, name: &str) -> bool {
        fn holds_one_step(expr: &Expr) -> bool {
            match expr {
                Expr::Symbol { .. } => true,
                Expr::Choice { alternatives, .. } => alternatives.iter().all(holds_one_step),
                Expr::Optional { item, .. } => holds_one_step(item),
                Expr::Sequence { .. } | Expr::Reference { .. } | Expr::Repetition { .. } => false,
            }
        }
        self.rules.get(name).and_then(|r| r.body.as_ref()).is_some_and(holds_one_step)
    }
}

// The elements of the fold.

/// The size of an element: its width, and its extents above and below the
/// main line.
#[derive(Debug, Clone, Copy)]
struct Size {
    w: u32,
    a: u32,
    b: u32,
}

struct Element<'g> {
    kind: Kind<'g>,
    size: Size,
    /// Whether the element ends with a self tail call, so that no flow leaves
    /// it at the right end.
    tail: bool,
}

enum Kind<'g> {
    Task {
        span: Span,
        symbol: &'g str,
        lines: Vec<String>,
        height: u32,
    },
    Reference {
        span: Span,
        rule: &'g str,
        lines: Vec<String>,
        height: u32,
        one_step: bool,
        link: Option<LoopLink>,
    },
    Sequence(Vec<Element<'g>>),
    /// The main line of each alternative lies at its offset below the main
    /// line of the choice.
    Choice {
        span: Span,
        alternatives: Vec<Element<'g>>,
        offsets: Vec<u32>,
    },
    Optional {
        span: Span,
        item: Box<Element<'g>>,
    },
    /// An optional part whose item is a self tail call: its split gateway
    /// sends the back edge.
    OptionalTail {
        span: Span,
        reference: Span,
        rule: &'g str,
        up: bool,
    },
    /// A self tail call in a row: the flow turns into the lane of the back
    /// edge.
    TailReference {
        span: Span,
        rule: &'g str,
        up: bool,
    },
    Repetition {
        span: Span,
        star: bool,
        marker: Option<String>,
        item: Box<Element<'g>>,
    },
}

/// The builder of the elements of one rule.
struct Builder<'f, 'g> {
    facts: &'f Facts<'g>,
    /// The count of choices around the current place, in an alternative that
    /// is not their first.
    not_first: u32,
    /// The count of choices around the current place, in an alternative that
    /// is not their last.
    not_last: u32,
    /// The counts of back edges above and below the body.
    up: u32,
    down: u32,
}

impl<'f, 'g> Builder<'f, 'g> {
    /// The side of the lane of a back edge from the current place: true for
    /// above. After a self tail call, the rule body must end. So in each
    /// choice around it, only later alternatives lie below it, and only
    /// earlier alternatives lie above it. `None` when no straight route is
    /// clear.
    fn route(&self) -> Option<bool> {
        if self.not_last == 0 {
            Some(false)
        } else if self.not_first == 0 {
            Some(true)
        } else {
            None
        }
    }

    /// The route of a self tail call at `span`, or `None` when the reference
    /// is no self tail call or has no straight route.
    fn tail_route(&mut self, span: Span) -> Option<bool> {
        let up = self.facts.self_tail_calls.contains(&span).then(|| self.route()).flatten()?;
        if up {
            self.up += 1;
        } else {
            self.down += 1;
        }
        Some(up)
    }

    fn build(&mut self, expr: &'g Expr) -> Element<'g> {
        match expr {
            Expr::Symbol { value, span } => {
                let (lines, size) = box_size(value, false);
                let kind = Kind::Task { span: *span, symbol: value, lines, height: size.b * 2 };
                Element { kind, size, tail: false }
            }
            Expr::Reference { name, span } => {
                if let Some(up) = self.tail_route(*span) {
                    let kind = Kind::TailReference { span: *span, rule: name, up };
                    return Element { kind, size: Size { w: FLOW, a: 0, b: 0 }, tail: true };
                }
                let one_step = self.facts.one_step(name);
                let (lines, size) = box_size(name, !one_step);
                let link = self.facts.links.get(span).copied();
                let kind = Kind::Reference { span: *span, rule: name, lines, height: size.b * 2, one_step, link };
                Element { kind, size, tail: false }
            }
            Expr::Sequence { items, .. } => {
                let items: Vec<Element> = items.iter().map(|item| self.build(item)).collect();
                let mut size = Size { w: 0, a: 0, b: 0 };
                for (i, item) in items.iter().enumerate() {
                    size.w += item.size.w + if i > 0 && !item.is_tail_reference() { FLOW } else { 0 };
                    size.a = size.a.max(item.size.a);
                    size.b = size.b.max(item.size.b);
                }
                let tail = items.last().is_some_and(|item| item.tail);
                Element { kind: Kind::Sequence(items), size, tail }
            }
            Expr::Choice { alternatives, span } => {
                let n = alternatives.len();
                let mut built = Vec::with_capacity(n);
                for (i, alternative) in alternatives.iter().enumerate() {
                    let (not_first, not_last) = (u32::from(i > 0), u32::from(i + 1 < n));
                    self.not_first += not_first;
                    self.not_last += not_last;
                    built.push(self.build(alternative));
                    self.not_first -= not_first;
                    self.not_last -= not_last;
                }
                let mut offsets = vec![0];
                for pair in built.windows(2) {
                    let last = *offsets.last().unwrap_or(&0);
                    offsets.push(last + pair[0].size.b + ALT_GAP + pair[1].size.a);
                }
                let widest = built.iter().map(|e| e.size.w).max().unwrap_or(0);
                let first_a = built.first().map_or(0, |e| e.size.a);
                let last_b = built.last().map_or(0, |e| e.size.b) + offsets.last().copied().unwrap_or(0);
                let size = Size { w: 2 * (GATEWAY + FLOW) + widest, a: first_a.max(HALF), b: last_b.max(HALF) };
                let tail = built.iter().all(|e| e.tail);
                Element { kind: Kind::Choice { span: *span, alternatives: built, offsets }, size, tail }
            }
            Expr::Optional { item, span } => {
                if let Expr::Reference { name, span: reference } = &**item
                    && let Some(up) = self.tail_route(*reference)
                {
                    let kind = Kind::OptionalTail { span: *span, reference: *reference, rule: name, up };
                    return Element { kind, size: Size { w: GATEWAY, a: HALF, b: HALF }, tail: false };
                }
                let item = self.build(item);
                let size =
                    Size { w: 2 * (GATEWAY + FLOW) + item.size.w, a: item.size.a + SKIP, b: item.size.b.max(HALF) };
                Element { kind: Kind::Optional { span: *span, item: Box::new(item) }, size, tail: false }
            }
            Expr::Repetition { min, max, item, span } => {
                let item = self.build(item);
                let star = *min == 0;
                let marker = loop_marker(*min, *max);
                let below = item.size.b + BACK + if marker.is_some() { BELOW_MARKER } else { BELOW_PLAIN };
                let size = if star {
                    Size { w: 4 * GATEWAY + 2 * PAIR + 2 * FLOW + item.size.w, a: item.size.a + SKIP, b: below }
                } else {
                    Size { w: 2 * (GATEWAY + FLOW) + item.size.w, a: item.size.a.max(HALF), b: below }
                };
                Element {
                    kind: Kind::Repetition { span: *span, star, marker, item: Box::new(item) },
                    size,
                    tail: false,
                }
            }
        }
    }
}

impl Element<'_> {
    fn is_tail_reference(&self) -> bool {
        matches!(self.kind, Kind::TailReference { .. })
    }
}

/// The lines and the size of a box. A reference box with a drill-down marker
/// adds the band of the marker at its bottom.
fn box_size(label: &str, drill: bool) -> (Vec<String>, Size) {
    let lines = label_lines(label);
    let widest = lines.iter().map(|l| cells(l)).max().unwrap_or(0);
    let w = even_up(widest * CELL_FIFTHS + 5 * 2 * PAD_X).max(MIN_W);
    let lines_h = u32::try_from(lines.len()).unwrap_or(u32::MAX).saturating_mul(LINE_HEIGHT);
    let h = even_up(5 * (lines_h + 2 * PAD_Y)).max(MIN_H) + if drill { DRILL } else { 0 };
    (lines, Size { w, a: h / 2 + BAND, b: h / 2 })
}

/// The loop marker of a bound: ↻ and its numbers. A repetition with `*` or
/// `+` has none.
fn loop_marker(min: u32, max: Option<u32>) -> Option<String> {
    match max {
        None if min <= 1 => None,
        None => Some(format!("↻ {min}+")),
        Some(max) if max == min => Some(format!("↻ {min}")),
        Some(max) => Some(format!("↻ {min}–{max}")),
    }
}

// Placement.

/// A back edge whose lane is not yet known: the points up to the place where
/// it turns to its lane.
struct PendingBackEdge {
    reference: Span,
    rule: String,
    points: Vec<Point>,
    up: bool,
}

#[derive(Default)]
struct Canvas {
    shapes: Vec<Shape>,
    flows: Vec<Flow>,
    back_edges: Vec<PendingBackEdge>,
}

fn pt(x: u32, y: u32) -> Point {
    Point { x, y }
}

impl Canvas {
    fn flow(&mut self, role: FlowRole, span: Option<Span>, label: Option<FlowLabel>, points: &[Point]) {
        self.flows.push(Flow { role, span, label, points: simplified(points) });
    }

    fn sequence(&mut self, points: &[Point]) {
        self.flow(FlowRole::Sequence, None, None, points);
    }

    /// A gateway whose left corner lies on the main line at `x`, `y`.
    fn gateway(&mut self, role: GatewayRole, span: Span, x: u32, y: u32) {
        self.shapes.push(Shape::Gateway { role, span, x, y: y - HALF, size: GATEWAY });
    }

    fn place(&mut self, element: &Element, x: u32, y: u32) {
        let Size { w, a, .. } = element.size;
        match &element.kind {
            Kind::Task { span, symbol, lines, height } => self.shapes.push(Shape::Task {
                span: *span,
                symbol: (*symbol).to_string(),
                x,
                y: y - height / 2,
                width: w,
                height: *height,
                label: lines.clone(),
            }),
            Kind::Reference { span, rule, lines, height, one_step, link } => self.shapes.push(Shape::Reference {
                span: *span,
                rule: (*rule).to_string(),
                x,
                y: y - height / 2,
                width: w,
                height: *height,
                one_step: *one_step,
                loop_link: *link,
                label: lines.clone(),
            }),
            Kind::Sequence(items) => {
                let mut cx = x;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 && !item.is_tail_reference() {
                        self.sequence(&[pt(cx, y), pt(cx + FLOW, y)]);
                        cx += FLOW;
                    }
                    self.place(item, cx, y);
                    cx += item.size.w;
                }
            }
            Kind::Choice { span, alternatives, offsets } => {
                self.gateway(GatewayRole::ChoiceSplit, *span, x, y);
                let bx = x + GATEWAY + FLOW;
                let jx = x + w - GATEWAY;
                let live = alternatives.iter().filter(|e| !e.tail).count();
                for (i, (alternative, offset)) in alternatives.iter().zip(offsets).enumerate() {
                    let by = y + offset;
                    if i == 0 {
                        self.sequence(&[pt(x + GATEWAY, y), pt(bx, y)]);
                    } else {
                        self.sequence(&[pt(x + HALF, y + HALF), pt(x + HALF, by), pt(bx, by)]);
                    }
                    self.place(alternative, bx, by);
                    if alternative.tail {
                        continue;
                    }
                    let ex = bx + alternative.size.w;
                    if live < 2 {
                        // The join gateway receives one flow, so it is not
                        // drawn. The flow passes straight through its place.
                        self.sequence(&[pt(ex, by), pt(jx + HALF, by), pt(jx + HALF, y), pt(x + w, y)]);
                    } else if i == 0 {
                        self.sequence(&[pt(ex, y), pt(jx, y)]);
                    } else {
                        self.sequence(&[pt(ex, by), pt(jx + HALF, by), pt(jx + HALF, y + HALF)]);
                    }
                }
                if live >= 2 {
                    self.gateway(GatewayRole::ChoiceJoin, *span, jx, y);
                }
            }
            Kind::Optional { span, item } => {
                self.gateway(GatewayRole::OptionSplit, *span, x, y);
                let ix = x + GATEWAY + FLOW;
                self.sequence(&[pt(x + GATEWAY, y), pt(ix, y)]);
                self.place(item, ix, y);
                let jx = x + w - GATEWAY;
                let skip = [pt(x + HALF, y - HALF), pt(x + HALF, y - a), pt(jx + HALF, y - a)];
                if item.tail {
                    // The item ends with a self tail call, so the join gateway
                    // receives only the skip flow. It is not drawn, and the
                    // skip flow passes straight through its place.
                    let points = [&skip[..], &[pt(jx + HALF, y), pt(x + w, y)]].concat();
                    self.flow(FlowRole::Skip, Some(*span), None, &points);
                } else {
                    let points = [&skip[..], &[pt(jx + HALF, y - HALF)]].concat();
                    self.flow(FlowRole::Skip, Some(*span), None, &points);
                    self.sequence(&[pt(ix + item.size.w, y), pt(jx, y)]);
                    self.gateway(GatewayRole::OptionJoin, *span, jx, y);
                }
            }
            Kind::OptionalTail { span, reference, rule, up } => {
                self.gateway(GatewayRole::OptionSplit, *span, x, y);
                let sy = if *up { y - HALF } else { y + HALF };
                self.back_edges.push(PendingBackEdge {
                    reference: *reference,
                    rule: (*rule).to_string(),
                    points: vec![pt(x + HALF, sy)],
                    up: *up,
                });
            }
            Kind::TailReference { span, rule, up } => {
                self.back_edges.push(PendingBackEdge {
                    reference: *span,
                    rule: (*rule).to_string(),
                    points: vec![pt(x, y), pt(x + FLOW, y)],
                    up: *up,
                });
            }
            Kind::Repetition { span, star, marker, item } => {
                let jx = if *star {
                    self.gateway(GatewayRole::SkipSplit, *span, x, y);
                    self.sequence(&[pt(x + GATEWAY, y), pt(x + GATEWAY + PAIR, y)]);
                    x + GATEWAY + PAIR
                } else {
                    x
                };
                self.gateway(GatewayRole::LoopJoin, *span, jx, y);
                let ix = jx + GATEWAY + FLOW;
                self.sequence(&[pt(jx + GATEWAY, y), pt(ix, y)]);
                self.place(item, ix, y);
                let sx = ix + item.size.w + FLOW;
                self.sequence(&[pt(ix + item.size.w, y), pt(sx, y)]);
                self.gateway(GatewayRole::LoopSplit, *span, sx, y);
                let by = y + item.size.b + BACK;
                let label =
                    marker.as_ref().map(|text| FlowLabel { text: text.clone(), x: (sx + jx + GATEWAY) / 2, y: by });
                let back = [pt(sx + HALF, y + HALF), pt(sx + HALF, by), pt(jx + HALF, by), pt(jx + HALF, y + HALF)];
                self.flow(FlowRole::Back, Some(*span), label, &back);
                if *star {
                    let ex = sx + GATEWAY + PAIR;
                    self.sequence(&[pt(sx + GATEWAY, y), pt(ex, y)]);
                    self.gateway(GatewayRole::SkipJoin, *span, ex, y);
                    let skip =
                        [pt(x + HALF, y - HALF), pt(x + HALF, y - a), pt(ex + HALF, y - a), pt(ex + HALF, y - HALF)];
                    self.flow(FlowRole::Skip, Some(*span), None, &skip);
                }
            }
        }
    }
}

/// The points of a flow without a repeated point, and without a point in the
/// middle of a straight run.
fn simplified(points: &[Point]) -> Vec<Point> {
    let mut out: Vec<Point> = Vec::with_capacity(points.len());
    for &p in points {
        if out.last() == Some(&p) {
            continue;
        }
        if let [.., a, b] = out[..] {
            let between = |s: u32, m: u32, e: u32| (s <= m && m <= e) || (e <= m && m <= s);
            let straight = (a.x == b.x && b.x == p.x && between(a.y, b.y, p.y))
                || (a.y == b.y && b.y == p.y && between(a.x, b.x, p.x));
            if straight {
                out.pop();
            }
        }
        out.push(p);
    }
    out
}

/// Lays out the diagram of one rule. A rule without a body has no diagram,
/// but a grammar without errors has none.
fn lay_out_rule(facts: &Facts, rule: &Rule) -> Option<Diagram> {
    let body = rule.body.as_ref()?;
    let mut builder = Builder { facts, not_first: 0, not_last: 0, up: 0, down: 0 };
    let body = builder.build(body);
    let has_back_edge = builder.up + builder.down > 0;

    let above = body.size.a.max(EVENT / 2);
    let below = body.size.b.max(EVENT / 2);
    let lanes_above = if builder.up > 0 { LANE * builder.up + LANE_LABEL } else { 0 };
    let y = MARGIN + lanes_above + above;
    let top = y - above;

    let mut canvas = Canvas::default();
    let mut x = MARGIN;
    canvas.shapes.push(Shape::StartEvent { x, y: y - EVENT / 2, size: EVENT });
    x += EVENT;
    canvas.sequence(&[pt(x, y), pt(x + FLOW, y)]);
    x += FLOW;
    let join_x = x;
    if has_back_edge {
        canvas.gateway(GatewayRole::BackEdgeJoin, rule.span, x, y);
        x += GATEWAY;
        canvas.sequence(&[pt(x, y), pt(x + FLOW, y)]);
        x += FLOW;
    }
    canvas.place(&body, x, y);
    x += body.size.w;
    if !body.tail {
        canvas.sequence(&[pt(x, y), pt(x + FLOW, y)]);
        x += FLOW;
        canvas.shapes.push(Shape::EndEvent { x, y: y - EVENT / 2, size: EVENT });
        x += EVENT;
    }

    let mut bottom = y + below;
    let (mut ups, mut downs) = (0, 0);
    for edge in std::mem::take(&mut canvas.back_edges) {
        let turn = *edge.points.last().unwrap_or(&pt(x, y));
        let (lane, end) = if edge.up {
            ups += 1;
            (top - LANE * ups, pt(join_x + HALF, y - HALF))
        } else {
            downs += 1;
            (bottom + LANE * downs, pt(join_x + HALF, y + HALF))
        };
        let mut points = edge.points.clone();
        points.extend([pt(turn.x, lane), pt(join_x + HALF, lane), end]);
        let label = FlowLabel { text: format!("↻ {}", edge.rule), x: (turn.x + join_x + HALF) / 2, y: lane };
        canvas.flow(FlowRole::BackEdge, Some(edge.reference), Some(label), &points);
    }
    if downs > 0 {
        bottom += LANE * downs + LANE_LABEL;
    }
    Some(Diagram {
        rule: rule.name.clone(),
        width: x + MARGIN,
        height: bottom + MARGIN,
        shapes: canvas.shapes,
        flows: canvas.flows,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cells_follow_the_east_asian_width_and_combining_marks() {
        assert_eq!(cells("install"), 7);
        assert_eq!(cells("↻ 1–3"), 5);
        // A wide East Asian character takes two cells.
        assert_eq!(cells("日本"), 4);
        assert_eq!(cells("ＡＢ"), 4);
        // A combining mark takes no cell: e and U+0301.
        assert_eq!(cells("e\u{301}"), 1);
        // A spacing combining mark is a combining mark too: U+0903.
        assert_eq!(cells("\u{915}\u{903}"), 1);
        assert_eq!(UNICODE_VERSION, "17.0.0");
    }

    #[test]
    fn labels_break_after_a_space_an_underscore_or_a_hyphen() {
        assert_eq!(label_lines("install"), ["install"]);
        assert_eq!(label_lines("shows the simplest case"), ["shows the", "simplest case"]);
        // The tie of the design review: the first line holds as much as it can.
        assert_eq!(label_lines("aaaaaa bbbbbb cccccc"), ["aaaaaa bbbbbb", "cccccc"]);
        // A break after `_` or `-` keeps that character.
        assert_eq!(label_lines("request_changes_to_order"), ["request_changes_", "to_order"]);
        assert_eq!(label_lines("long-running-batch-job"), ["long-running-", "batch-job"]);
        // A word longer than 18 cells takes a line of its own.
        assert_eq!(label_lines("a abcdefghijklmnopqrstuvwxyz b"), ["a", "abcdefghijklmnopqrstuvwxyz", "b"]);
        // 18 cells fit on one line, and 19 do not.
        assert_eq!(label_lines("aaaaaaaaa bbbbbbbb"), ["aaaaaaaaa bbbbbbbb"]);
        assert_eq!(label_lines("aaaaaaaaa bbbbbbbbb"), ["aaaaaaaaa", "bbbbbbbbb"]);
    }

    /// The finding F1 of the review `diagram-layout-k40`: a label keeps its
    /// spaces, except at a break.
    #[test]
    fn labels_keep_their_spaces_except_at_a_break() {
        // Without a break, the spaces at the end stay and count as cells.
        assert_eq!(label_lines("long_symbol   "), ["long_symbol   "]);
        assert_eq!(cells(&label_lines("long_symbol   ")[0]), 14);
        assert_eq!(label_lines("  x"), ["  x"]);
        // A symbol of spaces only keeps its spaces.
        assert_eq!(label_lines("   "), ["   "]);
        // Spaces inside a line stay.
        assert_eq!(label_lines("a   b"), ["a   b"]);
        // A break after a run of spaces drops the whole run, and the next
        // line starts with text.
        assert_eq!(label_lines("aaaaaaaaa    bbbbbbbbb"), ["aaaaaaaaa", "bbbbbbbbb"]);
        assert_eq!(label_lines("abcdefghijklmnopqrstuvwxyz  x"), ["abcdefghijklmnopqrstuvwxyz", "x"]);
        // No break falls inside the spaces at the start, so no line is empty.
        assert_eq!(label_lines("  abcdefghijklmnopqrstuvwxyz"), ["  abcdefghijklmnopqrstuvwxyz"]);
        // No break falls inside the spaces at the end. The last line keeps
        // them, and they count towards its 18 cells.
        assert_eq!(label_lines("long_symbol          "), ["long_", "symbol          "]);
        // A break after `_` that spaces follow falls after the spaces.
        assert_eq!(label_lines("aaaaaaaaaaaa_ bbbbbbbbbbbb"), ["aaaaaaaaaaaa_", "bbbbbbbbbbbb"]);
        // A leading `_` or `-` can end a line, because it is text.
        assert_eq!(label_lines("-abcdefghijklmnopqrstuvwxyz"), ["-", "abcdefghijklmnopqrstuvwxyz"]);
    }

    #[test]
    fn a_box_has_even_sizes_from_its_label() {
        // "install": 7 cells are 54.6 px, plus 20 is 74.6, up to 76.
        assert_eq!(box_size("install", false).1.w, 76);
        // 10 cells are exactly 78 px, plus 20 is 98, which is even.
        assert_eq!(box_size("abcdefghij", false).1.w, 98);
        // 15 cells are exactly 117 px, plus 20 is 137, up to 138.
        assert_eq!(box_size("abcdefghijklmno", false).1.w, 138);
        assert_eq!(box_size("a", false).1.w, MIN_W);
        let (_, one) = box_size("a", false);
        assert_eq!((one.a, one.b), (18 + BAND, 18));
        let (_, two) = box_size("shows the simplest case", true);
        assert_eq!(two.b * 2, 48 + DRILL);
    }

    #[test]
    fn a_loop_marker_shows_the_numbers_of_a_bound() {
        assert_eq!(loop_marker(1, Some(3)).as_deref(), Some("↻ 1–3"));
        assert_eq!(loop_marker(0, Some(3)).as_deref(), Some("↻ 0–3"));
        assert_eq!(loop_marker(2, None).as_deref(), Some("↻ 2+"));
        assert_eq!(loop_marker(3, Some(3)).as_deref(), Some("↻ 3"));
        assert_eq!(loop_marker(0, None), None);
        assert_eq!(loop_marker(1, None), None);
    }

    #[test]
    fn a_flow_keeps_only_its_corners() {
        let points = [pt(0, 5), pt(10, 5), pt(10, 5), pt(20, 5), pt(20, 9)];
        assert_eq!(simplified(&points), [pt(0, 5), pt(20, 5), pt(20, 9)]);
        let back = [pt(0, 0), pt(0, 9), pt(5, 9), pt(5, 0)];
        assert_eq!(simplified(&back), back);
    }
}
