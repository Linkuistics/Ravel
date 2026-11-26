//! The strings of the notation, which follow the string syntax of KDL 2.0.
//!
//! Source: the KDL 2.0 specification, sections "Quoted String", "Multi-line
//! String", "Raw String", "Whitespace", "Newline" and "Disallowed Literal Code
//! Points" (https://kdl.dev/spec/, the draft `draft-marchan-kdl2.md` in
//! https://github.com/kdl-org/kdl). The notation takes no identifier strings.
//!
//! One function scans a string: it finds the end of the string, decodes the
//! value and lists each fault with its span. The lexer uses the end and the
//! faults, and the lowering uses the value.

use crate::diagnostic::Code;

/// One fault in a string, with a span relative to the start of the string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringFault {
    pub code: Code,
    pub start: usize,
    pub end: usize,
    pub message: String,
}

/// The result of scanning one string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScannedString {
    /// The length of the string in bytes, with its delimiters.
    pub len: usize,
    /// The value of the string. After a fault, the value is the best guess.
    pub value: String,
    pub faults: Vec<StringFault>,
    /// Whether the string reaches the end of the text without its closing quote.
    pub unterminated: bool,
}

/// Whether a character is white space in KDL 2.0, other than a line break.
pub fn is_unicode_space(c: char) -> bool {
    matches!(
        c,
        '\u{0009}' | '\u{0020}' | '\u{00A0}' | '\u{1680}' | '\u{2000}'
            ..='\u{200A}' | '\u{202F}' | '\u{205F}' | '\u{3000}'
    )
}

/// Whether a character starts a line break in KDL 2.0. CR LF is one line break.
pub fn is_newline(c: char) -> bool {
    matches!(c, '\u{000A}' | '\u{000B}' | '\u{000C}' | '\u{000D}' | '\u{0085}' | '\u{2028}' | '\u{2029}')
}

/// Whether KDL 2.0 forbids a code point as a literal character.
pub fn is_disallowed(c: char) -> bool {
    matches!(
        c,
        '\u{0000}'..='\u{0008}'
            | '\u{000E}'..='\u{001F}'
            | '\u{007F}'
            | '\u{200E}'..='\u{200F}'
            | '\u{202A}'..='\u{202E}'
            | '\u{2066}'..='\u{2069}'
            | '\u{FEFF}'
    )
}

/// The length in bytes of the line break at the start of `text`, or 0.
fn newline_len(text: &str) -> usize {
    if text.starts_with("\r\n") { 2 } else { text.chars().next().filter(|&c| is_newline(c)).map_or(0, char::len_utf8) }
}

/// Scans the string at the start of `text`. The text starts with `"`, or with
/// one or more `#` and then `"`.
pub fn scan(text: &str) -> ScannedString {
    let hashes = text.bytes().take_while(|&b| b == b'#').count();
    debug_assert_eq!(text.as_bytes().get(hashes), Some(&b'"'));
    if text[hashes..].starts_with("\"\"\"") { scan_multi_line(text, hashes) } else { scan_single_line(text, hashes) }
}

struct Scanner<'a> {
    text: &'a str,
    raw: bool,
    faults: Vec<StringFault>,
}

