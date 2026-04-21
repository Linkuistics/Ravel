# Memory — Sub-project F: Plan Hierarchy, Actor Model, Dispatch, Declarative Routing

This plan implements sub-project F of the Ravel orchestrator merge. The
design is fully specified in the spec referenced below. Implementation
questions not answered by the spec go into this memory file (and
possibly back into the spec) rather than being invented ad hoc.

## Primary reference

**`{{PROJECT}}/docs/superpowers/specs/2026-04-14-sub-F-hierarchy-design.md`**
is the authoritative design document. Every task in this plan's backlog
derives from §11 of that spec. Consult §1–§10 of the spec before acting
on any implementation task.

## Parent plan

The orchestrator-level plan lives at
`{{PROJECT}}/LLM_STATE/ravel-orchestrator/`; moves to
`{{PROJECT}}/ravel/project-root/` after sub-G's migration lands. It
coordinates this sub-plan with siblings (A, B, C, D, E, G, H, I, M, N).
The parent plan's `memory.md` holds cross-sub-project architectural
state; this file holds only sub-F-specific implementation state.

## Key architectural anchors (quick reference; spec is canonical)

Consult §1–§10 of the design doc before acting on any of the decisions below.

### F-1. Ravel is a persistent actor daemon on BEAM

V1 runs as a single long-running Elixir/OTP application.
`ravel daemon` is the entry point; it hosts all actors, message routing,
supervision, harness adapters, fact extraction, and ingestion. OTP
supervision, message passing, hot code reload, and distribution
transparency are BEAM primitives — use them, do not hand-roll.

### F-2. Two sealed actor types: PlanActor and ExpertActor

Both implement a shared `Ravel.Actor` behavior on top of `GenServer`.
The set is **sealed** for v1 — adding a third type is a code change, not
a plugin mechanism. PlanActor progresses through phase cycles (wraps
sub-B's `PhaseRunner`). ExpertActor is a type-hole stub returning
`:not_yet_implemented` until sub-N lands.

### F-3. Two message types: Dispatch and Query

Both share target resolution, declarative routing, Level 2 fallback, and
audit trail conventions. Dispatch is fire-and-forget to a target's
backlog `Received` section. Query is request-response via the tool-call
boundary in sub-C's adapter.

### F-4. Plan hierarchy via `project-root` convention

Every adopted project has exactly one root plan at
`<project>/ravel/project-root/`. Name is **reserved**. Nested child
plans nest arbitrarily beneath it. A directory is a plan iff it contains
`plan-state.md`. `knowledge/` lives as a sibling of `project-root/`,
not inside it.

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
  Ravel spawns a **Level 2 routing agent** — fresh-context Claude Code
  session scoped to the target project's vault subtree (plans + source
  code), with authority to pick a specific target plan or reject with
  reasoning.

### F-7. Vault catalog as LLM-facing enumeration

`{{VAULT_CATALOG}}` is substituted at phase-prompt render time with the
full vault catalog at `<vault>/plan-catalog.md` — every plan and expert,
grouped by project, with 120-char descriptions and dispatch rules.
Auto-regenerated on plan mutation and every phase-prompt render. Replaces
`{{RELATED_PLANS}}`. No plan's LLM phase ever reads another plan's files
directly.

### F-8. Description discipline: 120-character hard cap

Every plan and expert has a `description:` frontmatter field, ≤120
characters, enforced at load time (hard error on overflow).
Keyword-dense, noun-phrase-led, no self-reference, no placeholders.
Permanent scope declaration, not current-state description.

### F-9. Declarative routing with LLM-fallback learning loop

Routing rules live in `<vault>/ontology/routing.dl`, a user-editable
Datalog file loaded by `Ravel.Datalog.Server` (singleton GenServer,
sub-R). Rules derive a `route/3` fact `route(MsgType, Target,
TargetKind)`. Hot reload via filesystem watcher re-parses on change;
edits take effect without daemon restart. Concern keywords are extracted
by a cheap LLM pass (Haiku via sub-C) and asserted as `concerns/1`
facts, then normalized via `resolves_to/2` queries against sub-R's
ontology before rule evaluation. When no `route/3` fact is derived (or
multiple are ambiguously derived), the Level 2 routing agent runs as
fallback and may propose a new Datalog rule the user accepts into
`routing.dl` — closing a learning loop. `routing.dl` replaces both the
pattern-matched Elixir `routing.ex` and the Erlog reservation (Appendix
C Q7). Parse errors on startup are a hard failure; parse errors during
hot reload keep the previous rule set and surface the failure in the TUI
(spec §9.3).

