# Changelog

## Unreleased

- Deliver Ravel as one executable with three Rhai examples and Rust dependency
  notices under `share/ravel/`. Remove the retired Bun worker, TypeScript SDK
  and their delivery tooling. Installation, archives and the Homebrew formula
  now exercise Rhai directly.

- Remove retired TypeScript worker prerequisites from development setup, builds,
  checks and native-entry checks. Keep the checkout-local Bun for book tools.
- Retire the frozen native-entry campaign drivers and their Linux Git-object
  staging. Preserve historical receipts and current Rust, Rhai and native-entry
  checks.
- Move configuration scripts and their request to schema 4. The request gains
  `originalPrompt`, the caller's prompt unchanged, and `skills`, which is `()`.
  `prompt` holds the same bytes as `originalPrompt`. A script that exports
  `SCHEMA_VERSION` 3 refuses as `unsupported_version`, exit 3. Export 4, and
  read `request.originalPrompt` where the script analyses the task. The
  shipped `llm.rhai` sends `originalPrompt` to its classifier. The JSON routing
  notice gains the member `skills`, which is `null`.
- Check the evaluate frame before the worker receives it. The frame bound is
  38,541,312 bytes, and every request within the public bounds fits it. A
  larger frame refuses as `malformed_input`, stage `cli`, exit 2, and the
  refusal names the largest members. Only a very long kind, working directory
  or task-file path can pass the bound.
- Add `ravel skills resolve` for local skill sources. The settings file accepts
  a `skills` object, with `schemaVersion` 1, `sources`, `mandatory` and
  `resolutionTimeoutMs`. Ravel copies each local source into an immutable
  snapshot under `~/.cache/ravel/skills/v1`, publishes a directory descriptor
  and prints the loading instructions. `--json` prints one object. With
  delivery disabled, the command prints nothing and creates no cache. The
  snapshots and descriptors stay after the session, after a change or removal
  of the source and after a change of the settings. Ravel has no collector
  and no prune command. In this
  build, a Git source refuses. A later stage of this release adds it.
- Deliver skill directories in `start-session`. With delivery enabled, Ravel
  resolves the sources after it reads the settings, the inputs and the entry,
  and before confinement and the worker. `request.prompt` holds the discovery
  instructions, the line `Caller task` and the caller's bytes, unchanged,
  between two delimiter lines with a random nonce. `request.originalPrompt`
  keeps the caller's prompt, and `request.skills` holds `descriptorPath`,
  `sources`, `directories` and `mandatory`. The managed process and each
  harness run receive `RAVEL_SKILL_DIRECTORIES`. That value replaces an
  inherited one, and a nested `start-session` gives its own value only to its
  own managed process. The JSON routing notice gives the `skills` object
  with prompt byte counts and digests. A prepared
  prompt over 1 MiB refuses as `skill_limit_exceeded`. A source that overlaps
  the directory of the configuration entry refuses as
  `skill_access_unavailable`. The example `harness-table.rhai` gives its
  lead harness each snapshot root.
- Grant confined sessions the reads of skill delivery. Under `--confine`, the
  managed process and each harness run read the descriptor directory and each
  snapshot tree, read-only, on macOS `sandbox-exec` and Linux bubblewrap. The
  grants exclude the staging area, the snapshot manifests and the sources.
  A granted directory that overlaps the configuration directory, the settings
  file or the result channel of an enclosing session refuses as
  `skill_access_unavailable`, exit 3, also through an alias. So does a
  writable grant that overlaps the skill cache, and a granted directory that
  the sandbox cannot list. Ravel never retries unconfined. A nested
  `start-session` in a confined parent cannot read the owner settings.
- Check each local capture and each cache object. A source that changes while
  Ravel copies it refuses as `skill_source_changed`, with a remedy to try
  again. Ravel reads back each copy, and names that the filesystem of the cache
  cannot keep apart or spell exactly refuse as `skill_resource_invalid`. A run
  that finds an object with the same name reads it again in full, and a
  tampered object refuses as `skill_cache_corrupt`. Ravel examines each
  directory on the path to the cache and keeps the cache open while it writes.
  An unsafe or changed path refuses as `skill_cache_unavailable`. A failed removal of its own
  temporary directory after a successful run refuses as
  `skill_cleanup_failed`, exit 5.
