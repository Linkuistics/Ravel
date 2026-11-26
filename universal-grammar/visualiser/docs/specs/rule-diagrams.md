# rule-diagrams

## Problem

The grammar view draws each rule of a grammar as a diagram in BPMN style. It must also show where each reading stands, the next options and the current pass of each loop. The record `rule-diagrams-in-bpmn-style` sets the frame: one diagram for each rule, gateways for choices and loops, and reference boxes with drill-down. The record `layout-in-the-core` puts the layout into the core, so every host gets the same coordinates.

Three questions stayed open. The first is the exact drawing of each construct of the notation. The second is the form of a reading marker, which must look different from a BPMN token. The third is the font, because the core must know the width of each label without a browser.

A trace adds a fourth question: how a position in a tree becomes a mark on the open diagram. The tutorial grammar shows why this matters. Most of its rules hold only references to rules for one step, so most of its diagrams have no task box at all.

## Solution

Each rule diagram is one BPMN process that flows from left to right, from a start event to an end event. A symbol occurrence is a task box, and a reference is a reference box with a thick border. A choice is a pair of exclusive gateways. An optional part has a skip flow above its item, and a repetition has a back flow below its item. A bound writes its numbers beside the loop marker on the back flow. A self tail call is a back edge to the start of the diagram, where a straight route to a lane is clear.

The core lays out each diagram as a fold over the expression of the rule. Each box takes its size from its children, and the order of the drawing is the order of the text. Labels use one bundled monospace font, so the width of a label is its count of cells. The core writes the coordinates as the diagram KDL, and each box names the span of its grammar element.

The grammar view shows one diagram at a time, with breadcrumbs for the open path, and it follows the reading on view. A reading marker is a numbered tab on the top edge of a box. A next-option mark outlines each element that a next option passes through, and its ▸ button appends the symbol. Each loop that holds a reading shows its current pass. The core computes these marks for the open diagram, so the page only draws them.

The core follows the routes of each listed reading to its next positions, so the reading cap removes none. It keeps of each next position only the nodes that the rules for marks read, so the work stays polynomial.

The mock-ups of this design are in the presentation `docs/design/rule-diagrams`.

## Decisions

### The diagram of a rule

- Each rule has one diagram. The diagram is one BPMN process, and its flows run from left to right.
- The start event is a thin circle at the left end of the main line. The end event is a thick circle at the right end.
- The main line is a straight horizontal line. It passes through the start event, the first alternative of each choice and the end event.
- A flow is a BPMN sequence flow: a solid line with an arrowhead at its target. Flows run horizontally and vertically, and the page rounds their corners.
- A gateway is an exclusive gateway: a diamond with no marker. The BPMN 2.0.2 standard makes the X marker optional, and it asks a diagram to mark every exclusive gateway or none. The X reads as an error or a stop, so the diagrams mark none. The notation has no parallel construct, so no other kind of gateway occurs.
- Gateways come in pairs. A split gateway opens a construct, and a join gateway closes it. Guideline G4 of the seven process-modelling guidelines asks for such matched pairs (automata survey).
- A join gateway that receives only one flow is not drawn. Its flow passes straight through its place.
- A group in parentheses has no drawing of its own.

### The drawing of each construct

| Construct | Drawing |
|---|---|
| Symbol occurrence | A task box: a rounded rectangle with a thin border and the symbol, without quotes |
| Sequence | The items in a row, joined by flows |
| Choice | A split gateway, the alternatives from top to bottom in the order of the text, and a join gateway. The first alternative stays on the main line |
| Optional part | A split gateway, the item and a join gateway, with a skip flow above the item |
| `x+` | A join gateway, the item and a split gateway, with a back flow below the item |
| `x*` | A split gateway and a join gateway, the item, and a split gateway and a join gateway. A skip flow runs above the item, and a back flow runs below it |
| Bound | The drawing of `x+` when the least number is 1 or more, and the drawing of `x*` when it is 0. The back flow carries the loop marker with the numbers |
| Reference | A reference box: a rounded rectangle with a thick border, the rule name and a drill-down marker at the bottom |
| Reference to a rule for one step | A reference box without the drill-down marker |
| Self tail call | A back edge from the place of the reference to a join gateway after the start event. Without a straight route to a lane, a reference box with the loop sign |

- The loop marker is ↻, followed by the numbers of the bound. `{1,3}` shows `↻ 1–3`, `{0,3}` shows `↻ 0–3`, `{2,}` shows `↻ 2+` and `{3}` shows `↻ 3`.
- A repetition with `*` or `+` shows no loop marker. Its skip flow, or the absence of one, already shows the least number.
- The drill-down marker is the marker of a collapsed sub-process in BPMN: a small square with a plus sign.
- A rule is for one step when each node that it can make holds exactly one step and nothing else, as the output-view spec defines. Examples are `install = "install" ;` and `answer = "yes" | "no" ;`.
- A reference to a rule for one step follows the BPMN call activity of a global task: a task shape with a thick border. The rule holds exactly one step, so the box stands for that step.
- A reference that links a loop through rules shows the loop sign ↻ in its lower left corner. An example is `question?` in `answer = "answers" question? ;`. Such a loop is a set of linked diagrams, as the record `rule-diagrams-in-bpmn-style` states.
- A self tail call is a tail reference to its own rule that links a loop of tail references. Rule 3 of the section "Loops through rules" of the tracer spec can keep such a reference out of a loop. The reference then draws as a reference box.
- A self tail call draws as a back edge only when a straight route to a lane is clear, as the section "Back edges" states. Otherwise, it draws as a reference box with the loop sign, like the other links of loops through rules.
- A self head call, as in `list = list "item" | "first_item" ;`, is a reference box with the loop sign. An edge would need a path that the order of the text does not give.

#### Back edges

