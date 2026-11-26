//! The property tests of the tracer at the core interface:
//! `docs/specs/tracer.md`, sections "The reference recogniser" and "The
//! generators".
//!
//! The reference recogniser expands the checked grammar into plain BNF. It
//! grows three relations over the spans of a sequence until they stop growing:
//! a rule derives a span, a rule starts a span, and a rule is productive. It
//! shares only the checked grammar with the tracer. It has no automaton, no
//! Earley set and no pass count.
//!
//! The generators start from one seed in every run and need no dependency.
//!
//! The tree oracle of the module `tree_oracle` finds the readings of each short
//! prefix from the grammar text, and the tracer must give the same readings.

// A file in a subdirectory of `tests` is no test crate of its own.
#[path = "tracer_properties/tree_oracle.rs"]
mod tree_oracle;

use std::{
    collections::{BTreeSet, HashMap},
    rc::Rc,
    sync::{
        Mutex,
        atomic::{AtomicUsize, Ordering as AtomicOrdering},
    },
    time::{Duration, Instant},
};

use workbench_core::{
    Session, TraceGrammar,
    checked::{CheckedGrammar, Expr, Severity, Span},
    checked_grammar,
    trace::{Attribute, AttributeValue, Float, READING_CAP, Readings, Step, TreeKind, TreeNode},
};

// The cost bounds of the section "The generators".
const MAX_RULES: usize = 4;
const MAX_DEPTH: u32 = 3;
const MAX_STEPS: usize = 12;
const MIN_GRAMMARS: usize = 1_000;
const MAX_DECISIONS: u32 = 64;
const SENTENCE_STEPS: usize = 8;
const SYMBOLS: [&str; 3] = ["x", "y", "z"];
/// The symbols of the sequences: the symbols of the grammars and `w`, which no
/// grammar uses.
const ALL_SYMBOLS: [&str; 4] = ["x", "y", "z", "w"];

/// A small linear congruential generator, so the test needs no dependency.
struct Lcg(u64);

impl Lcg {
    fn next(&mut self) -> usize {
        self.0 = self.0.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407);
        (self.0 >> 33) as usize
    }

    fn below(&mut self, n: usize) -> usize {
        self.next() % n
    }

    fn chance(&mut self, numerator: usize, denominator: usize) -> bool {
        self.below(denominator) < numerator
    }
}

// The random grammars.

/// A random grammar text: 1 to 4 rules over the symbols `x`, `y` and `z`, with
/// `r0` as the start rule.
fn random_grammar(rng: &mut Lcg) -> String {
    let rules = 1 + rng.below(MAX_RULES);
    let mut text = String::from("%start r0 ;\n");
    for r in 0..rules {
        let body = random_expr(rng, MAX_DEPTH, rules);
        if rng.chance(1, 2) {
            let extra = SYMBOLS[rng.below(SYMBOLS.len())];
            text.push_str(&format!("r{r} = ({body}) | \"{extra}\" ;\n"));
        } else {
            text.push_str(&format!("r{r} = {body} ;\n"));
        }
    }
    text
}

fn random_expr(rng: &mut Lcg, depth: u32, rules: usize) -> String {
    if depth == 0 || rng.chance(1, 3) {
        return if rng.chance(1, 3) {
            format!("r{}", rng.below(rules))
        } else {
            format!("\"{}\"", SYMBOLS[rng.below(SYMBOLS.len())])
        };
    }
    match rng.below(4) {
        0 => {
            let items: Vec<String> = (0..2 + rng.below(2)).map(|_| random_expr(rng, depth - 1, rules)).collect();
            format!("({})", items.join(" "))
        }
        1 => format!("({} | {})", random_expr(rng, depth - 1, rules), random_expr(rng, depth - 1, rules)),
        2 => format!("({})?", random_expr(rng, depth - 1, rules)),
        _ => {
            let bound = ["*", "+", "{2}", "{0,2}", "{1,3}", "{2,}"][rng.below(6)];
            format!("({}){bound}", random_expr(rng, depth - 1, rules))
        }
    }
}

// The BNF expansion.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Sym {
    T(u8),
    N(usize),
}

/// A grammar in plain BNF: the alternatives of each nonterminal. The rules of
/// the checked grammar come first, in their order.
struct Bnf {
    alternatives: Vec<Vec<Vec<Sym>>>,
    start: usize,
    productive: Vec<bool>,
}

fn terminal(value: &str) -> u8 {
    ALL_SYMBOLS.iter().position(|s| *s == value).expect("a symbol of the generators") as u8
}

impl Bnf {
    fn new(grammar: &CheckedGrammar) -> Self {
        let names: Vec<&str> = grammar.rules.iter().map(|r| r.name.as_str()).collect();
        let mut bnf = Self { alternatives: vec![Vec::new(); names.len()], start: 0, productive: Vec::new() };
        bnf.start = names.iter().position(|n| *n == grammar.start_rules[0].name).expect("the start rule");
        for (i, rule) in grammar.rules.iter().enumerate() {
            let body = rule.body.as_ref().expect("a body");
            bnf.alternatives[i] = bnf.alts(body, &names);
        }
        bnf.productive = bnf.productive_nonterminals();
        bnf
    }

