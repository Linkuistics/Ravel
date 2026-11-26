**Later Page Design**

The later mutation page should restate only the operational fact it needs locally:

`NormalizedKey` is the canonical tenant-qualified identity for the record this write path mutates.

It should not re-explain why that is true or how the type is constructed. Instead, it should link back to the early identity page section that established:

- the definition of `NormalizedKey`
- the source fragment that proves it
- any invariants about qualification or normalization
- any terminology the later page relies on

On the later page, link to that earlier section with a precise anchor such as “Canonical tenant-qualified identity (`NormalizedKey`)”.

For the source fragment, do not repeat the full early-page excerpt. Repeat only the minimal exact lines if the write-path explanation depends on seeing that source text in place. Label it as a repeated excerpt and link to the full earlier treatment. If the later page can stand with a local sentence plus a link, prefer no repeated code.

A good rule is:

- repeat a minimal excerpt when the reader must hold the identity fact in working memory while following the mutation steps
- otherwise use a local reminder sentence and a backlink only

**Representative Later-Page Paragraph**

This write path is described from the point where the target record is identified by `NormalizedKey`, the canonical tenant-qualified identity established earlier. On this page, any step that still handles a pre-normalized or non-tenant-qualified identifier belongs to `[earlier resolution step: verify in code]`, not to the mutation proper. The mutation actor `[function or method name: verify in code]` therefore either receives a `NormalizedKey` or derives one before calling `[storage operation: verify in code]`; any stronger claim about validation order, conflict handling, or retry behavior is a verification obligation unless the source on this page shows it directly.

**Prose Rules**

Claims:
- Every behavioral claim must be tagged mentally as one of: direct source claim, explicit inference, or verification obligation.
- Direct claims need local evidence or a precise backlink.
- Inferences must say what they are inferred from.
- Unknown implementation details stay as placeholders or `verification obligation` notes.

Stable vocabulary:
- Use `NormalizedKey` consistently for the canonical tenant-qualified identity.
- Use one term per concept across pages.
- Do not introduce casual synonyms such as “key”, “ID”, or “name” unless the page explicitly states the relationship.

Explicit actors:
- Every important sentence names the actor: caller, normalization step, mutation function, storage layer, error-mapping layer, or reviewer-unknown placeholder.
- Avoid actorless prose such as “the record is written” when the code actor is what matters.

Failure categories:
- Group failures by stage, not by guessed implementation detail.
- Use placeholders where needed, for example: `[identity derivation failure]`, `[precondition failure]`, `[storage failure]`, `[post-write follow-up failure]`.
- Do not collapse distinct stages into one generic “write failed” label.

**Assurance Before Publication**

Mechanical checks:
- Claim ledger: every nontrivial sentence maps to a source anchor, inference note, or verification obligation.
- Vocabulary check: `NormalizedKey` and related terms are used consistently across pages.
- Actor check: mutation-path sentences have explicit actors.
- Cross-link check: the later page links to the exact early-page anchor for the identity fact.
- Repeated-excerpt check: any repeated code is byte-for-byte identical to the earlier excerpt and marked as repeated.
- Placeholder check: every placeholder is either resolved or listed as a publication blocker.
- Failure-taxonomy check: each failure mentioned fits one stable category name.

Independent review:
- A source reviewer verifies that every later-page claim matches the cited code.
- A domain-naive reviewer verifies that the later page is understandable with only the early `NormalizedKey` page as prerequisite.
- An editorial reviewer verifies stable vocabulary, explicit actors, and that unknowns remain marked as obligations rather than being narrated as facts.

Publication should block on any unresolved claim without evidence, any unmarked synonym for `NormalizedKey`, or any repeated excerpt that differs from the original.
