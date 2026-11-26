//! The one mapping between the Rust types of the core and KDL 2.0:
//! `docs/adr/kdl-for-derived-artifacts.md`.
//!
//! The mapping uses the `kdl` crate (https://docs.rs/kdl/6.7.1/kdl/). It
//! follows these rules, so that one program can later derive a schema or a
//! JSON form from the Rust types:
//!
//! 1. A struct becomes a node. The node name is the kebab-case name of the
//!    struct or of its role, such as `rule` or `start-rule`.
//! 2. A field with a scalar value becomes a property with the kebab-case field
//!    name. A node takes no positional arguments.
//! 3. A field with no value leaves its property out.
//! 4. A span becomes the two properties `start` and `end`, which are byte
//!    offsets into the grammar text.
//! 5. A list of structs becomes one child node for each struct, in order. A
//!    child struct becomes one child node.
//! 6. An enum whose variants hold data, such as an expression, becomes a node
//!    whose name is the kebab-case name of the variant. An enum whose variants
//!    hold no data, such as a severity or a badge, is a scalar value. Its
//!    property holds the kebab-case name of the variant as a string.
//!
//! The reader is strict. It refuses an unknown node, an unknown property, a
//! missing property and a positional argument.

use std::fmt;

use kdl::{KdlDocument, KdlEntry, KdlEntryFormat, KdlNode, KdlValue};

use crate::{
    checked::{
        Badge, CheckedGrammar, Code, Diagnostic, Doc, Expr, FORMAT_VERSION, GrammarDoc, GroupMember, RecursiveGroup,
        Rule, Severity, Span, StartRule, Symbol, SymbolUse,
    },
    string,
};

/// A KDL text that does not hold a checked grammar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MappingError {
    pub message: String,
}

impl fmt::Display for MappingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for MappingError {}

fn error<T>(message: impl Into<String>) -> Result<T, MappingError> {
    Err(MappingError { message: message.into() })
}

// Writing.

/// Writes a checked grammar as a KDL 2.0 document.
pub fn to_kdl(grammar: &CheckedGrammar) -> String {
    let mut root = KdlNode::new("checked-grammar");
    push_u32(&mut root, "format-version", grammar.format_version);
    push_str(&mut root, "grammar-sha256", &grammar.grammar_sha256);
    let children = root.ensure_children();
    for doc in &grammar.grammar_docs {
        let mut node = KdlNode::new("grammar-doc");
        push_str(&mut node, "text", &doc.text);
        push_span(&mut node, doc.span);
        children.nodes_mut().push(node);
    }
    for start in &grammar.start_rules {
        let mut node = KdlNode::new("start-rule");
        push_str(&mut node, "name", &start.name);
        push_span(&mut node, start.span);
        children.nodes_mut().push(node);
    }
    for rule in &grammar.rules {
        children.nodes_mut().push(write_rule(rule));
    }
    for symbol in &grammar.symbols {
        let mut node = KdlNode::new("symbol");
        push_str(&mut node, "value", &symbol.value);
        for symbol_use in &symbol.uses {
            let mut child = KdlNode::new("use");
            push_str(&mut child, "rule", &symbol_use.rule);
            push_span(&mut child, symbol_use.span);
            node.ensure_children().nodes_mut().push(child);
        }
        children.nodes_mut().push(node);
    }
    for group in &grammar.recursive_groups {
        let mut node = KdlNode::new("recursive-group");
        push_str(&mut node, "badge", group.badge.as_str());
        for member in &group.members {
            let mut child = KdlNode::new("member");
            push_str(&mut child, "rule", &member.rule);
            node.ensure_children().nodes_mut().push(child);
        }
        children.nodes_mut().push(node);
    }
    for diagnostic in &grammar.diagnostics {
        let mut node = KdlNode::new("diagnostic");
        push_str(&mut node, "severity", diagnostic.severity.as_str());
        push_str(&mut node, "code", diagnostic.code.as_str());
        push_str(&mut node, "message", &diagnostic.message);
        push_span(&mut node, diagnostic.span);
        push_u32(&mut node, "line", diagnostic.line);
        push_u32(&mut node, "column", diagnostic.column);
        children.nodes_mut().push(node);
    }
    let mut document = KdlDocument::new();
    document.nodes_mut().push(root);
    document.autoformat();
    document.to_string()
}

