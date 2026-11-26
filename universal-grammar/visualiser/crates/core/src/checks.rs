//! The rule checks: the check diagnostics of `docs/specs/grammar-notation.md`,
//! section "Diagnostics", and the badges of the recursive groups.
//!
//! The checks read the rules that the lowering kept, so they also run after a
//! syntax error. A rule without a body takes part in each check as a rule that
//! refers to nothing and matches nothing. `Recovery` names the parts that the
//! lowering could not read. With it, a syntax error does not make a check
//! report a fault that the author did not write.

use std::collections::HashMap;

use crate::{
    checked::{Badge, CheckedGrammar, Expr, GroupMember, RecursiveGroup, Rule},
    diagnostic::{Code, RawDiagnostic, Span},
};

/// The bound cap of the section "The bound cap" of the notation spec.
pub const BOUND_CAP: u64 = 100;

/// The results of the rule checks.
pub struct Checks {
    /// The check diagnostics, in the order of the checks.
    pub diagnostics: Vec<RawDiagnostic>,
    pub recursive_groups: Vec<RecursiveGroup>,
}

/// The parts of a grammar that the lowering could not read after a syntax
/// error.
#[derive(Debug, Default)]
pub struct Recovery {
    /// The spans of the items whose bound is malformed. The lowering keeps each
    /// such item without its bound, so its number of passes is unknown.
    pub unread_bounds: Vec<Span>,
    /// Whether a `%start` declaration has no name that the lowering can read.
    pub nameless_start: bool,
}

/// Runs every rule check on a checked grammar.
pub fn check(grammar: &CheckedGrammar, recovery: &Recovery) -> Checks {
    let rules = RuleIndex::new(&grammar.rules);
    let nullable = rules.nullable();
    let mut diagnostics = Vec::new();
    check_declarations(grammar, recovery, &rules, &mut diagnostics);
    check_rule_names(&rules, &mut diagnostics);
    check_references(&rules, &mut diagnostics);
    check_unused(grammar, &rules, &mut diagnostics);
    check_repetitions(&rules, &nullable, &mut diagnostics);
    check_cycles(&rules, &nullable, &recovery.unread_bounds, &mut diagnostics);
    check_symbols(grammar, &mut diagnostics);
    Checks { diagnostics, recursive_groups: recursive_groups(&rules) }
}

/// The rules by name. Each name has one index, in the order of its first rule.
/// A name that has more than one rule keeps all its rules, so that each check
/// reads every body.
struct RuleIndex<'a> {
    names: Vec<&'a str>,
    by_name: HashMap<&'a str, usize>,
    rules: Vec<Vec<&'a Rule>>,
}

impl<'a> RuleIndex<'a> {
    fn new(all: &'a [Rule]) -> Self {
        let mut index = Self { names: Vec::new(), by_name: HashMap::new(), rules: Vec::new() };
        for rule in all {
            let i = *index.by_name.entry(&rule.name).or_insert_with(|| {
                index.names.push(&rule.name);
                index.rules.push(Vec::new());
                index.names.len() - 1
            });
            index.rules[i].push(rule);
        }
        index
    }

    fn len(&self) -> usize {
        self.names.len()
    }

    fn get(&self, name: &str) -> Option<usize> {
        self.by_name.get(name).copied()
    }

    /// The span of the first rule of a name.
    fn span(&self, i: usize) -> Span {
        self.rules[i][0].span
    }

    /// The bodies of the rules of a name.
    fn bodies(&self, i: usize) -> impl Iterator<Item = &'a Expr> + '_ {
        self.rules[i].iter().filter_map(|rule| rule.body.as_ref())
    }

