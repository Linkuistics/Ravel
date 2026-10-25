# rhai-llm-bounds-k17

## Goal

Complete the bounds of an LLM call. The first LLM leaf already bounds every response body. This leaf bounds the answer text and proves that a call in flight stops at the deadline and on cancellation. An endpoint cannot exhaust the memory of the CLI, and it cannot hold the CLI after a stop.

## Context

- The input is the review finding F2 of `rhai-policy-k4` and its integration in `rhai-policy-k5`. Find both with `grove-llm resolve`. The spec sections are "LLM endpoints and credentials", "Evaluation budgets and their composition" and "Cancellation and deadlines".
- `rhai-llm-anthropic` built Ravel's own client over the `HttpClientExt` interface of Rig, with the body bounds in it. Read its running log for the places of the bounds.
- Rig keeps the raw provider document in `CompletionResponse.raw`. Check that the bound still holds the memory of that copy.

## Done when

- The body bounds of `rhai-llm-anthropic` still hold. They are 8 MiB for a successful body and 64 KiB for a failure body, and their tests still pass.
- The memory of the CLI stays bounded when Rig keeps the raw provider document in `CompletionResponse.raw`.
- The answer text has at most 1 MiB. A longer text fails the call as `output_limit`.
- Tests at the CLI seam cover the scenario "Stalled endpoint". A further case has an answer text beyond 1 MiB inside a body below 8 MiB.
- A stalled endpoint ends the evaluation with exit 124, and the fake endpoint sees its connection close.
- A TERM during a call in flight exits 143, the fake endpoint sees its connection close, and nothing launches. A timeout then signal case follows the spec rule that a signal wins.
- `task check` and `task test` pass.

## Notes

- Dropping a request cancels local work only. A provider may finish, and may bill, a request that it already accepted. The spec says so, and no test needs to prove otherwise.
- The connection deadline is no later than the deadline of the call.

## Decisions (running log)

- **The 8 MiB byte bound did not hold the memory of `raw`.** Measured with a peak counter on the counting allocator of the test build (`rhai_worker/heap.rs`, `reset_peak` and `peak`, test builds only): a body of 8 MB that holds `0,0,0,…` peaked at 586 MiB, 73 times the body. Rig builds a `serde_json::Value` of the whole document (`driver/http_transport.rs:144`, `serde_json::from_slice(&sent.body)`), keeps it as `CompletionResponse.raw`, and reads the body again for the answer. Four million values of 32 bytes each, and a copy or two more, explain the figure.
- **A second bound counts the JSON values, before Rig decodes.** `llm/shape.rs` walks the body with a serde visitor and builds no tree. It counts every scalar, array, object and object key, and it stops at the first value past 262,144 (`MAX_BODY_VALUES`). The transport runs it after the bytes are collected and fails the call as `output_limit` (`Fault::TooManyValues`). A body that is not JSON passes to Rig, which fails on it before it builds anything large. At the limit the decode peaked at 36 MiB for zeros and 52 MiB for distinct object keys, and the 8 MB body of zeros fails at 18 MiB, the copy of the fake server included. A legitimate Messages answer holds a few dozen values. The limit is a constant of the transport, so `rhai-llm-openai-types` meets it too: an OpenAI response with `logprobs` would hold more, and Ravel asks for none.
- **Test of the memory bound.** Three adapter tests in `llm/tests.rs` assert a ceiling of 128 MiB (`CEILING`) for the 8 MB body of zeros, for a body of 130,000 distinct keys and for one string of 8 MiB. With the limit set to `u64::MAX` the first test fails (seen). nextest runs each test in its own process, so the peak counts that call alone.
- **The text bound is checked after the redaction.** `Adapter::call` fails a text over 1 MiB (`MAX_TEXT_BYTES`) as `output_limit`, not retryable, with the length in the message and the request id kept. It checks the text the script would see, because the redaction of a credential shorter than ten bytes makes a text longer. Tests: exactly 1 MiB passes, one byte more fails, the bound counts bytes not characters, and a one-byte credential cannot carry a text past it.
- **The runtime is retired without waiting for a blocking task.** `front::retire` calls `shutdown_background`. Dropping a Tokio runtime waits with no limit for a blocking task (`tokio` 1.54.1, `runtime/blocking/pool.rs`, `BlockingPool::shutdown(None)`), and the resolver of the HTTP client looks a host name up with `getaddrinfo` in one (`hyper-util` 0.1.21, `client/legacy/connect/dns.rs:118`). A stalled lookup would have held the Ravel CLI after a stop. The unit test shows both sides: a plain drop waits for the task, and `retire` does not. No seam test can stall `getaddrinfo`, so that is the proof there is. Nothing else changes, because no blocking task exists on the path to a launch.
- **Dropping the request does not close the connection by itself.** With `retire` replaced by `mem::forget(runtime)`, the stalled endpoint saw the close only at the exit of the CLI (seen on Linux). The hyper connection task closes the socket when the runtime drops its tasks. The runtime is dropped before the teardown of the worker, so the close comes first.
- **The seam tests of the stall hold the teardown with a stopped worker, on Linux.** A close seen after the exit of the CLI proves nothing, because the exit closes every descriptor. On Linux the test stops the worker (SIGSTOP). A stopped process takes the TERM of the teardown only when it continues, so the CLI waits out its grace of one second, and the test sees the close while the CLI still runs (`assert_closed_before_exit`). Without the hold the CLI exits 13 ms after the close. macOS ends a stopped process on TERM, so there the hold does nothing and the check that needs it does not run. A file-heavy private directory was tried as the hold on both platforms and dropped: creating 40,000 files took longer than the deadline.
- **Tests at the CLI seam.** `a_stalled_endpoint_ends_the_evaluation_at_the_deadline_and_the_connection_closes` covers the scenario "Stalled endpoint" for an endpoint that stalls after its head and for one that never answers (the new `Body::Silent` of the fake server): exit 124, `selection_timeout`, nothing launched, no private directory left. `a_signal_during_a_call_in_flight_closes_its_connection_and_launches_nothing` sends TERM, INT and HUP during a call: the CLI ends by the same signal (143 for TERM to a shell), reports `selection_cancelled`, and launches nothing. `a_signal_that_arrives_after_the_deadline_wins_and_the_report_keeps_the_timeout` is Linux only: TERM arrives during the teardown after a deadline, the CLI ends by TERM, and the report keeps `selection_timeout`. The answer text beyond 1 MiB inside a body below 8 MiB is a seam case too. `worker_of` and `gone` moved to `tests/support/rhai.rs`, because both test files use them.
- **Mutations seen to fail.** Limit of values at `u64::MAX`: the memory test fails. `retire` as `mem::forget`: the stall test fails on Linux with "the front had exited when the close arrived".
- **Verified on Linux.** `cargo nextest run -p ravel --test rhai_llm` in the arm64 image of `scripts/native-linux.sh` passes all 27 tests, the Linux-only case and `a_trust_store_that_blocks_cannot_hold_the_whole_deadline` among them. The review integration `rhai-llm-anthropic-k57` could not run the latter on macOS.
- **A fact for `rhai-llm-budgets`.** A call whose text passes 1 MiB fails as `output_limit`, but Rig had decoded the answer, so the endpoint reported usage and the tokens were spent. `Failure` carries no usage. The accounting of that leaf must debit the effective `maxTokens` for such a call, or carry the usage on the failure.
- **Not done, by the brief.** The connection deadline is no later than the deadline of the call because `tokio::time::timeout` wraps the whole send, connect included. No test pins that apart from the stall tests, which stall after the connection.
