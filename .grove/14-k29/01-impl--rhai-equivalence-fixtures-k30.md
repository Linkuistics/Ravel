# rhai-equivalence-fixtures-k30

## Goal

Freeze the results of the TypeScript policies as fixtures, while the TypeScript path still works. The Rhai translations then keep a reference after the removal.

## Context

- The input is the equivalence test of `rhai-example-policies`. Read its running log for the table of requests and for the skipped fields.
- The spec says that the increment which removes the TypeScript path first freezes these results as fixtures. See "Migration" in `docs/specs/policy-evaluation.md`.
- A fixture holds, for each request, the routing notice, the argument vector that the fake harness received, the delivered context and the refusal. It omits the fields that must differ between languages.

## Done when

- A fixture file exists for each translated policy: the static route table, the sample, the owner-shaped policy and, where it applies, the LLM example.
- The Rhai side of the equivalence test compares against the fixtures and needs no TypeScript policy.
- A test shows that a change to a fixture makes the comparison fail. This is the firing control.
- The test of the owner-shaped policy keeps its secondary jj workspace and its skip message.
- The old TypeScript side of the test stays in place until the cutover leaf. A comment names the leaf that removes it.
- `task check` and `task test` pass.

## Notes

- Generate the fixtures with a command, and record the command in the running log. Do not write them by hand.
- Run the generator once, review the result, and commit it. A later session must never regenerate a fixture to make a test pass.
