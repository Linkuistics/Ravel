//! The lowering from the lossless syntax tree to the checked grammar. The
//! lowering then runs the rule checks of `checks` on the result.
//!
//! The lowering reads what the parser built. After a syntax error, it keeps
//! each part that it can read, so that an editor can still show the rules. It
//! tells the checks which parts it could not read.

use sha2::{Digest, Sha256};

use crate::{
    checked::{CheckedGrammar, Diagnostic, Doc, Expr, FORMAT_VERSION, GrammarDoc, Rule, StartRule, Symbol, SymbolUse},
    checks::{self, Recovery},
    diagnostic::{RawDiagnostic, Span},
    string,
    syntax::{SyntaxKind, SyntaxNode, SyntaxToken},
};

pub fn lower(text: &str, root: &SyntaxNode, diagnostics: &[RawDiagnostic]) -> CheckedGrammar {
    let mut grammar = CheckedGrammar {
        format_version: FORMAT_VERSION,
        grammar_sha256: sha256_hex(text),
        grammar_docs: Vec::new(),
        start_rules: Vec::new(),
        rules: Vec::new(),
        symbols: Vec::new(),
        recursive_groups: Vec::new(),
        diagnostics: Vec::new(),
    };
    let mut recovery = Recovery::default();
    for node in root.children() {
        match node.kind() {
            SyntaxKind::DocDeclaration => {
                if let Some(token) = string_token(&node) {
                    grammar
                        .grammar_docs
                        .push(GrammarDoc { text: string_value(&token), span: token.text_range().into() });
                }
            }
            SyntaxKind::StartDeclaration => match name_token(&node) {
                Some(token) => {
                    grammar.start_rules.push(StartRule { name: name_value(&token), span: token.text_range().into() })
                }
                None => recovery.nameless_start = true,
            },
            SyntaxKind::Rule => {
                if let Some(rule) = lower_rule(&node, &mut recovery) {
                    grammar.rules.push(rule);
                }
            }
            _ => {}
        }
    }
    grammar.symbols = collect_symbols(&grammar.rules);
    let checks = checks::check(&grammar, &recovery);
    grammar.recursive_groups = checks.recursive_groups;
    // The syntax errors come first for one offset. The sort is stable, so each
    // list keeps its own order.
    let mut all: Vec<&RawDiagnostic> = diagnostics.iter().chain(&checks.diagnostics).collect();
    all.sort_by_key(|d| d.span.start);
    let lines = LineIndex::new(text, root);
    grammar.diagnostics = all
        .into_iter()
        .map(|d| {
            let (line, column) = lines.line_column(text, d.span.start as usize);
            Diagnostic { severity: d.severity, code: d.code, message: d.message.clone(), span: d.span, line, column }
        })
        .collect();
    grammar
}

/// The SHA-256 hash of a text, in lower-case hex.
pub fn sha256_hex(text: &str) -> String {
    Sha256::digest(text.as_bytes()).iter().map(|b| format!("{b:02x}")).collect()
}

fn lower_rule(node: &SyntaxNode, recovery: &mut Recovery) -> Option<Rule> {
    let name = name_token(node)?;
    let doc = node
        .children()
        .find(|n| n.kind() == SyntaxKind::DocString)
        .and_then(|n| string_token(&n))
        .map(|t| Doc { text: string_value(&t), span: t.text_range().into() });
    let body = node.children().find(|n| n.kind().is_expression()).and_then(|n| lower_expr(&n, recovery));
    Some(Rule { name: name_value(&name), span: name.text_range().into(), doc, body })
}

fn lower_expr(node: &SyntaxNode, recovery: &mut Recovery) -> Option<Expr> {
    let children = || node.children().filter(|n| n.kind().is_expression());
    match node.kind() {
        SyntaxKind::Symbol => {
            let token = string_token(node)?;
            Some(Expr::Symbol { value: string_value(&token), span: token.text_range().into() })
        }
        SyntaxKind::Reference => {
            let token = name_token(node)?;
            Some(Expr::Reference { name: token.text().to_string(), span: token.text_range().into() })
        }
        SyntaxKind::Group => children().next().and_then(|n| lower_expr(&n, recovery)),
        SyntaxKind::Sequence | SyntaxKind::Choice => {
            let mut items: Vec<Expr> = children().filter_map(|n| lower_expr(&n, recovery)).collect();
            match items.len() {
                0 => None,
                1 => items.pop(),
                _ => {
                    let span = Span { start: items[0].span().start, end: items[items.len() - 1].span().end };
                    Some(if node.kind() == SyntaxKind::Sequence {
                        Expr::Sequence { items, span }
                    } else {
                        Expr::Choice { alternatives: items, span }
                    })
                }
            }
        }
        SyntaxKind::Postfix => {
            let item = Box::new(children().next().and_then(|n| lower_expr(&n, recovery))?);
            let span = Span::from(node.text_range());
            // The first operator counts. A second operator sits in an error node.
            for element in node.children_with_tokens() {
                match element {
                    rowan::NodeOrToken::Token(token) => match token.kind() {
                        SyntaxKind::Question => return Some(Expr::Optional { item, span }),
                        SyntaxKind::Star => return Some(Expr::Repetition { min: 0, max: None, item, span }),
                        SyntaxKind::Plus => return Some(Expr::Repetition { min: 1, max: None, item, span }),
                        _ => {}
                    },
                    rowan::NodeOrToken::Node(child) if child.kind() == SyntaxKind::Bound => {
                        return Some(match bound_numbers(&child) {
                            Some((min, max)) => Expr::Repetition { min, max, item, span },
                            None => {
                                recovery.unread_bounds.push(item.span());
                                *item
                            }
                        });
                    }
                    rowan::NodeOrToken::Node(_) => {}
                }
            }
            Some(*item)
        }
        SyntaxKind::BracketSlip => {
            let item = Box::new(children().next().and_then(|n| lower_expr(&n, recovery))?);
            Some(Expr::Optional { item, span: node.text_range().into() })
        }
        SyntaxKind::BraceSlip => {
            let item = Box::new(children().next().and_then(|n| lower_expr(&n, recovery))?);
            Some(Expr::Repetition { min: 0, max: None, item, span: node.text_range().into() })
        }
        _ => None,
    }
}

