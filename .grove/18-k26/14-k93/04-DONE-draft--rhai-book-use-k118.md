# rhai-book-use-k118

## Goal

Read chapter 6 against the full contract and lessons.
Then run the mechanical gates for all of chapters 1–6, and close node `rhai-book-use-k93`.

## Context

- The node brief `rhai-book-use-k93` gives the full requirements and the decomposition.
- Drafts `rhai-book-use-k115` to `rhai-book-use-k117` changed chapters 1–5. Chapter 6 quotes their examples in fragments.
- Derive the CLI and configuration inventories of contract §1 from the source and the built CLI.
- Draft `rhai-book-use-k116` named chapter 5's examples. The `--json` review command now belongs to the example `notices`.
  Its real-run fragment in chapter 5 now shows a new run in `/tmp/ravel-chapter-5/notices`, with `selectionMs` 3. The script bytes and digest did not change.
  Chapter 6 quotes the earlier run in `/tmp/ch5r`. Its values are still those of that run, but its paths and label differ from chapter 5's fragment.

## Done when

- Chapter 6's fragments agree with the current examples of chapters 1–5. Search the whole document for each changed example (L64).
- Each entry's files, arguments, settings, fields, diagnostics and help agree with the implementation and the inventories.
- Links and reference fragments agree across chapters 1–6.
- `task docs:examples`, `task docs:excerpts`, `task docs:figures` and `task docs:book:build` pass. Strict prose and break checks pass for chapters 1–6.
- Figure work is under “Handed forward” in `rhai-book-use-k26`. New lessons are in `docs/documentation-lessons.md`.
- The node `rhai-book-use-k93` closes, with copy-edit `rhai-book-use-k94` already standing.

## Decisions (running log)

- **Structure brief, 2026-10-29.** `docs/book/contract.md` §1 names the reader, the ordered section plan, the emphasis and the exclusions. Chapter 6 is row 6 of that plan, the lookup chapter.
- **Baseline, 2026-10-29.** Before any edit, `task docs:examples` passed 22 examples, with 60 fragments not run. The checkout build reports `ravel 1.1.0`.
- **Reserved grant classes, 2026-10-29.** Chapter 6 listed nine classes, with `BUN_*` and four `NODE_*` names. `environment.rs`, `excluded`, refuses only `LD_*`, `DYLD_*`, `RAVEL_*` and `HARNESS_DISPATCH_*`. Its unit test asserts that the `BUN_*` and `NODE_*` names are grantable. The help and the spec name the same four classes. The entry and the `excluded_grant` row now give four classes and the code's two reasons.
- **Old grant names, 2026-10-29.** `--policy-env` and `policyEnv` have no special handling. `tests/rhai_settings.rs` shows that they fail as an unknown option (`malformed_input`) and an unknown key (`settings_invalid`). The sentence that named them was an old-name explanation, so it goes (node brief).
- **Default entry, 2026-10-29.** The `config.rhai` entry said that Ravel does not select `config.rhai` as a default, which contradicts the line above it and `authority.rs`. That sentence survived from the old `policy.rhai` default. The `policy.ts` row stays: `authority.rs` still refuses that retired default by its exact name.
- **Tautology, 2026-10-29.** Two notes said that “the configuration script” in a diagnostic means the configuration script. They survived from an old-literal explanation. Each now says which parts Ravel supplies and which the script supplies, in chapter 5's words.
- **Fragments of chapter 5, 2026-10-29.** Three chapter 6 fragments quote the example `notices`. The script block is identical to chapter 5's. The JSON fragment now quotes chapter 5's recorded run in `/tmp/ravel-chapter-5/notices` (k116), with `selectionMs` 3. The labels name the example and its directory, and `<scratch>/work` gets chapter 5's explanation.
- **Chapter 6's own runs, 2026-10-29.** The checkout build ran the request, context, refusal, settings, discovery, harness-run and `end-session` fragments again in `/tmp/ch6`, with `TMPDIR=/tmp`. Each matched byte for byte, except `elapsedMs` (712, not 389), which the chapter says changes. A control with `--param repo=other` failed the request comparison. The settings and refusal labels now carry this date.
- **Code inventory, 2026-10-29.** Every snake-case string literal of the non-test source was compared with the code-font tokens of chapter 6. Only test data (`no_reviewer`, `session_name`, `spawn_missing`, `spawn_other`) and Rust or Rhai identifiers were absent. The chapter-only tokens are owner codes of example scripts and Rhai built-ins. A planted omission of `harness_cleanup_failed` fired.
- **Installed layout, 2026-10-29.** `stage_installation` in `scripts/release-common.sh` installs `bin/ravel`, three examples and `share/ravel/notices/THIRD-PARTY-NOTICES.md`. The table lacked the notice file, so it gets a row. `crates/ravel/examples/harness-table.rhai` is a test example that the installation omits. The entry now says so, because it names that directory. The installed-files figure has no notices directory; that goes to art.
- **Chapter 1's real run in chapter 6, 2026-10-29.** The opening said that “the LLM found a problem” in chapter 1's run, and two notice labels said “the review session in chapter 1”. That run used a fake agent, which explains `local` and `fake`. The opening and both labels now name the script and the fake agent, in chapter 1's words (L71).
- **Interface inventory, 2026-10-29.** `ravel --help` matches the chapter's fragment byte for byte. `start-session --help` lists the fifteen options that the chapter counts. `settings.rs` accepts the five keys of the settings table. `limits.rs` and `rhai_worker/mod.rs` hold each value of the bounds and engine tables. The environment names that the non-test source reads are all in the chapter. `RAVEL_ROLE_FIXTURE_BUILD` is read only under `cfg(test)`.
- **Hand-forward, 2026-10-29.** The installed-files figure entry is under “Handed forward” in `rhai-book-use-k26`, for art `rhai-book-use-k95`. No other figure source of chapters 1–6 has an old name; the same count found two in `contract.md` as a control.
- **Checks, 2026-10-29.** `task docs:examples`: 22 passed, 0 failed, 0 skipped. `task docs:excerpts`: 9 passed. `task docs:figures`: 18 figures match. `task docs:book:build` passed with no warning or error.
  Strict `docs:ste` and `docs:breaks` passed for chapters 1–6. A planted semicolon in a copy of chapter 6 failed the strict prose check.
  A check of the built chapters 1–6 resolved all 613 local links and anchors, found no raw Markdown, and found an id on each heading. A planted broken anchor, a planted fence and a planted heading without an id each failed it.
  The digests of chapters 1–6 did not change during the gate run. The final chapter 6 has SHA-256 `115b8e773efd…`, unchanged across the last example run.
- **Lessons, 2026-10-29.** L63 now records the old-name explanations that hold only new tokens. L72 records the stale reference list of a removed component. The book checker reports the same findings for the lessons file as for the parent revision.
- **No in-session reviewer, 2026-10-29.** Each correction states a fact that a source line or a fresh run settles, and each has its source in this log. Copy-edit `rhai-book-use-k94` reads the changed sentences next.
