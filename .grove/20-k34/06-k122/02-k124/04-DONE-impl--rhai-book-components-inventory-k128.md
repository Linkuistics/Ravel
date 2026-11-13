# rhai-book-components-inventory-k128


## Goal
Derive the component inventory from both Rust packages and the separate Rhai
worker. Map all ten contract component obligations to explanatory book text.



## Context
Use `docs/book/contract.md`, component evidence and the parent brief. Chapter 8 is
an orientation; do not silently turn its excerpts into a complete reconstruction.

## Done when
Caller semantics, authority, no fallback, direct launch context, first result,
successful reap and group cleanup, cancellation, terminal recovery, exact stdout
and worker identity are independently checked. Record source/book locations,
digests, method and limits in durable evidence. Correct gaps and run affected
checks. Leave the fresh whole-book read and owner Read 2 to the final proof.

## Notes

## Decisions (running log)

- 2026-11-14: Keep this inventory at the contract's ten component obligations.
  Chapters 7–9 explain the boundaries and failure paths; chapter 8 remains an
  orientation, without a source-reconstruction claim. Record each obligation's
  source and teaching locations in `docs/evidence/rhai-book-components-inventory-k128/`.
- 2026-11-14: Source comparison found no component teaching gap. Retain the
  channel/launch pairing and conditional terminal-recovery limits already in
  the book. Run the chapter 7–9 examples, excerpts, strict STE and breaks checks,
  and local book build against frozen inputs. These checks do not replace the
  whole-book read or owner Read 2 assigned to k130.
- 2026-11-14: All ten obligations are mapped in the durable component inventory;
  all 73 source files are classified and exact path/scope coverage is recorded.
  The five documentation checks passed: six complete examples, 22 excerpts,
  strict STE and prose-break checks for chapters 7–9, and the local book build.
  All 372 measured inputs stayed unchanged; all 369 preceding lifecycle inputs
  still match. Keep runtime qualification, fresh whole-book review, owner Read 2
  and published rendering with k130. No runtime or book source correction was
  needed.
