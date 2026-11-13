# CLI inventory for rhai-book-cli-inventory-k125

This is the source-derived CLI inventory for the 2.0.0 checkout, checked on
2026-11-14 on native Darwin arm64. It supplies one input to the later whole-book
proof, rhai-book-inside-k130. It does not establish reader comprehension, owner
Read 2, Linux qualification, a published render or real paid-agent behaviour.
Historical evidence under `ravel-book-k311` is unchanged.

## Derivation and retained measurements

Start with `crates/ravel/src/cli.rs`: `Cli`, `Command`, `SkillsCommand`,
`RunArgs` and `SelectionArgs`. Follow `main::main` into `run::run`,
`completion::end_session` and `skills::command::run`. Check `inputs::Inputs::read`,
`choice::prepare`, `limits::Limits::read`, environment grant checking and notice
serialisation. Follow the launch environment into the worker, the final managed
process and evaluation harness runs. Search runtime environment operations in
both crates, distinguishing production code from `cfg(test)` fixtures and
compile-time build identity. This enumeration starts from source definitions;
book headings supply the destinations after enumeration.

Tier 2 graph discovery used exact CLI symbols, the outbound depth-one main trace
(11 results, complete), and narrowed function searches (33 results, complete).
Coverage checked all cited source paths and both source scopes with no recorded
gap, metadata matching generation `2026-11-13T15:30:27Z`. That is a best-effort
signal, not proof of completeness; direct source reads and the environment
literal sweep supplied the cross-check.

`help.json` retains argv, status and separate stdout/stderr for 23 probes, plus
the SHA-256 of the freshly built `target/debug/ravel`. `mise exec -- task build`
completed successfully before capture. The probes cover all five help positions
and their short forms, both version forms, missing verbs, unsupported help and
nested versions, malformed JSON requests and both withdrawn options. They never
call `end-session` against the enclosing author's live channel. Help stdout and
bare-command stderr were compared with the book. The reference now includes
bare `skills`, its help position and the current root help wording.

`subjects.sha256` freezes both crates' Rust sources, workspace manifests and
lockfile, toolchain, Taskfile, book contract and book source before validation.
`checks.log` records commands and their statuses; `subjects-after.sha256` checks
that those subjects did not change during validation. The retained help capture
is a one-time measurement, not a new supported task runner. Runtime environment
and system libraries are not fully frozen. Source/test/build files in the
manifest identify the measured checkout; listing a subject does not claim that
every test or every book chapter was read in this inventory session.

## Command surface

All book destinations below are in `docs/book/src/06-reference.md`, under the
named heading. Its links lead to complete tasks in chapters 3–5. CLI definitions
are in `crates/ravel/src/cli.rs`; input checks are in
`crates/ravel/src/inputs.rs` and bounds in `crates/ravel/src/limits.rs`.

| Source item | Contract and explanatory destination |
|---|---|
| Root `ravel` | Exactly `start-session`, `end-session`, `skills`; bare root prints help to stderr, status 2. “ravel, help and version”. |
| Root `-h`, `--help`; `-V`, `--version` | Informational stdout/status 0. Version is top-level only; no `help` subcommand. “ravel, help and version”. |
| Each command's `-h`, `--help` | Includes the `skills` wrapper and `skills resolve`; stdout/status 0. Bare `skills` requires `resolve`, stderr/status 2. Same heading. |
| `start-session` | Route, prepare skills, evaluate and supervise one selection. “start-session”, including the complete option table. |
| `--kind TEXT` | Required nonempty opaque caller value, no inference from files. “--kind TEXT”. |
| `--prompt TEXT`, `--prompt-file PATH` | Exactly one; UTF-8, at most 1 MiB; file relative to caller cwd, read once; no stdin prompt. Hyphen-looking inline text remains data. “--prompt TEXT and --prompt-file PATH”. |
| `--task-file PATH`, `--task-id ID` | Optional data; joined file path need not exist or supply identity; identity nonempty UTF-8, at most 1024 bytes, accepts hyphen-looking text. “--task-file PATH and --task-id ID”. |
| `--config PATH` | Nonempty explicit Rhai entry replaces `$HOME/.config/ravel/config.rhai`, relative to caller cwd. “--config PATH”. |
| `--param NAME=VALUE` | Repeatable; split first `=`, nonempty name, UTF-8 values, unique names, combined names/values at most 65536 bytes. “--param NAME=VALUE”. |
| `--context PATH` | Version-1 JSON data, relative to caller cwd; measured and validated before evaluation. “--context PATH”. |
| `--timeout-ms MS` | Decimal 1000–600000; default 30000; flag overrides settings, limits selection rather than final managed lifetime. “--timeout-ms MS”. |
| `--context-bytes BYTES` | Decimal 1–8388608; default 262144; overrides settings; refuse rather than truncate. “--context-bytes BYTES”. |
| `--config-env NAME` | Repeatable additional grants, deduplicated with settings; reserved names refuse; reports name/set, never value. “--config-env NAME” and environment sections below. |
| `--exit-dir DIR` | Existing writable parent of private result channel; default system temporary directory. “--exit-dir DIR”. |
| `start-session --json` | Routing/failure documents on stderr, no change to opaque stdout. “--json”, “JSON routing notice”, “JSON failure document”. |
| Withdrawn `--confine[=VALUE]`, `--runtime-read[=FILE]` | Hidden, optional-value/repeatable parse compatibility; first parsed flag refuses `confinement_withdrawn`, status 2, before settings/worker/channel. Text consumed as another option's value is data. “start-session” and its withdrawn-option example. |
| `end-session RESULT` | One required opaque UTF-8 argument, including empty; use `--` for option-looking result. First valid publication wins, at most 1 MiB. No session-id/path selector, config or fallback. “end-session RESULT”. |
| `skills resolve` | Source acquisition only; no script, result channel, harness, name/task/source override. Snapshots/descriptors stored immutably; no automatic deletion. “skills resolve”. |
| `skills resolve --json` | Schema 1 resolved object (`descriptorPath`, `sources`, `directories`, `mandatory`, `instructions`) or disabled object; plain instructions end in newline, disabled plain output empty. “skills resolve”. |
| Option delimiter and rejected input | `--` ends option parsing; malformed/unknown input status 2. Informational clap output bypasses JSON failure rendering. “ravel, help and version”, “--json”, “end-session RESULT”. |