/// The numbers of a well-formed bound: `{n}`, `{m,n}` or `{m,}`.
fn bound_numbers(node: &SyntaxNode) -> Option<(u32, Option<u32>)> {
    let tokens: Vec<SyntaxToken> = node
        .children_with_tokens()
        .filter_map(rowan::NodeOrToken::into_token)
        .filter(|t| !t.kind().is_trivia())
        .collect();
    let kinds: Vec<SyntaxKind> = tokens.iter().map(SyntaxToken::kind).collect();
    let number = |t: &SyntaxToken| t.text().parse::<u32>().unwrap_or(u32::MAX);
    use SyntaxKind::{Comma, LeftBrace, Number, RightBrace};
    match kinds.as_slice() {
        [LeftBrace, Number, RightBrace] => {
            let n = number(&tokens[1]);
            Some((n, Some(n)))
        }
        [LeftBrace, Number, Comma, Number, RightBrace] => Some((number(&tokens[1]), Some(number(&tokens[3])))),
        [LeftBrace, Number, Comma, RightBrace] => Some((number(&tokens[1]), None)),
        _ => None,
    }
}

fn collect_symbols(rules: &[Rule]) -> Vec<Symbol> {
    let mut symbols: Vec<Symbol> = Vec::new();
    fn walk(expr: &Expr, rule: &str, symbols: &mut Vec<Symbol>) {
        match expr {
            Expr::Symbol { value, span } => {
                let symbol_use = SymbolUse { rule: rule.to_string(), span: *span };
                match symbols.iter_mut().find(|s| &s.value == value) {
                    Some(symbol) => symbol.uses.push(symbol_use),
                    None => symbols.push(Symbol { value: value.clone(), uses: vec![symbol_use] }),
                }
            }
            Expr::Reference { .. } => {}
            Expr::Sequence { items, .. } | Expr::Choice { alternatives: items, .. } => {
                items.iter().for_each(|item| walk(item, rule, symbols));
            }
            Expr::Optional { item, .. } | Expr::Repetition { item, .. } => walk(item, rule, symbols),
        }
    }
    for rule in rules {
        if let Some(body) = &rule.body {
            walk(body, &rule.name, &mut symbols);
        }
    }
    symbols
}

/// The first string token of a node: a string or a string in single quotes.
fn string_token(node: &SyntaxNode) -> Option<SyntaxToken> {
    node.children_with_tokens().filter_map(rowan::NodeOrToken::into_token).find(|t| t.kind().is_string())
}

/// The token that names a rule: a name, or a string after a syntax error.
fn name_token(node: &SyntaxNode) -> Option<SyntaxToken> {
    node.children_with_tokens()
        .filter_map(rowan::NodeOrToken::into_token)
        .find(|t| t.kind() == SyntaxKind::Name || t.kind().is_string())
}

fn name_value(token: &SyntaxToken) -> String {
    if token.kind() == SyntaxKind::Name { token.text().to_string() } else { string_value(token) }
}

fn string_value(token: &SyntaxToken) -> String {
    match token.kind() {
        SyntaxKind::String => string::scan(token.text()).value,
        _ => token.text().trim_matches('\'').to_string(),
    }
}

/// The offsets of the line starts of a text, for lines and columns.
struct LineIndex {
    starts: Vec<usize>,
}

impl LineIndex {
    /// Finds the line starts. Outside strings, a line break is LF or CR LF, as
    /// the section "Text and tokens" of the spec states. Inside a string, each
    /// newline of KDL 2.0 is a line break, and CR LF is one line break. The
    /// rest of a broken string counts as inside the string.
    fn new(text: &str, root: &SyntaxNode) -> Self {
        let mut starts: Vec<usize> = std::iter::once(0).chain(text.match_indices('\n').map(|(i, _)| i + 1)).collect();
        let string_tokens = root
            .descendants_with_tokens()
            .filter_map(rowan::NodeOrToken::into_token)
            .filter(|t| matches!(t.kind(), SyntaxKind::String | SyntaxKind::BrokenStringRest));
        for token in string_tokens {
            let base = usize::from(token.text_range().start());
            for (i, c) in token.text().char_indices() {
                let end = base + i + c.len_utf8();
                let crlf = c == '\r' && text[end..].starts_with('\n');
                if c != '\n' && !crlf && string::is_newline(c) {
                    starts.push(end);
                }
            }
        }
        starts.sort_unstable();
        Self { starts }
    }

    fn line_column(&self, text: &str, offset: usize) -> (u32, u32) {
        let line = self.starts.partition_point(|&start| start <= offset) - 1;
        let column = text[self.starts[line]..offset].chars().count() + 1;
        (to_u32(line + 1), to_u32(column))
    }
}

fn to_u32(n: usize) -> u32 {
    u32::try_from(n).unwrap_or(u32::MAX)
}
