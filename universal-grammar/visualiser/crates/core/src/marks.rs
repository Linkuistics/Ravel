//! The marks of a trace in the grammar view: `docs/specs/rule-diagrams.md`,
//! sections "Following a reading", "Marks of a trace" and "The interface of
//! the core".
//!
//! The marks of one open diagram come from the listed readings of the state on
//! view. A marker sits on the first box below the diagram node of a reading.
//! The marks of a next option come from the cut successors of the reading,
//! by the four rules for next positions. The module `forest` evaluates the cut
//! successors.

use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fmt,
};

use kdl::{KdlDocument, KdlEntry, KdlNode};

use crate::{
    automaton::{Automata, PositionKind, RuleId},
    checked::{CheckedGrammar, Expr, Span},
    forest::{self, CutSearch, Forest, Label, LoopKind, Proj, Token},
    kdl_map::keep_string_text,
};

/// An element of the open diagram that carries a mark. A box, a back flow and
/// a back edge name the span of their grammar element. A diagram has one
/// start event and one end event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "wasm",
    derive(serde::Serialize, tsify::Tsify),
    serde(tag = "type", rename_all = "kebab-case", rename_all_fields = "camelCase")
)]
pub enum Element {
    /// The box of a symbol occurrence.
    TaskBox {
        span: Span,
    },
    /// The box of a reference.
    ReferenceBox {
        span: Span,
    },
    /// The back flow of a repetition.
    BackFlow {
        span: Span,
    },
    /// The back edge of a self tail call, by the span of its reference.
    BackEdge {
        span: Span,
    },
    StartEvent,
    EndEvent,
}

impl Element {
    /// The order of the marks: the elements with a span in the order of the
    /// text, then the start event and the end event.
    fn order(&self) -> (u8, Span, u8) {
        let none = Span::new(0, 0);
        match *self {
            Self::TaskBox { span } => (0, span, 0),
            Self::ReferenceBox { span } => (0, span, 1),
            Self::BackFlow { span } => (0, span, 2),
            Self::BackEdge { span } => (0, span, 3),
            Self::StartEvent => (1, none, 0),
            Self::EndEvent => (2, none, 0),
        }
    }
}

fn sorted(mut elements: Vec<Element>) -> Vec<Element> {
    elements.sort_by_key(Element::order);
    elements.dedup();
    elements
}

/// The marks of one next option of a reading, without the end event.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize, tsify::Tsify), serde(rename_all = "camelCase"))]
pub struct OptionMarks {
    pub symbol: String,
    pub elements: Vec<Element>,
}

/// The mark of the end event for one reading: the next options that lie after
/// its diagram node, and whether the sequence can end.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize, tsify::Tsify), serde(rename_all = "camelCase"))]
pub struct EndMark {
    pub options: Vec<String>,
    pub can_end: bool,
}

/// The place where the grammar view shows the current pass of a loop.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(
    feature = "wasm",
    derive(serde::Serialize, tsify::Tsify),
    serde(tag = "type", rename_all = "kebab-case", rename_all_fields = "camelCase")
)]
pub enum PassPlace {
    /// The back flow of a repetition of the open diagram.
    BackFlow { span: Span },
    /// The back edge of a self tail call of the open diagram.
    BackEdge { span: Span },
    /// The breadcrumb of a pass rule, by its index in the open path.
    Breadcrumb { index: usize },
}

/// The current pass of one loop that holds a reading, by its number from 1.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize, tsify::Tsify), serde(rename_all = "camelCase"))]
pub struct PassMark {
    pub place: PassPlace,
    pub pass: usize,
}