There are thirteen public `start-session` options excluding generated help:
eleven `SelectionArgs` fields plus `exit_dir` and `json`. The two hidden withdrawn
options are accounted for separately. The worker's argv[0] role is internal,
not another public verb. Test-only K28/C2/C3/C4 and fixture environment controls
are not supported CLI options or runtime configuration.

## Status and stream inventory

Sources: `crates/ravel/src/main.rs`, `crates/ravel/src/run.rs`,
`crates/ravel/src/refusal.rs`, `crates/ravel/src/completion.rs`,
`crates/ravel/src/cancellation.rs`, `crates/ravel/src/skills/command.rs` and
`crates/keyed-launch/src/run.rs`. Destinations: “Exit statuses”, “Standard
streams”, “JSON routing notice” and “JSON failure document”.

| Outcome | Status and output |
|---|---|
| Accepted session result | 0 after reap, group absence and required cleanup; exact UTF-8 bytes flushed to stdout, no added newline. Empty differs from missing. Managed exit alone does not decide success. |
| Missing result / publication failure | 1. `end-session` succeeds silently with 0, fails with `ravel: end-session:` on stderr; does not read settings. |
| Malformed input/settings, withdrawn option | 2. Ordinary parse failures use `malformed_input`; help without required command is clap help on stderr. |
| Rejected authority/script/context/selection/skills | 3; no fallback route. |
| Worker, protocol, supervision, result validity/output, required cleanup | 5; failed stdout write may leave a prefix that must be discarded. |
| Selection or skill-resolution deadline | 124; no final managed launch. Cleanup failure or cancellation may replace it. |
| Unexecutable selected program / pre-start launch error | 126 except ENOENT. |
| Program not found / pre-start ENOENT | 127. |
| INT / TERM / HUP cancellation | Re-raise original signal after cleanup; numeric diagnostics 130 / 143 / 129. Suppress result. Secondary reports retained. |
| Skills resolver | 0, 2, 3, 5, 124 or signal; no instructions on failure. |
| Help / version | 0 informational stdout, independent of result contract. |

The managed process's stdin stays the caller's input; its stdout/stderr are
redirected to caller stderr so stdout belongs to the final result. Worker streams
are captured: text mode prefixes `config stdout:` / `config stderr:`; JSON mode
uses diagnostics strings (lossy UTF-8 conversion). Text routing starts
`ravel: starting provider`; failures start `ravel: refused (CODE, stage STAGE):`.
Runner reports use `ravel:`. Stderr remains mixed and child text can imitate a
notice, so a notice is not an authenticated result or evidence of final success.

Routing schema 2 fields: `selection` (provider/model/effort/reason), `config`
(path/version/modules), `worker` (path/version/buildId/rhaiVersion), `selectionMs`,
`skills`, `context`, `bounds`, `configEnvironment`, `resolution`, `promptSource`,
`diagnostics`, `reads`, `harnessRuns`; `operationReports` only with a returned
runner account. The chapter tables describe every nested record, unit, origin,
ordering and null case. Notice emission follows the supervision attempt and
precedes final outcome/cleanup decisions. Selection, result-output reservation
and channel allocation must first succeed.

