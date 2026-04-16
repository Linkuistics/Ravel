# Sub-I — Obsidian Coverage

**Status:** design complete (brainstorm Session 23, 2026-04-16)

**Owning plan:** `LLM_STATE/sub-I-obsidian-coverage/`

**Parent plan:** `LLM_STATE/mnemosyne-orchestrator/`

**Primary dependencies:** sub-A (vault layout, init scaffolding, git-tracking policy), sub-F (plan-catalog.md, plan-state.md frontmatter schema, NDJSON event types), sub-N (expert declaration format, default expert set), sub-E (knowledge entry format, Stage 5 output), sub-B (phase cycle frontmatter fields), sub-H (command palette — TUI-native, not Obsidian-surfaced in v1).

---

## §1 Scope and motivation

Sub-I documents which Obsidian features cover which Mnemosyne data
surfaces, and produces the `.obsidian/` template (plus supporting
Templater templates and starter Dataview query files) that ships with
v1. The deliverable is a curated vault configuration that makes every
file Mnemosyne writes browsable, queryable, and auditable by humans in
Obsidian.

Obsidian's role in v1 is as a **file-observation-based daemon client** —
it sees file changes on disk and renders them. It does not consume the
NDJSON protocol directly (that's sub-K, v1.5+). Real-time daemon events
are observable via the Rust TUI; Obsidian is the review/browse/audit
interface.

### Design philosophy

Obsidian is a **pragmatic bridge**, not the destination. The
long-term explorer UI is a Phoenix LiveView web application powered by
the Elixir daemon. The Obsidian template should be functional and
self-documenting for v1 without over-investing in elaborate
Obsidian-specific infrastructure. Focus on what Obsidian does uniquely
well as a file browser: native file tree, full-text search, tags
panel, backlinks, Dataview queries against YAML frontmatter, and
Templater for correct file creation.

### In scope

- Complete data-surface inventory mapping every Mnemosyne file type to
  Obsidian features
- Required community plugin set with configuration
- Templater templates for knowledge entries and expert declarations
- Starter Dataview query files (plan inventory, expert inventory,
  recent knowledge, knowledge by tag)
- `.obsidian/` file manifest with git-tracking policy
- Symlink handling verification
- First-open experience design
- Daemon event stream v1 vs v1.5+ coverage documentation
- Cross-sub-project requirements on siblings

### Out of scope

- Graph view configuration (not useful in practice; plan hierarchy is
  a filesystem tree that Obsidian's file explorer already shows)
- CSS snippets (non-markdown files like `.toml` and `.ex` render as
  plain text in Obsidian's editor; no CSS snippet can add syntax
  highlighting to them; users who need code intelligence edit in their
  IDE)
- Custom dashboard note (plan-catalog.md is the homepage; no separate
  dashboard file)
- Obsidian plugin development (sub-K, v1.5+)
- Wikilink generation in machine-owned files for graph view purposes

---

## §2 Data surface inventory

### §2.1 Vault-level tracked surfaces

| Surface | Format | Obsidian coverage | Notes |
|---|---|---|---|
| `mnemosyne.toml` | TOML | File tree browse + plain text editor. No syntax highlighting for `.toml`. | Read-only for most users |
| `daemon.toml` | TOML | Same as above. User-edited. | Power-user file |
| `routing.ex` | Elixir source | Plain text editing. No Elixir syntax highlighting. Users who need code intelligence edit in their IDE. | User-edited; Obsidian is the browser, IDE is the editor |
| `plan-catalog.md` | Markdown | Full rendering — headings, wikilinks, reading mode. Homepage target. Machine-owned, auto-regenerated. | Primary vault orientation file |
| `experts/*.md` | Markdown + YAML frontmatter | Full rendering. Frontmatter visible in reading mode properties view. Dataview-queryable. Tags panel indexes expert tags. Search indexes persona body. | User-editable; Templater template for creation |
| `knowledge/` (Tier 2) | Markdown + YAML frontmatter | Full rendering. Dataview-queryable by tags, date, provenance. Backlinks show cross-references. Tags panel for navigation. | Expert-owned writes; human review/edit/reject |
| `archive/` | Mixed (session artifacts) | File tree browse + git history. Not Dataview-queryable unless entries have markdown with frontmatter. | Audit trail — browse, don't query |
| `.obsidian/` | JSON/CSS config | Not user-browsed — Obsidian's own settings UI is the interface. | Template is the deliverable |

### §2.2 Per-project surfaces (via `<vault>/projects/<name>` symlinks)

| Surface | Format | Obsidian coverage | Notes |
|---|---|---|---|
| `knowledge/` (Tier 1) | Markdown + YAML frontmatter | Same as Tier 2 — full rendering, Dataview-queryable. Appears under `projects/<name>/knowledge/`. | Plan-owned |
| `project-root/plan-state.md` | Markdown + YAML frontmatter | Frontmatter shows phase, description, tags. Reading mode renders body. Dataview-queryable for plan inventory views. | Machine-managed |
| `project-root/backlog.md` | Markdown | Full rendering. Human-readable task lists. Not Dataview-queryable (free-form markdown, no structured frontmatter). | LLM + human authored |
| `project-root/memory.md` | Markdown | Full rendering. Search-indexable. | LLM-authored, human-reviewable |
| `project-root/session-log.md` | Markdown | Full rendering. Scroll for history. | Append-only |
| Nested child plans | Same as above per plan | File tree mirrors hierarchy naturally. Each child's `plan-state.md` independently Dataview-queryable. | Hierarchy = filesystem |

### §2.3 Runtime/ephemeral surfaces (gitignored)

| Surface | Obsidian coverage | Notes |
|---|---|---|
| `runtime/daemon.sock` | Not visible (gitignored). Existence confirmable if "show gitignored" enabled. | Daemon liveness indicator |
| `runtime/staging/` | Not meaningfully browsable — transient per-phase scratch. | Ephemeral |
| `runtime/mailboxes/*.jsonl` | Browse raw JSONL for debugging. Not Dataview-queryable. | Rebuildable from archive |
| `runtime/events/` | Same as mailboxes — raw file browse only. | Ephemeral queue |

### §2.4 Daemon event stream (v1.5+ via sub-K)

| Event type | V1 coverage | V1.5+ coverage (sub-K) |
|---|---|---|
| `phase_transition` | Observable via TUI; reflected in `plan-state.md` frontmatter (Obsidian sees the file update) | Live panel in Obsidian plugin |
| `mailbox_received` | Observable via TUI | Live notification |
| `harness_output` | Observable via TUI only | Streamed in Obsidian terminal pane |
| `dispatch_processed` | Observable via TUI | Live panel |
| `query_answered` | Observable via TUI | Live panel |
| `rule_fired` | Observable via TUI | Live panel |
| `rule_suggestion` | Observable via TUI | Prompted in Obsidian |
| `actor_state_change` | Observable via TUI; reflected in `plan-state.md` | Live panel |

V1 coverage of real-time events is indirect — Obsidian sees file changes
when `plan-state.md` updates after a phase transition, but does not see
streaming events. The TUI is the live view; Obsidian is the
review/browse/audit interface.

---

## §3 Required plugins

### §3.1 Community plugins

| Plugin | ID | Why required |
|---|---|---|
| **Dataview** | `dataview` | Queries against YAML frontmatter across plan-state.md, expert declarations, and knowledge entries. Without it, the vault is navigable but not queryable. |
| **Templater** | `templater-obsidian` | Pre-fills correct YAML frontmatter schemas for knowledge entries and expert declarations. Prevents hard-error failures from malformed frontmatter. |
| **Homepage** | `homepage` | Sets `plan-catalog.md` as the file that opens on vault launch. Without it, Obsidian opens to whatever was last viewed — on first open, nothing. |

Three plugins. No more.

### §3.2 Core plugin configuration

**Enabled:**
- File explorer — primary navigation
- Search — full-text search across all vault files
- Tags view — browse by tag (expert scope tags, knowledge tags)
- Backlinks — incoming references between knowledge entries
- Properties view — structured YAML frontmatter rendering in reading mode
- File recovery — safety net for accidental edits

**Disabled (reduce noise):**
- Graph view — not useful for this vault's structure
- Daily notes — not relevant to Mnemosyne's workflow
- Slides — not relevant
- Publish — not relevant
- Sync — vault is git-backed, not Obsidian Sync

### §3.3 App settings

| Setting | Value | Rationale |
|---|---|---|
| `showFrontmatter` | `true` | YAML frontmatter visible in source mode for editing |
| `readableLineLength` | `true` | Comfortable reading width for memory.md, backlog.md |
| `strictLineBreaks` | `true` | Predictable markdown rendering |
| `defaultViewMode` | `"preview"` | Reading mode by default (users browse more than edit) |
| `showLineNumber` | `true` | Useful for referencing specific lines in backlogs |

---

## §4 Templater templates

Two templates ship at `<vault>/templates/`. Templater is configured to
use this directory. Templates are git-tracked (shared config). Embedded
in the daemon release from `priv/obsidian/templates/`.

### §4.1 Knowledge entry template (`templates/knowledge-entry.md`)

```markdown
---
schema-version: 1
tags:
  - <% tp.system.prompt("Tags (comma-separated)", "")
       .split(",").map(t => t.trim()).filter(t => t).join("\n  - ") %>
created: <% tp.date.now("YYYY-MM-DDTHH:mm:ssZ") %>
source: manual
---

# <% tp.system.prompt("Entry title") %>

<% tp.system.prompt("Entry content (one concrete claim or fact)") %>
```

Design rationale:
- `schema-version: 1` always present — prevents parse rejection
- `tags` emitted as a YAML list, not a comma-separated string
- `created` timestamp auto-filled
- `source: manual` distinguishes human-authored from ingestion-pipeline
  entries
- Prompt for "one concrete claim or fact" nudges toward the knowledge
  format that `memory-style.md` requires

### §4.2 Expert declaration template (`templates/expert-declaration.md`)

```markdown
---
description: <% tp.system.prompt("Description (≤120 chars, keyword-dense, noun-phrase-led)") %>
kind: expert
schema-version: 1
tags:
  - <% tp.system.prompt("Tags (comma-separated, kebab-case)")
       .split(",").map(t => t.trim()).filter(t => t).join("\n  - ") %>
scope:
  tier2:
    - knowledge/<% tp.system.prompt("Tier 2 scope directory (e.g. 'rust' for knowledge/rust/**)") %>/**
retrieval:
  strategy: keyword_section
  top_n: 8
  max_bytes_per_entry: 4096
model: null
dialogue:
  allow_clarifying_questions: true
  max_clarification_rounds: 3
---

# Persona

<% tp.system.prompt("Describe how this expert thinks and responds") %>

## How you answer

- If the question is precise and your retrieved knowledge covers it,
  answer directly in 1-3 paragraphs.
- If the question is ambiguous, emit a clarifying question rather than
  guessing.
- Never fabricate claims. If you don't find evidence in retrieved
  knowledge, say so.

## How you curate (Stage 5 absorption)

When asked to absorb a candidate entry, return `ABSORB` only if:

- The entry is within your domain
- The entry makes a concrete claim
- The entry would change how a practitioner works

Return `REJECT` with a reason otherwise.
```

Design rationale:
- All required fields present: `description`, `kind`, `schema-version`,
  `tags`, `scope.tier2`, `retrieval.strategy`
- `kind: expert` hardcoded — prevents the "anything else rejects"
  error from sub-N's `DeclarationLoader`
- Retrieval defaults pre-filled — users can adjust but start correct
- Persona scaffold follows the convention from sub-N §3.1
- Description prompt reminds about 120-char cap and keyword-dense style

---

## §5 Starter Dataview query files

Four view files shipped by `mnemosyne init` into `<vault>/views/`.
Human-owned — users can edit, delete, or add more. Not machine-generated,
not auto-regenerated.

### §5.1 Plan inventory (`views/plan-inventory.md`)

```markdown
# Plan Inventory

All plans across all projects, with current phase and description.

```dataview
TABLE description AS "Description",
      current-phase AS "Phase",
      started-at AS "Started"
FROM "projects"
WHERE contains(file.name, "plan-state")
SORT file.path ASC
```
```

Queries every `plan-state.md` across symlinked projects. `FROM "projects"`
scopes to the symlink directory. Shows the three most useful frontmatter
fields at a glance.

### §5.2 Expert inventory (`views/expert-inventory.md`)

```markdown
# Expert Inventory

All registered expert declarations with their domain tags and scope.

```dataview
TABLE description AS "Description",
      tags AS "Tags",
      scope.tier2 AS "Tier 2 Scope"
FROM "experts"
WHERE kind = "expert"
SORT file.name ASC
```
```

Scoped to `experts/` directory. Shows what each expert covers and where
it reads from.

### §5.3 Recent knowledge (`views/recent-knowledge.md`)

```markdown
# Recent Knowledge

Knowledge entries across all tiers, most recently modified first.

```dataview
TABLE tags AS "Tags",
      source AS "Source",
      file.folder AS "Location"
FROM "projects" OR "knowledge"
WHERE file.frontmatter
  AND file.frontmatter.schema-version
  AND !contains(file.name, "plan-state")
  AND !(kind = "expert")
SORT file.mtime DESC
LIMIT 25
```
```

Captures both Tier 1 (under `projects/*/knowledge/`) and Tier 2 (under
`knowledge/`). Filters exclude `plan-state.md` files and expert
declarations (both also carry `schema-version`). Limited to 25 most
recent.

### §5.4 Knowledge by tag (`views/knowledge-by-tag.md`)

```markdown
# Knowledge by Tag

Knowledge entries grouped by their first tag.

```dataview
TABLE WITHOUT ID file.link AS "Entry",
      tags AS "All Tags",
      file.folder AS "Location"
FROM "projects" OR "knowledge"
WHERE file.frontmatter
  AND file.frontmatter.tags
  AND !contains(file.name, "plan-state")
  AND !(kind = "expert")
GROUP BY file.frontmatter.tags[0] AS "Primary Tag"
SORT rows.file.mtime DESC
```
```

Groups by the first tag in each entry's `tags` list (by convention, the
primary domain tag). Useful for seeing what knowledge exists per domain
and spotting gaps.

### §5.5 Design rationale

- **Four files, not more.** Each covers a distinct concern (plans,
  experts, knowledge-recent, knowledge-by-domain). Diminishing returns
  after these four.
- **Human-owned.** No "do not edit" warnings. Users are expected to
  modify or extend as they learn Dataview.
- **`views/` directory.** Keeps query files separate from vault content.
  Conventional name, not reserved — no special meaning to the daemon.
- **No session-log view.** Session logs are append-only markdown without
  structured frontmatter — Dataview cannot usefully query them. Browsing
  the file directly is the right interface.

---

## §6 `.obsidian/` file manifest

### §6.1 Tracked files

| File | Content |
|---|---|
| `community-plugins.json` | `["dataview", "templater-obsidian", "homepage"]` |
| `core-plugins.json` | Enables: file-explorer, search, tag-pane, backlink, properties, file-recovery. Disables: graph, daily-notes, slides, publish, sync. |
| `app.json` | Settings per §3.3 |
| `plugins/dataview/data.json` | `enableDataviewJs: false` (security), `enableInlineQueries: true`, `refreshEnabled: true` |
| `plugins/templater-obsidian/data.json` | `templates_folder: "templates"`, `trigger_on_file_creation: false` (explicit invocation only) |
| `plugins/homepage/data.json` | `defaultNote: "plan-catalog"`, `openMode: "replaceAll"`, `openOnStartup: true` |

### §6.2 Gitignored files

| File/dir | Rationale |
|---|---|
| `workspace.json` | Per-machine pane layout, churns constantly |
| `workspace-mobile.json` | Same, mobile variant |
| `cache/` | Obsidian internal cache, derived data |

### §6.3 Embedding in daemon release

`.obsidian/`, `templates/`, and `views/` are embedded in the daemon
release from `priv/obsidian/`. At `mnemosyne init` time, the daemon
copies these into the new vault. After init, the files are human-owned
(users may modify plugin configs, templates, and views).

Complete init-time file set:

```
<vault>/
├── .obsidian/
│   ├── community-plugins.json
│   ├── core-plugins.json
│   ├── app.json
│   └── plugins/
│       ├── dataview/data.json
│       ├── templater-obsidian/data.json
│       └── homepage/data.json
├── templates/
│   ├── knowledge-entry.md
│   └── expert-declaration.md
└── views/
    ├── plan-inventory.md
    ├── expert-inventory.md
    ├── recent-knowledge.md
    └── knowledge-by-tag.md
```

### §6.4 CSS snippets

Zero shipped. Non-markdown files (`mnemosyne.toml`, `daemon.toml`,
`routing.ex`) render as plain text in Obsidian's editor — no CSS snippet
can add syntax highlighting to non-markdown files. Users who need code
intelligence for `routing.ex` edit in their IDE.

---

## §7 Symlink handling

Obsidian follows symlinks natively on macOS and Linux. The
`<vault>/projects/` directory contains symlinks into project repos:

```
projects/mnemosyne -> /Users/antony/Development/Mnemosyne/mnemosyne/
projects/apianyware-macos -> /Users/antony/Development/APIAnyware-MacOS/mnemosyne/
```

Obsidian treats symlinked directories as normal directories — file tree
shows them inline, search indexes their contents, Dataview queries
traverse into them, backlinks work across the symlink boundary.

**Git interaction:** `projects/` is gitignored (A§A5 — symlink targets
are machine-local absolute paths). Obsidian's git integration won't
track changes under `projects/`. Correct — project files are under
sovereign git in their own repos.

**Verification requirement:** Sub-I's implementation should include a
guivision + OCR spike confirming symlink traversal on macOS. Checks:
file tree shows symlinked project directories, Dataview queries in
`views/plan-inventory.md` return results from symlinked plans, search
finds content within symlinked knowledge directories.

---

## §8 First-open experience

Complete flow when a user runs `mnemosyne init` and opens the vault in
Obsidian:

1. **Trust plugins dialog.** Obsidian prompts to trust community plugins.
   User clicks "Trust author and enable plugins."
2. **Homepage opens `plan-catalog.md`.** Shows machine-owned header and
   (for a fresh vault) zero plan entries. Expected — the vault is empty
   until the user adopts projects.
3. **File tree shows vault structure.** User sees: `archive/`,
   `experts/`, `knowledge/`, `templates/`, `views/`, plus root-level
   `.toml` and `.ex` files. `projects/` appears empty (no symlinks yet).
   `runtime/` appears only if the daemon is running.
4. **User explores `views/`.** Four starter queries visible. Opening any
   view shows a Dataview table — empty on first open, but query structure
   visible and will populate as projects are adopted and experts arrive.
5. **User can create an expert immediately.** `Ctrl/Cmd+P` → "Templater:
   Create new note from template" → select `expert-declaration` → fill
   prompts → file appears in `experts/` with correct frontmatter.

The cold-start experience is deliberately sparse. A fresh vault with no
adopted projects has no plans, no knowledge, and only the default experts
(shipped by sub-N's init). The template's job is to make the vault
**ready** — navigable, queryable, correctly configured — not to populate
it with content.

---

## §9 Cross-sub-project requirements

Sub-I imposes lightweight requirements on siblings. Most are
confirmations of existing commitments.

### On Sub-A (global store)

- **`templates/` and `views/` in init scaffolding.** A's `init` command
  copies Templater templates and Dataview view files alongside the
  existing `.obsidian/` copy. Source: `priv/obsidian/templates/` and
  `priv/obsidian/views/`. Mechanically identical to existing `.obsidian/`
  copy.
- **`templates/` and `views/` in git-tracking policy.** Both directories
  tracked (shared config, same rationale as `.obsidian/`). Add rows to
  A§A5's policy table.

### On Sub-N (domain experts)

- **Default expert set ships at init.** Already committed in N§3.5.
  Sub-I depends on this but imposes no new requirement.

### On Sub-F (hierarchy)

- **`plan-catalog.md` filename stable.** Already committed. Homepage
  plugin points at it. No new requirement.

### On Sub-B (phase cycle)

- **`plan-state.md` frontmatter fields queryable.** Already committed
  in F§1.3 — `description`, `current-phase`, `started-at`, `tags`.
  Sub-I's plan-inventory view queries these exact fields. No new
  requirement.

### On Sub-G (migration)

- **No impact.** `templates/` and `views/` live in the vault, not in
  project repos. Unaffected by G's Phase 1 rename.

### On Sub-E (ingestion)

- **Knowledge entries must include `schema-version` in frontmatter.**
  Sub-I's recent-knowledge view filters on `schema-version` to
  distinguish knowledge entries from other markdown files. Already
  implied by E's format but should be explicitly confirmed in E's
  amendment task.

---

## §10 Implementation task summary

Ordered by dependency:

1. **Symlink verification spike** — guivision + OCR evidence that
   Obsidian follows symlinks on macOS: file tree, Dataview queries,
   search across symlink boundary.
2. **Author `.obsidian/` configuration files** — `app.json`,
   `core-plugins.json`, `community-plugins.json`, plugin config JSONs
   per §6.1.
3. **Author Templater templates** — `knowledge-entry.md` and
   `expert-declaration.md` per §4.
4. **Author Dataview view files** — four views per §5.
5. **Sub-A amendment** — extend init scaffolding to copy `templates/`
   and `views/`; update git-tracking policy table.
6. **Sub-E amendment** — confirm `schema-version` presence in
   ingestion-written knowledge entries.
7. **End-to-end acceptance test** — `mnemosyne init` → open vault in
   Obsidian → verify first-open experience per §8 flow → verify
   Dataview queries return results after adopting a project with
   plans. Guivision + OCR evidence.

---

## Appendix A — Decision Trail

### Q1. How opinionated should the template be?

**Context:** Obsidian templates range from minimal (just enable plugins)
to rich (CSS themes, graph config, dozens of plugins). Mnemosyne's
long-term explorer UI is Phoenix LiveView, making Obsidian a pragmatic
bridge.

**Decision:** Approach 2 — lean configuration plus starter Dataview
query files. Ship Dataview + Templater + Homepage as required plugins,
two Templater templates, four Dataview view files. No CSS snippets,
no graph view configuration. Provides immediate utility without
over-investing in a bridge technology.

### Q2. Should the vault ship a custom dashboard note?

**Context:** Plan-catalog.md (machine-owned, auto-regenerated) vs a
human-editable dashboard.md with embedded Dataview queries.

**Decision:** Plan-catalog.md as homepage via the Homepage plugin. No
custom dashboard. The catalog is the natural orientation file; a second
dashboard creates confusion about which is authoritative.

### Q3. Which community plugins are required?

**Context:** Prior decisions locked Dataview as required, Templater as
optional. User open to requiring more.

**Decision:** Three required: Dataview, Templater, Homepage. Templater
upgraded from optional to required because it directly prevents
hard-error failures from malformed frontmatter. Homepage required for
the first-open experience. No other plugins — each candidate evaluated
and rejected on grounds of insufficient data-surface coverage.

### Q4. Should graph view be configured?

**Context:** Obsidian's graph view could visualize plan hierarchy and
expert-knowledge relationships if files contain wikilinks.

**Decision:** No. Plan hierarchy is a filesystem tree that Obsidian's
file explorer already shows. Most cross-references in Mnemosyne are
implicit (filesystem paths, qualified IDs, frontmatter tags) rather
than explicit wikilinks. Graph view without real wikilinks would be
misleading. User confirmed graph view is not useful in practice.

### Q5. Where do Dataview queries live?

**Context:** Options: embedded in plan-catalog.md (mixes machine-owned
static content with dynamic Dataview), satellite `views/` directory
(separate files), or no shipped queries.

**Decision:** Separate `views/` directory with four human-owned files.
Plan-catalog.md stays machine-owned static text (consumed by LLM phases
via `{{VAULT_CATALOG}}`). View files are starter material, not
machine-managed.
