//! The diagram cases at the core interface, the first test seam of the slice:
//! `docs/specs/rule-diagrams.md`, section "Test seams".
//!
//! A diagram case is a grammar file `NAME.ebnf` in `tests/diagrams`, with the
//! expected diagram KDL in `NAME.diagrams.kdl` beside it. Each example grammar
//! in the `examples` directory of the repository has one too. Set
//! `UPDATE_EXPECT=1`, or run `task test:update`, to rewrite the expected files
//! from the output.
//!
//! Each case also checks two properties. The KDL output survives a round trip
//! through the Rust types. Each diagram keeps the three invariants of the
//! spec: no two boxes overlap, the free band above each box is empty, and no
//! flow crosses a box.

use std::{
    fs,
    path::{Path, PathBuf},
};

use workbench_core::{
    checked::Span,
    checked_grammar,
    diagram::{Diagram, Diagrams, Flow, Shape},
    diagrams,
};

/// The free band above each box, in px.
const BAND: u32 = 22;

fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn update_mode() -> bool {
    std::env::var_os("UPDATE_EXPECT").is_some()
}

fn grammar_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let mut dirs = vec![dir.to_path_buf()];
    while let Some(dir) = dirs.pop() {
        for entry in fs::read_dir(&dir).unwrap_or_else(|e| panic!("{}: {e}", dir.display())) {
            let path = entry.expect("a directory entry").path();
            if path.is_dir() {
                dirs.push(path);
            } else if path.extension().is_some_and(|e| e == "ebnf") {
                files.push(path);
            }
        }
    }
    files.sort();
    files
}

/// Runs every case in a directory and reports every failure at once.
fn run_cases(dir: &Path) {
    let mut failures = Vec::new();
    let files = grammar_files(dir);
    assert!(!files.is_empty(), "no grammar files in {}", dir.display());
    for grammar in &files {
        if let Err(failure) = run_case(grammar) {
            failures.push(format!("{}:\n{failure}", grammar.display()));
        }
    }
    assert!(failures.is_empty(), "{} of {} cases failed:\n\n{}", failures.len(), files.len(), failures.join("\n\n"));
}

fn run_case(grammar: &Path) -> Result<(), String> {
    let text = fs::read_to_string(grammar).map_err(|e| e.to_string())?;
    let laid_out = diagrams(&text).map_err(|d| format!("the grammar has errors: {d:?}"))?;
    let output = laid_out.to_kdl();
    let read = Diagrams::from_kdl(&output).map_err(|e| format!("the output does not read back: {e}\n{output}"))?;
    if read != laid_out || read.to_kdl() != output {
        return Err("the output changes in a round trip through the Rust types".into());
    }
    check_invariants(&laid_out)?;

    let expected_path = grammar.with_extension("diagrams.kdl");
    let expected = fs::read_to_string(&expected_path).unwrap_or_default();
    if expected == output {
        return Ok(());
    }
    if update_mode() {
        fs::write(&expected_path, &output).map_err(|e| e.to_string())?;
        return Ok(());
    }
    if expected.is_empty() {
        return Err(format!("{} is missing; run `task test:update`", expected_path.display()));
    }
    Err(format!("the output differs from {}:\n{}", expected_path.display(), diff(&expected, &output)))
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

// The invariants.

/// A rectangle: left, top, right and bottom.
#[derive(Debug, Clone, Copy)]
struct Rect {
    left: u32,
    top: u32,
    right: u32,
    bottom: u32,
}

impl Rect {
    fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self { left: x, top: y, right: x + width, bottom: y + height }
    }

    /// Whether the insides of two rectangles meet.
    fn overlaps(&self, other: &Rect) -> bool {
        self.left < other.right && other.left < self.right && self.top < other.bottom && other.top < self.bottom
    }

    /// Whether a horizontal or vertical segment passes through the inside.
    fn crossed_by(&self, (x1, y1): (u32, u32), (x2, y2): (u32, u32)) -> bool {
        let (lo_x, hi_x, lo_y, hi_y) = (x1.min(x2), x1.max(x2), y1.min(y2), y1.max(y2));
        if y1 == y2 {
            self.top < y1 && y1 < self.bottom && lo_x.max(self.left) < hi_x.min(self.right)
        } else {
            self.left < x1 && x1 < self.right && lo_y.max(self.top) < hi_y.min(self.bottom)
        }
    }
}

