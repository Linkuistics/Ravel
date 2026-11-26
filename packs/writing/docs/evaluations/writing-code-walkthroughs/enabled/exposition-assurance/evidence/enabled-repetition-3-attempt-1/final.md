Use a two-level treatment on the later mutation page: give a one-sentence local reminder of what `NormalizedKey` means, then deep-link to the early identity page and its anchored source fragment for the full definition. Do not repeat the full early source fragment on the mutation page; repeat only the semantic consequence in prose. The later page needs local readability, but the code fragment should stay single-sourced unless the later explanation depends on exact tokens or control flow from that fragment. If that happens, reuse a short anchored excerpt and mark it as repeated from the earlier page.

What the later page should state locally:
- "`NormalizedKey` is the canonical tenant-qualified identity used by this write path."
- "All claims below about lookup, comparison, overwrite, or persistence apply to that canonical identity unless a cited fragment shows a different intermediate form."
- "Any detail about how the key is formed, validated, or propagated that is not shown here is delegated to the earlier page or marked as a verification obligation."

What it should link to:
- The early explanatory page where `NormalizedKey` is introduced.
- The exact early source fragment anchor that establishes the canonical-identity claim.
- Any mutation-page fragment that first consumes or propagates `NormalizedKey`.

Whether to repeat the source fragment:
- Default: no.
- Exception: repeat a short excerpt only if the later page makes a line-specific argument that cannot be understood from prose plus a link.
- If repeated, keep the earlier page authoritative and label the later excerpt as contextual reuse.

Representative later-page paragraph:

“From `[mutation entry point]` onward, this walkthrough treats `NormalizedKey` as the subject of the write, not the caller’s pre-normalized input. That meaning is established earlier in `[Early page: NormalizedKey]` and grounded in `[Early source fragment anchor]`; this page relies on that contract rather than re-deriving it. As a result, each downstream step described here, including `[lookup step]`, `[conflict/precondition step]`, and `[storage mutation step]`, should be read as operating on the canonical tenant-qualified identity unless a cited fragment explicitly shows another representation. Any unshown detail about construction, validation, or failure at this boundary remains a verification obligation.”

Prose rules:
- Claims: every concrete claim must be tagged as either directly shown by a cited fragment, inferred from adjacent shown code, or left as a verification obligation. Do not let inference read like observation.
- Stable vocabulary: define terms once and reuse them exactly. If `NormalizedKey` is the canonical tenant-qualified identity, do not later rename it to “qualified key,” “canonical key,” or similar.
- Explicit actors: each important sentence names the actor, such as “caller,” “service code,” “normalization step,” “storage API,” or `[actor pending verification]`. Avoid actorless passive voice for control-flow or failure claims.
- Failure categories: only name a failure category when the source supports it. Otherwise use placeholders such as `[failure category pending verification]`. Keep categories disjoint, for example `[identity formation failure]`, `[precondition/state failure]`, `[storage failure]`, `[unknown from current evidence]`, but only where verified.

Mechanical checks before publication:
- Every nontrivial claim has a citation or an explicit verification-obligation marker.
- Every reuse link resolves to the exact earlier page section or fragment anchor.
- `NormalizedKey` terminology is consistent across pages with no unchecked synonyms.
- No invented identifiers, behaviors, or error modes appear without placeholder brackets.
- Repeated excerpts, if any, are byte-identical to the source excerpt they reuse.
- Each paragraph has an explicit actor for state changes, checks, and failures.
- Every stated failure category is evidenced by code, tests, or intentionally marked unresolved.

Independent review before publication:
- Source-fidelity review by a reader who checks each claim against the cited fragments and removes any unstated behavior.
- Reader-comprehension review by someone who knows the language and storage APIs but not the service domain; they should confirm the later page is understandable with only the local reminder plus the backlink.
- Final editorial pass that checks vocabulary stability, link hygiene, placeholder resolution, and that unresolved points remain clearly marked as verification obligations rather than implied facts.
