# Backlog — Sub-I Obsidian Coverage

Tasks for producing the `.obsidian/` template, Templater templates,
starter Dataview query files, and verifying Obsidian coverage of
Mnemosyne data surfaces. Design doc:
`{{PROJECT}}/docs/superpowers/specs/2026-04-16-sub-I-obsidian-coverage-design.md`.

## Task 1 — Symlink verification spike

- **Status:** not_started
- **Dependencies:** none
- **Description:** Guivision + OCR evidence that Obsidian follows
  symlinks on macOS. Verify: file tree shows symlinked project
  directories, Dataview queries in plan-inventory view return results
  from symlinked plans, search finds content within symlinked knowledge
  directories. Evidence artifacts at
  `{{PROJECT}}/tests/fixtures/obsidian-symlink-spike/results/macos/`.
  Follow the guivision + OCR evidence pattern established in memory.
- **Results:** _pending_

## Task 2 — Author `.obsidian/` configuration files

- **Status:** not_started
- **Dependencies:** none (can parallel with Task 1)
- **Description:** Create the `.obsidian/` configuration files at
  `{{PROJECT}}/priv/obsidian/.obsidian/`:
  - `app.json` — settings per design doc §3.3
  - `core-plugins.json` — enables/disables per §3.2
  - `community-plugins.json` — `["dataview", "templater-obsidian", "homepage"]`
  - `plugins/dataview/data.json` — per §6.1
  - `plugins/templater-obsidian/data.json` — per §6.1
  - `plugins/homepage/data.json` — per §6.1
  
  These are the embedded resources that `mnemosyne init` copies into new
  vaults.
- **Results:** _pending_

## Task 3 — Author Templater templates

- **Status:** not_started
- **Dependencies:** none (can parallel with Tasks 1-2)
- **Description:** Create two Templater templates at
  `{{PROJECT}}/priv/obsidian/templates/`:
  - `knowledge-entry.md` — per design doc §4.1
  - `expert-declaration.md` — per design doc §4.2
  
  Verify Templater prompt syntax is correct by testing in a scratch
  Obsidian vault.
- **Results:** _pending_

## Task 4 — Author Dataview view files

- **Status:** not_started
- **Dependencies:** none (can parallel with Tasks 1-3)
- **Description:** Create four starter Dataview query files at
  `{{PROJECT}}/priv/obsidian/views/`:
  - `plan-inventory.md` — per design doc §5.1
  - `expert-inventory.md` — per design doc §5.2
  - `recent-knowledge.md` — per design doc §5.3
  - `knowledge-by-tag.md` — per design doc §5.4
  
  Verify Dataview query syntax is correct by testing against a vault
  with sample plan-state.md, expert declarations, and knowledge entries.
- **Results:** _pending_

## Task 5 — Sub-A amendment: init scaffolding

- **Status:** not_started
- **Dependencies:** Tasks 2-4 complete (files must exist to reference)
- **Description:** Amend sub-A's design doc and backlog to extend the
  `init` command scaffolding:
  - Copy `priv/obsidian/templates/` to `<vault>/templates/`
  - Copy `priv/obsidian/views/` to `<vault>/views/`
  - Add `templates/` and `views/` rows to A§A5 git-tracking policy
    table (both tracked, rationale: shared config)
  
  This is a lightweight amendment — mechanically identical to the
  existing `.obsidian/` copy in A's init flow.
- **Results:** _pending_

## Task 6 — Sub-E amendment: schema-version confirmation

- **Status:** not_started
- **Dependencies:** none
- **Description:** Confirm that sub-E's ingestion pipeline writes
  `schema-version` to knowledge entry frontmatter. Sub-I's
  recent-knowledge Dataview view filters on this field. If not already
  explicit in E's design doc, add it as a requirement in E's amendment
  task.
- **Results:** _pending_

## Task 7 — End-to-end acceptance test

- **Status:** not_started
- **Dependencies:** Tasks 1-6 complete
- **Description:** Full acceptance test via guivision + OCR:
  1. `mnemosyne init` creates a vault with all template files
  2. Open vault in Obsidian — verify trust-plugins dialog
  3. Verify Homepage opens plan-catalog.md
  4. Verify file tree shows expected structure (templates/, views/,
     experts/, knowledge/, etc.)
  5. Adopt a project with existing plans
  6. Verify plan-inventory Dataview query returns results
  7. Verify expert-inventory query shows default experts
  8. Create an expert via Templater template — verify correct
     frontmatter
  9. Create a knowledge entry via Templater — verify correct frontmatter
  
  Evidence artifacts at
  `{{PROJECT}}/tests/fixtures/obsidian-e2e/results/macos/`.
- **Results:** _pending_
