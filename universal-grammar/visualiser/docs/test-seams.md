# Test seams

The human agreed these two seams on 2026-11-17, for the first vertical slice of the grammar workbench. Every part of the slice is tested through them.

1. **Core interface.** A test case gives a grammar file and an optional sequence of symbols. The test compares the KDL output of the core with an expected file. These files also form the example suite of the KDL form. Property tests compare the tracer with a reference recogniser on random grammars and sequences. Run this seam with `cargo test -p workbench-core`.
2. **Workbench page.** A few end-to-end tests drive the page in a browser. They load a grammar, find one diagram for each rule and step through a trace. They check structure, not positions. Run this seam with `task page:e2e`.

No stand-in replaces the core inside the workbench. The end-to-end tests drive the real core through WebAssembly.

Small tests inside a Rust module are allowed where the seams cannot reach a case.

`task check` runs both seams. It also runs the format check, Clippy on the native and WebAssembly builds, Biome and `tsc`.
