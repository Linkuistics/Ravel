# Backlog — Sub-project R: Knowledge Ontology

Implementation backlog for sub-project R. All tasks derive from §1–§11
of the design doc at
`{{PROJECT}}/docs/superpowers/specs/2026-04-16-sub-R-knowledge-ontology-design.md`.
Consult the spec before starting any task.

**Sibling plan scaffolded** in the orchestrator plan (2026-04-16)
immediately after the sub-R brainstorm. The spec is frozen; this plan
executes against it.

**Implementation structure:** Three research tasks gate the engine
implementation. The engine is domain-agnostic and testable in isolation.
The ontology fact schema and consumer integration follow. Obsidian views
and LLM exposure are the final layer.

**Adoption by siblings:** Sub-R imposes amendment tasks on F, N, E, A,
C, H, M, I. Those tasks live in their respective sibling backlogs, not
here. Sub-R delivers the engine + ontology + integration API; siblings
consume them.

## Tasks

### Literature survey: Datalog evaluation strategies

**Category:** `research`
**Status:** `not_started`
**Dependencies:** none

**Description:**

Per spec §8.1. Survey semi-naive evaluation (Ullman 1988), magic sets
(Bancilhon et al. 1986), demand-driven / top-down with tabling (XSB),
incremental maintenance (DRed, Differential Dataflow), trie-based
indexing. Assess each against Ravel's query patterns: small fact base
(<10k facts), frequent point queries, infrequent bulk re-evaluation,
BEAM runtime. Output: a research note naming the chosen strategy with
rationale, filed at
`{{PROJECT}}/docs/research/datalog-evaluation-strategies.md`. This
gates the `Ravel.Datalog` core evaluator task.

**Results:** _pending_

---

### Literature survey: ontology design

**Category:** `research`
**Status:** `not_started`
**Dependencies:** none

**Description:**

Per spec §8.2. Survey SKOS (W3C 2009), OWL, CYC microtheory
architecture, folksonomy research (Gruber 2007; Mika 2007), Luhmann
Zettelkasten indexing. Ground Sub-R's fact schema in established KR
practice. Output: additions to
`{{PROJECT}}/docs/research-sources.md` and any schema adjustments.
May refine the predicate vocabulary in the spec (§3).

**Results:** _pending_

---

### Audit: Datalog on BEAM

**Category:** `research`
**Status:** `not_started`
**Dependencies:** none

**Description:**

Per spec §8.3. Audit Erlog (maintenance status, capabilities, Datalog
restriction feasibility), Hex.pm for any Elixir/Erlang Datalog
packages, ETS performance for our access patterns. Confirm "build
custom" is justified per "prefer integration over reinvention."
Output: a "why build custom" justification or adoption recommendation.
Gates the `Ravel.Datalog` core evaluator task.

**Results:** _pending_

---

### Ravel.Datalog core evaluator

**Category:** `impl`
**Status:** `not_started`
**Dependencies:** Literature survey: Datalog evaluation strategies, Audit: Datalog on BEAM

**Description:**

Per spec §2. Implement the Datalog evaluator using the strategy chosen
by the evaluation-strategies literature survey. Data model per §2.2.
API per §2.4: `new/0`, `assert/2`, `retract/2`, `load_file/2`,
`evaluate/1`, `query/2`, `query_one/2`. ETS-backed storage per §2.3
with base/derived fact partitioning. Stratified negation with
stratifiability check at load time. Domain-agnostic — zero Ravel
imports. ~300-500 lines (evaluator only, excluding parser).

**Results:** _pending_

---

### Ravel.Datalog.Parser

**Category:** `impl`
**Status:** `not_started`
**Dependencies:** Ravel.Datalog core evaluator

**Description:**

Per spec §2.5. Custom parser for Datalog syntax: facts
(`.`-terminated), rules (`:-`), capitalized variables, lowercase
underscore-separated atoms, anonymous variable `_`, `%` comments,
`not` negation in rule bodies. Validation: syntax errors with file
path + line number, range-restriction enforcement, tag format
enforcement. ~150-200 lines.

**Dependency context:** needs data model types from the core evaluator.

**Results:** _pending_

---

### Engine unit tests

**Category:** `test`
**Status:** `not_started`
**Dependencies:** Ravel.Datalog core evaluator, Ravel.Datalog.Parser

**Description:**

Per spec §9.1. Pure Datalog tests with no Ravel dependencies:

- Parser: valid syntax, syntax errors with line numbers, edge cases
- Evaluation: fixed-point, transitive closure, stratified negation,
  recursive rules, multiple rules for same predicate
