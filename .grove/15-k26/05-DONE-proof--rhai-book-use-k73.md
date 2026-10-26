# rhai-book-use-k73

## Goal

Read Parts I and II as one document and complete the proof charter for chapters 1–6.
Use `docs/book/contract.md` and the node brief's remaining handoffs.

## Context

- Draft and copy-edit describe the Ravel 2.0.0 design before the scheduled TypeScript removal.
  Keep that dependency explicit. The current checkout is not the finished release.
- Art `rhai-book-use-k72` updated seven figures with the existing Typst/CeTZ style.
  Its task file records source checks, viewing evidence, declined figures and validation.
- The process-authority view omits evaluation harness runs explicitly.
  The source-components view shows them, and the selection-cancellation sequence shows their cleanup before worker teardown.
  Harness-run data reaches the policy. It cannot complete the final session.
- The successful sequence follows chapter 3's policy, which makes no host call.
  Its former change to `/` is gone.
- The final-outcome key distinguishes failed harness cleanup during evaluation from later cleanup after an original failure.
- Chapter 6 shows one executable and three editable Rhai examples.
  The third-party notice path still belongs to the delivery-removal increment.
- Chapter 8's old cancellation example and alternative await its already scheduled draft.
  The Parts III and IV node brief records the mismatch with the shared figure.
  Do not expand this proof into that document's rewrite.

## Done when

- The proof reads every chapter, table, example, alternative, caption and linked figure in Parts I and II.
- Coverage inventories come from the source, as the contract requires.
  The remaining node handoffs close or pass to their named owners.
- The applicable book gates pass, including examples, excerpts, figures, strict prose checks and the build.
- The local book receives a visual inspection. State which other viewing environments remain unavailable or uninspected.
- Cut the required Read 1 requirements leaf after proof, with the bare document stem.
  The owner uses Read 1 to write a fake-agent policy and run a session.
  The owner interprets refusal and cancellation, then uses a direct reference entry.
  Both reader checkpoints remain required. This proof does not replace the owner's read.

## Decisions (running log)

- The graph CLI refused its active generation before it could list projects.
  Source checks use direct reads. No graph coverage or completeness claim follows.
- Chapter 4's deadline summary now states the cancellation and harness-cleanup exception.
  Chapters 5 and 6 already state it. This keeps the teaching summary consistent with the reference and final-outcome figure.
- Preserve the release placeholder and the scheduled format cutover.
  The proof checks the Rhai examples in this checkout, not a completed 2.0.0 installation.
  Send the notice-path check to delivery removal and the exact environment-scrub correction to spec reconciliation.
  The Parts III and IV brief already owns chapter 8's shared-figure reconciliation.
- Read 1 will be a root sibling immediately after this document node, before TypeScript removal.
  This keeps the reader checkpoint outside the editorial pipeline and before dependent drafting.
  It carries the bare document stem. Both owner reader checkpoints remain required.

## Coverage evidence

The source read covered chapters 1–6, with every table, example, caption and alternative.
The figure inventory came from the image references in those chapter sources.
It contains `system-context`, `process-authority`, `successful-session`, `session-outcomes`,
`settled-exit-status`, `selection-cancellation`, `supervision-cancellation`, `source-components` and `installed-files`.
The read checked their labels and keys against the surrounding explanation.

The five contract inventories were derived from the subjects below.
The check concerns Parts I and II and records the remaining delivery and contributor dependencies explicitly.

