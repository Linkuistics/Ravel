# Backlog — Sub-project C: Harness Adapter Layer

Implementation backlog for sub-project C. All tasks derive from the design
doc at
`{{PROJECT}}/docs/superpowers/specs/2026-04-13-sub-C-adapters-design.md`
(original Session 6 brainstorm 2026-04-13; inline rewrite for the
BEAM/Elixir pivot + BEAM PTY spike findings 2026-04-15). The spec is
Elixir-native end to end; Session 6 material survives only as historical
decisions in Appendix A.

Tasks are listed in approximately recommended order, following the
implementation strategy in `{{PLAN}}/memory.md`: setup → day-1
verification → tool-call-boundary spike → `FixtureReplay` → ClaudeCode
wire layer → Session GenServer → lifecycle → tool-call boundary
implementation → instrumentation → tests → dogfood acceptance gate →
conditional warm-pool. The work phase picks the best next task with
input from the user.

> **Top-notice (2026-04-21).** Design doc was rewritten inline
> Session 11 (2026-04-15) for Elixir/OTP; the downstream task list was
> rewritten inline on 2026-04-21 (Task 0 — the orchestrator's
> critical-path dispatch from 2026-04-16; now complete). Tasks 1-29
> target Elixir idioms: `@behaviour Ravel.HarnessAdapter`, one
> `GenServer` per session, `erlexec` + `:exec.run/2`, `Jason`-based
> NDJSON, `:telemetry.execute/3`, `Mix.Task` for dev tooling, ExUnit
> for tests. Tasks 28-29 cover the §4.5 tool-call boundary (new
> material absorbed from sub-F's amendment, not present in the Rust
> backlog). Task 30 covers the Layer 3 tool-call boundary smoke test
> (split from Task 21 to unblock the core Layer 3 test suite from the
> Task 29 dependency). Tasks 26, 27 pre-dated the rewrite and were
> already Elixir-native. **The `spikes/beam_pty/` corpus is the
> implementation reference** — capture files live there, the parser
> field names are locked against them, and the cmux mitigation flags
> came from it. Implementation begins at Task 1.

## Received

### From ravel-orchestrator (2026-04-16) — Critical path audit

All dispatches absorbed.

1. **Model-configurability dispatch** → promoted to Task 27 on 2026-04-20.
2. **Elixir backlog rewrite gate** → landed as Task 0 on 2026-04-21 (now
   complete); Tasks 1-25 rewritten inline during the same work phase.

## Task Backlog

### Task 1 — `mix.exs` deps + module skeleton `[setup]`
- **Status:** not_started
- **Dependencies:** Task 0
- **Description:** Add `{:erlexec, "~> 2.2"}` to `mix.exs` deps and add
  `:exec` to `extra_applications` so the erlexec OTP app starts with
  the daemon. Confirm `:jason` and `:telemetry` are already present
  (they will be; they are daemon-wide deps). Create the empty Elixir
  module skeleton under `lib/ravel/harness_adapter/` per spec §2.2:

  ```
  lib/ravel/harness_adapter/
  ├── harness_adapter.ex
  ├── event.ex
  ├── supervisor.ex
  ├── claude_code/
  │   ├── adapter.ex
  │   ├── session.ex
  │   ├── stream_json.ex
  │   ├── spawn.ex
  │   ├── input.ex
  │   └── tool_call_boundary.ex
  └── fixture_replay/
      ├── adapter.ex
      ├── session.ex
      └── format.ex
  ```

  Each file contains only a `defmodule` with a `@moduledoc` stub
  (filled in by later tasks). Add `Ravel.HarnessAdapter.Supervisor`
  (a `DynamicSupervisor` skeleton) to the top-level `Ravel.Supervisor`
  children list per spec §2.3. Run `mix compile` to confirm the
  skeleton compiles warning-free. No tests yet.
- **Results:** _pending_

### Task 2 — `harness_adapter.ex` + `event.ex`: §3.1 behaviour + §3.3 typed events `[types]`
- **Status:** not_started
- **Dependencies:** Task 1
- **Description:** Implement the §3.1 behaviour surface and the §3.3
  typed event set verbatim per the spec.

  In `harness_adapter.ex`:
  - `@behaviour Ravel.HarnessAdapter` with callbacks `kind() ::
    harness_kind()` and `spawn(spawn_opts()) :: {:ok, pid()} |
    {:error, term()}`.
  - `@type tool_profile :: :ingestion_minimal | :research_broad`.
  - `@type harness_kind :: :claude_code | :fixture_replay`.
  - `@type spawn_opts :: %{required(:prompt) => String.t(),
    required(:working_dir) => Path.t(), required(:tool_profile) =>
    tool_profile(), required(:session_id) => String.t(),
    optional(:staging_dir) => Path.t(), optional(:router) => pid() |
    atom(), optional(:model) => String.t() | nil}` — the `:model` key
    is reserved here but honoured by Task 27.
  - `@moduledoc` documents the Session GenServer client-API surface
    (§3.2 message set: `send_user_message/2`, `terminate/1`,
    `await_exit/2`, `session_id/1`, `attach_consumer/2`,
    `detach_consumer/2`).

  In `event.ex` (one module per struct, per spec §3.3):
  - `%Ravel.Event.HarnessOutput{}` with `@enforce_keys [:session_id,
    :kind, :text, :at]` plus optional `:meta`. `kind: :stdout |
    :stderr | :tool_use | :internal_message`.
  - `%Ravel.Event.SessionLifecycle{}` with `@enforce_keys
    [:session_id, :transition, :at]`, transition `:ready |
    {:turn_complete, String.t()} | {:exited, term()}`.
  - `%Ravel.Event.SpawnLatencyReport{}` with `@enforce_keys
    [:session_id, :harness_kind, :tool_profile, :spawned_at,
    :first_chunk_at, :init_event_at]`.
  - `%Ravel.Event.SessionExitStatus{}` with `@enforce_keys
    [:session_id, :reason]`; reason `{:clean_exit, integer()} |
    {:terminated, integer() | nil} | :crashed_before_ready |
    {:tool_profile_violation, map()} | {:protocol_error, term()} |
    {:spawn_failed, term()} | {:io_error, term()} |
    {:exec_port_lost, term()}`.
  - `%Ravel.Event.HarnessError{}` with `@enforce_keys [:session_id,
    :error, :at]` plus optional `:dump_ref`.

  Each struct gets `@type t :: %__MODULE__{...}` (dialyzable). Unit
  tests cover: `@enforce_keys` rejection of missing fields; Jason
  encode/decode round-trip for every struct that crosses the
  fixture-format boundary (`HarnessOutput`, `SessionExitStatus`); atom
  preservation across decode via either `Jason.decode!(_, keys:
  :atoms!)` or manual re-atomisation in the format layer (pick one
  discipline and document it in `format.ex` moduledoc when Task 5
  lands).
- **Results:** _pending_

