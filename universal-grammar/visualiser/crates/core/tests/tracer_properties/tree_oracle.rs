//! The tree oracle: `docs/specs/tracer.md`, section "The reference
//! recogniser".
//!
//! The oracle finds the readings of a prefix from the grammar text. It uses no
//! automaton, no Earley set, no forest and no cap. It keeps a set of distinct
//! projections for each grammar element, start, end and loop context. An
//! element can also start at a step and stay open after the last step. Equal
//! projections join their origins, so the count of derivations does not matter.
//!
//! The oracle applies the tree rules and the loop contexts of the core, because
//! the spec says that both sides apply them. They are a copy of `render` in
//! `src/forest.rs` and of `Contexts` in `src/loops.rs`. The search for the
//! derivations is its own: it follows the expressions of the rule bodies, and it
//! grows the open sets until they stop growing. So a trip around a cycle of the
//! open layer that added a tree would add that tree to the sets of the oracle.

use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashMap, HashSet},
    rc::Rc,
};

use workbench_core::{
    checked::{CheckedGrammar, Expr, Span},
    trace::LoopKind,
};

/// The most trees in one set. A case with a larger set is skipped.
pub const MAX_TREES: usize = 1_000;

type RuleId = usize;
type NodeId = usize;

// The grammar, without the parts that derive no finite sequence.

/// An expression of a rule body. A reference knows whether it is a head
/// reference and whether it is a tail reference.
#[derive(Debug, Clone)]
enum Node {
    Symbol {
        symbol: usize,
        span: Span,
    },
    Reference {
        rule: RuleId,
        span: Span,
        head: bool,
        tail: bool,
    },
    Sequence(Vec<NodeId>),
    Choice(Vec<NodeId>),
    Optional(NodeId),
    Repetition {
        min: u32,
        max: Option<u32>,
        item: NodeId,
        span: Span,
    },
    /// The rest of a part that derives only the empty sequence.
    Empty,
}

/// A checked grammar without its dead parts, with the graphs of its loops
/// through rules.
pub struct Grammar {
    nodes: Vec<Node>,
    /// The body of each rule that derives a finite sequence.
    bodies: Vec<Option<NodeId>>,
    nullable: Vec<bool>,
    /// The fewest steps that each rule derives.
    min_steps: Vec<usize>,
    start: RuleId,
    pub rule_names: Vec<String>,
    pub symbols: Vec<String>,
    head_targets: Vec<Vec<RuleId>>,
    tail_targets: Vec<Vec<RuleId>>,
}

impl Grammar {
    pub fn new(grammar: &CheckedGrammar) -> Self {
        let rule_names: Vec<String> = grammar.rules.iter().map(|r| r.name.clone()).collect();
        let symbols: Vec<String> = grammar.symbols.iter().map(|s| s.value.clone()).collect();
        let start = rule_names.iter().position(|n| *n == grammar.start_rules[0].name).expect("the start rule");
        let n = rule_names.len();
        let mut g = Self {
            nodes: Vec::new(),
            bodies: vec![None; n],
            nullable: vec![false; n],
            min_steps: vec![usize::MAX; n],
            start,
            rule_names,
            symbols,
            head_targets: vec![Vec::new(); n],
            tail_targets: vec![Vec::new(); n],
        };
        let exprs: Vec<&Expr> = grammar.rules.iter().map(|r| r.body.as_ref().expect("a body")).collect();
        // The rules that derive a finite sequence, by a fixpoint from none.
        let mut productive = vec![false; n];
        loop {
            let next: Vec<bool> = exprs.iter().map(|e| g.productive(e, &productive)).collect();
            if next == productive {
                break;
            }
            productive = next;
        }
        for (r, expr) in exprs.iter().enumerate() {
            if productive[r] {
                g.bodies[r] = g.build(expr, &productive);
            }
        }
        loop {
            let next: Vec<bool> = (0..n).map(|r| g.bodies[r].is_some_and(|b| g.node_nullable(b))).collect();
            if next == g.nullable {
                break;
            }
            g.nullable = next;
        }
        loop {
            let next: Vec<usize> = (0..n).map(|r| g.bodies[r].map_or(usize::MAX, |b| g.node_min_steps(b))).collect();
            if next == g.min_steps {
                break;
            }
            g.min_steps = next;
        }
        for r in 0..n {
            if let Some(body) = g.bodies[r] {
                g.mark_heads(body, true);
                g.mark_tails(body, true);
            }
        }
        for r in 0..n {
            for target in g.references(r).into_iter().filter(|&(_, head, _)| head).map(|(t, _, _)| t) {
                if !g.head_targets[r].contains(&target) {
                    g.head_targets[r].push(target);
                }
            }
        }
        // By rule 7, a tail reference links no loop when one of its two rules
        // lies on a cycle of head references.
        let on_head_cycle: Vec<bool> = (0..n).map(|r| reaches(&g.head_targets, r, r, &[])).collect();
        for r in 0..n {
            let Some(body) = g.bodies[r] else { continue };
            let mut stack = vec![body];
            while let Some(id) = stack.pop() {
                g.children(id, &mut stack);
                if let Node::Reference { rule, tail, .. } = &mut g.nodes[id]
                    && *tail
                {
                    if on_head_cycle[r] || on_head_cycle[*rule] {
                        *tail = false;
                    } else if !g.tail_targets[r].contains(rule) {
                        g.tail_targets[r].push(*rule);
                    }
                }
            }
        }
        g
    }

