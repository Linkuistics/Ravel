# rhai-llm-bounds-k17

## Goal

Bound every byte that an LLM endpoint can make the Ravel CLI hold. Prove that a call in flight stops at the deadline and on cancellation. An endpoint cannot exhaust the memory of the CLI, and it cannot hold the CLI after a stop.

## Context

- The input is the review finding F2 of `rhai-policy-k4` and its integration in `rhai-policy-k5`. Find both with `grove-llm resolve`. The spec sections are "LLM endpoints and credentials", "Evaluation budgets and their composition" and "Cancellation and deadlines".
- In `rig-reqwest` 0.45.0, `into_response` collects a successful body with `response.bytes().await`. `non_success_status_error` collects an error body with `response.text().await`. Both are unbounded, so Ravel must not use them.
- The first LLM leaf built Ravel's own client over the `HttpClientExt` interface of Rig. Put the bound there, before Rig decodes anything.
- Rig keeps the raw provider document in `CompletionResponse.raw`. Check that the bound still holds the memory of that copy.

## Done when

- A successful response body has at most 8 MiB. A longer body fails the call as `output_limit`.
- Ravel reads at most 64 KiB of a failure body and then closes the connection. The call keeps the class of its status, so a large 429 page stays `rate_limited`. Its message holds that part of the body.
- Ravel counts the body bytes as they arrive. A declared length beyond the bound fails at once. A body without a declared length stops at the first byte beyond the bound.
- Ravel decodes no compressed body, so the bound counts the bytes that Rig receives.
- The answer text has at most 1 MiB. A longer text fails the call as `output_limit`.
- Tests at the CLI seam cover these scenarios: "Oversized successful body", "Large failure body" and "Stalled endpoint". The first one answers 200 with short text and a 9 MiB metadata field, with no declared length. The fake endpoint sees its connection close before it sends the whole body. A further case has a declared length beyond the bound.
- A stalled endpoint ends the evaluation with exit 124, and the fake endpoint sees its connection close.
- A TERM during a call in flight exits 143, the fake endpoint sees its connection close, and nothing launches. A timeout then signal case follows the spec rule that a signal wins.
- `task check` and `task test` pass.

## Notes

- Dropping a request cancels local work only. A provider may finish, and may bill, a request that it already accepted. The spec says so, and no test needs to prove otherwise.
- The connection deadline is no later than the deadline of the call.