    /// Whether each name can match the empty sequence, by a fixed point. A
    /// work queue checks a rule again only when a rule that it refers to
    /// becomes nullable. Repeated passes over all rules cost time in the square
    /// of the rule count on a long chain of rules.
    fn nullable(&self) -> Vec<bool> {
        let mut users: Vec<Vec<usize>> = vec![Vec::new(); self.len()];
        for i in 0..self.len() {
            for body in self.bodies(i) {
                for_each_reference(body, &mut |name, _| {
                    if let Some(j) = self.get(name) {
                        users[j].push(i);
                    }
                });
            }
        }
        let mut nullable = vec![false; self.len()];
        let mut queued = vec![true; self.len()];
        let mut pending: std::collections::VecDeque<usize> = (0..self.len()).collect();
        while let Some(i) = pending.pop_front() {
            queued[i] = false;
            if nullable[i] || !self.bodies(i).any(|body| self.can_be_empty(body, &nullable)) {
                continue;
            }
            nullable[i] = true;
            for &user in &users[i] {
                if !nullable[user] && !std::mem::replace(&mut queued[user], true) {
                    pending.push_back(user);
                }
            }
        }
        nullable
    }

    /// Whether an expression can match the empty sequence. A reference without
    /// a rule matches nothing.
    fn can_be_empty(&self, expr: &Expr, nullable: &[bool]) -> bool {
        match expr {
            Expr::Symbol { .. } => false,
            Expr::Reference { name, .. } => self.get(name).is_some_and(|i| nullable[i]),
            Expr::Sequence { items, .. } => items.iter().all(|item| self.can_be_empty(item, nullable)),
            Expr::Choice { alternatives, .. } => alternatives.iter().any(|alt| self.can_be_empty(alt, nullable)),
            Expr::Optional { .. } => true,
            Expr::Repetition { min, item, .. } => *min == 0 || self.can_be_empty(item, nullable),
        }
    }
}

/// Calls `f` for each reference in an expression, in the order of the text.
fn for_each_reference<'a>(expr: &'a Expr, f: &mut impl FnMut(&'a str, Span)) {
    match expr {
        Expr::Symbol { .. } => {}
        Expr::Reference { name, span } => f(name, *span),
        Expr::Sequence { items, .. } | Expr::Choice { alternatives: items, .. } => {
            items.iter().for_each(|item| for_each_reference(item, f));
        }
        Expr::Optional { item, .. } | Expr::Repetition { item, .. } => for_each_reference(item, f),
    }
}

/// The `%doc` and `%start` declarations.
fn check_declarations(grammar: &CheckedGrammar, recovery: &Recovery, rules: &RuleIndex, out: &mut Vec<RawDiagnostic>) {
    for doc in grammar.grammar_docs.iter().skip(1) {
        out.push(RawDiagnostic::error(
            Code::SecondGrammarDoc,
            doc.span,
            "The grammar has more than one `%doc`. Put the whole text into the first `%doc`.",
        ));
    }
    // A `%start` without a name is a syntax error, and that error states the fault.
    if grammar.start_rules.is_empty() && !recovery.nameless_start {
        out.push(RawDiagnostic::error(
            Code::MissingStart,
            Span::at(0),
            "The grammar has no `%start`. Add `%start NAME ;`, where NAME is the start rule.",
        ));
    }
    for start in grammar.start_rules.iter().skip(1) {
        out.push(RawDiagnostic::error(
            Code::SecondStart,
            start.span,
            "The grammar has more than one `%start`. Keep one, because every reading is a tree of one start rule.",
        ));
    }
    for start in &grammar.start_rules {
        if rules.get(&start.name).is_none() {
            out.push(RawDiagnostic::error(
                Code::UndefinedStartRule,
                start.span,
                format!("`%start` names `{}`, but no rule has that name.", start.name),
            ));
        }
    }
}

/// Rules that are defined twice, and rule names that differ only by case.
fn check_rule_names(rules: &RuleIndex, out: &mut Vec<RawDiagnostic>) {
    for (i, name) in rules.names.iter().enumerate() {
        for rule in rules.rules[i].iter().skip(1) {
            out.push(RawDiagnostic::error(
                Code::DuplicateRule,
                rule.span,
                format!("The rule `{name}` is defined more than once. Join its bodies into one rule with `|`."),
            ));
        }
    }
    let mut first_by_key: HashMap<String, &str> = HashMap::new();
    for (i, name) in rules.names.iter().enumerate() {
        let key = name.to_lowercase();
        match first_by_key.get(&key) {
            Some(earlier) => out.push(RawDiagnostic::warning(
                Code::SimilarRuleNames,
                rules.span(i),
                format!("The rule name `{name}` differs from the rule name `{earlier}` only by case."),
            )),
            None => {
                first_by_key.insert(key, name);
            }
        }
    }
}

