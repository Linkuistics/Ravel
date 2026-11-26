# Grammar notations: prior art for the grammar workbench

**Status: complete, with gaps.** Each section marks a check that is still open with the word "Open". The section "Silences" lists the searches that found nothing.

**Topic:** notations for hand-written grammars, and the critiques of each. The questions that matter most are Q2, Q3 and Q5. The human chose the grammar workbench, where a terminal is a symbol such as a move label, not a character.

## Findings that change decisions

Each finding names the question that it affects. The word "Inference" marks a conclusion that is mine and not the source's.

- **Q2.** No grammar notation is standard in practice. Zaytsev found 42 notations in 77 grammars, and ISO/IEC 14977 "only ended up adding yet another three dialects to the chaos" ([Wheeler](https://dwheeler.com/essays/dont-use-iso-14977-ebnf.html)). Inference: readability for the workbench user must decide the notation, not the authority of a standard.
- **Q2.** The request notation has a close ancestor whose tokens are words. JSGF writes `<rule> = expansion ;`, uses white space for sequence, and defines `|`, `( )`, `*`, `+` and `[ ]` ([JSGF](https://www.w3.org/TR/jsgf/)).
- **Q2, Q6.** A rule end that depends on layout made the meta-grammar of ABNF ambiguous. Verified errata 2968 and 3076 show one input that parses in two ways ([RFC 5234 errata](https://errata.rfc-editor.org/rfc5234)). ABNF also starts a comment with `;`, which ends a rule in ISO 14977 and JSGF. Inference: a required `;` gives the parser a fixed point to resume after an error.
- **Q2.** Each engine that expands a bounded repeat into copies caps the count or warns. Go rejects counts "above 1000", and a huge GBNF minimum "OOM-kills the server". Braces mean repetition in ISO 14977 and tags in JSGF. Inference: `{m,n}` is free in W3C EBNF, and the diagram must draw the bound, not the copies.
- **Q2.** Quoted terminals let a misspelt move label pass as a new terminal. A declared alphabet catches it, but ANTLR reports an undeclared token only as a warning. Inference: the observed traces give an alphabet, so the editor can flag a terminal that no trace holds.
- **Q3.** Both kinds of choice can hide an ambiguity. RFC 3986 calls its host rule "ambiguous" and fixes it with a prose "first-match-wins" rule. Under ordered choice, "an expression `"a"/"ab"` does not match string “ab”" ([Bílka](https://arxiv.org/html/1205.1877v1)). Inference: the tracer must report each parse of a sequence.
- **Q5.** Each schema language after SGML removed or restricted free order. XML dropped the `&` connector in 1996, and RELAX NG forbids interleave operands that share an element name. Schemas that use the relaxed `xs:all` of XML Schema 1.1 still load in no XSD 1.0 validator. Inference: the interleave of n symbols needs 2^n automaton states.
- **Q2, Q3.** Positional access to the parts of a tree broke when a rule changed. Lark 1.0 put `None` in place of an absent optional part and broke the ROS 2 IDL parser. Ohm throws an error when the arity of an action differs from its rule. Inference: named parts survive such edits.
- **Q2.** Serialised grammar forms broke their consumers when the shape changed without a format version. tree-sitter 0.25.7 changed `node-types.json` and broke the Node.js binding. KDL 2.0 changed literal syntax and split the tools of Zellij users. Inference: the workbench needs a format version for its grammar data.
- **Q2.** No study found measures LLM accuracy on KDL, and no constrained-decoding engine found ships a KDL grammar. On TOML, GPT-4o scores 90.22 and GPT-4o-mini scores 29.56 ([StructEval](https://arxiv.org/abs/2505.20139)). Inference: LLM output in KDL needs a parser check.

## Q1 Purpose

Grammar notations served three jobs:

- **Specification for human readers.** W3C EBNF, ABNF and ISO 14977 describe the syntax of a standard. Their terminals are characters.
- **Recognition of symbol sequences.** DTD content models, RELAX NG, XML Schema, JSGF and SRGS constrain sequences of element names or words.
- **Parser generation.** ANTLR, Lark, Ohm and tree-sitter compile a grammar into a parser, and add actions, labels and precedence.

Inference: the workbench does the second job. **Fallback to beat:** W3C EBNF, because most published grammars use a dialect of it. A departure needs evidence that a symbol-level family serves the workbench better.

## Q2 Notation

**Fallback to beat:** the request notation. It joins the rule form of ISO 14977, `Name = Expression ;`, to the operators of W3C EBNF: juxtaposition, `|`, `( )` and postfix `* + ?`. A departure needs evidence that URG authors already write another notation, or that a required `;` causes errors.

### The notation with the most use

No survey names one notation as the most used. Wheeler recommends W3C EBNF, and ABNF only inside RFCs ([Wheeler](https://dwheeler.com/essays/dont-use-iso-14977-ebnf.html)). He states that ISO 14977 "is widely perceived as a failure, as it is often not used (even by the organization that created it)". Open: the primary text of the Zaytsev survey.

### Notations over symbols

Two families already use symbols as terminals:

- **Document schemas.** A DTD content model, a RELAX NG pattern and an XML Schema particle each constrain a sequence of element names. A DTD writes sequence with `,` and choice with `|`, and one group cannot hold both ([XML 1.0 §3.2.1](https://www.w3.org/TR/xml/#sec-element-content)). So an author must write `(head, (p | list | note)*, div2*)` with an inner group. Open: RELAX NG compact syntax.
- **Speech grammars.** In JSGF, "A token is a reference to an entry in a recognizer's vocabulary", and "Most often, a token is equivalent to a word" ([JSGF](https://www.w3.org/TR/jsgf/)). Quotes enclose a token that holds a space, such as `"Los Angeles"`. Open: SRGS 1.0.

Inference: neither family splits lexical rules from syntactic rules. The workbench needs only a rule that says what counts as one symbol, and a quoted form for labels that hold a space.

### Syntax that caused errors

- **Commas for sequence.** ISO 14977 "requires a sea of commas, so using it produces hard-to-read grammars" ([Wheeler](https://dwheeler.com/essays/dont-use-iso-14977-ebnf.html)).
- **One or more as `{ x }-`.** A missing comma after it means that "the following expression will be subtracted from the former".
- **Prefix repetition.** ABNF writes `1*x` for one or more. Wheeler states "There's no good justification for using a syntax that's different from what developers use every day."
- **Mixed sequence and alternation.** RFC 5234 §3.10 ranks concatenation above alternation, and then warns that the mix "can be confusing". W3C EBNF states that `A B | C D` "is identical to `(A B) | (C D)`".
- **Subtraction without a stated precedence.** W3C EBNF defines `A - B`, but section 6 gives no precedence for `-`.
- **Case folding.** ABNF strings match any case, and "Rule names are case insensitive" ([RFC 5234](https://www.rfc-editor.org/rfc/rfc5234)). RFC 7405 added `%s"…"` because the workaround was "inconvenient and error prone" ([RFC 7405](https://www.rfc-editor.org/rfc/rfc7405)). XML states "No case folding is performed". Inference: the editor should warn when two move labels differ only by case.

### Rule terminators

| Notation | End of a rule | Source |
| --- | --- | --- |
| ISO 14977 | A `;` | Wheeler essay |
| JSGF | A required `;`, the "closing semi-colon" | JSGF §3 |
| ANTLR | A required `;` | [ANTLR parser rules](https://github.com/antlr/antlr4/blob/master/doc/parser-rules.md) |
| Instaparse | An optional `;` or `.` | [Instaparse](https://github.com/Engelberg/instaparse) |
| pest | The `}` that closes the rule body | [pest](https://pest.rs/book/grammars/syntax.html) |
| DTD | The `>` that closes each `<!ELEMENT … >` declaration | XML 1.0 §3.2 |
| W3C EBNF | No terminator. The next `Name ::=` starts a new rule. | XML 1.0 §6 |
| Ohm | No terminator. The next `Name =` starts a new rule. | [Ohm](https://ohmjs.org/docs/syntax-reference) |
| ABNF | A line break, unless the next line starts with white space | RFC 5234 §2.2 |
| Lark | A line break, unless the next line starts with `\|` | [Lark](https://lark-parser.readthedocs.io/en/latest/grammar.html) |
| GBNF | A line break, except after `\|` or inside parentheses (inferred) | [GBNF](https://github.com/ggml-org/llama.cpp/blob/master/grammars/README.md) |
| Others | Open: Wirth EBNF, SRGS, RELAX NG compact | Open |

The W3C form forces a parser to look two tokens ahead at each name, because only the `::=` shows that a new rule starts. ABNF avoids the look-ahead and makes line breaks significant: "When a rule requires multiple lines, the continuation lines are indented." Erratum 2968 gives the input `X=Y`, a line break, a space and `;Z`. That input parses as one rule, or as a rule and a comment line ([RFC 5234 errata](https://errata.rfc-editor.org/rfc5234)). The verifier wrote "The current document is clearly incorrect", and noted that the fix "has not been tested".

Inference: a required `;` helps error recovery in two ways. First, a parser that meets an error can skip to the next `;` and report errors in later rules in the same pass. Second, `=` never occurs inside an expression, so a missing `;` shows at the next `Name =`. The cost is one character for each rule.

### Bounded repetition

The human wants EBNF with minimum and maximum repeats. Five notations write the same bound in five ways:

| Notation | Exactly three | Two to four | Source |
|---|---|---|---|
| ISO 14977 | `3 * x` | `2 * x, 2 * [x]` | Open |
| ABNF | `3x` | `2*4x` | [RFC 5234 §3.6 and §3.7](https://www.rfc-editor.org/rfc/rfc5234) |
| Regular expressions, GBNF | `x{3}` | `x{2,4}` | [Go regexp](https://pkg.go.dev/regexp/syntax), [GBNF](https://github.com/ggml-org/llama.cpp/blob/master/grammars/README.md) |
| Lark | `x ~ 3` | `x ~ 2..4` | [Lark grammar reference](https://lark-parser.readthedocs.io/en/latest/grammar.html) |
| XML Schema | `minOccurs="3" maxOccurs="3"` | `minOccurs="2" maxOccurs="4"` | Open |

Engines that expand a bound into copies set a cap or warn:

- Lark marks `item ~ n..m` as "not recommended for wide ranges, due to performance issues". Lark rewrites EBNF into plain BNF before it parses.
- Go rejects "a minimum or maximum repetition count above 1000", and rewrites `(x){1,2}` as `(x)(x)?`.
- For `xsd:sequence`, "the validating parser may use space (memory) proportional to the value of the `maxOccurs` occurrence indicator" ([JAXP guide](https://docs.oracle.com/en/java/javase/17/security/java-api-xml-processing-jaxp-security-guide.html)). The JDK caps it at 5000 content model nodes by default.
- The GBNF README warns that `x? x? x?.... x?` "may result in extremely slow sampling", and recommends `x{0,N}`. A huge minimum count "OOM-kills the server" ([llama.cpp issue 29462](https://github.com/ggml-org/llama.cpp/issues/29462)).
- llguidance asks for "`one{N}` and not `one one ... one`". It warns that right recursion hits "parser item limits quite quickly (after 100-1000 repetitions)" ([llguidance syntax](https://github.com/guidance-ai/llguidance/blob/main/docs/syntax.md)).

Inference: a Glushkov automaton has one state for each position, so `x{1,100}` draws a hundred copies of `x`. The diagram becomes illegible long before the engine slows. Open: counting automata as an alternative picture.

Each bound syntax clashes with at least one dialect:

- ISO 14977 uses braces for zero or more repetitions, and the comma for sequence. A bound `x{2,4}` has another meaning there, or none.
- JSGF uses braces for tags, which "do not affect the recognition of a grammar" ([JSGF](https://www.w3.org/TR/jsgf/)).
- ABNF writes the count before the element. Inference: `a *b` means `a` and then any number of `b` in ABNF. In a postfix notation, the same text means any number of `a` and then `b`.
- Lark keeps braces for templates such as `_separated{x, sep}`, and writes its bounds with `~`.
- W3C EBNF gives braces no meaning, so `{m,n}` is free there. Inference: ISO `3 * x` collides with postfix `*` when a terminal can be a bare number.

### Telling terminals from rule names

Three ways tell a terminal from a rule name, and each catches a different set of misspelt names:

| Way | Examples | Misspelt rule name | Misspelt terminal |
|---|---|---|---|
| Quoted terminals | ISO 14977, W3C EBNF, the request | An undefined name | A new terminal that no check can see |
| Declared alphabet | Bison `%token`, ANTLR `tokens { }`, DTD element declarations | An undefined name | An undeclared name |
| Naming convention | Lark upper case, ANTLR initial capital | An undefined name | ANTLR warning 125 |

ANTLR reports a token name with no lexer rule as "warning(125): A.g4:2:13: implicit definition of token INT in parser" ([ANTLR errata](https://pragprog.com/cms/errata/tpantlr2-errata/)). The ANTLR tool lists it as a warning, not an error ([ErrorType](https://www.antlr.org/api/JavaTool/org/antlr/v4/tool/ErrorType.html)). Inference: the grammar still compiles, and the misspelt token never matches. A second inference: the observed sequences give an alphabet without a declaration, so a terminal that occurs in no trace is a candidate misspelling.

**Fallback to beat:** quoted terminals, as in the request. A closed move vocabulary in URG would justify a declared alphabet, because it catches both kinds of misspelling.

### Shaping the parse tree

The human wants the parse tree as the main result of a trace. Six tools let the grammar decide which nodes the tree shows:

| Tool | Hide a node | Name an alternative | Name a part of a rule |
|---|---|---|---|
| Lark | `_rule` always, `?rule` with one child, `_TERM` and quoted literals | `-> alias` | No |
| tree-sitter | `_rule`, `supertypes`, `inline` | `alias($.x, $.y)` | `field(name, rule)` |
| pest | `_{ }`, and the interior of `@{ }` | No | `#tag = …`, behind the "grammar-extras" feature |
| ANTLR | No | `# Label` | `x=e` and `x+=e` |
| Ohm | No, because "Grammars are purely about recognition" | `-- caseName` makes the rule `Rule_caseName` | No |
| Instaparse | `<rule>` and `<'token'>` | No | No |

Sources: [Lark](https://lark-parser.readthedocs.io/en/latest/tree_construction.html), [tree-sitter DSL](https://tree-sitter.github.io/tree-sitter/creating-parsers/2-the-grammar-dsl.html), [pest](https://pest.rs/book/grammars/syntax.html), [ANTLR](https://github.com/antlr/antlr4/blob/master/doc/parser-rules.md), [Ohm](https://ohmjs.org/docs/syntax-reference), [Instaparse](https://github.com/Engelberg/instaparse). tree-sitter hides wrapper rules because "they would add substantial depth and noise to the syntax tree" ([tree-sitter](https://tree-sitter.github.io/tree-sitter/creating-parsers/3-writing-the-grammar.html)).

Each mechanism failed in real use:

- **Lark placeholders.** Release 1.0.0 made "`maybe_placeholders` is now True by default" ([Lark 1.0.0](https://github.com/lark-parser/lark/releases/tag/1.0.0)). The ROS 2 IDL parser broke, because "the AST will contain the value `None` when there aren't any parentheses or parameters" ([rosidl PR 664](https://github.com/ros2/rosidl/pull/664), 18 February 2022).
- **Lark conditional inlining.** Lark removes a `?rule` node "if they have a single child, after filtering". Inference: one rule then gives two tree shapes, and each consumer must handle both.
- **tree-sitter inlining.** Inlining copied "the reference's alias and field onto those steps" ([PR 6054](https://github.com/tree-sitter/tree-sitter/pull/6054), merged 21 October 2026). So the node-type analysis "described trees that the generated parser does not build". The effects included "missing entries for aliased nodes" and "fields that never occur in the resulting tree".
- **pest modifiers.** Atomicity passes to called rules, but silence does not. An issue open since 13 June 2021 states "there is currently no way to create a rule that is both silent and atomic" ([issue 520](https://github.com/pest-parser/pest/issues/520)).
- **ANTLR labels.** "All alternatives within a rule must be labeled, or none of them." A label clashes with a rule name after capitalisation: "error(124): A.g4:6:23: rule alt label Stat conflicts with rule stat". A false label clash from 2013 got a fix in 4.1, and came back in 4.6 for left-recursive rules ([issue 195](https://github.com/antlr/antlr4/issues/195)).
- **Ohm arity.** An action takes the children by position, and "An error is thrown if the function arity does not match the number of values produced by the expression" ([Ohm API](https://ohmjs.org/docs/api-reference)). Inference: a term added to a rule breaks each action on that rule.

Inference: positional access broke in Lark and in Ohm when a rule gained or lost a child. Named parts, as in ANTLR and tree-sitter, survive such edits. Two parses that differ only inside a hidden node print as the same tree, so a hidden node can hide an ambiguity.

**Fallback to beat:** each rule makes a node, each terminal makes a leaf, and nothing hides. A departure needs traces whose wrapper rules make the tree too deep to read. Then `_rule`, which Lark, tree-sitter and pest share in some form, has the widest precedent.

### Serialised forms of grammars

The human chose KDL as a serialised form that many languages can read. Prior forms use JSON or generated code, and their lessons on versions apply to any format.

- **tree-sitter node types.** The CLI generates `node-types.json`, which "provides structured data about every possible syntax node in a grammar" ([tree-sitter docs](https://tree-sitter.github.io/tree-sitter/using-parsers/6-static-node-types.html)). Code generators in Rust and Haskell read it. The file has no version field, and the docs make no promise of stability.
- **A changed shape broke a binding.** Release 0.25.7 wrapped the items of `node-types.json` in an object with one `Ok` key. The Node.js binding then failed with "TypeError: nodeTypeInfo.find is not a function" ([issue 4598](https://github.com/tree-sitter/tree-sitter/issues/4598)).
- **Grammar changes break consumers.** Issue 2611 states "Just about any time a change is made to a grammar, the change is somehow breaking to consumers of the grammar" ([issue 2611](https://github.com/tree-sitter/tree-sitter/issues/2611)). Emacs finds the version of a grammar by "writing a simple query and seeing if it throws an error". The maintainers proposed semantic versions, where a major release removes or changes nodes that queries use.
- **Generated types.** Semantic generated Haskell types from `node-types.json`. Its earlier code "could break at runtime if we changed the structure of the grammar, without any compile-time error" ([Semantic codegen](https://github.com/github/semantic/blob/main/docs/codegen.md)).
- **Ohm recipes.** Release 0.12.0 states "Grammar recipe format is now pure JSON, rather than JavaScript source" ([Ohm changelog](https://github.com/ohmjs/ohm/blob/main/packages/ohm-js/CHANGELOG.md)). The recipe format has no version number of its own. A Bruno issue proposes to tie each cached recipe to the Ohm release and the grammar source ([Bruno issue 9025](https://github.com/usebruno/bruno/issues/9025)).
- **ANTLR serialised automata.** ANTLR stores each automaton "in serialized form as a bunch of integers stored in the generated parser and lexer files" ([ANTLR 4.10](https://github.com/antlr/antlr4/releases/tag/4.10)). Release 4.10 changed the format and the version number inside it. Its notes state "You must regenerate all of your code from grammars to use the new runtime."

Inference: each form broke when its shape changed and no consumer checked a format version. ANTLR carries one, so an old runtime can refuse a new parser instead of misreading it. A grammar in KDL therefore needs a format version in the document.

#### KDL as the serialised form

- **Status.** The site states "KDL 2.0.0 has been finalized, and no further changes are expected" ([kdl.dev](https://kdl.dev/)). It calls the migration from v1 "forward-and-backward-compatible and safe".
- **Implementations.** The site marks each library as v1 only, v2 only, or neither. Rust has kdl-rs without a mark, JavaScript has @bgotink/kdl and kdljs, and Python has ckdl and kdl-py.
- **Changes from v1.** Version 2.0.0 dates from 21 December 2024 ([KDL changelog](https://github.com/kdl-org/kdl/blob/main/CHANGELOG.md)). `true`, `false` and `null` became `#true`, `#false` and `#null`, and the old forms are now a parse error. A bare identifier became a legal string, and raw strings lost the `r` prefix. An optional first line `/- kdl-version 2` marks the version of a document.
- **Schema language.** KDL Schema 1.0.0 dates from 11 September 2021, before KDL 2.0 ([KDL Schema](https://github.com/kdl-org/kdl/blob/main/SCHEMA-SPEC.md)). It names "automated generation of bindings" as one use, but describes no generator. It bounds child counts with `min` and `max`, and checks values with `enum` and `pattern`.
- **JSON in KDL.** JiK 4.0.0 encodes an array as a node with `-` children, and an empty array as `(array)-` ([JiK](https://github.com/kdl-org/kdl/blob/main/JSON-IN-KDL.md)). "If a JiK document contains an invalid node, the entire document must fail to encode."
- **Breakage in a user.** Zellij reads its configuration with kdl-rs. A pull request adds KDL v2 through the "v1-fallback" of kdl-rs, which tries v2 first and then v1 ([Zellij PR 3908](https://github.com/zellij-org/zellij/pull/3908)). On that branch, Zellij failed to start with "Failed to parse KDL document". In June 2025, a user reported Zellij syntax errors on files that a current KDL language server accepted. Inference: two versions of one format split the tools of one user for at least six months.

Inference: the KDL version marker marks the syntax of the document, not the shape of the grammar data. So the workbench still needs its own format version.

#### LLM accuracy and constrained decoding

- **No KDL evidence.** No benchmark or study found measures LLM output in KDL. One user wrote "KDL is very picky about syntax. I.e.: single word string MUST be quoted." A reply noted that the current version dropped that rule ([Hacker News](https://news.ycombinator.com/item?id=48982011)). Inference: a model trained on both versions can mix v1 and v2 syntax in one file.
- **Other formats.** StructEval scores 18 formats ([StructEval](https://arxiv.org/abs/2505.20139)). From text, GPT-4o scores 99.36 on JSON, 97.68 on YAML, 90.22 on TOML and 70.32 on XML. Open models average 8.58 on TOML, and closed models average 75.78.
- **Constrained decoding.** XGrammar takes JSON Schema, EBNF and Lark grammars ([XGrammar](https://xgrammar.mlc.ai/docs/latest/start/quick_start.html)). It is the default backend of vLLM, SGLang, TensorRT-LLM and MLC-LLM ([XGrammar repository](https://github.com/mlc-ai/xgrammar)). llguidance takes JSON Schema and a Lark variant, and runs in llama.cpp, SGLang and vLLM ([llguidance](https://github.com/guidance-ai/llguidance)).
- **EBNF dialects.** No study found compares how well LLMs write each EBNF dialect. Grammar Prompting has a model write a specialised BNF before its answer ([Grammar Prompting](https://arxiv.org/abs/2305.19234)). SynCode constrains output with an EBNF adapted from Lark ([SynCode](https://arxiv.org/abs/2403.01632)).

Inference: JSON is the only format with ready support in each engine. KDL output needs a hand-written GBNF or Lark grammar, or a parser check after generation. This report leaves one tension open. The human chose KDL, but the evidence on LLM output favours JSON.

## Q3 Semantics

**Fallback to beat:** unordered choice, with a report of each parse, as the Glushkov automaton and the tracer of the request give. A departure to ordered choice needs evidence that authors want the first match to win.

### Ambiguity under unordered choice

RFC 3986 admits an ambiguity in its own host rule: "The syntax rule for host is ambiguous because it does not completely distinguish between an IPv4address and a reg-name." Section 4.1 states that the grammar "along with the 'first-match-wins' disambiguation rule, is sufficient to define a validating parser" ([RFC 3986](https://www.rfc-editor.org/rfc/rfc3986#section-3.2.2)). The ordering rule lives in prose outside the grammar, so a reader of the grammar alone misses it.

RFC 5234 does not say whether `/` tries its alternatives in order. It says only that `foo / bar` "will accept `<foo>` or `<bar>`" ([RFC 5234](https://www.rfc-editor.org/rfc/rfc5234)). Inference: readers take `/` as unordered, and RFC 3986 needed the prose rule for that reason. The meta-grammar of ABNF shows the same hazard, because errata 2968 and 3076 report two parses of one input. Inference: a tool that reports each parse of a sample would have shown this ambiguity to the authors.

### Determinism in content models

XML requires each content model to be deterministic, for compatibility with SGML ([XML 1.0 Appendix E](https://www.w3.org/TR/xml/#determinism)). The appendix rejects `((b, c) | (b, d))`, because "given an initial b the XML processor cannot know which b in the model is being matched". A model is in error "if any position has a follow set in which more than one following position is labeled with the same element type name". Inference: these positions and follow sets are the states and transitions of the Glushkov automaton in the request. The XML test then checks that this automaton is deterministic.

The XML working group voted twice to keep the ban, with Bray, Paoli and Sharpe in dissent ([XML WG reports](https://www.w3.org/XML/9712-reports.html), C.10). The reason was that "Existing SGML tools, for example the SP family, have this rule wired deeply into their logic". Opponents of a change also argued that some nondeterministic models have no deterministic equivalent. Inference: the workbench does not need the ban, because its tracer can follow several states at once. The check still has value as a warning, because it marks where one symbol starts two paths.

### Ordered choice

PEG tools such as Ohm and pest use ordered choice, where the first alternative that matches wins. Bílka writes "This definition without backtracking introduced problem of prefix hiding", because "an expression `"a"/"ab"` does not match string “ab”" ([Bílka 2012](https://arxiv.org/html/1205.1877v1)). A longest-match operator "does not eliminate the prefix hiding completely". Inference: ordered choice turns an ambiguity into a silent failure, so the fallback of unordered choice stands.

## Q5 Scope

**Fallback to beat:** the operators in the request, with no interleave, no subtraction and no case folding. A departure needs evidence that URG episodes hold moves in free order, or that authors need to exclude a sequence.

### Features that prior notations removed or regretted

- **The SGML `&` connector.** XML removed it from DTDs, together with inclusions and exclusions ([Clark 1997](https://www.w3.org/TR/NOTE-sgml-xml-971215)). Clark gives no reason, but the decision records of the SGML review board do ([XML WG reports](https://www.w3.org/XML/9712-reports.html)). On 23 October 1996 the board "Agreed unanimously to forbid use of the '&' connector in XML", for "harmonization with conventional regular expressions" (C.9). For inclusions and exclusions, the reason is "simplification of validation and harmonization of XML parsing model with standard formal-language theory and practice" (C.7). A later report notes "We never did clarify how AND makes determinism more important".
- **Alternative characters.** ISO 14977 allows substitutes such as `(:` for braces, for typewriters. Wheeler writes "There is no reason today to use that nonsense."
- **A whitespace rule with a warning.** ABNF defines `LWSP` and then warns "Do not use when defining mail headers and use with caution in other contexts." ([RFC 5234](https://www.rfc-editor.org/rfc/rfc5234)).
- **Case-insensitive strings.** ABNF kept the default for backward compatibility and added `%s` later ([RFC 7405](https://www.rfc-editor.org/rfc/rfc7405)).

### Interleave

Two schema languages after XML kept an operator for free order, and each one restricted it.

- **RELAX NG.** The two operands of `interleave` cannot share an element name, and "a text pattern must not occur in both p1 and p2" ([RELAX NG §7.4](https://relaxng.org/spec-20011203.html)). Section 7.1.2 also forbids the path `oneOrMore//interleave//attribute`.
- **XML Schema 1.0.** An element inside `xs:all` can occur zero times or once. A schema author asked for the reason in 2002 ([question](https://lists.w3.org/Archives/Public/xmlschema-dev/2002Mar/0203.html)). Jeni Tennison replied "I'm not sure about the rationale for this restriction" ([reply](https://lists.w3.org/Archives/Public/xmlschema-dev/2002Mar/0207.html)). She guessed at an unclear meaning: "does it mean that the elements of the same type have to be next to each other or not?" Her workarounds were a canonical order with "a script or stylesheet" to sort each document, or "Schematron or another schema language" to check counts.
- **XML Schema 1.1.** The working group wrote "maxOccurs>1 will be allowed on `<xs:all>` groups" ([bug 2506](https://lists.w3.org/Archives/Public/www-xml-schema-comments/2007AprJun/0025.html)). An `all` group inside a sequence or a choice "was deemed sufficiently complicated to implement that the function is not being provided in XML Schema 1.1". The group "could not reach consensus" on keeping the repeats of one element together.
- **Cost of the change.** A schema with `maxOccurs` above 1 inside `xs:all` loads in no XSD 1.0 validator ([issue 18](https://github.com/natresnet/buildingregistry-schema-v2/issues/18), September 2026). The issue names libxml2, "and therefore Python's `lxml`, PHP's `DOMDocument` and most command-line XML tooling".

Inference: each restriction ties one symbol to one operand, so a tracer can tell which operand a symbol advances. A deterministic automaton for the interleave of n distinct symbols needs one state for each subset of the symbols, so it has 2^n states. The diagram should therefore draw interleave as one box with parallel lanes, and not expand it.

**Fallback to beat:** no interleave. A departure needs URG episodes where one set of moves occurs in more than one order. The size of such sets decides whether the 2^n expansion matters.

### Annotations

W3C EBNF attaches `[ wfc: … ]` and `[ vc: … ]` constraints that name a check on a production ([XML 1.0 §6](https://www.w3.org/TR/xml/#sec-notation)). JSGF tags "do not affect the recognition of a grammar" ([JSGF](https://www.w3.org/TR/jsgf/)). JSGF weights almost follow the same rule, but a zero weight means "the item can never be spoken". Inference: an annotation in the workbench must either leave the language unchanged or show its effect in the diagram.

### Incremental rules

ABNF lets a later rule add alternatives to an earlier rule with `=/`, because "It is sometimes convenient to specify a list of alternatives in fragments" ([RFC 5234 §3.3](https://www.rfc-editor.org/rfc/rfc5234)). Inference: URG refinement could add alternatives in the same way. That use is a candidate feature, not a requirement.

### Recursion

JSGF allows right recursion only, and states "Technically speaking, the JSpeech Grammar Format defines what is formally called a regular grammar." Inference: the request plans a tracer for full recursion, so it goes beyond the JSGF limit.

## Q6 Test seams

The verifier accepted the fix for ABNF erratum 2968 and wrote that it "has not been tested" ([RFC 5234 errata](https://errata.rfc-editor.org/rfc5234)). Inference: the workbench should write the grammar of its own notation in that notation, and test it with the same tracer.

Prior tools report errors in three forms:

- **ANTLR, for grammar files.** Each report has the form `error(code): file:line:col: message`. An undefined rule gives code 56, "reference to undefined rule: <arg>" ([ErrorType](https://www.antlr.org/api/JavaTool/org/antlr/v4/tool/ErrorType.html)).
- **Instaparse, for input that fails.** The report starts "Parse error at line 1, column 5:", then shows the input line, a caret and the expected tokens ([Instaparse](https://github.com/Engelberg/instaparse)).
- **Ohm, for input that fails.** A `message` is "suitable for end users of a language", and `getRightmostFailurePosition()` returns an index ([Ohm API](https://ohmjs.org/docs/api-reference)). The page shows no line or column.

Open: the error formats of Lark and GBNF.

**Fallback to beat:** `file:line:col: message` with a caret line, as in ANTLR and Instaparse. Inference: a test can then compare the exact error text, and the editor can jump to the place.

## Systems examined

| Name | URL | Purpose | Licence | Latest release | Maintenance status | Walk-away result |
| --- | --- | --- | --- | --- | --- | --- |
| ISO/IEC 14977 EBNF | https://dwheeler.com/essays/dont-use-iso-14977-ebnf.html | Standard syntax notation | ISO copyright, free download | 1996 | No revision | Legible, but `{ x }-` needs the standard |
| W3C EBNF | https://www.w3.org/TR/xml/#sec-notation | Notation of the XML specification | W3C document licence | XML 1.0 Fifth Edition, 2008 | Stable | Legible to a reader of regular expressions |
| ABNF | https://www.rfc-editor.org/rfc/rfc5234 | Notation of IETF protocols | IETF Trust | RFC 5234 in 2008, RFC 7405 in 2014 | Errata open since 2011 | Legible, but prefix counts need the RFC |
| JSGF | https://www.w3.org/TR/jsgf/ | Speech grammars over words | Sun copyright, W3C Note | 5 June 2000 | Frozen | Legible, but tags and weights need the Note |
| SGML and XML DTD | https://www.w3.org/TR/NOTE-sgml-xml-971215 | Content models over element names | W3C document licence | XML 1.0 Fifth Edition, 2008 | Stable | Legible |
| RELAX NG | https://relaxng.org/spec-20011203.html | Patterns over element names | Not checked | 3 December 2001 | Stable | Legible, but interleave limits need the specification |
| XML Schema 1.1 | https://www.w3.org/TR/xmlschema11-1/ | Schemas over element names | W3C document licence | 5 April 2012 | libxml2 reads 1.0 only | XML syntax, hard to read by hand |
| Lark | https://github.com/lark-parser/lark | Parser generator for Python | MIT, standalone tool MPL-2.0 | Not shown | Not checked | Legible |
| tree-sitter | https://github.com/tree-sitter/tree-sitter | Incremental parser generator | MIT | 0.25.7 seen | Merge on 21 October 2026 | `grammar.js` needs JavaScript |
| pest | https://github.com/pest-parser/pest | PEG parser generator for Rust | MIT or Apache-2.0 | Not shown | Issue 520 open since 2021 | Legible |
| ANTLR | https://github.com/antlr/antlr4 | Parser generator for many languages | BSD-3-Clause | Not shown, README names 4.14 | Not checked | Legible, but actions need the target language |
| Ohm | https://github.com/ohmjs/ohm | PEG toolkit for JavaScript | Not checked | 17.2.1, 28 July 2025 | Not checked | Legible, but recipes need Ohm |
| Instaparse | https://github.com/Engelberg/instaparse | EBNF and ABNF parsers for Clojure | EPL-1.0 | 1.5.0 | Not checked | Plain EBNF or ABNF |
| GBNF | https://github.com/ggml-org/llama.cpp/blob/master/grammars/README.md | Constrained decoding in llama.cpp | Not checked | Numbered builds | Issues filed in 2026 | Legible |
| llguidance | https://github.com/guidance-ai/llguidance | Constrained decoding with a Lark variant | MIT | v1.0.0, 23 June 2025 | Runs in llama.cpp, SGLang and vLLM | Legible |
| XGrammar | https://github.com/mlc-ai/xgrammar | Constrained decoding | Apache-2.0 | 0.2.9 documented, XGrammar-2 in May 2026 | Default in vLLM and SGLang | Inputs stay JSON Schema, EBNF or Lark |
| KDL | https://kdl.dev/ | Serialised form | CC BY-SA 4.0 for the text | 2.0.0, 21 December 2024 | Final, libraries split between v1 and v2 | Legible |

## Silences

These searches found nothing, or less than the question needed:

- The query "why XML removed SGML "&" connector content model and-group ambiguity exponential" found the decision records. They give no measurement of a cost of `&`.
- The comparison of SGML and XML by Clark gives no reason for any limit that it lists.
- RFC 7405 cites no failure that the case-insensitive default of ABNF caused.
- The queries "KDL document language LLM generation accuracy" and "kdl config ChatGPT OR Claude OR LLM niri OR zellij writes wrong syntax" found no study. They found one anecdote on Hacker News.
- The query "KDL grammar GBNF OR lark OR json schema constrained decoding llama.cpp" found no KDL grammar for any engine.
- The query "LLM generate EBNF grammar accuracy evaluation ABNF W3C EBNF dialect" found no comparison of dialects.
- The ISO 14977 form `3 * x` stays unchecked. The PDF of the standard came back as compressed binary. The pages of Wheeler and of [Kuhn](https://www.cl.cam.ac.uk/~mgk25/iso-ebnf.html) do not cover it.
- The Lark reference does not say what an undefined rule name does. The GBNF README documents no error format.

## Sources

- https://dwheeler.com/essays/dont-use-iso-14977-ebnf.html
- https://www.cl.cam.ac.uk/~mgk25/iso-ebnf.html
- https://www.w3.org/TR/xml/#sec-notation
- https://www.w3.org/TR/xml/#sec-element-content
- https://www.w3.org/TR/xml/#determinism
- https://www.w3.org/TR/NOTE-sgml-xml-971215
- https://www.w3.org/XML/9712-reports.html
- https://www.w3.org/TR/jsgf/
- https://www.rfc-editor.org/rfc/rfc5234
- https://errata.rfc-editor.org/rfc5234
- https://www.rfc-editor.org/rfc/rfc7405
- https://www.rfc-editor.org/rfc/rfc3986#section-3.2.2
- https://relaxng.org/spec-20011203.html
- https://www.w3.org/TR/xmlschema11-1/
- https://lists.w3.org/Archives/Public/xmlschema-dev/2002Mar/0203.html
- https://lists.w3.org/Archives/Public/xmlschema-dev/2002Mar/0207.html
- https://lists.w3.org/Archives/Public/www-xml-schema-comments/2007AprJun/0025.html
- https://github.com/natresnet/buildingregistry-schema-v2/issues/18
- https://docs.oracle.com/en/java/javase/17/security/java-api-xml-processing-jaxp-security-guide.html
- https://pkg.go.dev/regexp/syntax
- https://arxiv.org/html/1205.1877v1
- https://github.com/lark-parser/lark
- https://lark-parser.readthedocs.io/en/latest/grammar.html
- https://lark-parser.readthedocs.io/en/latest/tree_construction.html
- https://github.com/lark-parser/lark/releases/tag/1.0.0
- https://github.com/ros2/rosidl/pull/664
- https://github.com/tree-sitter/tree-sitter
- https://tree-sitter.github.io/tree-sitter/creating-parsers/2-the-grammar-dsl.html
- https://tree-sitter.github.io/tree-sitter/creating-parsers/3-writing-the-grammar.html
- https://tree-sitter.github.io/tree-sitter/using-parsers/6-static-node-types.html
- https://github.com/tree-sitter/tree-sitter/pull/6054
- https://github.com/tree-sitter/tree-sitter/issues/4598
- https://github.com/tree-sitter/tree-sitter/issues/2611
- https://github.com/github/semantic/blob/main/docs/codegen.md
- https://github.com/pest-parser/pest
- https://pest.rs/book/grammars/syntax.html
- https://github.com/pest-parser/pest/issues/520
- https://github.com/antlr/antlr4
- https://github.com/antlr/antlr4/blob/master/doc/parser-rules.md
- https://github.com/antlr/antlr4/issues/195
- https://github.com/antlr/antlr4/releases/tag/4.10
- https://www.antlr.org/api/JavaTool/org/antlr/v4/tool/ErrorType.html
- https://pragprog.com/cms/errata/tpantlr2-errata/
- https://github.com/ohmjs/ohm
- https://ohmjs.org/docs/syntax-reference
- https://ohmjs.org/docs/api-reference
- https://github.com/ohmjs/ohm/blob/main/packages/ohm-js/CHANGELOG.md
- https://github.com/usebruno/bruno/issues/9025
- https://github.com/Engelberg/instaparse
- https://github.com/ggml-org/llama.cpp/blob/master/grammars/README.md
- https://github.com/ggml-org/llama.cpp/issues/29462
- https://github.com/guidance-ai/llguidance
- https://github.com/guidance-ai/llguidance/blob/main/docs/syntax.md
- https://xgrammar.mlc.ai/docs/latest/start/quick_start.html
- https://github.com/mlc-ai/xgrammar
- https://kdl.dev/
- https://github.com/kdl-org/kdl/blob/main/CHANGELOG.md
- https://github.com/kdl-org/kdl/blob/main/SCHEMA-SPEC.md
- https://github.com/kdl-org/kdl/blob/main/JSON-IN-KDL.md
- https://github.com/zellij-org/zellij/pull/3908
- https://news.ycombinator.com/item?id=48982011
- https://arxiv.org/abs/2505.20139
- https://arxiv.org/abs/2305.19234
- https://arxiv.org/abs/2403.01632

## Decision records

These decision records cite this survey:

- `docs/adr/regex-style-ebnf-over-whole-symbols.md`: the notation uses postfix operators as regular expressions do, over whole symbols written as quoted strings.
- `docs/adr/documentation-strings-in-rules.md`: a string in each rule documents it, in place of the doc comments of pest.
- `docs/adr/kdl-for-derived-artifacts.md`: the core writes every derived artifact as KDL 2.0.
