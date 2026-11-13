# rhai-book-use-k74


## Goal

Complete Read 1 of Parts I and II with the owner as the labelled stand-in reader.
Record the owner's task performance and corrections before dependent drafting continues.

## Context

- Section 4 of `docs/book/contract.md` requires this checkpoint after proof.
  The initial proof is `rhai-book-use-k73`, and the first correction proof is `rhai-book-use-k78`.
  The latest completed proof is `rhai-book-use-k96`. This resumption follows that correction proof.
  Read that proof's evidence and correction recap first.
- The owner completed Part I, then reached chapter 4's example-setup section and reported F17.
  Resume at the interrupted setup section after correction. Make amended Part I available without a restart at chapter 1.
- Refresh the local book with `mise exec -- task docs:book` after `task setup`. Check that its figures load.
  Prepare and check the private reader shell again before the owner runs the outstanding tasks.
- The book describes the agreed 2.0.0 design. Use the latest proof's dependency record for any difference from the checkout.
  Complete Rhai examples run through the built CLI. The release still owns the version placeholder.
- Proof `rhai-book-use-k96` reconciled the final notice path after delivery, spec reconciliation and `configuration-names-k92`.
  Its evidence records the final interface and environment-scrub wording. Release-version substitution remains with the release leaf.

## Done when

- The owner uses the book to write a configuration script for a fake agent and run a session.
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
Use fake agents and the examples' private environment. No real model call or personal configuration conversion is part of this read.

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
The ADR `configuration-evaluation-precedes-the-launch` records the process boundary, hard deadline and failure-isolation reasons.
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

**Resume after the second correction proof, 2026-10-28.** Evidence class: preparation, not the owner's read.

The parent at entry is `5b76154d`, which retired proof `rhai-book-use-k87`.
The working copy was empty. All six chapter digests match the proof's recorded subjects.
The agreed tasks and chapter 4 entry remain settled, so this session opens no requirements interview.
Part I stays available for inspection without a repeated read from chapter 1.

`mise exec -- task setup` and `mise exec -- task build` passed.
`mise exec -- task docs:book PORT=8796` built without a warning and serves the fresh book at `http://127.0.0.1:8796`.
All six chapter pages and thirteen distinct SVG paths returned HTTP 200. The absent-image control returned 404.
These responses establish delivery, not visible rendering.

Safari in an isolated macOS 27 guest opened a copy of this fresh build.
The chapter 4 opening and the upper part of its evaluation-harness figure were visible at the chapter width.
This is a bounded load check, not a repeated whole-book visual proof. The guest is stopped after the check.
The local server stays available for the owner's read.

The fresh reader workspace is `/private/tmp/ravel-read1-k74-Cq83i6uO`.
Its `enter` script uses the same cleared-environment procedure as the earlier shell, with new private directories.
The checked shell has the expected home, temporary directory and working directory.
It has neither `RAVEL_EXIT_FILE` nor `RAVEL_POLICY_RUN`, no Grove variable, and neither real harness on its path.
Its `ravel` points to this checkout's fresh build and reports `ravel 1.1.0`, as expected before release.
No reader exercise ran during this preparation.

The graph CLI refused access because a pre-coordination or unverified generation is active.
This session uses document reads and claims no graph project, generation or coverage.
The checkpoint remains live for the owner's Part II observations and practical-task results. Read 2 remains required.
The six chapter digests stayed unchanged through this preparation. The checkpoint note is the only tracked change.

**Owner findings F17–F18, chapter 4, 2026-10-28.** Evidence class: the owner's stand-in read and product instruction.

The owner reached “Run the examples of this chapter” in chapter 4.
They reported no practical-task result, later Part II progress or viewing surface.
The chapter's digest is unchanged from proof `rhai-book-use-k87`.

**F17, the example setup has no concrete subject.** The owner's words follow.

<!-- tc: off because this is the owner's observation -->

> "Run the examples of this chapter" lacks context - for example what files need to be created there - and the table doesn't make sense.

<!-- tc: on -->