    fn productive(&self, expr: &Expr, rules: &[bool]) -> bool {
        match expr {
            Expr::Symbol { .. } => true,
            Expr::Reference { name, .. } => rules[self.rule(name)],
            Expr::Sequence { items, .. } => items.iter().all(|i| self.productive(i, rules)),
            Expr::Choice { alternatives, .. } => alternatives.iter().any(|a| self.productive(a, rules)),
            Expr::Optional { .. } => true,
            Expr::Repetition { min, item, .. } => *min == 0 || self.productive(item, rules),
        }
    }

    fn rule(&self, name: &str) -> RuleId {
        self.rule_names.iter().position(|n| n == name).expect("a rule")
    }

    fn push(&mut self, node: Node) -> NodeId {
        self.nodes.push(node);
        self.nodes.len() - 1
    }

    fn is_empty(&self, id: NodeId) -> bool {
        matches!(self.nodes[id], Node::Empty)
    }

    /// The node of an expression without its dead parts, or `None` when the
    /// expression derives no finite sequence.
    fn build(&mut self, expr: &Expr, productive: &[bool]) -> Option<NodeId> {
        Some(match expr {
            Expr::Symbol { value, span } => {
                let symbol = self.symbols.iter().position(|s| s == value).expect("a symbol");
                self.push(Node::Symbol { symbol, span: *span })
            }
            Expr::Reference { name, span } => {
                let rule = self.rule(name);
                if !productive[rule] {
                    return None;
                }
                self.push(Node::Reference { rule, span: *span, head: false, tail: false })
            }
            Expr::Sequence { items, .. } => {
                let mut ids = Vec::new();
                for item in items {
                    let id = self.build(item, productive)?;
                    if !self.is_empty(id) {
                        ids.push(id);
                    }
                }
                match ids.len() {
                    0 => self.push(Node::Empty),
                    1 => ids[0],
                    _ => self.push(Node::Sequence(ids)),
                }
            }
            Expr::Choice { alternatives, .. } => {
                let ids: Vec<NodeId> = alternatives.iter().filter_map(|a| self.build(a, productive)).collect();
                if ids.is_empty() {
                    return None;
                }
                if ids.iter().all(|&id| self.is_empty(id)) {
                    self.push(Node::Empty)
                } else {
                    self.push(Node::Choice(ids))
                }
            }
            Expr::Optional { item, .. } => match self.build(item, productive) {
                Some(id) if !self.is_empty(id) => self.push(Node::Optional(id)),
                _ => self.push(Node::Empty),
            },
            Expr::Repetition { min, max, item, span } => match self.build(item, productive) {
                None if *min > 0 => return None,
                Some(id) if !self.is_empty(id) => {
                    self.push(Node::Repetition { min: *min, max: *max, item: id, span: *span })
                }
                _ => self.push(Node::Empty),
            },
        })
    }

    fn node_nullable(&self, id: NodeId) -> bool {
        match &self.nodes[id] {
            Node::Symbol { .. } => false,
            Node::Reference { rule, .. } => self.nullable[*rule],
            Node::Sequence(items) => items.iter().all(|&i| self.node_nullable(i)),
            Node::Choice(alternatives) => alternatives.iter().any(|&a| self.node_nullable(a)),
            Node::Optional(_) | Node::Empty => true,
            Node::Repetition { min, item, .. } => *min == 0 || self.node_nullable(*item),
        }
    }

    /// The fewest steps that a node derives.
    fn node_min_steps(&self, id: NodeId) -> usize {
        match &self.nodes[id] {
            Node::Symbol { .. } => 1,
            Node::Reference { rule, .. } => self.min_steps[*rule],
            Node::Sequence(items) => items.iter().fold(0, |sum, &i| sum.saturating_add(self.node_min_steps(i))),
            Node::Choice(alternatives) => {
                alternatives.iter().map(|&a| self.node_min_steps(a)).min().expect("an alternative")
            }
            Node::Optional(_) | Node::Empty => 0,
            Node::Repetition { min, item, .. } => self.node_min_steps(*item).saturating_mul(*min as usize),
        }
    }

