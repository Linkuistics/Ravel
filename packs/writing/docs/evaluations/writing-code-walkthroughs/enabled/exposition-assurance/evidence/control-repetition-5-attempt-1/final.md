**Later Page Design**

The mutation page should restate only the minimum local fact it needs: `NormalizedKey` is the canonical tenant-qualified identity, and the write path uses that identity as its stable handle for locating or mutating state. It should not re-teach the full concept, derivation, or rationale there.

It should link to:
- the early conceptual section where `NormalizedKey` is defined
- the original source fragment that first grounded that definition
- the exact mutation-path source fragment being discussed on the later page

It should not repeat the full early source fragment by default. Reuse a link unless the write-path explanation depends on exact tokens from that earlier fragment. If it does, repeat only the smallest excerpt needed, verbatim, with the original citation and a note that this is reused evidence, not a second definition.

Representative later-page paragraph:

> This write path operates on a `NormalizedKey`, the canonical tenant-qualified identity introduced earlier. At `[mutation entry point]`, actor `[caller or handler]` must already hold that value or derive it in `[normalization step: verification obligation]`; the walkthrough should name which of those is true rather than imply both. From this point onward, the page should describe reads, checks, and writes in terms of `NormalizedKey` only, unless the code shows a later step also consults `[raw input placeholder]`. The construction details belong to the earlier identity page; this page links back to `[NormalizedKey section]` and to `[early source fragment citation]` because that earlier evidence explains why the write path treats this value as the authoritative identity.

**Prose Rules**

Claims:
- Every behavioral claim must be backed by a cited code fragment or labeled `verification obligation`.
- Distinguish `observed in code`, `inferred from structure`, and `unknown`.
- Do not claim ordering, uniqueness, atomicity, retries, idempotence, or durability unless the code shown establishes them.

Stable vocabulary:
- Use `NormalizedKey` exactly for the canonical tenant-qualified identity.
- Use one placeholder term for any pre-normalized input, such as `[raw identifier placeholder]`, and do not switch synonyms.
- Reserve `identity` for semantic meaning and `key` for the concrete program value only if the page keeps that distinction consistently.

Explicit actors:
- Each operational sentence should name an actor: `[caller]`, `[normalizer]`, `[mutation handler]`, `[storage layer]`, `[validator]`.
- Avoid actorless passive prose like “the key is checked” when the code shows who checks it.

Failure categories:
- Every failure mention must be assigned to a category visible to the reader.
- Use category labels such as `input/validation failure`, `identity construction or lookup failure`, `storage/write failure`, and `[other category: verification obligation]` only where code evidence exists.
- If the code does not expose failure handling, state that as unknown rather than invent a category outcome.

**Assurance Before Publication**

Mechanical checks:
- Terminology lint: `NormalizedKey` spelling and capitalization are consistent across pages.
- Link check: the later page links to the early definition and cited fragments resolve to the intended anchors.
- Claim ledger: each sentence is tagged as `evidenced`, `inferred`, or `verification obligation`.
- Placeholder scan: all bracketed placeholders are either resolved or intentionally retained as explicit unknowns.
- Reuse check: if the early fragment is repeated, the excerpt matches the original exactly and is shorter than the original fragment.
- Consistency check: the later page does not redefine `NormalizedKey` or broaden its meaning.

Independent review:
- One domain-naive reviewer checks that the later page is understandable without rereading the full early section.
- One implementation-aware reviewer checks that every claim matches the cited code and that unknowns remain marked.
- Publication gate: reject if any unstated inference appears as fact, if vocabulary drifts, or if a failure mode is described without evidence.