- A back edge starts where its reference stands. It runs straight down or straight up to a lane outside the body of the rule. It runs along the lane, and it ends at the join gateway after the start event.
- The label of a back edge is ↻ and the rule name, such as `↻ refinement`. The label stands at the middle of the lane.
- When the reference is the item of an optional part, the split gateway of the part sends the back edge. The main line goes on to the end event, and the part has no join gateway.
- After a self tail call, its rule body must end. So in each choice around it, only the later alternatives hold boxes below it, and only the earlier alternatives hold boxes above it.
- The lane lies below the body when the reference lies in the last alternative of each choice around it. The lane lies above the body when the reference lies in the first alternative of each choice around it. When both hold, the lane lies below.
- In any other place, no straight route is clear, and the self tail call draws as a reference box with the loop sign. An example is the second `menu` in `menu = "show_menu" ("choose_item" menu | "search" menu | "quit") ;`.
- Each of the two conditions holds for at most one place in a rule. So a rule has at most one back edge below its body and at most one above it.
- A back edge can cross a skip flow, but it crosses no box and no free band.

### Labels and the font

The record `bundled-monospace-font-for-diagram-labels` holds the choice of the font.

- Every label uses one bundled monospace font at 13 px. Each cell of the font is 0.6 em wide, which is 7.8 px.
- The width of a text is its count of cells times 7.8 px. A wide East Asian character takes two cells, and a combining mark takes no cell. The Unicode property East Asian Width decides.
- A label can break after a space, a `_` or a `-` when the next character is not a space. So a label never breaks inside a run of spaces or at its end.
- Before a break, its line holds a character that is not a space. So a label never breaks inside the spaces at its start.
- A break after spaces drops all of those spaces. A break after `_` or `-` keeps that character at the end of its line.
- Every other space stays and counts as a cell. That includes the spaces at the start and at the end of a label. So symbols that differ only by spaces get different labels.
- A line holds 18 cells at most, unless one word without a break is longer.
- A label takes the fewest lines that this limit allows. With that count of lines, its box is as narrow as possible. So "shows the simplest case" breaks into "shows the" and "simplest case".
- When several sets of breaks give the same count of lines and the same width, the first line holds as much as it can. Then the second line does the same, and so on. So "aaaaaa bbbbbb cccccc" breaks into "aaaaaa bbbbbb" and "cccccc".
- Each line of a label is 16 px high. The lines stand in the centre of the box.
- The page loads the font before it draws the first diagram, and it turns off ligatures.
- A character that the font does not hold falls back to a system font, whose width can differ. Such a label can overflow its box a little. The example grammars use only ASCII.

### The layout

- The layout is a fold over the expression of the rule, as in the railroad library of Tab Atkins (layout survey). First each element measures its children. Then it places them in the space that it measured.
- Each element has a width, an extent above the main line and an extent below it. Its flow enters at the left end of the main line and leaves at the right end.
- A sequence places its items from left to right. A choice places its alternatives from top to bottom. So the order of the drawing is the order of the text.
- An edit inside one element changes the size of that element and of each element that holds it. The elements after it move, but none changes its order.
- The other rules keep their diagrams, with one exception. A reference box depends on two facts of its target rule. The first fact is whether the rule is for one step. The second fact is whether the reference links a loop through rules.
- An edit of one rule can change these facts for a reference in another rule. An edit that closes or opens a loop through rules is one example. In `s = r ; r = "x" ;`, an edit of `r` to `"x" "y"` gives the box of `r` in `s` a drill-down marker.
- Such a change of facts can change the size of the reference box, its marker and its loop sign. A self tail call can change between a back edge and a reference box. The elements after it move, but no element in that diagram changes its order.
- The layout reads each element once to measure it and once to place it. It needs no search and no optimisation.
- Every box keeps a free band of 22 px above its top edge. No flow and no other box enters the band. The reading markers and the data object use it.
- All coordinates are whole pixels. Each width and each height of a box is even, so the main line passes through whole pixels. A midpoint rounds down.

The constants of the layout:

| Constant | Pixels |
|---|---|
| Font size | 13 |
| Width of a cell | 7.8 |
| Height of a line of a label | 16 |
| Padding of a label: left and right, top and bottom | 10, 8 |
| Least size of a box | 64 by 36 |
| Free band above a box | 22 |
| Band of the drill-down marker at the bottom of a reference box | 12 |
| Size of a gateway, and diameter of an event | 24 |
| Flow between two elements in a row | 24 |
| Flow between the two gateways of a pair in `x*` | 10 |
| Gap between the free band of an item and its skip flow | 12 |
| Gap between an item and its back flow | 18 |
| Space below a back flow: with a loop marker, without one | 9, 2 |
| Gap between two alternatives | 14 |
| Gap between the body and the lane of a back edge | 24 |
| Space beyond the lane of a back edge, for its label | 8 |
| Margin around a diagram | 16 |

The sizes of the elements follow from these constants. In the list, w, a and b are the width and the extents above and below the main line.

- A box: w is the width of its widest line plus 20, at least 64, rounded up to an even number. Its height is 16 for each line plus 16, at least 36, rounded up to an even number. A reference box with a drill-down marker adds 12. Then a is half the height plus 22, and b is half the height.
- A sequence: w is the sum of the widths of its items, plus 24 between each pair. Then a and b are the greatest extents of its items.
- A choice: w is 24 + 24 + the widest alternative + 24 + 24. The first alternative lies on the main line. The main line of each next alternative lies below the one before it. The distance is the extent b of the one before, plus 14, plus its own extent a.
- An optional part: w is 24 + 24 + the width of the item + 24 + 24. The skip flow lies 12 above the extent a of the item.
- `x+`: w is 24 + 24 + the width of the item + 24 + 24. The back flow lies 18 below the extent b of the item.
- `x*`: w is 24 + 10 + 24 + 24 + the width of the item + 24 + 24 + 10 + 24. It has the skip flow of an optional part and the back flow of `x+`.
- A rule: the margin, the start event, a flow, the body, a flow, the end event and the margin. With back edges, their join gateway and a flow come before the body. The lane of a back edge lies 24 beyond the extent of the body and the events, on its side. The diagram extends 8 beyond the lane, for half the height of the label.

