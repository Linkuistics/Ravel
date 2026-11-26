//! The diagram KDL: `docs/specs/rule-diagrams.md`, section "The diagram KDL".
//!
//! The form departs from the mapping rules of `kdl_map` in two places, as the
//! spec states. The lines of a label are the arguments of its `label` node.
//! The points of a flow are its arguments, as `x1 y1 x2 y2 …`.

use kdl::{KdlDocument, KdlEntry, KdlNode, KdlValue};

use crate::{
    checked::Span,
    diagram::{
        CELL_WIDTH, DIAGRAM_FORMAT_VERSION, Diagram, Diagrams, FONT_SIZE, Flow, FlowLabel, FlowRole, GatewayRole,
        LINE_HEIGHT, LoopLink, Point, Shape,
    },
    kdl_map::{MappingError, first_disallowed, keep_string_text, push_str},
};

// Writing.

/// Writes the diagrams as a KDL 2.0 document.
pub(crate) fn write(diagrams: &Diagrams) -> String {
    let mut root = KdlNode::new("diagrams");
    push_u32(&mut root, "format-version", diagrams.format_version);
    push_str(&mut root, "grammar-sha256", &diagrams.grammar_sha256);
    push_u32(&mut root, "font-size", diagrams.font_size);
    root.push(KdlEntry::new_prop("cell-width", diagrams.cell_width));
    push_u32(&mut root, "line-height", diagrams.line_height);
    for diagram in &diagrams.diagrams {
        root.ensure_children().nodes_mut().push(write_diagram(diagram));
    }
    let mut document = KdlDocument::new();
    document.nodes_mut().push(root);
    document.autoformat();
    document.to_string()
}

fn write_diagram(diagram: &Diagram) -> KdlNode {
    let mut node = KdlNode::new("diagram");
    push_str(&mut node, "rule", &diagram.rule);
    push_u32(&mut node, "width", diagram.width);
    push_u32(&mut node, "height", diagram.height);
    let children = node.ensure_children();
    for shape in &diagram.shapes {
        children.nodes_mut().push(write_shape(shape));
    }
    for flow in &diagram.flows {
        children.nodes_mut().push(write_flow(flow));
    }
    node
}

fn write_shape(shape: &Shape) -> KdlNode {
    match shape {
        Shape::StartEvent { x, y, size } | Shape::EndEvent { x, y, size } => {
            let name = if matches!(shape, Shape::StartEvent { .. }) { "start-event" } else { "end-event" };
            let mut node = KdlNode::new(name);
            push_u32(&mut node, "x", *x);
            push_u32(&mut node, "y", *y);
            push_u32(&mut node, "size", *size);
            node
        }
        Shape::Task { span, symbol, x, y, width, height, label } => {
            let mut node = KdlNode::new("task");
            push_span(&mut node, *span);
            push_str(&mut node, "symbol", symbol);
            push_box(&mut node, *x, *y, *width, *height);
            push_label(&mut node, label);
            node
        }
        Shape::Reference { span, rule, x, y, width, height, one_step, loop_link, label } => {
            let mut node = KdlNode::new("reference");
            push_span(&mut node, *span);
            push_str(&mut node, "rule", rule);
            push_box(&mut node, *x, *y, *width, *height);
            node.push(KdlEntry::new_prop("one-step", *one_step));
            if let Some(link) = loop_link {
                push_str(&mut node, "loop-link", link.as_str());
            }
            push_label(&mut node, label);
            node
        }
        Shape::Gateway { role, span, x, y, size } => {
            let mut node = KdlNode::new("gateway");
            push_str(&mut node, "role", role.as_str());
            push_span(&mut node, *span);
            push_u32(&mut node, "x", *x);
            push_u32(&mut node, "y", *y);
            push_u32(&mut node, "size", *size);
            node
        }
    }
}

fn write_flow(flow: &Flow) -> KdlNode {
    let mut node = KdlNode::new("flow");
    for point in &flow.points {
        node.push(KdlEntry::new(i128::from(point.x)));
        node.push(KdlEntry::new(i128::from(point.y)));
    }
    push_str(&mut node, "role", flow.role.as_str());
    if let Some(span) = flow.span {
        push_span(&mut node, span);
    }
    if let Some(label) = &flow.label {
        push_str(&mut node, "label", &label.text);
        push_u32(&mut node, "label-x", label.x);
        push_u32(&mut node, "label-y", label.y);
    }
    node
}

fn push_u32(node: &mut KdlNode, key: &str, value: u32) {
    node.push(KdlEntry::new_prop(key, i128::from(value)));
}

fn push_span(node: &mut KdlNode, span: Span) {
    push_u32(node, "start", span.start);
    push_u32(node, "end", span.end);
}