/// The marks of one listed reading in the open diagram.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize, tsify::Tsify), serde(rename_all = "camelCase"))]
pub struct ReadingMarks {
    /// The number of the reading from 1, as in the output view.
    pub reading: usize,
    /// The elements of its marker. The list is empty when the reading does
    /// not stand in the open diagram.
    pub marker: Vec<Element>,
    /// Its next options with a mark in the open diagram, in the order of the
    /// symbols.
    pub options: Vec<OptionMarks>,
    /// The mark of the end event, when the reading can leave its diagram node.
    pub end: Option<EndMark>,
    /// The current pass of each loop that holds the reading.
    pub passes: Vec<PassMark>,
    /// The box of the data object of the step in focus, when the step has a box
    /// in the open diagram.
    pub data_object: Vec<Element>,
}

/// The marks of the open diagram for the state on view.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize, tsify::Tsify), serde(rename_all = "camelCase"))]
pub struct Marks {
    pub open_path: Vec<String>,
    /// Each listed reading that has a mark in the open diagram, in the order
    /// of the numbers.
    pub readings: Vec<ReadingMarks>,
    /// The next options of the state that no listed reading allows. The header
    /// lists them.
    pub unlisted_options: Vec<String>,
}

/// An open path that is not a chain of references from the start rule, or
/// from a rule that the start rule does not reach.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize, tsify::Tsify), serde(rename_all = "camelCase"))]
pub struct MarksError {
    pub message: String,
}

impl fmt::Display for MarksError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for MarksError {}

/// The facts of the drawing that the marks need: the reference spans that draw
/// as back edges, whether each rule is for one step, and the rules that each
/// rule refers to.
#[derive(Debug)]
pub(crate) struct ViewFacts {
    pub back_edges: HashSet<Span>,
    pub one_step: Vec<bool>,
    /// For each rule, the rules of its reference boxes, in the order of the
    /// text, each once. They come from the rule expression, as the diagram
    /// does. The automata leave out the references in a part that cannot
    /// derive a finite sequence, but the diagram still draws their boxes.
    pub references: Vec<Vec<RuleId>>,
}

/// The rules that each rule refers to, from the rule expressions: in the order
/// of the text, each once.
pub(crate) fn rule_references(grammar: &CheckedGrammar) -> Vec<Vec<RuleId>> {
    fn visit(expr: &Expr, ids: &HashMap<&str, RuleId>, out: &mut Vec<RuleId>) {
        match expr {
            Expr::Symbol { .. } => {}
            Expr::Reference { name, .. } => {
                if let Some(&id) = ids.get(name.as_str())
                    && !out.contains(&id)
                {
                    out.push(id);
                }
            }
            Expr::Sequence { items, .. } | Expr::Choice { alternatives: items, .. } => {
                items.iter().for_each(|item| visit(item, ids, out));
            }
            Expr::Optional { item, .. } | Expr::Repetition { item, .. } => visit(item, ids, out),
        }
    }
    let ids: HashMap<&str, RuleId> = grammar.rules.iter().enumerate().map(|(i, r)| (r.name.as_str(), i)).collect();
    grammar
        .rules
        .iter()
        .map(|rule| {
            let mut out = Vec::new();
            if let Some(body) = &rule.body {
                visit(body, &ids, &mut out);
            }
            out
        })
        .collect()
}

/// Checks an open path against the rules of the section "The interface of the
/// core", and returns its rules. The references come from the rule
/// expressions, so each reference box of a diagram extends an open path.
pub(crate) fn resolve_open_path(
    references: &[Vec<RuleId>],
    start: RuleId,
    rule_names: &[String],
    open_path: &[String],
) -> Result<Vec<RuleId>, MarksError> {
    let error = |message: String| Err(MarksError { message });
    if open_path.is_empty() {
        return error("The open path is empty.".into());
    }
    let mut rules = Vec::with_capacity(open_path.len());
    for name in open_path {
        match rule_names.iter().position(|r| r == name) {
            Some(rule) => rules.push(rule),
            None => return error(format!("The open path names {name:?}, which is no rule of the grammar.")),
        }
    }
    for pair in rules.windows(2) {
        if !references[pair[0]].contains(&pair[1]) {
            return error(format!(
                "The open path is not a chain of references: the rule {:?} does not refer to the rule {:?}.",
                rule_names[pair[0]], rule_names[pair[1]]
            ));
        }
    }
    let first = rules[0];
    if first != start && reached(references, start)[first] {
        return error(format!(
            "The open path starts at the rule {:?}, which the start rule reaches. An open path starts at the start \
             rule, or at a rule that the start rule does not reach.",
            rule_names[first]
        ));
    }
    Ok(rules)
}

