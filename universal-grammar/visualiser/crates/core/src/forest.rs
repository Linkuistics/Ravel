//! The shared packed forest and its readings:
//! `docs/adr/readings-from-a-shared-forest.md` and `docs/specs/tracer.md`,
//! section "Readings and the forest".
//!
//! A forest node is a rule, a state of its automaton, an origin and an end, as
//! in Scott (2008) (https://dl.acm.org/doi/10.1016/j.entcs.2008.03.044). Its
//! packed alternatives are the edges into the state, with the Earley items that
//! allow them. The evaluator finds them from the Earley sets when it needs a
//! node, and it keeps the capped list of each node and loop context in a memo.
//!
//! A projection is a tree as a list of tokens in pre-order. A token has a
//! depth, a label and its origins. So the order of the spec is the order of
//! the lists, and two trees are the same when their lists agree without the
//! origins.

use std::{cmp::Ordering, collections::HashMap, rc::Rc};

use crate::{
    automaton::{Automata, END, LoopEvent, LoopEventKind, PositionKind, RuleId, START, StateId, SymbolId},
    checked::Span,
    earley::{Earley, Item},
    loops::{Contexts, Links, LoopGraphs, Role},
};

/// The kind of a loop node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize, tsify::Tsify), serde(rename_all = "kebab-case"))]
pub enum LoopKind {
    Repetition,
    TailReferences,
    HeadReferences,
}

/// The label of a tree node. The derived order is the order of the spec at
/// equal depth: a loop, a pass, a rule, then a step. Rules compare by their
/// order in the text, and loops by their kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) enum Label {
    Loop(LoopKind, Option<RuleId>),
    Pass,
    Rule(RuleId),
    /// A step, by its number from 1.
    Step(usize),
}

/// One node of a tree in pre-order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Token {
    pub depth: u32,
    pub label: Label,
    /// The spans of the origins, in the order of the text, each once.
    pub origins: Origins,
}

/// The origins of a token. Most tokens have one origin or none, and a clone of
/// a token allocates nothing.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) enum Origins {
    #[default]
    None,
    One(Span),
    Many(Rc<[Span]>),
}

impl Origins {
    pub fn as_slice(&self) -> &[Span] {
        match self {
            Self::None => &[],
            Self::One(span) => std::slice::from_ref(span),
            Self::Many(spans) => spans,
        }
    }

    fn from_slice(spans: &[Span]) -> Self {
        match spans {
            [] => Self::None,
            [span] => Self::One(*span),
            _ => Self::Many(Rc::from(spans)),
        }
    }

    fn union(&mut self, other: &Self) {
        if other.as_slice().iter().all(|s| self.as_slice().binary_search(s).is_ok()) {
            return;
        }
        let mut spans = self.as_slice().to_vec();
        union(&mut spans, other.as_slice());
        *self = Self::from_slice(&spans);
    }
}

fn union(into: &mut Vec<Span>, from: &[Span]) {
    for span in from {
        if let Err(at) = into.binary_search(span) {
            into.insert(at, *span);
        }
    }
}

/// The order of the spec: at the first node that differs, the deeper node
/// comes first. Origins never take part.
fn compare(a: &[Token], b: &[Token]) -> Ordering {
    for (x, y) in a.iter().zip(b) {
        let order = y.depth.cmp(&x.depth).then(x.label.cmp(&y.label));
        if order != Ordering::Equal {
            return order;
        }
    }
    a.len().cmp(&b.len())
}

fn join(into: &mut [Token], from: &[Token]) {
    for (x, y) in into.iter_mut().zip(from) {
        x.origins.union(&y.origins);
    }
}

fn shifted(tokens: &[Token], by: u32) -> impl Iterator<Item = Token> + '_ {
    tokens.iter().map(move |t| Token { depth: t.depth + by, ..t.clone() })
}

/// The passes that a node passes up to a loop through rules around it, at one
/// index of its stack. Each pass starts with its pass token at depth 0.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Part {
    pub passes: Vec<Token>,
    /// In a loop of head references: the links to the pass rule that end the
    /// current pass below the node.
    pub link: Vec<Span>,
}

/// A repetition around the current state of a rule node, and whether its loop
/// token and the token of its current pass are written yet. A token waits for
/// the first step inside it, so a loop or a pass without a step makes no node.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Rep {
    span: Span,
    loop_shown: bool,
    pass_shown: bool,
}

/// A projection: the part of a rule node up to a state, or a whole node.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct Proj {
    /// The tokens: the children of a rule node so far, or a whole node.
    pub tokens: Vec<Token>,
    reps: Vec<Rep>,
    /// The parts for the loops of head references, by stack index.
    pub head: Vec<Part>,
    /// The parts for the loops of tail references, by stack index.
    pub tail: Vec<Part>,
    /// The items of the last set that end the partial derivations, in the
    /// open layer.
    pub frontier: Vec<Item>,
    /// Whether a derivation of this projection completes the start rule.
    pub can_end: bool,
    /// The split of the projection, which `add` sets when it lists it.
    split: Vec<Option<usize>>,
    /// In the open layer: the alternatives that give this projection.
    pub alternatives: Vec<Alternative>,
}

