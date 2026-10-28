# rhai-book-use-k78

## Goal

Proof chapters 1–6 as one document after the Read 1 correction run.

## Context

- Draft `rhai-book-use-k75`, copy-edit `rhai-book-use-k76` and art `rhai-book-use-k77` applied the owner's Read 1 patterns.
  Art also drew two new figures.
  Section 2 of `docs/book/contract.md` states the patterns, and “Figure needs” has a row for each new figure.
- The proof `rhai-book-use-k73` is the model for the coverage inventories, the gates and the viewing evidence.
- Read 1, `rhai-book-use-k74`, stays live. It resumes after this proof, with the owner's re-review from chapter 1.

## Done when

- The proof reads every chapter, table, example, caption and figure of chapters 1–6.
  It checks each pattern on every page, not only at the six passages of findings F1–F6.
- The coverage inventories come from the source again, as the contract requires.
- `task docs:examples`, `task docs:excerpts`, `task docs:figures` and `task docs:book:build` pass.
  Strict `docs:ste` and `docs:breaks` pass for chapters 1–6.
- A local link check passes. A planted broken target shows that the check can fail.
- The local book receives a visual inspection. The record names each viewing environment that the proof did not inspect.
- Each new lesson about documentation or communication is in `docs/documentation-lessons.md`.

## Notes

- Cut nothing. Read 1 already exists as `rhai-book-use-k74`, so do not cut a second Read 1 leaf.
- Read 2 remains required after the proof of Parts III and IV.

## Decisions (running log)

- The graph CLI could not start because a pre-coordination or unverified generation is active.
  This proof uses direct source reads and makes no graph-coverage claim.
- The whole-document read found summaries that omit exceptions stated in the detailed entries.
  Qualify notice timing for text and JSON, refusal and bound summaries for the final managed launch,
  and the failure-document entry for a partial final write.
  Carry the existing inheritance qualification into each repeated figure introduction.
- Strict STE of the figure layout records found semicolons in the three new figures,
  and a literal diagnostic read as prose. Replace the punctuation and describe the notice
  with its real labels; keep the complete literal diagnostic in the adjacent chapter example.
  Regenerate the SVGs and layouts before the final measurement.

## Coverage evidence

The complete source read covered chapters 1–6, including their tables, code
blocks, alternatives, captions and full-size image links. It checked the
Read 1 patterns throughout each chapter, including direct entries in chapter 6.
The proof retained useful repetition and the existing distinction between
fake-agent checks, recorded real harness calls and reader evidence.

The image references in the six sources give twelve distinct figures:
`system-context`, `routing-notice`, `inherited-result-path`, `process-authority`,
`successful-session`, `session-outcomes`, `evaluation-harness`,
`settled-exit-status`, `selection-cancellation`, `supervision-cancellation`,
`source-components` and `installed-files`.
Their Typst sources were read against the surrounding explanations, including
the keys, omitted phases and conditions on lifetime crosses.
Page-native tables carry the other mappings and sequences.

The inventories below were derived again from the source declarations and
the built help, rather than taken from the earlier proof's list.

