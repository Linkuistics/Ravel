//! The lexer of the notation: `docs/specs/grammar-notation.md`, section "Text and tokens".
//!
//! The lexer covers every byte of the text with tokens, so the syntax tree is
//! lossless. It reports the faults that it can see in one token, such as a
//! slip from another notation or an invalid escape.

use crate::{
    diagnostic::{Code, RawDiagnostic, Span},
    string,
    syntax::SyntaxKind,
};

/// One token: its kind and its byte range in the text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token {
    pub kind: SyntaxKind,
    pub start: usize,
    pub end: usize,
    /// Whether the lexer reported a fault in this token.
    pub faulty: bool,
}

/// The tokens of a text, with the faults that the lexer found.
#[derive(Debug, Default)]
pub struct Lexed {
    pub tokens: Vec<Token>,
    pub diagnostics: Vec<RawDiagnostic>,
    /// Whether a string reaches the end of the text without its closing quote.
    /// The parser then reports nothing more at the end of the text.
    pub unterminated_at_eof: bool,
}

pub fn lex(text: &str) -> Lexed {
    let mut lexer = Lexer { text, pos: 0, out: Lexed::default(), reported: 0 };
    while lexer.pos < text.len() {
        lexer.token();
    }
    lexer.out
}

struct Lexer<'a> {
    text: &'a str,
    pos: usize,
    out: Lexed,
    /// The number of diagnostics before the current token.
    reported: usize,
}