### Task 3 — Day-1 verification of Claude Code CLI surface `[verification]`
- **Status:** not_started
- **Dependencies:** Task 1 (deps), Task 2 (struct types for parser reference)
- **Description:** Resolve spec §10 Q1 and Q2 against the pinned
  `claude` version. Q3 and Q4 are **already resolved by the BEAM PTY
  spike** (Session 10) and this task only needs to lock their
  resolutions into code (see below).

  - **Q1 (empty-tool-list spelling).** Run `claude --print --verbose
    --input-format stream-json --output-format stream-json
    --setting-sources project,local --no-session-persistence
    --disallowed-tools "*" --permission-mode default` and prompt it to
    use the `Read` tool; verify Claude declines. If `--disallowed-tools
    "*"` does not work on the pinned version, fall back to
    `--allowed-tools ""`. Lock the chosen flag into
    `ClaudeCode.Spawn.tool_profile_to_args/1` (Task 4).
  - **Q2 (`--permission-mode` for headless).** Run `--print` sessions
    under each candidate (`acceptEdits`, `bypassPermissions`,
    `dangerouslySkipPermissions`) that attempt to touch a tmpdir file.
    Document observed behaviour and pick the least-surprising mode for
    v1. Lock the choice into the tool-profile helper.
  - **Q3/Q4 (stream-json input/output shape).** Copy the canonical
    samples from `spikes/beam_pty/results/full-run.log` into
    `test/fixtures/harness_adapter/captured_stream_json/` (one sample
    per `type/subtype` pair). These are the regression baseline for
    Task 9's parser. No further verification needed — the spike locked
    the envelope shape.
  - Pin the verified `claude` version in `{{PROJECT}}/README.md` (or a
    new `test/HARNESS_VERSION.md`) and implement a `claude --version`
    pin check invoked at `Ravel.HarnessAdapter.ClaudeCode` adapter
    init (called by Task 17's supervision wiring); mismatches log a
    hard `Logger.warning/1`. Record the observed flag spellings and
    permission-mode decision in `{{PLAN}}/memory.md` under "Verified
    Claude Code CLI surface".
- **Results:** _pending_

### Task 4 — `ClaudeCode.Spawn.tool_profile_to_args/1` helper `[tool-profile]`
- **Status:** not_started
- **Dependencies:** Task 3 (verified flag spellings)
- **Description:** Implement `tool_profile_to_args/1` in
  `lib/ravel/harness_adapter/claude_code/spawn.ex` per spec §6.1:

  ```elixir
  def tool_profile_to_args(:ingestion_minimal),
    do: ["--disallowed-tools", "*", "--permission-mode", "default"]

  def tool_profile_to_args(:research_broad),
    do: ["--permission-mode", "acceptEdits"]
  ```

  (Exact flag spellings substituted from Task 3's resolutions.)

  Unit tests in `test/ravel/harness_adapter/claude_code/spawn_test.exs`
  assert exact argv list contents for each profile (positional match,
  not set-match).
- **Results:** _pending_

### Task 5 — `FixtureRecord` types + `format.ex` + round-trip tests `[fixture-replay]`
- **Status:** not_started
- **Dependencies:** Task 2 (uses `%HarnessOutput{}` and
  `%SessionExitStatus{}`)
- **Description:** Implement the JSON-Lines fixture schema per spec
  §5.2 in `lib/ravel/harness_adapter/fixture_replay/format.ex`.

  Define tagged structs for the four record variants:
  - `%FixtureRecord.Output{chunk: %HarnessOutput{}}` → `{"t": "output",
    "chunk": {...}}`
  - `%FixtureRecord.Delay{ms: non_neg_integer()}` → `{"t": "delay",
    "ms": 250}`
  - `%FixtureRecord.ExpectUserInput{}` → `{"t": "expect_user_input"}`
  - `%FixtureRecord.Exit{status: %SessionExitStatus{}}` → `{"t":
    "exit", "status": {...}}`

  Implement `encode/1`, `decode/1`, `encode_jsonl/1` (list → newline-
  delimited binary), `decode_jsonl/1` (binary → list), plus a
  `from_disk/1` / `to_disk/1` pair per runtime `%Event.*{}` struct
  that crosses the on-disk boundary (`HarnessOutput` and
  `SessionExitStatus`). The `from_disk/1` / `to_disk/1` pair is the
  lockstep seam that catches drift between runtime and on-disk
  representations.

  Unit tests:
  - JSON-Lines round-trip for every `FixtureRecord` variant.
  - `%HarnessOutput{}` runtime ↔ disk round-trip identity.
  - `%SessionExitStatus{}` runtime ↔ disk round-trip identity
    (including every `reason:` tuple variant).
  - Atom preservation across decode (enforce the chosen discipline
    from Task 2's decision).
- **Results:** _pending_

### Task 6 — `FixtureReplay.Session` GenServer + replay loop `[fixture-replay]`
- **Status:** not_started
- **Dependencies:** Task 5
- **Description:** Implement `Ravel.HarnessAdapter.FixtureReplay.Session`
  per spec §5.4. A `GenServer` that walks the fixture record list,
  mirroring `ClaudeCode.Session`'s GenServer shape exactly (same
  client-API message set, same consumer-delivery semantics, same
  `{:harness_event, session_id, event}` info-message shape).

  - State struct: `:session_id, :records, :consumers, :awaiting_user_input?,
    :final_status, :terminating`.
  - `init/1` sends `:step` to self and seeds state.
  - `handle_info(:step, state)` walks the head of `state.records`:
    - `%Output{chunk: _}` → broadcast to consumers, self-send `:step`.
    - `%Delay{ms: _}` → `Process.send_after(self(), :step, ms)`.
      Interruptible by `:terminate` cast.
    - `%ExpectUserInput{}` → block (no self-step; advance waits on
      `send_user_message/2`).
    - `%Exit{status: _}` → broadcast `%SessionLifecycle{transition:
      {:exited, reason}}` + `%SessionExitStatus{}` to consumers,
      `{:stop, :normal, state}`.
    - `[]` (records exhausted with no explicit Exit) → broadcast a
      synthesised clean-exit, stop.
  - `handle_call({:send_user_message, _}, _, state)` when
    `state.awaiting_user_input?` advances past the
    `%ExpectUserInput{}` record and resumes stepping.
  - Shared client-API handlers: `{:attach_consumer, pid}` (with
    `Process.monitor/1` auto-detach on consumer crash),
    `{:detach_consumer, pid}`, `:session_id`, `{:await_exit, timeout}`.
  - `handle_cast(:terminate, state)` — stop with a synthesised
    `{:terminated, nil}` exit status; idempotent.

  No erlexec, no subprocess, no process group. Unit tests drive the
  GenServer with a handcrafted record list and assert consumer
  receives events in order.
- **Results:** _pending_

### Task 7 — `FixtureReplay.Adapter` module `[fixture-replay]`
- **Status:** not_started
- **Dependencies:** Task 6
- **Description:** Implement `Ravel.HarnessAdapter.FixtureReplay`
  per spec §5. The module implements `@behaviour Ravel.HarnessAdapter`:

  - `kind/0` returns `:fixture_replay`.
  - `spawn/1` loads the fixture file from a path stored on the module
    (the file path is passed via `Application.get_env/3` or through
    the `spawn_opts` as an implementation-specific `:fixture_path`
    key; document the choice in the module doc) and calls
    `DynamicSupervisor.start_child(Ravel.HarnessAdapter.Supervisor,
    {FixtureReplay.Session, %{session_id: sid, records: records,
    tool_profile: profile, model: model}})`.
  - The `:prompt` and `:tool_profile` spawn opts are recorded in the
    session struct for verification but otherwise ignored (the fixture
    is the source of truth for what the session emits).

  Unit test: `spawn/1` → attach consumer → drain events → assert all
  fixture chunks arrive in order.
- **Results:** _pending_

### Task 8 — Layer 2 fixture files + ExUnit integration tests `[testing]`
- **Status:** not_started
- **Dependencies:** Task 7
- **Description:** Create the spec §5.3 fixture files under
  `test/fixtures/harness_adapter/`:
  - `replay_clean_linear.jsonl` — output → output → exit
  - `replay_multi_turn.jsonl` — output → expect_user_input → output → exit
  - `replay_terminated.jsonl` — output → delay → (terminated during delay) → exit
  - `replay_tool_violation.jsonl` — assistant `tool_use` chunk under `:ingestion_minimal`
  - `replay_crashed_before_ready.jsonl` — exit with no chunks (shape-only fixture)

  ExUnit integration tests in
  `test/ravel/harness_adapter/fixture_replay_session_test.exs` per
  spec §8.2 matrix:
  - Clean linear drain with `attach_consumer/2`.
  - Multi-turn with user interjection: driver process calls
    `send_user_message/2` during an `expect_user_input` record; assert
    replay continues and post-interjection events arrive.
  - Mid-stream termination: driver casts `:terminate` mid-delay;
    consumer receives a `{:terminated, _}` exit event; `await_exit/2`
    returns the terminated status.
  - Tool-profile violation fixture surfaces
    `%SessionExitStatus{reason: {:tool_profile_violation, _}}` (the
    defence-in-depth layer from Task 12's `handle_info/2` fires on the
    `tool_use` event).
  - `:crashed_before_ready` fixture shape round-trips cleanly
    (live-heuristic coverage is a Task 16 concern).
  - Multi-consumer attach: two consumer pids attach to one session,
    both receive the same events in order, one detaches mid-stream,
    only the remaining consumer receives subsequent events.

  These tests exercise the full GenServer + typed-event delivery
  surface deterministically, without a real `claude` process. They
  are also the surface that sub-B's executor tests and sub-E's
  pipeline tests exercise once those plans land.
- **Results:** _pending_

### Task 9 — `stream_json.ex` parser + event mapping `[claude-code]`
- **Status:** not_started
- **Dependencies:** Task 3 (canonical JSON samples), Task 2 (event
  structs)
- **Description:** Implement `Ravel.HarnessAdapter.ClaudeCode.StreamJson`
  per spec §4.3.1 and §4.3.2.

  - `parse_line(binary)` calls `Jason.decode!/1` and pattern-matches
    on the `"type"` / `"subtype"` fields, returning `{:ok, [event]}` |
    `{:error, reason}`. Decode failures are hard errors
    (`{:protocol_error, _}` per spec goal 2).
  - Dispatch table per spec §4.3.1:
    - `"system"` with `subtype: "init"` → `%SessionLifecycle{transition:
      :ready}`; also records that the init event was observed so the
      caller can snapshot `init_event_at` (the Session GenServer does
      this in `handle_info/2`).
    - `"system"` (other subtype) → `%HarnessOutput{kind:
      :internal_message, meta: %{raw: decoded}}`.
    - `"rate_limit_event"` → `%HarnessOutput{kind: :internal_message,
      meta: %{event: :rate_limit, raw: decoded}}`.
    - `"assistant"` → walk `message.content`; per block:
      - `"thinking"` → `%HarnessOutput{kind: :internal_message, meta:
        %{block: :thinking}}`.
      - `"text"` → `%HarnessOutput{kind: :stdout}` (feeds B's sentinel
        matcher).
      - `"tool_use"` → `%HarnessOutput{kind: :tool_use, text: name,
        meta: %{tool_use_id: id, input: input}}` (Session GenServer
        runs defence-in-depth + tool-call-boundary intercept on this
        event before forwarding).
      - `"tool_result"` → `%HarnessOutput{kind: :internal_message,
        meta: %{block: :tool_result, tool_use_id: id}}`.
    - `"user"` (echo) → `%HarnessOutput{kind: :internal_message, meta:
      %{block: :user_echo}}`.
    - `"result"` → `%SessionLifecycle{transition: {:turn_complete,
      subtype}}`.
    - Unknown `"type"` → `%HarnessOutput{kind: :internal_message,
      meta: %{unknown_type: type, raw: decoded}}` (forward-compat for
      additive schema changes per spec §9 Risk 1).

  Unit tests drive `parse_line/1` with each canonical JSON sample from
  `test/fixtures/harness_adapter/captured_stream_json/` and assert the
  expected event list. Edge cases covered here (rather than in Task 19):
  empty content blocks, mixed text/tool_use blocks, missing optional
  fields, multi-block assistant messages with shared timestamps.
- **Results:** _pending_

### Task 10 — `spawn.ex` argv builder + `:exec.which/1` + erlexec opts `[claude-code]`
- **Status:** not_started
- **Dependencies:** Task 4 (`tool_profile_to_args/1`)
- **Description:** Implement the rest of
  `lib/ravel/harness_adapter/claude_code/spawn.ex` per spec §4.1.1
  and §4.1.2:

  - `build_argv/1` takes `%{session_id: _, tool_profile: _, model:
    model_or_nil}` and returns the argv list. Fixed order (spec §4.1.1
    + Task 27 acceptance criteria):
    ```
    ["claude", "--print", "--verbose",
     "--input-format", "stream-json", "--output-format", "stream-json",
     "--session-id", sid,
     "--setting-sources", "project,local",
     "--no-session-persistence"]
     ++ model_args(model)                       # Task 27: [] | ["--model", model_id]
     ++ tool_profile_to_args(profile)           # Task 4
    ```
    The cmux mitigation flags (`--setting-sources`,
    `--no-session-persistence`) are mandatory on every daemon-spawned
    session — the spike proved they silence ~10 KB of spurious JSON.
  - `find_claude_binary/0` calls `:exec.which(~c"claude")` →
    `{:ok, path}` | `{:error, :claude_not_found}`.
  - `erlexec_opts/1` takes the working directory and returns the
    erlexec option list: `[:monitor, :stdin, {:stdout, self()},
    {:stderr, self()}, :kill_group, {:kill_timeout, 1}, {:cd,
    to_charlist(cwd)}]`. The shape is load-bearing and validated by
    the BEAM PTY spike; deviations break the input path.

  Unit tests assert: exact argv ordering for each
  `(tool_profile, model)` combination; `find_claude_binary/0` returns
  a charlist path; `erlexec_opts/1` returns the exact option list with
  `self()` substituted at call time.
- **Results:** _pending_

### Task 11 — `input.ex` user-message + tool_result envelope encoders `[claude-code]`
- **Status:** not_started
- **Dependencies:** Task 1
- **Description:** Implement `Ravel.HarnessAdapter.ClaudeCode.Input`
  per spec §4.1.3 and §4.5.3.

  - `encode_user_message(text) :: binary()` produces the NDJSON
    envelope `{"type":"user","message":{"role":"user","content":
    [{"type":"text","text":text}]}}\n` via `Jason.encode!/1`.
  - `encode_tool_result(tool_use_id, content, is_error?) :: binary()`
    produces the NDJSON envelope carrying a `user/tool_result` block
    that Claude Code consumes to continue after an injected-tool call.

  Unit tests assert exact JSON shape against captured user-message
  envelopes recorded from the spike (the spike's `full-run.log`
  includes echoed envelopes that are the canonical reference).
- **Results:** _pending_

### Task 12 — `ClaudeCode.Session` GenServer (state + `handle_info/2` core) `[claude-code]`
- **Status:** not_started
- **Dependencies:** Task 9 (stream_json), Task 10 (spawn), Task 11 (input)
- **Description:** Implement the core of
  `lib/ravel/harness_adapter/claude_code/session.ex` per spec §4.2.1
  and §4.2.2.

  - State struct per §4.2.1: `:session_id, :ospid, :tool_profile,
    :staging_dir, :spawned_at, :first_chunk_at, :init_event_at,
    :consumers, :sentinel_buffer_hook, :router, :injected_tools,
    :first_chunk_received, :terminating, :model, :await_exit_waiters`.
  - `init/1` records `:spawned_at`, calls `:exec.run(argv, opts)` via
    Task 10 helpers, stores the returned `ospid`, schedules the
    `:crashed_before_ready_check` timer (`Process.send_after(self(),
    :crashed_before_ready_check, 2_000)` — Task 16), returns `{:ok,
    state}`. Spawn failures translate to
    `{:stop, {:spawn_failed, reason}}`.
  - `handle_info({:stdout, ospid, line}, state)` per §4.2.2: set
    `:first_chunk_at` on first call; parse via
    `StreamJson.parse_line/1`; for each event, run defence-in-depth
    (Task 4's profile logic — §6.2 check), then tool-call-boundary
    intercept (Task 29), then broadcast to consumers via
    `{:harness_event, sid, event}` info messages. Record
    `:init_event_at` when a `%SessionLifecycle{transition: :ready}`
    event fires; trigger the `SpawnLatencyReport` emission once all
    three timestamps are populated (Task 19).
  - `handle_info({:stderr, ospid, line}, state)` — forward as
    `%HarnessOutput{kind: :stderr}`; append to the error-tail ring
    buffer (spec §11.4 — budget in Risk 5).
  - `handle_info({:DOWN, _, :process, _, reason}, state)` — emit
    `%SessionLifecycle{transition: {:exited, reason}}`, synthesise a
    `%SessionExitStatus{}` (cross-checking with the
    crashed-before-ready heuristic — Task 16), notify any
    registered `await_exit` waiters, `{:stop, :normal, state}`.
  - `handle_info({:router_reply, tool_use_id, result}, state)` —
    Task 29's tool-call-boundary reply handler (stubbed in Task 12;
    completed in Task 29).

  Unit tests drive `handle_info/2` directly with synthetic messages
  (no real subprocess), exercising every handler branch. The TestHelper
  injects fake `:exec.run/2` returns via a behaviour-level mock if
  needed (or by parameterising the spawn call; pick whichever is less
  invasive).
- **Results:** _pending_

### Task 13 — `ClaudeCode.Session` GenServer: `handle_call/3` client API `[claude-code]`
- **Status:** not_started
- **Dependencies:** Task 12
- **Description:** Complete the Session GenServer's synchronous client
  API per spec §4.2.2 and §3.2.

  Implement `handle_call/3` clauses:
  - `{:send_user_message, text}` — guard against `state.terminating`,
    call `Input.encode_user_message/1`, `:exec.send(ospid, binary)`,
    reply `:ok` or `{:error, :io_error | :session_terminated}`.
  - `{:attach_consumer, pid}` — add to `state.consumers` (a
    `MapSet.t(pid())`), `Process.monitor/1` the pid so auto-detach on
    consumer crash; reply `:ok`.
  - `{:detach_consumer, pid}` — remove from `state.consumers`,
    de-monitor; reply `:ok`.
  - `:session_id` — reply `{:ok, state.session_id}`.
  - `{:await_exit, timeout}` — register the caller in
    `state.await_exit_waiters`; **do not reply synchronously** — the
    `{:DOWN, ...}` handler (Task 12) sends the reply via
    `GenServer.reply/2` when the session exits.

  Also implement a thin client module `Ravel.HarnessSession` that
  wraps `GenServer.call/3` / `cast/2` calls against a session pid, per
  spec §3.2. This is what B's executor / TUI / E's pipeline import
  and use; they do not call `GenServer.call/3` directly.

  Unit tests against `FixtureReplay.Session` (shared contract) and
  against `ClaudeCode.Session` with mocked `:exec.send/2` calls.
- **Results:** _pending_

### Task 14 — Process-group termination: `:kill_group` + two-phase escalation `[lifecycle]`
- **Status:** not_started
- **Dependencies:** Task 12 (`handle_info/2` core), Task 10 (erlexec
  opts supply `:kill_group`)
- **Description:** Implement the `:terminate` cast + escalation timer
  per spec §4.4.2 in `session.ex`:

  ```elixir
  def handle_cast(:terminate, %{terminating: true} = state),
    do: {:noreply, state}

  def handle_cast(:terminate, state) do
    :ok = :exec.kill(state.ospid, 15)   # SIGTERM to the group
    Process.send_after(self(), :escalate_kill, 500)
    {:noreply, %{state | terminating: true}}
  end

  def handle_info(:escalate_kill, state) do
    case :exec.status(state.ospid) do
      {:status, _} -> :ok = :exec.kill(state.ospid, 9)   # SIGKILL
      _ -> :ok
    end
    {:noreply, state}
  end
  ```

  The `:kill_group` option in erlexec's spawn opts (Task 10) makes
  these signals target the whole process group, killing Claude Code's
  tool subprocesses and MCP server children alongside the main
  process. Idempotency: the `state.terminating` guard makes a second
  `:terminate` cast a no-op; the 500 ms grace matches
  `systemd-stop`'s default.

  Unit test: a `:terminate` cast sets `state.terminating: true` and
  issues the SIGTERM (mocked `:exec.kill/2` assertion). The real
  `pgrep` cleanup test lives in Layer 3 (Task 23).
- **Results:** _pending_

### Task 15 — `CrashedBeforeReady` heuristic `[lifecycle]`
- **Status:** not_started
- **Dependencies:** Task 12 (handle_info core), Task 13 (await_exit)
- **Description:** Implement the crashed-before-ready classification
  per spec §4.6.1:

  - `init/1` schedules `Process.send_after(self(),
    :crashed_before_ready_check, 2_000)`.
  - `handle_info(:crashed_before_ready_check, state)` records a flag
    that the 2-second threshold has elapsed. No state change to exit
    classification here — the classification happens in the `{:DOWN,
    ...}` handler (whichever arrives second).
  - `handle_info({:DOWN, ...}, state)` consults both the threshold
    flag and `state.first_chunk_received`: if DOWN arrives before the
    2 s threshold AND `first_chunk_received == false` AND reason is
    non-zero, classify as `%SessionExitStatus{reason:
    :crashed_before_ready}`. Otherwise classify per the reason tuple
    (`{:clean_exit, _}` | `{:terminated, _}`).

  Catches: missing API key, invalid model name, malformed config, OOM
  at startup, missing dependencies. The 2-second threshold is a module
  attribute `@crashed_before_ready_ms 2_000` for future tuning.

  Unit test drives the handler with both interleavings (DOWN first,
  timer first) and asserts the correct classification.
  `FixtureReplay.Session` does NOT emulate the live heuristic — its
  `replay_crashed_before_ready.jsonl` fixture tests the shape of the
  emitted event only (per Task 8's scenario matrix).
- **Results:** _pending_

### Task 16 — `claude_code/adapter.ex` + `DynamicSupervisor` wiring `[claude-code]`
- **Status:** not_started
- **Dependencies:** Task 13 (Session handle_call), Task 14 (terminate),
  Task 15 (crashed-before-ready)
- **Description:** Implement `Ravel.HarnessAdapter.ClaudeCode` per
  spec §4 and §2.3:

  - `@behaviour Ravel.HarnessAdapter`.
  - `kind/0 :: :claude_code`.
  - `spawn/1` calls `DynamicSupervisor.start_child(Ravel.HarnessAdapter.Supervisor,
    {Ravel.HarnessAdapter.ClaudeCode.Session, spawn_opts})`; returns
    `{:ok, pid}` | `{:error, reason}`. The Session GenServer's
    `init/1` does the actual `:exec.run/2` call, so adapter-level code
    stays thin.
  - Expose an adapter-level `start_link/1` lifecycle (for `Ravel.Application`
    to fold in the pin check): at application boot,
    `ClaudeCode.Adapter.check_claude_version/0` runs `:exec.run/2`
    with `claude --version`, parses the output, and
    `Logger.warning/1`s on mismatch against the pinned version (spec
    §9 Risk 1).

  Wire `Ravel.HarnessAdapter.Supervisor` (a `DynamicSupervisor`) into
  the top-level `Ravel.Supervisor` children list per §2.3. Session
  GenServers are started with `restart: :temporary` — a crashed
  session is not auto-restarted; the owning `PlanActor` decides
  whether to re-spawn via `Process.monitor/1` observation.

  Unit tests for the adapter surface are integration-flavoured and
  live in Layer 3 (Task 23).
- **Results:** _pending_

### Task 17 — `%SpawnLatencyReport{}` emission (tactical three-path) `[instrumentation]`
- **Status:** not_started
- **Dependencies:** Task 16 (adapter)
- **Description:** Implement the always-on latency instrumentation
  per spec §7.2. Record timestamps in the Session GenServer state
  (`:spawned_at` set in `init/1`; `:first_chunk_at` set on first
  `{:stdout, _, _}`; `:init_event_at` set on the `system/init` event
  detected by the parser). When all three are populated, emit the
  report via three parallel paths (this is the "tactical seed"
  described in §11.4 — sub-M's Task 25 later replaces the auxiliary
  paths):

  1. **Consumer event**: broadcast `%HarnessOutput{kind:
     :internal_message, meta: %{report: :spawn_latency, data:
     %SpawnLatencyReport{}}}` via the standard `{:harness_event, sid,
     event}` delivery. Emitted immediately after the first real
     `%HarnessOutput{kind: :stdout}` event.
  2. **Telemetry execute**: `:telemetry.execute([:ravel, :harness,
     :claude_code, :spawn_latency], %{cold_spawn_ms: _,
     first_chunk_ms: _, first_output_ms: _}, %{session_id: _,
     tool_profile: _, model: _})`.
  3. **Staging file**: `File.write!(Path.join(state.staging_dir,
     "spawn-latency.json"), Jason.encode!(report, pretty: true))`.

  Unit tests drive a `FixtureReplay.Session` with a time-controlled
  fixture (fixtures can carry synthetic timestamps via the on-disk
  `%HarnessOutput{at: _}` field) and assert that: (a) the consumer
  receives the internal_message report; (b) a telemetry handler
  attached via `:telemetry.attach/4` receives the event; (c) the
  staging file is written with the expected contents.

  (Sub-M's Task 25 later deletes paths 1 and 3 after the verification
  window closes. The `%SpawnLatencyReport{}` struct and path 2 survive
  as the canonical shape.)
- **Results:** _pending_

### Task 18 — `mix ravel.dev.record_fixture` Mix task `[dev-tools]`
- **Status:** not_started
- **Dependencies:** Task 16 (adapter), Task 5 (fixture format)
- **Description:** Implement the dev-only fixture-recording Mix task
  per spec §5.5 at `lib/mix/tasks/ravel.dev.record_fixture.ex`:

  ```
  mix ravel.dev.record_fixture --output <path>
                                [--prompt <prompt>]
                                [--profile research_broad|ingestion_minimal]
                                [--max-delay-ms <ms>]
                                [--interactive]
  ```

  Implementation: spawn `Ravel.HarnessAdapter.ClaudeCode` against the
  user's `claude` binary, attach the Mix task process as the session
  consumer, capture every `%HarnessOutput{}` event into a
  `%FixtureRecord.Output{}` record, measure wall-clock gaps between
  events into `%FixtureRecord.Delay{ms: gap_ms}` records (capped at
  `--max-delay-ms`, default 500), terminate with the captured
  `%SessionExitStatus{}` as a `%FixtureRecord.Exit{}`. Writes JSON
  Lines to `--output`.

  `--interactive` mode: mirrors live output to the user's terminal,
  reads stdin lines via `IO.gets/1`, forwards them via
  `Ravel.HarnessSession.send_user_message/2`, records each as a
  `%FixtureRecord.ExpectUserInput{}` record (the user text is not
  stored; only the shape). Recording terminates when the live session
  exits or the user types `:done`.

  Mix task `@moduledoc` is tagged "dev-only — not for end users" and
  the `ravel.dev.*` namespace is documented in the daemon's
  `DEVELOPMENT.md` (or equivalent) as the home for future dev tools
  (e.g., `mix ravel.dev.replay_fixture`).

  Smoke test: record a trivial session, read back the output file via
  `FixtureReplay.Format.decode_jsonl/1`, assert the sequence parses
  cleanly and roundtrips through `encode_jsonl/1`.
- **Results:** _pending_

### Task 19 — Layer 1 unit tests catch-all `[testing]`
- **Status:** not_started
- **Dependencies:** Task 9 (parser), Task 5 (format), plus any later
  tasks that leave uncovered edges
- **Description:** Catch-all for spec §8.1 unit coverage not already
  written by per-feature tasks. Particularly:
  - `StreamJson.parse_line/1` edge cases not covered by Task 9:
    malformed JSON (hard error via `Jason.decode!/1`), truncated
    events, unknown content block types, extremely large single
    events, multi-megabyte content fields.
  - `FixtureRecord` schema drift cases: an on-disk JSON object with an
    unknown `"t"` variant, missing required fields, extra fields
    (forward-compat), atoms vs strings for `kind:` / `reason:`.
  - Struct `@enforce_keys` invariant tests for every
    `%Ravel.Event.*{}` struct (smoke-check that `@enforce_keys`
    produces a compile error if fields are missing — not a runtime
    test).

  Run with `mix test`. No live tag.
- **Results:** _pending_

### Task 20 — Layer 2 multi-consumer + concurrent stress test `[testing]`
- **Status:** not_started
- **Dependencies:** Task 13 (Session client API), Task 7
  (`FixtureReplay.Adapter`)
- **Description:** Per spec §8.2 last two rows. Two programmatic tests
  in `test/ravel/harness_adapter/`:

  - **Multi-consumer attach** (no fixture file needed; uses a
    programmatic record list): two consumer pids attach to one
    `FixtureReplay.Session`; assert both receive the same events in
    order; one detaches mid-stream; assert only the remaining
    consumer receives subsequent events; the detached consumer is
    auto-re-attached and the test asserts it resumes from the current
    position (no replay).
  - **Concurrent stress / deadlock canary**: programmatic fixture of
    10 000 events with `expect_user_input` records interjected every
    100 events; two driver `Task`s hammer the session concurrently —
    one calls `send_user_message/2` in a loop, the other drains
    consumer events. Assert: no deadlocks (the test completes within
    a generous `assert_receive`/`Task.await` timeout), no process
    crashes, all events delivered in order, all user-message calls
    processed.

  This is the regression test for the GenServer's message-ordering
  correctness — BEAM mailboxes are FIFO but a subtle
  `handle_info/handle_call` ordering bug could still deliver events
  out of order to consumers. Tests must fail loudly on any such bug.
- **Results:** _pending_

### Task 21 — Layer 3 ExUnit integration tests against real `claude` (core scenarios) `[testing]`
- **Status:** not_started
- **Dependencies:** Task 16 (adapter), Task 17 (latency
  instrumentation), Task 14 (termination)
- **Description:** Implement spec §8.3 Layer 3 tests under
  `test/ravel/harness_adapter/live/`, all tagged `@moduletag :live`.
  Run with `mix test --only live`; NOT run in normal CI.

  - **Smoke test** — spawn + first chunk: a `"say hi"` session emits
    at least one `%HarnessOutput{kind: :stdout}` event.
  - **Tool profile enforcement (`:ingestion_minimal`)** — a prompt
    asking for `Read` tool use returns either zero `:tool_use` events
    or terminates with
    `%SessionExitStatus{reason: {:tool_profile_violation, _}}`.
  - **Tool profile enforcement (`:research_broad`)** — a prompt
    asking for file reading produces one or more `:tool_use` events.
  - **Multi-turn live** — spawn → first turn → `send_user_message/2`
    → second turn → assert second turn references first-turn content.
  - **Cold-spawn latency baseline** — 10 spawn cycles, record the
    emitted `%SpawnLatencyReport{}`, assert p95 < 10 s (the *test*
    gate; the C-1 *acceptance* gate is 5 s, checked separately by
    Task 22).
  - **Process group cleanup** — spawn, immediately `terminate/1`,
    assert `System.cmd("pgrep", ["-f", "claude.*" <> sid])` returns
    `{_, 1}` (no matches) within 1 second.
  - **CrashedBeforeReady** — set a bogus API key in env, spawn,
    assert the consumer receives
    `%SessionExitStatus{reason: :crashed_before_ready}` within 5 s.
  - **cmux noise mitigation** — spawn with and without
    `--setting-sources project,local`; assert the with-flag run emits
    a clean NDJSON stream (first event is `system/init`) while the
    without-flag run contains ~10 KB of SessionStart hook JSON before
    the first assistant event. (This test may be gated by whether the
    running user has cmux hooks configured; skip with a clear
    `@tag :requires_cmux_hooks` if not.)

  Layer 3 is the canary that catches Claude Code version drift —
  schema changes and flag renames fail here first. The tool-call
  boundary smoke test is in Task 30 (split out to unblock this suite
  from the Task 29 dependency).
- **Results:** _pending_

### Task 22 — C-1 dogfood acceptance gate (the swap-in moment for B's stub) `[acceptance]`
- **Status:** not_started
- **Dependencies:** Task 21 (Layer 3 core green), Task 30 (tool-call
  boundary smoke test green), and B's executor far enough along to
  accept the swap (coordinated with sub-B's plan)
- **Description:** The v1 acceptance gate for sub-project C and the
  joint v1 acceptance gate with sub-project B. Per spec §7.3:

  - Swap B's `LlmHarnessExecutor` stub adapter for the real
    `Ravel.HarnessAdapter.ClaudeCode`.
  - Run the Ravel orchestrator plan's full four-phase cycle (work →
    reflect → compact → triage), **N≥10 times**, against the real
    adapter on the user's primary dev machine.
  - Walk every staging directory created during those cycles and parse
    every `<staging>/spawn-latency.json` file.
  - Compute cold-spawn latency p50/p95/p99 across all sessions in all
    cycles.
  - **Decision**: if p95 < 5 s, C-1 PASSES. Warm-pool work is deferred
    to v1.5. Document the decision in
    `{{PROJECT}}/LLM_STATE/ravel-orchestrator/memory.md` with
    measured p50/p95/p99 and the date.
  - **Decision (alternate)**: if p95 ≥ 5 s, C-1 FAILS. Unblock Task 25
    (warm-pool reset spike) and Task 26 (warm-pool implementation),
    fold them into v1. Document the failure with measurements in the
    orchestrator parent's memory and proceed to Task 25.

  This task is the swap-in moment: the real
  `Ravel.HarnessAdapter.ClaudeCode` becomes B's adapter of record.
  Until this passes, B keeps using `FixtureReplay` for end-to-end
  testing.
- **Results:** _pending_

### Task 23 — (CONDITIONAL) Warm-pool reset mechanism spike `[spike]`
- **Status:** not_started
- **Dependencies:** Task 22 = FAIL
- **Description:** Conditional on Task 22 tripping the C-1 gate.
  Execute the spec §7.4 three-check protocol against the pinned
  `claude` version under an ExUnit harness running in a tmpdir (no
  Ravel code changes required):

  - **Check 1 — Structured stream-json reset envelope.** Inspect
    `claude --help` and recorded session output for a control envelope
    (`{"type":"control","action":"reset"}` or similar). If found,
    write a ~30-line ExUnit test that spawns `claude`, sends a user
    message, sends the reset envelope, sends a second user message,
    and verifies the second message is processed in fresh context.
  - **Check 2 — `/clear` injected as user-message text.** If Check 1
    turns up nothing, send `/clear` as the text content of a
    user-message envelope and verify the same fresh-context property.
  - **Check 3 — Pre-spawned single-use degradation.** If neither
    Check 1 nor Check 2 works, fall back to the single-use pool model.

  Output: a markdown report at
  `test/fixtures/sub_c_warm_pool_spike/results/spike-report.md`
  documenting which check passed, the exact reset mechanism used,
  recorded session output verifying fresh-context behaviour, and a
  recommendation for Task 26. ~½ day estimated.
- **Results:** _pending_

### Task 24 — (CONDITIONAL) v1 warm-pool implementation `[claude-code]`
- **Status:** not_started
- **Dependencies:** Task 23
- **Description:** Conditional on Task 22 tripping and Task 23
  returning a usable reset mechanism. Implement
  `Ravel.HarnessAdapter.ClaudeCode.WarmPool` (a GenServer) per spec
  §7.5 sketch using the spike-validated reset path.

  - Per-profile pools: `:ingestion_minimal` depth 3-4;
    `:research_broad` depth 1-2; both configurable via daemon config
    under `[harnesses.claude_code.warm_pool]`.
  - Background spawner process maintains target depth by starting
    fresh `ClaudeCode.Session` GenServers and parking them in a
    ready-queue.
  - `WarmPool.spawn/1` pulls from the queue of the matching profile,
    sends the spike-validated reset envelope (or `/clear`) via
    `:exec.send/2`, sends the actual prompt as the next user-message
    envelope, returns the GenServer pid. Falls back to
    `ClaudeCode.spawn/1` if the pool is empty.

  Re-run Task 22's dogfood cycles to verify p95 now passes. Update
  `{{PLAN}}/memory.md` with the new measurements.
- **Results:** _pending_

### Task 25 — Adopt sub-M observability framework (parallel-emit of `SpawnLatencyReport`) `[m-adoption]`
- **Status:** not_started
- **Dependencies:** sub-M Task 12 (`ObservabilityHarness`), sub-M
  Task 9 (`MetricsRecorderLayer` / telemetry handler attachment), this
  plan's Task 17 (where `%SpawnLatencyReport{}` is currently emitted).
- **Description:** Landed by sub-project M's brainstorm (2026-04-13,
  Session 7 of the ravel-orchestrator plan). M's design doc at
  `{{PROJECT}}/docs/superpowers/specs/2026-04-13-sub-M-observability-design.md`
  §9 specifies the staged migration of C's `%SpawnLatencyReport{}`
  onto M's `:telemetry`-based metrics framework via a parallel-emit
  window. This task is C's share.

  **Phase 1 — parallel emit (this task lands).** Add `:telemetry.span/3`
  wrappers around the Session GenServer's hot-path handlers
  (`handle_info({:stdout, _, _}, _)`, `handle_call({:send_user_message,
  _}, _, _)`, `handle_cast(:terminate, _)`). Span name `:harness_session`,
  measurement-map keys `:session_id, :tool_profile, :harness_kind`.

  Add parallel `:telemetry.execute/3` counter/gauge calls alongside
  C's existing three-path `%SpawnLatencyReport{}` emission (Task 17):

  - `:telemetry.execute([:ravel, :harness, :spawned], %{count: 1},
    %{session_id: _, tool_profile: _, harness_kind: _})` at successful
    spawn.
  - `:telemetry.execute([:ravel, :harness, :exited_clean], %{count:
    1}, _)` at clean exit.
  - `:telemetry.execute([:ravel, :harness, :exited_error], %{count:
    1}, %{reason: _})` at error exit.
  - `:telemetry.execute([:ravel, :harness, :live_sessions], %{count:
    _current_}, _)` on spawn / terminate.

  The three histogram events already emitted by Task 17 (cold_spawn,
  first_chunk, first_output) continue to fire — this task adds the
  counter/gauge companions.

  Wire the sub-M dump-on-error path: every branch in
  `ClaudeCode.Session` that emits a non-clean `%SessionExitStatus{}`
  or `%HarnessError{}` calls
  `Ravel.Observability.dump_event_tail(session_id, 1000)` before the
  GenServer stops. The dump path is defined by sub-M's Task 13.

  **DO NOT delete Task 17's existing three-path emission** in this
  task. Both paths run in parallel during the verification window.
  Sub-M Task 15's verification test confirms the metric values match
  the staging JSON within ±10 ms. Only after ≥10 consecutive CI
  builds pass the verification does the v1.1 cleanup task delete
  Task 17's consumer-event + staging-file paths (created
  automatically by sub-M's first triage after the verification window
  closes).

  **Phase 2 — cleanup (separate future task).** After the
  verification window closes, a new task lands in this backlog from
  sub-M's triage that:
  1. Deletes the `%HarnessOutput{kind: :internal_message, meta:
     %{report: :spawn_latency}}` delivery path.
  2. Deletes the `<staging>/spawn-latency.json` writer.
  3. Updates spec §7.2 to reflect the telemetry-only emission
     surface.

  Sub-G's migration plan owns the staging schema cleanup (removing
  `<staging>/spawn-latency.json` from the documented schema).

  TDD: write tests that drive a `FixtureReplay.Session` cycle and
  assert the expected telemetry events fire (attach a test handler
  via `:telemetry.attach/4`). Layer 3 integration test exists in
  sub-M Task 15; this task only needs unit tests on C's side.