### F-10. Runtime: Elixir on BEAM

Erlang/OTP 28 + Elixir 1.19.x. Dependencies: `erlexec`, `jason`,
`telemetry`, `yaml_elixir`. Rust is used only for the TUI client binary
(separate plan). Gleam is a future migration target if Elixir's dynamic
typing proves painful for the invariant-heavy design.

### F-11. Two official clients over Unix socket NDJSON

`ravel-tui` is a standalone Rust binary using `ratatui` + `tokio` +
`serde_json`. Connects to the daemon over a local Unix socket at
`<vault>/runtime/daemon.sock` speaking NDJSON. No Rustler / no NIFs —
the socket is the integration boundary. This plan scaffolds the daemon
side of that contract; the TUI binary belongs to a separate
implementation plan.

`ravel cmd` is a second official client: a headless CLI wrapper speaking
the same NDJSON socket protocol. Enables scripted / CI access to the
daemon without a TUI session. No extra daemon-side ceremony required —
the existing forward-compat (§7.3) and multi-client (§7.4) design covers
it. Sub-F owns the daemon socket; both clients are sub-H's scope.

### F-12. Reserved extensibility hooks (sub-O, sub-P)

`daemon.toml` reserves `[harnesses.*]` sections for sub-O (mixture of
models, v1.5+). Actor declarations reserve the `model:` field. The
`[peers]` section and `<peer>@<qualified-id>` syntax are reserved for
sub-P (team mode, v2+). V1 parses these sections with forward
compatibility (unknown `[harnesses.foo]` → warn and ignore; non-empty
`peers: []` → hard error with "team mode not supported in v1").

## Contract with sibling sub-projects

### Consumes from sub-A

Vault discovery and identity verification. F's `VaultDirectory.load` is
called from A's `verify_vault` path at daemon startup. A owns the vault
discovery chain (`--vault` → `RAVEL_VAULT` → user config → hard error);
F owns what happens next.

### Consumes from sub-B

`Ravel.PhaseRunner` and its 13-step `run_phase/4` flow. F's `PlanActor`
hosts this runner inside its `GenServer` state and invokes it in response
to `{:run_phase, _}` messages from attached clients. F's
`DispatchProcessor` and `QueryProcessor` are called by B's `PhaseRunner`
as phase-exit hooks on non-compact phases.

### Consumes from sub-C

`Ravel.HarnessAdapter` behaviour. F's Level 2 routing agent and
fact-extraction LLM pass spawn sessions via the same adapter the phase
runner uses. F's Query messages are delivered mid-session via C's
tool-call boundary (`Ravel.Router.handle_tool_call/4`).

**`handle_tool_call/4` signature is locked** (sub-C §4.5.3/§11.3):
accepts `reply_ref` in opts; dispatches async `{:router_reply,
tool_use_id, result}` back to the calling process. Do not change
unilaterally.

### Consumes from sub-D: SingletonLock

`Ravel.Coordination.SingletonLock` (not `Ravel.DaemonLock` — early
drafts used the wrong name) is at `lib/ravel/coordination/singleton_lock.ex`.
F's daemon boot (Task 10) calls `SingletonLock.acquire/1` for a `flock(2)`
on `<runtime_dir>/ravel/daemon.lock` — **outside any vault** (Linux
`$XDG_RUNTIME_DIR/ravel/`, macOS `$TMPDIR/ravel/`), so Obsidian never
syncs it and two vaults on one machine still conflict. Release is implicit
via BEAM port close on daemon exit; survives SIGKILL/OOM because flock
release is tied to file-handle close, not API calls. Task 0's gate is
cleared (module landed; required before Tasks 10 and 12). See F design
doc §4.6 and Appendix C Q8.

### Consumes from sub-D: FileIO mandatory I/O layer

`Ravel.FileIO` is mandatory for all daemon file I/O. Design doc:
`{{PROJECT}}/docs/superpowers/specs/2026-04-16-sub-D-coordination-design.md`.

**Writes** — use `FileIO.write_safe/3` with file category:
- `:exclusive` — daemon regenerates unconditionally, no conflict check
  (e.g. `<vault>/plan-catalog.md` by `CatalogRenderer`, Task 9)
- `:phase_owned` — daemon-authoritative, warn on conflict
- `:human_editable` — human-authoritative, discard on conflict

**Reads** — use `FileIO.read_tracked/1` (not bare `File.read/1`). Causes
`FreshnessTracker` (sub-D) to record SHA-256 hashes, enabling
content-hash freshness checking. Tasks 1, 4, 6, 7, and 8 read plan files
and must use `read_tracked/1`.

