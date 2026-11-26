**Exposition Plan**

On the later mutation page, state the meaning locally in one short reminder sentence and then use that meaning operationally. The page should not re-teach the concept; it should establish the minimum needed to follow the write path:

- Local statement: "`NormalizedKey` is the canonical tenant-qualified identity for the record this mutation addresses."
- Local consequence: "This page assumes that meaning and traces where the write path establishes, preserves, or depends on that identity before the storage call."
- Verification obligation: "If any step accepts a broader, pre-normalized, or partially qualified identifier, the page must mark that boundary as a verification obligation unless the code shown proves normalization there."

The later page should link to two earlier targets:

- The early explanatory section that defines `NormalizedKey` conceptually.
- The early source fragment anchor that shows the defining code or first establishment point.

It should not repeat the full early source fragment on the later page. Reuse the earlier fragment by link, not by duplication, unless the later page needs exact syntax from that fragment for a new claim. In that case, repeat only the minimal excerpt and label it as a reused excerpt from the earlier page. The default is: no full repetition.

**Representative Later-Page Paragraph**

Before the write reaches `[storage write call]`, the path must already be operating on `NormalizedKey`, the canonical tenant-qualified identity introduced on the earlier `NormalizedKey` page. This page does not re-derive that definition; it relies on it while tracing how `[mutation entrypoint]`, `[normalization or lookup step]`, and `[storage write call]` preserve the identity of the record being changed. Where the code shown does not prove that the incoming identifier is already normalized, this walkthrough treats that boundary as a verification obligation rather than an established fact and links back to the earlier defining fragment for the source basis.

**Prose Rules**

Claims:
- Every factual claim must be either source-backed on the page, explicitly linked to an earlier source-backed claim, or marked as a verification obligation.
- Distinguish observed behavior from inference. Use labels such as `Observed`, `Inference`, or `Verification obligation` if needed.
- Do not state intended behavior as implemented behavior without source proof.

Stable vocabulary:
- Use `NormalizedKey` exactly; do not alternate with synonyms such as “canonical key,” “tenant key,” or “qualified id.”
- Use one term consistently for each role: `mutation`, `caller`, `storage write`, `tenant`, `failure`.
- After the first full definition, refer to the concept as `NormalizedKey` or “the canonical tenant-qualified identity,” and do not introduce new paraphrases.

Explicit actors:
- Name the actor in each important sentence: the caller, the mutation handler, the normalization step, the storage layer, or the reviewer.
- Avoid passive constructions when they hide responsibility for establishing or preserving identity.
- If a sentence contains “it,” the antecedent must be unambiguous on first read.

Failure categories:
- Use only stable, high-level categories unless code proves finer ones.
- Recommended categories: `identity establishment failure`, `precondition or validation failure`, `storage operation failure`, `outcome unknown; verification required`.
- Do not invent concrete error types, retries, or recovery behavior without evidence.

**Assurance Before Publication**

Mechanical checks:
- Every page-local claim has a citation, backward link, or `verification obligation` label.
- Every cross-reference resolves to the intended section or source fragment anchor.
- `NormalizedKey` spelling and capitalization are consistent everywhere.
- Banned synonym scan: no uncontrolled substitutes for `NormalizedKey`.
- Actor scan: key explanatory sentences have explicit actors and no ambiguous pronouns.
- Failure-language scan: each failure statement uses one of the approved categories.
- Duplicate-fragment check: the full early source fragment appears only once unless an intentional minimal reused excerpt is marked as such.
- Placeholder sweep: every placeholder is either resolved or left as an explicit verification obligation.

Independent review:
- A domain-blind reviewer who knows the language and storage APIs reads the early `NormalizedKey` page and the later mutation page to confirm the later page is understandable without domain knowledge.
- A source-trace reviewer checks that each claim on the later page is supported by local code, a valid backward link, or a clearly marked obligation.
- An editorial reviewer checks vocabulary stability, actor clarity, and failure-category consistency.
- Publication should block on any unresolved claim presented as fact; unresolved items may remain only as explicit verification obligations.
