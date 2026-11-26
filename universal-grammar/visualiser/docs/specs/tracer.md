# tracer

## Problem

The tracer reads a sequence against a grammar, one step at a time. After each step, the workbench must show every reading up to the reading cap, where each reading stands, and the next options. When no reading accepts a step, the workbench must show the failure point and keep the state before it. A person can step back. After the slice, other programs will drive the tracer step by step.

The decision record `one-earley-tracer` gives the frame: one Earley parser over one position automaton for each rule. The record `trees-follow-rules-and-passes` and the spec `output-view` define the trees of the readings, with loop nodes and passes. The output-view spec also lists what the output view needs from a trace.

Two risks shape this design. Scott (2008) showed that Earley's own method for trees "is incorrect". A tracer that keeps only its first readings can also drop the reading that a later step needs. Readings can grow exponentially: `("a" | "a" "a")*` gives 89 readings for ten steps.

A trace file serves as a test fixture, a saved session and an export. Other programs, such as LLM tools, write sequences as trace files. This spec defines the tracer, the trace file and the interface between the core and the workbench page.

## Solution

The core compiles each rule to a position automaton. A state is a symbol occurrence or a reference, with a pass count for each bounded repetition around it. Each edge carries the loop events that it crosses. The tracer adds one Earley set for each step that it reads. The next options come from the last set, and a step back drops that set.

While the sets grow, the core builds a shared packed forest by the method of Scott (2008). A projection turns each derivation of the forest into a tree, by the tree rules of the output-view spec. Each forest node keeps its first distinct trees in one deterministic order. So the core lists the first readings without a list of every derivation. The reading cap is 10.

A trace file is a KDL document. Another program writes only its root node and its sequence. The core derives the rest: the next options after each step, the failure point, and the readings with their trees. The page calls the core through typed objects, and it never parses KDL.

Each tree node names the grammar elements that it came from by their spans in the grammar text. The checked grammar and the rule diagrams use the same spans. So one identity links a reading to a task box.

The presentation `docs/design/tracer` shows the design as diagrams.

## Decisions

### Grammar elements

- A grammar element is a symbol occurrence, a reference or a repetition in a rule body.
- The identity of a grammar element is its span in the grammar text: the byte offsets `start` and `end`. The checked grammar writes the same span for the element.
- In a grammar without errors, no two expressions have the same span. The start offset alone is not an identity, because a repetition and its item can start at the same byte. The checked grammars of the repository hold 521 expressions without a shared span.
- The rule-diagram design links each task box to the span of its symbol occurrence. It links each reference box to the span of its reference.
- An identity changes when the grammar text changes. Every derived artifact records the hash of its grammar text, and the core derives each artifact again after an edit.

Rejected: an identity from the rule name and a path of child indices, such as `lesson.2.1`. It survives an edit in another rule, but no artifact needs that. It would also add a second identity beside the spans that the checked grammar already writes.

### The automaton of a rule

- Each rule compiles to one position automaton. Its positions are the symbol occurrences and the references of the rule body. Its states are a start state, the positions and an end state.
- An edge leads from a state to each position that can come next, or to the end state.
- An edge carries the loop events of its path through the rule body, in order. A repetition starts with its first pass, a new pass starts, or the repetition ends.
- Two paths with the same events make one edge. Two paths with different events make two edges, because they give different trees. In `("x"+ "y"?)+`, two edges lead from `"x"` back to `"x"`. One starts a new pass of the inner repetition. The other ends the inner repetition and starts a new pass of the outer one.
- A state also holds a pass count for each bounded repetition around its position. The tracer counts passes in the state, and it never copies the body of a repetition.
- An edge that starts a new pass needs a count below the greatest number of the bound. An edge that ends the repetition needs at least the least number.
- An open bound `{m,}` counts to m and then stays at m. `*` and `+` need no count.
- So one position has at most as many states as the count of its bounds under the bound cap, which is 100.
- The core removes each part of a rule that can derive no finite sequence. An example is each reference to `a = "x" a ;`. After the removal, every state can reach the end state.

The edges with loop events follow Thompson's construction: each edge stands for one path through the junctions of the rule body. The automata survey shows that one path of a position automaton can give two trees. The events keep those trees apart.

### Recognition