- Termination: property-based with StreamData (random fact bases +
  rules → always terminates)
- Query: variable binding, multi-variable, no-result, ground queries
- Stratification: valid accepted, unstratifiable rejected

**Results:** _pending_

---

### Ontology fact schema: vocabulary + relations + rules

**Category:** `impl`
**Status:** `not_started`
**Dependencies:** Literature survey: ontology design, Ravel.Datalog core evaluator, Ravel.Datalog.Parser

**Description:**

Per spec §3. Author the shipped `.dl` files:

- `vocabulary.dl` — core `tag/1`, `synonym/2`, `parent/2` facts for
  the six default experts' domains
- `relations.dl` — typed relations (`bridges/2`, `uses/2`,
  `requires/2`, `related/2`) between concepts
- `rules.dl` — derived predicates: `ancestor/2`, `resolves_to/2`,
  `relevant_expert/2`

Naming convention: snake_case atoms. Open relation set — document
that users add predicates freely.

**Dependency context:** ontology literature may refine predicates.

**Results:** _pending_

---

### Ravel.Datalog.Server GenServer

**Category:** `impl`
**Status:** `not_started`
**Dependencies:** Ravel.Datalog core evaluator, Ravel.Datalog.Parser

**Description:**

Per spec §2.6. GenServer wrapping the Datalog API. Client API:
`query/1`, `query_one/1`, `reload_file/1`, `assert_facts/1`,
`retract_facts/1`. Startup: load `ontology/*.dl`, parse expert
declarations → assert `expert_tag/2`, evaluate, emit
`%OntologyLoaded{}`. Hot reload: re-parse changed file, retract old
facts, assert new, re-evaluate, regenerate views, emit event.
Filesystem watcher integration for `.dl` and expert declaration files.

**Results:** _pending_

---

### Ontology integration tests

**Category:** `test`
**Status:** `not_started`
**Dependencies:** Ontology fact schema: vocabulary + relations + rules, Ravel.Datalog.Server GenServer

**Description:**

Per spec §9.2. Test shipped predicates against known scenarios:
synonym resolution (`rustlang` → `rust`), hierarchy traversal,
typed-relation matching (`bridges(ffi, rust)` → `rust_expert`
relevant for `ffi`), expert matching with six default declarations,
orphan detection.

**Results:** _pending_

---

### ScopeMatcher integration (Sub-N consumer)

**Category:** `impl`
**Status:** `not_started`
**Dependencies:** Ravel.Datalog.Server GenServer, Ontology fact schema: vocabulary + relations + rules

**Description:**

Per spec §4.1. Rewrite `ScopeMatcher.match_candidate/2` internals:
assert temporary `candidate_tag/1` facts, query `relevant_expert/2`,
retract temporaries. Interface unchanged. Kebab-to-snake conversion
on input.

**Results:** _pending_

---

### Stage 2 classification integration (Sub-E consumer)

**Category:** `impl`
**Status:** `not_started`
**Dependencies:** Ravel.Datalog.Server GenServer, Ontology fact schema: vocabulary + relations + rules

**Description:**

Per spec §4.2. Replace flat tag-set intersection with
`resolves_to/2` queries for synonym normalization and optional
`ancestor/2` queries for broader tags. Stays deterministic Elixir.

**Results:** _pending_

---

### Fact extraction normalization (Sub-F consumer)

**Category:** `impl`
**Status:** `not_started`
**Dependencies:** Ravel.Datalog.Server GenServer, Ontology fact schema: vocabulary + relations + rules

**Description:**

Per spec §4.3. After LLM extracts keywords, normalize each via
`resolves_to/2` query. Unresolved keywords pass through. Feeds into
routing queries.

**Results:** _pending_

---

### Routing migration: routing.ex → routing.dl (Sub-F amendment)

**Category:** `impl`
**Status:** `not_started`
**Dependencies:** Ravel.Datalog.Server GenServer, Ontology fact schema: vocabulary + relations + rules

**Description:**

Per spec §4.4 and §7. Author `ontology/routing.dl` with routing
rules. Rewrite `Ravel.Router.evaluate_rules/2` to call Datalog query.
Update Level 2 routing agent prompt to propose Datalog rules. Update
rule staging path to `<vault>/runtime/staging/proposed-rules/`.
Remove `routing.ex` references from Sub-F codebase.

**Results:** _pending_

---

### Consumer integration tests