| Inventory | Source-derived items | Coverage and limits |
|---|---|---|
| CLI inputs and outputs | `cli.rs` and the help of `ravel`, `start-session` and `end-session`: two verbs; start inputs `kind`, `prompt`, `prompt-file`, `task-file`, `task-id`, `config`, `param`, `context`, `timeout-ms`, `context-bytes`, `policy-env`, `confine`, `runtime-read`, `exit-dir`, `json`; end argument `RESULT`; help `-h`/`--help`; top-level version `-V`/`--version`. `refusal.rs`, `run.rs`, `main.rs` and `cancellation.rs` supply statuses 0, 1, 2, 3, 5, 124, 126, 127, 129, 130 and 143, and ten stages: cli, authority, worker, evaluation, load, context, validation, selection, resolution, exec. | Chapters 3–5 teach the use, and chapter 6 has the entries, missing-verb case, informational output, both notice schemas and mixed-stream rules. Corrected the JSON-failure entry for a possible partial final write. |
| Environment, within CLI coverage | `environment.rs` gives base helper reads HOME, PATH, TMPDIR, LANG and LC_*, explicit grants, nine reserved grant classes and six exact launch controls. `confinement.rs` adds retained USER/LOGNAME and assigns TMPDIR/TMP/TEMP. `completion.rs`, `run.rs` and `rhai_worker/harness.rs` supply RAVEL_EXIT_FILE and RAVEL_POLICY_RUN. | Chapters 4–6 distinguish helper access from inherited process environments and confined grants. Chapter 6 names the exact controls and classes and explains XDG_CONFIG_HOME. Direct environment-access searches also found test-only K28_NATIVE_ENTRY_MAIN, K28_BARRIER_DIR, campaign row controls, C4_RECEIPTS and RAVEL_ROLE_FIXTURE_BUILD; those are contributor fixtures, not installed inputs. Bun startup controls belong to the still-scheduled removal. |
| Configuration | `settings.rs`, `harness_entries.rs`, `context.rs`, `policy.rs` and `rhai_worker/engine.rs` supply the five settings keys, nested program/capabilities/harnessRuns fields, schema-3 exports and function arities, nine request fields, seven selected fields, four refused fields, six policy-context fields, assessment/source fields and attached measured records. `bridge.rs` registers both arities of the two reads, env, harnesses, remaining and run_harness; `helpers.rs` registers json_parse, json_encode, json_check, uuid_v4, path_resolve and path_parent. | Chapters 4 and 6 cover the shapes, unknown fields, errors, module and read bases, shipped examples and harness request/result/discovery fields. The loader runs when present even with caller context, and the request retains the original caller document. The default-entry and retired-format rules remain the agreed 2.0.0 design pending cutover. |
| Configuration bounds | `limits.rs`, `inputs.rs`, `completion.rs`, `rhai_worker/mod.rs`, `role.rs`, `reader.rs`, `host.rs`, `harness.rs`, `convert.rs` and `schema.rs` supply adjustable deadlines/context, input sizes, nine engine/heap limits, stack, imports, messages, diagnostics, read replies, JSON/schema depths, run output and stop bounds. | Chapter 6 includes each bound; chapter 4 teaches the adjustable ones and distinguishes per-run outcomes from fatal evaluation limits. Corrected the bounds summary and deadline row to retain the final-launch scope and cleanup/cancellation exception. |
| Lifecycle and failure outcomes | `session-execution.md`'s managed-process/result obligations, `result-job-completion.md`'s completion and precedence rules, and `policy-evaluation.md`'s harness/worker stop rules were compared with the source paths. | Chapters 1–6 distinguish first/empty/absent/Unicode/multiline results, natural exit and later nonzero exit, refusals, failed evaluation, deadlines, resolution, cancellation in both phases, escalation, cleanup, nesting, confinement, terminal recovery and accepted limits. Tables and summaries now keep the notice-time and refusal exceptions. The literal-code scan's two unmatched candidates were classified: context_source_unreadable belongs to the old TypeScript loader; stopped is internal worker stop data. Neither is a missing Rhai CLI entry. |
| Component obligations | The session specification and cited ADRs require caller-owned task meaning, separate evaluation without final-result authority, no fallback, caller execution context, first unchanged result, cleanup before success, cancellation suppression, terminal recovery, result-only standard output and the installed worker boundary. | Chapters 1–6 explain these at the use level. The 2.0.0 policy ADR/spec replaces the old installed pair with re-execution of one binary. Chapter 6 and the shared installed figure retain the delivery-owned notice-path dependency. Parts III and IV own the internal walkthrough and final delivery reconciliation. |
| Contributor workflows | `Taskfile.yml` and fresh `task --list` enumerate setup/deps, build/worker, typecheck/probes/check, nextest and focused/native campaigns, install/smoke, release, book checks, Rhai upgrade and both real harness smoke tasks. | Chapter 4 names the opt-in real-call tasks and their subscription use. Chapters 10–11 belong to the later node and must derive their final inventory after TypeScript removal. This proof makes no whole-book contributor-coverage claim. |

