# Memory — Sub-project F: Plan Hierarchy, Actor Model, Dispatch, Declarative Routing

This plan implements sub-project F of the Ravel orchestrator merge.
F's brainstorm is complete; this plan is the implementation work, not
a design phase. The design is fully specified in the spec referenced
below. If an implementation question arises that the spec does not
answer, the answer goes into this memory file (and possibly back into
the spec) rather than being invented ad hoc.

## Primary reference

**`{{PROJECT}}/docs/superpowers/specs/2026-04-14-sub-F-hierarchy-design.md`**
is the authoritative design document. Every task in this plan's backlog
derives from §11 of that spec. Consult §1–§10 of the spec before acting
on any implementation task.

## Parent plan

The orchestrator-level plan lives at
`{{PROJECT}}/LLM_STATE/ravel-orchestrator/` (currently — will move
to `{{PROJECT}}/ravel/project-root/` after sub-project G's
migration lands). It coordinates this sub-plan with its siblings (A, B,
C, D, E, G, H, I, M, N). The parent plan's `memory.md` holds
cross-sub-project architectural state. This file holds only
sub-project-F-specific implementation state.

## Key architectural anchors (quick reference; spec is canonical)

These are the decisions most load-bearing for implementation. Consult
§1–§10 of the design doc for full context before acting on any of them.

### F-1. Ravel is a persistent actor daemon on BEAM

V1 runs as a single long-running Elixir/OTP application. 
`ravel daemon` is the entry point; it hosts all actors, message routing,
supervision, harness adapters, fact extraction, and ingestion. OTP
supervision, message passing, hot code reload, and distribution
transparency are BEAM primitives — use them, do not hand-roll.

### F-2. Two sealed actor types: PlanActor and ExpertActor

Both implement a shared `Ravel.Actor` behavior on top of
`GenServer`. The set is **sealed** for v1 — adding a third type is a
code change, not a plugin mechanism. PlanActor progresses through phase
cycles (wraps sub-B's `PhaseRunner`). ExpertActor is a type-hole stub
returning `:not_yet_implemented` until sub-N lands.

### F-3. Two message types: Dispatch and Query

Both share target resolution, declarative routing, Level 2 fallback,
and audit trail conventions. Dispatch is fire-and-forget to a target's
backlog `Received` section. Query is request-response via the tool-call
boundary in sub-C's adapter.

### F-4. Plan hierarchy via `project-root` convention

Every adopted project has exactly one root plan at
`<project>/ravel/project-root/`. Name is **reserved**. Nested child
plans nest arbitrarily beneath it. A directory is a plan iff it
contains `plan-state.md`. `knowledge/` lives as a sibling of
`project-root/`, not inside it.

### F-5. Path-based qualified plan IDs, never stored

A plan's qualified ID is a pure function of its filesystem path:
`strip_prefix(plan_path, "<vault>/projects/")`. Examples:
`Ravel/project-root`, `Ravel/project-root/sub-F-hierarchy`.
Never stored in `plan-state.md` frontmatter. Filesystem is
authoritative; qualified IDs computed at read time.

### F-6. Dispatch target resolution is asymmetric

- **Same-project** → origin names the specific `target-plan:
  <qualified-id>`. Ravel writes to the target's `Received` section
  mechanically. No LLM in the loop.
- **Cross-project** → origin names only `target-project: <name>`.
  Ravel spawns a **Level 2 routing agent** — fresh-context Claude
  Code session scoped to the target project's vault subtree (plans +
  source code), with authority to pick a specific target plan or
  reject with reasoning.

### F-7. Vault catalog as LLM-facing enumeration

`{{VAULT_CATALOG}}` is substituted at phase-prompt render time with
the full vault catalog at `<vault>/plan-catalog.md` — every plan and
expert, grouped by project, with 120-char descriptions and dispatch
rules. Auto-regenerated on plan mutation and every phase-prompt
render. Replaces the old `{{RELATED_PLANS}}` placeholder. No plan's
LLM phase ever reads another plan's files directly.

### F-8. Description discipline: 120-character hard cap

