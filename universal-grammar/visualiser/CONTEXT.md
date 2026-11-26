# Grammar workbench

A workbench for grammars of steps. People write the grammars by hand, and the workbench traces sequences of steps through them.

## Language

### Grammars

**Grammar**:
A set of rules in the workbench notation that describes or prescribes sequences of steps.
_Avoid_: model, schema, process model

**Descriptive grammar**:
A grammar that describes observed sequences, such as the discourse moves of a URG analysis. A rejected sequence is evidence against a descriptive grammar.
_Avoid_: analysis grammar

**Prescriptive grammar**:
A grammar that prescribes the allowed sequences, such as a tutorial step model or a process. A rejected sequence contains a wrong step.
_Avoid_: workflow, process definition

**Rule**:
A named definition in a grammar, ended by a semicolon.
_Avoid_: production, non-terminal

**Symbol**:
A terminal of a grammar, written as a quoted string such as `"submit_order"` or `"states the problem"`. A grammar does not declare its symbols.
_Avoid_: token, character, label, literal

**Symbol occurrence**:
One place in a rule body where a **Symbol** appears. Each **Step** of a **Reading** matches one or more symbol occurrences.
_Avoid_: position, symbol use

**Grammar element**:
A **Symbol occurrence**, a **Reference** or a repetition in a rule body. Its span in the grammar text identifies it.
_Avoid_: grammar node, item

**Start rule**:
The rule that a grammar names with `%start`. Every reading is a tree of the start rule.
_Avoid_: root rule, top rule, entry rule

**Doc string**:
The string between a rule name and its `=`, which documents the rule, or the string after `%doc`, which documents the grammar. LLMs read it to follow the grammar.
_Avoid_: doc comment, docstring, annotation, description

**Reference**:
A use of a rule name inside a rule, which can be the same rule.
_Avoid_: call, non-terminal edge

**Tail reference**:
A **Reference** after which its rule body must end.
_Avoid_: tail call, right recursion

**Head reference**:
A **Reference** that can come first in its rule node, because each item before it can match the empty sequence.
_Avoid_: left call, left corner

**Bound**:
The least and the greatest number of passes of a repetition, written as `{2,4}`.
_Avoid_: min/max repeat, quantifier, range

**Bound cap**:
The largest count that the bounds around one item can reach inside one rule. It is 100. The count is the number of copies that an expansion needs, so it does not limit passes.
_Avoid_: cap

**Self-embedding**:
A rule that can reach itself with symbols on both sides, such as `a = x a y ;`. Only self-embedding makes a grammar need a stack.
_Avoid_: context-free, non-regular

### Sequences

**Step**:
One symbol at one position in a sequence, with its attributes.
_Avoid_: event, activity, token

**Sequence**:
An ordered list of steps that the tracer reads.
_Avoid_: trace, log, case, input string

**Attribute**:
A KDL property that a step carries, such as a file reference. An attribute never affects acceptance.
_Avoid_: data, payload, annotation

**Tracer**:
The part of the core that reads a sequence against a grammar, one step at a time.
_Avoid_: engine, simulator, recogniser

**Reading**:
One way in which a grammar accepts the sequence so far, as the start of at least one complete sequence. Each reading has exactly one tree, and no two readings have the same tree.
_Avoid_: parse, derivation, interpretation, alignment

**Reading cap**:
The largest number of readings that the tracer returns. It is 10. The tracer says when it reaches the cap.
_Avoid_: cap, limit

**Next option**:
A symbol that can follow the sequence so far in at least one reading.
_Avoid_: expected token, continuation, enabled activity

**Next position**:
The position that a **Reading** takes when one **Next option** is the next step. One **Next option** can lead to several next positions.
_Avoid_: successor position, next state

**Cut successor**:
A tree of a **Reading** with one more **Step**. It keeps only the new nodes that the rules for a **Next-option mark** read. The core finds the marks from cut successors, so it never lists every **Next position**.
_Avoid_: truncated tree, partial successor