/// One way in which the open layer gives a projection of an open node:
/// `docs/specs/rule-diagrams.md`, section "Next positions". The indices point
/// into lists that the forest keeps for the state: the content lists, which
/// never change, and the lists of the open nodes after their last round.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Alternative {
    /// An item of the last set at `state`, after the projection at index
    /// `content` of the content up to that state.
    Item { state: StateId, content: usize },
    /// The projection at index `index` of the open node `child`, after the
    /// projection at index `content` of the content up to the edge `edge` out
    /// of the state `from`. That content ends at step `h`.
    Child { from: StateId, edge: usize, h: usize, content: usize, child: OpenKey, index: usize },
}

impl Proj {
    /// The order of projections: the parts of the head loops come first in
    /// pre-order, then the tokens, then the parts of the tail loops from the
    /// innermost. The flags of the repetitions decide a tie.
    fn compare(&self, other: &Self) -> Ordering {
        fn parts(a: &[Part], b: &[Part], rev: bool) -> Ordering {
            let n = a.len().max(b.len());
            let empty = Part::default();
            for i in 0..n {
                let i = if rev { n - 1 - i } else { i };
                let (x, y) = (a.get(i).unwrap_or(&empty), b.get(i).unwrap_or(&empty));
                let order = compare(&x.passes, &y.passes).then(x.link.is_empty().cmp(&y.link.is_empty()));
                if order != Ordering::Equal {
                    return order;
                }
            }
            Ordering::Equal
        }
        parts(&self.head, &other.head, false)
            .then_with(|| compare(&self.tokens, &other.tokens))
            .then_with(|| parts(&self.tail, &other.tail, true))
            .then_with(|| {
                let flags = |p: &Self| p.reps.iter().map(|r| (r.loop_shown, r.pass_shown)).collect::<Vec<_>>();
                flags(self).cmp(&flags(other))
            })
    }

    /// Joins the origins, the frontier, the ends and the alternatives of an
    /// equal projection.
    fn join(&mut self, other: &Self) {
        join(&mut self.tokens, &other.tokens);
        for (x, y) in self.head.iter_mut().zip(&other.head).chain(self.tail.iter_mut().zip(&other.tail)) {
            join(&mut x.passes, &y.passes);
            union(&mut x.link, &y.link);
        }
        self.add_frontier(&other.frontier);
        self.can_end |= other.can_end;
        for alternative in &other.alternatives {
            if !self.alternatives.contains(alternative) {
                self.alternatives.push(alternative.clone());
            }
        }
    }

    fn add_frontier(&mut self, items: &[Item]) {
        for item in items {
            if let Err(at) = self.frontier.binary_search(item) {
                self.frontier.insert(at, *item);
            }
        }
    }

    /// Adds a child node to the part of its parent: its tokens, and the parts
    /// that it passes up. A child that starts a later pass of a loop of head
    /// references gives that pass the link `span`.
    fn combine(&mut self, child: &Proj, head: Role, span: Span) {
        self.append(&child.tokens);
        for (mine, theirs, is_head) in [(&mut self.head, &child.head, true), (&mut self.tail, &child.tail, false)] {
            if mine.len() < theirs.len() {
                mine.resize_with(theirs.len(), Part::default);
            }
            for (k, part) in theirs.iter().enumerate() {
                mine[k].passes.extend(part.passes.iter().cloned());
                union(&mut mine[k].link, &part.link);
                // A pass without a step links to nothing, so it sets no link.
                if is_head && head == Role::Pass(k) && !part.passes.is_empty() {
                    union(&mut mine[k].link, &[span]);
                }
            }
        }
        self.add_frontier(&child.frontier);
        self.can_end |= child.can_end;
    }

    /// Applies the loop events of an edge.
    fn apply(&mut self, events: &[LoopEvent]) {
        for event in events {
            match event.kind {
                LoopEventKind::First => {
                    self.reps.push(Rep { span: event.repetition, loop_shown: false, pass_shown: false })
                }
                LoopEventKind::NewPass => {
                    if let Some(rep) = self.reps.last_mut() {
                        rep.pass_shown = false;
                    }
                }
                LoopEventKind::End => {
                    self.reps.pop();
                }
            }
        }
    }

    /// Appends a child that holds a step, inside the open repetitions.
    fn append(&mut self, child: &[Token]) {
        if child.is_empty() {
            return;
        }
        let mut depth = 0;
        for rep in &mut self.reps {
            if !rep.loop_shown {
                let label = Label::Loop(LoopKind::Repetition, None);
                self.tokens.push(Token { depth, label, origins: Origins::One(rep.span) });
                rep.loop_shown = true;
            }
            if !rep.pass_shown {
                self.tokens.push(Token { depth: depth + 1, label: Label::Pass, origins: Origins::None });
                rep.pass_shown = true;
            }
            depth += 2;
        }
        self.tokens.extend(shifted(child, depth));
    }
}