- The tracer keeps one Earley set for each count of steps read, from set 0 for the empty prefix. An item names a rule, a state of the automaton of that rule, and the origin. The origin is the number of steps before the rule node of the item started.
- Prediction, scan and completion follow the edges of the automata. A rule that can match the empty sequence completes as soon as the tracer predicts it. So the next options pass through such rules and through rule ends, to the rules that wait for them.
- The tracer accepts a step when an item of the last set has an edge to an occurrence of its symbol. The tracer then adds the next set.
- The failure point is the first step that no item accepts. The tracer keeps the last set, so the state on view is the state before the rejected step.
- A step back drops the last set, and the forest nodes that end at that step. The state is then the same as the state of a new trace of the shorter sequence.
- The sequence can end after the steps read when the last set holds the start rule at its end state, with origin 0.

The removal of dead parts gives the tracer the correct-prefix property: each accepted prefix starts at least one sequence that the grammar accepts. Without it, `s = a | "y" ; a = "x" a ;` would accept `"x"`. It would then offer `"x"` after every step, although no sequence that starts with `"x"` can end. If the start rule derives no sequence, the tracer has no reading, even before the first step.

The tracer does without the method of Leo, which keeps right recursion linear. At 300 steps, a right recursion costs about 90,000 item steps (automata survey).

### Next options

- The next options after the steps read are the symbols on the edges out of the states of the last set, after prediction. An edge counts only when the pass counts of its state allow it.
- Each next option names the symbol occurrences that offer it.
- Next options come in the order of the symbols in the checked grammar, which is the order of their first use in the text. Occurrences come in the order of the text.
- Each reading names the next options that it allows, with the occurrences that offer them in that reading. A next option that only unlisted readings allow is a next option of the state, but of no listed reading.

### Readings and the forest

The decision record `readings-from-a-shared-forest` holds the choice of method. This section states how it works.

- While the sets grow, the core builds a shared packed forest by the method of Scott (2008). The states of the automata take the place of the dotted rules.
- The forest has a node for each step and a node for each completed rule over each span of steps. It also has an intermediate node for each state of a rule over each span. A packed node records each alternative, with the edge that it took.
- The completed part of the forest has no cycle. The rule checks reject a rule that can derive itself with no step, and a repeat whose body can match the empty sequence.
- For the state after the steps read, the core adds an open layer, as the section "The open layer" states. The core builds the layer again after each operation.
- A projection turns a derivation into a tree, by the tree rules of the output-view spec. A completed rule makes a rule node, unless it holds no step. The loop events of the edges make the loop nodes and the passes of repetitions. The section "Loops through rules" states how references make loops and passes.
- A reading is one distinct projection. Equal projections are one reading, and their origins join.
- Each forest node keeps its first C+1 distinct projections in the order of the section "The order of readings", where C is the reading cap. A node merges the lists of its alternatives. A pair of child nodes combines their lists.
- The projection of a node depends on its loop context, as the section "Loop contexts" states. So a node keeps one list for each loop context in which the open layer reaches it.
- The readings after the steps read are the first C projections of the open root. When projection C+1 exists, the tracer reports that it reached the reading cap.
- Each listed reading keeps the open nodes of its partial derivations. Its next options, and whether it can end, come from them.
- Each projection of an open node keeps its alternatives: the items of the last set and the child projections that give it. The spec `rule-diagrams` finds the next positions of a listed reading from them, in the section "Next positions".

When the tracer reaches the reading cap, acceptance, the failure point and the next options stay exact, because they come from the Earley sets. Only the list of readings stops at the cap. The next positions of each listed reading stay exact too, because they follow the alternatives of that reading.

#### Why the capped lists are exact

- In each combination of two child nodes, the nodes from the left child come before the nodes from the right child in pre-order. In a loop of head references, the passes so far come before a new pass. In a loop of tail references, a pass comes before the passes that its tail reference starts.
- The nodes that a combination adds itself, such as the node of a new pass, are the same for each pair from the two lists.
- Two different projections of one child end with the same step: the last step of its span. So neither projection is the start of the other.
- So two combinations compare first by their left children, and then by their right children.
- So a combination among the first C+1 of a node takes a left projection among the first C+1 of the left child. It also takes a right projection among the first C+1 of the right child. A merge keeps the first C+1 of each alternative in the same way.
- This holds for each derivation of a listed tree, not for one derivation only. So each listed tree joins the origins of every derivation that gives it. Its next options, and whether it can end, are exact too.
- The argument needs one condition: the two projections split their steps at the same place. A node on a chain of head links carries the passes so far that it found below it. Two of its projections can end those passes at different steps, and then the nodes above the node decide their order. So each forest node keeps the first C+1 projections of each split. The split names, for each loop of head references around the node, the step where the passes so far end. For each loop of tail references, it names the step where the later passes start.
- A split names at most one step for each loop around a node, so the lists stay polynomial in the length of the sequence. The property tests compare the capped lists with uncapped lists. Without the splits, that comparison fails after two steps.

