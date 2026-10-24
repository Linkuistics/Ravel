# rhai-policy-k4

**Reviews:** rhai-policy-k2

## Goal

Read the Ravel 2.0.0 policy design adversarially, and report findings without
fixes. The design is the spec `docs/specs/policy-evaluation.md`, the ADR set in
`docs/adr/`, the glossary changes in `CONTEXT.md` and the views in
`docs/design/rhai-policy/`.

## Context

- Find the commit of `rhai-policy-k2` by its handle, and read its diff.
- GitHub issue 2 binds through its "Required architecture" and "Acceptance
  criteria". Its "Design questions to resolve" are what the design must answer.
- The running log of `plan-k1` holds the requirements that the owner settled.
  The running log of `rhai-policy-k2` holds the design decisions, and the owner
  chose three of them directly.
- The spec names Rhai 1.27.0 and Rig 0.45.0. Check a claim against the crate
  source from crates.io at that version, not against memory.

## Done when

- Each finding names the artifact, the section and the defect, with evidence.
- The review checks each design question and each acceptance criterion of issue
  2 against the spec or an ADR.

## Notes

These doubts are the producer's own. Read each one against the source.

- The spec claims that killing the worker never orphans host work. Check every
  host capability, module reads included, for a path that a kill leaves open.
- On macOS the worker re-executes the canonical path of the binary. Check
  whether a Homebrew upgrade during a session can break that path or replace
  the build. Then check that the identity check covers each case.
- Check that the engine's strict property mode and optional fields that hold
  `()` work together in Rhai 1.27.0.
- The schema subset may not meet the strict rules of a provider, such as
  required properties and `additionalProperties: false`. Decide whether such a
  call fails clearly.
- The routing notice moves to schema version 2. Decide whether that contradicts
  "callers do not change", or whether no caller reads the notice.
- Check the token accounting when an endpoint reports no usage. Also check how
  the deadline of a call composes with the whole deadline.
- `json_encode` sorts keys, but Grove's provider receipt has a fixed member
  order. Check that the spec gives Grove's port a workable path.
- The ADR set should be a minimum coherent set. Check that no record restates
  the spec, and that every citation resolves.
