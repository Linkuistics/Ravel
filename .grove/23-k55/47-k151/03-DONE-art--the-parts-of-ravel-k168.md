# the-parts-of-ravel-k168

## Goal

Take `docs/book/src/07-the-parts-of-ravel.md` through the art stage under
`docs/book/contract.md` and `docs/book/figures.md`. Decide whether the
chapter needs a figure on its own page, and make the figure text, captions
and alt text follow the prose contract.

## Context

- The node brief's “Handed forward” list holds the art decision. Chapter 7
  links to the process-and-authority figure in chapter 2
  (`02-actors-and-words.md#the-managed-process-and-its-result`) and to the
  source and installed views in chapter 6
  (`06-reference.md#installed-layout`). It has no figure of its own. Seven
  tables carry the parts, owners, conditions and interfaces. Clear the
  handoff entry when you decide it.
- The contract makes every figure a direct entry. A figure that repeats a
  chapter 2 or chapter 6 view on this page needs the local context of this
  chapter in its caption and alt text. Reuse the proved layout record where
  the question is the same. A new figure is justified only by a reader
  question that the tables and the linked figures do not answer.
- Copy-edit `the-parts-of-ravel-k167` changed sentences and table cells
  only. No heading, anchor, section order, table row or code block changed.
  The chapter now says “managed process” throughout, as the rest of the book
  does. It links “managed process”, “refusal”, “confinement” and
  “persistent runner” at their first use. Figure labels follow the same
  words.
- The chapter passes the strict STE and prose-break checks. No prose run is
  above 450 words, so a figure is not needed as a break.
- Small text in the dense shared figures belongs to whole-book art
  `ravel-book-k101`, not to this leaf.

## Done when

- The art handoff in the node brief is decided and cleared. If a figure
  changed or is new, `task docs:figures` passes and `docs/book/figures.md`
  records the dated view.
- Strict STE, prose-break checks, book build and complete examples pass.
- Last act: add the `proof` leaf under this node, with the bare stem
  `the-parts-of-ravel`, unless a live later sibling already holds that stage.

## Decisions (running log)

- 2026-10-18: Read the whole chapter against the art charter. It has eight
  tables, not seven: parts, decision owners, worker conditions, SDK, runner,
  result knowledge, loose-contract changes and installed-pair inputs.
  Keep those tables and the links to the shared views in chapters 2 and 6.
  Add an adjacent takeaway to each table. This work changes figure text only.
- Declined figures: a repeated process-authority view, a repeated source
  component view and a repeated installed-file view. Each is an editorial
  judgement. The local tables carry the chapter's comparisons and mappings,
  and its prose states the process and package boundaries without a link.
  The linked figures offer the same relations as optional depth. Neither a
  contract prohibition nor a limit of the medium prevents their reuse here.
- A new worker sequence is also declined as an editorial judgement. The
  worker-boundary table and its adjacent order explanation carry the required
  before-launch relation. Chapter 8 owns the detailed session sequence.
- No architecture decision changes. The ADR set stays unchanged. No guide
  sentence moves in this stage; draft k152's destination tables still apply.
- The decision table's last column now says “Information at the boundary”.
  Its rows include both received and sent information, so the former
  “What the other parts receive” heading did not describe all its cells.
  The cells and their facts stay unchanged.
- The first strict STE check rejected “permitting” in the new worker-table
  takeaway. The sentence now says “prevents a managed launch after an
  invalid response or failed cleanup”.

## Verification

- Read the whole chapter, the contract, the figure conventions, shared word
  decisions, glossary, cited ADRs and root-to-leaf brief chain. Tier 2 graph
  coverage at generation `2026-10-18T08:25:38Z` reported no recorded issue and
  matching metadata for the relied-on documentation paths. This art pass
  makes no new claim of source or runtime verification.
- Inspected all eight tables and adjacent text in locally served mdBook
  through Chrome on macOS, at a 1227-by-919 viewport and a 750-pixel column.
  The changed heading and final takeaway were inspected again after editing.
  `docs/book/figures.md` records the view and its limits.
- Final checks all exited 0: strict STE on chapter 7 (zero problems), strict
  prose breaks (zero runs above 450 words), book build, figure comparison
  (nine matches), and complete examples (13 passed, zero failed or skipped,
  six labelled fragments not run, zero page errors). Chapter 7 has no code
  block. The example task rebuilt the matched CLI and worker.
- The first whole-tree digest overlapped the first checks. A second complete
  check pass ran after that digest finished, with no edits during either
  pass. All 3,468 tracked files outside `.grove/` have the same per-file
  SHA-256 values before and after the final pass. This includes the chapters,
  figure assets, contracts, scripts, manifests, lockfiles and crate sources.
  The sorted manifest digest is
  `a879d8f3566dcd8c5345b57b85670e11e823b3a6f76ef2f05e3c308eddb264b5`.
  Scratch manifests are under `/tmp/ravel-art-k168.vMkKxi/`; they are not
  durable evidence or book assets.
- No in-session reviewer was needed. The next stage independently reads the
  whole chapter and verifies the technical claims, inventory and entry routes.
  No unresolved art defect needs a correction run. The node stays open for
  its proof leaf.