#### Loop contexts

- The loop context of a node is what the tree rules need to know about the ancestors of the node. It names each loop through rules that holds the node, with the kind of loop and the pass rule.
- For each such loop, the context also names the place of the node. The node enters the loop, starts a pass, holds the passes so far, or lies inside a pass.
- A context names each pass rule at most once. Inside one pass, a reference to the pass rule is a link of the loop. Otherwise it enters a new loop, by rule 1 of the section "Loops through rules". So a nested loop avoids the pass rule of each loop around it.
- A reference that enters a new loop by rule 1 starts a new context. No link inside the new loop reaches a loop around it. A chain of head links stops at a reference with a step before it. A chain of tail links stops at a reference that is not a tail reference.
- So a grammar has finitely many loop contexts.

#### The open layer

- An open node names a rule, a state of the automaton of that rule and an origin, as an item does. It stands for the part of a rule node up to that state, which ends after the last step.
- Each item of the last set is an open node. Take an open node of a rule R with origin i. Each item of set i with an edge on a reference to R is a caller of it.
- Through that edge, the caller makes an open node of its own rule, at the state of the reference. The last child of that open node is any open node of R with origin i.
- The chain of callers ends at the start rule with origin 0: the open root.
- The layer has at most one open node for each rule, state and origin, so it is finite. Unfolded, it holds every partial derivation of the steps read.
- The layer can hold a cycle. In `a = a "x" | "y" ;` after `y`, the open node of `a` at the reference `a`, with origin 0, waits for `"x"`. Its last child can be that same open node. Unfolded, the cycle gives `a => a "x"^n => "y" "x"^n` for every n.
- A cycle keeps one origin, because the origin of a caller is never greater than the origin of its child. So each caller on the cycle holds no step before its reference, and the reference acts as a head reference. The cycle follows a cycle of head references in the grammar.
- Each reference on the cycle acts as a head reference, so it is a link of a loop of head references. It never enters a new loop by rule 1. A head reference enters a new loop only with a step before it. By rule 3, a cycle of tail references through its target makes no loop.
- So each trip around the cycle adds passes to loops of head references. Every step of such a pass comes after the last step. So the pass holds no step and makes no node, and a trip leaves the projection unchanged.
- So the core evaluates the layer without the trips around its cycles. The list of each open node on a cycle holds the projections of its derivations that take no trip.
- A reading with an open node on a cycle also has the partial derivations that take the trips. So its open nodes include each open node on the cycle, and each adds its next options and its way to end.
- In the example, the one reading of `y` can end, and it offers `"x"`.

The fixtures of the section "Test seams" hold three cases of the open layer:

| Case | Grammar | Steps | The reading |
|---|---|---|---|
| Direct head recursion | `a = a "x" \| "y" ;` | `y` | One cycle passes through `a`. One reading, which can end and offers `"x"`. |
| Indirect head recursion | `a = b "x" \| "y" ; b = a "z" ;` | `y` | One cycle passes through `a` and `b`. One reading, which can end and offers `"z"`. |
| A sub-cycle | `a = b "x" \| "z" ; b = a "y" \| b "z" \| "x" ;` | `z` | One cycle passes through `a` and `b`, and another through `b` alone. One reading, which can end and offers `"y"`. |

### The order of readings

- Readings come in one deterministic order. The order depends only on the trees, never on derivations or origins.
- Two trees compare node by node, in pre-order. At the first node that differs, the tree whose node is deeper comes first.
- At equal depth, a loop node comes before a pass node, and a pass node before a rule node. A rule node comes before a step node. Rule nodes compare by the order of their rules in the grammar text. Loop nodes compare by their kind of loop: repetition, tail references, then head references.
- A new step adds nodes only at the end of the pre-order, and it never changes an earlier node. So the readings after step k keep the order of the readings that they extend, as the output-view spec requires.
- The deeper node first means that a reading that adds to an open node comes before a reading that opens a new node. In `("a" | "a" "a")*`, the readings of `a a a` come in this order: `[a a] [a]`, `[a] [a a]`, `[a] [a] [a]`.