/// The split of a projection: for each loop through rules around it, the step
/// where its passes so far end, or where its later passes start.
///
/// Two projections with one split compare in the order of their final trees.
/// Their passes of a head loop cover the same steps and end with the same
/// step, so neither is the start of the other, and the nodes after them are
/// the same. Their tokens before the passes of a tail loop also end with the
/// same step. Two projections with different splits can compare in either
/// order, by the nodes around them. So the lists keep each split apart, and
/// the cap holds for each split.
fn split(proj: &Proj) -> Vec<Option<usize>> {
    let step = |t: &Token| match t.label {
        Label::Step(n) => Some(n),
        _ => None,
    };
    let head = proj.head.iter().map(|p| p.passes.iter().rev().find_map(step));
    let tail = proj.tail.iter().map(|p| p.passes.iter().find_map(step));
    head.chain(std::iter::once(None)).chain(tail).collect()
}

/// Adds a projection to a list in order, with at most `cap` entries for each
/// split. An equal projection joins the one in the list.
fn add(list: &mut Vec<Proj>, mut proj: Proj, cap: usize) {
    proj.split = split(&proj);
    let key = proj.split.clone();
    match list.binary_search_by(|p| p.split.cmp(&key).then_with(|| p.compare(&proj))) {
        Ok(at) => list[at].join(&proj),
        Err(at) => {
            let first = list[..at].iter().rposition(|p| p.split != key).map_or(0, |i| i + 1);
            if at - first < cap {
                list.insert(at, proj);
                let end = list[at..].iter().position(|p| p.split != key).map_or(list.len(), |i| at + i);
                if end - first > cap {
                    list.remove(end - 1);
                }
            }
        }
    }
}

/// The tables of a grammar that the forest needs.
#[derive(Debug)]
pub(crate) struct ForestTables {
    pub graphs: LoopGraphs,
    /// For each rule and state, the edges into the state: the source state
    /// and the index of the edge among its edges.
    incoming: Vec<Vec<Vec<(StateId, usize)>>>,
}

impl ForestTables {
    pub fn new(automata: &Automata) -> Self {
        let incoming = automata
            .rules
            .iter()
            .map(|rule| {
                let mut incoming = vec![Vec::new(); rule.states.len()];
                for (from, edges) in rule.edges.iter().enumerate() {
                    for (e, edge) in edges.iter().enumerate() {
                        incoming[edge.target].push((from, e));
                    }
                }
                incoming
            })
            .collect();
        Self { graphs: LoopGraphs::new(automata), incoming }
    }
}

/// The roles of a node in the loops of head and tail references.
pub(crate) type Roles = (Role, Role);

/// A forest node in a loop context: its rule, its state, its origin, its end
/// and its context.
type ContentKey = (RuleId, StateId, usize, usize, u32);

/// An open node in a loop context: its rule, its origin, the end of the layer,
/// its context, its roles, the reference that made it, and whether it is the
/// open root.
pub(crate) type OpenKey = (RuleId, usize, usize, u32, Roles, Option<Span>, bool);

struct OpenEntry {
    value: Rc<Vec<Proj>>,
    round: u32,
    busy: bool,
}

/// The evaluator of the forest over the Earley sets of one session.
pub(crate) struct Forest<'a> {
    automata: &'a Automata,
    tables: &'a ForestTables,
    earley: &'a Earley,
    /// The symbol of each step read.
    steps: Vec<SymbolId>,
    cap: usize,
    contexts: Contexts,
    /// The loop contexts: a stack of head loops and a stack of tail loops.
    ctxs: Vec<(Vec<u32>, Vec<u32>)>,
    ctx_ids: HashMap<(Vec<u32>, Vec<u32>), u32>,
    content: HashMap<ContentKey, Rc<Vec<Proj>>>,
    open: HashMap<OpenKey, OpenEntry>,
    /// The readings of each prefix that the evaluator found.
    found: HashMap<usize, Vec<Proj>>,
    /// The end of the open layer: the number of steps of the prefix.
    n: usize,
    round: u32,
    changed: bool,
    cyclic: bool,
}

impl<'a> Forest<'a> {
    /// An evaluator that keeps the first `cap` projections of each node.
    pub fn new(
        automata: &'a Automata,
        tables: &'a ForestTables,
        earley: &'a Earley,
        steps: Vec<SymbolId>,
        cap: usize,
    ) -> Self {
        Self {
            automata,
            tables,
            earley,
            steps,
            cap,
            contexts: Contexts::default(),
            ctxs: Vec::new(),
            ctx_ids: HashMap::new(),
            content: HashMap::new(),
            open: HashMap::new(),
            found: HashMap::new(),
            n: 0,
            round: 0,
            changed: false,
            cyclic: false,
        }
    }

    fn ctx(&mut self, head: Vec<u32>, tail: Vec<u32>) -> u32 {
        let key = (head, tail);
        if let Some(&id) = self.ctx_ids.get(&key) {
            return id;
        }
        let id = self.ctxs.len() as u32;
        self.ctxs.push(key.clone());
        self.ctx_ids.insert(key, id);
        id
    }

