# Backlog — Sub-project B: Phase Cycle Reimplementation

Implementation backlog for sub-project B. All tasks derive from the design
doc at
`{{PROJECT}}/docs/superpowers/specs/2026-04-12-sub-B-phase-cycle-design.md`.
Consult the spec before starting any task.

**Design doc rewritten inline for Elixir/BEAM.** In Session 12
(2026-04-15) of the parent orchestrator plan, the three pending amendment
tasks (Sub-C contracts, Ravel-Lite 2026-04 overhaul, BEAM pivot) were
absorbed inline across §1–§6 of the design doc, following the sub-C
precedent (no supersede layer). The design is now Elixir-native with a
four-phase cycle (`work → reflect → compact → triage` with conditional
compact), `PhaseRunner` hosted inside a sub-F `PlanActor` GenServer,
`plan-state.md` schema pruned per sub-F, `{{VAULT_CATALOG}}` placeholder
replacing `{{RELATED_PLANS}}`, `pre-work.sh` hook, and F's
`DispatchProcessor` / `QueryProcessor` as phase-exit hooks on non-compact
phases. **The implementation task list below has been rewritten against
Session-12's Elixir-native contracts** (this rewrite landed as the "Rewrite
downstream implementation tasks" gate task, now complete).

**Key scope handoffs to sub-F** (Session 12 decisions):

- **No B-owned TUI.** The ratatui / Headless / IPC driver tasks are
  deleted. Sub-F owns the separate Rust TUI client binary; B emits typed
  events through sub-M's `:telemetry` boundary and receives action
  messages via the PlanActor's mailbox.
- **No B-owned daemon startup / shutdown.** Daemon lifecycle is sub-F's
  scope. B's entry point is `PlanActor.handle_info/2` dispatch for
  `{:run_phase, _}` messages. The legacy "full startup sequence" and
  "shutdown sequence" tasks are deleted.
- **No per-plan advisory lock.** Sub-F's daemon-singleton lock (sub-D's
  scope collapsed per §6) replaces the per-plan `fs2`/`:flock` stub.

**Task 0 (the Obsidian + symlinks validation spike) PASSED on both
macOS and Linux** in Session 5 (2026-04-13). Evidence at
`tests/fixtures/obsidian-validation/results/{macos,linux}/` (commit
`98ef7db`). Implementation blockers cleared.

## Task Backlog

### Task 0 — Obsidian + symlinks validation spike (cross-platform) `[spike]`
- **Status:** done
- **Dependencies:** none (blocks everything else)
- **Description:** Pre-implementation blocker. Validate that Obsidian on
  macOS and Linux correctly renders, indexes, and navigates a vault whose
  `projects/` subdirectory contains symlinks into a sibling project's
  `ravel/` directory. Use TestAnyware's golden images
  (`testanyware-golden-macos-tahoe` and `testanyware-golden-linux-24.04`) per
  the workflow in §5.2 of the spec. Consult
  `{{DEV_ROOT}}/TestAnyware/LLM_INSTRUCTIONS.md`
  for VM commands. Validation checks (same list for both platforms):
  (1) file tree rendering, (2) file opening, (3) graph view follows
  symlinks, (4) wikilink resolution across symlinks, (5) Dataview queries
  resolve symlinked frontmatter, (6) file-watcher picks up external edits
  via the symlinked path, (7) backlinks across symlink boundaries.
  Capture evidence via `testanyware agent snapshot --mode interact` and
  `testanyware screenshot` for every check. Commit captured outputs to
  `tests/fixtures/obsidian-validation/`. Acceptance: all seven checks
  pass on both platforms. If any check fails on Linux (a v1 target),
  stop and open a brainstorm discussion for the hard-copy + two-way-sync
  fallback layout.