### The parent of a reading

- Before the first step, the one reading is the start rule node alone, when the start rule derives a finite sequence. Otherwise no reading exists, as the section "Recognition" states.
- Each reading after step k names its parent: the reading after step k−1 that it extends. Without step k and the nodes that hold only step k, the tree of the reading is the tree of its parent.
- A reading names its parent by its number in the list after step k−1. When the parent is not in that list, because of the cap, the reading names no parent.

### Loops through rules

The output-view spec states the rules for loops through rules and their passes. The tracer adds three rules. The spec `output-view` and the record `trees-follow-rules-and-passes` now carry them too.

1. A reference enters a new loop when it is not a link of the loop of its target. For example, in `e = "x"? e "y" | "z" ;`, the reference `e` acts as a head reference only when `"x"` is absent. With `"x"`, it enters a new loop inside the rule node of `e`.
2. In a loop of head references, the node of the first pass has the references that entered the loop. A loop of tail references does the same. Each later pass and its node have the head references that link the pass to the loop so far. In a loop through several rules, the origin is the last link of the chain. That link names the pass rule. The other nodes of the pass keep the reference that made them. So a later step never adds an origin to a node.
3. A cycle of tail references makes no loop when one of its rules lies on a cycle of head references. Its tail references then make nested rule nodes.

Rule 3 settles the point that the output-view spec left open. In `a = a "x" | "y" a | "z" ;`, the rule `a` lies on a cycle of tail references and on a cycle of head references. Only the cycle of head references makes a loop.

- The loop of head references is necessary. Without it, the depth of the left spine would depend on steps that have not come yet.
- Nested rule nodes of tail references also grow at the end. So the property "Trees grow at the end" of the output-view spec still holds.
- Both loops cannot nest in one deterministic order. A pass would then hold a loop node, not one node of its pass rule.
- The cost is that such a rule loses the folds of its tail passes. Such a grammar is self-embedding, and its badge is "may need a stack". No example grammar has such a rule.
- The grammar has several derivations for one sequence, and each distinct tree is a reading. In the example, `y z x` has two readings, because `x` can start a pass of the outer loop or of the inner loop.

### The reading cap

The reading cap is 10. The evidence:

- The example grammars give at most two readings for any prefix of their example sequences. The order-approval grammar gives two from the first `remind` or `escalate` of the outcome. They last until `approve` or `reject` decides between `approval` and `rejection`. The tutorial and URG grammars give one.
- The tools in the visualiser survey that show trees to a person show two or a few. The ANTLR plug-in draws the competing trees in a dialog, and Bison prints two derivations for a conflict. The tools that return every tree, Lark and nearley, warn of an exponential explosion or of inefficient parsing.
- JFLAP users freeze or remove configurations, because "the number of such configurations can become quite large". The JFLAP book warns: "After one Step, you will see six configurations!"
- Readings can grow exponentially. `("a" | "a" "a")*` gives 13 readings for six steps, 89 for ten and 10,946 for twenty. A cap of 10 stops it at six steps.
- The output view shows one reading at a time, with a switch, and the grammar view marks the position of each listed reading.
- A trace file holds each listed reading with its whole tree. A fixture at the cap then holds 10 trees, which a person can still review in a diff.
- The cost of each step grows with the cap, because each forest node keeps C+1 projections.

A later version can raise the cap. Only the trace files at the cap would then change, and each trace file records the cap that it used.

### The interface of the core

A trace session holds a grammar without errors, a sequence and the number of steps read. The state on view is the state after the steps read.

| Operation | Effect |
|---|---|
| Open | Takes a grammar text, and optionally the text of a trace file. It returns a session, or the diagnostics of the grammar or the file. With a trace file, the session reads as many steps as it can. |
| Read | Reads the next step of the sequence. It returns the new state, or the failure point. |
| Back | Drops the last step read. The step stays in the sequence. |
| Insert | Inserts a step right after the steps read, and does not read it. At the end of the sequence, this appends the step. |
| State | Returns the state on view, without trees. |
| Tree | Returns the tree of one listed reading. |
| Save | Returns the trace file of the session, as KDL text. |
| Marks | Takes an open path, and optionally the step in focus. Returns the marks of the open diagram, as the spec `rule-diagrams` states. |

