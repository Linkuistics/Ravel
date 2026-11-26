# Documentation strings in rules

A grammar documents its rules with strings, not with comments. The string between a rule name and its `=` documents the rule, as in `install "Tell the reader how to install one tool." = "install" ;`. The directive `%doc` documents the grammar. LLMs read this documentation to follow a grammar, and the checked grammar carries it. A comment that starts with `///` or `//!` is a syntax error, and its message names the doc string.

## Trade-off

A doc string is a quoted string beside quoted symbols, so only its place before the `=` marks it as documentation. Doc comments would look different from symbols at a glance. In return, the documentation is part of the rule in the syntax, as in Common Lisp and Python. No rule must say which comment documents which rule. Doc strings use the KDL 2.0 strings of the notation, so a long text can use a multi-line string. The human chose this on 2026-11-18: "I do prefer the common lisp strategy of explicit documentation slots."

## Rejected alternative

Doc comments, `///` before a rule and `//!` for the grammar, as in Rust and in pest (https://pest.rs/book/grammars/syntax.html). A `///` line as a plain comment was also rejected. An LLM that writes doc comments by habit would then lose its documentation without a message.

Source: the human's decisions of 2026-11-18 in `notation-k3`, and the grammar-notations survey.
