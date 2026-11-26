# Hand-written parser over a lossless tree

The core uses a hand-written Rust parser for grammar text. The parser builds a lossless syntax tree with `rowan` (https://github.com/rust-analyzer/rowan), the tree crate of `rust-analyzer`. The tree keeps every character, so later editor features can map each error and each highlight to an exact span. The parser builds the tree in one place, so the tree crate stays replaceable.

## Trade-off

The project maintains its own parser and its own error messages. Editors such as Neovim, Helix and Zed get no highlighting until someone writes a tree-sitter grammar. In return, the core builds for WebAssembly without a C compiler, and each error message can say what the author probably meant. The human judged that incremental parsing, the main benefit of tree-sitter, "is unlikely to be a bottleneck" for grammars.

A `rowan` syntax node is `!Send` and `!Sync` (https://docs.rs/rowan/0.17.0/rowan/api/struct.SyntaxNode.html). A later language server with several threads must share the green tree, which is `Send` and `Sync`, and build syntax nodes again in each thread. `rust-analyzer` works in this way. In return, the parser uses the tree crate with the most use. On 2026-11-18, crates.io counted about 7.2 million downloads of `rowan` in 90 days, and about 15,000 downloads of `cstree`. The direct dependencies of `rowan` are Rust crates, with no C library.

## Rejected alternatives

A tree-sitter grammar, which the human first proposed. The language-tooling survey found that tree-sitter writes no error messages (https://blog.jez.io/tree-sitter-limitations/) and gives no colour in VS Code (https://github.com/microsoft/vscode/issues/50140). Tree-sitter stays the fallback if the notation grows, or if users of other editors ask for it early.

`cstree` (https://docs.rs/cstree/0.14.0/cstree/), a fork of `rowan`. Its syntax nodes are `Send` and `Sync`, its syntax nodes persist after their first use, and it interns the text of tokens. These gains matter for large files in a language server with several threads. A grammar has a few hundred lines, and the WebAssembly module runs in one thread.

Source: the human's decisions of 2026-11-17 in `docs/milestones/grammar-workbench.md`, the language-tooling survey, and the checks of `notation-k3` on 2026-11-18.
