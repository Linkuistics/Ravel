# Hierarchical pivot: stack-based plan execution in Ravel-Lite

**Date:** 2026-04-20
**Status:** design complete — pending implementation plan
**Consumers:** Ravel-Lite runtime, Ravel coordinator plan, Ravel sub-plans
**Produces:** a generic mechanism for nested plan execution under an
unchanged sub-plan tree.

## Problem

Ravel's v1 design is split across 12 sub-plan directories in
`LLM_STATE/`, each with its own `backlog.md`, `memory.md`, and phase
cycle. Driving implementation requires running
`ravel-lite run <plan>` per sub-plan, gated by cross-plan dependencies
tracked informally in `ravel-orchestrator/backlog.md`.

Today this is tedious: the user context-switches per plan; the
orchestrator's `subagent-dispatch.yaml` mechanism propagates
coordination notes but cannot drive a target plan's phase cycle;
cross-plan readiness is human-maintained.

A prior spec (`2026-04-20-superplan-design.md`) proposed flattening
all sub-plan backlogs into a single "superplan" driven by parallel
subagent dispatch. That spec has been abandoned — see "Alternatives
considered" at the end of this document.

## Goal

Add a **stack-based execution primitive** to Ravel-Lite so that a
plan's work phase can trigger a full cycle of a child plan and
transparently resume afterward. The mechanism is generic — any plan
can be a "coordinator" by authoring its work prompt to push child
paths onto the stack. Ravel-Lite itself has no special notion of
coordinator plans.

The hierarchy is additive: every existing sub-plan remains a valid
target without any changes to its prompts, backlog, memory, or
session log.

## Non-goals

- Parallel task execution within a single cycle. The mechanism is
  strictly serial: one plan active at a time, one cycle per push.
- Automatic child discovery. A coordinator plan's prompt enumerates
  its specific children by name. Ravel-Lite never walks the
  filesystem looking for plans.
- A "coordinator plan type" with bespoke phase handling. Ravel-Lite
  treats every plan uniformly; a coordinator is simply a plan whose
  work prompt happens to manipulate stack.yaml.
- Distributed execution, multi-machine stacks, or anything resembling
  v2 team mode. Strictly single-machine, single-ravel-lite-process.

## Architecture

### Two repos, three artefact roles

| Repo | Role | What changes |
|---|---|---|
| **Ravel-Lite** | Runtime | Phase loop becomes stack-aware. `Vec<PlanContext>` threaded through driver. New: `pivot.rs` (stack.yaml read/write). Modified: `phase_loop.rs`, `main.rs`, `ui.rs`, init templates. No changes to agent, format, git, or prompt modules. |
| **Ravel (coordinator)** | One plan per stack-root | `LLM_STATE/ravel-orchestrator/prompt-work.md` rewritten as the routing prompt. Memory gains a *Gates* section tracking cross-plan deps. Other phase prompts and files unchanged. |
| **Ravel (children)** | 12 existing sub-plans | No changes. Each runs as an ordinary plan whether invoked directly or pushed by the coordinator. |

### Scope sketch

- Ravel-Lite: ~300–400 LoC Rust + tests. Rust changes are localized
  to the driver layer.
- Ravel coordinator: one rewritten prompt file + a memory seed edit.
  No new files, no new directories.

### Session lifecycle

1. User invokes `ravel-lite run LLM_STATE/ravel-orchestrator`.
2. Driver checks for `<root>/stack.yaml`. If present, resumes at the
   deepest frame. If absent, starts at the root plan.
3. Coordinator's work phase runs interactively. User picks a target.
   Agent writes a new frame to `<root>/stack.yaml`, leaves
   `phase.md` at `work`.
4. Driver detects the short-circuit condition (see §Protocol) and
   immediately pushes the child frame, skipping the coordinator's
   remaining phases for this cycle.
5. Child cycle runs a full `work → ... → git-commit-triage`.
6. After git-commit-triage, driver pops the child, resumes
   coordinator at its next work phase.
7. Loop continues until the user declines the
   end-of-cycle "Proceed to next work phase?" confirmation.

