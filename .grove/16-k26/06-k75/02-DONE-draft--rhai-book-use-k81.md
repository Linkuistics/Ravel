# rhai-book-use-k81

## Goal

Apply the owner's Read 1 patterns to chapter 4, “Write an owner policy”.

## Context

- The node brief `rhai-book-use-k75` gives the findings and the facts that need a check against the source.
- Section 2 of `docs/book/contract.md` gives the ten patterns.
- Chapter 1 now introduces Rhai, a selection, the routing notice and `RAVEL_EXIT_FILE` (`rhai-book-use-k80`).
  Chapter 4, line 6, repeated the one-sentence description of Rhai that F1 names. Restate the fuller introduction of chapter 1 there, in short.
- F4 applies most here: put the reader's goal before each capability, such as a harness run or a measured read.

## Done when

- Chapter 4 follows each pattern in section 2 of the contract.
- `task docs:examples`, `task docs:excerpts` and `task docs:book:build` pass.
- `task docs:ste -- --strict` and `task docs:breaks -- --strict` pass for chapter 4.
- Each link into chapter 4 still resolves.
- Each new lesson about documentation or communication is in `docs/documentation-lessons.md`.

## Notes

- Do not change chapters 1–3 or 5–11, except for a link that this draft breaks.

## Decisions (running log)

- **Structure brief, 2026-10-27.** The leaf's chain names `docs/book/contract.md`. It states the reader (§1, “The reader”), the ordered section plan with chapter 4's row, and the emphasis and exclusions.
- **Facts confirmed against the source, 2026-10-27.** The engine runs the top-level statements, then `load_context` when it exists, then `select` (`crates/ravel/src/rhai_worker/engine.rs`, `run`).
  The Ravel CLI starts each harness run when the worker asks for it, with a result channel of its own in place of `RAVEL_EXIT_FILE` (`rhai_worker/host.rs`, `rhai_worker/harness.rs`).
  Settings always come from `$HOME/.config/ravel/settings.json`, also with `--config` (`settings.rs`, `SETTINGS_FILE`).
  A notice of schema 2 has a `harnessRuns` member, and each record has `harness`, `status`, `exitCode`, `signal`, `elapsedMs` and `sentResult` (`run.rs`, `RunRecord::to_json`).
- **Real values, 2026-10-27.** The checkout build (`ravel 1.1.0`, Rhai path) ran the chapter's Claude Code example with `--json` in a private scratch home.
  Its `harnessRuns` member is the fragment in “Run a harness during evaluation”. The `elapsedMs` value changes in each run.
  The run also printed stop attempts with `Operation not permitted`. This session's sandbox denies those signals, so the chapter does not quote them.
- **Refusal and parse failure as exact claims, 2026-10-27.** A refusal launches no program, so its standard error has no stop-attempt lines.
  The refused commands of “Select a command or refuse” and a new parse-failure example claim the whole standard error. Thus, `task docs:examples` checks each real line that the chapter explains.
- **Shape of chapter 4, 2026-10-28.** The chapter opens with the owner's goal and the four terms of every page. Three new H2 sections follow: “Know the Rhai language”, “Follow the evaluation of a policy” and “Run the examples of this chapter”.
  “Know the Rhai language” restates chapter 1's characteristics and links. A preview table gives each section and why the reader may want it. A closing section, “Review your policy before real use”, summarises the chapter and lists the points to test.
  Each existing H2 keeps its title and anchor. Each now opens with the reader's goal, and order tables follow the examples to the caller's result.
- **Terms explained at first use, 2026-10-28.** Session, caller, kind, owner, owner policy, policy worker, selection, refusal, routing notice, host capability, measured read, harness, harness run and managed process.
  Also reap, process group, result channel, POSIX session, confinement, evaluation budget, structured output and the three meanings of “context”.
- **Three reasons removed, 2026-10-28.** No source states why imports and measured reads use different bases, why Ravel captures policy output or why the recipes send the prompt on standard input.
  The text states each difference and its effect instead. The specification sections “Measured reads” and “Re-entry and completion authority” and the log of `rhai-harness-smoke-k66` were read. Lesson L54 records the rule.
- **Two corrections against the source, 2026-10-28.** The old text said that the routing notice reports module measurements. Only the JSON form does, in `policy.modules`.
  The draft's first order of a harness run put the budget check before the lookup. `host.rs` resolves the program first, so the table follows that order.
- **New complete example, 2026-10-28.** “Validate a policy with fake agents” has one example with `print` and a parse failure. The parse failure claims the whole standard error. The `print` command claims only its stable line, because stop-attempt lines vary.
- **No in-session reviewer, 2026-10-28.** Each new fact came from the source, the specification or a real run, and two exact claims put real values under the examples check. Copy-edit, art and proof follow.
- **Handed forward, 2026-10-28.** The document node brief gives art the figure of a harness run during evaluation, with its place, order and real values.
- **Checks, 2026-10-28.** Evidence class: mechanical.
  `task docs:examples`: 20 passed, 0 failed, 0 skipped, 35 fragments not run. A changed character in the exact refusal claim failed the check. `task docs:excerpts`: 9 passed. `task docs:book:build` had no error or warning.
  `task docs:ste -- --strict` and `task docs:breaks -- --strict` passed for chapter 4. The skill's prose checker reports no hard finding for chapter 4, the lessons record or the node brief.
  A script compared 184 links into and out of chapter 4, from all chapters and the README, with the ids of the built HTML. None failed. A planted bad anchor failed the same comparison.
- **Lessons, 2026-10-28.** `docs/documentation-lessons.md` has L54. The applications of L04, L08, L09, L13 and L21 name chapter 4.