/// Bare names without a rule.
fn check_references(rules: &RuleIndex, out: &mut Vec<RawDiagnostic>) {
    for (i, rule_name) in rules.names.iter().enumerate() {
        for body in rules.bodies(i) {
            for_each_reference(body, &mut |name, span| {
                if rules.get(name).is_none() {
                    out.push(RawDiagnostic::error(
                        Code::UndefinedRule,
                        span,
                        format!(
                            "In the rule `{rule_name}`, `{name}` is not the name of a rule. \
                             If you meant a symbol, write \"{name}\"."
                        ),
                    ));
                }
            });
        }
    }
}

/// Rules that the start rule does not reach. The check needs a `%start` that
/// names a rule. Without one, the start diagnostics state the cause.
fn check_unused(grammar: &CheckedGrammar, rules: &RuleIndex, out: &mut Vec<RawDiagnostic>) {
    let starts: Vec<usize> = grammar.start_rules.iter().filter_map(|s| rules.get(&s.name)).collect();
    let Some(&first_start) = starts.first() else {
        return;
    };
    let mut reached = vec![false; rules.len()];
    let mut pending = starts;
    while let Some(i) = pending.pop() {
        if std::mem::replace(&mut reached[i], true) {
            continue;
        }
        for body in rules.bodies(i) {
            for_each_reference(body, &mut |name, _| pending.extend(rules.get(name)));
        }
    }
    for (i, name) in rules.names.iter().enumerate() {
        if !reached[i] {
            out.push(RawDiagnostic::warning(
                Code::UnusedRule,
                rules.span(i),
                format!("The start rule `{}` does not reach the rule `{name}`.", rules.names[first_start]),
            ));
        }
    }
}

/// Repetitions whose body can match the empty sequence, and bounds above the
/// bound cap.
fn check_repetitions(rules: &RuleIndex, nullable: &[bool], out: &mut Vec<RawDiagnostic>) {
    for (i, rule_name) in rules.names.iter().enumerate() {
        for body in rules.bodies(i) {
            let mut walk = RepetitionWalk { rules, nullable, rule_name, out: &mut *out };
            walk.visit(body, 1, false);
        }
    }
}

struct RepetitionWalk<'a, 'b> {
    rules: &'a RuleIndex<'a>,
    nullable: &'a [bool],
    rule_name: &'a str,
    out: &'b mut Vec<RawDiagnostic>,
}

impl RepetitionWalk<'_, '_> {
    /// Visits an expression. `copies` is the product of the bounds around it,
    /// and `capped` says that an enclosing repetition already broke the cap.
    fn visit(&mut self, expr: &Expr, copies: u64, capped: bool) {
        match expr {
            Expr::Symbol { .. } | Expr::Reference { .. } => {}
            Expr::Sequence { items, .. } | Expr::Choice { alternatives: items, .. } => {
                items.iter().for_each(|item| self.visit(item, copies, capped));
            }
            Expr::Optional { item, .. } => self.visit(item, copies, capped),
            Expr::Repetition { min, max, item, span } => {
                if self.rules.can_be_empty(item, self.nullable) {
                    // Only an open repetition can take any number of empty passes.
                    let consequence = if max.is_none() {
                        "So one sequence can have infinitely many trees."
                    } else {
                        "So one sequence can have several trees that differ only by empty passes."
                    };
                    self.out.push(RawDiagnostic::error(
                        Code::NullableRepetition,
                        *span,
                        format!(
                            "In the rule `{}`, the body of this repetition can match the empty sequence. \
                             {consequence} Make each pass take at least one step.",
                            self.rule_name
                        ),
                    ));
                }
                // `*` and `+` count as 1, and an open bound `{m,}` counts as m.
                let copies = copies.saturating_mul(u64::from(max.unwrap_or(*min).max(1)));
                let breaks_cap = !capped && copies > BOUND_CAP;
                if breaks_cap {
                    self.out.push(RawDiagnostic::error(
                        Code::BoundAboveCap,
                        *span,
                        format!(
                            "In the rule `{}`, the bounds around the body of this repetition multiply to {copies}. \
                             The bound cap is {BOUND_CAP}. To mean \"many\", write `*` or `+`.",
                            self.rule_name
                        ),
                    ));
                }
                self.visit(item, copies, capped || breaks_cap);
            }
        }
    }
}