### Consumes from sub-M

`:telemetry` + typed `Ravel.Event.*` structs. F emits
`%ActorStateChange{}`, `%MessageRouted{}`, `%RuleFired{}`,
`%RuleSuggestion{}`, `%DispatchProcessed{}`, and `%QueryAnswered{}` at
the boundary. M owns transport and subscribers.

**Pending cross-plan drift (Appendix C Q10):** F's spec names the
routing-reload error event `%RuleReloadError{}`; sub-M has
`%RuleCompileError{}`. Awaiting orchestrator dispatch to resolve before
either side implements it.

### Sub-H owns both client binaries

Sub-H owns `ravel-tui` and `ravel cmd`. Both speak the NDJSON socket
protocol that F's Task 11 defines. Sub-H's brainstorm finalised a
17-command daemon palette; `create_plan` and `adopt_project` are in Task
11's command set and must be handled daemon-side by F. Local HTTP docs
server capability (per-command help pages, each with a help URL):
provisionally sub-F; Task 13a is the placeholder. The design doc must
confirm scope and describe the `[docs]` section in `daemon.toml` before
Task 13a is implemented.

### Sub-N depends on F Tasks 2 and 5

F's `Ravel.Actor` behaviour (Task 2) and `ActorSupervisor` child-spec
API (Task 5) are on the critical path for sub-N Phase 5+. Sub-N Tasks
16+ cannot start until both are shipped. Sub-N Tasks 1–15 can run
immediately. Spec:
`{{PROJECT}}/docs/superpowers/specs/2026-04-15-sub-N-domain-experts-design.md`.
Prioritise Tasks 2 and 5.

Sub-N's 29-task plan is complete; ExpertActor internals are locked — treat
the following as fixed contracts when implementing F's interfaces:

- **ExpertActor model:** stateless dialogue over ETS-backed
  `Ravel.Expert.DialogueRegistry`; tag-based exact-string scope matching
  via `ScopeMatcher`; `KeywordSection` retrieval strategy behind
  `ExpertRetrieval`; parallel Stage 5 fan-out with per-expert absorb
  writes.
- **`ExpertRetrieval` behaviour:** locked signature `retrieve/3 ::
  (expert_id, query_text, opts) -> {:ok, [%KnowledgeEntry{}]} |
  {:error, term()}`. F's `Ravel.Actor` callback set must not constrain
  sub-N's implementation.
- **`ActorSupervisor` API:** sub-N calls `start_actor/2`, `stop_actor/1`,
  `send_message/2` (generic forms) plus `start_plan_actor/1` and
  `start_expert_actor/1`. All five must be in Task 5's public API.
- **Message/event structs from sub-N:** `%ExpertAbsorbCandidate{}`,
  verdict structs (`READY ABSORB`, `READY REJECT`, `READY CROSS_LINK`),
  `%Expert.DialogueStarted{}`, `%Expert.TurnCompleted{}`,
  `%Expert.ConflictDetected{}`. F's actor behaviour must not conflict
  with these shapes.

### F's gate on sub-B and sub-C

F cannot start until sub-B and sub-C backlogs are rewritten against their
design-doc rewrites. F's Task 0 is the gate. Sub-A, sub-E, and sub-M
amendments are not blockers — F can progress against their design docs
— but integration tasks (22–24) become real only once those siblings land.

## Non-goals for this plan

- **Sub-N (ExpertActor internals)** — F ships the type hole only.
- **Sub-O (mixture of models)** — F reserves schema hooks; does not wire
  them.
- **Sub-P (team mode)** — F reserves schema hooks; does not wire them.
- **Rust TUI binary (`ravel-tui`) and `ravel cmd`** — sub-H's scope.
  This plan scaffolds only the daemon-side client-listener (Task 11).
- **Phase-cycle mechanics (B's scope)** — F consumes `PhaseRunner`; does
  not reimplement or extend the phase flow.

## Bootstrap discipline

This plan runs on Ravel-Lite machinery until sub-G's migration lands. Do
not assume features Ravel does not yet have. The `run-plan.sh` cycle
driver, `phases/work.md`, fixed-memory conventions, and placeholder
substitutions all come from Ravel-Lite.

## Hard errors by default

Every invariant violation, I/O failure, parse error, ambiguous state, and
unexpected condition fails hard with a clear diagnostic naming the
offending file and line. Soft fallbacks require explicit written rationale
in the design doc. Applies to every actor, router, and
dispatch-processor path.

## Amendment tasks rewrite specs inline

Invalidated spec sections are rewritten inline with a new Decision Trail
entry in Appendix A. Do not append supersede blocks.