- The interface takes one step at a time. So a later program can embed the core and drive it step by step, as the live runs on the horizon need.
- The state on view holds the steps with their numbers, symbols and attributes, and the number of steps read. It holds the failure point, if a read failed, and the count of readings with the cap flag. It also holds whether the sequence can end, and the next options.
- A state has no reading when the start rule derives no finite sequence. It then has no next option, and the sequence cannot end. When the sequence has a step, step 1 is the failure point. The page asks for no tree.
- For each listed reading, the state holds its parent, whether it can end, its next options, its position and its follow path. The position of a reading is the path of its tree from the root to its last step, with the origins of each node. The spec `rule-diagrams` defines the follow path, and the marks that the grammar view draws from the positions.
- The page asks for the state after each operation. It asks for the tree of the reading on view only.
- An insert clears the failure point, because the tracer has not read the steps after the insert. An insert before a rejected step moves that step one place later, and the page then reads the inserted step, as the output-view spec states.
- No operation removes a step that a person or a program wrote. To change the sequence in another way, the page opens a new session.
- A session refuses a grammar with an error, because such a grammar has no traces.

### The WebAssembly boundary

The decision record `typed-objects-at-the-webassembly-boundary` holds the choice. The operations above cross the boundary as typed objects, with TypeScript declarations from the Rust types. KDL text crosses only as the content of a file: a trace file that the person loads or saves. A number in an attribute crosses as its decimal text, so no value loses precision.

### The trace file

A trace file is a KDL 2.0 document with one root node, `trace`.

| Property of `trace` | Value | Writer |
|---|---|---|
| `format-version` | `1` | Every writer. It is required. |
| `grammar-sha256` | The SHA-256 hash of the grammar text, in lower-case hex | Optional for another program. The core always writes it. |
| `reading-cap` | The reading cap of the core that wrote the file | The core |

The children of `trace` come in this order:

1. `sequence`, with one `step` for each step.
2. One `prefix` for each count of steps read, from 0.
3. `failure-point`, when a step is rejected.
4. One `reading` for each listed reading of the state after the steps read.

#### Steps

- A `step` takes one argument: its symbol, as a KDL string. The symbol is not empty, and it holds no line break.
- Each property of a step is an attribute. Its key, its value and its type annotation stay as the writer wrote them. The core can write a number in another notation, such as `16` for `0x10`.
- A step has no children. Two attributes with the same key on one step are an error.
- A step departs from the mapping rules of the core. It takes an argument, and its properties are open. So an attribute can have any key, also `symbol`.

#### Derived nodes

| Node | Properties | Children |
|---|---|---|
| `prefix` | `steps`: the count of steps read, k. `reading-count`: the number of listed readings. `cap-reached`: whether more readings exist. `can-end`: whether the sequence can end after step k. | One `next` for each next option, with the property `symbol` |
| `failure-point` | `step`: the number of the rejected step | None |
| `reading` | `parent`, when the parent is listed. `can-end`. | One `next` for each next option that the reading allows, then the root node of its tree |
| `next` in a reading | `symbol` | One `occurrence` for each symbol occurrence that offers it |
| `rule` | `name` | `origin` nodes, then the child nodes |
| `loop` | `kind`: `repetition`, `tail-references` or `head-references`. `pass-rule`, for a loop through rules. | `origin` nodes, then `pass` nodes |
| `pass` | `number`, from 1 | `origin` nodes, then the child nodes |
| `step` in a tree | `number`, from 1, and `symbol` | `origin` nodes |
| `origin`, `occurrence` | `start` and `end`: the span of the grammar element | None |

- A tree node lists its origins in the order of the text. The output-view spec defines the origins of each kind of node.
- The symbol of a step in a tree repeats the symbol in the sequence, so a person can read a fixture without a lookup. The reader checks that the two agree.
- When the start rule derives no finite sequence, the file has one `prefix`, with `reading-count=0` and no `next`. It has no `reading`. When the sequence has a step, the file has `failure-point step=1`.
- The trace file holds the readings of the last state only. Readings for each earlier step would grow with the square of the length. The page gets the readings of an earlier state from the core, through Back.