Located: chapter 4, lines 182–208. The section says to create “the listed files”, but its own text lists no files.
The first complete example, with its file blocks and command, comes after the next section's explanation of the configuration interface.
The table explains isolated command fragments before it shows a complete command. Its claim about “Each command line” also hides differences between examples.

These observations explain the session's reading of the finding. They are not further words from the owner.

Disposition: accept. Draft owns the setup sequence and its placement.
Use one named example workspace. Show which directories the reader creates, which files they save, their paths and purposes, and their complete contents.

Distinguish those inputs from the private directories that the execution command creates.
Show the complete command before any table explains its parts. Explain later examples' differences where they occur.

Apply this pattern to every procedure that assumes files or paths the book has not yet given the reader.
Copy-edit checks the sequence as a reader instruction. Proof checks the visible files and command through the existing example seam.
The read stops at this comprehension failure for correction. Part I remains complete, and the practical tasks remain outstanding.

**F18, remove the old term from the product interface.** The owner's words follow.

> Also we should eliminate the term policy, in file names and arguments.

Disposition: accept the product rename. This supersedes the earlier choice to preserve literal filenames and CLI options after F10.
The proposed spellings are `config.rhai` and `--config-env`, consistent with the existing `--config` option.
The session asked whether the rename also covers settings, JSON fields and diagnostic codes, with no old aliases in 2.0.0.
That scope choice remains unanswered at this entry. No product name has changed yet.

Implementation must precede the book's literal updates, so examples and help remain executable and exact.
The original process and cleanup contracts remain the test oracle for the rename.

**Owner intention: TestSubject for tutorial execution, 2026-10-28.** The owner's words follow.

<!-- tc: off because this is the owner's intended use and feasibility question -->

> My intention is to use the project `TestSubject` to run through tutorials/examples like in chapter 4, validating that each step works, and that the steps executed in order, in the environment specified in the tutorial/example, produce the right outputs and not require more expertise or tools than are specified. Is it feasible to do that for our chapter 4. Note that TestSubject is a shell at the moment.

<!-- tc: on -->

The local TestSubject repository is `/Users/antony/Development/AgenticWriting/TestSubject`.
Its README describes the learner-validation goal and marks the project as pre-development.
Its TODO proposes models and a simulation API. Neither document describes an implemented tutorial executor.
This inspection changes no TestSubject file and uses no graph evidence.

The current Ravel checker prepares more than a reader's empty directory.
`scripts/examples.sh`, `run_example`, creates parent directories, copies every labelled file and makes listed fake programs executable before the commands run.
`docs/book/examples.md` declares those mechanics. The checker executes commands in order and compares the stated results.
That evidence does not establish that the visible tutorial supplied every preparation step.
F17 is a concrete case for a separate check of instruction sufficiency.

The session recommends chapter 4 as TestSubject's first bounded use case.
Start with its first complete example, then cover the other examples and their declared resets.
Give the learner the visible instructions, the material they explicitly reference, the stated prerequisites, and the declared starting environment.
Each action needs a supporting instruction or prerequisite. Record an unsupported inference or missing tool instead of silently supplying it.

Preserve shell state between steps when the instructions require it. Reset only where the tutorial tells the reader to reset.
Compare actual streams, status and relevant files with the stated outcomes, and record the first failure with its source passage.

Proposed controls remove a setup instruction, reorder dependent steps, remove a required tool, and change an expected result.
These controls have not run. They would test whether the new validator detects the failure it claims to detect.
An execution trace can establish behaviour in a measured environment. A model's account of prerequisite knowledge remains model evidence.
It cannot establish novice comprehension on its own, and no TestSubject result has replaced either required reader checkpoint.

The owner has asked about feasibility. No TestSubject implementation or new release dependency is assumed from that question.

**Owner clarification: completeness and an LLM reader, 2026-10-28.** The owner's words follow.

<!-- tc: off because these are the owner's purpose and implementation direction -->

> The purpose of TestSubject was to ensure that each tutorial or example contains all instructions, all prerequisites, all the context required, as well as actually doing the steps to ensure that the tutorial is correct.