impl<'a> Lexer<'a> {
    fn rest(&self) -> &'a str {
        &self.text[self.pos..]
    }

    fn push(&mut self, kind: SyntaxKind, len: usize) {
        let start = self.pos;
        self.pos += len;
        let faulty = self.out.diagnostics.len() > self.reported;
        self.reported = self.out.diagnostics.len();
        self.out.tokens.push(Token { kind, start, end: self.pos, faulty });
    }

    fn report(&mut self, code: Code, start: usize, end: usize, message: impl Into<String>) {
        self.out.diagnostics.push(RawDiagnostic::error(code, Span::new(start, end), message));
    }

    /// The length of the text up to the next line break outside strings.
    fn line_len(&self) -> usize {
        self.rest()
            .find('\n')
            .map_or(self.rest().len(), |i| if i > 0 && self.rest().as_bytes()[i - 1] == b'\r' { i - 1 } else { i })
    }

    fn token(&mut self) {
        let rest = self.rest();
        let c = rest.chars().next().expect("the lexer stops at the end of the text");
        let start = self.pos;
        match c {
            ' ' | '\t' | '\n' => self.push(SyntaxKind::Whitespace, whitespace_len(rest)),
            '\r' if rest.starts_with("\r\n") => self.push(SyntaxKind::Whitespace, whitespace_len(rest)),
            '/' if rest.starts_with("//") => self.comment(),
            '/' if rest.starts_with("/*") => {
                let len = rest[2..].find("*/").map_or(rest.len(), |i| i + 4);
                self.report(
                    Code::SlipComment,
                    start,
                    start + 2,
                    "the notation has no block comments; start each comment line with `//`",
                );
                self.push(SyntaxKind::CommentSlip, len);
            }
            '(' if rest.starts_with("(*") => {
                let len = rest[2..].find("*)").map_or(rest.len(), |i| i + 4);
                self.report(
                    Code::SlipComment,
                    start,
                    start + 2,
                    "the notation has no `(* … *)` comments; start each comment line with `//`",
                );
                self.push(SyntaxKind::CommentSlip, len);
            }
            '#' => {
                let hashes = rest.bytes().take_while(|&b| b == b'#').count();
                if rest.as_bytes().get(hashes) == Some(&b'"') {
                    self.string();
                } else {
                    let len = self.line_len();
                    self.report(
                        Code::SlipComment,
                        start,
                        start + hashes,
                        "`#` does not start a comment; start a comment with `//`",
                    );
                    self.push(SyntaxKind::CommentSlip, len);
                }
            }
            '"' => self.string(),
            '\'' => self.single_quote(),
            'a'..='z' | 'A'..='Z' => {
                let len = rest.bytes().take_while(|b| b.is_ascii_alphanumeric() || *b == b'_').count();
                self.push(SyntaxKind::Name, len);
            }
            '0'..='9' => {
                let len = rest.bytes().take_while(u8::is_ascii_digit).count();
                self.push(SyntaxKind::Number, len);
            }
            '%' if rest[1..].starts_with(|c: char| c.is_ascii_alphabetic()) => {
                let len = 1 + rest[1..].bytes().take_while(|b| b.is_ascii_alphanumeric() || *b == b'_').count();
                let kind = match &rest[..len] {
                    "%doc" => SyntaxKind::DocKeyword,
                    "%start" => SyntaxKind::StartKeyword,
                    other => {
                        let message = format!("`{other}` is not a directive; the directives are `%doc` and `%start`");
                        self.report(Code::UnknownDirective, start, start + len, message);
                        SyntaxKind::UnknownDirective
                    }
                };
                self.push(kind, len);
            }
            ':' => {
                let len = if rest.starts_with("::=") {
                    3
                } else if rest.starts_with(":=") {
                    2
                } else {
                    1
                };
                self.equals_slip(len);
            }
            '-' if rest.starts_with("->") => self.equals_slip(2),
            '&' => {
                self.report(
                    Code::ReservedOperator,
                    start,
                    start + 1,
                    "`&` is reserved for a later operator for steps in any order",
                );
                self.push(SyntaxKind::Ampersand, 1);
            }
            _ => {
                if let Some(kind) = punctuation(c) {
                    self.push(kind, 1);
                } else {
                    self.unknown();
                }
            }
        }
    }

    fn equals_slip(&mut self, len: usize) {
        let start = self.pos;
        let message = format!("write `=` after a rule name, not `{}`", &self.rest()[..len]);
        self.report(Code::SlipEquals, start, start + len, message);
        self.push(SyntaxKind::EqualsSlip, len);
    }

    fn comment(&mut self) {
        let len = self.line_len();
        let rest = self.rest();
        let slip = (rest.starts_with("///") && !rest.starts_with("////")) || rest.starts_with("//!");
        if slip {
            let start = self.pos;
            self.report(
                Code::SlipDocComment,
                start,
                start + 3,
                "the notation documents with strings: write a doc string after the rule name, or `%doc` for the grammar",
            );
            self.push(SyntaxKind::DocCommentSlip, len);
        } else {
            self.push(SyntaxKind::Comment, len);
        }
    }

    fn string(&mut self) {
        let start = self.pos;
        let hashes = self.rest().bytes().take_while(|&b| b == b'#').count();
        let scanned = string::scan(self.rest());
        let broken = scanned.faults.iter().any(|f| f.code == Code::LineBreakInString);
        for fault in scanned.faults {
            self.report(fault.code, start + fault.start, start + fault.end, fault.message);
        }
        self.out.unterminated_at_eof |= scanned.unterminated;
        self.push(SyntaxKind::String, scanned.len);
        if broken {
            self.broken_string_rest(hashes);
        }
    }

    /// Recovers after a quoted string that ends at a literal line break.
    /// `hashes` is the number of `#` of the broken string.
    ///
    /// The author probably meant the string to go on to its close on the next
    /// line, as in `"x` and `y"`. The recovery reads the next line as the rest
    /// of the string, with the escapes of the string scanner, up to the first
    /// closing delimiter. It takes that close if each string after it ends on
    /// the line, and some string on the whole line does not. The text up to
    /// the close then becomes a trivia token, so the rest of the string makes
    /// no second syntax error. Otherwise only the line break itself becomes
    /// trivia, unless it is a line break of the notation.
    fn broken_string_rest(&mut self, hashes: usize) {
        let rest = self.rest();
        let first = rest.chars().next().map_or(0, char::len_utf8);
        let break_len = if rest.starts_with("\r\n") { 2 } else { first };
        let line = &rest[break_len..];
        let line = &line[..line.find('\n').unwrap_or(line.len())];
        let close =
            string::quoted_rest_len(line, hashes).filter(|&len| !whole_strings(line) && whole_strings(&line[len..]));
        if let Some(len) = close {
            self.push(SyntaxKind::BrokenStringRest, break_len + len);
        } else if !(rest.starts_with('\n') || rest.starts_with("\r\n")) {
            self.push(SyntaxKind::BrokenStringRest, break_len);
        }
    }

    fn single_quote(&mut self) {
        let start = self.pos;
        let line = self.line_len();
        let len = self.rest()[1..line].find('\'').map_or(line, |i| i + 2);
        let content = self.rest()[1..len].trim_end_matches('\'');
        let message = format!("write a symbol in double quotes: `\"{content}\"`");
        self.report(Code::SlipSingleQuote, start, start + len, message);
        self.push(SyntaxKind::SingleQuoteSlip, len);
    }

    /// A run of characters outside the notation.
    fn unknown(&mut self) {
        let start = self.pos;
        let len = self
            .rest()
            .char_indices()
            .find(|&(i, c)| i > 0 && starts_token(c, &self.rest()[i..]))
            .map_or(self.rest().len(), |(i, _)| i);
        let text = &self.rest()[..len];
        let shown: String = text.chars().map(|c| format!("U+{:04X}", c as u32)).collect::<Vec<_>>().join(" ");
        let message = if text.chars().all(|c| c.is_ascii_graphic()) {
            format!("`{text}` is outside the notation")
        } else if text.chars().count() == 1 {
            format!("the character {shown} is outside the notation")
        } else {
            format!("the characters {shown} are outside the notation")
        };
        self.report(Code::UnknownCharacter, start, start + len, message);
        self.push(SyntaxKind::UnknownCharacter, len);
    }
}