/// Groups of rules that can derive themselves with no step. The check follows
/// each reference that can be the whole match of its rule body, because the
/// items around it can match the empty sequence. A cycle of such references
/// derives a rule from itself.
fn check_cycles(rules: &RuleIndex, nullable: &[bool], unread_bounds: &[Span], out: &mut Vec<RawDiagnostic>) {
    let (edges, components) = cyclic_components(rules, nullable, unread_bounds);
    for component in components {
        let first = component[0];
        let Some((path, span)) = cycle_from(first, &component, &edges) else {
            continue;
        };
        let path: Vec<String> = path.iter().map(|&i| format!("`{}`", rules.names[i])).collect();
        out.push(RawDiagnostic::error(
            Code::CyclicRule,
            span,
            format!(
                "The rule `{}` can derive itself with no step: {}. So one sequence can have infinitely many trees. \
                 Make one rule of the cycle take a step.",
                rules.names[first],
                path.join(" → ")
            ),
        ));
    }
}

/// For each rule, the targets and the spans of its whole-match references.
type WholeMatchEdges = Vec<Vec<(usize, Span)>>;

/// The whole-match references, and the components of rules that can derive
/// themselves through them.
fn cyclic_components(
    rules: &RuleIndex,
    nullable: &[bool],
    unread_bounds: &[Span],
) -> (WholeMatchEdges, Vec<Vec<usize>>) {
    let edges: WholeMatchEdges = (0..rules.len())
        .map(|i| {
            let mut edges = Vec::new();
            for body in rules.bodies(i) {
                whole_match_references(rules, nullable, unread_bounds, body, &mut edges);
            }
            edges
        })
        .collect();
    let targets: Vec<Vec<usize>> = edges.iter().map(|e| e.iter().map(|&(to, _)| to).collect()).collect();
    let components =
        strongly_connected(&targets).into_iter().filter(|c| c.len() > 1 || targets[c[0]].contains(&c[0])).collect();
    (edges, components)
}

/// Collects each reference that can be the whole match of an expression.
///
/// The walk does not enter an item whose bound is malformed. A least number of
/// 2 or more would make another pass take a step, and the number is unknown.
fn whole_match_references(
    rules: &RuleIndex,
    nullable: &[bool],
    unread_bounds: &[Span],
    expr: &Expr,
    out: &mut Vec<(usize, Span)>,
) {
    if unread_bounds.contains(&expr.span()) {
        return;
    }
    let mut walk = |expr: &Expr| whole_match_references(rules, nullable, unread_bounds, expr, out);
    match expr {
        Expr::Symbol { .. } => {}
        Expr::Reference { name, span } => out.extend(rules.get(name).map(|i| (i, *span))),
        Expr::Sequence { items, .. } => {
            // Only an item whose neighbours can all be empty can be the whole match.
            let solid: Vec<&Expr> = items.iter().filter(|item| !rules.can_be_empty(item, nullable)).collect();
            match solid.as_slice() {
                [] => items.iter().for_each(walk),
                [item] => walk(item),
                _ => {}
            }
        }
        Expr::Choice { alternatives, .. } => {
            alternatives.iter().for_each(walk);
        }
        Expr::Optional { item, .. } => walk(item),
        Expr::Repetition { min, max, item, .. } => {
            // The other passes must be absent or empty.
            if *max != Some(0) && (*min <= 1 || rules.can_be_empty(item, nullable)) {
                walk(item);
            }
        }
    }
}