fn shape_rect(shape: &Shape) -> Rect {
    match shape {
        Shape::StartEvent { x, y, size } | Shape::EndEvent { x, y, size } | Shape::Gateway { x, y, size, .. } => {
            Rect::new(*x, *y, *size, *size)
        }
        Shape::Task { x, y, width, height, .. } | Shape::Reference { x, y, width, height, .. } => {
            Rect::new(*x, *y, *width, *height)
        }
    }
}

fn is_box(shape: &Shape) -> bool {
    matches!(shape, Shape::Task { .. } | Shape::Reference { .. })
}

fn segments(flow: &Flow) -> impl Iterator<Item = ((u32, u32), (u32, u32))> + '_ {
    flow.points.windows(2).map(|w| ((w[0].x, w[0].y), (w[1].x, w[1].y)))
}

/// Checks the three invariants of the spec on each diagram, and that every
/// shape and every flow lies inside its diagram with whole, orthogonal
/// segments.
fn check_invariants(diagrams: &Diagrams) -> Result<(), String> {
    for diagram in &diagrams.diagrams {
        check_diagram(diagram).map_err(|e| format!("diagram `{}`: {e}", diagram.rule))?;
    }
    Ok(())
}

fn check_diagram(diagram: &Diagram) -> Result<(), String> {
    let frame = Rect::new(0, 0, diagram.width, diagram.height);
    let rects: Vec<Rect> = diagram.shapes.iter().map(shape_rect).collect();
    for (i, rect) in rects.iter().enumerate() {
        if rect.left < frame.left || rect.right > frame.right || rect.top < frame.top || rect.bottom > frame.bottom {
            return Err(format!("shape {i} lies outside the diagram: {:?}", diagram.shapes[i]));
        }
        for (j, other) in rects.iter().enumerate().skip(i + 1) {
            if rect.overlaps(other) {
                return Err(format!("shapes {i} and {j} overlap: {:?} and {:?}", diagram.shapes[i], diagram.shapes[j]));
            }
        }
    }
    for (i, shape) in diagram.shapes.iter().enumerate().filter(|(_, s)| is_box(s)) {
        let rect = rects[i];
        if rect.top < BAND {
            return Err(format!("the free band of box {i} lies outside the diagram"));
        }
        let band = Rect { left: rect.left, top: rect.top - BAND, right: rect.right, bottom: rect.top };
        for (j, other) in rects.iter().enumerate() {
            if j != i && band.overlaps(other) {
                return Err(format!("shape {j} enters the free band of box {i}: {:?}", diagram.shapes[j]));
            }
        }
        for flow in &diagram.flows {
            for (a, b) in segments(flow) {
                if band.crossed_by(a, b) {
                    return Err(format!("a flow enters the free band of {shape:?}: {flow:?}"));
                }
                if rect.crossed_by(a, b) {
                    return Err(format!("a flow crosses {shape:?}: {flow:?}"));
                }
            }
        }
    }
    for flow in &diagram.flows {
        if flow.points.len() < 2 {
            return Err(format!("a flow has fewer than two points: {flow:?}"));
        }
        for (a, b) in segments(flow) {
            if (a.0 != b.0) == (a.1 != b.1) {
                return Err(format!("a segment is neither horizontal nor vertical: {flow:?}"));
            }
        }
        for p in &flow.points {
            if p.x > diagram.width || p.y > diagram.height {
                return Err(format!("a flow leaves the diagram: {flow:?}"));
            }
        }
    }
    Ok(())
}

// The tests.

#[test]
fn diagram_cases() {
    run_cases(&manifest_dir().join("tests/diagrams"));
}

#[test]
fn example_grammars() {
    run_cases(&manifest_dir().join("../../examples"));
}

