# lua-runtime-k18

## Goal

Survey the Lua runtimes and Rust bindings that Ravel could embed as its
configuration language. Issue 6 names Lua 5.4 and `mlua` as candidates, not
choices.

## Context

- Ravel evaluates a configuration script in a separate worker process today,
  with explicit capability grants, measured reads, resource bounds and
  cancellation. The spec `launcher/docs/specs/configuration-evaluation.md` states that
  contract for Rhai.
- Root brief, "Research to commission". Issue 6, steps 2 and 3.

## Done when

The survey answers these questions for each consumer below, in sections named
by the consumer's subject.

- **The configuration-contract design** (`configuration-contract`):
  - Which Lua versions and runtimes are candidates? Compare PUC Lua 5.4, Lua
    5.5 if released, LuaJIT and Luau on stability, licence and maintenance.
  - Which Rust bindings are candidates? Compare `mlua`, `rlua` and others on
    maintenance, safety and vendoring of the Lua source.
  - Which standard libraries are safe to expose in a configuration? How do
    real embedders restrict module loading?
  - How do Lua tables map to Rust values? Name the known traps with arrays,
    maps, empty tables, `nil` and JSON `null`.
  - Why is an embedded runtime not a sandbox? Cite the known escapes through
    the debug library, `load`, bytecode, native modules and memory exhaustion.
- **The lua-routing implementation** (`lua-routing`):
  - How do embedders bound memory, instructions and wall time? How does each
    fail when a script ignores a bound?
  - How does a host stop a script that is inside a native call?
  - Which build features vendor Lua into a static release binary on macOS and
    Linux? What does that do to the release archives?
- **The release** (`release`): which licence notices does the chosen runtime
  require?

## Notes

- Search for what went wrong after years of real use. Neovim, Redis, Roblox,
  game engines and WezTerm are embedders with public post-mortems.
- The fallback to beat is "Lua 5.4 through `mlua` with vendored source". Show
  where it fails, with a citation.
- Record silence where the search found no primary source.