fn push_box(node: &mut KdlNode, x: u32, y: u32, width: u32, height: u32) {
    push_u32(node, "x", x);
    push_u32(node, "y", y);
    push_u32(node, "width", width);
    push_u32(node, "height", height);
}

fn push_label(node: &mut KdlNode, lines: &[String]) {
    let mut label = KdlNode::new("label");
    for line in lines {
        let mut entry = KdlEntry::new(line.as_str());
        keep_string_text(&mut entry, line);
        label.push(entry);
    }
    node.ensure_children().nodes_mut().push(label);
}

// Reading.

fn error<T>(message: impl Into<String>) -> Result<T, MappingError> {
    Err(MappingError { message: message.into() })
}

/// Reads the diagram KDL. The reader refuses an unknown node, an unknown
/// property, a missing property and an argument where the form has none.
pub(crate) fn read(text: &str) -> Result<Diagrams, MappingError> {
    if let Some((offset, c)) = first_disallowed(text) {
        return error(format!(
            "the text is not KDL 2.0: it holds the code point U+{:04X} at byte {offset}, \
             which KDL 2.0 forbids as a literal character",
            c as u32
        ));
    }
    let document = match KdlDocument::parse_v2(text) {
        Ok(document) => document,
        Err(e) => {
            let details: Vec<String> = e.diagnostics.iter().filter_map(|d| d.message.clone()).collect();
            return error(format!("the text is not KDL 2.0: {}", details.join("; ")));
        }
    };
    let [root] = document.nodes() else {
        return error("the diagram KDL is one `diagrams` node");
    };
    let f = Entries::new(
        root,
        "diagrams",
        &["format-version", "grammar-sha256", "font-size", "cell-width", "line-height"],
        false,
    )?;
    let format_version = f.u32("format-version")?;
    if format_version != DIAGRAM_FORMAT_VERSION {
        return error(format!(
            "format version {format_version} is not {DIAGRAM_FORMAT_VERSION}, the version that this core reads"
        ));
    }
    let cell_width = match f.value("cell-width") {
        Some(KdlValue::Float(w)) => *w,
        Some(KdlValue::Integer(w)) => *w as f64,
        Some(_) => return error("the property `cell-width` of a `diagrams` node is a number"),
        None => return error("a `diagrams` node needs the property `cell-width`"),
    };
    let diagrams = Diagrams {
        format_version,
        grammar_sha256: f.string("grammar-sha256")?,
        font_size: f.u32("font-size")?,
        cell_width,
        line_height: f.u32("line-height")?,
        diagrams: children(root).iter().map(read_diagram).collect::<Result<_, _>>()?,
    };
    if (diagrams.font_size, diagrams.cell_width, diagrams.line_height) != (FONT_SIZE, CELL_WIDTH, LINE_HEIGHT) {
        return error(format!(
            "the font metrics are not those of this core: font-size={FONT_SIZE} cell-width={CELL_WIDTH} \
             line-height={LINE_HEIGHT}"
        ));
    }
    Ok(diagrams)
}

fn read_diagram(node: &KdlNode) -> Result<Diagram, MappingError> {
    let f = Entries::new(node, "diagram", &["rule", "width", "height"], false)?;
    let mut diagram = Diagram {
        rule: f.string("rule")?,
        width: f.u32("width")?,
        height: f.u32("height")?,
        shapes: Vec::new(),
        flows: Vec::new(),
    };
    for child in children(node) {
        if child.name().value() == "flow" {
            diagram.flows.push(read_flow(child)?);
        } else if diagram.flows.is_empty() {
            diagram.shapes.push(read_shape(child)?);
        } else {
            return error(format!("a `{}` node comes after the flows of a diagram", child.name().value()));
        }
    }
    Ok(diagram)
}

