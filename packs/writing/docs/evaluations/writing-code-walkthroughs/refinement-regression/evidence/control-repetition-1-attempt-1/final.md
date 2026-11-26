**Later Page Design**

The later mutation page should restate only the semantic dependency it needs:

- Local statement: the write path is operating on a `NormalizedKey`, and `NormalizedKey` means the canonical tenant-qualified identity established earlier.
- Link 1: the earlier concept page that defines `NormalizedKey`.
- Link 2: the anchored early source fragment that shows where that canonical form is established.
- Source fragment on later page: do not repeat it by default.

Reasoning: the later page needs the meaning, not a second full proof. Repeating the fragment creates drift risk and invites readers to compare two copies instead of following one canonical reference. If the later page truly needs exact code text for a narrow local argument, use a short quoted excerpt with an explicit “canonical source is on the earlier page” note; otherwise link only.

A good local pattern is:

- One reminder sentence.
- One sentence narrowing the claim to this page’s concern.
- One link sentence sending readers back for the derivation proof.

**Representative Paragraph**

“From this point onward, the write path is using a `NormalizedKey`, meaning the canonical tenant-qualified identity established on the earlier identity page. This page relies on that earlier definition and its linked source fragment; it does not re-derive normalization here. The local claim is only that the code reaching `[storage write API]` is operating on that canonical identity rather than on a pre-normalized form. Any claim about additional transformation, revalidation, or recovery behavior is a `[verification obligation]` until matched to specific code.”

**Prose Rules**

- Claims: every factual claim must be either backed by a cited code location or marked inline as a verification obligation such as `[verify from code]`. Do not write “ensures,” “guarantees,” “prevents,” or similar strong language without direct evidence.
- Stable vocabulary: use one term per concept across pages. Always write `NormalizedKey` exactly, and always gloss it as “canonical tenant-qualified identity” unless the earlier definition is being quoted. Do not alternate with synonyms like “full key,” “qualified key,” or “canonical id.”
- Explicit actors: every action sentence must name the actor. Prefer forms like “the write path passes…”, “[normalization step] derives…”, “[storage client] writes…”. Avoid actorless prose such as “it is written” or “the key is checked” unless the responsible code is genuinely unknown, in which case write `[actor to verify]`.
- Failure categories: use a closed, evidence-backed set of failure labels for the whole walkthrough. Do not infer categories from intuition. If the code support is not yet verified, use placeholders like `[failure category pending verification]` rather than inventing names or behavior.

**Mechanical Checks Before Publication**

- Link check: every cross-page link and source-fragment anchor resolves.
- Claim check: each nontrivial claim has either a citation or a verification marker.
- Terminology check: `NormalizedKey` and “canonical tenant-qualified identity” are used consistently.
- Duplication check: the early source fragment is not silently duplicated on the later page.
- Actor check: sentences describing behavior have explicit subjects.
- Placeholder gate: unresolved placeholders or `[verification obligation]` markers block publication unless intentionally tracked in an exceptions list.
- Failure-label check: only approved failure categories appear in the draft.
- Drift check: the later page does not make a stronger claim than the earlier identity page or cited code supports.

**Independent Review**

- Implementation review by a reader who knows the code: verify every claim, every link target, and every stated failure category against source.
- Domain-naive editorial review by a reader matching the target audience: confirm the later page is understandable without domain background and that the back-link to the identity page is sufficient.
- Final publication review: one reviewer other than the author confirms all verification obligations are resolved or explicitly deferred in a tracked pre-publication list.