/// Whether each string that starts on `line` also ends on it. The strings
/// follow the string scanner. A comment, or a `#` that does not start a raw
/// string, runs to the end of the line, so its quotes are not strings.
fn whole_strings(line: &str) -> bool {
    let mut pos = 0;
    while let Some(c) = line[pos..].chars().next() {
        let rest = &line[pos..];
        if rest.starts_with("//") {
            return true;
        }
        if c == '"' || c == '#' {
            let hashes = rest.bytes().take_while(|&b| b == b'#').count();
            if rest.as_bytes().get(hashes) != Some(&b'"') {
                return true;
            }
            let scanned = string::scan(rest);
            if scanned.unterminated || scanned.faults.iter().any(|f| f.code == Code::LineBreakInString) {
                return false;
            }
            pos += scanned.len;
            continue;
        }
        pos += c.len_utf8();
    }
    true
}

/// The length of the run of white space at the start of `rest`: spaces,
/// tabs, LF and CR LF.
fn whitespace_len(rest: &str) -> usize {
    let bytes = rest.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b' ' | b'\t' | b'\n' => i += 1,
            b'\r' if bytes.get(i + 1) == Some(&b'\n') => i += 2,
            _ => break,
        }
    }
    i
}

fn punctuation(c: char) -> Option<SyntaxKind> {
    Some(match c {
        '=' => SyntaxKind::Equals,
        ';' => SyntaxKind::Semicolon,
        '|' => SyntaxKind::Pipe,
        '(' => SyntaxKind::LeftParen,
        ')' => SyntaxKind::RightParen,
        '?' => SyntaxKind::Question,
        '*' => SyntaxKind::Star,
        '+' => SyntaxKind::Plus,
        '{' => SyntaxKind::LeftBrace,
        '}' => SyntaxKind::RightBrace,
        ',' => SyntaxKind::Comma,
        '[' => SyntaxKind::LeftBracket,
        ']' => SyntaxKind::RightBracket,
        _ => return None,
    })
}

/// Whether a known token starts at `rest`, whose first character is `c`.
fn starts_token(c: char, rest: &str) -> bool {
    matches!(c, ' ' | '\t' | '\n' | '/' | '#' | '"' | '\'' | '&' | ':' | '%')
        || (c == '\r' && rest.starts_with("\r\n"))
        || (c == '-' && rest.starts_with("->"))
        || c.is_ascii_alphanumeric()
        || punctuation(c).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kinds(text: &str) -> Vec<SyntaxKind> {
        lex(text).tokens.iter().map(|t| t.kind).collect()
    }

    #[test]
    fn tokens_cover_every_byte() {
        let text = "%doc \"d\" ;\r\n// c\nrule = ##\"x\"## ! \u{a0} ;";
        let lexed = lex(text);
        let mut end = 0;
        for token in &lexed.tokens {
            assert_eq!(token.start, end);
            end = token.end;
        }
        assert_eq!(end, text.len());
    }

    #[test]
    fn slips_have_their_own_tokens() {
        use SyntaxKind::*;
        assert_eq!(kinds("a ::= 'x'"), vec![Name, Whitespace, EqualsSlip, Whitespace, SingleQuoteSlip]);
        assert_eq!(kinds("a -> b"), vec![Name, Whitespace, EqualsSlip, Whitespace, Name]);
        assert_eq!(kinds("/// d\n//// c"), vec![DocCommentSlip, Whitespace, Comment]);
        assert_eq!(kinds("# c\n(* c *)/* c */"), vec![CommentSlip, Whitespace, CommentSlip, CommentSlip]);
    }

    #[test]
    fn a_lone_carriage_return_is_outside_the_notation() {
        assert_eq!(kinds("a\rb"), vec![SyntaxKind::Name, SyntaxKind::UnknownCharacter, SyntaxKind::Name]);
    }
}
