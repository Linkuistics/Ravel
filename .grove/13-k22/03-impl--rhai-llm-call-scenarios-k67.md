# rhai-llm-call-scenarios-k67

## Goal

Prove at the command-line seam that an LLM call is a harness run. A fake harness plays the model. The leaf also proves the credential boundary of a harness run, with one ungranted sentinel for both scenarios.

## Context

- The input is the requirements "Generic LLM interaction" and "Credential boundary", and the sections "LLM calls through a harness" and "Credentials", in `docs/specs/policy-evaluation.md`.
- `ravel::run_harness` exists after `rhai-harness-run-k23`, and `ravel::json_check` exists after `rhai-json-check-k62`. The leaves that built `ravel::llm` tested the same scenarios against a fake provider endpoint. The removal deleted those tests.
- A fake harness for an LLM call prints a scripted answer. It records its arguments and its `stdin` for the test. See "Test seams" in the spec.
- The leaf adds no host function. If a scenario shows a gap in the code, fix it here and record the fix in the running log.

## Done when

- A test for "LLM-assisted selection". A fake harness classifies the prompt, and the policy maps the answer to a route. Ravel launches that route. The notice lists one harness run. The fake harness received exactly the arguments and the `stdin` of the script.
- A test for "Another purpose". `load_context` runs a fake harness for a summary, and `select` receives it in the delivered context.
- A test for "A structured answer that does not match". The fake harness prints JSON that lacks a required property. The policy checks it with `ravel::json_check` under `try`. The catch block gets `json_mismatch` with the location and no part of the answer, and the policy goes on.
- A test for "A failed call and a fallback". The first fake harness exits 1 with a rate-limit message. The policy then runs a second harness and uses its answer.
- A test for "Leak probes". The caller's environment holds a sentinel `ANTHROPIC_API_KEY` that no grant names. A policy that makes no measured read and runs no harness calls `ravel::env` with that name. `env` throws `env_not_granted`, and the sentinel appears in no diagnostic or notice.
- A test for "A harness finds its own sign-in". A fake harness in an unconfined run prints its `HOME` and the same sentinel. The script receives both unchanged.
- The notice of each test holds no prompt and no answer. The notice lists each run with its harness, status and elapsed time.
- Each test keeps a firing control in the same test. The control for the leak probe is a run where a grant names the sentinel, and the sentinel then reaches `env`.
- `task check` and `task test` pass.

## Notes

- Use one sentinel value for both credential tests. Pick a value that no other part of the output can contain by chance.
- The fake harness never reads a real sign-in. The test sets `HOME` in the sandbox.
- `rhai-harness-smoke-k66` follows and makes the real calls. Do not make a real call here.
