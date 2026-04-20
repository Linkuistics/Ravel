# Memory — Sub-project N: Domain Experts (ExpertActor internals)

This plan implements sub-project N of the Ravel orchestrator merge.
N's brainstorm is complete; this plan is the implementation work, not
a design phase. The design is fully specified in the spec referenced
below. If an implementation question arises that the spec does not
answer, the answer goes into this memory file (and possibly back into
the spec) rather than being invented ad hoc.

## Primary reference

**`{{PROJECT}}/docs/superpowers/specs/2026-04-15-sub-N-domain-experts-design.md`**
is the authoritative design document. Every task in this plan's backlog
derives from the spec's §1–§11. Consult §1–§7 of the spec before acting
on any implementation task. Appendix A documents the decision trail;
Appendix B is the `mix.exs` projection; Appendix C is the glossary.

## Parent plan

The orchestrator-level plan lives at
`{{PROJECT}}/LLM_STATE/ravel-orchestrator/` (currently — will move
to `{{PROJECT}}/ravel/project-root/` after sub-project G's
migration lands). It coordinates this sub-plan with its siblings. The
parent plan's `memory.md` holds cross-sub-project architectural state.
This file holds only sub-project-N-specific implementation state.

## Key architectural anchors (quick reference; spec is canonical)

These are the decisions most load-bearing for implementation. Consult
§1–§11 of the design doc for full context before acting on any of them.

### N-1. ExpertActor fills F's type hole

Sub-N delivers `Ravel.ExpertActor` as a GenServer implementing F's
`Ravel.Actor` behaviour, replacing F's stub that returns
`{:rejected, :not_yet_implemented, "see sub-N-domain-experts"}`. One
actor per expert declaration file; actor lives in F's existing
`ActorSupervisor` (`dynamic_supervisor, restart: :transient`). Sub-N
does NOT introduce a new supervisor branch. See spec §2.2 and §7.1.

### N-2. Declaration format: YAML frontmatter + markdown persona body

Every expert lives at `<vault>/experts/<expert-id>.md`. YAML frontmatter
carries machine-readable fields (description, kind, schema-version,
tags, scope.tier1/tier2, retrieval.*, model, dialogue.*). Markdown body
is injected verbatim as the session system prompt, hard-capped at 8 KB.
Description ≤120 chars. Reserved IDs: `uncategorized`, `vault`, `plan`,
`daemon`, `router`, `ravel`. See spec §3.

### N-3. Knowledge scope spans both tiers with explicit per-project opt-in

Q1 of the brainstorm: experts span both Tier 1 (per-project
`<project>/ravel/knowledge/`) and Tier 2 (global
`<vault>/knowledge/`) via explicit `scope.tier1` and `scope.tier2`
fields. A literal `*` in the first segment of a `tier1` glob means "any
project" — there is no implicit project inclusion. Grep
`<vault>/experts/*.md` for `tier1:` to audit cross-project scope. See
spec §4.5.

### N-4. Hybrid ownership: experts own Tier 2, Tier 1 is plan-read-only

Q3 of the brainstorm: experts have exclusive write authority over their
Tier 2 dedicated directories (resolved from the first entry of
`scope.tier2`). Tier 1 is plan-owned from sub-N's perspective: experts
can read it but never write to it. Stage 5 candidates that land in
Tier 1 are plan-owned writes (sub-E handles directly); Tier 2
candidates flow through experts. See spec §6.2 and Appendix A Q3.

### N-5. Keyword + section-aware retrieval behind a behaviour chokepoint

Q4 of the brainstorm: `Ravel.ExpertRetrieval` is an `@behaviour`
with exactly one v1 implementor, `KeywordSectionRetrieval`, using
ripgrep + section-aware scoring. Weights are **module-level constants**,
not per-expert config. Future strategies (semantic via vector store —
deferred to sub-Q) plug in behind the chokepoint. See spec §4.

### N-6. Two explicit timers: retrieval vs session

**Retrieval timeout 5s** is a structural sanity check on the ripgrep
pipeline (not a reasoning budget). **Per-turn session timeout 5 min
default** (configurable in `daemon.toml` under `[experts]
turn_timeout_seconds`) bounds actual LLM reasoning. Dialogue TTL is a
30-minute idle timeout that resets on every successful turn. These are
three distinct concepts; do not collapse them. See spec §5.4.

### N-7. Stateless dialogue turns over ETS-backed registry

Q2 of the brainstorm: dialogue state lives in an ETS-backed singleton
`Ravel.Expert.DialogueRegistry` under `Ravel.Supervisor`. Every
turn spawns a brand-new fresh-context session; the actor does NOT hold
a long-lived harness across turns. TTL reaper sweeps expired entries.
Daemon restart wipes the table — consumers get
`:dialogue_not_found_or_expired` and fall back to a fresh `ask_expert`.
Audit trail lives in sub-M's event log, NOT in a parallel dialogue
file. See spec §2.5 and §5.3.

### N-8. Tag-based scope matching, exact-string only in v1

