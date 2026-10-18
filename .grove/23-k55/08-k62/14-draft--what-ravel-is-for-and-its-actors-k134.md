# what-ravel-is-for-and-its-actors-k134

## Goal

Draft chapters 1 and 2 again for the owner's read-1 findings R14, R15, R16
and R11. This is the fourth correction run on this document. The owner
authorised it on 2026-10-18 in `book-read-one-k70`. Another send-back on this
document goes to the owner.

## Context

- The owner's words and the locations at `1ef02398` are in
  `book-read-one-k70`'s log, “Owner findings R10–R17”. The contract records
  the decisions in place: “The intended domain”, “Emphasis”, the chapter 1
  row of the section plan and “Breaks in prose”.
- **R14, the reason for a harness.** Chapter 1, lines 1–30, introduce the
  harness without the reason. Where the chapter first describes Ravel, say
  that a harness can run under the subscription of its user. Thus, a caller
  can use that subscription instead of paid API access, which costs much
  more. Do not name a price or a plan. Point to the documentation of each
  harness for its plans and terms. The contract's “Exclusions” keep accounts
  and credentials out of the book.
- **R15, the loose contract.** Chapter 1, lines 60–63, say this in two
  sentences. Make it clear and easy to find. The policy selects a program,
  Ravel runs it, and a program in the session sends one value with
  `end-session`. Ravel needs nothing more from the program. `start-session`
  takes a prompt (`--prompt` or `--prompt-file`), and its help calls it “The
  harness prompt”. But the policy decides which program runs and whether that
  program receives the prompt. A table of what Ravel needs and what it only
  expects can carry this, and it also breaks a long run. Check the section of
  chapter 2 on the managed process against the same statement.
- **R16, the race.** Chapter 2, line 128: “A later `end-session` does not
  replace it.” Say that this is a race. The first `end-session` normally ends
  the harness. Ravel waits 2 seconds, sends TERM, waits up to 5 seconds, then
  sends KILL and reaps the group (`ravel start-session --help` and
  `docs/specs/session-execution.md`). A second `end-session` can arrive only
  in that short time, for example from another process in the group. Ravel
  keeps the first result. Chapters 5 and 6 have the same sentence, and their
  drafts in this run use your wording. Write that wording in this node's
  brief for them.
- **R11, breaks in prose.** Chapter 1 has runs of 900 words (lines 1–107)
  and 723 words (lines 183–266). Chapter 2 has a run of 1,329 words (lines
  1–155). Measure with the check from `book-prose-breaks-check-k133`. Add a
  table where items compare on the same attributes. Where a figure answers a
  reader question better, write the figure need under `## Handed forward` in
  this node's brief, with its reader question, for set art
  `book-first-half-k144`. Do not add decoration.
- **R12 and R13** are wording for the copy-edit (`docs/book/ste.md`, “The
  result”). Apply them only in a passage that you write again.
- Keep the earlier corrections R1–R9, the STE writing rules and “send”.
  Other pages link to anchors in these chapters. Before you change a heading,
  find its inbound links with a search that you see find a known link. Then
  change each one.
- Set art `book-first-half-k144` and set proof `book-first-half-k147` serve
  all six chapters. Thus, this node gets no art or proof leaf.

## Done when

- Chapters 1 and 2 state R14, R15 and R16 in the book's own words. Each
  technical statement agrees with the source and the specifications.
- The prose check finds no run that is longer than the limit in chapters 1
  and 2.
- This node's brief gives the race wording and each figure need, with its
  reader question.
- Every link to a changed anchor works. A search with a control shows it.
- `task docs:book:build` and `task docs:examples` pass.

## Notes

- Do not state facts about real agents or subscription terms beyond the
  documentation of each harness. Record the date and the source of each such
  fact that you use.
- Change `CONTEXT.md` only if R15 changes a definition, for example
  **Harness** or **Managed process**.
