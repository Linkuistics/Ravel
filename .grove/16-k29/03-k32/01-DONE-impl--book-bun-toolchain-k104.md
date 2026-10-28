# book-bun-toolchain-k104


## Goal
Give the book tools their own pinned checkout-local Bun fetch, so deleting the TypeScript worker dispatcher cannot remove book setup.

## Context

The existing fetch is `setup_bun` in `crates/ravel/scripts/dispatch.sh`. The book's `ste.sh` and `breaks.sh` execute `target/tools/bun`; `BOOK_TOOLS_DIR` already redirects the book tools. Preserve Bun 1.4.2 and the three existing archive digests. Use `scripts/book-bun.sh` as the new owner. The old dispatcher remains unchanged until k106 removes delivery consumers and deletes it.

The graph CLI refused startup: “CBM CLI could not start because a pre-coordination or unverified CBM generation is active; close all CBM sessions and commands, then retry”. No graph generation or coverage result was available; source searches mapped the script and task consumers instead.

## Preservation ledger

| Surface | Consumers | Disposition and destination | Verification |
|---|---|---|---|
| Checkout Bun version, archive digests and executable path | STE, prose-break checks, setup | Preserve 1.4.2 and the three digests; move setup to `scripts/book-bun.sh`, retain `target/tools/bun` by default | Offline setup controls, `task setup`, both prose-check control suites |
| `BOOK_TOOLS_DIR` | Book tooling and integrity instruments | Use the same override for Bun setup | Private tool directory in the integrity suite |
| Worker runtime/build identity, install and release | Dispatcher, frozen incumbent, release scripts | Keep unchanged in this child; k105/k106 own removal | No dispatcher diff; no product source changes |
| Archive manifest, notices and installed files | Release and formula | Keep in this child; k106 changes them together | k106 acceptance criteria |

## Implementation plan

1. Add `scripts/book-bun.test.sh`, following the existing offline book-integrity instrument. Exercise fresh fetch, verified-cache reuse, corrupt-cache recovery, corrupt-download refusal, wrong-pin refusal and failed fetch in private directories. Run it before adding the implementation and observe failure.
2. Add `scripts/book-bun.sh` with `setup` and `version`, source-safe definitions, the existing host/package mapping and pinned digests. Verify and extract the same private archive copy. Keep the checkout path and support `BOOK_TOOLS_DIR`.
3. Point `task setup` at the book fetch, add `book:bun:integrity`, and run that control from `task check`. Correct book/figure comments that tied them to dispatcher identity. Do not remove worker dependency setup until k105.
4. Run the new control and shellcheck, `task setup`, the prose tool controls, both book checker suites, and complete examples/excerpts. Inspect the diff, record evidence, retire k104 and seal its focused jj change.

## Done when

- `task setup` obtains the book Bun through `scripts/book-bun.sh`; the old dispatcher does not own book setup.
- Bun is pinned to 1.4.2 with the existing three archive digests, default path `target/tools/bun` and the book tools' directory override.
- The offline integrity suite demonstrates successful fetch/reuse/recovery and rejects a corrupt archive, wrong digest and failed download.
- The prose tools and complete book examples/excerpts, including checker controls, pass.
- The worker dispatcher and historical observations remain unchanged, with the remaining removal and packaging work explicit in k105/k106.

## Notes

## Decisions (running log)

- 2026-10-28: Preserve the existing tool version/digests and keep the dispatcher unchanged. Moving its definitions would change its worker identity and the incumbent's measured inputs before delivery removal; a separate book owner is temporary duplication with an explicit deletion owner in k106.
- 2026-10-28: Baseline `mise exec -- task docs:excerpts` passed all nine excerpts, and `mise exec -- task book:integrity` passed its offline fetch/cache/refusal controls. The working copy was empty before decomposition.
- 2026-10-28: The new integrity instrument first refused because `scripts/book-bun.sh` did not exist. After implementation, its fresh-fetch, cache-reuse/recovery, corrupt-download, wrong-pin and failed-fetch cases pass against private directories. The wrong-pin case deliberately rejects the real archive. Keep the repository's existing standalone `.test.sh` instrument convention rather than adding a test framework for this extraction.
- 2026-10-28: `mise exec -- task setup` passed with the book Bun fetch first, followed by mdBook, figures and the still-required worker dependencies. `mise exec -- task build` passed. Shellcheck passed the new scripts and changed consumers. The dispatcher was not edited; its pin remains only for the delivery consumers that k106 removes.
- 2026-10-28: `mise exec -- task book:bun:integrity docs:ste:test docs:breaks:test docs:excerpts docs:excerpts:test docs:examples docs:examples:test` passed. Every STE/prose-break control and their mutation runs passed; excerpt controls killed all 24 deliberate checker mutations. Nine excerpts and 22 complete book examples passed (57 fragments were outside the execution check). The example checker controls passed, including absent worker, stale source and CLI-seam drift. Full local output: `target/book-bun-validation.log`.
- 2026-10-28: `mise exec -- task check` passed formatting, shellcheck, existing worker/probe/type/integrity checks, the new book Bun integrity check, fake harness checks, Clippy, terminal recovery and the workspace suite: 602 passed, 48 skipped. Full local output: `target/book-bun-check.log` and `target/check.Gb9CZg`. No product source changed. This host is macOS arm64; the existing Linux package digests and mapping are preserved but were not executed in this child.
- 2026-10-28: Before recording these results, the validation subject manifests matched byte for byte across both book validation and `task check`: all 3,715 versioned files, plus `target/tools/bun`, its host archive `bun-darwin-aarch64-1.4.2.tgz`, mdBook, Typst and `target/debug/ravel`. Local manifests: `target/book-bun-subject.{before,after,final}.sha256`; each has SHA-256 `28970d8b6818e91dd6e8f90a5bcf07263cd9277fbeaab9a8775103ae9ab5dc20`. Subsequent task-note/retirement edits are bookkeeping after measurement.