    /// Marks each reference that can come first in its rule node: each item
    /// before it can match the empty sequence.
    fn mark_heads(&mut self, id: NodeId, first: bool) {
        match self.nodes[id].clone() {
            Node::Reference { .. } => {
                if let Node::Reference { head, .. } = &mut self.nodes[id] {
                    *head = first;
                }
            }
            Node::Sequence(items) => {
                let mut first = first;
                for item in items {
                    self.mark_heads(item, first);
                    first = first && self.node_nullable(item);
                }
            }
            Node::Choice(alternatives) => alternatives.into_iter().for_each(|a| self.mark_heads(a, first)),
            // The body of a repetition cannot match the empty sequence, so only
            // its first pass can come first.
            Node::Optional(item) | Node::Repetition { item, .. } => self.mark_heads(item, first),
            Node::Symbol { .. } | Node::Empty => {}
        }
    }

    /// Marks each reference after which its rule body must end. `last` says
    /// whether nothing can follow the node in its rule.
    fn mark_tails(&mut self, id: NodeId, last: bool) {
        match self.nodes[id].clone() {
            Node::Reference { .. } => {
                if let Node::Reference { tail, .. } = &mut self.nodes[id] {
                    *tail = last;
                }
            }
            Node::Sequence(items) => {
                let n = items.len();
                for (t, &item) in items.iter().enumerate() {
                    self.mark_tails(item, last && t + 1 == n);
                }
            }
            Node::Choice(alternatives) => alternatives.into_iter().for_each(|a| self.mark_tails(a, last)),
            Node::Optional(item) => self.mark_tails(item, last),
            // A new pass can follow each pass, unless the bound allows one pass.
            Node::Repetition { max, item, .. } => self.mark_tails(item, last && max == Some(1)),
            Node::Symbol { .. } | Node::Empty => {}
        }
    }

    fn children(&self, id: NodeId, out: &mut Vec<NodeId>) {
        match &self.nodes[id] {
            Node::Sequence(items) | Node::Choice(items) => out.extend(items),
            Node::Optional(item) | Node::Repetition { item, .. } => out.push(*item),
            Node::Symbol { .. } | Node::Reference { .. } | Node::Empty => {}
        }
    }

    /// The references in the body of a rule: the target, and whether the
    /// reference is a head reference and a tail reference.
    fn references(&self, rule: RuleId) -> Vec<(RuleId, bool, bool)> {
        let mut out = Vec::new();
        let mut stack: Vec<NodeId> = self.bodies[rule].into_iter().collect();
        while let Some(id) = stack.pop() {
            self.children(id, &mut stack);
            if let Node::Reference { rule, head, tail, .. } = self.nodes[id] {
                out.push((rule, head, tail));
            }
        }
        out
    }

    fn targets(&self, links: Links) -> &[Vec<RuleId>] {
        match links {
            Links::Head => &self.head_targets,
            Links::Tail => &self.tail_targets,
        }
    }
}

/// Whether `from` reaches `to` by one edge or more, without the rules in
/// `excluded`.
fn reaches(targets: &[Vec<RuleId>], from: RuleId, to: RuleId, excluded: &[RuleId]) -> bool {
    let mut seen = vec![false; targets.len()];
    let mut stack = vec![from];
    while let Some(r) = stack.pop() {
        for &t in &targets[r] {
            if excluded.contains(&t) {
                continue;
            }
            if t == to {
                return true;
            }
            if !seen[t] {
                seen[t] = true;
                stack.push(t);
            }
        }
    }
    false
}

// The loop contexts: a copy of `Contexts` in `src/loops.rs`.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Links {
    Head,
    Tail,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Role {
    Nested,
    Enter,
    Pass(usize),
}

type Roles = (Role, Role);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct LoopDesc {
    links: Links,
    pass_rule: RuleId,
    members: Vec<bool>,
}

/// The loop contexts of one grammar. A context is a stack of head loops and a
/// stack of tail loops, each from the outermost.
#[derive(Default)]
pub struct Contexts {
    loops: Vec<LoopDesc>,
    loop_ids: HashMap<LoopDesc, u32>,
    ctxs: Vec<(Vec<u32>, Vec<u32>)>,
    ctx_ids: HashMap<(Vec<u32>, Vec<u32>), u32>,
}

impl Contexts {
    fn intern_loop(&mut self, desc: LoopDesc) -> u32 {
        if let Some(&id) = self.loop_ids.get(&desc) {
            return id;
        }
        let id = self.loops.len() as u32;
        self.loops.push(desc.clone());
        self.loop_ids.insert(desc, id);
        id
    }

