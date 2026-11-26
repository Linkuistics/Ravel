**Exposition Plan**

Use a two-level treatment on the later mutation page:

1. State the dependency locally.
2. Link back to the early identity page for the full definition and original evidence.

The later page should locally say, in plain prose, that the write path operates on `NormalizedKey`, and that `NormalizedKey` is the canonical tenant-qualified identity already established earlier. That reminder is necessary because the mutation explanation depends on the meaning, not just the name.

It should link to:
- The early page section that defines `NormalizedKey`.
- The early source fragment that demonstrates or motivates that definition.
- If available, the later page’s own write-path fragment showing where that identity is used.

It should not repeat the full early source fragment on the later page. Repeat only if a specific line is required to understand immediate control flow on that page; in that case, use a short excerpt with a note that it is reused context, not new evidence. Default: no full duplication, only a local semantic reminder plus a deep link.

**Representative Later-Page Paragraph**

Before this write reaches storage, the request is understood in terms of `NormalizedKey`, the canonical tenant-qualified identity defined earlier ([Identity Definition], [Earlier Source Fragment]). This page does not re-establish that meaning; it relies on it. The walkthrough here should verify where the write path first requires a canonical identity, what actor performs that step, and whether any branch can proceed without it. If the code shows additional preconditions, normalization stages, or rejection paths, document them here with direct citations; otherwise mark them as verification obligations rather than inferring behavior.

**Prose Rules**

Claims:
- Every behavioral claim must be backed by a cited source fragment or explicitly labeled as a verification obligation.
- Do not infer ordering, atomicity, validation, or error handling unless the code shows it.
- Separate observed facts from explanatory interpretation.

Stable vocabulary:
- Use `NormalizedKey` exactly and consistently.
- Choose one term for each stable concept and keep it fixed across pages: `request`, `actor`, `write path`, `storage`, `failure`.
- Do not introduce synonyms for the same concept unless the code uses them and the distinction matters.

Explicit actors:
- Every important sentence should name the actor.
- Prefer forms like “the write-path entrypoint constructs...”, “the storage adapter receives...”, or “the caller handles...”.
- If the actor is unknown from current evidence, say “an as-yet-unverified actor” and attach a verification obligation.

Failure categories:
- Classify failures by where they occur, not by guessed implementation detail.
- Minimum categories: identity/precondition failure, storage operation failure, concurrency/order failure, and propagation/reporting failure.
- Only populate a category when code evidence exists; otherwise leave it as a review prompt.

**Mechanical Checks Before Publication**

- Claim-to-citation check: every nontrivial claim has a source link or a verification-obligation label.
- Vocabulary lint: `NormalizedKey` spelling is uniform; no drift between “tenant-qualified identity” and other aliases.
- Actor check: each paragraph has explicit actors for key actions.
- Failure taxonomy check: every mentioned failure is placed in an approved category.
- Link check: all cross-page anchors and source-fragment links resolve.
- Repetition check: no full duplicate fragment appears unless explicitly justified as necessary context.
- Placeholder check: all unknown details are marked as `[verification required]` or equivalent, not narrated as fact.
- Scope check: no domain explanation exceeds what the cited code supports.

**Independent Review Before Publication**

Use two reviewers:

1. A code-familiar reviewer checks factual fidelity.
   - Verifies each claim against code.
   - Confirms no invented behavior, names, or guarantees.
   - Confirms the later page uses `NormalizedKey` consistently with the early page.

2. A domain-new reviewer checks readability.
   - Verifies the later page is understandable without re-reading the entire early section.
   - Confirms the local reminder is sufficient and the link is enough for deeper context.
   - Flags any sentence where actor, obligation, or failure mode is unclear.

Publication gate:
- No uncited factual claims.
- No unresolved wording drift around `NormalizedKey`.
- No duplicated source fragment without a written justification.
- All verification obligations either closed or explicitly carried as open review items.
