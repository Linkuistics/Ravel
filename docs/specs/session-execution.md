# Session execution

Ravel routes a caller's request through the owner's configuration script and supervises the selected process.
Its public boundary is [start-session and end-session](../book/src/06-reference.md#command-line).
The source resolution command `skills resolve` of [skill discovery](skill-discovery.md#ravel-skills-resolve) is also public.
Caller prompts, result meanings, task conventions and durable provenance belong to callers.
This specification describes the active executable. The [glossary](../../CONTEXT.md) defines its terms.

The [configuration evaluation specification](configuration-evaluation.md) owns discovery, requests, selection context, selections, settings, evaluation and delivery.
The [completion specification](result-job-completion.md) owns detailed settlement and native-entry requirements.
The [skill discovery specification](skill-discovery.md) owns skill sources, the prepared prompt and their read grants.
The [token handoff specification](token-handoff.md) owns the owner-minted token that a confined harness receives.
The [component explanation](../book/src/07-the-parts-of-ravel.md) describes the participants.
These specifications remain the behavioural authority for that explanation.

## Installed boundary

The [installed layout and delivery contract](configuration-evaluation.md#installed-layout-and-delivery) owns the executable, examples, notices and installation checks.
The Rhai worker is the Ravel executable in an internal role.

<!-- tc: depart synonym-rotation because launch names process admission while start describes creating the worker -->
It must finish evaluation and required cleanup before the managed launch.
The [worker contract](configuration-evaluation.md#the-rhai-worker) owns its protocol, identity, isolation and admission rules.

## Archives and platform floors

The [delivery contract](configuration-evaluation.md#installed-layout-and-delivery) owns archive contents, formula checks and measured platform floors.
The [delivery instructions](../book/src/11-deliver-ravel.md) describe prerequisites and publication procedures.

## Configuration authority and discovery

The [entry and discovery contract](configuration-evaluation.md#configuration-entry-and-discovery) owns configuration authority and file admission.
The [settings and environment rules](configuration-evaluation.md#settings-and-environment-grants) own settings and grants.

## Request, context and selection

The [request, context and selection contract](configuration-evaluation.md#request-context-and-selection) owns caller input checks and script results.
A successful selection identifies a command. The worker and every host call must finish before that command launches.
A refusal launches nothing. Ravel substitutes no route after a failure.

With skill delivery enabled, the Ravel CLI resolves the skill sources and prepares the prompt before the worker launches.
A skill refusal launches neither the worker nor the managed process.

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
Ravel also sets or removes `RAVEL_SKILL_DIRECTORIES`, as the [skill discovery specification](skill-discovery.md#ravel_skill_directories) states.
The separate [re-entry rule](configuration-evaluation.md#re-entry-and-completion-authority) prevents an inherited `RAVEL_CONFIG_RUN` from reaching a managed launch.
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
| Skill-source refusal, stage `skills` | 3 |
| Token handoff refusal, stage `handoff` | 3 |
| Worker or supervision failure | 5 |
| Whole evaluation timeout | 124 |
| Skill resolution deadline | 124 |
| Selected program cannot start | 126 or 127 |

### Confinement

`--confine` confines the managed process and each evaluation harness run, without an unconfined fallback.
It requires macOS sandbox-exec or Linux bubblewrap.
The Rhai worker evaluates outside this confinement and has its own isolation rules.
Canonical working directory and a private run directory are writable.
`--exit-dir` adds an existing writable parent for the result channel.

`--runtime-read` grants repeatable regular files read-only and requires confinement. System runtime reads and the canonical Ravel executable are admitted. Canonical grants must not overlap the configuration directory or settings file.

With skill delivery enabled, Ravel also grants the descriptor directory and the snapshot trees read-only, as [read access and delegation](skill-discovery.md#read-access-and-delegation) states.
A granted directory that overlaps the configuration directory, the settings file or a known result channel refuses as `skill_access_unavailable`, exit 3.
So does a writable grant that overlaps the skill cache, and a granted directory that the backend cannot list.
A granted directory that changed after its check gives no grant, as the skill discovery specification states.

Each declared token file is a protected path, and a delivered token handoff adds an inherited socket, as the [token handoff specification](token-handoff.md) states.

The confined environment keeps `HOME`, `USER`, `LOGNAME`, `PATH`, `LANG` and `LC_*`. `TMP`, `TEMP` and `TMPDIR` point into the private run directory. A delivered token handoff adds its declared variable. The backend's filesystem contract and platform limits live in
[the runner explanation](../book/src/09-jobs-signals-and-the-terminal.md#apply-filesystem-confinement).

## Verification and scope

The Taskfile owns reusable checks. `task check` checks formatting, shell scripts, all Rust targets and the workspace tests with complete saved logs. It also checks the book-tool integrity, fake harness recipes and terminal recovery.

Private subprocess roles run through acceptance tests. `task smoke` verifies an installation. `task release:smoke` verifies archives on their target userlands.

The [configuration test seam](configuration-evaluation.md#test-seams) owns Rhai acceptance checks and opt-in real harness calls. Default checks need no models, owner credentials, personal configuration or old store.

Ravel has no session persistence, run identity, observation API, history API, importer, caller adapter or public direct-command bypass.
The retained skill-content cache of [skill discovery](skill-discovery.md#publication-concurrency-and-cleanup) is its one persistence surface. It holds no run identity or result.
Abrupt death can leave ephemeral debris.
The [retired-project retrieval record](../retired-project.md) remains independent.
