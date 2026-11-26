//! The position automaton of each rule: `docs/specs/tracer.md`, section "The
//! automaton of a rule".
//!
//! The positions of a rule are the symbol occurrences and the references of its
//! body. A state is the start state, the end state, or a position with a pass
//! count for each counted repetition around it. An edge leads from a state to a
//! position that can come next, or to the end state. It carries the loop events
//! of its path through the rule body, in the way of Thompson's construction.
//!
//! The compiler removes each part of a rule that can derive no finite sequence.
//! So every state can reach the end state, and the tracer has the
//! correct-prefix property.

use std::collections::HashMap;

use crate::checked::{CheckedGrammar, Expr, Span};

pub(crate) type RuleId = usize;
pub(crate) type SymbolId = usize;
pub(crate) type StateId = usize;

/// The start state of every automaton.
pub(crate) const START: StateId = 0;
/// The end state of every automaton.
pub(crate) const END: StateId = 1;

/// The automata of a grammar without errors.
#[derive(Debug)]
pub(crate) struct Automata {
    /// One automaton for each rule, in the order of the text.
    pub rules: Vec<RuleAutomaton>,
    /// The start rule.
    pub start: RuleId,
    /// The symbols, in the order of the checked grammar.
    pub symbols: Vec<String>,
    symbol_ids: HashMap<String, SymbolId>,
    /// Whether each rule can match the empty sequence.
    pub nullable: Vec<bool>,
}

impl Automata {
    pub fn symbol(&self, value: &str) -> Option<SymbolId> {
        self.symbol_ids.get(value).copied()
    }
}

/// The automaton of one rule.
#[derive(Debug)]
pub(crate) struct RuleAutomaton {
    /// The positions of the rule body that the automaton reaches, in the order
    /// of the text.
    pub positions: Vec<Position>,
    /// The states. `START` and `END` come first.
    pub states: Vec<State>,
    /// The edges out of each state, in the order of the rule body.
    pub edges: Vec<Vec<Edge>>,
}

/// A symbol occurrence or a reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Position {
    pub kind: PositionKind,
    /// The span of the grammar element: its identity.
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PositionKind {
    Symbol(SymbolId),
    Reference(RuleId),
}

/// A state: a position with the pass count of each counted repetition around
/// it, from the outermost. The start state and the end state have no position.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct State {
    pub position: Option<usize>,
    pub counts: Vec<u32>,
}

/// An edge to the next position, or to the end state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Edge {
    pub target: StateId,
    /// The loop events of the path, in order.
    pub events: Vec<LoopEvent>,
}

/// A loop event of a repetition, which the repetition names by its span.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct LoopEvent {
    pub kind: LoopEventKind,
    pub repetition: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum LoopEventKind {
    /// The repetition starts with its first pass.
    First,
    /// A new pass starts.
    NewPass,
    /// The repetition ends.
    End,
}

impl RuleAutomaton {
    /// The position of a state, if it has one.
    pub fn position(&self, state: StateId) -> Option<&Position> {
        self.states[state].position.map(|p| &self.positions[p])
    }
}

/// Compiles the automata of a grammar. The grammar has no error, so each name
/// has one rule, each reference has a rule, and the start rule exists.
pub(crate) fn compile(grammar: &CheckedGrammar) -> Automata {
    let rule_ids: HashMap<&str, RuleId> = grammar.rules.iter().enumerate().map(|(i, r)| (r.name.as_str(), i)).collect();
    let symbols: Vec<String> = grammar.symbols.iter().map(|s| s.value.clone()).collect();
    let symbol_ids: HashMap<String, SymbolId> = symbols.iter().enumerate().map(|(i, s)| (s.clone(), i)).collect();
    let start_name = grammar.start_rules.first().map(|s| s.name.as_str()).unwrap_or_default();
    let start = rule_ids.get(start_name).copied().unwrap_or(0);

    let bodies: Vec<Body> =
        grammar.rules.iter().map(|rule| Body::new(rule.body.as_ref(), &rule_ids, &symbol_ids)).collect();
    let productive = productive_rules(&bodies);
    let rules: Vec<RuleAutomaton> = bodies.iter().map(|body| Builder::new(body, &productive).build()).collect();
    let nullable = nullable_rules(&rules);
    Automata { rules, start, symbols, symbol_ids, nullable }
}