    /// The context and the roles of the node that a reference makes. `first`
    /// says whether the rule node holds no step before the reference.
    fn child_ctx(&mut self, ctx: u32, rule: RuleId, state: StateId, target: RuleId, first: bool) -> (u32, Roles) {
        let position = self.automata.rules[rule].states[state].position.expect("a reference state");
        let graphs = &self.tables.graphs;
        let (head, tail) = self.ctxs[ctx as usize].clone();
        let head_link = first && graphs.head[rule][position];
        let (head, head_role) = self.contexts.child(graphs, Links::Head, &head, target, head_link);
        let tail_link = graphs.tail[rule][position];
        let (tail, tail_role) = self.contexts.child(graphs, Links::Tail, &tail, target, tail_link);
        (self.ctx(head, tail), (head_role, tail_role))
    }

    /// The projections of a rule node from origin i up to `state`, over the
    /// steps i+1 to j.
    fn content(&mut self, rule: RuleId, state: StateId, i: usize, j: usize, ctx: u32) -> Rc<Vec<Proj>> {
        let key = (rule, state, i, j, ctx);
        if let Some(found) = self.content.get(&key) {
            return found.clone();
        }
        let mut out = Vec::new();
        if state == START {
            if i == j {
                out.push(Proj::default());
            }
        } else {
            let automaton = &self.automata.rules[rule];
            let target = automaton.position(state).map(|p| (p.kind, p.span));
            for &(from, e) in &self.tables.incoming[rule][state] {
                let events = &automaton.edges[from][e].events;
                let before = Item { rule, state: from, origin: i };
                match target {
                    None => {
                        if self.earley.contains(j, before) {
                            for p in self.content(rule, from, i, j, ctx).iter() {
                                let mut q = p.clone();
                                q.apply(events);
                                add(&mut out, q, self.cap);
                            }
                        }
                    }
                    Some((PositionKind::Symbol(symbol), span)) => {
                        if j > i && self.steps[j - 1] == symbol && self.earley.contains(j - 1, before) {
                            let step = [Token { depth: 0, label: Label::Step(j), origins: Origins::One(span) }];
                            for p in self.content(rule, from, i, j - 1, ctx).iter() {
                                let mut q = p.clone();
                                q.apply(events);
                                q.append(&step);
                                add(&mut out, q, self.cap);
                            }
                        }
                    }
                    Some((PositionKind::Reference(child), span)) => {
                        for k in i..=j {
                            if !self.earley.contains(k, before)
                                || !self.earley.contains(j, Item { rule: child, state: END, origin: k })
                            {
                                continue;
                            }
                            let (child_ctx, roles) = self.child_ctx(ctx, rule, state, child, k == i);
                            let children: Vec<Proj> = self
                                .content(child, END, k, j, child_ctx)
                                .iter()
                                .map(|c| self.render(c, child, child_ctx, roles, &[span], false))
                                .collect();
                            for p in self.content(rule, from, i, k, ctx).iter() {
                                for c in &children {
                                    let mut q = p.clone();
                                    q.apply(events);
                                    q.combine(c, roles.0, span);
                                    add(&mut out, q, self.cap);
                                }
                            }
                        }
                    }
                }
            }
        }
        let out = Rc::new(out);
        self.content.insert(key, out.clone());
        out
    }

