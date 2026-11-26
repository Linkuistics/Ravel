//! The property tests of the marks at the core interface:
//! `docs/specs/rule-diagrams.md`, section "Test seams".
//!
//! For each prefix of at most 6 steps, the test takes each listed reading, each
//! of its next options and each open path of at most 3 rules from the start
//! rule. It traces the prefix with that step added, with lists that have no
//! cap. The readings after the step whose parent is the listed reading are its
//! successors. The four rules give the marks of those successors, and the test
//! compares them with the marks of the operation Marks, which come from the cut
//! successors. It skips an option when the step without a cap gives 3,000
//! readings or more, and it reports each skipped option.
//!
//! The first set of grammars is the set of the tracer spec. The second set adds
//! a bare reference to another rule as an alternative of some rules, so more
//! loops through rules gain links that make no node.

use std::{
    collections::BTreeSet,
    sync::{
        Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    time::Instant,
};

use workbench_core::{Session, TraceGrammar, marks::Element, trace::Step};

// The bounds of the section "Test seams".
const GRAMMARS: usize = 1_000;
const MAX_STEPS: usize = 6;
const MAX_OPEN_PATH: usize = 3;
const MAX_READINGS: usize = 3_000;
const WALKS: usize = 3;

// The generators of the tracer spec, section "The generators".
const MAX_RULES: usize = 4;
const MAX_DEPTH: u32 = 3;
const SYMBOLS: [&str; 3] = ["x", "y", "z"];

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

/// A random grammar text: 1 to 4 rules over the symbols `x`, `y` and `z`, with
/// `r0` as the start rule. With `bare`, a rule can also take a bare reference
/// to another rule as an alternative.
fn random_grammar(rng: &mut Lcg, bare: bool) -> String {
    let rules = 1 + rng.below(MAX_RULES);
    let mut text = String::from("%start r0 ;\n");
    for r in 0..rules {
        let body = random_expr(rng, MAX_DEPTH, rules);
        if bare && rng.chance(1, 3) {
            text.push_str(&format!("r{r} = r{} | ({body}) ;\n", rng.below(rules)));
        } else if rng.chance(1, 2) {
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

fn steps(symbols: &[String]) -> Vec<Step> {
    symbols.iter().map(Step::new).collect()
}

/// The prefixes of one grammar: the prefixes of random walks over the next
/// options, each once.
fn prefixes(grammar: &TraceGrammar, rng: &mut Lcg) -> BTreeSet<Vec<String>> {
    let mut seen = BTreeSet::new();
    for _ in 0..WALKS {
        let mut prefix: Vec<String> = Vec::new();
        loop {
            seen.insert(prefix.clone());
            if prefix.len() == MAX_STEPS {
                break;
            }
            let session = Session::with_steps(grammar, steps(&prefix)).expect("valid steps");
            let options: Vec<String> = session.next_options_after(prefix.len()).into_iter().map(|o| o.symbol).collect();
            if options.is_empty() {
                break;
            }
            prefix.push(options[rng.below(options.len())].clone());
        }
    }
    seen
}

#[derive(Default)]
struct Tally {
    prefixes: usize,
    compared: usize,
    with_marks: usize,
    mismatches: Vec<String>,
    skipped: Vec<String>,
}

/// Compares the marks of the cut successors with the marks of the uncapped
/// successors on each listed reading, next option and open path of a prefix.
fn compare_prefix(grammar: &TraceGrammar, text: &str, prefix: &[String], tally: &mut Tally) {
    let session = Session::with_steps(grammar, steps(prefix)).expect("valid steps");
    if session.steps_read() != prefix.len() {
        return;
    }
    tally.prefixes += 1;
    let readings = session.readings_after(prefix.len()).readings;
    let options: BTreeSet<String> =
        readings.iter().flat_map(|r| r.next_options.iter().map(|o| o.symbol.clone())).collect();
    let open_paths = grammar.reference_chains(MAX_OPEN_PATH);
    let marks: Vec<_> = open_paths.iter().map(|path| session.marks(path, None).expect("a valid open path")).collect();
    for symbol in &options {
        let Some(step) = session.uncapped_step(symbol, MAX_READINGS) else {
            tally.skipped.push(format!("{text}after {prefix:?}, the option {symbol}"));
            continue;
        };
        for (path, marks) in open_paths.iter().zip(&marks) {
            for reading in 0..readings.len() {
                let expected = session.uncapped_marks(&step, reading, path);
                let mut got: Vec<Element> = Vec::new();
                if let Some(r) = marks.readings.iter().find(|r| r.reading == reading + 1) {
                    if let Some(option) = r.options.iter().find(|o| &o.symbol == symbol) {
                        got.extend(option.elements.iter().copied());
                    }
                    if r.end.as_ref().is_some_and(|end| end.options.contains(symbol)) {
                        got.push(Element::EndEvent);
                    }
                }
                tally.compared += 1;
                if !expected.is_empty() {
                    tally.with_marks += 1;
                }
                if got != expected {
                    tally.mismatches.push(format!(
                        "{text}after {prefix:?}, reading {}, option {symbol}, open path {path:?}:\n  uncapped \
                         {expected:?}\n  cut {got:?}",
                        reading + 1
                    ));
                }
            }
        }
    }
}

fn run(bare: bool, seed: u64) {
    let started = Instant::now();
    let mut rng = Lcg(seed);
    let mut cases: Vec<(String, BTreeSet<Vec<String>>)> = Vec::new();
    let mut attempts = 0;
    while cases.len() < GRAMMARS {
        attempts += 1;
        assert!(attempts < 20 * GRAMMARS, "too few grammars without errors: {} of {attempts}", cases.len());
        let text = random_grammar(&mut rng, bare);
        let Ok(grammar) = TraceGrammar::new(&text) else { continue };
        let prefixes = prefixes(&grammar, &mut rng);
        cases.push((text, prefixes));
    }
    // The cases come from one seed in one order. Only the checks run on
    // several threads, so the cases stay the same in every run.
    let next = AtomicUsize::new(0);
    let total = Mutex::new(Tally::default());
    let threads = std::thread::available_parallelism().map_or(1, |n| n.get());
    std::thread::scope(|scope| {
        for _ in 0..threads {
            scope.spawn(|| {
                let mut mine = Tally::default();
                while let Some((text, prefixes)) = cases.get(next.fetch_add(1, Ordering::Relaxed)) {
                    let grammar = TraceGrammar::new(text).expect("a grammar without errors");
                    for prefix in prefixes {
                        compare_prefix(&grammar, text, prefix, &mut mine);
                    }
                }
                let mut total = total.lock().expect("the tally");
                total.prefixes += mine.prefixes;
                total.compared += mine.compared;
                total.with_marks += mine.with_marks;
                total.mismatches.extend(mine.mismatches);
                total.skipped.extend(mine.skipped);
            });
        }
    });
    let mut tally = total.into_inner().expect("the tally");
    tally.skipped.sort();
    tally.mismatches.sort();
    eprintln!(
        "marks (bare references {bare}): {GRAMMARS} grammars, {} prefixes, {} comparisons ({} with marks), {} \
         mismatches, {} skipped options, in {:?} on {threads} threads",
        tally.prefixes,
        tally.compared,
        tally.with_marks,
        tally.mismatches.len(),
        tally.skipped.len(),
        started.elapsed()
    );
    for skipped in &tally.skipped {
        eprintln!("  skipped, because the uncapped step gives {MAX_READINGS} readings or more: {skipped}");
    }
    assert!(tally.with_marks > 0, "no comparison had marks");
    assert!(
        tally.mismatches.is_empty(),
        "{} mismatches, the first ones:\n{}",
        tally.mismatches.len(),
        tally.mismatches.iter().take(8).cloned().collect::<Vec<_>>().join("\n")
    );
}

#[test]
fn the_marks_of_cut_successors_agree_with_an_uncapped_step() {
    run(false, 0x7ace);
}

#[test]
fn the_marks_agree_on_grammars_with_bare_references() {
    run(true, 0x0b1e);
}
