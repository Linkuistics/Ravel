# Backlog — Sub-project F: Plan Hierarchy, Actor Model, Dispatch, Declarative Routing

Implementation backlog for sub-project F. All tasks derive from §11 of
the design doc at
`{{PROJECT}}/docs/superpowers/specs/2026-04-14-sub-F-hierarchy-design.md`.
Consult the spec (§1–§10) before starting any task.

**Sibling plan scaffolded Session 13 (2026-04-15)** of the parent
orchestrator plan. The BEAM PTY spike (Session 10) validated pipes-only
erlexec, sub-C's design doc was rewritten inline for Elixir/BEAM in
Session 11, and sub-B's was rewritten in Session 12 — together these
unblock F's implementation runway. **Task 0 gate cleared 2026-04-21**
— all preconditions satisfied, drifts rewritten inline, Tasks 1+ are
unblocked. **Tasks 1 and 2 landed 2026-04-21** (VaultDirectory +
Actor behaviour) and archived from this backlog. Immediate next:
**Task 3** (ExpertActor stub, no sibling deps) and **Task 9**
(CatalogRenderer, no sibling deps) can start in parallel. Task 5
(ActorSupervisor, gates sub-N Phase 5+) follows Task 3. Task 4
waits on sub-B's rewritten `PhaseRunner`.

## Task Backlog

## §11.1 Elixir scaffolding

### Task 3 — `Ravel.ExpertActor` stub `[foundation]`
- **Status:** not_started
- **Dependencies:** Task 2 (done 2026-04-21 — unblocked)
- **Description:** Implement `Ravel.ExpertActor` as a GenServer
  satisfying the `Ravel.Actor` behaviour but returning
  `{:error, :not_yet_implemented}` for every Query it receives.
  The type hole exists so the supervision tree, router, and
  dispatch processor can wire against ExpertActor targets before
  sub-N lands. Do not implement persona / retrieval / scope logic
  here — that is sub-N's scope.
- **Stub-replacement reference (2026-04-15):** When sub-N lands and
  this stub is replaced by a full implementation, the authoritative
  spec for ExpertActor's internals (persona format, ScopeMatcher,
  retrieval strategies, message structs, event structs) is
  `{{PROJECT}}/docs/superpowers/specs/2026-04-15-sub-N-domain-experts-design.md`.
  The stub replacement is not free-form — it must conform to that
  design doc. Sub-N Task 15 (early-deliverable PR) and Tasks 16+
  implement the actual ExpertActor; this Task 3 only provides the
  typed shell.
- **Results:** _pending_

### Task 4 — `Ravel.PlanActor` wrapping sub-B's `PhaseRunner` `[foundation]`
- **Status:** not_started
- **Dependencies:** Task 2 (done 2026-04-21), sub-B's `PhaseRunner`
  module landed per the rewritten sub-B backlog — still blocked on sub-B
- **Description:** GenServer implementing `Ravel.Actor` that
  hosts sub-B's `PhaseRunner` in its state. Message protocol per
  §4.5 of sub-B's rewritten spec: accept `{:run_phase, phase}` from
  attached clients, delegate to `PhaseRunner.run_phase/4`, emit
  `%PhaseLifecycle{}` events to sub-M. On Dispatch/Query arrival,
  append to the plan's backlog file via `DispatchProcessor`
  (Task 7) at phase exit, not on arrival. Crash recovery per §9.4:
  rebuild actor state from the plan's filesystem files on OTP
  restart; do not hold in-memory state the filesystem can't
  reproduce.
- **Results:** _pending_

### Task 5 — `Ravel.ActorSupervisor` `[foundation]`
- **Status:** not_started
- **Dependencies:** Tasks 3, 4
- **Description:** OTP DynamicSupervisor supervising all actor
  GenServers with `restart: :transient` per §4.5 of F's design doc.
  Provides `start_plan_actor/1` and `start_expert_actor/1`
  functions. Children are started lazily on first message to a
  qualified ID; idle actors may be stopped to reclaim memory (v1
  policy: no idle eviction — simpler).