Every plan and expert has a `description:` frontmatter field, ≤120
characters, enforced at load time (hard error on overflow).
Keyword-dense, noun-phrase-led, no self-reference, no placeholders.
Permanent scope declaration, not current-state description.

### F-9. Declarative routing with LLM-fallback learning loop

Routing rules live in `<vault>/ontology/routing.dl` — a user-editable
Datalog rule file loaded by `Ravel.Datalog.Server` (singleton gen_server
delivered by sub-R). Rules derive a `route/3` fact
`route(MsgType, Target, TargetKind)`. Hot reload via a filesystem watcher
re-parses the `.dl` fact base and re-evaluates; edits take effect without
daemon restart. Facts (concern keywords) are extracted by a cheap LLM
pass (Haiku via sub-C) and asserted as `concerns/1` facts, then
normalized via `resolves_to/2` queries against sub-R's ontology before
rule evaluation. When Datalog evaluation derives no `route/3` fact (or
ambiguously derives multiple), the Level 2 routing agent runs as
fallback and may propose a new Datalog rule the user accepts into
`ontology/routing.dl` — closing a learning loop. Datalog unifies
ontology resolution and routing in a single declarative substrate;
`routing.dl` replaces the earlier pattern-matched Elixir `routing.ex`
and the Erlog reservation (Appendix C Q7). Parse errors on startup are
a hard failure; parse errors during hot reload keep the previous rule
set loaded and surface the failure in the TUI (spec §9.3).

### F-10. Runtime: Elixir on BEAM

Erlang/OTP 28 + Elixir 1.19.x. Dependencies: `erlexec` (validated by
the BEAM PTY spike in Session 10), `jason`, `telemetry`, `yaml_elixir`.
Rust is used only for the TUI client binary (separate implementation
plan). Gleam is a future migration target if Elixir's dynamic typing
proves painful for the invariant-heavy design.

### F-11. Two official clients over Unix socket NDJSON (updated 2026-04-16)

`ravel-tui` is a standalone Rust binary using `ratatui` + `tokio`
+ `serde_json`. Connects to the daemon over a local Unix socket at
`<vault>/runtime/daemon.sock` speaking NDJSON. No Rustler / no NIFs —
the socket is the integration boundary. This plan scaffolds the
daemon side of that contract; the TUI binary itself belongs to a
**separate implementation plan to be scaffolded later**.

`ravel cmd` (finalised in sub-H's brainstorm, Session 21,
2026-04-16) is a second official client: a headless CLI wrapper that
speaks the same NDJSON socket protocol. It enables scripted / CI
access to the daemon without a TUI session. No extra daemon-side
ceremony is required — the existing forward-compat (§7.3) and
multi-client (§7.4) design already covers it. Sub-F owns only the
daemon socket; both clients are sub-H's implementation scope.

### F-12. Reserved extensibility hooks (sub-O, sub-P)

`daemon.toml` reserves `[harnesses.*]` sections for sub-O (mixture of
models, v1.5+). Actor declarations reserve the `model:` field. The
`[peers]` section and `<peer>@<qualified-id>` syntax are reserved for
sub-P (team mode, v2+). V1 parses these sections with forward
compatibility (unknown `[harnesses.foo]` → warn and ignore; non-empty
`peers: []` → hard error with "team mode not supported in v1").

## Contract with sibling sub-projects

### Consumes from sub-A

Vault discovery and identity verification. F's `VaultDirectory.load`
is called from A's `verify_vault` path at daemon startup. A owns the
vault discovery chain (`--vault` → `RAVEL_VAULT` → user config →
hard error); F owns what happens next.

### Consumes from sub-B

`Ravel.PhaseRunner` and its 13-step `run_phase/4` flow. F's
`PlanActor` hosts this runner inside its `GenServer` state and
invokes it in response to `{:run_phase, _}` messages from attached
clients. F's `DispatchProcessor` and `QueryProcessor` are called by
B's `PhaseRunner` as phase-exit hooks on non-compact phases.

### Consumes from sub-C

`Ravel.HarnessAdapter` behaviour. F's Level 2 routing agent and
fact-extraction LLM pass spawn sessions via the same adapter the
phase runner uses. F's Query messages are delivered mid-session via
C's tool-call boundary (`Ravel.Router.handle_tool_call/4`).