    /// The alternatives of an expression.
    fn alts(&mut self, expr: &Expr, names: &[&str]) -> Vec<Vec<Sym>> {
        match expr {
            Expr::Symbol { .. } | Expr::Reference { .. } => vec![vec![self.sym(expr, names)]],
            Expr::Sequence { items, .. } => {
                vec![items.iter().map(|item| self.sym(item, names)).collect()]
            }
            Expr::Choice { alternatives, .. } => alternatives.iter().flat_map(|a| self.alts(a, names)).collect(),
            Expr::Optional { item, .. } => {
                let mut alts = self.alts(item, names);
                alts.push(Vec::new());
                alts
            }
            Expr::Repetition { min, max: Some(max), item, .. } => {
                let one = self.sym(item, names);
                (*min..=*max).map(|k| vec![one; k as usize]).collect()
            }
            Expr::Repetition { min, max: None, item, .. } => {
                // An open repetition becomes a helper rule with an empty alternative.
                let one = self.sym(item, names);
                let helper = self.alternatives.len();
                self.alternatives.push(vec![vec![one, Sym::N(helper)], Vec::new()]);
                let mut alt = vec![one; *min as usize];
                alt.push(Sym::N(helper));
                vec![alt]
            }
        }
    }

    /// One symbol for an expression: a helper nonterminal for a compound one.
    fn sym(&mut self, expr: &Expr, names: &[&str]) -> Sym {
        match expr {
            Expr::Symbol { value, .. } => Sym::T(terminal(value)),
            Expr::Reference { name, .. } => Sym::N(names.iter().position(|n| n == name).expect("a rule")),
            _ => {
                let helper = self.alternatives.len();
                self.alternatives.push(Vec::new());
                let alts = self.alts(expr, names);
                self.alternatives[helper] = alts;
                Sym::N(helper)
            }
        }
    }

    fn productive_nonterminals(&self) -> Vec<bool> {
        let mut productive = vec![false; self.alternatives.len()];
        loop {
            let mut changed = false;
            for (x, alts) in self.alternatives.iter().enumerate() {
                if !productive[x]
                    && alts.iter().any(|alt| {
                        alt.iter().all(|s| match s {
                            Sym::T(_) => true,
                            Sym::N(n) => productive[*n],
                        })
                    })
                {
                    productive[x] = true;
                    changed = true;
                }
            }
            if !changed {
                return productive;
            }
        }
    }
}

// The reference recogniser.

/// A set of positions in a sequence, as a bit mask. A sequence has at most 13
/// steps, with the one-step extension, so 14 positions fit.
type Positions = u16;

fn positions(mut set: Positions) -> impl Iterator<Item = usize> {
    std::iter::from_fn(move || {
        if set == 0 {
            return None;
        }
        let p = set.trailing_zeros() as usize;
        set &= set - 1;
        Some(p)
    })
}

/// The relations of the recogniser over one sequence `w` of length m.
struct Chart {
    /// `derives[x][i]`: the positions j such that nonterminal x derives `w[i..j]`.
    derives: Vec<Vec<Positions>>,
    /// `starts[x][i]`: x derives `w[i..m]` followed by some sequence.
    starts: Vec<Vec<bool>>,
}

/// For each t, the positions that the symbols `alt[..t]` can reach from i, by
/// `derives`.
fn reach(w: &[u8], derives: &[Vec<Positions>], alt: &[Sym], i: usize) -> Vec<Positions> {
    let m = w.len();
    let mut reached = vec![1 << i];
    for sym in alt {
        let mut next: Positions = 0;
        for p in positions(*reached.last().expect("a set")) {
            match sym {
                Sym::T(a) => {
                    if p < m && w[p] == *a {
                        next |= 1 << (p + 1);
                    }
                }
                Sym::N(n) => next |= derives[*n][p],
            }
        }
        reached.push(next);
    }
    reached
}

/// The positions that all the symbols of `alt` can reach from i: the last set
/// of `reach`, without the sets before it.
fn reach_end(w: &[u8], derives: &[Vec<Positions>], alt: &[Sym], i: usize) -> Positions {
    let m = w.len();
    let mut reached: Positions = 1 << i;
    for sym in alt {
        let mut next: Positions = 0;
        for p in positions(reached) {
            match sym {
                Sym::T(a) => {
                    if p < m && w[p] == *a {
                        next |= 1 << (p + 1);
                    }
                }
                Sym::N(n) => next |= derives[*n][p],
            }
        }
        if next == 0 {
            return 0;
        }
        reached = next;
    }
    reached
}

fn chart(bnf: &Bnf, w: &[u8]) -> Chart {
    let m = w.len();
    assert!(m < 15, "a sequence of {m} steps does not fit the bit masks");
    let n = bnf.alternatives.len();
    let mut derives: Vec<Vec<Positions>> = vec![vec![0; m + 1]; n];
    loop {
        let mut changed = false;
        for x in 0..n {
            for i in 0..=m {
                for alt in &bnf.alternatives[x] {
                    let ends = reach_end(w, &derives, alt, i);
                    if ends & !derives[x][i] != 0 {
                        derives[x][i] |= ends;
                        changed = true;
                    }
                }
            }
        }
        if !changed {
            break;
        }
    }
    let mut starts = vec![vec![false; m + 1]; n];
    loop {
        let mut changed = false;
        for x in 0..n {
            for i in 0..=m {
                if starts[x][i] {
                    continue;
                }
                let found = bnf.alternatives[x].iter().any(|alt| {
                    let reached = reach(w, &derives, alt, i);
                    if reached[alt.len()] & (1 << m) != 0 {
                        return true;
                    }
                    (0..alt.len()).any(|t| {
                        let rest_productive = alt[t + 1..].iter().all(|s| match s {
                            Sym::T(_) => true,
                            Sym::N(n) => bnf.productive[*n],
                        });
                        rest_productive
                            && positions(reached[t]).any(|p| match alt[t] {
                                Sym::T(a) => p == m || (p + 1 == m && w[p] == a),
                                Sym::N(y) => starts[y][p],
                            })
                    })
                });
                if found {
                    starts[x][i] = true;
                    changed = true;
                }
            }
        }
        if !changed {
            break;
        }
    }
    Chart { derives, starts }
}