**Pass**:
One trip around a loop: one iteration of a repetition, or one trip around a **Loop through rules**.
_Avoid_: round, cycle

**Loop through rules**:
A cycle of **Rules** that only **Tail references** link, or that only **Head references** link. A self tail call and direct left recursion are its one-rule cases.
_Avoid_: recursion loop, mutual recursion

**Pass rule**:
The **Rule** at which a reading enters a **Loop through rules**. Each **Pass** of that loop holds one node of the pass rule.
_Avoid_: entry rule, loop header, start rule

**Loop node**:
A tree node for a repetition or for a **Loop through rules**, with one child for each **Pass**.
_Avoid_: list node, star node

**Origin**:
A grammar element that a tree node came from, such as the **Reference** that made a rule node. Origins never make two trees different, so one node of a **Reading** can have several.
_Avoid_: source, provenance

**Failure point**:
The first step of a sequence that no reading accepts.
_Avoid_: error position, deviation

**Trace file**:
A KDL file that records a sequence, its readings and their trees. It serves as a test fixture, a saved session and an export.
_Avoid_: log, dump

**File reference**:
An **Attribute** value with the KDL type annotation `url`, `url-reference`, `irl` or `irl-reference`. It is a web address, a relative file reference, or text that the page never opens.
_Avoid_: link, path attribute

### Workbench

**Workbench**:
The browser page in which a person edits a grammar and traces sequences through it.
_Avoid_: playground, web component

**Core**:
The Rust library that parses, checks, traces and lays out grammars, for native hosts and for WebAssembly.
_Avoid_: engine, backend, compiler

**Checked grammar**:
The derived artifact that holds the symbols, the rules and the doc strings of a grammar, with its diagnostics and badges.
_Avoid_: AST, compiled grammar, parsed grammar

**Derived artifact**:
A KDL file that the core writes from a grammar text, with a format version and a hash of that text.
_Avoid_: output file, build product

**Grammar view**:
The view that shows the rule diagrams, the position of each reading and the next options.
_Avoid_: FSM view, state view

**Output view**:
The view that shows the steps so far as the tree of one reading, with folds and attributes.
_Avoid_: history view, parse-tree view

**Fold**:
A row in the output view that stands for completed passes, with their count.
_Avoid_: collapse, summary row

**Details panel**:
The panel under the **Output view** that shows the row in focus. It shows the **Attributes** of its step and the doc string of its rule.
_Avoid_: inspector, properties pane

**Rule diagram**:
The diagram of one rule, drawn in BPMN style.
_Avoid_: railroad diagram, state machine

**Task box**:
A box in a **Rule diagram** that stands for one **Symbol occurrence**.
_Avoid_: activity, symbol box

**Reference box**:
A box in a rule diagram that stands for a reference and opens the diagram of its rule.
_Avoid_: call box, sub-process

**Rule for one step**:
A **Rule** each of whose nodes holds exactly one **Step** and nothing else, such as `install = "install" ;`. Its **Reference box** has no drill-down marker.
_Avoid_: step rule, leaf rule

**Back flow**:
The flow of a repetition in a **Rule diagram**, from the end of its item back to its start. It carries the loop marker of a **Bound**.
_Avoid_: loop edge, return arc

**Back edge**:
The flow that draws a self tail call, from the place of the **Reference** back to the start of its **Rule diagram**.
_Avoid_: recursion edge, back flow

**Reading marker**:
The numbered tab on a box of a **Rule diagram** that shows where one **Reading** stands. On a **Reference box**, it shows that the reading stands inside that rule.
_Avoid_: token, cursor, dot

**Next-option mark**:
The outline on an element of a **Rule diagram** that a **Next option** passes through.
_Avoid_: highlight, hint

**Open path**:
The chain of rules to the open rule diagram, shown as breadcrumbs. It starts at the start rule, or at a rule that the start rule does not reach.
_Avoid_: call stack

**Follow path**:
The **Open path** of a **Reading**, short of a **Rule for one step** that holds its last step. It keeps the start rule even when that rule is for one step. The grammar view follows the reading on view.
_Avoid_: current path, active path