### Consumes from sub-D

`Ravel.Coordination.SingletonLock` — the singleton lock module owned
by sub-D's coordination implementation plan (landed 2026-04-21 at
`lib/ravel/coordination/singleton_lock.ex`). F's daemon boot sequence
(Task 10) calls `SingletonLock.acquire/1` to acquire a `flock(2)` on
the system-wide `<runtime_dir>/ravel/daemon.lock` — **outside any
vault** (Linux `$XDG_RUNTIME_DIR/ravel/`, macOS `$TMPDIR/ravel/`), so
Obsidian never sees it and two vaults on one machine still conflict.
Release is implicit via BEAM port close on daemon exit (the lock
survives SIGKILL/OOM correctly because flock release is tied to file-
handle close, not explicit API calls). See F design doc §4.6 and
Appendix C Q8. Task 0's gate (cleared 2026-04-21) verified the module
is landed before Tasks 10/12 run.

Earlier drafts of F referred to this module as `Ravel.DaemonLock` and
placed the lock at `<vault>/runtime/daemon.lock`; both were corrected
inline at the Task 0 gate to match sub-D's landed names and paths.

### Consumes from sub-D — FileIO mandatory I/O layer

`Ravel.FileIO` is the mandatory layer for all daemon file I/O
(also from sub-D). Design doc:
`{{PROJECT}}/docs/superpowers/specs/2026-04-16-sub-D-coordination-design.md`.

**Writes** — every daemon write must use `FileIO.write_safe/3` with
an appropriate file category:
- `:exclusive` — daemon regenerates unconditionally, no conflict check
  (e.g. `<vault>/plan-catalog.md` written by `CatalogRenderer`)
- `:phase_owned` — daemon-authoritative, warn on conflict
- `:human_editable` — human-authoritative, discard on conflict
F's `CatalogRenderer` (Task 9) uses `:exclusive`.

