//! Diagnostics: the syntax errors of the parser and the diagnostics of the rule checks.

use std::fmt;

/// A byte range in the grammar text, from `start` up to but not including `end`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize, tsify::Tsify), serde(rename_all = "camelCase"))]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start: to_u32(start), end: to_u32(end) }
    }

    /// A span of no width at one offset, for a missing token.
    pub fn at(offset: usize) -> Self {
        Self::new(offset, offset)
    }

    pub fn range(self) -> std::ops::Range<usize> {
        self.start as usize..self.end as usize
    }
}

fn to_u32(offset: usize) -> u32 {
    u32::try_from(offset).unwrap_or(u32::MAX)
}

impl From<rowan::TextRange> for Span {
    fn from(range: rowan::TextRange) -> Self {
        Self { start: range.start().into(), end: range.end().into() }
    }
}

/// How serious a diagnostic is. A grammar with an error has no traces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize, tsify::Tsify), serde(rename_all = "kebab-case"))]
pub enum Severity {
    Error,
    Warning,
}

impl Severity {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warning => "warning",
        }
    }

    pub fn parse(text: &str) -> Option<Self> {
        match text {
            "error" => Some(Self::Error),
            "warning" => Some(Self::Warning),
            _ => None,
        }
    }
}

macro_rules! codes {
    ($($(#[$doc:meta])* $variant:ident = $text:literal,)*) => {
        /// The stable code of a diagnostic. Tests and other programs match on
        /// the code, never on the message.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[cfg_attr(feature = "wasm", derive(serde::Serialize, tsify::Tsify), serde(rename_all = "kebab-case"))]
        pub enum Code {
            $($(#[$doc])* $variant,)*
        }

        impl Code {
            /// Every code, in the order of the spec tables.
            pub const ALL: &'static [Code] = &[$(Code::$variant,)*];

            pub fn as_str(self) -> &'static str {
                match self {
                    $(Code::$variant => $text,)*
                }
            }

            pub fn parse(text: &str) -> Option<Self> {
                match text {
                    $($text => Some(Code::$variant),)*
                    _ => None,
                }
            }
        }
    };
}

codes! {
    // The syntax errors of `docs/specs/grammar-notation.md`, section "Diagnostics".
    /// A character outside the notation, such as `!`.
    UnknownCharacter = "unknown-character",
    /// `::=`, `:=`, `:` or `->` after a rule name.
    SlipEquals = "slip-equals",
    /// A string in single quotes, such as `'x'`.
    SlipSingleQuote = "slip-single-quote",
    /// A comment that starts with `///` or `//!`.
    SlipDocComment = "slip-doc-comment",
    /// A comment that starts with `#`, `/*` or `(*`.
    SlipComment = "slip-comment",
    /// The optional part `[x]` of ISO 14977.
    SlipBrackets = "slip-brackets",
    /// The repetition `{x}` of ISO 14977.
    SlipBraces = "slip-braces",
    /// A comma between items, as in ISO 14977.
    SlipComma = "slip-comma",
    /// The reserved operator `&`.
    ReservedOperator = "reserved-operator",
    /// `%` and a name that is not a directive.
    UnknownDirective = "unknown-directive",
    /// A string without its closing quote.
    UnterminatedString = "unterminated-string",
    /// A literal line break in a quoted string, outside a whitespace escape.
    LineBreakInString = "line-break-in-string",
    /// An unknown or invalid escape in a string.
    InvalidEscape = "invalid-escape",
    /// A symbol whose value is empty.
    EmptySymbol = "empty-symbol",
    /// A symbol whose value holds a line break.
    LineBreakInSymbol = "line-break-in-symbol",
    /// A string as a rule name or as the start rule.
    StringAsName = "string-as-name",
    /// A second doc string on one rule.
    SecondDocString = "second-doc-string",
    /// A missing `;`.
    MissingSemicolon = "missing-semicolon",
    /// A missing `=`.
    MissingEquals = "missing-equals",
    /// An empty alternative.
    EmptyAlternative = "empty-alternative",
    /// An empty group.
    EmptyGroup = "empty-group",
    /// An empty rule body.
    EmptyRuleBody = "empty-rule-body",
    /// A group without its `)`.
    MissingCloseParen = "missing-close-paren",
    /// A second operator on one item.
    SecondOperator = "second-operator",
    /// A bound that has not the form `{n}`, `{m,n}` or `{m,}`.
    MalformedBound = "malformed-bound",
    /// A second spelling of a bound, such as `{0,1}` for `?`.
    SecondSpelling = "second-spelling",
    /// A least number above the greatest number in a bound.
    LeastAboveGreatest = "least-above-greatest",
    /// A number with a leading zero.
    LeadingZero = "leading-zero",
    // The two general syntax errors.
    /// A token where the syntax allows no such token, or the end of the file
    /// where the syntax needs a token.
    UnexpectedToken = "unexpected-token",
    /// A string that breaks the string syntax of KDL 2.0 in another way.
    MalformedString = "malformed-string",
    // The check diagnostics of the same section, from the rule checks.
    /// More than one `%doc`.
    SecondGrammarDoc = "second-grammar-doc",
    /// No `%start`.
    MissingStart = "missing-start",
    /// More than one `%start`.
    SecondStart = "second-start",
    /// A `%start` that names a name without a rule.
    UndefinedStartRule = "undefined-start-rule",
    /// A bare name without a rule.
    UndefinedRule = "undefined-rule",
    /// A rule that is defined twice.
    DuplicateRule = "duplicate-rule",
    /// A repetition whose body can match the empty sequence.
    NullableRepetition = "nullable-repetition",
    /// A rule that can derive itself with no step.
    CyclicRule = "cyclic-rule",
    /// A bound above the bound cap.
    BoundAboveCap = "bound-above-cap",
    /// A rule that the start rule does not reach.
    UnusedRule = "unused-rule",
    /// Two symbols that differ only by case, spaces, `_` or `-`.
    SimilarSymbols = "similar-symbols",
    /// Two rule names that differ only by case.
    SimilarRuleNames = "similar-rule-names",
}

impl Code {
    /// Whether the rule checks report this code. The parser reports the others.
    pub fn is_check(self) -> bool {
        self >= Code::SecondGrammarDoc
    }
}

impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// One diagnostic, before the core places it in lines and columns.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawDiagnostic {
    pub severity: Severity,
    pub code: Code,
    pub message: String,
    pub span: Span,
}

impl RawDiagnostic {
    pub fn error(code: Code, span: Span, message: impl Into<String>) -> Self {
        Self { severity: Severity::Error, code, message: message.into(), span }
    }

    pub fn warning(code: Code, span: Span, message: impl Into<String>) -> Self {
        Self { severity: Severity::Warning, code, message: message.into(), span }
    }
}
