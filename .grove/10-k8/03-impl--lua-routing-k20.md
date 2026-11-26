# lua-routing-k20

## Goal

Implement Lua routing beside Rhai, as the spec `configuration-evaluation`
states it after `configuration-contract`. A `config.lua` routes every kind that
a `config.rhai` can route.

## Context

- The reworked spec `launcher/docs/specs/configuration-evaluation.md` and the decision
  records that `configuration-contract-k19` reworked.
- The survey `launcher/docs/research/lua-runtime-a.md`, section on the lua-routing
  implementation.

## Done when

- Ravel embeds the pinned Lua, and the build needs no separate Lua
  installation.
- The worker evaluates `config.lua` with the specified libraries, host calls,
  modules, bounds, cancellation and termination.
- The rule for a directory with both scripts works as specified.
- The Rhai module is separate from the Lua module and from the rest of Ravel.
  The Rhai tests pass unchanged.
- Tests at the command-line seam cover discovery, conflicts, host calls,
  modules, invalid values and errors. Others cover bounds, cancellation,
  protected paths and a script that runs a harness.
- The third-party notices include the Lua runtime and its bindings.
- The book is not this leaf's. The root leaf `book` revises it.

## Notes

- If the work is larger than one session, finish a working slice first. A
  likely first slice is evaluation and routing. Bounds and cancellation can
  follow in a sibling.