**Reads** — all plan-file reads in `PlanActor` and routing must use
`FileIO.read_tracked/1` (not bare `File.read/1`). This causes
`FreshnessTracker` (a GenServer in sub-D's coordination layer) to
record SHA-256 hashes of each read, which is the mechanism that makes
content-hash freshness checking work for plan files. Tasks 1, 4, 6,
7, and 8 all read plan files and must route those reads through
`read_tracked/1`.

### Consumes from sub-M

`:telemetry` + typed `Ravel.Event.*` structs for observability.
F emits `%ActorStateChange{}`, `%MessageRouted{}`, `%RuleFired{}`,
`%RuleSuggestion{}`, and `%DispatchProcessed{}` / `%QueryAnswered{}`
structs at the boundary. M owns the transport and subscribers.

### Relationship with sub-H (added 2026-04-16)

Sub-H owns the TUI binary (`ravel-tui`) and the `ravel cmd`
CLI wrapper. Both speak the NDJSON socket protocol that F's Task 11
defines. Sub-H's Session-21 brainstorm finalised a 17-command daemon
palette; two of those commands (`create_plan`, `adopt_project`) are
now listed explicitly in Task 11's command set and must be handled
daemon-side by F. Sub-H also identified a **local HTTP docs server**
capability (per-command help pages, each with a help URL). Scope
allocation: provisionally sub-F (the HTTP listener runs in the daemon
process). Task 13a in this backlog is the placeholder; the design doc
must be updated to confirm scope and describe the `[docs]` section in
`daemon.toml` before Task 13a is implemented.

### Depended on by sub-N (added 2026-04-15; updated 2026-04-16)

F's `Ravel.Actor` behaviour (Task 2) and `ActorSupervisor`
child-spec API (Task 5) are on the **critical path for sub-N Phase 5+**.
Sub-N's internal Task 0 is an explicit gate: sub-N Tasks 16+
(ExpertActor implementation) cannot start until F has shipped both
interfaces. Sub-N Tasks 1–15 are independent and can run immediately.
The authoritative ExpertActor spec (persona format, ScopeMatcher,
retrieval strategies, message/event structs) lives at
`{{PROJECT}}/docs/superpowers/specs/2026-04-15-sub-N-domain-experts-design.md`
and must be consulted when Task 3's stub is replaced. **Prioritise
delivering Tasks 2 and 5 early** so sub-N's Phase 5 is not delayed.

**Sub-N plan complete (2026-04-16):** Sub-N's brainstorm has produced
a concrete 29-task implementation plan. ExpertActor internals are now
locked — the following must be treated as fixed contracts when
implementing F's interfaces:

- **ExpertActor model:** stateless dialogue over ETS-backed
  `Ravel.Expert.DialogueRegistry`; tag-based exact-string scope
  matching via `ScopeMatcher` behaviour; `KeywordSection` retrieval
  strategy behind `ExpertRetrieval` behaviour; parallel Stage 5
  fan-out with per-expert absorb writes.
- **`ExpertRetrieval` behaviour:** `@behaviour Ravel.ExpertRetrieval`
  with locked signature `retrieve/3 :: (expert_id, query_text, opts)
  -> {:ok, [%KnowledgeEntry{}]} | {:error, term()}`. F's `Ravel.Actor`
  callback set must not constrain how sub-N implements this.
- **`ActorSupervisor` API:** sub-N calls `start_actor/2`, `stop_actor/1`,
  `send_message/2` (generic forms) in addition to the per-type
  `start_plan_actor/1` / `start_expert_actor/1`. All five must be in
  Task 5's public API.
- **Message/event structs from sub-N** (for consistency when F's
  actor behaviour is designed): `%ExpertAbsorbCandidate{}`, verdict
  structs (`READY ABSORB`, `READY REJECT`, `READY CROSS_LINK`),
  `%Expert.DialogueStarted{}`, `%Expert.TurnCompleted{}`,
  `%Expert.ConflictDetected{}`. These flow into sub-M's event bus;
  F's actor behaviour must not conflict with these shapes.

### Depends on (not yet landed at F's scaffolding time)

Implementation of F **cannot start** until the downstream task lists
for sub-B and sub-C are rewritten against their Session-11 / Session-12
design-doc rewrites. Both sibling backlogs carry explicit gate tasks
for that rewrite. F's Task 0 is the symmetric gate. Sub-A, sub-E, and
sub-M amendments are not blockers for F to begin — F can progress
against their design docs — but some integration tasks (Tasks 22–24)
become real only once those siblings land.

## Non-goals for this plan

- **Sub-N (ExpertActor internals)** — F ships the type hole only.
  Persona format, retrieval strategies, default expert set, and
  ingestion Stage 5 integration are all sub-N's scope.
- **Sub-O (mixture of models)** — F reserves the schema hooks; does
  not wire them.
- **Sub-P (team mode)** — F reserves the schema hooks; does not wire
  them.
- **Rust TUI binary (`ravel-tui`) and `ravel cmd` CLI** —
  both belong to sub-H's implementation scope. This plan scaffolds
  only the daemon-side client-listener (Task 11) that both clients
  connect to.
- **Phase-cycle mechanics (B's scope)** — F consumes `PhaseRunner`;
  it does not reimplement or extend the phase flow.

## Bootstrap discipline

Until sub-G's migration lands and Ravel v1 replaces Ravel-Lite,
this plan runs on existing Ravel-Lite machinery. Do not assume
features Ravel does not yet have. The `run-plan.sh` cycle driver,
`phases/work.md`, fixed-memory conventions, and placeholder
substitutions all come from Ravel-Lite for the duration of this
plan's life.

## Hard errors by default

Every invariant violation, I/O failure, parse error, ambiguous state,
and unexpected condition fails hard with a clear diagnostic naming
the offending file and line. Soft fallbacks require explicit written
rationale in the design doc. Project-wide; F carries it into every
actor, router, and dispatch-processor path.

## Amendment tasks rewrite specs inline, not as supersede layers

If a downstream pivot invalidates parts of F's design doc during
implementation, rewrite the affected sections inline and record the
correction as a new Decision Trail entry in Appendix A. Do not
append supersede blocks. Validated at scale by sub-B (2296 lines,
three simultaneous pivots absorbed) and sub-C (1186 lines, one
major pivot absorbed).