/// The recogniser for one grammar, with a memo of its charts by sequence.
struct Recogniser {
    bnf: Bnf,
    memo: HashMap<Vec<u8>, (bool, bool)>,
}

impl Recogniser {
    fn new(grammar: &CheckedGrammar) -> Self {
        Self { bnf: Bnf::new(grammar), memo: HashMap::new() }
    }

    /// Whether the start rule starts the sequence, and whether it derives it.
    fn decide(&mut self, w: &[u8]) -> (bool, bool) {
        if let Some(&result) = self.memo.get(w) {
            return result;
        }
        let chart = chart(&self.bnf, w);
        let s = self.bnf.start;
        let result = (chart.starts[s][0], chart.derives[s][0] & (1 << w.len()) != 0);
        self.memo.insert(w.to_vec(), result);
        result
    }

    fn accepts(&mut self, w: &[u8]) -> bool {
        self.decide(w).0
    }

    fn can_end(&mut self, w: &[u8]) -> bool {
        self.decide(w).1
    }

    /// The next options after a prefix: each symbol that the start rule starts
    /// after it.
    fn next_options(&mut self, w: &[u8]) -> Vec<String> {
        let mut options = Vec::new();
        for (a, symbol) in ALL_SYMBOLS.iter().enumerate() {
            let mut longer = w.to_vec();
            longer.push(a as u8);
            if self.accepts(&longer) {
                options.push(symbol.to_string());
            }
        }
        options
    }
}

// The random sequences.

/// The cheapest completion of each rule: the fewest steps, then the fewest
/// expansions. `None` means that the rule derives no finite sequence.
type Cost = Option<(u32, u32)>;

fn add(a: Cost, b: Cost) -> Cost {
    Some((a?.0 + b?.0, a?.1 + b?.1))
}

struct Generator<'a> {
    grammar: &'a CheckedGrammar,
    rule_costs: Vec<Cost>,
}

impl<'a> Generator<'a> {
    fn new(grammar: &'a CheckedGrammar) -> Self {
        let mut generator = Self { grammar, rule_costs: vec![None; grammar.rules.len()] };
        loop {
            let costs: Vec<Cost> = grammar
                .rules
                .iter()
                .map(|r| generator.cost(r.body.as_ref().expect("a body")).map(|(s, e)| (s, e + 1)))
                .collect();
            if costs == generator.rule_costs {
                return generator;
            }
            generator.rule_costs = costs;
        }
    }

    fn rule(&self, name: &str) -> usize {
        self.grammar.rules.iter().position(|r| r.name == name).expect("a rule")
    }

    fn cost(&self, expr: &Expr) -> Cost {
        match expr {
            Expr::Symbol { .. } => Some((1, 0)),
            Expr::Reference { name, .. } => self.rule_costs[self.rule(name)],
            Expr::Sequence { items, .. } => items.iter().try_fold((0, 0), |c, i| add(Some(c), self.cost(i))),
            Expr::Choice { alternatives, .. } => alternatives.iter().filter_map(|a| self.cost(a)).min(),
            Expr::Optional { .. } => Some((0, 0)),
            Expr::Repetition { min, item, .. } => {
                if *min == 0 {
                    return Some((0, 0));
                }
                let one = self.cost(item)?;
                Some((one.0 * min, one.1 * min))
            }
        }
    }

    /// A sentence of the start rule, by generation with a cost bound.
    fn sentence(&self, rng: &mut Lcg) -> Option<Vec<String>> {
        let start = self.rule(&self.grammar.start_rules[0].name);
        self.rule_costs[start]?;
        let mut state = Budget { out: Vec::new(), decisions: 0 };
        self.expand(self.grammar.rules[start].body.as_ref().expect("a body"), rng, &mut state);
        Some(state.out)
    }

    fn expand(&self, expr: &Expr, rng: &mut Lcg, state: &mut Budget) {
        match expr {
            Expr::Symbol { value, .. } => state.out.push(value.clone()),
            Expr::Reference { name, .. } => {
                let rule = &self.grammar.rules[self.rule(name)];
                self.expand(rule.body.as_ref().expect("a body"), rng, state);
            }
            Expr::Sequence { items, .. } => items.iter().for_each(|i| self.expand(i, rng, state)),
            Expr::Choice { alternatives, .. } => {
                let live: Vec<&Expr> = alternatives.iter().filter(|a| self.cost(a).is_some()).collect();
                let chosen = if state.decide() {
                    live[rng.below(live.len())]
                } else {
                    *live.iter().min_by_key(|a| self.cost(a)).expect("a productive alternative")
                };
                self.expand(chosen, rng, state);
            }
            Expr::Optional { item, .. } => {
                if self.cost(item).is_some() && state.decide() && rng.chance(1, 2) {
                    self.expand(item, rng, state);
                }
            }
            Expr::Repetition { min, max, item, .. } => {
                for _ in 0..*min {
                    self.expand(item, rng, state);
                }
                let mut passes = *min;
                while self.cost(item).is_some()
                    && max.is_none_or(|max| passes < max)
                    && state.decide()
                    && rng.chance(1, 2)
                {
                    self.expand(item, rng, state);
                    passes += 1;
                }
            }
        }
    }
}

/// The bound on the work of one sentence.
struct Budget {
    out: Vec<String>,
    decisions: u32,
}

impl Budget {
    /// Whether one more decision at random may come. After the bound, each
    /// open part takes its cheapest completion.
    fn decide(&mut self) -> bool {
        if self.out.len() >= SENTENCE_STEPS || self.decisions >= MAX_DECISIONS {
            return false;
        }
        self.decisions += 1;
        true
    }
}