### The diagram KDL

The core writes the diagrams of a grammar as one KDL document. It is a derived artifact, so it records a format version and the hash of its grammar text (`kdl-for-derived-artifacts`).

| Node | Properties | Children |
|---|---|---|
| `diagrams` | `format-version=1`, `grammar-sha256`, `font-size`, `cell-width` and `line-height` | One `diagram` for each rule, in the order of the text |
| `diagram` | `rule`, `width` and `height` | The shapes, then the flows |
| `start-event`, `end-event` | `x`, `y` and `size` | None |
| `task` | `start` and `end`: the span of the symbol occurrence. `symbol`, `x`, `y`, `width` and `height` | One `label` |
| `reference` | `start` and `end`: the span of the reference. `rule`, `x`, `y`, `width` and `height`. `one-step`: whether the rule is for one step. `loop-link`, for a link of a loop through rules: `tail-references` or `head-references` | One `label` |
| `label` | None. Its arguments are the lines of the label | None |
| `gateway` | `role`, `start` and `end`: the span of its construct. `x`, `y` and `size` | None |
| `flow` | `role`. For a skip flow or a back flow, `start` and `end`: the span of its optional part or repetition. For a back edge, the span of its reference. For a loop marker or the label of a back edge: `label`, `label-x` and `label-y` | None. Its arguments are the coordinates of its points |

- The coordinates `x` and `y` of a shape name its upper left corner. The point `label-x`, `label-y` is the centre of a label.
- The arguments of a flow are `x1 y1 x2 y2 …`: the corners of the flow, from its source to its target. The arrowhead sits at the last point.
- A flow through the place of a join gateway that is not drawn ends at the right edge of that place. The next flow of the row starts at that point. The page draws no arrowhead where a flow ends at the first point of another flow.
- The roles of a gateway are `choice-split`, `choice-join`, `option-split`, `option-join`, `skip-split`, `loop-join`, `loop-split`, `skip-join` and `back-edge-join`. The span of a `back-edge-join` is the span of the rule name.
- The roles of a flow are `sequence`, `skip`, `back` and `back-edge`.
- The shapes and the flows each come in the order in which the layout places them. That order follows the text.
- A task names the span of its symbol occurrence. That is the span that the tracer names in the origins of a step node. A reference box, a repetition and a back edge also name the spans of their grammar elements. So one identity links each reading to the drawing, as the tracer spec requires.
- At the WebAssembly boundary, the diagrams cross as typed objects, by the record `typed-objects-at-the-webassembly-boundary`. The page never parses the KDL.

An example: the diagram of `setup = install+ configure? ;` in the tutorial grammar. The hash is shortened here.

```kdl
diagrams format-version=1 grammar-sha256="…" font-size=13 cell-width=7.8 line-height=16 {
    diagram rule=setup width=512 height=122 {
        start-event x=16 y=56 size=24
        gateway role=loop-join start=546 end=554 x=64 y=56 size=24
        reference start=546 end=553 rule=install x=112 y=50 width=76 height=36 one-step=#true {
            label install
        }
        gateway role=loop-split start=546 end=554 x=212 y=56 size=24
        gateway role=option-split start=555 end=565 x=260 y=56 size=24
        reference start=555 end=564 rule=configure x=308 y=50 width=92 height=36 one-step=#true {
            label configure
        }
        gateway role=option-join start=555 end=565 x=424 y=56 size=24
        end-event x=472 y=56 size=24
        flow 40 68 64 68 role=sequence
        flow 88 68 112 68 role=sequence
        flow 188 68 212 68 role=sequence
        flow 224 80 224 104 76 104 76 80 role=back start=546 end=554
        flow 236 68 260 68 role=sequence
        flow 284 68 308 68 role=sequence
        flow 272 56 272 16 436 16 436 56 role=skip start=555 end=565
        flow 400 68 424 68 role=sequence
        flow 448 68 472 68 role=sequence
    }
}
```

### The grammar view

- The grammar view shows one rule diagram at a time: the open diagram. Breadcrumbs above it show the open path.
- The open path is a chain of rule names that ends at the open rule. It starts at the start rule, or at a rule that the start rule does not reach. Each rule in the chain refers to the next one. One rule can occur twice, as in `order › outcome › approval › approval`.
- The breadcrumb of the pass rule of a loop through rules shows the loop sign and the current pass, such as `refinement ↻ 2`.
- The header shows the name of the open rule, the badge of its group of recursive rules and the first line of its doc string. A click shows the whole doc string.
- A rule index lists every rule in the order of the text, with its badge. A click opens that rule. The open path is then the first shortest chain of references from the start rule, in the order of the text. A rule that the start rule does not reach opens alone.
- At first, the open diagram fits the width of the view, at a scale of 1 at most. A person can zoom and pan it.
- A click on a box selects it, as the output-view spec states.
- A click on the drill-down marker of a reference box opens the diagram of its rule. A double click on a reference box, or Enter on a focused one, also opens it. A reference to a rule for one step opens in the same two ways.
- A click on a breadcrumb opens its diagram, with the shorter open path.

#### Following a reading

- An operation on the trace is a read, a step back, an insert, a switch of reading or the open of a trace file.
- After each such operation, the open path becomes the follow path of the reading on view.
- The follow path of a reading holds the rules of the rule nodes on its position, from the root. When the last of them is a rule for one step, the follow path leaves it out, unless it is the only one.
- So the follow path always starts at the start rule. In `s = "x" ;`, the follow path after the step `x` is `s`.
- Before the first step, the follow path holds the start rule alone. When the state has no reading, the open path holds the start rule.
- A drill-down, a breadcrumb or the rule index keeps its open path until the next operation on the trace.
- A selection in the output view sets the open path, as the output-view spec states.
- So the tutorial grammar opens `tutorial › lesson` after the step `introduce_concept`, not the diagram of the rule `introduce_concept`.

