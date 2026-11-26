**Later Mutation Page Design**

The later mutation page should state the meaning of `NormalizedKey` locally in one sentence, then link back to the early identity page for the full definition and the original source fragment. It should not fully repeat the early fragment by default.

Recommended local statement:
`NormalizedKey` is the canonical tenant-qualified identity used to decide which record a mutation targets.

What to link:
- Link to the earlier page section that defines `NormalizedKey` conceptually.
- Link to the earlier source fragment that shows where `NormalizedKey` is formed, validated, or normalized.
- If the write path uses a later transformation of that same value, link to the exact step where the already-normalized identity is consumed.

Whether to repeat the source fragment:
- Default: do not repeat it.
- Repeat only a minimal excerpt if the mutation-page argument fails without immediate visual recall of one specific line.
- If repeated, the excerpt must be explicitly marked as reused context and must be byte-identical to the earlier excerpt.

Reasoning:
- The later page needs local readability, so it must restate the meaning.
- The earlier page owns the full explanation and evidence, so the later page should link rather than duplicate.
- Repeating the whole fragment increases drift risk and weakens page ownership.

**Representative Later-Page Paragraph**

On this write path, the actor `[request handler / coordinator: verify]` first determines the target record by using the caller’s input to obtain a `NormalizedKey` `[construction step: verify]`. From this point onward, the mutation explanation should refer to that value as the canonical tenant-qualified identity, not as raw input. The next actor `[mutation component: verify]` uses that identity when deciding which existing state to read, update, or reject `[decision rule: verify]`. The earlier identity page shows the source fragment where `NormalizedKey` is established; this page relies on that meaning and only traces how the already-canonical identity is consumed during the write.

**Prose Rules**

Claims:
- Every factual claim must be backed by code, tests, or a marked verification obligation.
- If evidence is absent, write `[verify: ...]` rather than implying behavior.
- Separate observed facts from inferred interpretation.

Stable vocabulary:
- Use one term only for the canonical identity: `NormalizedKey`.
- Do not alternate with synonyms such as “key,” “ID,” “lookup key,” or “tenant key” unless quoting code, and then explain the mapping once.
- Keep page-local placeholders stable too, for example `[request handler]`, `[mutation component]`, `[storage API call]`.

Explicit actors:
- Every sentence about behavior should name the actor performing it.
- Prefer forms like “`[actor]` validates...”, “`[actor]` passes...”, “`[actor]` rejects...”.
- Avoid actorless prose such as “the key is checked” unless the actor is genuinely unknown, in which case say `[actor unknown: verify]`.

Failure categories:
- Classify failures by source, not by symptom.
- Minimum categories:
  - Identity formation failure `[verify exact condition]`
  - Authorization or tenant-boundary failure `[verify exact condition]`
  - Concurrency or stale-state failure `[verify if applicable]`
  - Storage-operation failure `[verify exact API surface]`
  - Post-write visibility or acknowledgment failure `[verify if applicable]`
- If the implementation does not distinguish a category, mark that as a verification obligation instead of inventing one.

**Mechanical Checks Before Publication**

- Terminology lint: every use of canonical identity on later pages is `NormalizedKey`.
- Link check: later mutation page links to the earlier identity section and the original source fragment anchor.
- Duplication check: no repeated source fragment unless explicitly approved; if repeated, excerpt text matches the earlier page exactly.
- Claim marking check: all unresolved statements include `[verify: ...]`.
- Actor check: sentences describing behavior name an actor or explicitly mark actor unknown.
- Failure taxonomy check: every mentioned failure maps to one allowed category or is marked unverified.
- Placeholder check: all placeholders are either resolved from code or intentionally retained as verification obligations.
- Drift check: if the early identity page changes, the later mutation page’s local restatement is reviewed in the same change.

**Independent Review Before Publication**

Use a reviewer who did not draft the walkthrough and who reads it in page order.

Review checklist:
- Can the reviewer explain the later write-path page without rereading the full identity page, using only the local restatement and links?
- Does the reviewer see any claim that goes beyond cited code?
- Is `NormalizedKey` used consistently and without synonym drift?
- Are actors explicit at each important step?
- Are failure cases classified by source rather than mixed together?
- If a fragment is repeated, is the repetition necessary and identical to the original?

Publication gate:
- No unresolved factual claim without a verification marker.
- No uncited repeated fragment.
- No vocabulary drift on `NormalizedKey`.
- At least one independent reviewer signs off that the later page correctly uses, but does not redefine, the earlier identity fact.