- **Results:** _pending_

## Sub-R adoption

### Task 26 — Absorb Sub-R: add `ontology.query` to tool profiles `[r-adoption]`
- **Status:** not_started
- **Dependencies:** Sub-R engine landed
- **Description:** Per Sub-R design doc §5.2. Register `ontology.query`
  as a tool in the Claude Code adapter's tool profile for plan phase
  sessions and expert consultation sessions. Internal daemon sessions
  (fact extraction, routing) do NOT get the tool — they are too
  cheap/fast for tool-call overhead.

  Tool spec: input is a Datalog query string, output is a JSON list of
  variable bindings. The adapter routes the tool call to
  `Ravel.Datalog.Server.query/1` via the tool-call boundary (Task 29).
  Tool profile gating per spec §4.5 — the `tool_profile` mechanism
  already supports per-session tool sets.
- **Results:** _pending_

## Model configurability (absorbed from Sub-O v1 scope)

### Task 27 — Per-actor model selection: `model:` → `--model <model-id>` `[model-config]`
- **Status:** not_started
- **Dependencies:** Task 0 (Elixir backlog rewrite — **done 2026-04-21**),
  Task 4 (`tool_profile_to_args/1` / spawn argv builder lands first),
  Task 7 (`FixtureReplay.Adapter` — fixture-replay parity test surface
  exists)
