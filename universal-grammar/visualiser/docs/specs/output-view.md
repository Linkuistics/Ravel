# output-view

## Problem

The output view shows what a run produced so far: the steps as the tree of one reading. The human described it as "in effect the entire generated output of the grammar to that point", with completed passes "under the repetition operator" in a form that stays manageable. No maintained playground in the visualiser survey draws a partial tree for a prefix. So the output view has the most risk in the slice, and little prior art to copy.

A partial tree changes at every step. When the tree grows, the rows that a person already read must stay in place. A grammar can read the steps so far in several ways, until a later step decides. Two readings can also disagree on the number of passes. A rejected step must leave both views useful. Steps carry attributes, such as a file reference, that the grammar never reads.

The decision record `trees-follow-rules-and-passes` gives each reading one tree, with loop nodes and pass nodes. It left one point open: where a pass starts in a loop through several rules. The records `symbols-decide-acceptance`, `one-earley-tracer` and `rule-diagrams-in-bpmn-style` also bind this design.

## Solution

The output view is an outline. Each step has its own row, under the rows of the nodes that hold it. A gutter numbers each step from 1, and a step keeps its number in every reading. Completed passes of each loop fold into one row with counts. The current pass and the path to the last step stay open.

A header counts the readings of the steps so far, and a switch shows one reading at a time. A row at the end lists the next options, and a click on one adds that symbol. At a failure point, both views keep the state before the rejected step, and the rejected step gets a row of its own.

Attributes show as chips on each step row, and a details panel shows every attribute of the step in focus. The grammar view draws a data object beside the task box of that step. A web address opens directly, and a relative file reference opens after the person opens its folder. A selection in either view selects the matching element in the other view.

A pass of a loop through several rules starts at the rule where the reading entered the loop. Left recursion also makes a loop. The tree of a reading only grows: a new step never moves an earlier step to another node.

The mock-ups of this design are in the presentation `docs/design/output-view`.

## Decisions

### The outline

- The output view is an outline of rows, from top to bottom in the order of the steps. Each row is one tree node or one fold. A shared row, as the section "Rows" defines, holds a node together with its only child, which can share in turn.
- The indent of a row is the depth of its node in the tree.
- The gutter shows the number of each step row, from 1. A fold row shows the range of its steps, such as `5–12`.
- A new step adds rows only at the end of the outline. It can also fold the pass before a new pass, and fold a loop that the reading left, as the section "Folds" states. Every other row keeps its place and its indent.
- The section "Trees grow at the end" states the property of the core that allows this. The rules for shared rows depend only on the grammar, so no row splits when its node grows.
- The view follows the last step: it scrolls to keep the last row visible, unless the person scrolled away.
- The page draws the outline with HTML elements and a D3 data join, not with SVG. Text, scrolling and keyboard access then work as in any page.
- The outline is an ARIA tree. Each row exposes its kind, its label, its step range and its fold state. The end-to-end tests check this structure, not positions.

### Rows

| Row | Label | Right side |
|---|---|---|
| Rule row | The rule name | Nothing |
| Step row | The symbol, in quotes, and the attribute chips | Nothing |
| Loop row of a repetition | The repetition as written in its rule, such as `show_example{1,3}` | The current pass and the bound, such as "pass 2 · 1 to 3 passes" |
| Loop row of a loop through rules | The pass rule with `↻`, and the kind of loop | The current pass |
| Pass row | "pass" and its number | Nothing |
| Fold row | "completed", the count of passes and the count of steps | Nothing |
| Next-options row | "next" and one chip for each next option | Nothing |
| Rejected-step row | The rejected symbol and the reason | Nothing |

- The node of a rule for one step shares one row with its step, such as `overview · "overview"`. A rule is for one step when each node that it can make holds exactly one step and nothing else. The step rules of the tutorial grammar are examples, and so is `answer = "yes" | "no" ;`.
- A rule that can hold more steps never shares a row, even while its node holds one step. So the first step of such a rule keeps its row when the second step comes.
- A pass shares one row with its node when each pass of its loop holds exactly one node, such as `pass 2 · lesson`. Each pass of a loop through rules holds one node of the pass rule. A pass of a repetition holds one node when the repeated item always makes one node, as in `lesson+`.
- Such a row is a shared row. It shows the labels of its nodes, joined by `·`. Both rules depend only on the grammar. The section "Selection across the views" states what a shared row stands for.
- The loop node of a repetition can have several repetitions among its origins. Its row then shows each written form once, in the order of the grammar text, separated by `|`. The right side shows the bound only when all these repetitions have it.
- Two repetitions share one loop node only when they are in the same rule. A repetition is mixed when its item can make one node in one pass and more nodes in another pass, as in `("x" "y"?)+`. In a rule with a mixed repetition, no pass of a repetition shares a row. So a row never merges or splits when a later step removes an origin of its loop node.
- A row that holds rows has a fold control. Only fold rows start folded.
- Each count names its unit: passes, steps, readings, files or links.

