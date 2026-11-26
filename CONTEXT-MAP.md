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

## Relationships

The product's components share no terms yet. The decision record
[`the-product-components-are-peers`](./docs/adr/the-product-components-are-peers.md)
states the layout that places each component.
