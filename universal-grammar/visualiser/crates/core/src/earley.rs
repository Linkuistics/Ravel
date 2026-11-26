//! The Earley sets of the tracer: `docs/specs/tracer.md`, sections
//! "Recognition" and "Next options".
//!
//! The tracer keeps one set for each count of steps read. An item names a rule,
//! a state of the automaton of that rule, and its origin. Prediction, scan and
//! completion follow the edges of the automata. A rule that can match the empty
//! sequence completes as soon as the tracer predicts it, by the method of
//! Aycock and Horspool (https://doi.org/10.1093/comjnl/45.6.620). So a caller
//! that joins a set after such a completion still moves past the reference.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use crate::{
    automaton::{Automata, END, PositionKind, RuleId, START, StateId, SymbolId},
    checked::Span,
};

/// An Earley item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct Item {
    pub rule: RuleId,
    pub state: StateId,
    /// The number of steps before the rule node of the item started.
    pub origin: usize,
}

/// One Earley set.
#[derive(Debug, Default, Clone)]
pub(crate) struct EarleySet {
    pub items: Vec<Item>,
    seen: HashSet<Item>,
    /// For each rule, the items that move past a reference to it when it
    /// completes with this set as its origin.
    waiting: HashMap<RuleId, Vec<Item>>,
}

impl EarleySet {
    fn add(&mut self, item: Item) {
        if self.seen.insert(item) {
            self.items.push(item);
        }
    }
}

/// The Earley sets for the steps read, from set 0 for the empty prefix.
#[derive(Debug, Clone)]
pub(crate) struct Earley {
    sets: Vec<EarleySet>,
}

impl Earley {
    /// The sets before the first step.
    pub fn new(automata: &Automata) -> Self {
        let mut set = EarleySet::default();
        set.add(Item { rule: automata.start, state: START, origin: 0 });
        let mut earley = Self { sets: vec![set] };
        earley.close(automata, 0);
        earley
    }

    /// The number of steps read.
    pub fn steps_read(&self) -> usize {
        self.sets.len() - 1
    }

    /// Reads one step. When no item accepts the symbol, the sets stay as they
    /// were and the result is false.
    pub fn read(&mut self, automata: &Automata, symbol: &str) -> bool {
        let Some(symbol) = automata.symbol(symbol) else {
            return false;
        };
        let k = self.steps_read();
        let mut next = EarleySet::default();
        for item in &self.sets[k].items {
            if item.state == END {
                continue;
            }
            let rule = &automata.rules[item.rule];
            for edge in &rule.edges[item.state] {
                if matches!(rule.position(edge.target), Some(p) if p.kind == PositionKind::Symbol(symbol)) {
                    next.add(Item { state: edge.target, ..*item });
                }
            }
        }
        if next.items.is_empty() {
            return false;
        }
        self.sets.push(next);
        self.close(automata, k + 1);
        true
    }

    /// Drops the last set. It returns false before the first step.
    pub fn back(&mut self) -> bool {
        if self.sets.len() > 1 {
            self.sets.pop();
            true
        } else {
            false
        }
    }

    /// Adds the predictions and the completions of set k.
    fn close(&mut self, automata: &Automata, k: usize) {
        let mut i = 0;
        while i < self.sets[k].items.len() {
            let item = self.sets[k].items[i];
            i += 1;
            if item.state == END {
                let callers = self.sets[item.origin].waiting.get(&item.rule).cloned().unwrap_or_default();
                for caller in callers {
                    self.sets[k].add(caller);
                }
                continue;
            }
            let rule = &automata.rules[item.rule];
            for edge in &rule.edges[item.state] {
                match rule.position(edge.target).map(|p| p.kind) {
                    None => self.sets[k].add(Item { state: END, ..item }),
                    Some(PositionKind::Symbol(_)) => {}
                    Some(PositionKind::Reference(target)) => {
                        let moved = Item { state: edge.target, ..item };
                        let set = &mut self.sets[k];
                        set.waiting.entry(target).or_default().push(moved);
                        set.add(Item { rule: target, state: START, origin: k });
                        if automata.nullable[target] {
                            set.add(moved);
                        }
                    }
                }
            }
        }
    }

    /// Whether set k holds an item.
    pub fn contains(&self, k: usize, item: Item) -> bool {
        self.sets[k].seen.contains(&item)
    }

    /// The items of set k.
    pub fn items(&self, k: usize) -> &[Item] {
        &self.sets[k].items
    }

    /// Whether the sequence can end after the steps read.
    pub fn can_end(&self, automata: &Automata, k: usize) -> bool {
        self.sets[k].seen.contains(&Item { rule: automata.start, state: END, origin: 0 })
    }

    /// The next options after k steps: each symbol on an edge out of a state of
    /// set k, with the occurrences that offer it. Symbols come in the order of
    /// the checked grammar, and occurrences in the order of the text.
    pub fn next_options(&self, automata: &Automata, k: usize) -> Vec<(SymbolId, Vec<Span>)> {
        let mut options: BTreeMap<SymbolId, BTreeSet<Span>> = BTreeMap::new();
        for item in &self.sets[k].items {
            if item.state == END {
                continue;
            }
            let rule = &automata.rules[item.rule];
            for edge in &rule.edges[item.state] {
                if let Some(position) = rule.position(edge.target)
                    && let PositionKind::Symbol(symbol) = position.kind
                {
                    options.entry(symbol).or_default().insert(position.span);
                }
            }
        }
        options.into_iter().map(|(s, spans)| (s, spans.into_iter().collect())).collect()
    }
}
