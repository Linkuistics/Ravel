# Skill discovery design evidence

The [specification](specs/skill-discovery.md) states the proposed directory
contract of the `skill-directory-delivery` line. This record gives the source
observations behind it, the exact base of the line, the stage handoff and the
checks of these documents. It also records the
[handoff of the local stage](#local-stage-record) and the
[withdrawal of Git acquisition](#withdrawal-of-git-acquisition). The design session read
Ravel source at `main` and changed no product code. The decision records own the decisions, and this
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
| `LAUNCH_CONTROLS` holds six names, and `CLAUDE_CONFIG_DIR` is not one of them. The `sandboxed` function of the smoke script removes only four credential names and two Ravel names | `crates/ravel/src/environment.rs`. `crates/ravel/src/rhai_worker/harness.rs`, `execute`. `scripts/harness-smoke.sh`, `sandboxed` | An inherited Claude Code override reaches the harness. The sandbox wrapper must control it |
| `read_frame` takes its bound as an argument, and its unit test refuses a 16-byte frame at a bound of 15 | `crates/ravel/src/frame.rs` | The new check can receive its limit in the same way. A unit test can then cross a known frame size by one byte |
| `context::deliver` measures the compact encoding of the caller's document before the evaluate frame, also when the script has a `load_context`. A caller document cannot hold `measured`, so that encoding includes every caller member | `choice.rs`, `evaluate_and_resolve_rhai`. `crates/ravel/src/context.rs`, `deliver` | `request.context` encodes to at most the context budget, as the frame derivation assumes |
| `--kind` and `--task-file` are command-line arguments. The working directory comes from the process | `crates/ravel/src/cli.rs`. `crates/ravel/src/inputs.rs` | The argument limit of the platform bounds the kind and the task-file path. The [frame-bound platforms](#frame-bound-platforms) show that the platform does not bound the working directory at these sizes |
| `llm.rhai` sends `request.prompt` to a classifier in `stdin` | `crates/ravel/examples/llm.rhai` | Under schema 4, the example reads `originalPrompt` |
| `docs:result-job` uses port 8774, and `docs:rhai-configuration` uses port 8776 | `Taskfile.yml` | The skill views use port 8778 |
| Book chapter 10 lists TypeScript check stages, such as `worker-types`, that `scripts/check.sh` no longer runs | `docs/book/src/10-develop-ravel.md`, `scripts/check.sh` | The Rhai book work is unfinished. This line does not assume that it is complete |

The design session read these files at `main` through `jj file show` and in the
new workspace. The review integration read the source of four added rows in
this workspace, whose source matches `main`. Neither session used a
knowledge-graph result, so this record makes no claim of graph completeness. The positions above name functions, not line numbers,
because the line will move them.

## Harness observations

The review integration read the Claude Code
[environment reference](https://code.claude.com/docs/en/env-vars),
[authentication page](https://code.claude.com/docs/en/authentication#credential-management)
and [skills page](https://code.claude.com/docs/en/skills) on 2026-11-02. These
pages describe the published interface. No harness trial measured them.

| Observation | Consequence for the sandbox home |
| --- | --- |
| `CLAUDE_CONFIG_DIR` replaces `~/.claude`. Settings, session history and plugins move with it | An inherited value can expose a real configuration |
| With `CLAUDE_CONFIG_DIR`, the sign-in file moves under that directory. The macOS Keychain entry is keyed to that directory | A default-path sign-in link need not supply the account of an override. A sandbox value would move the sign-in away from the link |
| `CLAUDE_CODE_PLUGIN_CACHE_DIR` sets the plugins root. `CLAUDE_CODE_PLUGIN_DIRS` loads plugin directories. `CLAUDE_CODE_PLUGIN_SEED_DIR` adds seed directories | Each inherited value can expose real plugins and their skills |
| A terminal session with a claude.ai sign-in downloads the account's skills into `~/.claude/skills/synced/` | The sandbox `.claude` receives these platform-managed skills |
| On macOS, Claude Code writes `~/.claude/.credentials.json` when the Keychain refuses the write | The sandbox links the keychain only. The sign-in check of the run then fails, and the owner moves the sign-in back to the Keychain |

The local stage checks the installed Claude Code against these rows before it
changes the smoke script. A published page is not a measurement of that build.

### Measured by the local stage

`ravel-smoke-sandbox-home-k60` measured these facts on 2026-11-02, before it
changed the smoke script. It read the minified source in each binary, and it
ran `auth status` or `login status`, which make no model call.

| Harness and platform | Build |
| --- | --- |
| Claude Code 2.1.316, aarch64 macOS, Darwin 27.0.0 | Native install, binary SHA-256 `64c327e92b2c5e0f78c0a980c5a518121e1f0d140ca771ca5dd4488d1a2dcb3f` |
| Claude Code 2.1.316, aarch64 GNU/Linux, a container from `ravel-native-linux:arm64`, kernel 7.0.14-linuxkit | The official installer, binary SHA-256 `55cdb8e162d190432696ea54010e0fca2e18d06fa7a7127fd563b9ffd4f22624` |
| Codex 0.166.0, aarch64 macOS | Homebrew |

The Linux sign-in path is `.claude/.credentials.json`. The source joins
`.credentials.json` to the store directory. The store directory is
`CLAUDE_SECURESTORAGE_CONFIG_DIR` when that variable is set, and otherwise the
configuration directory. With a fake token, `auth status` reports a claude.ai
sign-in for a regular file in the default location. It reports the same for a
sandbox `.claude/.credentials.json` that links to that file, and for the file
under `CLAUDE_CONFIG_DIR`. When `CLAUDE_CONFIG_DIR` names a directory without
the file, it reports no sign-in.

Each row of the table above holds for 2.1.316:

| Row | Measurement |
| --- | --- |
| `CLAUDE_CONFIG_DIR` replaces `~/.claude` | `auth status` reports that directory as `configDirectory` on both platforms |
| The sign-in moves with `CLAUDE_CONFIG_DIR` | Linux: the file moves, as above. macOS: the Keychain service is `Claude Code-credentials`, with a suffix of eight hexadecimal digits of the SHA-256 of the directory when the variable is set. With the Keychain link and the variable, `auth status` reports no sign-in |
| Three plugin variables | The source reads `CLAUDE_CODE_PLUGIN_CACHE_DIR` as the plugins root, `CLAUDE_CODE_PLUGIN_DIRS` as plugin directories and `CLAUDE_CODE_PLUGIN_SEED_DIR` as a list of seed directories |
| Account skills in `skills/synced` | The source names the directory `synced` and marks those skills `syncedSkills`. This stage ran no claude.ai session in a sandbox, so it did not see the download |
| macOS writes `.credentials.json` when the Keychain refuses | The macOS store is a Keychain store with a plaintext fallback. It reads `.credentials.json` in the configuration directory when the Keychain holds no item |

Three more facts change or qualify the sandbox home:

- `CLAUDE_SECURESTORAGE_CONFIG_DIR` moves only the sign-in store on both
  platforms. With it set to a real `.claude` and an empty home, `auth status`
  reports a sign-in. The published environment reference does not list it.
  The smoke run refuses it with the other four names.
- The Linux binary also holds a second store reader. It opens the file with
  `O_NOFOLLOW` and treats a link as the state `refused-symlink`, which reads
  as no sign-in. A gate that the minified source does not name selects it.
  `auth status` in 2.1.316 accepts the link. A later build that selects this
  reader would refuse the Linux sign-in link, and the smoke run would fail at
  its sign-in check.
- Codex 0.166.0 reports a ChatGPT sign-in when `CODEX_HOME` names a sandbox
  `.codex` that holds only a link to the real `auth.json`. It writes `tmp/`
  beside the link and leaves the link in place.

Token variables such as `CLAUDE_CODE_OAUTH_TOKEN`, and the host-gateway file
`CLAUDE_CODE_HOST_CREDS_FILE`, are other sign-in routes. They move no
configuration, and the smoke run neither refuses nor removes them. This stage
made no real call on Linux, because the owner has no Linux sign-in.

## Frame-bound platforms

The local stage `ravel-schema-four-k53` measured these limits on 2026-11-02.
A probe program found the largest single argument that `exec` admits. It then
entered 2,344 nested directories, one level at a time, and read the working
directory with the `current_dir` function of Rust. Each directory name was 255
bytes of U+0001, the byte that JSON escapes to six bytes.

The request of test 1 fills every public bound with that byte, with delivery
disabled. Its evaluate frame encodes to 36,048,481 bytes. The refused frame of
test 3 encodes to 39,648,476 bytes. Its kind encodes to 3,600,002 bytes, and
the kind `"check"` of test 1 encodes to 7 bytes.

The request of test 1 thus has 2,492,831 bytes of room under the frame bound.
Short parameters can encode to at most 393,216 bytes more than its one long
parameter. Every request within the public bounds therefore has at least
2,099,615 bytes of room.

| Platform | Largest single argument | Working directory read | Test 3, long kind | Variant, deep working directory |
| --- | --- | --- | --- | --- |
| aarch64 macOS, Darwin 27.0.0, `kern.argmax` 1,048,576 | 1,039,252 bytes | 600,218 bytes | Runs and passes. A kind of 600,000 bytes encodes to 3,600,002 bytes | Runs and passes |
| aarch64 GNU/Linux, Docker image `ravel-native-linux:arm64`, kernel 7.0.14-linuxkit, 4 KiB pages, glibc 2.43 | 131,071 bytes | 600,073 bytes | Cannot run. A kind encodes to at most 786,428 bytes, and a task-file path to about as much. Together they stay under the room of 2,099,615 bytes | Runs and passes |
| x86-64 GNU/Linux | Not measured. The Rust toolchain of the emulated image stopped with a segmentation fault under QEMU | Not measured | Cannot run, by derivation. Linux limits one argument to 32 pages, which is 131,072 bytes with the NUL at the 4 KiB pages of x86-64 | Not run |

The two macOS fixtures are compiled only on macOS. The arm64 Linux run used
`cargo nextest run --locked -p ravel --test rhai_request` in the image above,
and all seven tests that Linux compiles passed. Test 2, the unit test of the
check at a reduced limit, runs on every platform.

The x86-64 row is a derivation, not a measurement. The frame check is the same
code on each platform, so only the platform limits differ. An aarch64 Linux
kernel with 64 KiB pages admits one argument of 2 MiB. Test 3 could then run
there, but no such kernel was measured.

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

[Skill sources are local directories](adr/skill-sources-are-local-directories.md)
records the later withdrawal of Git acquisition.

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
   its check, `skills resolve`, the notice member, the stage and the codes. The
   check receives its limit as an argument, for the three frame-bound tests.
   The stage moves the smoke tasks to the narrow sandbox home. That home
   refuses the inherited Claude Code location variables, and fake-harness
   controls prove each refusal. The stage migrates the examples, the book and
   the changelog with that behaviour.
4. The local build refuses a valid Git source before configuration evaluation.
   The remedy names the pinned stage. The refusal has no stable public code,
   and it is not `settings_invalid`. This refusal is temporary.
5. The owner withdrew the pinned stage, `ravel-pinned-delivery-k25`. Ravel
   reads local sources only, and its formula declares no Git. The next
   implementation stage rebases at entry in the same way. It replaces the
   temporary refusal and its fixture, as the
   [implementation handoff](#implementation-handoff) lists.
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

The local stage establishes two facts before it changes the smoke script and
the frame tests. First, it reads the installed Claude Code on Linux to
establish the sign-in path `.claude/.credentials.json` before it links it.
[Measured by the local stage](#measured-by-the-local-stage) records the result
of `ravel-smoke-sandbox-home-k60`.
Second, it establishes on which release platforms a request can pass the
frame bound. It records each platform where no request can pass it. The
[frame-bound platforms](#frame-bound-platforms) record the result of
`ravel-schema-four-k53`.

## Local stage record

Grove's `ravel-local-handoff-k61` closed the local stage on 2026-11-02. The next
implementation stage and Grove's dual-Ravel stage use this build as their base. Nobody
pushed the line, and no release consumes it.

| Subject | Record |
| --- | --- |
| Base | Local `main` and `main@origin` are both `24a545236185b0910ba6117fe8f9e8b2e2c8a748`. `jj git fetch` reported no change, so the line needed no rebase |
| Build commit | `6436e66fdb308e1c06ee683b6e1b7bc341fabe8b`, with the parent `bc737938a72e`. Every check below ran on this commit, with an empty working copy |
| Output commit | The child of the build commit. It changes only this record. Grove's task record and the brief of `ravel-pinned-delivery-k25` name its identity |
| Build identity | `2ee59f33657690da4facfb7d055527cd3fd12268494dc03bb48e7c0e954bf312`, from the routing notice of the directory launch. It digests the Rust sources, the manifest, the build script and the lockfile, so the output commit has the same identity |
| Version output | `ravel 1.1.0`. The manifest keeps that version until the cut of the release |
| Toolchain | Rust 1.99.1, `rustc` commit `58cf8f9c3` of 2026-10-13, from `rust-toolchain.toml`. Cargo 1.99.1. Rhai 1.27.0. The SHA-256 of `Cargo.lock` is `a06e97c65ecca6d1490180467621834d4cd1c4a7d7fcffb142d4c86566e07266` |
| Platform | aarch64 macOS 27.0.1, build 26A434, Darwin 27.0.0, host target `aarch64-apple-darwin` |
| Installed executable | `task install PREFIX=<temporary prefix>`, outside the real home. The SHA-256 of `bin/ravel` is `456c0bb270ef161dd421e05a86fc7130f63f47a2ef8b43fe3df00c633f703fd0`. The tree digest of `task smoke:pair` for the prefix is `75ce73757643abfecf83d20635512d4343456ec810b729f71658f3974d404a6d`. An earlier installation from the same sources gave the same tree digest |

| Check on the build commit | Result |
| --- | --- |
| `task check` | Exit 0. 727 of 727 run tests passed, and 48 were skipped |
| `task test:cli` | Exit 0. 14 of 14 run tests passed, and 1 was skipped |
| `task harness:smoke:test` | Exit 0 |
| The seven book tasks | Each exits 0. `task docs:excerpts:test` killed 24 of 24 mutants |
| `task test:native-linux PLATFORM=arm64` | Exit 0, with `failed=0`. 720 of 720 run tests passed, and 48 were skipped. The image is `ravel-native-linux:arm64`, Ubuntu 26.04.1 with glibc 2.43, on kernel 7.0.14-linuxkit. No amd64 run occurred |
| `task install` | Exit 0. The installed smoke passed through the executable and through a relative link, each run with its own empty home under `env -i` |
| `task smoke:pair` | Exit 0, as described below |

The pair smoke ran the Homebrew release at `/opt/homebrew/Cellar/ravel/1.1.0`
beside the installed feature build. The SHA-256 of its `bin/ravel` is
`c52f990b2059c8bbe8efba10a741d3a0311000ac1712d71e882638f54c3ac1ed`. Its worker
reported the build identity `fa130cc262cfb22541798debe937ecd9e81488ebef494d873d377978808c9d9c`
and Bun 1.4.2.

- The 1.1.0 release made a native-only launch under one temporary home. Its
  fake program received the caller prompt byte for byte and no
  `RAVEL_SKILL_DIRECTORIES`. It sent its result through the 1.1.0
  executable.
- The feature build made a directory launch under a second temporary home.
  Its fake program received the preamble with the caller bytes. It read the
  mandatory file at its retained path under that home, and
  `RAVEL_SKILL_DIRECTORIES` named the retained descriptor.
- Each fake program wrote a marker and waited for the marker of the other, so
  the two sessions overlapped.
- A settings file with `skills` under the first home made 1.1.0 refuse as
  `settings_invalid`, exit 2, before it launched its fake program.
- `sandbox-exec` denied both runs each read and write in the real home.
- The tree digests of the 1.1.0 installation, the feature installation and the
  real `~/.config/ravel` were the same before and after. The real
  `~/.cache/ravel` was absent before and after. The 1.1.0 tree digest is
  `ca5e9de82b220caf7973c10adc718e7225d07df0cb20903abca52ccdf621d796`.

Five mutated copies of the script each failed with the expected message. A
guard without its deny rule failed the guard control. A directory launch with
the real home failed, because the guard hid the real configuration. A file
added to the feature prefix failed the digest comparison. A settings file
without `skills` failed the refusal check. A script that passed
`originalPrompt` failed the preamble check.

The pair smoke runs on macOS only, because its guard uses `sandbox-exec`.
This stage ran no installed release of 1.1.0 on Linux. The local build refuses
every Git source, so it needs no Git executable.

## Withdrawal of Git acquisition

Grove's `ravel-git-acquire-k70` implemented online exact Git acquisition on
this line as commit `1898dc8603a0`. Grove's review `ravel-git-acquire-k76`
reported six findings against it. During the integration
`ravel-git-acquire-k77` on 2026-11-02, the owner withdrew Git acquisition from
Ravel. Their words: "The process of 'installing' skills and keeping them up to
date should not be part of Ravel."
[Skill sources are local directories](adr/skill-sources-are-local-directories.md)
records the decision.

The integration reproduced each finding before the decision. A scratch probe
ran each counterexample with an isolated `HOME`, against Apple Git 2.54 and
Homebrew Git 2.56. Both versions behaved the same.

| Finding | Reproduced behaviour |
| --- | --- |
| F1 | Git fetched a protected bare repository through four local routes. One is a percent-encoded `file://` locator. Another is a missing path beside a link with the suffix `.git`. The others are a directory whose `.git` file names the repository, and an HTTPS locator that a global `insteadOf` rule rewrites |
| F2 | The locator parser admitted a query and a fragment, and its refusal showed the complete URL |
| F3 | A global `protocol.ext.allow=always` with a rewrite to `ext::` ran the helper |
| F4 | A global `fetch.bundleURI` imported the pinned commit from a bundle when the configured repository did not hold it |
| F5 | Git removes `GIT_CONFIG_PARAMETERS` before it runs a local `upload-pack`. No setting that Ravel passed on the command line reached that server side |
| F6 | Protocol 0 reports an object-format mismatch as "Server does not support this repository's object format", and Ravel gave the transport remedy |

The integration applied none of its partial fixes. The removal of the Git path
resolves all six findings, because no Git child remains. A jj backout of
`1898dc8603a0` is commit `c2e24d371db2`. Its tree equals the local stage output
`1d50dae5d29e`, so the checks of the [local stage record](#local-stage-record)
cover it. Ravel `main` stays at `24a545236185`.

### Implementation handoff

The specification now states the permanent outcome of a Git record. The next
implementation stage changes the build to match it:

1. Make `type` accept only `local`, and check it before the other fields of a
   record.
2. Refuse any other value as `settings_invalid`, exit 2, at
   `skills.sources[N].type`, with the checkout remedy.
3. Remove the Git record parser, its locator check, the Git source origin and
   their unit tests.
4. Remove each branch that reads a Git record, such as the notice `revision`
   member and an unreachable Git branch.
5. Remove `refuse_git`, its internal code `skill_git_unsupported` and the
   launch fixture of that refusal.
6. Add the fixtures of the "Local sources only" acceptance case at both public
   boundaries.
7. Make the public tamper test check the remedy of `skill_cache_corrupt`.
8. Remove the Git parts of the book and of `CHANGELOG.md`.
9. Remove the pinned-stage notes in code comments and tests.

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

## Review integration

Grove's `ravel-rhai-skill-delivery-k49` reviewed commit `fc87cb41a422` and
reported two findings. Grove's `ravel-rhai-skill-delivery-k50` integrated
both findings in the next commit of this line. Its task record holds the
triage.

- The sandbox home now controls `CLAUDE_CONFIG_DIR` and the three Claude Code
  plugin-location variables. A wrapper refuses an inherited value, records
  the effective configuration directory and proves each refusal with a
  fake-harness control.
- The frame bound keeps its value of 38,541,312 bytes. Three tests replace
  the one-byte control: a public-bound fit, a unit test at a reduced limit
  and a long-kind refusal where the platform admits one.

The integration changed no rule of the discovery contract other than these
two. The views show neither the frame tests nor the sandbox home, so they did
not change.

## Checks of these documents

The technical-communication checker reports no hard finding in the new and
changed documents. Ravel's own `scripts/ste.sh --strict` passes `CONTEXT.md`
with the pinned book tools. A planted control file fails the same command.
Local links and anchors resolve, and the view manifest parses. The rendered
views and their inspection are in Grove's task record.

The review integration ran the technical-communication checker on the three
changed documents, with no hard finding. It checked 91 local links and
anchors in the changed specifications, this record, the session
specification, the two new decision records and the views index. All
resolved, and a planted control failed. The view manifest still parses.

The narrowing of Grove's `ravel-local-sources-k78` ran the same two prose
checkers on its changed documents, with no hard finding. Planted controls
failed both checkers. Its local links and anchors resolve, and a stale Git
anchor fails the link check. A sweep for the Git vocabulary that the
narrowing removed found no remaining requirement for Ravel to run Git. The two
changed views render in a browser.

These checks cover the documents only. No product test, installed check or
real harness trial ran. The design claims no acquisition, confinement,
compatibility or harness behaviour in advance.
