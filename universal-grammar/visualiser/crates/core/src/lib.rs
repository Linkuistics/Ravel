//! The core of the grammar workbench. It parses grammar text in the notation of
//! `docs/specs/grammar-notation.md`, runs the rule checks, and writes the
//! checked grammar as KDL. Its tracer reads sequences against a grammar, one
//! step at a time, by the spec `docs/specs/tracer.md`. It lays out the diagram
//! of each rule, by the spec `docs/specs/rule-diagrams.md`.
//!
//! The core builds as a native crate and for WebAssembly:
//! `docs/adr/rust-core-with-webassembly.md`.

mod automaton;
mod boundary;
mod cells_table;
pub mod checked;
mod checks;
pub mod diagnostic;
pub mod diagram;
mod diagram_file;
mod earley;
mod forest;
pub mod kdl_map;
mod lexer;
mod loops;
mod lower;
pub mod marks;
mod parser;
mod string;
pub mod syntax;
pub mod trace;
mod trace_file;

pub use checked::CheckedGrammar;
pub use diagram::{Diagrams, diagrams};
pub use kdl_map::{MappingError, from_kdl, to_kdl};
pub use trace::{Session, TraceGrammar};

/// The lossless syntax tree of a grammar text, with its syntax errors.
#[derive(Debug, Clone)]
pub struct Parse {
    green: rowan::GreenNode,
    diagnostics: Vec<diagnostic::RawDiagnostic>,
}

impl Parse {
    /// The root of the syntax tree. Its text is the whole grammar text.
    pub fn syntax(&self) -> syntax::SyntaxNode {
        syntax::SyntaxNode::new_root(self.green.clone())
    }

    /// The syntax errors, in the order of their spans.
    pub fn diagnostics(&self) -> &[diagnostic::RawDiagnostic] {
        &self.diagnostics
    }
}

/// Parses a grammar text into its lossless syntax tree.
pub fn parse(text: &str) -> Parse {
    let parser::Parse { green, diagnostics } = parser::parse(text);
    Parse { green, diagnostics }
}

/// Parses a grammar text and returns the checked grammar as Rust types.
pub fn checked_grammar(text: &str) -> CheckedGrammar {
    let parse = parse(text);
    lower::lower(text, &parse.syntax(), &parse.diagnostics)
}

/// Parses a grammar text and returns the checked grammar as KDL 2.0 text.
pub fn check_grammar(text: &str) -> String {
    to_kdl(&checked_grammar(text))
}