/// The sequences of one grammar: up to 3 sentences, each single edit of each
/// sentence, and one random sequence. No sequence has more than 12 steps.
fn sequences(grammar: &CheckedGrammar, rng: &mut Lcg) -> BTreeSet<Vec<String>> {
    let generator = Generator::new(grammar);
    let mut out = BTreeSet::new();
    for _ in 0..3 {
        let Some(sentence) = generator.sentence(rng) else { break };
        for i in 0..=sentence.len() {
            for symbol in ALL_SYMBOLS {
                let mut edited = sentence.clone();
                edited.insert(i, symbol.to_string());
                out.insert(edited);
                if i < sentence.len() {
                    let mut replaced = sentence.clone();
                    replaced[i] = symbol.to_string();
                    out.insert(replaced);
                }
            }
            if i < sentence.len() {
                let mut removed = sentence.clone();
                removed.remove(i);
                out.insert(removed);
            }
        }
        out.insert(sentence);
    }
    let length = rng.below(SENTENCE_STEPS + 1);
    out.insert((0..length).map(|_| ALL_SYMBOLS[rng.below(4)].to_string()).collect());
    out.retain(|s| s.len() <= MAX_STEPS);
    out
}

/// Strings that KDL 2.0 must quote or escape, for random keys, values and
/// type annotations.
const AWKWARD: &[&str] = &["+.1", "-.5", "a\u{1}b", "a\u{7f}b", "a\u{2028}b", "a\u{ff1d}b", "true", "1", "", "a b"];

fn random_attributes(rng: &mut Lcg, i: usize) -> Vec<Attribute> {
    let mut attributes: Vec<Attribute> = Vec::new();
    for k in 0..rng.below(3) {
        let value = match rng.below(6) {
            0 => AttributeValue::String(format!("value {i}.{k}")),
            1 => AttributeValue::Integer(rng.below(1000) as i128 - 500),
            2 => AttributeValue::Float(Float(rng.below(1000) as f64 / 8.0)),
            3 => AttributeValue::Bool(rng.chance(1, 2)),
            4 => AttributeValue::String(AWKWARD[rng.below(AWKWARD.len())].to_string()),
            _ => AttributeValue::Null,
        };
        let annotation = match rng.below(6) {
            0 | 1 => Some("url".to_string()),
            2 => Some(AWKWARD[rng.below(AWKWARD.len())].to_string()),
            _ => None,
        };
        let key = if rng.chance(1, 4) { AWKWARD[rng.below(AWKWARD.len())].to_string() } else { format!("key-{k}") };
        if attributes.iter().all(|a| a.key != key) {
            attributes.push(Attribute { key, value, annotation });
        }
    }
    attributes
}

/// What one case compares: for each prefix up to the failure point, whether
/// the sequence can end and the next options, and the failure point.
#[derive(Debug, PartialEq, Eq)]
struct Observed {
    prefixes: Vec<(bool, Vec<String>)>,
    failure_point: Option<usize>,
}

/// The tracer gives next options in the order of the symbols in the checked
/// grammar. The comparison with the recogniser compares them as sets, in the
/// order of `ALL_SYMBOLS`.
fn sorted(mut symbols: Vec<String>) -> Vec<String> {
    symbols.sort_by_key(|s| terminal(s));
    symbols
}

fn observe_tracer(session: &Session) -> Observed {
    Observed {
        prefixes: (0..=session.steps_read())
            .map(|k| {
                (
                    session.can_end_after(k),
                    sorted(session.next_options_after(k).into_iter().map(|o| o.symbol).collect()),
                )
            })
            .collect(),
        failure_point: session.failure_point(),
    }
}

/// Checks that the next options of each prefix come in the order of the
/// symbols in the checked grammar.
fn check_option_order(session: &Session, grammar: &CheckedGrammar, text: &str) {
    for k in 0..=session.steps_read() {
        let order: Vec<usize> = session
            .next_options_after(k)
            .iter()
            .map(|o| grammar.symbols.iter().position(|s| s.value == o.symbol).expect("a symbol of the grammar"))
            .collect();
        assert!(order.windows(2).all(|p| p[0] < p[1]), "next options out of order after {k} steps: {text}");
    }
}

fn observe_recogniser(recogniser: &mut Recogniser, sequence: &[String]) -> Observed {
    let w: Vec<u8> = sequence.iter().map(|s| terminal(s)).collect();
    let mut prefixes = Vec::new();
    for k in 0..=w.len() {
        // The empty prefix holds no step to reject. When the start rule derives
        // no sequence, its state cannot end and has no next option.
        if k > 0 && !recogniser.accepts(&w[..k]) {
            return Observed { prefixes, failure_point: Some(k) };
        }
        prefixes.push((recogniser.can_end(&w[..k]), recogniser.next_options(&w[..k])));
    }
    Observed { prefixes, failure_point: None }
}

