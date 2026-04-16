# Sub-H — Skill Fold (User-Facing Actions for the Daemon Architecture)

**Status:** design complete (brainstorm Session 21, 2026-04-16)

**Owning plan:** `LLM_STATE/sub-H-skills/`

**Parent plan:** `LLM_STATE/mnemosyne-orchestrator/`

**Primary dependencies:** sub-F (NDJSON protocol, Router.Server, plan hierarchy invariants, vault catalog), sub-C (harness tool injection), sub-N (ExpertActor query/dialogue protocol, Stage 5 absorb contract), sub-E (ingestion pipeline, `submit_candidate/2` entry point), sub-A (adopt-project as library function), sub-M (sealed event set for new events), sub-B (phase execution via PlanActor), sub-G (Phase 1 switch-over is the clean-break moment).

---

## §1 Scope and motivation

Sub-H defines the user-facing action surface for the Mnemosyne daemon. Every action a user or LLM can perform — running phases, querying experts, creating plans, submitting knowledge for curation — flows through the client-daemon NDJSON protocol. Sub-H specifies:

- The complete command registry derived from the daemon architecture
- The TUI command palette with modal which-key support and user-configurable keybindings
- The `mnemosyne cmd` CLI subcommand for external automation
- NDJSON protocol additions (`create_plan`, `adopt_project`)
- Harness-injected tools for LLM-driven actions
- Cross-sub-project requirements on siblings
- Co-equal-actors discipline: every action has both an LLM-driven and a human-driven form

### Architectural derivation, not legacy mapping

The command set is derived from what the daemon architecture naturally requires. That the resulting actions cover ground 7 legacy Claude Code skills previously occupied is confirmation that the architecture is complete, not the design's motivation. Three legacy concepts disappear entirely:

- **`promote-global` disappears.** In the old model, "promote" meant "copy from Tier 1 to Tier 2." In the daemon architecture, Tier 2 is expert-owned — experts absorb candidates through their curation verdict. Whether the candidate originated from Tier 1 or a session artifact is a property of the input, not a different operation.
- **`explore-knowledge` disappears as a distinct concept.** Querying an expert IS knowledge exploration — the expert's retrieval pipeline + persona is the exploration mechanism.
- **`curate-global` merges with `promote-global`** into a single "submit candidate for expert curation" action.

### In scope for v1

- Command registry with metadata (ID, name, description ≤80 chars, help URL, scope, default keybinding)
- Command palette: fuzzy-filterable, scope-aware (plan-scoped commands hidden when no plan attached)
- Keybinding system: single keys, modifiers (`C-`, `S-`, `M-`), chord sequences for modal/which-key, hot-reloaded from config
- `mnemosyne cmd` CLI wrapper for external automation via the NDJSON socket
- Two new NDJSON commands: `create_plan`, `adopt_project`
- One new harness-injected tool: `create_plan` (expert tools already specified by sub-N)
- Local documentation server hosted by the daemon
- Cross-sub-project adoption stubs

### Out of scope for v1 (explicit)

- **TUI widget implementation.** Sub-H specifies command behavior and palette interaction design. The actual `ratatui` rendering, widget layout, and input handling belong to TUI implementation tasks under sub-F.
- **Harness plugin.** No Mnemosyne-provided harness plugin ships. The harness is a pure worker with no user-facing command surface.
- **Interim compatibility.** The 7 legacy Claude Code skills stop existing at G's Phase 1 switch-over. No deprecation warnings, no shim layer, no bridge.
- **`mnemosyne cmd` implementation.** Sub-H specifies the interface. Implementation is a TUI-side task (Rust, `tokio`, `serde_json`, Unix socket).

---

## §2 Command registry

### §2.1 Command definition

Every daemon action is registered as a command with canonical metadata:

```elixir
defmodule Mnemosyne.Command do
  @enforce_keys [:id, :name, :description, :help_path, :scope]
  defstruct [
    :id,                  # "expert.query" — canonical string ID
    :name,                # "Query Expert" — human-readable
    :description,         # ≤80 chars, shown in palette
    :help_path,           # "/docs/commands/expert-query" — relative to docs server
    :scope,               # :vault | :plan
    :default_keybinding   # "e" | "C-e" | "p w" | nil
  ]
end
```

