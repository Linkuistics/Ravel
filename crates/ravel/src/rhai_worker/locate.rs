//! Where a top-level declaration sits in a configuration script source.
//!
//! Rhai 1.27.0 reports the position of an error, but its public API gives no
//! position for a declaration that is valid and merely wrong, such as
//! `export const VERSION = "";` or a `select` with one parameter. The contract
//! check needs those positions, so this scan finds them. It reads the tokens of
//! the source that sit outside every brace, skipping comments and string
//! literals, and is a locator only: it never decides whether the source is
//! valid, which the engine did already.

/// A 1-based line and column, counted in characters as Rhai counts them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Position {
    pub line: u64,
    pub column: u64,
}

struct Word<'a> {
    text: &'a str,
    at: Position,
}

/// The position of `name` in a top-level `const name`, `let name` or `fn name`
/// declaration, after any `export` or `private`. A name declared more than once
/// reports the last, which is the one the module holds.
pub fn declaration(source: &str, keyword: &str, name: &str) -> Option<Position> {
    let words = top_level_words(source);
    words
        .windows(2)
        .filter(|pair| pair[0].text == keyword && pair[1].text == name)
        .map(|pair| pair[1].at)
        .next_back()
}

fn top_level_words(source: &str) -> Vec<Word<'_>> {
    let mut words = Vec::new();
    let mut chars = source.char_indices().peekable();
    let (mut line, mut column) = (1u64, 1u64);
    let mut depth = 0usize;
    // The Rhai position of the next character, advanced as characters pass.
    macro_rules! step {
        ($ch:expr) => {{
            if $ch == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }};
    }
    while let Some((start, ch)) = chars.next() {
        match ch {
            '/' if matches!(chars.peek(), Some((_, '/'))) => {
                step!(ch);
                for (_, next) in chars.by_ref() {
                    step!(next);
                    if next == '\n' {
                        break;
                    }
                }
            }
            '/' if matches!(chars.peek(), Some((_, '*'))) => {
                step!(ch);
                let (_, star) = chars.next().expect("peeked");
                step!(star);
                let mut nesting = 1;
                let mut previous = ' ';
                for (_, next) in chars.by_ref() {
                    step!(next);
                    if previous == '/' && next == '*' {
                        nesting += 1;
                        previous = ' ';
                    } else if previous == '*' && next == '/' {
                        nesting -= 1;
                        previous = ' ';
                        if nesting == 0 {
                            break;
                        }
                    } else {
                        previous = next;
                    }
                }
            }
            '"' | '\'' | '`' => {
                step!(ch);
                let mut escaped = false;
                for (_, next) in chars.by_ref() {
                    step!(next);
                    if escaped {
                        escaped = false;
                    } else if next == '\\' && ch != '`' {
                        escaped = true;
                    } else if next == ch {
                        break;
                    }
                }
            }
            '{' => {
                depth += 1;
                step!(ch);
            }
            '}' => {
                depth = depth.saturating_sub(1);
                step!(ch);
            }
            _ if ch.is_alphabetic() || ch == '_' => {
                let at = Position { line, column };
                let mut end = start + ch.len_utf8();
                step!(ch);
                while let Some(&(index, next)) = chars.peek() {
                    if !(next.is_alphanumeric() || next == '_') {
                        break;
                    }
                    end = index + next.len_utf8();
                    step!(next);
                    chars.next();
                }
                if depth == 0 {
                    words.push(Word {
                        text: &source[start..end],
                        at,
                    });
                }
            }
            _ => step!(ch),
        }
    }
    words
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at(line: u64, column: u64) -> Option<Position> {
        Some(Position { line, column })
    }

    #[test]
    fn a_declaration_is_found_after_export_and_private() {
        let source = "// c\nexport const SCHEMA_VERSION = 3;\n  export const VERSION = \"\";\nprivate fn helper() {}\nfn select(a) {}\n";
        assert_eq!(declaration(source, "const", "SCHEMA_VERSION"), at(2, 14));
        assert_eq!(declaration(source, "const", "VERSION"), at(3, 16));
        assert_eq!(declaration(source, "fn", "select"), at(5, 4));
        assert_eq!(declaration(source, "fn", "helper"), at(4, 12));
        assert_eq!(declaration(source, "fn", "load_context"), None);
    }

    #[test]
    fn comments_and_strings_hide_what_they_hold() {
        let source = "/* const VERSION = 1; /* nested */ fn select() {} */\n\
             let text = \"const VERSION = 2; \\\" fn select\";\n\
             let tick = '\"';\n\
             let raw = `fn select`;\n\
             // fn select\n\
             const VERSION = 3;\n";
        assert_eq!(declaration(source, "const", "VERSION"), at(6, 7));
        assert_eq!(declaration(source, "fn", "select"), None);
    }

    #[test]
    fn a_nested_declaration_is_no_top_level_one() {
        let source = "fn select(a, b) {\n    const VERSION = 1;\n    fn inner() {}\n}\n";
        assert_eq!(declaration(source, "const", "VERSION"), None);
        assert_eq!(declaration(source, "fn", "inner"), None);
        assert_eq!(declaration(source, "fn", "select"), at(1, 4));
    }

    #[test]
    fn columns_count_characters_not_bytes() {
        let source = "let é = \"ü\"; const VERSION = 1;";
        assert_eq!(declaration(source, "const", "VERSION"), at(1, 20));
    }
}
