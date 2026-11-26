# Trees follow rules and passes

Each reading of a sequence has one tree, and no two readings have the same tree. A rule makes a named node, and a repetition makes a loop node with one child for each pass. Choices and optional parts make no node, and the core does not inline rules in the tree. Each grammar gives one hierarchy. A step can carry an identifier, such as a URG evidence reference, but relations between steps stay in URG records.

A loop through rules also makes a loop node. Such a loop is a cycle of rules that tail references link, or a cycle that head references link, as in left recursion. A self tail call and direct left recursion are its one-rule cases. The loop starts where a reading enters the cycle, and the rule that it enters is the pass rule. Each later reference to the pass rule around the cycle starts a new pass. So each pass holds one node of the pass rule, and the other rules of the cycle stay nested in it.

A reference that is not a link of the loop of its target enters a new loop. An example is a head reference with a step before it. A rule can lie on a cycle of tail references and on a cycle of head references, as in `a = a "x" | "y" a | "z" ;`. Then only the cycle of head references makes a loop, and the tail references make nested rule nodes.

A node holds at least one step, except for the start rule node. So the tree of a reading only grows: a new step never moves an earlier step to another node.

Two trees are the same when their nodes agree in shape, kind, rule and step. The grammar elements that a node came from are its origins, and origins never make two trees different. So one node can have several origins, and a later step can remove some of them. The spec `output-view` gives the exact terms and rules.

## Trade-off

Pass nodes add a node type, and two readings can disagree on the number of passes, as `(a | a a)*` does on `a a a`. In return, the output view can fold completed passes into one row with a count. Lark (https://lark-parser.readthedocs.io/en/latest/tree_construction.html) and tree-sitter (https://github.com/tree-sitter/tree-sitter/issues/223) flatten a repetition and lose its passes. One hierarchy cannot hold every relation in discourse. In the Discourse Graphbank, 41.22% of nodes have more than one incoming relation (https://aclanthology.org/J05-2005.pdf).

A pass of a loop through rules depends on where the reading entered the loop, not only on the grammar text. Two loops of one cycle can therefore start their passes at different rules. In return, one pass is one trip around the loop, and the count of passes matches the repetition that the loop is equal to. The automata survey proposed this start of a pass. Control-flow analysis meets the same problem in loops with several entries (https://dl.acm.org/doi/pdf/10.1145/570886.570887).

The tree builder must also handle left recursion as a loop. Without a loop node, the depth of a left-recursive spine depends on steps that have not come yet. Each step would then move the earlier steps deeper. The rows of the output view could then not keep their places. The tracer must also merge derivations that give the same tree, because the views cannot tell such readings apart.

A rule on both kinds of cycle loses the folds of its tail passes. Its loop of head references stays, because only that loop keeps the tree growing at the end. Nested nodes of tail references grow at the end too. No example grammar has such a rule, and its badge says "may need a stack".

## Rejected alternatives

Flat repetitions, as in Lark and tree-sitter. A graph for each sequence, which would turn the grammar into a knowledge model. The human wants grammars "for structure more than knowledge modelling".

A pass at each tail call of a loop through rules. Passes would alternate between the rules, and the count would double for two rules. One fixed rule of each cycle as the start of every pass. A reading that enters at another rule would start with a part of a pass. Either alternative becomes worth a new look if real grammars show that people cannot tell where a pass starts.

Both loops for a rule on both kinds of cycle, nested in one order. One pass would then hold a loop node, not one node of its pass rule.

One reading for each set of origins. In `s = item "y" | item "z" ; item = "x" ;`, the sequence `"x"` would then have two readings with the same visible tree. The count of readings would grow with each choice between equal trees.

Source: the human's decisions of 2026-11-17 in `docs/milestones/grammar-workbench.md`, the design of `output-view-design-k4` with its review, the design of `tracer-design-k7`, and the grammar-visualisers, automata-algorithms and discourse-grammars surveys.
