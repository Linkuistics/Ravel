# One Earley tracer for every grammar

One Earley parser traces every sequence through every grammar. It works over one position automaton for each rule, and it adds one Earley set for each step. The next options come from the last set, and a step back drops that set. The tracer returns every reading up to the reading cap, and it says when it reaches the cap. The state-machine diagrams are views of this parser, not separate engines.

## Trade-off

An Earley parser takes cubic time in the worst case, more than a finite automaton needs for rules without self-embedding. In return, one parser gives every reading, the next options without a second search, and step back without snapshots. Other tools pay extra for next options. Lark offers them only for its LALR parser (https://lark-parser.readthedocs.io/en/latest/classes.html), and antlr4-c3 reports "millions of visited ATN states for complex input" (https://github.com/mike-lischke/antlr4-c3). A badge marks each group of mutually recursive rules as "no self-embedding" or "may need a stack", by the method of Nederhof (https://arxiv.org/pdf/cs/9910022).

## Rejected alternative

Glushkov automata for the regular parts and a graph-structured stack for the rest, as the pasted request asked. Two engines would need two trace formats, and they could disagree about one sequence.

Source: proposals that the human agreed on 2026-11-18, recorded in `docs/milestones/grammar-workbench.md`, and the automata-algorithms and grammar-visualisers surveys.
