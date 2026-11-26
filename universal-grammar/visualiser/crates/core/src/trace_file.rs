//! The trace file in KDL 2.0: `docs/specs/tracer.md`, section "The trace file".
//!
//! Another program writes the root node and the sequence. The core derives the
//! rest: the next options after each step and the failure point. The reader
//! checks the derived nodes against their format, but the core never takes a
//! derived node from a file. It derives each node again, and Save writes the
//! result.
//!
//! A step departs from the mapping rules of `kdl_map`: its symbol is its one
//! argument, and its properties are its attributes, with open keys.

use kdl::{KdlDocument, KdlEntry, KdlNode, KdlValue};

use crate::{
    checked::Span,
    kdl_map::{first_disallowed, keep_string_text, push_str, string_text},
    trace::{
        Attribute, AttributeValue, Float, READING_CAP, Reading, Session, Step, TRACE_FORMAT_VERSION, TraceCode,
        TraceDiagnostic, TreeKind, TreeNode,
    },
};

/// Whether a character is a newline of KDL 2.0.
pub(crate) fn is_line_break(c: char) -> bool {
    matches!(c, '\n' | '\r' | '\u{000B}' | '\u{000C}' | '\u{0085}' | '\u{2028}' | '\u{2029}')
}

// Writing.

/// Writes the trace file of a session.
pub(crate) fn write(session: &Session) -> String {
    let mut root = KdlNode::new("trace");
    push_int(&mut root, "format-version", i128::from(TRACE_FORMAT_VERSION));
    push_str(&mut root, "grammar-sha256", session.grammar().grammar_sha256());
    push_int(&mut root, "reading-cap", i128::from(READING_CAP));
    let children = root.ensure_children();

    let mut sequence = KdlNode::new("sequence");
    for step in session.steps() {
        sequence.ensure_children().nodes_mut().push(write_step(step));
    }
    children.nodes_mut().push(sequence);

    let (counts, readings) = session.all_readings();
    for (k, (count, cap_reached)) in counts.into_iter().enumerate() {
        let mut prefix = KdlNode::new("prefix");
        push_int(&mut prefix, "steps", k as i128);
        push_int(&mut prefix, "reading-count", count as i128);
        prefix.push(KdlEntry::new_prop("cap-reached", cap_reached));
        prefix.push(KdlEntry::new_prop("can-end", session.can_end_after(k)));
        for option in session.next_options_after(k) {
            let mut next = KdlNode::new("next");
            push_str(&mut next, "symbol", &option.symbol);
            prefix.ensure_children().nodes_mut().push(next);
        }
        children.nodes_mut().push(prefix);
    }

    if let Some(step) = session.failure_point() {
        let mut node = KdlNode::new("failure-point");
        push_int(&mut node, "step", step as i128);
        children.nodes_mut().push(node);
    }

    for reading in &readings.readings {
        children.nodes_mut().push(write_reading(reading));
    }

    let mut document = KdlDocument::new();
    document.nodes_mut().push(root);
    document.autoformat();
    // `autoformat` resets the text of each key and type annotation. So the
    // writer sets that text afterwards.
    set_identifier_texts(&mut document);
    document.to_string()
}

/// Sets the text of each key and type annotation to the text of
/// `string_text`. The `kdl` crate can write these as invalid KDL 2.0.
fn set_identifier_texts(document: &mut KdlDocument) {
    for node in document.nodes_mut() {
        for entry in node.entries_mut() {
            if let Some(key) = entry.name_mut() {
                key.set_repr(string_text(key.value()));
            }
            if let Some(annotation) = entry.ty_mut() {
                annotation.set_repr(string_text(annotation.value()));
            }
        }
        if let Some(children) = node.children_mut() {
            set_identifier_texts(children);
        }
    }
}

fn write_step(step: &Step) -> KdlNode {
    let mut node = KdlNode::new("step");
    node.push(string_entry(None, &step.symbol));
    for attribute in &step.attributes {
        let mut entry = match &attribute.value {
            AttributeValue::String(s) => string_entry(Some(&attribute.key), s),
            AttributeValue::Integer(n) => KdlEntry::new_prop(attribute.key.as_str(), *n),
            AttributeValue::Float(Float(f)) => KdlEntry::new_prop(attribute.key.as_str(), *f),
            AttributeValue::Bool(b) => KdlEntry::new_prop(attribute.key.as_str(), *b),
            AttributeValue::Null => KdlEntry::new_prop(attribute.key.as_str(), KdlValue::Null),
        };
        if let Some(annotation) = &attribute.annotation {
            entry.set_ty(annotation.as_str());
        }
        node.push(entry);
    }
    node
}