| Inventory | Source and derived items | Coverage and limits |
|---|---|---|
| CLI inputs | `cli.rs` and the built help of all three command lines. Verbs: `start-session`, `end-session`. Start options: `kind`, `prompt`, `prompt-file`, `task-file`, `task-id`, `config`, `param`, `context`, `timeout-ms`, `context-bytes`, `policy-env`, `confine`, `runtime-read`, `exit-dir`, `json`. End argument: `RESULT`. Help: `-h`, `--help`. Top-level version: `-V`, `--version`. | Chapters 3–5 teach their use. Chapter 6 has each entry, the missing-verb case, stream rules and informational output. |
| CLI outputs | `refusal.rs`, `main.rs`, `run.rs`, cancellation handling and the completion specification. Statuses: 0, 1, 2, 3, 5, 124, 126, 127, 129, 130, 143. Ten stages: `cli`, `authority`, `worker`, `evaluation`, `load`, `context`, `validation`, `selection`, `resolution`, `exec`. | Chapters 5 and 6 agree on statuses, signals, partial final writes and mixed standard error. Chapter 6 separates routing-notice schema 2 from failure schema 1 and lists their fields. |
| Environment | `environment.rs`, `confinement.rs`, `run.rs`, `authority.rs`, `settings.rs` and `rhai_worker/harness.rs`. Base reads: `HOME`, `PATH`, `TMPDIR`, `LANG`, `LC_*`, plus grants. Confined inheritance adds `USER`, `LOGNAME`; temporary assignments are `TMPDIR`, `TMP`, `TEMP`. The six exact launch controls and nine reserved grant classes were enumerated from their source definitions. `RAVEL_POLICY_RUN` is an additional harness marker. `RAVEL_EXIT_FILE` is replaced per launch. | Chapters 4–6 distinguish helper grants from process inheritance. Chapter 6 lists every class and exact control name. `XDG_CONFIG_HOME` selects no configuration. The test-only `K28_NATIVE_ENTRY_MAIN` and receipt variable `C4_RECEIPTS` belong to contributor fixtures. Bun-only worker controls remain with the scheduled removal. The spec's broad scrub claim is assigned to `rhai-spec-reconciliation-k33`. |
| Configuration | `settings.rs`, `harness_entries.rs`, `limits.rs`, `policy.rs`, `context.rs`, Rhai engine, bridge, helpers, resolver and harness code; `policy-evaluation.md`. Settings: `timeoutMs`, `contextBytes`, `policyEnv`, `harnesses`, `budgets`. Nested fields: `program`, `capabilities`, `harnessRuns`. Module exports: schema 3, version, `select`, optional `load_context`. Request fields: schema, kind, prompt, cwd, params, taskFile, taskId, context, limits. Selected and refused fields came from their source arrays. Context fields came from `FIELDS`, `SOURCE_FIELDS` and `ASSESSMENT_FIELDS`. | Chapters 4 and 6 cover the shapes, unknown fields, bounds, import bases, measured records and examples. The default-entry and retired-format table describes the agreed 2.0.0 design. `typescript-worker-cutover-k31` still supplies that behaviour. |
| Host operations and bounds | The bridge registers `read_text`, `read_json` with both arities, `env`, `harnesses`, `remaining`, `run_harness`. Helpers register `json_parse`, `json_encode`, `json_check`, `uuid_v4`, `path_resolve`, `path_parent`. `Limit::ALL` supplies nine engine/heap limits. Other definitions supply module, source, protocol, diagnostics, JSON/schema depth, worker stack, harness output and stop bounds. | Chapter 6 includes each operation, its arguments and errors, discovery fields, remaining budgets, the harness request and every result field/status. Chapter 4 teaches their use. The Claude Code and Codex recipes match the argument vectors, prompt placement and answer extraction recorded in `rhai-harness-smoke-k66`. No new real call ran. |
| Lifecycle and component obligations | `session-execution.md` supplies the result/job obligations. `result-job-completion.md` supplies failure precedence and recovery limits. `policy-evaluation.md` supplies evaluation authority, host stop order and cleanup exceptions. The Rhai ADRs supply the worker design. | Chapters 1–6 distinguish sent and absent values, first result, empty/multiline/Unicode values, natural exit, later nonzero exit, refusals, context errors, worker failure, deadlines, resolution, both cancellation phases, nested sessions, confinement, escalation, terminal recovery and accepted limits. The chapter 4 deadline summary needed the exception now added. Internal mechanics remain with chapters 7–11. |
| Contributor workflows | The actual `task --list` and `Taskfile.yml` supply setup, build, worker/deps/typecheck/probes, checks, focused tests, native campaigns, install/smoke, release, book checks, Rhai upgrade and real harness smoke tasks. | Chapter 4 names the two opt-in harness smoke tasks and their cost. Chapters 10–11 own the contributor inventory after delivery removal changes it. `rhai-book-inside-k34` must derive that final inventory again. This proof makes no whole-book contributor-coverage claim. |

The final third-party-notice path remains a delivery decision.
`typescript-delivery-removal-k32` now records the required handoff to `rhai-book-inside-k34`.
That node owns the chapter 6 table and shared installed figure after the path settles.
Its brief also retains the chapter 8 cancellation example and alternative correction.
The proof cleared every entry from this node's hand-forward list.

## Viewing evidence

Chrome on macOS displayed the local book at `http://127.0.0.1:8788` on 2026-10-26.
The viewport screenshot was 1467 by 1000 pixels, with a chapter column of about 750 pixels.
The inspection covered the chapter 4 recipe table, the chapter 6 installed-layout entry,
the process/source/installed views and the success, outcome and cancellation figures.
Full-size SVG views exposed the sequence order and figure keys.
No missing image, clipped label or overlapping label was visible in the inspected regions.
The ordinary view used the light theme and 100% zoom; a reduced success-sequence overview also ran, then zoom returned to 100%.

Raw Markdown supplied the complete prose, table, caption and alternative read.
No editor preview, dark theme, narrow viewport, pushed GitHub source or released Pages site was inspected.
No instrumented browser connection was available. The local inspection used the native Chrome surface.
The TestAnyware doctor reported a stale app configuration and an inactive login item; this proof changed no machine configuration.
These observations establish no new platform qualification, real-model result or reader comprehension.
Read 1 still needs the owner's own task performance, and Read 2 remains required after the later whole-book proof.

## Verification

- `mise exec -- task setup` completed with the pinned checkout tools.
- `mise exec -- task docs:examples` passed all 19 examples, with no failure or skip.
  The final run compared each example's output and status, not only the total.
- `mise exec -- task docs:excerpts` passed all nine excerpts.
- `mise exec -- task docs:figures` reproduced all fourteen figures and layout records.
- Strict `docs:ste` passed chapters 1–6 and all fourteen figure layout records: zero problems in twenty files.
- Strict `docs:breaks` passed chapters 1–6: no prose run exceeds 450 words.
- `mise exec -- task docs:book:build` passed without a warning.
- A local rendered-link check examined 635 link and asset targets from chapters 1–6, including fragment identifiers, with zero failures.
  Existing reference anchors and an absent planted anchor gave different results.
  This check covers local targets only, not external websites.
- The final prose, build, examples, excerpts and figure checks ran after edits to their inputs ended.
  SHA-256 digests remained unchanged for all 3,591 tracked files outside `.grove/`.
  This subject set includes the book, scripts, Taskfile, glossary, specifications, manifests, lockfiles and source fixtures.
  The scratch digest manifest is `/tmp/ravel-proof-k73-subjects.json`, SHA-256 `7642c3e4f927c8d79eb488a0af0993e21dd2b97e959ab38ec72d79ec8eaf5c31`.
- No Rust code, test, figure or example code changed. The proof changed one prose summary and the task-tree obligations.
  No new real harness call, platform campaign or owner reader checkpoint ran.
