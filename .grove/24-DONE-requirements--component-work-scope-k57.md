# component-work-scope-k57

## Goal

Settle with the human the scope of the component work that the root still
requires. Use the places in the code that the owner's two stand-in reads of
the book found hard.

## Context

- The root brief holds the component-boundary, extraction and delivery goals.
  It also holds three records:
  - K4's comparison retained the existing implementation and left its
    comparison incomplete. Its open items are unread alternatives, six-month
    support cost, delivery and use beyond entry, target execution, and runtime
    and lifecycle debt.
  - K6 made a separate observation.
  - K22 closed and left these items as scope questions for a requirements
    session with the human.
- The book contract `docs/book/contract.md`, the book's chapters 7–9 and the
  recorded results of read 1 and read 2 inside `ravel-book-k55`. By then,
  chapters 7–9 explain the current components.
- `book-structure-k5` placed this leaf after the book, by the human's decision
  of 2026-10-16. The decision was to draft the book against the current code
  first. Thus, this step can use the places where a reader got lost.

## Done when

This leaf records the human's answers verbatim, with the meaning of each, on
these questions:

- Whether to resume a comparative redesign, and with which frame, or to keep
  the current arrangement.
- Which component, if any, becomes independently usable by other projects.
  The answer includes its interface, consumer and packaging. The book's
  chapter 10 is part of that work.
- Whether any consumer needs a persistent-runner (returning host) contract.
- The delivery of this workstream's corrected code: the merge to `main`, the
  next release, and the book's first publication with that release.

Each answer that implies work gets its next precise leaf. This leaf narrows no
requirement.

## Notes

The root's standing budget direction applies. Budget does not gate required
work, and zero production disruption still binds. If the human wants the
corrections released before the book is complete, the delivery question can
move ahead of the book node by `leaf-insert`. That is the human's decision.

## Decisions (running log)

**Question 1, the comparative redesign (2026-10-21).** The human selected
“Keep current (Recommended)”. The meaning:

- No comparative redesign starts again. The current arrangement of the Ravel
  CLI, the TypeScript policy worker and the `keyed-launch` runner stays.
- K4's comparison closes as “not pursued, by the human's decision”. This
  record does not say that its alternatives are worse.
- G1, G2, G4, G5 and the R-W debt concern the alternatives or their
  migration. No session owes them now. They stay as unresolved facts in k4's
  record. G3, the delivery and use of a component beyond the entry, moves to
  question 2.
- The root's design goal rests on the book's explanation of the current
  parts and on the answers to questions 2–4. A change of structure does not
  supply it.
- A redesign can start again when a concrete change or a reader task shows
  a structural problem.

The evidence that the human saw before the answer:

- Read 1 has 17 findings. Each concerns the framing, wording or figures of
  the book, not the structure of the code.
- Read 2 has no finding. The owner did no task, and its log does not record
  the chapters that the owner read.
- The drafting logs of chapters 7–9 record no place where the code was hard
  to explain. This is model evidence. A search with a control that found a
  known phrase supports it.
- K4 ended undetermined, with an incomplete comparison.
- Campaigns k43 and k52 measured the corrections after k4. On aarch64 macOS
  and aarch64 GNU/Linux, 153 of 153 rows and 310 of 310 protected tests pass.

**Question 2, independent use (2026-10-21).** After the answer to question 1,
and before question 2, the human wrote these words:

<!-- tc: off because the human's words are a quotation -->

> “Grove already uses ravel I think, and that old code in grove is obsolete
> and should be purged.”

<!-- tc: on -->

Then the human selected “No extraction (Recommended)”. The meaning:

- Ravel extracts no component now. `keyed-launch` stays a private package of
  Ravel.
- Grove drops its copy of `keyed-launch`. It starts `ravel` as an ordinary
  child process and leaves job control to Ravel. This is work in another
  repository, and its design is Grove's. After the human agrees to the
  filing, it becomes an issue in `Linkuistics/grove`.
- The human's decision that no component earns extraction now meets the
  root's extraction goal. Plan-k1 does not require the extraction of each
  generic-looking helper.
- Nobody writes chapter 10. The book contract, and each chapter that mentions
  chapter 10, will say that Ravel supports no separate component. That change
  goes through the editorial stages.
- Extraction can start again when a project needs Ravel's runner without
  Ravel's CLI.

The facts that the human saw before the answer:

- Grove runs every session through `ravel`. It starts that process with its
  own copy of the runner. The call sites are `run_observed_with_stdout` at
  `crates/grove-loop/src/loop_driver.rs:404` and
  `run_noninteractive_with_stdout` at `crates/grove/src/standalone.rs:180`, in
  the checkout `/Users/antony/Development/grove` on 2026-10-21.
