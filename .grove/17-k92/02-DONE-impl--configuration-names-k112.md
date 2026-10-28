# configuration-names-k112


## Goal
Rename environment grants to `--config-env` and `configEnv`, and the evaluation
marker to `RAVEL_CONFIG_RUN`, through every current producer and consumer.



## Context
Follows default discovery in `configuration-names-k111`. The parent owns the
complete contract. Inspect CLI parsing, settings, environment, harness requests,
worker/front and confinement consumers before changing names.

## Done when
- Both flag and settings grants work through the real CLI fake-harness seam.
- `--policy-env` and `policyEnv` refuse rather than alias; old-input test cases
  remain exact. Tests cover duplicate/reserved grants and unchanged authority.
- `RAVEL_CONFIG_RUN` causes re-entry refusal before evaluation; it is propagated
  only to harness runs and scrubbed where required. Stop order remains unchanged.
- Update current documentation, examples, smoke scripts and checks that consume
  these names. Keep exact excerpts and examples executable.
- Relevant tests, `task check`, `task test`, examples and excerpts pass.

## Notes
Do not change the installed driver, its ending command or live settings. JSON
output names and remaining artifact filenames have later children. Record graph
availability and test/platform limits.

## Plan

1. Update CLI seam tests to use the new grants and marker; add old-spelling
   refusal cases and verify they fail before the implementation changes.
2. Rename the CLI/settings grant fields and harness-run marker through their
   producers and consumers. Keep authority, cleanup and output schemas unchanged.
3. Reconcile current documentation, smoke tooling and generated figure consumers.
4. Run focused tests, repository checks, examples, excerpts and the book build;
   record evidence, retire this leaf and seal its focused jj change.

## Decisions (running log)

2026-10-28: `codebase-memory-mcp cli list_projects` refused because a
pre-coordination or unverified generation is active. No project/generation is
usable; discovery uses direct source reads and literal searches. No graph
completeness claim is made.

2026-10-28: The parent brief already settles the interface: accept only
`--config-env`, `configEnv` and `RAVEL_CONFIG_RUN`. This is a mechanical rename,
with no new design choice or compatibility alias. Preserve opaque result bytes
that mention old names, frozen comparison observations and historical evidence.
JSON output names and diagnostic codes remain for k113; filenames remain for k114.

2026-10-28: Before production edits, six focused CLI seam tests failed as
expected: the new settings/flag names were unknown, the new marker was absent
from a harness run and did not preempt malformed settings, and the retired flag
still launched. A second red run puts the retired settings case first to expose
its independent acceptance and checks marker propagation under confinement.
Existing opaque-result literals containing old names stay byte-exact.

2026-10-28: The first focused run exposed a substring replacement that renamed
the routing notice's `policyEnvironment` while changing `policyEnv`. Restore that
output field and its reference text: k113 owns the serialized-output rename.
The new ordered/deduplicated-grants assertion caught the unintended change.
The production diff otherwise only renames grant fields, labels and marker
producers/consumers; the six launch controls and stop ordering stay unchanged.

2026-10-28: All 120 focused CLI tests pass (one ignored test is skipped).
They cover settings and flag grants, duplicate ordering and value privacy,
reserved/malformed names, both retired-spelling refusals in text and JSON,
empty/nonempty marker refusal before settings, marker propagation to confined
and unconfined harnesses, absence from the managed process, cancellation and
cleanup. This mechanical rename needs no fresh-context review: executable
boundary checks supply the relevant evidence, and k114 retains the whole-node
reconciliation. No framework or dependency API decision was introduced.

2026-10-28: Final verification on aarch64 macOS passed:

- `mise exec -- task check`: formatting, ShellCheck, book-tool integrity,
  fake-harness smoke, Clippy and terminal recovery passed. Nextest ran 607
  passing tests with 48 skipped, including the frozen-comparison mutation
  control and existing stop/cancellation/cleanup tests.
- `mise exec -- task test`: independently ran 607 passing tests with 48 skipped.
  The passing test identities, extracted from both logs, compare equal item by
  item; this conclusion does not rely only on their totals.
- `mise exec -- task docs:examples docs:examples:test`: all 22 complete examples
  passed, with no failures or skipped examples; 57 fragments were not executable
  examples. The example-checker controls passed.
- `mise exec -- task docs:excerpts`: all nine exact excerpts passed.
- `mise exec -- task docs:book:build`, strict prose checks on the four edited
  chapters, and `mise exec -- task docs:figures` passed. All 18 figures match
  their sources. The changed harness-run figure was regenerated through
  `task docs:figures:update`, not hand-edited.
- `RAVEL_CONFIG_RUN=1 mise exec -- task harness:smoke:test` passed with fake
  harnesses, exercising the smoke script's removal of the renamed inherited
  marker. No signed-in harness call ran.
- The built `start-session --help` advertises `--config-env` and `configEnv`.
  The frozen equivalence directory has no diff.

The exact old-token search covered the repository, including hidden files,
outside VCS metadata, `target/` and `.grove/`. Its residuals are the released
1.1.0 changelog, a historical admission transcript, the explicit old-input
rejection tests and documentation, and the opaque-result byte-preservation
case. `RAVEL_POLICY_RUN` also remains a reserved-class rejection input. A scratch
control with the new tokens read clean, then the same command and flags found
each deliberately inserted old token. Historical and rejection surfaces supply
the cross-tree control. This is a bounded grant/marker audit; k114 still owns
the complete old-terminology inventory.

The final checks read unchanged product inputs. `target/k112-inputs` enumerates
all 3,586 tracked files outside `.grove/`, including source, manifests, lockfile,
scripts, documentation, fixtures and historical evidence. Per-file SHA-256
manifests before and after the full checks and final smoke compare equal; their
shared manifest digest is
`d29e125f93f1f70f91efb8f0de76be16226e0aec6f930351d90f7a93d026a249`.
Logs, inventories and manifests are disposable under `target/k112-*`.

This is local macOS evidence. No Linux target/floor campaign or real signed-in
harness call ran. No installed executable or live owner configuration changed.
The JSON/output and remaining filename/prose work stays with k113 and k114, so
k92 remains open. No new documentation/communication rule was established by
this mechanical rename; no lesson entry is invented.