#### Errors and access

- When the grammar text has an error, the core lays out nothing. The grammar view keeps the diagrams of the last text without errors, dimmed, under a note.
- The open diagram is an SVG with the role `graphics-document` and the name of its rule (Graphics ARIA, layout survey).
- Each box has the role `graphics-symbol` and a label such as "task remind" or "reference approval". The label also names the marks on the box.
- The Tab key moves through the boxes in the order of the text. The end-to-end tests check this structure, not positions.

### Marks of a trace

The marks show the state on view: where each reading stands, the next options and the current pass of each loop. They show no history, because the output view holds the steps so far.

- A reading stands in the open diagram when the rules of the rule nodes on its position start with the open path.
- The diagram node of such a reading is its rule node at the place of the open rule in that chain.
- The first box below a node, on a path, comes from the nodes of the path below that node, from the top. The first of them whose origins hold a box of the open diagram gives the box, or the boxes.

#### Reading markers

- A reading marker is a numbered tab on the top edge of a box, in its free band. The number is the number of the reading in the output view.
- The marker of the reading on view is filled. The marker of another reading is outlined.
- Several readings on one box put their tabs side by side from the left, in the order of their numbers. After three tabs, a tab "+n" counts the rest.
- The marker of a reading sits on the first box below its diagram node, on its position.
- So the marker sits on the task box of the last step in the diagram of the rule that holds the step. In each diagram above it, the marker sits on the reference box that holds the reading. Camunda Cockpit marks a collapsed sub-process in a similar way when a child instance runs in it (layout survey).
- Before the first step, the one reading stands at the start event of the start rule. Its marker sits on the start event.
- The header names each reading that does not stand in the open diagram, such as "Reading 2 stands outside this diagram".

A BPMN token is a dot, or a count badge in a corner. It counts the running instances of a process. Readings are alternatives of one run, not parallel instances. So a reading marker has the form of a numbered tab, and the grammar view never draws a dot.

#### Next-option marks

- A next-option mark outlines an element of the open diagram that a next option passes through. It has the colour of the reading markers.
- The outline is solid for a next option of the reading on view. It is dashed for a next option that only other readings allow, as in the output view.
- A next-option mark on a box or on the end event carries a ▸ button at its upper right corner.
- On a task box, the button appends the symbol of the box. On a reference box or the end event, the button opens a popover. The popover lists the next options that pass through the element, and a click on one appends it. When only one next option passes through, the button appends it directly.
- A next option that starts a new pass passes through a back flow or a back edge. That flow carries the mark too.
- The end event carries a mark when a reading can leave its diagram node. That is so when a next option lies after the node, or when the sequence can end there. The popover lists those next options, and "end" when the sequence can end.
- At the reading cap, a next option of the state can belong to no listed reading, as the tracer spec states. Such an option has no next position, so no element carries its mark. The header lists each such option with a ▸ button, which appends it.

Each next option of a reading leads to one or more next positions. A next position is the position that the reading takes when that symbol is the next step. It keeps the nodes of the reading down to some node, and it adds new nodes below that node. The mark of each next position follows the first of these four rules that applies:

1. The next position keeps the diagram node of the reading. Then the step lies inside that node. The mark goes on the first box below the diagram node, on the next position. A new pass of a repetition of the open diagram on the way also marks its back flow.
2. The next position adds a pass of a loop through rules, whose origins hold a link of the open diagram. The mark goes on that link: a back edge, or a reference box with the loop sign. The next position can also stand in the open diagram. Then the first box below its new diagram node takes the mark too.
3. The reading stands in the open diagram. The step lies after its diagram node, and the end event carries the mark.
4. Only the next position stands in the open diagram. Its new diagram node enters the diagram, and the first box below that node takes the mark.

Before the first step, the diagram node of the one reading is the start rule node. The first step can put that node into a loop, as in `a = a "x" | "y" ;`. Each next position of the first step still keeps the diagram node, for rule 1. The node of the start rule nearest the root takes its place.

The rules mark a path to the next step only inside one node of the open rule, or into the next pass of a loop. In `s = item item ; item = "x" ;`, after the first step, the diagram of `item` marks its end event. It does not mark the box of the second `item`, which belongs to another node.

These cases come from the mock-ups:

| State | Open diagram | Marks | Rule |
|---|---|---|---|
| Tutorial, after `overview install` | `setup` | `install` with its back flow, and `configure` | 1 |
| Tutorial, after `overview install` | `setup` | The end event, for `introduce_concept` | 3 |
| Tutorial, after `overview install` | `tutorial` | `setup`, for `install` and `configure`. `lesson`, for `introduce_concept` | 1 |
| URG, after `revises the solution` in pass 2 | `refinement` | The back edge and `identifies a gap` | 2 |
| `question = "asks" answer ; answer = "answers" question? ;`, after `asks answers` | `answer` | The reference box `question` | 2 |
| Tutorial, after `overview install introduce_concept show_example` | `practice`, opened from the rule index | `set_exercise` | 4 |
| `a = a "x" \| "y" ;`, before the first step | `a` | `y` | 1 |

#### The current pass

- Each loop of the open diagram that holds a reading shows the number of the current pass of the reading on view.
- A repetition shows the pass on its back flow, beside its loop marker, as in `pass 2 · ↻ 1–3`. A self tail call shows it on its back edge, as in `↻ refinement · pass 2`. A loop through several rules shows it in the breadcrumb of its pass rule. So does a self tail call that draws as a reference box.
- The back edge shows the pass only in the diagram of the pass rule. In each other diagram on the open path, the breadcrumb of the pass rule shows it.
- A loop can have a self tail call that draws as a back edge and a link in another rule, as in `a = "x" a? | b ; b = "y" a? ;`. After `x y`, the diagram of `a` shows pass 2 on the back edge. The diagram of `b`, opened as `a › b`, shows it in the breadcrumb `a`.
- A repetition holds a reading when its loop node lies on the position, between the diagram node and the box of the marker. A self tail call holds a reading when the diagram node lies in a pass of its loop.
- Earlier passes leave no mark. On 2026-11-17 the human described a grammar view that "shows only the current pass of a loop", and the output view folds the completed passes.