- `id`: dot-namespaced canonical identifier. Used in keybinding config, `mnemosyne cmd`, and event telemetry.
- `description`: ≤80 characters, shown inline in the command palette. Hard error at registration if exceeded.
- `help_path`: relative path served by the daemon's local documentation server. Selecting "Help" on a palette entry opens this URL in the system browser.
- `scope`: `:plan` commands require an attached plan context. `:vault` commands are available regardless.
- `default_keybinding`: the shipped default. Users override in config.

### §2.2 V1 command set

Derived from the daemon architecture's natural action surfaces:

| Command ID | Name | Scope | Default binding | Motivation |
|---|---|---|---|---|
| `phase.work` | Run Work Phase | plan | `p w` | PlanActor phase cycle |
| `phase.reflect` | Run Reflect Phase | plan | `p r` | PlanActor phase cycle |
| `phase.compact` | Run Compact Phase | plan | `p c` | PlanActor phase cycle |
| `phase.triage` | Run Triage Phase | plan | `p t` | PlanActor phase cycle |
| `plan.create` | Create Plan | plan | `n` | F's plan hierarchy |
| `plan.attach` | Attach to Plan | vault | `a` | Multi-actor TUI navigation |
| `plan.detach` | Detach from Plan | vault | `d` | Multi-actor TUI navigation |
| `expert.query` | Query Expert | vault | `e` | ExpertActor consultation |
| `expert.create` | Create Expert | vault | `E` | Expert lifecycle (`$EDITOR`) |
| `expert.edit` | Edit Expert | vault | — | Expert lifecycle (`$EDITOR`) |
| `knowledge.submit` | Submit for Curation | vault | `s` | Sub-E Stage 5 manual trigger |
| `vault.adopt` | Adopt Project | vault | — | Sub-A vault bootstrap |
| `vault.list` | List Actors | vault | `l` | Vault enumeration |
| `vault.status` | Actor Status | vault | — | Actor state inspection |
| `vault.rescan` | Rescan Vault | vault | `R` | Vault re-enumeration |
| `daemon.shutdown` | Shutdown Daemon | vault | `C-S-q` | Daemon lifecycle |
| `palette.open` | Command Palette | — | `:` | TUI meta-command |

The `palette.open` binding is mandatory — removing it from the config is a hard error. All other bindings may be removed or rebound.

### §2.3 Legacy skill mapping (consequence, not cause)

For traceability, the mapping from legacy skills to architecture-derived commands:

| Legacy skill | Maps to | Why |
|---|---|---|
| `begin-work` | `phase.work` | Phase execution |
| `reflect` | `phase.reflect` | Phase execution |
| `setup-knowledge` | `vault.adopt` | Project adoption |
| `create-plan` | `plan.create` | Plan hierarchy |
| `explore-knowledge` | `expert.query` | Expert consultation is knowledge exploration |
| `curate-global` | `knowledge.submit` | Manual Stage 5 trigger |
| `promote-global` | `knowledge.submit` | Same operation, different source file origin |

---

## §3 Command palette and keybinding system

### §3.1 Command palette behavior

The palette is a fuzzy-filterable overlay showing all available commands. Behavior:

- Opened by the `palette.open` keybinding (default `:`)
- Shows command name, description, and current keybinding for each entry
- Fuzzy matching on both name and ID (typing "phase" shows all `phase.*` commands)
- Plan-scoped commands appear greyed out with "(requires attached plan)" when no plan is attached — visible for discoverability, not invocable
- Selecting a command executes it; some commands prompt for additional input (e.g., `plan.create` prompts for name and description)
- `Esc` dismisses the palette

### §3.2 Keybinding configuration

User-configurable keybindings live at `<vault>/tui-keybindings.toml`:

```toml
[keybindings]
# Single keys
"palette.open" = ":"
"plan.create"  = "n"

# Modifier keys: C- (Ctrl), S- (Shift), M- (Alt/Meta)
"daemon.shutdown" = "C-S-q"
"expert.create"   = "C-S-e"
"vault.rescan"    = "C-M-r"

# Chord sequences (modal / which-key)
"phase.work"    = "p w"
"phase.reflect" = "p r"
"phase.compact" = "p c"
"phase.triage"  = "p t"
```

