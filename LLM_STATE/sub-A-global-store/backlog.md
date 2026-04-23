# Backlog — Sub-project A: Vault Location, Discovery, and Bootstrap

Implementation backlog for sub-project A. All tasks derive from the
design doc at
`{{PROJECT}}/docs/superpowers/specs/2026-04-13-sub-A-global-store-design.md`.
Consult the spec before starting any task.

Tasks are listed in approximately recommended order. The ordering
follows the dependency chain: core types → I/O primitives → resolution
functions → template authoring → commands → v0.1.0-legacy verification →
observability stub → tests → docs. The work phase picks the best next
task with input from the user.

## BEAM pivot + sub-F commitments absorbed (Session 14, inline rewrite)

The orchestrator's Session 9 committed Ravel to a persistent BEAM
daemon (Elixir/OTP) and sub-F locked the architectural commitments
(`project-root/`, `ontology/routing.dl`, `plan-catalog.md`, `experts/`,
Unix socket client protocol, daemon singleton lock collapsing sub-D's
per-plan-lock scope). Sub-A's design doc was rewritten inline in
Session 14 (2026-04-15) to absorb both pivots — see
`specs/2026-04-13-sub-A-global-store-design.md` for the authoritative
Elixir-native spec, and `memory.md § BEAM pivot + sub-F commitments
absorbed` for the net-change summary. The backlog below was translated
into Elixir/OTP idioms in a follow-up rewrite; task intents and
dependency relationships are preserved.

**Layout additions Tasks 7 (init) and 8 (clone) must scaffold**, beyond
what the original Session-7 design listed:

- Tracked at vault root: `daemon.toml` (two-section stub),
  `ontology/` directory with minimal `.dl` stubs
  (`vocabulary.dl`, `relations.dl`, `rules.dl`, `routing.dl`) and an
  empty `views/` subdirectory, `plan-catalog.md` (machine-owned
  header, zero entries), empty `experts/` directory,
  `tui-keybindings.toml` (default keybindings stub, hot-reloaded by
  the TUI — introduced by sub-H brainstorm, Session 21, 2026-04-16).
- Gitignored under `runtime/`: `mailboxes/`,
  `staging/proposed-rules/` directories only — the daemon creates
  `daemon.sock` at boot, not at init. (The singleton lock is
  system-wide at `<runtime_dir>/ravel/daemon.lock`, not in the vault.)
- `<project>/ravel/plans/` is gone. A's symlink target is still
  `<project>/ravel/` (the container); the rename only affects
  Task 5's walk-up logic, which now searches for
  `ravel/project-root/` instead of `ravel/plans/`.

**Tasks 11 and 12 (v0.1.0 legacy-path verification) reduce to
verification** that the Elixir daemon does not carry over the old
hardcoded `~/.ravel/` paths or YAML config — full Rust CLI deletion is
sub-G's migration scope, not sub-A's.

## Task Backlog

### Task 0 — Backlog Rust→Elixir translation sweep

**Category:** `meta`
**Status:** `done`
**Dependencies:** none

**Description:**

One-shot rewrite of Tasks 1–15 to replace Rust
idioms (`serde`, `include_str!`, `PathBuf`, `chrono`, `scopeguard`,
`cargo`, trait/derive syntax, `src/foo.rs` layout) with the
Elixir/OTP equivalents recorded in the spec and in memory.md. The
task descriptions below are now authoritative — implementers do not
need to translate at task-start time.

**Results:**