- **Description:** Absorbs the v1 slice of sub-O (mixture of models)
  into sub-C's single-adapter surface. Plan actors and expert actors
  carry a `model:` field in their actor declaration (sub-F §F-12
  reserved the schema hook; orchestrator memory commits v1 to
  honouring it). This task wires that field end-to-end through the
  Claude Code adapter so cost-sensitive callers can pin Opus for plan
  phases and Haiku for ingestion / extraction / fact-distillation.

  **Concretely:**
  1. Extend `Ravel.HarnessAdapter.spawn_opts()` (spec §3.1) with an
     `optional(:model) => String.t() | nil` field. Absent / `nil` =
     pass no `--model` flag and let Claude Code pick its own default.
     Document the field in the behaviour's `@moduledoc` as a pass-through
     model-id string — the adapter does not validate the value against
     a whitelist (forward compatibility with new Claude Code releases).
  2. Update `Ravel.HarnessAdapter.ClaudeCode.Spawn.build_argv/1` (the
     argv builder in `spawn.ex`, spec §4.1.1) to append
     `["--model", model_id]` to the argv when `spawn_opts.model` is a
     non-empty string. Append **after** the mandatory flags
     (`--print --verbose --input-format --output-format --session-id
     --setting-sources --no-session-persistence`) and **before** the
     profile-derived flags from §6, so the final argv shape stays
     stable and grep-friendly.
  3. Store the resolved model-id (or `nil`) on the session GenServer
     state struct (spec §4.2.1) as `:model` for inclusion in
     `%Ravel.Event.SpawnLatencyReport{}` metadata — downstream cost
     analysis (and the eventual sub-O cost telemetry) needs to know
     which model each recorded session actually ran on. Add `:model`
     to the report's `@enforce_keys` only if the adapter guarantees
     it; otherwise add as an optional field with `nil` default.
  4. `FixtureReplay.Adapter.spawn/1` accepts and records the `:model`
     field but otherwise ignores it (the fixture is the source of
     truth for session output). Include the recorded value in any
     session-id-correlated test assertions so fixture-replay stays a
     faithful mirror of the live adapter's public surface.
  5. Sub-F's actor-declaration parser (its own task, not sub-C's)
     surfaces the `model:` field in the `spawn_opts` map passed to
     `Ravel.HarnessAdapter.ClaudeCode.spawn/1`. Sub-C does not parse
     actor-declaration files; it only honours whatever `spawn_opts`
     provides.

