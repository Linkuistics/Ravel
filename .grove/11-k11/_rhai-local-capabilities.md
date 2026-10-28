# rhai-local-capabilities-k11 — brief

## Goal

Give a Rhai policy every host capability that needs no network and starts no process. A policy can read and measure files, import modules and load a context. It can also use the data helpers and read granted environment variables. It can discover the configured harnesses and LLM endpoints.

## Done when

- A Rhai policy reaches parity with the capabilities that the TypeScript policies use. These are measured reads, module imports, `load_context`, JSON, UUID, path helpers, environment reads and discovery.
- Each capability that Grove's helpers need has a test at the CLI seam. The capabilities are listed in the notes. No test uses Grove code.
- The routing notice of the Rhai path lists the modules and every measured read.
- A read never holds the Ravel CLI. The reader thread, the nonblocking open and the stop order of the spec hold, and the FIFO case has its firing control.
- The three new `settings.json` keys `endpoints`, `harnesses` and `budgets` parse and validate. Nothing yet uses an endpoint to call a model.
- The TypeScript path works as before, and `task check` and `task test` pass.

## Decomposition

- `rhai-measured-reads` adds the bridge between the worker and the Ravel CLI, with reads, imports and `load_context`. It comes first because every other host function uses the bridge.
- `rhai-data-helpers` adds the pure helpers and `env`, and it proves the Grove parity list.
- `rhai-settings-discovery` adds the three settings keys, the credential name rules and the discovery functions.

The last two leaves do not depend on each other. The order follows the risk, with the bridge first.

## Pointers

- Spec: `docs/specs/configuration-evaluation.md`. The sections are "The bridge between the script and the CLI", "Host capabilities", "LLM endpoints and credentials" and "Harness entries".
- ADR: this increment followed an ADR about LLM endpoints in settings. `docs/adr/a-configuration-script-asks-a-model-through-a-harness-run.md` replaced it, and the running log of `llm-calls-through-harnesses-k58` records why.
- Review finding F1 of `rhai-policy-k4` explains why a read needs a reader thread. Read it with `grove-llm resolve rhai-policy-k4`. The integration `rhai-policy-k5` records the chosen mechanism.
- Glossary: Host capability, Policy context, LLM endpoint and Evaluation budget.
- Test seam: the real `ravel start-session` against a sandbox home.

## Notes

- Grove's helpers, which Linkuistics/grove#24 will port, need these generic capabilities. Ravel proves each one without Grove code:
  - A measured read of a task file, within its limits.
  - JSON parsing and canonical JSON encoding.
  - String matching, enough to parse a handle and a marker line.
  - A random UUID, version 4.
  - One policy that wraps another and changes its selection.
  - A module import by path.
- The notice of the Rhai path has schema version 2. Each leaf adds the members it owns.
- Run `task docs:excerpts` after a change to code that the book quotes. Fix a moved excerpt.