/// The rules that a chain of references from `from` reaches, with `from`.
fn reached(references: &[Vec<RuleId>], from: RuleId) -> Vec<bool> {
    let mut seen = vec![false; references.len()];
    let mut stack = vec![from];
    seen[from] = true;
    while let Some(rule) = stack.pop() {
        for &target in &references[rule] {
            if !seen[target] {
                seen[target] = true;
                stack.push(target);
            }
        }
    }
    seen
}

/// The chains of references from the start rule, with 1 to `max` rules, in
/// the order of the text. The property tests take their open paths from them.
pub(crate) fn chains(references: &[Vec<RuleId>], start: RuleId, max: usize) -> Vec<Vec<RuleId>> {
    let mut out = vec![vec![start]];
    let mut frontier = out.clone();
    for _ in 1..max {
        let mut next = Vec::new();
        for chain in &frontier {
            let last = *chain.last().expect("a rule");
            for &target in &references[last] {
                let mut longer = chain.clone();
                longer.push(target);
                next.push(longer);
            }
        }
        out.extend(next.iter().cloned());
        frontier = next;
    }
    out
}

/// The follow path of a reading: the rules of the rule nodes on its position,
/// from the root. When the last of them is a rule for one step, the follow
/// path leaves it out, unless it is the only one.
pub(crate) fn follow_path(tokens: &[Token], one_step: &[bool]) -> Vec<RuleId> {
    let mut rules: Vec<RuleId> = forest::spine(tokens)
        .into_iter()
        .filter_map(|i| match tokens[i].label {
            Label::Rule(rule) => Some(rule),
            _ => None,
        })
        .collect();
    if rules.len() > 1 && rules.last().is_some_and(|&r| one_step[r]) {
        rules.pop();
    }
    rules
}

/// A mark of a next position, before the open diagram names its element.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Raw {
    Box(Span),
    BackFlow(Span),
    Link(Span),
    End,
}

/// The elements of the open diagram that the rules read.
pub(crate) struct OpenDiagram<'p> {
    open_path: &'p [RuleId],
    /// The element of each box of the open rule, by its span: a symbol
    /// occurrence or a reference.
    boxes: HashMap<Span, Element>,
    repetitions: HashSet<Span>,
    /// The references of the open rule that link a loop through rules.
    links: HashSet<Span>,
    /// The references of the open rule that draw as back edges.
    back_edges: Vec<Span>,
}

impl<'p> OpenDiagram<'p> {
    pub fn new(automata: &Automata, tables: &forest::ForestTables, facts: &ViewFacts, open_path: &'p [RuleId]) -> Self {
        let rule = *open_path.last().expect("an open path");
        let automaton = &automata.rules[rule];
        let mut boxes = HashMap::new();
        let mut links = HashSet::new();
        let mut back_edges = Vec::new();
        for (p, position) in automaton.positions.iter().enumerate() {
            let span = position.span;
            let element = match position.kind {
                PositionKind::Symbol(_) => Element::TaskBox { span },
                PositionKind::Reference(_) => {
                    if tables.graphs.link(automata, rule, p).is_some() {
                        links.insert(span);
                    }
                    if facts.back_edges.contains(&span) {
                        back_edges.push(span);
                        Element::BackEdge { span }
                    } else {
                        Element::ReferenceBox { span }
                    }
                }
            };
            boxes.insert(span, element);
        }
        let repetitions =
            automaton.edges.iter().flatten().flat_map(|e| e.events.iter().map(|event| event.repetition)).collect();
        Self { open_path, boxes, repetitions, links, back_edges }
    }

