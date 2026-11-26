# lua-configuration-k8 — brief

## Goal

Make embedded Lua the primary configuration language of Ravel for routing. A
`config.lua` selects the harness, the model and the effort for each kind, as a
`config.rhai` does today. Rhai keeps its 2.0 behaviour exactly.

## Done when

- Ravel embeds a pinned standard Lua, and it needs no separate Lua
  installation. A release smoke shows that.
- A `config.lua` routes every kind that a `config.rhai` can route. The rule
  for a configuration directory that holds both files is implemented and
  documented.
- Rhai reads every 2.0 configuration script as before, and it gets no new
  feature.
- The Rhai code is one module. A later major removes it by deleting that module
  and its tests, and by changing the one place that registers the language.
- A build without the Rhai module passes the Lua tests at the command-line
  seam.
- A Taskfile task on the owner's machine compares the owner's real
  `config.rhai` with its Lua translation. It covers every configured kind and an
  unknown kind. The results match, and they stay outside the repository.
- The spec `configuration-evaluation` and the decision record
  `configuration-script-uses-rhai` match the delivered contract.

## Decomposition

This is the first working increment of the product. It changes routing only,
and skill selection is the next node. So a user can adopt Lua for routing
before any skill feature exists. The leaves share the Lua runtime, the worker
and the Rhai coexistence, which the skill leaves need only through the
delivered contract.

1. `lua-runtime`: a research survey of the Lua runtime and its bindings.
2. `configuration-contract`: the design of the Lua configuration contract.
3. `lua-routing`: the implementation of Lua routing beside Rhai.
4. `configuration-parity`: the owner comparison, the build without Rhai and
   the release smoke.

## Pointers

- Decision records: `configuration-script-uses-rhai`,
  `configuration-evaluation-precedes-the-launch`,
  `a-configuration-script-asks-a-model-through-a-harness-run`.
- Spec: `launcher/docs/specs/configuration-evaluation.md`.
- Terms: **Configuration script**, **Rhai worker**.
- The seams in play are the command-line seam, the owner comparison and the
  build without Rhai. The root brief states them.