> TestSubject is intended to be an LLM system that reads the tutorial, rather than a deterministic system using traditional code.

<!-- tc: on -->

This settles the intended validator: an LLM reads the actual tutorial as the intended learner and performs its steps through tools.
It examines whether the tutorial supplies the instructions, prerequisites and context needed for each action and for interpretation of its result.
The shell supplies execution and observations. It does not replace the LLM with a parser or a prewritten sequence of example actions.
No hidden fixture, extracted action plan or repository source may silently supply knowledge or preparation that the tutorial omits.

The learner profile distinguishes declared knowledge from material taught by the tutorial or an explicit reference.
Ordinary actions within a stated prerequisite are allowed. Ravel-specific knowledge is not silently treated as ordinary shell knowledge.
At a gap, TestSubject records the passage, what the learner needs, why the supplied material does not establish it, and the observed execution if reached.
It does not fix the tutorial or use expert knowledge to turn that same attempt into a pass.

The resulting record separates an observed command result from an LLM judgement about instructional sufficiency.
The earlier proposal for a tutorial executor is subordinate to this LLM-reader design, not a separate deterministic validation product.

**Continuation and the concrete name map, 2026-10-28.** The owner said “continue”.

The session proceeds with consistent public configuration names, as its interpretation of the instruction to eliminate the old term.
This is not a separate owner quotation or a claimed answer to the earlier scope question.
`configuration-names-k92` records the map: `config.rhai`, `--config-env`, `configEnv`, `config`, `configEnvironment`, `configCode`, `config_*` and `RAVEL_CONFIG_RUN`.
It preserves explicit arbitrary paths and opaque data. It supplies no old option, setting or default-path aliases in 2.0.0.
Historical evidence and old-input refusal cases remain exact, as do stable Grove handles and third-party identifiers.

The failure JSON moves to schema version 2 because the published version 1 has the old field names.
The unreleased routing notice and Rhai script retain schema versions 2 and 3.
The rename also reconciles current source and document filenames, links, help, examples, tests, specs and ADRs.
It does not change routing, result transport or cleanup behaviour.

The already agreed test seam remains the real CLI in a private home with fake programs.
The session put the rename cases to the owner: paths, environment grants, refusals, JSON output and retired spellings.
No different seam or additional owner constraint was reported before this record.
The personal settings file was absent when checked. This is a point-in-time observation, not a conversion-time assumption.

**Bounded TestSubject trial, 2026-10-28.** Evidence classes: one LLM-reader judgement and one measured example execution.

One fresh subagent received the reader prerequisites, chapters 1–3 and chapter 4, with a stopping instruction before “Select a command or refuse”.
It received no owner finding, repository source, example extractor or prepared example files.
The private workspace was `/private/tmp/testsubject-ravel-ch4-oDmXRPm8`, with the checkout build preinstalled.
Installation and release-version matching were explicitly outside the trial.

The LLM created the labelled files manually and ran the published commands through the private shell.
It cited the earlier chapters and declared shell knowledge for directory creation, file saving, permissions and interpretation of cleanup reports.
Its judgement was that no material information gap prevented completion of the first example.
It did not identify the context complaint that the owner reported in F17. That result does not overturn the owner's observation.

The captured standard output was exactly `Explain this change.`, 20 bytes without a final newline, with status 0.
This session independently checked those bytes and the captured status file.
Standard error contained the routing notice and two `Operation not permitted` cleanup reports, which chapter 3 explains.
Neither session independently instrumented process-group cleanup. Status 0 has the documented interpretation, with that limit.
The four supplied chapter digests and executable digest were unchanged after the trial.

The subagent reported that a citation lookup after the run showed lines beyond the stopping boundary.
This is an actionable defect in the trial's document boundary. Future bounded trials receive a physically truncated chapter copy.
The measured command result remains evidence, while the model's reading judgement carries the boundary limitation.
Its full report and captured files remain in the private workspace. The essential observations and limitations are preserved here.

