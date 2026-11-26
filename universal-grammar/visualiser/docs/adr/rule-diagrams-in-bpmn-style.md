# One diagram for each rule, in BPMN style

The workbench draws one diagram for each rule, never one diagram for the whole grammar. A symbol is a task box, and a choice is a pair of exclusive gateways. Optional parts and repetitions are gateways with skip or back flows, and a bound appears as a loop marker with its numbers. A reference box opens the diagram of its rule, with breadcrumbs back.

A self tail call is a back edge where a straight route to a lane is clear, and a linked reference box elsewhere. A loop through several rules is a set of linked diagrams. Reading markers look different from BPMN tokens, because a marker shows one reading, not one running instance.

## Trade-off

A rule never appears with the rules that it references in one flat state machine, until a flattened view arrives after the slice. In return, each diagram stays small and maps to one rule of the text. Gateways also reduce edges. A repeated choice of ten symbols needs 110 edges as a plain state diagram, and 24 with gateways. Each diagram comes from the checked grammar, so it cannot drift like the railroad diagrams of SQLite (https://arxiv.org/html/2509.15834v2).

## Rejected alternative

One diagram for the whole grammar, which the human excluded. References that expand in place, which bpmn.io judged insufficient: "Expanding and collapsing sub-processes in place is not enough." (https://bpmn.io/blog/posts/2022-bpmn-js-900-collapsed-subprocesses.html)

Source: the human's decision of 2026-11-17 in `docs/milestones/grammar-workbench.md`, and the grammar-visualisers and layout-interaction surveys.
