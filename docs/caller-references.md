# Caller-reference inventory

The active tracked tree is the scope of this inventory, including root manifests,
Taskfile, dotfiles, Rust/TypeScript, scripts, examples, tests, notices and docs.
Paths are freshly enumerated with jj file list; every file is read to enumerate
identifier/path tokens before classifying caller families. Generated builds,
development dependencies, logs, VCS internals and retired revisions are separate
inputs or provenance, not active product source. Repeat the inventory after edits.
The raw enumeration and exact line matches from this handoff are saved under
ignored target/k23-all-tokens.tsv and target/k23-caller-lines.txt.

| Surface | Classification |
|---|---|
| .cargo/config.toml | Forced-empty old/current launch, signal, completion, run-ID and state-control guards; intentional authority safety |
| keyed-launch tests/authority.rs and tests/support/mod.rs | Direct subprocess scrubs and regression tests of those guards |
| ravel src/environment.rs and src/run.rs | Exclude old dispatch authority grants and scrub inherited obsolete controls before publishing fresh child context |
| ravel tests/cli.rs | Temporary old/current authority sentinels verify exclusion; no real session channel is supplied |
| installed-smoke.sh | Explicit check that the isolated execution PATH contains no host runtime or source-project executables |
| docs/local-runner.md and docs/runtime-evidence.md | Exact source-project revision/URLs and historical experiment attribution; no executable dependency |
| docs/authority.md and this inventory | Explain the guard/provenance exceptions and this bounded sweep |
| docs/owner-configuration.md | Machine-local configuration handoff and caller-owned routing choices; no embedded policy or runtime registry |

There is no embedded caller adapter, provider-specific policy, task-kind registry,
review-creator lookup, task-tree walk, teardown interpretation or persistent history
implementation in the active Ravel boundary. SDK taskFile/taskId, kind, params and
context facts are opaque caller data. The generic routing sample uses owner-editable
wrapper/model/provider labels and open kinds. Result tokens carry no built-in meaning.
Retired commands named by a CLI rejection test are rejected inputs, not compatibility
verbs. Unknown storage-key rejection tests exercise the settings contract and do
not access old storage. No SQL dependency or storage fixture is transferred.

The textual instrument uses rg --hidden --no-ignore with explicit enumerated files,
including the Cargo guard dotfile. Positive controls find the known authority guard
and known historical source link. The identical expression/flags find live caller
references in the source project's caller adapter (cross-tree control). Deliberately
mutating the expression to miss those known subjects is observed to fail each
positive assertion before the genuine reading is credited. Removing a scratch
fixture's caller token likewise produces a clean reading, and adding it produces
a dirty reading. Controls are observations, not assumptions about exit code 0.

These are source/instrument claims, not graph-completeness claims. The Ravel graph
predates replacement source and excludes docs, so the inventory uses direct full
tracked-file reads. Build inputs and generated declarations/notices are separately
digested and checked by executable tests. The retained retired-project retrieval
document describes earlier history only; it authorizes no source or store revival.