/// A string argument or property, with quotes and escapes where KDL 2.0 needs
/// them.
fn string_entry(key: Option<&str>, value: &str) -> KdlEntry {
    let mut entry = match key {
        Some(key) => KdlEntry::new_prop(key, value),
        None => KdlEntry::new(value),
    };
    keep_string_text(&mut entry, value);
    entry
}

fn push_int(node: &mut KdlNode, key: &str, value: i128) {
    node.push(KdlEntry::new_prop(key, value));
}

// Reading.

/// What the reader takes from a trace file: the sequence and the grammar hash.
/// The diagnostics hold every fault that the reader found.
pub(crate) struct ReadFile {
    pub steps: Vec<Step>,
    /// The grammar hash, with the span of its property.
    pub grammar_sha256: Option<(String, Span)>,
    pub diagnostics: Vec<TraceDiagnostic>,
}

/// Reads a trace file.
pub(crate) fn read(text: &str) -> ReadFile {
    let mut reader = Reader { text, diagnostics: Vec::new() };
    let mut file = ReadFile { steps: Vec::new(), grammar_sha256: None, diagnostics: Vec::new() };
    if let Some((offset, c)) = first_disallowed(text) {
        reader.fault(
            TraceCode::NotKdl,
            Span::new(offset, offset + c.len_utf8()),
            format!(
                "The text is not KDL 2.0. It holds the code point U+{:04X}, which KDL 2.0 forbids as a literal \
                 character. A string can hold it as the escape `\\u{{{:x}}}`.",
                c as u32, c as u32
            ),
        );
        file.diagnostics = reader.diagnostics;
        return file;
    }
    let document = match KdlDocument::parse_v2(text) {
        Ok(document) => document,
        Err(e) => {
            // One diagnostic names every fault that the KDL parser found, at
            // the place of the first.
            let mut messages: Vec<String> = Vec::new();
            for message in e.diagnostics.iter().filter_map(|d| d.message.clone()) {
                if !messages.contains(&message) {
                    messages.push(message);
                }
            }
            let span = e.diagnostics.first().map_or(Span::at(0), |d| reader.span(d.span));
            let details = if messages.is_empty() {
                String::new()
            } else {
                format!(" The parser reports: {}.", messages.join("; "))
            };
            reader.fault(TraceCode::NotKdl, span, format!("The text is not KDL 2.0.{details}"));
            file.diagnostics = reader.diagnostics;
            return file;
        }
    };
    match document.nodes() {
        [root] if root.name().value() == "trace" => reader.trace(root, &mut file),
        [root] => reader.fault(
            TraceCode::NotATrace,
            reader.span(root.name().span()),
            format!("The root node is `{}`. A trace file has one root node, `trace`.", root.name().value()),
        ),
        [] => reader.fault(
            TraceCode::NotATrace,
            Span::at(0),
            "The document is empty. A trace file has one root node, `trace`.",
        ),
        [_, second, ..] => reader.fault(
            TraceCode::NotATrace,
            reader.span(second.name().span()),
            "The document has a second root node. A trace file has one root node, `trace`.",
        ),
    }
    file.diagnostics = reader.diagnostics;
    file
}

struct Reader<'a> {
    text: &'a str,
    diagnostics: Vec<TraceDiagnostic>,
}

/// The parts of a trace file, in their order.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Part {
    Start,
    Sequence,
    Prefix,
    FailurePoint,
    Reading,
}

