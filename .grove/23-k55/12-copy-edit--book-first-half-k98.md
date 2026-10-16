# book-first-half-k98


## Goal

Rewrite chapters 1–6 as one set under the amended prose contract in
`docs/book/contract.md`. The owner wants plain language for a 16-year-old
software engineer and specifically found “front” unfamiliar and “admits”
unnecessarily technical. This is a wording pass, with technical meaning and
the existing programming prerequisites preserved.



## Context

- Read `book-first-half-k67` for the set proof, source inventories, edits and
  checks. This pass applies a new owner requirement, rather than reopening
  the earlier units' correction runs.
- The settled term is **Ravel CLI**, first defined as the `ravel` command-line
  program. The owner suggested “cli” after considering “Ravel executable”.
  `CONTEXT.md` and the contract carry the current term. Do not preserve the
  superseded choice merely because it appears in an earlier log.
- Scope: all six `docs/book/src/0*.md` chapters as one document, their local
  links and the glossary where wording needs to agree. Review sentences,
  headings, tables and examples' surrounding explanation. Use “checks”,
  “accepts”, “allows”, “starts” or another exact ordinary verb for the actual
  action. Do not treat this as two global substitutions: simplify other
  specialist phrasing, including unexplained “admission” and “settlement”.
- Assume the reader knows software engineering, the contract's programming
  prerequisites and the diagram types. Keep necessary Ravel/OS concepts
  explained where needed; simplify without talking down or losing conditions.
- Literal CLI output, identifiers, exact examples and normative source
  terminology remain exact. If a heading changes, reconcile its inbound
  links. Report figure-owned wording to art, which is already queued next.

## Done when

- The whole set meets the revised prose contract, with Ravel CLI used
  consistently and first explained/linked where needed. Every specialised
  word earns its place and explanation. Record representative wording choices
  and any deliberate literal/source exceptions.
- Technical distinctions proved in k67 survive: worker-before-launch,
  publication versus successful delivery, refusal/failure/cancellation,
  policy context versus session-result context, limits and evidence scope.
- Build and complete-example checks pass on final unchanged inputs; changed
  anchors resolve. Any figure label/caption/alt-text changes art owns are
  written into its live body or the book brief, then cleared by art.

## Notes

Art `book-first-half-k99` and proof `book-first-half-k100` already follow this
leaf before guide migration. Cut no duplicate successor. If wording reveals
a missing explanation or a substantive technical defect requiring draft,
use the editorial correction procedure with the precise case; do not conceal
it with an easier synonym.
