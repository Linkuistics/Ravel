# jobs-signals-and-the-terminal-k175

## Goal

Copy-edit the whole chapter 9,
`docs/book/src/09-jobs-signals-and-the-terminal.md`, after both draft halves,
under `docs/book/contract.md` and `docs/book/ste.md`.

## Context

The document node brief names the contract and contains the running handoffs.
Read both draft records, starting with `jobs-signals-and-the-terminal-k158`.
Part A teaches job groups, escalation, pre-reap group stops, cancellation,
noninteractive isolation and filesystem confinement. Part B adds terminal
recovery, entry signals and guarantees with limits. Read them as one document.

Preserve the source distinctions: stop attempts versus confirmation, group
signals before reap versus a query after reap, ESRCH-only absence, conditional
EPERM reports, and bounded retries versus a wall-clock guarantee. An ordinary
CLI launch without a terminal is not `run_noninteractive`. Keep the exact
example, API identifiers and the first-use links.

## Done when

- The whole chapter follows the STE writing rules, British spelling and shared
  word decisions. No change silently alters a technical claim or exceeds the
  copy-edit charter. Route a technical or structural defect by the editorial
  family procedure.
- The book build, strict STE, strict prose-break check and complete examples
  pass. Clear the copy-edit handoff in the node brief when addressed.
- Last act: add an `art` leaf with the bare stem
  `jobs-signals-and-the-terminal`, unless a live later sibling already holds
  that stage. Give it the final escalation and terminal figure needs from
  both drafts. The scaffold escalation figure exists but part A did not add
  it to the page. Reuse chapter 5's cancellation sequences as appropriate.