**Modifier syntax:** `C-` (Ctrl), `S-` (Shift), `M-` (Alt/Meta), composable in any combination (e.g., `C-S-e` for Ctrl+Shift+e). Follows terminal convention. `ratatui`'s `crossterm` backend supports all modifiers natively via `KeyEvent` + `KeyModifiers`.

**Chord sequences:** Space-separated key sequences for modal interaction. `p w` means: press `p`, then press `w`. When the user presses a prefix key that has continuations, the TUI shows a which-key popup listing all available continuations with their command names and descriptions. The popup has a configurable timeout (default 1 second) after which the prefix is cancelled and the TUI returns to normal mode.

**Validation rules:**

- Conflicting bindings (two commands bound to the same key or sequence) are a hard error — the TUI refuses to start
- `palette.open` must always have a binding — removing it is a hard error
- Unknown command IDs in the config are warnings (logged), not errors — forward compatibility for user configs across daemon upgrades
- The TUI hot-reloads keybinding config on file change via `notify`/file-watcher, same pattern as `DeclarationLoader`

### §3.3 Which-key modal support

When a keypress matches a chord prefix (a key that has continuations), the TUI displays a which-key overlay:

```
┌─── p: Phase ──────────────────────┐
│  w  Run Work Phase                │
│  r  Run Reflect Phase             │
│  c  Run Compact Phase             │
│  t  Run Triage Phase              │
└───────────────────────────────────┘
```

- The overlay appears after the prefix key is pressed
- Shows all available continuations with command names
- Timeout (default 1s, configurable in `daemon.toml` under `[tui] which_key_timeout_ms`) dismisses the overlay and cancels the prefix
- Pressing a valid continuation executes the command
- Pressing an invalid key dismisses the overlay and cancels the prefix
- Nesting is supported: chord sequences can be arbitrarily deep (e.g., `p w` is depth 2; `g p w` would be depth 3)

### §3.4 Default keybinding layout

The shipped defaults use a modal which-key layout for grouped commands and single keys for frequent standalone actions:

```toml
[keybindings]
# Meta
"palette.open"    = ":"

# Phase commands (grouped under p)
"phase.work"      = "p w"
"phase.reflect"   = "p r"
"phase.compact"   = "p c"
"phase.triage"    = "p t"

# Plan commands
"plan.create"     = "n"
"plan.attach"     = "a"
"plan.detach"     = "d"

# Expert commands
"expert.query"    = "e"
"expert.create"   = "E"

# Knowledge
"knowledge.submit" = "s"

# Vault
"vault.list"      = "l"
"vault.rescan"    = "R"

# Daemon
"daemon.shutdown" = "C-S-q"
```

Commands without default bindings (`expert.edit`, `vault.adopt`, `vault.status`) are palette-only by default. Users can bind them if desired.

---

## §4 Detailed action specifications

Every action is specified with both its LLM-driven form (how the harness LLM triggers it during a phase session) and its human-driven form (how a human triggers it via TUI or `mnemosyne cmd`), per sub-E's co-equal-actors requirement.

### §4.1 Phase execution (`phase.work`, `phase.reflect`, `phase.compact`, `phase.triage`)

Triggers a phase on the attached PlanActor. Uses F's existing `run_phase` NDJSON command.

| | LLM-driven | Human-driven |
|---|---|---|
| **Trigger** | B's phase prompts + sentinel detection drive transitions; B's executor sends `run_phase` | TUI palette or keybinding (e.g., `p w`) |
| **NDJSON** | `{"cmd": "run_phase", "actor_id": "...", "phase": "work", "request_id": "..."}` | Same |
| **Scope** | Plan-scoped | Plan-scoped |
| **Output** | Harness output streams to attached clients via `harness_output` events | Rendered in TUI output pane |

No new protocol surface. No new work for sub-H beyond command registry entries.

### §4.2 Project adoption (`vault.adopt`)

Creates `<project>/mnemosyne/` with `project-root/` and `knowledge/`, symlinks into the vault. Sub-A's existing command exposed over the NDJSON socket.

| | LLM-driven | Human-driven |
|---|---|---|
| **Trigger** | Rare — adoption is almost always human-initiated. Phase prompts do not instruct adoption. | TUI palette: "Adopt Project" → inline prompt for project path |
| **NDJSON** | `{"cmd": "adopt_project", "project_path": "/absolute/path", "request_id": "..."}` | Same |
| **Scope** | Vault-scoped | Vault-scoped |
| **Validation** | A's existing checks: not already adopted, valid directory, no conflicting `mnemosyne/` layout | Same |
| **Side effects** | Creates symlink at `<vault>/projects/<name>`, regenerates vault catalog | Same |

