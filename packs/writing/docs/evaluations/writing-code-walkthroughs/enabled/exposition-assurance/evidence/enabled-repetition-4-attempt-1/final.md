**Exposition Plan**

The later mutation page should restate the minimum semantic fact it needs locally: `NormalizedKey` is the canonical tenant-qualified identity used to determine which logical item the write targets. It should not re-teach the full concept, derivation, or normalization rules on that page unless the write-path explanation depends on one of those rules. Instead, it should link back to the early page for the full definition and any source-backed explanation of construction or invariants.

It should also link directly to the earlier source fragment if that fragment is the best evidence for the identity claim used in the write path. The later page should not fully repeat that fragment by default. Repeat it only if one of these is true:
1. The reader cannot follow the later page without seeing the exact lines in place.
2. The later page makes a new claim about those exact lines that would be hard to verify by cross-page navigation.
3. The fragment is short enough that repetition improves comprehension more than it harms continuity.

In the normal case, use a short local reminder plus a link, not full fragment repetition. That keeps the later page self-sufficient at the concept level while preserving one authoritative explanation of the identity model.

**Later-Page Paragraph**

Before this write can affect storage, the service identifies the target item by its `NormalizedKey`, the canonical tenant-qualified identity introduced earlier at [Identity page: `NormalizedKey`](#). This page relies on that meaning but does not re-derive it: the verification obligation here is to show where the write path receives, preserves, or reconstructs that canonical identity before any storage mutation occurs. If the implementation also validates, transforms, or rejects candidate keys along this path, those steps should be described only against cited source and named here as `[key validation step]`, `[normalization step]`, or `[rejection path]` until the code excerpt confirms the exact behavior.

**Prose Rules**

Claims:
- Every behavioral claim must be backed by a cited source fragment, API contract, test, or marked as a verification obligation.
- Do not compress multiple facts into one sentence if they require different evidence.
- If behavior is unknown, say `unknown from current evidence` or `verification obligation`, not an inferred implementation story.

Stable vocabulary:
- Use `NormalizedKey` exactly and consistently for the canonical tenant-qualified identity.
- Choose one term each for `write`, `mutation`, `request`, `storage operation`, and `failure`, then keep them stable across pages.
- Do not alternate between near-synonyms such as `record`/`entry`/`item` unless the code distinguishes them and the page explains that distinction.

Explicit actors:
- Name the actor in each operational sentence: `caller`, `service`, `[write handler]`, `storage API`, `[validation step]`, `[authorization step]`.
- Avoid passive constructions like “the key is checked” when the responsible component matters.
- When the code boundary is not yet verified, use placeholders such as `[mutation coordinator]` and mark them for confirmation.

Failure categories:
- Separate failures by cause, not by generic outcome.
- Minimum categories should be placeholders such as `[identity formation failure]`, `[tenant mismatch failure]`, `[precondition failure]`, `[storage write failure]`, and `[concurrency/conflict outcome]` only if source confirms they exist.
- If the code does not yet prove a category, list it as a candidate category with a verification obligation rather than presenting it as established behavior.

**Assurance Plan**

Mechanical checks:
- Terminology lint: every use of `NormalizedKey` matches the approved spelling and meaning.
- Claim audit: each nontrivial sentence is tagged internally as `cited`, `derived from cited facts`, or `verification obligation`.
- Cross-reference check: the later mutation page links to the early identity page and any reused source fragment.
- Fragment reuse check: repeated source appears only when justified by the page policy above.
- Actor check: each step in the write path names an actor.
- Failure taxonomy check: each failure statement fits one defined category or is explicitly marked unknown.
- Placeholder sweep: all `[placeholder]` names are resolved or retained with publication-blocking verification notes.
- Drift check: the later page’s summary of `NormalizedKey` matches the early page wording on canonical meaning.

Independent review before publication:
- One domain-naive reviewer reads the pages in order and flags any place where the later mutation page assumes identity knowledge not actually carried forward locally.
- One implementation-aware reviewer checks every claim against source and storage API evidence, with special attention to the reused early fragment.
- One editorial reviewer checks vocabulary stability, actor naming, and whether any unknown behavior was narrated as fact.
- Publication is blocked if reviewers find uncited behavior, inconsistent identity terminology, ambiguous actor ownership, or failure categories stated without evidence.
