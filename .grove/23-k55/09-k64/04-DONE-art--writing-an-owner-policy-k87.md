# writing-an-owner-policy-k87

## Goal

Decide and make chapter 4's figures, `docs/book/src/04-writing-an-owner-policy.md`,
under `docs/book/contract.md` and `docs/book/figures.md`, in the media the
contract admits (PlantUML 1.2026.8 with its bundled C4-PlantUML, or d2 0.9.0),
following the first half's existing visual vocabulary in
`docs/book/src/figures/_style.puml` and `_uml-style.puml`.

## Context

The chapter has no figures yet. Copy-edit (`writing-an-owner-policy-k86`)
fixed the chapter's terms to chapter 1–3's spellings; use them in labels:
policy worker, native front, managed process, the managed process's group,
session-result context, policy context, refusal.

Reader questions in the chapter that a figure may answer better than prose
(art decides which earn a figure, and may decide none of them does):

- **Policy evaluation order.** Import, then the optional `loadContext`, then
  `select`, all under one selection deadline (`selectionMs`); where each kind
  of refusal leaves the flow (an explicit policy refusal, a context refusal
  such as `context_invalid` before the loader runs, `selection_timeout`), and
  that nothing launches until a selection is admitted. Sections "Select a
  command or refuse" and "Supply policy context".
- **What reaches `select`.** The caller's `--context` document versus the
  loader's returned document, with Ravel's `measured` ledger attached; the
  loader sees `request.context` and decides whether to retain it. Section
  "Supply policy context".
- **Two environments.** The policy worker's environment (HOME, PATH, TMPDIR,
  LANG, `LC_*`, plus `policyEnv`/`--policy-env` grants, minus the excluded
  classes) against the unconfined managed process's (the caller's environment,
  scrubbed, plus the session-result context). Section "Set bounds and grant
  environment names". Chapter 3's `process-authority` figure may already carry
  part of this; reuse rather than duplicate.

The settings and bounds tables, refusal-code tables and agent-flag table are
figures made of the page's own text and need no source file.

## Done when

- Each figure added has its editable source beside its SVG, an alt text that
  carries its content, notation conventions identifiable on or beside it, and
  a regeneration record in `docs/book/figures.md`; `task docs:figures`,
  `task docs:examples` and `task docs:book:build` pass.
- A decision not to draw a candidate is recorded in this leaf's log with the
  reason.
- Last act: add the `proof` leaf with bare stem `writing-an-owner-policy`
  unless a live later sibling already holds it. Retire and commit normally.

## Decisions (running log)

- Read the whole chapter and the contract. Use page-native tables for the
  evaluation sequence, caller/loader document cases and worker/managed-process
  environment comparison. A fourth table compares import and host-read
  resolution bases, another relation presently carried only by prose. Keep
  the examples and their outputs intact; give existing tables an explicit
  reader takeaway where their introduction only names their subject.
- Decline an evaluation-order UML diagram as an editorial judgement: a short
  ordered table can keep the stopping conditions beside each step, with the
  single deadline stated adjacent. Decline a context-flow diagram as an
  editorial judgement: the four caller/loader combinations are easier to
  compare as rows. Decline another process/environment drawing as an editorial
  judgement: chapter 2's process-authority view already establishes the
  processes; link it and compare environment rules locally. Decline a
  resolution diagram as an editorial judgement: paths and exceptions need
  direct lookup. None is a contract prohibition or evidence that the admitted
  medium cannot carry the relation.
- Verify new figure claims against the session specification and targeted
  source (worker assembly/selection, native worker deadline/admission,
  context validation and grants). Tier 2 graph generation
  2026-10-16T04:39:52Z reports matching metadata and no recorded gaps for
  these paths; direct source reads supplement the bounded graph lookup and
  select call trace. This is source support, not new runtime evidence.
- Visual inspection in native Chrome found the caller-document column heading
  cramped; shortened it and inspected the rendered table again. All four new
  tables are legible at the local book's chapter width on a light page. The
  shared PlantUML includes have untracked graph freshness metadata; they were
  read directly and remain unchanged. Viewing scope and unavailable surfaces
  are recorded in `docs/book/figures.md`. No in-session reviewer was needed.
- Final verification after the heading and link adjustment: `task docs:figures
  docs:examples docs:book:build` exits 0. Four existing SVGs match their sources;
  all seven complete examples pass individually, with none failed or skipped
  and four labelled fragments not run; mdBook reports no warning. SHA-256
  manifests before and after match for all `rg --files` entries under
  `docs/book/src`, `scripts` and `crates`, plus `Cargo.toml`, `Cargo.lock`,
  `Taskfile.yml` and `docs/book/book.toml`. Every chapter code block and example
  label is byte-identical to the parent commit. The temporary viewing server
  and browser tab were closed.
- The source and contract agree on these figure claims; no earlier-stage
  correction or architectural decision is earned. The document still needs
  proof, so its node and ancestors remain open. No later proof sibling is
  present: cut that stage with the bare document stem as the last work act.