### Folds

- Every completed pass folds. A pass is complete when it does not hold the last step. The fold rule is fixed: it never depends on the distance from the last step.
- The completed passes of one loop share one fold row, above the current pass. A new pass folds the pass before it into that row, so the fold row stays in place and its counts grow.
- A loop that holds no part of the current path is finished. All of its passes are complete, so its loop row and its fold become one row, such as `install+ · 2 passes · 2 steps`.
- A fold row counts the references inside it: relative file references as files, and web addresses as links.
- A click opens a fold. Each completed pass then shows its own content, and its loops fold by the same rule. The fold stays open while the run goes on, until the person folds it again.
- The current pass never folds, and nor does any node on the path from the start rule to the last step.

The layout survey found no speed gain from folds that follow a computed degree of interest. Users also did not always see why an element appeared. A fixed rule answers both findings. Of the tools in the visualiser survey, only Python tracebacks fold repeated passes with a count.

### Readings

- The header shows "Reading i of n", where n counts the readings of the steps so far. A switch shows the readings one at a time, in the order of the core.
- When the tracer reaches the reading cap, the count reads "Reading 1 of C+", where C is the cap. A note says that the tracer stopped at the cap.
- When the state has no reading, the header says "no reading", and the view shows no tree and no next option. This happens only when the start rule derives no finite sequence.
- When a step extends the reading on view, the view shows the first reading that extends it. When that reading has no extension, the view shows reading 1 and says that the earlier reading ended.
- After the tracer reaches the reading cap, the list can leave out each extension of that reading. This can happen only when each listed reading extends an earlier listed reading. The view then shows reading 1 and says that the cap can leave out the extensions.
- When the person removes the last step, the view shows the reading that the reading on view extended. When the cap left that reading out of the list, the view shows reading 1.
- The header also states whether the sequence can end after its last step: "can end here", "cannot end yet" or "rejected at step k".

### The next options

- The last row lists the next options of the state on view. A chip of the reading on view is solid. A chip that only other readings allow is dashed. Both kinds of chip are clickable.
- A click on a next option inserts that symbol right after the state on view. At the end of the sequence, this appends the symbol, as the root brief requires.
- When some reading can end after the last step, the row says "or end here".

### Steps that the tracer has not read

The sequence can hold steps after the state on view: after a failure point, or after a step back. These steps fold into one row at the end, such as "2 later steps, not read". A click opens the row, and each step shows its symbol and its attributes. These rows have no indent, because they belong to no node.

### The failure point

At a failure point, step k is the first step that no reading accepts.

- The header says "rejected at step k" and "showing the state after step k−1". The count of readings is the count after step k−1.
- The tree, the folds and the next options are those of the state after step k−1.
- Step k gets a rejected-step row after the tree, with no indent, because it belongs to no node. The row shows the symbol, its attributes and the reason.
- The reason has two forms. "The grammar uses "ship" in outcome, but not at this point." Or "The grammar has no symbol "aprove"." The page decides between them from the symbols of the checked grammar.
- The next-options row follows, as "next at k". A click inserts the symbol before step k, and the tracer reads the inserted step. The click never removes a step that a person or a program wrote.
- The steps after step k fold into one row, as the section above states.
- The grammar view keeps its reading markers and next-option marks of the state after step k−1. A banner names the rejected step, its reason and the next options.

For a prescriptive grammar, the failure point and its next options show the wrong step and the steps allowed instead. For a descriptive grammar, they show where the evidence leaves the grammar. Explanations that count extra and skipped symbols come after the slice.

### Passes in loops through rules

These terms define the loops:

