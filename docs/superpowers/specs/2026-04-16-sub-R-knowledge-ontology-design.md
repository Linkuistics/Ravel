# Sub-R: Knowledge Ontology — Design Spec

> A declarative reasoning substrate for Ravel: a purpose-built Datalog
> engine, a typed ontology of concept relations, and a unified fact base
> that replaces both tag-vocabulary drift management and Elixir-based
> routing rules.

**Status:** design complete
**Date:** 2026-04-16
**Depends on:** sub-N (ScopeMatcher interface), sub-A (vault layout),
sub-F (routing architecture)
**Consumed by:** sub-N (scope matching), sub-E (Stage 2 classification),
sub-F (routing rule evaluation, fact extraction normalization), sub-C
(ontology.query tool profile), sub-H (TUI routing-rule review), sub-M
(ontology events), sub-I (Obsidian coverage for ontology views)

---

## §1 Scope and non-goals

### §1.1 Scope

Sub-R delivers three capabilities, co-designed:

1. **R1 — Datalog engine.** A pure-Elixir, domain-agnostic Datalog
   evaluator with literature-informed evaluation strategy, ETS-backed
   fact storage, and guaranteed termination over finite domains. The
   engine knows nothing about Ravel — it is a general-purpose
   Datalog implementation.
2. **R2 — Ontology fact schema.** The domain-specific vocabulary of
   predicates (tags, synonyms, hierarchy, typed relations) and derived
   rules that Ravel uses for concept resolution, scope matching,
   and routing. Stored as `.dl` files in the vault.
3. **R3 — Consumer integration.** Four existing consumers migrate to
   Datalog queries: sub-N's ScopeMatcher, sub-E's Stage 2
   classification, sub-F's fact extraction normalization, and sub-F's
   routing rule evaluation (replacing `routing.ex`).

Additionally:

4. **R4 — LLM exposure.** Passive context injection via
   `{{ONTOLOGY_CONTEXT}}` placeholder and active query via
   `ontology.query` tool.
5. **R5 — Obsidian views.** Auto-generated Markdown views of the
   ontology for browsing in Obsidian.
6. **R6 — Research tasks.** Three literature surveys inform the
   implementation: Datalog evaluation strategies, ontology design, and
   existing BEAM Datalog libraries.

### §1.2 Non-goals

- **Ontology learning / auto-induction.** The ontology is hand-authored.
  No LLM pass infers new synonyms or relations automatically. A
  compelling v2 candidate (the routing agent's learning loop could
  propose ontology facts, not just routing rules), but out of scope.
- **OWL/RDF compatibility.** The fact schema is informed by SKOS/OWL
  literature but does not implement or import those standards.
- **Vector/embedding integration.** Sub-Q (v2) owns semantic retrieval.
  The `ExpertRetrieval` behaviour chokepoint is the hook; Sub-R does
  not touch it.
- **Phoenix ontology browser.** V2. Auto-generated Markdown views are
  the v1 bridge.
- **Multi-user ontology editing.** V1 is single-user. Sub-P (team mode)
  owns concurrent editing.
- **Negation-as-failure beyond stratified.** Well-founded semantics and
  answer-set programming are out. Stratified negation is sufficient.
- **Performance optimization for >10k facts.** V1 targets a small
  knowledge base. The literature search identifies strategies to adopt
  if scale becomes an issue.

### §1.3 Key invariant

**The Datalog engine is domain-agnostic.** `Ravel.Datalog` knows
nothing about tags, experts, routing, or knowledge tiers. It is a
general-purpose Datalog evaluator that happens to be used by
Ravel's ontology. The domain knowledge lives in the `.dl` files and
the consumer modules. This separation is load-bearing: it keeps the
engine testable in isolation and potentially reusable.

---

## §2 Datalog engine (`Ravel.Datalog`)

### §2.1 Algorithm

The engine implements bottom-up evaluation over a finite Herbrand
universe with stratified negation. The specific evaluation strategy
(semi-naive, magic sets, demand-driven, incremental maintenance) is
determined by a literature survey (§8.1) before implementation begins.
Semi-naive (Ullman 1988) is the baseline reference; the implementation
may choose a strategy better suited to Ravel's query patterns (small
fact base, frequent point queries, infrequent bulk re-evaluation).

