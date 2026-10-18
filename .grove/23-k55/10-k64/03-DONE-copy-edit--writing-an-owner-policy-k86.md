# writing-an-owner-policy-k86

## Goal

Copy-edit the whole chapter 4, `docs/book/src/04-writing-an-owner-policy.md`,
under `docs/book/contract.md`, after both planned draft sessions. Own sentences,
terminology and consistency with the prose contract; structure and technical
truth remain draft's charter.

## Context

Read the whole chapter, the document-node brief and its handoffs, and the
contract. Part A is `writing-an-owner-policy-k73`; part B is
`writing-an-owner-policy-k85`. Part A's complete examples and both declaration
recipes passed. Its examples intentionally distinguish exact stdout/status
from stable stderr excerpts; the choice example tests all sample arrangements
and refusal codes but not every filesystem condition producing one code.
The installed-prefix recipe uses a separate development compiler and does not
claim that the installed worker needs it. Preserve those qualifications.

Use British spelling, task headings, locally introduced terms and honest
fragment labels. Keep the examples checker's labels and code-block adjacency
intact while editing prose. The dated personal configuration handoff stays
outside the book. Mechanical checks do not establish human comprehension.

## Done when

- The complete chapter has been read and corrected within copy-edit's charter.
  Route earlier-stage defects through the editorial family's bounded correction
  machinery; place later-stage matters in the node's Handed forward list.
- `task docs:examples` and `task docs:book:build` pass after the edits; do not
  silently change command semantics or exact expected output to repair prose.
- Last act: add the `art` leaf with bare stem `writing-an-owner-policy` unless a
  live later sibling already holds it, with the actual figure/text needs in its
  body. Retire and commit this leaf normally.

## Decisions (running log)

- One spelling per term, matching chapters 1–3: "managed program", "managed
  agent" and "the managed group" became the managed process and the managed
  process's group; "the front" became the native front; the environment
  variable the managed process inherits is the session-result context, not the
  result channel (contract term risks).
- First-use links on this page added for caller, session, policy worker,
  native front, managed process, session-result context and confinement, to
  the chapter 2 anchors chapter 3 uses. Forward chapter 5, 6 and 9 mentions
  stay plain text (their pages do not exist; the set proof links them).
- Uncheckable or figurative wording replaced by its mechanism: the default
  export sentence now names the `policy` export the worker checks
  (`crates/ravel/worker/src/main.ts`, "no named export"); "package shadow" and
  "embedded registration" now say the alias never reaches the embedded module;
  the selection-deadline outcome is named `selection_timeout`; helpers that
  "return" a code now return a refusal with that code; "terminal routes" now
  says both routes start interactive agents that need a terminal.
- Slash pairs written out (`invalid or unserialisable`, `context or read`,
  `private and obsolete`, `name and exports`, `main` and `exports`);
  "unserializable" to British spelling; demonstratives given nouns ("This
  check", "Changing to `/`").
- No structural or technical-truth defect found; no correction run cut.
  Every code block and example label is byte-identical to the draft (checked
  by extracting both from `@-` and the working copy). `task docs:examples`:
  7 passed, 0 failed, 0 skipped; `task docs:book:build` passed.