impl Scanner<'_> {
    fn fault(&mut self, code: Code, start: usize, end: usize, message: impl Into<String>) {
        self.faults.push(StringFault { code, start, end, message: message.into() });
    }

    /// Reads the escape at `pos`, which holds `\`. Returns the end of the
    /// escape and the text it stands for. A whitespace escape stands for
    /// nothing.
    fn escape(&mut self, pos: usize) -> (usize, Option<char>) {
        let rest = &self.text[pos + 1..];
        let Some(next) = rest.chars().next() else {
            self.fault(Code::InvalidEscape, pos, pos + 1, "a `\\` at the end of the text escapes nothing");
            return (pos + 1, None);
        };
        let simple = match next {
            'n' => Some('\n'),
            'r' => Some('\r'),
            't' => Some('\t'),
            '\\' => Some('\\'),
            '"' => Some('"'),
            'b' => Some('\u{0008}'),
            'f' => Some('\u{000C}'),
            's' => Some(' '),
            _ => None,
        };
        if let Some(value) = simple {
            return (pos + 2, Some(value));
        }
        if next == 'u' {
            return self.unicode_escape(pos);
        }
        if is_unicode_space(next) || is_newline(next) {
            let mut end = pos + 1;
            loop {
                let rest = &self.text[end..];
                let n = newline_len(rest);
                if n > 0 {
                    end += n;
                    continue;
                }
                match rest.chars().next() {
                    Some(c) if is_unicode_space(c) => end += c.len_utf8(),
                    _ => break,
                }
            }
            return (end, None);
        }
        let end = pos + 1 + next.len_utf8();
        self.fault(
            Code::InvalidEscape,
            pos,
            end,
            format!(
                "`\\{next}` is not an escape; the escapes are `\\n`, `\\r`, `\\t`, `\\\\`, `\\\"`, `\\b`, `\\f`, `\\s` and `\\u{{…}}`"
            ),
        );
        (end, None)
    }

    /// Reads `\u{…}` at `pos`.
    fn unicode_escape(&mut self, pos: usize) -> (usize, Option<char>) {
        let rest = &self.text[pos + 2..];
        if !rest.starts_with('{') {
            self.fault(
                Code::InvalidEscape,
                pos,
                pos + 2,
                "a Unicode escape has the form `\\u{…}`, with 1 to 6 hex digits",
            );
            return (pos + 2, None);
        }
        let digits = rest[1..].bytes().take_while(u8::is_ascii_hexdigit).count();
        let close = pos + 3 + digits;
        if self.text.as_bytes().get(close) != Some(&b'}') || digits == 0 || digits > 6 {
            let end = if self.text.as_bytes().get(close) == Some(&b'}') { close + 1 } else { close };
            self.fault(Code::InvalidEscape, pos, end, "a Unicode escape has the form `\\u{…}`, with 1 to 6 hex digits");
            return (end, None);
        }
        let hex = &self.text[pos + 3..close];
        match u32::from_str_radix(hex, 16).ok().and_then(char::from_u32) {
            Some(c) => (close + 1, Some(c)),
            None => {
                self.fault(
                    Code::InvalidEscape,
                    pos,
                    close + 1,
                    format!("`\\u{{{hex}}}` is not a Unicode scalar value"),
                );
                (close + 1, None)
            }
        }
    }

    fn disallowed(&mut self, pos: usize, c: char) {
        self.fault(
            Code::MalformedString,
            pos,
            pos + c.len_utf8(),
            format!(
                "U+{:04X} cannot stand in a string as a literal character; write `\\u{{{:X}}}` in a string that is not raw",
                c as u32, c as u32
            ),
        );
    }
}

/// Whether the closing delimiter `"` and `hashes` times `#` starts at `pos`.
fn closes(text: &str, pos: usize, hashes: usize) -> bool {
    let bytes = text.as_bytes();
    bytes.get(pos) == Some(&b'"') && (1..=hashes).all(|i| bytes.get(pos + i) == Some(&b'#'))
}

fn scan_single_line(text: &str, hashes: usize) -> ScannedString {
    scan_quoted(text, hashes, hashes + 1)
}

/// The length of the rest of a quoted string, with its closing delimiter. The
/// opening delimiter is before `text`, and `hashes` is its number of `#`. The
/// scan uses the escapes of a quoted string, and a raw string has no escapes.
/// The result is `None` if a line break or the end of `text` comes first.
pub fn quoted_rest_len(text: &str, hashes: usize) -> Option<usize> {
    let scanned = scan_quoted(text, hashes, 0);
    let closed = !scanned.unterminated && !scanned.faults.iter().any(|f| f.code == Code::LineBreakInString);
    closed.then_some(scanned.len)
}

