# Exposition and assurance — planned repetition 2

## Exact prompt

The exact prompt is preserved in the [campaign record](README.md#shared-execution-contract).

## Attempts

| Attempt | UTC start | Run directory | Wall time | Exit | Raw evidence |
|---|---|---|---:|---:|---|
| 1 | `2026-08-29T07:05:18Z` | `/private/tmp/grove-exposition-c.05MKyS/repetition-2-attempt-1/run-directory` | 1,200 s | 124 | [`evidence/repetition-2-attempt-1/`](evidence/repetition-2-attempt-1/) |
| 2 | `2026-08-29T07:25:31Z` | `/private/tmp/grove-exposition-c.05MKyS/repetition-2-attempt-2/run-directory` | 1,200 s | 124 | [`evidence/repetition-2-attempt-2/`](evidence/repetition-2-attempt-2/) |
| 3 | `2026-08-29T07:45:44Z` | `/private/tmp/grove-exposition-c.05MKyS/repetition-2-attempt-3/run-directory` | 1,200 s | 124 | [`evidence/repetition-2-attempt-3/`](evidence/repetition-2-attempt-3/) |

Each raw stream contains 29 events, no model tool call, and no final assistant
message. Each post-run manifest is empty. All three attempts are invalid because
the timeout exited nonzero and no final message exists. Atomic scores and total
successes are not applicable. The third invalid attempt exhausts the replacement
allowance, leaving this planned repetition short by one valid sample.
