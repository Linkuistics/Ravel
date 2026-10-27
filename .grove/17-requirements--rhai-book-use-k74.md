# rhai-book-use-k74


## Goal

Complete Read 1 of Parts I and II with the owner as the labelled stand-in reader.
Record the owner's task performance and corrections before dependent drafting continues.

## Context

- Section 4 of `docs/book/contract.md` requires this checkpoint after proof.
  The proof is `rhai-book-use-k73`. Its task records coverage, visual evidence and mechanical checks.
- Open the locally built book at chapter 1. Use `mise exec -- task docs:book` after `task setup`.
  Check that the figures load before the read.
- The book describes the agreed 2.0.0 design. The checkout still has the old-format path until `typescript-worker-cutover-k31`.
  Complete Rhai examples already run through the built CLI. The release version remains a placeholder.
- The final notice path belongs to `typescript-delivery-removal-k32`, then the whole-book reconciliation in `rhai-book-inside-k34`.
  Spec reconciliation owns the exact environment-scrub wording. These are recorded dependencies.

## Done when

- The owner uses the book to write a policy for a fake agent and run a session.
- The owner interprets a refusal and a cancellation from their observed outcomes.
- The owner uses a direct reference entry, such as an exit code or error message, without its preceding teaching text.
- The record preserves the owner's words, progress, reported locations and places they did not reach.
  An early comprehension failure stops the read. The stage that owns the correction fixes it before dependent drafting continues.
- Each finding has a disposition and a named owner. Cut correction work before the next dependent draft when necessary.
- The result distinguishes the owner's observations and task performance from mechanical checks and model-proxy results.
  It states that this read does not establish whether a developer new to Ravel can use the book.
- Read 2 remains required after proof of Parts III and IV. This read waives neither checkpoint.

## Notes

This checkpoint needs the owner's live participation. Do not substitute another model for that reader.
Use fake agents and the examples' private environment. No real model call or personal policy conversion is part of this read.

## Decisions (running log)

**Mechanical checks before the read, 2026-10-26.** Evidence class: mechanical, not the owner's result.
The subject is commit `cf28b89f` (`rhai-book-use-k73`) with an empty working copy.
A SHA-256 manifest of the 3,591 tracked files outside `.grove/` was equal before and after the checks (`f79ff291…e959`).
A planted change in a copy of that manifest showed as a difference, so the comparison can fail.

- `task setup` passed with the pinned tools.
- `task docs:examples`: 19 passed, 0 failed, 0 skipped, 25 fragments not run, 0 page errors.
  Chapters 3–5 hold 18 of these examples, and chapter 9 holds one.
- `task docs:figures`: 14 of 14 figures match their sources and layout records.
- `task docs:excerpts`: 9 passed. `task docs:book:build` had no error or warning.
- `task docs:book PORT=8796` serves the book on `http://127.0.0.1:8796`. Another local server already used port 8775.
- In a headless Playwright browser, each of the nine figures of chapters 1, 2, 3, 5 and 6 loaded.
  Each was complete, had a nonzero natural size and showed at the 750-pixel column. Chapter 4 has no figure.
  A control image with no file gave a load error and a 0×0 size. Its 404 was the only console error.
- The console also had 40 warnings. The highlighter of mdBook 0.5.4 has no `rhai` language, so Rhai blocks show without syntax colours.
  Whether this hinders a reader is a question for the owner's read.

**Probe of the checkout build, 2026-10-26.** Evidence class: mechanical.

- In a private home, the checkout build found `policy.rhai` at the default path and ran it.
  With an empty home, it refused with `policy_missing` and status 3.
- A lone `policy.ts` still reached the old TypeScript path and refused with `policy_invalid`.
  Chapter 6, line 616, gives `policy_format_retired` for that case.
  `typescript-worker-cutover-k31` supplies that refusal, so this is a recorded dependency.
- The checkout build prints `ravel 1.1.0`, as Homebrew's installation does.
  Chapter 3 shows the placeholder `@RAVEL_BOOK_VERSION@`, which the release replaces.

**The owner's environment, 2026-10-26.**
The read uses the checkout build, because no Homebrew release has Rhai policies yet.
A script in a scratch workspace outside the repository starts zsh through `env -i`.
The shell has a private, empty `HOME` and `TMPDIR`, and its `PATH` holds the checkout `ravel` and the system directories only.
Thus, it has no `RAVEL_EXIT_FILE`, no Grove variable, no personal policy and no real `claude` or `codex`.

One run of chapter 3's first example through that shell returned `finished` with status 0.
Then this session emptied the workspace again.
This session's own shell holds the `RAVEL_EXIT_FILE` of the Grove session, so the owner runs the tasks in a separate terminal.

**The handover, 2026-10-26.** This session gave the owner the address of the book, the path of the private shell and the three tasks.
It also listed the known differences between the book and this checkout.

