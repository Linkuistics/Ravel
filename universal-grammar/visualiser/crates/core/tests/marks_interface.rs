//! The marks cases at the core interface, the first test seam of the slice:
//! `docs/specs/rule-diagrams.md`, section "Test seams".
//!
//! A marks case for `NAME.ebnf` is a file `NAME--DETAIL.marks.kdl` in
//! `tests/marks`. The grammar lies in the same directory, or in `examples`.
//! The case holds its input and its derived nodes, as a trace fixture does:
//!
//! - `sequence` holds the symbols of the steps. The session reads as many of
//!   them as it can, so a rejected step leaves the state before it.
//! - `focus` holds the number of the step in focus, if any.
//! - `open-path` names an open path beyond the follow paths, if any.
//!
//! The test writes the derived nodes: the `state`, with the follow path of each
//! listed reading, and the `marks` of each open path. The open paths are each
//! open path along the follow path of each listed reading, and then the paths
//! that the case names. Set `UPDATE_EXPECT=1`, or run `task test:update`, to
//! rewrite the derived nodes.
//!
//! Each case runs in a worker thread with a deadline of 60 seconds. An
//! evaluation that keeps no list for a projection on a cycle does not end the
//! case `cycle--twelve-steps`, so the deadline separates it from the
//! evaluation of the spec. The worker reads the grammar and the case itself,
//! because the syntax trees of `rowan` cannot move to another thread. At the
//! deadline, the test fails and names the case. It does not wait for the
//! worker thread.