/// A shortest cycle from `start` back to `start` inside a component, as the
/// list of its rules, with the span of its first reference.
fn cycle_from(start: usize, component: &[usize], edges: &[Vec<(usize, Span)>]) -> Option<(Vec<usize>, Span)> {
    let mut parent: HashMap<usize, (usize, Span)> = HashMap::new();
    let mut queue = std::collections::VecDeque::from([start]);
    while let Some(from) = queue.pop_front() {
        for &(to, span) in &edges[from] {
            if to == start {
                // Walk back from the last rule of the cycle to `start`.
                let mut inner = Vec::new();
                let mut first_span = span;
                let mut at = from;
                while at != start {
                    inner.push(at);
                    let (previous, previous_span) = parent[&at];
                    first_span = previous_span;
                    at = previous;
                }
                inner.reverse();
                let path = std::iter::once(start).chain(inner).chain(std::iter::once(start)).collect();
                return Some((path, first_span));
            }
            if component.contains(&to) && !parent.contains_key(&to) {
                parent.insert(to, (from, span));
                queue.push_back(to);
            }
        }
    }
    None
}

/// Symbols that differ only by case, spaces, `_` or `-`.
fn check_symbols(grammar: &CheckedGrammar, out: &mut Vec<RawDiagnostic>) {
    let mut first_by_key: HashMap<String, &str> = HashMap::new();
    for symbol in &grammar.symbols {
        // Two symbols differ only by case when their case foldings match
        // (https://www.unicode.org/Public/UCD/latest/ucd/CaseFolding.txt). The
        // standard library has no case folding. The lower case of the upper
        // case of the lower case gives the same classes as the full case
        // folding. The only difference is that it also joins the dotless `ı`
        // with `i`. The mapping of each character needs no context, so `Σ`,
        // `σ` and `ς` share a key, and so do `ß`, `ẞ` and `ss`.
        let key: String = symbol
            .value
            .chars()
            .filter(|&c| !c.is_whitespace() && c != '_' && c != '-')
            .flat_map(char::to_lowercase)
            .flat_map(char::to_uppercase)
            .flat_map(char::to_lowercase)
            .collect();
        match first_by_key.get(&key) {
            Some(earlier) => {
                if let Some(first_use) = symbol.uses.first() {
                    out.push(RawDiagnostic::warning(
                        Code::SimilarSymbols,
                        first_use.span,
                        format!(
                            "In the rule `{}`, the symbol {:?} differs from the symbol {earlier:?} only by case, \
                             spaces, `_` or `-`. A misspelt symbol is a new symbol.",
                            first_use.rule, symbol.value
                        ),
                    ));
                }
            }
            None => {
                first_by_key.insert(key, &symbol.value);
            }
        }
    }
}

/// The groups of mutually recursive rules, each with its badge.
///
/// The badge follows Nederhof (https://arxiv.org/pdf/cs/9910022), section 2: a
/// group is self-embedding exactly when some reference inside the group has
/// steps before it and some reference inside the group has steps after it.
/// The two references can differ, because each rule of a group reaches every
/// other. In a rule body, an item before or after a reference can give such
/// steps, and so can another pass of a repetition around the reference.
fn recursive_groups(rules: &RuleIndex) -> Vec<RecursiveGroup> {
    let targets: Vec<Vec<usize>> = (0..rules.len())
        .map(|i| {
            let mut targets = Vec::new();
            for body in rules.bodies(i) {
                for_each_reference(body, &mut |name, _| targets.extend(rules.get(name)));
            }
            targets
        })
        .collect();
    let mut groups: Vec<(usize, RecursiveGroup)> = Vec::new();
    for component in strongly_connected(&targets) {
        let first = component[0];
        if component.len() == 1 && !targets[first].contains(&first) {
            continue;
        }
        let (mut steps_before, mut steps_after) = (false, false);
        for &i in &component {
            for body in rules.bodies(i) {
                reference_contexts(body, false, false, &mut |name, before, after| {
                    if rules.get(name).is_some_and(|to| component.contains(&to)) {
                        steps_before |= before;
                        steps_after |= after;
                    }
                });
            }
        }
        let badge = if steps_before && steps_after { Badge::MayNeedAStack } else { Badge::NoSelfEmbedding };
        let members = component.iter().map(|&i| GroupMember { rule: rules.names[i].to_string() }).collect();
        groups.push((first, RecursiveGroup { badge, members }));
    }
    groups.sort_by_key(|&(first, _)| first);
    groups.into_iter().map(|(_, group)| group).collect()
}