Completed during the sub-A backlog rewrite session.
Task 0 is this rewrite itself; the caveat previously attached to the
BEAM-pivot section ("All tasks below were authored assuming a Rust
CLI") has been removed from the backlog and from memory.md.

---

### Task 1 — Core types + `@vault_schema_version` attribute

**Category:** `types`
**Status:** `not_started`
**Dependencies:** none

**Description:**

Author the core types that the rest of sub-project A
depends on. Land them first so sibling sub-projects (B, D, E, F) can
consume them as real types rather than stubs.

Types to define (per spec §A2, §A3, §A10), each implemented as an
Elixir module under `lib/ravel/vault/`:

- `Ravel.Vault.Marker` — `defstruct`/`@enforce_keys` sealed struct
  carrying the parsed `[vault]` table: `schema_version :: non_neg_integer()`,
  `created :: DateTime.t()`, `created_by_version :: String.t()`.
  Optional override sections (`[language_profiles.*]`,
  `[context_mappings.*]`) are parsed into `Ravel.Config.LanguageProfile`
  / `Ravel.Config` shapes; absent sections fall through to the
  compile-time-embedded binary defaults. Provide `decode/1` that
  takes raw TOML text (via the `Toml` hex package,
  `{:toml, "~> 0.7"}`) and returns `{:ok, %Marker{}} | {:error, term()}`;
  `encode/1` renders back to a TOML-serialisable map.
- `Ravel.Vault.UserConfig` — sealed struct with a single field
  `vault :: String.t()` (absolute path). `decode/1` parses a TOML
  string; `encode/1` writes it out. Designed so `[preferences]` and
  similar future sections can be added without breaking
  deserialisation.
- `Ravel.Vault.Roots` — plain struct with `vault :: String.t()`,
  `tier2 :: String.t()`, `tier1 :: String.t() | nil`. No
  TOML-parsing surface; this is an in-memory type only. Sketch lives
  verbatim in spec §A10.
- `@vault_schema_version 1` — module attribute on
  `Ravel.Vault`. Exposed via `Ravel.Vault.schema_version/0` for
  downstream modules that would otherwise bake the constant in.

Unit tests (TDD, under `test/ravel/vault/`): round-trip TOML
parse/encode for `%Marker{}` with and without override sections;
`%UserConfig{}` decode from minimal `vault = "..."` content;
`%Marker{}` decode rejects missing `[vault]` section and missing
`schema_version` field with the spec's exact message prefixes.

**Results:** _pending_

---

### Task 2 — `ravel.toml` marker file I/O

**Category:** `io`
**Status:** `not_started`
**Dependencies:** Task 1

**Description:**

Read/write functions for the vault marker file
(`<vault>/ravel.toml`). Per spec §A3 and the reference algorithm in
the design doc. Author on `Ravel.Vault.Marker` (or a
`Ravel.Vault.MarkerIO` sub-module if preferred for test isolation).

API to author:

- `read_marker(vault_root :: String.t()) :: {:ok, %Marker{}} | no_return()`
  — reads `<vault>/ravel.toml`, parses it, returns the marker.
  Raises (`raise`) with the specific message strings from spec §A3:
  - `"not a Ravel vault (missing ravel.toml): <path>"`
  - `"invalid ravel.toml: <path>: <parse error>"`
- `write_marker(vault_root :: String.t(), marker :: %Marker{}) :: :ok`
  — atomic write-temp (`File.write!/2` to `ravel.toml.tmp`) +
  `File.rename!/2`. Use the same pattern as `write_user_config/1`.
- `check_schema_compatibility(marker :: %Marker{}) :: :ok | no_return()`
  — raises with the spec's exact message string if
  `marker.schema_version > Ravel.Vault.schema_version()`.

Unit tests: missing file → expected raise; malformed TOML → expected
raise; newer schema_version → expected raise; valid marker
round-trips through write/read. Use `ExUnit.Case, async: true` with
per-test tmpdirs created via `Briefly` (or equivalent) and
`on_exit/1` cleanup.

**Results:** _pending_

---

### Task 3 — User config file I/O

**Category:** `io`
**Status:** `not_started`
**Dependencies:** Task 1

**Description:**

Read/write the user-level config file at the
platform-specific location via
`:filename.basedir(:user_config, "ravel") |> Path.join("config.toml")`.
Per spec §A2.

API to author on `Ravel.Vault.UserConfig`:

- `path!/0 :: String.t()` — returns the platform path; raises if
  `:filename.basedir/2` returns an empty list.
- `read/0 :: {:ok, %UserConfig{}} | {:ok, nil} | {:error, term()}`
  — returns `{:ok, nil}` when the file doesn't exist, `{:ok, cfg}`
  when it parses, `{:error, _}` on parse error. Note: missing file
  is the normal "no vault configured yet" state, not an error.
- `write!(cfg :: %UserConfig{}) :: :ok` — creates the parent
  directory via `File.mkdir_p!/1` if missing, writes atomically
  (`File.write!/2` to `<path>.tmp` then `File.rename!/2`).

Unit tests: round-trip; missing file → `{:ok, nil}`; malformed TOML
→ `{:error, _}`; atomic write leaves no temp file on success; atomic
write leaves original file intact on temp-write failure (simulate by
mocking `File.rename/2` or making the target directory read-only).
Use a per-test tmpdir with a `RAVEL_CONFIG_HOME` override or a
dependency-injected path for isolation; track env vars via
`on_exit/1`.

**Results:** _pending_

---

### Task 4 — `resolve_vault` + `verify_vault`

**Category:** `resolution`
**Status:** `not_started`
**Dependencies:** Task 2, Task 3

**Description:**

Implement the vault discovery precedence chain from
spec §A1 and the reference algorithm at the end of the design doc,
as `Ravel.Vault.resolve/1` and `Ravel.Vault.verify/1`.

API to author:

- `resolve(flag :: String.t() | nil) :: {:ok, String.t()} | no_return()`
  — applies the precedence chain: `--vault` flag → `RAVEL_VAULT`
  env var (non-empty) → user config → raise with "no vault
  configured" guidance. Returns an absolute path resolved via
  `Path.expand/1`.
- `verify(path :: String.t()) :: {:ok, String.t()} | no_return()`
  — `Path.expand/1`s the path, rejects a symlinked vault root with
  `File.lstat!/1`, reads the marker, calls
  `check_schema_compatibility/1`, returns the absolute path.
  Raises per spec §A1 / §A3.

Unit tests: each precedence level (flag > env > user config >
raise); empty env var falls through; missing user config falls
through to raise; verification rejects non-vault directories;
verification rejects a symlinked vault root; `Path.expand/1`
collapses `..`/`~` correctly. Tests use tmpdirs for all vault
locations and guard env vars with `on_exit/1` blocks.

**Results:** _pending_

---

### Task 5 — `Ravel.Vault.Roots.resolve/2` + `derive_tier1_from_plan/1`

**Category:** `resolution`
**Status:** `not_started`
**Dependencies:** Task 4

**Description:**

Tier 1 / Tier 2 resolution per spec §A10. The module
skeleton (with `@enforce_keys` + `defstruct` + typespecs) is shown
verbatim in the spec and can be copied into
`lib/ravel/vault/roots.ex`.

API to author:

- `resolve(vault :: String.t(), active_plan :: String.t() | nil) :: {:ok, %Roots{}} | no_return()`
  — derives Tier 2 from vault, Tier 1 from the walk-up logic or the
  env var override. Verifies both resolved roots exist as
  directories.
- `derive_tier1_from_plan(plan_path :: String.t()) :: String.t() | no_return()`
  — walks up from the plan path looking for the topmost `ravel/`
  ancestor that has a `project-root/` sibling (i.e., the ancestor
  sits at `<project>/ravel/project-root/...`), returns
  `Path.join(that_ravel_dir, "knowledge")`. Per sub-F invariants #3
  and #4 the walk-up always terminates uniquely.

Unit tests: default derivation (Tier 2 = `<vault>/knowledge`, Tier 1
walked from plan); env var overrides (`RAVEL_TIER1_ROOT`,
`RAVEL_TIER2_ROOT`) bypass defaults; walk-up succeeds on
well-formed plan path ending in `ravel/project-root/...`; walk-up
raises on malformed path lacking a `ravel/project-root/`
ancestor; missing Tier 2 directory raises; `active_plan: nil`
yields `tier1: nil`.

**Results:** _pending_

---

### Task 6 — Author the embedded `.obsidian/` template

**Category:** `template`
**Status:** `not_started`
**Dependencies:** none

**Dependency context:**

none (parallelisable with Tasks 1–5)

**Description:**

Create the `priv/obsidian/` directory in the Ravel
source tree and author the minimal template files that `ravel init`
materialises per spec §A4 and §A6 step 4. `priv/` is the standard
Elixir home for compile-time-embedded assets and matches sub-B's
`priv/prompts/` precedent.

Files to author under `priv/obsidian/`:

- `community-plugins.json` — lists Dataview as enabled.
- `core-plugins.json` — sensible defaults (file-explorer, search,
  graph, backlinks, outline, tag-pane, page-preview enabled).
- `app.json` — canonical settings: `alwaysUpdateLinks = true`,
  `useMarkdownLinks = false` (wikilinks preferred),
  `newLinkFormat = "shortest"`, other sensible defaults.
- `plugins/dataview/data.json` — default Dataview config
  (refresh-on-file-change, inline queries enabled).
- `snippets/ravel.css` — empty file as a placeholder for future CSS
  styling; created so the directory exists and Obsidian-ingested
  templates can use it.

Wire the files into the binary via `@external_resource` +
`File.read!/1` at the use-site in Task 7. No Elixir code changes in
this task — just the template files.

**Deferred refinement:** if sub-project I's Obsidian coverage
brainstorm has landed before this task is executed, consult I's
design doc and incorporate recommendations into the template. If
I has not landed, ship the minimal template listed above; I's
later brainstorm can refine in place.

Unit tests: not applicable (template authoring task); Task 14's
integration tests exercise materialisation end-to-end.

**Results:** _pending_

---

### Task 7 — `ravel init <path>` fresh vault command

**Category:** `command`
**Status:** `not_started`
**Dependencies:** Task 2, Task 3, Task 6

**Description:**

Implement the fresh init flow per spec §A6 as an
escript-style bootstrap subcommand in
`lib/ravel/cli/init.ex` (dispatched from `Ravel.CLI.main/1`).
Replaces v0.1.0's `run_init` entry point.

Step-by-step per spec §A6:

1. Resolve target (default `Path.join(System.user_home!(), "Ravel-vault")`
   when omitted; otherwise `Path.expand/1`).
2. Pre-flight checks: target doesn't exist (`File.exists?/1`),
   parent exists, `System.find_executable("git")` is non-nil.
3. Create directory tree via `File.mkdir_p!/1` (the full list in
   §A6 step 3).
4. Materialise embedded `.obsidian/` template. Each template file
   is embedded via `@external_resource` + `File.read!/1` from
   `priv/obsidian/`; init writes each to its target.
5. Write `ravel.toml` using `Ravel.Vault.Marker.write_marker/2`,
   with `created` rendered via
   `DateTime.utc_now() |> DateTime.to_iso8601()` and
   `created_by_version` read from `Mix.Project.config()[:version]`
   embedded at compile time.
6. Write minimal `daemon.toml` stub (spec §A6 step 6) and scaffold
   `ontology/` with minimal `.dl` stubs.
7. Write empty `plan-catalog.md` with the machine-owned header.
8. Write `.gitignore` with the tracked-vs-gitignored policy.
9. `git init`, `git add .`, `git commit` via `System.cmd/3` — each
   raises on non-zero exit with git stderr in the message.
10. Update user config via `Ravel.Vault.UserConfig.write!/1` (with
    "different vault exists" notice check).
11. Print success with next-step hints.

Integration tests (under `test/ravel/cli/init_test.exs`): fresh
init in tmpdir succeeds; pre-existing target raises; missing parent
raises; missing git raises (via PATH manipulation inside
`on_exit/1`-guarded `System.put_env/2` scope); user config update
both writes fresh and preserves different-vault state;
`Ravel.Vault.verify/1` passes against the created vault.

**Results:** _pending_

---

### Task 8 — `ravel init --from <git-url>` clone vault command

**Category:** `command`
**Status:** `not_started`
**Dependencies:** Task 2, Task 3, Task 7

**Description:**

Implement the clone init flow per spec §A7 as a
branch inside the `Ravel.CLI.Init` dispatch.

Step-by-step per spec §A7:

1. Resolve target (default `~/Ravel-vault/` when omitted).
2. Pre-flight checks: target doesn't exist, parent exists, `git` on
   PATH, URL non-empty.
3. `git clone <url> <path>` via `System.cmd/3` — raise with git
   stderr on non-zero exit.
4. `Ravel.Vault.verify/1` — three failure modes all trigger
   `File.rm_rf!/1` cleanup.
5. Create the gitignored runtime tree
   (`runtime/{staging,staging/proposed-rules,mailboxes,interrupts,events}`,
   `cache/`, `projects/`) via `File.mkdir_p!/1`.
6. Update user config.
7. Print success.

Integration tests: clone from a local bare-repo fixture succeeds;
clone of a non-vault repo triggers cleanup and raises; schema
version mismatch triggers cleanup and raises.

**Results:** _pending_

---

### Task 9 — `ravel config use-vault <path>` command

**Category:** `command`
**Status:** `not_started`
**Dependencies:** Task 3, Task 4

**Description:**

Implement the vault-switching command per spec §A8
in `lib/ravel/cli/config.ex`.

Steps:

1. Parse the path argument, make absolute via `Path.expand/1`.
2. Call `Ravel.Vault.verify/1` on the target — raise if invalid.
3. Create user config parent directory if missing via
   `File.mkdir_p!/1`.
4. Write `%UserConfig{vault: path}` atomically via
   `Ravel.Vault.UserConfig.write!/1`, preserving any future
   `[preferences]` section.
5. Print confirmation.

Integration tests: switch from vault A to vault B; refuse to switch
to a non-vault target; refuse to switch to a newer-schema vault;
assert the user config file contents after switch.

**Results:** _pending_

---

### Task 10 — `ravel adopt-project <project-path>` command

**Category:** `command`
**Status:** `not_started`
**Dependencies:** Task 4

**Description:**

Implement the per-machine project mounting command
per spec §A9 in `lib/ravel/cli/adopt_project.ex`.

Steps:

1. Verify `<project-path>/ravel/` exists — raise otherwise.
2. Verify `<project-path>/ravel/project-root/plan-state.md`
   exists — raise otherwise with the "run `ravel daemon
   --init-project` first" guidance.
3. Compute symlink name as `String.downcase(Path.basename(project_path))`.
4. Resolve the active vault via `Ravel.Vault.resolve/1`.
5. Create symlink via `File.ln_s!/2` at
   `Path.join([vault, "projects", name])` pointing at
   `Path.expand(Path.join(project_path, "ravel"))`. Raise if a
   symlink or file with that name already exists.
6. If a daemon is running against the active vault (detected by
   reading the system-wide lock file at
   `<runtime_dir>/ravel/daemon.lock` and comparing the recorded
   vault path), send a `:rescan` NDJSON message over
   `runtime/daemon.sock`. Skip the notification if no daemon is
   serving this vault.
7. Print confirmation.

Integration tests: successful adoption creates the symlink with
the expected absolute target; project without `ravel/` subdir
raises; project without `project-root/plan-state.md` raises;
duplicate adoption raises; the resulting symlink is readable and
points at the expected path.

**Results:** _pending_

---

### Task 11 — Verify the Elixir daemon carries no `~/.ravel/` hardcoded paths

**Category:** `cleanup`
**Status:** `not_started`
**Dependencies:** Task 4, Task 5, Task 7, Task 8, Task 9, Task 10

**Description:**

The full Rust CLI deletion is sub-G's migration
scope, so this task reduces to a verification sweep over the new
Elixir daemon and its tests. Ensure that:

- No module under `lib/ravel/` references
  `Path.join(System.user_home!(), ".ravel")` or any literal
  `.ravel` path except in historical-notes strings.
- Every daemon entry point that needs the vault receives a
  `%Ravel.Vault.Roots{}` (or the resolved vault path) via the
  `Ravel.Vault` GenServer rather than reconstructing from the home
  directory.
- A `--vault` CLI flag is plumbed through `Ravel.CLI.main/1` (via
  `OptionParser`) for every bootstrap subcommand that resolves the
  vault (`daemon`, `config use-vault`, `adopt-project`; `init`
  creates rather than consumes, as does `init --from`).
- Any `Ravel.CLI` command handler that used to accept a
  `ravel_dir` path is updated to accept a `%Roots{}` argument.

Verification: a content search across `lib/` and `test/` for the
literal `.ravel` returns zero matches outside deliberate history
notes; `mix test` and `mix dialyzer` pass.

**Results:** _pending_

---

### Task 12 — Verify the Elixir daemon carries no `config.yml` reads; move language profiles to embedded defaults

**Category:** `cleanup`
**Status:** `not_started`
**Dependencies:** Task 11

**Description:**

v0.1.0's `src/config.rs` exposed `Config::load(dir)`
which read `<dir>/config.yml`. That call site has no analogue in the
Elixir daemon, but sub-A needs to positively confirm the same
shape in Elixir-native form. Refactor so that:

- `Ravel.Config.default/0` (module returning a default
  `%Ravel.Config{}` struct) is the source of language profiles and
  context mappings, populated from compile-time-embedded
  `priv/defaults/config.toml` via `@external_resource` +
  `File.read!/1`.
- Optional overrides come from the `[language_profiles.*]` and
  `[context_mappings.*]` sections of `<vault>/ravel.toml`, loaded
  via `Ravel.Vault.Marker.read_marker/1` from Task 2 and merged
  over the defaults using a deterministic deep-merge helper.
- No Elixir code reads `config.yml`; no `Ravel.Config.load/1` or
  `Ravel.Config.save/1` function exists.
- No `config.yml` fixtures under `test/fixtures/`; `config_test.exs`
  covers the merge semantics against TOML input.

Verification: content search for `config.yml`, `Ravel.Config.load`,
`Ravel.Config.save`, `yaml_elixir`, `YamlElixir` across `lib/` and
`test/` returns zero matches; `mix test` passes.

**Results:** _pending_

---

### Task 13 — Observability adoption stub for sub-project M

**Category:** `observability`
**Status:** `not_started`
**Dependencies:** Task 4, Task 7, Task 10

**Description:**

Per sub-project M's "cross-cutting brainstorms own
their own sibling adoption stubs" discipline, emit typed
`Ravel.Event.*` structs at the boundaries sub-project A
introduces, routed through the sub-M `:telemetry` pipeline. Stub
format follows M's spec at
`{{PROJECT}}/docs/superpowers/specs/2026-04-13-sub-M-observability-design.md`
and M's implementation plan at
`{{PROJECT}}/LLM_STATE/sub-M-observability/`.

Boundaries to instrument:

- **Vault discovery**: emit `%Ravel.Event.VaultResolved{source:
  source, path: path}` after `Ravel.Vault.resolve/1` succeeds,
  where `source` is one of `:flag`, `:env_var`, `:user_config`.
  Helps diagnose "which config layer won" during incidents.
- **Verification failure**: emit typed variants for
  `VaultMarkerMissing{path}`, `VaultMarkerInvalid{path, error}`,
  `VaultSchemaTooNew{path, found, supported}`.
- **Init success**: emit `VaultInitialised{path, schema_version,
  fresh_or_clone}`.
- **Adopt-project success**: emit `ProjectAdopted{vault_path,
  project_path, symlink_name}`.

If M's framework hasn't landed yet at task time, author the event
structs as plain `Logger.info/1` / `Logger.error/1` calls with
the same structured metadata (`Logger.metadata/1`). The typed-event
`:telemetry.execute/3` swap is mechanical once M's implementation
lands; M's migration discipline (parallel-emit + verification
window) applies.

Verification: each boundary site has an observability emission
point; event payloads match M's spec shape; at minimum `mix compile
--warnings-as-errors` passes.

**Results:** _pending_

---

### Task 14 — Integration tests for init/clone/resolve

**Category:** `tests`
**Status:** `not_started`
**Dependencies:** Task 7, Task 8, Task 9, Task 10

**Description:**

End-to-end integration tests that exercise the
full discovery → init → adopt flow against fixture directories
under tmpdirs. Complements the per-task unit tests with
cross-boundary scenarios. Live under
`test/integration/vault_bootstrap_test.exs`.

Scenarios to cover:

- **Fresh init flow**: `ravel init $tmpdir/vault` →
  `Ravel.Vault.verify/1` passes → `RAVEL_VAULT=$tmpdir/vault ravel
  status` succeeds.
- **Clone flow**: create a bare git fixture containing a minimal
  vault, `ravel init --from $fixture_url $tmpdir/vault` → same
  downstream.
- **Discovery precedence**: with all three layers set to distinct
  valid vaults, confirm `--vault` wins; with `--vault` omitted,
  env var wins; with both omitted, user config wins.
- **Adopt-project round trip**: `ravel adopt-project
  $tmpdir/fake-proj` → symlink exists at
  `$tmpdir/vault/projects/fake-proj` → readable.
- **Test override env vars**: `RAVEL_TIER1_ROOT=$tmpdir/t1
  RAVEL_TIER2_ROOT=$tmpdir/t2` bypass the default derivation.

All tests use per-test tmpdirs and `on_exit/1`-guarded
`System.put_env/2` + `System.delete_env/1` calls to avoid
cross-contamination. Tests opt out of `async: true` wherever they
mutate process-wide env.

**Results:** _pending_

---

### Task 15 — Update user-facing docs

**Category:** `docs`
**Status:** `not_started`
**Dependencies:** Task 11, Task 12

**Description:**

Update the documentation files that reference
v0.1.0's `~/.ravel/` location. Per user feedback memory
(`feedback_documentation.md`), documentation is a first-class
deliverable.

Files to update:

- `{{PROJECT}}/README.md` — replace any `~/.ravel/` references
  with the new discovery model. Update the quick-start to show
  `ravel init ~/Ravel-vault`.
- `{{PROJECT}}/docs/reference.md` — the `init` command section
  must describe the new flow (fresh + clone subcommands, user
  config update, the marker file, default path). Update the
  `status` and `query` sections to drop `~/.ravel/` references.
- `{{PROJECT}}/docs/user-guide.md` — replace the "Getting started"
  walkthrough with the new discovery / init flow.
- `{{PROJECT}}/docs/configuration.md` — describe the user config
  file location (Linux/macOS/Windows), the override sections
  inside `ravel.toml`, and the env vars (`RAVEL_VAULT`,
  `RAVEL_TIER1_ROOT`, `RAVEL_TIER2_ROOT`).

Verification: content search for `.ravel` under `docs/` returns
zero matches (except historical research notes under
`docs/research-sources.md` if any); content search for `.ravel` in
`README.md` returns zero matches.

**Results:** _pending_

---

### Task 16 — Absorb Sub-R: vault layout amendment (`ontology/` replaces `tag-vocabulary.md`)

**Category:** `r-adoption`
**Status:** `not_started`
**Dependencies:** none

**Dependency context:**

none (design doc amendment — no code dependency)

**Description:**

Per Sub-R design doc §10.2. Update A's design doc
§A4 vault layout:

1. Remove `tag-vocabulary.md` reservation from the layout
2. Remove `routing.ex` from the layout (now `ontology/routing.dl`)
3. Add `ontology/` directory with contents: `vocabulary.dl`,
   `relations.dl`, `rules.dl`, `routing.dl`, `views/` (auto-generated)
4. Add `runtime/staging/proposed-rules/` for Level 2 agent proposals
5. Evaluate `ravel.toml` schema version bump
6. Update `init` command scaffolding to create `ontology/` directory
   with shipped `.dl` files from `priv/ontology/`

**Note:** The design doc amendment itself is being done by a Sub-R
adoption agent. This backlog task covers the implementation-time
changes to `init` scaffolding.

**Results:** _pending_

---