fn read_shape(node: &KdlNode) -> Result<Shape, MappingError> {
    let name = node.name().value();
    match name {
        "start-event" | "end-event" => {
            let f = Entries::new(node, name, &["x", "y", "size"], false)?;
            f.no_children()?;
            let (x, y, size) = (f.u32("x")?, f.u32("y")?, f.u32("size")?);
            Ok(if name == "start-event" { Shape::StartEvent { x, y, size } } else { Shape::EndEvent { x, y, size } })
        }
        "task" => {
            let f = Entries::new(node, name, &["start", "end", "symbol", "x", "y", "width", "height"], false)?;
            Ok(Shape::Task {
                span: f.span()?,
                symbol: f.string("symbol")?,
                x: f.u32("x")?,
                y: f.u32("y")?,
                width: f.u32("width")?,
                height: f.u32("height")?,
                label: read_label(node)?,
            })
        }
        "reference" => {
            let allowed = ["start", "end", "rule", "x", "y", "width", "height", "one-step", "loop-link"];
            let f = Entries::new(node, name, &allowed, false)?;
            let one_step = match f.value("one-step") {
                Some(KdlValue::Bool(b)) => *b,
                Some(_) => return error("the property `one-step` of a `reference` node is a boolean"),
                None => return error("a `reference` node needs the property `one-step`"),
            };
            let loop_link = match f.optional_string("loop-link")? {
                Some(text) => Some(
                    LoopLink::parse(&text).ok_or_else(|| mapping(format!("`{text}` is not a kind of loop link")))?,
                ),
                None => None,
            };
            Ok(Shape::Reference {
                span: f.span()?,
                rule: f.string("rule")?,
                x: f.u32("x")?,
                y: f.u32("y")?,
                width: f.u32("width")?,
                height: f.u32("height")?,
                one_step,
                loop_link,
                label: read_label(node)?,
            })
        }
        "gateway" => {
            let f = Entries::new(node, name, &["role", "start", "end", "x", "y", "size"], false)?;
            f.no_children()?;
            let text = f.string("role")?;
            let role =
                GatewayRole::parse(&text).ok_or_else(|| mapping(format!("`{text}` is not a role of a gateway")))?;
            Ok(Shape::Gateway { role, span: f.span()?, x: f.u32("x")?, y: f.u32("y")?, size: f.u32("size")? })
        }
        other => error(format!("`{other}` is not a node of a diagram")),
    }
}

fn read_label(node: &KdlNode) -> Result<Vec<String>, MappingError> {
    let [label] = children(node) else {
        return error(format!("a `{}` node has exactly one `label` node", node.name().value()));
    };
    let f = Entries::new(label, "label", &[], true)?;
    f.no_children()?;
    f.arguments
        .iter()
        .map(|v| match v {
            KdlValue::String(s) => Ok(s.clone()),
            _ => error("each argument of a `label` node is a string"),
        })
        .collect()
}

fn read_flow(node: &KdlNode) -> Result<Flow, MappingError> {
    let f = Entries::new(node, "flow", &["role", "start", "end", "label", "label-x", "label-y"], true)?;
    f.no_children()?;
    let text = f.string("role")?;
    let role = FlowRole::parse(&text).ok_or_else(|| mapping(format!("`{text}` is not a role of a flow")))?;
    let span = match (f.value("start"), f.value("end")) {
        (None, None) => None,
        _ => Some(f.span()?),
    };
    let label = match f.optional_string("label")? {
        Some(text) => Some(FlowLabel { text, x: f.u32("label-x")?, y: f.u32("label-y")? }),
        None if f.value("label-x").is_some() || f.value("label-y").is_some() => {
            return error("a `flow` node with `label-x` or `label-y` needs the property `label`");
        }
        None => None,
    };
    let coordinates = f
        .arguments
        .iter()
        .map(|v| match v {
            KdlValue::Integer(n) => u32::try_from(*n).map_err(|_| mapping("a point of a `flow` node is out of range")),
            _ => error("each argument of a `flow` node is an integer"),
        })
        .collect::<Result<Vec<u32>, _>>()?;
    if coordinates.len() < 4 || coordinates.len() % 2 != 0 {
        return error("a `flow` node has two points or more, as pairs of integers");
    }
    let points = coordinates.chunks(2).map(|c| Point { x: c[0], y: c[1] }).collect();
    Ok(Flow { role, span, label, points })
}

fn mapping(message: impl Into<String>) -> MappingError {
    MappingError { message: message.into() }
}

fn children(node: &KdlNode) -> &[KdlNode] {
    node.children().map_or(&[], KdlDocument::nodes)
}

/// The entries of one node, checked against the properties that its type
/// allows.
struct Entries<'a> {
    node: &'a KdlNode,
    name: &'a str,
    arguments: Vec<&'a KdlValue>,
}

impl<'a> Entries<'a> {
    fn new(node: &'a KdlNode, name: &'a str, allowed: &[&str], arguments: bool) -> Result<Self, MappingError> {
        if node.name().value() != name {
            return error(format!("expected a `{name}` node, found `{}`", node.name().value()));
        }
        let mut seen: Vec<&str> = Vec::new();
        let mut values = Vec::new();
        for entry in node.entries() {
            let Some(key) = entry.name() else {
                if !arguments {
                    return error(format!("a `{name}` node takes no positional arguments"));
                }
                values.push(entry.value());
                continue;
            };
            let key = key.value();
            if !allowed.contains(&key) {
                return error(format!("`{key}` is not a property of a `{name}` node"));
            }
            if seen.contains(&key) {
                return error(format!("a `{name}` node has the property `{key}` twice"));
            }
            seen.push(key);
        }
        Ok(Self { node, name, arguments: values })
    }