/// The tracer agrees with the reference recogniser on random grammars and
/// sequences. A step back gives the state of a new trace, and attributes never
/// change a compared value.
#[test]
fn the_tracer_agrees_with_the_reference_recogniser() {
    let started = Instant::now();
    // The cases come from one seed in one order, as before. Only the checks
    // run on several threads, so the cases stay the same in every run.
    let mut rng = Lcg(0x7ace);
    let mut grammars: Vec<(String, CheckedGrammar, Vec<Vec<Step>>)> = Vec::new();
    let mut attempts = 0;
    while grammars.len() < MIN_GRAMMARS {
        attempts += 1;
        assert!(attempts < 20 * MIN_GRAMMARS, "too few grammars without errors: {} of {attempts}", grammars.len());
        let text = random_grammar(&mut rng);
        let checked = checked_grammar(&text);
        if checked.diagnostics.iter().any(|d| d.severity == Severity::Error) {
            continue;
        }
        let cases = sequences(&checked, &mut rng)
            .into_iter()
            .map(|sequence| {
                sequence
                    .iter()
                    .enumerate()
                    .map(|(i, s)| Step { symbol: s.clone(), attributes: random_attributes(&mut rng, i) })
                    .collect()
            })
            .collect();
        grammars.push((text, checked, cases));
    }
    let cases: usize = grammars.iter().map(|g| g.2.len()).sum();
    let without_sentence = grammars.iter().filter(|g| Generator::new(&g.1).sentence(&mut Lcg(0)).is_none()).count();
    let next = AtomicUsize::new(0);
    let times = Mutex::new([Duration::ZERO; 5]);
    let oracle_cases = Mutex::new(OracleCases::default());
    let threads = std::thread::available_parallelism().map_or(1, |n| n.get());
    std::thread::scope(|scope| {
        for _ in 0..threads {
            scope.spawn(|| {
                let mut mine = [Duration::ZERO; 5];
                let mut counted = OracleCases::default();
                while let Some((text, checked, cases)) = grammars.get(next.fetch_add(1, AtomicOrdering::Relaxed)) {
                    check_grammar_cases(text, checked, cases, &mut mine, &mut counted);
                }
                let mut oracle_cases = oracle_cases.lock().expect("the oracle cases");
                oracle_cases.compared += counted.compared;
                oracle_cases.skipped.extend(counted.skipped);
                let mut times = times.lock().expect("the times");
                for (total, t) in times.iter_mut().zip(mine) {
                    *total += t;
                }
            });
        }
    });
    let [recogniser_time, invariant_time, back_time, attribute_time, oracle_time] = *times.lock().expect("the times");
    let OracleCases { compared, mut skipped } = oracle_cases.into_inner().expect("the oracle cases");
    skipped.sort();
    assert!(without_sentence > 0, "no grammar tested a start rule without a sentence");
    eprintln!(
        "the tracer agreed on {cases} cases of {} grammars ({without_sentence} without a sentence) in {:?} on \
         {threads} threads. The threads spent {recogniser_time:?} in the recogniser, {invariant_time:?} in the \
         invariants, {back_time:?} in the step backs, {attribute_time:?} in the attributes and {oracle_time:?} in \
         the tree oracle.",
        grammars.len(),
        started.elapsed()
    );
    eprintln!(
        "the tree oracle compared {compared} cases. It skipped {} cases, because a set would hold more than {} trees:",
        skipped.len(),
        tree_oracle::MAX_TREES
    );
    for case in &skipped {
        eprintln!("  {case}");
    }
    assert!(compared >= MIN_ORACLE_CASES, "the tree oracle compared only {compared} cases");
}

/// The cases that the tree oracle compared, and the cases that it skipped.
#[derive(Default)]
struct OracleCases {
    compared: usize,
    skipped: Vec<String>,
}

/// The checks of the cases of one grammar. `times` adds the time of the
/// recogniser, the invariants, the step backs, the attributes and the tree
/// oracle. `oracle_cases` counts the cases of the tree oracle.
fn check_grammar_cases(
    text: &str,
    checked: &CheckedGrammar,
    cases: &[Vec<Step>],
    times: &mut [Duration; 5],
    oracle_cases: &mut OracleCases,
) {
    let grammar = TraceGrammar::new(text).expect("a grammar without errors");
    let mut recogniser = Recogniser::new(checked);
    let mut oracle = TreeOracle::new(checked);
    for with in cases {
        let sequence: Vec<String> = with.iter().map(|s| s.symbol.clone()).collect();
        let steps: Vec<Step> = sequence.iter().map(Step::new).collect();
        let session = Session::with_steps(&grammar, steps.clone()).expect("valid steps");
        let observed = observe_tracer(&session);
        check_option_order(&session, checked, text);
        let timer = Instant::now();
        let expected = observe_recogniser(&mut recogniser, &sequence);
        times[0] += timer.elapsed();
        assert_eq!(
            observed.prefixes, expected.prefixes,
            "the tracer and the recogniser disagree\ngrammar:\n{text}sequence: {sequence:?}"
        );
        assert_eq!(observed.failure_point, expected.failure_point, "grammar:\n{text}sequence: {sequence:?}");
        let timer = Instant::now();
        let readings = check_invariants(&session, &format!("{text}{sequence:?}"));
        check_capped_lists(&session, &readings, &format!("{text}{sequence:?}"));
        times[1] += timer.elapsed();

        let timer = Instant::now();
        if check_tree_oracle(&mut oracle, &readings, &sequence, &format!("{text}{sequence:?}")) {
            oracle_cases.compared += 1;
        } else {
            oracle_cases.skipped.push(format!("{} {sequence:?}", text.replace('\n', " ")));
        }
        times[4] += timer.elapsed();

        // A step back gives the state of a new trace of the shorter prefix.
        let timer = Instant::now();
        let mut back = session.clone();
        while back.back() {
            let k = back.steps_read();
            let fresh = Session::with_steps(&grammar, steps[..k].to_vec()).expect("valid steps");
            assert_eq!(observe_tracer(&back).prefixes, observe_tracer(&fresh).prefixes, "{text}{sequence:?} at {k}");
            assert_eq!(back.state().next_options, fresh.state().next_options, "{text}{sequence:?} at {k}");
            // The readings of a new trace of the prefix are the readings of
            // the prefix in the whole trace, which the invariants checked.
            assert_eq!(back.readings_after(k), readings[k], "{text}{sequence:?} at {k}");
        }
        times[2] += timer.elapsed();

        // Random attributes change no compared value, and they come back.
        let timer = Instant::now();
        let session = Session::with_steps(&grammar, with.clone()).expect("valid steps");
        assert_eq!(observe_tracer(&session), observed, "attributes changed a value: {text}{sequence:?}");
        let again = Session::from_trace_file(&grammar, &session.save()).expect("Save reads back").session;
        assert_eq!(again.steps(), with.as_slice(), "the attributes changed: {text}{sequence:?}");
        times[3] += timer.elapsed();
    }
}

