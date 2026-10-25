# rhai-json-check-k62

## Goal

Add the data helper `ravel::json_check` to the worker. A policy checks a parsed value against a schema of the subset, and it catches the mismatch. This is the expand step. `ravel::llm` keeps working until the next leaf removes it.

## Context

- The input is "Data helpers" in `docs/specs/policy-evaluation.md`. It holds the subset table, the nesting bound of 32, the location style, and the codes `schema_invalid`, `json_mismatch` and `json_unencodable`.
- `rhai-llm-structured-output` built the checker for `ravel::llm`. It lives in the module `src/llm/schema.rs`, and its tests are in `tests/rhai_llm_schema.rs`. The checker does not depend on Rig.
- The other data helpers run in the worker and send no message. `json_check` does the same.
- The wording of messages changes. A key of the checked value belonged to an endpoint before. It now belongs to the policy's own data, but the message still never holds a part of the value.

## Done when

- `ravel::json_check(value, schema)` returns the value unchanged when it matches. The helper runs in the worker and sends no message to the CLI.
- A value that does not match throws `json_mismatch`. Its message gives the location, such as `value.items[1].name`, and holds no part of the value.
- A keyword outside the subset, or a nesting deeper than 32 levels, throws `schema_invalid`. The message names the keyword or the schema, such as `schema.properties.a.format`. A script can catch the throw.
- A value with no JSON form throws `json_unencodable`, as `json_encode` does.
- The checker no longer lives under `llm`. It sits in a module that both `ravel::llm` and the helper use, so `ravel::llm` still passes its tests.
- Tests at the CLI seam cover a match and a mismatch with its location. They cover a schema outside the subset and a nesting past 32 levels. They also cover a value with no JSON form. They also cover three rules of the spec. An integer matches a float with no fraction. A keyword applies to one kind of value. `enum` accepts scalars only.
- The tests run a real `ravel start-session` against a sandbox home. The firing control of each refusal case sits in the same test.
- `task check` and `task test` pass. `task docs:examples` and `task docs:excerpts` pass.

## Notes

- Port the cases of `tests/rhai_llm_schema.rs` that test the checker. Leave the cases about the place of the schema in a request body. The next leaf removes them with `ravel::llm`.
- Do not remove `ravel::llm`, its budgets or its endpoints here.
- `docs/book` may quote the moved module. Fix a moved excerpt in this leaf.
