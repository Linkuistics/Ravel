# a-complete-session-k72

## Goal

Draft chapter 3 as the document node's brief describes, under
`docs/book/contract.md`, into `docs/book/src/03-a-complete-session.md`, linked
from `docs/book/src/SUMMARY.md`, with its complete example passing
`task docs:examples`.

## Context

- Chapters 1–2 are proved before this runs. Read them first: this chapter
  uses their terms and links to them.
- Build the example in a scratch directory before writing prose, run it, and
  take the output from the run. Then write the stages from the code path the
  run took, so the narrative is the observed order.

## Done when

- The chapter exists, builds and meets the node brief's "Done when" for the
  draft stage; its complete example passes the checker.
- Last act: `leaf-add` the `copy-edit` leaf under the family rule.

## Decisions (running log)

- The named structure brief is `docs/book/contract.md`: “The reader is a
  software engineer with no background in Ravel's context”; the ordered plan
  gives chapter 3 “run a real session with a tiny policy and a fake agent, then
  describe its stages”; Emphasis starts with “What success means”, and
  Exclusions puts installing and configuring agents outside the book. Draft
  follows that plan: obtain the checkout build, run the example, explain
  admission/evaluation/judgement, launch, publication and settlement, then name
  the other outcomes and the next reading routes.
- Use a private installation prefix built from this checkout, and a fake
  `fake-agent` with an explicit policy and empty private HOME/TMPDIR. The
  teaching does not use the installed v1.0.0 launch binary or personal policy.
- The scratch run returned exactly eight stdout bytes (`finished`), status 0
  and no temporary residue. Its stderr also contained PID-dependent stop
  reports. A second run, matching the checker's process-group setup and using
  `exec` for publication, reproduced those reports. `supervise_reported` in
  `crates/keyed-launch/src/run.rs` identifies the accepted zombie-only Darwin
  EPERM case and requires confirmed reap/recovery/group absence. Keep the
  simpler fake that calls `end-session` and exits. Use the contract's existing
  `Standard error includes` convention for the two stable lines, with an
  explicit explanation that they are not the entire stream; stdout and status
  remain exact. Do not conceal reports, normalise PIDs or claim exact stderr
  across platforms. This qualifies the node's requested exact transcript by
  the observed stream and the book's existing example convention.
- The draft owns prose and technical sequence. The UML sequence and outcome
  state-machine figures, notation teaching and captions belong to art and
  will be handed forward in the document brief. No figure placeholder will
  break the draft build.
- One fresh-context, aimed reviewer inspected the chapter's technical teaching
  and first-session route against source and contract, without the drafting
  reasoning. This spends the leaf's one reviewer allowance. Its one finding
  was actionable: the prerequisite list omitted Java 11+ and tar's bzip2
  support, both required by `task setup` through `scripts/figures.sh`.
  Direct reading of `setup_figures`, `require_tools` and `find_java` confirms
  it; the chapter now names both and explains why Java is needed. This is a
  mechanical prerequisite-list correction, not a new design requiring another
  reviewer. No other finding was reported. No second reviewer or review leaf
  is commissioned.

## Validation and handoff

- Read the whole draft against the structure brief and checked the subject
  claims against the specifications, ADRs and current source. Tier 2 graph
  search, complete relevant trace pages, exact snippets and direct source
  reads covered admission, worker evaluation/teardown, executable resolution,
  launch/terminal lending, publication, pre-reap group stopping, recovery,
  result reading and final settlement/write. Coverage metadata reported no
  recorded issue and matching source metadata (generation
  `2026-10-16T03:45:45Z`; best effort, not proof of completeness). No product
  source or architectural decision changed; the ADR set needs no revision.
- `task setup` and `task install PREFIX="$PWD/target/book-install"` passed.
  The example checker also passed against that installed release pair using
  `RAVEL_FRONT`, separate from the normal checkout debug pair. The setup
  block remains labelled as a fragment because it omits prerequisite
  installation and variable build output; it was exercised separately here.
- Final sequential `task docs:examples`, `task docs:book:build` and
  `task docs:figures` all exited 0. The example report is 1 passed, 0 failed,
  0 skipped, 1 labelled fragment, 0 page errors. The book built without
  warnings; both existing figures match their sources. All 3,511 subjects
  (the tracked files, built debug front and worker, and every file under
  `target/tools`) had identical before/after SHA-256 records. Local logs and
  the subject manifest are under `target/chapter3-draft/`.
- Chapter 3 is linked in SUMMARY; its links into chapters 1–2 name existing
  generated heading anchors. There are no source excerpts to validate.
  This draft claims the successful fake route and source-supported teaching,
  not exhaustive outcome execution, terminal interaction, new platform
  qualification or reader comprehension. Remote rendering remains untested.
- The document brief carries art's two figure obligations and the observed
  stderr limitation. Copy-edit follows as a live sibling; it reads the whole
  chapter under its own charter. The document node remains open through art
  and proof, so no parent cascade is due. No live review names this leaf or
  its document node; no Creator line is required.