This used the leaf's one fresh-context review allowance. No second trial or reviewer ran in this leaf.
Lesson L67 records that a successful derivation does not establish instructional sufficiency.
No TestSubject product was implemented, and no model result replaces either owner checkpoint.

**Correction routing, 2026-10-28.** The existing TypeScript-removal node now precedes the next correction.

It freezes the pre-rename observations first, then removes the old worker and its delivery machinery.
`configuration-names-k92` follows that node. Draft `rhai-book-use-k93`, copy-edit `rhai-book-use-k94`, art `rhai-book-use-k95` and proof `rhai-book-use-k96` follow it.
This is the owner's directed correction after F17–F18 and “continue”, not an unattended repeat of a completed judgement.
Each stage has its successor already cut. Proof returns to this same live checkpoint and creates no duplicate.

The root and document briefs record the order. Later conversion work names `config.rhai` and preserves the running driver's `policy.ts` and versioned binary.
The active shared settings stay unchanged until that driver restarts. The later whole-book work inherits F17–F18 and L67.
This leaf remains live: Part II and the owner's practical tasks are incomplete. No retirement or parent close is claimed at this handover.

**Handover checks, 2026-10-28.** The whole-file prose check passed for all sixteen new or edited documents.
It reported zero unaccepted hard findings. This session reviewed the 45 advisory findings. Existing pending regions remain unchanged.
The separate preparatory commit `a99a4ea0` fixes three pre-existing prose findings in the documentation lessons without changing their meaning.

The tree diagnostic selects `rhai-equivalence-fixtures-k30` next, and the mandated handle still resolves to this live checkpoint.
The new draft's brief chain resolves through the root and `rhai-book-use-k26`.
This session stopped its local book server before the handover. The resumed read needs a fresh build, server and private shell after proof.
The changes are task and contract records. No chapter, runtime source or TestSubject product changed in this requirements handover.

**Resume after the third correction proof, 2026-10-29.** Evidence class: preparation, not the owner's read.

The parent at entry is `e7079a53`, which retired proof `rhai-book-use-k96` and closed `rhai-book-use-k26`.
The working copy was empty. The agreed tasks and chapter 4 entry remain settled, so this session opens no requirements interview.
The read resumes at “Run the examples of this chapter”. Part I remains available without a repeated read.

`mise exec -- task setup` and `mise exec -- task build` passed.
`mise exec -- task docs:book PORT=8796` built without warnings and serves the book at `http://127.0.0.1:8796`.
All six chapter pages and thirteen distinct SVG paths returned HTTP 200. An absent-image control returned 404.

These checks establish file delivery. The in-app browser was unavailable, so this preparation claims no fresh browser view.
The six chapter digests stayed unchanged through preparation. The prior proof retains its recorded visual evidence and limits.

The new private reader workspace is `/private/tmp/ravel-read1-k74-p9JPUF9r`.
Its `enter` script starts zsh through `env -i`, with new home, temporary and work directories.
Its private `bin/ravel` links to this checkout's built executable. The remaining PATH entries are system directories.

The shell reports the expected directories and `ravel 1.1.0`. Neither real harness is on its path.
It has no Grove variable, `RAVEL_EXIT_FILE`, `RAVEL_CONFIG_RUN` or `RAVEL_POLICY_RUN`. No reader exercise ran during preparation.

The graph CLI refused to start because a pre-coordination or unverified generation is active.
This session uses document reads and claims no graph project, generation or coverage.
The prior proof recap now has a historical-text marker, preserving its wording while the current record follows the prose rules.

The owner received the chapter 4 setup link and the private shell command.
The next requested observation is the first unclear instruction, or the first example's output and exit status if completed.
The checkpoint stays live for the owner's Part II read and practical results. Read 2 remains required.

**Owner direction: publish the current work, 2026-10-30.**

<!-- tc: off because this is the owner's publication instruction -->

> I would like you to advance main and push to git right now, so that this book work is current in the repo - people are looking at it.

<!-- tc: on -->