/// Each row of the table of constructs in the spec has a case, and so has
/// each further case that the spec and the task name.
#[test]
fn every_construct_has_a_case() {
    let rows = [
        ("Symbol occurrence", "symbol-occurrence"),
        ("Sequence", "sequence"),
        ("Choice", "choice"),
        ("Optional part", "optional-part"),
        ("x+", "one-or-more"),
        ("x*", "zero-or-more"),
        ("Bound from 1", "bound--from-one"),
        ("Bound from 0", "bound--from-zero"),
        ("Reference", "reference"),
        ("Reference to a rule for one step", "reference--one-step"),
        ("Self tail call", "self-tail-call--back-edge"),
        ("Self tail call, from an optional part", "self-tail-call--optional"),
        ("Self tail call, without a straight route", "self-tail-call--no-straight-route"),
        ("A back edge above the body", "self-tail-call--back-edge-above"),
        ("A join gateway with one flow", "join-gateway--one-flow"),
        ("A loop through rules", "loop-link--two-rules"),
        ("A self head call", "self-head-call"),
        ("The label of the design review", "label--tie"),
        ("The spaces of labels, from the implementation review", "label--spaces"),
    ];
    let dir = manifest_dir().join("tests/diagrams");
    let missing: Vec<&str> =
        rows.iter().filter(|(_, stem)| !dir.join(format!("{stem}.ebnf")).exists()).map(|(row, _)| *row).collect();
    assert!(missing.is_empty(), "no diagram case for {missing:?}");
}

/// The two cases of the design review `rule-diagram-design-k37`.
#[test]
fn the_cases_of_the_design_review() {
    let text =
        fs::read_to_string(manifest_dir().join("tests/diagrams/self-tail-call--no-straight-route.ebnf")).unwrap();
    let laid_out = diagrams(&text).unwrap();
    let r = &laid_out.diagrams[0];
    assert!(r.flows.iter().all(|f| f.role.as_str() != "back-edge"), "{r:?}");
    let link = r.shapes.iter().find_map(|s| match s {
        Shape::Reference { rule, loop_link, .. } if rule == "r" => Some(*loop_link),
        _ => None,
    });
    assert_eq!(link.flatten().map(|l| l.as_str()), Some("tail-references"));

    let text = fs::read_to_string(manifest_dir().join("tests/diagrams/label--tie.ebnf")).unwrap();
    let laid_out = diagrams(&text).unwrap();
    let label = laid_out.diagrams[0].shapes.iter().find_map(|s| match s {
        Shape::Task { symbol, label, .. } if symbol == "aaaaaa bbbbbb cccccc" => Some(label.clone()),
        _ => None,
    });
    assert_eq!(label, Some(vec!["aaaaaa bbbbbb".to_string(), "cccccc".to_string()]));
}

/// The example of the diagram KDL in the spec: the rule `setup` of the
/// tutorial grammar. The spec shortens the hash.
#[test]
fn the_example_of_the_spec() {
    let spec = fs::read_to_string(manifest_dir().join("../../docs/specs/rule-diagrams.md")).unwrap();
    let start = spec.find("```kdl\ndiagrams").expect("the example in the spec") + "```kdl\n".len();
    let end = start + spec[start..].find("```").unwrap();
    // A real hash is quoted only when KDL 2.0 needs the quotes, as in the
    // checked grammar. The shortened hash `…` needs none.
    let example = spec[start..end].replacen("grammar-sha256=\"…\"", "grammar-sha256=…", 1);

    let text = fs::read_to_string(manifest_dir().join("../../examples/tutorial.ebnf")).unwrap();
    let mut laid_out = diagrams(&text).unwrap();
    laid_out.diagrams.retain(|d| d.rule == "setup");
    laid_out.grammar_sha256 = "…".into();
    assert_eq!(laid_out.to_kdl(), example);
}

/// The other grammars of the test cases have no expected diagram files. Each
/// of their diagrams still keeps the invariants.
#[test]
fn every_valid_test_grammar_keeps_the_invariants() {
    let mut count = 0;
    for dir in ["tests/cases", "tests/traces"] {
        for grammar in grammar_files(&manifest_dir().join(dir)) {
            let text = fs::read_to_string(&grammar).unwrap();
            if let Ok(laid_out) = diagrams(&text) {
                count += 1;
                check_invariants(&laid_out).unwrap_or_else(|e| panic!("{}: {e}", grammar.display()));
            }
        }
    }
    assert!(count > 20, "only {count} grammars without errors");
}