    /// Turns the content of a rule node into the node, by its roles in the
    /// loops through rules. `origin` holds the reference that made the node.
    fn render(&self, p: &Proj, rule: RuleId, ctx: u32, roles: Roles, origin: &[Span], root: bool) -> Proj {
        let (head_len, tail_len) = (self.ctxs[ctx as usize].0.len(), self.ctxs[ctx as usize].1.len());
        let part = |parts: &[Part], k: usize| parts.get(k).cloned().unwrap_or_default();
        let pad = |parts: &[Part], n: usize| (0..n).map(|k| part(parts, k)).collect::<Vec<Part>>();
        let node = |origins: &[Span]| -> Vec<Token> {
            if p.tokens.is_empty() {
                return Vec::new();
            }
            let mut tokens = vec![Token { depth: 0, label: Label::Rule(rule), origins: Origins::from_slice(origins) }];
            tokens.extend(shifted(&p.tokens, 1));
            tokens
        };
        let pass = |origins: &[Span], node: &[Token]| -> Vec<Token> {
            if node.is_empty() {
                return Vec::new();
            }
            let mut tokens = vec![Token { depth: 0, label: Label::Pass, origins: Origins::from_slice(origins) }];
            tokens.extend(shifted(node, 1));
            tokens
        };
        // A loop node: its first pass has no origin, and the node of its first
        // pass has the references that entered the loop.
        let wrap = |kind: LoopKind, mut passes: Vec<Token>| -> Vec<Token> {
            if passes.is_empty() {
                return passes;
            }
            passes[0].origins = Origins::None;
            passes[1].origins = Origins::from_slice(origin);
            let mut tokens =
                vec![Token { depth: 0, label: Label::Loop(kind, Some(rule)), origins: Origins::from_slice(origin) }];
            tokens.extend(shifted(&passes, 1));
            tokens
        };
        let mut out = Proj { frontier: p.frontier.clone(), can_end: p.can_end, ..Proj::default() };
        out.head = pad(&p.head, head_len);
        out.tail = pad(&p.tail, tail_len);
        match roles {
            (Role::Enter, _) => {
                let current = part(&p.head, head_len - 1);
                let mut passes = current.passes.clone();
                passes.extend(pass(&current.link, &node(&current.link)));
                out.tokens = wrap(LoopKind::HeadReferences, passes);
                out.head.truncate(head_len - 1);
            }
            (Role::Pass(k), _) => {
                let current = part(&p.head, k);
                let mut passes = current.passes.clone();
                passes.extend(pass(&current.link, &node(&current.link)));
                out.head.truncate(k);
                out.head.push(Part { passes, link: Vec::new() });
            }
            (Role::Nested, Role::Enter) => {
                let mut passes = pass(&[], &node(origin));
                passes.extend(part(&p.tail, tail_len - 1).passes);
                out.tokens = wrap(LoopKind::TailReferences, passes);
                out.tail.truncate(tail_len - 1);
            }
            (Role::Nested, Role::Pass(k)) => {
                let mut passes = pass(origin, &node(origin));
                passes.extend(part(&p.tail, k).passes);
                out.tail.truncate(k);
                out.tail.push(Part { passes, link: Vec::new() });
            }
            (Role::Nested, Role::Nested) => out.tokens = node(origin),
        }
        if root && out.tokens.is_empty() {
            out.tokens = vec![Token { depth: 0, label: Label::Rule(rule), origins: Origins::None }];
        }
        out
    }
}

impl Forest<'_> {
    /// The projections of the open nodes of a rule with origin i, after the
    /// last step: `docs/specs/tracer.md`, section "The open layer". A node on
    /// a cycle of the layer evaluates to its value so far, and the rounds of
    /// `readings` repeat until no value changes. A trip around a cycle adds
    /// only passes without a step, so the values stop changing.
    fn open(
        &mut self,
        rule: RuleId,
        i: usize,
        ctx: u32,
        roles: Roles,
        origin: Option<Span>,
        root: bool,
    ) -> Rc<Vec<Proj>> {
        let key = (rule, i, self.n, ctx, roles, origin, root);
        let old = match self.open.get_mut(&key) {
            Some(entry) if entry.busy => {
                self.cyclic = true;
                return entry.value.clone();
            }
            Some(entry) if entry.round == self.round => return entry.value.clone(),
            Some(entry) => {
                entry.busy = true;
                entry.value.clone()
            }
            None => {
                let value = Rc::new(Vec::new());
                self.open.insert(key, OpenEntry { value: value.clone(), round: self.round, busy: true });
                value
            }
        };
        let n = self.n;
        let origin: Vec<Span> = origin.into_iter().collect();
        let mut out = Vec::new();
        let items: Vec<Item> =
            self.earley.items(n).iter().filter(|item| item.rule == rule && item.origin == i).copied().collect();
        for item in items {
            for (content, p) in self.content(rule, item.state, i, n, ctx).iter().enumerate() {
                let mut q = p.clone();
                q.frontier = vec![item];
                q.can_end = root && item.state == END;
                let mut node = self.render(&q, rule, ctx, roles, &origin, root);
                node.alternatives = vec![Alternative::Item { state: item.state, content }];
                add(&mut out, node, self.cap);
            }
        }
        let automaton = &self.automata.rules[rule];
        for state in 0..automaton.states.len() {
            let Some(&crate::automaton::Position { kind: PositionKind::Reference(child), span }) =
                automaton.position(state)
            else {
                continue;
            };
            for &(from, e) in &self.tables.incoming[rule][state] {
                let events = &automaton.edges[from][e].events;
                for h in i..=n {
                    if !self.earley.contains(h, Item { rule, state: from, origin: i }) {
                        continue;
                    }
                    let (child_ctx, child_roles) = self.child_ctx(ctx, rule, state, child, h == i);
                    let children = self.open(child, h, child_ctx, child_roles, Some(span), false);
                    if children.is_empty() {
                        continue;
                    }
                    let child_key = (child, h, n, child_ctx, child_roles, Some(span), false);
                    for (content, p) in self.content(rule, from, i, h, ctx).iter().enumerate() {
                        for (index, c) in children.iter().enumerate() {
                            let mut q = p.clone();
                            q.apply(events);
                            q.combine(c, child_roles.0, span);
                            let mut node = self.render(&q, rule, ctx, roles, &origin, root);
                            node.alternatives =
                                vec![Alternative::Child { from, edge: e, h, content, child: child_key, index }];
                            add(&mut out, node, self.cap);
                        }
                    }
                }
            }
        }
        if *old != out {
            self.changed = true;
        }
        let out = Rc::new(out);
        let entry = self.open.get_mut(&key).expect("an entry");
        entry.value = out.clone();
        entry.round = self.round;
        entry.busy = false;
        out
    }

    /// The first projections of the open root after n steps, in the order of
    /// the spec: up to `cap`, so one more than the reading cap shows whether
    /// the tracer reached it. A start rule that derives no finite sequence
    /// has no reading.
    pub fn readings(&mut self, n: usize) -> Vec<Proj> {
        let start = self.automata.start;
        if self.automata.rules[start].edges[START].is_empty() {
            return Vec::new();
        }
        if let Some(found) = self.found.get(&n) {
            return found.clone();
        }
        self.n = n;
        let graphs = &self.tables.graphs;
        let (head, head_role) = self.contexts.child(graphs, Links::Head, &[], start, false);
        let (tail, tail_role) = self.contexts.child(graphs, Links::Tail, &[], start, false);
        let ctx = self.ctx(head, tail);
        for _ in 0..64 {
            self.round += 1;
            self.changed = false;
            self.cyclic = false;
            let value = self.open(start, 0, ctx, (head_role, tail_role), None, true);
            if !self.cyclic || !self.changed {
                self.found.insert(n, value.to_vec());
                return value.to_vec();
            }
        }
        unreachable!("the rounds of the open layer stop changing")
    }

    /// The next options of a projection of the open root: the symbols on the
    /// edges out of its frontier, with their occurrences in the order of the
    /// text. They come in the order of the symbols.
    pub fn next_options(&self, proj: &Proj) -> Vec<(SymbolId, Vec<Span>)> {
        let mut options: Vec<(SymbolId, Vec<Span>)> = Vec::new();
        for item in &proj.frontier {
            let rule = &self.automata.rules[item.rule];
            for edge in &rule.edges[item.state] {
                if let Some(position) = rule.position(edge.target)
                    && let PositionKind::Symbol(symbol) = position.kind
                {
                    let at = match options.binary_search_by_key(&symbol, |o| o.0) {
                        Ok(at) => at,
                        Err(at) => {
                            options.insert(at, (symbol, Vec::new()));
                            at
                        }
                    };
                    union(&mut options[at].1, &[position.span]);
                }
            }
        }
        options
    }
}