/// A rule body as an arena with parent links.
struct Body {
    nodes: Vec<Node>,
    root: Option<usize>,
}

struct Node {
    kind: NodeKind,
    parent: Option<usize>,
    span: Span,
}

enum NodeKind {
    Symbol(SymbolId),
    Reference(RuleId),
    Sequence(Vec<usize>),
    Choice(Vec<usize>),
    Optional(usize),
    Repetition { min: u32, max: Option<u32>, item: usize },
}

impl Body {
    fn new(expr: Option<&Expr>, rules: &HashMap<&str, RuleId>, symbols: &HashMap<String, SymbolId>) -> Self {
        let mut body = Self { nodes: Vec::new(), root: None };
        body.root = expr.and_then(|e| body.add(e, None, rules, symbols));
        body
    }

    fn add(
        &mut self,
        expr: &Expr,
        parent: Option<usize>,
        rules: &HashMap<&str, RuleId>,
        symbols: &HashMap<String, SymbolId>,
    ) -> Option<usize> {
        let id = self.nodes.len();
        let kind = match expr {
            Expr::Symbol { value, .. } => NodeKind::Symbol(*symbols.get(value)?),
            Expr::Reference { name, .. } => NodeKind::Reference(*rules.get(name.as_str())?),
            Expr::Sequence { .. } => NodeKind::Sequence(Vec::new()),
            Expr::Choice { .. } => NodeKind::Choice(Vec::new()),
            Expr::Optional { .. } => NodeKind::Optional(usize::MAX),
            Expr::Repetition { min, max, .. } => NodeKind::Repetition { min: *min, max: *max, item: usize::MAX },
        };
        self.nodes.push(Node { kind, parent, span: expr.span() });
        match expr {
            Expr::Sequence { items, .. } | Expr::Choice { alternatives: items, .. } => {
                let children: Vec<usize> = items.iter().filter_map(|i| self.add(i, Some(id), rules, symbols)).collect();
                match &mut self.nodes[id].kind {
                    NodeKind::Sequence(c) | NodeKind::Choice(c) => *c = children,
                    _ => unreachable!(),
                }
            }
            Expr::Optional { item, .. } | Expr::Repetition { item, .. } => {
                let child = self.add(item, Some(id), rules, symbols)?;
                match &mut self.nodes[id].kind {
                    NodeKind::Optional(c) | NodeKind::Repetition { item: c, .. } => *c = child,
                    _ => unreachable!(),
                }
            }
            Expr::Symbol { .. } | Expr::Reference { .. } => {}
        }
        Some(id)
    }

    /// Whether a node can derive a finite sequence, given the productive rules.
    fn productive(&self, node: usize, rules: &[bool]) -> bool {
        match &self.nodes[node].kind {
            NodeKind::Symbol(_) => true,
            NodeKind::Reference(r) => rules[*r],
            NodeKind::Sequence(items) => items.iter().all(|&i| self.productive(i, rules)),
            NodeKind::Choice(alts) => alts.iter().any(|&i| self.productive(i, rules)),
            NodeKind::Optional(_) => true,
            NodeKind::Repetition { min, item, .. } => *min == 0 || self.productive(*item, rules),
        }
    }

    /// Whether a repetition needs a pass count: `*`, `+` and `{1,}` need none.
    fn counted(&self, node: usize) -> bool {
        matches!(self.nodes[node].kind, NodeKind::Repetition { min, max, .. } if !(min <= 1 && max.is_none()))
    }
}

/// The rules that can derive a finite sequence, by a fixpoint from none.
fn productive_rules(bodies: &[Body]) -> Vec<bool> {
    let mut productive = vec![false; bodies.len()];
    loop {
        let mut changed = false;
        for (i, body) in bodies.iter().enumerate() {
            if !productive[i] && body.root.is_some_and(|root| body.productive(root, &productive)) {
                productive[i] = true;
                changed = true;
            }
        }
        if !changed {
            return productive;
        }
    }
}

