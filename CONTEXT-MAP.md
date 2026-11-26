# Context Map

Ravel is one product made of peer components. Each component that has terms
of its own keeps its glossary, decision records and specs in its own
directory. The root `docs/adr/` holds the decisions that bind the whole
product.

"Ravel" alone means the product. The glossary of a component can use "Ravel"
for that component, and it says so under its flagged ambiguities.

## Contexts

- [Launcher](./launcher/CONTEXT.md): the `ravel` program, which runs an LLM
  session in an agent harness as a supervised part of another program. Its
  decision records are in `launcher/docs/adr/`, and its specs are in
  `launcher/docs/specs/`.
- [UniversalGrammar](./universal-grammar/core/CONTEXT.md): Universal
  Refinement Grammar (URG), which discovers and represents how authors explain
  technical subjects. Its decision records are in
  `universal-grammar/core/docs/adr/`.
- [Visualiser](./universal-grammar/visualiser/CONTEXT.md): the grammar
  workbench, which traces sequences of steps through hand-written grammars.
  Its decision records are in `universal-grammar/visualiser/docs/adr/`.

## Relationships

A **Descriptive grammar** of the Visualiser can describe the discourse moves
of a URG analysis. The Visualiser's research links to UniversalGrammar's
glossary for **Move**, **Pattern** and **Alternative reading**. Two words
cross the boundary with different meanings:

- "Reading". A Visualiser **Reading** is one way in which a grammar accepts a
  sequence of steps. A URG **Alternative reading** is a competing
  interpretation of the same evidence.
- "Step". A Visualiser **Step** is one element of a sequence that a grammar
  traces. A URG **Move** is one step inside an explanatory episode.

No other components share terms yet. The decision record
[`the-product-components-are-peers`](./docs/adr/the-product-components-are-peers.md)
states the layout that places each component.