/// Calls `f` for each reference, with whether steps can come before it and
/// after it inside the rule body.
fn reference_contexts(expr: &Expr, before: bool, after: bool, f: &mut impl FnMut(&str, bool, bool)) {
    match expr {
        Expr::Symbol { .. } => {}
        Expr::Reference { name, .. } => f(name, before, after),
        Expr::Sequence { items, .. } => {
            let last = items.len().saturating_sub(1);
            for (k, item) in items.iter().enumerate() {
                reference_contexts(item, before || k > 0, after || k < last, f);
            }
        }
        Expr::Choice { alternatives, .. } => {
            alternatives.iter().for_each(|alt| reference_contexts(alt, before, after, f));
        }
        Expr::Optional { item, .. } => reference_contexts(item, before, after, f),
        Expr::Repetition { max, item, .. } => {
            let other_passes = max.is_none_or(|max| max >= 2);
            reference_contexts(item, before || other_passes, after || other_passes, f);
        }
    }
}

/// The strongly connected components of a graph, by the algorithm of Tarjan
/// with an explicit stack, so that a long chain of rules cannot overflow the
/// call stack. Each component lists its nodes in increasing order.
fn strongly_connected(targets: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let mut t = Tarjan {
        index: vec![Tarjan::UNSEEN; targets.len()],
        low: vec![0; targets.len()],
        on_stack: vec![false; targets.len()],
        stack: Vec::new(),
        calls: Vec::new(),
        next_index: 0,
    };
    let mut components = Vec::new();
    for root in 0..targets.len() {
        if t.index[root] != Tarjan::UNSEEN {
            continue;
        }
        t.enter(root);
        while let Some(&(v, edge)) = t.calls.last() {
            if let Some(&w) = targets[v].get(edge) {
                if let Some(top) = t.calls.last_mut() {
                    top.1 += 1;
                }
                if t.index[w] == Tarjan::UNSEEN {
                    t.enter(w);
                } else if t.on_stack[w] {
                    t.low[v] = t.low[v].min(t.index[w]);
                }
                continue;
            }
            t.calls.pop();
            if let Some(&(parent, _)) = t.calls.last() {
                t.low[parent] = t.low[parent].min(t.low[v]);
            }
            if t.low[v] == t.index[v] {
                let mut component = Vec::new();
                while let Some(w) = t.stack.pop() {
                    t.on_stack[w] = false;
                    component.push(w);
                    if w == v {
                        break;
                    }
                }
                component.sort_unstable();
                components.push(component);
            }
        }
    }
    components
}

/// The state of `strongly_connected`. `calls` holds each node in progress,
/// with the position of its next edge.
struct Tarjan {
    index: Vec<usize>,
    low: Vec<usize>,
    on_stack: Vec<bool>,
    stack: Vec<usize>,
    calls: Vec<(usize, usize)>,
    next_index: usize,
}

impl Tarjan {
    const UNSEEN: usize = usize::MAX;