The owner authorises advancing `main` and pushing the current work before the remaining reader tasks finish.
This publication preserves the live checkpoint. It supplies no reader observation or practical-task result.
The current book build passed without warnings before publication. The owner still needs to complete Read 1, and Read 2 remains required.

**Owner direction: stop for reconfiguration, 2026-11-01.**

The owner said, “I want to stop this grove so I can do some reconfiguration and then restart it”.
This explicit stop request prevents further work in this session. Continuing or decomposing the task would not satisfy that request.
The session will send `grove-force-stop` through the exact versioned Ravel command in its original mandate.
The leaf remains live. The session records no teardown.

The earlier publication succeeded. Remote `main` matched `24a545236185b0910ba6117fe8f9e8b2e2c8a748`, and the working copy was clean after the push.
The remaining work is the owner's Part II read, fake-agent session, refusal and cancellation interpretation, and direct-reference task.
The owner reported no new reader result since the last preparation. Read 2 remains required.

After reconfiguration, rerun `grove` from this working tree to resume the live checkpoint.
Refresh the local book and private reader shell before resuming chapter 4 at “Run the examples of this chapter”.
Part I remains complete. No personal configuration change is part of this stop.

**Owner question and rebase, 2026-11-13.**

The owner said, “I think this may already be done!”
They then noted that the work came from a separate grove and might need a rebase.

`main` at `d6d81765` contains the later whole-book proof, `424af53d`, and the `v2.0.0` release, `ed6cedbf`.
The proof's record is `docs/evidence/ravel-book-k311/README.md`.
It explicitly preserves both owner reader checkpoints with this paused workstream and supplies no owner result.
The original task files on `main` still leave this checkpoint live.

The only local change above the common ancestor was the reconfiguration stop note in this file.
`jj rebase -s ylpyxwvs -o main` moved that change and its empty descendant onto the current `main` without conflicts.
The workspace had no changes after the rebase. Its difference from `main` was confined to the preserved stop note.
No local source or book correction was lost, and no build or example check ran during that rebase.

The book and release work in the other grove does not establish the owner's Part II results that were still due.
Those results cover a fake-agent session, interpretation of refusal and cancellation, and one direct reference entry.
The owner's completion question supplies no specific task result or instruction to drop the checkpoint.
This leaf stays live for that clarification. Read 2 remains separate.

**Owner confirmation and verification scope, 2026-11-13.**

The session asked whether the owner completed the Part II exercises elsewhere.
The question named the fake-agent session, refusal and cancellation interpretation, and direct reference lookup.
The owner answered, “yes, but you should check things completely yourself to confirm”.

This records the owner's reported completion of those tasks.
The answer supplied no command transcript, individual outputs, new observation or detail about how the owner read the book.
This session will independently check the current book, each acceptance condition and the executable examples before retirement.
Those mechanical checks remain distinct from the owner's report. This read does not establish novice comprehension.
Read 2 remains a separate requirement.

**Independent verification and acceptance, 2026-11-14.**

The owner's confirmation completes the reported practical tasks of Read 1.
The session independently checked the current book and executable before it accepted this checkpoint.
The subject is `main` at `d6d81765`, with the preserved stop note and this task record above it.

The completed correction runs retain the dispositions and owners of F1–F18.
This session checked their material changes against the current text:

