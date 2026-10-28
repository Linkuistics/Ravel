# configuration-names-k111


## Goal
Use `$HOME/.config/ravel/config.rhai` as the sole default Rhai entry while
preserving explicit arbitrary `.rhai` paths and the historical TypeScript refusal.



## Context
The parent contains the full rename contract. This child owns only default-path
discovery and its working consumers. Read the discovery and entry-precedence
sections of `docs/specs/policy-evaluation.md` and the Rhai entry ADR.

## Done when
- A real CLI test independently writes the new default and proves it runs.
- A lone old `policy.rhai` is not selected; explicitly naming it still works,
  including a relative path. Exact opaque result bytes remain intact.
- `config.rhai` wins beside the historical `policy.ts`; a lone `policy.ts` refuses
  with a remedy naming `config.rhai`. Dangling/unreadable new defaults do not fall back.
- Default-path test helpers, smoke checks, CLI help, glossary, specification,
  affected ADR and book/design references describe the new default.
- Focused CLI tests, `task check`, `task test`, book examples, exact excerpts and
  applicable smoke checks pass; record limits without claiming another platform.

## Notes
Leave environment grants, marker, JSON keys, diagnostic codes and filenames to
the later children. Do not modify frozen observations or owner-supplied values.

## Plan

1. Add CLI regression cases with literal default paths; run them against the old
   implementation and observe the expected failures.
2. Change discovery and its retired-format remedy. Preserve the literal historical
   `policy.ts` probe rather than deriving `config.ts` from the new default.
3. Reconcile the existing default-path helpers, smoke scripts, discovery contract
   and book consumers. Explicit arbitrary filenames remain valid.
4. Run focused and repository checks after all edits, inspect the diff, record
   evidence, retire this child and seal its focused jj change.

## Decisions (running log)

2026-10-28: The codebase-memory CLI refused `list_projects`: a pre-coordination or
unverified generation is active. No graph project or generation was usable.
Discovery evidence comes from direct reads of authority, CLI, sandbox helpers,
Rhai CLI tests and consumers. No graph completeness claim is made.

2026-10-28: Preserve the historical `policy.ts` diagnostic probe. It is a refusal,
not a selected default or alias. The new default has no fallback to `policy.rhai`.
Explicit `.rhai` basenames remain caller-controlled.

2026-10-28: Both new default-discovery tests failed against the original code:
one evaluated the old file instead of `config.rhai`, and the other launched a
lone `policy.rhai` instead of refusing. They passed after the discovery change.
The unreadable-entry check also covers a mode-000 file and a directory beside
both old names. Existing dangling-symlink and historical TypeScript cases remain.

2026-10-28: The first whole-suite run exposed an implicit default-file consumer
in the import-cycle fixture: its `import "policy"` now named a missing module.
Change that deliberate back-reference to `import "config"`, and keep its cycle
assertion. This updates the fixture's intended topology, not an arbitrary owner
value. The module-inventory expectation likewise now names `config.rhai`.

2026-10-28: One inspection-only fresh reviewer examined discovery, tests and
changed consumers and reported no concrete findings. This spends this session's
one review allowance. No second review is needed for the fixture correction,
which has the executable cycle assertion. The new exact-result test deliberately
retains `--policy-env` and `policy.rhai` inside an opaque result; later mechanical
renames must preserve those bytes.

2026-10-28: Final verification on aarch64 macOS passed:

- `mise exec -- task check`: formatting, ShellCheck, book-tool integrity,
  fake-harness smoke, Clippy and terminal recovery passed; nextest ran 605 passing
  tests with 48 skipped. The corrected cycle test, mode-000/directory refusal and
  frozen-comparison mutation control passed in that run.
- `mise exec -- task test`: independently ran 605 passing tests with 48 skipped.
- `mise exec -- task docs:examples docs:examples:test docs:excerpts`: 22 complete
  examples passed; 57 fragments were not executable examples; checker controls
  passed, and all nine exact excerpts passed.
- `mise exec -- task docs:book:build` and strict prose checks on the four edited
  chapters passed. The first prose run caught the new word “including”; the
  corrected sentence uses “such as”.
- `mise exec -- task smoke`: private-prefix native release installation passed
  direct and symlink checks for imports, context, parameters, exact/empty/multiline/
  flag-shaped results, retired formats and stale result context. No live
  installation or owner configuration was changed.
- `target/debug/ravel start-session --help` describes the new default.
  The diff under the frozen equivalence directory is empty.

The final check/test/document runs read unchanged product inputs. The inventory
`target/configuration-names-inputs` lists all 3,586 tracked files outside `.grove/`
(Rust/manifests/lockfile, scripts, book/spec/ADR sources, fixtures and historical
evidence). Per-file SHA-256 manifests before and after those runs compare equal;
their shared manifest digest is
`bed156c60810ced9771363a511d1a629439a18ebe29d94a8f205f87b1ee3beaf`.
Logs and manifests are disposable under `target/configuration-names-*`.
The final documentation run followed the fixture and prose corrections.

This is local macOS evidence. No Linux target/floor campaign or real signed-in
harness call ran. The remaining environment/output/file-and-prose rename criteria
belong to k112–k114, so k92 stays open. No new documentation/communication rule
was established beyond the existing lessons; no lesson entry is invented for a
mechanical default-path update.
