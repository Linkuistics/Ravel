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
