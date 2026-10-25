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

## Decisions (running log)

- **The leaf needed no code change.** `ravel::run_harness`, `ravel::json_check` and the harness-run notice already served every scenario. The six tests are in `crates/ravel/tests/rhai_llm_calls.rs`, apart from `rhai_harness_run.rs`, which tests the run itself. This file tests what a policy does with a run as an LLM call, and what the run's environment holds.
- **The fake model records itself.** `fake_model` writes its arguments (one per line) and its whole standard input to files in the sandbox, and then prints the scripted answer. The selection test compares both records with what the script sent, byte for byte. The recording agent is still the managed process, so a test sees the route that Ravel launched.
- **Every case has a firing control in the same test.** Selection: a second prompt maps to the other route. Summary: another summary is another delivered fact. Mismatch: the same answer with the missing property passes the check. Fallback: a first call that exits 0 leaves the second harness unstarted. Leak probe: a `--policy-env` grant makes `ravel::env` return the sentinel, and the probe's own `print` then shows it in stderr, so the stream scan reads a stream that can hold it. Sign-in: a caller without the variable makes the run print `unset`.
- **The notice holds the summary of "Another purpose", and that is the policy's act.** The first run of the test failed on the check "the notice holds no answer". The notice reports the delivered context (`context.value`), as it did in 1.x, and the scenario has the policy put the summary in `facts`. I kept the code. A change would remove the owner's view of what `select` received, and the harness run itself leaves no output in the notice. The test now checks the rest of the notice without the `context` member, and it asserts that the summary is in `context.value.facts` and nowhere else. The spec sentence in "LLM calls through a harness" claimed more than the code gives, so it now says that a script that copies an answer into the context it returns, or into its own output, publishes the answer there.
- **Every other notice is one line of stderr.** `notice_of` asserts that the notice is the only line, so a policy that prints nothing leaves no diagnostic. That also proves the prompt and the answer reach no log of Ravel.
- **The sentinel is `sk-ant-SENTINEL-…`, one value for both credential tests.** It contains no digit run that a pid or a time can make. The tests set `HOME` and the sentinel in the sandbox and read no real sign-in. No test made a real call.
- **The probe policy prints every value it can reach.** With no measured read and no harness run, that is `request`, `context`, `ravel::harnesses()` and `ravel::remaining()`, and the message and name of the `env_not_granted` error. The test also asserts that the notice lists no read and no run, so the probe made neither.
- **The in-session review was not spent.** Each claim is an executable assertion at the seam with a control, and no hard-to-reverse decision rests on a framework API.
- **`task check` and `task test` pass.** Both run 605 tests with 48 skipped, including the six new ones. The STE lint of the spec stays at 0 hard findings. `task test:native-linux` belongs to the last leaf of the increment, `rhai-harness-smoke-k66`. These tests use no process-group behaviour that differs on Linux.