impl Reader<'_> {
    fn fault(&mut self, code: TraceCode, span: Span, message: impl Into<String>) {
        self.diagnostics.push(TraceDiagnostic::new(code, span, message));
    }

    /// The byte span of a span of the `kdl` crate, which counts bytes.
    fn span(&self, span: miette::SourceSpan) -> Span {
        let start = span.offset().min(self.text.len());
        Span::new(start, (start + span.len()).min(self.text.len()))
    }

    fn node_span(&self, node: &KdlNode) -> Span {
        self.span(node.name().span())
    }

    fn trace(&mut self, root: &KdlNode, file: &mut ReadFile) {
        let props = self.properties(root, &["format-version", "grammar-sha256", "reading-cap"], &["format-version"]);
        if let Some(entry) = props.get("format-version") {
            match entry.value() {
                KdlValue::Integer(n) if *n == i128::from(TRACE_FORMAT_VERSION) => {}
                KdlValue::Integer(n) => self.fault(
                    TraceCode::UnknownFormatVersion,
                    self.span(entry.span()),
                    format!("The format version {n} is not {TRACE_FORMAT_VERSION}, the version that this core reads."),
                ),
                _ => self.wrong_type(entry, "trace", "an integer"),
            }
        }
        if let Some(entry) = props.get("grammar-sha256") {
            match entry.value() {
                KdlValue::String(s) => file.grammar_sha256 = Some((s.clone(), self.span(entry.span()))),
                _ => self.wrong_type(entry, "trace", "a string"),
            }
        }
        if let Some(entry) = props.get("reading-cap") {
            self.non_negative(entry, "trace");
        }

        let mut part = Part::Start;
        let mut sequence_length = 0;
        let mut symbols: Vec<Option<String>> = Vec::new();
        let mut prefixes = 0;
        for node in children(root) {
            let name = node.name().value();
            let this = match name {
                "sequence" => Part::Sequence,
                "prefix" => Part::Prefix,
                "failure-point" => Part::FailurePoint,
                "reading" => Part::Reading,
                other => {
                    self.fault(
                        TraceCode::UnknownNode,
                        self.node_span(node),
                        format!("`{other}` is not a child node of `trace`."),
                    );
                    continue;
                }
            };
            let in_order = this > part || (this == part && matches!(this, Part::Prefix | Part::Reading));
            let first_is_sequence = this == Part::Sequence || part >= Part::Sequence;
            if !in_order || !first_is_sequence {
                self.fault(
                    TraceCode::NodeOutOfOrder,
                    self.node_span(node),
                    format!(
                        "The node `{name}` is out of order. The children of `trace` are one `sequence`, \
                         then the `prefix` nodes, then at most one `failure-point`, then the `reading` nodes."
                    ),
                );
                continue;
            }
            part = this;
            match this {
                Part::Sequence => {
                    self.properties(node, &[], &[]);
                    for step in children(node) {
                        let step = self.step(step);
                        symbols.push(step.as_ref().map(|s| s.symbol.clone()));
                        if let Some(step) = step {
                            file.steps.push(step);
                        }
                    }
                    sequence_length = children(node).len();
                }
                Part::Prefix => {
                    self.prefix(node, prefixes, sequence_length);
                    prefixes += 1;
                }
                Part::FailurePoint => self.failure_point(node, prefixes, sequence_length),
                Part::Reading => self.reading(node, &symbols, prefixes.checked_sub(1)),
                Part::Start => unreachable!(),
            }
        }
        if part == Part::Start {
            self.fault(TraceCode::MissingNode, self.node_span(root), "The `trace` node needs a `sequence` child.");
        }
    }

    fn step(&mut self, node: &KdlNode) -> Option<Step> {
        let span = self.node_span(node);
        if node.name().value() != "step" {
            self.fault(
                TraceCode::UnknownNode,
                span,
                format!("`{}` is not a child node of `sequence`. Each child is a `step`.", node.name().value()),
            );
            return None;
        }
        let mut ok = true;
        if !children(node).is_empty() || node.children().is_some_and(|c| !c.nodes().is_empty()) {
            self.fault(TraceCode::StepChildren, span, "A step has no children.");
            ok = false;
        }
        let arguments: Vec<&KdlEntry> = node.entries().iter().filter(|e| e.name().is_none()).collect();
        let symbol = match arguments.as_slice() {
            [argument] => match argument.value() {
                KdlValue::String(s) if argument.ty().is_none() => Some((s.clone(), self.span(argument.span()))),
                _ => {
                    self.fault(
                        TraceCode::StepArgument,
                        self.span(argument.span()),
                        "The argument of a step is its symbol, as a KDL string without a type annotation.",
                    );
                    None
                }
            },
            _ => {
                self.fault(
                    TraceCode::StepArgument,
                    span,
                    format!("A step takes exactly one argument, its symbol. This step has {}.", arguments.len()),
                );
                None
            }
        };
        if let Some((symbol, symbol_span)) = &symbol {
            if symbol.is_empty() {
                self.fault(TraceCode::EmptySymbol, *symbol_span, "The symbol of a step is empty.");
                ok = false;
            } else if symbol.chars().any(is_line_break) {
                self.fault(TraceCode::LineBreakInSymbol, *symbol_span, "The symbol of a step holds a line break.");
                ok = false;
            }
        }
        let mut attributes: Vec<Attribute> = Vec::new();
        for entry in node.entries() {
            let Some(key) = entry.name() else { continue };
            let key = key.value().to_string();
            if attributes.iter().any(|a| a.key == key) {
                self.fault(
                    TraceCode::DuplicateAttribute,
                    self.span(entry.span()),
                    format!("The step has a second attribute with the key {key:?}."),
                );
                ok = false;
                continue;
            }
            let value = match entry.value() {
                KdlValue::String(s) => AttributeValue::String(s.clone()),
                KdlValue::Integer(n) => AttributeValue::Integer(*n),
                KdlValue::Float(f) => AttributeValue::Float(Float(*f)),
                KdlValue::Bool(b) => AttributeValue::Bool(*b),
                KdlValue::Null => AttributeValue::Null,
            };
            attributes.push(Attribute { key, value, annotation: entry.ty().map(|t| t.value().to_string()) });
        }
        match symbol {
            Some((symbol, _)) if ok => Some(Step { symbol, attributes }),
            _ => None,
        }
    }

    fn prefix(&mut self, node: &KdlNode, index: usize, sequence_length: usize) {
        let names = ["steps", "reading-count", "cap-reached", "can-end"];
        let props = self.properties(node, &names, &names);
        if let Some(entry) = props.get("reading-count") {
            self.non_negative(entry, "prefix");
        }
        if let Some(entry) = props.get("cap-reached")
            && !matches!(entry.value(), KdlValue::Bool(_))
        {
            self.wrong_type(entry, "prefix", "a boolean");
        }
        if let Some(entry) = props.get("steps")
            && let Some(steps) = self.non_negative(entry, "prefix")
            && (steps != index as i128 || steps > sequence_length as i128)
        {
            self.fault(
                TraceCode::MalformedDerivedNode,
                self.span(entry.span()),
                format!(
                    "This `prefix` has `steps={steps}`. The prefixes count the steps read from 0, one by one, \
                     and this one must have `steps={index}`, within the {sequence_length} steps of the sequence."
                ),
            );
        }
        if let Some(entry) = props.get("can-end")
            && !matches!(entry.value(), KdlValue::Bool(_))
        {
            self.wrong_type(entry, "prefix", "a boolean");
        }
        for next in children(node) {
            if next.name().value() != "next" {
                self.fault(
                    TraceCode::UnknownNode,
                    self.node_span(next),
                    format!("`{}` is not a child node of `prefix`.", next.name().value()),
                );
                continue;
            }
            let props = self.properties(next, &["symbol"], &["symbol"]);
            if let Some(entry) = props.get("symbol")
                && !matches!(entry.value(), KdlValue::String(_))
            {
                self.wrong_type(entry, "next", "a string");
            }
            if !children(next).is_empty() {
                self.fault(
                    TraceCode::MalformedDerivedNode,
                    self.node_span(next),
                    "A `next` in a `prefix` has no children.",
                );
            }
        }
    }

    /// Checks a `reading`: its properties, its `next` nodes, then one tree.
    /// The symbols are those of the sequence, with `None` for a faulty step.
    /// The steps read come from the prefixes, when the file has them.
    fn reading(&mut self, node: &KdlNode, symbols: &[Option<String>], steps_read: Option<usize>) {
        let props = self.properties(node, &["parent", "can-end"], &["can-end"]);
        if let Some(entry) = props.get("parent") {
            match entry.value() {
                KdlValue::Integer(n) if *n >= 1 => {}
                _ => self.wrong_type(entry, "reading", "an integer of 1 or more"),
            }
        }
        if let Some(entry) = props.get("can-end")
            && !matches!(entry.value(), KdlValue::Bool(_))
        {
            self.wrong_type(entry, "reading", "a boolean");
        }
        let nodes = children(node);
        let trees = nodes.iter().filter(|n| n.name().value() != "next").count();
        for (i, child) in nodes.iter().enumerate() {
            match child.name().value() {
                "next" if trees == 0 || i + 1 < nodes.len() => {
                    let props = self.properties(child, &["symbol"], &["symbol"]);
                    if let Some(entry) = props.get("symbol")
                        && !matches!(entry.value(), KdlValue::String(_))
                    {
                        self.wrong_type(entry, "next", "a string");
                    }
                    for occurrence in children(child) {
                        self.span_node(occurrence, "occurrence", "next");
                    }
                }
                "rule" | "loop" | "pass" | "step" if i + 1 == nodes.len() => {
                    let mut steps = TreeSteps { symbols, count: 0, faulty: false };
                    self.tree(child, "reading", &mut steps);
                    if let Some(read) = steps_read
                        && !steps.faulty
                        && steps.count != read
                    {
                        self.fault(
                            TraceCode::MalformedDerivedNode,
                            self.node_span(child),
                            format!(
                                "The tree of this reading holds {} steps, but the prefixes read {read} steps. \
                                 A tree holds each step read, once.",
                                steps.count
                            ),
                        );
                    }
                }
                "rule" | "loop" | "pass" | "step" | "next" => self.fault(
                    TraceCode::MalformedDerivedNode,
                    self.node_span(child),
                    "A `reading` holds its `next` nodes, then one tree node as its last child.",
                ),
                other => self.fault(
                    TraceCode::UnknownNode,
                    self.node_span(child),
                    format!("`{other}` is not a child node of `reading`."),
                ),
            }
        }
        if trees == 0 {
            self.fault(
                TraceCode::MalformedDerivedNode,
                self.node_span(node),
                "A `reading` needs the root of its tree.",
            );
        }
    }

    /// Checks a node with the span of a grammar element and no children.
    fn span_node(&mut self, node: &KdlNode, name: &str, parent: &str) {
        if node.name().value() != name {
            self.fault(
                TraceCode::UnknownNode,
                self.node_span(node),
                format!("`{}` is not a child node of `{parent}` at this place.", node.name().value()),
            );
            return;
        }
        let props = self.properties(node, &["start", "end"], &["start", "end"]);
        let start = props.get("start").and_then(|e| self.non_negative(e, name));
        let end = props.get("end").and_then(|e| self.non_negative(e, name));
        if let (Some(start), Some(end)) = (start, end)
            && start > end
        {
            self.fault(
                TraceCode::MalformedDerivedNode,
                self.node_span(node),
                format!("The span of this `{name}` starts at {start}, after its end at {end}."),
            );
        }
        if !children(node).is_empty() {
            self.fault(TraceCode::MalformedDerivedNode, self.node_span(node), format!("An `{name}` has no children."));
        }
    }

    /// Checks a tree node: its properties, its `origin` nodes, then its
    /// children. A `pass` is a child of a `loop` only, and a `loop` holds only
    /// passes after its origins. The steps of a tree are the steps of the
    /// sequence, from 1 in order, with the same symbols.
    fn tree(&mut self, node: &KdlNode, parent: &str, steps: &mut TreeSteps<'_>) {
        let name = node.name().value();
        if (parent == "loop") != (name == "pass") && ["rule", "loop", "pass", "step"].contains(&name) {
            let message = if parent == "loop" {
                "A `loop` holds its `origin` nodes, then `pass` nodes only."
            } else {
                "A `pass` is a child of a `loop` only."
            };
            self.fault(TraceCode::MalformedDerivedNode, self.node_span(node), message);
        }
        match name {
            "rule" => {
                let props = self.properties(node, &["name"], &["name"]);
                if let Some(entry) = props.get("name")
                    && !matches!(entry.value(), KdlValue::String(_))
                {
                    self.wrong_type(entry, "rule", "a string");
                }
            }
            "loop" => {
                let props = self.properties(node, &["kind", "pass-rule"], &["kind"]);
                if let Some(entry) = props.get("kind")
                    && !matches!(entry.value(), KdlValue::String(s)
                        if ["repetition", "tail-references", "head-references"].contains(&s.as_str()))
                {
                    self.wrong_type(entry, "loop", "`repetition`, `tail-references` or `head-references`");
                }
                if let Some(entry) = props.get("pass-rule")
                    && !matches!(entry.value(), KdlValue::String(_))
                {
                    self.wrong_type(entry, "loop", "a string");
                }
                // A loop through rules names its pass rule, and a repetition
                // names none.
                match (props.get("kind").map(KdlEntry::value), props.get("pass-rule")) {
                    (Some(KdlValue::String(kind)), None) if kind != "repetition" => self.fault(
                        TraceCode::MissingProperty,
                        self.node_span(node),
                        format!(
                            "A `loop` of kind `{kind}` is a loop through rules, and it needs the property `pass-rule`."
                        ),
                    ),
                    (Some(KdlValue::String(kind)), Some(entry)) if kind == "repetition" => self.fault(
                        TraceCode::MalformedDerivedNode,
                        self.span(entry.span()),
                        "A `repetition` loop has no pass rule. Only a loop through rules has `pass-rule`.",
                    ),
                    _ => {}
                }
            }
            "pass" | "step" => {
                let allowed: &[&str] = if name == "pass" { &["number"] } else { &["number", "symbol"] };
                let props = self.properties(node, allowed, allowed);
                if let Some(entry) = props.get("number")
                    && !matches!(entry.value(), KdlValue::Integer(n) if *n >= 1)
                {
                    self.wrong_type(entry, name, "an integer of 1 or more");
                }
                if let Some(entry) = props.get("symbol")
                    && !matches!(entry.value(), KdlValue::String(_))
                {
                    self.wrong_type(entry, name, "a string");
                }
                if name == "step" {
                    self.tree_step(&props, steps);
                }
            }
            other => {
                self.fault(TraceCode::UnknownNode, self.node_span(node), format!("`{other}` is not a node of a tree."));
                return;
            }
        }
        let mut in_children = false;
        for child in children(node) {
            if child.name().value() == "origin" && !in_children {
                self.span_node(child, "origin", name);
            } else if child.name().value() == "origin" || name == "step" {
                self.fault(
                    TraceCode::MalformedDerivedNode,
                    self.node_span(child),
                    format!("A `{name}` holds its `origin` nodes first, and a step holds nothing else."),
                );
            } else {
                in_children = true;
                self.tree(child, name, steps);
            }
        }
    }

    /// Checks that a step of a tree is the next step of the sequence, and that
    /// it repeats the symbol of that step.
    fn tree_step(&mut self, props: &Properties<'_>, steps: &mut TreeSteps<'_>) {
        steps.count += 1;
        let expected = steps.count;
        let Some(number_entry) = props.get("number") else { return };
        let KdlValue::Integer(number) = number_entry.value() else { return };
        if *number != expected as i128 {
            steps.faulty = true;
            self.fault(
                TraceCode::MalformedDerivedNode,
                self.span(number_entry.span()),
                format!(
                    "This step of a tree has `number={number}`. The steps of a tree run from 1 in the order \
                     of the sequence, so this step has `number={expected}`."
                ),
            );
            return;
        }
        let Some(sequence_symbol) = steps.symbols.get(expected - 1) else {
            steps.faulty = true;
            self.fault(
                TraceCode::MalformedDerivedNode,
                self.span(number_entry.span()),
                format!(
                    "This step of a tree has `number={number}`, but the sequence has {} steps.",
                    steps.symbols.len()
                ),
            );
            return;
        };
        if let (Some(sequence_symbol), Some(entry)) = (sequence_symbol, props.get("symbol"))
            && let KdlValue::String(symbol) = entry.value()
            && symbol != sequence_symbol
        {
            steps.faulty = true;
            self.fault(
                TraceCode::MalformedDerivedNode,
                self.span(entry.span()),
                format!(
                    "This step of a tree has the symbol {symbol:?}, but step {number} of the sequence has \
                     the symbol {sequence_symbol:?}. A step of a tree repeats the symbol in the sequence."
                ),
            );
        }
    }

    fn failure_point(&mut self, node: &KdlNode, prefixes: usize, sequence_length: usize) {
        let props = self.properties(node, &["step"], &["step"]);
        if !children(node).is_empty() {
            self.fault(TraceCode::MalformedDerivedNode, self.node_span(node), "A `failure-point` has no children.");
        }
        if let Some(entry) = props.get("step")
            && let Some(step) = self.non_negative(entry, "failure-point")
            && (step < 1 || step > sequence_length as i128 || (prefixes > 0 && step != prefixes as i128))
        {
            self.fault(
                TraceCode::MalformedDerivedNode,
                self.span(entry.span()),
                format!(
                    "The failure point names step {step}. It must name a step of the sequence, \
                     the step after the last prefix."
                ),
            );
        }
    }

    /// Checks the properties of a node, and returns them by key. A node other
    /// than a step takes no argument and no property twice.
    fn properties<'n>(&mut self, node: &'n KdlNode, allowed: &[&str], required: &[&str]) -> Properties<'n> {
        let name = node.name().value();
        let mut found = Properties(Vec::new());
        for entry in node.entries() {
            let Some(key) = entry.name() else {
                self.fault(
                    TraceCode::UnexpectedArgument,
                    self.span(entry.span()),
                    format!("A `{name}` node takes no argument."),
                );
                continue;
            };
            let key = key.value();
            if !allowed.contains(&key) {
                self.fault(
                    TraceCode::UnknownProperty,
                    self.span(entry.span()),
                    format!("`{key}` is not a property of a `{name}` node."),
                );
            } else if found.get(key).is_some() {
                self.fault(
                    TraceCode::DuplicateProperty,
                    self.span(entry.span()),
                    format!("The `{name}` node has the property `{key}` twice."),
                );
            } else {
                found.0.push((key.to_string(), entry));
            }
        }
        for key in required {
            if found.get(key).is_none() {
                self.fault(
                    TraceCode::MissingProperty,
                    self.node_span(node),
                    format!("A `{name}` node needs the property `{key}`."),
                );
            }
        }
        found
    }

    fn wrong_type(&mut self, entry: &KdlEntry, node: &str, expected: &str) {
        let key = entry.name().map_or("", |k| k.value());
        self.fault(
            TraceCode::WrongType,
            self.span(entry.span()),
            format!("The property `{key}` of a `{node}` node is {expected}."),
        );
    }

    fn non_negative(&mut self, entry: &KdlEntry, node: &str) -> Option<i128> {
        match entry.value() {
            KdlValue::Integer(n) if *n >= 0 => Some(*n),
            _ => {
                self.wrong_type(entry, node, "an integer of 0 or more");
                None
            }
        }
    }
}

