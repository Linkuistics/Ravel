# a-complete-session-k63 — brief

## Goal

Chapter 3 (`docs/book/src/03-a-complete-session.md`), taken through draft →
copy-edit → art → proof under `docs/book/contract.md`. After it the reader can
run a real session with a tiny policy and a fake agent, then describe its
stages; the process and terminal ideas are introduced as the session needs
them.

## Context

- Needs chapters 1–2 (terms, the system-context figure, the authority
  statement) and the scaffold: the examples check for its complete example,
  the figures check for its figures.
- This chapter carries the book's first complete example, run through
  `task docs:examples`: a tiny policy, a fake agent that calls
  `ravel end-session`, the command, and the exact stdout, stderr and exit
  status. `crates/ravel/tests/cli.rs` (`routed_result_is_exact_*`) shows the
  seam.
- The stages to teach, each verified against the code: admission of the
  request and policy path, evaluation in the policy worker, judgement of the
  selection, the worker reaped, launch as a job, `end-session` publishing,
  the escalation after publication, reaping and cleanup, the result on stdout.
  Sources: `docs/specs/session-execution.md` ("Policy authority and runtime
  discovery", "Managed process and result"), the `ravel` run composition under
  `crates/ravel/src`, `docs/adr/the-launched-child-is-a-job.md`.
- Introduce exactly what the session needs: a process group, the managed
  process as its own group, the terminal's foreground group and its lending,
  reaping. Defer escalation detail, signal masks and failure guarantees to
  chapter 9, and say so in text.
- The reader installs `ravel` with Homebrew and checks its version against
  the line that the release writes. The book has a placeholder until then,
  under the owner's R17 decision below. The examples check uses the build of
  this checkout. Do not teach agent installation; the example uses a fake.
- Figures from the contract: a UML sequence for the successful session (it
  returns in full in chapter 8) and a UML state machine for the outcomes (it
  returns in chapter 5).

## Done when

- Chapter 3 exists, is linked from `SUMMARY.md`, and has passed draft,
  copy-edit, art and proof.
- Its complete example runs under the checker and matches exactly; the
  reader's first session works from the chapter alone.
- Refusal, cancellation and natural exit without a result are named as normal,
  distinct outcomes and pointed at chapter 5, not taught here.
- The chapter says, in the form a reader can quote, what success means: the
  result reaches the caller only after the managed process is reaped, its
  whole group is gone and cleanup succeeded.

## Notes

- "Session" here is Ravel's, never a POSIX session; say so where the two could
  be confused, as the contract's term risks direct.

## Draft evidence carried to later stages

- The example convention is `docs/book/examples.md`. Stdout and exit status
  are exact; stderr uses the supported `Standard error includes` label.
  Scratch runs on macOS produced additional PID-dependent EPERM stop reports.
  `supervise_reported` explicitly retains such reports after confirmed
  reap/recovery/group absence. The chapter explains this limitation instead
  of hiding reports or inventing an exact cross-platform stderr transcript.
- The install fragment uses Homebrew under the owner's R17 decision. The
  Ravel CLI and worker keep their paired layout. The version placeholder
  identifies this book as unreleased until the release writes its version.
- Forward chapter mentions are plain text while those pages do not exist;
  the parent brief assigns their link conversion to the first-half set proof
  and second-half work. The example check supplies no terminal interaction,
  human comprehension result, GitHub rendering or published-site inspection.

## Read 1 correction run (2026-10-17)

The owner's read 1 (`book-read-one-k70`) made ASD-STE100 the prose base for
all documentation, chose “send” to replace “publish” for the result, and
changed six chapter titles to commands. The owner did not read this
chapter yet, so these changes come from the contract, not from findings.
Copy-edit k126 converts chapter 3 to STE, with the word decisions in
`docs/book/ste.md` (from k123). It also renames the heading “Publish the
result and finish the job” and changes every link to it.
Set art `book-first-half-k130` and set proof `book-first-half-k132` serve
this document. Thus, no stage under this node cuts an art or proof leaf.
Copy-edit k126 completed this conversion on 2026-10-18. Chapter 3 passes the
strict STE check, and chapter 5's link uses the new anchor
`#send-the-result-and-finish-the-job`. The figure labels that still say
“publish” are in the parent brief for set art k130.

## Read 1 correction run (2026-10-18)

The owner's full read 1 (`book-read-one-k70`) found R10–R17. Two reach this
document. The reader installs Ravel with Homebrew and checks its version
against a placeholder that the release writes (R17, the owner's choice “Set
at release time”). Runs of prose must not be longer than the new limit
(R11). Draft k136 and copy-edit k137 run under this node. They are the first
draft re-run and the second copy-edit re-run. The owner authorised both on
2026-10-18. Set art `book-first-half-k144` and set proof
`book-first-half-k147` serve this document. Thus, no stage under this node
cuts an art or proof leaf. The book brief carries the installation, version
and figure handoffs from draft k136.

## Handed forward

None. Copy-edit k137 kept the version line intact and closed this node on
2026-10-18. Set art k144 and set proof k147 closed its first-half handoffs.
The book brief retains chapter 12’s release-version teaching with its owner.