- **Critical-path note (2026-04-15, updated 2026-04-21):** Task 2
  (`Ravel.Actor` behaviour) shipped 2026-04-21. This task alone now
  gates sub-N Phase 5+. Sub-N's ExpertActor (Tasks 16+) calls
  `ActorSupervisor.start_expert_actor/1` directly; until this
  child-spec API exists sub-N cannot wire its GenServer into the
  supervision tree.
- **API spec update (2026-04-16):** Sub-N's completed plan locked
  the `ActorSupervisor` child-spec API as used by sub-N. In addition
  to `start_plan_actor/1` and `start_expert_actor/1`, the module
  must expose: `start_actor/2`, `stop_actor/1`, `send_message/2`.
  Sub-N's ExpertActor implementation (Tasks 16+) calls these
  generic forms. Ensure all three are part of the public API when
  this task ships.
- **Results:** _pending_

### Task 6 — `Ravel.Router.Server` + mailbox NDJSON + cursor `[foundation]`
- **Status:** not_started
- **Dependencies:** Task 5
- **Description:** Router GenServer that owns message routing per
  §4.7. Accepts inbound messages (from clients, from phase-exit
  processors, from Level 2 agent responses), delegates routing
  decisions to `Ravel.Datalog.Server` (Task 14) via a `route/3`
  Datalog query, and dispatches to target actors via the supervisor.
  Persists mailbox state to `<vault>/runtime/mailboxes/<qid>.jsonl`
  (NDJSON append) with a cursor file tracking last-processed index
  per §4.8. Crash recovery replays from the cursor.
  **Also exposes `Ravel.Router.handle_tool_call(name, input,
  session_id, opts)`** (spec §5.3, Appendix C Q9) — the tool-call-
  boundary entry point that sub-C's session GenServer calls when a
  harness's LLM invokes an injected Ravel tool. `opts` carries
  `reply_ref: <tool_use_id>`; the router runs the query in a
  supervised task and replies asynchronously via
  `{:router_reply, <tool_use_id>, result}` info message to the
  calling session pid. Full reply shape and responsibilities are
  locked by sub-C §4.5.3 / §11.3.
- **Results:** _pending_

### Task 7 — `Ravel.DispatchProcessor` `[foundation]`
- **Status:** not_started
- **Dependencies:** Tasks 1, 6
- **Description:** Phase-exit hook that parses `dispatches.yaml`
  from the plan's staging directory (§5.2), validates each entry
  (§9.1: cannot target self, target-plan only for same-project,
  target-project only for cross-project, `suggested-target-plan`
  scope check), routes through the router (Task 6), and writes to
  the target's `Received` section mechanically. Uses the
  `dispatches.yaml.processing` marker + index pattern for at-most-
  once delivery (§9.4 crash recovery). Called by `PhaseRunner` on
  non-compact phase exit.
- **Results:** _pending_

### Task 8 — `Ravel.QueryProcessor` `[foundation]`
- **Status:** not_started
- **Dependencies:** Tasks 1, 6
- **Description:** Phase-exit hook mirroring `DispatchProcessor`
  for `queries.yaml` per §2.5 and §5.2. Same validation rules,
  same crash-recovery pattern. Query responses are routed back to
  the originating plan's `Dispatched` section as audit trail;
  mid-session Query responses flow via sub-C's tool-call boundary
  (§5.3) and are NOT written via this processor.
- **Results:** _pending_

### Task 9 — `Ravel.CatalogRenderer` `[foundation]`
- **Status:** not_started
- **Dependencies:** Task 1
- **Description:** Render `<vault>/plan-catalog.md` from a
  `%VaultDirectory{}` per §3.2. Grouping by project, each plan
  and expert rendered with qualified ID + description + dispatch
  rule summary. Also provides the substitution content for the
  `{{VAULT_CATALOG}}` placeholder consumed by sub-B's phase
  prompts. Regeneration triggers per §3.3: plan mutation events
  from the router, and every phase-prompt render. File header
  marks it "machine-owned" (§9.3). Hard error on description
  violations discovered at render time.
  Write `<vault>/plan-catalog.md` via
  `Ravel.FileIO.write_safe/3` with category `:exclusive` —
  the daemon regenerates this file unconditionally; no conflict
  check needed.