/// Scans a quoted string from `open`, the end of its opening delimiter.
fn scan_quoted(text: &str, hashes: usize, open: usize) -> ScannedString {
    let mut s = Scanner { text, raw: hashes > 0, faults: Vec::new() };
    let mut pos = open;
    let mut value = String::new();
    loop {
        let rest = &text[pos..];
        let Some(c) = rest.chars().next() else {
            s.fault(Code::UnterminatedString, 0, open, "this string has no closing quote before the end of the file");
            return ScannedString { len: pos, value, faults: s.faults, unterminated: true };
        };
        if closes(text, pos, hashes) {
            return ScannedString { len: pos + 1 + hashes, value, faults: s.faults, unterminated: false };
        }
        let n = newline_len(rest);
        if n > 0 {
            s.fault(
                Code::LineBreakInString,
                pos,
                pos + n,
                "a quoted string cannot hold a line break; write `\\n`, or end the line with `\\` to continue the string",
            );
            return ScannedString { len: pos, value, faults: s.faults, unterminated: false };
        }
        if c == '\\' && !s.raw {
            let (end, decoded) = s.escape(pos);
            value.extend(decoded);
            pos = end;
            continue;
        }
        if is_disallowed(c) {
            s.disallowed(pos, c);
        }
        value.push(c);
        pos += c.len_utf8();
    }
}

fn scan_multi_line(text: &str, hashes: usize) -> ScannedString {
    let mut s = Scanner { text, raw: hashes > 0, faults: Vec::new() };
    let open = hashes + 3;
    let after_open = &text[open..];
    if after_open.is_empty() {
        s.fault(Code::UnterminatedString, 0, open, "this string has no closing `\"\"\"` before the end of the file");
        return ScannedString { len: open, value: String::new(), faults: s.faults, unterminated: true };
    }
    let first_newline = newline_len(after_open);
    if first_newline == 0 {
        // KDL 2.0 forbids `"""` for a one-line string. The string ends at its
        // closing delimiter on this line, or else at the end of the line.
        let mut pos = open;
        let end = loop {
            let rest = &text[pos..];
            let Some(c) = rest.chars().next() else { break pos };
            if newline_len(rest) > 0 {
                break pos;
            }
            if text[pos..].starts_with("\"\"\"") && closes(text, pos + 2, hashes) {
                break pos + 3 + hashes;
            }
            pos += c.len_utf8();
        };
        s.fault(
            Code::MalformedString,
            0,
            end,
            "a multi-line string starts with a line break after its opening `\"\"\"`; write a one-line string as `\"…\"`",
        );
        let value = text[open..end].trim_end_matches('#').trim_end_matches('"').to_string();
        return ScannedString { len: end, value, faults: s.faults, unterminated: false };
    }

    // Find the closing delimiter, and check the escapes and the characters on the way.
    let body_start = open + first_newline;
    let mut pos = body_start;
    let body_end = loop {
        let rest = &text[pos..];
        let Some(c) = rest.chars().next() else {
            s.fault(
                Code::UnterminatedString,
                0,
                open,
                "this string has no closing `\"\"\"` before the end of the file",
            );
            let len = text.len();
            let value = dedent(&resolve_whitespace_escapes(&text[body_start..], s.raw), "").0;
            let value = if s.raw { value } else { decode_escapes(&value) };
            return ScannedString { len, value, faults: s.faults, unterminated: true };
        };
        if rest.starts_with("\"\"\"") && closes(text, pos + 2, hashes) {
            break pos;
        }
        if c == '\\' && !s.raw {
            let (end, _) = s.escape(pos);
            pos = end;
            continue;
        }
        if is_disallowed(c) {
            s.disallowed(pos, c);
        }
        pos += c.len_utf8();
    };
    let len = body_end + 3 + hashes;

    // KDL 2.0: resolve the whitespace escapes, then dedent, then resolve the
    // other escapes.
    let body = resolve_whitespace_escapes(&text[body_start..body_end], s.raw);
    let (lines, last) = split_last_line(&body);
    let value = match last.chars().all(is_unicode_space) {
        true => {
            let (value, ok) = dedent(&lines, &last);
            if !ok {
                s.fault(
                    Code::MalformedString,
                    0,
                    len,
                    "each line of a multi-line string starts with the white space before its closing `\"\"\"`",
                );
            }
            value
        }
        false => {
            s.fault(
                Code::MalformedString,
                0,
                len,
                "the closing `\"\"\"` of a multi-line string stands on its own line, after white space only",
            );
            body.clone()
        }
    };
    let value = if s.raw { value } else { decode_escapes(&value) };
    ScannedString { len, value, faults: s.faults, unterminated: false }
}