/// The rules that can match the empty sequence. A rule is nullable when a path
/// of its automaton leads from the start state to the end state through
/// references to nullable rules only.
fn nullable_rules(rules: &[RuleAutomaton]) -> Vec<bool> {
    let mut nullable = vec![false; rules.len()];
    loop {
        let mut changed = false;
        for (i, rule) in rules.iter().enumerate() {
            if nullable[i] {
                continue;
            }
            let mut seen = vec![false; rule.states.len()];
            let mut stack = vec![START];
            seen[START] = true;
            while let Some(state) = stack.pop() {
                if state == END {
                    nullable[i] = true;
                    changed = true;
                    break;
                }
                for edge in &rule.edges[state] {
                    let passes = match rule.position(edge.target) {
                        None => true,
                        Some(Position { kind: PositionKind::Reference(r), .. }) => nullable[*r],
                        Some(Position { kind: PositionKind::Symbol(_), .. }) => false,
                    };
                    if passes && !seen[edge.target] {
                        seen[edge.target] = true;
                        stack.push(edge.target);
                    }
                }
            }
        }
        if !changed {
            return nullable;
        }
    }
}

/// Builds the automaton of one rule.
struct Builder<'a> {
    body: &'a Body,
    productive_rules: &'a [bool],
    /// Whether each node of the body is productive.
    productive: Vec<bool>,
    /// The position index of each symbol or reference node.
    position_of: HashMap<usize, usize>,
    positions: Vec<Position>,
    /// The node of each position.
    position_nodes: Vec<usize>,
    states: Vec<State>,
    state_ids: HashMap<State, StateId>,
}

/// A path in progress: the pass counts of the counted repetitions entered so
/// far, from the outermost, and the loop events so far.
#[derive(Clone, Default)]
struct Path {
    counts: Vec<(usize, u32)>,
    events: Vec<LoopEvent>,
}

impl<'a> Builder<'a> {
    fn new(body: &'a Body, productive_rules: &'a [bool]) -> Self {
        let productive = (0..body.nodes.len()).map(|n| body.productive(n, productive_rules)).collect();
        let mut builder = Self {
            body,
            productive_rules,
            productive,
            position_of: HashMap::new(),
            positions: Vec::new(),
            position_nodes: Vec::new(),
            states: Vec::new(),
            state_ids: HashMap::new(),
        };
        builder.intern(State { position: None, counts: Vec::new() });
        builder.states.push(State { position: None, counts: vec![u32::MAX] });
        builder
    }

    fn intern(&mut self, state: State) -> StateId {
        if let Some(&id) = self.state_ids.get(&state) {
            return id;
        }
        let id = self.states.len();
        self.states.push(state.clone());
        self.state_ids.insert(state, id);
        id
    }

    fn build(mut self) -> RuleAutomaton {
        let mut edges: Vec<Vec<Edge>> = vec![Vec::new(), Vec::new()];
        let mut next = START;
        while next < self.states.len() {
            let state = next;
            next += 1;
            if state == END {
                continue;
            }
            let mut targets = Vec::new();
            match self.states[state].position {
                None => {
                    if let Some(root) = self.body.root.filter(|&r| self.productive[r]) {
                        self.enter(root, Path::default(), &mut targets);
                    }
                }
                Some(position) => {
                    let node = self.position_nodes[position];
                    let counted = self.counted_ancestors(node);
                    let counts = counted.into_iter().zip(self.states[state].counts.iter().copied()).collect();
                    self.exit(node, Path { counts, events: Vec::new() }, &mut targets);
                }
            }
            let mut out: Vec<Edge> = Vec::new();
            for (target, events) in targets {
                let target = match target {
                    Target::End => END,
                    Target::Position(node, counts) => {
                        let position = self.position(node);
                        self.intern(State { position: Some(position), counts })
                    }
                };
                let edge = Edge { target, events };
                if !out.contains(&edge) {
                    out.push(edge);
                }
            }
            edges.resize_with(self.states.len(), Vec::new);
            edges[state] = out;
        }
        edges.resize_with(self.states.len(), Vec::new);
        // The end state holds no counts. Its placeholder only kept it apart
        // from the start state while the builder interned states.
        self.states[END].counts.clear();
        let mut order: Vec<usize> = (0..self.positions.len()).collect();
        order.sort_by_key(|&p| self.positions[p].span);
        let mut renumber = vec![0; order.len()];
        for (new, &old) in order.iter().enumerate() {
            renumber[old] = new;
        }
        let positions = order.iter().map(|&p| self.positions[p]).collect();
        for state in &mut self.states {
            if let Some(p) = &mut state.position {
                *p = renumber[*p];
            }
        }
        RuleAutomaton { positions, states: self.states, edges }
    }