**Owner findings F1–F6, chapter 1, 2026-10-27.** Evidence class: the owner's stand-in read.
The owner's quotations match chapter 1 at `cf28b89f`. The chapters did not change after that commit.

The owner did not say which view of the book they used or how far they read. The owner did not report a task.
“F” numbers the findings of this read, apart from the R numbers of the earlier Read 1, `book-read-one-k70`.

- <!-- tc: depart long-sentence because the item quotes the owner's words verbatim --> **F1, Rhai at its first mention.** The owner's words: “As soon as we mention Rhai we need to describe its characteristics, show an example, and link to some documentation - does Rhai have a user manual/tutorial/reference website(s)?”
  Located: chapter 1, lines 51–54, “a configuration file with Rhai code. Rhai is a small language that Ravel embeds.”
  Chapter 2, lines 36–37, and chapter 4, line 6, repeat that description. No chapter links to Rhai's own documentation.
  Owning stage: draft, with a contract change.
- <!-- tc: depart long-sentence because the item quotes the owner's words verbatim --> **F2, a selection and the routing notice.** The owner's words: “"The provider, model and effort fields of a selection are labels for the notice that Ravel writes when the session starts." - 'a selection' is not described/contextualised, and "labels for the notice" is not clear. Also "Ravel writes" isn't clear - we haven't said where it writes, or that such a write is a communication mechanism. This could do with a diagram and concrete example.”
  Located: chapter 1, lines 58–62. Chapter 2, line 55, first says what a selection contains.
  Chapter 3, line 155, first says that the routing notice is the `ravel: starting` line on standard error.
  Chapter 5, lines 587–650, gives its text and JSON forms. Owning stages: draft and art, with a contract change.
- <!-- tc: depart long-sentence,synonym-rotation because the item quotes the owner's words verbatim --> **F3, the kind and the prompt.** The owner's words: “"The caller can also use the kind to choose instructions for the prompt." - this would be better expressed as ".. to extend or modify the prompt".”
  Located: chapter 1, line 64. This is wording, but it is in the passage that F2 and F4 change. Thus, the draft takes it.
- <!-- tc: depart long-sentence because the item quotes the owner's words verbatim --> **F4, a model call in the policy.** The owner's words: “"Before it selects a command, the policy can ask a model a question through a harness run." - would be better expressed by "It may be that you want to invoke an LLM to determine the best combination of harness/model/effort/prompt additions for a given kind and prompt. You can do this by ..."”
  Located: chapter 1, lines 93–98. Chapter 4 teaches the recipes of a harness run.
  The policy builds the arguments, so it can add text to the prompt that it passes. Thus, “prompt additions” agree with chapter 1, lines 67–69.
  Owning stage: draft.
- <!-- tc: depart long-sentence because the item quotes the owner's words verbatim --> **F5, how `end-session` finds its session.** The owner's words: “"end-session finds the enclosing session through its inherited session-result context." - inherited how? what is the mechanism. We should have a diagram to explain how this works.”
  Located: chapter 1, lines 115–118. Chapter 3, lines 236–238, says that Ravel gives the path to the managed process in `RAVEL_EXIT_FILE`.
  Chapter 6, line 587, says that the Ravel CLI sets it for each managed process and each evaluation harness run.
  No chapter shows the variable go from the harness, through the child processes of the harness, to `end-session`.
  Owning stages: draft and art, with a contract change.
- <!-- tc: depart long-sentence because the item quotes the owner's words verbatim --> **F6, what `end-session` does.** The owner's words: “"When end-session sends the result, it requests the end of the session." - better expressed as "When the program that Ravel has started (likely a harness running an LLM) calls ... with a result, Ravel ends that program ... and the result becomes the value returned as the Ravel result."”
  Located: chapter 1, line 120, and the “Send” row on line 139. Owning stage: draft.

**Owner findings F7 and F8, all chapters, 2026-10-27.** Evidence class: the owner's stand-in read.
The owner sent them while this session worked on F1–F6.

- **F7, repetition.** The owner's words: “Good human-centered writing doesn't use DRY at all. Repetition and restatement are important techniques.”
  The prose contract forbade “a closing paragraph that restates the section”. It also put a link, not an explanation, at the first use of a term.
  Owning stages: a contract change, then draft and copy-edit.
- **F8, tone.** The owner's words: “We should also use a more conversational tone where possible”.
  The register of the contract was “plain written prose in STE”.
  This session reads “where possible” as “where the STE rules permit it”. Thus, the book still has no contractions.
  The owner can change that reading. Owning stages: a contract change, then draft and copy-edit.

<!-- tc: depart long-sentence because the paragraph quotes the owner's words verbatim -->
**Owner decision: apply the patterns before the re-review, 2026-10-27.** The owner's words: “I would like you to apply the patterns of these re-statements to the result of Part I and II, and then I will re-review.”
This session reads “the result” as “the rest”, so the patterns apply to chapters 1–6.
The read pauses, and this leaf stays live. The re-review is the resumed read of this leaf.

