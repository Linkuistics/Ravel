# grammar-notation

## Problem

People and LLMs write the grammars of the workbench by hand. The core needs one exact notation, so that it can parse, check and trace each grammar. LLMs also read grammars and follow them, for example when an LLM creates a tutorial. The notation must therefore use forms that LLMs know, and it must document each rule in words. A misspelt rule name must fail, and a later operator for steps in any order must not break a valid grammar.

The decision record `regex-style-ebnf-over-whole-symbols` sets the frame. The notation is EBNF with postfix operators over whole symbols, and a semicolon ends each rule. A symbol is a quoted string, and a bare name refers to a rule. The record `documentation-strings-in-rules` puts the documentation into the rules. This spec gives the exact syntax, the start rule, the bound cap, the documentation and the diagnostics.

## Solution

A grammar file holds the documentation of the grammar, one start declaration and the rules.

```text
%doc "A tutorial in three parts." ;

%start tutorial ;

tutorial "The whole tutorial."
    = install+ "configure"? lesson{1,5} ;

install "Tell the reader how to install one tool."
    = "install" ;

lesson "Teach one concept with an example."
    = "introduce_concept" "show_example" ;
```

- `%doc` documents the grammar, and `%start` names the start rule.
- A rule is a name, an optional doc string, `=`, an expression and `;`.
- In an expression, a quoted string is a symbol, and a bare name refers to a rule.
- White space separates the items of a sequence, `|` separates alternatives, and parentheses make a group.
- The postfix operators `?`, `*` and `+`, and bounds such as `{1,5}`, follow an item.
- `//` starts a plain comment.

The example suite holds three grammars in the notation. They are a URG move grammar, a tutorial step model and an order-approval process with recursive escalation. Each grammar comes with sequences that it accepts and sequences that it rejects.

## Decisions

### Text and tokens