/// The reference recogniser agrees with hand-checked cases, so that a fault of
/// the recogniser cannot hide behind a matching fault of the tracer.
#[test]
fn the_reference_recogniser_agrees_with_hand_checked_cases() {
    // The grammar, the prefix, whether the recogniser accepts it, whether the
    // sequence can end, and the next options.
    type Case<'a> = (&'a str, &'a [&'a str], bool, bool, &'a [&'a str]);
    let cases: &[Case] = &[
        ("r0 = r0 \"x\" | \"y\" ;", &["y"], true, true, &["x"]),
        ("r0 = r0 \"x\" | \"y\" ;", &["x"], false, false, &[]),
        ("r0 = \"x\" r0 ;", &[], false, false, &[]),
        ("r0 = r1 | \"y\" ;\nr1 = \"x\" r1 ;", &[], true, false, &["y"]),
        ("r0 = (\"x\"){2,3} \"y\" ;", &["x"], true, false, &["x"]),
        ("r0 = (\"x\"){2,3} \"y\" ;", &["x", "x", "x"], true, false, &["y"]),
        ("r0 = (\"x\"){2,} ;", &["x", "x", "x"], true, true, &["x"]),
        ("r0 = r1 \"z\" ;\nr1 = (\"x\")? ;", &[], true, false, &["x", "z"]),
    ];
    for (rules, prefix, accepts, can_end, next) in cases {
        let text = format!("%start r0 ;\n{rules}\n");
        let checked = checked_grammar(&text);
        assert!(!checked.has_errors(), "{text}: {:?}", checked.diagnostics);
        let mut recogniser = Recogniser::new(&checked);
        let w: Vec<u8> = prefix.iter().map(|s| terminal(s)).collect();
        assert_eq!(recogniser.accepts(&w), *accepts, "{text}{prefix:?}");
        assert_eq!(recogniser.can_end(&w), *can_end, "{text}{prefix:?}");
        assert_eq!(
            recogniser.next_options(&w),
            next.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
            "{text}{prefix:?}"
        );
    }
}

// The invariants of the output-view spec, section "Test seams".

fn holds_step(node: &TreeNode) -> bool {
    matches!(node.kind, TreeKind::Step { .. }) || node.children.iter().any(holds_step)
}

/// Whether two trees are the same, whatever their origins.
fn same_tree(a: &TreeNode, b: &TreeNode) -> bool {
    a.kind == b.kind
        && a.children.len() == b.children.len()
        && a.children.iter().zip(&b.children).all(|(x, y)| same_tree(x, y))
}

/// Whether each origin of each node of `a` is an origin of the matching node
/// of `b`, in two trees that are the same.
fn origins_within(a: &TreeNode, b: &TreeNode) -> bool {
    a.origins.iter().all(|o| b.origins.contains(o))
        && a.children.iter().zip(&b.children).all(|(x, y)| origins_within(x, y))
}

/// The tree without step k and the nodes that hold only step k.
fn without_step(node: &TreeNode, k: usize) -> Option<TreeNode> {
    match node.kind {
        TreeKind::Step { number, .. } if number == k => None,
        TreeKind::Step { .. } => Some(node.clone()),
        _ => {
            let children: Vec<TreeNode> = node.children.iter().filter_map(|c| without_step(c, k)).collect();
            children.iter().any(holds_step).then(|| TreeNode { children, ..node.clone() })
        }
    }
}

fn check_node_steps(node: &TreeNode, root: bool) -> bool {
    (root || holds_step(node)) && node.children.iter().all(|c| check_node_steps(c, false))
}

/// The three invariants after each prefix: no two readings have the same
/// tree, each node holds a step except the start rule node, and each reading
/// extends the reading that it names as its parent.
fn check_invariants(session: &Session, case: &str) -> Vec<Readings> {
    let all = session.readings_of_each_prefix();
    let mut before: Option<&Readings> = None;
    for (k, now) in all.iter().enumerate() {
        for (i, a) in now.readings.iter().enumerate() {
            for b in &now.readings[..i] {
                assert!(!same_tree(&a.tree, &b.tree), "two readings have one tree after {k} steps: {case}");
            }
            let root_alone = k == 0 && a.tree.children.is_empty() && matches!(a.tree.kind, TreeKind::Rule { .. });
            assert!(root_alone || check_node_steps(&a.tree, k == 0), "a node holds no step after {k} steps: {case}");
            let Some(before) = before else { continue };
            match a.parent {
                Some(p) => {
                    let parent = &before.readings[p - 1].tree;
                    match without_step(&a.tree, k) {
                        Some(stripped) => {
                            assert!(same_tree(&stripped, parent), "reading {} after {k} steps: {case}", i + 1);
                            assert!(origins_within(&stripped, parent), "an origin appears after {k} steps: {case}");
                        }
                        None => assert!(k == 1 && parent.children.is_empty(), "{case}"),
                    }
                }
                None => assert!(before.cap_reached, "reading {} after {k} steps has no parent: {case}", i + 1),
            }
        }
        before = Some(now);
    }
    all
}