    fn position(&mut self, node: usize) -> usize {
        if let Some(&p) = self.position_of.get(&node) {
            return p;
        }
        let kind = match self.body.nodes[node].kind {
            NodeKind::Symbol(s) => PositionKind::Symbol(s),
            NodeKind::Reference(r) => PositionKind::Reference(r),
            _ => unreachable!("only symbols and references are positions"),
        };
        let p = self.positions.len();
        self.positions.push(Position { kind, span: self.body.nodes[node].span });
        self.position_nodes.push(node);
        self.position_of.insert(node, p);
        p
    }

    /// The counted repetitions around a node, from the outermost.
    fn counted_ancestors(&self, node: usize) -> Vec<usize> {
        let mut out = Vec::new();
        let mut current = self.body.nodes[node].parent;
        while let Some(n) = current {
            if self.body.counted(n) {
                out.push(n);
            }
            current = self.body.nodes[n].parent;
        }
        out.reverse();
        out
    }

    /// Follows every path into a productive node, up to the first position.
    fn enter(&self, node: usize, path: Path, out: &mut Vec<(Target, Vec<LoopEvent>)>) {
        match &self.body.nodes[node].kind {
            NodeKind::Symbol(_) => {
                let counts = path.counts.iter().map(|&(_, c)| c).collect();
                out.push((Target::Position(node, counts), path.events));
            }
            NodeKind::Reference(r) => {
                if self.productive_rules[*r] {
                    let counts = path.counts.iter().map(|&(_, c)| c).collect();
                    out.push((Target::Position(node, counts), path.events));
                }
            }
            NodeKind::Sequence(items) => {
                if let Some(&first) = items.first() {
                    self.enter(first, path, out);
                }
            }
            NodeKind::Choice(alternatives) => {
                for &alternative in alternatives {
                    if self.productive[alternative] {
                        self.enter(alternative, path.clone(), out);
                    }
                }
            }
            NodeKind::Optional(item) => {
                if self.productive[*item] {
                    self.enter(*item, path.clone(), out);
                }
                self.exit(node, path, out);
            }
            NodeKind::Repetition { min, item, .. } => {
                if self.productive[*item] {
                    let mut inner = path.clone();
                    inner.events.push(LoopEvent { kind: LoopEventKind::First, repetition: self.body.nodes[node].span });
                    if self.body.counted(node) {
                        inner.counts.push((node, 1));
                    }
                    self.enter(*item, inner, out);
                }
                if *min == 0 {
                    self.exit(node, path, out);
                }
            }
        }
    }

