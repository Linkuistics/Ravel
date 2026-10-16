# writing-an-owner-policy-k64 — brief

## Goal

Chapter 4 (`docs/book/src/04-writing-an-owner-policy.md`), taken through draft
→ copy-edit → art → proof under `docs/book/contract.md`. After it the reader can
write, typecheck and validate a policy with fake agents, covering the SDK,
refusals, policy context, settings, environment grants, routes to real agent
programs and copying an existing policy.

## Decomposition

One document drafted in two sessions, then the three remaining stages once.
Part A is the anatomy of a policy and its validation with fake agents; part B
is everything a policy then needs to be used for real. The seam is the point
at which a reader has a policy that routes and refuses correctly under fake
agents. Part B starts from that and cuts nothing out of part A.

- **Part A**: the policy file and `definePolicy`; the request fields; a
  selection and a refusal with their fields; the starter `ravel/examples/static`
  table and the editable `sample/policy.ts` with `readChoice`; typechecking
  against the generated declarations, in a checkout and from an installed
  prefix; validating every route and every refusal with fake agents on a private
  PATH with a cleared environment; running a policy with `--config`.
- **Part B**: policy context (the version-1 document, `loadContext`, host
  reads and their bounds, measured sources); `settings.json` (`timeoutMs`,
  `contextBytes`, `policyEnv`, unknown keys refuse, the obsolete `stateDir`);
  environment grants and the names never grantable; module resolution (what
  imports resolve, embedded names, package.json autoloading); routes to Claude
  Code (`claude`) and Codex (`codex`) with their actual flags, validated
  through fake executables of the same name; copying an existing policy.

## Context

- Needs chapters 1–3 and the examples check: every complete example runs
  through `task docs:examples`. Sources: `docs/specs/session-execution.md`
  ("Request, context and selection", "Policy authority and runtime discovery"),
  `crates/ravel/worker/sdk/index.ts`, `examples/static.ts`, `sample/policy.ts`,
  `docs/owner-configuration.md` (being replaced; verify, and re-verify its
  typecheck recipe against the installed declarations), `crates/ravel/tests/cli.rs`
  for the settings, grants and context cases.
- Real agent flags (part B): check each agent's flags against its own
  `--help` at a recorded version and date. Claude Code 2.1.301 and codex-cli
  0.162.1 were installed on 2026-10-16; recheck, and record the versions and the
  date in the chapter. Which flags and modes the owner's own policy uses today
  may be read from the owner's policy, but its routing table, labels and model
  names are personal and must not be copied into the book; the book's example
  policies are the book's. Installing the agents, accounts and credentials are
  out of scope with a pointer to each agent's own documentation. An
  interactive agent takes the terminal; say so where a route depends on it
  and point at chapter 9.
- The ending instructions a caller's prompt gives the agent are shown, because
  Ravel injects none.

## Done when

- Chapter 4 exists, is linked from `SUMMARY.md` and has passed all four stages.
- Each complete example runs under the checker; every route and every refusal
  a chapter policy defines is shown validated with a fake agent.
- The chapter states which settings keys, bounds and refusal codes exist by
  enumeration from the settings reader, SDK and spec, and chapter 6 entries
  will be consistent with it (they are written later from the same sources).
- Each statement from `docs/owner-configuration.md` outside the dated handoff
  record is either in chapter 4 or noted as chapter 6's, in this unit's log, so
  the guide fold-in can verify.

## Notes

- Part A cuts part B as its last act, ahead of copy-edit: cut a `draft` leaf
  with the bare stem for part B and its scope, then cut copy-edit as the
  family file directs (it lands after part B). Part B's session cuts nothing if
  copy-edit already stands.