New NDJSON command (§5.2). Requires sub-A to expose `adopt_project` as a library-callable function.

### §4.3 Expert consultation (`expert.query`)

Sends a query to an ExpertActor. Uses F's existing `send` command with a query message, backed by sub-N's `ask_expert` / `reply_to_expert` dialogue protocol (sub-N §5.1).

| | LLM-driven | Human-driven |
|---|---|---|
| **Trigger** | Phase prompt instructs: "use `ask_expert` tool to consult domain experts." Sub-C injects the tool. | TUI palette: "Query Expert" → fuzzy-select expert ID from known experts → type question |
| **NDJSON** | `{"cmd": "send", "actor_id": "...", "message": {"type": "query", "target-expert": "rust-expert", "question": "..."}, "request_id": "..."}` | Same |
| **Scope** | Vault-scoped | Vault-scoped |
| **Multi-turn** | LLM handles `reply_to_expert` automatically per sub-N §5.1 prompt instructions | TUI shows clarifying question, user types reply, TUI sends next turn via `reply_to_expert` |
| **Response** | Tool result returned inline to the harness session | Answer rendered in TUI output pane with citations |

No new protocol surface. Sub-N's tools and dialogue protocol handle everything.

### §4.4 Knowledge curation submission (`knowledge.submit`)

Submits a knowledge candidate for expert curation — sub-E's Stage 5 flow triggered manually. The candidate (a knowledge file or session artifact) is dispatched to tag-matching experts for absorb/reject/cross-link verdicts.

| | LLM-driven | Human-driven |
|---|---|---|
| **Trigger** | Not LLM-triggered. Sub-E's automated pipeline runs Stage 5 at phase boundaries. | TUI palette: "Submit for Curation" → file picker to select candidate |
| **NDJSON** | `{"cmd": "send", "actor_id": "...", "message": {"type": "dispatch", "target-expert": "auto", "body": {"kind": "absorb_candidate", "source_path": "..."}}, "request_id": "..."}` | Same |
| **Scope** | Vault-scoped | Vault-scoped |
| **`target-expert: "auto"`** | "Run scope matching and fan out to all tag-matching experts." The daemon's dispatch processor handles multi-expert orchestration. | Same |
| **Flow** | Sub-N's `ScopeMatcher` finds matching experts → parallel fan-out of `%ExpertAbsorbCandidate{}` → experts return verdicts → files written to Tier 2 directories | Same, with verdict summary rendered in TUI |
| **Orphan path** | If no experts match, candidate writes to `<vault>/knowledge/uncategorized/` with `%Ingestion.OrphanCandidate{}` event | Same |

This is the unified action that replaces both `curate-global` and `promote-global`. Whether the source is Tier 1 (`<project>/mnemosyne/knowledge/`), a session artifact, or a manually written file is a property of the `source_path`, not a different action.

### §4.5 Plan creation (`plan.create`)

Creates a child plan under the attached plan. The only action requiring a new NDJSON command.

| | LLM-driven | Human-driven |
|---|---|---|
| **Trigger** | Phase prompt instructs: "if the task requires a child plan, use the `create_plan` tool." Sub-C injects the tool. | TUI palette: "Create Plan" → inline prompts for name (kebab-case) and description (≤120 chars) |
| **NDJSON** | `{"cmd": "create_plan", "parent_plan": "<qualified-id>", "name": "sub-x-topic", "description": "...", "request_id": "..."}` | Same |
| **Scope** | Plan-scoped | Plan-scoped |
| **Daemon behavior** | See §5.1 | Same |

### §4.6 Expert lifecycle (`expert.create`, `expert.edit`)

Experts are user-authored filesystem artifacts. The TUI opens `$EDITOR`; sub-N's `DeclarationLoader` file watcher handles registration automatically.

