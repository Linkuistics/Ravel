//! The loops through rules: `docs/specs/output-view.md`, section "Passes in
//! loops through rules", and `docs/specs/tracer.md`, sections "Loops through
//! rules" and "Loop contexts".
//!
//! A head reference can come first in its rule node. A tail reference ends its
//! rule body. A loop of head references follows a cycle of the graph of head
//! references. A loop of tail references follows a cycle of the graph of tail
//! references without the rules that lie on a cycle of head references, by
//! rule 7 of the output-view spec. Inside one pass, a nested loop avoids the
//! pass rule of each loop around it, so its cycle lies in the graph without
//! those rules.

use std::{collections::HashMap, rc::Rc};

use crate::automaton::{Automata, END, PositionKind, RuleId, START};

/// The kind of a loop through rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) enum Links {
    Head,
    Tail,
}

/// The references of a grammar that can link a loop through rules.
#[derive(Debug)]
pub(crate) struct LoopGraphs {
    /// For each rule and position, whether the position is a head reference.
    pub head: Vec<Vec<bool>>,
    /// For each rule and position, whether the position is a tail reference
    /// between two rules that lie on no cycle of head references.
    pub tail: Vec<Vec<bool>>,
    head_targets: Vec<Vec<RuleId>>,
    tail_targets: Vec<Vec<RuleId>>,
}

impl LoopGraphs {
    pub fn new(automata: &Automata) -> Self {
        let n = automata.rules.len();
        let mut head = Vec::with_capacity(n);
        let mut tail = Vec::with_capacity(n);
        let mut head_targets = vec![Vec::new(); n];
        for (r, rule) in automata.rules.iter().enumerate() {
            // A head reference: a path from the start state reaches it through
            // references to nullable rules only.
            let mut reached = vec![false; rule.states.len()];
            let mut stack = vec![START];
            let mut heads = vec![false; rule.positions.len()];
            reached[START] = true;
            while let Some(state) = stack.pop() {
                for edge in &rule.edges[state] {
                    let Some(p) = rule.states[edge.target].position else { continue };
                    if let PositionKind::Reference(target) = rule.positions[p].kind {
                        heads[p] = true;
                        if automata.nullable[target] && !reached[edge.target] {
                            reached[edge.target] = true;
                            stack.push(edge.target);
                        }
                    }
                }
            }
            for (p, position) in rule.positions.iter().enumerate() {
                if let (true, PositionKind::Reference(target)) = (heads[p], position.kind)
                    && !head_targets[r].contains(&target)
                {
                    head_targets[r].push(target);
                }
            }
            head.push(heads);
            // A tail reference: every edge out of every state at it leads to
            // the end state.
            let mut tails: Vec<bool> =
                rule.positions.iter().map(|p| matches!(p.kind, PositionKind::Reference(_))).collect();
            for (s, state) in rule.states.iter().enumerate() {
                if let Some(p) = state.position
                    && rule.edges[s].iter().any(|e| e.target != END)
                {
                    tails[p] = false;
                }
            }
            tail.push(tails);
        }
        let on_head_cycle: Vec<bool> = (0..n).map(|r| reaches(&head_targets, r, r, &[])).collect();
        let mut tail_targets = vec![Vec::new(); n];
        for (r, rule) in automata.rules.iter().enumerate() {
            for (p, position) in rule.positions.iter().enumerate() {
                if let PositionKind::Reference(target) = position.kind
                    && tail[r][p]
                {
                    if on_head_cycle[r] || on_head_cycle[target] {
                        tail[r][p] = false;
                    } else if !tail_targets[r].contains(&target) {
                        tail_targets[r].push(target);
                    }
                }
            }
        }
        Self { head, tail, head_targets, tail_targets }
    }

    /// The kind of loop through rules that the reference at a position links,
    /// if any. A head reference links a loop of head references when its
    /// target reaches its rule by head references. A tail reference links a
    /// loop of tail references in the same way. The graph of tail references
    /// already leaves out the rules on a cycle of head references, by rule 3 of
    /// the tracer spec. So a reference links at most one kind of loop.
    pub fn link(&self, automata: &Automata, rule: RuleId, position: usize) -> Option<Links> {
        let PositionKind::Reference(target) = automata.rules[rule].positions[position].kind else {
            return None;
        };
        if self.head[rule][position] && reaches(&self.head_targets, target, rule, &[]) {
            Some(Links::Head)
        } else if self.tail[rule][position] && reaches(&self.tail_targets, target, rule, &[]) {
            Some(Links::Tail)
        } else {
            None
        }
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

/// A loop through rules in a loop context: its kind, its pass rule, and the
/// rules of its cycles.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct LoopDesc {
    pub links: Links,
    pub pass_rule: RuleId,
    pub members: Rc<[bool]>,
}

/// How a node takes part in the loops of one kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum Role {
    /// The node is no pass node of a loop of this kind.
    Nested,
    /// The node enters the innermost loop of its stack, as its pass rule.
    Enter,
    /// The node starts a later pass of the loop at this index of its stack.
    Pass(usize),
}

/// A kind of link, the rules left out of the graph, and a rule.
type CycleKey = (Links, Vec<RuleId>, RuleId);

/// The loop contexts of one evaluation. A stack names the loops of one kind
/// that hold a node on their chain of links, from the outermost.
#[derive(Default)]
pub(crate) struct Contexts {
    pub loops: Vec<LoopDesc>,
    loop_ids: HashMap<LoopDesc, u32>,
    cycles: HashMap<CycleKey, Option<Rc<[bool]>>>,
}

impl Contexts {
    fn intern(&mut self, desc: LoopDesc) -> u32 {
        if let Some(&id) = self.loop_ids.get(&desc) {
            return id;
        }
        let id = self.loops.len() as u32;
        self.loops.push(desc.clone());
        self.loop_ids.insert(desc, id);
        id
    }

    /// The rules on a cycle with `rule`, in the graph without `excluded`, or
    /// `None` when `rule` lies on no cycle there.
    fn cycle(&mut self, graphs: &LoopGraphs, links: Links, excluded: Vec<RuleId>, rule: RuleId) -> Option<Rc<[bool]>> {
        let key = (links, excluded, rule);
        if let Some(found) = self.cycles.get(&key) {
            return found.clone();
        }
        let targets = graphs.targets(links);
        let excluded = &key.1;
        let found = (!excluded.contains(&rule) && reaches(targets, rule, rule, excluded)).then(|| {
            (0..targets.len())
                .map(|r| {
                    !excluded.contains(&r)
                        && (r == rule || (reaches(targets, rule, r, excluded) && reaches(targets, r, rule, excluded)))
                })
                .collect::<Rc<[bool]>>()
        });
        self.cycles.insert(key, found.clone());
        found
    }

    /// The stack and the role of the node of `target`, which a reference makes
    /// inside a node with the stack `stack`. `link` says whether the reference
    /// can link a loop of this kind: a head reference with no step before it
    /// in its rule node, or a tail reference.
    pub fn child(
        &mut self,
        graphs: &LoopGraphs,
        links: Links,
        stack: &[u32],
        target: RuleId,
        link: bool,
    ) -> (Vec<u32>, Role) {
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
        match self.cycle(graphs, links, excluded, target) {
            Some(members) => {
                out.push(self.intern(LoopDesc { links, pass_rule: target, members }));
                (out, Role::Enter)
            }
            None => (out, Role::Nested),
        }
    }
}