- A grammar is UTF-8 text. Spaces, tabs and line breaks separate tokens, and they have no other meaning. Outside strings, a line break is LF or CR LF.
- `//` starts a plain comment, which ends at the end of the line. A comment that starts with `///` or `//!` is a slip, because the notation documents with strings. As in Rust, a comment that starts with `////` is a plain comment.
- A name starts with an ASCII letter. The other characters of a name are ASCII letters, digits and `_`. Names are case-sensitive.
- A string follows the string syntax of KDL 2.0 (https://kdl.dev/spec/), with no exception. The syntax has two forms: a quoted string with escapes, and a multi-line string in `"""`. Each form has a raw variant without escapes. A raw string opens with one or more `#` and closes with as many, such as `#"C:\path"#` or `##"x"##`.
- The notation does not take the identifier strings of KDL 2.0, because a bare name refers to a rule.
- Inside a string, the line breaks and the white space are those of KDL 2.0. So CR, U+2028 and the other KDL 2.0 newlines are line breaks there.
- A quoted string cannot hold a literal line break, except in a whitespace escape. That escape is `\` followed by white space and line breaks, and the value drops all of it. A multi-line string can hold line breaks, and the core removes the indentation of its closing line from each line, as KDL 2.0 states.
- A directive is `%` and a name, with no space between them. The notation has two directives: `%doc` and `%start`.
- A number is a run of decimal digits, so `02` is one number. A number has no sign and no leading zero, but `0` alone is a number. In the slice, numbers occur only in bounds.
- The punctuation is `=`, `;`, `|`, `(`, `)`, `?`, `*`, `+`, `{`, `}` and `,`.
- One or more `#` followed by `"` start a raw string. Any other character outside the notation, a comment or a string is a syntax error.

### Statements

| Statement | Form | Rule |
|---|---|---|
| Grammar documentation | `%doc string ;` | Each grammar has one at most. |
| Start declaration | `%start name ;` | Each grammar has exactly one. The name must have a rule. |
| Rule | `name string = expression ;` | The string is optional. A name has one rule at most. |

- The order of the statements has no meaning. The recommended order is `%doc`, `%start`, and then the rules from the start rule downwards.
- The recommended layout puts the rule name and its doc string on the first line, and `= expression ;` on the next lines, with an indent.
- Every reading is a tree of the start rule. The check for unused rules starts from the start rule.

### Symbols and rule names

- In an expression, a quoted string is a symbol, and a bare name refers to a rule.
- A symbol is the value of its string, after the escapes. `"submit_order"` and `#"submit_order"#` are the same symbol.
- A symbol can hold any text, such as `"states the problem"`. A symbol cannot be empty, and its value cannot hold a line break.
- The spelling of a symbol can hold a line break in a whitespace escape. So `"states \` and `the problem"` on two lines spell the symbol `states the problem`.
- A grammar does not declare its symbols. Its symbols are the strings in its rule bodies.
- A bare name without a rule is an error. So a misspelt rule name fails, and its message shows the quoted form, in case the author meant a symbol.
- A misspelt symbol is a new symbol, and no sequence matches it. The rule checks warn when two symbols differ only by case, spaces, `_` or `-`.

A step that needs documentation gets a rule of its own, such as `install "Tell the reader how to install one tool." = "install" ;`. The doc string of that rule documents the step. Each use of the rule is also a checked name, so a misspelt use fails. The rule adds one node above the symbol in each tree.

### Expressions

In the table, `x`, `y` and `z` stand for any items.

| Form | Meaning |
|---|---|
| `x y z` | A sequence: `x`, then `y`, then `z` |
| `x \| y` | A choice: `x` or `y` |
| `( … )` | A group |
| `x?` | An optional part: `x` or nothing |
| `x*` | A repetition of zero or more passes |
| `x+` | A repetition of one or more passes |
| `x{2,4}` | A repetition with a bound, here two to four passes |

- An item is a symbol, a rule name or a group.
- The postfix operators bind most tightly, then sequence, then choice. So `x y* | z` means `(x (y*)) | z`, as in regular expressions and W3C EBNF.
- An item takes one postfix operator at most. `x*?` and `x+{2}` are syntax errors. Perl and Go also reject a second operator (https://github.com/golang/go/blob/7bd807271cf4/src/regexp/syntax/parse.go). Parentheses give the second operator, as in `(x{2})*`.
- Each alternative, each group and each rule body holds at least one item. The notation has no empty alternative, so an optional part uses `?`.
- A choice has no order. Each alternative that matches gives its own reading, as `one-earley-tracer` states.
- A choice and an optional part make no tree node. `*`, `+` and bounds make repetitions, and a repetition makes a loop node with one child for each pass. The record `trees-follow-rules-and-passes` holds this decision.

### Bounds

| Bound | Passes | Condition |
|---|---|---|
| `{n}` | Exactly n | n is 2 or more |
| `{m,n}` | From m to n | m is less than n, and n is 2 or more |
| `{m,}` | m or more | m is 2 or more |

Each meaning has one spelling. The parser rejects a second spelling, and its message names the first.

| Written | Message names |
|---|---|
| `x{0,1}` | `x?` |
| `x{0,}` | `x*` |
| `x{1,}` | `x+` |
| `x{n,n}`, with n of 2 or more | `x{n}` |
| `x{1}` or `x{1,1}` | `x` |
| `x{0}` or `x{0,0}` | The item matches nothing, so remove it |

White space can occur inside the braces.

### The bound cap

The bound cap is 100. Inside one rule, the count of the bounds around one item is 100 at most. The count multiplies the greatest number of each bound that encloses the item. An open bound `{m,}` counts as m. `*` and `+` count as 1, because an expansion needs no copies for them.

The count is the number of copies of the item that an expansion needs. It does not limit passes: `("x"{50})*` allows any number of passes of `"x"`.

| Expression | Count for `"x"` | Result |
|---|---|---|
| `"x"{2,4}` | 4 | Accepted |
| `("x"{50})*` | 50 | Accepted |
| `("x"{10}){10}` | 100 | Accepted |
| `"x"{101}` | 101 | Error |
| `("x"{20}){10}` | 200 | Error |

A reference starts a new count. An expansion copies a reference, and the rule behind the reference stays one rule.

The evidence for the cap:

- Each engine that expands a bound into copies caps the count or warns. Go rejects a count above 1000 (https://pkg.go.dev/regexp/syntax). Its parser also caps the product of nested repetitions at 1000 copies of the innermost item, in the function `repeatIsValid` of the source above.
- The JDK caps an XML content model at 5000 nodes. Lark marks wide ranges as "not recommended". A huge minimum in GBNF "OOM-kills the server". The grammar-notations survey cites each source.
- A rule diagram draws a bound as a loop marker with its numbers, not as copies. A tracer can count passes without copies. Two consumers can still expand a bound. One is a simple reference recogniser for the property tests. The other is a later export to a constrained decoder such as GBNF.
- A step grammar counts the steps that a person or an LLM performs. The three example grammars use bounds of 3 at most. A bound above 100 more probably means "many", and `*` or `+` states that better.

The cap is 100 and not the 1000 of Go, for two reasons. First, 100 copies keep each expansion small. Second, a later version can raise the cap without a break, because every grammar under the old cap stays valid. A lower cap later would break grammars.

### Reserved syntax

- `&` is reserved for the later operator for steps in any order. The parser reports it with its own message.
- Strings and numbers in other places are reserved for values in later constructs, such as file paths in attribute declarations. Until then, such a string or number is an unexpected token, unless a syntax error names it, as for `"a" = "x" ;`.
- Every other character outside the notation is a syntax error. So a later version can give a character a meaning without a break. A later version can also add directives.

### Documentation

- `%doc` documents the grammar. The string between a rule name and its `=` documents the rule.
- A doc string uses any form of string, raw or not. A multi-line string suits a long text.
- The core does not interpret the text, so the text can use Markdown.
- The checked grammar carries the documentation of the grammar and of each rule. It keeps each text with the rule that it documents.
- The checked grammar does not carry comments. They stay in the grammar text and in its lossless tree.
- Doc strings are optional. A prescriptive grammar that guides an LLM documents each step through a rule of its own.

The documentation tells an LLM what to produce. Write each kind as the table states.

| Documentation | Contents |
|---|---|
| The grammar | The purpose of the grammar, whether it describes or prescribes, and the attributes that steps carry |
| A rule | What the part means, and the reason for each bound |
| A rule for one step | What the step is, in the words of the person or the LLM that performs it, and its attributes |

### Forms that LLMs know

The notation takes each form from a notation that LLMs see often:

- The postfix operators, the bounds, `|` and parentheses come from regular expressions and GBNF.
- Quoted symbols come from ISO 14977, W3C EBNF and GBNF. The string syntax comes from KDL 2.0, the format of the trace files.
- The precedence comes from W3C EBNF. `=` and the `;` at the end of a rule come from ISO 14977 and JSGF.
- The directives follow `%start` in Bison.
- The doc strings follow the documentation strings of Common Lisp and Python. The comments follow C, Rust and KDL.

URG gives evidence for the postfix forms. Its pattern discovery asked an LLM for grammar fragments of the form `Name -> Part Part? Part*`. The LLM wrote 73 distinct fragments, such as `Fix -> FailingCase Principle? Remedy`. It also used `+` and, in 4 fragments, groups in parentheses, which the prompt did not name.

An LLM that knows another notation makes predictable slips. The parser and the rule checks recognise each slip below, and the message names the form of this notation.

| Slip | Habit from | Message names |
|---|---|---|
| `::=`, `->` or `:` after a rule name | GBNF, W3C EBNF, BNF, Lark | `=` |
| `'x'` | ISO 14977, Lark | `"x"` |
| A bare name with no rule | Grammars with declared or upper-case terminals | `"x"`, in case the author meant a symbol |
| `///` or `//!` | Rust, pest | A doc string after the rule name, or `%doc` |
| `[x]` | ISO 14977, JSGF | `x?` |
| `{x}`, braces that open with a name, a string or `(` | ISO 14977 | `x*` |
| `x, y` | ISO 14977 | White space between items |
| `#` that does not start a raw string, `/*` or `(*` | GBNF, C, ISO 14977 | `//` |
| `x*?` or `x+?` | The lazy operators of regular expressions | One operator for each item |
| `&` | The SGML connector | The reserved operator |

The rows for `x*?` and for `&` are not separate slips. They are the syntax errors for a second operator and for the reserved operator, in the section "Diagnostics".

### The syntax of the notation

The grammar below defines the syntax in the notation itself. Each step of its sequences is one token. Punctuation, `%doc` and `%start` stand for themselves. `"name"`, `"string"` and `"number"` stand for any token of that kind. White space and comments are not tokens here.

```text
%doc """
    The syntax of the grammar notation, over its tokens.
    Punctuation and the directives "%doc" and "%start" stand for themselves.
    "name", "string" and "number" stand for any token of that kind.
    White space and comments are not tokens here.
    """ ;

%start grammar ;

grammar "A grammar is a series of statements."
    = statement+ ;

statement "Each statement ends with a semicolon."
    = doc_declaration | start_declaration | rule ;

doc_declaration "Documents the grammar, as in `%doc \"Order approval.\" ;`."
    = "%doc" "string" ";" ;

start_declaration "Names the start rule, as in `%start order ;`."
    = "%start" "name" ";" ;

rule "Defines a rule, with an optional doc string before the `=`."
    = "name" "string"? "=" choice ";" ;

choice "Alternatives, separated by `|`."
    = sequence ("|" sequence)* ;

sequence "Items in order, separated by white space."
    = item+ ;

item "A rule name, a symbol or a group, with at most one operator."
    = ("name" | "string" | group) operator? ;

group "A group in parentheses."
    = "(" choice ")" ;

operator "The postfix operators."
    = "?" | "*" | "+" | bound ;

bound "A bound: `{n}`, `{m,n}` or `{m,}`."
    = "{" "number" ("," "number"?)? "}" ;
```

The grammar does not state these constraints. A grammar has one `%doc` at most and exactly one `%start`. A symbol is not empty, and its value holds no line break. A bound meets the conditions of the section "Bounds". The section "Text and tokens" states the form of each token, and the section "Symbols and rule names" states the constraints on names.

### Diagnostics

Each diagnostic has a severity, a message and a span. A grammar with an error has no traces. A warning leaves the grammar usable.

The parser reports these syntax errors:

| Syntax error | Example |
|---|---|
| A character outside the notation | `a = "x" ! ;` |
| A slip from another notation, from the table above | `a ::= "x" ;` |
| The reserved operator `&` | `a = "x" & "y" ;` |
| An unknown directive | `%strat a ;` |
| A string without its closing quote | `%doc """` and no closing `"""` |
| A literal line break in a quoted string, outside a whitespace escape | `"x` and `y"` on two lines |
| An unknown or invalid escape in a string | `"x\q"`, `"\u{D800}"` |
| An empty symbol, or a symbol whose value holds a line break | `a = "" ;`, `a = "x\ny" ;` |
| A string as a rule name or as the start rule | `"a" = "x" ;` |
| A second doc string on one rule | `a "d" "e" = "x" ;` |
| A missing `;` | `a = "x" b = "y" ;` |
| A missing `=` | `a "x" ;` |
| An empty alternative, group or rule body | `a = "x" \| ;` |
| A group without its `)` | `a = ("x" "y" ;` |
| A second operator on one item | `a = "x"*? ;` |
| A malformed bound | `"x"{2,`, `"x"{,3}`, `"x"{}` |
| A second spelling of a bound | `"x"{0,1}`, `"x"{2,2}` |
| A least number above the greatest | `"x"{4,2}` |
| A number with a leading zero | `"x"{02}` |

Two general syntax errors cover every other fault.

| General syntax error | Example |
|---|---|
| An unexpected token: a token where the syntax allows no such token, or the end of the file where the syntax needs a token. The message names the tokens that can come at that point | `%doc ;`, `a = "x" ) ;` |
| A malformed string: a string that breaks the string syntax of KDL 2.0 in another way | `a """doc""" = "x" ;` |

These rules choose one syntax error for one fault:

- A row of the first table comes before a general syntax error. So `a = "x" b = "y" ;` is a missing `;`, not an unexpected `=`.
- A slip and the reserved operator come before a character outside the notation. So `a ::= "x" ;` is a slip.
- Braces that open with a name, a string or `(` are the ISO slip, wherever they stand. So `"x"{a}` is a slip, and its message names `a*`.
- Other braces after an item are a bound. A bound that does not have the form `{n}`, `{m,n}` or `{m,}` is a malformed bound.
- A quoted string ends at its first literal line break outside a whitespace escape, and the parser reports that line break. A string without its closing quote reaches the end of the file.

The rule checks report these diagnostics:

| Check diagnostic | Severity |
|---|---|
| More than one `%doc` | Error |
| No `%start`, or more than one | Error |
| A `%start` that names a name without a rule | Error |
| A bare name without a rule | Error |
| A rule that is defined twice | Error |
| A repetition whose body can match the empty sequence | Error |
| A rule that can derive itself with no step, as in `a = b ; b = a \| "x" ;` | Error |
| A bound above the bound cap | Error |
| A rule that the start rule does not reach | Warning |
| Two symbols that differ only by case, spaces, `_` or `-` | Warning |
| Two rule names that differ only by case | Warning |

The badges of the groups of mutually recursive rules are not diagnostics. The rule checks also own them.

### Files

A grammar file has the extension `.ebnf`. Each derived artifact is a KDL file.

## Test seams

The work tests through the core interface, the first seam that the human agreed on 2026-11-17. A test case gives a grammar file and compares the KDL output with an expected file.

- Each syntax error and each check diagnostic has a test grammar that triggers it, with its span.
- Each check diagnostic also has a test grammar that does not trigger it.
- Each rule that chooses one syntax error for one fault has a test grammar, such as `"x"{a}` for the brace slip.
- The string tests of the KDL 2.0 test suite can serve as cases for strings.
- The string cases include a raw string with two `#`, such as `##"x"##`, and a symbol whose spelling breaks a line in a whitespace escape.
- The three example grammars and their sequences are fixtures at the same seam.

The notation adds no seam.

## Out of scope

- The operator for steps in any order. The notation reserves `&` for it, and it comes after the slice.
- Rule names outside ASCII, and character classes. The human wrote: "Lexical sophistication can come later if required."
- Strings and numbers as values, and declarations of attributes. The record `symbols-decide-acceptance` keeps attribute declarations for later.
- Imports and includes. One grammar is one file.
- Block comments, a leading `|`, ordered choice, the `=/` of ABNF and hidden rules. Each adds a second form or a second meaning for a small gain.
- An explicit empty sequence. `?` and `*` cover each use of it.
