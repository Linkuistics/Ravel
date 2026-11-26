# EBNF with regex-style operators over whole symbols

People and LLMs write grammars in an EBNF notation. A semicolon ends each rule. Options and repetitions use postfix operators, as regular expressions do: `?`, `*`, `+` and bounds such as `{2,4}` (https://pkg.go.dev/regexp/syntax). A bound above the bound cap of 100 is an error. The notation reserves syntax for an operator for steps in any order.

Every terminal is a whole symbol, written as a quoted string, such as `"submit_order"` or `"states the problem"`. Strings follow the string syntax of KDL 2.0 (https://kdl.dev/spec/). A bare name always refers to a rule, and a name without a rule is an error. So a misspelt rule name cannot pass as a new symbol. A grammar does not declare its symbols.

## Trade-off

A misspelt symbol passes as a new symbol, and no sequence matches it. A rule-check warning catches only symbols that differ by case, spaces, `_` or `-`. A step that needs documentation, or a check of each use, gets a rule of its own, such as `install = "install" ;`. In return, the notation does not depend on steps. The human chose this on 2026-11-18, because "the concept of a step is particular to only one use case." Quoted terminals are also the form of ISO 14977, W3C EBNF and GBNF, so LLMs know them.

Whole symbols exclude character classes and a lexer. The human accepted this: "Lexical sophistication can come later if required." The postfix forms depart from ISO 14977 EBNF. In return, LLMs know these forms from regular expressions and from GBNF (https://github.com/ggml-org/llama.cpp/blob/master/grammars/README.md). LLMs read these grammars to guide their own output, for example when they create a tutorial.

## Rejected alternatives

A declared list of symbols. It catches a misspelt symbol, and it holds the documentation of each symbol. It also makes the notation depend on one use, in which each symbol is a step. Undeclared bare names as implicit terminals, which ANTLR accepts with only a warning (https://github.com/antlr/antlr4/issues/269). They let a misspelt rule name pass as a new symbol.

The braces and brackets of ISO 14977 for repetitions and options. KDL as the grammar input. A Scheme s-expression syntax, which the human considered on 2026-11-18. It gives a simple parser and native strings, but each rule becomes about four times longer.

Source: the human's decisions of 2026-11-17 and 2026-11-18 in `docs/milestones/grammar-workbench.md` and in `notation-k3`, and the grammar-notations and automata-algorithms surveys.