#### An example

The tutorial grammar of the example suite, after three steps:

```kdl
trace format-version=1 grammar-sha256="59de628ef10155e40c7695aff3aa161c188df78a83ac7c8263caa954ad0d70dc" reading-cap=10 {
    sequence {
        step overview
        step install
        step introduce_concept
    }
    prefix steps=0 reading-count=1 cap-reached=#false can-end=#false {
        next symbol=overview
    }
    prefix steps=1 reading-count=1 cap-reached=#false can-end=#false {
        next symbol=install
    }
    prefix steps=2 reading-count=1 cap-reached=#false can-end=#false {
        next symbol=install
        next symbol=configure
        next symbol=introduce_concept
    }
    prefix steps=3 reading-count=1 cap-reached=#false can-end=#false {
        next symbol=show_example
    }
    reading parent=1 can-end=#false {
        next symbol=show_example {
            occurrence start=1296 end=1310
        }
        rule name=tutorial {
            rule name=overview {
                origin start=409 end=417
                step number=1 symbol=overview {
                    origin start=917 end=927
                }
            }
            rule name=setup {
                origin start=418 end=423
                loop kind=repetition {
                    origin start=546 end=554
                    pass number=1 {
                        rule name=install {
                            origin start=546 end=553
                            step number=2 symbol=install {
                                origin start=988 end=997
                            }
                        }
                    }
                }
            }
            loop kind=repetition {
                origin start=424 end=431
                pass number=1 {
                    rule name=lesson {
                        origin start=424 end=430
                        rule name=introduce_concept {
                            origin start=644 end=661
                            step number=3 symbol=introduce_concept {
                                origin start=1208 end=1227
                            }
                        }
                    }
                }
            }
        }
    }
}
```

### What another program writes

Another program writes the root node with `format-version`, and the sequence. It can add `grammar-sha256`. A URG tool can write:

```kdl
trace format-version=1 {
    sequence {
        step "states the problem"
        step "shows the simplest case" evidence=(url)"https://example.org/urg/episode-7#move-2"
    }
}
```

The core reads such a file with these checks:

| Fault | Severity |
|---|---|
| Text that is not KDL 2.0 | Error |
| A root node other than `trace`, or a format version that the core does not know | Error |
| An unknown node, an unknown property, a missing property, a property of the wrong type, or a node out of order | Error |
| A step without exactly one argument, an empty symbol, a symbol with a line break, or a step with children | Error |
| Two attributes with the same key on one step | Error |
| A derived node that breaks the format of the section "Derived nodes" | Error |
| A grammar hash that differs from the hash of the grammar text | Warning |

- Each diagnostic has a severity, a message and a span in the text of the trace file, as the diagnostics of a grammar do.
- After a warning for the hash, the core still traces the sequence. The derived nodes of the file describe another grammar text.
- The core then traces the sequence like any other, and it reads steps until the first rejected step. A symbol that the grammar does not use is a rejected step, not a fault of the file.
- The core never takes a derived node from a file. It derives each node again, and Save writes the result.
- So a trace file that the core wrote is current when Save gives the same text for the same grammar. The tests compare files in this way.
- A program that writes a relative file reference writes it relative to the folder that holds the trace file, as the output-view spec states.

### What the output view needs

The output-view spec lists twelve needs. The trace answers each one.

| Need | Answer |
|---|---|
| 1. The steps, numbered from 1, with symbols and attributes, also after the state on view | `sequence`, and the steps of the state |
| 2. The readings up to the cap, and whether the tracer reached it | The listed readings, and `cap-reached` |
| 3. One distinct tree for each reading | The section "Readings and the forest" |
| 4. The kind and the label of each node, with children in step order | The tree nodes of the section "Derived nodes" |
| 5. The origins of each node | `origin` nodes, with rule 2 of the section "Loops through rules" |
| 6. A step in each node, except the start rule node | The projection makes no node for a rule that holds no step |
| 7. The reading that each reading extends | `parent` |
| 8. A deterministic order that keeps the order of the parents | The section "The order of readings" |
| 9. Whether each reading can end | `can-end` of each reading |
| 10. The readings and occurrences of each next option | The `next` nodes of each reading |
| 11. The failure point, with the state before it | `failure-point`, and the state before the rejected step |
| 12. The format version and the grammar hash | `format-version` and `grammar-sha256` |

