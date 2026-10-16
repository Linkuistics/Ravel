# what-ravel-is-for-and-its-actors-k78

## Goal

Correct the opening unit's account of the policy worker's working directory,
under `docs/book/contract.md`. Read both chapters as one document and change
only the draft charter's technical/teaching material.

## Context

This is the first correction run for this document, sent back by art k77.
Chapter 2, "Three different kinds of information", says: "The worker runs in a
private directory of its own". The normative specification,
`docs/specs/session-execution.md`, "Policy authority and runtime discovery",
says the worker starts in a private empty directory and moves to `/` before
registering modules or importing policy. The decision record
`docs/adr/policy-evaluation-precedes-the-launch.md` explains the move. Verify
the implementation and distinguish initial working directory, policy-time
working directory, the request's `cwd`, and the private directory that is still
removed during worker cleanup. Do not imply that policy is unable to change cwd.

The art output already exists: two C4 views, explanatory captions, alt text and
four new comparison/mapping tables. The private-directory cleanup in the
container figure is intentional and is not the erroneous working-directory
claim. Keep the caller-directory teaching and the distinction between policy
context and session-result context accurate.

The contiguous correction run is draft k78 → copy-edit k79 → art k80 → proof
k81. No stage is omitted: changed teaching reaches wording, figure relations
and final proof. Every successor is already live; do not append another tail.
A second correction run of an already rerun stage is escalation under the
editorial family procedure, not permission to regrow the chain.

## Done when

- The worker-directory explanation agrees with the verified specification and
  source, at the opening unit's level of detail.
- `task docs:book:build` and `task docs:examples` pass; figures remain coherent.
- Clear the correction note in the document brief when resolved, retain the
  proof handoffs, retire this leaf and leave the queued copy-edit leaf next.