**Guarantees:**
- **Termination.** Bottom-up evaluation on finite domains always
  terminates. No user input can cause infinite loops.
- **Determinism.** Same facts + rules = same derived facts. No
  evaluation-order dependence.
- **Stratified negation.** `not` is supported in rule bodies but only in
  stratifiable programs. Stratifiability is checked at load time — hard
  error if unstratifiable.

### §2.2 Data model

```elixir
defmodule Ravel.Datalog do
  @type atom_val :: String.t()

  @type term_val :: atom_val | variable()
  @type variable :: {:var, atom()}

  @type fact :: {predicate :: atom(), args :: [atom_val]}

  @type literal :: {predicate :: atom(), args :: [term_val]}
  @type negated_literal :: {:not, literal()}

  @type rule :: %{
    head: literal(),
    body: [literal() | negated_literal()]
  }

  @type query :: %{
    predicate: atom(),
    args: [atom_val | variable()]
  }

  @type binding :: %{variable() => atom_val}
end
```

All variables in rule heads must appear in at least one positive literal
in the body (range restriction). Enforced at parse time — hard error on
violation.

### §2.3 Storage

ETS table per fact base. Two logical partitions:

- **Base facts.** Asserted from `.dl` files and runtime sources (expert
  declarations, temporary per-query facts). Survive re-evaluation.
- **Derived facts.** Produced by rule evaluation. Cleared and
  regenerated on re-evaluation.

Separation enables efficient reload: on `.dl` file change, clear derived
facts, reload base facts from the changed file, re-evaluate. Unchanged
base facts from other files are not re-parsed.

### §2.4 API

```elixir
defmodule Ravel.Datalog do
  @spec new() :: fact_base()
  @spec assert(fact_base(), fact() | rule()) :: fact_base()
  @spec retract(fact_base(), fact()) :: fact_base()
  @spec load_file(fact_base(), Path.t()) :: {:ok, fact_base()} | {:error, reason()}
  @spec evaluate(fact_base()) :: fact_base()
  @spec query(fact_base(), query()) :: [binding()]
  @spec query_one(fact_base(), query()) :: binding() | nil
end
```

The API is functional over an opaque `fact_base()` handle. The
`Ravel.Datalog.Server` GenServer (§2.6) wraps this in a stateful
process for daemon use.

### §2.5 Parser

Custom parser for a clean Datalog syntax. No Prolog baggage.

**Syntax:**
- Facts end with `.`
- Rules use `:-`
- Variables are `Capitalized`
- Atoms are `lowercase_underscored`
- Anonymous variable `_` (matches anything, no binding)
- Comments with `%`
- Negation with `not` prefix in rule bodies

**Example:**

```
% Synonyms
synonym(rustlang, rust).
synonym(rust_lang, rust).

% Hierarchy
parent(rust, systems_programming).
parent(systems_programming, programming).

% Transitive closure
ancestor(X, Y) :- parent(X, Y).
ancestor(X, Y) :- parent(X, Z), ancestor(Z, Y).

% Resolution: X resolves to its canonical form
resolves_to(X, X) :- tag(X), not synonym(X, _).
resolves_to(X, Canonical) :- synonym(X, Canonical).
```

**Validation at parse time:**
- Syntax errors → hard error with file path, line number, and
  description
- Range-restriction violations → hard error
- Non-lowercase atoms → hard error (tag format enforcement)

### §2.6 `Ravel.Datalog.Server`

A `GenServer` supervised under the daemon's supervision tree. Owns the
ETS-backed fact base and serializes mutations.

```elixir
defmodule Ravel.Datalog.Server do
  use GenServer

  # Client API
  @spec query(query()) :: [binding()]
  @spec query_one(query()) :: binding() | nil
  @spec reload_file(Path.t()) :: :ok | {:error, reason()}
  @spec assert_facts([fact()]) :: :ok
  @spec retract_facts([fact()]) :: :ok
end
```

**Startup sequence:**
1. Load all `ontology/*.dl` files (vocabulary, relations, rules,
   routing)
2. Parse all expert declarations → assert `expert_tag/2` facts
3. Run `evaluate/1` to derive all consequences
4. Register with the daemon's supervision tree
5. Emit `%OntologyLoaded{}` event via sub-M's bus

**Hot reload** (triggered by filesystem watcher on `.dl` or expert
declaration file change):
1. Re-parse the changed file
2. Hard error on syntax errors (surfaced via `%OntologyReloadFailed{}`
   event; existing fact base unchanged)
