//! The trace cases at the core interface, the first test seam of the slice:
//! `docs/specs/tracer.md`, section "Test seams".
//!
//! A trace fixture for `NAME.ebnf` is a trace file `NAME--DETAIL.trace.kdl` in
//! the same directory. The test reads the sequence of the file, traces it, and
//! compares Save with the whole file. So one file is the input and the expected
//! output. Set `UPDATE_EXPECT=1`, or run `task test:update`, to rewrite the
//! derived nodes of each fixture.
//!
//! Two directories hold other cases:
//!
//! - `tests/traces/faults` holds one fault of the trace file in each case. The
//!   detail of the name starts with the code of the fault. The expected
//!   diagnostic is in `NAME--DETAIL.diagnostics` beside the case.
//! - `tests/traces/warnings` holds a case with one warning. Save writes the
//!   file again with the hash of the grammar text.
//!
//! Each example grammar has a fixture for each line of its file of example
//! sequences, `NAME.sequences.kdl`. The line states whether the grammar accepts
//! the sequence, and the failure point of a rejected sequence.

use std::{
    fs,
    path::{Path, PathBuf},
};

use kdl::{KdlDocument, KdlValue};
use workbench_core::{
    Session, TraceGrammar,
    trace::{Attribute, AttributeValue, OpenError, ReadOutcome, Step, TraceCode, TraceDiagnostic},
};

fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn examples_dir() -> PathBuf {
    manifest_dir().join("../../examples")
}

fn traces_dir() -> PathBuf {
    manifest_dir().join("tests/traces")
}

fn update_mode() -> bool {
    std::env::var_os("UPDATE_EXPECT").is_some()
}

/// The files under a directory whose names end with `suffix`, in order.
fn files_ending(dir: &Path, suffix: &str) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let mut dirs = vec![dir.to_path_buf()];
    while let Some(dir) = dirs.pop() {
        for entry in fs::read_dir(&dir).unwrap_or_else(|e| panic!("{}: {e}", dir.display())) {
            let path = entry.expect("a directory entry").path();
            if path.is_dir() {
                dirs.push(path);
            } else if path.file_name().and_then(|n| n.to_str()).is_some_and(|n| n.ends_with(suffix)) {
                files.push(path);
            }
        }
    }
    files.sort();
    files
}

/// The name of a fixture without `.trace.kdl`, split at each `--`.
fn name_parts(fixture: &Path) -> Vec<String> {
    let name = fixture.file_name().and_then(|n| n.to_str()).unwrap_or_default();
    name.trim_end_matches(".trace.kdl").split("--").map(str::to_string).collect()
}

/// The grammar of a fixture: `NAME.ebnf` in the same directory.
fn grammar_of(fixture: &Path) -> PathBuf {
    fixture.with_file_name(format!("{}.ebnf", name_parts(fixture)[0]))
}