    fn element(&self, raw: Raw) -> Option<Element> {
        match raw {
            Raw::Box(span) | Raw::Link(span) => self.boxes.get(&span).copied(),
            Raw::BackFlow(span) => Some(Element::BackFlow { span }),
            Raw::End => Some(Element::EndEvent),
        }
    }
}

/// The spine index of the diagram node of a path, when the rules of its rule
/// nodes start with the open path.
fn stands(tokens: &[Token], spine: &[usize], open_path: &[RuleId]) -> Option<usize> {
    let mut j = 0;
    for (s, &i) in spine.iter().enumerate() {
        if let Label::Rule(rule) = tokens[i].label {
            if rule != open_path[j] {
                return None;
            }
            j += 1;
            if j == open_path.len() {
                return Some(s);
            }
        }
    }
    None
}

/// The first box below the spine index `below`: the first node of the path
/// below it whose origins hold a box of the open diagram. It adds the boxes to
/// `out` and returns the spine index of that node, or the length of the spine.
fn first_box(tokens: &[Token], spine: &[usize], below: usize, d: &OpenDiagram, out: &mut BTreeSet<Raw>) -> usize {
    for s in below + 1..spine.len() {
        let origins = tokens[spine[s]].origins.as_slice();
        let hits: Vec<Span> = origins.iter().filter(|o| d.boxes.contains_key(o)).copied().collect();
        if !hits.is_empty() {
            out.extend(hits.into_iter().map(Raw::Box));
            return s;
        }
    }
    spine.len()
}

/// The marks of the next positions of one reading through one next option, by
/// the four rules of the section "Next-option marks". `n` is the count of
/// steps before the next step, and each successor holds the step `n + 1`.
fn option_marks(n: usize, start: RuleId, reading: &[Token], successors: &[Proj], d: &OpenDiagram) -> BTreeSet<Raw> {
    let mut out = BTreeSet::new();
    let step = n + 1;
    let reading_spine = forest::spine(reading);
    let reading_node = stands(reading, &reading_spine, d.open_path);
    for successor in successors {
        let t = &successor.tokens;
        let sp = forest::spine(t);
        let new_node = stands(t, &sp, d.open_path);
        if n == 0 {
            // Before the first step, the node of the start rule nearest the
            // root takes the place of the diagram node, for rule 1.
            if reading_node.is_some() {
                let ds = sp.iter().position(|&i| t[i].label == Label::Rule(start)).expect("a start rule node");
                first_box(t, &sp, ds, d, &mut out);
            } else if let Some(dn) = new_node {
                first_box(t, &sp, dn, d, &mut out);
            }
            continue;
        }
        // The first new node: the first node of the path that holds no step of
        // the reading.
        let k = (0..sp.len()).find(|&s| !forest::holds_earlier(t, sp[s], step)).unwrap_or(sp.len());
        // Rule 1: the next position keeps the diagram node.
        if let Some(dr) = reading_node
            && dr < k
        {
            let f = first_box(t, &sp, dr, d, &mut out);
            for s in dr + 1..f.min(sp.len()) {
                let new_pass = t[sp[s]].label == Label::Pass
                    && matches!(t[sp[s - 1]].label, Label::Loop(LoopKind::Repetition, _))
                    && s >= k
                    && s - 1 < k;
                if new_pass {
                    for o in t[sp[s - 1]].origins.as_slice() {
                        if d.repetitions.contains(o) {
                            out.insert(Raw::BackFlow(*o));
                        }
                    }
                }
            }
            continue;
        }
        // Rule 2: the next position adds a pass of a loop through rules, whose
        // origins hold a link of the open diagram.
        let added_pass = k < sp.len()
            && k > 0
            && t[sp[k]].label == Label::Pass
            && matches!(t[sp[k - 1]].label, Label::Loop(LoopKind::HeadReferences | LoopKind::TailReferences, _));
        let links: Vec<Span> = if added_pass {
            t[sp[k]].origins.as_slice().iter().filter(|o| d.links.contains(o)).copied().collect()
        } else {
            Vec::new()
        };
        if !links.is_empty() {
            out.extend(links.into_iter().map(Raw::Link));
            if let Some(dn) = new_node {
                first_box(t, &sp, dn, d, &mut out);
            }
        } else if reading_node.is_some() {
            // Rule 3: the step lies after the diagram node.
            out.insert(Raw::End);
        } else if let Some(dn) = new_node {
            // Rule 4: only the next position stands in the open diagram.
            first_box(t, &sp, dn, d, &mut out);
        }
    }
    out
}

