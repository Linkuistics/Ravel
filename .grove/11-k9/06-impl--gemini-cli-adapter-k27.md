# gemini-cli-adapter-k27

## Goal

Build Ravel's adapter for Gemini CLI. It delivers the selected skills natively
and applies their launch-time hooks, as the spec `skill-discovery` states.

## Context

- The reworked spec `docs/specs/skill-discovery.md`.
- The combined survey `docs/research/harness-delivery.md`, section on the
  adapter implementations.
- The adapters of `claude-code-adapter-k25` and `codex-adapter-k26` show the
  shared shape.

## Done when

- A Gemini CLI launch receives the applied and available skills natively,
  scoped to that launch. The user's home configuration is not changed.
- Hooks that a skill declares run on Gemini CLI, or the preview names the gap
  with its explicit alternative or actionable refusal.
- A fake Gemini CLI at the command-line seam records the arguments, the
  environment, the input and the readable files. Tests show the delivery and
  the hooks through the launch boundary in an isolated home.
- An opt-in Taskfile task can run the trial set on the real Gemini CLI in a
  sandbox home with the owner's sign-in. Its report stays separate from the
  simulated tests. A test against the fake shows that the task runs the trial
  set.

## Notes

- Gemini CLI is not installed on the owner's machine. This leaf does not need
  it.
- The root leaf `gemini-cli-trial-k33` runs the real trial. Do not run it here,
  and do not claim a real trial result.
