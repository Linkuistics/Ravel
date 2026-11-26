# configuration-contract-k19

## Goal

Design the Lua configuration contract for routing. Rework the spec
`configuration-evaluation` and the decision record
`configuration-script-uses-rhai` for a 2.x release that reads Lua and Rhai.

## Context

- The survey `launcher/docs/research/lua-runtime-a.md`.
- The owner's reactions in the running log of `configuration-experience-k7`.
- The running log of `plan-k1`, Topic 3, holds the owner's words on modular
  configuration.

## Done when

The spec and the decision records settle each item below.

- The pinned Lua runtime and bindings, with the reason against each rejected
  candidate.
- The module contract for routing: what a script returns, its version, how it
  refuses, and how it loads reusable local modules.
- The libraries that a script can use, and the host facilities that stay:
  measured reads, JSON, environment grants, discovery and a harness run.
- The conversion of arrays, maps, empty tables, `nil` and JSON `null`, and the
  error for an invalid or misspelt field.
- The worker, its bounds, its cancellation and its termination inside a native
  call. An embedded runtime is not treated as a sandbox.
- The rule when `config.lua` and `config.rhai` both exist. It never selects the
  wrong script in silence.
- The one place that registers a configuration language. Removing Rhai later
  deletes its module and its tests, and changes only that place.
- A table-oriented example and a programmable example.

## Notes

- Leave skill selection to `skill-contract`. Make the routing contract
  extensible for selection fields, and say how.
- The owner agrees the contract before this leaf retires. It is an agreement
  point, so record the owner's restatement.
- The spec is load-bearing for the whole product. Cut a `review-design`
  sibling as the last act, unless the session finds a reason against it and
  records that reason.
