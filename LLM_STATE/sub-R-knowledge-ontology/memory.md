# Memory — Sub-project R: Knowledge Ontology

This plan implements sub-project R of the Ravel orchestrator merge.
R's brainstorm is complete; this plan covers both research tasks and
implementation work. The design is fully specified in the spec referenced
below.

## Primary reference

**`{{PROJECT}}/docs/superpowers/specs/2026-04-16-sub-R-knowledge-ontology-design.md`**
is the authoritative design document. Every task in this plan's backlog
derives from the spec's §1–§11.

## Parent plan

The orchestrator-level plan lives at
`{{PROJECT}}/LLM_STATE/ravel-orchestrator/` (currently — will move
to `{{PROJECT}}/ravel/project-root/` after sub-project G's
migration lands). It coordinates this sub-plan with its siblings. The
parent plan's `memory.md` holds cross-sub-project architectural state.

## Sub-R is Ravel's declarative reasoning substrate

Sub-R owns both the Datalog engine and the ontology. The engine is
domain-agnostic; the ontology is the domain-specific fact schema.
This co-design avoids premature interface splits.

## Purpose-built Datalog in Elixir, not Erlog

Clean Datalog from scratch is simpler and safer than restricting Prolog.
Bottom-up evaluation guarantees termination by construction. Evaluation
strategy informed by literature survey (§8.1), not locked a priori.

## Datalog replaces routing.ex in v1

One reasoning substrate, not two. `routing.ex` (Elixir pattern matching)
→ `ontology/routing.dl` (Datalog rules). Sub-F amendment required.
Datalog is fundamentally better than imperative Elixir for user-facing
rules.

## .dl source of truth, auto-generated Obsidian views

Users edit Datalog files directly. The daemon regenerates Markdown views
for Obsidian browsing on every change. Phoenix LiveView replaces the
views post-v1.

## Research before implementation

Three literature surveys gate implementation: (1) Datalog evaluation
strategies, (2) ontology design literature (SKOS, OWL, CYC,
folksonomies), (3) existing BEAM Datalog libraries. Per user direction:
"there has been a lot of work on efficient datalog evaluation."

## Cross-sub-project requirements

Sub-R imposes adoption work on nine siblings. Adoption stubs are in the
orchestrator backlog's Sub-R results and detailed in the design doc §7.3.
Each sibling's backlog should receive its own amendment task.
