# rhai-book-use-k95


## Goal

Make the figures in chapters 1–6 agree with the corrected procedures and configuration names at the declared reading width.



## Context

- Read draft `rhai-book-use-k93`, copy-edit `rhai-book-use-k94`, Handed forward, the contract and all lessons.
- `configuration-names-k92` may move source paths used by the figures. Check the actual files and labels.
- Proof `rhai-book-use-k96` already follows. Cut no successor.

## Done when

- Read each figure in the corrected chapters. Its names, ordering, paths and alternative agree with the prose and implementation.
- Use the admitted medium and existing sources. Add a figure only for a concrete reader question.
- Reproduce the figures with pinned tools and inspect changed visuals at chapter width. State the inspection's bounds.
- Source and installed-file views use the names that now exist. Reconcile shared alternatives or identify the later chapter's exact work.
- Figure checks, required prose checks and the book build pass. Close art entries in Handed forward and record new lessons.

## Notes

- Prose and a file tree may explain setup best. The draft owns that structure. No assumed figure count requires decoration.

## Decisions (running log)

- Show the installed notice file in the directory view. Its exact location is part of the installed layout, and the table already names it. Stack the supporting directories to preserve reading width.

## Verification and handoff

- The installed-files view now groups the notice and examples below `share/ravel`. Its alternative and chapter 6 table name the same files. `scripts/release-common.sh` installs the listed notice; the source-responsibility view names existing crates and worker paths. The shared view appears in chapter 6; the later chapters retain their own source and delivery reconciliation.
- Read the figure placements, alternatives and adjacent role sentences in chapters 1–6. The setup sequences already have file trees, complete commands and tables. Declined a separate setup diagram as an editorial judgment: it would repeat those relations without answering a new reader question. This is not a limitation of the medium. No new documentation lesson emerged.
- Reproduced all 18 figures with Typst 0.15.1, CeTZ 0.4.2 and DejaVu 2.37. Inspected the changed figure in Safari in an isolated macOS guest at a 750-pixel column on a local test page. The full figure and key were visible and legible; this did not inspect the complete book or other widths.
- Proof `rhai-book-use-k96` already stands next. It owns the full document gate and the owner's Read 1 handoff.
- After setup, `task docs:figures` matched all 18 figures, the strict STE and prose-break checks passed for chapters 1–6, and `task docs:book:build` passed.
