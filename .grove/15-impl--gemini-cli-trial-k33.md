# gemini-cli-trial-k33

## Goal

Run the real trial set on Gemini CLI through the adapter that
`gemini-cli-adapter-k27` built. Record the result apart from the simulated
tests.

## Context

- The opt-in Taskfile task and the adapter of `gemini-cli-adapter-k27`, with
  its running log.
- The trial reports of `claude-code-adapter-k25` and `codex-adapter-k26` show
  the expected shape of a report.
- The reworked spec `launcher/docs/specs/skill-discovery.md`, for the documented gaps.

## Done when

- The owner installed Gemini CLI and signed in.
- The opt-in Taskfile task ran the trial set on the real Gemini CLI in a
  sandbox home with the owner's sign-in.
- The running log records the version, the result and each failure. The
  report stays separate from the simulated tests.
- This session repairs each failure in the adapter. Otherwise the spec and the
  preview name it as a gap, with its explicit alternative or actionable refusal.

## Notes

- Gemini CLI is not installed on the owner's machine. First, ask the owner to
  install it and sign in. If the answer is pending, keep this leaf live.
- Do not mark the trial done before it ran. Fake-seam results are not evidence
  of a real trial.
- This leaf sits after the work that does not use its result. The book and the
  release use the result, so they follow it.