**Category:** `test`
**Status:** `not_started`
**Dependencies:** ScopeMatcher integration (Sub-N consumer), Stage 2 classification integration (Sub-E consumer), Fact extraction normalization (Sub-F consumer), Routing migration: routing.ex → routing.dl (Sub-F amendment)

**Description:**

Per spec §9.3. One test per consumer:

- ScopeMatcher: `rustlang` → matches `rust_expert`; `ffi` → matches
  both `rust_expert` and `ffi_expert`
- Stage 2: "Rust FFI callbacks" → canonical tags `[rust, ffi]`
- Routing: facts `["rust", "ffi"]` → routes to `rust_expert`
- Ontology reload: change `.dl` file, verify consumers see updates

**Results:** _pending_

---

### {{ONTOLOGY_CONTEXT}} placeholder (passive LLM exposure)

**Category:** `impl`
**Status:** `not_started`
**Dependencies:** Ravel.Datalog.Server GenServer

**Description:**

Per spec §5.1. Implement placeholder substitution in
`StagingDirectory::render`. Scope rules: expert sessions get their
tag neighborhood, plan sessions get recent fact-extraction keyword
neighborhood, routing sessions get full routing rules + relevant
tags. Injection format: readable Datalog syntax.

**Results:** _pending_

---

### ontology.query tool (active LLM exposure)

**Category:** `impl`
**Status:** `not_started`
**Dependencies:** Ravel.Datalog.Server GenServer

**Description:**

Per spec §5.2. Register `ontology.query` as a tool in Sub-C's adapter
for plan/expert sessions. Input: Datalog query string. Output: list
of bindings as JSON. Gated by tool profile — internal daemon sessions
excluded.

**Results:** _pending_

---

### Obsidian view generation

**Category:** `impl`
**Status:** `not_started`
**Dependencies:** Ravel.Datalog.Server GenServer, Ontology fact schema: vocabulary + relations + rules

**Description:**

Per spec §6. Auto-generate four Markdown views from the fact base:
`tag-index.md`, `taxonomy.md`, `relations.md`, `routing-rules.md`.
Snake-to-kebab conversion for display. Wikilinks to expert
declarations. Regenerated on every ontology change. Written to
`<vault>/ontology/views/`.

**Results:** _pending_

---

### Sub-M event structs

**Category:** `impl`
**Status:** `not_started`
**Dependencies:** none

**Description:**

Per spec §7.3. Define three new event structs: `%OntologyLoaded{}`,
`%OntologyReloadFailed{}`, `%RoutingRuleProposed{}`. Add to Sub-M's
sealed event set.

**Results:** _pending_

---

### Sub-A vault layout amendment

**Category:** `amendment`
**Status:** `done`
**Dependencies:** none

**Description:**

Per spec §10.2. Update Sub-A's design doc §A4: remove
`tag-vocabulary.md` reservation, add `ontology/` directory with
contents. Evaluate whether `ravel.toml` schema version needs a bump.

**Results:**

Done (2026-04-16). Sub-A design doc rewritten inline:
`tag-vocabulary.md` removed, `routing.ex` removed, `ontology/`
directory added with full contents, init scaffolding updated, Decision
Trail Q8 added. Backlog stub (Task 16) pushed to Sub-A for
implementation-time `init` scaffolding changes.

---

### Sub-F design doc amendment

**Category:** `amendment`
**Status:** `done`
**Dependencies:** none

**Description:**

Per spec §7.1. Inline rewrite of Sub-F's routing sections:
`routing.ex` → `ontology/routing.dl`, Elixir pattern matching →
Datalog queries, BEAM hot reload → fact-base reload, Level 2 agent
prompt update.

**Results:**

Done (2026-04-16). Sub-F design doc rewritten inline: all routing
sections updated from Elixir pattern matching to Datalog, vault layout
updated, examples converted, Erlog reservation removed, Decision Trail
entry added. Backlog stubs (Tasks 29-30) pushed to Sub-F for routing
migration + fact extraction normalization.

---

### Sub-I Obsidian coverage amendment

**Category:** `amendment`
**Status:** `done`
**Dependencies:** none

**Description:**

Per spec §7.3. Add `ontology/views/` to Sub-I's data-surface
inventory. Document the four auto-generated views.

**Results:**

Done (2026-04-16). Sub-I design doc rewritten inline: six new rows in
data-surface table (ontology/*.dl + four views), first-open experience
updated, cross-sub-project requirements section added, Decision Trail
Q6 added. Backlog stub (Task 8) pushed to Sub-I.

---
