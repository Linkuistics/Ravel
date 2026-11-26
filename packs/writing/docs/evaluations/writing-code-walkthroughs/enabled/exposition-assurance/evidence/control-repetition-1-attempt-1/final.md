**Recommendation**

On the later mutation page, restate only the operational meaning the reader needs at that moment:

- `NormalizedKey` is the identity used to decide which tenant-scoped record a write targets.
- The page should say that the write path relies on the earlier definition, not redefine the type from scratch.
- Link back to the early identity page and to the exact earlier source fragment anchor.
- Do not repeat the full early source fragment by default. Reuse it by link unless the later page cannot be understood without seeing one specific line inline.
- If one line is essential, repeat only that minimal line or a paraphrased callout, and label it as reused context rather than new evidence.

This keeps the identity concept stable while avoiding duplicated explanation that can drift.

**What The Later Page Should Contain Locally**

The later page should state, in its own prose:

- where in the write path the actor first needs tenant-qualified identity,
- that the identity at that point is `NormalizedKey`,
- why this matters for the mutation being described,
- which later steps are claimed to consume that same identity or a verified derivative,
- which details are still unknown and must be verified.

It should link to:

- the early page section that defines `NormalizedKey`,
- the anchored early source fragment that demonstrates the canonical identity claim,
- any later source location that shows the write path consuming that identity.

It should not locally re-explain:

- background motivation already covered on the early page,
- alternate names for the same concept,
- inferred behavior not yet verified in code.

**Representative Later-Page Paragraph**

“In the mutation path, the request-handling actor first needs a tenant-qualified target identity at `[VERIFY: entry point location]`. This walkthrough uses the earlier established fact that `NormalizedKey` is the canonical tenant-qualified identity, so the write described here is understood to target the record identified by that `NormalizedKey`, not by any pre-normalized input form. The earlier identity fragment remains the basis for that claim; this page applies it to the write path and should link back to that fragment rather than restating it in full. Any claim that a later step reuses the same identity, or a mechanically derived form of it, remains a verification obligation until the relevant code location is cited.”

**Prose Rules**

Claims:
- Every implementation claim must be either cited to code or marked `[VERIFY]`.
- Do not infer ordering, validation, caching, deduplication, conflict handling, or persistence semantics without evidence.
- Separate “observed in code” from “required for reader understanding.”

Stable vocabulary:
- Use `NormalizedKey` exactly as the canonical term.
- Use one fixed term for the thing being written, such as `[RECORD]`, until code confirms a better name.
- Do not introduce synonyms like “entity key,” “storage key,” or “tenant key” unless the code itself distinguishes them.

Explicit actors:
- Every sentence describing behavior should name the actor: request-handling actor, normalization logic, mutation logic, storage client, or `[VERIFY: actual actor name]`.
- Avoid actor-free prose like “the key is checked” or “the write is applied.”

Failure categories:
- Identity formation failure: cannot produce or validate the `NormalizedKey`.
- Target resolution failure: cannot map the `NormalizedKey` to the intended record or storage location.
- Mutation precondition failure: write rejected because a required condition is not met.
- Storage operation failure: underlying storage call fails or returns an unexpected result.
- Post-write observation failure: the write may succeed but confirmation, logging, or follow-up observation fails.
- If the code uses different categories, replace these with code-backed names before publication.

**Mechanical Checks Before Publication**

- Terminology check: `NormalizedKey` appears with one spelling and no uncited synonyms.
- Claim tagging check: every factual sentence has either a code citation or a `[VERIFY]` marker.
- Link check: backlinks to the early identity page and anchored source fragment resolve.
- Duplication check: the early source fragment is not fully repeated on the later page unless explicitly justified.
- Actor check: each process sentence names an actor.
- Failure check: each described failure belongs to one declared category.
- Drift check: repeated statements about `NormalizedKey` match the early page wording.
- Placeholder check: all placeholders are either resolved or collected in a publication-blocking verification list.

**Independent Review Before Publication**

- Implementation reviewer: confirms every cited code claim and rejects uncited behavior.
- Fresh-reader reviewer: knows the language and storage APIs but not the service domain, and checks whether the later page is understandable using only the local restatement plus links.
- Editorial reviewer: checks vocabulary stability, actor clarity, and non-duplication of the early fragment.
- Publication gate: do not publish with unresolved `[VERIFY]` items unless the document is explicitly labeled draft and the open obligations are listed at the top.
