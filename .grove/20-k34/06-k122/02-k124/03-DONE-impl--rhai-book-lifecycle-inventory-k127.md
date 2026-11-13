# rhai-book-lifecycle-inventory-k127


## Goal
Derive the lifecycle inventory from the current session and result/job
specifications and implementation. Map every outcome and failure edge to the book.



## Context
Use `docs/book/contract.md` and the parent brief. Include missing/stale result
context, refusal, failure, timeout, exec failure, cancellation, cleanup, nesting,
terminal recovery and resource limits. Reconcile earlier inventory artifacts.

## Done when
Every contract lifecycle item has exact source and book locations in a durable
evidence artifact, with derivation method, digests and limits. Correct gaps and
rerun affected checks. Retain all unqualified platform and observation limits.
This is not the final whole-document proof or owner Read 2.

## Notes

## Decisions (running log)

- The inventory derives from both lifecycle specifications and the current Rust implementation.
  The evidence will retain separate source, specification and book locations for each assessed outcome.
- Chapter 8 needs one correction: `--exit-dir` overrides `TMPDIR` for the managed result channel.
  `run.rs:93` and the session specification establish that choice.
- The recorded checks will measure documentation after all edits.
  Existing evidence will be reconciled by file digest, without reusing its runtime or reader claims.
  The scheduled whole-book proof remains separate.
- The durable artifact is `docs/evidence/rhai-book-lifecycle-inventory-k127/README.md`.
  Its 134 items map all 16 contract classes to source and book locations.
  All five affected documentation checks passed, and all 369 subject digests remained identical across those checks.
- The inventory retains platform, harness, cleanup, rendering and reader limits.
  No runtime code changed, and this leaf supplies no new Linux, real-harness or reader observation.