/// The tree of a reading after step k without step k and the nodes that hold
/// only step k. Step k is the last step in pre-order, so these nodes are the
/// tokens after the last earlier step. Without any step, it is the start rule
/// node alone.
pub(crate) fn without_last_step(tokens: &[Token], k: usize, start: RuleId) -> Vec<Token> {
    let last = tokens.iter().rposition(|t| matches!(t.label, Label::Step(s) if s < k));
    match last {
        Some(last) => tokens[..=last].to_vec(),
        None => vec![Token { depth: 0, label: Label::Rule(start), origins: Origins::None }],
    }
}

/// Whether two trees are the same: the same shape and labels, whatever their
/// origins.
pub(crate) fn same_tree(a: &[Token], b: &[Token]) -> bool {
    compare(a, b) == Ordering::Equal
}

// The cut successors: `docs/specs/rule-diagrams.md`, sections "Next
// positions", "Cut successors" and "The cost of Marks".

/// The progress of a path against an open path: the count of its rules that
/// the path matches so far, or `None` when the path left the open path.
pub(crate) type Progress = Option<usize>;

/// A projection of an open node, with the progress above its node and the
/// progress at the loop node of each loop through rules in its context.
type CutKey = (OpenKey, usize, Progress, Vec<Progress>);

/// The cut successors of one projection for one progress, in the rounds of
/// the evaluation, as `Forest::open` keeps the projections of an open node.
struct CutEntry {
    value: Rc<Vec<Proj>>,
    round: u32,
    busy: bool,
}

/// One evaluation of the cut successors of one listed reading through one
/// next option, for one open path.
pub(crate) struct CutSearch<'o> {
    open_path: &'o [RuleId],
    symbol: SymbolId,
    memo: HashMap<CutKey, CutEntry>,
    round: u32,
    changed: bool,
    cyclic: bool,
    /// The rounds that the evaluation took.
    pub rounds: usize,
    /// The projections that the evaluation visited, once for each round.
    pub visits: usize,
}

impl<'o> CutSearch<'o> {
    pub fn new(open_path: &'o [RuleId], symbol: SymbolId) -> Self {
        Self { open_path, symbol, memo: HashMap::new(), round: 0, changed: false, cyclic: false, rounds: 0, visits: 0 }
    }
}

/// The progress after a rule node of `rule`.
fn advance(open_path: &[RuleId], progress: Progress, rule: RuleId) -> Progress {
    match progress {
        Some(j) if j < open_path.len() => (open_path[j] == rule).then_some(j + 1),
        other => other,
    }
}

