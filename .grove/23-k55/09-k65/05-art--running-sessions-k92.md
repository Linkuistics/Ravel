# running-sessions-k92


## Goal

Settle chapter 5's figure-delivery reproducibility defect under
`docs/book/contract.md`, then leave the existing proof successor to read it.



## Context

`running-sessions-k91` read the whole chapter and found two local style
corrections, now landed. Its initial required tasks passed. A frozen-input
re-run of `task docs:figures` then failed with:

```text
figures: selection-cancellation.svg differs from what selection-cancellation.puml renders; run 'task docs:figures:update' if the change is intended
```

The check's exit trap deleted the failing scratch render. Its exact differing
bytes are unavailable. Before/after SHA-256 inventories of the book, scripts,
crate sources and workspace manifests were identical. Two direct renders,
six subsequent full checks and a final ordinary `task docs:figures` all matched
all seven committed SVGs. No source, SVG, pin or renderer command changed.
Passing retries do not establish a cause or justify silently accepting a flake.
`docs/book/figures.md` records this limitation and proof's own viewing scope.

The rendering seam is `scripts/figures.sh`: `verify_figures` copies input into
scratch, calls `render_all`, compares bytes and deletes scratch at exit.
`render_all` calls the pinned PlantUML over all sources in one invocation.
During investigation, sourcing the script, assigning `WORK` to an independently
created scratch directory, then calling `verify_figures check` preserved the
scratch output without changing the production check. Successful comparisons
are still in `/tmp/ravel-k91-render-a`, `/tmp/ravel-k91-render-b` and the
`/tmp/ravel-k91-verify.*` / `/tmp/ravel-k91-repro.*` directories on this host;
they are temporary supporting observations, not durable failed-run evidence.

## Done when

- Investigate the mismatch at the figure-delivery seam, retaining any failing
  fresh output and comparing its actual differing bytes with the committed
  figure. Distinguish observed cause from a hypothesis. Do not regenerate
  blindly, mask comparison failures or change renderer pins without the
  contract's authority.
- Make any justified correction with discriminating verification, or state
  the precise unresolved limit rather than claiming deterministic delivery.
  Preserve the whole chapter's figures, their explanations and local viewing
  evidence. Inspect changed output in reachable declared environments.
- `task docs:figures`, `task docs:examples` and `task docs:book:build` pass;
  record both failures and green runs, with stable checked inputs.
- The existing later proof sibling is the successor: cut nothing extra.

## Notes

This is the first art correction run for chapter 5. Draft and copy-edit are
omitted because the specific discrepancy concerns generated figure bytes;
neither narrative nor example bytes were implicated. A second art correction
against this document is an escalation under the editorial-family rule.