- **Results:** _pending_

---

## §11.2 Daemon binary

### Task 10 — `ravel daemon` mix task / escript `[daemon]`
- **Status:** not_started
- **Dependencies:** Tasks 5, 6, 13, and sub-A vault discovery
- **Description:** Entry point per §4.6. Startup sequence:
  1. Parse CLI args (`--vault`, `--log-level`).
  2. Resolve vault via sub-A (`--vault` → `RAVEL_VAULT` →
     user config → hard error).
  3. Call `VaultDirectory.load` + hook into sub-A's
     `verify_vault`.
  4. Acquire singleton lock (Task 12).
  5. Start OTP application tree rooted at
     `Ravel.Application`.
  6. Start the `ClientListener` (Task 11).
  7. Install signal handlers (SIGTERM → graceful shutdown,
     SIGINT → graceful shutdown, SIGHUP → re-parse
     `<vault>/ontology/routing.dl` via `Datalog.Server`).
  Shutdown path: stop listener, drain in-flight phase runs, stop
  supervisor, release lock, exit.
- **Results:** _pending_

### Task 11 — `Ravel.ClientListener` + `Ravel.ClientConnection` `[daemon]`
- **Status:** not_started
- **Dependencies:** Task 6
- **Description:** Unix socket listener at
  `<vault>/runtime/daemon.sock` per §7.1 accepting NDJSON commands
  per §7.2. Per-connection `GenServer` handling the command set:
  `attach-plan`, `detach`, `run-phase`, `list-plans`,
  `show-catalog`, `subscribe-events`, `review-rule-suggestion`,
  `create_plan`, `adopt_project`, etc. `create_plan` creates a
  new plan directory under an existing project (initialises
  `plan-state.md` with required frontmatter and description).
  `adopt_project` registers an external directory as a
  Ravel-managed project under the vault's `projects/` tree
  (symlink + `VaultDirectory` reload). These two commands were
  finalised in sub-H's brainstorm (Session 21, 2026-04-16) as
  part of the 17-command daemon palette; they are daemon-side
  commands handled here, not by the TUI or `ravel cmd`.
  Forward-compatibility per §7.3 (unknown commands →
  structured error, not crash). Multi-client semantics per §7.4
  (multiple attached clients on one plan share event stream;
  `run-phase` requires exclusive control).
  **Note (2026-04-16):** `ravel cmd` (a headless CLI wrapper
  also finalised in sub-H Session 21) speaks the same NDJSON
  socket protocol as the TUI — it is a second official client.
  No extra daemon-side ceremony is required; the existing
  forward-compat and multi-client design already supports it.
- **Results:** _pending_

### Task 12 — Daemon singleton lock `[daemon]`
- **Status:** not_started
- **Dependencies:** Task 10, `Ravel.Coordination.SingletonLock` from
  sub-D (landed 2026-04-21 at
  `lib/ravel/coordination/singleton_lock.ex`)
- **Description:** Acquire `flock(2)` on the system-wide
  `<runtime_dir>/ravel/daemon.lock` (Linux
  `$XDG_RUNTIME_DIR/ravel/daemon.lock`, macOS
  `$TMPDIR/ravel/daemon.lock` — outside any vault) at startup via
  `Ravel.Coordination.SingletonLock.acquire/1`. Second daemon fails
  with an actionable diagnostic naming the incumbent daemon's PID and
  served vault path (sub-D writes both into the lock file at
  acquisition) per §9.3. Lock released on shutdown via the OS (BEAM
  closing the port's pipes on daemon exit → `flock(1)` exits →
  kernel releases the lock; this works on SIGKILL/OOM too because
  flock release is tied to file-handle close, not an explicit call).
  See F design doc §4.6 and Appendix C Q8.
- **Results:** _pending_