### The reference recogniser

The property tests compare the tracer with a reference recogniser over plain BNF.

- The recogniser expands the checked grammar into plain BNF, as the tests of the rule checks do. A bounded repetition becomes one alternative for each count. An open repetition becomes a helper rule with an empty alternative.
- It decides three relations over the spans of the sequence. A rule derives a span. A rule starts a span: it derives the span followed by some sequence. A rule is productive: it derives some sequence. The recogniser grows each relation until it stops growing.
- The recogniser accepts a prefix when the start rule starts it. The sequence can end when the start rule derives it. A next option is a symbol that the start rule starts after the prefix.
- The recogniser shares only the checked grammar and the BNF expansion with the tracer. It has no automaton, no Earley set, no forest and no pass count.
- Its meaning is the meaning of the notation spec: a choice has no order, and a bound allows exactly its numbers of passes. Oracles in the automata survey failed when their meaning drifted from the system under test. So the recogniser keeps to the plain definitions.

Each test case compares these values:

- For each prefix up to the failure point: acceptance, whether the sequence can end, and the next options. The next options range over the symbols of the grammar and one symbol that the grammar does not use.
- After the whole sequence, the tracer steps back to each shorter prefix. Its state must equal the state of a new trace of that prefix, readings included.
- Each case runs again with random attributes on its steps. Every compared value must stay the same, and the attributes must come back unchanged.

`reading-trees-k9` adds the invariants, and `reading-trees-k30` adds the tree oracle:

- The three invariants of the output-view spec, on every case.
- A tree oracle for prefixes of at most 6 steps. It finds the readings of each prefix from the grammar text, with no automaton, no Earley set, no forest and no cap.

The tree oracle lists no derivation. A left-recursive prefix has infinitely many partial derivations, such as those of `y` in `a = a "x" | "y" ;`. A grammar without recursion can also have more than 10^19 derivations of the empty prefix. An example nests choices between two equal optional references, three deep. So the oracle keeps sets of trees:

- The oracle keeps a set of distinct projected trees, with their origins, for each grammar element, start, end and loop context. An element can also start at a step and stay open after the last step.
- Equal trees join in the set, so the count of derivations does not matter.
- The oracle grows these sets until they stop growing, as the recogniser grows its relations.
- The sets are finite, because each node holds a step and no rule derives itself with no step. So the oracle stops on every grammar, and it needs no rule for cycles.
- A reading can end when its tree is also a tree of the start rule over the whole prefix.
- The next options of a reading come from the readings one step longer that name it as their parent. The origins of their last steps give the occurrences.
- The oracle sorts the readings in the order of this spec.

The tracer must give the same readings, up to the cap, with the same origins and parents. It must agree on whether each reading can end. For prefixes of at most 5 steps, it must also agree on the next options of each reading, with their occurrences.

The oracle skips a case when one of its sets would hold more than 1,000 trees. The comparison with the reference recogniser still runs on that case. The suite reports the skipped cases, and it fails when the oracle compares fewer than 1,000 cases.

The tree oracle checks the forest, the open layer and the capped lists. If a trip around a cycle of the open layer added a tree, the oracle would find that tree and the tracer would not. So the oracle also checks the section "The open layer".

The oracle does not check the tree rules or the loop contexts, because both sides apply them. The expected trace files check the tree rules. So the oracle covers prefixes of at most 6 steps, and cases whose sets stay under the bound. Longer prefixes and larger sets rely on the reference recogniser, the invariants and the expected trace files.

### The generators

The generators start from the same seed in every run, and they need no dependency, as the other property tests of the core do.

Random grammars:

- Each grammar has 1 to 4 rules, and `r0` is the start rule.
- Each rule body has a depth of 3 at most, over the symbols `"x"`, `"y"` and `"z"`. A sequence has 2 or 3 items, and a choice has 2 alternatives. Optional parts and repetitions also occur.
- A repetition takes `*`, `+`, `{2}`, `{0,2}`, `{1,3}` or `{2,}`. So no item needs more than 9 copies, and the BNF expansion stays small.
- A rule gets an extra alternative of one symbol with a probability of one half. So some rules derive no sequence, which tests the removal of dead parts.
- The generator skips each grammar with an error. The suite asserts that it kept at least 1,000 grammars.