fn read(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

/// Compares an output with an expected file, or rewrites the file in update mode.
fn compare(expected_path: &Path, output: &str) -> Result<(), String> {
    let expected = fs::read_to_string(expected_path).unwrap_or_default();
    if expected == output {
        return Ok(());
    }
    if update_mode() {
        return fs::write(expected_path, output).map_err(|e| e.to_string());
    }
    if expected.is_empty() {
        return Err(format!("{} is missing; run `task test:update`", expected_path.display()));
    }
    Err(format!("the output differs from {}:\n{}", expected_path.display(), diff(&expected, output)))
}

fn diff(expected: &str, actual: &str) -> String {
    let expected: Vec<&str> = expected.lines().collect();
    let actual: Vec<&str> = actual.lines().collect();
    let mut out = String::new();
    for i in 0..expected.len().max(actual.len()) {
        match (expected.get(i), actual.get(i)) {
            (Some(e), Some(a)) if e == a => {}
            (e, a) => {
                if let Some(e) = e {
                    out.push_str(&format!("{:>4} - {e}\n", i + 1));
                }
                if let Some(a) = a {
                    out.push_str(&format!("{:>4} + {a}\n", i + 1));
                }
            }
        }
    }
    out
}

fn grammar(path: &Path) -> TraceGrammar {
    TraceGrammar::new(&read(path)).unwrap_or_else(|d| panic!("{}: {d:?}", path.display()))
}

/// Reports every failure of a list of cases at once.
fn run_all(cases: &[PathBuf], run: impl Fn(&Path) -> Result<(), String>) {
    assert!(!cases.is_empty(), "no cases");
    let failures: Vec<String> =
        cases.iter().filter_map(|case| run(case).err().map(|e| format!("{}:\n{e}", case.display()))).collect();
    assert!(failures.is_empty(), "{} of {} cases failed:\n\n{}", failures.len(), cases.len(), failures.join("\n\n"));
}

fn is_special(fixture: &Path) -> bool {
    fixture.components().any(|c| matches!(c.as_os_str().to_str(), Some("faults" | "warnings")))
}

fn run_fixture(fixture: &Path) -> Result<(), String> {
    let text = read(fixture);
    let grammar = grammar(&grammar_of(fixture));
    let opened = Session::from_trace_file(&grammar, &text).map_err(|e| format!("the fixture does not open: {e:?}"))?;
    if !opened.warnings.is_empty() {
        return Err(format!("unexpected warnings: {:?}", opened.warnings));
    }
    let saved = opened.session.save();
    // The saved file must read back to the same text.
    let again = Session::from_trace_file(&grammar, &saved).map_err(|e| format!("Save does not read back: {e:?}"))?;
    if again.session.save() != saved {
        return Err("Save changes in a round trip".into());
    }
    // A stable text is not enough: a lossy Save can be stable after one round.
    if again.session.steps() != opened.session.steps() {
        return Err("the steps change in a round trip through Save".into());
    }
    compare(fixture, &saved)
}

/// Every trace fixture in `tests/traces` and in `examples`.
#[test]
fn trace_fixtures() {
    let mut cases: Vec<PathBuf> = files_ending(&traces_dir(), ".trace.kdl");
    cases.extend(files_ending(&examples_dir(), ".trace.kdl"));
    cases.retain(|c| !is_special(c));
    run_all(&cases, run_fixture);
}

/// One line of a file of example sequences.
struct ExampleSequence {
    accepted: bool,
    symbols: Vec<String>,
    failure_point: Option<usize>,
}

fn example_sequences(path: &Path) -> Vec<ExampleSequence> {
    let document = KdlDocument::parse_v2(&read(path)).unwrap_or_else(|e| panic!("{}: {e:?}", path.display()));
    document
        .nodes()
        .iter()
        .map(|node| {
            let symbols = node
                .entries()
                .iter()
                .filter(|e| e.name().is_none())
                .map(|e| match e.value() {
                    KdlValue::String(s) => s.clone(),
                    other => panic!("{}: a step is a string, not {other:?}", path.display()),
                })
                .collect();
            let failure_point = node.get("failure-point").map(|v| match v {
                KdlValue::Integer(n) => usize::try_from(*n).expect("a step number"),
                other => panic!("{}: a failure point is an integer, not {other:?}", path.display()),
            });
            ExampleSequence { accepted: node.name().value() == "accepted", symbols, failure_point }
        })
        .collect()
}

/// Each line of each file of example sequences has its fixture. The tracer
/// agrees with the line, and the fixture holds the sequence of the line.
#[test]
fn example_sequences_have_fixtures() {
    let files = files_ending(&examples_dir(), ".sequences.kdl");
    run_all(&files, |file| {
        let name = file.file_name().and_then(|n| n.to_str()).unwrap_or_default().trim_end_matches(".sequences.kdl");
        let grammar = grammar(&file.with_file_name(format!("{name}.ebnf")));
        let mut counts = [0, 0];
        let mut kinds = [false, false];
        for sequence in example_sequences(file) {
            let kind = if sequence.accepted { "accepted" } else { "rejected" };
            let count = &mut counts[usize::from(!sequence.accepted)];
            *count += 1;
            kinds[usize::from(!sequence.accepted)] = true;
            let fixture = file.with_file_name(format!("{name}--{kind}-{count}.trace.kdl"));
            let steps: Vec<Step> = sequence.symbols.iter().map(Step::new).collect();
            let session = Session::with_steps(&grammar, steps.clone()).expect("valid steps");
            if sequence.accepted {
                if session.failure_point().is_some() || !session.state().can_end {
                    return Err(format!("{}: the tracer does not accept the sequence", fixture.display()));
                }
            } else if session.failure_point() != sequence.failure_point {
                return Err(format!(
                    "{}: the failure point is {:?}, not {:?}",
                    fixture.display(),
                    session.failure_point(),
                    sequence.failure_point
                ));
            }
            if !fixture.exists() {
                if !update_mode() {
                    return Err(format!("{} is missing; run `task test:update`", fixture.display()));
                }
                fs::write(&fixture, session.save()).map_err(|e| e.to_string())?;
            }
            let opened = Session::from_trace_file(&grammar, &read(&fixture)).map_err(|e| format!("{e:?}"))?;
            if opened.session.steps() != steps {
                return Err(format!("{} does not hold the sequence of its line", fixture.display()));
            }
        }
        if kinds != [true, true] {
            return Err("each example grammar needs an accepted and a rejected sequence".into());
        }
        Ok(())
    });
}

fn diagnostics_text(text: &str, diagnostics: &[TraceDiagnostic]) -> String {
    diagnostics
        .iter()
        .map(|d| {
            format!(
                "{} {} {}..{} {:?}\n    {}\n",
                d.severity.as_str(),
                d.code,
                d.span.start,
                d.span.end,
                &text[d.span.range()],
                d.message
            )
        })
        .collect()
}

/// Each fault of the trace file gives exactly one diagnostic, with the code at
/// the start of the detail of its name.
#[test]
fn trace_file_faults() {
    let cases = files_ending(&traces_dir().join("faults"), ".trace.kdl");
    run_all(&cases, |case| {
        let text = read(case);
        let code_name = name_parts(case).get(1).cloned().unwrap_or_default();
        let code = TraceCode::parse(&code_name).ok_or_else(|| format!("`{code_name}` is not a trace code"))?;
        let diagnostics = match Session::from_trace_file(&grammar(&grammar_of(case)), &text) {
            Err(OpenError::TraceFile(d)) => d,
            other => return Err(format!("expected the diagnostics of the file, found {other:?}")),
        };
        let codes: Vec<TraceCode> = diagnostics.iter().map(|d| d.code).collect();
        if codes != [code] {
            return Err(format!("expected exactly one `{code}` diagnostic, found {codes:?}"));
        }
        let expected = case.with_file_name(format!("{}.diagnostics", name_parts(case).join("--")));
        compare(&expected, &diagnostics_text(&text, &diagnostics))
    });
}

/// A differing grammar hash is a warning. The core still traces the sequence,
/// and Save writes the hash of the grammar text.
#[test]
fn trace_file_warnings() {
    let cases = files_ending(&traces_dir().join("warnings"), ".trace.kdl");
    run_all(&cases, |case| {
        let text = read(case);
        let code_name = name_parts(case).get(1).cloned().unwrap_or_default();
        let code = TraceCode::parse(&code_name).ok_or_else(|| format!("`{code_name}` is not a trace code"))?;
        let grammar = grammar(&grammar_of(case));
        let opened = Session::from_trace_file(&grammar, &text).map_err(|e| format!("{e:?}"))?;
        let codes: Vec<TraceCode> = opened.warnings.iter().map(|d| d.code).collect();
        if codes != [code] {
            return Err(format!("expected exactly one `{code}` warning, found {codes:?}"));
        }
        let expected = case.with_file_name(format!("{}.diagnostics", name_parts(case).join("--")));
        compare(&expected, &diagnostics_text(&text, &opened.warnings))?;
        let hash = &text[opened.warnings[0].span.range()];
        let corrected = text.replacen(hash, &format!("grammar-sha256=\"{}\"", grammar.grammar_sha256()), 1);
        if opened.session.save() != corrected {
            return Err(format!("Save differs:\n{}", diff(&corrected, &opened.session.save())));
        }
        Ok(())
    });
}

#[test]
fn every_trace_code_has_a_case() {
    let names: Vec<String> = files_ending(&traces_dir(), ".trace.kdl")
        .iter()
        .filter(|c| is_special(c))
        .filter_map(|c| name_parts(c).get(1).cloned())
        .collect();
    let missing: Vec<&str> =
        TraceCode::ALL.iter().map(|c| c.as_str()).filter(|c| !names.iter().any(|n| n == c)).collect();
    assert!(missing.is_empty(), "no case for {missing:?}");
}

/// For each prefix, whether the sequence can end and the next options.
type Prefixes = Vec<(bool, Vec<String>)>;

/// The state of a session, without the sequence after the steps read.
fn derived(session: &Session) -> (usize, Option<usize>, Prefixes) {
    let prefixes = (0..=session.steps_read())
        .map(|k| (session.can_end_after(k), session.next_options_after(k).into_iter().map(|o| o.symbol).collect()))
        .collect();
    (session.steps_read(), session.failure_point(), prefixes)
}

/// A step back gives the same state as a new trace of the shorter sequence,
/// for each example sequence.
#[test]
fn a_step_back_gives_the_state_of_a_new_trace() {
    for file in files_ending(&examples_dir(), ".sequences.kdl") {
        let name = file.file_name().and_then(|n| n.to_str()).unwrap_or_default().trim_end_matches(".sequences.kdl");
        let grammar = grammar(&file.with_file_name(format!("{name}.ebnf")));
        for sequence in example_sequences(&file) {
            let steps: Vec<Step> = sequence.symbols.iter().map(Step::new).collect();
            let mut session = Session::with_steps(&grammar, steps.clone()).expect("valid steps");
            loop {
                let k = session.steps_read();
                let fresh = Session::with_steps(&grammar, steps[..k].to_vec()).expect("valid steps");
                assert_eq!(fresh.steps_read(), k, "{name}: a new trace reads the {k} steps");
                assert_eq!(derived(&session).2, derived(&fresh).2, "{name} after {k} steps");
                assert_eq!(session.state().next_options, fresh.state().next_options, "{name} after {k} steps");
                if !session.back() {
                    break;
                }
                assert_eq!(session.failure_point(), None, "a step back clears the failure point");
            }
        }
    }
}

/// The interface reads one step at a time, steps back and inserts.
#[test]
fn the_interface_reads_steps_back_and_inserts() {
    let grammar = grammar(&traces_dir().join("faults/faults.ebnf"));
    let mut session = Session::new(&grammar);
    assert_eq!(session.read(), ReadOutcome::EndOfSequence);
    session.insert(Step::new("x")).unwrap();
    session.insert(Step::new("z")).unwrap();
    assert_eq!(session.steps().iter().map(|s| s.symbol.as_str()).collect::<Vec<_>>(), ["z", "x"]);
    assert_eq!(session.read(), ReadOutcome::Rejected { step: 1 });
    assert_eq!(session.state().failure_point, Some(1));
    // An insert before the rejected step clears the failure point.
    session.insert(Step::new("x")).unwrap();
    assert_eq!(session.failure_point(), None);
    assert_eq!(session.read(), ReadOutcome::Accepted);
    assert_eq!(session.read(), ReadOutcome::Rejected { step: 2 });
    assert!(session.back());
    assert!(!session.back());
    assert_eq!(session.state().steps.len(), 3, "a step back keeps the steps");
    assert_eq!(session.insert(Step::new("")).unwrap_err().code, TraceCode::EmptySymbol);
    assert_eq!(session.insert(Step::new("a\u{2028}b")).unwrap_err().code, TraceCode::LineBreakInSymbol);
    let mut twice = Step::new("x");
    let attribute = Attribute { key: "k".into(), value: AttributeValue::Null, annotation: None };
    twice.attributes = vec![attribute.clone(), attribute];
    assert_eq!(session.insert(twice).unwrap_err().code, TraceCode::DuplicateAttribute);
}

/// A session refuses a grammar with an error.
#[test]
fn a_session_refuses_a_grammar_with_an_error() {
    match Session::open("%start a ;\na = b ;\n", None) {
        Err(OpenError::Grammar(diagnostics)) => assert!(!diagnostics.is_empty()),
        other => panic!("expected the diagnostics of the grammar, found {other:?}"),
    }
}

/// Attributes pass through unchanged, and they never change acceptance.
#[test]
fn attributes_pass_through_and_never_change_acceptance() {
    let case = traces_dir().join("attributes/attributes--each-type.trace.kdl");
    let grammar = grammar(&grammar_of(&case));
    let with = Session::from_trace_file(&grammar, &read(&case)).expect("the case opens").session;
    let attributes: Vec<&Attribute> = with.steps().iter().flat_map(|s| &s.attributes).collect();
    for expected in [
        AttributeValue::String("starter/main.rs".into()),
        AttributeValue::Integer(16),
        AttributeValue::Bool(false),
        AttributeValue::Null,
    ] {
        assert!(attributes.iter().any(|a| a.value == expected), "no attribute {expected:?}");
    }
    assert!(attributes.iter().any(|a| matches!(a.value, AttributeValue::Float(_))));
    assert!(attributes.iter().any(|a| a.annotation.as_deref() == Some("url")));
    assert!(attributes.iter().any(|a| a.key == "symbol"), "an attribute can have the key `symbol`");

    let plain: Vec<Step> = with.steps().iter().map(|s| Step::new(s.symbol.clone())).collect();
    let without = Session::with_steps(&grammar, plain).expect("valid steps");
    assert_eq!(derived(&with), derived(&without));

    let saved = with.save();
    let again = Session::from_trace_file(&grammar, &saved).expect("Save reads back").session;
    assert_eq!(again.steps(), with.steps(), "the attributes come back unchanged");
}

/// The core can write a number in another notation, and the value stays.
#[test]
fn a_number_keeps_its_value_in_another_notation() {
    let grammar = grammar(&traces_dir().join("faults/faults.ebnf"));
    let text = "trace format-version=1 {\n    sequence {\n        step x n=0x10 m=1.5e3\n    }\n}\n";
    let session = Session::from_trace_file(&grammar, text).expect("the file opens").session;
    let saved = session.save();
    assert!(saved.contains("n=16"), "{saved}");
    let again = Session::from_trace_file(&grammar, &saved).expect("Save reads back").session;
    assert_eq!(again.steps(), session.steps());
}

/// Strings that KDL 2.0 must quote or escape. Each one is valid as a symbol,
/// an attribute key, a string value and a type annotation.
const AWKWARD_STRINGS: &[&str] = &[
    "key\u{1}",
    "a\u{7f}b",
    "\u{feff}x",
    "\u{200e}",
    "a\u{2028}b",
    "a\u{85}b",
    "a\u{b}b",
    "a\tb",
    "+.1",
    "-.5",
    ".5",
    "+1",
    "-",
    "+.",
    "-.a",
    "1",
    "0x10",
    "true",
    "nan",
    "-inf",
    "a=b",
    "a\u{fe66}b",
    "a\u{ff1d}b",
    "\u{1f7f0}",
    "#a",
    "(a)",
    "a;b",
    "a/b",
    "a b",
    "a\"b",
    "a\\b",
    "",
    "é",
];

/// Save writes every key, string value and type annotation as valid KDL 2.0,
/// and the attributes come back unchanged.
#[test]
fn save_writes_awkward_strings_as_valid_kdl() {
    let grammar = TraceGrammar::new("%start s ;\ns = \"x\" ;\n").expect("a grammar");
    let mut failures = Vec::new();
    for &text in AWKWARD_STRINGS {
        let attribute =
            Attribute { key: text.into(), value: AttributeValue::String(text.into()), annotation: Some(text.into()) };
        let steps = vec![Step { symbol: "x".into(), attributes: vec![attribute] }];
        let session = Session::with_steps(&grammar, steps.clone()).expect("valid steps");
        let saved = session.save();
        match Session::from_trace_file(&grammar, &saved) {
            Ok(again) if again.session.steps() == steps && again.session.save() == saved => {}
            Ok(again) => failures.push(format!("{text:?}: the steps differ: {:?}\n{saved}", again.session.steps())),
            Err(e) => failures.push(format!("{text:?}: Save does not read back: {e:?}\n{saved}")),
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
}

/// A symbol that looks like a number survives Save, as a step and as a next
/// option. So does the checked grammar of such a symbol.
#[test]
fn a_symbol_that_looks_like_a_number_survives_save() {
    let text = "%start s ;\ns = \"+.1\" \"-.5\" ;\n";
    let grammar = TraceGrammar::new(text).expect("a grammar");
    let session = Session::with_steps(&grammar, vec![Step::new("+.1")]).expect("valid steps");
    let saved = session.save();
    assert!(saved.contains("step \"+.1\"") && saved.contains("next symbol=\"-.5\""), "{saved}");
    let again = Session::from_trace_file(&grammar, &saved).expect("Save reads back").session;
    assert_eq!(again.steps(), session.steps());
    let checked = workbench_core::check_grammar(text);
    assert!(workbench_core::from_kdl(&checked).is_ok(), "{checked}");
}

/// The constructor with a sequence checks each step, as Insert does.
#[test]
fn a_session_with_steps_checks_each_step() {
    let grammar = TraceGrammar::new("%start s ;\ns = \"x\" ;\n").expect("a grammar");
    let mut twice = Step::new("x");
    twice.attributes = vec![
        Attribute { key: "k".into(), value: AttributeValue::Integer(1), annotation: None },
        Attribute { key: "k".into(), value: AttributeValue::Integer(2), annotation: None },
    ];
    for (steps, code) in [
        (vec![twice], TraceCode::DuplicateAttribute),
        (vec![Step::new("x"), Step::new("")], TraceCode::EmptySymbol),
        (vec![Step::new("a\nb")], TraceCode::LineBreakInSymbol),
    ] {
        let error = Session::with_steps(&grammar, steps).expect_err("an invalid step");
        assert_eq!(error.code, code, "{}", error.message);
    }
    let error = Session::with_steps(&grammar, vec![Step::new("x"), Step::new("")]).expect_err("an empty symbol");
    assert!(error.message.starts_with("Step 2: "), "{}", error.message);
}

/// The reader refuses a code point that KDL 2.0 forbids as a literal
/// character, with one diagnostic at that code point. The same code point as
/// an escape is valid, and a byte order mark can start the file.
#[test]
fn the_reader_refuses_a_forbidden_literal_character() {
    let grammar = TraceGrammar::new("%start s ;\ns = \"x\" ;\n").expect("a grammar");
    let file = |step: &str| format!("trace format-version=1 {{\n    sequence {{\n        {step}\n    }}\n}}\n");
    for text in [file("step x note=\"a\u{7f}b\""), file("step x // a\u{7f}b"), file("step x note=\"a\u{1}b\"")] {
        let diagnostics = match Session::from_trace_file(&grammar, &text) {
            Err(OpenError::TraceFile(d)) => d,
            other => panic!("expected the diagnostics of the file, found {other:?}: {text:?}"),
        };
        let codes: Vec<TraceCode> = diagnostics.iter().map(|d| d.code).collect();
        assert_eq!(codes, [TraceCode::NotKdl], "{text:?}");
        let at = text.find(['\u{7f}', '\u{1}']).expect("a forbidden character");
        assert_eq!(diagnostics[0].span.range().start, at, "{text:?}");
    }
    for text in [file("step x note=\"a\\u{7f}b\""), format!("\u{feff}{}", file("step x"))] {
        assert!(Session::from_trace_file(&grammar, &text).is_ok(), "{text:?}");
    }
}

/// The tracer stops on a grammar with a repeat whose body can match the
/// empty sequence, and on a rule that can derive itself with no step. Such a
/// grammar could give infinitely many trees for one sequence.
#[test]
fn the_tracer_stops_on_a_grammar_with_infinitely_many_trees() {
    for (text, code) in [
        ("%start a ;\na = (\"x\"?)* ;\n", "nullable-repetition"),
        ("%start a ;\na = b ;\nb = a | \"x\" ;\n", "cyclic-rule"),
    ] {
        match Session::open(text, None) {
            Err(OpenError::Grammar(diagnostics)) => {
                assert!(diagnostics.iter().any(|d| d.code.as_str() == code), "{text}: {diagnostics:?}")
            }
            other => panic!("{text}: expected the diagnostics of the grammar, found {other:?}"),
        }
    }
}
