**Exposition Plan**

Use the early page as the normative definition site for `NormalizedKey`. The later mutation page should not try to redefine it. It should carry only the minimum local reminder needed to make the write-path explanation self-contained:

- Local statement on the later page: `NormalizedKey` is the canonical tenant-qualified identity, and this write path must preserve that meaning at every step that selects, writes, or deduplicates data.
- Link targets from the later page:
  - the earlier definition page section that establishes the meaning of `NormalizedKey`
  - the earlier source-fragment anchor, because that fragment is still the best evidence for the identity claim
  - any later-page placeholder anchor for the write-path step where the key is introduced or consumed, such as `[mutation entrypoint]` or `[key-construction step]`
- Repetition decision: do not repeat the full early source fragment on the mutation page. Repeat only a short paraphrase of its already-established meaning, plus a link back to the original fragment. Reuse the fragment only if review shows the later argument depends on exact syntax rather than on the already-explained semantic fact; if so, include a small quoted excerpt and mark the need as a verification obligation.

This keeps the later page readable without making readers page back for the basic meaning, while avoiding duplicate exposition and duplicate evidence blocks.

**Representative Later-Page Paragraph**

“In this mutation path, the actor `[caller or request handler]` supplies or derives inputs that must resolve to a `NormalizedKey`, which earlier pages define as the canonical tenant-qualified identity. From this point onward, every lookup, write, and conflict check in `[mutation entrypoint]` should be explained in terms of that identity rather than in terms of raw request fields or alternative key forms. The walkthrough should point back to the earlier `NormalizedKey` source fragment as the evidence for that meaning, and it should mark any unverified step in `[key-construction step]`, `[storage write step]`, or `[failure handling step]` as a verification obligation rather than as established behavior.”

**Prose Rules**

- Claims:
  - Separate facts, inferences, and obligations.
  - State a code-backed claim only when the page cites the exact source location.
  - Mark uncited reasoning as “inference” or “verification obligation.”
  - Never imply behavior that has not been checked in code.
- Stable vocabulary:
  - Use `NormalizedKey` exactly and consistently.
  - Use one term for each concept across all pages: `tenant`, `identity`, `mutation path`, `write`, `lookup`, `failure`.
  - Do not introduce synonyms such as “canonical key,” “qualified id,” or “record key” unless the code itself uses them and the page explains the distinction.
- Explicit actors:
  - Every sentence about behavior should name the actor: `[request handler]`, `[validation step]`, `[key normalizer]`, `[storage layer]`, `[conflict detector]`.
  - Avoid actorless prose such as “the key is written” when “`[storage layer]` writes using the `NormalizedKey`” is available.
- Failure categories:
  - Classify failures by cause, not by vague outcome.
  - Minimum categories:
    - input or validation failure
    - identity construction or normalization failure
    - storage read/write failure
    - conflict or precondition failure
    - invariant mismatch or unexpected-state failure
  - If the implementation’s actual categories are unknown, keep these as placeholders and label them verification obligations.

**Assurance Plan Before Publication**

Mechanical checks:

1. Claim-to-citation check: every factual statement about behavior has a source anchor or is explicitly labeled as inference or obligation.
2. Vocabulary check: `NormalizedKey` and other approved terms are used consistently; banned synonyms are flagged.
3. Actor check: sentences describing actions include an explicit actor.
4. Failure check: every mentioned failure belongs to a declared category; uncategorized failures are flagged.
5. Repetition check: the early source fragment appears only once in full unless an exception is justified and recorded.
6. Link check: the later mutation page links to the earlier definition section and the earlier fragment anchor, and those anchors resolve.
7. Placeholder check: all unknown implementation details remain bracketed placeholders or explicit verification obligations; none are accidentally presented as facts.

Independent review:

1. Domain-blind reader review: a reviewer who knows the language and storage APIs, but not the service domain, reads the pages and identifies any point where the write path cannot be followed from the local reminder plus the link.
2. Code-grounding review: a separate reviewer checks each claim against the cited code and either approves it, downgrades it to inference, or turns it into a verification obligation.
3. Consistency review: a reviewer compares the early identity page and later mutation page to ensure the meaning of `NormalizedKey` is unchanged and that the later page uses the same vocabulary and failure taxonomy.
4. Publication gate: do not publish until all unresolved items are either verified or explicitly retained as open verification obligations.