| | LLM-driven | Human-driven |
|---|---|---|
| **Trigger** | Not LLM-triggered. Expert authoring is a human activity. | TUI palette: "Create Expert" → prompt for expert ID → opens `$EDITOR` with template at `<vault>/experts/<id>.md` |
| **Mechanism** | — | TUI writes a template file, opens `$EDITOR`. On save, `DeclarationLoader` detects the new/changed file and hot-reloads. |
| **Scope** | — | Vault-scoped |
| **Template** | — | Sub-N's default declaration template with placeholder persona, empty scope globs, and the §3.2 frontmatter schema |

No NDJSON command. No daemon involvement beyond the existing `DeclarationLoader` hot-reload path (sub-N §2.1). Expert deletion is handled by the user deleting the file — `DeclarationLoader` detects the removal and unregisters the expert.

### §4.7 Vault enumeration and status (`vault.list`, `vault.status`, `vault.rescan`)

Use F's existing NDJSON commands. No new protocol surface.

- `vault.list`: `{"cmd": "list", "filter": {...}}` — returns all plans and experts with qualified IDs, descriptions, and states
- `vault.status`: `{"cmd": "status", "actor_id": "..."}` — returns detailed state for one actor
- `vault.rescan`: `{"cmd": "rescan"}` — re-enumerates vault, picks up new/removed plans and symlinks

### §4.8 Plan attachment (`plan.attach`, `plan.detach`)

Use F's existing `attach` / `detach` NDJSON commands. The TUI shows a plan picker (fuzzy-filtered from `vault.list` results) for `plan.attach`.

### §4.9 Daemon lifecycle (`daemon.shutdown`)

Uses F's existing `shutdown` NDJSON command. Default keybinding is `C-S-q` (Ctrl+Shift+q) — a modifier chord to prevent accidental invocation.

---

## §5 NDJSON protocol additions

Sub-H adds exactly two new commands to F's §7.2 protocol.

### §5.1 `create_plan`

```json
{"cmd": "create_plan", "parent_plan": "<qualified-id>", "name": "sub-x-topic", "description": "≤120 chars", "request_id": "r-1"}
```

Success response:

```json
{"type": "response", "request_id": "r-1", "body": {"result": "ok", "qualified_id": "Mnemosyne/project-root/sub-x-topic"}}
```

Error responses:

```json
{"type": "error", "code": "invalid_name", "message": "Name must be kebab-case", "request_id": "r-1"}
{"type": "error", "code": "reserved_name", "message": "Name 'project-root' is reserved", "request_id": "r-1"}
{"type": "error", "code": "duplicate_name", "message": "Plan 'sub-x-topic' already exists under parent", "request_id": "r-1"}
{"type": "error", "code": "description_too_long", "message": "Description exceeds 120 characters", "request_id": "r-1"}
{"type": "error", "code": "unknown_actor", "message": "Parent plan not found", "request_id": "r-1"}
```

**Daemon behavior on success:**

1. Validate `name`: kebab-case, not reserved (`project-root`, `knowledge`), not duplicate under parent
2. Validate `description`: non-empty, non-placeholder, ≤120 characters (F §1.5 invariant 7)
3. Create directory `<parent>/<name>/`
4. Write `plan-state.md`:
   ```yaml
   ---
   phase: work
   description: "<description>"
   ---
   ```
5. Create empty `backlog.md` (with `# Backlog` heading)
6. Create empty `memory.md` (with `# Memory` heading)
7. Create empty `session-log.md` (with `# Session Log` heading)
8. Regenerate vault catalog (`plan-catalog.md`)
9. Register the new PlanActor with `ActorSupervisor`
10. Emit `%Plan.Created{}` event (sub-M)

All file writes go through `Mnemosyne.FileIO.write_safe/3` with category `:exclusive` (daemon-authoritative, no conflict check needed for new files).

### §5.2 `adopt_project`

```json
{"cmd": "adopt_project", "project_path": "/absolute/path/to/project", "request_id": "r-2"}
```

Success response:

```json
{"type": "response", "request_id": "r-2", "body": {"result": "ok", "project_name": "MyProject", "root_plan": "MyProject/project-root"}}
```

Error responses:

```json
{"type": "error", "code": "already_adopted", "message": "Project is already adopted", "request_id": "r-2"}
{"type": "error", "code": "invalid_path", "message": "Path does not exist or is not a directory", "request_id": "r-2"}
{"type": "error", "code": "layout_conflict", "message": "Existing mnemosyne/ directory has incompatible layout", "request_id": "r-2"}
```