- Line 42 of Grove's `docs/specs/harness-selection-and-execution.md` records
  the decision to keep that copy. The same paragraph says that no Rust
  dependency crosses the project boundary. The purge reverses the decision.
- The copies diverged. `operation_reports`, `start_check` and
  `native_barrier` occur only in Ravel's copy, and `_with_stdout` occurs only
  in Grove's. `EntrySignals` occurs in both, so the search works in both
  trees.
- A library for Grove's loop would need a contract for a host that continues.
  Ravel's runner can change the signal state of the whole process, and its
  return does not restore that state.
- AgentAnyware has its own process code. Its last recorded change is from
  2026-09-06.

**Question 3, the persistent runner (2026-10-21).** The human selected “No
consumer needs it (Recommended)”. The meaning:

- This workstream designs and builds no persistent runner.
- `CONTEXT.md` and the book keep the term only as a limit. The book
  contract's “Undelivered contracts” already requires this. No text about the
  persistent runner changes.
- The work can start again when a consumer must supervise managed processes
  in a host that continues after each one. An example is a program that uses
  a runner library.

The facts that the human saw before the answer:

- After the Grove issue, Grove starts `ravel` as a child process for each
  session, and Ravel exits after each one. Grove needs only the launcher
  contract.
- AgentAnyware's README says: “One process hosts exactly one session.” It
  does not use Ravel.
- A search of the owner's other Rust projects for job-control calls found
  only AgentAnyware. The same search on Grove's runner found its files.

After the answer, the human added: “AgentAnyware is an old project, for
interest/reference only.” Thus, AgentAnyware is not a consumer.

**Question 4, part 1: when to deliver (2026-10-21).** The human selected “At
the end (Recommended)”. The meaning:

- This workstream finishes its remaining leaves first: the book change for
  chapter 10, the ADR leaf `adr-native-waitability-status-k191` and the
  release notes.
- Then the finish session removes `.grove/`. After that, `main` moves to the
  workstream, and one release carries the corrections and the book.
- The merge and the release are outside Grove's loop, as Grove's finish rule
  says. The release has public effects, so it needs the owner's word when it
  runs.
- Until that release, the owner's installed Ravel stays at 1.0.0.

The facts that the human saw before the answer:

- `main` is the 1.0.0 release commit `20df8840`. Each commit on `main` is in
  this workstream, so the merge can be a fast-forward.
- The owner's Grove loop runs the installed Ravel 1.0.0, which does not have
  the corrections. The corrections concern failure paths.
- No commit on `main` touches `.grove/`.
- `task release:minor` (or `patch` or `major`) runs the release from the
  default workspace. It pushes `main` and a tag, makes the GitHub release,
  puts the book on `gh-pages`, updates the tap and upgrades the local Ravel.
- The owner turns GitHub Pages on once, after the first release creates
  `gh-pages`. Until then, the release stops at its last check, `live`.
- `CHANGELOG.md` lists only the help rewording under “Unreleased”.

**Question 4, part 2: how `main` takes the workstream (2026-10-21).** The
human selected “Fast-forward (Recommended)”. The meaning:

- After the finish session, `main` moves to the last commit of the
  workstream. `main` keeps each commit of the workstream, with its leaf
  handle.
- Those commits hold `.grove/` in their trees. The finish commit removes it
  from the tip only.
- Each commit ID that the durable documents cite stays reachable from `main`.

The facts that the human saw before the answer:

- The documents outside `.grove/` cite 14 commit IDs of the workstream. An
  example is `0bdcb1f9…`, the reference commit of k4. A test with the known ID
  of `main`'s tip showed that the matcher works.
- A squash would keep those IDs only through a bookmark, as
  `retired/actor-daemon-2026-10-12` keeps an older line on GitHub.
- Grove's commit procedure says: “teardown removes `.grove/` from the tip
  rather than from history”.
- Both ways put the 3,277 evidence files (20 MB) on `main`.

**Question 4, part 3: the version (2026-10-21).** The human selected “Minor:
1.1.0 (Recommended)”. The meaning:

- The release that carries the corrections and the book is 1.1.0. It runs
  `task release:minor`.
- The cut writes `ravel 1.1.0` into the version line of chapter 3.

The facts that the human saw before the answer:

- The notes of 1.0.0 commit the CLI to semantic versioning.
- The JSON notice has a new field, `operationReports`. It is present only
  when the runner kept operational reports. This addition is compatible.