3. Retract base facts from the old version of the file
4. Assert base facts from the new version
5. Clear derived facts
6. Re-evaluate
7. Regenerate Obsidian views (§6)
8. Emit `%OntologyLoaded{}` event

Consumers see updated results on their next query — no consumer restart
needed.

### §2.7 Size estimate

~300-500 lines for the evaluator (pending literature survey outcome),
~150-200 for the parser, ~100 for ETS storage, ~100 for the GenServer
wrapper. Total: ~650-900 lines. Fully understandable in one LLM context
window.

---

## §3 Ontology fact schema

### §3.1 Core predicates (base facts — user-authored)

```
% Identity: declare a canonical tag
tag(rust).
tag(elixir).
tag(ffi).
tag(systems_programming).

% Synonyms: alternative names for the same concept
synonym(rustlang, rust).
synonym(rust_lang, rust).
synonym(foreign_function_interface, ffi).

% Hierarchy: parent-child taxonomy
parent(rust, systems_programming).
parent(elixir, functional_programming).
parent(ffi, interop).

% Typed relations: domain-specific connections (open set)
bridges(ffi, rust).
bridges(ffi, c).
uses(rust, llvm).
requires(unsafe_rust, ffi).
related(ownership, lifetimes).
```

Users define new relation predicates freely. The core rules (§3.2)
depend on a small set of well-known predicates (`tag`, `synonym`,
`parent`); user-defined relations are consumed by user-authored rules
and routing rules.

### §3.2 Derived predicates (rules — ship with Ravel, user-extensible)

```
% Transitive ancestry
ancestor(X, Y) :- parent(X, Y).
ancestor(X, Y) :- parent(X, Z), ancestor(Z, Y).

% Canonical resolution: any term resolves to its canonical tag
resolves_to(X, X) :- tag(X), not synonym(X, _).
resolves_to(X, Canonical) :- synonym(X, Canonical).

% Scope matching: an expert is relevant to a term via multiple paths
relevant_expert(Expert, Term) :-
    expert_tag(Expert, Tag), resolves_to(Term, Resolved),
    Tag = Resolved.
relevant_expert(Expert, Term) :-
    expert_tag(Expert, Tag), resolves_to(Term, Resolved),
    ancestor(Resolved, Ancestor), expert_tag(Expert, Ancestor).
relevant_expert(Expert, Term) :-
    expert_tag(Expert, Tag), resolves_to(Term, Resolved),
    bridges(Resolved, Tag).
relevant_expert(Expert, Term) :-
    expert_tag(Expert, Tag), resolves_to(Term, Resolved),
    related(Resolved, Tag).
```

The `expert_tag/2` facts are asserted automatically from expert
declaration frontmatter by `DeclarationLoader` (§4.1). They are not
hand-authored in `.dl` files.

### §3.3 Naming convention bridge

Datalog atoms use `snake_case` (standard for logic programming). Expert
declaration YAML frontmatter uses `kebab-case` (Obsidian convention).
The bridge is mechanical: `DeclarationLoader` converts `kebab-case` tags
to `snake_case` when asserting `expert_tag/2` facts. Obsidian view
generation (§6) converts back to `kebab-case` for display. One canonical
direction; no ambiguity.

### §3.4 Open relation set

The typed relations (`bridges`, `uses`, `requires`, `related`) are not a
closed set. Users define new predicates freely — they are just Datalog
predicates. The shipped rules (§3.2) only depend on `tag`, `synonym`,
`parent`, `expert_tag`, and the derived `resolves_to`, `ancestor`,
`relevant_expert`. User-authored rules can use any predicates.

---

## §4 Consumer integration

### §4.1 ScopeMatcher (Sub-N)

**Current v1:** `Ravel.Expert.ScopeMatcher.match_candidate/2` does
exact-string set intersection between candidate tags and expert tags.

**With Sub-R:** The implementation changes from set intersection to a
Datalog query. The function signature and return type are unchanged:

```elixir
@spec match_candidate(
  candidate_tags :: [String.t()],
  vault_experts :: [Declaration.t()]
) :: {:matched, [String.t()]} | :orphan
```

Internally:
1. Assert temporary `candidate_tag(Tag)` facts for each candidate tag
   (kebab-to-snake converted)
