//! The parser reads any text without a panic. Its tree keeps every byte, and
//! its output reads back through the mapping module.
//!
//! The texts come from fragments that stress the lexer and the parser: string
//! delimiters, escapes, line breaks of every kind, slips and characters
//! outside ASCII. A fixed seed keeps the run repeatable.

use std::{
    collections::BTreeSet,
    time::{Duration, Instant},
};

use workbench_core::{check_grammar, checked_grammar, from_kdl, parse};

const FRAGMENTS: &[&str] = &[
    "a", "rule_2", " ", "\t", "\n", "\r\n", "\r", "\u{2028}", "\u{85}", "\u{a0}", "\"", "\"\"\"", "#", "##", "\\",
    "\\u{", "\\u{41}", "\\n", "'", "=", "::=", "->", ":", ";", "|", "(", ")", "(*", "*)", "/*", "*/", "//", "///",
    "//!", "?", "*", "+", "{", "}", ",", "[", "]", "&", "%doc", "%start", "%", "%x", "0", "02", "17", "é", "日本",
    "\u{7}", "\u{feff}", "!", "x\"", "\"x\"", "#\"r\"#",
];

/// A small linear congruential generator, so the test needs no dependency.
struct Lcg(u64);

impl Lcg {
    fn next(&mut self) -> usize {
        self.0 = self.0.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407);
        (self.0 >> 33) as usize
    }
}

#[test]
fn any_text_parses_without_a_panic() {
    let mut rng = Lcg(0x5eed);
    let mut codes = BTreeSet::new();
    for _ in 0..4_000 {
        let len = rng.next() % 24;
        let text: String = (0..len).map(|_| FRAGMENTS[rng.next() % FRAGMENTS.len()]).collect();
        assert_eq!(parse(&text).syntax().to_string(), text, "the tree lost bytes of {text:?}");
        let output = check_grammar(&text);
        match from_kdl(&output) {
            Ok(checked) => codes.extend(checked.diagnostics.iter().map(|d| d.code)),
            Err(e) => panic!("the output for {text:?} does not read back: {e}\n{output}"),
        }
    }
    // The texts must reach most syntax errors, or the test proves little.
    assert!(codes.len() >= 20, "the texts reached only {} codes: {codes:?}", codes.len());
}

/// A guard, not a measurement: a long run of symbols parses quickly. Each
/// lookahead of the parser must take a constant time. With a lookahead that
/// walks the rest of the run, the time grows with the cube of the run length.
/// In a release build, 2,000 symbols then took 0.7 s. By that growth, 5,000
/// symbols take about 11 s, far above the limit.
#[test]
fn a_long_run_of_symbols_parses_quickly() {
    let text = format!("%start a ;\na = {};\n", "\"x\" ".repeat(5_000));
    let started = Instant::now();
    let checked = checked_grammar(&text);
    let elapsed = started.elapsed();
    assert!(checked.diagnostics.is_empty(), "{:?}", checked.diagnostics);
    assert!(elapsed < Duration::from_secs(2), "5,000 symbols took {elapsed:?}");
}
