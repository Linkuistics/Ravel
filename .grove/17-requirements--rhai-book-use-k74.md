# rhai-book-use-k74


## Goal

Complete Read 1 of Parts I and II with the owner as the labelled stand-in reader.
Record the owner's task performance and corrections before dependent drafting continues.

## Context

- Section 4 of `docs/book/contract.md` requires this checkpoint after proof.
  The initial proof is `rhai-book-use-k73`, and the first correction proof is `rhai-book-use-k78`.
  The next resumption follows correction proof `rhai-book-use-k87`. Read its evidence and correction recap first.
- The owner completed Part I on 2026-10-28 and asked for all lessons before Part II.
  Resume at chapter 4 after the correction run. Make the amended Part I available without requiring a restart at chapter 1.
- Refresh the local book with `mise exec -- task docs:book` after `task setup`. Check that its figures load.
  Prepare and check the private reader shell again before the owner runs the outstanding tasks.
- The book describes the agreed 2.0.0 design. Use the latest proof's dependency record for any difference from the checkout.
  Complete Rhai examples run through the built CLI. The release still owns the version placeholder.
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

**Resume after the correction proof, 2026-10-28.** Evidence class: session preparation, not the owner's read.
The subject is commit `3c80f840`, which retired proof `rhai-book-use-k78` and closed node `rhai-book-use-k26`.
The working copy was empty at entry. The reader tasks and the two checkpoints remain agreed, so this session opens no requirements interview.

`mise exec -- task setup` and `mise exec -- task build` passed.
`mise exec -- task docs:book PORT=8796` built the book without a warning and serves it at `http://127.0.0.1:8796`.

All twelve distinct figure paths from the rendered chapters 1–6 returned HTTP 200. An absent-image control returned 404.
These responses establish file delivery, not browser rendering.

The session opened chapter 1 in Dia and saw its routing, inherited-path and system-context figures.
The screenshot of the inherited-path figure covered only its upper portion. This was a load check, not a new whole-book visual proof.
The session left the browser at the start of chapter 1 for the owner.

The fresh reader workspace is `/private/tmp/ravel-read1-k74-45lo88uC`.
Its `enter` script starts zsh through `env -i`, with private `HOME` and `TMPDIR` directories and an empty `work` directory.
Its private `bin/ravel` links to this checkout's `target/debug/ravel`.

The remaining PATH entries are system directories. The shell found neither `claude` nor `codex`, and had neither `RAVEL_EXIT_FILE` nor `RAVEL_POLICY_RUN`.
The version is still `ravel 1.1.0`, as expected before release. No reader exercise ran in this preparation.

The graph CLI again refused to start because another, incompatible generation is active.
This session used document reads and makes no graph-coverage claim.
The owner now has the chapter 1 address and a request for their first observation, with the passage and heading.
No new owner observation or task result is recorded yet. This leaf stays live, and Read 2 remains required.

**Owner progress and findings F9–F14, 2026-10-28.** Evidence class: the owner's stand-in read.

The owner said, “I have read all of Chapter 1.” This establishes completion of that chapter, not of the practical tasks.
The owner reported no progress in chapters 2–6 and no fake-agent, refusal, cancellation or direct-reference task result.
The session prepared Dia, but the owner did not state which viewing surface they used.
Locations below name chapter 1 at commit `3c80f840`.

**F9, the caller's complete task contract and the order of explanation.** The owner's proposed replacement for lines 27–30 follows.
The owner also invited a better explanation and placement.

<!-- tc: off because this is the owner's proposed wording and explanation -->

> The orchestrator knows the work to be done (the files to review) and creates the prompt that instructs the harness/LLM what to do, and also instructs it to call `ravel end-session <result>` when the work has been done. The orchestrator knows what to do with the result it has instructed the harness/LLM to return. It will most likely give a number of options for the return result.
>
> However you might have a better way and place to explain this. It is always more useful I have found to use a concrete example/explanation and then abstract/broaden/generalise that, as opposed to introducing an abstract model and then concretising the concept.

<!-- tc: on -->

Disposition: accept the complete caller responsibility and the concrete-first pattern.
The draft owns the opening example and the later generalisation. The contract owns the whole-book rule, and copy-edit checks its application.
An illustrative review prompt can name files and give `approved` and `changes-needed` as result options.
Those words belong to that caller, and Ravel does not interpret them. The example must also say what the caller does after each result.

**F10, configuration terminology.** The owner's words follow.

<!-- tc: off because this is the owner's terminology preference -->

> I think the term "owner policy" is inferior to the term "configuration", which is more common and specific.

<!-- tc: on -->

Disposition: accept the replacement in reader-facing prose. Draft owns the glossary, chapter terms and their dependent explanations.
The session proposes “configuration” for the overall setup, “configuration script” for the Rhai file, and “settings” for `settings.json`.
This distinction is the session's interpretation, not a separate owner quotation.
The correction must reconcile dependent terms, including “policy worker” and “policy context”, without hiding the distinction between the script and settings.
The finding does not request changes to filenames, CLI options, JSON keys, error codes or Rust identifiers.