/// The indices of the tokens on the path from the root to the last token.
pub(crate) fn spine(tokens: &[Token]) -> Vec<usize> {
    let mut out = Vec::new();
    let mut min = u32::MAX;
    for i in (0..tokens.len()).rev() {
        if tokens[i].depth < min {
            out.push(i);
            min = tokens[i].depth;
        }
    }
    out.reverse();
    out
}

/// Whether the token at spine index `s` is a loop node or a pass node of a
/// repetition.
pub(crate) fn in_repetition(tokens: &[Token], spine: &[usize], s: usize) -> bool {
    match tokens[spine[s]].label {
        Label::Loop(LoopKind::Repetition, _) => true,
        Label::Pass => s > 0 && matches!(tokens[spine[s - 1]].label, Label::Loop(LoopKind::Repetition, _)),
        _ => false,
    }
}

/// Whether the subtree of the spine token at list index `i` holds a step
/// before step `step`. The token lies on the spine, so its subtree runs to the
/// end of the list.
pub(crate) fn holds_earlier(tokens: &[Token], i: usize, step: usize) -> bool {
    tokens[i..].iter().any(|t| matches!(t.label, Label::Step(s) if s < step))
}

/// Cuts the tokens of a successor that an open node without a step made. Each
/// of these nodes is new. The cut keeps the first two nodes of the path.
/// Below them, it keeps only the rule nodes that continue the open path, and
/// then the first node below the new diagram node that is not part of a
/// repetition.
fn cut_list(tokens: &mut Vec<Token>, progress: Progress, open_path: &[RuleId]) {
    let spine = spine(tokens);
    if spine.is_empty() {
        return;
    }
    let second = 1.min(spine.len() - 1);
    let m = open_path.len();
    let mut keep: Vec<usize> = Vec::new();
    if let Some(start) = progress {
        let mut j = start;
        let mut below_diagram_node = start == m;
        for s in 0..spine.len() {
            if below_diagram_node {
                if !in_repetition(tokens, &spine, s) {
                    if s > second {
                        keep.push(s);
                    }
                    break;
                }
                continue;
            }
            if let Label::Rule(rule) = tokens[spine[s]].label {
                if open_path[j] != rule {
                    break;
                }
                j += 1;
                if s > second {
                    keep.push(s);
                }
                below_diagram_node = j == m;
            }
        }
    }
    let mut out: Vec<Token> = tokens[..=spine[second]].to_vec();
    out.extend(keep.into_iter().map(|s| tokens[spine[s]].clone()));
    *tokens = out;
}

/// Adds a cut successor to a list in order. An equal successor joins the one
/// in the list. The list has no cap.
fn add_joined(list: &mut Vec<Proj>, proj: Proj) {
    match list.binary_search_by(|p| p.compare(&proj)) {
        Ok(at) => list[at].join(&proj),
        Err(at) => list.insert(at, proj),
    }
}