### Task 13 — `daemon.toml` parser `[daemon]`
- **Status:** not_started
- **Dependencies:** Task 0
- **Description:** Parse `<vault>/daemon.toml` with reserved
  sections per F-12. V1 fields: `[daemon]` (log level, socket
  path override), `[fact_extraction]` (model selection),
  `[harnesses.*]` (reserved for sub-O; parse-and-ignore-unknown
  with forward-compat warning per §9.3), `[peers]` (reserved for
  sub-P; hard error if non-empty per §9.3). Use `toml_elixir`
  or equivalent.
- **Results:** _pending_

### Task 13a — Local HTTP docs server `[daemon]` `[scope-tbd]`
- **Status:** not_started
- **Dependencies:** Task 10
- **Scope note (2026-04-16):** Identified in sub-H's brainstorm
  (Session 21) as a daemon capability: the daemon hosts a local
  HTTP server serving per-command help pages (each of the 17
  daemon commands has a help URL). The allocation between sub-F
  (daemon binary owner) and sub-H (TUI/client owner) is not yet
  settled. Provisional decision: sub-F owns the HTTP server
  because it runs inside the daemon process, not the TUI process.
  This task is a placeholder — before implementation begins,
  confirm the allocation and update the design doc accordingly.
- **Description (provisional):** Start a `Plug`/`Bandit` HTTP
  listener on a loopback port (configurable in `daemon.toml`
  under a new `[docs]` section; default: e.g. `127.0.0.1:4040`).
  Serve one HTML page per command at `/commands/<name>` (content
  is static, bundled at compile time or read from a priv/ tree).
  The TUI (sub-H) and `ravel cmd` can open help URLs in the
  user's browser via `xdg-open` / `open`. Hard error at startup
  if the port is already in use (do not fall back silently to a
  random port). The `[docs]` section of `daemon.toml` should be
  parsed in Task 13 once scope is confirmed.
- **Results:** _pending_

---

## §11.3 Declarative routing

### Task 14 — `Ravel.Datalog.Server` integration for `ontology/routing.dl` `[routing]`
- **Status:** not_started
- **Dependencies:** Task 6, Sub-R engine landed (`Ravel.Datalog.Server`)
- **Description:** Wire sub-R's `Ravel.Datalog.Server` (singleton
  gen_server delivered by the knowledge-ontology sub-project) into F's
  supervision tree per spec §4.5 and §6.1. On startup, `Datalog.Server`
  parses `<vault>/ontology/routing.dl` and builds the initial fact
  base. Rules derive `route/3` facts of shape
  `route(MsgType, Target, TargetKind)` where `MsgType ∈ {dispatch,
  query}` and `TargetKind ∈ {target_plan, target_project,
  target_expert}`. `Router.Server` (Task 6) queries `Datalog.Server`
  for routing decisions — no compiled Elixir module, no `@behaviour`
  shim. Parse error on startup is a hard failure per §9.3 naming the
  offending `.dl` line number; the daemon refuses to start. The file
  is pure Datalog with no host-language dependencies (§6.1). Sub-R's
  design doc §4.4 and §7 are the authoritative reference for the
  engine API.
- **Results:** _pending_

### Task 15 — `Ravel.FactExtractor.Server` + `resolves_to/2` normalization `[routing]`
- **Status:** not_started
- **Dependencies:** Task 14, sub-C adapter landed
- **Description:** GenServer that extracts concern keywords (facts)
  from each outbound message body via a cheap LLM pass per §6.3.
  Uses sub-C's `HarnessAdapter` with a tightly constrained prompt
  and tool profile. Default model: `claude-haiku-4-5` via the Claude
  Code adapter; overridable in `daemon.toml` under `[fact_extraction]`
  (`harness`, `model`, `max_topics`, `timeout_ms`). Reserved for sub-O
  to swap in local models. Extracted keywords are asserted as
  `concerns/1` facts into the Datalog fact base, then each keyword is
  normalized via a `resolves_to/2` query against sub-R's ontology so
  synonyms and aliases map to canonical concepts before rule
  evaluation (§6.3; e.g. `"rust_lang"` or
  `"foreign_function_interface"` resolve to canonical forms). Unresolved
  keywords pass through unchanged. The normalized `concerns/1` facts
  plus `msg_type/1` drive Datalog `route/3` derivation.