| Observation | Current evidence | Disposition and owner |
|---|---|---|
| F1 | Chapters 1 and 4 introduce Rhai, its characteristics, an example and its documentation. | Applied by draft. |
| F2 | Chapter 1 explains the selection and routing notice with real values, a table and a figure. Chapter 5 runs both notice forms. | Applied by draft and art, checked by proof. |
| F3 | Chapter 1 explains how the caller can extend or modify the prompt. | Applied by draft. |
| F4 | Chapters 1 and 4 explain model-assisted choice and the harness-run recipes. | Applied by draft. |
| F5 | Chapter 1 explains inherited `RAVEL_EXIT_FILE` paths and shows the nested paths in a figure. | Applied by draft and art. |
| F6 | Chapters 1 and 5 follow the sent result through shutdown, cleanup and the caller's next action. | Applied by draft. |
| F7 | Chapters 1, 2, 4 and 5 repeat definitions and the concrete review where the reader needs them. | Applied through the contract, draft and copy-edit. |
| F8 | The current explanations address the reader and state the reason before the mechanism. | Applied by draft and copy-edit within the prose contract. |
| F9 | Chapter 1 opens with named files, the complete review prompt, permitted results and the orchestrator's next action. | Applied through the contract and draft. |
| F10 | The current book distinguishes configuration, configuration script, settings, Rhai worker and selection context. | Applied by draft and copy-edit. |
| F11 | The explanations name JSON by purpose. Exact examples retain the required schema fields. | Applied by draft and copy-edit. |
| F12 | Chapters 1 and 4 explain the second executable process, deadline, host requests, failure isolation and launch order. | Applied by draft and art. |
| F13 | Chapter 1 links Grove to its repository at its introduction. | Applied by draft. Proof owns the link checks. |
| F14 | Chapter 1 ends with “See what we tested and what remains unchecked”. | Applied by draft and copy-edit. |
| F15 | Chapter 2 is “Who does what in a Ravel session”. | Applied by draft and copy-edit. |
| F16 | Chapter 3 gives explicit `mkdir` and `cd` commands. | Applied by draft and copy-edit. |
| F17 | Chapter 4 names `fixed-route`, its workspace, three saved files and their purposes before the complete command and command table. | Applied by the third correction draft, checked by copy-edit and proof. |
| F18 | Current examples and CLI help use `config.rhai`, `--config` and `--config-env`. | Applied by `configuration-names-k92`, then the third correction run. |

The later whole-book proof, `ravel-book-inside-k311`, supplies the broader editorial record at `docs/evidence/ravel-book-k311/README.md`.
This session did not repeat that independent editorial review or claim another reader's comprehension result.

The session manually saved the first chapter 4 example's three files and ran its setup and command in a private home.
It returned exactly `Explain this change.`, without a newline, with status 0.
A separate configuration refused the request. It returned no result, `config_refused` and status 3.

The chapter 5 cancellation fake sent `candidate`, then INT. The caller received no result and `session_cancelled`.
Its raw process return was `-2`, equivalent to shell status 130.
Chapter 6's direct entry also correctly describes `end-session` outside a session: no result, the enclosing-session diagnostic and status 1.

The first manual cancellation probe incorrectly expected numeric 130 from a process API.
Its original captured status and diagnostics remain in the private evidence directory.
A minimal signal control produced `-2` through the process API and 130 through a shell that collects the child status.
The corrected probe retains both forms and checks the diagnostic and empty output independently.
This was an error in the measurement, not a required product or book correction.

Fresh checks passed through the project's Taskfile:

| Check | Result |
|---|---|
| `mise exec -- task setup` and `task build` | Pinned book tools prepared. The Ravel 2.0.0 build passed. |
| `task check` | The format, shell, tool, installation, lint and terminal checks passed. All 713 enabled Rust tests passed. `nextest` skipped 48 tests. |
| `task docs:examples` | All 27 complete examples passed, with no page error. The 72 labelled fragments did not run. |
| Installation-prefix example run | The same 27 cases passed in the same order. A valid default skill source resolved in the positive control. |
| `task docs:excerpts` | All 22 excerpts matched. |
| `task docs:figures` | All 21 SVGs and layout records matched their sources. |
| `task docs:ste` and `task docs:breaks` | No prose problem in the default 35 files. No long run in eleven chapters. |
| `task docs:book:build` | Passed. The build reported no error. |
| Checker controls | Example, excerpt, figure, prose and break control tasks passed. Their declared mutation checks also passed. |
| Local link and delivery check | All 1,162 local references passed. Eleven chapters and 21 figures returned their exact file bytes over local HTTP. |

