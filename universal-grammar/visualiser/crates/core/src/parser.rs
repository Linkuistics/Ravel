//! The hand-written parser: `docs/adr/hand-written-parser.md`.
//!
//! The parser reads the tokens of the lexer and builds the lossless syntax
//! tree. It follows the syntax in `docs/specs/grammar-notation.md`, and it
//! reports the syntax errors of the section "Diagnostics". The spec's rules
//! that choose one syntax error for one fault decide the order of the checks:
//!
//! - A specific syntax error comes before a general one. A rule body that
//!   meets the head of the next rule, as in `a = "x" b = "y" ;`, is therefore a
//!   missing `;`.
//! - Only a rule head that starts with a name ends a rule body. A head that
//!   starts with a string needs two faults there: a missing `;` and a string as
//!   a name. A stray `=` is one fault. So `a = "x" = "y" ;` is an unexpected
//!   `=`. After a statement, a head that starts with a string still starts a
//!   rule, as in `"a" = "x" ;`.
//! - The lexer reports slips and the reserved operator before any character
//!   outside the notation, and the parser does not report those tokens again.
//! - Braces that open with a name, a string or `(` are the slip `{x}`. Other
//!   braces after an item are a bound.
//! - After an unexpected token, the parser skips the next unexpected tokens
//!   without a report, up to the next token that it can read.

use rowan::{Checkpoint, GreenNode, GreenNodeBuilder};

use crate::{
    diagnostic::{Code, RawDiagnostic, Span},
    lexer::{Token, lex},
    string,
    syntax::SyntaxKind::{
        self, Bound, BraceSlip, BracketSlip, Choice, Comma, DocDeclaration, DocKeyword, DocString, Eof, EqualsSlip,
        Error, Grammar, Group, LeftBrace, LeftBracket, LeftParen, Name, Number, Pipe, Plus, Postfix, Question,
        Reference, RightBrace, RightBracket, RightParen, Rule, Semicolon, Sequence, Star, StartDeclaration,
        StartKeyword, Symbol, UnknownDirective,
    },
};

/// The result of a parse: the green tree and the syntax errors.
#[derive(Debug, Clone)]
pub struct Parse {
    pub green: GreenNode,
    pub diagnostics: Vec<RawDiagnostic>,
}

pub fn parse(text: &str) -> Parse {
    let lexed = lex(text);
    let significant: Vec<usize> = (0..lexed.tokens.len()).filter(|&i| !lexed.tokens[i].kind.is_trivia()).collect();
    let mut string_run_end = vec![significant.len(); significant.len() + 1];
    for k in (0..significant.len()).rev() {
        string_run_end[k] = if lexed.tokens[significant[k]].kind.is_string() { string_run_end[k + 1] } else { k };
    }
    let mut p = Parser {
        text,
        tokens: lexed.tokens,
        significant,
        string_run_end,
        next_significant: 0,
        pos: 0,
        last_end: 0,
        builder: GreenNodeBuilder::new(),
        diagnostics: lexed.diagnostics,
        eof_reported: lexed.unterminated_at_eof,
    };
    p.builder.start_node(Grammar.into());
    while !p.at(Eof) {
        p.statement();
    }
    p.flush_trivia();
    p.builder.finish_node();
    let mut diagnostics = p.diagnostics;
    diagnostics.sort_by_key(|d| (d.span.start, d.span.end));
    Parse { green: p.builder.finish(), diagnostics }
}

/// What an expression turned out to be, for the messages that quote it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Shape {
    /// No item.
    Empty,
    /// One rule name, symbol or group, without an operator.
    Atom,
    /// Anything else.
    Compound,
}

struct Parser<'a> {
    text: &'a str,
    tokens: Vec<Token>,
    /// The indices in `tokens` of the tokens that are not trivia. With this
    /// list, each lookahead takes a constant time.
    significant: Vec<usize>,
    /// For each index `k` in `significant`, the first index at or after `k`
    /// that is not a string. The last entry is the length of `significant`.
    string_run_end: Vec<usize>,
    /// The index in `significant` of the next token that is not trivia.
    next_significant: usize,
    /// The index of the next token, trivia included.
    pos: usize,
    /// The end of the last token that the parser read, trivia excluded.
    last_end: usize,
    builder: GreenNodeBuilder<'static>,
    diagnostics: Vec<RawDiagnostic>,
    /// Whether the lexer reported a string that reaches the end of the text.
    eof_reported: bool,
}

