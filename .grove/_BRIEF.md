# Ravel.gh-issue-2 — brief

## Goal

Replace Ravel's TypeScript owner policy with embedded Rhai scripts, as GitHub
issue 2 asks. Rhai scripts get generic LLM access through Rig, and Ravel keeps
execution authority. The work ends with the Ravel 2.0.0 release and the owner's
local upgrade.

## Done when

- Every acceptance criterion in issue 2 holds, and tests through the agreed
  seam prove each one.
- Rhai is the only format of owner policy. The Bun policy worker, the
  TypeScript SDK, their declarations, and their build and release steps are
  gone.
- The book, the README, the CLI help and the installation documents describe
  Rhai policies. A migration guide translates the representative policies.
- The ADR set, the spec set and `CONTEXT.md` describe the new design.
- Ravel 2.0.0 is released.
- The owner's Homebrew installation runs Ravel 2.0.0, and the owner's local
  configuration is in Rhai.

## Decomposition

- `plan-k1` (requirements, done) settled what to build, with the owner.
- `rhai-policy-k2` (design) establishes how, as a spec and a reworked ADR set.
- `rhai-policy-increments-k3` (planning) cuts that design into increments.
  Each increment leaves main working.

The order is the dependency order. Planning needs the agreed design, and the
design needs the settled requirements.

## Pointers

- GitHub issue 2 states the work in the owner's words.
- The running log of `plan-k1` holds the settled requirements. Find its file
  with `grove-llm resolve plan-k1`.
- ADRs in play: `policy-evaluation-precedes-the-launch`,
  `ravel-supervises-the-session` and `the-launched-child-is-a-job`.
- Specs in play: `session-execution` and `result-job-completion`.
- Glossary terms in play: Owner policy, Policy worker, Policy context, Harness,
  Refusal, Caller and Owner. See `CONTEXT.md`.
- The test seam: the real `ravel start-session` against a sandbox home. Fake
  harness scripts and a local fake provider endpoint stand behind it.

## On the horizon

- A verb that evaluates a policy without a launch. The owner kept it out of
  this workstream. With LLM calls in a policy, `start-session` is the only way
  to exercise the policy, and it launches the selected program.

## Notes

- <https://github.com/Linkuistics/grove/issues/24> carries the port of Grove's
  policy helpers to Rhai. Ravel 2.0.0 does not wait for it.
- The Homebrew upgrade runs inside a Grove session that Ravel 1.1.0 launched.
  Keep the 1.1.0 version and `policy.ts` until the driver restarts. The running
  log of `plan-k1` gives the reason.