2. Query `relevant_expert(Expert, Term) :- candidate_tag(Term), ...`
3. Retract the temporary facts
4. Return matched expert IDs or `:orphan`

**Value delivered:** A candidate tagged `ffi` now matches `rust-expert`
via `bridges(ffi, rust)`. A candidate tagged `rustlang` matches
`rust-expert` via synonym resolution. Neither is possible with
exact-string matching.

### §4.2 Stage 2 classification (Sub-E)

**Current v1:** Stop-word-filtered lowercase nouns from the section
title, intersected with the flat tag set read from
`<vault>/experts/*.md`.

**With Sub-R:** Instead of flat intersection, Stage 2 queries:
1. Extract candidate terms from the section title (unchanged heuristic)
2. For each term, query `resolves_to(Term, Canonical)` to normalize
   synonyms
3. Optionally query `ancestor(Canonical, Broader)` to include broader
   tags when the candidate term is highly specific
4. Return the canonical tag set as the candidate's `tags:` frontmatter

This remains deterministic Elixir — the Datalog query is a function
call, not an LLM pass. Stays fast.

### §4.3 Fact extraction normalization (Sub-F)

**Current v1:** LLM extracts 1-5 lowercase keywords from a message
body. These raw keywords pass directly to routing.

**With Sub-R:** After the LLM extracts keywords, a normalization pass
queries the Datalog engine:
1. For each extracted keyword, query `resolves_to(Keyword, Canonical)`
2. Unresolved keywords pass through unchanged (the ontology does not
   need to know every possible term)
3. Normalized keywords feed into routing queries

This catches the case where the LLM extracts `"rust_lang"` or
`"foreign_function_interface"` — both resolve to their canonical forms
before routing sees them.

### §4.4 Routing (Sub-F amendment)

**Current v1:** `routing.ex` — user-editable Elixir pattern-matched
`route/2` clauses with BEAM hot code reload.

**With Sub-R:** `ontology/routing.dl` — user-editable Datalog rules
loaded by `Ravel.Datalog.Server.load_file/2`. Hot reload via
the same filesystem-watcher-triggered re-parse + re-evaluate cycle as
all other `.dl` files.

**Routing query pattern:**

```
% User-authored routing rules in routing.dl
route_to_expert(Expert, Facts) :-
    member(Fact, Facts), resolves_to(Fact, Resolved),
    relevant_expert(Expert, Resolved).

% Specific overrides
route_override(sub_ffi_callbacks, Facts) :-
    member(callback_registration, Facts),
    member(gc_protect, Facts).
```

**Design note on `member/2`:** Routing needs to query over a list of
extracted facts. Two options: (a) `member/2` as a built-in predicate, or
(b) assert extracted facts as temporary per-message facts (matching the
ScopeMatcher pattern in §4.1). The literature survey (§8.1) may inform
which is cleaner; both are straightforward.

**Sub-F amendment scope:**

| Changed | Detail |
|---------|--------|
| `Ravel.Router.evaluate_rules/2` | Calls Datalog query instead of compiled Elixir module |
| Routing file path | `routing.ex` → `ontology/routing.dl` |
| Hot reload mechanism | BEAM code reload → fact-base reload |
| Level 2 agent prompt | "Propose a Datalog rule" instead of "propose an Elixir clause" |
| Rule staging | Proposed rules as `.dl` fragments in `<vault>/runtime/staging/proposed-rules/` |

| Unchanged | Detail |
|-----------|--------|
| Dispatch/query resolution flow | Entire message routing architecture |
| Level 2 routing agent | Still the LLM fallback when rules don't decide |
| Fact extraction LLM pass | Still extracts keywords; feeds into Datalog query |
| Dispatch asymmetry | Same-project direct, cross-project via Level 2 |
| Audit trail | All routing decisions logged |

**New Sub-F dependency:** `Ravel.Router` depends on
`Ravel.Datalog.Server` in the supervision tree. Startup-order
dependency handled by supervisor child spec ordering.

### §4.5 Shared fact base lifecycle

The `Ravel.Datalog.Server` GenServer is a singleton in the daemon's
supervision tree. All consumers query the same fact base. Expert
declaration changes trigger re-assertion of `expert_tag/2` facts and
re-evaluation. Ontology file changes trigger file-level reload and
re-evaluation. Consumer modules hold no local cache of ontology state —
they query the server on every use.

