# harness-delivery-k6

## Goal

Combine the two surveys of the harness-delivery question into
`docs/research/harness-delivery.md`. Keep the consumer sections of the shared
brief, so that each downstream leaf reads its own section.

## Context

- The shared brief is the body of `harness-delivery-k4`. The surveys are
  `docs/research/harness-delivery-a.md` and `docs/research/harness-delivery-b.md`.

## Done when

- The combined survey answers each question of the shared brief, and it flags
  each disagreement between the two surveys.
- It states, per supported harness, what a launch-scoped configuration can set
  and what it cannot.
- It states whether delivery through the prompt alone reaches Pi and Hermes,
  and with which losses.

## Notes

- Each claim about a harness keeps its source date and version. The skill
  design relies on them, and the adapters re-check them.
