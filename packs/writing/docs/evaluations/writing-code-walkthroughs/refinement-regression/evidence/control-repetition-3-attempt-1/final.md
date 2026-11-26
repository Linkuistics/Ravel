**Exposition Plan**

The later mutation page should restate the meaning of `NormalizedKey` in one local sentence, but only at the level needed for the write path:

`NormalizedKey` is the canonical tenant-qualified identity, and this mutation path uses that identity to determine which logical record the write targets.

That page should link to:
- The early concept page that defines `NormalizedKey` and establishes why it is canonical.
- The anchored early source fragment that shows where the canonical identity is formed, normalized, or first relied on.
- Any later-page source fragment that shows the write path consuming that identity.

It should not repeat the full early source fragment. Duplicate code blocks make readers re-parse syntax instead of following the argument, and they create drift risk if one copy is updated and the other is not. If the layout system supports it, reuse the exact same anchored snippet by reference; otherwise use a textual link such as “see the earlier `NormalizedKey` construction fragment” and keep the later page focused on the mutation semantics.

**Representative Later-Page Paragraph**

In `[mutation entrypoint]`, the request is translated into a target identity before the write reaches `[storage write call]`. That target identity is the `NormalizedKey` introduced earlier: the canonical tenant-qualified identity for the record being changed. This page does not re-derive that form; it relies on the earlier definition and the earlier source fragment that shows where the canonical identity is established. The verification obligation here is narrower: confirm that every path from `[mutation entrypoint]` to `[storage write call]` uses the same `NormalizedKey` value, or explain any intentional re-computation and its equivalence conditions.

**Prose Rules**

- Claims: Every behavioral claim must be backed by a cited fragment, a test, or an explicit verification obligation. If the code was not inspected end-to-end, say so.
- Stable vocabulary: Use one term consistently for each concept. `NormalizedKey` always names the canonical tenant-qualified identity; do not alternate with “lookup key,” “storage key,” or “record id” unless the code distinguishes them and the page defines the distinction.
- Explicit actors: Each sentence that describes behavior should name the actor: request parser, normalization step, mutation handler, storage adapter, caller, or reviewer. Avoid actorless phrasing like “the key is checked” when the responsible component is unknown.
- Failure categories: Separate failures by class, not by vague error language. Use placeholders until verified, for example:
  - identity formation failure: `[missing tenant]`, `[invalid key shape]`
  - authorization or tenancy boundary failure: `[access check failure if present]`
  - conditional write or concurrency failure: `[stale version]`, `[compare-and-set miss if present]`
  - storage failure: `[backend write error]`
  - internal invariant failure: `[unexpected impossible state]`

**Mechanical Checks Before Publication**

- Every nontrivial claim has at least one citation or a `Verification obligation:` label.
- Every use of `NormalizedKey` matches the exact spelling and definition from the early page.
- Every later-page back-reference resolves to a stable anchor.
- No duplicated source fragment appears unless it is transcluded from a single source of truth.
- Placeholder terms are visibly marked as placeholders, for example `[storage write call]`.
- Failure labels are exhaustive for the page’s scope, or the page explicitly says which categories are out of scope.
- A terminology lint pass flags synonyms for defined concepts.
- A link check confirms that concept-page, fragment, and test references all resolve.

**Independent Review Before Publication**

Use two reviewers with different mandates:
- Domain-ignorant reader review: confirm the page is understandable without prior service context, and that the local reminder about `NormalizedKey` is sufficient.
- Code-accuracy review: confirm each claim against the code, especially the handoff from `[mutation entrypoint]` to `[storage write call]`, and either discharge or preserve every verification obligation.

Publication should block on unresolved contradictions, broken anchors, uncited behavioral claims, or ambiguous actor/failure wording. Unresolved unknowns may remain only if they are explicitly marked as verification obligations.
