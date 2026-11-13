# rhai-book-inside-k35

## Goal

Draft the changes to chapters 7, 8 and 9 of the Ravel book. A reader learns the parts of Ravel 2.0.0, follows a session through its code, and understands jobs, signals and the terminal.

## Context

- The structure brief is `docs/book/contract.md`. The brief of this node lists the other inputs.
- Apply the owner's Read 1 patterns in section 2 of the contract on every page. The owner asked for them in the entire book on 2026-10-27.
  The running log of `rhai-book-use-k74` keeps the owner's findings F1–F16.
  Read the complete lessons record and the correction proof `rhai-book-use-k87`, which applies the accumulated lessons to chapters 1–6.
  Apply the same patterns here, including configuration terms, concrete cases before general explanations and descriptive headings.
- Chapter 7 explains the CLI and its separate worker. It describes a Bun worker now. Rewrite it for the worker as Ravel's own binary, the host bridge, the reader thread and the harness runs. An LLM call is a harness run, and Ravel has no LLM adapter.
- Chapter 8 follows a session through the code with 8 exact excerpts. The code has changed, so choose new excerpts of the Rhai path. Follow the successful, refused and cancelled paths.
- Chapter 9 covers jobs, signals and the terminal. The runner is unchanged, so edit only what the new host calls change: the harness runs and the ordered stop. `typescript-worker-cutover` already ported the policy of its complete example to Rhai. Read its running log for the examples of chapters 7 to 11 that it ported.
- Every complete example runs through the CLI test seam. Every excerpt is whole lines of a tracked file. Read `docs/book/examples.md` and `docs/book/excerpts.md`.
- If the leaf proves too big, decompose it by chapter.

## Done when

- Chapters 7, 8 and 9 describe the code that exists. Each claim is checked against the source.
- The excerpts of chapter 8 follow the Rhai path from `main` to the result write, and `task docs:excerpts` passes.
- Chapter 9 shows the ordered stop with a harness run in progress, and the precedence of a signal over the deadline.
- Chapters 7, 8 and 9 follow each Read 1 pattern of the contract.
- Each new lesson about documentation or communication is in `docs/documentation-lessons.md`.
- The leaf names the defects that a later stage owns in the `## Handed forward` list of the node brief.
- The last act cuts the `copy-edit` leaf, unless a live copy-edit leaf already stands below the node.

## Notes

- The figures wait for the art stage. Record which figure each chapter needs changed.
- Chapter 9 may need no change. State the check, and the result, in the running log.

## Decisions (running log)

**The delivered work satisfies this leaf, 2026-11-14.**
The node brief and the Read 1 log say that a later grove delivered the book work on `main`.
They ask each session to compare its task contract with that work before it repeats a draft. This session made that comparison.

The later grove drafted chapter 7 in `56dac027` (`ravel-book-inside-k329`), chapter 8 in `f6ee3135` (`k330`) and chapter 9 in `47c6422e` (`k331`).
It then copy-edited the chapters in `8e07b803` (`k309`), changed their figures in `7d9526e9` (`k310`) and proofed the whole book in `424af53d` (`k311`).
The proof record is `docs/evidence/ravel-book-k311/README.md`.

After `424af53d`, the three chapters did not change. Under `crates/`, only the workspace version in `Cargo.toml` changed, from 1.1.0 to 2.0.0.
The other changes are in the changelog, the README, chapters 3 and 11, two specs, release scripts and one Taskfile line. None of them reaches a statement in chapters 7–9.

| Done-when item | Delivered location and check |
|---|---|
| Chapters 7–9 describe the existing code | Drafts k329–k331. Proof k311 read every chapter against the source. The source of the chapters did not change after that proof. This session checked again that chapter 7's seven host calls match the dispatch in `crates/ravel/src/rhai_worker/host.rs`. |
| Chapter 7 teaches the worker as Ravel's own binary, the host bridge, the reader thread and harness runs | Sections “The Rhai worker evaluates the script in its own process” and “The host bridge carries each request to the Ravel CLI”. Bun, the SDK and `libexec` appear only in the statement of what 2.0.0 removed. An LLM call is a harness run, and Ravel has no model adapter. |
| Chapter 8's excerpts follow the Rhai path from `main` to the result write | 21 excerpts, from `main.rs` through `choice.rs`, `rhai_worker/` and `worker.rs`, to the final write in `run.rs`. Sections follow refusal and cancellation. `task docs:excerpts` reports 22 passed and none failed. |
| Chapter 9 shows the ordered stop with a harness run in progress, and the precedence of a signal over the deadline | Sections “Stop a harness run”, “Stop the evaluation at its deadline” and “Let a signal win over the deadline”, with complete examples at lines 413 and 550. |
| Read 1 patterns | Proof k311 checked each pattern of contract section 2 in all eleven chapters. |
| Lessons | Drafts k329–k331 added L73–L79. This session adds L82. |

Chapter 9 did change, against the leaf's expectation. Draft k331 measured the runner's diff and found the new captured launch mode for harness runs, as L78 records.

Figures: chapter 7 embeds `source-components`, `session-authority` and `host-bridge`. Chapter 8 embeds `session-code-path`, `selection-cancellation`, `supervision-cancellation` and `outcome-precedence`.
Chapter 9 embeds `escalation-timing`, `selection-deadline`, `supervision-cancellation` and `terminal-lending-recovery`. Art k310 made those figures, and `task docs:figures` reports that all 21 match their sources. No figure needs another change.

Fresh checks on this tree, with `mise exec -- task`:

| Check | Result |
|---|---|
| `docs:excerpts` | 22 passed, none failed |
| `docs:examples` | 27 passed, none failed. Six are in chapters 7–9. |
| `docs:figures` | 21 figures match their sources |
| `docs:ste` | No problems in 35 files |

This session changed no chapter. It hands nothing forward, because it found no defect that a later stage owns.

**The editorial tail is cut, 2026-11-14.**
The family rule forbids skipping a stage because a session judges that it will find nothing. Its one exception is a live later stage under the node, and none stands there.
The later grove's copy-edit, art and proof ran on this text, but they ran outside this node. Thus, this session cuts the `copy-edit` leaf. Its body tells the stage what exists, so the stage can compare before it repeats work.

The proof stage of this node also cuts Read 2, as the node brief requires. If the owner judges k309–k311 sufficient, the owner can prune the tail and cut Read 2 directly.