/// The longest prefix for the comparison with uncapped lists.
const UNCAPPED_STEPS: usize = 6;

/// The capped lists give the readings of lists without a practical cap, for
/// prefixes of at most 6 steps.
fn check_capped_lists(session: &Session, readings: &[Readings], case: &str) {
    for (k, capped) in readings.iter().enumerate().take(UNCAPPED_STEPS + 1) {
        assert_eq!(&session.readings_with_list_cap(k, 1_000_000), capped, "after {k} steps: {case}");
    }
}

// The tree oracle of the section "The reference recogniser".

/// The longest prefix that the tree oracle compares.
const ORACLE_STEPS: usize = 6;
/// The longest prefix for which the tree oracle compares the next options of
/// each reading.
const ORACLE_OPTION_STEPS: usize = 5;
/// The fewest cases that the tree oracle must compare.
const MIN_ORACLE_CASES: usize = 1_000;

/// The tree oracle of one grammar, with a memo of its readings by prefix. A
/// prefix holds the indices of its symbols in the checked grammar. `None`
/// means that the oracle skipped the prefix.
struct TreeOracle {
    grammar: tree_oracle::Grammar,
    contexts: tree_oracle::Contexts,
    memo: HashMap<Vec<usize>, Option<Rc<tree_oracle::Readings>>>,
}

/// The next options of one reading: each symbol, with its occurrences.
type Options = Vec<(String, Vec<Span>)>;

impl TreeOracle {
    fn new(checked: &CheckedGrammar) -> Self {
        Self {
            grammar: tree_oracle::Grammar::new(checked),
            contexts: tree_oracle::Contexts::default(),
            memo: HashMap::new(),
        }
    }

    fn readings(&mut self, w: &[usize]) -> Option<Rc<tree_oracle::Readings>> {
        if let Some(found) = self.memo.get(w) {
            return found.clone();
        }
        let found = tree_oracle::readings(&self.grammar, &mut self.contexts, w).map(Rc::new);
        self.memo.insert(w.to_vec(), found.clone());
        found
    }

    /// The index of the parent of a tree after k steps among the readings
    /// after k−1 steps.
    fn parent(&self, tree: &[tree_oracle::Token], k: usize, before: &tree_oracle::Readings) -> usize {
        let stripped = tree_oracle::without_last_step(tree, k, self.grammar.start());
        before
            .trees
            .iter()
            .position(|t| tree_oracle::same_tree(t, &stripped))
            .expect("the oracle finds the parent of each reading")
    }

    /// The next options of each reading of the prefix `w`. They come from the
    /// readings one step longer that name the reading as their parent. The
    /// origins of their last steps give the occurrences.
    fn next_options(&mut self, w: &[usize], now: &tree_oracle::Readings) -> Option<Vec<Options>> {
        let k = w.len() + 1;
        let mut options: Vec<Options> = vec![Vec::new(); now.trees.len()];
        for symbol in 0..self.grammar.symbols.len() {
            let mut longer = w.to_vec();
            longer.push(symbol);
            let after = self.readings(&longer)?;
            for tree in &after.trees {
                let parent = self.parent(tree, k, now);
                let step = tree.iter().find(|t| t.label == tree_oracle::Label::Step(k)).expect("the last step");
                let name = &self.grammar.symbols[symbol];
                let list = &mut options[parent];
                if list.last().is_none_or(|(s, _)| s != name) {
                    list.push((name.clone(), Vec::new()));
                }
                let occurrences = &mut list.last_mut().expect("an option").1;
                for span in &step.origins {
                    if let Err(at) = occurrences.binary_search(span) {
                        occurrences.insert(at, *span);
                    }
                }
            }
        }
        Some(options)
    }

    fn label(&self, label: tree_oracle::Label) -> String {
        let names = &self.grammar.rule_names;
        match label {
            tree_oracle::Label::Rule(r) => format!("rule {}", names[r]),
            tree_oracle::Label::Loop(kind, rule) => {
                format!("loop {} {}", kind.as_str(), rule.map_or("-", |r| names[r].as_str()))
            }
            tree_oracle::Label::Pass => "pass".to_string(),
            tree_oracle::Label::Step(n) => format!("step {n}"),
        }
    }

    fn flat(&self, tree: &[tree_oracle::Token]) -> Vec<FlatNode> {
        tree.iter().map(|t| (t.depth, self.label(t.label), t.origins.clone())).collect()
    }
}

/// A tree node in pre-order: its depth, its label and its origins.
type FlatNode = (u32, String, Vec<Span>);

fn flatten(node: &TreeNode, depth: u32, out: &mut Vec<FlatNode>) {
    let label = match &node.kind {
        TreeKind::Rule { name } => format!("rule {name}"),
        TreeKind::Loop { kind, pass_rule } => format!("loop {} {}", kind.as_str(), pass_rule.as_deref().unwrap_or("-")),
        TreeKind::Pass { .. } => "pass".to_string(),
        TreeKind::Step { number, .. } => format!("step {number}"),
    };
    out.push((depth, label, node.origins.clone()));
    for child in &node.children {
        flatten(child, depth + 1, out);
    }
}

fn show(tree: &[FlatNode]) -> String {
    tree.iter()
        .map(|(depth, label, origins)| {
            let origins: Vec<String> = origins.iter().map(|o| format!("{}..{}", o.start, o.end)).collect();
            format!("{}{label} [{}]\n", "  ".repeat(*depth as usize), origins.join(" "))
        })
        .collect()
}