---

## §5 LLM exposure

### §5.1 Passive: context injection

A new placeholder `{{ONTOLOGY_CONTEXT}}` is available in phase prompts
(alongside `{{VAULT_CATALOG}}`). The injected content is the relevant
subset of the fact base, not the entire ontology.

**Scope rules for injection:**
- **Expert sessions:** All facts reachable from the expert's declared
  tags — synonyms, hierarchy, relations.
- **Plan sessions:** All facts reachable from the plan's recent
  fact-extraction keywords.
- **Routing agent sessions:** Full routing rule set plus the relevant
  tag neighborhood for the message being routed.

**Injection format:** Readable Datalog syntax, directly from the source
files filtered to the relevant subset:

```
% Ontology context for rust-expert
tag(rust). tag(ffi). tag(ownership). tag(lifetimes).
synonym(rustlang, rust). synonym(rust_lang, rust).
parent(rust, systems_programming).
bridges(ffi, rust). bridges(ffi, c).
related(ownership, lifetimes).
requires(unsafe_rust, ffi).
```

Small token budget. Gives the LLM grounded knowledge about concept
relationships without it having to infer them.

### §5.2 Active: query tool

For sessions spawned via Sub-C's harness adapter, the ontology is
exposed as a tool call:

```json
{
  "tool": "ontology.query",
  "input": {
    "query": "relevant_expert(Expert, ffi)"
  }
}
```

Returns:

```json
{
  "results": [
    {"Expert": "rust_expert"},
    {"Expert": "ffi_expert"}
  ]
}
```

The LLM can ask "what concepts relate to X?" or "which experts know
about Y?" during reasoning.

**Tool profile gating:** `ontology.query` is available in plan phase
sessions and expert consultation sessions. Internal daemon sessions
(fact extraction, routing) do not get it — they are too cheap/fast
for tool-call overhead.

### §5.3 Rule proposal

When the Level 2 routing agent encounters a novel case, it proposes a
new Datalog rule:

```
% Proposed by routing agent, 2026-04-16
% Reason: dispatch about callback registration + GC protection
%   consistently targets sub-ffi-callbacks plan
route_override(sub_ffi_callbacks, Facts) :-
    member(callback_registration, Facts),
    member(gc_protect, Facts).
```

Written to `<vault>/runtime/staging/proposed-rules/`. User reviews in
Obsidian or TUI and accepts (appends to `routing.dl`) or rejects. The
Datalog syntax is self-documenting and qualitatively easier to review
than Elixir `defp` clauses.

---

## §6 Obsidian views

### §6.1 Auto-generated views

The daemon regenerates Markdown views from the fact base on every
ontology change (same pattern as `plan-catalog.md`):

```
<vault>/
├── ontology/
│   ├── vocabulary.dl         # source of truth
│   ├── relations.dl          #   "
│   ├── rules.dl              #   "
│   ├── routing.dl            #   "
│   └── views/                # auto-generated, never hand-edited
│       ├── tag-index.md
│       ├── taxonomy.md
│       ├── relations.md
│       └── routing-rules.md
```

**`tag-index.md`** — all canonical tags with synonyms, relations, and
expert associations:

```markdown
---
generated: true
generator: ravel-ontology
---
# Tag Index

## rust
- **Synonyms:** rustlang, rust-lang
- **Parent:** systems-programming
- **Relations:** bridges(ffi, rust), uses(rust, llvm)
- **Experts:** [[rust-expert]]

## ffi
- **Synonyms:** foreign-function-interface
- **Parent:** interop
- **Relations:** bridges(ffi, rust), bridges(ffi, c)
- **Experts:** [[rust-expert]], [[ffi-expert]]
```

Wikilinks to expert declarations make Obsidian's backlinks useful for
navigation.

**`taxonomy.md`** — the hierarchy as an indented tree:

```markdown
# Taxonomy

- programming
  - systems-programming
    - rust
    - c
  - functional-programming
    - elixir
    - racket
- interop
  - ffi
```

**`relations.md`** — typed relations as tables:

```markdown
# Relations

## bridges
| From | To |
|------|-----|
| ffi | rust |
| ffi | c |

## uses
| From | To |
|------|-----|
| rust | llvm |
```

