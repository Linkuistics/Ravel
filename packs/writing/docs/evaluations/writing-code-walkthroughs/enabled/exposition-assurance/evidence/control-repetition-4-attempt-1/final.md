**Page Design**

The later mutation page should restate the minimum local meaning it needs: `NormalizedKey` is the canonical tenant-qualified identity used to decide which record a write targets. It should not re-teach the full concept or re-argue why that identity is canonical.

It should link to:
1. The early identity page section that defines `NormalizedKey`.
2. The original source fragment anchor from the early page.
3. Any mutation-path source fragment that shows where the write path consumes or derives that identity.

It should not repeat the full early source fragment. Reuse by link, not duplication, unless publication format makes cross-page lookup impossible. The later page can quote or paraphrase one short sentence of meaning, then point back to the original fragment for proof. That avoids drift between two copies of the same evidence.

**What The Later Page States Locally**

The later page should say, in local prose:

- This write path operates on a `NormalizedKey`, not on an unqualified or partially qualified key form.
- The page assumes the earlier definition of `NormalizedKey` and uses it to explain target selection, conflict scope, and storage effects.
- Any step not directly verified from code should be marked as a verification obligation, for example `[VERIFY: explain how the write path obtains or validates the NormalizedKey]`.
- Any storage or concurrency behavior not established from code should be described as a placeholder, for example `[storage write call]`, `[precondition check]`, `[failure return]`.

**Representative Later-Page Paragraph**

On this mutation path, the service treats `NormalizedKey` as the canonical tenant-qualified identity for the record being written; that definition is established earlier and is reused here rather than redefined. After `[caller or handler]` obtains or receives the `NormalizedKey` `[VERIFY: source of normalization or validation]`, `[service component]` uses that identity to select the write target and to scope any existence check, overwrite decision, or conflict handling `[VERIFY: exact decision points]`. The storage operation itself is `[storage API call or sequence]`, issued by `[actor]`, and any failure is reported as one of the categories listed below rather than as an unstructured error `[VERIFY: mapping from storage failures to service-visible failures]`.

**Prose Rules**

Claims:
- Every behavioral claim must be backed by either a code citation, a source citation, or an explicit verification obligation.
- Do not infer intent from naming alone.
- Distinguish observed behavior from editorial interpretation with phrases like “the code shows” versus “this implies”.

Stable vocabulary:
- Use `NormalizedKey` exactly, with consistent casing.
- Use one term for each role and keep it fixed across pages: for example `caller`, `service`, `storage layer`, `record`, `mutation path`.
- Ban near-synonyms that blur scope, such as switching between “key”, “identifier”, “name”, and “handle” unless the code distinguishes them.

Explicit actors:
- Every causal sentence names the actor: who normalizes, who checks, who writes, who returns the error.
- Avoid actorless prose such as “the record is written” when “[service component] writes the record through [storage API]” is possible.
- If the actor is unknown, say `[actor unknown: verify]`.

Failure categories:
- Group failures by service-relevant category, not raw low-level messages.
- Minimum categories should be placeholders until verified: `[invalid input]`, `[identity mismatch or normalization failure]`, `[precondition or conflict failure]`, `[storage write failure]`, `[timeout or cancellation]`, `[unexpected internal failure]`.
- Each category needs a verified trigger, a verified actor, and a verified outward effect.

**Mechanical Checks**

- Link check: every intra-doc link and source-fragment anchor resolves.
- Terminology check: `NormalizedKey` appears with exact spelling; banned synonyms are flagged.
- Claim audit: each paragraph contains either citations or `[VERIFY: ...]` markers for unverified points.
- Placeholder audit: all bracketed placeholders are either resolved before publication or listed in a formal open-obligations appendix; none may appear silently.
- Actor scan: flag sentences with passive voice or missing actors in write-path explanations.
- Failure taxonomy check: every mentioned failure maps to one approved category.
- Duplicate-evidence check: the early source fragment is referenced by link, not copied again, unless an exception is recorded.
- Drift check: if a short restatement of the early fragment appears later, it must match the early page’s wording and cited conclusion.

**Independent Review Before Publication**

1. Domain-naive review: a reviewer who knows the language and storage APIs, but not the service domain, reads only the walkthrough and marks every unexplained jump.
2. Code-evidence review: a separate reviewer verifies that each claim about the mutation path is supported by cited code or left as an explicit obligation.
3. Terminology review: one reviewer checks stable vocabulary and ensures `NormalizedKey` is used consistently across pages.
4. Failure-model review: one reviewer checks that failures are categorized consistently and that no storage-specific behavior is presented as confirmed without proof.
5. Final signoff rule: publish only when duplicated evidence is removed, links resolve, and every placeholder is either discharged or explicitly tracked as a verification obligation.