- A tail reference is a reference after which its rule body must end. Nothing can follow it in the rule.
- A head reference is a reference that can come first in its rule node, because each item before it can match the empty sequence. In a reading, it acts as a head reference only when its rule node holds no step before it.
- A loop through rules is a cycle of rules that tail references link, or a cycle that head references link. The self tail call of `trees-follow-rules-and-passes` is the one-rule case of the first kind. Direct left recursion is the one-rule case of the second kind.
- A cycle that mixes tail references and head references is self-embedding. It makes nested rule nodes, not a loop.

The passes follow these rules:

1. A reading enters a loop through rules at a reference from outside the cycle, or at the start rule. It also enters a new loop at a reference into the cycle that does not act as a link. The rule that it enters is the pass rule.
2. The loop node starts with the first step inside the loop, together with its first pass.
3. In a loop of tail references, each later tail reference to the pass rule starts a new pass.
4. In a loop of head references, a new pass starts with each new node of the pass rule. That node takes the loop so far as its head. The head references that link the node to the loop so far start the pass.
5. Each pass holds one node of the pass rule. The other rules of the cycle stay nested in it, as the grammar text nests them.
6. Inside one pass, a cycle that avoids the pass rule makes a nested loop by the same rules.
7. A cycle of tail references makes no loop when one of its rules lies on a cycle of head references. Its tail references then make nested rule nodes.

So one pass is one trip around the loop, back to where the reading entered it. In the grammar `question = "asks" answer ; answer = "answers" question? ;`, a reading that enters at `question` has the passes `"asks" "answers"`. The loop is equal to the repetition `("asks" "answers")+`, and the two give the same count of passes. A reading that enters at `answer` has the passes `"answers" "asks"`.

The open path of the grammar view follows the ancestors of a node in the tree. So it shows only the rules of the current pass, and it stays short in a long loop.

The rejected alternatives:

- Every tail call starts a pass. Passes then alternate between the rules of the loop, and the count doubles for a loop through two rules. A pass no longer means one trip, and the count no longer matches the equal repetition.
- One fixed rule of each cycle starts every pass, such as the first rule in the text. A reading that enters at another rule then starts with a part of a pass. The author can choose the start rule only through the order of the text.

Left recursion becomes a loop for a second reason. Without a loop node, the depth of a left-recursive spine depends on steps that have not come yet. So each new step would move the earlier steps one level deeper, and rows could not keep their places.

Rule 7 settles a rule on both kinds of cycle, such as `a = a "x" | "y" a | "z" ;`. Only its loop of head references keeps the tree growing at the end, so that loop stays. Nested nodes of tail references grow at the end too. The spec `tracer` gives the full reasons.

The decision record `trees-follow-rules-and-passes` records these rules, because they change the tree of the core.

### Readings and trees

- A reading is one distinct tree. Two derivations that give the same tree are one reading, because choices make no node and the views cannot tell them apart. The grammar view still marks each position of such a reading.
- Two trees are the same when they have the same shape, and each pair of matching nodes agrees on four labels. The labels are the kind of each node, and the rule of a rule node. They also include the kind of loop of a loop node, and the step of a step node.
- The origins of a node are the grammar elements that it came from, such as the reference that made a rule node. Origins never make two trees different. A node of a reading has each origin that any derivation of the reading gives it.
- So a node can have several origins. In `s = item "y" | item "z" ; item = "x" ;`, the reading of `"x"` has one `item` node, which both references to `item` made.
- A later step can exclude a derivation, and so remove origins from a node. It never adds an origin to a node of the earlier tree. In `s = "x" "y" | "x" "z" ;`, step 1 has both occurrences of `"x"` as origins until step `"y"` removes one.
- A node holds at least one step. The start rule node is the one exception: it is the root before the first step. So a rule that matches the empty sequence makes no node, and a repetition with no passes makes no loop node.
- Without this rule, `a = b? ; b = "x"? ;` would read the empty sequence as two trees, `a` and `a` over `b`. Both trees look the same in the output view.
- A rule that can derive itself with no step, as in `a = b ; b = a | "x" ;`, can give infinitely many trees for one sequence. The rule checks of `grammar-notation` report such a rule as an error, so the tracer never reads its grammar.

### Trees grow at the end

When step k extends a reading, the tree loses nothing and moves nothing. Without step k and the nodes that hold only step k, the tree is the tree of the reading after step k−1 that it extends. Each remaining node has the same origins as in that reading, or fewer, as the section "Readings and trees" states. This property keeps the rows of the earlier steps in their places.

Loop nodes for loops through rules give this property for left recursion too. The property-based tests at the core seam check it.

### Step attributes

