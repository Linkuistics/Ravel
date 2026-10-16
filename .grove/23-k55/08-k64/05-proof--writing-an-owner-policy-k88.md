# writing-an-owner-policy-k88

## Goal

Proof the whole chapter `docs/book/src/04-writing-an-owner-policy.md` against
`docs/book/contract.md`, under the editorial family's proof procedure. Check
its teaching, technical truth, prose, examples, tables and direct entries
together; re-enumerate the chapter's configuration coverage from the subject.

## Context

- The node brief carries the two-part draft, guide disposition and chapter 6
  handoff. Chapter 4 is one document, not only the figures changed by art.
- Art added page-native tables for evaluation order, caller/loader document
  combinations, worker versus unconfined managed-process environments, and
  import versus host-read resolution. No new SVGs. Each table has an adjacent
  takeaway. Keep timeout distinct from refusal, required worker cleanup before
  launch, loader replacement without automatic merging, and environment grants
  distinct from managed-process confinement.
- `docs/book/figures.md` records the local light-page inspection and its limits.
  Proof makes its own reachable viewing checks. Neither art nor mechanical
  validation establishes human comprehension, agent execution or terminal
  behaviour. GitHub and publication remain unverified for this unpublished book.
- Art's final figure, example and book checks pass: four SVGs match, seven
  complete examples pass (six in this chapter), no failures or skipped examples,
  four fragments not run, no book warning. All code blocks and example labels
  are unchanged from copy-edit. Re-run the repository gates on proof's output.
- The agent flags retain the draft's dated help provenance. The examples use
  fake programs and establish argument delivery, not agent permissions or
  obedience. The installed typechecking recipe uses a separate development
  checker; no installed execution dependency on it is claimed.
- No earlier-stage defect or correction run was identified by art. No later
  sibling is queued. This is the document's first proof run.

## Done when

- The complete chapter passes the contract's proof, inventory and direct-entry
  checks; fixes stay within the proof procedure, with any earlier-stage
  correction routed as its ordered run.
- `task docs:figures`, `task docs:examples` and `task docs:book:build` pass.
- Actual viewing coverage and remaining limits are recorded without claiming
  human comprehension. If the document node closes, check its done criteria
  and promote the relevant context to the book brief before retirement/commit.