#### The failure point

At a failure point, the marks are those of the state before the rejected step, as the output-view spec states. The banner of that spec stands above the open diagram.

#### The selection and the data object

- A selected element has an amber outline, distinct from the marks. A selected box gets the outline. A selected repetition outlines its gateways and its back flow, and a selected back edge outlines its path.
- A click on a gateway or on the back flow of a repetition selects the repetition. A choice gateway, an optional part and the events select nothing, as the output-view spec states.
- The data object of the step in focus sits in the free band above the box that stands for the step. That box is the first box below the diagram node, on the path from the root to the step. A dotted association joins the data object to the box.
- The output-view spec states the content of the data object and its click.

### The interface of the core

The grammar view adds three parts to the interface of the core. The record `typed-objects-at-the-webassembly-boundary` holds for each part.

| Part | Effect |
|---|---|
| Diagrams | Takes a grammar text without errors. Returns the diagram of each rule, as typed objects or as the diagram KDL |
| Marks | Takes an open path and, optionally, the number of the step in focus. Returns the marks of the open diagram for the state on view |
| Follow paths | The state on view holds the follow path of each listed reading |

The marks of the open diagram hold these values:

- The readings with marks: each listed reading that stands in the open diagram, or that has a next position in it.
- For each such reading: its number and the elements of its marker.
- For each of its next options with a mark in the open diagram: the symbol and its elements.
- Whether the end event carries a mark for the reading, with the next options that it lists, and whether the sequence can end.
- The current pass of each loop of the open diagram that holds the reading.
- The element of the data object, when a step is in focus and has a box in the open diagram.
- The next options of the state that no listed reading allows. The header lists them.

- An element names its kind and its span. The kinds are task box, reference box, back flow, back edge, start event and end event.
- The core computes the marks of each listed reading exactly. The reading cap does not cut the next positions that the marks need.
- So the marks cannot come from the listed readings after a next step, because the cap cuts that list. In `s = "seed" (r1 | r2 | … | r12) ;` with `r1 = "next" ;` to `r12 = "next" ;`, the one reading after `seed` has twelve next positions. The cap lists ten readings after `next`.
- The section "Next positions" states how the core finds every next position of a listed reading. The section "Cut successors" states how Marks read them without a list of every successor. The section "The cost of Marks" states what that costs.
- The alternatives depend on the state on view alone, so the core keeps them for each state. The cut successors depend on the open path too, so each call of Marks evaluates them again.
- The page asks for the marks after each operation on the trace. It asks again after each change of the open path or of the step in focus.
- An open path is a chain of references. It must start at the start rule, or at a rule that the start rule does not reach. Any other open path is an error.
- The references of an open path are the references of the rule expressions, as the diagrams draw them. A reference in a part that derives no finite sequence still counts. In `s = a | "y" ; a = "x" a ;`, the tracer never enters `a`, but `s › a` is an open path. The start rule reaches `a`, so the open path `a` is an error.
- No reading stands in a diagram whose open path starts at an unreachable rule, and no next position enters it.
- The core writes the marks as KDL for the tests only. The trace file does not hold them, because they describe a view of a run, not the run.

The core holds the trees, the next positions and the elements of each diagram, so it computes the marks. The core seam can then test the rules for marks against expected files.

Rejected: the page derives the marks from the positions of the readings. The page would need the next positions and the rules of the trees as well. Only end-to-end tests could then check the rules, and those tests check structure, not each case.

#### Next positions

The core finds the next positions of a listed reading from the derivations of that reading alone. The readings after a next step cannot supply them, because the reading cap cuts that list. The record `readings-from-a-shared-forest` holds this decision. The tracer spec defines the open layer, its open nodes and their projections.

- Each projection of an open node keeps its alternatives. An alternative is an item of the last Earley set, with the content that ends at that item.
- An alternative can also be a child projection, with the content before it and the reference that made the child.
- Equal projections join their alternatives, as they join their origins.
- A route of a reading is a chain of alternatives from the reading at the open root down to an item of the last set. It stands for partial derivations of the reading that share one tree.
- Each edge from the item of a route to an occurrence of a next option gives one successor of the reading. A successor is the tree of the reading with that step added.
- The successor takes the content of the item, the loop events of the edge and the new step. Then each alternative on the way back to the open root adds its content and its reference, by the tree rules.
- So a successor takes the origins of its own route only. A node that it keeps can have fewer origins than in the reading, where equal projections joined them.
- Equal successors join their origins, as equal projections do. A next position is the position of one distinct successor, with its joined origins.
- The core never lists these successors, because they can be too many. The section "Cut successors" states what Marks list instead.
- The open layer can hold a cycle. A trip around the cycle adds only passes without a step, as the tracer spec states. So a route with a trip gives the same successor as the route without the trip. The section "Cut successors" states how the evaluation treats a cycle.

The node that a next position keeps last follows from the successor:

- After the first step, a successor holds the nodes of its reading, and then a chain of new nodes that hold only the new step. The first new node hangs below the node that the successor keeps last.
- Before the first step, the reading is the start rule node alone. A successor can turn that node into a loop node. Then the node of the start rule nearest the root takes its place, as the section "Next-option marks" states.

An example shows why a route keeps its own origins. In `s = "y"{2,} | "y"{1,3} ;` after `y y y`, both repetitions give one tree. So the loop node of the reading joins the spans of both repetitions. Only `"y"{2,}` allows a fourth pass, so the successor through `y` keeps the loop node with the span of `"y"{2,}` alone. Rule 1 then marks the task box and the back flow of `"y"{2,}`, and no element of `"y"{1,3}`.

#### Cut successors