impl Forest<'_> {
    /// The key of the open root after n steps, and the counts of its loops of
    /// head and tail references.
    fn root_key(&mut self, n: usize) -> (OpenKey, usize, usize) {
        let start = self.automata.start;
        let graphs = &self.tables.graphs;
        let (head, head_role) = self.contexts.child(graphs, Links::Head, &[], start, false);
        let (tail, tail_role) = self.contexts.child(graphs, Links::Tail, &[], start, false);
        let (h, t) = (head.len(), tail.len());
        let ctx = self.ctx(head, tail);
        ((start, 0, n, ctx, (head_role, tail_role), None, true), h, t)
    }

    /// Cuts a successor that an open node without a step made, together with
    /// the passes that it passes up to each loop around it.
    fn cut(&self, p: &mut Proj, ctx: u32, progress: Progress, loops: &[Progress], open_path: &[RuleId]) {
        cut_list(&mut p.tokens, progress, open_path);
        let h = self.ctxs[ctx as usize].0.len();
        for (k, part) in p.head.iter_mut().enumerate() {
            cut_list(&mut part.passes, loops.get(k).copied().flatten(), open_path);
        }
        for (k, part) in p.tail.iter_mut().enumerate() {
            cut_list(&mut part.passes, loops.get(h + k).copied().flatten(), open_path);
        }
    }

    /// The progress of a child: the progress above its node, and the progress
    /// at the loop node of each loop in its context. A child that starts a
    /// later pass takes the progress at the loop node. A loop that the child
    /// enters is new, and it takes the progress of the child.
    #[allow(clippy::too_many_arguments)]
    fn child_progress(
        &self,
        open_path: &[RuleId],
        ctx: u32,
        progress: Progress,
        loops: &[Progress],
        rule: RuleId,
        child_ctx: u32,
        child_roles: Roles,
    ) -> (Progress, Vec<Progress>) {
        let parent_head = self.ctxs[ctx as usize].0.len();
        let (child_head, child_tail) = &self.ctxs[child_ctx as usize];
        let child_progress = match child_roles {
            (Role::Pass(k), _) => loops[k],
            (Role::Nested, Role::Pass(k)) => loops[parent_head + k],
            _ => advance(open_path, progress, rule),
        };
        // A child stack is a prefix of the parent stack, and an entered loop
        // comes last. A nested loop can have the same identity as a loop
        // around it, so the role, not the identity, says which loop is new.
        let parent = |k: usize| loops[k];
        let head = (0..child_head.len()).map(|k| {
            let new = child_roles.0 == Role::Enter && k + 1 == child_head.len();
            if new { child_progress } else { parent(k) }
        });
        let tail = (0..child_tail.len()).map(|k| {
            let new = child_roles.1 == Role::Enter && k + 1 == child_tail.len();
            if new { child_progress } else { parent(parent_head + k) }
        });
        (child_progress, head.chain(tail).collect())
    }

    /// The cut successors through `search.symbol` of the projection `index` of
    /// an open node, along its alternatives. A projection that the evaluation
    /// meets again while it evaluates that projection gives its list so far.
    /// The rounds of `cut_successors` repeat until no list changes.
    fn cut_successors_of(
        &mut self,
        search: &mut CutSearch,
        key: OpenKey,
        index: usize,
        progress: Progress,
        loops: Vec<Progress>,
    ) -> Rc<Vec<Proj>> {
        let memo_key = (key, index, progress, loops.clone());
        let old = match search.memo.get_mut(&memo_key) {
            Some(entry) if entry.busy => {
                search.cyclic = true;
                return entry.value.clone();
            }
            Some(entry) if entry.round == search.round => return entry.value.clone(),
            Some(entry) => {
                entry.busy = true;
                entry.value.clone()
            }
            None => {
                let value = Rc::new(Vec::new());
                search
                    .memo
                    .insert(memo_key.clone(), CutEntry { value: value.clone(), round: search.round, busy: true });
                value
            }
        };
        search.visits += 1;
        let (rule, i, n, ctx, roles, origin, root) = key;
        let automata = self.automata;
        let automaton = &automata.rules[rule];
        let origin: Vec<Span> = origin.into_iter().collect();
        let step = n + 1;
        // An open node that holds no step cuts its successors as soon as it
        // makes them.
        let holds_no_step = i == n;
        let alternatives = self.open[&key].value[index].alternatives.clone();
        let mut out = Vec::new();
        for alternative in alternatives {
            match alternative {
                Alternative::Item { state, content } => {
                    let p = self.content(rule, state, i, n, ctx)[content].clone();
                    for edge in &automaton.edges[state] {
                        let Some(position) = automaton.position(edge.target) else { continue };
                        if position.kind != PositionKind::Symbol(search.symbol) {
                            continue;
                        }
                        let mut q = p.clone();
                        q.apply(&edge.events);
                        q.append(&[Token { depth: 0, label: Label::Step(step), origins: Origins::One(position.span) }]);
                        q.frontier.clear();
                        q.can_end = false;
                        let mut successor = self.render(&q, rule, ctx, roles, &origin, root);
                        if holds_no_step {
                            self.cut(&mut successor, ctx, progress, &loops, search.open_path);
                        }
                        add_joined(&mut out, successor);
                    }
                }
                Alternative::Child { from, edge, h, content, child, index } => {
                    let p = self.content(rule, from, i, h, ctx)[content].clone();
                    let events = &automaton.edges[from][edge].events;
                    let (_, _, _, child_ctx, child_roles, span, _) = child;
                    let span = span.expect("a child that a reference made");
                    let (child_progress, child_loops) =
                        self.child_progress(search.open_path, ctx, progress, &loops, rule, child_ctx, child_roles);
                    let children = self.cut_successors_of(search, child, index, child_progress, child_loops);
                    for c in children.iter() {
                        let mut q = p.clone();
                        q.apply(events);
                        q.combine(c, child_roles.0, span);
                        q.frontier.clear();
                        q.can_end = false;
                        let mut successor = self.render(&q, rule, ctx, roles, &origin, root);
                        if holds_no_step {
                            self.cut(&mut successor, ctx, progress, &loops, search.open_path);
                        }
                        add_joined(&mut out, successor);
                    }
                }
            }
        }
        if *old != out {
            search.changed = true;
        }
        let out = Rc::new(out);
        let entry = search.memo.get_mut(&memo_key).expect("an entry");
        entry.value = out.clone();
        entry.round = search.round;
        entry.busy = false;
        out
    }

    /// The cut successors of the listed reading at index `reading` after n
    /// steps, through `search.symbol`, for `search.open_path`. The rounds
    /// repeat until no list changes. The lists only grow, and they are finite,
    /// so the rounds stop.
    pub fn cut_successors(&mut self, n: usize, reading: usize, search: &mut CutSearch) -> Vec<Proj> {
        let _ = self.readings(n);
        self.n = n;
        let (key, h, t) = self.root_key(n);
        let loops = vec![Some(0); h + t];
        loop {
            search.round += 1;
            search.rounds += 1;
            search.changed = false;
            search.cyclic = false;
            let out = self.cut_successors_of(search, key, reading, Some(0), loops.clone());
            if !search.cyclic || !search.changed {
                return out.to_vec();
            }
        }
    }
}