- **Results:** _pending_

### Task 16 — Filesystem watcher for `ontology/routing.dl` hot reload `[routing]`
- **Status:** not_started
- **Dependencies:** Task 14
- **Description:** Watch `<vault>/ontology/routing.dl` for mtime
  changes (via `FileSystem` or periodic poll — whichever is simpler
  for v1) and trigger `Ravel.Datalog.Server` to re-parse the `.dl`
  fact base and re-evaluate per §6.2. On success the new rules are
  active; subsequent routing decisions use them. On parse/validation
  failure, log a warning and leave the previous rule set loaded;
  surface the failure to attached TUI clients as a
  `%RuleReloadError{}` event (§9.3). Users can also trigger a reload
  manually via `ravel rescan` or a TUI command. The watcher may be
  absorbed into `Ravel.Datalog.Server`'s own filesystem-watcher
  implementation if sub-R provides one — defer that integration
  decision to implementation time.
- **Results:** _pending_

### Task 17 — Datalog rule validation + actionable error reporting `[routing]`
- **Status:** not_started
- **Dependencies:** Task 14
- **Description:** Validate rules in `ontology/routing.dl` at load
  time: syntax, stratifiability, range restriction, and that each
  derived `route/3` fact has the expected arity and variant shape
  (`MsgType ∈ {dispatch, query}`, `TargetKind ∈ {target_plan,
  target_project, target_expert}`). If a rule derives something
  outside the allowed variant set at evaluation time, log the
  offending rule + input facts + actual derivation, treat as
  `:no_route`, and fall back to Level 2 per §6.4. Error reporting
  must name the line in `ontology/routing.dl` (preserve source
  locations via the engine's parser diagnostics). Sub-R's engine is
  expected to expose parse/validation errors with line numbers; this
  task plumbs those diagnostics through to the startup hard-error
  path (§9.3) and to the TUI event stream on hot-reload failures.
- **Results:** _pending_

---

## §11.4 Level 2 routing agent

### Task 18 — Level 2 prompt template + spawn path `[level2]`
- **Status:** not_started
- **Dependencies:** Task 14, sub-C adapter landed
- **Description:** Prompt template for the Level 2 routing agent
  per §6.4. Fresh-context Claude Code session, tool profile
  scoped to read-only access of the target project's vault
  subtree (plans + source code), with authority to pick a
  specific target plan or reject with reasoning. Spawned via
  sub-C's adapter. Input: the message being routed, the
  target project name, and the vault catalog excerpt for that
  project. Output: a YAML block with `target-plan:` or
  `routing-response:` fields, optionally including a
  `suggested-rule:` block containing a proposed Datalog rule
  (see Task 20).
- **Results:** _pending_

### Task 19 — Rejection + retarget feedback to origin `[level2]`
- **Status:** not_started
- **Dependencies:** Task 18
- **Description:** When Level 2 rejects or times out (§9.1),
  write a structured entry to the origin plan's `Dispatched`
  section per §2.6 recording the rejection reason and any
  retarget suggestion. The origin plan's next phase sees the
  rejection and can re-dispatch manually. Timeout threshold:
  5 minutes per §9.1.
- **Results:** _pending_

### Task 20 — Datalog rule suggestion extraction `[level2]`
- **Status:** not_started
- **Dependencies:** Task 18
- **Description:** Parse optional `suggested-rule:` block from
  Level 2 output per §6.5. The suggestion is a Datalog rule
  fragment (a `route/3` clause body referencing `msg_type/1`,
  `concerns/1`, and `resolves_to/2`). Stage each suggestion as a
  `.dl` fragment at `<vault>/runtime/staging/proposed-rules/<id>.dl`
  for user review. Do not auto-apply — accepting the suggestion is
  a user action via the TUI per the learning-loop design.
- **Results:** _pending_

### Task 21 — TUI event emission + acceptance flow for pending rule suggestions `[level2]`
- **Status:** not_started
- **Dependencies:** Tasks 11, 20
- **Description:** Emit `%RuleSuggestion{}` events over the client
  event stream whenever a new `.dl` suggestion is staged. Accepted
  via the `review-rule-suggestion` NDJSON command from an attached
  client (Task 11's command set). On acceptance, append the
  suggestion to `<vault>/ontology/routing.dl` (committed to vault git
  per §6.5), let the filesystem watcher (Task 16) pick up the change
  to re-parse and hot-reload the fact base, and delete the staged
  file.
- **Results:** _pending_

---

## §11.5 Integration

### Task 22 — Hook `VaultDirectory.load` into sub-A's `verify_vault` `[integration]`
- **Status:** not_started
- **Dependencies:** Task 1, sub-A amendment absorbed, sub-A
  implementation landed
- **Description:** Extend sub-A's `verify_vault` to call
  `Ravel.VaultDirectory.load/1` as its second step (after
  vault identity verification). This surfaces description
  discipline errors (F-8) and plan-state parse errors (§9.2)
  at daemon startup rather than deferring them to first use.
- **Results:** _pending_

### Task 23 — Hook `DispatchProcessor` + `QueryProcessor` into sub-B phase exit `[integration]`
- **Status:** not_started
- **Dependencies:** Tasks 7, 8, sub-B rewritten `PhaseRunner` landed
- **Description:** Register F's processors as phase-exit hooks on
  sub-B's `PhaseRunner` per step 13 of B's rewritten 13-step flow.
  Processors run only on non-compact phases (compact is strictly
  lossless). Failures in processors must surface as hard errors
  to the attached client; they must not silently swallow routing
  failures.
- **Results:** _pending_

### Task 24 — `:telemetry` events for actor state / routing / rules `[integration]`
- **Status:** not_started
- **Dependencies:** sub-M amendment absorbed, sub-M implementation
  landed
- **Description:** Emit typed `Ravel.Event.*` structs at the
  boundary for every architecturally-meaningful state change per
  F §4 and §6. Events: `%ActorStateChange{}`, `%MessageRouted{}`,
  `%RuleFired{}`, `%RuleReloadError{}`, `%RuleSuggestion{}`,
  `%DispatchProcessed{}`, `%QueryAnswered{}`, `%Level2Invoked{}`,
  `%Level2Rejected{}`. `%RuleReloadError{}` covers both Datalog parse
  errors on hot reload and rule-validation failures (Tasks 16, 17).
  Each event also flows through
  `:telemetry.execute/3` for subscribers. M owns the handler and
  transport; F owns the event boundary.
- **Naming drift with sub-M (flagged 2026-04-21 at F's Task 0 gate;
  Appendix C Q10):** sub-M's event registry still lists
  `%Ravel.Event.RuleCompileError{}` (referring to the deleted
  `routing.ex` compile step); `%RuleReloadError{}` is the
  semantically-correct Datalog-era name F commits to. Sub-R
  separately emits `%OntologyReloadFailed{}` for any `ontology/*.dl`
  reload, so there is a legitimate open question of whether F should
  consume sub-R's event instead. Resolution is a cross-plan dispatch
  (sub-M amendment) owned by the orchestrator, not by F's Task 24
  implementation — this task keeps `%RuleReloadError{}` as specified
  until the dispatch lands.
- **Results:** _pending_

---

## §11.7 Tests

### Task 25 — Unit tests (Elixir) `[tests]`
- **Status:** not_started
- **Dependencies:** Tasks 1–9 (incremental — unit tests land with each)
- **Description:** Per §10.1 — ExUnit tests for every module's
  pure logic: VaultDirectory walk + parse, path-based qualified
  ID computation, description validation, mailbox NDJSON
  encoding / cursor advance, dispatch/query YAML parse +
  validation, catalog rendering, routing clause invocation. Tests
  must cover every §9 edge case. TDD: tests written first.
- **Results:** _pending_

### Task 26 — Integration tests with fixture vaults `[tests]`
- **Status:** not_started
- **Dependencies:** Task 25 baseline established
- **Description:** Per §10.2 — fixture vaults committed to
  `{{PROJECT}}/test/fixtures/vaults/` each exercising a specific
  shape (empty vault, single project, multi-project, nested
  children, symlinks, broken symlinks, description violations).
  Integration tests drive `VaultDirectory.load`, catalog
  rendering, and router startup end-to-end against each fixture.
- **Results:** _pending_

### Task 27 — Dispatch / Query processor tests `[tests]`
- **Status:** not_started
- **Dependencies:** Tasks 7, 8, 26
- **Description:** Per §10.3 — end-to-end tests of
  `dispatches.yaml` and `queries.yaml` processing including
  the `.processing` marker crash-recovery protocol. Simulate
  daemon crash between marker-write and file-delete, restart,
  verify at-most-once delivery (no duplicated `Received`
  entries).
- **Results:** _pending_

### Task 28 — Optional e2e test `[tests]`
- **Status:** not_started
- **Dependencies:** Tasks 10, 11, 23, 24
- **Description:** Per §10.4 — full daemon-up, attach-plan,
  run-phase, dispatch, level2-reject, rule-suggestion, accept-
  suggestion round trip against a scripted fixture. Gated
  behind `mix test --only e2e` so it doesn't run on every
  push. Evidence captured as a JSON-Lines event trace.
- **Results:** _pending_

---

## Out of scope — carry-forward notes

These items are referenced by F's §11 but belong to a different plan
and must NOT be implemented here:

- **Rust `ravel-tui` binary (§11.6)** — ratatui rendering, socket
  client, attach-detach UI, phase-run commands, live harness
  streaming, rule-suggestion review pane. This is a separate
  implementation plan to be scaffolded once F's daemon-side
  contract (Task 11) lands. F ships only the daemon side of the
  NDJSON protocol.
- **Cross-plan amendment landings (§11.8)** — done by F's own
  triage phase in Session 9 (2026-04-14) of the parent
  orchestrator plan. The amendment tasks are now on the
  orchestrator backlog under Priority 1. F does not execute them.
- **Sub-N ExpertActor internals** — F ships the type hole (Task 3)
  only. Persona format, retrieval strategies, default expert set
  all belong to sub-N's separate brainstorm and plan.
- **Sub-O mixture of models** — F reserves the schema hooks in
  Task 13 only. Multi-adapter wiring and per-actor model selection
  are v1.5+ work.
- **Sub-P team mode** — F reserves the schema hooks in Task 13
  only. Cross-daemon transport is v2+ work.

## Sub-R adoption — MERGED (2026-04-20)

The earlier Tasks 29 (routing migration from `routing.ex` to Datalog)
and 30 (fact-extraction normalization via Datalog) have been **folded
into the routing-task cluster**. Datalog via sub-R's
`Ravel.Datalog.Server` is the v1 routing substrate from the start —
not a migration destination. The spec was amended 2026-04-16 (Appendix
C Q7): `<vault>/ontology/routing.dl` loaded by `Ravel.Datalog.Server`
replaces the superseded `<vault>/routing.ex` + pattern-matched Elixir
design. Erlog / embedded Prolog is no longer reserved.

Task mapping (old → new location):
- Task 29.1 (`Router.evaluate_rules/2` via Datalog) → Task 14
- Task 29.2 (`routing.ex` → `ontology/routing.dl`) → Task 14
- Task 29.3 (watcher for `.dl` fact-base reload) → Task 16
- Task 29.4 (Datalog validation + errors) → Task 17
- Task 29.5 (Level 2 prompts propose Datalog rules) → Task 18
- Task 29.6 (rule suggestions staged as `.dl` fragments) → Task 20
- Task 29.7 (TUI events + acceptance appends to `routing.dl`) → Task 21
- Task 29.8 (`Router.Server` depends on `Datalog.Server`) → Task 14
  (supervision tree wiring) + Task 6 dependency on Task 14
- Task 30 (fact normalization via `resolves_to/2`) → Task 15

No separate Sub-R adoption task remains in this backlog. All Datalog
work is native to Tasks 14–21.
