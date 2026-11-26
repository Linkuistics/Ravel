**Exposition Plan**

The later mutation page should state one local fact explicitly and narrowly: `NormalizedKey` is the write path’s canonical tenant-qualified identity, and every mutation step described on that page is interpreted through that meaning. It should not re-explain normalization rules, construction rules, or any broader identity model unless the later page makes a new claim about them.

It should link to:
- the early identity page, as the normative explanation of what `NormalizedKey` means
- the original early source fragment, by stable anchor, as the canonical evidence already introduced there

It should not repeat the full early source fragment by default. Reuse the original fragment through a link, then restate only the operational consequence needed for the write-path explanation. Repeat the fragment only if the later page depends on exact line-level reading that would otherwise force the reader to bounce back mid-paragraph. If repeated, mark it as a deliberate reprise of the same evidence, not a new source.

**Decision for the Later Page**

Locally, the later page should say:
- this write path operates on a `NormalizedKey`
- on this page, that means “the already-canonical tenant-qualified identity introduced earlier”
- any step that derives, validates, stores, routes, or deduplicates by identity must be described in terms of that object, not in terms of raw inputs
- any missing implementation fact about where the key is created, checked, or persisted remains a verification obligation

It should link back for:
- the definition and rationale of `NormalizedKey`
- the first appearance of the source fragment
- any proof that multiple raw identifiers collapse to one canonical identity, if such proof exists

It should not repeat:
- the full earlier explanation
- speculative behavior about mutation semantics
- source lines unless exact syntax matters to the new claim

**Representative Later-Page Paragraph**

In this mutation path, the actor performing the write receives or reconstructs a `NormalizedKey`; for this walkthrough, that name retains its earlier meaning as the canonical tenant-qualified identity. The write explanation on this page therefore treats identity-bearing steps as operating on that canonical form, not on raw request fields or intermediate identifiers. The earlier identity fragment remains the primary evidence for that meaning and is linked here rather than re-explained. If the implementation also validates, persists, or indexes by `NormalizedKey`, those points must be shown with source citations on this page; otherwise they remain verification obligations.

**Prose Rules**

Claims:
- Every behavioral claim must be backed by a cited source fragment or labeled as a verification obligation.
- Do not infer ordering, uniqueness, idempotence, conflict handling, or persistence behavior without direct evidence.
- When a page reuses an earlier fact, restate only the consequence needed for the current step.

Stable vocabulary:
- Use `NormalizedKey` exactly and consistently.
- Use one stable phrase for its meaning: “canonical tenant-qualified identity.”
- Do not introduce synonyms such as “canonical key,” “tenant ID,” or “primary identity” unless the code uses them and the distinction is explained.

Explicit actors:
- Every sentence about behavior should name the actor: “the caller,” “the write handler,” “the storage layer,” “the validator,” or a placeholder such as `[mutation component]`.
- Avoid actorless prose like “the key is checked” when the code does not show who performs the check.

Failure categories:
- Separate failures into at least these buckets: invalid identity input, authorization/tenant-scope mismatch, storage write failure, concurrency/conflict outcome, and unknown behavior.
- If the code evidence does not support one of these categories, mark it as “not yet evidenced” rather than silently omitting it.
- Do not merge user-visible rejection with internal storage failure unless the implementation proves they are handled identically.

**Assurance Plan Before Publication**

Mechanical checks:
- Terminology lint: every occurrence of `NormalizedKey` uses the same gloss on first use per page.
- Claim audit: each nontrivial claim has a citation or an explicit verification-obligation label.
- Anchor check: later-page links to the early identity page and early fragment resolve correctly.
- Repetition check: repeated source fragments, if any, are byte-identical to the original excerpt and labeled as reprises.
- Actor check: sentences describing actions have an explicit actor.
- Failure check: mutation page has a dedicated section or callouts for evidenced failures and unknowns.
- Placeholder check: every placeholder or unknown is tracked in a verification list and none read like established fact.

Independent review:
- One reviewer checks source faithfulness only: no unstated behavior, no overclaiming, no softened uncertainty.
- A second reviewer checks reader usability only: the later page is understandable without rereading the entire early page, and links are sufficient where context is deferred.
- Reviewers should specifically test the mutation page for this question: “Can I tell what `NormalizedKey` means here, and do I know where to go for the full proof?”
- Publication is blocked if reviewers find invented behavior, unstable terminology, uncited claims, or missing verification obligations for key write-path assertions.