The link check rejected planted missing pages, anchors and figures. A missing HTTP figure returned 404.
The source manifest covers all 3,665 tracked files outside `.grove/`. The set covers manifests, scripts, sources, fixtures and book inputs.
Their SHA-256 digests were identical before and after verification. A deliberately changed manifest entry failed the equality control.

The rendered book and the manual probe's subjects also remained unchanged during their checks.
The two example runs were compared case by case, rather than only by total.

Private evidence is in `/private/tmp/ravel-k74-verification-mx0mmj79`.
It contains the before and after manifests, complete task logs, manual outputs, default-source control, example comparison and local-link results.
All probes used private homes and fake programs. This session made no real model call.

The refreshed graph reports a partial parse of `scripts/examples.test.sh`. Direct source inspection and executable controls cover that limitation here.
SVGs are excluded from the graph. Regeneration, XML inspection and HTTP delivery supplied their evidence.

The owner's report and these mechanical results remain separate evidence classes.
The report adds no command transcript, display method or interpretation beyond the owner's confirmation.
This read does not establish whether a developer new to Ravel can use the book.

No new reader observation needs a correction stage. Read 1 can retire.
Read 2 remains required under section 4 of `docs/book/contract.md` and the later book node's brief.
The next book sessions must compare their old task contracts with the work already delivered on `main` before they repeat any draft.

<!-- tc: off because this recap records the completed k87 proof before the owner's F17 and F18 findings -->

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

<!-- tc: on -->

<!-- tc: off because this recap records the completed k96 proof before the resumed reader checkpoint -->

## Correction recap after F17–F18, 2026-10-29

Proof `rhai-book-use-k96` completes the owner-directed third correction run of chapters 1–6.
Resume at chapter 4, “Run the examples of this chapter”, the setup section where this read stopped.
Refresh the local book build/server and private reader shell first. This remains the same live checkpoint.
The owner completed Part I and need not read it again. Its revised pages remain available through the normal contents entries.

- Chapters 3–5 now name each example and its workspace, saved files, generated paths and purpose before the actions.
  Chapter 4 starts with `~/ravel-chapter-4/fixed-route`, its tree, two files and complete command. The command table follows that command.
  Later examples name their differences and dependent blocks. Chapter 5 restates its own preparation and sandbox prerequisite.
- Chapters 1–2 explicitly have nothing to run and point to chapter 3's first session.
- The implemented interface now uses `config.rhai`, `--config-env`, `configEnv`, `RAVEL_CONFIG_RUN`, configuration diagnostics and JSON fields.
  Failure documents have schema 2; routing schema 2 and script/request schema 3 remain distinct. Arbitrary caller data and the rejected historical filename stay exact.
- Chapter 6 and the shared installed-file figure include `share/ravel/notices/THIRD-PARTY-NOTICES.md`.
  The final cutover, naming and notice-path dependencies are resolved for these parts. Release-version substitution remains with the release.
- Proof corrected chapter 5's repeated prompt label, distinguishing file reading from interpretation, and one word in the installed-files key.
  Lessons L40, L67 and L69 retain the transferable observations.

Final mechanical checks passed all 22 complete book examples, nine excerpts, 18 figure reproductions, strict prose and break checks, the build and local links.
The proof task records the fresh source inventories, 43 example-checker controls, failing link/prose controls, unchanged subjects and viewing limits.
Safari inspection covered the revised chapter 4 setup and the installed-layout table and figure at the 750-pixel column.

One fresh LLM reader received physically bounded chapter copies, ordinary prerequisites and an empty private home with the checkout executable.
It manually ran the first chapter 4 example once. Captured output was exactly `Explain this change.`, without newline, and status 0.
It reported no missing instruction, but its first report described standard error before inspecting it. The captured stream and a separate correction preserve that error.
This is one model judgement and one execution result, not evidence of novice comprehension or the owner's task performance.
The trial did not implement TestSubject or replace this read.

The practical tasks remain outstanding: write a fake-agent script and run a session, interpret refusal and cancellation, and use one direct reference entry.
Record the owner's results separately. Later whole-book proof and Read 2 remain required.

<!-- tc: on -->