/// The steps of the sequence, and the steps of one tree so far.
struct TreeSteps<'s> {
    symbols: &'s [Option<String>],
    count: usize,
    faulty: bool,
}

struct Properties<'n>(Vec<(String, &'n KdlEntry)>);

impl<'n> Properties<'n> {
    fn get(&self, key: &str) -> Option<&'n KdlEntry> {
        self.0.iter().find(|(k, _)| k == key).map(|(_, e)| *e)
    }
}

fn children(node: &KdlNode) -> &[KdlNode] {
    node.children().map_or(&[], KdlDocument::nodes)
}

fn write_reading(reading: &Reading) -> KdlNode {
    let mut node = KdlNode::new("reading");
    if let Some(parent) = reading.parent {
        push_int(&mut node, "parent", parent as i128);
    }
    node.push(KdlEntry::new_prop("can-end", reading.can_end));
    let children = node.ensure_children().nodes_mut();
    for option in &reading.next_options {
        let mut next = KdlNode::new("next");
        push_str(&mut next, "symbol", &option.symbol);
        for span in &option.occurrences {
            next.ensure_children().nodes_mut().push(span_node("occurrence", *span));
        }
        children.push(next);
    }
    children.push(write_tree(&reading.tree));
    node
}

fn span_node(name: &str, span: Span) -> KdlNode {
    let mut node = KdlNode::new(name);
    push_int(&mut node, "start", span.start as i128);
    push_int(&mut node, "end", span.end as i128);
    node
}

fn write_tree(tree: &TreeNode) -> KdlNode {
    let mut node = match &tree.kind {
        TreeKind::Rule { name } => {
            let mut node = KdlNode::new("rule");
            push_str(&mut node, "name", name);
            node
        }
        TreeKind::Loop { kind, pass_rule } => {
            let mut node = KdlNode::new("loop");
            push_str(&mut node, "kind", kind.as_str());
            if let Some(rule) = pass_rule {
                push_str(&mut node, "pass-rule", rule);
            }
            node
        }
        TreeKind::Pass { number } => {
            let mut node = KdlNode::new("pass");
            push_int(&mut node, "number", *number as i128);
            node
        }
        TreeKind::Step { number, symbol } => {
            let mut node = KdlNode::new("step");
            push_int(&mut node, "number", *number as i128);
            push_str(&mut node, "symbol", symbol);
            node
        }
    };
    for span in &tree.origins {
        node.ensure_children().nodes_mut().push(span_node("origin", *span));
    }
    for child in &tree.children {
        node.ensure_children().nodes_mut().push(write_tree(child));
    }
    node
}