The existing release placeholder remains release-owned. TypeScript removal
still owns retired-format implementation and delivery layout. The Parts III
and IV node already owns the notice-path and chapter 8 shared-figure checks.
The document node has no remaining Handed forward entries. Read 1 is already
live as `rhai-book-use-k74`; no second checkpoint or editorial tail is cut.

## Viewing evidence

Safari in an isolated macOS 27.0.1 TestAnyware guest showed the rebuilt book
at `http://127.0.0.1:8789`, with a 1467-by-1000 display and the ordinary
750-pixel column. The viewed chapter regions were chapter 1's selection and
inherited-path entries, chapter 4's harness-run table/sequence and chapter 6's
JSON-failure entry/table. All twelve distinct figures were also inspected in
750-pixel HTML views, with scroll continuations for long diagrams. Their
labels and keys had no visible clipping or overlap. `docs/book/figures.md`
records the scope and limits. Screenshots are scratch files in `/tmp/k78-*.png`.

Host headless Chrome returned blank screenshots; they are not visual evidence.
The complete prose read used raw Markdown. No editor preview, pushed GitHub
Markdown, released Pages site, dark theme, narrow viewport or other operating
system was inspected. Not every rendered chapter region was visually inspected.
No real model call or owner reader task ran. The recipes agree with the exact
argument vectors and answer extraction in `rhai-harness-smoke-k66` and
`scripts/harness-smoke.sh`. Read 1 remains required, and Read 2 follows the
later proof of Parts III and IV.

## Verification

The final checks ran after all edits to their inputs ended, through `mise exec -- task`.

- `setup` verified the pinned checkout tools and development dependencies.
- `docs:examples`: 21 passed, zero failed or skipped, with each example's output
  and status compared separately. The 53 fragments were not executed.
- `docs:excerpts`: nine passed, zero failed or page errors.
- `docs:figures`: all seventeen SVGs and layout records matched their sources.
- Strict `docs:ste`: zero problems in chapters 1–6 and all seventeen layout records,
  twenty-three files in total.
- Strict `docs:breaks`: no prose run above 450 words in chapters 1–6.
- `docs:book:build`: passed without a warning.
- The local link/asset check passed 762 targets from the six rendered chapters.
  A known valid reference anchor passed; a planted absent anchor produced a
  missing-anchor failure in the same checker. The control file was removed.
  The checker also compared source headings with rendered heading ids on each
  chapter. This check covers local targets, not external websites.
- SHA-256 digests matched before and after the final checks for 3,601 tracked
  files outside `.grove/` and the scratch link/digest instrument.
  Subjects include the chapters, figures, documentation notes, source, fixtures,
  scripts, Taskfile, glossary, specs, manifests and locks. The manifest is
  `/tmp/ravel-proof-k78-subjects.json`, SHA-256
  `bd8e434e76f7b8228943cdee97844df6025dbd3a078f4aff8ab26e61fd59faf3`.
- The TestAnyware guest reported `stopped: true` after the visual inspection.

This proof changed prose and three figures, with their generated artifacts.
It changed no product code or example code. The new evidence extends lessons
L16 and L40 in `docs/documentation-lessons.md`. It establishes no new native
platform qualification, real harness call or owner task performance.

## Node close

`rhai-book-use-k26` meets its Done when after this correction proof.
Its draft, copy-edit and art predecessors are terminal, and no hand-forward
entry remains. No correction run is needed. Read 1 already stands outside the
node as `rhai-book-use-k74`, and Read 2 remains with the later whole-book proof.
The surviving delivery and release dependencies remain with their existing
owners. No live review names this proof or its document node.
