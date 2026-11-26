**Exposition Plan**

Use a strict split between “local reminder” and “canonical explanation.”

On the later mutation page, state locally only the operational consequence of the earlier fact: the write path uses `NormalizedKey` as the identity under which the mutation is interpreted and persisted. Link back to the early page for the full meaning of `NormalizedKey`, its construction, and the original source fragment. Do not fully repeat the earlier source fragment on the later page by default. Repeat it only if the later argument depends on exact syntax from that fragment; otherwise, reuse it by link and, if needed, quote at most the smallest relevant line or condition.

Recommended later-page pattern:
1. One sentence re-establishing the fact in local terms.
2. A link to the early “identity” page.
3. A new source fragment from the mutation path showing where that fact is used.
4. An explicit note for any unknown step as a verification obligation.

**Decision For The Later Page**

State locally:
`NormalizedKey` is the canonical tenant-qualified identity already established earlier; this page relies on that meaning to explain which logical record the mutation targets.

Link to:
- The early page that defines `NormalizedKey`.
- The earlier source fragment, by anchored reference.
- The new mutation-path fragment where the write path consumes, propagates, or requires that identity.

Repeat source fragment:
- `No`, not in full.
- `Yes`, only as a minimal excerpt if the later reasoning would otherwise force readers to jump pages to resolve a single line-level dependency.

**Representative Later-Page Paragraph**

“In this write path, the service does not choose a new record identity locally; it acts on the `NormalizedKey` that earlier pages established as the canonical tenant-qualified identity. That fact matters here because every claim about ‘which entry is being updated’ must be read as a claim about the `NormalizedKey`, not about any pre-normalized input form. The earlier identity fragment shows where that canonical meaning is defined; this mutation fragment shows where the write path depends on it. If the code also derives, validates, or re-checks the `NormalizedKey` at this stage, that step should be documented from the exact lines that perform it; otherwise mark the point as a verification obligation.”

**Prose Rules**

- Claims:
  - Every behavioral claim must be backed by a cited source fragment, storage API contract, or an explicit verification obligation.
  - Distinguish observed code behavior from inferred intent.
  - Avoid “always,” “guarantees,” and “ensures” unless the cited code proves them.

- Stable vocabulary:
  - Use one term consistently for each concept: `NormalizedKey`, `tenant`, `canonical identity`, `mutation`, `write path`, `storage`.
  - Do not introduce synonyms such as “canonical key,” “resolved key,” or “tenant key” unless the code uses them and the page defines the relationship.

- Explicit actors:
  - Every sentence that describes an action should name the actor: caller, service method, helper, storage API, or background mechanism if one is proven to exist.
  - Avoid actorless phrasing like “the key is checked” when the code shows who checks it.

- Failure categories:
  - Group failures by mechanism visible in code, not by vague outcome.
  - Allowed template categories are placeholders until verified: `input rejection`, `identity derivation/validation failure`, `precondition failure`, `storage operation failure`, `post-write inconsistency/invariant failure`.
  - Remove or rename any category not evidenced by code.

**Assurance Plan**

Mechanical checks before publication:
- Link check for every intra-walkthrough reference and anchored source citation.
- Terminology check: every defined term has one spelling and one meaning.
- Claim-evidence check: every nontrivial claim is followed by a citation or a `Verification obligation: ...`.
- Placeholder check: no unresolved `<placeholder>` or “TBD” text remains without an owner.
- Fragment reuse check: repeated source excerpts are minimal and justified.
- Drift check: quoted or referenced line ranges still match the current revision under review.
- Style check: actor named, failure category named, and modal verbs constrained.

Independent review before publication:
- One domain-naive reviewer confirms the later mutation page is understandable using only its local reminder plus the link back.
- One implementation reviewer confirms that every claim about `NormalizedKey` and the write path is textually supported by code.
- One editorial reviewer checks vocabulary stability, non-repetition, and that unknowns are framed as verification obligations rather than guessed behavior.

Publication gate:
- No uncited behavioral claims.
- No unresolved verification obligations without explicit sign-off.
- No repeated source fragment unless the review record says the repetition is necessary for local reasoning.
