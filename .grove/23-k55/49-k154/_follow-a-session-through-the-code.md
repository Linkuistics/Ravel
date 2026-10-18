# follow-a-session-through-the-code-k154 — brief

## Goal

Chapter 8 (`docs/book/src/08-follow-a-session-through-the-code.md`), taken
through draft → copy-edit → art → proof under `docs/book/contract.md`. After it
the reader can follow one successful session from `main` to the result write,
then the refusal and cancellation forks and the error and signal precedence,
and can find where a change belongs.

## Decomposition

One document drafted in two sessions, then the three remaining stages once.
The seam is the point where the reader has followed the whole successful path
and holds a map of the code. Part B starts from that map and cuts nothing out
of part A.

- **Part A: the successful session.** From `crates/ravel/src/main.rs` through
  argument parsing, the check of the request and the policy path, the start and
  the reaping of the worker, the check of the selection, the executable lookup,
  the allocation of the result channel, the call into the runner, the group
  cleanup and reap inside the runner, the read of the result through the held
  directory, and the write to standard output. The chapter shows the order as
  a full UML sequence (chapter 3 gave the short one), and it shows the
  `end-session` side of the channel: `crates/ravel/src/completion.rs`
  (`end_session`, `publish_result`).
- **Part B: the forks, the precedence and where a change belongs.** The refusal
  fork (policy refusal, a refusal of the policy's file, behaviour or data, a
  selection timeout, a program that cannot run), the cancellation forks (during
  selection and during supervision, and after the result was sent), the
  precedence of errors and signals as the activity figure shows it
  (`run`, `settle`, `join_cancellation`, `supervision_failure` in
  `crates/ravel/src/run.rs`, and `crates/ravel/src/cancellation.rs`), and an
  independent-change scenario that tells the reader where a given change
  belongs (`docs/components.md`, “An independent-change scenario”; verify it).

## Context

- Needs chapters 1–3, 5 and 7, and the scaffold: the figure check, the example
  check and the source-excerpt check (`book-source-excerpts-k150`), and the
  activity module with its first figure (`book-figure-activity-k153`).
- **Method.** Use the `linkuistics:writing-code-walkthroughs` skill at
  orientation depth: concept-ordered, with source-exact excerpts, and with no
  complete-source fragment graph. The chapter says plainly that it follows one
  path and does not cover the complete source, and names what it leaves out.
  Load the skill before the draft starts.
- **Excerpts.** Every excerpt is labelled under the convention that
  `book-source-excerpts-k150` wrote (`docs/book/excerpts.md`), and
  `task docs:excerpts` passes. Choose each excerpt because it makes a decision
  of the session visible. Prefer short excerpts, each with a sentence before
  that says what to look for and a sentence after that says what it means. Do
  not quote a function in full when ten lines carry the point. A reader who
  does not read Rust fluently must still follow the chapter, but the contract
  assumes the reader reads Rust and TypeScript. Never teach the language.
- **The path.** Trace the successful path in the code, with the graph tools
  and direct reads, before you write. Take the order from the code and from a
  real run of the example in chapter 3, not from the guides. The guide
  `docs/components.md` (“Composition and coordination”, “A successful
  operation”, “Cancellation and partial failure”, “Error and signal
  precedence”) and `docs/session-channel.md` are sources that may be stale and
  that use words the book does not use (“front”, “admission”, “publication”,
  “settle”). Write in the book's words. Explain `settle` as the step that
  decides the end of supervision and name the steps it includes, as
  `docs/book/ste.md` directs.
- **Inventories.** Inventory 4 (component obligations) is owned here for O3
  (policy decisions are judged without fallback), O4 (the selected program runs
  directly in the caller's context), O5 (Ravel keeps the first complete
  result, exactly), O7 (cancellation suppresses results) and O9 (only result
  bytes reach standard output). O6 and O8 belong to chapter 9. Enumerate the
  ten obligations again and record the section that teaches each one. Inventory
  3 (lifecycle and failure outcomes) is chapters 3 and 5's; this chapter shows
  where in the code each outcome is chosen, and links back.
- **Figures** (contract figure list): the full successful sequence (here), the
  outcome-precedence activity (here, from `book-figure-activity-k153`), and
  the two cancellation sequences that chapter 5 already has
  (`selection-cancellation`, `supervision-cancellation`). Reuse those two
  unless the code path requires a different one. Chapter 9 reuses them too.
- **The read 2 task.** “Follow one session through the code to where its final
  cancellation outcome is chosen.” The chapter must make that task possible:
  after it, the reader can name the function and the rule that choose the
  final outcome when a signal arrives, and say why the choice is there.
- The non-`--json` form of an operation report (`ravel: <report>` on standard
  error, `crates/ravel/src/run.rs`) is in no chapter. If this chapter meets it
  on the path, teach it here; the second half's set proof decides whether
  chapter 6 gains an entry.

## Done when

- Chapter 8 exists, is linked from `SUMMARY.md`, and has passed all four
  stages.
- Every excerpt passes `task docs:excerpts`; every complete example passes
  `task docs:examples`; every claim about the order of steps was checked against
  the code and, for the main path, against a real run.
- The chapter states what it does not cover. It does not claim to cover the
  complete source.
- The chapter makes the read 2 task possible, and it tells a reader who wants
  to change something where the change belongs.
- Each statement of the guides that this chapter replaces is listed in the log
  with its destination, so that the migration leaf can verify it.

## Notes

- Part A cuts part B as its last act, ahead of copy-edit: cut a `draft` leaf
  with the bare stem `follow-a-session-through-the-code` and part B's scope,
  then cut copy-edit under the family rule (it lands after part B). Part B's
  session cuts nothing if copy-edit already stands.
- Part A settles the heading structure for the whole chapter now and writes
  down in its log which headings part B owns. Do not emit empty headings.
- Chapter 10 is not written. A change that affects an extracted component is
  the work that delivers that component. Do not describe a plan.
- The chapter describes the code at this checkout. A change by later component
  work updates the chapter as part of that change (contract, section plan).
