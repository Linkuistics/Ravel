//! The kinds of the tokens and nodes in the lossless syntax tree.
//!
//! The tree uses `rowan` (https://docs.rs/rowan/0.17.0/rowan/). Only this
//! module and the parser name `rowan` types, so the tree crate stays
//! replaceable, as `docs/adr/hand-written-parser.md` requires.

/// The kind of one token or one node of the syntax tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum SyntaxKind {
    // Trivia: the parser skips these tokens, but the tree keeps them.
    /// Spaces, tabs and line breaks.
    Whitespace,
    /// A plain comment that starts with `//`.
    Comment,
    /// A comment that starts with `///` or `//!`. The lexer reports it.
    DocCommentSlip,
    /// A comment from another notation: `#`, `/*` or `(*`. The lexer reports it.
    CommentSlip,
    /// The reserved operator `&`. The lexer reports it.
    Ampersand,
    /// Characters outside the notation. The lexer reports them.
    UnknownCharacter,
    /// The rest of a quoted string after a literal line break, up to its
    /// closing `"`. The lexer reports the line break.
    BrokenStringRest,

    // Tokens that the parser reads.
    /// A rule name.
    Name,
    /// A string in any form of KDL 2.0.
    String,
    /// A string in single quotes. The lexer reports it, and the parser reads
    /// it as a string.
    SingleQuoteSlip,
    /// A run of decimal digits.
    Number,
    /// `%doc`.
    DocKeyword,
    /// `%start`.
    StartKeyword,
    /// `%` and a name that is not a directive. The lexer reports it.
    UnknownDirective,
    /// `=`.
    Equals,
    /// `::=`, `:=`, `:` or `->`. The lexer reports it, and the parser reads it
    /// as `=` after a rule name.
    EqualsSlip,
    /// `;`.
    Semicolon,
    /// `|`.
    Pipe,
    /// `(`.
    LeftParen,
    /// `)`.
    RightParen,
    /// `?`.
    Question,
    /// `*`.
    Star,
    /// `+`.
    Plus,
    /// `{`.
    LeftBrace,
    /// `}`.
    RightBrace,
    /// `,`.
    Comma,
    /// `[`, which only the slip `[x]` uses.
    LeftBracket,
    /// `]`, which only the slip `[x]` uses.
    RightBracket,
    /// The end of the text. It never enters the tree.
    Eof,

    // Nodes.
    /// The whole grammar text.
    Grammar,
    /// `%doc string ;`.
    DocDeclaration,
    /// `%start name ;`.
    StartDeclaration,
    /// `name string? = expression ;`.
    Rule,
    /// The doc string of a rule.
    DocString,
    /// Alternatives, separated by `|`.
    Choice,
    /// Two or more items in order.
    Sequence,
    /// An item with its postfix operator.
    Postfix,
    /// A bound, such as `{2,4}`.
    Bound,
    /// A group in parentheses.
    Group,
    /// A quoted string in a rule body.
    Symbol,
    /// A rule name in a rule body.
    Reference,
    /// The slip `[x]`, which means `x?`.
    BracketSlip,
    /// The slip `{x}`, which means `x*`.
    BraceSlip,
    /// Tokens that the parser skipped after a syntax error.
    Error,
}

impl SyntaxKind {
    /// Whether the parser skips this token kind.
    pub fn is_trivia(self) -> bool {
        matches!(
            self,
            Self::Whitespace
                | Self::Comment
                | Self::DocCommentSlip
                | Self::CommentSlip
                | Self::Ampersand
                | Self::UnknownCharacter
                | Self::BrokenStringRest
        )
    }

    /// Whether this node kind is an expression.
    pub fn is_expression(self) -> bool {
        matches!(
            self,
            Self::Choice
                | Self::Sequence
                | Self::Postfix
                | Self::Group
                | Self::Symbol
                | Self::Reference
                | Self::BracketSlip
                | Self::BraceSlip
        )
    }

    /// Whether this token kind is a string for the parser.
    pub fn is_string(self) -> bool {
        matches!(self, Self::String | Self::SingleQuoteSlip)
    }

    /// Whether this token kind stands for `=` after a rule name.
    pub fn is_equals(self) -> bool {
        matches!(self, Self::Equals | Self::EqualsSlip)
    }

    const LAST: Self = Self::Error;
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

/// The `rowan` language of the grammar notation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Notation {}

impl rowan::Language for Notation {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> SyntaxKind {
        assert!(raw.0 <= SyntaxKind::LAST as u16, "unknown syntax kind {}", raw.0);
        // SAFETY: `SyntaxKind` is `repr(u16)` with dense discriminants from 0
        // to `LAST`, and the assertion above checks the range.
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
    }

    fn kind_to_raw(kind: SyntaxKind) -> rowan::SyntaxKind {
        kind.into()
    }
}

/// A node of the lossless syntax tree.
pub type SyntaxNode = rowan::SyntaxNode<Notation>;
/// A token of the lossless syntax tree.
pub type SyntaxToken = rowan::SyntaxToken<Notation>;
/// A node or a token of the lossless syntax tree.
pub type SyntaxElement = rowan::NodeOrToken<SyntaxNode, SyntaxToken>;