    fn intern(&mut self, head: Vec<u32>, tail: Vec<u32>) -> u32 {
        let key = (head, tail);
        if let Some(&id) = self.ctx_ids.get(&key) {
            return id;
        }
        let id = self.ctxs.len() as u32;
        self.ctxs.push(key.clone());
        self.ctx_ids.insert(key, id);
        id
    }

    fn cycle(g: &Grammar, links: Links, excluded: &[RuleId], rule: RuleId) -> Option<Vec<bool>> {
        let targets = g.targets(links);
        (!excluded.contains(&rule) && reaches(targets, rule, rule, excluded)).then(|| {
            (0..targets.len())
                .map(|r| {
                    !excluded.contains(&r)
                        && (r == rule || (reaches(targets, rule, r, excluded) && reaches(targets, r, rule, excluded)))
                })
                .collect()
        })
    }

    fn child(&mut self, g: &Grammar, links: Links, stack: &[u32], target: RuleId, link: bool) -> (Vec<u32>, Role) {
        let mut base: &[u32] = &[];
        if link && !stack.is_empty() {
            if let Some(k) = stack.iter().position(|&l| self.loops[l as usize].pass_rule == target) {
                return (stack[..=k].to_vec(), Role::Pass(k));
            }
            let k = stack.iter().rposition(|&l| self.loops[l as usize].members[target]).map_or(0, |k| k + 1);
            base = &stack[..k];
        }
        let excluded: Vec<RuleId> = base.iter().map(|&l| self.loops[l as usize].pass_rule).collect();
        let mut out = base.to_vec();
        match Self::cycle(g, links, &excluded, target) {
            Some(members) => {
                out.push(self.intern_loop(LoopDesc { links, pass_rule: target, members }));
                (out, Role::Enter)
            }
            None => (out, Role::Nested),
        }
    }

    /// The context and the roles of the node that a reference makes inside a
    /// node with the context `ctx`. `first` says whether the rule node holds
    /// no step before the reference.
    fn child_ctx(&mut self, g: &Grammar, ctx: u32, reference: &Node, first: bool) -> (u32, Roles) {
        let Node::Reference { rule, head, tail, .. } = *reference else { unreachable!("a reference") };
        let (h, t) = self.ctxs[ctx as usize].clone();
        let (h, head_role) = self.child(g, Links::Head, &h, rule, first && head);
        let (t, tail_role) = self.child(g, Links::Tail, &t, rule, tail);
        (self.intern(h, t), (head_role, tail_role))
    }
}

// The projections.

/// The label of a tree node. The derived order is the order of the spec at
/// equal depth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Label {
    Loop(LoopKind, Option<RuleId>),
    Pass,
    Rule(RuleId),
    Step(usize),
}

/// One node of a tree in pre-order, with its origins in the order of the text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub depth: u32,
    pub label: Label,
    pub origins: Vec<Span>,
}

/// The passes that a node passes up to a loop through rules around it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct Part {
    passes: Vec<Token>,
    link: Vec<Span>,
}

/// A projection: the tokens of an element or a whole node, and the parts that
/// it passes up to the loops of head and tail references, by stack index.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct Proj {
    tokens: Vec<Token>,
    head: Vec<Part>,
    tail: Vec<Part>,
}

type Shape = Vec<(u32, Label)>;

/// The structure of a projection, without its origins.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Key {
    tokens: Shape,
    head: Vec<Shape>,
    tail: Vec<Shape>,
}

fn shape(tokens: &[Token]) -> Shape {
    tokens.iter().map(|t| (t.depth, t.label)).collect()
}

fn parts_shape(parts: &[Part]) -> Vec<Shape> {
    let mut out: Vec<Shape> = parts.iter().map(|p| shape(&p.passes)).collect();
    while out.last().is_some_and(|s| s.is_empty()) {
        out.pop();
    }
    out
}

impl Proj {
    fn key(&self) -> Key {
        Key { tokens: shape(&self.tokens), head: parts_shape(&self.head), tail: parts_shape(&self.tail) }
    }

    fn join(&mut self, other: &Proj) {
        join_tokens(&mut self.tokens, &other.tokens);
        for (x, y) in self.head.iter_mut().zip(&other.head).chain(self.tail.iter_mut().zip(&other.tail)) {
            join_tokens(&mut x.passes, &y.passes);
            union(&mut x.link, &y.link);
        }
        for (mine, theirs) in [(&mut self.head, &other.head), (&mut self.tail, &other.tail)] {
            if mine.len() < theirs.len() {
                mine.extend(theirs[mine.len()..].iter().cloned());
            }
        }
    }