**Badge**:
The label on a group of mutually recursive rules, either "no self-embedding" or "may need a stack".
_Avoid_: regular, context-free

## Relationships

- A **Grammar** has one or more **Rules**, and its **Rules** use one or more **Symbols**. Exactly one **Rule** is its **Start rule**.
- A **Doc string** documents one **Rule** or the whole **Grammar**. A **Rule** that holds one **Symbol** documents that step. The **Checked grammar** carries the text.
- A **Rule** contains **Symbols** and **References**, and has exactly one **Rule diagram**.
- A **Sequence** has zero or more **Steps**. Each **Step** has exactly one **Symbol** and zero or more **Attributes**.
- A **Trace file** records one **Sequence** and its **Readings**, up to the **Reading cap**. It also records a **Failure point** if no reading accepts every step.
- Another program writes only the **Sequence** of a **Trace file**. The **Core** derives the rest of the file.
- Each tree node names its **Origins** as **Grammar elements**. A **Reading marker** sits on the **Task box** of a **Symbol occurrence**, or on the **Reference box** that holds the reading.
- Each **Reading** has exactly one tree, and no two **Readings** have the same tree. Each tree node holds at least one **Step**, except the node of the **Start rule**.
- Each **Loop node** has one child for each **Pass**. In a **Loop through rules**, each **Pass** holds one node of the **Pass rule**.
- A tree node has zero or more **Origins**. A later step can remove an **Origin** from a node, but it never adds one.

## Example dialogue

> **Dev:** "The tutorial grammar is `tutorial = "install" "configure"? lesson+ ;`. If a learner skips configure, does the tracer reject the sequence?"
> **Domain expert:** "No. `"configure"?` is optional, so the sequence still has one **Reading**, and its tree has no node for the skipped step."
> **Dev:** "When the third lesson starts, what does the **Output view** show?"
> **Domain expert:** "A **Fold** for the two completed **Passes** of `lesson`, with the count, and the current pass below it."
> **Dev:** "The exercise step carries a file reference. Does the tracer check the file?"
> **Domain expert:** "No. Only symbols decide acceptance. The **Output view** shows the file as an **Attribute**, and the **Grammar view** shows the **Next options**."
>
> **Dev:** "In `question = "asks" answer ; answer = "answers" question? ;`, where does a **Pass** start?"
> **Domain expert:** "The two rules form a **Loop through rules**. If the reading enters at `question`, then `question` is the **Pass rule**, and each pass is "asks" then "answers"."

## Flagged ambiguities

- "Trace" named both the input and the record of a run. Resolved: a **Sequence** is the input, and a **Trace file** records a run.
- "Token" names a BPMN token, a parser token and a unit of LLM text. Resolved: write **Reading marker** for a mark on a diagram, and **Symbol** for the name of a step. "Token" names only a lexical unit of grammar text, as in `rowan`.
- "Cap" named two limits. Resolved: **Reading cap** and **Bound cap**.
- "Position" names a state of a position automaton and the place where a reading stands. Resolved: write **Symbol occurrence** for the grammar element, and **Reading marker** for the mark in the grammar view.
- "Loop" covers a repetition and a **Loop through rules**, such as a self tail call or left recursion. Resolved: each makes a **Loop node**, with one **Pass** for each trip.
- "Entry rule" once named the **Start rule** and could name the rule where a reading enters a loop. Resolved: write **Start rule** for the first and **Pass rule** for the second.
- "Right-regular" names a narrower class than the **Badge** tests. Resolved: the workbench says "no self-embedding" for any group of rules that needs no stack.
- A grammar with self-embedding can still describe a regular language. Resolved: the **Badge** says "may need a stack", never "not regular".
- "Any order" has two meanings: whole items in any order, as with the SGML `&` connector, or interleaved steps, as with BPMN parallel branches. Unresolved: the operator comes after the slice.
- Process-mining terms such as case, activity and alignment stay in the research reports.