    /// Follows every path out of a node, up to the next position or the end.
    fn exit(&self, node: usize, path: Path, out: &mut Vec<(Target, Vec<LoopEvent>)>) {
        let Some(parent) = self.body.nodes[node].parent else {
            out.push((Target::End, path.events));
            return;
        };
        match &self.body.nodes[parent].kind {
            NodeKind::Sequence(items) => {
                let index = items.iter().position(|&i| i == node).expect("a child of its sequence");
                match items.get(index + 1) {
                    Some(&next) => self.enter(next, path, out),
                    None => self.exit(parent, path, out),
                }
            }
            NodeKind::Choice(_) | NodeKind::Optional(_) => self.exit(parent, path, out),
            NodeKind::Repetition { min, max, item } => {
                let span = self.body.nodes[parent].span;
                let counted = self.body.counted(parent);
                let count = if counted {
                    path.counts.iter().rev().find(|&&(n, _)| n == parent).map_or(1, |&(_, c)| c)
                } else {
                    1
                };
                if max.is_none_or(|max| count < max) {
                    let mut again = path.clone();
                    again.events.push(LoopEvent { kind: LoopEventKind::NewPass, repetition: span });
                    if counted {
                        // An open bound `{m,}` counts to m and then stays at m.
                        let next = if max.is_none() { (count + 1).min(*min) } else { count + 1 };
                        if let Some(entry) = again.counts.iter_mut().rev().find(|(n, _)| *n == parent) {
                            entry.1 = next;
                        }
                    }
                    self.enter(*item, again, out);
                }
                if count >= *min {
                    let mut done = path;
                    done.events.push(LoopEvent { kind: LoopEventKind::End, repetition: span });
                    if counted {
                        done.counts.retain(|&(n, _)| n != parent);
                    }
                    self.exit(parent, done, out);
                }
            }
            NodeKind::Symbol(_) | NodeKind::Reference(_) => unreachable!("a symbol or a reference has no child"),
        }
    }
}

enum Target {
    End,
    /// A position node, with the pass counts of its counted repetitions.
    Position(usize, Vec<u32>),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::checked_grammar;

    fn automata(text: &str) -> Automata {
        let grammar = checked_grammar(text);
        assert!(!grammar.has_errors(), "{:?}", grammar.diagnostics);
        compile(&grammar)
    }

    /// The example of the spec: two edges lead from `"x"` back to `"x"` in
    /// `("x"+ "y"?)+`, one for each pair of loop events.
    #[test]
    fn two_paths_with_different_events_give_two_edges() {
        let a = automata("%start a ;\na = (\"x\"+ \"y\"?)+ ;\n");
        let rule = &a.rules[0];
        let x = rule.states.iter().position(|s| s.position == Some(0)).expect("the state of x");
        let back_to_x: Vec<&Edge> = rule.edges[x].iter().filter(|e| e.target == x).collect();
        assert_eq!(back_to_x.len(), 2, "{:?}", rule.edges[x]);
        let kinds: Vec<Vec<LoopEventKind>> =
            back_to_x.iter().map(|e| e.events.iter().map(|ev| ev.kind).collect()).collect();
        assert!(kinds.contains(&vec![LoopEventKind::NewPass]), "{kinds:?}");
        // The second edge ends the inner repetition, starts a new pass of the
        // outer one, and enters the inner repetition again.
        assert!(kinds.contains(&vec![LoopEventKind::End, LoopEventKind::NewPass, LoopEventKind::First]), "{kinds:?}");
    }

    /// A bounded repetition counts its passes in the state. It never copies
    /// its body.
    #[test]
    fn a_bound_counts_passes_in_the_state() {
        let a = automata("%start a ;\na = \"x\"{2,4} ;\n");
        let rule = &a.rules[0];
        assert_eq!(rule.positions.len(), 1);
        let counts: Vec<Vec<u32>> =
            rule.states.iter().filter(|s| s.position.is_some()).map(|s| s.counts.clone()).collect();
        assert_eq!(counts, vec![vec![1], vec![2], vec![3], vec![4]]);
        let open = automata("%start a ;\na = \"x\"{3,} ;\n");
        assert_eq!(open.rules[0].states.len(), 2 + 3, "an open bound counts to 3 and stays at 3");
    }

    /// The compiler removes each part that derives no finite sequence.
    #[test]
    fn dead_parts_are_removed() {
        let a = automata("%start s ;\ns = a | \"y\" ;\na = \"x\" a ;\n");
        let s = &a.rules[0];
        assert!(
            s.edges[START]
                .iter()
                .all(|e| matches!(s.position(e.target), Some(p) if matches!(p.kind, PositionKind::Symbol(_))))
        );
        assert!(a.rules[1].edges[START].is_empty(), "the rule `a` derives no finite sequence");
    }

    #[test]
    fn nullable_rules_through_references() {
        let a = automata("%start a ;\na = b c \"z\" ;\nb = \"x\"? ;\nc = b ;\n");
        assert_eq!(a.nullable, vec![false, true, true]);
    }
}