/// Removes each whitespace escape. Other escapes stay as they are.
fn resolve_whitespace_escapes(body: &str, raw: bool) -> String {
    if raw {
        return body.to_string();
    }
    let mut out = String::with_capacity(body.len());
    let mut chars = body.char_indices().peekable();
    while let Some((i, c)) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.peek() {
            Some(&(_, next)) if is_unicode_space(next) || is_newline(next) => {
                while let Some(&(_, next)) = chars.peek() {
                    if is_unicode_space(next) || is_newline(next) {
                        chars.next();
                    } else {
                        break;
                    }
                }
            }
            Some(&(_, next)) => {
                out.push('\\');
                out.push(next);
                chars.next();
            }
            None => out.push_str(&body[i..]),
        }
    }
    out
}

/// Splits a body into its lines before the last line, and its last line.
/// Each line break becomes one LF, as KDL 2.0 normalises line breaks.
fn split_last_line(body: &str) -> (String, String) {
    let mut lines = Vec::new();
    let mut current = String::new();
    let mut rest = body;
    while let Some(c) = rest.chars().next() {
        let n = newline_len(rest);
        if n > 0 {
            lines.push(std::mem::take(&mut current));
            rest = &rest[n..];
        } else {
            current.push(c);
            rest = &rest[c.len_utf8()..];
        }
    }
    (lines.join("\n"), current)
}

/// Removes `prefix` from each line. A line of white space only becomes empty.
/// Returns false if a line with text does not start with the prefix.
fn dedent(lines: &str, prefix: &str) -> (String, bool) {
    // `split_last_line` gives no lines before the last line as an empty string.
    // A body with one line break gives one empty line, which `split` also gives.
    let mut ok = true;
    let mut out = Vec::new();
    if lines.is_empty() && !prefix.is_empty() {
        return (String::new(), true);
    }
    for line in lines.split('\n') {
        if line.chars().all(is_unicode_space) {
            out.push("");
        } else if let Some(stripped) = line.strip_prefix(prefix) {
            out.push(stripped);
        } else {
            ok = false;
            out.push(line);
        }
    }
    (out.join("\n"), ok)
}

