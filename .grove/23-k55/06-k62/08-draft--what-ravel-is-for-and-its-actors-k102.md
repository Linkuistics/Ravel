# what-ravel-is-for-and-its-actors-k102

## Goal

Correct chapters 1 and 2's account of what Ravel is for, under the amended
`docs/book/contract.md`. Ravel runs LLM sessions in agent harnesses as
supervised parts of another program, and `end-session` exists because an
interactive harness does not end itself. Read both chapters as one document
and change only the draft charter's structure and technical truth.

## Context

This is the document's **second correction run**. Read 1
(`book-read-one-k70`) cut it from the owner's stand-in read. The first run was
draft k78 → proof k81, sent back by art k77. Under the editorial family a
second run is an escalation; the owner authorised this one on 2026-10-17: “I
would like you to apply those fixes you already know about before I continue
reading.” That decision covers this run only. If any stage in it finds work
an earlier stage must redo, stop and say so to the owner instead of cutting
another run.

The owner's findings, in their words (k70's running log has them with their
evidence class):

- **R1, framing:** “We should be explicit that the programs we are running are
  LLM sessions in harnesses such as codex or claude code. We should also point
  out that these harnesses typically cannot exit themselves, which is why we
  have the `end-session` command. Thus ravel allows you to use such
  interactive harness as 'sub components' of another process, typically an
  orchestrator.”
- **R2, the publication sentence:** “"Publication is the start of completion"
  wasn't clear that publication refers to the result value from the para
  before the diagram.”

The contract now carries R1: "The intended domain" (rewritten, quoting the
owner), chapter 1's row in "The section plan" ("including why `end-session`
exists"), the system-context row in "Figure needs" and the "Harness" term
risk. `CONTEXT.md`'s opening line, **Caller** and **Harness** were amended to
match. Where R1 lands (line numbers at commit `34c901ed`; neither chapter has
changed since `book-first-half-k100` proved it):

- **Chapter 1's opening**, lines 3–14: a generic “tool” asks for work, with a
  code-review example. Nothing says that the program is an LLM session in a
  harness, or that the caller is typically an orchestrator running sessions
  as steps of its own work.
- **"From a request to a result"**, lines 37–39: “An AI coding agent is one
  possible program; … Ravel also works with ordinary programs that use its
  completion command.” This makes the harness one case among many. The
  contract now makes it the case Ravel exists for, and keeps the generic
  mechanism as the reason any program that runs `end-session` can stand in
  (which is why the book's examples use fake agents).
- **The `end-session` paragraph**, lines 41–49, says what the command does but
  not why it exists: an interactive harness does not exit when its LLM
  finishes; it waits for more input. The prompt's ending instructions have the
  LLM publish the result. Ravel then stops whatever is still running in the
  harness's process group, cleans up and returns the result. The rule is in
  `docs/specs/session-execution.md` (“Publication requests the supervisor's
  2-second grace, then TERM, up to 5 seconds kill-grace, then KILL when
  needed”); chapter 3's "Publish the result and finish the job" teaches the
  mechanism and chapter 9 will give the timing, so chapter 1 needs the reason,
  not the schedule.
- **"Decide whether Ravel fits"**, lines 99–144, states the fit in generic
  terms (lines 101–115), and names Grove as one illustrative caller without
  saying what kind of caller it is (lines 131–133). Check the section, and
  the reading-route paragraph about the fake agent (lines 168–171), against
  the new domain statement.
- **Chapter 2**: "Caller and owner" defines the caller (line 12) without the
  orchestrator case. "The managed process and its result" defines the harness
  generically (lines 79–84: “If the selected program is an agent working on
  the prompt…”).

R2 lands at chapter 1, line 62: “Publication is the start of completion, not
proof that completion succeeded.” It is the first sentence after the
system-context figure. Its referent, the result published with `end-session`
(lines 41–49), comes before the paragraph introducing the figure (lines
51–56) and the figure itself. The sentence sits inside the passage R1
changes, so this run fixes both: name what is published where the sentence
stands, or move the explanation of publication and cleanup next to the
`end-session` paragraph it continues.

Constraints the routing session checked:

- **Other chapters.** The rest of the first half agrees with R1. Chapter 3
  introduces its fake agent as “an ordinary shell program” that needs no model
  service or agent account. Chapter 4's "Route to Claude Code and Codex"
  already starts the interactive CLIs and puts the ending instructions in the
  caller's prompt. Do not edit another unit's document; if this correction
  makes a statement in chapters 3–6 false, record it in the book node brief
  (`.grove/23-k55/_ravel-book.md`) for the resumed read.
- **Anchors.** The section plan's headings stand. Chapters 3–6, `SUMMARY.md`,
  `README.md` and `docs/authority.md` link to chapter 2's `#caller-and-owner`,
  `#policy-evaluation-and-supervision`, `#the-managed-process-and-its-result`,
  `#three-different-kinds-of-information` and `#trust-and-confinement`, and to
  chapter 1's `#from-a-request-to-a-result` and
  `#understand-the-evidence-limits`. Keep every one of them resolving.
- **Real agents.** No example runs a real agent, so claim no more than the
  recorded checks support: chapter 4's help check of 2026-10-16 (the normal
  `claude` CLI is interactive and `-p`/`--print` is not; omitting a `codex`
  subcommand selects the interactive CLI) and the owner's statement.
- **Hedges.** The prose contract forbids a hedge without its condition. The
  owner's “typically” becomes a stated condition in the book: an interactive
  harness; a caller such as an orchestrator (Grove is the book's one
  illustrative caller).
- **Figures** are art's. The system-context figure's labels are generic
  (“Person or program: supplies the task and interprets the result”; “The
  selected program: performs the work”), and art k104's body already says so.
  If your change makes anything else in a figure, its key or its alt text
  false, add it to the document brief's `## Handed forward`.

## Done when

- Chapters 1 and 2 state the domain as the amended contract does, give the
  reason for `end-session` where the command is introduced, and define the
  caller and harness consistently with `CONTEXT.md`.
- Chapter 1's sentence about publication names its referent where it stands.
- Every listed anchor still resolves, and `task docs:book:build` and
  `task docs:examples` pass.
- Copy-edit k103, art k104 and proof k105 already stand after this leaf, so
  cut nothing. Retire this leaf.

## Notes

- After this run, `book-readme` aligns the README's opening with the
  corrected chapter 1, and then read 1 resumes (`book-read-one-k70`).