    /// This projection followed by another one in pre-order.
    fn then(&self, other: &Proj) -> Proj {
        let mut out = self.clone();
        out.tokens.extend(other.tokens.iter().cloned());
        merge_parts(&mut out.head, &other.head);
        merge_parts(&mut out.tail, &other.tail);
        out
    }
}

fn union(into: &mut Vec<Span>, from: &[Span]) {
    for span in from {
        if let Err(at) = into.binary_search(span) {
            into.insert(at, *span);
        }
    }
}

fn join_tokens(into: &mut [Token], from: &[Token]) {
    for (x, y) in into.iter_mut().zip(from) {
        union(&mut x.origins, &y.origins);
    }
}

fn merge_parts(into: &mut Vec<Part>, from: &[Part]) {
    if into.len() < from.len() {
        into.resize_with(from.len(), Part::default);
    }
    for (k, part) in from.iter().enumerate() {
        into[k].passes.extend(part.passes.iter().cloned());
        union(&mut into[k].link, &part.link);
    }
}

fn shifted(tokens: &[Token], by: u32) -> impl Iterator<Item = Token> + '_ {
    tokens.iter().map(move |t| Token { depth: t.depth + by, ..t.clone() })
}

/// A set of projections. Equal projections join their origins.
#[derive(Debug, Default, PartialEq, Eq)]
struct Set(BTreeMap<Key, Proj>);

// The work of the oracle.

/// The readings of one prefix: each tree, whether it can end, in the order of
/// the spec.
#[derive(Debug)]
pub struct Readings {
    pub trees: Vec<Vec<Token>>,
    pub can_end: Vec<bool>,
}

struct OpenEntry {
    value: Rc<Set>,
    round: u32,
    busy: bool,
}

/// The evaluation of the oracle over one prefix.
struct Oracle<'a> {
    g: &'a Grammar,
    cx: &'a mut Contexts,
    /// The symbol of each step.
    w: &'a [usize],
    complete: HashMap<(NodeId, usize, usize, u32, bool), Rc<Set>>,
    busy: HashSet<(NodeId, usize, usize, u32, bool)>,
    open: HashMap<(NodeId, usize, u32, bool), OpenEntry>,
    round: u32,
    changed: bool,
    cyclic: bool,
    overflow: bool,
}

/// The rendered node of a rule: a copy of `render` in `src/forest.rs`.
fn render(cx: &Contexts, p: &Proj, rule: RuleId, ctx: u32, roles: Roles, origin: &[Span], root: bool) -> Proj {
    let (head_len, tail_len) = (cx.ctxs[ctx as usize].0.len(), cx.ctxs[ctx as usize].1.len());
    let part = |parts: &[Part], k: usize| parts.get(k).cloned().unwrap_or_default();
    let pad = |parts: &[Part], n: usize| (0..n).map(|k| part(parts, k)).collect::<Vec<Part>>();
    let node = |origins: &[Span]| -> Vec<Token> {
        if p.tokens.is_empty() {
            return Vec::new();
        }
        let mut tokens = vec![Token { depth: 0, label: Label::Rule(rule), origins: origins.to_vec() }];
        tokens.extend(shifted(&p.tokens, 1));
        tokens
    };
    let pass = |origins: &[Span], node: &[Token]| -> Vec<Token> {
        if node.is_empty() {
            return Vec::new();
        }
        let mut tokens = vec![Token { depth: 0, label: Label::Pass, origins: origins.to_vec() }];
        tokens.extend(shifted(node, 1));
        tokens
    };
    let wrap = |kind: LoopKind, mut passes: Vec<Token>| -> Vec<Token> {
        if passes.is_empty() {
            return passes;
        }
        passes[0].origins = Vec::new();
        passes[1].origins = origin.to_vec();
        let mut tokens = vec![Token { depth: 0, label: Label::Loop(kind, Some(rule)), origins: origin.to_vec() }];
        tokens.extend(shifted(&passes, 1));
        tokens
    };
    let mut out = Proj { tokens: Vec::new(), head: pad(&p.head, head_len), tail: pad(&p.tail, tail_len) };
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
        out.tokens = vec![Token { depth: 0, label: Label::Rule(rule), origins: Vec::new() }];
    }
    out
}

/// The tokens of a repetition from its passes: a loop node with one pass node
/// for each pass that holds a step. A loop without a step makes no node.
fn pass_tokens(pass: &Proj) -> Proj {
    let mut out = Proj { tokens: Vec::new(), head: pass.head.clone(), tail: pass.tail.clone() };
    if !pass.tokens.is_empty() {
        out.tokens.push(Token { depth: 0, label: Label::Pass, origins: Vec::new() });
        out.tokens.extend(shifted(&pass.tokens, 1));
    }
    out
}