**`routing-rules.md`** — current routing rules in human-readable form,
with any pending proposals from `runtime/staging/proposed-rules/` shown
separately.

### §6.2 Display convention

Obsidian views display tags in `kebab-case` (Obsidian convention).
Internal Datalog atoms are `snake_case`. The view generator performs the
conversion. This is the reverse of the `DeclarationLoader` bridge (§3.3)
— the two conversions are inverses.

### §6.3 Phoenix future

Auto-generated Markdown views are explicitly a bridge. Phoenix LiveView
(post-v1) replaces them with an interactive ontology browser — graph
visualization, click-to-navigate, inline editing, query playground. The
`.dl` source files and the Datalog engine are unchanged; only the view
layer swaps.

---

## §7 Sub-F amendment and migration

### §7.1 Amendment scope

Sub-F's design doc specifies `routing.ex` as user-editable Elixir with
pattern-matched `defp route/2` clauses. Sub-R replaces this with
Datalog-based routing.

**Changed in Sub-F:**
- `Ravel.Router.evaluate_rules/2` calls Datalog query
- `routing.ex` → `ontology/routing.dl`
- Hot reload: BEAM code reload → fact-base reload
- Level 2 agent prompt: propose Datalog rules, not Elixir clauses
- Rule staging: `.dl` fragments in `<vault>/runtime/staging/proposed-rules/`

**Unchanged in Sub-F:**
- Dispatch/query resolution flow
- Level 2 routing agent existence and role
- Fact extraction LLM pass
- Same-project vs cross-project dispatch asymmetry
- Audit trail
- All message types and target variants

### §7.2 Migration path

V1 has not shipped. There are no existing `routing.ex` files to migrate.
The migration is purely a design-doc amendment — Sub-F implementation
tasks referencing `routing.ex` are rewritten to reference
`ontology/routing.dl`. No runtime migration tooling needed.

### §7.3 Cross-sub-project requirements

Sub-R imposes adoption work on siblings. Per the brainstorming
methodology, adoption stubs are pushed to sibling backlogs:

| Sibling | Adoption work |
|---------|--------------|
| **Sub-F** | Amendment: replace `routing.ex` evaluation with Datalog query in `Router`; update Level 2 agent prompt; update rule staging path |
| **Sub-N** | `ScopeMatcher.match_candidate/2` implementation swaps from set intersection to Datalog query. Interface unchanged. |
| **Sub-E** | Stage 2 classification queries `resolves_to/2` and optionally `ancestor/2` instead of flat tag-set intersection. |
| **Sub-A** | Vault layout: `tag-vocabulary.md` reservation replaced by `ontology/` directory. Schema version bump if layout change is breaking. |
| **Sub-C** | Add `ontology.query` to tool profiles for plan/expert sessions. Internal sessions excluded. |
| **Sub-H** | Routing rule review may get a dedicated view in TUI output pane. No new commands — ontology files edited via `$EDITOR`. |
| **Sub-M** | New event structs: `%OntologyLoaded{}`, `%OntologyReloadFailed{}`, `%RoutingRuleProposed{}`. Added to sealed event set. |
| **Sub-I** | Document `ontology/views/` as auto-generated Markdown surfaces in data-surface inventory. |
| **Sub-G** | No change. `ontology/` is new; no rename needed. |

---

## §8 Research requirements

### §8.1 Datalog evaluation strategy survey

**Goal:** Identify the best evaluation algorithm for Ravel's query
patterns — small fact base (<10k facts initially), frequent point
queries, infrequent bulk re-evaluation, BEAM runtime.

**Literature to survey:**
- **Semi-naive evaluation** (Ullman 1988) — baseline. Bottom-up with
  delta optimization.
- **Magic sets** (Bancilhon et al. 1986) — transforms bottom-up to
  simulate top-down goal-directedness. Valuable if most queries are
  point queries (which ours are).
- **Demand-driven / top-down with tabling** (XSB Prolog approach) —
  evaluates only what is needed for the current query.
- **Incremental maintenance** (DRed — Gurevich et al. 1993;
  Differential Dataflow — McSherry et al. 2013) — updates derived facts
  incrementally on fact changes.
- **Trie-based indexing** (Ramakrishnan et al.) — efficient storage and
  join ordering.
- **Modern implementations** — Soufflé, Crepe, BottomUp — for
  implementation patterns.