/// A grammar with an error has no diagrams.
#[test]
fn a_grammar_with_an_error_has_no_diagrams() {
    let errors = diagrams("%start r ;\nr = undefined ;").unwrap_err();
    assert!(!errors.is_empty());
    assert!(errors.iter().all(|d| d.severity.as_str() == "error"));
}

// Random rules.

/// A small generator of pseudo-random numbers, so the sweep is the same on
/// every run (https://en.wikipedia.org/wiki/Xorshift).
struct Random(u64);

impl Random {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

const SYMBOLS: [&str; 6] = ["a", "b", "very_long_symbol", "shows the simplest case", "x-y-z", "注文"];
const RULES: [&str; 3] = ["r", "s", "t"];

/// A random expression. With `tail`, the expression can end with a self
/// tail call of `rule`.
fn expression(random: &mut Random, rule: &str, depth: u32, tail: bool) -> String {
    let leaf = |random: &mut Random| -> String {
        if random.below(3) == 0 {
            RULES[random.below(3) as usize].to_string()
        } else {
            format!("\"{}\"", SYMBOLS[random.below(SYMBOLS.len() as u64) as usize])
        }
    };
    let pick = if depth == 0 { 0 } else { random.below(7) };
    match pick {
        0 | 1 => {
            let mut item = leaf(random);
            match random.below(8) {
                0 => item.push('?'),
                1 => item.push('*'),
                2 => item.push('+'),
                3 => item.push_str("{1,3}"),
                4 => item.push_str("{0,2}"),
                _ => {}
            }
            item
        }
        2 | 3 => {
            let n = 2 + random.below(3);
            let mut items: Vec<String> = (0..n).map(|_| expression(random, rule, depth - 1, false)).collect();
            if tail {
                match random.below(3) {
                    0 => items.push(rule.to_string()),
                    1 => items.push(format!("{rule}?")),
                    _ => {
                        items.pop();
                        items.push(expression(random, rule, depth - 1, true));
                    }
                }
            }
            items.join(" ")
        }
        4 | 5 => {
            let n = 2 + random.below(3);
            let alternatives: Vec<String> = (0..n).map(|_| expression(random, rule, depth - 1, tail)).collect();
            format!("({})", alternatives.join(" | "))
        }
        _ => {
            let inner = expression(random, rule, depth - 1, false);
            let op = ["?", "*", "+", "{2,4}"][random.below(4) as usize];
            format!("({inner}){op}")
        }
    }
}

/// Random grammars keep the invariants. The design review found a back edge
/// through a box only in such a sweep.
#[test]
fn random_grammars_keep_the_invariants() {
    let mut random = Random(0x9E37_79B9_7F4A_7C15);
    let mut laid_out_count = 0;
    let mut back_edges = 0;
    // Many random grammars have a repetition that can match the empty
    // sequence, which is an error. So the sweep draws until it has enough.
    for _ in 0..30_000 {
        if laid_out_count == 2000 {
            break;
        }
        let mut text = String::from("%start r ;\n");
        for rule in RULES {
            let body = expression(&mut random, rule, 3, true);
            text.push_str(&format!("{rule} = {body} | \"end\" ;\n"));
        }
        let Ok(laid_out) = diagrams(&text) else { continue };
        laid_out_count += 1;
        back_edges +=
            laid_out.diagrams.iter().flat_map(|d| &d.flows).filter(|f| f.role.as_str() == "back-edge").count();
        if let Err(e) = check_invariants(&laid_out) {
            panic!("{e}\n{text}");
        }
        let output = laid_out.to_kdl();
        assert_eq!(Diagrams::from_kdl(&output).as_ref(), Ok(&laid_out), "{text}");
    }
    assert_eq!(laid_out_count, 2000, "too few random grammars without errors");
    assert!(back_edges > 100, "only {back_edges} back edges in the sweep");
}

// Stability.

/// The span after an edit that replaces the bytes `at..at + old` with `new`
/// bytes.
fn shifted(span: Span, at: u32, old: u32, new: u32) -> Span {
    let moved = |offset: u32| if offset >= at + old { offset + new - old } else { offset };
    Span { start: moved(span.start), end: if span.end > at { moved(span.end.max(at + old)) } else { span.end } }
}

fn shift_diagram(diagram: &Diagram, at: u32, old: u32, new: u32) -> Diagram {
    let mut out = diagram.clone();
    for shape in &mut out.shapes {
        match shape {
            Shape::Task { span, .. } | Shape::Reference { span, .. } | Shape::Gateway { span, .. } => {
                *span = shifted(*span, at, old, new);
            }
            Shape::StartEvent { .. } | Shape::EndEvent { .. } => {}
        }
    }
    for flow in &mut out.flows {
        flow.span = flow.span.map(|s| shifted(s, at, old, new));
    }
    out
}

/// The identity of a shape: its kind, its role and its span.
fn shape_identity(shape: &Shape) -> String {
    match shape {
        Shape::StartEvent { .. } => "start-event".to_string(),
        Shape::EndEvent { .. } => "end-event".to_string(),
        Shape::Task { span, .. } => format!("task {span:?}"),
        Shape::Reference { span, .. } => format!("reference {span:?}"),
        Shape::Gateway { role, span, .. } => format!("gateway {} {span:?}", role.as_str()),
    }
}

/// The identity of the shape at one end of a flow. A flow that ends at the
/// first point of another flow has no shape there.
fn end_identity(diagram: &Diagram, x: u32, y: u32) -> String {
    let at = |r: &Rect| r.left <= x && x <= r.right && r.top <= y && y <= r.bottom;
    diagram.shapes.iter().find(|s| at(&shape_rect(s))).map_or_else(|| "a flow".to_string(), shape_identity)
}

/// The identity of a flow: its role, its span and the shapes at its two ends.
fn flow_identity(diagram: &Diagram, flow: &Flow) -> String {
    let (first, last) = (flow.points[0], flow.points[flow.points.len() - 1]);
    format!(
        "flow {} {:?} from {} to {}",
        flow.role.as_str(),
        flow.span,
        end_identity(diagram, first.x, first.y),
        end_identity(diagram, last.x, last.y)
    )
}

/// The identity of each element of a diagram, in order: the shapes, then the
/// flows.
fn identities(diagram: &Diagram) -> Vec<String> {
    let shapes = diagram.shapes.iter().map(shape_identity);
    let flows = diagram.flows.iter().map(|f| flow_identity(diagram, f));
    shapes.chain(flows).collect()
}

/// The strict relations of two rectangles: left of, right of, above and
/// below.
fn relations(a: &Rect, b: &Rect) -> [bool; 4] {
    [a.right <= b.left, b.right <= a.left, a.bottom <= b.top, b.bottom <= a.top]
}

/// Checks that the drawing `after` keeps the order of each element of
/// `before` that survives an edit. The spans of `before` must already name
/// the text after the edit.
///
/// An element survives when its identity stays. A flow survives when it keeps
/// its role, its span and the shapes at its two ends. The elements that
/// survive keep their order in the lists of shapes and of flows.
///
/// Each pair of shapes that survives keeps at least one of its strict
/// relations. The layout orders each pair along one axis by the text: a
/// sequence from left to right, and a choice from top to bottom. So every
/// layout that follows the spec passes this check, and a layout that swaps two
/// siblings fails it.
fn check_order(before: &Diagram, after: &Diagram) -> Result<(), String> {
    let now = identities(after);
    let kept: Vec<String> = identities(before).into_iter().filter(|id| now.contains(id)).collect();
    let mut rest = now.iter();
    for id in &kept {
        if !rest.any(|n| n == id) {
            return Err(format!("`{id}` changes its order in the lists:\n{kept:?}\n{now:?}"));
        }
    }
    let placed =
        |d: &Diagram| -> Vec<(String, Rect)> { d.shapes.iter().map(|s| (shape_identity(s), shape_rect(s))).collect() };
    let (old, new) = (placed(before), placed(after));
    let survivors: Vec<(&String, &Rect, &Rect)> =
        old.iter().filter_map(|(id, o)| new.iter().find(|(n, _)| n == id).map(|(_, r)| (id, o, r))).collect();
    for (i, (a, a_old, a_new)) in survivors.iter().enumerate() {
        for (b, b_old, b_new) in &survivors[i + 1..] {
            let (was, is) = (relations(a_old, b_old), relations(a_new, b_new));
            if !was.iter().any(|&r| r) {
                return Err(format!("`{a}` and `{b}` have no strict relation before the edit"));
            }
            if !was.iter().zip(is).any(|(&w, i)| w && i) {
                return Err(format!("`{a}` and `{b}` change their order in the drawing: {was:?} becomes {is:?}"));
            }
        }
    }
    Ok(())
}

/// The facts of the references of a diagram that depend on other rules:
/// whether the target rule is for one step, the loop link and the back edges
/// of self tail calls.
fn reference_facts(diagram: &Diagram) -> Vec<String> {
    let boxes = diagram.shapes.iter().filter_map(|s| match s {
        Shape::Reference { span, one_step, loop_link, .. } => {
            Some(format!("reference {span:?} one-step={one_step} loop-link={:?}", loop_link.map(|l| l.as_str())))
        }
        _ => None,
    });
    let edges =
        diagram.flows.iter().filter(|f| f.role.as_str() == "back-edge").map(|f| format!("back edge {:?}", f.span));
    boxes.chain(edges).collect()
}

/// Applies one edit to a grammar and checks the stability of the layout, as
/// the section "The layout" of the spec states it. Returns the other rules
/// whose diagrams change because the facts of one of their references change.
fn check_edit(text: &str, old: &str, new: &str) -> Vec<String> {
    let before = diagrams(text).unwrap();
    let at = text.find(old).unwrap_or_else(|| panic!("`{old}` in the grammar"));
    let edited = text.replacen(old, new, 1);
    // The smallest edit: without the text that the old and the new parts
    // share at their start and at their end.
    let prefix = old.bytes().zip(new.bytes()).take_while(|(a, b)| a == b).count();
    let suffix = old[prefix..].bytes().rev().zip(new[prefix..].bytes().rev()).take_while(|(a, b)| a == b).count();
    let (at, old, new) = (at + prefix, &old[prefix..old.len() - suffix], &new[prefix..new.len() - suffix]);
    let after = diagrams(&edited).unwrap_or_else(|d| panic!("the edited grammar has errors: {d:?}\n{edited}"));
    check_invariants(&after).unwrap_or_else(|e| panic!("{e}\n{edited}"));
    assert_eq!(before.diagrams.len(), after.diagrams.len());
    // The edited rule: the last rule whose body starts before the edit.
    let edited_rule = checked_grammar(text)
        .rules
        .iter()
        .filter(|r| r.body.as_ref().is_some_and(|b| b.span().start as usize <= at))
        .max_by_key(|r| r.body.as_ref().map(|b| b.span().start))
        .map(|r| r.name.clone())
        .unwrap();
    let (at, old, new) = (at as u32, old.len() as u32, new.len() as u32);
    let mut changed = 0;
    let mut by_facts = Vec::new();
    for (b, a) in before.diagrams.iter().zip(&after.diagrams) {
        let moved = shift_diagram(b, at, old, new);
        if b.rule == edited_rule {
            changed += 1;
            assert_ne!(&moved, a, "the edit changes the diagram of `{}`", b.rule);
        } else if reference_facts(&moved) == reference_facts(a) {
            assert_eq!(&moved, a, "the edit of `{edited_rule}` changes the diagram of `{}`", b.rule);
            continue;
        } else {
            by_facts.push(b.rule.clone());
        }
        check_order(&moved, a).unwrap_or_else(|e| panic!("diagram `{}`: {e}\n{edited}", b.rule));
    }
    assert_eq!(changed, 1);
    by_facts
}

/// An edit inside one element changes the size of that element and of each
/// element that holds it. No element changes its order, and the other rules
/// keep their diagrams.
#[test]
fn an_edit_inside_one_rule_moves_no_other_rule() {
    let text = fs::read_to_string(manifest_dir().join("../../examples/tutorial.ebnf")).unwrap();
    for (old, new) in [
        ("\"show_example\"", "\"show_one_more_worked_example\""),
        ("introduce_concept show_example", "introduce_concept overview show_example"),
    ] {
        assert_eq!(check_edit(&text, old, new), Vec::<String>::new(), "the edit `{old}` to `{new}`");
    }
}

/// The finding F2 of the review `diagram-layout-k40`. An edit of one rule can
/// change two facts of a reference in another rule: whether its target is for
/// one step, and whether it links a loop through rules. That diagram then
/// changes, but no element in it changes its order.
#[test]
fn an_edit_changes_another_rule_only_through_the_facts_of_its_references() {
    // The target of the reference `r` stops being for one step.
    let one_step = "%start s ;\ns = \"a\" r \"b\" ;\nr = \"x\" ;\n";
    assert_eq!(check_edit(one_step, "\"x\" ;", "\"x\" \"y\" ;"), ["s"]);
    // The edit closes a loop of tail references through `t` and `u`.
    let loop_link = "%start s ;\ns = \"a\" t ;\nt = \"b\" u ;\nu = \"c\" ;\n";
    assert_eq!(check_edit(loop_link, "\"c\" ;", "\"c\" t? ;"), ["t"]);
    // The edit puts `q` on a cycle of head references. By rule 3 of the
    // tracer spec, its self tail call stops being a link of a loop, so its
    // back edge becomes a reference box.
    let head_cycle = "%start q ;\nq = \"a\" q? | b ;\nb = \"c\" ;\n";
    assert_eq!(check_edit(head_cycle, "\"c\" ;", "q \"c\" ;"), ["q"]);
}

/// The finding F3 of the review `diagram-layout-k40`: the order check fails
/// when the drawing swaps two siblings, although their order in the lists
/// and their spans stay the same.
#[test]
fn the_order_check_fails_on_a_swap_of_two_siblings() {
    let text = fs::read_to_string(manifest_dir().join("../../examples/tutorial.ebnf")).unwrap();
    let laid_out = diagrams(&text).unwrap();
    let mut boxes_swapped = 0;
    let mut flows_swapped = 0;
    for diagram in &laid_out.diagrams {
        check_order(diagram, diagram).unwrap_or_else(|e| panic!("diagram `{}`: {e}", diagram.rule));
        // Two boxes on one row: swap their places.
        let row: Vec<usize> = (0..diagram.shapes.len()).filter(|&i| is_box(&diagram.shapes[i])).collect();
        let centre = |i: usize| {
            let r = shape_rect(&diagram.shapes[i]);
            r.top + r.bottom
        };
        if let Some((&i, &j)) = row.iter().zip(&row[1..]).find(|&(&i, &j)| centre(i) == centre(j)) {
            let mut swapped = diagram.clone();
            let (xi, xj) = (shape_rect(&diagram.shapes[i]).left, shape_rect(&diagram.shapes[j]).left);
            for (k, x) in [(i, xj), (j, xi)] {
                match &mut swapped.shapes[k] {
                    Shape::Task { x: left, .. } | Shape::Reference { x: left, .. } => *left = x,
                    _ => unreachable!(),
                }
            }
            assert!(check_order(diagram, &swapped).is_err(), "a swap of two boxes in `{}` passes", diagram.rule);
            boxes_swapped += 1;
        }
        // Two sequence flows between shapes: swap their points.
        let between: Vec<usize> = (0..diagram.flows.len())
            .filter(|&i| {
                let f = &diagram.flows[i];
                f.role.as_str() == "sequence" && !flow_identity(diagram, f).contains("a flow")
            })
            .collect();
        if let [i, j, ..] = between[..] {
            let mut swapped = diagram.clone();
            let points = swapped.flows[i].points.clone();
            swapped.flows[i].points = swapped.flows[j].points.clone();
            swapped.flows[j].points = points;
            assert!(check_order(diagram, &swapped).is_err(), "a swap of two flows in `{}` passes", diagram.rule);
            flows_swapped += 1;
        }
    }
    assert!(boxes_swapped > 0 && flows_swapped > 0, "{boxes_swapped} box swaps and {flows_swapped} flow swaps");
}