use std::{
    fs,
    path::{Path, PathBuf},
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use kdl::{KdlDocument, KdlEntry, KdlNode, KdlValue};
use workbench_core::{Session, TraceGrammar, trace::Step};

/// The deadline of each case: `docs/specs/rule-diagrams.md`, section "Test
/// seams".
const DEADLINE: Duration = Duration::from_secs(60);

fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn marks_dir() -> PathBuf {
    manifest_dir().join("tests/marks")
}

fn update_mode() -> bool {
    std::env::var_os("UPDATE_EXPECT").is_some()
}

fn read(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

/// The marks cases, in the order of their names.
fn cases() -> Vec<PathBuf> {
    let mut cases: Vec<PathBuf> = fs::read_dir(marks_dir())
        .expect("the directory of the marks cases")
        .map(|entry| entry.expect("a directory entry").path())
        .filter(|path| path.file_name().and_then(|n| n.to_str()).is_some_and(|n| n.ends_with(".marks.kdl")))
        .collect();
    cases.sort();
    cases
}

fn case_name(case: &Path) -> String {
    case.file_name().and_then(|n| n.to_str()).unwrap_or_default().trim_end_matches(".marks.kdl").to_string()
}

/// The grammar of a case: `NAME.ebnf` beside it, or in `examples`.
fn grammar_of(case: &Path) -> PathBuf {
    let name = case_name(case);
    let stem = name.split("--").next().unwrap_or_default();
    let beside = marks_dir().join(format!("{stem}.ebnf"));
    if beside.exists() { beside } else { manifest_dir().join(format!("../../examples/{stem}.ebnf")) }
}

/// The input of a case.
struct Input {
    sequence: Vec<String>,
    focus: Option<usize>,
    open_paths: Vec<Vec<String>>,
}

fn strings(node: &KdlNode) -> Vec<String> {
    node.entries()
        .iter()
        .filter(|e| e.name().is_none())
        .map(|e| e.value().as_string().unwrap_or_else(|| panic!("a string argument in {node}")).to_string())
        .collect()
}

fn read_input(text: &str) -> Input {
    let document: KdlDocument = text.parse().unwrap_or_else(|e| panic!("a KDL document: {e:?}"));
    let root = document.get("marks-case").expect("a marks-case node");
    let mut input = Input { sequence: Vec::new(), focus: None, open_paths: Vec::new() };
    for node in root.children().map(|c| c.nodes()).unwrap_or_default() {
        match node.name().value() {
            "sequence" => input.sequence = strings(node),
            "focus" => {
                let value = node.entries().first().and_then(|e| e.value().as_integer()).expect("a step number");
                input.focus = Some(usize::try_from(value).expect("a step number"));
            }
            "open-path" => input.open_paths.push(strings(node)),
            "state" | "marks" => {}
            other => panic!("an unknown node {other} in a marks case"),
        }
    }
    input
}

fn node_with_strings(name: &str, values: &[String]) -> KdlNode {
    let mut node = KdlNode::new(name);
    for value in values {
        node.push(KdlEntry::new(value.as_str()));
    }
    node
}

/// The whole case file: the input, the state and the marks of each open path.
fn derive(grammar_text: &str, case_text: &str) -> String {
    let input = read_input(case_text);
    let grammar = TraceGrammar::new(grammar_text).unwrap_or_else(|d| panic!("a grammar without errors: {d:?}"));
    let steps = input.sequence.iter().map(Step::new).collect();
    let session = Session::with_steps(&grammar, steps).expect("valid steps");
    let n = session.steps_read();
    let readings = session.readings_after(n);

    let mut root = KdlNode::new("marks-case");
    let children = root.ensure_children();
    children.nodes_mut().push(node_with_strings("sequence", &input.sequence));
    if let Some(focus) = input.focus {
        let mut node = KdlNode::new("focus");
        node.push(KdlEntry::new(focus as i128));
        children.nodes_mut().push(node);
    }
    for path in &input.open_paths {
        children.nodes_mut().push(node_with_strings("open-path", path));
    }

    let mut state = KdlNode::new("state");
    state.push(KdlEntry::new_prop("steps-read", n as i128));
    if let Some(failure) = session.failure_point() {
        state.push(KdlEntry::new_prop("failure-point", failure as i128));
    }
    state.push(KdlEntry::new_prop("reading-count", readings.readings.len() as i128));
    state.push(KdlEntry::new_prop("cap-reached", KdlValue::Bool(readings.cap_reached)));
    for (i, reading) in readings.readings.iter().enumerate() {
        let mut node = node_with_strings("follow-path", &reading.follow_path);
        node.insert(0, KdlEntry::new((i + 1) as i128));
        state.ensure_children().nodes_mut().push(node);
    }
    children.nodes_mut().push(state);

    let mut open_paths: Vec<Vec<String>> = Vec::new();
    for reading in &readings.readings {
        for len in 1..=reading.follow_path.len() {
            let path = reading.follow_path[..len].to_vec();
            if !open_paths.contains(&path) {
                open_paths.push(path);
            }
        }
    }
    if open_paths.is_empty() {
        // A state without a reading opens the start rule.
        let start = grammar_text
            .lines()
            .find_map(|l| l.trim().strip_prefix("%start ").map(|r| r.trim_end_matches(';').trim().to_string()))
            .expect("a start rule");
        open_paths.push(vec![start]);
    }
    for path in &input.open_paths {
        if !open_paths.contains(path) {
            open_paths.push(path.clone());
        }
    }
    for path in &open_paths {
        let marks = session.marks(path, input.focus).unwrap_or_else(|e| panic!("the open path {path:?}: {e}"));
        let text = session.marks_kdl(&marks);
        let document: KdlDocument = text.parse().unwrap_or_else(|e| panic!("the marks KDL: {e:?}\n{text}"));
        children.nodes_mut().extend(document.nodes().iter().cloned());
    }

    let mut document = KdlDocument::new();
    document.nodes_mut().push(root);
    document.autoformat();
    document.to_string()
}

/// Runs one case in a worker thread, and waits for its output until the
/// deadline.
fn run_with_deadline(case: &Path) -> Result<(String, Duration), String> {
    let grammar_path = grammar_of(case);
    let case_path = case.to_path_buf();
    let (sender, receiver) = mpsc::channel();
    let started = Instant::now();
    thread::spawn(move || {
        let output = derive(&read(&grammar_path), &read(&case_path));
        let _ = sender.send(output);
    });
    match receiver.recv_timeout(DEADLINE) {
        Ok(output) => Ok((output, started.elapsed())),
        Err(mpsc::RecvTimeoutError::Timeout) => {
            Err(format!("the case gave no result within its deadline of {} seconds", DEADLINE.as_secs()))
        }
        Err(mpsc::RecvTimeoutError::Disconnected) => Err("the worker thread panicked".into()),
    }
}

/// Compares an output with the case file, or rewrites the file in update mode.
fn compare(case: &Path, output: &str) -> Result<(), String> {
    let expected = read(case);
    if expected == output {
        return Ok(());
    }
    if update_mode() {
        return fs::write(case, output).map_err(|e| e.to_string());
    }
    Err(format!("the output differs from the case file:\n{}", diff(&expected, output)))
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

#[test]
fn each_marks_case_gives_its_expected_marks_within_the_deadline() {
    let cases = cases();
    assert!(!cases.is_empty(), "no marks case in {}", marks_dir().display());
    let mut failures = Vec::new();
    for case in &cases {
        let name = case_name(case);
        match run_with_deadline(case) {
            Ok((output, took)) => {
                eprintln!("marks case {name}: {took:?}");
                if let Err(e) = compare(case, &output) {
                    failures.push(format!("{name}: {e}"));
                }
            }
            Err(e) => failures.push(format!("{name}: {e}")),
        }
    }
    assert!(failures.is_empty(), "{} of {} marks cases failed:\n{}", failures.len(), cases.len(), failures.join("\n"));
}

/// The errors of the operation Marks for an open path that breaks the rules of
/// the section "The interface of the core".
#[test]
fn an_open_path_that_is_no_chain_from_the_start_is_an_error() {
    let grammar = TraceGrammar::new("%start s ;\ns = a \"x\" ;\na = \"y\" ;\nu = a ;\n").expect("a grammar");
    let session = Session::new(&grammar);
    let path = |rules: &[&str]| rules.iter().map(|r| r.to_string()).collect::<Vec<_>>();
    assert!(session.marks(&path(&["s"]), None).is_ok());
    assert!(session.marks(&path(&["s", "a"]), None).is_ok());
    assert!(session.marks(&path(&["u", "a"]), None).is_ok());
    for bad in [&[][..], &["a"], &["s", "u"], &["q"], &["a", "s"]] {
        let error = session.marks(&path(bad), None).expect_err("an error");
        assert!(!error.message.is_empty(), "{bad:?}");
    }
}

/// The references of an open path come from the rule expressions, as the
/// diagrams draw them. In `dead-part.ebnf`, the alternative `a` of `s` derives
/// no finite sequence, so the tracer leaves it out. The diagram of `s` still
/// draws the reference box `a`, so `s › a` is an open path, and the start rule
/// reaches `a`.
#[test]
fn a_reference_that_derives_no_finite_sequence_still_extends_an_open_path() {
    let grammar = TraceGrammar::new(&read(&marks_dir().join("dead-part.ebnf"))).expect("a grammar");
    let session = Session::new(&grammar);
    let path = |rules: &[&str]| rules.iter().map(|r| r.to_string()).collect::<Vec<_>>();
    let marks = session.marks(&path(&["s", "a"]), None).expect("the open path s › a");
    assert!(marks.readings.is_empty(), "no reading stands in the diagram of a: {marks:?}");
    let error = session.marks(&path(&["a"]), None).expect_err("the start rule reaches a");
    assert!(error.message.contains("which the start rule reaches"), "{}", error.message);
    assert!(grammar.reference_chains(2).contains(&path(&["s", "a"])));
}