fn loop_tokens(passes: &Proj, span: Span) -> Proj {
    let mut out = Proj { tokens: Vec::new(), head: passes.head.clone(), tail: passes.tail.clone() };
    if !passes.tokens.is_empty() {
        out.tokens.push(Token { depth: 0, label: Label::Loop(LoopKind::Repetition, None), origins: vec![span] });
        out.tokens.extend(shifted(&passes.tokens, 1));
    }
    out
}

impl Oracle<'_> {
    fn n(&self) -> usize {
        self.w.len()
    }

    fn insert(&mut self, set: &mut Set, p: Proj) {
        let key = p.key();
        match set.0.get_mut(&key) {
            Some(q) => q.join(&p),
            None => {
                set.0.insert(key, p);
                if set.0.len() > MAX_TREES {
                    self.overflow = true;
                }
            }
        }
    }

    fn single(p: Proj) -> Set {
        Set(BTreeMap::from([(p.key(), p)]))
    }

    /// Each projection of `a` followed by each projection of `b`.
    fn product(&mut self, out: &mut Set, a: &Set, b: &Set) {
        for p in a.0.values() {
            for q in b.0.values() {
                if self.overflow {
                    return;
                }
                let r = p.then(q);
                self.insert(out, r);
            }
        }
    }

    /// The rendered node of a reference, with the link that it gives to the
    /// pass of a loop of head references, as `combine` in `src/forest.rs`.
    fn reference(&mut self, set: &Set, id: NodeId, ctx: u32, first: bool, out: &mut Set, root: bool) {
        let reference = self.g.nodes[id].clone();
        let Node::Reference { rule, span, .. } = reference else { unreachable!("a reference") };
        let (child_ctx, roles) = self.cx.child_ctx(self.g, ctx, &reference, first);
        for p in set.0.values() {
            let mut q = render(self.cx, p, rule, child_ctx, roles, &[span], root);
            if let Role::Pass(k) = roles.0
                && q.head.get(k).is_some_and(|part| !part.passes.is_empty())
            {
                union(&mut q.head[k].link, &[span]);
            }
            self.insert(out, q);
        }
    }

    fn reference_ctx(&mut self, id: NodeId, ctx: u32, first: bool) -> (RuleId, u32) {
        let reference = self.g.nodes[id].clone();
        let Node::Reference { rule, .. } = reference else { unreachable!("a reference") };
        (rule, self.cx.child_ctx(self.g, ctx, &reference, first).0)
    }

    /// The projections of the complete derivations of a node over the steps
    /// i+1 to j. `first` says whether the rule node holds no step before i.
    fn complete(&mut self, id: NodeId, i: usize, j: usize, ctx: u32, first: bool) -> Rc<Set> {
        let key = (id, i, j, ctx, first);
        if let Some(found) = self.complete.get(&key) {
            return found.clone();
        }
        // A node never takes fewer steps than its least, so a rule never asks
        // for its own set over the same steps. The rule checks reject a rule
        // that can derive itself with no step.
        assert!(self.busy.insert(key), "the complete sets have a cycle");
        let mut out = Set::default();
        if !self.overflow && j - i >= self.g.node_min_steps(id) {
            match self.g.nodes[id].clone() {
                Node::Symbol { symbol, span } => {
                    if j == i + 1 && self.w[i] == symbol {
                        let step = Token { depth: 0, label: Label::Step(j), origins: vec![span] };
                        out = Self::single(Proj { tokens: vec![step], ..Proj::default() });
                    }
                }
                Node::Empty => {
                    if i == j {
                        out = Self::single(Proj::default());
                    }
                }
                Node::Reference { .. } => {
                    let (rule, child_ctx) = self.reference_ctx(id, ctx, first);
                    let body = self.g.bodies[rule].expect("a productive rule");
                    let set = self.complete(body, i, j, child_ctx, true);
                    self.reference(&set, id, ctx, first, &mut out, false);
                }
                Node::Sequence(items) => {
                    let mut reached: BTreeMap<usize, Set> = BTreeMap::from([(i, Self::single(Proj::default()))]);
                    for (t, &item) in items.iter().enumerate() {
                        // The items after this one need their fewest steps.
                        let rest = items[t + 1..].iter().map(|&r| self.g.node_min_steps(r)).sum::<usize>();
                        let mut next: BTreeMap<usize, Set> = BTreeMap::new();
                        for (h, before) in &reached {
                            for k in *h..=j.saturating_sub(rest) {
                                let set = self.complete(item, *h, k, ctx, first && *h == i);
                                if !set.0.is_empty() {
                                    let mut into = next.remove(&k).unwrap_or_default();
                                    self.product(&mut into, before, &set);
                                    next.insert(k, into);
                                }
                            }
                        }
                        reached = next;
                    }
                    out = reached.remove(&j).unwrap_or_default();
                }
                Node::Choice(alternatives) => {
                    for a in alternatives {
                        let set = self.complete(a, i, j, ctx, first);
                        for p in set.0.values() {
                            self.insert(&mut out, p.clone());
                        }
                    }
                }
                Node::Optional(item) => {
                    let set = self.complete(item, i, j, ctx, first);
                    for p in set.0.values() {
                        self.insert(&mut out, p.clone());
                    }
                    if i == j {
                        self.insert(&mut out, Proj::default());
                    }
                }
                Node::Repetition { min, span, .. } => {
                    for (_, passes) in self.passes(id, i, j, ctx, first, true).into_iter().filter(|(t, _)| *t >= min) {
                        for p in passes.0.values() {
                            let q = loop_tokens(p, span);
                            self.insert(&mut out, q);
                        }
                    }
                }
            }
        }
        self.busy.remove(&key);
        let out = Rc::new(out);
        self.complete.insert(key, out.clone());
        out
    }

    /// The complete passes of a repetition from i to j, by their count. The
    /// count stops at the least number of an open bound. Each projection holds
    /// the pass nodes, without the loop node. `complete` says whether the
    /// repetition ends at j, so that each pass leaves the steps that the least
    /// number of passes needs.
    fn passes(&mut self, id: NodeId, i: usize, j: usize, ctx: u32, first: bool, complete: bool) -> BTreeMap<u32, Set> {
        let Node::Repetition { min, max, item, .. } = self.g.nodes[id] else { unreachable!("a repetition") };
        // The states: the end of the passes so far and their count.
        let mut states: BTreeMap<(usize, u32), Set> = BTreeMap::from([((i, 0), Self::single(Proj::default()))]);
        let mut done: BTreeMap<u32, Set> = BTreeMap::new();
        while let Some(((h, t), before)) = states.pop_first() {
            if h == j {
                let mut into = done.remove(&t).unwrap_or_default();
                for p in before.0.values() {
                    self.insert(&mut into, p.clone());
                }
                done.insert(t, into);
            }
            if max.is_some_and(|max| t >= max) {
                continue;
            }
            let next_t = if max.is_none() { (t + 1).min(min) } else { t + 1 };
            // The body of a repetition cannot match the empty sequence, so each
            // pass holds a step.
            let rest = if complete { min.saturating_sub(t + 1) as usize * self.g.node_min_steps(item) } else { 0 };
            for k in h + 1..=j.saturating_sub(rest) {
                let set = self.complete(item, h, k, ctx, first && h == i);
                if set.0.is_empty() {
                    continue;
                }
                let wrapped: Vec<Proj> = set.0.values().map(pass_tokens).collect();
                let mut into = states.remove(&(k, next_t)).unwrap_or_default();
                for p in before.0.values() {
                    for q in &wrapped {
                        if self.overflow {
                            return BTreeMap::new();
                        }
                        let r = p.then(q);
                        self.insert(&mut into, r);
                    }
                }
                states.insert((k, next_t), into);
            }
        }
        done
    }

    /// The projections of the derivations of a node from i that cover the
    /// steps i+1 to n and stay open after the last step. They include the
    /// complete derivations over those steps.
    fn open(&mut self, id: NodeId, i: usize, ctx: u32, first: bool) -> Rc<Set> {
        let key = (id, i, ctx, first);
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
                let value = Rc::new(Set::default());
                self.open.insert(key, OpenEntry { value: value.clone(), round: self.round, busy: true });
                value
            }
        };
        let n = self.n();
        let mut out = Set::default();
        if !self.overflow {
            match self.g.nodes[id].clone() {
                Node::Symbol { .. } | Node::Empty => {
                    if i == n {
                        self.insert(&mut out, Proj::default());
                    }
                    let set = self.complete(id, i, n, ctx, first);
                    for p in set.0.values() {
                        self.insert(&mut out, p.clone());
                    }
                }
                Node::Reference { .. } => {
                    let (rule, child_ctx) = self.reference_ctx(id, ctx, first);
                    let body = self.g.bodies[rule].expect("a productive rule");
                    let set = self.open(body, i, child_ctx, true);
                    self.reference(&set, id, ctx, first, &mut out, false);
                }
                Node::Sequence(items) => {
                    // The items before item t are complete, item t is open, and
                    // the items after it have not started.
                    let mut reached: BTreeMap<usize, Set> = BTreeMap::from([(i, Self::single(Proj::default()))]);
                    for item in items {
                        for (h, before) in &reached {
                            let set = self.open(item, *h, ctx, first && *h == i);
                            self.product(&mut out, before, &set);
                        }
                        let mut next: BTreeMap<usize, Set> = BTreeMap::new();
                        for (h, before) in &reached {
                            for k in *h..=n {
                                let set = self.complete(item, *h, k, ctx, first && *h == i);
                                if !set.0.is_empty() {
                                    let mut into = next.remove(&k).unwrap_or_default();
                                    self.product(&mut into, before, &set);
                                    next.insert(k, into);
                                }
                            }
                        }
                        reached = next;
                    }
                }
                Node::Choice(alternatives) => {
                    for a in alternatives {
                        let set = self.open(a, i, ctx, first);
                        for p in set.0.values() {
                            self.insert(&mut out, p.clone());
                        }
                    }
                }
                Node::Optional(item) => {
                    let set = self.open(item, i, ctx, first);
                    for p in set.0.values() {
                        self.insert(&mut out, p.clone());
                    }
                    if i == n {
                        self.insert(&mut out, Proj::default());
                    }
                }
                Node::Repetition { max, item, span, .. } => {
                    let complete = self.complete(id, i, n, ctx, first);
                    for p in complete.0.values() {
                        self.insert(&mut out, p.clone());
                    }
                    // Complete passes up to some step h, then one open pass.
                    for h in i..=n {
                        for (t, before) in self.passes(id, i, h, ctx, first, false) {
                            // An open bound counts to its least number only.
                            if max.is_some_and(|max| t >= max) {
                                continue;
                            }
                            let set = self.open(item, h, ctx, first && h == i);
                            for p in before.0.values() {
                                for q in set.0.values() {
                                    if self.overflow {
                                        break;
                                    }
                                    let r = loop_tokens(&p.then(&pass_tokens(q)), span);
                                    self.insert(&mut out, r);
                                }
                            }
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

/// The readings of a prefix, or `None` when a set would hold more than
/// `MAX_TREES` trees.
pub fn readings(g: &Grammar, cx: &mut Contexts, w: &[usize]) -> Option<Readings> {
    let Some(body) = g.bodies[g.start] else {
        return Some(Readings { trees: Vec::new(), can_end: Vec::new() });
    };
    let (head, head_role) = cx.child(g, Links::Head, &[], g.start, false);
    let (tail, tail_role) = cx.child(g, Links::Tail, &[], g.start, false);
    let ctx = cx.intern(head, tail);
    let roles = (head_role, tail_role);
    let mut oracle = Oracle {
        g,
        cx,
        w,
        complete: HashMap::new(),
        busy: HashSet::new(),
        open: HashMap::new(),
        round: 0,
        changed: false,
        cyclic: false,
        overflow: false,
    };
    let mut value = Rc::new(Set::default());
    for round in 0.. {
        assert!(round < 1_000, "the open sets keep growing");
        oracle.round += 1;
        oracle.changed = false;
        oracle.cyclic = false;
        value = oracle.open(body, 0, ctx, true);
        if oracle.overflow {
            return None;
        }
        if !oracle.cyclic || !oracle.changed {
            break;
        }
    }
    let ends = oracle.complete(body, 0, w.len(), ctx, true);
    if oracle.overflow {
        return None;
    }
    let root = |p: &Proj| render(oracle.cx, p, g.start, ctx, roles, &[], true);
    let mut trees: Vec<Vec<Token>> = Vec::new();
    for p in value.0.values() {
        let tokens = root(p).tokens;
        match trees.iter_mut().find(|t| compare(t, &tokens) == Ordering::Equal) {
            Some(t) => join_tokens(t, &tokens),
            None => trees.push(tokens),
        }
    }
    trees.sort_by(|a, b| compare(a, b));
    let end_trees: Vec<Vec<Token>> = ends.0.values().map(|p| root(p).tokens).collect();
    let can_end = trees.iter().map(|t| end_trees.iter().any(|e| compare(e, t) == Ordering::Equal)).collect();
    Some(Readings { trees, can_end })
}

/// The tree without step k and the nodes that hold only step k, as
/// `without_last_step` in `src/forest.rs`.
pub fn without_last_step(tokens: &[Token], k: usize, start: RuleId) -> Vec<Token> {
    let last = tokens.iter().rposition(|t| matches!(t.label, Label::Step(s) if s < k));
    match last {
        Some(last) => tokens[..=last].to_vec(),
        None => vec![Token { depth: 0, label: Label::Rule(start), origins: Vec::new() }],
    }
}

pub fn same_tree(a: &[Token], b: &[Token]) -> bool {
    compare(a, b) == Ordering::Equal
}

impl Grammar {
    pub fn start(&self) -> RuleId {
        self.start
    }
}