**Output:** A section in the implementation plan naming the chosen
strategy with rationale.

### §8.2 Ontology design literature

**Goal:** Ground the fact schema in established knowledge representation
practice.

**Literature to survey:**
- **SKOS** (Simple Knowledge Organization System, W3C 2009) —
  `broader`/`narrower`/`related`/`altLabel` map closely to our
  `parent`/`synonym`/`related`.
- **OWL** (Web Ontology Language) — heavyweight; instructive for
  understanding typed-relation expressiveness.
- **CYC microtheory architecture** — how CYC partitions knowledge into
  context-scoped subsets. Relevant to Ravel's tiered scoping.
- **Folksonomy research** (Gruber 2007; Mika 2007) — how tagging
  systems evolve into structured vocabularies.
- **Luhmann's Zettelkasten** — already cited in `research-sources.md`.
  Revisit for index-term-to-ontology evolution.

**Output:** Additions to `docs/research-sources.md` and any schema
adjustments the literature suggests.

### §8.3 Datalog on BEAM audit

**Goal:** Confirm "build custom" is the right call per "prefer
integration over reinvention."

**Projects to audit:**
- **Erlog** (Robert Virding) — Prolog in Erlang. Already evaluated;
  confirm maintenance status.
- **`ascent`** — Rust Datalog. Not BEAM-native; implementation patterns
  only.
- **Hex.pm search** — any Elixir/Erlang Datalog packages.
- **ETS performance** — characteristics for our access patterns (point
  queries, small tables, concurrent reads).

**Output:** "Why build custom" justification or adoption recommendation.

---

## §9 Testing strategy

### §9.1 Engine tests (unit — no Ravel dependencies)

The engine module has zero Ravel-specific knowledge. Tests are pure
Datalog:

- **Parser tests.** Valid syntax, syntax errors with line numbers, edge
  cases (empty files, comments only, Unicode atoms).
- **Evaluation tests.** Fixed-point convergence, transitive closure,
  stratified negation, recursive rules, multiple rule definitions for
  the same predicate.
- **Termination tests.** Property-based testing with randomly generated
  fact bases and rules (StreamData). Verify evaluation always
  terminates.
- **Query tests.** Variable binding, multi-variable queries, no-result
  queries, ground queries (boolean check).
- **Stratification tests.** Valid stratification accepted,
  unstratifiable programs rejected with clear error.

These tests run without ETS, without GenServers, without the daemon.
Pure function in, result out.

### §9.2 Ontology tests (integration — Ravel's fact schema)

Test the shipped predicates and rules against known scenarios:

- **Synonym resolution.** `rustlang` resolves to `rust`; unknown terms
  resolve to themselves.
- **Hierarchy traversal.** `rust` is a child of `systems_programming`;
  querying `ancestor(rust, X)` returns `systems_programming`,
  `programming`.
- **Typed relations.** `bridges(ffi, rust)` makes `rust_expert`
  relevant for an `ffi` query.
- **Expert matching.** Given the six default expert declarations, verify
  representative candidates route to expected experts.
- **Orphan detection.** Candidate with tags matching no expert (after
  full resolution) returns `:orphan`.

### §9.3 Consumer integration tests

One test per consumer:

- **ScopeMatcher.** Candidate tagged `rustlang` matches `rust_expert`
  (synonym resolution). Candidate tagged `ffi` matches both
  `rust_expert` and `ffi_expert` (typed-relation resolution).
- **Stage 2.** Section titled "Rust FFI callbacks" produces canonical
  tags `[rust, ffi]`.
- **Routing.** Extracted facts `["rust", "ffi"]` route to `rust_expert`
  via Datalog rules.
- **Ontology reload.** Change a `.dl` file, verify consumers see
  updated results without restart.

### §9.4 What is not tested here

- LLM behavior with ontology context injection (cannot unit-test LLM
  reasoning).
- Phoenix ontology browser (v2).
- Performance at scale beyond v1 targets.

---

## §10 Vault layout changes

### §10.1 New directory

