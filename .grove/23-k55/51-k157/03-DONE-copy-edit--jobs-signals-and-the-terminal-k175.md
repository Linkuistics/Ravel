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

## Decisions (running log)

- Inputs read: the brief chain, `docs/book/contract.md` (emphasis, term risks,
  prose contract), `docs/book/ste.md`, `CONTEXT.md`, both draft logs (k158,
  k174), chapter 8's copy-edit k171 as the precedent, and the whole chapter.
- Before any edit, strict STE and strict prose breaks on chapter 9 passed.
  The defects are those a script cannot find: glossary terms with no
  first-use link, two spellings of one signal, figures of speech, hedges,
  undefined jargon, an American spelling and noun stacks.
- First-use links. Four glossary terms had no link at their first use:
  caller, session, harness and persistent runner. Each now links to the
  section that teaches it (chapter 2, or chapter 1 for the persistent runner,
  as chapter 7 does). The chapter no longer uses “refusal”: see the review.
- Signal names. The chapter wrote one signal two ways (KILL and SIGKILL,
  TERM and SIGTERM, INT and SIGINT). The escalation figure, chapter 3 and
  chapter 8 use the short names. The chapter now states the rule once, before
  the escalation table, and uses INT, TERM, HUP and KILL without the prefix.
  Other signals keep their full names, because the chapter never shortens
  them. `docs/book/ste.md` records the rule for the book. Chapters 5 and 6
  still mix the two forms; this goes to the whole-book proof by the book brief.
- Figures of speech replaced by their mechanism: “a stubborn process”, “hit
  the wrong job”, “the group still answers”, “invents a reap”, “happens to have
  no terminal”, “brings that launcher forward”, “survives exec” and “a
  shielded reader”.
- Hedges with no condition: “normally checks” now names the condition (until
  KILL, from `POLL_INTERVAL` and `KILLED_POLL_INTERVAL` in `run.rs`);
  “generally preserves” now states its one exception; “If necessary” names
  the child that still runs; “Ravel aims to give” now points to the
  exclusions in the table; “usual terminal modes” is “default terminal modes”.
- Undefined words: “payload” is the selected program and “backend monitor” is
  bubblewrap's monitor process. “Handback” is defined at its first use.
  “Noninteractive runner child” was used before its section defines it. The
  sentence now says that the child can itself be a supervisor and that
  `run_noninteractive` uses the same rule, as its source comment states.
- One term per concept: “launcher” named the Ravel CLI in one place and the CLI
  with the runner in others. Each use now names its agent. “stdout” and
  “stderr” are “standard output” and “standard error”, as elsewhere in the
  chapter. “Result success” is the shared phrase “return no result to the
  caller” (`ste.md`, R13). “Initializer” is British “initialiser”.

## Review

The leaf's one fresh reviewer received the chapter diff and the rule that a
copy-edit changes no technical claim, with the distinctions above, and checked
each hunk against the Rust source. It found nine items. Classification:

- Valid, fixed:
  - “the group query still succeeds” excluded `EPERM`, which `confirm_gone`
    treats as present. The text now says success or `EPERM`.
  - “Only KILL ends it” was false for INT and HUP. It is now “Ravel's
    escalation must reach KILL to end it”.
  - Linking “refusal” for an absent backend imported chapter 2's definition,
    which is about the owner policy and its data. The text now uses chapter 5's
    words: the invocation stops with `confinement_unusable` and status 2.
    “Refusal cases” in the chapter 5 pointer is now “the errors that stop a
    launch”.
  - “Ravel gives … the signal state” lost the condition that “aims” carried.
    It now starts “On its supported targets”.
  - The supervisor reason is stated in source only for `run_noninteractive`
    (`run.rs`, its doc comment). The sentence keeps that scope and now points
    forward to the section that defines the interface.
  - “A selected run of these tests” suggested chosen runs. It is “A selection
    of these tests”.
  - The signal-name rule came after the first KILL. It now comes first, and it
    says that other signals keep their full names.
  - “If Ravel stops/reaps” lost the counterfactual. “Stopped/reaped” and
    “could” are back.
- Noise: the EINTR retry in the watch loop has no sleep. This does not change
  the schedule that the reader needs.

No second reviewer is needed: each fix restores the draft's claim or takes
an exact existing book sentence, and the source line for each is above.

## Verification and limits

- After the last chapter edit, all exited 0: `task docs:book:build`,
  `task docs:ste -- --strict` on chapter 9, `SUMMARY.md` and `CONTEXT.md`
  (0 problems), `task docs:breaks -- --strict` on chapter 9 (no long run),
  `task docs:examples` (14 passed, no failures, skips or page errors),
  `task docs:excerpts` (8 passed) and `task docs:figures` (12 match).
- Controls: a scratch copy with a planted semicolon made strict STE exit 1.
  All 19 chapter link occurrences resolve to built files and heading IDs, and
  a planted absent anchor was reported missing.
- No reader comprehension, rendering view, runtime measurement or new source
  claim follows. No structural or teaching defect needed a correction run.
- The node's two copy-edit handoffs are cleared from its brief. The art
  handoffs stay. No live later sibling held art, so this leaf cut art k176.