**F11, JSON version labels.** The owner's words follow.

> I don't think we need version-1/version-2 labels when we mention JSON.

Disposition: remove incidental version labels from explanations, including chapter 1, line 241.
Draft and copy-edit own that correction. Keep exact `schemaVersion` fields in executable examples and explain required values at the reference entry.
This preserves the data contract while the narrative says “JSON document” or names its purpose, such as “routing notice”.

**F12, why and how Ravel creates the worker.** The owner's words follow.

<!-- tc: off because this is the owner's requested explanation -->

> "The policy worker is the separate process that runs the owner policy" - we should explain that ravel forks itself to evaluate the policy for concurrency/safety reasons.

<!-- tc: on -->

Located: lines 382–387, after the nested-session example.
Disposition: draft explains the process creation and its purpose at the first relevant mention. Art checks that the figure agrees with that explanation.

Ravel creates a child process that runs its own executable in the worker role.
The parent remains able to enforce the deadline and serve host requests.
The process boundary keeps a script abort or exhausted worker heap from aborting the supervisor.
The worker finishes before Ravel starts the final managed process.
This is not concurrent evaluation and execution of that final program.

Source check: `rhai_worker/front.rs`, `executable` and `launch`, identify the same executable and the worker role.
`worker.rs`, `supervise`, constructs `Command`, sets the pre-exec controls and calls `spawn`.
The ADR `policy-evaluation-precedes-the-launch` records the process boundary, hard deadline and failure-isolation reasons.
These are direct source reads after graph access failed, not graph evidence.

**F13, the Grove link.** The owner's words follow.

> When we mention grove, we shoud make it a link to the repo.

Located: lines 516–520. Disposition: draft links Grove to `https://github.com/Linkuistics/grove` where the book introduces it on each page.
The repository page opened successfully on 2026-10-28. This check establishes the destination, not the current accuracy of its README.
Proof owns the link checks.

**F14, the final section heading.** The owner's words follow.

> "Understand the evidence limits" is a bit opaque - we should use a more descriptive title to that.

Located: line 571. Disposition: draft replaces the heading and reconciles every inbound anchor.
The session proposes “See what we tested and what remains unchecked”.
The section still distinguishes platform measurements, the owner's stand-in read, local rendering and unchecked publication surfaces.

**Correction timing.** These findings would require a second correction run against `rhai-book-use-k26`.
The editorial procedure requires escalation for that run. This session records the findings and prepares the contract changes before it cuts more editorial leaves.

The owner completed chapter 1 and reported no inability to continue. The choice is a correction now or more reader findings before one combined correction.
No chapter changed during this record, so later observations can still name the same text.

**Owner decision: collect the remaining notes first, 2026-10-28.**
The owner chose “Collect the remaining notes first (Recommended)” over “Apply corrections before chapter 2”.
The read continues through the rest of Parts I and II against the existing chapter text.
Then one combined correction run applies the collected findings before dependent drafting continues.
This choice settles the timing of the second correction run. No additional timing confirmation is needed unless the scope or the owner's preference changes.

Contract §2 records the new whole-book requirements, and the lessons record preserves their sources and interpretations.
The terminology qualifiers and the replacement heading remain proposals for that correction.
The prose checker passed the changed records with zero hard findings. No chapter or product source changed.

**Owner reminder: generalise the feedback for a skill, 2026-10-28.**

The owner's words: “Remember that we want to generalise this feedback to construct a skill”.
The lessons record now states the extraction method: reader problem, trigger, corrective action, scope and a way to check the result.
The future skill will combine related rules into a reusable procedure, with product-specific corrections and their evidence in supporting examples.
Its scope remains all documentation and communication lessons from this grove, including later reader findings.
The root brief carries that obligation. The read and the agreed combined correction sequence continue.

**Owner progress and findings F15–F16, 2026-10-28.** Evidence class: the owner's stand-in read.

The owner said, “Part I is full reviewed.” This records their review of chapters 1–3.
The owner did not separately report practical task performance, Part II progress or their viewing surface.
The pending tasks remain the fake-agent session, interpretation of refusal and cancellation, and use of a direct reference entry.

**F15, the chapter 2 title.** The owner's words follow.

<!-- tc: off because this is the owner's assessment of the chapter title -->

> "The actors and their words" is an unusual phrase. I think we need something more descriptive, less shorthand. "their words" in particular is a jarring construct.

<!-- tc: on -->

Located: chapter 2's title and its entry in `SUMMARY.md` at commit `3c80f840`.
Disposition: draft replaces the title with one that states the chapter's subject and reconciles the contents, chapter references and links.
The session recommends “Who does what in a Ravel session”. Copy-edit checks that the title describes the whole chapter.
The general lesson extends L61: headings name the subject or reader question, without unusual shorthand that requires explanation.

**F16, the directory instruction.** The owner's words follow.

> "... then make it the current directory." - "... and `cd` into it." is more appropriate.

