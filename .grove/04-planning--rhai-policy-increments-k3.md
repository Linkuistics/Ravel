# rhai-policy-increments-k3

## Goal

Cut the agreed `rhai-policy` design into the smallest independently useful
increments, and order them by dependency. Each increment leaves main working.

## Context

- The spec and the ADR set from `rhai-policy-k2`, and any review of them.
- The running log of `plan-k1`. Main makes no release until 2.0.0. The
  workstream ends with the release, the Homebrew upgrade and the conversion of
  the owner's local configuration.

## Done when

- The tree holds each increment as ordered leaves or nodes, and each body names
  its input.
- The last increments cover the documentation, the removal of the TypeScript
  path, the release, the Homebrew upgrade and the local conversion.

## Notes

- Give the upgrade leaf the two consequences in the running log of `plan-k1`.
  The upgrade keeps Ravel 1.1.0 and `policy.ts` until the driver restarts.
- The book changes for 1.1.0 ran through the editorial kinds: draft, copy-edit,
  art and proof. Consider the same pipeline for the Rhai chapters.