- **Results:** **PASS on both platforms.** Executed at the orchestrator-plan
  level in Session 5 (2026-04-13) of `ravel-orchestrator`, driven
  end-to-end via the `testanyware` CLI against
  `testanyware-golden-macos-tahoe` and `testanyware-golden-linux-24.04` with
  Obsidian 1.12.7 + Dataview 0.5.67 pinned identically. The orchestrator
  spike's six-check enumeration (Dataview / graph view / backlinks / file
  tree+open / file watcher / safety checks) covers this task's seven
  checks (file tree+file opening collapsed into a single explorer check,
  wikilinks subsumed by Dataview cross-boundary queries). All checks
  passed on both platforms. Evidence:
  `{{PROJECT}}/tests/fixtures/obsidian-validation/results/{macos,linux}/`
  with per-platform `result.md` summary tables and per-check screenshots
  + OCR transcripts. Commit `98ef7db` ("test: add Obsidian symlink
  validation spike fixture and evidence"). Architectural consequence
  (vault-as-view-over-symlinks stands; hard-copy fallback NOT activated)
  recorded in `{{DEV_ROOT}}/Ravel/LLM_STATE/ravel-orchestrator/memory.md`
  under the "Dedicated Ravel-vault" decision. **Task 0 is cleared;
  all downstream B implementation tasks are now unblocked.** Two
  platform-specific operational notes surfaced and should be remembered
  by any future B GUI work on the same images: (1) Electron-in-virtio-gpu-
  under-tart on ARM64 Ubuntu requires `--disable-gpu --no-sandbox` for
  visible rendering; (2) macOS Notification Center widgets in the Tahoe
  golden image occlude part of the Obsidian window area and should be
  dismissed at VM setup time.

### Absorb Sub-C trait amendments into B's design + types `[types]` `[amendment]`
- **Status:** done (Session 12, 2026-04-15)
- **Dependencies:** Task 0
- **Description:** Sub-project C's design doc was **fully rewritten in
  Session 11 (2026-04-15) for Elixir/BEAM** — an inline rewrite, not a
  supersede layer. C's §11.1 now defines concrete B-C contracts that
  replace the original five Rust-era amendments from Session 6.

  **Amendments 1–3 eliminated.** The Rust trait-object plumbing
  (`&mut self` → `&self`, `Box<dyn>` → `Arc<dyn>`, `Send + Sync`)
  has no BEAM analogue. B consumes the session GenServer via
  `attach_consumer/2` and `handle_info/2`.

  **Two substantive requirements survive** (source:
  `{{PROJECT}}/docs/superpowers/specs/2026-04-13-sub-C-adapters-design.md`
  §11.1):

  1. **`%Ravel.Event.SessionLifecycle{}` is a consumed typed event.**
     B's phase-cycle machine pattern-matches on
     `%SessionLifecycle{transition: :ready}`,
     `%SessionLifecycle{transition: {:turn_complete, _}}`, and
     `%SessionLifecycle{transition: {:exited, _}}` as the protocol-level
     signals. These are **distinct from task-level completion signals**
     (see requirement 2).
  2. **B's executor runs a sliding-buffer sentinel matcher on every
     `%HarnessOutput{kind: :stdout}` event** to detect task-level
     completion. Sentinels are per-phase, defined in the phase prompt
     files. When the matcher fires, B's executor emits an internal
     `{:phase_completion_detected, sentinel}` message that the phase
     state machine consumes as the trigger to wind down the session and
     transition phases. Sentinel detection lives in B (not C) because
     sentinels are coupled to phase prompts (which B owns) and the
     mechanism is harness-agnostic. The matcher must be sliding-buffer
     based (sentinel may span multiple chunks) and configurable per
     phase. **Validated by the BEAM PTY spike** (Session 10, 2026-04-15;
     `spikes/beam_pty/`): sliding-buffer with window bounded to
     `sentinel_size - 1` bytes works across single-chunk, split, drip,
     false-prefix, and false-overlap cases. See memory.md for details.
- **Results:** Absorbed inline into the design doc's Session-12 rewrite.
  §4.1 now carries the full sub-C consumption contract (Requirements 1
  and 2) with the `%Ravel.Event.SessionLifecycle{}` pattern-match
  surface and the sliding-buffer sentinel matcher requirement. The
  Rust-era Amendments 1–3 are explicitly retired there; Appendix A Q1
  (post-write clarification) records the control-vs-observation
  disambiguation. §2.3.4 documents `LlmHarnessExecutor`'s
  `attach_consumer/2` + `handle_info/2` consumption loop. Session-12
  rewrite spans the whole doc; no supersede layer.

### Absorb Ravel-Lite 2026-04 overhaul into B's design + types `[amendment]`
- **Status:** done (Session 12, 2026-04-15)
- **Dependencies:** Task 0
- **Description:** Ravel-Lite's upstream shape shifted after B's
  brainstorm: four-phase cycle (work → reflect → compact → triage,
  compact conditional on a wc-word-count trigger), phase-file-factored
  composition (shared `phases/<phase>.md` + optional per-plan
  `prompt-<phase>.md` override), a `fixed-memory/memory-style.md` file
  read by both reflect and compact, an opt-in `pre-work.sh` executable
  hook invoked before every work phase, a new synthesised placeholder
  for related plans (later renamed `{{VAULT_CATALOG}}` by the BEAM
  pivot), a `related-plans.md` schema using Parents/Children only
  (siblings auto-discovered), and ISO 8601 UTC timestamps for
  session-log entries. All seven changes post-date B's 2026-04-12
  brainstorm and had to be absorbed into the design doc and backlog
  before downstream implementation work could start.
- **Results:** Absorbed inline into the design doc's Session-12 rewrite.
  All seven Ravel-Lite 2026-04 overhaul items are now load-bearing in
  the design: (a) four-phase cycle in `PhaseRunner` and `PlanState`
  (§2.3.2, §2.3.3, §3.3); (b) phase-composition shared-file + additive
  override pattern (§2.3.5, §2.5); (c) `fixed-memory/memory-style.md`
  read by both reflect and compact (§2.5, explicit invariant); (d)
  `pre-work.sh` hook (§2.3.3 step 2, module `Ravel.PreWorkHook`);
  (e) `{{VAULT_CATALOG}}` placeholder (§2.4 table + §2.3.5); (f)
  `related-plans.md` **deleted entirely** (§6 sub-G requirements call
  this out); (g) ISO 8601 session-log timestamps + latest-session.md
  write-then-append lifecycle (§2.7). Sub-G owns the user-facing
  rename and `related-plans.md` deletion during migration. Appendix A
  Q18 records the seven-point absorption summary.

### Absorb BEAM pivot into B's design, spec, and backlog `[amendment]`
- **Status:** done (Session 12, 2026-04-15)
- **Dependencies:** Task 0
- **Description:** The orchestrator's Session 9 committed Ravel to a
  persistent BEAM daemon (Elixir/OTP), replacing the original Rust
  single-process architecture. This is a load-bearing amendment that
  touches every task in this backlog. The spec was rewritten inline
  to: (a) replace Rust constructs with Elixir/OTP equivalents
  (behaviours, GenServers, `@external_resource`, `yaml_elixir`,
  `:telemetry`, daemon-singleton flock); (b) recast `PhaseRunner` as a
  pure module hosted inside sub-F's `PlanActor` GenServer, invoked on
  `{:run_phase, _}` messages; (c) prune `plan-state.md` per sub-F
  (`plan-id`, `host-project`, `dev-root` removed; `description:`
  added); (d) rename `{{RELATED_PLANS}}` → `{{VAULT_CATALOG}}` and
  delete `related-plans.md`; (e) hand the TUI off to sub-F as a
  separate Rust client binary (no B-owned drivers); (f) add
  `DispatchProcessor` and `QueryProcessor` as phase-exit hooks on
  non-compact phases.
- **Results:** Design doc fully rewritten inline for Elixir/BEAM in
  Session 12 (2026-04-15), mirroring the sub-C precedent (no
  supersede layer). The Rust runtime substrate (traits, `Arc<dyn>`,
  `tokio`, `ratatui` as process main loop, `include_str!`,
  `serde_yaml`, `fs2`, `crossbeam-channel`, `nix`) has been replaced
  wholesale with Elixir/OTP equivalents (`@behaviour`, GenServer
  consumption via `attach_consumer/2`, PlanActor mailbox,
  `@external_resource` + `File.read!/1`, `yaml_elixir`, daemon
  singleton flock, `:kill_group`, sub-M telemetry). `PhaseRunner` is
  now a pure module called from inside sub-F's `PlanActor`
  GenServer; phase transitions arrive as `{:run_phase, _}` messages
  (§2.3.3, §3.3, §4.5). `plan-state.md` schema pruned per sub-F
  (§2.3.2): `plan-id`, `host-project`, `dev-root` removed;
  `description:` (≤120 chars, hard-capped) added; `ravel-pid` →
  `daemon-pid`; `compact-baseline` added. `{{RELATED_PLANS}}` →
  `{{VAULT_CATALOG}}` (§2.4); `related-plans.md` deleted outright.
  Separate Rust TUI client binary replaces the in-process ratatui
  driver (§1.2, Appendix A Q9). `DispatchProcessor` and
  `QueryProcessor` added as phase-exit hooks on work/reflect/triage
  (§2.3.3 step 13, §3.5, §4.3). Appendix A Q20 carries the full
  Rust→BEAM translation table and weaves in the BEAM PTY spike
  findings (pipes-only erlexec, sliding-buffer sentinel matcher
  validation, two-completion-signals orthogonality). Downstream
  implementation tasks below have been rewritten against the new
  contract (see "Rewrite downstream implementation tasks" gate task).

### Adopt sub-R `{{ONTOLOGY_CONTEXT}}` passive placeholder in `StagingDirectory.render/4` `[substitution]` `[amendment]`
- **Status:** not_started
- **Dependencies:** none (feeds into "Implement placeholder substitution algorithm" and "Implement `StagingDirectory.render/4`" before those tasks start)
- **Description:** Sub-R (knowledge ontology) brainstorm (2026-04-16) introduced
  `{{ONTOLOGY_CONTEXT}}` as a new passive placeholder injected into LLM sessions at
  staging-render time. Sub-B owns all placeholder substitution logic and
  `StagingDirectory.render/4`; this amendment captures sub-B's share of the
  adoption contract.

  **The requirement:** When `StagingDirectory.render/4` renders a staging directory,
  substitute `{{ONTOLOGY_CONTEXT}}` with a compact, LLM-readable summary of the
  vault's loaded ontology — tag hierarchy and synonym sets. The source is the
  `<vault>/ontology/` directory. If that directory is absent or empty (ontology not
  yet initialised), substitute an empty string. **No hard error** on missing
  ontology — it is an optional v1 surface that may not be present at plan-creation
  time. The active half (`ontology.query` LLM tool) is sub-R's own implementation
  concern; sub-B only handles the passive read-and-format side.

  **Work to perform:**

  (a) Add `{{ONTOLOGY_CONTEXT}}` to the placeholder table in
  `{{PROJECT}}/docs/superpowers/specs/2026-04-12-sub-B-phase-cycle-design.md` §2.4
  alongside the existing five (`{{DEV_ROOT}}`, `{{PROJECT}}`, `{{PLAN}}`,
  `{{PROMPTS}}`, `{{VAULT_CATALOG}}`). Mark it **forward-only** — reverse
  substitution does not apply (same as `{{VAULT_CATALOG}}`).

  (b) Specify the rendering contract in the spec: walk `<vault>/ontology/`; if the
  directory does not exist or contains no loaded data, the substituted value is `""`
  (empty string, no error, no warning); otherwise produce a compact Markdown summary
  (tag hierarchy as a nested list, synonym sets as indented entries). The exact
  output format is to be agreed with sub-R before the rendering task is implemented;
  a sensible default is a flat Markdown list of tags with indented synonyms.

  (c) Update the "Implement placeholder substitution algorithm" task (once rewritten
  by the "Rewrite downstream" gate task) to include test coverage for:
  `{{ONTOLOGY_CONTEXT}}` with a fixture `<vault>/ontology/` tree; `{{ONTOLOGY_CONTEXT}}`
  with no ontology directory (must produce empty string, no error); and that
  `{{ONTOLOGY_CONTEXT}}` is excluded from reverse substitution.

  (d) Update the "Implement `StagingDirectory.render/4`" task (once rewritten) to
  reference the six-placeholder list and include an ontology-load step before
  substitution.

  **Backlog + spec editing only** — no runtime code. Acceptance: the spec's §2.4
  placeholder table lists all six placeholders, and the two downstream implementation
  tasks reference `{{ONTOLOGY_CONTEXT}}` consistently.
- **Results:** _pending_

### Rewrite downstream implementation tasks against Session-12 design doc `[amendment]`
- **Status:** done (this rewrite landed inline below)
- **Dependencies:** (the three amendment tasks above, all done)
- **Description:** The three amendment tasks landed a fresh
  Elixir/BEAM + four-phase cycle + sub-F-integrated design doc,
  but every implementation task from "Define core abstractions and
  types" onwards still described Rust types, `include_str!`,
  `serde_yaml`, `gray_matter`, `ratatui`, `fs2`, `tokio`, `Box<dyn>`,
  `InteractionDriver`, the `--ipc` client mode, the per-plan
  advisory lock stub, and the previous three-phase cycle. This
  rewrite replaces every downstream task with Session-12-aligned
  Elixir-native descriptions and deletes the tasks that moved to
  sub-F (TUI drivers, daemon startup / shutdown) or dissolved into
  sub-F's daemon-singleton lock (per-plan advisory lock stub).
- **Results:** Completed as part of this backlog rewrite. The task
  list below is internally consistent with the Session-12 design
  doc and with C's §11.1 contracts. No stale Rust crate names,
  trait shapes, three-phase references, `InteractionDriver`
  surfaces, or per-plan lock references remain. Tasks deleted in
  this rewrite: `HeadlessDriver`, `IpcDriver`, `RatatuiDriver`
  (TUI moved to sub-F's Rust client binary, §1.2); "Implement the
  full startup sequence" (daemon startup is sub-F's scope);
  "Implement shutdown sequence" (daemon shutdown is sub-F's scope);
  "Implement per-plan advisory lock stub" (replaced by sub-F's
  daemon-singleton lock, sub-D scope collapsed per §6).

### Define core abstractions and types `[types]`
- **Status:** not_started
- **Dependencies:** Task 0, Absorb Sub-C trait amendments into B's design + types, Absorb Ravel-Lite 2026-04 overhaul into B's design + types, Absorb BEAM pivot into B's design, spec, and backlog, Rewrite downstream implementation tasks against Session-12 design doc
- **Description:** Define the Elixir modules and typed structs from §2.3
  of the spec:

  **Structs (via `defstruct` + `@type t :: %__MODULE__{}`):**
  `Ravel.PlanContext`, `Ravel.PlanState`, `Ravel.LastExit`,
  `Ravel.Phase` (atom enum wrapper: `:work | :reflect | :compact | :triage`),
  `Ravel.ResolvedPaths`, `Ravel.PhaseOutcome`, `Ravel.StagingDirectory`,
  `Ravel.CopyBackReport`, `Ravel.PhaseContext`.

  **Behaviours (`@behaviour` modules):**
  `Ravel.PhaseExecutor` (§2.3.4) with three implementors —
  `Ravel.LlmHarnessExecutor`, `Ravel.ManualEditorExecutor`,
  `Ravel.FixtureReplayExecutor`;
  `Ravel.HarnessAdapter` (§4.1, consumed from sub-C);
  `Ravel.ReflectExitHook` (§4.2, exposed to sub-E).

  **Consumed event structs** (emitted by sub-C, pattern-matched by B):
  `%Ravel.Event.SessionLifecycle{}` (transitions `:ready |
  {:turn_complete, _} | {:exited, _}`), `%Ravel.Event.HarnessOutput{}`
  (`kind: :stdout | :stderr`).

  **Emitted event structs** (B → sub-M via `:telemetry`):
  `%Ravel.Event.PhaseLifecycle{}` (kinds `:started | :exited_clean |
  :interrupted | :executor_failed | :takeover_offered | :prior_interrupt_surfaced`).

  **Error shapes** via tagged tuples, not exception modules:
  `{:error, {:executor, reason}}`, `{:error, {:staging, reason}}`,
  `{:interrupted, forensics_dir}`.

  No logic; just shapes. This establishes the contract every downstream
  task is written against. Unit tests cover struct construction,
  pattern-match coverage on the phase enum, and round-trip serialisation
  for any struct that crosses the PlanActor mailbox boundary.
- **Results:** _pending_

### Implement `plan-state.md` parse/serialise `[plan-state]`
- **Status:** not_started
- **Dependencies:** Define core abstractions and types
- **Description:** Implement `Ravel.PlanState.load/1` and
  `Ravel.PlanState.persist/2` using `yaml_elixir` and a direct
  `---`-boundary split (no `gray_matter` equivalent needed — the
  frontmatter/body split is a two-line scan). Reads/writes
  `plan-state.md` (markdown with YAML frontmatter per §2.3.2 of the
  spec). `persist/2` delegates to `Ravel.FileIO.write_safe/3` with
  `:phase_owned` (never direct `File.write/2` — see the FileIO
  constraint). Unit tests cover: round-trip of every pruned-schema
  field (`description`, `current-phase`, `compact-baseline`,
  `daemon-pid`, `last-exit.*`, `ingestion-fired`, `schema-version`);
  `schema-version: 99` rejection (hard error); malformed frontmatter
  rejection; `description` > 120 char rejection (hard cap);
  `ingestion-fired` flag semantics; atomic-write survival of
  concurrent readers; Dataview-friendly kebab-case naming of all
  fields. Rejected fields that must NOT appear in the struct or the
  file: `plan-id`, `host-project`, `dev-root` (pruned by sub-F per
  §2.3.2). No filesystem mocks — use `tmp_dir` fixtures.
- **Results:** _pending_

### Implement embedded prompts module `[prompts]`
- **Status:** not_started
- **Dependencies:** Define core abstractions and types
- **Description:** Create `Ravel.Prompts` (in `lib/ravel/prompts.ex`)
  using `@external_resource` + `File.read!/1` at compile time to embed
  vendored upstream files from `{{PROJECT}}/prompts/` into the module's
  beam file as module attributes. Expose via `phase/1` and
  `materialise_into/1` per the example in spec §2.5:

  - Module attributes: `@phase_work`, `@phase_reflect`, `@phase_compact`,
    `@phase_triage` (four shared phase prompts); `@coding_style`,
    `@coding_style_rust`, `@memory_style` (three fixed-memory files —
    `@memory_style` is read by both reflect and compact and is the
    lossless-rewrite contract's stability anchor); `@create_plan` (the
    plan-creation spec).
  - `phase(:work | :reflect | :compact | :triage) :: binary()`.
  - `materialise_into(prompts_dir :: String.t()) :: :ok` writes every
    embedded binary into a target directory preserving upstream's
    `phases/` and `fixed-memory/` subdirectory layout.

  Copy the actual content from
  `{{DEV_ROOT}}/Ravel-Lite/defaults/phases/{work,reflect,compact,triage}.md`,
  `{{DEV_ROOT}}/Ravel-Lite/defaults/fixed-memory/{coding-style,coding-style-rust,memory-style}.md`,
  and `{{DEV_ROOT}}/Ravel-Lite/defaults/create-plan.md` into
  `{{PROJECT}}/prompts/` as part of this task. The pre-overhaul
  filenames `backlog-plan.md` and `create-a-multi-session-plan.md`
  are deleted/renamed upstream and must NOT appear in the module's
  `@external_resource` list or in the vendored tree. Unit tests verify:
  `materialise_into/1` produces the expected files at the expected
  paths, byte-equal content with the vendored sources, subdirectory
  layout preserved, `mix compile` recompiles the module when any
  vendored file changes (via the `@external_resource` tracking).
- **Results:** _pending_

### Implement placeholder substitution algorithm `[substitution]`
- **Status:** not_started
- **Dependencies:** Define core abstractions and types
- **Description:** Implement forward and reverse substitution as pure
  functions in `Ravel.Substitution` against the five placeholders from
  spec §2.4: `{{DEV_ROOT}}`, `{{PROJECT}}`, `{{PLAN}}`, `{{PROMPTS}}`,
  `{{VAULT_CATALOG}}`. Forward: walk a file's text and replace each
  placeholder with its resolved value — absolute paths for
  `{{DEV_ROOT}}` / `{{PROJECT}}` / `{{PLAN}}` / `{{PROMPTS}}`, and the
  **full content of `<vault>/plan-catalog.md`** (read via F's
  `CatalogRenderer` at render time) for `{{VAULT_CATALOG}}`. Reverse:
  walk a file's text and replace each absolute path with its
  placeholder, using longest-match-first ordering (PLAN before PROJECT
  before DEV_ROOT) to avoid prefix collisions. `{{PROMPTS}}` and
  `{{VAULT_CATALOG}}` are **forward-only** — their resolved values are
  per-render and never appear in canonical plan files, so reverse
  substitution does not touch them. Use Elixir's `Regex` with a
  module-attribute precompiled pattern. Unit tests cover: all five
  placeholders substituted correctly, longest-match-first ordering
  against prefix-collision inputs, round-trip (forward then reverse)
  returns the original for `{{DEV_ROOT}}` / `{{PROJECT}}` / `{{PLAN}}`,
  edge cases (empty file, file with no placeholders, file with
  multiple occurrences of the same placeholder), `{{VAULT_CATALOG}}`
  excluded from reverse substitution. (When sub-R's
  `{{ONTOLOGY_CONTEXT}}` amendment task lands, the algorithm and tests
  extend to six placeholders with `{{ONTOLOGY_CONTEXT}}` also
  forward-only.)
- **Results:** _pending_

### Implement `StagingDirectory.render/4` `[staging]`
- **Status:** not_started
- **Dependencies:** Implement embedded prompts module, Implement
  placeholder substitution algorithm
- **Description:** Implement the staging render per §2.3.5 of the spec.
  Walks the plan directory, copies every `.md` file to the staging
  root with forward substitution applied (via
  `Ravel.FileIO.read_tracked/1` for reads and
  `Ravel.FileIO.write_safe/3` for writes), materialises embedded prompt
  content into `<staging>/prompts/` via
  `Ravel.Prompts.materialise_into/1`, composes the phase prompt
  (shared `phases/<phase>.md` + optional per-plan
  `<plan>/prompt-<phase>.md` override, additive only — never
  replacement), substitutes all five placeholders in the composed
  prompt, and stages the narrow project allowlist resolved from the
  phase prompt's `{{PROJECT}}/...` tokens. Enforces the critical
  invariants: (a) never follow symlinks into project repos (hard
  error), (b) refuse to descend into subdirectories containing
  `plan-state.md` (hard error — enforces "one plan per PlanActor"
  against nested hierarchy; F owns nested-plan semantics), (c) write
  only inside the staging root. Unit tests cover: correct forward
  substitution on all five placeholders, embedded prompts
  materialisation, allowlist resolution, symlink rejection,
  child-plan-descent rejection, binary file passthrough (don't
  substitute non-text files), phase composition with and without the
  optional override file.
- **Results:** _pending_

### Implement `StagingDirectory.copy_back/2` `[staging]`
- **Status:** not_started
- **Dependencies:** Implement `StagingDirectory.render/4`
- **Description:** Implement copy-back per §2.3.5 and §4.5 of the
  spec. Walks the staging root after executor exit, compares each file
  against the canonical plan directory via content hash (using
  `Ravel.FileIO.read_tracked/1`), applies reverse substitution on
  `.md` files only, writes through changed files via
  `Ravel.FileIO.write_safe/3` with category `:phase_owned`. Produces a
  `%Ravel.CopyBackReport{}` with `files_updated`, `files_unchanged`,
  `files_added`, `files_rejected`. Rejection happens when a staged
  file's relative path is outside the plan directory or the
  explicitly-staged project subtree — this is a hard error that aborts
  the phase outcome and preserves the staging dir for forensics. Unit
  tests cover: clean write-through, unchanged-file detection,
  new-file propagation, reverse substitution correctness, rejection
  rule enforcement, atomic rename semantics (inherited from
  `FileIO.write_safe/3`).

  **FileIO constraint (from sub-D Session 18):** Direct `File.write/2`
  / `File.rename/2` calls are **forbidden** — all writes delegate to
  `Ravel.FileIO.write_safe/3`. Plan files written by `copy_back/2` are
  `:phase_owned` — the daemon is authoritative during an active phase.
  Conflicts surface as warnings (not hard errors), daemon copy wins.
- **Results:** _pending_

### Implement `StagingDirectory.preserve_on_interrupt/2` and `cleanup/1` `[staging]`
- **Status:** not_started
- **Dependencies:** Implement `StagingDirectory.render/4`
- **Description:** `preserve_on_interrupt/2` moves the staging directory
  to `<vault>/runtime/interrupted/<qualified-id>/<phase>-<timestamp>/`
  for forensics (qualified-id format per sub-F §4.4). `cleanup/1`
  deletes the staging directory on clean phase exit. Both operations
  are atomic (`File.rename/2` for the move, `File.rm_rf/1` for the
  cleanup — note these are OS-level directory operations, not tracked
  plan files, so the `FileIO.write_safe/3` constraint does not apply
  here). Unit tests cover both paths and the idempotency of cleanup.
- **Results:** _pending_

### Implement `Ravel.PreWorkHook` `[runner]`
- **Status:** not_started
- **Dependencies:** Define core abstractions and types
- **Description:** New module surfaced by the Session-12 rewrite
  (spec §2.3.3 step 2). `Ravel.PreWorkHook.run/2` probes
  `<plan>/pre-work.sh` for executability (via `File.stat/1` mode
  bits); if absent or non-executable, returns `:ok` silently. If
  executable, first deletes any stale `<plan>/latest-session.md`
  (defensive cleanup), then invokes the hook synchronously from the
  **project root** (not the plan dir, not the staging dir) via
  `System.cmd/3` with `stderr_to_stdout: true`. Non-zero exit or any
  I/O failure returns `{:error, reason}`, which `PhaseRunner.run_phase/4`
  propagates as a hard error that aborts the cycle before staging is
  rendered. This matches Ravel-Lite's upstream `run-plan.sh` contract.
  Only fires for `:work` phase (never reflect, compact, or triage).
  Unit tests cover: absent hook (silent skip), non-executable hook
  (silent skip), clean exit (continues), non-zero exit (hard error),
  I/O failure (hard error), cwd is project root not plan dir,
  `latest-session.md` cleanup precedes hook invocation.
- **Results:** _pending_

### Implement `Ravel.Sentinel.SlidingBuffer` `[executor]`
- **Status:** not_started
- **Dependencies:** Define core abstractions and types
- **Description:** New module surfaced by the Session-12 rewrite
  (spec §2.3.4, §4.1). Implements the sliding-buffer sentinel matcher
  that `LlmHarnessExecutor` uses to detect task-level phase completion
  in the stdout stream from sub-C's harness session. API:

  - `new(sentinel :: binary()) :: t()` — initialise state with the
    per-phase sentinel string (defined in the phase prompt file).
  - `push(state :: t(), chunk :: binary()) :: {:match | :no_match, new_state}` —
    feed a chunk; returns `:match` when the sentinel has been observed
    (possibly spanning chunk boundaries).

  Window is bounded to `sentinel_size - 1` bytes — enough to catch
  sentinels that span chunk boundaries without unbounded memory
  growth. **Validated by the BEAM PTY spike** (Session 10,
  2026-04-15; `spikes/beam_pty/`): correct across single-chunk
  delivery, two-chunk split, grapheme-by-grapheme drip, false-prefix
  (chunk matches a prefix of the sentinel but not the full sentinel),
  and false-overlap (suffix of one chunk + prefix of next reconstructs
  the sentinel). Unit tests replicate the spike's five cases plus:
  zero-length chunk (no-op), sentinel exactly at chunk boundary,
  multiple chunks after match (subsequent pushes remain `:match`,
  idempotent), unicode sentinels (byte-level matching, not grapheme —
  document this choice).
- **Results:** _pending_

### Implement `FixtureReplayExecutor` and `FixtureReplayAdapter` `[executor]`
- **Status:** not_started
- **Dependencies:** Define core abstractions and types
- **Description:** Implement the simplest executor first:
  `Ravel.FixtureReplayExecutor` reads a captured output stream from a
  JSON fixture file, invokes `emit_fn` with chunk events at simulated
  pacing, then returns `:ok`. No process spawn, no `EDITOR`, no
  harness adapter. Also implement `Ravel.FixtureReplayAdapter` — an
  implementation of the `Ravel.HarnessAdapter` behaviour that reads a
  captured Claude Code session transcript from JSON and emits
  `%Ravel.Event.HarnessOutput{}` chunks through the consumption
  protocol (`attach_consumer/2` delivers the chunks to the caller's
  mailbox via `handle_info/2`). Both are used by downstream integration
  tests to drive `PhaseRunner.run_phase/4` end-to-end without a live
  harness. Unit tests verify: fixture parsing, chunk pacing, clean
  exit semantics, the `interrupt/1` no-op semantics for replay,
  consumer-protocol compliance (sub-C's §11.1 contract).
- **Results:** _pending_

### Implement `Ravel.PhaseRunner.run_phase/4` `[runner]`
- **Status:** not_started
- **Dependencies:** Implement `plan-state.md` parse/serialise, Implement
  `StagingDirectory.render/4`, Implement `StagingDirectory.copy_back/2`,
  Implement `FixtureReplayExecutor` and `FixtureReplayAdapter`,
  Implement `Ravel.PreWorkHook`
- **Description:** Wire the 13-step `run_phase/4` flow from §2.3.3 of
  the spec as a **pure module** (no process state; invoked from inside
  sub-F's `PlanActor` GenServer `handle_info/2` clause for
  `{:run_phase, _}`). Signature:

  ```elixir
  @spec run_phase(
          ctx :: Ravel.PlanContext.t(),
          phase :: Ravel.Phase.t(),
          executor :: module(),
          parent :: pid()
        ) :: Ravel.PhaseRunner.outcome()
  ```

  The 13 steps:

  1. **Validate transition** legal from `ctx.state.current_phase`.
     Compact-trigger exception: if entering compact and
     `wc -w memory.md <= compact-baseline + HEADROOM` (HEADROOM =
     1500 words, module constant), skip directly to triage (the
     triage transition is still legality-checked).
  2. **Run `pre-work.sh` hook** (only for `:work` — see dedicated task).
  3. **Render staging** via `StagingDirectory.render/4`.
  4. **Update `ctx.state`** (current_phase, started_at, daemon_pid),
     persist atomically.
  5. **Emit `%PhaseLifecycle{kind: :started, ...}`** via sub-M's
     `:telemetry` boundary (forwarded to attached consumers by the
     PlanActor).
  6. **Invoke `executor.execute/3`** with `(ctx, staging, emit_fn)`.
     Blocks until executor returns `:ok | {:interrupted, _} | {:error, _}`.
  7. **On clean exit**: run `StagingDirectory.copy_back/2`. Reject
     out-of-bounds writes as hard errors.
  8. **Compute next phase**: work → reflect, reflect → compact,
     compact → triage (write new `compact-baseline` from
     `wc -w memory.md`), triage → work. Compact-skipped branch goes
     directly to triage.
  9. **Post-work session-log append**: if completed phase was `:work`,
     append `<plan>/latest-session.md` content to `<plan>/session-log.md`
     via `FileIO.write_safe/3` (temp-then-rename is the category's
     default atomic semantic), then delete `latest-session.md`. Soft
     warning if `latest-session.md` missing (the work phase wrote
     nothing).
  10. **Update `last_exit`** (phase, exited_at, clean_exit: true,
      ingestion_fired: false), set current_phase to computed next,
      persist atomically.
  11. **Emit `%PhaseLifecycle{kind: :exited_clean, ...}`**.
  12. **If completed phase was `:reflect`**, call
      `reflect_exit_hook.on_reflect_exit/1` non-blockingly (PlanActor
      dispatches to a supervised task; runner returns immediately).
  13. **Run `DispatchProcessor.process/2` and `QueryProcessor.process/2`
      for non-compact phases** (`:work`, `:reflect`, `:triage`).
      Synchronous from runner's perspective, F's internal timeouts
      apply. Compact is skipped (lossless-rewrite contract has no
      dispatches or queries to emit).
  14. **Cleanup staging** on clean consumption.
  15. **Return** `{:ok, {:transitioned, next_phase}}`.

  Every step is a hard-error boundary; any unhandled failure raises
  into the PlanActor, which catches via supervisor and reports
  through sub-M. Integration tests use `FixtureReplayExecutor` and a
  fixture plan directory. Test coverage: full happy path
  (`work → reflect → compact → triage → work` round-trip AND
  `work → reflect → triage → work` round-trip when the compact
  trigger skips compact), illegal transition rejection, copy-back
  rejection aborts cleanly, reflect-exit hook fires exactly once on
  reflect exit and does NOT fire on work / compact / triage exit,
  state persistence at every transition boundary, event emission
  order, compact-trigger wc-word-count logic skips compact under
  the headroom and runs it above, `DispatchProcessor` /
  `QueryProcessor` fire on work/reflect/triage and skip on compact,
  `pre-work.sh` failure aborts the cycle cleanly.
- **Results:** _pending_

### Implement `Ravel.PhaseRunner.interrupt/1` and the Interrupted flow `[runner]`
- **Status:** not_started
- **Dependencies:** Implement `Ravel.PhaseRunner.run_phase/4`, Implement
  `StagingDirectory.preserve_on_interrupt/2` and `cleanup/1`
- **Description:** Implement the interrupt flow from §3.4 of the spec.
  When the PlanActor receives `{:interrupt_phase, _}` while in
  `PhaseRunning`, it invokes the runner's interrupt path, which:
  signals `executor.interrupt/1` (which forwards to the harness
  adapter's `kill/2` for `LlmHarnessExecutor` — graceful → grace
  period → force kill via sub-C's two-phase termination; no-op for
  `ManualEditorExecutor`); captures partial output into
  `<vault>/runtime/interrupted/<qualified-id>/<phase>-<ts>/partial-output.log`
  as JSON-Lines; preserves staging under
  `<vault>/runtime/interrupted/<qualified-id>/<phase>-<ts>/staging/`;
  updates `plan-state.md` with `interrupted: true`,
  `last-exit.phase: <current>`, `last-exit.clean-exit: false`, and
  does **not** transition `current-phase`; emits
  `%PhaseLifecycle{kind: :interrupted, ...}` and
  `%PhaseLifecycle{kind: :takeover_offered, options: [:manual, :retry_llm, :dismiss]}`.

  Also implement the **pending-interrupt flag**: an interrupt during
  `CopyingBack` is recorded in a map field in the runner's local
  state, `copy_back/2` runs to completion, then the runner checks the
  flag and transitions to Interrupted state post-copy. This is a
  deliberate asymmetry — aborting mid-copy risks an inconsistent
  plan directory, which violates the "hard errors by default, state
  always consistent on disk" principle (§3.3).

  Unit tests cover: interrupt during ExecutorRunning (immediate),
  interrupt during CopyingBack (deferred until copy-back completes),
  idempotency of double-interrupt, correct forensic log path,
  `interrupted` flag persistence, obsidian-wikilink syntax in the
  emitted forensics_dir string (`[[...]]`) so the takeover
  notification is immediately actionable when the user opens the
  vault.
- **Results:** _pending_

### Implement `ManualEditorExecutor` `[executor]`
- **Status:** not_started
- **Dependencies:** Implement `Ravel.PhaseRunner.run_phase/4`
- **Description:** Implement the human-driven executor per §2.3.4 of
  the spec. Writes `<staging>/phase-prompt.readonly.md` with the
  substituted phase prompt text. Determines target file(s) for the
  phase (`work` → backlog.md + latest-session.md; `reflect` →
  memory.md (reads latest-session.md as input); `compact` → memory.md
  (lossless rewrite); `triage` → backlog.md). Spawns `$EDITOR`
  (resolved from `System.get_env("EDITOR")`; hard error if unset)
  via `System.cmd/3` blocked on the target files. Waits for editor
  exit. Returns `:ok` on clean exit, `{:error, {:executor, :editor_failed}}`
  on non-zero. `interrupt/1` is a no-op. Integration tests use a
  scripted editor stub (e.g., a shell script that edits a file then
  exits) to exercise the full flow through `PhaseRunner.run_phase/4`,
  including the compact phase path.
- **Results:** _pending_

### Implement `LlmHarnessExecutor` against sub-C's `Ravel.HarnessAdapter` `[executor]`
- **Status:** not_started
- **Dependencies:** Implement `Ravel.PhaseRunner.run_phase/4`, Implement
  `Ravel.Sentinel.SlidingBuffer`, Define core abstractions and types
- **Description:** Implement the LLM executor per §2.3.4 and §4.1 of the
  spec. Consumes a sub-C harness session via the `attach_consumer/2`
  + `handle_info/2` pattern; pattern-matches on
  `%Ravel.Event.SessionLifecycle{}` for protocol-level signals
  (`:ready | {:turn_complete, _} | {:exited, _}`) and on
  `%Ravel.Event.HarnessOutput{kind: :stdout}` for content, feeding
  each chunk into `Ravel.Sentinel.SlidingBuffer` to detect task-level
  completion. When the sentinel matcher fires, the executor emits
  an internal `{:phase_completion_detected, sentinel}` message that
  the state machine consumes as the trigger to wind down the session
  and return `:ok`.

  Reads the composed phase prompt from `<staging>/phase-prompt.md`
  (already substituted by `StagingDirectory.render/4`), invokes
  `adapter.spawn/2` with the phase-appropriate tool profile,
  `attach_consumer/2` to start receiving events, forwards streamed
  chunks via `emit_fn` for sub-M telemetry and PlanActor consumer
  forwarding. `interrupt/1` calls `adapter.kill/2` (two-phase
  termination from sub-C: graceful → grace period → force).

  Sub-C's real adapter lands separately; until then, use
  `FixtureReplayAdapter` (already landed above) for deterministic
  integration tests. Document the consumption contract explicitly —
  `%SessionLifecycle{}` signals are protocol-level and distinct from
  sentinel-based task-level completion; the executor must honour both
  orthogonally (session may complete a turn without sentinel firing,
  and sentinel may fire before the adapter reports session exit).
- **Results:** _pending_

### Implement `Ravel.ReflectExitHook` wiring for sub-project E `[integration]`
- **Status:** not_started
- **Dependencies:** Implement `Ravel.PhaseRunner.run_phase/4`
- **Description:** Define the `Ravel.ReflectExitHook` behaviour from
  §4.2 of the spec and wire its invocation into step 12 of
  `run_phase/4`. The hook is called **non-blockingly** — the PlanActor
  dispatches it to a supervised task (`Task.Supervisor.start_child/2`)
  so the runner can return control immediately; errors from the hook
  are not observed by B. Until sub-project E's real implementation
  lands, ship a `Ravel.NullReflectExitHook` stub that logs the call
  via `:telemetry` and returns `:ok`. When E lands, the production
  configuration swaps in E's concrete implementation module. Unit
  tests verify: hook fires exactly once per clean reflect exit, hook
  receives the correct `%Ravel.ReflectExitContext{}`, hook does not
  fire when `last-exit.ingestion-fired` was already true, hook is
  non-blocking (runner returns before the hook completes).
- **Results:** _pending_

### Implement typed-event emission via `:telemetry` `[events]`
- **Status:** not_started
- **Dependencies:** Define core abstractions and types
- **Description:** Implement the `:telemetry` emission points in
  `PhaseRunner` and each `PhaseExecutor` implementor per §4.4 of the
  spec. All events are typed `%Ravel.Event.*{}` structs; the
  `:telemetry` payload carries the struct verbatim plus `plan_id`
  and phase metadata. The PlanActor attaches handlers that forward
  events to attached consumers through its mailbox (consumer
  forwarding is F's scope; B only emits).

  The `emit_fn` closure passed to each executor is how executors
  push streamed chunks and intermediate events — the closure wraps
  `:telemetry.execute/3` with the span/plan-id context baked in.
  Unit tests cover: event ordering preserved, backpressure semantics
  (the closure is synchronous, so a slow consumer cannot drop events
  silently), struct shape stability across emission points.
- **Results:** _pending_

### Implement symlink rescan and vault bootstrap `[vault]`
- **Status:** not_started
- **Dependencies:** Task 0 (symlinks validation spike must pass first)
- **Description:** Implement the symlink rescan lifecycle per §2.6 of
  the spec. Walks the dev root looking for project repos (directories
  containing `.git`), checks for `<project>/ravel/`, creates
  relative symlinks under `<vault>/projects/<project-name>`,
  garbage-collects dangling links, detects cycles via `File.lstat/1`
  inode comparison, handles the `ravel rescan` CLI subcommand as an
  explicit trigger plus automatic invocation on daemon start (the
  daemon-start invocation lives in sub-F's startup sequence; B
  provides the rescan function). Also implement vault bootstrap per
  §2.6: create the vault directory if missing, create `knowledge/`,
  `runtime/`, `projects/` subdirectories, initialise `.git` and
  `.gitignore`, write a minimal `.obsidian/` stub and a vault
  `README.md`. Unit tests use `tmp_dir`-based fake dev roots with
  fixture project layouts. No B-owned daemon startup — the bootstrap
  function is called by sub-F's startup sequence.
- **Results:** _pending_

### Implement crash recovery `[lifecycle]`
- **Status:** not_started
- **Dependencies:** Implement `plan-state.md` parse/serialise, Implement
  `Ravel.ReflectExitHook` wiring for sub-project E
- **Description:** Implement the three crash recovery scenarios from
  §3.2 of the spec as a pure function called by sub-F's PlanActor
  immediately after it spawns and loads `plan-state.md`. **Scenario A
  (interrupted):** emit
  `%PhaseLifecycle{kind: :prior_interrupt_surfaced, forensics_dir, ...}`
  on first attached-consumer attach; the user decides retry / dismiss
  via `{:retry_phase}` / `{:dismiss_interrupt}` messages (F-level
  messages; B provides the handler function). **Scenario B
  (ingestion-didn't-fire):** fire the ingestion pipeline on the prior
  reflect's outputs as the first post-Active action, with the hook
  flipping `ingestion-fired` at Stage 5 start (E's responsibility).
  **Scenario C (clean state):** no-op. All three run at the PlanActor
  `Dormant → Active` transition. Idempotency: running recovery twice
  on the same state produces the same result. Unit tests cover all
  three scenarios, crash-during-recovery survival, and the "last
  reflect was already fired" no-op path.
- **Results:** _pending_

### End-to-end integration test corpus `[testing]`
- **Status:** not_started
- **Dependencies:** Implement `Ravel.PhaseRunner.run_phase/4`, Implement
  `FixtureReplayExecutor` and `FixtureReplayAdapter`, Implement crash
  recovery
- **Description:** Build a corpus of fixture plans under
  `test/fixtures/plans/` and corresponding scripted PlanActor message
  sequences under `test/fixtures/scripts/`. Each integration test
  drives a fixture plan through the full pipeline by sending
  `{:run_phase, _}` messages to a test PlanActor (stubbed from F's
  real implementation or minimal harness) using
  `FixtureReplayExecutor` and asserts the final plan state plus the
  emitted `:telemetry` event stream match expectations in
  `test/fixtures/expected/`. Coverage target: full
  `work → reflect → compact → triage → work` cycle (compact-run
  branch), `work → reflect → triage → work` cycle (compact-skipped
  branch), interrupt-and-retry, interrupt-and-takeover,
  crash-recovery-scenario-A, crash-recovery-scenario-B, copy-back
  rejection abort, nested plan (parent + child) with descent rule
  enforcement, `pre-work.sh` failure aborts cleanly. Tests are
  tagged `:integration` for ExUnit.
- **Results:** _pending_

### Live gated test against real Claude Code `[testing]`
- **Status:** not_started
- **Dependencies:** Implement `LlmHarnessExecutor` against sub-C's
  `Ravel.HarnessAdapter`, End-to-end integration test corpus
- **Description:** Create an ExUnit-tagged (`@tag :live`) test that
  runs the full cycle against a real Claude Code instance with a
  fixture plan, asserting only structural invariants: phases
  complete, no hard errors, daemon-singleton lock acquired/released
  (sub-F's scope), staging cleaned up, `plan-state.md` consistent,
  ingestion event log written (if E's real implementation is also
  landed by then). The `:live` tag excludes the test from the default
  run; `mix test --include live` runs it. Not run on every commit.
  Runs on a clean-room dev-root layout torn down after the test.
  Produces characterisation data for the cold-spawn latency risk from
  §5.4.
- **Results:** _pending_

### Dogfood task: host the orchestrator seed plan on Ravel v1 `[dogfood]`
- **Status:** not_started
- **Dependencies:** End-to-end integration test corpus, Live gated
  test against real Claude Code, sub-F daemon + TUI client landed,
  sub-F PlanActor implementation landed
- **Description:** The v1 acceptance test. Migrate the orchestrator
  seed plan and sub-E's sibling plan from `phase.md` +
  Ravel-Lite's `run-plan.sh` to `plan-state.md` + Ravel v1 (the
  BEAM daemon + sub-F TUI client). This depends on sub-project G's
  migration logic being available — if G is not yet done, perform
  the migration manually as a one-shot for these two plans as part
  of this task's scope (document the manual steps clearly). Run a
  full `work → reflect → (compact) → triage` cycle of the
  orchestrator plan on Ravel v1 end-to-end: sub-F's Rust TUI
  client attached to the daemon, real Claude Code harness via
  sub-C's `Ravel.HarnessAdapter`, real ingestion pipeline from
  sub-E, real vault at `<dev-root>/Ravel-vault/`. Must exercise
  both the compact-skipped and compact-run branches of the
  wc-word-count trigger. Success criteria: cycle completes cleanly,
  all typed events emitted correctly, plan state persisted
  consistently, ingestion fired exactly once, no hard errors, the
  user can retire Ravel-Lite's `run-plan.sh` for these two plans
  and proceed on Ravel for all future sessions. This is the
  moment v1 ships.
- **Results:** _pending_

### Implementation notes and handoff documentation `[docs]`
- **Status:** not_started
- **Dependencies:** Dogfood task: host the orchestrator seed plan on
  Ravel v1
- **Description:** Document any implementation decisions, discovered
  constraints, or design-doc deviations in `{{PLAN}}/memory.md`.
  Produce a short user-facing note for the main Ravel docs
  explaining how the phase cycle works, when each phase fires, how
  to use the takeover prompt, and how to inspect plan state through
  Obsidian (Dataview query examples for common views). Update the
  main Ravel `README.md` with a brief "v0.2.0 orchestrator"
  section.
- **Results:** _pending_

### Adopt sub-M observability framework — phase lifecycle instrumentation `[m-adoption]`
- **Status:** not_started
- **Dependencies:** sub-M's `:telemetry` boundary defined; this
  task lives in B's backlog because B owns the call sites being
  instrumented
- **Description:** Landed by sub-project M's brainstorm
  (2026-04-13, Session 7 of the ravel-orchestrator plan). M's
  design doc at
  `{{PROJECT}}/docs/superpowers/specs/2026-04-13-sub-M-observability-design.md`
  §10 specifies M's cross-plan adoption requirements; this task is B's
  share.

  The Session-12 BEAM pivot collapses sub-M's `tracing`-based Rust
  design into Elixir's native `:telemetry`. Concretely:

  1. Emit `%Ravel.Event.PhaseLifecycle{}` typed struct events at the
     numbered steps of `PhaseRunner.run_phase/4` via
     `:telemetry.execute/3`:
     - Step 5: `kind: :started, phase, plan_id, executor_kind, at`
     - Step 11: `kind: :exited_clean, phase, transitioned_to, at`
     - Step 12: `kind: :reflect_hook_fired, plan_id, at`
     - Interrupted path: `kind: :interrupted, phase, forensics_dir, at`
     - ExecutorFailed path: `kind: :executor_failed, phase, error, at`
     - Interrupted path: `kind: :takeover_offered, phase, forensics_dir, options, at`
     - Crash-recovery: `kind: :prior_interrupt_surfaced, phase, forensics_dir, at`
  2. The `emit_fn` closure (passed to executors in step 6 of
     `run_phase/4`) wraps `:telemetry.execute/3` with plan-id and
     phase context baked in, so executor-level streamed events
     carry the right attribution without each executor calling
     telemetry directly.
  3. The PlanActor attaches handlers (F's scope) that forward
     `:telemetry` events to attached consumers via its mailbox.
     **Do not emit directly to a TUI channel from B** — the TUI is
     sub-F's Rust client binary, consumed through the PlanActor's
     attached-consumer protocol.
  4. Replace any draft `Logger.debug/1` calls in `PhaseRunner` with
     `:telemetry` events of kind `%Ravel.Event.Diagnostic{}`. The
     telemetry subscriber filters visibility per sub-M's log-level
     config; events are always emitted.
  5. In `run_phase/4`'s error branches (all 13 steps' hard-error
     boundaries, executor failure branch, interrupt branch), emit a
     dump event carrying the last N internal state snapshots before
     returning the error outcome. The dump event shape is defined by
     sub-M; B calls a sub-M helper rather than formatting the dump
     itself.

  TDD: write ExUnit tests that drive a fixture
  `PhaseRunner.run_phase/4` cycle with a `:telemetry` test handler
  attached and assert the expected event sequence and payload
  shapes. Layer 3 integration test lives in sub-M; this task only
  needs unit tests on B's side.

  No changes to B's type surface beyond the `%Ravel.Event.*{}`
  structs. The Rust-era parallel-channel coexistence (B's
  `PhaseEvent` channel + M's `RavelEvent` bus) is collapsed by
  the BEAM pivot — `:telemetry` is the single emission surface.
- **Results:** _pending_