```
<vault>/
├── ontology/                          # NEW — Sub-R
│   ├── vocabulary.dl                  # canonical tags, synonyms, hierarchy
│   ├── relations.dl                   # typed relations between concepts
│   ├── rules.dl                       # derived predicates (ships with Ravel)
│   ├── routing.dl                     # routing rules (replaces routing.ex)
│   └── views/                         # auto-generated Obsidian views
│       ├── tag-index.md
│       ├── taxonomy.md
│       ├── relations.md
│       └── routing-rules.md
├── routing.ex                         # DELETED — replaced by ontology/routing.dl
├── runtime/
│   └── staging/
│       └── proposed-rules/            # NEW — routing agent proposals
```

### §10.2 Sub-A impact

Sub-A's vault layout has a reserved `tag-vocabulary.md` path (currently
unused). Sub-R replaces this single-file reservation with the
`ontology/` directory. Sub-A's design doc §A4 layout section needs an
inline rewrite: remove `tag-vocabulary.md`, add `ontology/` with its
contents. `ravel.toml` schema version bump if the layout change is
considered breaking.

---

## §11 Hard error policy

Consistent with the project-wide convention:

| Condition | Behavior |
|-----------|----------|
| Syntax error in any `.dl` file | Hard error; daemon refuses to start (startup) or refuses reload (runtime) |
| Unstratifiable rule set | Hard error |
| Range-restriction violation | Hard error at parse time |
| Undefined predicate after all files loaded | Hard error |
| Tag format violation (non-lowercase, non-underscore) | Hard error |
| Duplicate facts | Silently deduplicated (natural when multiple files overlap) |
| `.dl` file missing from expected `ontology/` set | Hard error at startup |
| Expert declaration with unparseable tags | Hard error (existing sub-N behavior, unchanged) |

---

## Appendix A — Decision Trail

### Q1. What scale of ontology?

Rich ontology with typed relations — option (C) in the brainstorm.
Hierarchy alone (option B) doesn't capture cross-domain connections like
`bridges(ffi, rust)` that are load-bearing for scope matching. The user
has AI/KR background from the 1980s and sees this as a path toward
exposing CYC-like structured knowledge to LLMs. The typed-relation set
is open — users define new predicates freely.

### Q2. Does Sub-R own the Datalog engine or just the ontology?

Sub-R owns both. The engine and ontology are co-designed — the fact
schema shapes what queries are expressible, and vice versa. Splitting
them would force premature interface decisions. Sub-R is "Ravel's
declarative reasoning substrate."

### Q3. What Datalog engine?

Purpose-built minimal Datalog in Elixir, not Erlog. Erlog is Prolog,
not Datalog — bolting safety (guaranteed termination) onto an unsafe
substrate is harder than building a clean Datalog from scratch.
Semi-naive bottom-up evaluation is the baseline reference; the specific
strategy is informed by a literature survey (§8.1). No suitable
BEAM-native Datalog library exists at the quality level needed (to be
confirmed by §8.3 audit).

### Q4. How is the ontology stored?

Datalog `.dl` files as source of truth, with auto-generated Obsidian
Markdown views. Users edit `.dl` directly; Obsidian views are for
browsing. Phoenix LiveView replaces the views post-v1. The `.dl` format
is the long-lived artifact — expressive from day one, consumed by
progressively smarter resolvers.

### Q5. Does the Datalog engine replace `routing.ex` in v1?

Yes — v1 replacement, not parallel path. Having two reasoning substrates
(Elixir pattern matching + Datalog) is conceptual overhead. Routing is
actually a simpler Datalog consumer than ontology resolution, so it is a
good proving ground. The Sub-F amendment is well-scoped: swap the rule
evaluation backend, keep everything else.

### Q6. Why Datalog over Elixir pattern matching for user-facing rules?

Datalog is fundamentally better than imperative Elixir code for
exposing reasoning to users. Users can understand
`parent(rust, systems_programming)` and
`synonym(rustlang, rust)` more naturally than `defp route/2` clauses.
LLMs can reason over Datalog facts in context. The learning loop
(Level 2 agent proposing rules) produces more reviewable output.
User input: "datalog is fundamentally better than imperative elixir
code for exposing to users."

### Q7. Research before implementation?

Yes. Three literature surveys: (1) Datalog evaluation strategies —
semi-naive is baseline but the implementation should make an informed
choice; (2) ontology design literature — SKOS, OWL, CYC microtheories,
folksonomy research; (3) existing BEAM Datalog implementations — confirm
"build custom" is justified per "prefer integration over reinvention."
User input: "there has been a lot of work on efficient datalog
evaluation, so we should add a research task to first do a literature
search."