- This workstream rewords the `end-session` help and describes
  `--policy-env`.
- Corrections change failure paths only. For example, an unconfirmed worker
  reap stops the session before the managed process starts. A refusal that a
  cancellation replaces keeps the original failure details in its
  diagnostics.
- The command-line arguments and the owner configuration do not change.

**The book without chapter 10 (2026-10-21).** This follows from question 2,
and the book contract gives structure decisions to the human. The human
selected “Remove and renumber (Recommended)”. The meaning:

- Chapter 10, “Use Ravel's components in your project”, leaves the book plan
  and the summary.
- “Develop Ravel” becomes chapter 10, and “Deliver Ravel” becomes chapter 11.
  Their files take the new numbers, and each link to them changes.
- Part IV becomes “Develop and deliver Ravel”.
- Chapter 1's “Reuse a component” route says that Ravel supports no separate
  component, and that a program uses the `ravel` CLI.
- Each reference to chapters 10–12 changes: in the book, the contract, the
  README, the release scripts and the Taskfile.
- If extraction starts again, that work adds its chapter and renumbers.

The facts that the human saw before the answer:

- The summary keeps an empty draft entry for chapter 10, so the sidebar
  shows it as a greyed item with no page.
- The book, the contract and the README hold 34 references to chapters
  10–12. Six release-script messages and two Taskfile comments name
  “chapter 12”. No figure names chapters 10–12.
- The contract says that undelivered work appears “never as features or
  plans”.
- No published page or outside link uses the chapter names yet.

<!-- tc: depart synonym-rotation because the quoted option label is the human's selection, kept exactly -->
**Shared understanding (2026-10-21).** The human selected “Confirm, file
issues (Recommended)” for the complete mandate. The mandate:

- The live leaf `adr-native-waitability-status-k191` stays first.
- A new book node, `ravel-book-without-chapter-ten`, starts with a `draft`
  leaf. That leaf converts the contract sections that it touches, then
  changes the contract. It removes chapter 10, renames the two chapter files
  and updates each reference and link. That covers the book, the README, the
  other documents, the six release-script messages and the two Taskfile
  comments. It also writes why Ravel supports no separate component. Each
  later stage adds the next one, and the proof covers the whole book.
- A new `impl` leaf, `release-notes`, follows the node. It writes the 1.1.0
  notes in `CHANGELOG.md` and runs `task release:doctor` and
  `task release:lifecycle:test`. It does not cut, push or tag a release.
- Two public issues: one in `Linkuistics/grove` for the removal of Grove's
  runner, and one in `Linkuistics/Ravel` for the delivery steps after the
  finish session.
- No new test seam. The book run uses the existing book tasks and a link
  check with a control. The script messages use `task release:lifecycle:test`
  and `task check`.
- No ADR, because each decision is easy to reverse, and no glossary change.

These questions stay open:

- the design of Grove's change
- whether a developer who is new to Ravel can use the book
- GitHub's view of the book
- native x86-64 Linux (`Linkuistics/Ravel#1`)
- the later iteration of the book's details

**Ravel issue #2 (2026-10-21).** After the shared understanding, the search
for duplicate issues found `Linkuistics/Ravel#2`. The owner opened it on
2026-10-18. It replaces the TypeScript owner policy with embedded Rhai
scripts and LLM calls through Rig, and it retires the TypeScript/Bun worker.
This session did not know about it when it asked question 1, and said so.

The human selected “Separate, after 1.1.0 (Recommended)”. The meaning:

- This workstream delivers 1.1.0 on the current TypeScript configuration.
  The root's rule that owner configuration stays unchanged still binds.
- Issue #2 becomes its own workstream after 1.1.0, with its own
  requirements. Its new configuration format is an incompatible change, so it
  normally needs a major release.
- The answers to questions 1–4 stand. Question 1 concerned k4's comparison
  of result and job completion, not the configuration language. Issue #2
  needs no extracted component and no persistent runner.

**Issues and leaves (2026-10-21).**

- This session filed <https://github.com/Linkuistics/grove/issues/22> and
  <https://github.com/Linkuistics/Ravel/issues/3>. A read-back showed each
  title and body. A search found no earlier issue for either subject, and a
  listing of Grove's issues showed that the search works.
- This session cut the node `ravel-book-without-chapter-ten-k196`, with
  `ravel-book-k197` (draft) and `release-script-chapter-names-k199` (impl).
  It cut `release-notes-k198` (impl) after the node.
- The root brief's Notes hold the outcome.