    fn no_children(&self) -> Result<(), MappingError> {
        if children(self.node).is_empty() { Ok(()) } else { error(format!("a `{}` node has no children", self.name)) }
    }

    fn value(&self, key: &str) -> Option<&KdlValue> {
        self.node.get(key)
    }

    fn optional_string(&self, key: &str) -> Result<Option<String>, MappingError> {
        match self.value(key) {
            None => Ok(None),
            Some(KdlValue::String(s)) => Ok(Some(s.clone())),
            Some(_) => error(format!("the property `{key}` of a `{}` node is a string", self.name)),
        }
    }

    fn string(&self, key: &str) -> Result<String, MappingError> {
        self.optional_string(key)?.ok_or_else(|| mapping(format!("a `{}` node needs the property `{key}`", self.name)))
    }

    fn u32(&self, key: &str) -> Result<u32, MappingError> {
        match self.value(key) {
            Some(KdlValue::Integer(n)) => u32::try_from(*n)
                .map_err(|_| mapping(format!("the property `{key}` of a `{}` node is out of range", self.name))),
            Some(_) => error(format!("the property `{key}` of a `{}` node is an integer", self.name)),
            None => error(format!("a `{}` node needs the property `{key}`", self.name)),
        }
    }

    fn span(&self) -> Result<Span, MappingError> {
        Ok(Span { start: self.u32("start")?, end: self.u32("end")? })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Diagrams {
        let span = Span { start: 3, end: 9 };
        Diagrams {
            format_version: DIAGRAM_FORMAT_VERSION,
            grammar_sha256: "00ff".into(),
            font_size: FONT_SIZE,
            cell_width: CELL_WIDTH,
            line_height: LINE_HEIGHT,
            diagrams: vec![Diagram {
                rule: "a rule".into(),
                width: 300,
                height: 100,
                shapes: vec![
                    Shape::StartEvent { x: 16, y: 20, size: 24 },
                    Shape::Task {
                        span,
                        symbol: "x \"y\"\n#".into(),
                        x: 64,
                        y: 14,
                        width: 64,
                        height: 36,
                        label: vec!["x \"y\"".into(), "#true".into()],
                    },
                    Shape::Reference {
                        span,
                        rule: "true".into(),
                        x: 1,
                        y: 2,
                        width: 3,
                        height: 4,
                        one_step: false,
                        loop_link: Some(LoopLink::HeadReferences),
                        label: vec!["true".into()],
                    },
                    Shape::Gateway { role: GatewayRole::BackEdgeJoin, span, x: 5, y: 6, size: 24 },
                    Shape::EndEvent { x: 200, y: 20, size: 24 },
                ],
                flows: vec![
                    Flow {
                        role: FlowRole::Sequence,
                        span: None,
                        label: None,
                        points: vec![Point { x: 40, y: 32 }, Point { x: 64, y: 32 }],
                    },
                    Flow {
                        role: FlowRole::BackEdge,
                        span: Some(span),
                        label: Some(FlowLabel { text: "↻ a rule".into(), x: 7, y: 8 }),
                        points: vec![Point { x: 1, y: 2 }, Point { x: 1, y: 9 }, Point { x: 4, y: 9 }],
                    },
                ],
            }],
        }
    }

    #[test]
    fn types_survive_a_round_trip() {
        let diagrams = sample();
        let text = write(&diagrams);
        assert_eq!(read(&text), Ok(diagrams), "{text}");
        assert_eq!(write(&read(&text).unwrap()), text);
        assert!(text.contains("cell-width=7.8"), "{text}");
        assert!(text.contains("one-step=#false"), "{text}");
    }

    #[test]
    fn the_reader_refuses_unknown_parts() {
        let text = write(&sample());
        let property = text.replacen("start-event x", "start-event colour=red x", 1);
        assert!(read(&property).unwrap_err().message.contains("`colour`"));
        let argument = text.replacen("start-event x", "start-event 1 x", 1);
        assert!(read(&argument).unwrap_err().message.contains("positional"));
        let node = text.replacen("start-event", "middle-event", 1);
        assert!(read(&node).unwrap_err().message.contains("`middle-event`"));
        let version = text.replacen("format-version=1", "format-version=99", 1);
        assert!(read(&version).unwrap_err().message.contains("format version 99"));
        let role = text.replacen("role=back-edge-join", "role=parallel", 1);
        assert!(read(&role).unwrap_err().message.contains("`parallel`"));
        let odd = text.replacen("flow 40 32 64 32", "flow 40 32 64", 1);
        assert!(read(&odd).unwrap_err().message.contains("pairs"));
        let metrics = text.replacen("cell-width=7.8", "cell-width=8.0", 1);
        assert!(read(&metrics).unwrap_err().message.contains("font metrics"));
    }
}
