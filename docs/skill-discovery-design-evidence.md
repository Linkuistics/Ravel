# Skill discovery design evidence

The [specification](specs/skill-discovery.md) states the proposed directory
contract of the `skill-directory-delivery` line. This record gives the source
observations behind it, the exact base of the line, the stage handoff and the
checks of these documents. The design session read Ravel source at `main` and
changed no product code. The decision records own the decisions, and this
record adds no second decision register.

## Base of the line

Observed on 2026-11-02 through `jj git fetch` and local reads.

| Subject | Observation |
| --- | --- |
| Ravel `main` | Local `main` and `main@origin` are both `24a545236185b0910ba6117fe8f9e8b2e2c8a748`. The fetch reported no change. |
| Upstream changes | None after Grove's `coordinated-ravel-release-k36` observed the same commit. |
| Release-channel contract | `80e382223129`, `c94741960df5` and `2a77ffbda3b2` sit above `main` on the local bookmark `coordinated-release-design`. Nobody pushed them. |
| Historical directory design | `b68af6e4`, `66aaec06`, `4c16f8e0` and `b93bb927` sit on the v1.1.0 release commit `2095cbd8`, not on `main`. They supply intent, not the TypeScript runtime, schema or version assignments. |
| This line | The bookmark `skill-directory-delivery` sits on `2a77ffbda3b2`, so it carries the channel contract unchanged. The design commit is its first commit above that. Grove's task records its exact identity. |
| Other workstream | This session edited neither the `Ravel.gh-issue-2` workspace nor the `.grove/` tree that `main` carries. |

## Source observations at `main`

| Observation | Source | Consequence for the design |
| --- | --- | --- |
| `run` checks `RAVEL_CONFIG_RUN`. `choice::prepare` then reads the settings, the inputs and the entry. Confinement follows, and `choose_prepared` then launches the worker | `crates/ravel/src/run.rs`, `run`. `crates/ravel/src/choice.rs`, `prepare` | Resolution belongs at the end of `prepare`, before confinement and before the worker |
| The Ravel CLI sends `kind`, `prompt`, `cwd`, `params` and the optional `taskFile`, `taskId` and `context`. The worker adds `schemaVersion` and `limits` | `choice.rs`, `rhai_request`. `crates/ravel/src/rhai_worker/engine.rs`, `request_map` | Schema 4 adds `originalPrompt` and `skills` in both halves |
| `load_context` receives a clone of the request map, and `select` receives the map in its own scope | `engine.rs`, `run` and `load_context` | Each phase receives a copy, so a change in the loader cannot reach `select` |
| The evaluate frame carries `entry`, `source`, `request`, `bounds` and `selectContext`. With a caller context, the context is in `request.context` and again in `selectContext` | `crates/ravel/src/rhai_worker/serve.rs`, `serve`. `choice.rs`, `evaluate_and_resolve_rhai` | The derived frame bound counts the context twice |
| `FRONT_FRAME_BYTES` is the larger of 2 × 8 MiB + 1 MiB and the 19 MiB harness reply. An evaluate frame over it fails its write as a conversation failure | `crates/ravel/src/frame.rs`. `serve.rs`, `write_body` | At `main`, an 8 MiB context and a 1 MiB prompt of escaped characters can pass that bound. This is a derivation from the source, not a measurement. The line derives a new bound and refuses an oversized frame as caller input |
| The worker checks its result against `messageBytes` and reports `message_too_large` | `crates/ravel/src/rhai_worker/role.rs`, `bounded` | The script-result bound stays 1 MiB |
| The settings keys are `timeoutMs`, `contextBytes`, `configEnv`, `harnesses` and `budgets`. An unknown key refuses as `settings_invalid`, stage `cli`, exit 2. The parser reads a JSON map, which keeps one value of a repeated member | `crates/ravel/src/settings.rs`, `Settings::read` | `skills` joins the keys. Detection of repeated members is new work |
| Ravel 1.1.0 accepts `timeoutMs`, `contextBytes` and `policyEnv`. Any other key refuses as `settings_invalid`, `Stage::Cli`, exit 2 | `settings.rs` at `2095cbd8` | The exact 1.1 behaviour that the owner-delivery design asks for |
| A cancellation before evaluation settles as `session_cancelled`, stage `exec` | `run.rs`, `settle` | Resolution has its own code, so that the stage names the phase |
| The prompt bound is 1 MiB, the task ID bound is 1,024 bytes and the parameter bound is 64 KiB. A prompt with NUL refuses | `crates/ravel/src/inputs.rs` | The inputs of the frame derivation |
| The smoke script links `Library/Keychains` on macOS, the whole `.claude` on Linux and the whole `.codex` for Codex | `scripts/harness-smoke.sh`, `link_store` | The smoke tasks adopt the narrower sandbox home |
| `llm.rhai` sends `request.prompt` to a classifier in `stdin` | `crates/ravel/examples/llm.rhai` | Under schema 4, the example reads `originalPrompt` |
| `docs:result-job` uses port 8774, and `docs:rhai-configuration` uses port 8776 | `Taskfile.yml` | The skill views use port 8778 |
| Book chapter 10 lists TypeScript check stages, such as `worker-types`, that `scripts/check.sh` no longer runs | `docs/book/src/10-develop-ravel.md`, `scripts/check.sh` | The Rhai book work is unfinished. This line does not assume that it is complete |