- Bound each skill resolution. At most 16 sources, 256 search directories and
  256 mandatory files can be configured. A mandatory file has at most 65,536
  bytes, a regular file 16 MiB, one source 128 MiB, all sources 512 MiB and
  one source 20,000 entries. The descriptor and the discovery instructions
  each have at most 1 MiB. A larger value refuses as `skill_limit_exceeded`,
  exit 3, with the value, the bound and the subject. Ravel never truncates or
  omits guidance to meet a bound.
- Enforce `skills.resolutionTimeoutMs` for capture, checks and publication. A
  resolution that passes it refuses as `skill_resolution_timeout`, exit 124,
  with the deadline and the active source and operation. INT, TERM or HUP
  during a resolution refuses as `skill_resolution_cancelled`, exit 128 plus
  the signal number. Ravel removes its staging and raises the same signal
  again. The signal wins over another refusal, which the report keeps. Both
  `skills resolve` and `start-session` apply these rules.
- Flush the copied files of a snapshot with one device flush before
  publication. On macOS each file had its own full flush, at about 5 ms for
  each file. A resolution of 2,000 small files took 10 seconds and now takes
  less than 1 second.
- Refuse a settings file that repeats a member name in any object, as
  `settings_invalid`, stage `cli`, exit 2. The `location` names the repeated
  member.
- Narrow the sandbox home of `task harness:smoke:claude` and
  `task harness:smoke:codex`. The home links one sign-in path from the real
  home: `Library/Keychains` for Claude Code on macOS,
  `.claude/.credentials.json` on Linux and `.codex/auth.json` for Codex. A
  Codex run sets `CODEX_HOME` to the sandbox `.codex`. A Claude Code run
  refuses before any launch of the harness when the environment holds
  `CLAUDE_CONFIG_DIR`, `CLAUDE_SECURESTORAGE_CONFIG_DIR`,
  `CLAUDE_CODE_PLUGIN_CACHE_DIR`, `CLAUDE_CODE_PLUGIN_DIRS` or
  `CLAUDE_CODE_PLUGIN_SEED_DIR`. A Codex run refuses a `CODEX_HOME` in the
  real home.
- List the links and the configuration directory of each smoke run. Fail the
  run when the harness replaces the sign-in link. `task harness:smoke:test`
  proves each refusal with fake harnesses in a fake real home.

## v1.1.0

- Correct five failure paths. Only the behaviour after a failure changes. The
  command-line arguments, the owner configuration and the results of a
  successful session stay the same.
  - Ravel keeps a cancellation when the wait for the managed process fails.
    It also keeps the cancellation when the reap of that process fails.
  - If Ravel cannot check that it reaped the policy worker, the session stops
    before the managed process starts.
  - After Ravel acquires only some of the resources for a session, it reports
    each failure of the cleanup that follows.
  - The terminal recovers after the exec of the managed process fails.
  - When its process starts, Ravel checks each signal and terminal
    transition.
- Add the field `operationReports` to the JSON notice that `--json` prints. The
  field is an array of the reports that Ravel kept while it supervised the
  managed process. The array is empty when Ravel kept no report.
- Reword the help text of `ravel end-session`. The LLM sends the result, and
  Ravel keeps the first result that it receives. The behaviour does not change.
- Describe `--policy-env` in the help text of `ravel start-session`.
- Add the book, a user guide for Ravel. Each release publishes it on GitHub
  Pages.
- Measure Ravel on aarch64 macOS and aarch64 GNU/Linux. The x86-64 GNU/Linux
  target runs only under emulation. The measurements of that target come from
  the emulation.

## v1.0.0

- Declare the two-verb public CLI stable: `ravel start-session` and
  `ravel end-session RESULT`, with opaque caller-owned results, owner policy
  routing and confinement behaving exactly as released in 0.1.0.
- Commit to semantic versioning for that CLI: compatible changes arrive in minor
  and patch releases; incompatible changes require a new major version.
- Ship the same matching front and private worker, generic SDK/examples and
  runtime notices as 0.1.0, with no source or behavior change.

## v0.1.0

- Introduce the independent Ravel session process manager with `start-session`
  and `end-session RESULT`, preserving opaque results and owner policy routing.
- Ship a matching front and private worker, generic SDK/examples and runtime
  notices without a Grove or host TypeScript runtime dependency.
- Add independent release tasks, exact three-target archives, installed-layout
  smoke checks, Linux glibc/CPU-floor controls and a tested Homebrew formula.
