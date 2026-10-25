# rhai-policy-increments-k6

**Reviews:** rhai-policy-increments-k3

## Goal

Read the planned subtree adversarially, and report findings without fixes. The subtree holds every root entry after this leaf, from `rust-2024-toolchain` to `owner-policy-conversion`. Judge whether the cut gives the smallest independently useful increments in dependency order, and whether each one leaves main working.

## Context

- Find the commit of `rhai-policy-increments-k3` by its handle, and read its diff. It adds the whole subtree.
- The root brief states what the workstream must deliver. The running log of `plan-k1` holds the settled requirements. GitHub issue 2 binds through its "Required architecture" and "Acceptance criteria".
- The design is `docs/specs/policy-evaluation.md`, the ADR set and `CONTEXT.md`. Its review is `rhai-policy-k4`, and the integration is `rhai-policy-k5`.
- The code is the Ravel workspace in this checkout. Check a claim about the code against the source.

## Done when

- Each finding names the entry, the defect and the evidence.
- The review checks every requirement scenario of the spec for an owner leaf that tests it at the CLI seam.
- The review checks every acceptance criterion of issue 2 for a leaf that delivers it.

## Notes

These doubts are the producer's own. Read each one against the source.

- Main keeps both policy paths until `typescript-removal`. Check that every earlier leaf can leave the TypeScript path and all its tests green. Check the interim refusal of `load_context` in `rhai-policy-worker`.
- The book changes in two nodes, before and after the removal. Check that the split keeps `task docs:examples` and `task docs:excerpts` passing at every handoff, and that the contract's reader checkpoints still fit.
- Check that no leaf is too big for one session. The candidates are `rhai-policy-worker`, `rhai-llm-anthropic`, `rhai-book-use` (first draft), `typescript-worker-cutover` and `typescript-delivery-removal`. Decide whether any of them needs to be a node now.
- Check the order inside `rhai-local-capabilities` and `rhai-llm-calls`. Look for a leaf that uses something a later leaf creates.
- The release leaves carry three consequences. The upgrade keeps Ravel 1.1.0 and `policy.ts` until the driver restarts. Ravel 1.1.0 refuses new keys in the shared `settings.json`. `task release:major` refuses outside the default workspace. Check that each consequence reaches the leaf that it affects, and that no leaf violates one.
- Check that the leaves hand a verifiable result to the next one, and that each body names its input.
