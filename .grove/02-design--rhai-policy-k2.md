# rhai-policy-k2

## Goal

Establish how Ravel 2.0.0 evaluates Rhai owner policies with Rig-backed LLM
access. Deliver a spec and a reworked ADR set that the owner can agree before
planning cuts the work.

## Context

- The "Design questions to resolve" in issue 2 are this session's work. Its
  "Required architecture" and "Acceptance criteria" bind as written.
- The running log of `plan-k1` holds what the owner settled. Synthesise the
  spec from it, and do not interview the owner again.
- This work contradicts the ADR `policy-evaluation-precedes-the-launch`.
  Rework the ADR set in place, as `ADR-FORMAT.md` directs.
- The owner's personal policy, `~/.config/ravel/policy.ts`, has the shape that
  the migration guide must translate. Read it, and leave it unchanged.
- Grove's helpers in `/Users/antony/Development/grove/assets/ravel-policy/`
  show why each capability in the running log exists.
- Chapters 4 and 6 of the book document today's owner-facing contract.

## Done when

- A spec in `docs/specs/` describes how Ravel evaluates a Rhai policy. It
  covers the host capabilities, the budgets, cancellation, the typed
  boundaries, the errors and the migration.
- The spec's "Test seams" section records the seams that the owner agreed.
- Each design question in issue 2 has an answer in the spec or in an ADR.
- The ADR set is a minimum coherent set for the new design.
- The spec names the pinned Rhai and Rig versions. This session verified each
  API that the design relies on against their source at those versions.
- `CONTEXT.md` gives "provider" one meaning, and it defines each new term.

## Notes

- If you cut a `review-design` leaf, insert it before the planning leaf with
  `grove-llm leaf-insert`. Planning must read a reviewed design.
- The isolation question needs particular care. Today a separate process
  evaluates the policy, and Ravel can kill that process at the deadline. Do not
  assume that a thread keeps the same guarantees. Issue 2 says this too.
- Rig released seven minor versions between June and October 2026. State in the
  design how the tests prove a Rig upgrade, because upgrades come often.
- The running log also fixes which policy file 2.0.0 uses when both exist. The
  local upgrade depends on that rule.