Delegates to sub-A's `Mnemosyne.Vault.adopt_project/2`, then triggers a rescan to pick up the new project's plans.

---

## §6 Harness-injected tools

Sub-C's tool injection mechanism (sub-C §4.5) injects the following tools into every harness session. Sub-H adds one new tool; the others are already specified by sub-N.

| Tool | Specified by | Purpose |
|---|---|---|
| `ask_expert(target_expert, question, dialogue_id?)` | Sub-N §5.1 | Expert consultation |
| `reply_to_expert(dialogue_id, reply)` | Sub-N §5.1 | Multi-turn expert dialogue |
| `create_plan(name, description)` | **Sub-H** (this spec) | Child plan creation |

### §6.1 `create_plan` tool specification

Injected into harness sessions. The LLM can create child plans during any phase session.

```json
{
  "name": "create_plan",
  "description": "Create a child plan under the current plan for decomposing work",
  "parameters": {
    "name": {"type": "string", "description": "Kebab-case directory name (e.g., 'sub-x-topic')"},
    "description": {"type": "string", "description": "Plan description, ≤120 characters"}
  }
}
```

The tool invocation is intercepted by sub-C's adapter, which translates it to a `create_plan` NDJSON command with `parent_plan` set to the session's owning PlanActor's qualified ID. The tool result is the success or error response from the daemon.

**Tool profile:** `create_plan` is available in `work` and `triage` phases. Excluded from `reflect` (lossy pruning, not structural changes) and `compact` (lossless rewrite, no side effects).

---

## §7 `mnemosyne cmd` — CLI wrapper for external automation

A thin Rust binary (or subcommand of the existing `mnemosyne` binary) that connects to the daemon's NDJSON socket, sends one command, prints the response, and exits.

### §7.1 Interface

```bash
# Execute a command
mnemosyne cmd phase.work --actor Mnemosyne/project-root
mnemosyne cmd expert.query --expert rust-expert --question "Does Pin<&mut T> implement Unpin?"
mnemosyne cmd plan.create --parent Mnemosyne/project-root --name sub-x-topic --description "..."
mnemosyne cmd vault.adopt --project-path /path/to/project
mnemosyne cmd vault.list
mnemosyne cmd vault.rescan
mnemosyne cmd daemon.shutdown

# Discovery
mnemosyne cmd --list          # print all commands with IDs, names, descriptions, keybindings
mnemosyne cmd --help <id>     # print help for a specific command
```

### §7.2 Behavior

1. Resolve vault path via the standard discovery chain (flag → env → config → hard error)
2. Connect to `<vault>/runtime/daemon.sock`
3. Translate CLI arguments into one NDJSON command line
4. Send the command, read one response line
5. Print the response body to stdout as JSON
6. Exit with code 0 (success), 1 (daemon error), 2 (connection failure)

No state, no TUI rendering, no interactive prompts. The `mnemosyne cmd` subcommand is a pure NDJSON client.

### §7.3 External automation use case

Any process that can invoke `mnemosyne cmd` can drive the daemon: shell scripts, cron jobs, keyboard macro systems (Hammerspoon, skhd, Karabiner), IDE extensions, CI pipelines. The NDJSON socket is equally accessible directly for tools that prefer raw socket communication.

The `--list` flag queries the daemon's command registry and prints a table for discovery:

```
ID                  NAME                 SCOPE  BINDING  DESCRIPTION
phase.work          Run Work Phase       plan   p w      Run the work phase on the attached plan
phase.reflect       Run Reflect Phase    plan   p r      Run the reflect phase on the attached plan
expert.query        Query Expert         vault  e        Ask a domain expert a question
plan.create         Create Plan          plan   n        Create a child plan under the attached plan
...
```

---

## §8 Local documentation server

The daemon hosts a minimal HTTP server serving static documentation:

- **Source:** `priv/docs/` in the Elixir application (shipped with the daemon, not user-editable)
- **Endpoint:** `http://localhost:<port>/docs/...` where port is configured in `daemon.toml` under `[docs] port` (default auto-assigned, written to `<vault>/runtime/docs-port`)
- **Content:** one HTML page per command (at the `help_path` specified in the command definition), plus an index page listing all commands
- **No API, no dynamic content.** Pure static file serving.
- **Dependency:** `bandit` (lightweight Elixir HTTP server) or `plug_cowboy` — one small dependency added to the daemon's supervision tree