const STATEMENT: &[&str] = &["a rule name", "`%doc`", "`%start`"];

impl Parser<'_> {
    // The token cursor.

    fn nth_token(&self, n: usize) -> Option<&Token> {
        self.significant.get(self.next_significant + n).map(|&i| &self.tokens[i])
    }

    /// The number of strings in a row from the `n`th token that is not trivia.
    fn string_run(&self, n: usize) -> usize {
        let k = (self.next_significant + n).min(self.significant.len());
        self.string_run_end[k] - k
    }

    fn nth(&self, n: usize) -> SyntaxKind {
        self.nth_token(n).map_or(Eof, |t| t.kind)
    }

    fn current(&self) -> SyntaxKind {
        self.nth(0)
    }

    fn at(&self, kind: SyntaxKind) -> bool {
        self.current() == kind
    }

    fn current_span(&self) -> Span {
        self.nth_token(0).map_or(Span::at(self.text.len()), |t| Span::new(t.start, t.end))
    }

    fn current_text(&self) -> &str {
        self.nth_token(0).map_or("", |t| &self.text[t.start..t.end])
    }

    fn current_faulty(&self) -> bool {
        self.nth_token(0).is_some_and(|t| t.faulty)
    }

    // Tree building. Trivia goes into the tree just before the next token, so
    // a node starts at a token that the parser reads.

    fn flush_trivia(&mut self) {
        while let Some(token) = self.tokens.get(self.pos).filter(|t| t.kind.is_trivia()) {
            self.builder.token(token.kind.into(), &self.text[token.start..token.end]);
            self.pos += 1;
        }
    }

    fn bump(&mut self) {
        self.flush_trivia();
        let token = self.tokens[self.pos];
        self.builder.token(token.kind.into(), &self.text[token.start..token.end]);
        self.pos += 1;
        self.next_significant += 1;
        self.last_end = token.end;
    }

    fn start(&mut self, kind: SyntaxKind) {
        self.flush_trivia();
        self.builder.start_node(kind.into());
    }

    fn checkpoint(&mut self) -> Checkpoint {
        self.flush_trivia();
        self.builder.checkpoint()
    }

    fn start_at(&mut self, checkpoint: Checkpoint, kind: SyntaxKind) {
        self.builder.start_node_at(checkpoint, kind.into());
    }

    fn finish(&mut self) {
        self.builder.finish_node();
    }

    /// Reads the current token into an error node.
    fn bump_error(&mut self) {
        self.start(Error);
        self.bump();
        self.finish();
    }

    // Reports.

    fn error(&mut self, code: Code, span: Span, message: impl Into<String>) {
        self.diagnostics.push(RawDiagnostic::error(code, span, message));
    }

    /// Whether the parser is at the end of the text after the lexer reported a
    /// string that reaches it. The string swallowed the rest of the text, so
    /// the parser reports nothing more there.
    fn at_reported_eof(&self) -> bool {
        self.at(Eof) && self.eof_reported
    }

    /// Reports the current token as unexpected, unless the lexer reported it.
    fn unexpected(&mut self, expected: &[&str]) {
        if self.at_reported_eof() || matches!(self.current(), EqualsSlip | UnknownDirective) {
            return;
        }
        let found =
            if self.at(Eof) { "end of the file".to_string() } else { format!("`{}`", shorten(self.current_text())) };
        let message = format!("unexpected {found}: expected {}", list(expected));
        self.error(Code::UnexpectedToken, self.current_span(), message);
    }

    // Lookahead.

    /// Whether a rule head starts here: a name or a string, any strings, and
    /// then `=` or a slip for `=`.
    fn at_rule_head(&self) -> bool {
        (self.at(Name) || self.current().is_string()) && self.nth(1 + self.string_run(1)).is_equals()
    }

    /// Whether a rule head that starts with a name starts here.
    fn at_named_rule_head(&self) -> bool {
        self.at(Name) && self.at_rule_head()
    }

    fn at_statement_start(&self) -> bool {
        matches!(self.current(), DocKeyword | StartKeyword | UnknownDirective) || self.at_rule_head()
    }

    /// Whether a statement starts here, in a rule body. In a body, a head that
    /// starts with a string is a symbol and an unexpected `=`.
    fn at_statement_start_in_body(&self) -> bool {
        matches!(self.current(), DocKeyword | StartKeyword | UnknownDirective) || self.at_named_rule_head()
    }

    /// Whether the current token ends an expression: `;`, the end of the text,
    /// the start of a statement, or a closing bracket of an enclosing group.
    fn at_stop(&self, closers: &[SyntaxKind]) -> bool {
        self.at(Semicolon) || self.at(Eof) || self.at_statement_start_in_body() || closers.contains(&self.current())
    }

    /// Whether braces here open with a name, a string or `(`: the slip `{x}`.
    fn at_brace_slip(&self) -> bool {
        self.at(LeftBrace) && (matches!(self.nth(1), Name | LeftParen) || self.nth(1).is_string())
    }

    fn at_item_start(&self) -> bool {
        matches!(self.current(), Name | LeftParen | LeftBracket) || self.current().is_string() || self.at_brace_slip()
    }

    fn at_operator(&self) -> bool {
        matches!(self.current(), Question | Star | Plus) || (self.at(LeftBrace) && !self.at_brace_slip())
    }

    // Statements.

    fn statement(&mut self) {
        match self.current() {
            DocKeyword => self.doc_declaration(),
            StartKeyword => self.start_declaration(),
            Name => self.rule(),
            _ if self.at_rule_head() => self.rule(),
            _ => {
                // The lexer reported an unknown directive. Skip its statement.
                self.unexpected(STATEMENT);
                self.start(Error);
                self.bump();
                while !(self.at(Semicolon) || self.at(Eof) || self.at_statement_start()) {
                    self.bump();
                }
                if self.at(Semicolon) {
                    self.bump();
                }
                self.finish();
            }
        }
    }

    fn doc_declaration(&mut self) {
        self.start(DocDeclaration);
        self.bump();
        let ok = self.current().is_string();
        if ok {
            self.bump();
        } else {
            self.unexpected(&["a string"]);
        }
        self.semicolon("the `%doc` declaration", !ok);
        self.finish();
    }

    fn start_declaration(&mut self) {
        self.start(StartDeclaration);
        self.bump();
        let mut ok = true;
        if self.at(Name) {
            self.bump();
        } else if self.current().is_string() {
            let value = self.current_string_value();
            let message = format!("write the start rule as a bare name, without quotes: `%start {value} ;`");
            self.error(Code::StringAsName, self.current_span(), message);
            self.bump();
        } else {
            self.unexpected(&["a rule name"]);
            ok = false;
        }
        self.semicolon("the `%start` declaration", !ok);
        self.finish();
    }

    fn rule(&mut self) {
        self.start(Rule);
        let name = if self.at(Name) {
            let name = self.current_text().to_string();
            self.bump();
            name
        } else {
            let value = self.current_string_value();
            let message = format!("a string cannot name a rule; write the bare name `{value}`");
            self.error(Code::StringAsName, self.current_span(), message);
            self.bump();
            value
        };
        let strings = self.string_run(0);
        let mut quiet = false;
        if strings > 0 && self.nth(strings).is_equals() {
            self.start(DocString);
            self.bump();
            self.finish();
            for _ in 1..strings {
                let message = format!("the rule `{name}` has a second doc string; a rule has one doc string at most");
                self.error(Code::SecondDocString, self.current_span(), message);
                self.bump_error();
            }
            quiet = self.body(&name);
        } else if self.current().is_equals() {
            quiet = self.body(&name);
        } else {
            let message = format!("missing `=` after the rule name `{name}`");
            self.error(Code::MissingEquals, Span::at(self.last_end), message);
            if self.at_item_start() && !self.at_statement_start() {
                self.choice(&[]);
            } else {
                quiet = true;
            }
        }
        self.semicolon(&format!("the rule `{name}`"), quiet);
        self.finish();
    }

    /// Reads `=` and the rule body. Returns true if the body is missing at the
    /// end of the text, so that the `;` needs no report of its own.
    fn body(&mut self, name: &str) -> bool {
        let equals = self.current_span();
        self.bump();
        if self.at(Semicolon) || self.at(Eof) || self.at_statement_start_in_body() {
            let message = format!("the body of the rule `{name}` is empty; a rule body holds at least one item");
            self.error(Code::EmptyRuleBody, equals, message);
            return self.at(Eof);
        }
        self.choice(&[]);
        false
    }

    /// Reads the `;` that ends a statement. `quiet` is true after a syntax
    /// error that already covers a missing `;`.
    fn semicolon(&mut self, what: &str, quiet: bool) {
        if self.at(Semicolon) {
            self.bump();
            return;
        }
        if self.at(Eof) || self.at_statement_start() {
            if !quiet && !self.at_reported_eof() {
                self.error(
                    Code::MissingSemicolon,
                    Span::at(self.last_end),
                    format!("missing `;` at the end of {what}"),
                );
            }
            return;
        }
        if !quiet {
            self.unexpected(&["`;`"]);
        }
        self.start(Error);
        while !(self.at(Semicolon) || self.at(Eof) || self.at_statement_start()) {
            self.bump();
        }
        self.finish();
        if self.at(Semicolon) {
            self.bump();
        }
    }

    // Expressions.

    fn choice(&mut self, closers: &[SyntaxKind]) -> Shape {
        let checkpoint = self.checkpoint();
        let mut shape = Shape::Empty;
        if self.at(Pipe) {
            self.error(Code::EmptyAlternative, self.current_span(), empty_alternative());
        } else {
            shape = self.sequence(closers);
        }
        let mut alternatives = false;
        while self.at(Pipe) {
            alternatives = true;
            let pipe = self.current_span();
            self.bump();
            if self.at(Pipe) || self.at_stop(closers) {
                if !self.at_reported_eof() {
                    self.error(Code::EmptyAlternative, pipe, empty_alternative());
                }
            } else {
                self.sequence(closers);
            }
        }
        if alternatives {
            self.start_at(checkpoint, Choice);
            self.finish();
            shape = Shape::Compound;
        }
        shape
    }

    fn sequence(&mut self, closers: &[SyntaxKind]) -> Shape {
        let checkpoint = self.checkpoint();
        let mut items = 0;
        let mut shape = Shape::Empty;
        let mut operator_allowed = false;
        let mut skipping = false;
        loop {
            if self.at(Pipe) || self.at_stop(closers) {
                break;
            }
            if self.at_item_start() {
                let (item_shape, has_operator) = self.item(closers);
                shape = item_shape;
                operator_allowed = !has_operator;
                items += 1;
                skipping = false;
                continue;
            }
            if self.at(Comma) {
                let message = "separate the items of a sequence with white space, not `,`";
                self.error(Code::SlipComma, self.current_span(), message);
                self.bump_error();
                continue;
            }
            if !skipping {
                let mut expected = vec!["a rule name", "a symbol", "`(`"];
                if operator_allowed {
                    expected.push("an operator");
                }
                expected.push("`|`");
                expected.push(closer_name(closers));
                self.unexpected(&expected);
            }
            skipping = true;
            self.bump_error();
        }
        match items {
            0 => Shape::Empty,
            1 => shape,
            _ => {
                self.start_at(checkpoint, Sequence);
                self.finish();
                Shape::Compound
            }
        }
    }

    /// Reads one item with its operator. Returns its shape, and whether it has
    /// an operator.
    fn item(&mut self, closers: &[SyntaxKind]) -> (Shape, bool) {
        let checkpoint = self.checkpoint();
        let item_start = self.current_span().start as usize;
        let shape = match self.current() {
            Name => {
                self.start(Reference);
                self.bump();
                self.finish();
                Shape::Atom
            }
            LeftParen => {
                self.group(closers);
                Shape::Atom
            }
            LeftBracket => {
                self.iso_slip(BracketSlip, RightBracket, closers);
                Shape::Compound
            }
            LeftBrace => {
                self.iso_slip(BraceSlip, RightBrace, closers);
                Shape::Compound
            }
            _ => {
                self.symbol();
                Shape::Atom
            }
        };
        if !self.at_operator() {
            return (shape, false);
        }
        let item = quote_item(&self.text[item_start..self.last_end]);
        self.start_at(checkpoint, Postfix);
        self.operator(&item, true);
        let mut reported = false;
        while self.at_operator() {
            let start = self.current_span().start as usize;
            self.start(Error);
            self.operator(&item, false);
            self.finish();
            if !reported {
                let message = "an item takes one operator at most; put the item and its first operator in parentheses, as in `(x{2})*`";
                self.error(Code::SecondOperator, Span::new(start, self.last_end), message);
                reported = true;
            }
        }
        self.finish();
        (Shape::Compound, true)
    }

    fn symbol(&mut self) {
        let faulty = self.current_faulty();
        let span = self.current_span();
        let value = self.current_string_value();
        self.start(Symbol);
        self.bump();
        self.finish();
        if faulty {
            return;
        }
        if value.is_empty() {
            self.error(Code::EmptySymbol, span, "a symbol cannot be empty; use `?` for an optional part");
        } else if value.chars().any(string::is_newline) {
            let message =
                "the value of a symbol cannot hold a line break; to break the line in the text, end it with `\\`";
            self.error(Code::LineBreakInSymbol, span, message);
        }
    }

    fn group(&mut self, closers: &[SyntaxKind]) {
        self.start(Group);
        let open = self.current_span();
        self.bump();
        if self.at(RightParen) {
            let span = Span::new(open.start as usize, self.current_span().end as usize);
            self.error(Code::EmptyGroup, span, "the group is empty; a group holds at least one item");
            self.bump();
            self.finish();
            return;
        }
        let mut inner = closers.to_vec();
        inner.push(RightParen);
        self.choice(&inner);
        if self.at(RightParen) {
            self.bump();
        } else if !self.at_reported_eof() {
            self.error(Code::MissingCloseParen, open, "this `(` has no matching `)`");
        }
        self.finish();
    }

    /// Reads `[x]` or `{x}` of ISO 14977, and reports the form of this notation.
    fn iso_slip(&mut self, kind: SyntaxKind, close: SyntaxKind, closers: &[SyntaxKind]) {
        self.start(kind);
        let open = self.current_span();
        self.bump();
        let mut inner = closers.to_vec();
        inner.push(close);
        let shape = if self.at(close) { Shape::Empty } else { self.choice(&inner) };
        let content = self.text[open.end as usize..self.last_end.max(open.end as usize)].trim().to_string();
        if self.at(close) {
            self.bump();
        }
        self.finish();
        let x = match shape {
            Shape::Atom => quote_item(&content),
            _ if content.is_empty() || content.contains('\n') => "(x)".to_string(),
            _ => format!("({content})"),
        };
        let span = Span::new(open.start as usize, self.last_end);
        if kind == BracketSlip {
            self.error(Code::SlipBrackets, span, format!("write `{x}?` for an optional part, not `[…]`"));
        } else {
            self.error(Code::SlipBraces, span, format!("write `{x}*` for a repetition, not `{{…}}`"));
        }
    }

    /// Reads one operator. `check` is false for a second operator, which has
    /// its own report.
    fn operator(&mut self, item: &str, check: bool) {
        if self.at(LeftBrace) {
            self.bound(item, check);
        } else {
            self.bump();
        }
    }

    fn bound(&mut self, item: &str, check: bool) {
        self.start(Bound);
        let open = self.current_span();
        self.bump();
        let mut numbers: Vec<(String, Span)> = Vec::new();
        let mut well_formed = false;
        let mut open_ended = false;
        if self.at(Number) {
            numbers.push((self.current_text().to_string(), self.current_span()));
            self.bump();
            if self.at(Comma) {
                self.bump();
                if self.at(Number) {
                    numbers.push((self.current_text().to_string(), self.current_span()));
                    self.bump();
                } else {
                    open_ended = true;
                }
            }
            well_formed = self.at(RightBrace);
        }
        if well_formed {
            self.bump();
        } else {
            while self.at(Number) || self.at(Comma) {
                self.bump();
            }
            if self.at(RightBrace) {
                self.bump();
            }
        }
        self.finish();
        if !check {
            return;
        }
        let span = Span::new(open.start as usize, self.last_end);
        let written = &self.text[span.range()];
        if !well_formed {
            let message = format!("`{written}` is not a bound; a bound has the form `{{n}}`, `{{m,n}}` or `{{m,}}`");
            self.error(Code::MalformedBound, span, message);
            return;
        }
        let mut leading_zero = false;
        for (text, number_span) in &numbers {
            if text.len() > 1 && text.starts_with('0') {
                let trimmed = text.trim_start_matches('0');
                let trimmed = if trimmed.is_empty() { "0" } else { trimmed };
                let message = format!("a number has no leading zero; write `{trimmed}`");
                self.error(Code::LeadingZero, *number_span, message);
                leading_zero = true;
            }
        }
        if leading_zero {
            return;
        }
        let value = |i: usize| numbers[i].0.parse::<u32>().unwrap_or(u32::MAX);
        let least = value(0);
        let first = |form: &str| format!("`{written}` is a second spelling of `{form}`; write `{item}{form}`");
        let fault = match (numbers.len(), open_ended) {
            (1, false) => match least {
                0 => Some((Code::SecondSpelling, format!("`{written}` matches nothing; remove the item"))),
                1 => Some((Code::SecondSpelling, format!("`{written}` matches the item once; write `{item}` alone"))),
                _ => None,
            },
            (1, true) => match least {
                0 => Some((Code::SecondSpelling, first("*"))),
                1 => Some((Code::SecondSpelling, first("+"))),
                _ => None,
            },
            _ => {
                let greatest = value(1);
                if least > greatest {
                    let message = format!("the least number {least} is above the greatest number {greatest}");
                    Some((Code::LeastAboveGreatest, message))
                } else if least == greatest {
                    match greatest {
                        0 => Some((Code::SecondSpelling, format!("`{written}` matches nothing; remove the item"))),
                        1 => Some((
                            Code::SecondSpelling,
                            format!("`{written}` matches the item once; write `{item}` alone"),
                        )),
                        n => Some((Code::SecondSpelling, first(&format!("{{{n}}}")))),
                    }
                } else if (least, greatest) == (0, 1) {
                    Some((Code::SecondSpelling, first("?")))
                } else {
                    None
                }
            }
        };
        if let Some((code, message)) = fault {
            self.error(code, span, message);
        }
    }

    /// The value of the current string token, for messages.
    fn current_string_value(&self) -> String {
        let text = self.current_text();
        match self.current() {
            SyntaxKind::String => string::scan(text).value,
            _ => text.trim_matches('\'').to_string(),
        }
    }
}

fn empty_alternative() -> &'static str {
    "an alternative is empty; each alternative holds at least one item, and `?` makes an optional part"
}

fn closer_name(closers: &[SyntaxKind]) -> &'static str {
    match closers.last() {
        Some(RightParen) => "`)`",
        Some(RightBracket) => "`]`",
        Some(RightBrace) => "`}`",
        _ => "`;`",
    }
}

/// The text of an item for a message, or `x` if the text is long.
fn quote_item(text: &str) -> String {
    let text = text.trim();
    if text.is_empty() || text.contains('\n') || text.chars().count() > 40 { "x".to_string() } else { text.to_string() }
}

fn shorten(text: &str) -> String {
    let mut chars = text.chars();
    let short: String = chars.by_ref().take(30).collect();
    if chars.next().is_some() { format!("{short}…") } else { short }
}

fn list(items: &[&str]) -> String {
    match items {
        [] => String::new(),
        [one] => (*one).to_string(),
        [rest @ .., last] => format!("{} or {last}", rest.join(", ")),
    }
}