The successors of one reading can be too many to list. In this family of depth 3, the one reading after `seed` has 2³ successors through `next`:

```text
%start s ;
s = "seed" (r0 | t0) ;
r0 = r1 | t1 ;
t0 = r1 | t1 ;
r1 = r2 | t2 ;
t1 = r2 | t2 ;
r2 = "next" ;
t2 = "next" ;
```

At each level, a successor chooses between two rule names, and different rule names give different trees. At depth d, the family has 2d + 1 rules, and the one reading has 2^d successors. In the start diagram, all of them mark only the reference boxes `r0` and `t0`.

So Marks list cut successors instead. A cut successor is a successor without the new nodes that no rule for next-option marks reads.

- A cut successor keeps the nodes of the reading down to the node that the successor keeps last. These nodes take the origins of the route.
- It keeps each new node that comes from an open node that holds a step. In an added pass of a loop of head references, these nodes hold the chain of links of the pass.
- Below them, it keeps the first two new nodes. These come from open nodes that hold no step. The second one keeps the pass rule node of a new pass. The tree rules need that node when a loop node takes the pass as its first pass.
- Below those, it keeps only the rule nodes that continue the open path. When they reach the open rule, it also keeps the first node below the new diagram node that is not part of a repetition.
- Equal cut successors join their origins, as equal successors do. Their lists have no cap.
- Marks evaluate the cut successors along the routes of the reading, as the section "Next positions" evaluates the successors. An open node that holds no step cuts its successors as soon as it makes them.
- The progress of a node is the count of rules of the open path that the path above the node matches. A path that leaves the open path has no progress.
- Each alternative passes the progress of the child to the child. It also passes the progress at the loop node of each loop through rules around the child.
- A child that starts a later pass takes the progress at the loop node, because its earlier passes are not on its path. A loop that the child enters is new, and it takes the progress of the child.
- The evaluation keeps a list of the cut successors of each projection for each progress. It fills the lists in rounds, as the core evaluates the open layer.
- In one round, the evaluation visits each projection once for each progress. When the round meets a projection that it is still evaluating, that projection gives the list that it has so far. The rounds repeat until no list changes.
- The lists only grow from one round to the next, and they are finite. So the rounds stop.
- The final lists hold the cut successors of every route. A route with a trip gives the same cut successor as the route without the trip, so the trips add nothing.
- Without the rounds, the list of a projection on a cycle is wrong or costly. One pass over the layer gives a list that lacks the routes around the cycle through the projection where the pass entered it.
- An evaluation that keeps no such list repeats almost every visit in a grammar where most routes meet a cycle. Its visits grow exponentially with the length of the sequence.

#### Why cut successors give exact marks

The four rules read five parts of a next position. These are the node that it keeps last, the first box below a diagram node, and the new passes of repetitions on the way. They are also the rules of its rule nodes, and the origins of a pass of a loop through rules that it adds. A cut successor keeps each of these parts.

- The first box below a node comes from one node below it. That node is the first one that is not a loop node or a pass node of a repetition.
- It is a step, or a node that a reference of the open rule makes. So its origins are symbol occurrences or references of the open rule, and each of them is a box.
- So the first box lies at the same node on every route of one successor. The join of equal successors only unions its boxes.
- Each other part is a fact of the tree, or a test whether the origins of a node hold an element of the open diagram. So for each rule but one, the marks of a join are the union of the marks of its routes.

The exception is the order of rules 2 and 3. Rule 2 tests whether the origins of an added pass hold a link of the open diagram. When they hold none, rule 3 can mark the end event instead.

- In `a = a "x" | b "x" | "y" ; b = a ;` after `y`, one successor through `x` comes from two routes.
- Its added pass takes the reference `a` in the rule `a` from one route. It takes the reference `a` in the rule `b` from the other.
- In the diagram of `a`, the joined next position marks the link `a` and both boxes `x`. The route through `b` alone would mark the end event instead of the second box `x`.
- So the origins of an added pass must join over the routes of each successor before the rules apply.

The cut keeps that join exact:

- An added pass is always the first new node, below the loop node that the successor keeps last. Its origins are links: the last link of its chain of links, or the reference that makes the pass.
- In a loop of head references, the nodes of the added pass down to its last link come from open nodes that hold steps. So the cut keeps them.
- When the rule node that holds the last link makes a node, the cut shows its rule. So equal cut successors take links in the same rule.
- When that rule node makes no node, the routes of equal cut successors differ only in nodes that make no node. Below the chain, the first new node comes from an open node that holds no step.
- That first node follows the earlier passes, so its reference links no loop. So its loop context and its roles depend on its rule alone, and these routes have the same continuations below it.
- In a loop of tail references, the reference that makes the pass rule node is the link. The pass rule node comes from an open node that holds no step, and the loop node fixes its loop context and its roles. So all routes to it have the same continuations.
- Take one set of equal cut successors. Either every successor of the set has a route whose added pass holds a link of the open diagram, or none has. So the gate of rule 2 is the same for each of them.

The prototype of `exact-marks-k44` compared the marks of the cut successors with the marks of an uncapped next step. It found no mismatch in 132,755 comparisons, of which 83,092 had marks. Of these, 715 came from 19 written cases, which hold the cases of this section. The rest came from 2,000 random grammars of the property test in the section "Test seams". The test skipped 37 options, because the uncapped step gave 3,000 readings or more.

#### Why the capped lists keep every next position

- A next position comes from a route of the reading and one edge to a next option.
- The section "Why the capped lists are exact" of the tracer spec shows that each derivation of a listed tree combines projections from the capped lists only. It holds for each derivation of the tree, not for one only.
- So each route of a listed reading runs through alternatives that the capped lists keep.
- The cut successors follow those alternatives, and their lists have no cap. So no cap removes a next position, and the four rules see each one.
- In the case of twelve rules, the one reading after `seed` has an alternative for each reference box from `r1` to `r12`. Each alternative gives one cut successor, so the core marks all twelve boxes.