The TUI's palette exposes a "Help" action on each command entry that opens the corresponding URL in the system browser via `open` (macOS) / `xdg-open` (Linux).

---

## §9 Cross-sub-project requirements

### Sub-B (phase cycle)

No changes required. `run_phase` already exists in F's protocol. B's `PhaseRunner` is unchanged. The command palette entries for phase commands are purely sub-H + TUI concerns.

### Sub-C (harness adapters)

- **Inject `create_plan` tool** into harness sessions alongside `ask_expert` / `reply_to_expert`. Same mechanism (sub-C §4.5 tool injection), one additional tool definition.
- **Tool profile configuration:** `create_plan` available in `work` and `triage` phases; excluded from `reflect` and `compact`.
- The `create_plan` tool invocation is intercepted by the adapter, which translates it to a `create_plan` NDJSON command with `parent_plan` inferred from the session's owning PlanActor.

### Sub-E (ingestion)

- **Expose `Mnemosyne.Ingestion.submit_candidate/2`** as a library-callable function. Accepts a file path and an options map; runs scope matching → expert fan-out → verdict collection → file writes. The automated pipeline calls this internally at phase boundaries. The daemon's `knowledge.submit` command handler calls it on behalf of TUI/CLI clients.
- The function returns `{:ok, %{verdicts: [...], orphan: bool}}` or `{:error, reason}`.

### Sub-F (hierarchy)

- **Add `create_plan` command** to the NDJSON command dispatcher in `Router.Server`. Handler: directory creation, file scaffolding, invariant validation (F §1.5), catalog regeneration, actor registration.
- **Add `adopt_project` command** to the NDJSON command dispatcher. Handler: delegates to sub-A's `Mnemosyne.Vault.adopt_project/2`, then triggers rescan.
- **Command registry module:** `Mnemosyne.CommandRegistry` — a GenServer or compile-time registry that holds all `%Command{}` definitions. Queried by TUI for palette rendering, by `mnemosyne cmd --list` for discovery, and by the docs server for the index page.

### Sub-M (observability)

Two new events added to the sealed set:

```elixir
defmodule Mnemosyne.Event.Plan.Created do
  @enforce_keys [:qualified_id, :parent_plan, :name, :description, :created_at]
  defstruct @enforce_keys
end

defmodule Mnemosyne.Event.Command.Executed do
  @enforce_keys [:command_id, :source, :executed_at]
  defstruct [
    :command_id,     # "expert.query", "phase.work", etc.
    :actor_id,       # qualified ID if plan-scoped, nil if vault-scoped
    :source,         # :tui | :cli | :external
    :executed_at
  ]
end
```

`Command.Executed` provides the instrumentation called for by the sub-H amendment's `%Diagnostic{target: "mnemosyne.tui_action"}` requirement. The `source` field distinguishes TUI palette/keybinding invocations from `mnemosyne cmd` CLI invocations from raw NDJSON socket clients.

### Sub-N (domain experts)

No changes required. Expert query and Stage 5 absorb protocols are already specified in sub-N §5 and §6. Sub-H consumes them.

Expert lifecycle (create/edit/delete) uses the filesystem + `DeclarationLoader` hot-reload path already specified in sub-N §2.1. No new daemon commands for expert CRUD.

### Sub-A (global store)

- **`adopt_project` as library function.** If A's current implementation is structured as a CLI entry point, it must be refactored to expose `Mnemosyne.Vault.adopt_project/2` as a callable function that the daemon's NDJSON handler can invoke. The function accepts `(vault_state, project_path)` and returns `{:ok, project_name}` or `{:error, reason}`.

### Sub-G (migration)

- **Phase 1 switch-over documentation.** G's migration docs must note that Phase 1 is the clean-break moment for the 7 legacy Claude Code skills. No bridge, no shim, no deprecation period. The daemon + TUI + `mnemosyne cmd` are the only interfaces post-switch-over.

---

## §10 What sub-H does NOT produce

