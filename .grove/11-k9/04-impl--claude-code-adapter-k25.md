# claude-code-adapter-k25

## Goal

Build Ravel's adapter for Claude Code. It delivers the selected skills
natively and applies their launch-time hooks, as the spec `skill-discovery`
states.

## Context

- The reworked spec `launcher/docs/specs/skill-discovery.md`.
- The combined survey `launcher/docs/research/harness-delivery.md`, section on the
  adapter implementations.
- The existing Taskfile task `harness:smoke:claude`.

## Done when

- A Claude Code launch receives the applied and available skills natively,
  scoped to that launch. The user's home configuration is not changed.
- Hooks that a skill declares run on Claude Code, such as those of the writing
  rules.
- A fake Claude Code at the command-line seam records the arguments, the
  environment, the input and the readable files. Tests show the delivery and
  the hooks through the launch boundary in an isolated home.
- The preview names the native delivery mode and each remaining limitation.
- An opt-in Taskfile task runs the trial set on the real Claude Code in a
  sandbox home with the owner's sign-in. Its report stays separate from the
  simulated tests.
- This session ran that task. The running log records the Claude Code
  version, the result and each failure.

## Notes

- If the real trial cannot run, record the blocker and keep this leaf live.
  Fake-seam results are not evidence of a real trial.
- Re-check each flag against the installed Claude Code version. The survey's
  sources age.