- **Acceptance criteria:**
  - `spawn_opts` type accepts `optional(:model) => String.t() | nil`;
    typespec dialyzes cleanly.
  - Unit test in `spawn_test.exs`: `build_argv/1` with
    `spawn_opts.model = "claude-opus-4-5"` produces an argv whose
    ordering is `["claude", "--print", "--verbose",
    "--input-format", "stream-json", "--output-format", "stream-json",
    "--session-id", "<id>", "--setting-sources", "project,local",
    "--no-session-persistence", "--model", "claude-opus-4-5",
    ...profile-flags...]` — exact positional match.
  - Unit test: `build_argv/1` with `spawn_opts.model = nil` (and with
    the key absent entirely) produces an argv that does **not**
    contain `"--model"` anywhere.
  - Unit test: `build_argv/1` with `spawn_opts.model = ""` is treated
    as absent (no `--model` flag emitted). Justification: an empty
    string model-id is almost certainly a config bug; silently
    passing `--model ""` to `claude` would produce an opaque error.
    Emit a `Logger.warning/1` in this branch.
  - Fixture-replay adapter test: spawn a `FixtureReplay.Adapter` with
    `:model = "claude-haiku-4-5"` against a clean linear fixture and
    assert that (a) the session drains cleanly, (b) the recorded
    `%SpawnLatencyReport{}` (if emitted by the fixture) carries the
    model-id through to consumers, (c) no `--model` CLI flag is
    synthesised (fixture-replay does not spawn a real process).
  - Fixture-replay adapter test: spawn with `:model` absent, assert
    session drains cleanly and reported `model` field is `nil`.
  - Layer 3 live test (`@moduletag :live`, per Task 21 pattern):
    spawn a real `ClaudeCode` adapter with `:model = "claude-haiku-4-5"`,
    capture the first `system/init` event, and assert the `model`
    field in the event's JSON matches the requested model-id. This is
    the end-to-end proof that the flag actually reaches `claude` and
    is honoured.
  - Layer 3 live test: spawn with `:model = "bogus-model-that-does-not-exist"`
    and assert the session either fails `:crashed_before_ready` or
    surfaces `%HarnessError{}` / `%SessionExitStatus{reason: {:spawn_failed,
    _}}` with the underlying claude error reason within 5 s. The
    adapter does **not** pre-validate model-ids (forward compat);
    claude's own error path is the validator of record.
  - Spec cross-reference: add a forward-reference from §3.1 of the
    spec to this task's `spawn_opts` extension, so future readers
    trace the field's provenance.

