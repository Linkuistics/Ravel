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
