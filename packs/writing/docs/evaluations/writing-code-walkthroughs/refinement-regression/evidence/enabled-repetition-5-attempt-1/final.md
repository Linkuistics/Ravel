**Exposition Plan**

The later mutation page should restate the minimum semantic fact locally: `NormalizedKey` is the canonical tenant-qualified identity used by the write path once identity has been established. It should not re-explain how that fact was derived or broaden it into unstated behavior. The page should link back to:

1. The early concept page section that defines `NormalizedKey`.
2. The earlier source-fragment anchor where the code first makes that canonicalization visible.

It should not repeat the full early source fragment by default. Reuse the link instead, and keep the later page focused on the mutation flow. Repeat only a tiny excerpt if the write-path argument depends on exact syntax in a way the link alone would interrupt comprehension; otherwise treat repetition as a verification obligation not yet justified.

A good local pattern is:

- One sentence of semantic recap.
- One explicit backlink to the earlier definition.
- Then the mutation-path analysis using that meaning.

Representative later-page paragraph:

> From this point in the write path, the walkthrough treats `NormalizedKey` as the canonical tenant-qualified identity, as established earlier in [Definition of `NormalizedKey`](#placeholder-early-page-link) and shown in [the earlier source fragment](#placeholder-fragment-link). The mutation logic here therefore needs to be explained in terms of which actor produces, carries, validates, or persists that identity, not in terms of any pre-normalized input form. If this code also performs deduplication, conflict detection, routing, or storage lookup, each such claim must be tied to a cited fragment on this page or marked as a verification obligation.

**Prose Rules**

Claims:
- Every behavioral claim must be source-backed on the same page or explicitly labeled `Verification obligation`.
- Do not infer storage semantics, ordering, idempotence, conflict policy, or durability from naming alone.
- When a page relies on a fact established earlier, restate the fact briefly and cite the earlier page.

Stable vocabulary:
- Use `NormalizedKey` exactly once as the canonical term and do not introduce synonyms such as “final key”, “resolved key”, or “tenant key”.
- Use one term for each role placeholder, such as `[caller]`, `[service layer]`, `[storage layer]`, until the code justifies a concrete name.
- Distinguish `input identifier` from `NormalizedKey` whenever both appear.

Explicit actors:
- Every action sentence should name the actor: `[caller] supplies`, `[service code] derives`, `[validator] rejects`, `[storage API] persists`.
- Avoid actorless prose like “the key is checked” unless the actor is unknown, in which case say `actor not yet verified`.

Failure categories:
- Separate failures into `input rejection`, `normalization failure`, `lookup miss`, `write failure`, and `post-write uncertainty` only when each category is source-supported.
- If the code does not clearly distinguish categories, collapse them and mark the missing distinction as a verification obligation.
- Do not assign retryability, user visibility, or transactional consequences without evidence.

**Assurance Plan**

Mechanical checks before publication:
- Check every nontrivial claim has a citation or `Verification obligation` label.
- Check every reuse of `NormalizedKey` matches the early-page wording exactly.
- Check all backlinks resolve to the intended page section and source-fragment anchor.
- Check repeated excerpts, if any, are byte-identical to the cited source fragment.
- Check placeholder terms for unknown actors or behaviors are visibly marked and not silently concretized.
- Check failure labels are used consistently across pages.
- Check no later page redefines an earlier concept.

Independent review before publication:
- One reviewer traces each claim back to cited source and flags unsupported inference.
- One reviewer reads only the later mutation page and verifies it is understandable with the local recap plus links.
- One reviewer checks terminology stability across the full walkthrough, especially `NormalizedKey`.
- One reviewer performs a “domain ignorance” pass: if a reader who knows the language and storage APIs but not the service domain would need unstated background, add prose or convert the gap into a verification obligation.