- **No harness plugin.** No Mnemosyne-provided Claude Code plugin, skill, or harness-side artifact ships. The harness is a pure worker with no user-facing command surface.
- **No TUI widget implementation.** Sub-H specifies the command registry, palette behavior, keybinding system, and which-key support. Actual `ratatui` rendering, widget layout, event loop integration, and input handling are TUI implementation tasks.
- **No new message types.** All actions use existing F dispatch/query messages or the two new NDJSON commands.
- **No interim compatibility.** The 7 legacy skills stop existing at G's Phase 1. No deprecation warnings, no shim.
- **No `mnemosyne cmd` implementation.** Sub-H specifies the interface. Implementation is Rust-side (TUI crate or standalone binary).

---

## Appendix A — Decision trail

### Q1. New NDJSON commands vs. `send` with typed payloads for expert actions?

**Decision:** Reuse `send` with typed payloads. The whole point of F's architecture is that everything flows through dispatch and query — adding parallel command paths undermines uniformity. Orchestration complexity lives in the daemon's `DispatchProcessor`, not at the protocol layer. Only `create_plan` and `adopt_project` are genuinely new commands because they don't map to dispatch or query semantics.

### Q2. TUI surface area — command palette, keybindings, or combination?

**Decision:** Command palette with user-configurable keybindings for every command. Every command is a first-class palette entry; keybindings are a configuration layer on top. No two-tier system. All bindings configurable in `<vault>/tui-keybindings.toml`, hot-reloaded on change. Full modifier support (`C-`, `S-`, `M-`) and chord sequences for modal/which-key interaction.

### Q3. Legacy transition — clean break or interim bridge?

**Decision:** Clean break at G's Phase 1 switch-over. No shim, no bridge, no deprecation period. The harness is explicitly "no slash commands" — building a compatibility layer contradicts that commitment. G already requires the daemon + TUI to be operational before Phase 1 triggers. Zero real users of the legacy skills means no migration burden.

### Q4. Dynamic expert creation — new NDJSON command or filesystem-based?

**Decision:** Filesystem-based via `$EDITOR`. The TUI's "Create Expert" command writes a template file and opens `$EDITOR`. Sub-N's `DeclarationLoader` file watcher picks up the new file automatically. No new daemon command — the daemon learns about experts through its existing hot-reload path. Co-equal-actors: the Obsidian path is already "create a file" — the TUI path is the same operation from a different surface.

### Q5. Plan-scoped vs. vault-scoped action distinction?

**Decision:** Plan-scoped commands (phase execution, plan creation, dispatch) require an attached plan. Vault-scoped commands (list, rescan, expert query, expert CRUD, knowledge submission, adoption, shutdown) are available regardless. The palette shows plan-scoped commands greyed out when no plan is attached — visible for discoverability, not invocable.

### Q6. `create-plan` interaction flow?

**Decision:** Minimal inline TUI prompts for directory name and description (≤120 chars). Opening `$EDITOR` for a 120-char description is overkill. The daemon handles directory creation, file scaffolding, invariant validation, and catalog regeneration.

### Q7. External automation mechanism?

**Decision:** The NDJSON socket is the universal automation interface. `mnemosyne cmd` is a thin CLI wrapper (connect, send, print, exit) that makes the socket scriptable from shell, keyboard macro systems, or any tool that can invoke a subprocess. The daemon doesn't care whether commands come from the TUI, Obsidian, `mnemosyne cmd`, or raw `socat`.

### Q8. Architecture-first vs. legacy-mapping derivation?

**Decision:** The command set is derived from what the daemon architecture naturally requires, not from mapping legacy skills. Three legacy concepts disappear entirely: `promote-global` (Tier 2 is expert-owned; promotion is curation with a different source), `explore-knowledge` (expert query IS knowledge exploration), and the curate/promote distinction (unified into `knowledge.submit`). The legacy mapping table exists for traceability, not as the design's motivation.

### Q9. Command metadata — description and help?

**Decision:** Every command carries a short description (≤80 chars, shown in palette) and a `help_path` URL pointing to local documentation served by the daemon's HTTP server. The daemon serves static HTML from `priv/docs/`. The TUI opens help URLs in the system browser.

### Q10. Keybinding modifier and chord support?

**Decision:** Full modifier support (`C-` Ctrl, `S-` Shift, `M-` Alt/Meta, composable). Chord sequences (space-separated, e.g., `p w`) for modal/which-key interaction. Shipped defaults use chord grouping for related commands (phases under `p`) and single keys for frequent standalone actions. Users can reconfigure entirely.