#### The cost of Marks

The cost uses the terms of the tracer spec. Here n is the number of steps read, C is the reading cap, and m is the length of the open path.

- One call of Marks evaluates the cut successors of each listed reading through each of its next options. The tracer lists at most C readings.
- The evaluation visits only projections on the routes of the reading. Each of them is a projection in the list of an open node, which holds at most C + 1 projections for each split. The tracer spec shows that the open nodes and the splits stay polynomial in n.
- In each round, it visits each such projection once for each progress. A progress takes at most m + 2 values at the node and at each loop node around it. So a projection has at most (m + 2)^(λ + 1) progresses, where λ is the largest count of loops through rules around one node.
- Each round that changes a list adds a cut successor or an origin to it. So the rounds are at most one more than the count of cut successors and their origins in all lists. That count is polynomial, as the next points show. In the prototype of `exact-marks-k44`, no search took more than 4 rounds, on the written cases and on the 2,000 random grammars.
- At each projection, the evaluation combines each alternative with the cut successors of its child.
- The cut successors of one projection differ only in three things. These are the node that they keep last, their new nodes down to the cut, and the place where they leave the open path. So one projection keeps at most P × H × (m + G) cut successors.
- Here P is the length of the path of the reading. H counts the chains of new nodes down to the cut that can start below one node, and G counts the labels of a node. H, G and λ depend only on the grammar.
- A cut successor holds the tree of the reading, so its size grows with that tree, as the size of a reading does.
- So for one grammar, the work of one call grows polynomially with n and with m. The count of readings of the state does not enter.
- In the size of the grammar, the work grows with the size of the open layer, with H and with (m + 2)^(λ + 1). λ is the deepest nesting of loops through rules.
- H can grow exponentially with the longest chain of links of a loop of head references, because the cut keeps that chain whole. Below the chain, the cut keeps only two nodes and the rules of the open path.
- The family of the section "Cut successors" has no loop, so λ is 0 and H is 4 there. The prototype found 4 distinct pairs of first new nodes at each depth and on each open path that it measured.
- In a grammar without self-embedding, P stays within a number of the grammar, because each loop adds passes, not deeper nodes. With self-embedding, P can grow linearly with the length of the sequence.
- The prototype of `exact-marks-k44` measured the family of the section "Cut successors". At depth 64, with 129 rules and 2^64 successors of one reading, one open path took at most 4.4 milliseconds. The evaluation visited at most 503 projections, and the open root kept at most 66 cut successors. These counts grow linearly with the depth.
- On the open paths `s` and `s › r0`, the open root kept 4 cut successors at each depth. On the open paths through `r63`, it kept d + 2.
- At depth 16, an uncapped next step took 15.5 s to list the 65,536 successors of that reading.
- In `r0 = (((r0 | "y") | r0)){2} ;`, almost every route meets a cycle of the open layer. After 12 steps `y`, one call for the open path `r0` visited at most 102 projections in 2 rounds. It took at most 113 ms, and the readings of that state took 174 ms in the tracer.
- In the same grammar, an evaluation that kept no list for a projection on a cycle repeated almost every visit. Its visits grew tenfold for each step, to 555,555 after five steps.

## Test seams

The work tests through the two seams that the human agreed on 2026-11-17. This design adds no seam.

