# Backlog — Sub-H Skills (User-Facing Actions)

Tasks for implementing the daemon's user-facing action surface: command
registry, TUI command palette, keybinding system, NDJSON protocol
additions, harness tool injection, `mnemosyne cmd` CLI wrapper, and local
docs server. Design doc:
`docs/superpowers/specs/2026-04-16-sub-H-skills-design.md`.

## Task 0 — Readiness gate
- **Status:** not_started
- **Dependencies:** none
- **Description:** Before starting implementation, verify that consumed
  upstream interfaces match the design doc's assumptions. Check:
  - Sub-F's `Router.Server` NDJSON dispatcher — does the command dispatch
    mechanism support adding new commands via a module-based registry?
  - Sub-C's tool injection mechanism (§4.5) — is the tool definition
    format concrete enough to add `create_plan`?
  - Sub-N's `ask_expert` / `reply_to_expert` tool definitions — do they
    match the shapes assumed in §4.3?
  - Sub-E's ingestion pipeline — does a `submit_candidate/2` entry
    point exist or need to be created?
  - Sub-A's `adopt_project` — is it callable as a library function or
    only as a CLI entry point?
  If any interface has drifted since the design doc, update §5/§6/§9 of
  the design doc inline (rewrite, not supersede layer) before proceeding.
- **Results:** _pending_

## Daemon-side (Elixir)

### Task 1: Implement `Mnemosyne.CommandRegistry`
- **Status:** not_started
- **Dependencies:** Task 0
- **Description:** Implement the command registry module per §2.1. A
  GenServer or compile-time registry holding all `%Command{}` definitions
  from §2.2. Provides `list_commands/0`, `get_command/1`,
  `commands_for_scope/1`. Used by the TUI for palette rendering, by
  `mnemosyne cmd --list` for discovery, and by the docs server for the
  index page. All 17 v1 commands registered with IDs, names, descriptions
  (≤80 chars), help paths, scopes, and default keybindings.
- **Results:** _pending_

### Task 2: Implement `create_plan` NDJSON command handler
- **Status:** not_started
- **Dependencies:** Task 0, sub-F's `Router.Server` dispatcher
- **Description:** Add the `create_plan` command to F's NDJSON command
  dispatcher per §5.1. Handler: validate name (kebab-case, not reserved,
  not duplicate), validate description (non-empty, ≤120 chars), create
  directory, write `plan-state.md` + empty `backlog.md` / `memory.md` /
  `session-log.md`, regenerate vault catalog, register PlanActor with
  `ActorSupervisor`, emit `%Plan.Created{}` event. All file writes via
  `Mnemosyne.FileIO.write_safe/3` with `:exclusive` category. Error
  responses for invalid name, reserved name, duplicate, description too
  long, unknown parent.
- **Results:** _pending_

### Task 3: Implement `adopt_project` NDJSON command handler
- **Status:** not_started
- **Dependencies:** Task 0, sub-A's `adopt_project` as library function
- **Description:** Add the `adopt_project` command to F's NDJSON command
  dispatcher per §5.2. Handler: delegates to sub-A's
  `Mnemosyne.Vault.adopt_project/2`, then triggers rescan. Error
  responses for already adopted, invalid path, layout conflict. If sub-A's
  implementation is CLI-only, create an adoption stub task on sub-A's
  backlog for the library-callable refactor.
- **Results:** _pending_

### Task 4: Implement `knowledge.submit` command handler
- **Status:** not_started
- **Dependencies:** Task 0, sub-E's `submit_candidate/2`
- **Description:** Wire the `knowledge.submit` command to sub-E's
  `Mnemosyne.Ingestion.submit_candidate/2`. The `send` command with
  `target-expert: "auto"` and `kind: "absorb_candidate"` payload triggers
  scope matching → expert fan-out → verdict collection. Returns verdict
  summary. If sub-E's entry point doesn't exist yet, create an adoption
  stub task on sub-E's backlog.
- **Results:** _pending_

### Task 5: Add `create_plan` harness tool definition
- **Status:** not_started
- **Dependencies:** Task 2, sub-C's tool injection mechanism
- **Description:** Define the `create_plan` tool for sub-C's harness tool
  injection per §6.1. Tool available in `work` and `triage` phases;
  excluded from `reflect` and `compact`. Sub-C intercepts the invocation
  and translates to a `create_plan` NDJSON command with `parent_plan`
  inferred from the session's owning PlanActor. Create adoption stub on
  sub-C's backlog if needed.
- **Results:** _pending_

### Task 6: Define sub-M event structs
- **Status:** not_started
- **Dependencies:** Task 0
- **Description:** Add `%Mnemosyne.Event.Plan.Created{}` and
  `%Mnemosyne.Event.Command.Executed{}` to sub-M's sealed event set per
  §9. `Plan.Created` fields: `qualified_id`, `parent_plan`, `name`,
  `description`, `created_at`. `Command.Executed` fields: `command_id`,
  `actor_id` (optional), `source` (`:tui | :cli | :external`),
  `executed_at`. Create adoption stub on sub-M's backlog.
- **Results:** _pending_

