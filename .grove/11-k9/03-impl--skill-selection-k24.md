# skill-selection-k24

## Goal

Let the Lua configuration select the applied and the available skills for each
kind. Deliver them through the prompt to every harness. This is the smallest
end-to-end path of the skill work.

## Context

- The reworked spec `launcher/docs/specs/skill-discovery.md`, and the selection fields
  that the spec adds to the Lua contract.
- The combined survey `launcher/docs/research/harness-delivery.md`, section on the
  prompt-only delivery for other harnesses.

## Done when

- A Lua script selects applied and available skills per kind, including an
  empty selection. A missing or unreadable selected skill fails with a clear
  refusal.
- Ravel builds the delivery after the selection, and the caller task stays
  unmodified in the prompt.
- Selected skill content stays stable for a running session and its
  continuations.
- Prompt size, compaction and delegation behave as the spec states.
- The evaluated preview shows the route, the selected skills, the delivery
  mode, the limitations and the exact prompt. Ordinary logs show no prompt
  body by default.
- The fake harness that accepts only a prompt exists at the command-line seam.
  Tests show exact task preservation, selection and exclusion, order, snapshot
  stability, the preview, the diagnostics and the prompt-only delivery.
- The documentation lists each gap on a harness without an adapter, with its
  explicit alternative or actionable refusal.

## Notes

- Rhai scripts keep their 2.0 behaviour and get no selection.
- If the work is larger than one session, finish selection with prompt
  delivery first. The preview can follow in a sibling.