fn write_rule(rule: &Rule) -> KdlNode {
    let mut node = KdlNode::new("rule");
    push_str(&mut node, "name", &rule.name);
    push_span(&mut node, rule.span);
    if let Some(doc) = &rule.doc {
        let mut child = KdlNode::new("doc");
        push_str(&mut child, "text", &doc.text);
        push_span(&mut child, doc.span);
        node.ensure_children().nodes_mut().push(child);
    }
    if let Some(body) = &rule.body {
        node.ensure_children().nodes_mut().push(write_expr(body));
    }
    node
}

fn write_expr(expr: &Expr) -> KdlNode {
    let (name, span) = match expr {
        Expr::Symbol { span, .. } => ("symbol", span),
        Expr::Reference { span, .. } => ("reference", span),
        Expr::Sequence { span, .. } => ("sequence", span),
        Expr::Choice { span, .. } => ("choice", span),
        Expr::Optional { span, .. } => ("optional", span),
        Expr::Repetition { span, .. } => ("repetition", span),
    };
    let mut node = KdlNode::new(name);
    match expr {
        Expr::Symbol { value, .. } => push_str(&mut node, "value", value),
        Expr::Reference { name, .. } => push_str(&mut node, "name", name),
        Expr::Repetition { min, max, .. } => {
            push_u32(&mut node, "min", *min);
            if let Some(max) = max {
                push_u32(&mut node, "max", *max);
            }
        }
        Expr::Sequence { .. } | Expr::Choice { .. } | Expr::Optional { .. } => {}
    }
    push_span(&mut node, *span);
    let items: &[Expr] = match expr {
        Expr::Sequence { items, .. } | Expr::Choice { alternatives: items, .. } => items,
        Expr::Optional { item, .. } | Expr::Repetition { item, .. } => std::slice::from_ref(item),
        Expr::Symbol { .. } | Expr::Reference { .. } => &[],
    };
    for item in items {
        node.ensure_children().nodes_mut().push(write_expr(item));
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

pub(crate) fn push_str(node: &mut KdlNode, key: &str, value: &str) {
    let mut entry = KdlEntry::new_prop(key, value);
    keep_string_text(&mut entry, value);
    node.push(entry);
}

/// Makes an entry write its string value as `string_text` gives it, also
/// after `autoformat`.
pub(crate) fn keep_string_text(entry: &mut KdlEntry, value: &str) {
    entry.set_format(KdlEntryFormat {
        value_repr: string_text(value),
        leading: " ".into(),
        autoformat_keep: true,
        ..Default::default()
    });
}

/// The KDL 2.0 text of a string: the string itself where KDL 2.0 takes it as
/// an identifier string, and a quoted string with escapes elsewhere.
///
/// The `kdl` crate 6.7.1 writes some strings as invalid KDL 2.0. It writes a
/// disallowed code point literally, also inside quotes. It also writes a
/// string such as `+.1` without quotes. So the core writes each string itself.
pub(crate) fn string_text(value: &str) -> String {
    if is_identifier_string(value) { value.to_string() } else { escaped(value) }
}

/// Whether KDL 2.0 reads a string without quotes as an identifier string with
/// that value. Source: the KDL 2.0 specification, section "Identifier String".
fn is_identifier_string(value: &str) -> bool {
    let mut chars = value.chars();
    let (first, second, third) = (chars.next(), chars.next(), chars.next());
    let digit = |c: Option<char>| c.is_some_and(|c| c.is_ascii_digit());
    let starts_like_a_number = match first {
        None => return false,
        Some(c) if c.is_ascii_digit() => true,
        Some('+' | '-') => digit(second) || (second == Some('.') && digit(third)),
        Some('.') => digit(second),
        Some(_) => false,
    };
    !starts_like_a_number
        && !matches!(value, "true" | "false" | "null" | "inf" | "-inf" | "nan")
        && value.chars().all(is_identifier_char)
}

/// Whether a character can be part of an identifier string in KDL 2.0. The
/// equals signs other than `=` are the ones that KDL 2.0 lists. The `kdl`
/// crate does not read them as equals signs, so the core quotes them for
/// both readings.
fn is_identifier_char(c: char) -> bool {
    !(string::is_unicode_space(c)
        || string::is_newline(c)
        || string::is_disallowed(c)
        || matches!(
            c,
            '\\' | '/'
                | '('
                | ')'
                | '{'
                | '}'
                | ';'
                | '['
                | ']'
                | '"'
                | '#'
                | '='
                | '\u{FE66}'
                | '\u{FF1D}'
                | '\u{1F7F0}'
        ))
}

/// Whether a character needs a `\u{…}` escape in a KDL 2.0 quoted string.
fn needs_unicode_escape(c: char) -> bool {
    string::is_disallowed(c) || matches!(c, '\u{000B}' | '\u{0085}' | '\u{2028}' | '\u{2029}')
}

/// The first code point in a text that KDL 2.0 forbids as a literal
/// character, with its byte offset. A byte order mark can start a document.
///
/// The parser of the `kdl` crate 6.7.1 accepts a literal U+007F, so a reader
/// checks the text with this function too.
pub(crate) fn first_disallowed(text: &str) -> Option<(usize, char)> {
    let skip = if text.starts_with('\u{FEFF}') { '\u{FEFF}'.len_utf8() } else { 0 };
    text[skip..].char_indices().find(|&(_, c)| string::is_disallowed(c)).map(|(i, c)| (skip + i, c))
}

fn escaped(value: &str) -> String {
    let mut out = String::from("\"");
    for c in value.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{0008}' => out.push_str("\\b"),
            '\u{000C}' => out.push_str("\\f"),
            c if needs_unicode_escape(c) => out.push_str(&format!("\\u{{{:x}}}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

// Reading.

/// Reads a checked grammar from a KDL 2.0 document.
pub fn from_kdl(text: &str) -> Result<CheckedGrammar, MappingError> {
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
        return error("a checked grammar is one `checked-grammar` node");
    };
    let fields = Fields::new(root, "checked-grammar", &["format-version", "grammar-sha256"])?;
    let format_version = fields.u32("format-version")?;
    if format_version != FORMAT_VERSION {
        return error(format!(
            "format version {format_version} is not {FORMAT_VERSION}, the version that this core reads"
        ));
    }
    let mut grammar = CheckedGrammar {
        format_version,
        grammar_sha256: fields.string("grammar-sha256")?,
        grammar_docs: Vec::new(),
        start_rules: Vec::new(),
        rules: Vec::new(),
        symbols: Vec::new(),
        recursive_groups: Vec::new(),
        diagnostics: Vec::new(),
    };
    for node in children(root) {
        match node.name().value() {
            "grammar-doc" => {
                let f = Fields::new(node, "grammar-doc", &["text", "start", "end"])?;
                f.no_children()?;
                grammar.grammar_docs.push(GrammarDoc { text: f.string("text")?, span: f.span()? });
            }
            "start-rule" => {
                let f = Fields::new(node, "start-rule", &["name", "start", "end"])?;
                f.no_children()?;
                grammar.start_rules.push(StartRule { name: f.string("name")?, span: f.span()? });
            }
            "rule" => grammar.rules.push(read_rule(node)?),
            "symbol" => grammar.symbols.push(read_symbol(node)?),
            "recursive-group" => grammar.recursive_groups.push(read_recursive_group(node)?),
            "diagnostic" => grammar.diagnostics.push(read_diagnostic(node)?),
            other => return error(format!("`{other}` is not a node of a checked grammar")),
        }
    }
    Ok(grammar)
}

fn read_rule(node: &KdlNode) -> Result<Rule, MappingError> {
    let f = Fields::new(node, "rule", &["name", "start", "end"])?;
    let mut rule = Rule { name: f.string("name")?, span: f.span()?, doc: None, body: None };
    for child in children(node) {
        if child.name().value() == "doc" {
            if rule.doc.is_some() || rule.body.is_some() {
                return error(format!("the rule `{}` has its `doc` node out of place", rule.name));
            }
            let d = Fields::new(child, "doc", &["text", "start", "end"])?;
            d.no_children()?;
            rule.doc = Some(Doc { text: d.string("text")?, span: d.span()? });
        } else if rule.body.is_none() {
            rule.body = Some(read_expr(child)?);
        } else {
            return error(format!("the rule `{}` has more than one body", rule.name));
        }
    }
    Ok(rule)
}

fn read_expr(node: &KdlNode) -> Result<Expr, MappingError> {
    let name = node.name().value();
    let items = || children(node).iter().map(read_expr).collect::<Result<Vec<_>, _>>();
    let one_item = |items: Vec<Expr>| -> Result<Box<Expr>, MappingError> {
        match <[Expr; 1]>::try_from(items) {
            Ok([item]) => Ok(Box::new(item)),
            Err(_) => error(format!("a `{name}` node has exactly one child")),
        }
    };
    match name {
        "symbol" => {
            let f = Fields::new(node, name, &["value", "start", "end"])?;
            f.no_children()?;
            Ok(Expr::Symbol { value: f.string("value")?, span: f.span()? })
        }
        "reference" => {
            let f = Fields::new(node, name, &["name", "start", "end"])?;
            f.no_children()?;
            Ok(Expr::Reference { name: f.string("name")?, span: f.span()? })
        }
        "sequence" | "choice" => {
            let f = Fields::new(node, name, &["start", "end"])?;
            let items = items()?;
            if items.len() < 2 {
                return error(format!("a `{name}` node has at least two children"));
            }
            let span = f.span()?;
            Ok(if name == "sequence" {
                Expr::Sequence { items, span }
            } else {
                Expr::Choice { alternatives: items, span }
            })
        }
        "optional" => {
            let f = Fields::new(node, name, &["start", "end"])?;
            Ok(Expr::Optional { item: one_item(items()?)?, span: f.span()? })
        }
        "repetition" => {
            let f = Fields::new(node, name, &["min", "max", "start", "end"])?;
            Ok(Expr::Repetition {
                min: f.u32("min")?,
                max: f.optional_u32("max")?,
                item: one_item(items()?)?,
                span: f.span()?,
            })
        }
        other => error(format!("`{other}` is not an expression node")),
    }
}

fn read_symbol(node: &KdlNode) -> Result<Symbol, MappingError> {
    let f = Fields::new(node, "symbol", &["value"])?;
    let mut symbol = Symbol { value: f.string("value")?, uses: Vec::new() };
    for child in children(node) {
        if child.name().value() != "use" {
            return error(format!("`{}` is not a child node of `symbol`", child.name().value()));
        }
        let u = Fields::new(child, "use", &["rule", "start", "end"])?;
        u.no_children()?;
        symbol.uses.push(SymbolUse { rule: u.string("rule")?, span: u.span()? });
    }
    Ok(symbol)
}

fn read_recursive_group(node: &KdlNode) -> Result<RecursiveGroup, MappingError> {
    let f = Fields::new(node, "recursive-group", &["badge"])?;
    let badge = f.string("badge")?;
    let badge = Badge::parse(&badge).ok_or_else(|| MappingError { message: format!("`{badge}` is not a badge") })?;
    let mut group = RecursiveGroup { badge, members: Vec::new() };
    for child in children(node) {
        let m = Fields::new(child, "member", &["rule"])?;
        m.no_children()?;
        group.members.push(GroupMember { rule: m.string("rule")? });
    }
    if group.members.is_empty() {
        return error("a `recursive-group` node has at least one `member` child");
    }
    Ok(group)
}

fn read_diagnostic(node: &KdlNode) -> Result<Diagnostic, MappingError> {
    let f = Fields::new(node, "diagnostic", &["severity", "code", "message", "start", "end", "line", "column"])?;
    f.no_children()?;
    let severity = f.string("severity")?;
    let code = f.string("code")?;
    Ok(Diagnostic {
        severity: Severity::parse(&severity)
            .ok_or_else(|| MappingError { message: format!("`{severity}` is not a severity") })?,
        code: Code::parse(&code)
            .ok_or_else(|| MappingError { message: format!("`{code}` is not a diagnostic code") })?,
        message: f.string("message")?,
        span: f.span()?,
        line: f.u32("line")?,
        column: f.u32("column")?,
    })
}

fn children(node: &KdlNode) -> &[KdlNode] {
    node.children().map_or(&[], KdlDocument::nodes)
}

/// The properties of one node, checked against the properties that its type allows.
struct Fields<'a> {
    node: &'a KdlNode,
    name: &'a str,
}

impl<'a> Fields<'a> {
    fn new(node: &'a KdlNode, name: &'a str, allowed: &[&str]) -> Result<Self, MappingError> {
        if node.name().value() != name {
            return error(format!("expected a `{name}` node, found `{}`", node.name().value()));
        }
        let mut seen: Vec<&str> = Vec::new();
        for entry in node.entries() {
            let Some(key) = entry.name() else {
                return error(format!("a `{name}` node takes no positional arguments"));
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
        Ok(Self { node, name })
    }

    fn no_children(&self) -> Result<(), MappingError> {
        if children(self.node).is_empty() { Ok(()) } else { error(format!("a `{}` node has no children", self.name)) }
    }

    fn value(&self, key: &str) -> Option<&KdlValue> {
        self.node.get(key)
    }

    fn string(&self, key: &str) -> Result<String, MappingError> {
        match self.value(key) {
            Some(KdlValue::String(s)) => Ok(s.clone()),
            Some(_) => error(format!("the property `{key}` of a `{}` node is a string", self.name)),
            None => error(format!("a `{}` node needs the property `{key}`", self.name)),
        }
    }

    fn optional_u32(&self, key: &str) -> Result<Option<u32>, MappingError> {
        match self.value(key) {
            None => Ok(None),
            Some(KdlValue::Integer(n)) => match u32::try_from(*n) {
                Ok(n) => Ok(Some(n)),
                Err(_) => error(format!("the property `{key}` of a `{}` node is out of range", self.name)),
            },
            Some(_) => error(format!("the property `{key}` of a `{}` node is an integer", self.name)),
        }
    }

    fn u32(&self, key: &str) -> Result<u32, MappingError> {
        self.optional_u32(key)?
            .ok_or_else(|| MappingError { message: format!("a `{}` node needs the property `{key}`", self.name) })
    }

    fn span(&self) -> Result<Span, MappingError> {
        Ok(Span { start: self.u32("start")?, end: self.u32("end")? })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> CheckedGrammar {
        let span = Span { start: 1, end: 2 };
        CheckedGrammar {
            format_version: FORMAT_VERSION,
            grammar_sha256: "00ff".into(),
            grammar_docs: vec![GrammarDoc { text: "line one\nline \"two\"".into(), span }],
            start_rules: vec![StartRule { name: "a".into(), span }],
            rules: vec![
                Rule {
                    name: "a".into(),
                    span,
                    doc: Some(Doc { text: "doc".into(), span }),
                    body: Some(Expr::Sequence {
                        items: vec![
                            Expr::Symbol { value: "x y".into(), span },
                            Expr::Choice {
                                alternatives: vec![
                                    Expr::Optional { item: Box::new(Expr::Reference { name: "b".into(), span }), span },
                                    Expr::Repetition {
                                        min: 2,
                                        max: None,
                                        item: Box::new(Expr::Symbol { value: "true".into(), span }),
                                        span,
                                    },
                                ],
                                span,
                            },
                            Expr::Repetition {
                                min: 0,
                                max: Some(3),
                                item: Box::new(Expr::Symbol { value: "\u{1}\u{2028}#".into(), span }),
                                span,
                            },
                        ],
                        span,
                    }),
                },
                Rule { name: "b".into(), span, doc: None, body: None },
            ],
            symbols: vec![Symbol { value: "x y".into(), uses: vec![SymbolUse { rule: "a".into(), span }] }],
            recursive_groups: vec![RecursiveGroup {
                badge: Badge::MayNeedAStack,
                members: vec![GroupMember { rule: "a".into() }, GroupMember { rule: "b".into() }],
            }],
            diagnostics: vec![Diagnostic {
                severity: Severity::Warning,
                code: Code::UnexpectedToken,
                message: "a `message`".into(),
                span,
                line: 1,
                column: 2,
            }],
        }
    }

    #[test]
    fn types_survive_a_round_trip() {
        let grammar = sample();
        let text = to_kdl(&grammar);
        assert_eq!(from_kdl(&text), Ok(grammar), "{text}");
        assert_eq!(to_kdl(&from_kdl(&text).unwrap()), text);
    }

    #[test]
    fn the_reader_refuses_unknown_parts() {
        let text = to_kdl(&sample());
        let unknown_property = text.replacen("grammar-doc text", "grammar-doc colour=red text", 1);
        assert!(from_kdl(&unknown_property).unwrap_err().message.contains("`colour`"));
        let argument = text.replacen("start-rule name=a", "start-rule a name=a", 1);
        assert!(from_kdl(&argument).unwrap_err().message.contains("positional"));
        let unknown_node = text.replacen("start-rule", "end-rule", 1);
        assert!(from_kdl(&unknown_node).unwrap_err().message.contains("`end-rule`"));
        let version = text.replacen(&format!("format-version={FORMAT_VERSION}"), "format-version=99", 1);
        assert!(from_kdl(&version).unwrap_err().message.contains("format version 99"));
        let badge = text.replacen("badge=may-need-a-stack", "badge=not-regular", 1);
        assert!(from_kdl(&badge).unwrap_err().message.contains("`not-regular` is not a badge"));
    }
}