## Protocol

### `stack.yaml` schema and location

```yaml
# <root-plan>/stack.yaml — runtime state, .gitignore'd
frames:
  - path: LLM_STATE/ravel-orchestrator
    pushed_at: 2026-04-20T18:30:00Z
  - path: LLM_STATE/sub-F-hierarchy
    pushed_at: 2026-04-20T18:32:14Z
    reason: sub-D/T2 landed last cycle; sub-F/T4 now unblocked
```

- **Location.** At the originally-invoked plan's directory. Ravel-Lite
  resolves this plan at invocation time (`main::run_phase_loop` call
  site) and remembers it as the stack root.
- **Schema.** Ordered list of `frames`. Each frame has `path`
  (required, either absolute or relative to the repo root) and
  optional `pushed_at` + `reason`. `reason` is free-form; it surfaces
  in the TUI breadcrumb on push and in the session log.
- **Lifecycle.** Created when the root plan's work phase writes the
  first non-root frame (depth 1→2). Trimmed by the driver on each
  pop. Deleted when depth returns to 1 (the absent file means "just
  at root").
- **Git status.** Added to a `.gitignore` entry at the root plan dir
  (or at the project's top-level `.gitignore`). Runtime state is not
  part of the plan's durable record. Audit of routing decisions lives
  in the coordinator's own git commits (during stateful pivots) and
  in its memory file (`Gates` section).

### Driver state transitions after a work phase

| `phase.md` after work | `stack.yaml` after work | Driver action |
|---|---|---|
| advanced (e.g., `analyse-work`) | no change to top | Normal cycle — run analyse-work next. |
| advanced | new top (last frame added) | **Stateful pivot** — run full cycle; at end of `git-commit-triage`, push the new top and enter child's cycle. |
| still `work` | new top | **Stateless pivot (short-circuit)** — immediately push the new top and enter child's cycle. Coordinator's remaining phases skipped this cycle. |
| still `work` | no change | Existing "phase did not advance" error (the current behavior at `phase_loop.rs:265`). Stop. |

### Driver state transitions at end of any full cycle

After `git-commit-triage` completes:

- If the plan just wrote a new top to stack.yaml (stateful pivot
  path): push it, enter child's cycle.
- Otherwise, if in-memory stack depth > 1: pop. Trim stack.yaml.
  Resume parent's next phase (which is `work` — parent's
  `git-commit-triage` runs at end of the parent's cycle, not the
  child's; the parent is between cycles when the child pops).
- Otherwise (depth 1, no new top): continue the root plan's loop
  normally. The existing end-of-cycle "Proceed to next work
  phase?" confirmation fires.

### Startup resume

On startup, if `<invoked-plan>/stack.yaml` exists and is non-empty:

1. Parse frames. On YAML error or empty frames list: hard error, log,
   exit.
2. Validate each frame's `path` resolves to a directory containing
   `phase.md`. First invalid frame: hard error, log full frames +
   offending index, exit.
3. Initialize in-memory stack from the frames list.
4. Resume at the deepest frame, reading that plan's `phase.md` for
   the next phase to execute.

### Safety rails

- **Max depth: 5.** Compile-time constant `MAX_STACK_DEPTH: usize = 5`.
  A push attempt at depth 5 is a hard error — log the full stack and
  the attempted target, exit. Prevents runaway recursion from a
  buggy coordinator prompt.
- **Invalid pivot target.** If a frame's `path` doesn't resolve to a
  directory containing `phase.md`, hard error with the full stack and
  offending path. No auto-pop, no auto-repair.
- **Corrupt stack.yaml.** YAML parse error, unknown fields, missing
  required fields → hard error with the parse error + file contents.
  User edits manually and resumes.
- **Cycles in the stack.** A push attempt targeting a path already
  present in the in-memory stack is a hard error. You cannot pivot
  to your grandparent.

### Invariants preserved

- Every plan's commits are made under that plan's own
  `git_commit_plan` machinery. The driver never commits on behalf of
  a plan.
- `phase.md` per plan remains the single source of truth for that
  plan's phase position.
- `subagent-dispatch.yaml` (existing cross-plan dispatch) is
  orthogonal to the stack and unchanged. A child's triage can still
  dispatch notes to other plans' mailboxes; the stack doesn't touch
  these files.
- A plan invoked directly — without any coordinator pushing it —
  behaves exactly as today. Stack awareness is additive.

## Coordinator plan pattern

### File roles in `LLM_STATE/ravel-orchestrator/`

- **`prompt-work.md`** — the only substantively new file. Authored
  once per coordinator; used every work-phase cycle. Its body
  instructs the agent to:

  1. Read its own memory (`memory.md`), focusing on the *Stable
     architectural decisions* and *Gates* sections.
  2. Read each named child's `backlog.md` — scoped to the file
     header plus the first ~5 `not_started` entries whose
     within-plan deps are satisfied. Cap per child at ~30 lines.
  3. Brief the user with a compact status grid:
     `child · ready_count · last_touched · bottleneck_note`.
  4. Recommend one target with a one-line rationale.
  5. On user accept/override, append a frame to
     `<root>/stack.yaml` with `path` and `reason`, and **leave
     `phase.md` unchanged** — this is the short-circuit pivot
     signal. On user "hold" / "done" / empty acknowledgement,
     leave stack.yaml untouched and **advance `phase.md` to
     `analyse-work` as normal**. The full coordinator cycle
     runs (mostly no-op for these phases when there's no local
     work to commit), and the existing end-of-cycle
     "Proceed to next work phase?" confirmation handles the
     continue/stop choice.

  Agents must never leave both `phase.md` at `work` and
  `stack.yaml` unchanged — that's the existing "phase did not
  advance" error case and will halt the loop.

  The prompt explicitly names each child plan the coordinator knows
  about. No filesystem discovery.

- **`memory.md`** — two active sections relevant to routing:
  - *Stable architectural decisions* (carried over from the current
    orchestrator memory).
  - *Gates* — free-form prose entries tracking cross-plan
    dependencies: "sub-F/T4 waits on sub-D/T2 (last checked cycle
    5, still open)". Populated by the reflect phase when the
    coordinator runs a stateful cycle, or by the user directly
    between sessions.

- **`backlog.md`** — optional. Not consumed by the routing prompt.
  If the user wants to track coordinator-level tasks ("revisit gate
  structure after sub-R lands"), they can add them; otherwise the
  file remains mostly historical.

- **`prompt-analyse-work.md`, `prompt-reflect.md`, `prompt-triage.md`**
   — not authored. Ravel-Lite's default templates apply. These run
  only during a stateful pivot or when the coordinator itself has
  local work to commit.

### Memory bootstrap

The existing `ravel-orchestrator/memory.md` already contains the
*Stable architectural decisions* section. The Gates section is
seeded by translating the current `backlog.md`'s "Priority 1 —
Implementation phase coordination" entries (Sub-B/C rewrite gate,
Sub-F Task 0 readiness, Sub-D DaemonLock + FileIO) into Gates
prose. This is a one-time manual edit before the first coordinator
run.

### Gates memory discipline

- **Prose, not structured YAML.** The LLM reasons over this
  in-context. Structured data would add schema burden without
  reasoning benefit.
- **Terse entries.** Each Gate is 1–3 sentences. Dream compresses
  when the section bloats.
- **Updated during reflect.** When the coordinator runs a stateful
  cycle (e.g., the user edits memory.md directly and advances
  phase.md), reflect's normal behaviour updates this section along
  with the rest of memory.

## TUI

Two small additions to `ui.rs`:

- **Breadcrumb in phase headers.** `log_phase_header` at
  `phase_loop.rs:63` currently renders
  `◆  WORK  ·  ravel-orchestrator / core`. With a non-trivial stack,
  the line becomes `◆  WORK  ·  ravel-orchestrator → sub-F-hierarchy / core`.
  Breadcrumb = stack path joined by ` → `, replacing the plan name
  in the scope string.
- **Stack transition lines.** On push: `▷▷ PUSH · sub-F-hierarchy · "{reason}"`.
  On pop: `◁◁ POP · sub-F-hierarchy → ravel-orchestrator`. Written
  via `ui.log` (inserted into scrollback). Informational; no new
  viewport state.

No new viewport (1-row inline area stays as today). No new event
variants on `UIMessage`. The breadcrumb threading requires the phase
loop to know the stack at log time, so `log_phase_header` gains a
`stack: &[PlanContext]` parameter.

## Rollout

1. **Writing-plans handoff.** After this spec is approved, produce
   a detailed implementation plan at
   `docs/superpowers/plans/2026-04-20-hierarchical-pivot.md`. The
   plan targets the Ravel-Lite repo (Rust changes) plus a small
   Ravel-side step (coordinator prompt + memory seed).
2. **Ravel-Lite implementation.** Subagent-driven TDD: types →
   `pivot.rs` (stack.yaml serde + read/write) → phase_loop
   integration → TUI breadcrumb → integration tests against a
   synthetic nested plan tree.
3. **Ravel coordinator authoring.** Edit
   `LLM_STATE/ravel-orchestrator/prompt-work.md` to the routing
   prompt. Seed `memory.md`'s Gates section from the current
   Priority 1 backlog entries.
4. **Pilot.** One invocation of `ravel-lite run LLM_STATE/ravel-orchestrator`
   against a child with short ready tasks. Verify: push works,
   short-circuit detection works, child cycle commits cleanly, pop
   works, restart from stack.yaml works.
5. **Steady state.** Run continuously. Observe whether Gates memory
   stays useful; adjust the coordinator prompt if briefing density
   is wrong.

## Completion semantics

There is no automatic "done" state. When every child has an empty
ready set, the coordinator's work-phase briefing shows an all-empty
grid; the agent recommends nothing; the user declines the pivot and
exits via the end-of-cycle confirmation. The user owns the stop
decision because "is the system complete?" is a judgement call that
requires inspecting the actual state.

## What doesn't happen

- No extraction pipeline, no fixture parsing, no Python tooling.
- No sub-plan archiving. Every sub-plan directory stays live and
  its session-log + memory continue to accumulate per-sub-plan
  context.
- No custom phase prompts for the coordinator beyond `prompt-work.md`.
- No new plan type in Ravel-Lite. No new config options.
- No parallel subagent dispatch inside a work phase (beyond what
  Claude's Task tool already provides — out of scope for this
  design).

## Alternatives considered

### Flat-backlog superplan

Documented in `docs/superpowers/specs/2026-04-20-superplan-design.md`
(abandoned). That approach collapsed all 12 sub-plan backlogs into a
single `LLM_STATE/superplan/` plan via a Python extraction pipeline,
with custom phase prompts for parallel subagent dispatch and per-task
commit splitting.

Reasons for rejection:
- Required a ~600-line Python tooling layer under
  `tools/superplan-scaffold/` just for the one-time scaffold.
- Discarded per-sub-plan memory and session-log accumulation by
  archiving the sub-plan directories — losing the fresh-context
  reasoning trail the sub-plans had earned during brainstorming.
- Special-cased to Ravel v1's current structure; not reusable for
  future hierarchical work.
- Introduced a custom "superplan" plan with four custom phase
  prompts and a dep graph construction pass, all of which would
  need to be maintained.

The hierarchical-pivot approach in this spec achieves the same
user-visible goal ("run ravel-lite continuously and progress the
whole v1 backlog") with a smaller, reusable primitive and zero
changes to the sub-plans.

### Triage-time pivot

An earlier sketch had the coordinator's *triage* phase write the
pivot, not work. Rejected because triage is headless; routing
decisions benefit from user interaction; work is where the user
already engages with the loop.

### Child-driven pop (one-to-many cycles)

Considered letting a child run multiple cycles until it writes its
own `pop.yaml` or its backlog empties. Rejected in favour of strict
one-cycle-per-push because it:
- Keeps the user in the loop at every routing decision.
- Makes cycle boundaries predictable.
- Preserves the "human as co-equal actor" principle (`README.md:307`):
  the user gets a chance to re-route between every child cycle.