- **Out of scope:**
  - Per-adapter model routing (sub-O v2 owns multi-adapter dispatch).
  - Cost telemetry / token accounting (sub-O v2).
  - Default-model policy enforcement (sub-F / orchestrator config
    concern; sub-C only honours whatever `spawn_opts` provides).
  - Model-id validation against a known list (explicit non-goal —
    pass-through).

- **Rationale:** Per orchestrator memory, v1 ships per-actor model
  selection within the single Claude Code adapter. This is essential
  for cost control (Opus for plan phases where reasoning quality
  dominates, Haiku for ingestion / fact-extraction / routing where
  latency and cost dominate). Without this task the `model:` field
  exists in actor declarations (reserved by sub-F) but nothing
  downstream honours it, which would make the reserved-field
  commitment a lie. Absorbing the v1 slice into sub-C keeps sub-O's
  brainstorm scope clean (multi-adapter, local models, cost telemetry)
  without blocking it on v1.

- **Results:** _pending_

## Tool-call boundary (from sub-F amendment — new work absorbed into sub-C)

### Task 28 — Tool-call boundary injection mechanism spike (§10 Q6) `[spike]`
- **Status:** not_started
- **Dependencies:** Task 3 (pinned `claude` version established)
- **Description:** Spec §10 Q6 is a day-1 open question: how are
  Ravel-injected tools (`ask_expert`, `dispatch_to_plan`,
  `read_vault_catalog`) exposed to a running `claude` session?
  Three candidates per spec §4.5.2:

  - **(a) MCP server on a Unix socket**, pointed at by `--mcp-config`.
    Preferred. Claude Code already speaks MCP; an MCP server running
    inside the daemon over a Unix socket under `<vault>/runtime/`
    exposes the Ravel tools as ordinary MCP tools; the adapter
    intercepts the `tool_use` events via stream-json parsing.
  - **(b) Tool-definition preamble on stdin**, inlined into the first
    user-message envelope. Depends on Claude Code accepting inline
    tool definitions via stream-json; not yet verified.
  - **(c) Plugin shim via `--allowed-tools` whitelist**. Uses Claude
    Code's plugin layer; most fragile, may shift version-to-version.

  Run a focused ExUnit spike against the pinned `claude` version:
  - Stand up a minimal MCP server in Elixir (or reuse an existing one
    from the daemon's dep graph if one lands first) that exposes a
    single `read_vault_catalog` tool returning a hardcoded JSON
    string.
  - Spawn `claude` with `--mcp-config` pointing at the socket.
  - Prompt: "use the read_vault_catalog tool to list my plans".
  - Verify the `tool_use` event arrives in the stream-json output
    with `name: "read_vault_catalog"` and the server receives the
    call.
  - If (a) fails, attempt (b); if (b) fails, attempt (c).

  Output: a markdown report at
  `test/fixtures/sub_c_tool_call_boundary_spike/results/spike-report.md`
  documenting which mechanism passed, the exact wiring used, and any
  version-sensitivity caveats. Updates spec §4.5.2 and §10 Q6 in a
  follow-up commit — a spec-doc-level update, not a code change.

  Must land before Task 29. ~½ day estimated.
- **Results:** _pending_

### Task 29 — `tool_call_boundary.ex` intercept + `{:router_reply, _, _}` wiring `[claude-code]`
- **Status:** not_started
- **Dependencies:** Task 28 (injection mechanism resolved), Task 12
  (Session `handle_info/2` core), Task 11 (`Input.encode_tool_result/3`)
- **Description:** Implement the tool-call boundary per spec §4.5.3
  in `lib/ravel/harness_adapter/claude_code/tool_call_boundary.ex` +
  wiring in `claude_code/session.ex`.

  - `state.injected_tools :: MapSet.t(String.t())` is populated at
    `init/1` time from the `spawn_opts.router` + the adapter's fixed
    v1 tool set (`"ask_expert"`, `"dispatch_to_plan"`,
    `"read_vault_catalog"`). Injected tools are **always allowed**
    regardless of `tool_profile`, per §6.1.
  - When `StreamJson.parse_line/1` yields a `%HarnessOutput{kind:
    :tool_use, text: name, meta: %{tool_use_id: id, input: input}}`
    event and `MapSet.member?(state.injected_tools, name)`:
    1. **Do not** forward the event to consumers.
    2. Async-call
       `Ravel.Router.handle_tool_call(name, input, state.session_id,
       reply_ref: id)` in a supervised task. The router resolves the
       target (expert / plan) and runs the query/dispatch without
       blocking the Session GenServer.
    3. The router replies with `{:router_reply, id, result}` info
       message (guaranteed by sub-F's contract — §11.3).
  - `handle_info({:router_reply, tool_use_id, result}, state)` in
    `session.ex`:
    1. Encode the result via `Input.encode_tool_result(tool_use_id,
       result, is_error?: false)` (or `true` if the router returned
       an error tuple).
    2. `:exec.send(state.ospid, envelope)` to push it to the child.
    3. Claude Code consumes the `tool_result` and continues its turn.
  - Error paths: router timeout / crash → the supervised task's
    `{:DOWN, _, _, _, _}` observation translates to an error
    `tool_result` envelope sent to the child; the session continues
    (the harness can retry or recover in its own logic).

  Unit tests drive the Session GenServer's `handle_info/2` with
  synthetic `:stdout` lines containing injected-tool `tool_use` blocks
  + synthetic `{:router_reply, _, _}` messages; assert the intercept
  suppresses the consumer event, assert the router is called with the
  expected args (via a stub router pid), assert the `tool_result`
  envelope is encoded and sent.

  Integration test (Task 30): the Layer 3 tool-call boundary smoke
  test exercises the full intercept → router stub → `tool_result` →
  harness-continues loop.
- **Results:** _pending_

### Task 30 — Layer 3 tool-call boundary smoke test `[testing]`
- **Status:** not_started
- **Dependencies:** Task 28 (injection mechanism resolved), Task 29
  (boundary intercept wiring), Task 21 (core Layer 3 tests green)
- **Description:** The tool-call boundary live test extracted from
  Task 21 to unblock the core Layer 3 test suite from the Task 29
  dependency. A single `@moduletag :live` scenario in
  `test/ravel/harness_adapter/live/tool_call_boundary_test.exs`:

  - **Tool-call boundary smoke test** — stand up a stub router pid
    that acts as `state.router`; inject the `read_vault_catalog` tool
    via the Session GenServer's `injected_tools` set; prompt "list the
    plans in my vault"; assert the session intercepts the `tool_use`
    event (does **not** forward it to consumers), calls the stub router
    with `{:tool_call, "read_vault_catalog", input, reply_ref: id}`,
    the stub sends `{:router_reply, id, result}` back, the session
    encodes a `tool_result` envelope and `:exec.send/2`s it to the
    child, and Claude Code consumes the result and continues its turn
    (a subsequent `:stdout` chunk arrives).

  Tagged `@moduletag :live`; run with `mix test --only live`. This
  test is a hard dependency of the C-1 dogfood gate (Task 22) because
  real orchestrator cycles use injected tools.
- **Results:** _pending_