Random sequences, for each grammar:

- Up to 3 sentences of the grammar, by generation with a cost bound. The Fuzzing Book gives this method, because naive generation does not stop on recursive rules.
- Before it generates, the generator finds which parts of the grammar derive a finite sequence. It also finds the cheapest completion of each part: the fewest steps, and then the fewest expansions.
- A decision chooses an alternative, or whether an optional part or one more pass comes. Each decision picks at random among the parts that derive a finite sequence.
- The decisions stop when the sentence has 8 steps, or after 64 decisions. Then each open part takes its cheapest completion. So the work of a sentence has a bound, also when a head-recursive alternative emits no step.
- When the start rule derives no finite sequence, the grammar has no sentence. Its cases then come from the random sequence only, so the tests still check that the tracer rejects step 1.
- One sequence for each single edit of each sentence: a step inserted, removed or replaced. The new symbol comes from `"x"`, `"y"`, `"z"` and `"w"`, and no grammar uses `"w"`.
- One random sequence of 0 to 8 steps over the four symbols.
- Each sequence has at most 12 steps, and the generator drops a longer sentence.

The cost bounds:

| Bound | Limit |
|---|---|
| Rules in a grammar | 4 |
| Depth of a rule body | 3 |
| Copies of one item in the BNF expansion | 9 |
| Steps in a sequence for the reference recogniser | 12 |
| Steps in a sequence for the tree oracle | 6 |
| Valid grammars in one run | At least 1,000 |
| Decisions at random in one sentence | 64 |
| Trees in one set of the tree oracle | 1,000 |
| Cases that the tree oracle compares | At least 1,000 |

The work of the recogniser for one case grows with the cube of the sequence length, times the size of the BNF expansion. The work of the tree oracle grows with the count of distinct trees, not of derivations, and its set bound caps that count. The decision bound caps the work of each sentence. These bounds aim to keep the whole suite inside one ordinary run of `task test`. `tracer-k8`, `reading-trees-k9` and `reading-trees-k30` record its measured time in their task files. The suite checks the cases of the grammars on several threads, in the order of one seed.

## Test seams

The work tests through the two seams that the human agreed on 2026-11-17. This design adds no seam.

1. **Core interface.**
   - A trace fixture is a grammar file and a trace file. The test reads the sequence of the trace file, traces it, and compares Save with the whole file. So one file is the input and the expected output, and `task test:update` writes its derived nodes.
   - A fixture for `NAME.ebnf` has the name `NAME--DETAIL.trace.kdl`, in the same directory.
   - Each example grammar has accepted and rejected fixtures, from its file of example sequences.
   - The test grammars of the output-view spec each have a fixture.
   - More fixtures cover `a = a "x" | "y" a | "z" ;` on `y z x`, and `e = "x"? e "y" | "z" ;` with and without `"x"`. Others cover a rule that derives no sequence, and `("x"+ "y"?)+` on `x x`.
   - A start rule that derives no sequence, as in `a = "x" a ;`, has a fixture with no reading. Its trace rejects step 1.
   - Each case of the section "The open layer" has a fixture.
   - A grammar reaches the cap: `("a" | "a" "a")*` with six steps.
   - A written trace file has attributes of each KDL type, and another has a grammar hash that differs. Each fault of the trace file has its own fixture.
   - The property tests of the sections "The reference recogniser" and "The generators" run at this seam.
2. **Workbench page.** The end-to-end tests of the output-view spec run here. Another test loads a trace file of an example grammar and shows its result. A save then gives the same text.

Small tests inside a Rust module stay allowed where the seams cannot reach a case. An example is the check that no two expressions share a span.

## Out of scope

- Explanations of a rejected sequence, with the fewest extra and skipped symbols. They wait until after the slice.
- An exact count of readings beyond the cap. A count of distinct trees needs the trees, so the tracer counts to C+1 only.
- Readings for each earlier step in the trace file, and the step on view of a saved session. A loaded file reads as many steps as it can.
- Several sequences in one trace file.
- KDL in the page, and a JSON encoding of the derived artifacts.
- The method of Leo for right recursion.
- A rule check for a rule that derives no sequence. The tracer removes such parts, so it never offers a step into them. A warning for such a rule stays a candidate for later.
- Generation of sequences for a person, which waits until after the slice.