/// Compares the readings of each prefix of at most 6 steps with the tree
/// oracle. It returns false when the oracle skips the case.
fn check_tree_oracle(oracle: &mut TreeOracle, readings: &[Readings], sequence: &[String], case: &str) -> bool {
    let cap = READING_CAP as usize;
    let w: Vec<usize> = sequence[..readings.len() - 1]
        .iter()
        .map(|s| oracle.grammar.symbols.iter().position(|g| g == s).expect("a step that the tracer read"))
        .collect();
    let mut before: Option<Rc<tree_oracle::Readings>> = None;
    for (k, tracer) in readings.iter().enumerate().take(ORACLE_STEPS + 1) {
        let Some(expected) = oracle.readings(&w[..k]) else { return false };
        let options = if k <= ORACLE_OPTION_STEPS {
            let Some(options) = oracle.next_options(&w[..k], &expected) else { return false };
            Some(options)
        } else {
            None
        };
        let found = expected.trees.len();
        assert_eq!(
            (tracer.readings.len(), tracer.cap_reached),
            (found.min(cap), found > cap),
            "the count of readings after {k} steps: {case}\nthe oracle found:\n{}",
            expected.trees.iter().map(|t| show(&oracle.flat(t))).collect::<Vec<_>>().join("--\n")
        );
        for (i, reading) in tracer.readings.iter().enumerate() {
            let mut tree = Vec::new();
            flatten(&reading.tree, 0, &mut tree);
            let want = oracle.flat(&expected.trees[i]);
            assert!(
                tree == want,
                "reading {} after {k} steps: {case}\nthe tracer:\n{}the oracle:\n{}",
                i + 1,
                show(&tree),
                show(&want)
            );
            assert_eq!(reading.can_end, expected.can_end[i], "can-end of reading {} after {k} steps: {case}", i + 1);
            let parent = before.as_ref().map(|b| oracle.parent(&expected.trees[i], k, b));
            assert_eq!(
                reading.parent,
                parent.filter(|p| *p < cap).map(|p| p + 1),
                "the parent of reading {} after {k} steps: {case}",
                i + 1
            );
            if let Some(options) = &options {
                let tracer_options: Options =
                    reading.next_options.iter().map(|o| (o.symbol.clone(), o.occurrences.clone())).collect();
                assert_eq!(tracer_options, options[i], "the next options of reading {} after {k} steps: {case}", i + 1);
            }
        }
        before = Some(expected);
    }
    true
}

/// The tree oracle agrees with hand-checked cases of the spec `tracer`, so
/// that a fault of the oracle cannot hide behind a matching fault of the
/// tracer.
#[test]
fn the_tree_oracle_agrees_with_hand_checked_cases() {
    // The rules, the prefix, the count of readings, and for the first reading
    // whether it can end and its next options.
    type Case<'a> = (&'a str, &'a [&'a str], usize, bool, &'a [&'a str]);
    let cases: &[Case] = &[
        ("r0 = (\"x\" | \"x\" \"x\")* ;", &["x", "x", "x"], 3, true, &["x"]),
        ("r0 = (\"x\" | \"x\" \"x\")* ;", &["x", "x", "x", "x", "x", "x"], 13, true, &["x"]),
        // The three cases of the section "The open layer".
        ("r0 = r0 \"x\" | \"y\" ;", &["y"], 1, true, &["x"]),
        ("r0 = r1 \"x\" | \"y\" ;\nr1 = r0 \"z\" ;", &["y"], 1, true, &["z"]),
        ("r0 = r1 \"x\" | \"z\" ;\nr1 = r0 \"y\" | r1 \"z\" | \"x\" ;", &["z"], 1, true, &["y"]),
        // Rule 3 of the section "Loops through rules".
        ("r0 = r0 \"x\" | \"y\" r0 | \"z\" ;", &["y", "z", "x"], 2, true, &["x"]),
        ("r0 = \"x\" r0 ;", &[], 0, false, &[]),
    ];
    for (rules, prefix, count, can_end, next) in cases {
        let text = format!("%start r0 ;\n{rules}\n");
        let checked = checked_grammar(&text);
        assert!(!checked.has_errors(), "{text}: {:?}", checked.diagnostics);
        let mut oracle = TreeOracle::new(&checked);
        let w: Vec<usize> =
            prefix.iter().map(|s| oracle.grammar.symbols.iter().position(|g| g == s).expect("a symbol")).collect();
        let readings = oracle.readings(&w).expect("small sets");
        assert_eq!(readings.trees.len(), *count, "{text}{prefix:?}");
        if *count == 0 {
            continue;
        }
        assert_eq!(readings.can_end[0], *can_end, "{text}{prefix:?}");
        let options = oracle.next_options(&w, &readings).expect("small sets");
        let symbols: Vec<&str> = options[0].iter().map(|(s, _)| s.as_str()).collect();
        assert_eq!(symbols, *next, "{text}{prefix:?}");
    }
    // The order of the spec: `[a a] [a]`, `[a] [a a]`, `[a] [a] [a]`.
    let checked = checked_grammar("%start r0 ;\nr0 = (\"x\" | \"x\" \"x\")* ;\n");
    let mut oracle = TreeOracle::new(&checked);
    let readings = oracle.readings(&[0, 0, 0]).expect("small sets");
    let passes: Vec<Vec<usize>> = readings
        .trees
        .iter()
        .map(|tree| {
            let steps = |at: usize| tree[at + 1..].iter().take_while(move |t| t.depth > tree[at].depth);
            (0..tree.len())
                .filter(|&at| tree[at].label == tree_oracle::Label::Pass)
                .map(|at| steps(at).filter(|t| matches!(t.label, tree_oracle::Label::Step(_))).count())
                .collect()
        })
        .collect();
    assert_eq!(passes, [vec![2, 1], vec![1, 2], vec![1, 1, 1]]);
}