Located: chapter 3, lines 74–75, under “Run the first session”, at commit `3c80f840`.
Disposition: draft uses the familiar `cd` command in the instruction. Copy-edit applies the same principle where a procedure uses the reader's stated prerequisites.
The general lesson is L62: name a familiar command directly when it states the action more clearly than an abstract paraphrase.
This does not make unexplained commands appropriate for a reader who does not know them.

**Owner decision: apply all lessons before Part II, 2026-10-28.**

The owner's words: “I would like you to apply all the lessons before I proceed to Part II.”
This replaces the earlier timing choice to collect the remaining Parts I–II notes first.
The second correction run is now authorised before the owner's Part II read. No further timing confirmation is needed.
It reads chapters 1–6 against the full accumulated lessons and the current contract, including F1–F16 and their general patterns.

The existing document node owns draft, copy-edit, art and proof in that order.
Read 1 stays live. After the proof, the owner resumes at chapter 4, with the amended Part I available for inspection.
The next requirements session gives a correction recap and prepares the private reader environment again.
It does not restart the owner at chapter 1 or treat Part I review as proof of practical task performance.

The contract and lessons record carry the new rules for the whole book.
Parts III and IV retain their scheduled rewrite and apply all current lessons before Read 2.

**Correction handover, 2026-10-28.**

One `leaf-add` call cut the ordered run under `rhai-book-use-k26`: draft `rhai-book-use-k84`, copy-edit `rhai-book-use-k85`, art `rhai-book-use-k86` and proof `rhai-book-use-k87`.
Each body gives its charter, the whole chapters 1–6 scope, the accumulated lessons and its specific checks.
Every stage finds its later siblings already standing. Proof returns to this checkpoint and cuts nothing.

The root and document briefs carry the latest timing choice.
The later Parts III–IV brief and both drafts now point to F1–F16 and the complete lessons record.
The shared contract states the new whole-book rules. The lessons record distinguishes each transferable rule from its Ravel example and the owner's exact words.

The diagnostic `pick` names `rhai-book-use-k84`, before this live checkpoint.
The new draft's path resolves through the root and document briefs. Its four stage bodies pass the prose checker with zero hard or advisory findings.
The current task still resolves to its live path. It cannot retire until the remaining read and practical tasks meet its Done when.

The next session drafts the corrections. The resumed checkpoint reads the final proof's recap, refreshes the local book and private shell, and opens chapter 4.
Its recap includes the amended Part I so the owner can inspect those changes without repeating the completed read.

Validation passed for all twelve changed records: zero hard prose findings, with existing advisories and declared exclusions retained.
A comparison with `3c80f840` found no change under `docs/book/src`. The chapter corrections belong to the ordered run above.
This session stopped its local book server before the handover. The resumed checkpoint must start a fresh server for the proofed text.

## Correction recap for the Part II read, 2026-10-28

Proof `rhai-book-use-k87` completed the authorised correction run of chapters 1–6.
Resume at chapter 4 after a fresh local build, server and private reader shell. The owner already reviewed Part I and need not restart it.
The amended Part I remains available through its normal contents entries for inspection.

- Chapter 1 now begins with one complete review: its files, prompt, permitted results, completion commands and the orchestrator's next action.
  A sequence connects that request to the result and the next task. The general roles follow the concrete case.
- Chapters 1–6 use “configuration script”, “settings”, “Rhai worker” and “selection context”. Literal names, flags, fields and diagnostics stay exact.
  Each chapter restates its terms and carries the same review into its explanations.
- The worker explanation names the second copy of `ravel`, the deadline, host requests and failure isolation.
  It states that worker cleanup precedes the final managed launch. Figures agree with that order.
- Chapter 2's title is “Who does what in a Ravel session”. Chapter 1's final heading names the checks and what remains unchecked.
  Chapter 3 uses the familiar `cd` instruction. JSON descriptions name their purpose, and Grove links to its repository.
- Chapter 5 now runs the caller's side as an executable example. The orchestrator checks the status, reads `changes-needed` and requests a `fix` session.
  Chapter 6's fragments agree with that revised review. Its links describe their current destinations.

Proof corrected two remaining summaries: chapter 3's refusal concerns the final managed launch, and chapter 6's permitted environment variables belong to the helper.
The Rhai worker's environment stays empty. Lessons L16 and L36 retain the transferable findings and the inventory-instrument correction.

All 22 examples, nine excerpts, 18 figure reproductions, strict prose checks and the book build passed.
Local checks passed 2,075 HTML targets and nine README links, with failing controls. All eleven external destinations responded successfully.
Fresh Safari inspection covered the six openings and all 13 figures at chapter width. The proof task records its source inventories, subject digests and visual limits.

The practical tasks remain outstanding: a fake-agent session, refusal and cancellation interpretation, and one direct reference entry.
The proof and the completed Part I review supply no practical-task result. Record the owner's results separately as this checkpoint resumes.
Read 2 remains required after the later whole-book proof.
