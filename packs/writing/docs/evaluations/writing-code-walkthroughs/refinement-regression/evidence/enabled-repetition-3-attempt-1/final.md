**Exposition Plan**

The later mutation page should restate the meaning of `NormalizedKey` in one local sentence, because the write-path explanation depends on that meaning and readers should not have to reopen the earlier page to follow a critical invariant. It should not re-teach the full concept, history, or derivation. A good local statement is: `NormalizedKey` is the canonical tenant-qualified identity used to decide which logical record a mutation targets.

The later page should link to:
1. The early page section that defines `NormalizedKey` and its motivation.
2. The earlier source fragment, if that fragment is still the clearest place where canonicalization or tenant-qualification is visible in code.
3. Any later-page source fragment that shows where the write path consumes a `NormalizedKey` or relies on it as a precondition.

It should usually not repeat the full early source fragment inline. Repeat only a minimal excerpt if the mutation-page argument cannot be followed without seeing the exact earlier lines. Default rule: link back; only duplicate when omission would force the reader to context-switch at the point of a key behavioral claim. If duplicated, mark it as a previously introduced fragment and keep the excerpt narrow.

**Representative Later-Page Paragraph**

Before the service performs the write step `[WRITE_STEP_PLACEHOLDER]`, it is already operating on a `NormalizedKey`, the canonical tenant-qualified identity introduced earlier. That matters here because the mutation path is not choosing “which key seems close enough”; it is acting on the exact logical record identified by the canonical form. The earlier normalization fragment is therefore relevant again: it establishes the precondition that this path receives a key whose tenant scope and canonical representation have already been fixed. If the implementation also revalidates, transforms, or reconstructs that identity at this stage, this page must name that actor and cite the exact source lines; otherwise, treat “write consumes previously normalized identity” as a verification obligation until confirmed.

**Prose Rules**

Claims:
- Every behavioral claim must be either source-backed, explicitly inferred, or labeled as a verification obligation.
- Do not claim ordering, atomicity, idempotence, durability, validation, or error handling unless the cited code or authoritative design text shows it.
- When a page depends on an earlier invariant, restate the invariant briefly at the point of use.

Stable vocabulary:
- Pick one term for each concept and keep it fixed across pages.
- Use `NormalizedKey` exactly for the canonical tenant-qualified identity, and do not alternate with synonyms like “resolved key” or “final key” unless the code does.
- Distinguish carefully between placeholders such as `[REQUEST_KEY_PLACEHOLDER]`, `[NORMALIZED_KEY_PLACEHOLDER]`, and `[STORAGE_KEY_PLACEHOLDER]` until the code proves they are the same or different.

Explicit actors:
- Every operational sentence should name the actor: client, request parser, normalization logic, mutation handler, storage layer, background process, or `[ACTOR_PLACEHOLDER]`.
- Avoid actorless prose like “the key is checked” when “`[ACTOR_PLACEHOLDER]` checks the key” is what the evidence supports.
- If the actor is unknown, say so directly and record a verification obligation.

Failure categories:
- Separate failures by layer and cause.
- Minimum categories: input/validation failure, identity/tenant-scope failure, concurrency/conflict failure, storage API failure, and internal invariant violation.
- Do not merge distinct categories into “write failed” unless the code does not expose finer distinctions and that limitation is stated.

**Assurance Plan**

Mechanical checks before publication:
1. Claim audit: every nontrivial claim has a citation, an explicit inference label, or a verification-obligation marker.
2. Vocabulary audit: `NormalizedKey` is used consistently and no uncited synonym replaces it.
3. Actor audit: each step in the write path names the actor or marks the actor as unknown.
4. Cross-page link audit: the mutation page links back to the `NormalizedKey` definition and to the reused fragment.
5. Fragment reuse audit: duplicated fragments, if any, are byte-identical to the earlier excerpt and labeled as previously introduced.
6. Placeholder audit: all unknown implementation details remain placeholders or obligations; none are silently concretized.
7. Failure taxonomy audit: every described failure maps to one stated category, with no mixed categories in a single sentence.
8. Drift audit: if the early page changes the `NormalizedKey` explanation, the later page is rechecked for wording drift.

Independent review before publication:
1. A domain-fresh reviewer reads the mutation page alone and reports where the local `NormalizedKey` restatement is insufficient.
2. A code-grounded reviewer verifies that every write-path claim matches cited source and that no storage behavior is implied without evidence.
3. A terminology reviewer checks that the same identity concept is named consistently across pages.
4. A red-team reviewer looks specifically for invented behavior, hidden assumptions about failure handling, and places where a link should replace repeated exposition or vice versa.

Publication gate:
- Publish only if all verification obligations are either resolved or explicitly listed in a visible “Open Verification” section on the relevant page.