The views show attributes, and the tracer never reads them (`symbols-decide-acceptance`). Attributes are decorations: they never change the height of a row or the place of a box.

- The output view draws attribute chips on each step row. References come first, then the other attributes, ordered by key. A chip shows a short form of its value. Chips stay on the row while they fit, and a last chip such as "+2" counts the rest.
- The details panel sits under the output view and shows the row in focus. The row in focus is the selected row. When the selection is a grammar element, it is the last row of the outline that matches the element. With no selection, it is the row of the last step that the tracer read.
- For a step, the details panel shows every attribute: the key, the type annotation and the whole value. For a rule node, it shows the doc string of the rule. For a loop node, it shows each repetition among its origins with its bound, and the count of passes.
- For a shared row, the details panel shows each node that the row holds. So the doc string of a rule for one step and the attributes of its step appear together.
- The step in focus is the step of the row in focus. A row in focus that holds no step leaves no step in focus.
- The grammar view draws a BPMN data object beside the task box of the step in focus, joined by a dotted association. The data object shows the count of attributes, and a click on it selects the step. The rule-diagram design places the data object on a free side of the box.

Disco and Inductive Visual Miner show event attributes in a table or panel beside the model. Graphviz attaches a link to a node. bpmn-auto-layout treats data references as decorations that never move flow nodes. The design combines the three.

### File references

- A value is a reference when its type annotation is `url`, `url-reference`, `irl` or `irl-reference`. KDL 2.0 reserves these annotations for RFC 3986 and RFC 3987 identifiers and references. A plain string is never a reference, even when it looks like a path.
- A reference with the scheme `http` or `https` is a web address. A click opens it in a new tab, with no access to the page that opened it.
- A relative path, with no scheme and no leading `/`, is a relative file reference. It is relative to the folder that holds the trace file.
- Every other reference stays text: another scheme, a path that starts with `/`, or a path that starts with `//`. The page never opens such a reference and never runs a value as a script.
- Before any folder is open, a relative file reference shows its path, and the details panel offers a folder picker. The picker is the `webkitdirectory` file input, which MDN marks as Baseline 2025 in the major browsers.
- After the person opens the folder, the page compares each relative path with the paths of the files in that folder. The page resolves `.` and `..` and decodes percent escapes first. It ignores a query and a fragment.
- A file with that path opens in a new tab, from the file that the picker gave. A path that no file has shows "not found in the folder". A path that leaves the folder shows "outside the folder".
- The opened folder lasts until the page reloads or the person opens another folder. The File System Access API is not used, because only Chromium browsers support it.

A browser page cannot open a local file by its path (layout survey). The folder picker is the one form that works in each major browser.

### Selection across the views

A selection is one row of the output view or one element of the grammar view. Both views show it in one style, distinct from the reading markers and from the next-option marks. Each grammar element has the identity that the checked grammar gives it, and the rule-diagram design links each box to that identity.

From the output view to the grammar view, a row stands for its node. A shared row stands for its rule node, or else for its innermost node. The grammar view selects the elements that the table below gives for that node. It opens the diagram that holds them, and the open path follows the ancestors of the node in the tree.

| Output-view row | Grammar-view element |
|---|---|
| Step row | The task box of each symbol occurrence among the origins of the step |
| Rule row | Each reference box among the origins of the node. The start rule node selects the diagram of the start rule |
| Loop row or fold row of a repetition | Each repetition among the origins of the loop node |
| Loop row or fold row of a loop through rules | Each reference box among the origins of the loop node. A loop that the reading entered at the start rule selects the diagram of the start rule |
| Pass row | The drawing of each tail or head reference among the origins of the pass: a back edge, or a reference box with the loop sign. The first pass of a loop through rules and each pass of a repetition select the elements of their loop row |
| Shared row | The elements of the node that the row stands for |
| Next option | The next-option marks of that symbol |
| Rejected-step row | No element |

The pass of a shared row needs no element of its own. Its loop row selects the same loop. In a loop through rules, the reference that started a pass also made the node of the pass.

From the grammar view to the output view, the output view selects every matching row of the reading on view. A row matches when it holds a node with the element among its origins. A shared row matches through each node that it holds. So a selection of the task box in a rule for one step selects the shared rows of that rule.

- When some matching rows are inside a fold, the fold row shows how many selected rows it holds, and it stays folded.
- The output view scrolls to the last matching row, or to the fold row that holds it. That row is the row in focus of the details panel.
- A choice gateway, a start event and an end event select no row, because choices make no node.