    fn enter(&mut self, v: usize) {
        self.index[v] = self.next_index;
        self.low[v] = self.next_index;
        self.next_index += 1;
        self.stack.push(v);
        self.on_stack[v] = true;
        self.calls.push((v, 0));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn components_of_a_small_graph() {
        // 0 → 1 → 2 → 0, 2 → 3, 3 → 3, 4 alone.
        let targets = vec![vec![1], vec![2], vec![0, 3], vec![3], vec![]];
        let mut components = strongly_connected(&targets);
        components.sort();
        assert_eq!(components, vec![vec![0, 1, 2], vec![3], vec![4]]);
    }

    #[test]
    fn a_long_chain_does_not_overflow_the_stack() {
        let n = 200_000;
        let targets: Vec<Vec<usize>> = (0..n).map(|i| vec![(i + 1) % n]).collect();
        let components = strongly_connected(&targets);
        assert_eq!(components.len(), 1);
        assert_eq!(components[0].len(), n);
    }

    // An oracle for the badges and the cycle check. It rewrites each rule into
    // plain BNF with helper rules, and then searches the derivations directly:
    // a rule self-embeds when it derives `u A v` with `u` and `v` not empty,
    // and it is cyclic when it derives itself alone. In BNF, neither question
    // needs the rules for repetitions that the checks use.

    #[derive(Clone, Copy, PartialEq)]
    enum Atom {
        Step,
        Rule(usize),
    }

    /// The productions of each BNF rule. The first rules are the grammar rules.
    struct Bnf(Vec<Vec<Vec<Atom>>>);

    impl Bnf {
        fn helper(&mut self) -> usize {
            self.0.push(Vec::new());
            self.0.len() - 1
        }

        fn expand(&mut self, expr: &Expr, index: &RuleIndex) -> Atom {
            let productions = match expr {
                Expr::Symbol { .. } => return Atom::Step,
                Expr::Reference { name, .. } => return Atom::Rule(index.get(name).expect("a defined rule")),
                Expr::Sequence { items, .. } => vec![items.iter().map(|item| self.expand(item, index)).collect()],
                Expr::Choice { alternatives, .. } => {
                    alternatives.iter().map(|alt| vec![self.expand(alt, index)]).collect()
                }
                Expr::Optional { item, .. } => vec![vec![self.expand(item, index)], vec![]],
                Expr::Repetition { min, max, item, .. } => {
                    let x = self.expand(item, index);
                    let copies = |k: u32| vec![x; k as usize];
                    match max {
                        Some(max) => (*min..=*max).map(copies).collect(),
                        None => {
                            let rest = self.helper();
                            self.0[rest] = vec![vec![], vec![x, Atom::Rule(rest)]];
                            let mut first = copies(*min);
                            first.push(Atom::Rule(rest));
                            vec![first]
                        }
                    }
                }
            };
            let h = self.helper();
            self.0[h] = productions;
            Atom::Rule(h)
        }

        /// A least fixed point over the rules: `holds` decides one production.
        fn fixed_point(&self, holds: impl Fn(&[Atom], &[bool]) -> bool) -> Vec<bool> {
            let mut result = vec![false; self.0.len()];
            let mut changed = true;
            while changed {
                changed = false;
                for (n, productions) in self.0.iter().enumerate() {
                    if !result[n] && productions.iter().any(|p| holds(p, &result)) {
                        result[n] = true;
                        changed = true;
                    }
                }
            }
            result
        }

        /// Whether `start` reaches itself. An edge is one rule in one
        /// production, and `edge` gives its flags from the atoms around it.
        /// The search succeeds when it returns with both flags set.
        fn reaches_itself(&self, start: usize, edge: impl Fn(&[Atom], &[Atom]) -> Option<(bool, bool)>) -> bool {
            let mut seen = std::collections::HashSet::new();
            let mut pending = vec![(start, false, false)];
            let mut first = true;
            while let Some((n, l, r)) = pending.pop() {
                if !first && n == start && l && r {
                    return true;
                }
                first = false;
                for p in &self.0[n] {
                    for (i, atom) in p.iter().enumerate() {
                        let Atom::Rule(to) = *atom else { continue };
                        let Some((lf, rf)) = edge(&p[..i], &p[i + 1..]) else { continue };
                        let state = (to, l || lf, r || rf);
                        if seen.insert(state) {
                            pending.push(state);
                        }
                    }
                }
            }
            false
        }
    }

    fn span() -> Span {
        Span { start: 0, end: 0 }
    }

    /// A small linear congruential generator, so the test needs no dependency.
    struct Lcg(u64);

    impl Lcg {
        fn next(&mut self) -> usize {
            self.0 = self.0.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407);
            (self.0 >> 33) as usize
        }
    }

