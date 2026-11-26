# Grammar visualisers: survey for Topic 1

Status: revision 5, 2026-11-17. Lines and table cells that start with "Gap" name work that this survey did not cover.

Scope: syntax-diagram generators, parser playgrounds with tracing, automaton simulators and workflow state-machine tools. Decisions in force: a Rust core, a TypeScript and D3 workbench, EBNF with bounded repeats and whole-symbol terminals, and KDL artifacts.

The human wants two views of a run. The grammar view shows the current position, the next options and the current pass of a loop. Rule groups without a stack draw as state machines, and references that need a stack draw as call boxes. The output view shows a partial parse tree, with every completed pass under its repetition. Both views show step attributes, such as an attached file. Railroad findings stay as precedent for a layout that follows expression structure.

## Findings that change decisions

- Q4: two grammar tools show a parse position on a syntax diagram. They are `FOAM-GrammaticVisualizer` from 2026 (<https://github.com/foam-foundation/foam3/pull/5607>) and ANTLRWorks 1 from 2007 (<https://www.antlr.org/papers/antlrworks-draft.pdf>). The mature generators `railroad-diagrams`, RR and librrd have no trace overlay.
- Q4: the next options come from the state of the recogniser, at very different costs. Lark offers `accepts()` for its LALR parser only (<https://lark-parser.readthedocs.io/en/latest/classes.html>). antlr4-c3 reports "millions of visited ATN states for complex input" (<https://github.com/mike-lischke/antlr4-c3>). An Earley parser reads the next terminals from its last state set (<https://pkg.go.dev/sigs.k8s.io/instrumentation-tools/promq/autocomplete/earley>). Inference: an Earley-style tracer gives the next options without a second search.
- Q3: no maintained playground in this survey draws a partial parse tree for a prefix of the input. The ANTLRWorks 1 debugger did, and its trees grew and shrank as the user stepped forward and back (<https://www.antlr.org/papers/antlrworks-draft.pdf>). Inference: the output view revives a stopped feature.
- Q3: parse trees in this survey flatten a repetition or split it by position, never by pass. Lark (<https://lark-parser.readthedocs.io/en/latest/tree_construction.html>) and tree-sitter (<https://github.com/tree-sitter/tree-sitter/issues/223>) flatten it. ANTLR gives a loop no node (inference), and Ohm splits it by expression (inference). Inference: a pass node needs its own node type in the KDL tree.
- Q4: among the tools in this survey, only Python tracebacks fold repeated passes with a count (<https://docs.python.org/3/library/traceback.html>). Flame graphs merge identical stacks but give up time order (<https://www.brendangregg.com/flamegraphs.html>). JFLAP and Stately Studio list every step. Inference: a fold with a count suits the output view after 20 passes.
- Q4: two tools give step back from a record of the run. They are ANTLRWorks 1 (<https://www.antlr.org/papers/antlrworks-draft.pdf>) and Stately Studio (<https://stately.ai/docs/simulate-mode>). JFLAP has only Reset. Inference: a trace stored as KDL serves step back, export and test fixtures.
- Q4: the playgrounds link tree, grammar and input. Ohm highlights the consumed text and the rule definition for the node under the pointer (<https://ohmjs.org/pubs/live2016/>). This three-way link is the fallback to beat. No grammar tool in this survey shows links to resources on tree nodes.
- Q3: tools handle several trees for one input in five ways. Lark nests derivations under `_ambig` nodes, and nearley returns an array. The ANTLR IntelliJ plugin draws competing trees in a dialog, and Bison prints two text derivations. Ohm and tree-sitter show one tree only.
- Q4: JFLAP building blocks are the nearest precedent for call boxes with drill-down (<https://www.jflap.org/tutorial/turing/buildingblocks/buildingblocks.htm>). They do not support nondeterminism (<https://www.jflap.org/tutorial/turing/changes7.1/index.html>). The search found no maintained visualiser for recursive transition networks.
- Q2 and Q5: railroad diagrams failed in real use when diagram and grammar lived apart, as at SQLite (<https://arxiv.org/html/2509.15834v2>) and afw (<https://github.com/afw-org/afw/issues/495>). Only `FOAM-GrammaticVisualizer` shows both bounds of a repeat. Inference: draw only from the checked grammar.

## Q1: Purpose

Comparable tools do four jobs for four kinds of reader. Each job has its own failure.

- Railroad generators serve the readers of a specification. RR draws diagrams for W3C-style grammars (<https://github.com/GuntherRademacher/rr>). The failure is drift between drawing and grammar, as in SQLite (<https://arxiv.org/html/2509.15834v2>).
- Parse playgrounds serve the author of a grammar. Dubroy built the Ohm visualiser so that authors stop "playing computer" (<https://dubroy.com/blog/visualizing-packrat-parsing/>). The failure is fragility. ANTLR Lab cannot draw trees for most grammars-v4 grammars, "likely because the server crashes during the parse" (<https://github.com/antlr/antlr4-lab/issues/11>).
- Automaton simulators serve students. JFLAP steps an input through an automaton and lists each live configuration (<https://www.jflap.org/tutorial/fa/createfa/fa.html>). The failure is volume, because the list grows fast for nondeterministic machines.
- Workflow tools serve the designers of an application. Stately Studio simulates an XState machine on its canvas, but not when the machine "contains errors" (<https://stately.ai/docs/simulate-mode>). Stately Sketch turns pasted machine code into a diagram that simulates (<https://stately.ai/blog/2026-03-26-introducing-stately-sketch>).

Teaching tools with syntax diagrams meet resistance. SyntaxTrain (2011) shows syntax diagrams beside Java syntax errors. A 2019 tool left out such diagrams "as the students have not yet learned how to interpret them" (<https://juholeinonen.com/assets/pdf/leinonen2019exploring.pdf>).

Inference: the URG use is closest to the grammar-author job. An author tests a grammar fragment against example episodes. Fallback to beat: the Ohm editor, where an author edits a grammar beside example inputs (<https://github.com/ohmjs/ohm-editor>).

## Q2: Notation

`railroad-diagrams` reads no grammar text. Authors write nested constructor calls, such as `Diagram(Sequence(...))` (<https://github.com/tabatkins/railroad-diagrams/blob/gh-pages/README-js.md>). RR reads W3C-style EBNF, and ebnf-convert translates other notations. RR 2.2 added separators to repetition operators (<https://github.com/GuntherRademacher/rr/releases>). Chevrotain draws from a serialised grammar tree, so any front end can feed it (<https://chevrotain.io/docs/guide/generating_syntax_diagrams.html>).

Section 6 of the XML 1.0 specification defines the W3C notation (<https://www.w3.org/TR/xml/>). Its only repeats are `A?`, `A+` and `A*`. It has no counted form such as `A{n,m}`. Inference: RR cannot read `x{2,4}` without an extension or a rewrite into plain sequences.

`FOAM-GrammaticVisualizer` draws `foam.parse` combinator objects. A table, NOTATION.md, maps each parser class to a glyph. Its repeat badges show both bounds, such as "x4, x1-2, x3+" (<https://github.com/foam-foundation/foam3/pull/5607>).

Fallback to beat: W3C-style EBNF as RR reads it. URG adds the bounded repeat, which no railroad input format in this survey has.

## Q3: Semantics

### Several trees for one input

The ANTLR IntelliJ preview commits to one parse tree. Its profiler marks the ambiguous decisions. On request, a dialog draws the competing trees and outlines the nodes that differ from the chosen tree. The dialog refuses input with syntax errors: `Cannot display ambiguous trees while there are syntax errors in your input.` (<https://github.com/antlr/intellij-plugin-v4/blob/master/src/main/java/org/antlr/intellij/plugin/preview/ShowAmbigTreesDialog.java>).

Lark returns all derivations under `_ambig` nodes when its Earley parser runs with `ambiguity="explicit"` (<https://lark-parser.readthedocs.io/en/latest/classes.html>). A recipe expands such a tree into a list of trees, and the documentation warns of "an exponential explosion" (<https://lark-parser.readthedocs.io/en/stable/recipes.html>). A third-party package counts the derivations and iterates over them lazily (<https://github.com/chanicpanic/lark-ambig-tools>). Lark cannot detect ambiguity inside a regular-expression terminal (<https://lark-parser.readthedocs.io/en/latest/grammar.html>). Inference: whole-symbol terminals avoid that blind spot.

nearley returns `parser.results`, an array of every parse "in a deterministic order" (<https://github.com/kach/nearley/blob/master/docs/md/parser.md>). More than one result signals ambiguity. The npm page warns that "parsing ambiguous grammars is inefficient" (<https://www.npmjs.com/package/nearley/v/2.10.4>). The playground source returns 404.

Bison prints counterexamples to conflicts as text trees when it runs with `-Wcounterexamples` (<https://www.gnu.org/software/bison/manual/html_node/Counterexamples.html>). It prints one example and then two derivations, such as "Shift derivation" and "Reduce derivation". A bullet `•` marks the conflict point. Inference: this form shows two trees for one input in little space, and it can serve as a test fixture.

Ohm and `FOAM-GrammaticVisualizer` use PEG semantics, so each input has at most one tree. The tree-sitter playground shows one tree. The vscode-antlr4 debugger picks the first of several lookahead candidates, because VS Code allows one position per stack frame (<https://github.com/mike-lischke/vscode-antlr4/blob/master/doc/grammar-debugging.md>). Inference: no tool in this survey draws a shared packed parse forest as a graph.

### Repetition in parse trees

Lark flattens a repetition: "Using `item+` or `item*` will result in a list of items, equivalent to writing `item item item ..`" (<https://lark-parser.readthedocs.io/en/latest/tree_construction.html>). The page has no example for a group such as `(a b)*`. Inference: such a group splices `a b a b` into the parent.

tree-sitter also flattens. An issue describes the children of a node as "an unstructured sequence", with nested sequences and hidden rules "flattened into a single sequence" (<https://github.com/tree-sitter/tree-sitter/issues/223>). Inside, `repeat` builds "a balanced binary tree" of hidden nodes, which the tree view does not show (<https://github.com/tree-sitter/tree-sitter/discussions/1543>). The maintainer advises `repeat` over recursion for long lists. Recursion gives "a leftward or rightward leaning linked list".

ANTLR gives each rule a context class, and its documentation shows no class for a subrule such as `(x|y|z)*` (<https://github.com/antlr/antlr4/blob/master/doc/parser-rules.md>). A rule that appears more than once gets a list accessor, such as `List<EContext> e()`. Inference: a loop makes no node of its own, and its matches are flat children of the rule node.

Ohm passes "iteration nodes, which are associated with expressions inside a repetition operator" to semantic actions (<https://ohmjs.org/docs/api-reference>). Each expression "produces a fixed number of values". Inference: `(a b)*` gives two iteration nodes, one with all `a` matches and one with all `b` matches. The pages read in this survey do not state this case.

Inference: no tool in this survey groups a repetition by pass. The URG output view needs a pass node under each repetition. Fallback to beat: the flat list of Lark and tree-sitter.

### Partial trees for a prefix

The JFLAP LL(1) parser starts with an empty tree panel and shows the input that remains (<https://www.jflap.org/tutorial/grammar/LL/index.html>). A red row in the parse table marks the production about to replace the variable. The tutorial does not describe how the tree grows.

The ANTLRWorks 1 debugger drew the parse tree and the AST as the parser built them (<https://www.antlr.org/papers/antlrworks-draft.pdf>). "ASTs grow and shrink as the developer steps forward and backwards in the parse." A step back "partially deconstructs parse trees and ASTs".

The vscode-antlr4 debugger changes its parse tree at each step. A left-recursive rule shows no change until the parser leaves it (<https://github.com/mike-lischke/vscode-antlr4/blob/master/doc/grammar-debugging.md>).

tree-sitter recovers from errors with inserted nodes. Its playground labels them `MISSING identifier` or `MISSING ";"`, and it gives the links of `ERROR` and `MISSING` rows the class `error` (<https://raw.githubusercontent.com/tree-sitter/tree-sitter/master/docs/src/assets/js/playground.js>). Ohm draws no tree for a failed input: "When an input fails to match, it does not produce a parse tree" (<https://ohmjs.org/pubs/live2016/>).

The interactive parser of Lark takes one token at a time (<https://lark-parser.readthedocs.io/en/latest/classes.html>). Its documentation names no method that returns a tree for the prefix.

Fallback to beat: the ANTLRWorks 1 debugger, whose trees grew and shrank with each step. Inference: an Earley chart holds the open rules that a tree for a prefix needs.

### Failure

The "show failures" mode of Ohm draws failed expressions in red (<https://ohmjs.org/pubs/live2016/>). `FOAM-GrammaticVisualizer` reports how far a parse got. The ANTLR IntelliJ preview underlines errors in the input. The vscode-antlr4 debugger reports only an error count. JFLAP colours rejected configurations red (<https://www.jflap.org/tutorial/fa/createfa/fa.html>).

## Q4: Layout and interaction

### Railroad generators compared

| Generator | Wide rules | Long choices | Bounded repeats |
|---|---|---|---|
| `railroad-diagrams` | The author adds a `Stack` by hand. | `HorizontalChoice` places branches side by side. | No construct. `OneOrMore` takes a comment, such as a count. |
| RR | Greedy wrap at a set width, 992 pixels by default. | Branches stack vertically. Users cannot set their order. | No construct. Separators since release 2.2. |
| librrd | Automatic wrap to a target width, under user control. | Not covered in the sections read. | Kleene star only. |
| `FOAM-GrammaticVisualizer` | Fold and unfold rules, find, pan and zoom. | Branch numbers show PEG priority. | Badges show both bounds. |

`railroad-diagrams` guesses font metrics, because SVG "can't actually respond to the sizes of content" (<https://github.com/tabatkins/railroad-diagrams/blob/gh-pages/README-js.md>). Long text can overflow its box. RR factors rules and removes direct recursion, which gives smaller diagrams (<https://github.com/GuntherRademacher/rr>). RR does not keep the order of alternatives, and one user asks for it for teaching (<https://github.com/GuntherRademacher/rr/issues/62>).

librrd compiles a diagram into a layout and wraps it to a target width. Its authors write that "few other tools perform automatic wrapping, and none let the user control it" (<https://arxiv.org/html/2509.15834v2>). They optimise only the wrapping step, to avoid "the instability typical of optimization-based 2-dimensional layouts". librrd uses Scala and runs as a Node command and as a static web page (<https://github.com/epfl-systemf/librrd>). Inference: a Rust core would port its algorithm, not link its code. Fallback to beat for a layout of expression structure: librrd.

### Trace position on railroad and syntax diagrams

`FOAM-GrammaticVisualizer` draws "any `foam.parse` grammar as a railroad diagram and steps through a parse of sample text on it" (<https://github.com/foam-foundation/foam3/pull/5607>). Rules that the parse never reached fade. An input ribbon shows the consumed text, and a derivation panel shows the final path. The text does not say how it draws the current position or backtracking.

ANTLRWorks 1 drew syntax diagrams of the grammar rules (<https://www.antlr.org/papers/antlrworks-draft.pdf>). It drew the chosen path of a nondeterministic decision in thick green and the other path in dashed red. It could "single step through the syntax diagram along any nondeterminism path". The paper appeared in 2008 (<https://onlinelibrary.wiley.com/doi/abs/10.1002/spe.872>).

Debuggex draws a regular expression as a railroad diagram and moves a slider through a match (<https://www.debuggex.com/>). The railroad view of vscode-antlr4 follows "the rule under the caret", not the element (<https://github.com/mike-lischke/vscode-antlr4/blob/master/doc/graphical-visualizations.md>). The search found no other tool that draws a trace on a railroad diagram. "Silences" lists the queries.

### Current state, next options and history

JFLAP shades the current state of each configuration in the automaton (<https://www.jflap.org/tutorial/fa/createfa/fa.html>). Trace opens "the traceback of that configuration", with one entry after each input symbol. Reset clears all configurations, and no step back exists. Users freeze or remove configurations, because "the number of such configurations can become quite large" (<https://turing.plymouth.edu/~zshen/Webfiles/notes/CS378/JFLAPActivities.pdf>).

The vscode-antlr4 debugger stops at rule entry and rule exit. Its call stack is the rule invocation stack, and it has no step back (<https://github.com/mike-lischke/vscode-antlr4/blob/master/doc/grammar-debugging.md>).

regex101 has a step debugger for the PCRE family only (<https://regex101.com/>).

ANTLRWorks 1 recorded the parser as an event stream over a socket (<https://www.antlr.org/papers/antlrworks-draft.pdf>). It could "rewind and replay the parse multiple times by re-executing the events without having to restart the actual parser". The debugger could still move through the stream after the parser process ended. Users could save the stream as a text file. `FOAM-GrammaticVisualizer` has Step, Back, Run to end and a slider. Inference: it records the parse and replays it.

Stately Studio highlights in blue every event that the current state accepts (<https://stately.ai/docs/simulate-mode>). It logs each event with its target state, and a click on an entry returns the machine to that point. Stately Sketch highlights the current state, shows guard results and logs events (<https://stately.ai/blog/2026-03-26-introducing-stately-sketch>). Its announcement mentions no rewind.

Three libraries compute the next options of a grammar. Lark `accepts()` "Returns the set of possible tokens that will advance the parser into a new valid state" (<https://lark-parser.readthedocs.io/en/latest/classes.html>). It works only with the LALR parser. antlr4-c3 walks the ATN, "taking every path instead of using the prediction implementation" (<https://github.com/mike-lischke/antlr4-c3>). It returns candidate tokens with the tokens that always follow them, and candidate rules with their call stacks. A rule context makes the walk faster but less complete.

An Earley completer for PromQL takes each item of the last state set with a terminal after the dot (<https://pkg.go.dev/sigs.k8s.io/instrumentation-tools/promq/autocomplete/earley>). A 2024 paper states that each terminal the last chart can scan extends the prefix to a valid prefix (<https://arxiv.org/html/2402.17988>). It does not state the converse. Inference: with whole-symbol terminals, the next options form a short list of move labels.

Sources disagree on time. The Ohm authors write that, "Unlike a stepping debugger", their view "abstracts over time" (<https://ohmjs.org/pubs/live2016/>). ANTLRWorks 1, JFLAP and Stately step through time. Fallback to beat: the JFLAP configuration list with Trace, plus the event log of Stately Studio.

### Loops in trace and history views

Python tracebacks fold a run of identical frames (<https://docs.python.org/3/library/traceback.html>). They show "the first few repetitions, followed by a summary line stating the exact number of further repetitions". The documentation gives no cutoff. Inference: recursion makes such runs, so the fold also suits a right-recursive rule.

Flame graphs merge identical stacks. The x-axis is "sorted alphabetically (it is not the passage of time)", which "maximizes frame merging" (<https://www.brendangregg.com/flamegraphs.html>). Flame charts keep time order and merge less. Gregg wants tools to offer both. Inference: the output view faces the same trade between order and grouping.

JFLAP shows a loop pass by pass. The shaded state and the configuration tape show the current pass (<https://www.jflap.org/tutorial/fa/createfa/fa.html>). The traceback lists one entry per input symbol, without a count of passes. Ohm collapses any node on a click and marks it with a dark grey bottom border (<https://ohmjs.org/pubs/live2016/>).

Fallback to beat: the Python fold, with a few passes in full and then a count.

### Tree views: links, attributes and resources

Ohm draws the trace as a variant of an icicle plot (<https://ohmjs.org/pubs/live2016/>). Each node lines up with the text that it consumed. When the pointer is over a node, Ohm highlights that text and the rule definition in the grammar pane. The authors write that the view "does not scale well to larger inputs that span multiple lines".

The ANTLR IntelliJ preview links views with modifier keys (<https://github.com/antlr/intellij-plugin-v4>). Alt shows the input span of the rule under the pointer, and a click opens the rule definition. Each tree node shows the number of the chosen alternative. The preview uses the grammar interpreter, so actions and predicates do not run. In the ANTLRWorks 1 debugger, a click on a tree node highlighted its token in the input and the token reference in the grammar (<https://www.antlr.org/papers/antlrworks-draft.pdf>).

The tree-sitter playground prints each row as indent, field name, node type and range, such as `name: identifier [3, 2] - [3, 15]` (<https://raw.githubusercontent.com/tree-sitter/tree-sitter/master/docs/src/assets/js/playground.js>). Anonymous nodes appear only when a checkbox is on. A click on a row selects its code. A cursor move highlights the named node that `namedDescendantForPosition` returns for the selection. The tree then scrolls to that row.

ANTLR Lab shows a tree view and a hierarchy view next to the input (<http://lab.antlr.org/>). Open issues report that the tree ignores rule labels (<https://github.com/antlr/antlr4-lab/issues/86>) and draws characters from U+0080 upwards wrongly (<https://github.com/antlr/antlr4-lab/issues/68>). Lark prints a tree as indented text with `pretty()` (<https://lark-parser.readthedocs.io/en/latest/classes.html>).

AST explorers show every property of a node, with checkboxes that hide noise. The Babel AST Explorer offers "Hide empty keys", "Hide location data", "Hide type keys" and "Hide comments" (<https://lihautan.com/babel-ast-explorer/>).

No grammar tool in this survey shows links to resources on tree nodes. Graph tools do. Graphviz attaches a `URL` to nodes, edges and clusters in SVG output and image maps (<https://graphviz.org/docs/attrs/URL/>). Inference: an attached file can show as a link on its node, with the other attributes in a side panel.

Fallback to beat: the three-way link of Ohm, plus the attribute panel of the AST explorers.

### Mixed state-machine and call views

The vscode-antlr4 extension draws one ATN graph per rule and a separate call graph (<https://github.com/mike-lischke/vscode-antlr4/blob/master/doc/graphical-visualizations.md>). Hover marks callers in red and callees in green.

JFLAP lets any state of a Turing machine hold a nested machine, called a building block (<https://www.jflap.org/tutorial/turing/buildingblocks/buildingblocks.htm>). Edit Block opens the nested machine in a new tab. A run "only shows what block the configurations are in". Focus follows one configuration "through the lowest level" until Defocus. Step By Building Block runs a whole block per Step.

The building-block window of JFLAP 7.1 does not support nondeterminism (<https://www.jflap.org/tutorial/turing/changes7.1/index.html>). Inference: JFLAP keeps drill-down and the parallel frontier apart. The URG tracer needs both, because one input can be inside several call boxes at once.

Unitex/GramLab draws local grammars as graphs, and a box can call a subgraph (<https://unitexgramlab.org/releases/3.2/man/Unitex-GramLab-3.2-usermanual-en.pdf>). A right click on a call opens the subgraph. Its debug mode shows the boxes that match each token of a found sequence (<https://unitexgramlab.org/releases/3.0/man/Unitex-GramLab-3.0-usermanual-en.pdf>). This survey read search excerpts of the manuals only.

On the diagram page of Chevrotain, a click on a non-terminal scrolls to its definition (<https://chevrotain.io/docs/guide/generating_syntax_diagrams.html>). Stately Sketch draws nested XState machines, but its README describes no drill-down (<https://github.com/statelyai/sketch>). A 1991 paper by Perlin mentions an interactive graphical tool for recursive transition networks (<https://aclanthology.org/P91-1013.pdf>).

Fallback to beat: JFLAP building blocks with Focus.

### Large diagrams

The vscode-antlr4 ATN graph uses force-directed layout, which "is rarely satisfying" (<https://github.com/mike-lischke/vscode-antlr4/blob/master/doc/graphical-visualizations.md>). Users drag nodes by hand. In 2021, a grammar translated from Bison produced an ATN of 60,948 states and 68,285 edges. That broke the serialisation limit of 65,535 (<https://groups.google.com/g/antlr-discussion/c/U2uSnV6zqyo>).

Inference: a state group grows in nodes and edges, and a call tree grows in depth. A layered or hand-tuned layout suits the state groups, and drill-down with breadcrumbs handles depth.

## Q5: Scope

Features that projects dropped or regretted:

- The afw project replaced RR with a Python parser of about 90 lines and the `railroad-diagrams` renderer (<https://github.com/afw-org/afw/issues/495>). RR needed Java in five Docker images "for one step". RR took about 2.7 seconds for 198 diagrams, and the Python prototype about 25 milliseconds. Four grammar errors drew without a warning. The issue adds: "We can't change the look or the layout, and rr never warns about grammar mistakes".
- SQLite moved from a Tcl layout language to hand drawings in Pikchr in 2020. The hand drawings "sometimes introduced stylistic inconsistencies" and one semantic bug (<https://arxiv.org/html/2509.15834v2>).
- The demonstration page of `railroad-diagrams` carried a DOM cross-site scripting flaw, CVE-2024-26467 (<https://app.opencve.io/cve/?product=railroad-diagram_generator&vendor=tabatkins>).
- The diagram page of Chevrotain loads scripts from a CDN, pinned to the Chevrotain version. After an upgrade, a saved page still loaded the old scripts (<https://github.com/honeynet/ochi/pull/171>).

Requests that never arrived: three RR requests have no reply from the maintainer. They ask for drop-down diagrams that expand non-terminals in place (<https://github.com/GuntherRademacher/rr/issues/19>, 2022), production chunking (#61, 2025) and alternative order (#62, 2025). A request to save sessions in ANTLR Lab is open since June 2024 (<https://github.com/antlr/antlr4-lab/issues/90>).

Stopped or slowed development: Debuggex still calls itself a beta, with 2013 in its footer (<https://www.debuggex.com/>). The Lark IDE repository shows its last update in June 2023 (<https://github.com/lark-parser>). ANTLRWorks 1 belongs to ANTLR 3, and its home page says "sorry, I am not fixing bugs anymore" (<https://www.antlr3.org/works/>).

Fallback to beat: a recorded trace that replays, as in ANTLRWorks 1. Inference: the first slice should record the trace, because step back, export and tests all read the same record.

## Q6: Test seams

The Ohm editor runs Playwright tests with screenshot snapshots in CI (<https://github.com/ohmjs/ohm-editor>). A pull-request branch regenerates the snapshots, and a reviewer accepts them before merge.

`FOAM-GrammaticVisualizer` reports "318 passed, 0 failed" across "20 client test classes" (<https://github.com/foam-foundation/foam3/pull/5607>). One test "fails by design whenever FOAM gains a parser with no row in NOTATION.md". The test names separate the trace, the overlay and the SVG output.

ANTLRWorks 1 saved its event stream as text and proposed it for unit tests (<https://www.antlr.org/papers/antlrworks-draft.pdf>). The vscode-antlr4 extension tests its back end with Jest and has no visible interface tests. The RR repository shows no test folder.

tree-sitter stores tests as corpus files (<https://tree-sitter.github.io/tree-sitter/creating-parsers/5-writing-tests.html>). Each test holds a name, an input and the expected tree as an S-expression of named nodes. The command `tree-sitter test -u` rewrites every expected tree from the current output. The attribute `:error` checks that a parse contains an error, without the tree.

XState generates test paths from a machine (<https://stately.ai/docs/xstate/v6/model-based-testing>). `getShortestPaths` keeps the shortest path to each state, and `getSimplePaths` walks every simple path. Inference: the same method can generate example episodes from a state group.

librrd states that its layout output is equivalent to its input. Inference: that property can serve as a test oracle for any layout of expression structure.

Fallback to beat: tree-sitter corpus files with `-u`. Inference: a URG corpus can hold each episode with its expected tree in KDL, with pass nodes and folded counts.

## Systems examined

| Name | URL | Purpose | Licence | Latest release | Maintenance status | Walk-away result |
|---|---|---|---|---|---|---|
| `railroad-diagrams` (Tab Atkins) | <https://github.com/tabatkins/railroad-diagrams> | Railroad diagrams from code | MIT | No tagged release | 23 open issues and pull requests, last push May 2026 | Diagram code and SVG stay legible. |
| RR (Gunther Rademacher) | <https://github.com/GuntherRademacher/rr> | Railroad diagrams from W3C-style EBNF | Apache-2.0 | 2.7, 13 November 2026 | One maintainer, releases continue | EBNF source and SVG stay legible. |
| librrd (EPFL) | <https://github.com/epfl-systemf/librrd> | Automatic railroad layout | MIT | No releases, paper July 2026 | Research code, 6 stars, 1 open issue | SVG with classes and nesting, according to the paper. |
| Chevrotain diagrams | <https://chevrotain.io/docs/guide/generating_syntax_diagrams.html> | Railroad page for a parser | Apache-2.0 | 13.2.0, 1 August 2026 | Active | The page needs CDN scripts. |
| `FOAM-GrammaticVisualizer` | <https://github.com/foam-foundation/FOAM-GrammaticVisualizer> | Railroad diagrams with a parse trace | Apache-2.0 | Merged 11 October 2026 | New, 6 commits | SVG output stays legible (inference from the test names). |
| ANTLR IntelliJ plugin | <https://github.com/antlr/intellij-plugin-v4> | Grammar IDE with preview and profiler | BSD-3-Clause | Bundles ANTLR 4.13.2 | Active, 69 open issues and pull requests | Grammar and tree images stay legible. |
| vscode-antlr4 | <https://github.com/mike-lischke/vscode-antlr4> | Grammar IDE with debugger and graphs | MIT | 2.4.7, 25 August 2024 | 45 open issues and pull requests, last push March 2026 | Grammar and SVG exports stay legible. |
| antlr4-c3 | <https://github.com/mike-lischke/antlr4-c3> | Completion candidates from an ANTLR ATN | MIT | 3.4.3, 9 March 2025 | 305 commits, ports to Java, C#, C++ and Dart | Candidate lists are plain data. |
| ANTLR Lab | <http://lab.antlr.org/> | Hosted ANTLR playground | MIT | No releases | 30 open issues, last push August 2024 | Only the grammar text stays. |
| ANTLRWorks 1 | <https://www.antlr3.org/works/> | Grammar IDE with a time-travelling debugger | BSD | 1.5.2, undated | Stopped. The author no longer fixes bugs. | Grammar and event dumps stay legible. |
| Ohm editor | <https://github.com/ohmjs/ohm-editor> | Live grammar editor with trace | MIT | Hosted | Active | Grammar text stays legible (inference). |
| tree-sitter playground | <https://tree-sitter.github.io/tree-sitter/7-playground.html> | Tree view for bundled grammars | MIT | 0.27.2, 7 November 2026 | Active, 116 open issues and pull requests | Corpus files stay legible. |
| Lark | <https://lark-parser.readthedocs.io/en/latest/> | Parsing library with explicit ambiguity and an interactive LALR parser | MIT | 1.3.1, 27 October 2025 | Last push 27 August 2026, 197 open issues and pull requests | Printed trees stay legible. |
| Lark IDE | <https://lark-parser.github.io/ide> | Lark in the browser through Pyodide | MIT | No releases | Last update June 2023 | Only the grammar text stays. |
| nearley playground | <https://omrelli.ug/nearley-playground/> | Hosted Earley playground | Gap for the playground. The nearley library is MIT. | nearley 2.20.1 on npm | Playground source returns 404. Library last push November 2024. | Only the grammar text stays. |
| Bison counterexamples | <https://www.gnu.org/software/bison/manual/html_node/Counterexamples.html> | Text derivations for grammar conflicts | GPL-3.0-or-later (<https://www.gnu.org/software/bison/>) | 3.8.2, 25 September 2021 | No release after September 2021 | The text output stays legible. |
| JFLAP | <https://www.jflap.org/> | Automaton simulator for teaching | Site footer names CC BY-NC-SA 2.5, scope unclear | 7.1, 27 July 2018. 8.0 beta since January 2015. | No later release found | Machines stay legible as XML files. |
| Unitex/GramLab | <https://unitexgramlab.org/> | IDE for local grammars drawn as graphs that call subgraphs | LGPL, language data LGPLLR | 3.3.7, January 2020 | Nightly 4.0alpha builds | Gap: the home page does not name the graph format. |
| regex101 | <https://regex101.com/> | Regex tester with a PCRE debugger | Proprietary | Hosted | Active | Only the regex text stays. |
| Debuggex | <https://www.debuggex.com/> | Regex railroad with a match slider | Proprietary | Beta | Footer shows 2013 | Only the regex text stays. |
| Stately Studio | <https://stately.ai/docs/simulate-mode> | Statechart editor with simulation | Closed source (inference) | Hosted | Active | Exports XState JSON, JavaScript, TypeScript and Mermaid. |
| Stately Sketch | <https://github.com/statelyai/sketch> | Paste-and-simulate viewer for state machines | MIT | Announced 26 March 2026 | New, 86 stars, 20 commits | The machine text stays legible. |

Release and licence data come from these pages, read on 2026-11-17: <https://api.github.com/repos/GuntherRademacher/rr/releases/latest>, <https://api.github.com/repos/tabatkins/railroad-diagrams>, <https://api.github.com/repos/tabatkins/railroad-diagrams/tags>, <https://api.github.com/repos/epfl-systemf/librrd>, <https://arxiv.org/abs/2509.15834v2>, <https://api.github.com/repos/antlr/intellij-plugin-v4>, <https://api.github.com/repos/antlr/antlr4-lab>, <https://api.github.com/repos/mike-lischke/vscode-antlr4>, <https://chevrotain.io/docs/changes/CHANGELOG.html>, <https://api.github.com/repos/mike-lischke/vscode-antlr4/releases/latest>, <https://api.github.com/repos/mike-lischke/antlr4-c3/releases/latest>, <https://api.github.com/repos/tree-sitter/tree-sitter>, <https://api.github.com/repos/tree-sitter/tree-sitter/releases/latest>, <https://api.github.com/repos/lark-parser/lark>, <https://api.github.com/repos/lark-parser/lark/releases/latest>, <https://api.github.com/repos/kach/nearley>, <https://registry.npmjs.org/nearley/latest>, <https://ftp.gnu.org/gnu/bison/>, <https://www.jflap.org/framebody.html>, <https://stately.ai/docs/export-as-code>.

Gap: Grammophone, Sketch.systems and nested statecharts in Stately Studio.

## Silences

- Trace on a railroad diagram. Queries: `SyntaxTrain syntax diagrams error location Moth Villadsen Ben-Ari`, `Debuggex regex railroad diagram slider step through match position highlight` and `"railroad diagram" highlight current parse position animation parser debugger`. No tool beyond those in Q4 appeared.
- Bounded repeats in RR. Query: `bottlecaps rr railroad diagram generator options "width" wrap long sequences EBNF repetition bounded`. No result described a counted repeat.
- Several parses in the nearley playground. Query: `nearley playground ambiguous grammar shows all parses "parse results" omrelli nearley-playground`. No result described the view.
- Tree view of the Lark IDE. Query: `"lark-parser.github.io/ide" Lark IDE online grammar tree`. No result described it.
- Iterations in the Ohm visualiser. The LIVE 2016 paper, the syntax reference and the API reference do not say how a repetition draws or how `(a b)*` splits.
- Pass counts in JFLAP and Stately Studio. The JFLAP tutorial and the simulate-mode page mention none.
- Links to resources on parse-tree nodes. Query: `parse tree viewer node metadata hyperlink attached file "tree view" grammar playground leaf attributes link resource`. The search service returned an error, so this query has no result.
- Maintained editors for recursive transition networks. Query: `recursive transition network diagram editor visualizer tool "RTN" grammar subnetwork call push pop interactive`. The results gave only the 1991 paper by Perlin and small code projects.
- Reason for the end of the ANTLRWorks 1 debugger. Query: `ANTLR 4 removed "-debug" option debug events ANTLRWorks remote debugger not supported ANTLR4 why`. No result gave a reason.
- Paths across subgraphs in the Unitex debug mode. The search excerpts of the manuals do not describe them.
- Drill-down into child machines in the Stately tools, and the layout engine of Stately Sketch. The simulate-mode page, the Sketch announcement and the Sketch README mention neither.

## Sources

- <https://github.com/foam-foundation/foam3/pull/5607>
- <https://www.antlr.org/papers/antlrworks-draft.pdf>
- <https://lark-parser.readthedocs.io/en/latest/classes.html>
- <https://github.com/mike-lischke/antlr4-c3>
- <https://pkg.go.dev/sigs.k8s.io/instrumentation-tools/promq/autocomplete/earley>
- <https://lark-parser.readthedocs.io/en/latest/tree_construction.html>
- <https://github.com/tree-sitter/tree-sitter/issues/223>
- <https://docs.python.org/3/library/traceback.html>
- <https://www.brendangregg.com/flamegraphs.html>
- <https://stately.ai/docs/simulate-mode>
- <https://ohmjs.org/pubs/live2016/>
- <https://www.jflap.org/tutorial/turing/buildingblocks/buildingblocks.htm>
- <https://www.jflap.org/tutorial/turing/changes7.1/index.html>
- <https://arxiv.org/html/2509.15834v2>
- <https://github.com/afw-org/afw/issues/495>
- <https://github.com/GuntherRademacher/rr>
- <https://dubroy.com/blog/visualizing-packrat-parsing/>
- <https://github.com/antlr/antlr4-lab/issues/11>
- <https://www.jflap.org/tutorial/fa/createfa/fa.html>
- <https://stately.ai/blog/2026-03-26-introducing-stately-sketch>
- <https://juholeinonen.com/assets/pdf/leinonen2019exploring.pdf>
- <https://github.com/ohmjs/ohm-editor>
- <https://github.com/tabatkins/railroad-diagrams/blob/gh-pages/README-js.md>
- <https://github.com/GuntherRademacher/rr/releases>
- <https://chevrotain.io/docs/guide/generating_syntax_diagrams.html>
- <https://www.w3.org/TR/xml/>
- <https://github.com/antlr/intellij-plugin-v4/blob/master/src/main/java/org/antlr/intellij/plugin/preview/ShowAmbigTreesDialog.java>
- <https://lark-parser.readthedocs.io/en/stable/recipes.html>
- <https://github.com/chanicpanic/lark-ambig-tools>
- <https://lark-parser.readthedocs.io/en/latest/grammar.html>
- <https://github.com/kach/nearley/blob/master/docs/md/parser.md>
- <https://www.npmjs.com/package/nearley/v/2.10.4>
- <https://www.gnu.org/software/bison/manual/html_node/Counterexamples.html>
- <https://github.com/mike-lischke/vscode-antlr4/blob/master/doc/grammar-debugging.md>
- <https://github.com/tree-sitter/tree-sitter/discussions/1543>
- <https://github.com/antlr/antlr4/blob/master/doc/parser-rules.md>
- <https://ohmjs.org/docs/api-reference>
- <https://www.jflap.org/tutorial/grammar/LL/index.html>
- <https://raw.githubusercontent.com/tree-sitter/tree-sitter/master/docs/src/assets/js/playground.js>
- <https://github.com/GuntherRademacher/rr/issues/62>
- <https://github.com/epfl-systemf/librrd>
- <https://onlinelibrary.wiley.com/doi/abs/10.1002/spe.872>
- <https://www.debuggex.com/>
- <https://github.com/mike-lischke/vscode-antlr4/blob/master/doc/graphical-visualizations.md>
- <https://turing.plymouth.edu/~zshen/Webfiles/notes/CS378/JFLAPActivities.pdf>
- <https://regex101.com/>
- <https://arxiv.org/html/2402.17988>
- <https://github.com/antlr/intellij-plugin-v4>
- <http://lab.antlr.org/>
- <https://github.com/antlr/antlr4-lab/issues/86>
- <https://github.com/antlr/antlr4-lab/issues/68>
- <https://lihautan.com/babel-ast-explorer/>
- <https://graphviz.org/docs/attrs/URL/>
- <https://unitexgramlab.org/releases/3.2/man/Unitex-GramLab-3.2-usermanual-en.pdf>
- <https://unitexgramlab.org/releases/3.0/man/Unitex-GramLab-3.0-usermanual-en.pdf>
- <https://github.com/statelyai/sketch>
- <https://aclanthology.org/P91-1013.pdf>
- <https://groups.google.com/g/antlr-discussion/c/U2uSnV6zqyo>
- <https://app.opencve.io/cve/?product=railroad-diagram_generator&vendor=tabatkins>
- <https://github.com/honeynet/ochi/pull/171>
- <https://github.com/GuntherRademacher/rr/issues/19>
- <https://github.com/antlr/antlr4-lab/issues/90>
- <https://github.com/lark-parser>
- <https://www.antlr3.org/works/>
- <https://tree-sitter.github.io/tree-sitter/creating-parsers/5-writing-tests.html>
- <https://stately.ai/docs/xstate/v6/model-based-testing>
- <https://github.com/tabatkins/railroad-diagrams>
- <https://github.com/foam-foundation/FOAM-GrammaticVisualizer>
- <https://github.com/mike-lischke/vscode-antlr4>
- <https://tree-sitter.github.io/tree-sitter/7-playground.html>
- <https://lark-parser.readthedocs.io/en/latest/>
- <https://lark-parser.github.io/ide>
- <https://omrelli.ug/nearley-playground/>
- <https://www.jflap.org/>
- <https://unitexgramlab.org/>
- <https://api.github.com/repos/GuntherRademacher/rr/releases/latest>
- <https://api.github.com/repos/tabatkins/railroad-diagrams>
- <https://api.github.com/repos/tabatkins/railroad-diagrams/tags>
- <https://api.github.com/repos/epfl-systemf/librrd>
- <https://arxiv.org/abs/2509.15834v2>
- <https://api.github.com/repos/antlr/intellij-plugin-v4>
- <https://api.github.com/repos/antlr/antlr4-lab>
- <https://api.github.com/repos/mike-lischke/vscode-antlr4>
- <https://chevrotain.io/docs/changes/CHANGELOG.html>
- <https://api.github.com/repos/mike-lischke/vscode-antlr4/releases/latest>
- <https://api.github.com/repos/mike-lischke/antlr4-c3/releases/latest>
- <https://api.github.com/repos/tree-sitter/tree-sitter>
- <https://api.github.com/repos/tree-sitter/tree-sitter/releases/latest>
- <https://api.github.com/repos/lark-parser/lark>
- <https://api.github.com/repos/lark-parser/lark/releases/latest>
- <https://api.github.com/repos/kach/nearley>
- <https://registry.npmjs.org/nearley/latest>
- <https://ftp.gnu.org/gnu/bison/>
- <https://www.jflap.org/framebody.html>
- <https://stately.ai/docs/export-as-code>
- <https://www.gnu.org/software/bison/>

## Decision records

These decision records cite this survey:

- `docs/adr/one-earley-tracer.md`: the next options come from the last Earley set, without a second search.
- `docs/adr/trees-follow-rules-and-passes.md`: a loop node keeps the passes that Lark and tree-sitter flatten.
- `docs/adr/rule-diagrams-in-bpmn-style.md`: the workbench draws each rule diagram from the checked grammar.
- `docs/adr/kdl-for-derived-artifacts.md`: each derived artifact records a hash of its grammar text.