### Task 7: Implement local docs server
- **Status:** not_started
- **Dependencies:** Task 1
- **Description:** Minimal HTTP server per §8. Add `bandit` dependency.
  Serve static files from `priv/docs/`. Port configured in `daemon.toml`
  under `[docs] port`; auto-assigned port written to
  `<vault>/runtime/docs-port`. Supervisor child under the daemon's main
  supervision tree. Index page auto-generated from `CommandRegistry`.
- **Results:** _pending_

### Task 8: Write command documentation pages
- **Status:** not_started
- **Dependencies:** Task 1, Task 7
- **Description:** Write one HTML or Markdown page per command in
  `priv/docs/commands/`. Each page covers: purpose, NDJSON format,
  examples, related commands. Plus an index page listing all commands.
  These are the targets of the `help_path` field in each `%Command{}`.
- **Results:** _pending_

## TUI-side (Rust)

### Task 9: Implement command palette widget
- **Status:** not_started
- **Dependencies:** Task 1 (command metadata served via NDJSON)
- **Description:** `ratatui` fuzzy-filterable overlay per §3.1. Queries
  the daemon's `CommandRegistry` via a `list_commands` request on attach.
  Shows command name, description, and current keybinding for each entry.
  Fuzzy matching on name and ID. Plan-scoped commands greyed out when no
  plan attached. `Esc` dismisses. Selecting a command dispatches to the
  appropriate NDJSON command with any required inline prompts (e.g.,
  `plan.create` prompts for name and description).
- **Results:** _pending_

### Task 10: Implement keybinding system
- **Status:** not_started
- **Dependencies:** Task 9
- **Description:** Parse `<vault>/tui-keybindings.toml` per §3.2. Support
  single keys, modifier combinations (`C-`, `S-`, `M-`), and chord
  sequences (space-separated). Validate: no conflicts, `palette.open`
  always bound. Hot-reload on file change via `notify` crate. Map
  `crossterm::event::KeyEvent` + `KeyModifiers` to the parsed bindings.
- **Results:** _pending_

### Task 11: Implement which-key modal overlay
- **Status:** not_started
- **Dependencies:** Task 10
- **Description:** When a chord prefix key is pressed, show which-key
  popup per §3.3. Display all continuations with command names. Timeout
  (configurable, default 1s) cancels the prefix. Support arbitrary
  nesting depth.
- **Results:** _pending_

### Task 12: Implement `mnemosyne cmd` CLI subcommand
- **Status:** not_started
- **Dependencies:** Task 1 (command registry for `--list`)
- **Description:** Thin Rust binary or subcommand per §7. Connect to
  `daemon.sock`, send one NDJSON line, print response JSON to stdout,
  exit. `--list` queries command registry and prints table. `--help <id>`
  prints help for one command. Exit codes: 0 success, 1 daemon error,
  2 connection failure. No state, no TUI rendering, no interactive
  prompts.
- **Results:** _pending_

### Task 13: Implement expert lifecycle TUI commands
- **Status:** not_started
- **Dependencies:** Task 9
- **Description:** Wire `expert.create` and `expert.edit` palette entries.
  `expert.create`: prompt for expert ID, write template file to
  `<vault>/experts/<id>.md` using sub-N's default template, open in
  `$EDITOR`. `expert.edit`: fuzzy-select from existing expert files, open
  in `$EDITOR`. No daemon command — `DeclarationLoader` hot-reload
  handles registration.
- **Results:** _pending_

## Integration and cross-sub-project stubs

### Task 14: Push adoption stubs to sibling backlogs
- **Status:** not_started
- **Dependencies:** Tasks 1–6 (identifies which stubs are needed)
- **Description:** After Task 0 identifies interface gaps, push concrete
  adoption stubs to sibling backlogs per the brainstorming methodology.
  Known candidates:
  - Sub-C: add `create_plan` to tool injection set (§9 sub-C)
  - Sub-E: expose `submit_candidate/2` entry point (§9 sub-E)
  - Sub-A: refactor `adopt_project` to library-callable (§9 sub-A)
  - Sub-M: add `Plan.Created` and `Command.Executed` to sealed event
    set (§9 sub-M)
  - Sub-G: note clean break at Phase 1 in migration docs (§9 sub-G)
- **Results:** _pending_

## Sub-R adoption

### Task 15 — Absorb Sub-R: routing rule review in TUI `[r-adoption]`
- **Status:** not_started
- **Dependencies:** Task 9 (command palette), Sub-R engine landed
- **Description:** Per Sub-R design doc §7.3. With routing rules moving
  from `routing.ex` (Elixir) to `ontology/routing.dl` (Datalog), the
  TUI's routing-rule review surface may benefit from a dedicated view
  in the output pane that shows: (a) current active routing rules from
  `routing.dl`, (b) pending rule proposals from
  `<vault>/runtime/staging/proposed-rules/`. Accept/reject actions for
  proposals. No new daemon commands needed — ontology files are edited
  via `$EDITOR` like expert declarations. The review view is a
  read-only display of `Mnemosyne.Datalog.Server` state.
- **Results:** _pending_
