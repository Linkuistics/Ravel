# ravel-book-k55 — brief

## Goal

Write the Ravel book to its contract, `docs/book/contract.md`: a pedagogical
user guide that lets a software engineer new to Ravel's context use, modify,
maintain and reuse Ravel, drafted against this workstream's current code. The
owner settled the contract in `book-structure-k5` on 2026-10-16.

## Done when

- Chapters 1–9, 11 and 12 exist under `docs/book/src/` in the contract's order,
  each taken through draft → copy-edit → art → proof under the contract.
- The book's Taskfile tasks exist and pass: the mdBook build and local serve,
  the figure regenerate-and-compare check, the complete-example check through
  the CLI/real-policy/fake-program seam, and the source-excerpt check. `task
  setup` fetches the pinned mdBook. Publishing the built book to GitHub Pages is
  part of the release procedure; the first publication happens with the next
  release, outside this node.
- The guides the contract lists are folded in and removed, every inbound link
  redirected, and the README reduced to a short entry into the book. The
  owner's configuration handoff record is kept outside the book.
- Read 1 (after Parts I and II) and read 2 (after Parts III and IV) have been
  held as the owner's labelled stand-in reads, each recorded with its evidence
  class, with their corrections routed to the stages that own them.
- The empty `--policy-env` description in `ravel start-session --help` is
  fixed, or handed on with its reason.
- Chapter 10 is not this node's: the work that delivers a supported extracted
  component writes it, together with that component's interface and consumer.

## Decomposition

Lazy, in two halves separated by read 1, because chapters 7–12 build on the
model chapters 1–3 teach and the contract stops dependent drafting until an
early comprehension failure is corrected. The first child plans the first half:
the scaffold the drafting needs, the drafting units for chapters 1–6, the guide
migration they imply, and read 1 placed after their proof. The session that
holds read 1 cuts the second half's planning leaf under this node once read 1's
corrections are routed. Everything for the book stays inside this node, so the
book runs before `component-work-scope-k57`, which follows the node at the root.

## Pointers

- Contract: `docs/book/contract.md`, with the structure brief, prose contract,
  format and media, and reader checkpoints. A document node brief or draft leaf
  body names it by path; it carries all three items the draft precondition
  cites.
- Term list: `CONTEXT.md`, including its flagged ambiguities.
- Normative sources the book restates: `docs/specs/session-execution.md`,
  `docs/specs/result-job-completion.md` and the three decision records in
  `docs/adr/`.
- Guides to fold in: the contract's "Relation to other documents".
- Example seam: the existing CLI tests' real-policy/fake-program fixtures in
  `crates/ravel/tests/cli.rs`.
- Chapter 8's method: the `linkuistics:writing-code-walkthroughs` skill at
  orientation depth (no complete-source fragment graph).

## Notes

- The pinned upstream design's contract format, figure contract, house style
  and toolchain snapshot are not installed in this checkout's Grove. The
  contract carries everything the book needs from them; stage sessions read
  the contract instead of looking for those files.
- This workstream's code is 48 commits past the v1.0.0 release and is not on
  GitHub, so GitHub's rendering of the book can be checked only after a push.
- Versions pinned in the contract (mdBook 0.5.4, PlantUML 1.2026.8 with
  C4-PlantUML 2.13.0, d2 0.9.0) were current on 2026-10-16; recheck a dated
  fact before a choice turns on it.
