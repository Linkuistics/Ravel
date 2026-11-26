I can’t produce the exact authoring plan from this environment because I can’t inspect `targets/ocaml/check_floor.ml`.

Every attempted read-only shell command fails before execution with `sandbox-exec: sandbox_apply: Operation not permitted`, and there is no exposed filesystem MCP resource to read the file instead. Because your requested output depends on exact line ranges and exact source coverage, anything beyond that would require invention.

If you paste `targets/ocaml/check_floor.ml` here, or rerun with working read-only shell access, I can return in one pass:
- the exact source inventory plus unknown/excluded material
- a concept-oriented reader sequence
- a fragment ledger with root, child fragments, ranges, and insertion relationships
- one low-resolution worked execution through stage boundaries
- the exact pre-publication mechanical checks
