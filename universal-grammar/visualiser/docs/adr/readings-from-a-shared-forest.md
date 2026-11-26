# Readings from a shared forest

The tracer builds a shared packed forest while it adds its Earley sets, by the method of Scott (2008) (https://dl.acm.org/doi/10.1016/j.entcs.2008.03.044). A projection turns each derivation of the forest into a tree, by the rules of `trees-follow-rules-and-passes`. Each forest node keeps its first distinct projections, up to one more than the reading cap, in one deterministic order. So the tracer lists the first readings, and it says whether more exist, without a list of every derivation.

A node on a chain of head links keeps such a list for each split of its steps. The nodes above it decide the order of two projections with different splits. The spec `tracer` defines the split.

Each projection of an open node also keeps its alternatives: the items and the child projections that give it. Marks follow these routes to the next positions of a listed reading. For one open path, they cut each successor to the nodes that the rules for marks read. The lists of cut successors have no cap. So the marks of the grammar view stay exact at the reading cap, and their work stays polynomial. The spec `rule-diagrams` defines the cut successors.

## Trade-off

The forest, the projection and the capped lists form the most complex part of the core. The lists also depend on a strict order of trees, which the spec `tracer` defines. In return, the readings are exact up to the cap. Acceptance, the failure point and the next options come from the Earley sets, so they stay exact at the cap too.

Derivations can far outnumber trees. `("a" | "a")*` has 2^n derivations for n steps, but one tree, because choices make no node. Lark warns of "an exponential explosion" when it expands every tree (https://lark-parser.readthedocs.io/en/stable/recipes.html). Earley's own method for trees "is incorrect", as Scott (2008) found.

The alternatives add memory to the open layer. The cut ties the evaluation of successors to the four rules for next-option marks, and each open path needs its own evaluation. Over the cycles of the open layer, that evaluation repeats in rounds, as the tracer does. In return, a reading with exponentially many successors costs no more than its routes. The spec `rule-diagrams` gives a family of depth d with 2d + 1 rules, in which one reading has 2^d successors. On each open path that the prototype measured, the open root kept at most d + 2 cut successors.

## Rejected alternatives

A set of readings that the tracer extends at each step, with a cap on its size. A later step can reject every reading in the set while a dropped reading survives. The tracer would then show no reading for an accepted sequence. Without a cap, `("a" | "a" "a")*` has 89 readings for ten steps and 10,946 for twenty.

A list of derivations, with equal trees removed afterwards. One reading of `("a" | "a")*` would then cost exponential time.

Earley's own method for trees, which Scott (2008) showed to be wrong.

Next positions from the readings after a next step. The cap cuts that list. In `s = "seed" (r1 | … | r12) ;` with `r1 = "next" ;` to `r12 = "next" ;`, the one reading after `seed` has twelve next positions, and the cap lists ten readings after `next`.

Lists of every successor of a listed reading. Such a list grows exponentially with the depth of nested choices between rules that start with the same symbol. At depth 30, that family needs 2^30 trees for one reading, while the tracer finds the readings after the step in 5.8 ms.

The union of the marks of each route. One successor can come from routes whose added passes take links in different rules. Rule 2 then reads the joined origins of the pass, so a union of the routes gives other marks.

Source: the automata-algorithms survey, the spec `output-view`, the design of `tracer-design-k7` in the spec `tracer`, and the designs of `exact-marks-k39` and `exact-marks-k44` in the spec `rule-diagrams`.
