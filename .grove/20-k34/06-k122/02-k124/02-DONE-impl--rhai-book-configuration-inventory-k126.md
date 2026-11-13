# rhai-book-configuration-inventory-k126


## Goal
Derive the configuration inventory from the current specifications, engine,
capabilities and settings. Map every public field, bound, import/export rule,
discovery rule and installed example to explanatory book text. Correct gaps.



## Context
Use `docs/book/contract.md` and the parent brief. The CLI inventory from k125 is
under `docs/evidence/rhai-book-cli-inventory-k125/`; reuse only after reconciling
its sources. Preserve historical evidence.

## Done when
The contract's configuration inventory is independently derived, checked against
all relevant source and mapped item by item to the book. Record exact source and
book locations, input digests, method and limits in a durable evidence artifact.
Correct discovered defects and rerun affected checks. Do not claim whole-book
proof, comprehension or owner Read 2.

## Notes

- Completed the source-derived configuration inventory at
  `docs/evidence/rhai-book-configuration-inventory-k126/README.md`.
  `inventory.json` maps 236 items to exact source, lookup and teaching
  locations. There are 234 covered items and two corrected items.
- Corrected the repository example description in chapter 4 and the
  encoded host-read reply bound in chapter 6. No runtime code changed.
- All five existing documentation checks passed through `mise exec -- task`:
  12 runnable examples, 22 excerpts, strict STE on chapters 4 and 6,
  strict prose-break checks on both chapters, and the mdBook build.
  The example runner separately reports 22 fragments not run.
- The 296 frozen subject hashes match before and after the checks.
  The evidence records method, graph coverage, per-item results, command
  statuses and limits. This completes only the configuration inventory.
  Whole-book proof, comprehension and owner Read 2 remain unclaimed.

## Decisions (running log)

- Derive this inventory directly from the current configuration and skill
  specifications, Rust validators and host registrations. Do not reuse k125's
  measurements. Keep the item map and fresh digests under
  `docs/evidence/rhai-book-configuration-inventory-k126/`.
- Correct chapter 4's description of `harness-table.rhai` to “repository
  example”. `scripts/release-common.sh` installs only `static.rhai`,
  `sample.rhai` and `llm.rhai`. Chapter 6 already distinguishes these files.
- Keep the claim bounded to configuration coverage. The later proof leaf owns
  whole-book reading and the owner's Read 2. Do not add a competing reviewer.
- Correct the encoded host-read reply bound to 38,541,312 bytes. The read
  checks `FRONT_FRAME_BYTES` in `rhai_worker/host.rs`. `frame.rs` selects the
  larger evaluation frame, not the 19 MiB harness-reply calculation. The
  current configuration specification agrees with the implementation.