A selected row survives a new step and a switch of reading while the node that it stands for exists. A step keeps its number in every reading, so a selected step row always survives a switch of reading. A selected grammar element always survives, and its matching rows follow the tree on view.

Camunda Cockpit shares the selection between its instance tree and its diagram in both directions. Ohm links its trace, its grammar and its input on hover. The design extends the Cockpit pattern to finished rules.

### What the output view needs from the trace

The spec `tracer` defines the trace file and the interface between the core and the page. It takes these needs as requirements, and it answers each one. The output view reads them from the core for the state on view, after step k.

1. The steps of the sequence: the number of each step from 1, its symbol and its attributes. The attributes stay KDL properties, unchanged, with their type annotations. The steps after the state on view stay in the sequence.
2. The readings after step k, up to the reading cap, and whether the tracer reached the cap.
3. Each reading is one distinct tree, as the section "Readings and trees" states. Origins never make two trees different.
4. Each node has a kind: rule, loop, pass or step. Its children come in the order of their steps. A rule node names its rule, and a loop node names its kind of loop. A loop node of a loop through rules also names its pass rule. A pass node names its number from 1, and a step node names its step.
5. Each node names its origins, as the section "Readings and trees" defines them. A rule node has the references that made it. A loop node has its repetitions, or the references that entered its loop through rules. A pass node has the tail or head references that started it. In a loop through rules, the node of a later pass has the references that started the pass. The node of the first pass has the references that entered the loop. A step node has the symbol occurrences that the reading allows. Some nodes have no origin. They are the start rule node, and the loop node of a loop entered at the start rule. The first pass of a loop through rules and each pass of a repetition have no origin either.
6. A node holds at least one step, except for the start rule node.
7. Each reading after step k names the reading after step k−1 that it extends, as the section "Trees grow at the end" states.
8. The order of readings is deterministic, and readings keep the order of the readings that they extend. When A comes before B after step k−1, each extension of A comes before each extension of B.
9. Each reading states whether it can end after step k.
10. Each next option names its symbol, the readings that allow it and the symbol occurrences that offer it.
11. At a failure point, the trace names step k. The state that it gives is the state after step k−1.
12. The format version and the hash of the grammar text, as `kdl-for-derived-artifacts` requires.

A program that writes a trace file writes each relative file reference relative to the folder that holds the trace file.

## Test seams

The work tests through the two seams that the human agreed on 2026-11-17. This design adds no seam.

1. **Core interface.** The expected trace files of the example grammars hold trees in the shape that this spec states. Test grammars cover a loop through two rules, entered at each of its rules. Other test grammars cover direct left recursion and a left-recursive cycle through two rules. Two more give two alternatives the same first symbol, or the same first reference. A later step then removes an origin. A loop through rules that the reading enters at the start rule also has a test grammar. A last test traces `a a a` through `(a | a a)*`. Property-based tests check three invariants on random grammars and sequences:
   - No two readings have the same tree.
   - Each node holds at least one step, except for the start rule node.
   - Each reading after step k extends the reading that it names as its parent. Without step k and the nodes that hold only step k, its tree is the tree of that parent. Its nodes have no origin that they lacked in that parent.
2. **Workbench page.** End-to-end tests step through the example sequences. They read the ARIA tree of the outline. They check the rows, the folds with their counts, and the count of readings. They check that a rule of two steps keeps the row of its first step when its second step comes. They also check the next options, the failure point and a selection in both directions. The selections include a shared row, and a task box that matches several rows. Last, they check the chips, a web address and a relative file reference. They check structure, not positions.

## Out of scope

- A shared view of all readings, and marks for the rows in which two readings differ. Both stay candidates after the slice, with the explanations of rejected sequences.
- Folds by the distance from the last step, and groups of identical passes. The fixed rule answers the evidence of the layout survey.
- A file viewer inside the page. A file opens in a new tab.
- The File System Access API, and a stored permission for a folder. Only Chromium browsers support the API.
- Search and filters in the outline.
- Markers for steps in any order. The operator comes after the slice.
- A text presentation. On 2026-11-19 the human noted that some grammars read best as text, like source code, with blocks only for loops. Such a presentation would draw the steps as lines of text, and only the loop nodes as blocks. It reads the same trees, so it needs no new tree in the core. What text each line shows stays open. Whether the grammar or the person chooses the presentation also stays open.
