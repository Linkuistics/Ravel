# rhai-book-use-k82

## Goal

Apply the owner's Read 1 patterns to chapter 5, “Run sessions”.

## Context

- The node brief `rhai-book-use-k75` gives the findings and the facts that need a check against the source.
- Section 2 of `docs/book/contract.md` gives the ten patterns.
- Chapter 1 now explains the routing notice and how `end-session` finds its path (`rhai-book-use-k80`).
  Chapter 5 teaches both in full: notices in “Read notices separately from results”, and nesting in “Keep nested sessions separate”.
  Restate the short explanations there, and keep the terms that chapter 1 uses.

## Done when

- Chapter 5 follows each pattern in section 2 of the contract.
- `task docs:examples`, `task docs:excerpts` and `task docs:book:build` pass.
- `task docs:ste -- --strict` and `task docs:breaks -- --strict` pass for chapter 5.
- Each link into chapter 5 still resolves.
- Each new lesson about documentation or communication is in `docs/documentation-lessons.md`.

## Notes

- Keep the recorded dependency on the final notice path.
- Do not change chapters 1–4 or 6–11, except for a link that this draft breaks.

## Decisions (running log)

- **Structure brief, 2026-10-28.** The node brief names `docs/book/contract.md`. It states the reader (§1, “The reader”), the ordered section plan with chapter 5's row, and the emphasis and exclusions.
- **Facts checked against the source, 2026-10-28.** Without `--json`, the Ravel CLI writes the policy diagnostics and then one `eprintln!` line, `ravel: starting provider …`, before the launch. With `--json`, it writes no text line and keeps the notice until after supervision (`crates/ravel/src/run.rs`).
  Each managed launch removes the six names of `LAUNCH_CONTROLS` and sets a new `RAVEL_EXIT_FILE` for the channel (`environment.rs`, `run.rs`). A harness run gets its own path in place of `RAVEL_EXIT_FILE` (`rhai_worker/harness.rs`).
- **Real values, 2026-10-28.** The checkout build (`ravel 1.1.0`, Rhai path) ran the chapter's prompt example in `/tmp/ch5`, once in text form and once with `--json`. In the JSON run, the fake agent's `profile=careful` came before the notice. That order shows that the notice follows supervision.
  It also ran the chapter's nested policy with fake agents that print `RAVEL_EXIT_FILE`, with `TMPDIR=/tmp`. The outer path was `/private/tmp/ravel-session-DW2gTg/signal-c8008b48…`, and the inner path was `/private/tmp/ravel-session-47e8Ar/signal-803d525c…`. The outer agent had its own path again after the inner session.
  The runs also printed stop attempts with `Operation not permitted`. This session's sandbox denies those signals, so the chapter does not quote them or the `operationReports` member.
- **A checked notice example, 2026-10-28.** “Read notices separately from results” gets a new complete example with chapter 1's review values (`provider local`, `model fake`, `effort low`, `Reviews go to the fake agent.`).
  Its text run claims the notice line and the agent's next line, so the check fixes their order. Its `--json` run claims the `selection` member, and its refused run claims the whole failure document. A run in `/tmp/ch5r` gave these outputs, and the fragment of the full JSON notice comes from the same run.
- **Confinement tools introduced, 2026-10-28.** Chapters 1–4 do not name `sandbox-exec` or bubblewrap, so chapter 5 introduces both. `crates/keyed-launch/src/confinement.rs` gives a deny-by-default Seatbelt profile and a bubblewrap command with an empty mount namespace and no network namespace. The local `man sandbox-exec` marks the command as deprecated.
- **Shape of chapter 5, 2026-10-28.** The chapter opens with the reader's goal and the words of every page. Chapter 1's order table follows, with a column that names the section for each row.
  A new H2, “Run the examples of this chapter”, restates chapter 4's table of command parts. A closing H2, “Check your caller before real use”, summarises the chapter and lists the points to check.
  Each existing H2 keeps its title and anchor, and each now opens with the reader's goal. A table after each complete example ends with what the caller receives.
- **Terms explained at first use, 2026-10-28.** The words of every page are session, caller, kind, owner, owner policy, Rhai, managed process, harness, result and Ravel CLI.
  Later sections explain the fake agent, the three meanings of “context” and the policy worker.
  They explain a host capability, a host call, an LLM call and the evaluation budget.
  They also explain a process group, a reap, the result channel, the session-result context, a harness run and confinement.
  The last group is a measured read, a POSIX session, the loan of the terminal, a selection, the routing notice, the runner and a cancellation.
- **Corrections of my own draft, 2026-10-28.** A re-read against the source narrowed seven new sentences. The cleanup steps at a cancellation are attempts. Ravel, not the sandbox, makes the confined environment smaller.
  The sandbox profile and the bubblewrap view permit the grants of the table, and a POSIX session can share a controlling terminal. `end-session` finds only its own session's path, which is not a defence against the same user.
- **No in-session reviewer, 2026-10-28.** Each new fact came from the source or a real run. Three new claims put the notice and the refusal under the examples check. Copy-edit, art and proof follow.
- **Handed forward, 2026-10-28.** The F2 and F5 entries of the document node brief now give the chapter 5 place of each figure. The F5 entry also names the other random paths of chapter 5's real run.
- **Checks, 2026-10-28.** Evidence class: mechanical.
  `task docs:examples`: 21 passed, 0 failed, 0 skipped, 37 fragments not run. A changed character in each of the three new claims failed the check alone. `task docs:excerpts`: 9 passed. `task docs:book:build` had no error or warning.
  `task docs:ste -- --strict` and `task docs:breaks -- --strict` passed for chapter 5. The skill's prose checker reports no hard finding for chapter 5, the lessons record or the node brief.
  A script compared 157 links into and out of chapter 5, from all chapters and the README, with the ids of the built HTML. None failed. Two planted bad anchors failed the same comparison. The first version of the script falsely failed each same-page anchor, and a read of the built HTML found the defect.
- **Lessons, 2026-10-28.** `docs/documentation-lessons.md` has L55. L39 has a new rule sentence. The applications of L04, L06, L07, L09, L10, L16 and L39 name chapter 5.
- **Next stage, 2026-10-28.** Copy-edit `rhai-book-use-k76` already follows this node, so this draft cuts nothing. Draft `rhai-book-use-k83` remains live, so the node stays open.