The session read these files at `main` through `jj file show` and in the new
workspace. It used no knowledge-graph result, so this record makes no claim of
graph completeness. The positions above name functions, not line numbers,
because the line will move them.

## Records

[Skill preparation precedes evaluation](adr/skill-preparation-precedes-evaluation.md)
replaces the historical record `skill-preparation-precedes-policy`. It adds the
refusal of schema 3 with delivery disabled.
[Skill resources outlive sessions](adr/skill-resources-outlive-sessions.md)
keeps its decision in the vocabulary of `main`.
[Ravel supervises the session](adr/ravel-supervises-the-session.md) gains the
delegate residue and the one persistence surface. The
[configuration evaluation specification](specs/configuration-evaluation.md)
gains schema 4, the `skills` settings key, the frame bounds and the sandbox
home. The [session execution specification](specs/session-execution.md) links
to the new rules.

## Feature line and stage handoff

This grove is the one executor of the Ravel release and of the owner cutover.
Grove's `docs/specs/coordinated-release.md` records that ownership. The rules
below are the integration contract for the Ravel owner. They bind before any
implementation.

1. The bookmark `skill-directory-delivery` holds the line. Its base is `main`
   `24a545236185`, through the channel contract `2a77ffbda3b2`.
2. The local stage, Grove's `ravel-local-delivery-k23`, rebases the line onto
   the current `main` at entry. It reconciles upstream changes, runs the
   affected checks again and records its exact base and output commits.
3. The local stage implements schema 4, the `skills` settings object, local
   capture, publication, the descriptor and prompt preparation. It also
   implements `RAVEL_SKILL_DIRECTORIES`, the read grants, the frame bound and
   its check, `skills resolve`, the notice member, the stage and the codes. It
   moves the smoke tasks to the narrow sandbox home. It migrates the examples,
   the book and the changelog with that behaviour.
4. The local build refuses a valid Git source before configuration evaluation.
   The remedy names the unfinished pinned stage. The refusal has no stable
   public code, and it is not `settings_invalid`.
5. The pinned stage, `ravel-pinned-delivery-k25`, rebases at entry in the same
   way. It implements Git acquisition and offline reuse. It decides whether
   the Homebrew formula declares Git. It removes the temporary refusal and its
   fixture.
6. Grove's `common-loading-acceptance-k29` runs the actual harness trials in
   sandbox homes. Rollout, under Grove's `discovery-rollout-k6`, owns the final
   rebase, the full checks, the landing on `main` and the 2.0.0 release
   through `ravel@2`.
7. The line stays off `main` and unpublished through acceptance. No incomplete
   build enters a release. No session edits the `Ravel.gh-issue-2` workspace or
   its task tree.

Each stage runs the Ravel tasks that its change touches. `task setup` installs
the book tools first.

The tasks are `task check`, `task test:cli`, `task smoke`
and `task install PREFIX=<prefix>`. The book tasks are `task docs:book:build`,
`task docs:examples:test`, `task docs:excerpts:test`, `task docs:figures:test`,
`task docs:ste:test`, `task docs:breaks:test` and `task book:integrity`. The
release preparation tasks are `task release:test`, `task release:formula` and
`task release:smoke`. `task harness:smoke:test` checks the smoke script. The
opt-in `task harness:smoke:claude` and `task harness:smoke:codex` spend
subscription usage.

The local stage checks one fact before it changes the smoke script. Claude
Code on Linux keeps its sign-in in `.claude/.credentials.json`. The stage reads
the installed Claude Code to establish that path before it links it.

## Consumer handoff

Grove's adoption design, `grove-rhai-skill-delivery-k35`, consumes these facts:

- Directory delivery needs Ravel 2.0.0. Its configuration script is
  `config.rhai` with `SCHEMA_VERSION` 4. Grove's caller admits only stable 1.x
  today, so it admits 2.0.0 explicitly.
- The native route stays Ravel 1.1.0, `policy.ts` with schema 2 and settings
  without 2.0 keys.
- Ravel 1.1.0 refuses a settings file with `skills`, `configEnv`, `harnesses`
  or `budgets` as `settings_invalid`, stage `cli`, exit 2. While a 1.1 reader
  remains, the directory route uses a separate `HOME`.
- Grove's examples for the directory route put `request.prompt` into the
  harness arguments. They read task data, such as a handle or a provider
  receipt, from `request.originalPrompt`.
- The mandate qualifies "first" with "after any mandatory guidance". The ending
  command stays in the caller block, byte for byte.
- Grove needs neither `RAVEL_SKILL_DIRECTORIES` nor `skills resolve`. Neither
  is a prerequisite of Grove.
- Grove parses neither the routing notice nor the failure document. Both keep
  `schemaVersion` 2.
- Until promotion, the directory route selects the keg-only `ravel@2` by its
  prefix, as Ravel's [release channels](specs/release-channels.md) state.

## Checks of these documents

The technical-communication checker reports no hard finding in the new and
changed documents. Ravel's own `scripts/ste.sh --strict` passes `CONTEXT.md`
with the pinned book tools. A planted control file fails the same command.
Local links and anchors resolve, and the view manifest parses. The rendered
views and their inspection are in Grove's task record.

These checks cover the documents only. No product test, installed check or
real harness trial ran. The design claims no acquisition, confinement,
compatibility or harness behaviour in advance.