/// The number from 1 of the pass at token `pass` among the passes of the loop
/// at token `loop_node`.
fn pass_number(tokens: &[Token], loop_node: usize, pass: usize) -> usize {
    let depth = tokens[loop_node].depth + 1;
    tokens[loop_node + 1..=pass].iter().filter(|t| t.depth == depth && t.label == Label::Pass).count()
}

/// The marks of a path that stands in the open diagram: its marker, and the
/// current pass of each loop that holds it.
fn marker_and_passes(
    automata: &Automata,
    facts: &ViewFacts,
    tokens: &[Token],
    d: &OpenDiagram,
) -> (Vec<Element>, Vec<PassMark>) {
    let sp = forest::spine(tokens);
    let Some(dn) = stands(tokens, &sp, d.open_path) else { return (Vec::new(), Vec::new()) };
    let mut boxes = BTreeSet::new();
    let f = first_box(tokens, &sp, dn, d, &mut boxes);
    let marker = if boxes.is_empty() {
        // Before the first step, the one reading stands at the start event.
        vec![Element::StartEvent]
    } else {
        sorted(boxes.into_iter().filter_map(|raw| d.element(raw)).collect())
    };
    let mut passes = Vec::new();
    // A repetition holds the reading when its loop node lies on the position,
    // between the diagram node and the box of the marker.
    for s in dn + 1..f.min(sp.len()) {
        if matches!(tokens[sp[s]].label, Label::Loop(LoopKind::Repetition, _))
            && s + 1 < sp.len()
            && tokens[sp[s + 1]].label == Label::Pass
        {
            let pass = pass_number(tokens, sp[s], sp[s + 1]);
            for &span in tokens[sp[s]].origins.as_slice() {
                if d.repetitions.contains(&span) {
                    passes.push(PassMark { place: PassPlace::BackFlow { span }, pass });
                }
            }
        }
    }
    // A loop through rules holds the reading when a rule node of the open path
    // lies in a pass of that loop. In the diagram of its pass rule, a loop of
    // tail references whose self tail call draws as a back edge shows the pass
    // on that back edge. In any other diagram, the breadcrumb of the pass rule
    // shows it. So a loop that has both a back edge and a link in another rule
    // keeps its pass while the diagram of that other rule is open.
    let m = d.open_path.len();
    let mut j = 0;
    for s in 0..sp.len() {
        if j == m {
            break;
        }
        let Label::Rule(rule) = tokens[sp[s]].label else { continue };
        if s >= 2
            && tokens[sp[s - 1]].label == Label::Pass
            && let Label::Loop(kind @ (LoopKind::HeadReferences | LoopKind::TailReferences), Some(pass_rule)) =
                tokens[sp[s - 2]].label
            && pass_rule == rule
        {
            let pass = pass_number(tokens, sp[s - 2], sp[s - 1]);
            let on_back_edges = kind == LoopKind::TailReferences
                && automata.rules[rule].positions.iter().any(|p| facts.back_edges.contains(&p.span));
            if on_back_edges && j + 1 == m {
                for &span in &d.back_edges {
                    passes.push(PassMark { place: PassPlace::BackEdge { span }, pass });
                }
            } else {
                passes.push(PassMark { place: PassPlace::Breadcrumb { index: j }, pass });
            }
        }
        j += 1;
    }
    (marker, passes)
}

