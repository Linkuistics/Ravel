# what-ravel-is-for-and-its-actors-k71

## Goal

Draft chapters 1 and 2 of the book as the document node's brief describes,
under `docs/book/contract.md` (the structure brief is its section 1; the prose
contract its section 2). Write `docs/book/src/01-what-ravel-is-for.md` and
`docs/book/src/02-actors-and-words.md`, link them from
`docs/book/src/SUMMARY.md`, and leave the book building.

## Context

- The scaffold (`book-toolchain-k58`, `book-figures-k59`, `book-examples-k60`)
  has landed, so `task docs:book` builds. Verify every claim about Ravel
  against the spec and source, not against `docs/session-cli.md` or
  `docs/authority.md`, which this book replaces.
- Report the contract's omissions that bear on these chapters (the reader
  checkpoints are the owner's stand-in reads; chapter 10 is unwritten; the
  support statement) in the form the chapters carry them.

## Done when

- Both chapters exist and meet the node brief's "Done when" for the draft
  stage: the reader outcomes are delivered, every term is defined where first
  needed, no placeholder text remains, and the book builds with
  `task docs:book`.
- The log lists any claim whose guide wording disagreed with the code, so the
  guide fold-in can reconcile it.
- Last act: `leaf-add` the `copy-edit` leaf under the family rule, naming in its
  body anything the draft left for wording that is really a teaching gap.

## Decisions (running log)

- The named structure brief is `docs/book/contract.md`, section 1: “The reader
  is a software engineer with no background in Ravel's context”; the ordered
  section plan gives chapters 1 and 2 the fit and authority outcomes; Emphasis
  requires success after cleanup and authority boundaries, and Exclusions keeps
  history, evidence campaigns and undelivered components out of the teaching.
  These discharge the draft precondition. The agreed plan supplies the design;
  this session does not reopen it.
- Chapter 1 will move from the problem through the two commands and the fit
  test to reading routes and evidence limits. Chapter 2 will follow the request
  through caller/owner, worker/front and managed process/result, then explain
  the three meanings often hidden by “context” and the trust limits. Canonical
  term definitions live at linked anchors; early plain-language introductions
  keep chapter 1 readable without following them.
- Source verification uses Tier 2 on graph project
  `Users-antony-Development-Ravel.simplify-and-componentize`, generation
  `2026-10-16T02:56:54Z`. The bounded searches and one-hop run trace were
  untruncated; exact snippets and direct reads checked the material claims.
  Coverage reported no recorded issue and matching metadata for every checked
  source/spec/guide path. This is source evidence, not a fresh runtime campaign
  or proof of graph completeness.
- The book will say “no persistent session store”, not “stores nothing”: the
  result channel uses temporary files. The policy receives the caller's prompt
  unchanged, but the selected program receives the arguments the policy builds;
  those are distinct boundaries. No new architectural decision is made.
- Added Owner, Native front, Runner, Result channel and Confinement to
  `CONTEXT.md`; the existing Launcher CLI and Persistent runner terms are
  taught at the fit test, with the latter explicitly undelivered. The opening
  chapters teach the other session terms at chapter 2's stable section anchors.
- Guide fold-in reconciliation: `docs/session-cli.md` says the generic
  policy/SDK handles routing “without database access”. The code's separated
  worker (`worker::evaluate`, with ordinary owner TypeScript) and the session
  specification permit direct policy filesystem/network access. Chapter 1
  preserves the intended absence of a Ravel-owned persistent store without
  implying a ban on database access by owner code. The same guide's “Ravel
  passes the prompt unchanged” is made precise as caller-to-policy delivery;
  `run::run` executes the arguments returned by the policy. No other
  contradiction was found in the guide claims used for this bounded unit;
  this is not a full guide-migration audit.
- The complete-example and figure contracts belong to later material: these
  two prose chapters contain no runnable blocks, fragments or source excerpts.
  Required figures are handed to art in the node brief, without placeholders.
  Unwritten forward targets are plain chapter numbers/titles until the set
  proof can link them. Reader-checkpoint, chapter-10, target qualification,
  GitHub-rendering and publication unknowns are stated in chapter 1.

## Draft validation

- Structure read: chapter 1 supplies the problem, request/result boundary, fit
  comparisons, entry routes and relevant unknowns; chapter 2 supplies the
  participants, their authority, result meaning, the three distinct inputs and
  trust/confinement limits. No code or diagram knowledge beyond the contract is
  assumed, and no figure or prose placeholder remains. No teaching gap is
  knowingly left for copy-edit. Art and proof obligations remain in the node
  brief; the parent carries the cross-unit link and guide reconciliation work.
- Technical read: `run::run`/`settle` and `SessionChannel::result_after` check
  publication, cancellation, cleanup and exact output; `end_session` and
  `publish_result` check inherited addressing, first publication and empty
  values. `choice::evaluate_and_resolve`, `worker::evaluate` and environment
  grant filtering check selected arguments, worker reap before admission and
  the missing worker result authority. The two specifications and three ADRs
  supply the normative scope, accepted limits, architecture reasons and dated
  support statement. This draft makes no new runtime-conformance measurement.
- `task docs:book:build` exited 0 with no mdBook warnings or errors.
  `task docs:book PORT=18775` built and served on loopback; an HTTP fetch of
  chapter 2 was byte-identical to the generated HTML. The owned server was then
  deliberately interrupted (task reports 130/201); that is server shutdown,
  not a failed build. Generated chapter links and their heading IDs were read
  together and matched. Browser visual proof remains the later stage's work.
- `task docs:examples` exited 0 after building the matched front and worker:
  0 passed, 0 failed, 0 skipped, 0 fragments, 0 page errors. There are no code
  blocks in this unit, so this is a successful page scan, not example execution.
- The final document/source coverage check reported matching metadata and no
  recorded issues (generation `2026-10-16T03:02:13Z`). No in-session reviewer
  was used. No inventory-wide coverage or reader comprehension is claimed:
  the owner's stand-in reads are still ahead, chapter 10 is unwritten, native
  x86-64 Linux remains unqualified, and remote rendering/publication unchecked.
- The ADR set is unchanged: the chapters explain its current decisions. The
  document node remains live for copy-edit, art and proof; no parent closes.
