**Later Page Design**

The later mutation page should restate the minimum semantic fact locally:

- `NormalizedKey` is the canonical tenant-qualified identity.
- This write path must be explained in terms of operations on that identity, not on raw caller input.
- The page does not re-explain how the identity is derived; that remains on the earlier page.
- If the code is not yet verified on whether the path receives or constructs the `NormalizedKey`, say so as a verification obligation.

It should link to:

- The earlier explanatory section that defines `NormalizedKey`.
- The exact earlier source fragment anchor that demonstrates the canonical-identity fact.
- Any glossary anchor for `NormalizedKey`, if the walkthrough has one.

It should not repeat the full early source fragment. Use a local prose reminder plus an anchor back to the original fragment. Repeat only if publication tooling supports canonical transclusion; do not paste a second independent copy.

Representative later-page paragraph:

> This mutation path relies on `NormalizedKey` as the canonical tenant-qualified identity introduced earlier in [Canonical Identity] and evidenced in [Fragment A]. On this page, every lookup, comparison, and storage write is described relative to that identity rather than to pre-normalized caller input. The remaining implementation question is whether `[mutation entrypoint]` receives a `NormalizedKey` directly or derives it in `[normalization step]`; until that is checked in code, treat that point as a verification obligation rather than an asserted behavior.

**Prose Rules**

Claims:

- Every nontrivial claim must cite code, tests, or an explicit verification obligation.
- Use assertive prose only for verified facts.
- Use `appears`, `if`, or `[verify: ...]` for unverified control flow, failure handling, or data transformations.

Stable vocabulary:

- Use `NormalizedKey` exactly, with that capitalization.
- Use one stable phrase for its meaning: `canonical tenant-qualified identity`.
- Do not alternate with uncited synonyms such as “logical key”, “full key”, or “resolved key”.

Explicit actors:

- Name the actor in each important sentence: caller, mutation path, normalization step, storage API, or `[actor to verify]`.
- Avoid actorless prose like “the key is written” when the code actually shows a component doing the work.

Failure categories:

- Use only evidence-backed categories.
- Preferred buckets are: input/eligibility failure, canonicalization failure, storage-operation failure, conflict-or-staleness failure if present, and post-write invariant failure if present.
- If the code does not clearly support a bucket, label it `[failure mode to verify]` instead of inventing one.

**Assurance Plan**

Mechanical checks before publication:

- Citation check: every factual paragraph has at least one source anchor or a marked verification obligation.
- Terminology check: `NormalizedKey` and `canonical tenant-qualified identity` are used consistently.
- Link check: the mutation page links resolve to the earlier concept section and exact fragment anchor.
- Fragment reuse check: the early fragment has one canonical identifier and is not duplicated inconsistently.
- Claim-modality check: unverified statements are visibly marked as conditional or pending verification.
- Actor check: key operational sentences have explicit subjects.
- Placeholder sweep: every `[ ... ]` placeholder is either resolved or carried into a tracked publication blocker.

Independent review before publication:

- A technical reviewer validates that each claim matches code and that all verification obligations are real and bounded.
- A domain-naive reviewer reads the later mutation page alone and confirms it is locally understandable, with links sufficient for deeper context.
- An editorial reviewer checks vocabulary stability, link precision, and that no duplicated fragment has drifted from the canonical source.

Publication gate:

- Do not publish while any unbounded placeholder remains, any asserted behavior lacks a citation, or the later page depends on rereading the early page for a fact it should have restated locally.
