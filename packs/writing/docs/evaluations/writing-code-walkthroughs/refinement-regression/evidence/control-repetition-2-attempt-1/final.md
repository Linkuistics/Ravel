**Later Page Design**

The later mutation page should state locally:

- `NormalizedKey` is already the canonical tenant-qualified identity established earlier.
- In this write path, that meaning matters because the mutation must be described in terms of one identity form from `[entry point]` through `[storage write]`.
- Any step that appears to use a different identity form is not explained away in prose; it becomes a verification obligation.

It should link to:

- the early-page definition anchor for `NormalizedKey`
- the early source-fragment anchor that shows the defining code or first canonicalization point
- a glossary anchor for `canonical` / `tenant-qualified` if those terms are defined separately

It should not repeat the full early source fragment by default. Repeat only a minimal excerpt if the later page needs exact token-level reading to justify a local claim. Otherwise, link back and restate the consequence in prose. That avoids drift and keeps the later page about the write path, not about re-teaching identity formation.

**Representative Paragraph**

Before `[storage write]`, this path is already operating on a `NormalizedKey`; the earlier page defines that type as the canonical tenant-qualified identity. The local point here is narrower: every comparison, guard, and persisted mutation in `[write path name or placeholder]` must refer to that same identity form, not to an alternate tenant/key representation. The defining explanation and source fragment are linked at `[NormalizedKey definition]` and `[source fragment]`; this page relies on them rather than restating the contract. Verification obligation: confirm in code that no step between `[entry point]` and `[storage write]` substitutes a different identity form or drops tenant qualification.

**Prose Rules**

Claims:
- Every substantive claim is one of: `Observed`, `Inference`, or `Verification obligation`.
- `Observed` claims cite a page anchor to code.
- `Inference` claims name the supporting observations.
- Unknown behavior stays in placeholders; do not fill gaps with plausible system behavior.

Stable vocabulary:
- Use `NormalizedKey` exactly, with that capitalization, everywhere.
- Use `canonical tenant-qualified identity` as the fixed explanatory phrase.
- Do not introduce synonyms such as “real key”, “storage key”, or “composite key” unless the code uses them and the relation is explicitly defined.

Explicit actors:
- Every operational sentence names the actor: `[handler]`, `[normalization step]`, `[storage adapter]`, `[caller]`, or another verified component name.
- Avoid passive constructions like “the key is written” when the code makes the writer identifiable.
- Pronouns are allowed only when the antecedent is unambiguous within the same paragraph.

Failure categories:
- Use a closed set per page.
- Categories are about where the failure arises, not about vague symptoms.
- Until verified, use placeholders such as `[identity formation failure]`, `[precondition failure]`, `[storage operation failure]`, `[result propagation failure]`.
- Each failure statement must say: actor, trigger, surfaced signal, and immediate effect on the path.

**Assurance Plan Before Publication**

Mechanical checks:
- Terminology check: every use of `NormalizedKey` matches the canonical spelling.
- Link check: first mention on the later page links to the early definition; callback link to the early source fragment resolves.
- Claim check: every nontrivial sentence is tagged or auditable as `Observed`, `Inference`, or `Verification obligation`.
- Placeholder check: unresolved placeholders block publication unless explicitly listed as open verification items.
- Duplication check: if any source fragment is repeated, it matches the canonical excerpt exactly and is marked as reused.
- Vocabulary drift check: no unapproved synonyms for the same concept across pages.
- Failure-taxonomy check: every named failure fits one declared category and uses the required actor/trigger/signal/effect form.

Independent review:
- One reviewer familiar with the implementation verifies claim-to-code traceability.
- One reviewer unfamiliar with the service domain but familiar with the language and storage APIs checks that the walkthrough is understandable without hidden domain knowledge.
- Reviewers specifically confirm three things: the later page does not silently redefine `NormalizedKey`, the callback to the early fragment is sufficient without duplication, and every unresolved behavior is called out as a verification obligation rather than presented as fact.