/// The box of the data object of the step `focus`: the first box below the
/// diagram node, on the path from the root to the step.
fn data_object(tokens: &[Token], focus: usize, d: &OpenDiagram) -> Vec<Element> {
    let Some(at) = tokens.iter().position(|t| t.label == Label::Step(focus)) else { return Vec::new() };
    let path = &tokens[..=at];
    let sp = forest::spine(path);
    let Some(dn) = stands(path, &sp, d.open_path) else { return Vec::new() };
    let mut boxes = BTreeSet::new();
    first_box(path, &sp, dn, d, &mut boxes);
    sorted(boxes.into_iter().filter_map(|raw| d.element(raw)).collect())
}

/// What the evaluation of the marks reads from the grammar.
pub(crate) struct Grammar<'g> {
    pub automata: &'g Automata,
    pub tables: &'g forest::ForestTables,
    pub facts: &'g ViewFacts,
}

/// The marks of each listed reading after n steps in the open diagram of
/// `open_path`. The path must be valid, and it must start at the start rule.
/// The forest must belong to the session of the state.
pub(crate) fn reading_marks(
    g: &Grammar,
    forest: &mut Forest,
    n: usize,
    listed: &[Proj],
    open_path: &[RuleId],
    focus: Option<usize>,
) -> Vec<ReadingMarks> {
    let d = OpenDiagram::new(g.automata, g.tables, g.facts, open_path);
    let mut out = Vec::new();
    for (index, proj) in listed.iter().enumerate() {
        let tokens = &proj.tokens;
        let (marker, passes) = marker_and_passes(g.automata, g.facts, tokens, &d);
        let stands = !marker.is_empty();
        let mut options = Vec::new();
        let mut end_options = Vec::new();
        for (symbol, _) in forest.next_options(proj) {
            let mut search = CutSearch::new(open_path, symbol);
            let successors = forest.cut_successors(n, index, &mut search);
            let raw = option_marks(n, g.automata.start, tokens, &successors, &d);
            let name = g.automata.symbols[symbol].clone();
            if raw.contains(&Raw::End) {
                end_options.push(name.clone());
            }
            let elements: Vec<Element> =
                raw.into_iter().filter(|r| *r != Raw::End).filter_map(|r| d.element(r)).collect();
            if !elements.is_empty() {
                options.push(OptionMarks { symbol: name, elements: sorted(elements) });
            }
        }
        let can_end = stands && proj.can_end;
        let end = (!end_options.is_empty() || can_end).then_some(EndMark { options: end_options, can_end });
        let data_object = focus.filter(|&f| (1..=n).contains(&f)).map_or_else(Vec::new, |f| data_object(tokens, f, &d));
        if stands || !options.is_empty() || end.is_some() || !data_object.is_empty() {
            out.push(ReadingMarks { reading: index + 1, marker, options, end, passes, data_object });
        }
    }
    out
}

/// The marks of the next positions of one reading through one option, from a
/// list of its successors, with the end event as an element. The property
/// tests compare the marks of the cut successors with the marks of the
/// successors from an uncapped next step.
pub(crate) fn successor_elements(
    g: &Grammar,
    n: usize,
    reading: &[Token],
    successors: &[Proj],
    open_path: &[RuleId],
) -> Vec<Element> {
    let d = OpenDiagram::new(g.automata, g.tables, g.facts, open_path);
    let raw = option_marks(n, g.automata.start, reading, successors, &d);
    sorted(raw.into_iter().filter_map(|r| d.element(r)).collect())
}

// The marks KDL, for the tests only.