Failure schema 2 fields: required `error.code`, `stage`, `message`, `remedy`,
`exit`; optional `input`, `source`, `location`, `configCode`, `bound`, `signal`;
optional top-level `diagnostics`. Bound holds name, unit value and origin.
The chapter enumerates all eleven stage strings: cli, authority, worker,
evaluation, load, context, validation, selection, resolution, exec, skills.
Operational reports can occur outside the document. `--json` belongs to
start-session or skills resolve, not root/end-session; a flag consumed as prompt
or identity text is data rather than a mode switch.

## Runtime environment inventory

Sources: `crates/ravel/src/environment.rs`, `crates/ravel/src/choice.rs`,
`crates/ravel/src/run.rs`, `crates/ravel/src/completion.rs`,
`crates/ravel/src/worker.rs`, `crates/ravel/src/rhai_worker/host.rs`,
`crates/ravel/src/rhai_worker/harness.rs`, `crates/ravel/src/skills/command.rs`,
`crates/ravel/src/skills/launch.rs`, `crates/ravel/src/skills/mod.rs` and
`crates/keyed-launch/src/run.rs`. Destinations: the four “Environment variables”
subsections; skill acquisition's protected paths are additionally explained in
“Skill delivery at launch”.

| Name/class | Read, set, pass or deny |
|---|---|
| `HOME` | Read for default entry/settings/skills/cache; base helper grant. Absolute HOME needed for default locations; explicit config can work without settings. |
| `PATH` | Read for final/harness program resolution and base helper; unset means no search; relative/empty entries use caller cwd. |
| `TMPDIR` | System temporary-directory selection for worker/harness/session staging; base helper grant. |
| `LANG`, every case-sensitive `LC_*` | Base helper names; returned from caller only upon request. |
| Any explicit `configEnv` / `--config-env` name | Read presence for diagnostic `{name,set}`; helper reads caller value only if permitted; unset returns Rhai unit, non-UTF-8 throws. No variable added to child by a grant. |
| `LD_*`, `DYLD_*`, `RAVEL_*`, `HARNESS_DISPATCH_*` | Case-sensitive grant denial even for empty suffix; `excluded_grant` 2. Ungranted helper request throws `env_not_granted`; reserved classes are not an inheritance scrub list. |
| `RAVEL_CONFIG_RUN` | Read presence before settings in start-session; present even empty refuses 3. Set to `1` for evaluation harness runs only; prevents nested selection there. |
| `RAVEL_EXIT_FILE` | Read by end-session for inherited channel authority, and by skill acquisition to protect an absolute enclosing channel from source overlap. Scrub enclosing value, then set a fresh path for each managed/harness launch after other grants. Worker never receives it. |
| `RAVEL_RUN_ID`, `RAVEL_STATE_DIR`, `HARNESS_DISPATCH_EXIT_FILE`, `HARNESS_DISPATCH_RUN_ID`, `HARNESS_DISPATCH_STATE_DIR` | Exactly these five plus enclosing `RAVEL_EXIT_FILE` scrubbed; obsolete names select no session/store. |
| `RAVEL_SKILL_DIRECTORIES` | Fresh absolute descriptor path set for managed/harness launch, replacing inherited value; removed if delivery disabled. No worker environment or helper grant; script gets request.skills. |
| `XDG_CONFIG_HOME` | Does not relocate settings/config; helper requires explicit grant. Otherwise inherited like other caller names. |
| Every other caller variable | Managed/harness children inherit, including application credentials and reserved-prefix names outside exact scrub list. Worker starts with env_clear and empty environment; helper runs in front process over private protocol. |

Launch order is scrub, explicit grants, fresh channel last. Native temporary-path
lookup and OS process inheritance are mechanisms, not extra Ravel options.
Compile-time `RAVEL_BUILD_ID` supplies worker identity, not caller runtime input.

## Disposition

Validation passed: book build; 22 excerpts; zero STE problems in 35 files;
zero prose-break violations across eleven chapters; book-integrity controls;
14 CLI integration tests. The one skipped test is the private PTY subprocess
helper `terminal_recovery_cli_role`, marked ignored for direct test discovery.
The exact root help fragment, thirteen-option count and all 196 unchanged
subject digests also passed evidence assertions. These results do not extend
the earlier platform or reader claims.

Corrected the root help fragment and added missing wrapper help/bare-wrapper
behaviour. All enumerated CLI items have explanatory reference destinations;
no CLI coverage defect is deferred. Other inventories, a fresh whole-book read,
release reconciliation and Read 2 remain with the live sibling/final-proof tasks.
The inventory does not replace those checks or comprehension review.
