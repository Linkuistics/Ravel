//! The Rust types of the checked grammar. The KDL form follows from these
//! types through the mapping in `kdl_map`.

pub use crate::diagnostic::{Code, Severity, Span};

/// The version of the KDL form of the checked grammar. A change to the form
/// that an older reader cannot read raises the version.
pub const FORMAT_VERSION: u32 = 2;

/// The checked grammar: the derived artifact that holds the symbols, the rules
/// and the doc strings of a grammar, with its diagnostics and badges.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize, tsify::Tsify), serde(rename_all = "camelCase"))]
pub struct CheckedGrammar {
    pub format_version: u32,
    /// The SHA-256 hash of the grammar text, in lower-case hex.
    pub grammar_sha256: String,
    /// The `%doc` declarations. A valid grammar has one at most.
    pub grammar_docs: Vec<GrammarDoc>,
    /// The `%start` declarations. A valid grammar has exactly one.
    pub start_rules: Vec<StartRule>,
    /// The rules, in the order of the text.
    pub rules: Vec<Rule>,
    /// The symbols, in the order of their first use.
    pub symbols: Vec<Symbol>,
    /// The groups of mutually recursive rules, in the order of their first rule.
    pub recursive_groups: Vec<RecursiveGroup>,
    /// The diagnostics, in the order of their spans.
    pub diagnostics: Vec<Diagnostic>,
}

impl CheckedGrammar {
    /// Whether the grammar has an error, and so has no traces.
    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(|d| d.severity == Severity::Error)
    }
}

/// A `%doc` declaration. The span covers its string.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize, tsify::Tsify), serde(rename_all = "camelCase"))]
pub struct GrammarDoc {
    pub text: String,
    pub span: Span,
}

/// A `%start` declaration. The span covers the rule name.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize, tsify::Tsify), serde(rename_all = "camelCase"))]
pub struct StartRule {
    pub name: String,
    pub span: Span,
}

/// A rule. The span covers the rule name.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize, tsify::Tsify), serde(rename_all = "camelCase"))]
pub struct Rule {
    pub name: String,
    pub span: Span,
    pub doc: Option<Doc>,
    /// The body. After a syntax error the body can be missing.
    pub body: Option<Expr>,
}

/// The doc string of a rule. The span covers the string.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize, tsify::Tsify), serde(rename_all = "camelCase"))]
pub struct Doc {
    pub text: String,
    pub span: Span,
}

/// An expression of a rule body. A group makes no expression of its own, and
/// `+`, `*` and bounds all make a repetition.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize, tsify::Tsify), serde(tag = "type", rename_all = "kebab-case"))]
pub enum Expr {
    /// A symbol. The span covers its string.
    Symbol { value: String, span: Span },
    /// A reference to a rule. The span covers the rule name.
    Reference { name: String, span: Span },
    /// Two or more items in order.
    Sequence { items: Vec<Expr>, span: Span },
    /// Two or more alternatives.
    Choice { alternatives: Vec<Expr>, span: Span },
    /// `x?`.
    Optional { item: Box<Expr>, span: Span },
    /// `x*` is `min: 0`, `x+` is `min: 1`, and `x{2,4}` is `min: 2, max: Some(4)`.
    Repetition { min: u32, max: Option<u32>, item: Box<Expr>, span: Span },
}

impl Expr {
    pub fn span(&self) -> Span {
        match self {
            Self::Symbol { span, .. }
            | Self::Reference { span, .. }
            | Self::Sequence { span, .. }
            | Self::Choice { span, .. }
            | Self::Optional { span, .. }
            | Self::Repetition { span, .. } => *span,
        }
    }
}

/// A symbol and each place that uses it.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize, tsify::Tsify), serde(rename_all = "camelCase"))]
pub struct Symbol {
    pub value: String,
    pub uses: Vec<SymbolUse>,
}

/// One use of a symbol, in the body of one rule.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize, tsify::Tsify), serde(rename_all = "camelCase"))]
pub struct SymbolUse {
    pub rule: String,
    pub span: Span,
}

/// A group of mutually recursive rules, with its badge. A group holds two or
/// more rules that each reach the others, or one rule that refers to itself.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize, tsify::Tsify), serde(rename_all = "camelCase"))]
pub struct RecursiveGroup {
    pub badge: Badge,
    /// The rules of the group, in the order of the text.
    pub members: Vec<GroupMember>,
}

/// One rule of a recursive group.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize, tsify::Tsify), serde(rename_all = "camelCase"))]
pub struct GroupMember {
    pub rule: String,
}

/// The badge of a recursive group, by the test of Nederhof
/// (https://arxiv.org/pdf/cs/9910022). Whether a language is regular cannot be
/// decided, so no badge claims "not regular".
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize, tsify::Tsify), serde(rename_all = "kebab-case"))]
pub enum Badge {
    /// The group has no self-embedding, so it needs no stack.
    NoSelfEmbedding,
    /// The group has self-embedding. Its language can still be regular.
    MayNeedAStack,
}

impl Badge {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NoSelfEmbedding => "no-self-embedding",
            Self::MayNeedAStack => "may-need-a-stack",
        }
    }

    pub fn parse(text: &str) -> Option<Self> {
        match text {
            "no-self-embedding" => Some(Self::NoSelfEmbedding),
            "may-need-a-stack" => Some(Self::MayNeedAStack),
            _ => None,
        }
    }
}

/// A diagnostic, placed in the text by its span and by the line and the column
/// of its start. Lines and columns count from 1. A column counts Unicode
/// scalar values. The line breaks are those of the notation: LF or CR LF
/// outside strings, and each newline of KDL 2.0 inside a string. An editor
/// that splits lines in another way must place a diagnostic by its span.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize, tsify::Tsify), serde(rename_all = "camelCase"))]
pub struct Diagnostic {
    pub severity: Severity,
    pub code: Code,
    pub message: String,
    pub span: Span,
    pub line: u32,
    pub column: u32,
}