/// Writes the marks of one open diagram as a KDL 2.0 node. Each element names
/// the symbol or the rule of its span, so that a person can review the file.
pub(crate) fn write(marks: &Marks, automata: &Automata, rule_names: &[String]) -> String {
    let mut names: HashMap<Span, String> = HashMap::new();
    for automaton in &automata.rules {
        for position in &automaton.positions {
            let name = match position.kind {
                PositionKind::Symbol(symbol) => automata.symbols[symbol].clone(),
                PositionKind::Reference(rule) => rule_names[rule].clone(),
            };
            names.insert(position.span, name);
        }
    }
    let mut root = KdlNode::new("marks");
    for rule in &marks.open_path {
        push_arg(&mut root, rule);
    }
    let children = root.ensure_children();
    for reading in &marks.readings {
        let mut node = KdlNode::new("reading");
        node.push(KdlEntry::new(reading.reading as i128));
        let inner = node.ensure_children();
        if !reading.marker.is_empty() {
            inner.nodes_mut().push(group("marker", &[], &reading.marker, &names));
        }
        for option in &reading.options {
            inner.nodes_mut().push(group("option", &[&option.symbol], &option.elements, &names));
        }
        if let Some(end) = &reading.end {
            let mut node = KdlNode::new("end");
            node.push(KdlEntry::new_prop("can-end", end.can_end));
            for option in &end.options {
                let mut child = KdlNode::new("option");
                push_arg(&mut child, option);
                node.ensure_children().nodes_mut().push(child);
            }
            inner.nodes_mut().push(node);
        }
        for pass in &reading.passes {
            let mut node = KdlNode::new("pass");
            node.push(KdlEntry::new(pass.pass as i128));
            let place = match pass.place {
                PassPlace::BackFlow { span } => element(&Element::BackFlow { span }, &names),
                PassPlace::BackEdge { span } => element(&Element::BackEdge { span }, &names),
                PassPlace::Breadcrumb { index } => {
                    let mut place = KdlNode::new("breadcrumb");
                    place.push(KdlEntry::new(index as i128));
                    push_arg(&mut place, &marks.open_path[index]);
                    place
                }
            };
            node.ensure_children().nodes_mut().push(place);
            inner.nodes_mut().push(node);
        }
        if !reading.data_object.is_empty() {
            inner.nodes_mut().push(group("data-object", &[], &reading.data_object, &names));
        }
        if inner.nodes().is_empty() {
            node.clear_children();
        }
        children.nodes_mut().push(node);
    }
    for option in &marks.unlisted_options {
        let mut node = KdlNode::new("unlisted-option");
        push_arg(&mut node, option);
        children.nodes_mut().push(node);
    }
    if children.nodes().is_empty() {
        root.clear_children();
    }
    let mut document = KdlDocument::new();
    document.nodes_mut().push(root);
    document.autoformat();
    document.to_string()
}

fn push_arg(node: &mut KdlNode, value: &str) {
    let mut entry = KdlEntry::new(value);
    keep_string_text(&mut entry, value);
    node.push(entry);
}

fn group(name: &str, args: &[&str], elements: &[Element], names: &HashMap<Span, String>) -> KdlNode {
    let mut node = KdlNode::new(name);
    for arg in args {
        push_arg(&mut node, arg);
    }
    let children = node.ensure_children();
    for e in elements {
        children.nodes_mut().push(element(e, names));
    }
    node
}

fn element(element: &Element, names: &HashMap<Span, String>) -> KdlNode {
    let (name, span, named) = match *element {
        Element::TaskBox { span } => ("task-box", Some(span), true),
        Element::ReferenceBox { span } => ("reference-box", Some(span), true),
        Element::BackFlow { span } => ("back-flow", Some(span), false),
        Element::BackEdge { span } => ("back-edge", Some(span), true),
        Element::StartEvent => ("start-event", None, false),
        Element::EndEvent => ("end-event", None, false),
    };
    let mut node = KdlNode::new(name);
    if let Some(span) = span {
        if named && let Some(text) = names.get(&span) {
            push_arg(&mut node, text);
        }
        node.push(KdlEntry::new_prop("start", i128::from(span.start)));
        node.push(KdlEntry::new_prop("end", i128::from(span.end)));
    }
    node
}