1. **Core interface.**
   - A diagram case gives a grammar file and compares the diagram KDL with an expected file. Each row of the table of constructs has a case. Further cases cover a back edge above the body and a join gateway with one flow. The three example grammars have expected diagram files.
   - Two diagram cases come from the design review. In `r = "very_long_symbol" | "b" r? | "c" ;`, the self tail call draws as a reference box. The label `aaaaaa bbbbbb cccccc` breaks into `aaaaaa bbbbbb` and `cccccc`.
   - A diagram case from the implementation review checks the spaces of labels. The symbols `long_symbol` and `long_symbol   ` get different labels, and a break after a run of spaces drops the whole run.
   - A stability test edits one part of a rule. It checks that no element changes its order, and that the other rules keep their diagrams, apart from the exception in the section "The layout". Its edits change the one-step fact of a reference, close a loop through rules and remove the back edge of a self tail call.
   - The stability test checks order by the drawing, not by the list of shapes. Each pair of shapes keeps a strict relation along one axis: left of, right of, above or below. Each flow keeps the shapes at its two ends.
   - Each diagram case also checks three invariants. No two boxes overlap. The free band above each box is empty. No flow crosses a box.
   - A marks case gives a grammar file and a sequence. Its expected file holds the marks for each open path along the follow path of each listed reading. A case can name more open paths, such as a diagram that only a next position enters.
   - The marks cases cover the four rules for next-option marks, a self tail call, a loop through two rules, two readings and a failure point.
   - Further marks cases cover these states:
     - An open path that starts at a rule that the start rule does not reach.
     - The step of `s = "x" ;`, after which the follow path is `s`.
     - A next option that only an unlisted reading allows.
     - The first step of a loop at the start rule, in `a = a "x" | "y" ;`.
     - One reading with more next positions than the cap, as in the example of twelve rules above.
     - A successor that keeps fewer origins than its reading joined, in `s = "y"{2,} | "y"{1,3} ;` after `y y y`. The marks of `y` hold only the task box and the back flow of `"y"{2,}`.
     - One successor from two routes whose added passes take links in different rules, in `a = a "x" | b "x" | "y" ; b = a ;` after `y`. In the diagram of `a`, the marks of `x` hold the link `a` and both boxes `x`, and not the end event.
     - An added pass whose last link lies deeper than its first three new nodes, in `a = b | e | "y" ; b = c | d ; e = c ; c = a "z" ; d = a "z" ;` after `y z`. For the reading `a › b › c` and the open path `a › b › c`, the marks of `z` hold the link `a` in `c` and the box `z` in `c`. They also hold the end event, from the successor through `b › d`.
     - The family of the section "Cut successors" at depth 64, which has 129 rules. After `seed`, the open path `s` marks the reference boxes `r0` and `t0` for `next`. The open path from `s` through `r0`, `r1` and on to `r63` marks the task box `next` of `r63`.
     - A grammar in which almost every route meets a cycle of the open layer: `r0 = (((r0 | "y") | r0)){2} ;` after 12 steps `y`. Its open path is `r0`.
     - A reference in a part that derives no finite sequence, in `s = a | "y" ; a = "x" a ;` after `y`. The open path `s › a` has no reading marks, and the open path `a` is an error.
     - A loop with a back edge and a link in another rule, in `a = "x" a? | b ; b = "y" a? ;` after `x y`. The open path `a` shows pass 2 on the back edge, and `a › b` shows it in the breadcrumb `a`.
     - A self tail call that draws as a back edge, with a diagram open below its pass rule. In `r = x r? | "c" ; x = "b" "d" ;` after `b d b`, the open path `r › x` shows pass 2 in the breadcrumb `r`.
   - A construction that lists every successor needs 2^64 trees for the one reading of the family at depth 64. So it runs out of memory before it can pass that case, and the case needs no deadline.
   - An evaluation that keeps no list for a projection on a cycle does not end the case of 12 steps. Its visits grew tenfold for each step. By that growth, 12 steps need about 5 × 10^12 visits.
   - So the case of 12 steps has a deadline of 60 seconds. The test runs the case in a worker thread, and the test thread waits for the result until the deadline.
   - The worker thread reads the grammar and the sequence itself, and it returns the marks KDL as text. The syntax trees of `rowan` cannot move to another thread.
   - At the deadline, the test fails, and its message names the case. The test harness ends the process with a nonzero status after its other tests. It does not wait for the worker thread.
   - The deadline is in the test, so `task test` and `task check` enforce it with no change. Plain `cargo test` has no deadline of its own. After 60 seconds, it only reports that a test still runs.
   - The deadline separates the two evaluations by a wide margin. The core builds at `opt-level = 2` in tests, as in the release runs of the prototype. There, one call of Marks on this case took 113 ms at most.
   - The deadline does not replace the expected marks. The case also compares its marks KDL with its expected file.
   - The implementation shows once that the deadline catches the costly evaluation. With a temporary evaluation that keeps no list for a projection on a cycle, `task test` must fail the case at the deadline.
   - A property test checks the marks on the random grammars of the tracer spec, for prefixes of at most 6 steps. It takes each listed reading, each of its next options and each open path of at most 3 rules from the start rule.
   - For each such case, it traces the prefix with that step added, with lists that have no cap. The readings after the step whose parent is the listed reading are the successors.
   - The test applies the four rules to those successors, and it compares their marks with the marks of the cut successors. It skips an option when the step without a cap gives 3,000 readings or more, and it reports each skipped option.
   - A second set of random grammars adds a bare reference to another rule as an alternative of some rules. So more loops through rules gain links that make no node.
2. **Workbench page.**
   - End-to-end tests open one diagram for each rule through the rule index. They open a reference box and go back through the breadcrumbs.
   - They build a sequence of each example grammar by clicks on ▸ buttons alone. They check the reading markers, the next-option marks and the pass labels by structure.
   - They check the selection in both directions, and the data object.
   - One test checks that the page draws labels in the bundled font. The width of a label must equal its cells times 7.8 px, within 1 px.

## Out of scope

- Dragging, and the expansion of a reference in place. The record `rule-diagrams-in-bpmn-style` keeps drill-down only.
- One flattened diagram for a group of rules without self-embedding. It comes after the slice.
- History on the diagrams: trails of earlier steps, counts of visits or heat maps. The output view holds the history of one run. Counts over many runs come after the slice.
- Line breaks in a long row of items, as librrd makes them. A wide diagram scrolls and zooms instead.
- Export of a diagram as SVG.
- A text mode of the grammar view. On 2026-11-20 the human described it as program text: whole constructs with implicit structure, and embedded diagrams for some parts of the grammar. The presentation holds a first sketch, which draws more structure than that.

The design rejects these alternatives:

- Mixed gateways. A join gateway that leads straight to a split gateway could merge with it, as in `("adds a complication" | fix)*`. That would save one diamond on each side. The pairs stay apart, because each construct keeps its own gateways for the selection, and guideline G4 asks for matched pairs.
- The while form of `x*`: a join gateway and a split gateway before the item, and a back flow from the item. It saves two gateways, but the item then has no flow forward on the main line.
- A general route for each back edge, around the boxes of the body. A route along the row of its reference overlaps the forward flows on that row. Columns kept free for back edges would add space at each level of choices, and they need their own proof. A self tail call without a straight route draws as a reference box instead.
- A bound drawn only as a loop marker on its item, without gateways. Each loop would then draw in one of two ways, and a bound would show no path back to the start of a new pass.
- A proportional font with a width table. The record `bundled-monospace-font-for-diagram-labels` gives the reasons.
- A dot or a count badge as a reading marker. Both are the forms of BPMN tokens.
- Next positions from the readings after a next step. The reading cap cuts that list, as the case of twelve rules shows.
- A trace of each next option with lists that keep every successor of their first parents. It needs one evaluation of the forest for each option. It also finds the successors of readings that are not listed.
- Lists of every successor of a listed reading, as an earlier version of this design specified. The family of the section "Cut successors" gives one reading 2^d successors with 2d + 1 rules. At depth 30, the tracer finds the readings after `seed next` in 5.8 ms, but such a list needs 2^30 trees.
- The union of the marks of each route, without the join of equal successors. One successor can add a pass from routes with links in different rules. The union then gives other marks, as the section "Why cut successors give exact marks" shows.
- A limit on the grammars that the workbench accepts. The cut successors keep exact marks within a polynomial bound, so no limit is necessary.
- Successors cut to a fixed number of new nodes. The link of an added pass can lie deeper than any fixed number of new nodes. Two successors that need different marks then share one cut.
