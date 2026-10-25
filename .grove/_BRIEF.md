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
  Each increment leaves main working. The increments follow, in order.
  - `rust-2024-toolchain` moves the workspace to Rust 1.99.1 and edition 2024.
  - `rhai-route-table` makes a deterministic Rhai policy work, and it makes the
    worker safe against a hostile script.
  - `rhai-local-capabilities` adds reads, imports, `load_context`, the data
    helpers, the three settings keys and discovery.
  - `rhai-llm-calls` adds `ravel::llm` for three API types, with bounds,
    budgets, structured output and the Rig upgrade proof.
  - `rhai-harness-runs` adds `ravel::run_harness`, the re-entry refusal and the
    ordered stop.
  - `rhai-example-policies` translates the example policies and proves their
    equivalence with the TypeScript originals.
  - `rhai-book-use` revises Parts I and II of the book and adds the migration
    guide.
  - `typescript-removal` freezes the equivalence results, removes the
    TypeScript path and its delivery machinery, and reconciles the specs.
  - `rhai-book-inside` revises Parts III and IV and proves the whole book.
  - `release-ravel-2-0-0`, `homebrew-upgrade` and `owner-policy-conversion`
    end the workstream.
  A `review-planning` leaf reads this cut before the first increment runs.

The order is the dependency order. Planning needs the agreed design, and the
design needs the settled requirements.

## Pointers

- GitHub issue 2 states the work in the owner's words.
- The running log of `plan-k1` holds the settled requirements. Find its file
  with `grove-llm resolve plan-k1`.
- ADRs in play: `owner-policy-is-a-rhai-script`,
  `policy-evaluation-precedes-the-launch`,
  `owner-settings-name-the-llm-endpoints`, `ravel-supervises-the-session` and
  `the-launched-child-is-a-job`.
- Specs in play: `policy-evaluation`, which is the 2.0.0 design, and
  `session-execution` and `result-job-completion`.
- Glossary terms in play: Owner policy, Policy worker, Host capability, LLM
  endpoint, LLM call and Provider. Also Harness run, Evaluation budget, Policy
  context, Harness, Refusal, Caller and Owner. See `CONTEXT.md`.
- `task docs:rhai-policy` serves the design views in `docs/design/rhai-policy/`.
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
- Ravel 1.1.0 refuses an unknown key in `settings.json`, and both versions read
  the same file. The conversion of the local configuration adds no new settings
  key until the driver restarts.
- `task release:major` refuses outside Ravel's default colocated jj workspace.
  The work of this workstream lives in a secondary workspace, so the release
  leaf asks the owner how the work lands on main.
- The book is published with each release. Both book nodes finish before the
  release leaves run.
- Run `task docs:excerpts` and `task docs:examples` after a change to code that
  the book quotes. Fix a moved excerpt in the same leaf.