/// Resolves the escapes that remain after the whitespace escapes. The scan has
/// already reported each invalid escape, so this function skips them.
fn decode_escapes(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('n') => out.push('\n'),
            Some('r') => out.push('\r'),
            Some('t') => out.push('\t'),
            Some('\\') => out.push('\\'),
            Some('"') => out.push('"'),
            Some('b') => out.push('\u{0008}'),
            Some('f') => out.push('\u{000C}'),
            Some('s') => out.push(' '),
            Some('u') if chars.peek() == Some(&'{') => {
                chars.next();
                let hex: String = chars.by_ref().take_while(|&c| c != '}').collect();
                out.extend(u32::from_str_radix(&hex, 16).ok().and_then(char::from_u32));
            }
            _ => {}
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn value(text: &str) -> String {
        let scanned = scan(text);
        assert!(scanned.faults.is_empty(), "{text:?}: {:?}", scanned.faults);
        assert_eq!(scanned.len, text.len(), "{text:?}");
        scanned.value
    }

    fn codes(text: &str) -> Vec<Code> {
        scan(text).faults.iter().map(|f| f.code).collect()
    }

    // The examples below come from the KDL 2.0 specification.

    #[test]
    fn quoted_strings_decode_their_escapes() {
        assert_eq!(value(r#""Hello \    World""#), "Hello World");
        assert_eq!(value(r#""Hello\       \nWorld""#), "Hello\nWorld");
        assert_eq!(value("\"Hello\\n\\\n    World\""), "Hello\nWorld");
        assert_eq!(value(r#""\u{1F600}\s\b\f""#), "\u{1F600} \u{8}\u{c}");
    }

    #[test]
    fn raw_strings_keep_backslashes() {
        assert_eq!(value(r##"#"\n will be literal"#"##), r"\n will be literal");
        assert_eq!(value(r###"##"hello\n\r\asd"#world"##"###), r##"hello\n\r\asd"#world"##);
        assert_eq!(value(r##"#""#"##), "");
    }

    #[test]
    fn multi_line_strings_dedent_by_the_closing_line() {
        let text = "\"\"\"\n        foo\n    This is the base indentation\n            bar\n    \"\"\"";
        assert_eq!(value(text), "    foo\nThis is the base indentation\n        bar");
        let text = "\"\"\"\n    Indented a bit\n\n    A second indented paragraph.\n    \"\"\"";
        assert_eq!(value(text), "Indented a bit\n\nA second indented paragraph.");
        assert_eq!(value("\"\"\"\n\"\"\""), "");
        assert_eq!(value("\"\"\"\r\n  a\r\n  \"\"\""), "a");
    }

    #[test]
    fn multi_line_strings_resolve_whitespace_escapes_before_dedent() {
        let text = "\"\"\"\n  foo \\\nbar\n  baz\n  \\   \"\"\"";
        assert_eq!(value(text), "foo bar\nbaz");
        let text = "\"\"\"\n  foo\n  bar\\\n  \"\"\"";
        assert_eq!(codes(text), vec![Code::MalformedString]);
    }

    #[test]
    fn raw_multi_line_strings_hold_quotes() {
        let text =
            "#\"\"\"\n    Here's a \"\"\"\n        multiline string\n        \"\"\"\n    without escapes.\n    \"\"\"#";
        assert_eq!(value(text), "Here's a \"\"\"\n    multiline string\n    \"\"\"\nwithout escapes.");
    }

    #[test]
    fn faults_of_multi_line_strings() {
        assert_eq!(codes("\"\"\"can't be single line\"\"\""), vec![Code::MalformedString]);
        assert_eq!(codes("\"\"\"\n  closing quote with non-whitespace prefix\"\"\""), vec![Code::MalformedString]);
        assert_eq!(codes("\"\"\"stuff\n  \"\"\""), vec![Code::MalformedString]);
        assert_eq!(codes("\"\"\"\n\ta\n  b\n \t\n\t\"\"\""), vec![Code::MalformedString]);
        assert_eq!(codes("\"\"\""), vec![Code::UnterminatedString]);
        assert_eq!(codes("\"\"\"\nabc"), vec![Code::UnterminatedString]);
    }

    #[test]
    fn faults_of_quoted_strings() {
        assert_eq!(codes(r#""x\q""#), vec![Code::InvalidEscape]);
        assert_eq!(codes(r#""\u{D800}""#), vec![Code::InvalidEscape]);
        assert_eq!(codes(r#""\u{110000}""#), vec![Code::InvalidEscape]);
        assert_eq!(codes(r#""\u{1234567}""#), vec![Code::InvalidEscape]);
        assert_eq!(codes("\"a\u{7}\""), vec![Code::MalformedString]);
        assert_eq!(codes("\"abc"), vec![Code::UnterminatedString]);
        let scanned = scan("\"x\ny\"");
        assert_eq!(scanned.len, 2);
        assert_eq!(scanned.faults[0].code, Code::LineBreakInString);
        assert_eq!((scanned.faults[0].start, scanned.faults[0].end), (2, 3));
    }

    #[test]
    fn a_raw_string_closes_only_with_its_own_hashes() {
        let scanned = scan(r###"##"a"#b"## rest"###);
        assert_eq!(scanned.value, r##"a"#b"##);
        assert_eq!(scanned.len, r###"##"a"#b"##"###.len());
    }
}