Q6 of the brainstorm: Stage 5 scope matching is pure set-intersection
on frontmatter `tags:` fields. **Exact-string matching only** — `rust`
≠ `rustlang`, no stemming, no case-insensitivity. Orphan candidates
(zero matching experts) bypass the expert fan-out and write directly
to `<vault>/knowledge/uncategorized/` with an `%OrphanCandidate{}`
event (owned by sub-E's event namespace). The ontology sub-project
(sub-R, research task) will eventually replace this with a richer
resolver behind the same `ScopeMatcher` interface. See spec §6.1.

### N-9. Parallel fan-out for Stage 5 with each expert writing its own file

Q5 of the brainstorm: Stage 5 dispatches `%ExpertAbsorbCandidate{}`
messages in parallel to every tag-matching expert, and each expert that
returns `READY ABSORB` writes the file into its own Tier 2 directory
itself. Physical duplication across directories is accepted — two
experts absorbing the same candidate produce two files with matching
`ingestion-event-id` in their provenance frontmatter. Wikilinks between
them are NOT auto-inserted by sub-N v1. The `READY CROSS_LINK` verdict
is a rejection-with-suggestion; the collector second-round dispatches
non-recursively (max depth 2). Contentful disagreement (one `:absorb` +
one `:reject` with non-trivial reason) emits `%ExpertConflict{}`. See
spec §6.

### N-10. Dual-tool surface: `ask_expert` + `reply_to_expert`

Sub-N adds two injected tools to sub-C's harness session tool set
(§4.5 of C's spec). `ask_expert(target, question, dialogue_id?)` for
fresh queries and optionally continuations; `reply_to_expert(id, reply)`
for follow-up turns where the target is implicit in the dialogue_id.
Distinct tools for tool-call trace readability; semantically
equivalent. Tool results are JSON-stringified objects with a `kind`
discriminator (`answer` / `clarifying_question` / `error`). See spec
§5.1.

### N-11. Sentinel matching for disposition detection

Sub-N reuses sub-B's `Sentinel.SlidingBuffer` implementation. Query
sentinels: `READY WITH ANSWER` or `READY WITH CLARIFICATION`. Ingestion
sentinels: `READY ABSORB`, `READY REJECT <reason>`, `READY CROSS_LINK
<expert-id>`. Missing sentinel surfaces as
`%Expert.SentinelMissing{}` and returns `{:error, :malformed_response}`.
See spec §5.5 and §6.2.

### N-12. Hard-error discipline with one documented exception

Every failure mode in §7.3's disposition matrix either emits an event
OR surfaces as a tool-call error with a machine-readable code. No
silent fall-through to a less-functional mode. **One documented
exception**: `KeywordSection` retrieval's per-file 64 KB read cap
truncates larger files silently (with a `%ExpertFileTruncated{}` event)
rather than rejecting them. Rationale: bounding expert sessions to
reasonable sizes matters more than "no silent truncation," and the
event gives humans a path to find over-large files. See spec §7.4.

### N-13. Default expert set ships in `priv/experts/`

Six starter declarations: `rust-expert`, `elixir-expert`,
`research-expert`, `software-architect`, `obsidian-expert`,
`ffi-expert`. The `elixir-expert` replaces the orchestrator memory's
earlier `distributed-systems-expert` for dogfooding reasons — Ravel
is Elixir-on-BEAM, and elixir-expert gets matched every session.
`ravel init` copies these into `<vault>/experts/` at init time.
Users can delete, rename, or replace any default after init. See spec
§3.5.

## Contract with sibling sub-projects

Sub-N depends on these interfaces from siblings — not yet landed at
sibling-plan scaffolding time but committed by the relevant sub-project
specs:

- **F**: `Ravel.Actor` behaviour, `ActorSupervisor` child-spec
  registration, `Router` accepting sub-N's three message structs,
  five-state actor lifecycle, dispatch asymmetry.
- **C**: `Ravel.HarnessAdapter` behaviour, session GenServer
  attach-consumer pattern, `FixtureReplay` adapter for Layer 2 tests,
  tool-call boundary with injected tool set, `--setting-sources
  project,local --no-session-persistence` cmux mitigation.
- **B**: `Sentinel.SlidingBuffer` module (consumed directly).
- **M**: `Ravel.Observability.emit/1` + sealed event set (sub-N
  adds its own `Ravel.Event.Expert.*` group per spec §9.5).
- **A**: `<vault>/experts/` as tracked, `<vault>/runtime/snapshots/`
  as gitignored, `<vault>/knowledge/uncategorized/` as an init-time
  created directory.

Sub-N commits deliverables back to:

- **F**: `%Ravel.Message.ExpertQuery{}`,
  `%Ravel.Message.ExpertDialogueReply{}`,
  `%Ravel.Message.ExpertAbsorbCandidate{}` structs +
  `Ravel.ExpertActor` replacing F's stub.
- **E**: `Ravel.Expert.ScopeMatcher.match_candidate/2` +
  verdict surface (`:absorb` / `:reject` / `:cross_link_suggested`)
  + `%ExpertAbsorbCandidate{}` message shape. These can ship as an
  early deliverable PR so sub-E's amendment task can code against
  real interfaces.
- **M**: the full `Ravel.Event.Expert.*` struct set +
  corresponding metric definitions contributed to sub-M's §6
  catalogue.
- **A**: `priv/experts/*.md` for the init flow to copy; an
  additional init-time directory `<vault>/knowledge/uncategorized/`.

## Non-goals (v1)

- **Vector-store / semantic retrieval** — deferred to new sub-project
  sub-Q (vector-store infrastructure). V1 ships keyword-only.
- **Tag ontology / vocabulary enforcement** — deferred to new
  sub-project sub-R (knowledge ontology, research task). V1 uses
  exact-string tag matching.
- **Per-expert model override** — `model:` field parsed but unused; sub-O
  consumes.
- **Expert-to-expert dispatch** — experts can suggest cross-links but
  cannot invoke `ask_expert` themselves. V1.5 may revisit.
- **Cross-vault expert consultation** — single-vault per F.
- **Dynamic expert creation via TUI** — file-editing only in v1;
  sub-H owns any TUI surface.
- **Expert quality metrics** — not scoped for v1.
- **Automatic wikilink insertion between multi-absorb files** —
  accepted as manual/triage work.
- **Query-time expert arbitration** — no "ask a panel" aggregation;
  parallel fan-out is only for ingestion. See spec §1 out-of-scope.

## Bootstrap discipline

This sibling plan runs on LLM_CONTEXT's existing four-phase cycle
machinery (work → reflect → compact → triage) during the bootstrap
period. The daemon does not exist yet; the "ExpertActor implementation"
work runs in fresh LLM_CONTEXT sessions, not inside a running
Ravel daemon. All tasks are code, tests, and specs that will later
be executed by the daemon once sub-F + sub-G ship.

### N-14. Mandatory FileIO layer for all expert file operations

`Ravel.FileIO` is the mandatory I/O layer for all daemon file
operations; raw `File.*` calls inside `ExpertActor` or
`DeclarationLoader` are forbidden. Two relevant entry points:

- **`FileIO.read_tracked/1`** — use for all reads (records SHA-256
  hash; subsequent `write_safe` can detect human edits since the
  last read).
- **`FileIO.write_safe(path, content, category)`** — use for all
  writes; handles atomic temp-then-rename internally.

Category for expert-absorbed knowledge files (Task 20 absorb path):
**`:human_editable`** — human-authoritative. If the human has
edited the target file since the expert read it via `read_tracked`,
`write_safe` discards the expert's new content silently (no error,
no merge). This is the correct behaviour for knowledge files the
user may curate manually.

Category for expert declaration files (`<vault>/experts/*.md`):
**`:human_editable`** — user-authored, same rationale.

Do not invent a bare `File.write/2` + manual rename anywhere in
sub-N. If sub-D has not yet shipped `FileIO` when a sub-N task
starts, block on Task 0 or coordinate via the orchestrator.

### N-15. Daemon socket protocol: `expert.query` is the only expert command

Confirmed in sub-H brainstorm (Session 21, 2026-04-16): the daemon NDJSON
socket protocol has exactly **one** expert-related command: `expert.query`.
This command maps directly to sub-N's `%Ravel.Message.ExpertQuery{}`
dispatch path — the TUI sends `expert.query` → daemon routes to the target
`ExpertActor` → response flows back over the socket.

There are **no** NDJSON commands for expert CRUD (create / edit / delete).
Expert management is a filesystem + `$EDITOR` operation handled entirely by
the TUI via the `DeclarationLoader` hot-reload path. The daemon receives
file-system events (via the `FileSystem` watcher in `DeclarationLoader`) and
reacts; it does not expose a "create expert" or "delete expert" socket
command.

`expert.list` is TUI-local: the TUI reads `<vault>/experts/*.md` directly
from the filesystem without a daemon round-trip.

Implications for sub-N:
- No new NDJSON command structs are needed in sub-N beyond the three message
  structs already defined in Task 12 (`ExpertQuery`, `ExpertDialogueReply`,
  `ExpertAbsorbCandidate`).
- `DeclarationLoader`'s file-watcher (Task 14 / Task 21) is the **sole**
  mechanism by which the daemon learns about new, edited, or deleted expert
  declarations. No socket-layer handler for declaration mutations is needed.

## Hard errors by default

Per project-wide discipline, fail hard on unexpected conditions,
invariant violations, I/O failures, and ambiguous state with clear
diagnostics. Documented exceptions (the §7.4 64 KB per-file retrieval
truncation) must name rationale in the design doc. Retrieval pipeline
errors, declaration validation failures, sentinel misses, session
timeouts, and dialogue lookup failures all follow this pattern.

## Amendment / spec drift discipline

If downstream interfaces in sub-F, sub-B, sub-C, sub-M, or sub-A drift
before sub-N implementation lands, the design doc is rewritten inline
per the project's pivot-rewrite feedback memory — not appended as a
supersede layer. Cross-sub-project contract changes reflect back into
the spec's §9 and appropriate anchors in this memory file get
re-stated accordingly.