This session does not edit the chapters. Each stage reads the whole document under its own charter.
The owner's re-review needs proofed text that no later stage rewrites.
The earlier Read 1, `book-read-one-k70`, routed the owner's “apply those fixes” in the same way on 2026-10-17.
This run is the first correction run against `rhai-book-use`, so it is not an escalation.
Another send-back against this document goes to the owner.

**The owner's question about Rhai's documentation, 2026-10-27.** Evidence class: a web check, not the owner's result.
Rhai has a user manual, a language reference and an online playground. Each address below gave status 200 to `curl` on 2026-10-27.

- The Rhai Book, <https://rhai.rs/book/>. It says that it documents Rhai 1.27.0. `Cargo.lock` has `rhai` 1.27.0.
- The stand-alone language reference, <https://rhai.rs/book/ref/>.
- The feature list, <https://rhai.rs/book/about/features.html>.
  It names a syntax like JavaScript and Rust, dynamic typing and a sandbox that changes its host only with permission.
  It also names protection against stack overflow, oversized data and runaway scripts.
- The online playground, <https://rhai.rs/playground/stable/>. It has no `ravel::` host capabilities and none of Ravel's limits.
- The Rust API, <https://docs.rs/rhai/1.27.0/rhai/>. It is for contributors, not for policy authors.

<https://rhai.rs/book/language/> gave status 404, so the book does not link to it. Its pages, such as `variables.html`, exist.

**Contract changes, 2026-10-27.** A commit before this leaf's commit converts section 2 of `docs/book/contract.md` to the STE rules.
That conversion does not change the meaning. This leaf's commit then adds the patterns:

- The chapter 1 row of the section plan now covers Rhai, the routing notice and how `end-session` finds its session.
- A new paragraph after the section plan moves the first presentation of Rhai to chapter 1, with the addresses above.
- “Figure needs” has two new rows: one for the selection and the routing notice, and one for the inherited `RAVEL_EXIT_FILE`.
- Section 2 has ten patterns. The owner's three general statements stand in a quotation, in the owner's words.
- “Register”, “Terms” and “Stock phrasing” now agree with F7 and F8.
- Restatement does not bring back the colour key in the text beside a figure. The owner removed that repetition as finding R8 of 2026-10-17.

Every stage reads the contract, so the patterns also apply to the drafts of Parts III and IV.

**Routing, 2026-10-27.** One run under `rhai-book-use-k26`, cut in pipeline order in one call:
draft `rhai-book-use-k75`, copy-edit `rhai-book-use-k76`, art `rhai-book-use-k77` and proof `rhai-book-use-k78`.
Each body carries its findings, their lines and its checks. Each stage finds its successor already cut, and the proof cuts nothing.
`grove-llm pick` returns the draft, so the run comes before this leaf in the walk.

The node brief and the root brief record the run. After the proof, this leaf resumes with the read from chapter 1 and the contract's tasks.

<!-- tc: depart long-sentence because the paragraph quotes the owner's words verbatim -->
**Owner decisions: the entire book, and lessons for a skill, 2026-10-27.** The owner's words: “I would like you to apply those new patterns to the entire book before I review more. Also remember that we want to remember/record all of our learnings/corrections etc through this process so we can apply them to future docs and books in other groves (as a skill).”
Later, while this session worked, the owner added: “By learnings I mean *all* the things about documentation and communication that we have learnt in this grove.”

The owner's words “those new patterns” accept the ten patterns in section 2 of the contract.
The owner did not change either reading of this session: the conversational tone stays within the STE rules, and R8 stands for figure keys.

Chapters 7–11 still describe the TypeScript worker. `rhai-book-inside-k34` rewrites them after the code work of `typescript-removal-k29`.
This session asked the owner when the next review must happen. The owner chose “Parts I–II first”, so the walk order stays.
The owner re-reviews Parts I and II after proof `rhai-book-use-k78`. Chapters 7–11 get the patterns in their rewrite, before Read 2.

For the entire book, the brief of `rhai-book-inside-k34` and its drafts `rhai-book-inside-k35` and `rhai-book-inside-k36` now require the patterns.
The whole-book proof checks each pattern in all eleven chapters.

For the lessons, `documentation-lessons-k79` (impl) now comes before the correction run.
It records every lesson about documentation and communication in this grove in `docs/documentation-lessons.md`.
Its body lists the sources, the format and this session's readings to check.
The insert moved seven root siblings, so this leaf is now at position 17.
The verb reported one position reference, in the finished task file `llm-harness-replan-k60`.
That record names design-time positions and says “Search again, because the list ages”, so it stays unchanged.

The root brief now tells each session to add its lessons to that file.
Each leaf of the correction run and both drafts of Parts III and IV have that duty in “Done when”.
The skill is under “On the horizon” in the root brief, because the owner has not named its home yet.
A note in the memory of this harness also keeps the practice for later Claude Code sessions in Ravel.