    /// A random expression over two symbols and `rules` rules.
    fn random_expr(rng: &mut Lcg, rules: usize, depth: u32) -> Expr {
        let choice = if depth == 0 { rng.next() % 2 } else { rng.next() % 7 };
        match choice {
            0 => Expr::Symbol { value: ["x", "y"][rng.next() % 2].into(), span: span() },
            1 => Expr::Reference { name: format!("r{}", rng.next() % rules), span: span() },
            2 => {
                let len = 2 + rng.next() % 2;
                Expr::Sequence { items: (0..len).map(|_| random_expr(rng, rules, depth - 1)).collect(), span: span() }
            }
            3 => Expr::Choice {
                alternatives: (0..2).map(|_| random_expr(rng, rules, depth - 1)).collect(),
                span: span(),
            },
            4 => Expr::Optional { item: Box::new(random_expr(rng, rules, depth - 1)), span: span() },
            _ => {
                let bounds = [(0, None), (1, None), (2, None), (0, Some(2)), (1, Some(3)), (2, Some(3))];
                let (min, max) = bounds[rng.next() % bounds.len()];
                Expr::Repetition { min, max, item: Box::new(random_expr(rng, rules, depth - 1)), span: span() }
            }
        }
    }

    #[test]
    fn badges_and_cycles_agree_with_a_search_over_plain_bnf() {
        let mut rng = Lcg(0x5eed);
        let (mut stacks, mut cycles) = (0, 0);
        for case in 0..3_000 {
            let n = 1 + rng.next() % 4;
            // Each rule also matches one symbol, so each rule matches some sequence.
            let rules: Vec<Rule> = (0..n)
                .map(|i| Rule {
                    name: format!("r{i}"),
                    span: span(),
                    doc: None,
                    body: Some(Expr::Choice {
                        alternatives: vec![
                            random_expr(&mut rng, n, 3),
                            Expr::Symbol { value: "z".into(), span: span() },
                        ],
                        span: span(),
                    }),
                })
                .collect();
            let index = RuleIndex::new(&rules);
            let mut bnf = Bnf(vec![Vec::new(); n]);
            for (i, rule) in rules.iter().enumerate() {
                let body = bnf.expand(rule.body.as_ref().expect("a body"), &index);
                bnf.0[i] = vec![vec![body]];
            }
            let nullable = bnf.fixed_point(|p, nullable| p.iter().all(|a| matches!(a, Atom::Rule(r) if nullable[*r])));
            let solid = bnf.fixed_point(|p, solid| {
                p.iter().any(|a| match a {
                    Atom::Step => true,
                    Atom::Rule(r) => solid[*r],
                })
            });
            let can_step = |atoms: &[Atom]| {
                atoms.iter().any(|a| matches!(a, Atom::Step) || matches!(a, Atom::Rule(r) if solid[*r]))
            };
            let all_empty = |atoms: &[Atom]| atoms.iter().all(|a| matches!(a, Atom::Rule(r) if nullable[*r]));

            let groups = recursive_groups(&index);
            let stack_rules: Vec<bool> = (0..n)
                .map(|i| {
                    groups
                        .iter()
                        .any(|g| g.badge == Badge::MayNeedAStack && g.members.iter().any(|m| m.rule == rules[i].name))
                })
                .collect();
            let checked_nullable = index.nullable();
            let (_, components) = cyclic_components(&index, &checked_nullable, &[]);
            for i in 0..n {
                let embeds = bnf.reaches_itself(i, |before, after| Some((can_step(before), can_step(after))));
                assert_eq!(stack_rules[i], embeds, "case {case}, rule r{i}: {rules:#?}");
                let cyclic = bnf
                    .reaches_itself(i, |before, after| (all_empty(before) && all_empty(after)).then_some((true, true)));
                let checked_cyclic = components.iter().any(|c| c.contains(&i));
                assert_eq!(checked_cyclic, cyclic, "case {case}, rule r{i}: {rules:#?}");
                assert_eq!(checked_nullable[i], nullable[i], "case {case}, rule r{i}: {rules:#?}");
                stacks += usize::from(embeds);
                cycles += usize::from(cyclic);
            }
        }
        // The generator must reach both answers of each question.
        assert!(stacks > 100 && cycles > 100, "self-embedding {stacks}, cyclic {cycles}");
    }
}
