# Session execution

Ravel routes a caller's request through the owner's configuration script and supervises the selected process.
Its public boundary is [start-session and end-session](../book/src/06-reference.md#command-line).
Caller prompts, result meanings, task conventions and durable provenance belong to callers.
This specification describes the active executable. The [glossary](../../CONTEXT.md) defines its terms.

The [configuration evaluation specification](policy-evaluation.md) owns discovery, requests, selection context, selections, settings, evaluation and delivery.
The [completion specification](result-job-completion.md) owns detailed settlement and native-entry requirements.
The [component explanation](../book/src/07-the-parts-of-ravel.md) describes the participants.
These specifications remain the behavioural authority for that explanation.

## Installed boundary

The [installed layout and delivery contract](policy-evaluation.md#installed-layout-and-delivery) owns the executable, examples, notices and installation checks.
The Rhai worker is the Ravel executable in an internal role.

<!-- tc: depart synonym-rotation because launch names process admission while start occurs in the retained worker-contract anchor -->
It must finish evaluation and required cleanup before the managed launch.
The [worker contract](policy-evaluation.md#the-policy-worker) owns its protocol, identity, isolation and admission rules.

## Archives and platform floors

The [delivery contract](policy-evaluation.md#installed-layout-and-delivery) owns archive contents, formula checks and measured platform floors.
The [delivery instructions](../book/src/11-deliver-ravel.md) describe prerequisites and publication procedures.

<a id="policy-authority-and-runtime-discovery"></a>

## Configuration authority and discovery

The [entry and discovery contract](policy-evaluation.md#policy-entry-and-discovery) owns configuration authority and file admission.
The [settings and environment rules](policy-evaluation.md#settings-and-environment-grants) own settings and grants.
The explicit anchor preserves existing references until the coordinated configuration rename.

## Request, context and selection

The [request, context and selection contract](policy-evaluation.md#request-context-and-selection) owns caller input checks and script results.
A successful selection identifies a command. The worker and every host call must finish before that command launches.
A refusal launches nothing. Ravel substitutes no route after a failure.

## Managed process and result

Each launch allocates a fresh private session directory and result path.
`RAVEL_EXIT_FILE` reaches only the managed process as its session-result context.
A nested session replaces that value only for its own child.
The caller must put `ravel` on the managed `PATH` or supply its absolute path in ending instructions.

An unconfined managed process otherwise receives the caller's environment.
Ravel scrubs these six exact launch controls before supplying the fresh result path:

- `HARNESS_DISPATCH_EXIT_FILE`
- `HARNESS_DISPATCH_RUN_ID`
- `HARNESS_DISPATCH_STATE_DIR`
- `RAVEL_EXIT_FILE`
- `RAVEL_RUN_ID`
- `RAVEL_STATE_DIR`

Other names in these classes remain ordinary caller variables.
The separate [re-entry rule](policy-evaluation.md#re-entry-and-completion-authority) prevents an inherited `RAVEL_POLICY_RUN` from reaching a managed launch.
Configuration environment grants have wider reserved classes, as the configuration spec states.

`end-session` takes exactly one opaque UTF-8 argument, reads no configuration and accepts no target. It writes a complete temporary regular file, then publishes with a hard link that cannot replace an existing file. The first value wins.

Repeated publication succeeds without replacing it. The bound is 1 MiB. Missing, empty or stale session-result context refuses.

An explicitly empty result remains distinct from no result.

Readers use a held directory and reject symlinks, FIFOs and other nonregular entries.
Cleanup checks directory identity. It preserves replacements and reports owned directories that moved and may need manual removal.
This is isolation by inherited context. It does not authenticate code of the same user.

Publication requests the supervisor's 2-second grace, then TERM, up to 5 seconds of kill-grace, then KILL when needed.
Descendant group cleanup occurs before reap, including on natural exit.
The runner preserves job control and recovers the terminal, as [the child is a job](../adr/the-launched-child-is-a-job.md) requires.
The [completion contract](result-job-completion.md) owns detailed cancellation, failure precedence, cleanup and native-entry obligations.

Only successful settlement permits the exact result bytes on stdout, with no added newline. Managed stdout goes to stderr before spawn. The runner retains stdin and terminal ownership.

Diagnostics and optional `--json` routing notices or refusals also go to stderr. Result stdout must be writable before launch. A write failure after reap remains an error and can leave a written prefix.

A supplied result succeeds regardless of the child's later status. Natural exit without a result is exit 1, even for status 0. Cancellation by INT, TERM or HUP suppresses the result and reports cleanup failures or group survivors.

Ravel propagates that signal after required cleanup attempts. A cleanup or supervision failure prevents result success. Ravel assigns no meaning to any result token.

| Outcome | Exit status |
|---|---|
| CLI input failure | 2 |
| Configuration refusal | 3 |
| Worker or supervision failure | 5 |
| Whole evaluation timeout | 124 |
| Selected program cannot start | 126 or 127 |

### Confinement

`--confine` confines the managed process and each evaluation harness run, without an unconfined fallback.
It requires macOS sandbox-exec or Linux bubblewrap.
The Rhai worker evaluates outside this confinement and has its own isolation rules.
Canonical working directory and a private run directory are writable.
`--exit-dir` adds an existing writable parent for the result channel.

`--runtime-read` grants repeatable regular files read-only and requires confinement. System runtime reads and the canonical Ravel executable are admitted. Canonical grants must not overlap the configuration directory or settings file.

The confined environment keeps `HOME`, `USER`, `LOGNAME`, `PATH`, `LANG` and `LC_*`. `TMP`, `TEMP` and `TMPDIR` point into the private run directory. The backend's filesystem contract and platform limits live in
[the runner explanation](../book/src/09-jobs-signals-and-the-terminal.md#apply-filesystem-confinement).

## Verification and scope

The Taskfile owns reusable checks. `task check` checks formatting, shell scripts, all Rust targets and the workspace tests with complete saved logs. It also checks the book-tool integrity, fake harness recipes and terminal recovery.

Private subprocess roles run through acceptance tests. `task smoke` verifies an installation. `task release:smoke` verifies archives on their target userlands.

The [configuration test seam](policy-evaluation.md#test-seams) owns Rhai acceptance checks and opt-in real harness calls. Default checks need no models, owner credentials, personal configuration or old store.

Ravel has no persistence, run identity, observation API, history API, importer, caller adapter or public direct-command bypass.
Abrupt death can leave ephemeral debris.
The [retired-project retrieval record](../retired-project.md) remains independent.
