# ravel-book-k101


## Goal

Unify the completed book's diagram styling across chapters 1–9, 11 and 12
before whole-book proof and read 2. The owner's explicit concern is visual:
colours, fonts, type sizes, line weights, arrows, borders and spacing across
all diagram types. Use the `art` producer; no new art-review kind is needed.



## Context

- Contract: `docs/book/contract.md`. The owner assumes familiar diagram
  notation, wants prose for a 16-year-old software engineer and chose
  **Ravel CLI** for the command-line program. Apply those choices to labels,
  captions, keys and alt text as well as prose beside figures.
- The first-half copy-edit/art/proof correction is
  `book-first-half-k98` → `book-first-half-k99` → `book-first-half-k100`.
  Read its final shared styles and `docs/book/figures.md` as the baseline.
- This leaf sits after the second half's units and guide migration:
  `ravel-book-second-half-k148` inserted the chapter units (7, 8, 9, 11, 12),
  their figure modules, the set proof `book-second-half-k163` and the two
  fold-in leaves before this leaf, and cut read 2 (`book-read-two-k166`) after
  it. Read 2 stays after the whole-book proof that follows this art pass. If a
  unit above is still live when this leaf runs, do not treat the book as
  complete: stop and say so.
- The second half added figures: the outcome-precedence activity figure
  (`book-figure-activity-k153`), the escalation timing figure
  (`book-figure-timing-k156`), and the figures that the chapter art stages drew
  with the shared modules. The first half's art left the small text of its dense
  figures to this pass. Check chapters 7–12 for the same.
- Chapter 10 is outside this node until a supported extracted component and
  consumer exist. Its later art applies the same shared style.
- Chapter 8 art k172 added the six-participant `session-code-path` sequence.
  Its body text is about 8.4 px at the 750 px chapter width, with a full-size
  link for detail. Include it in the dense-text correction. Its SVG is
  299,179 bytes; the eleven-figure set is 2,240,789 bytes, above the first-half
  2 MB benchmark in `docs/book/figures.md`. Assess the complete set's size
  alongside its legibility. This is not a claim that the earlier set budget
  passes or that the agreed medium must change.

## Done when

- Enumerate every delivered figure from the complete book, read its source
  and effective style, and compare actual renderings across chapter and
  notation boundaries. Fix inconsistent palette roles, typography, line/arrow
  weights, borders and spacing through shared definitions where possible.
- Every deliberate visual difference has a concrete reason. Preserve each
  notation's meaning without generic tutorials. Ravel CLI naming and plain
  language agree across all figures and surrounding text.
- Updated SVGs match their sources; build, examples and figure checks pass
  against final unchanged inputs. Inspect declared reachable viewing surfaces
  and document what was not inspected. Shared convention documentation is
  current and understandable for a later chapter author.
- Cut the whole-book `proof` successor with the bare stem `ravel-book` before
  read 2. Read 2 exists as the live later sibling `book-read-two-k166`
  (`ravel-book-second-half-k148` cut it), so use `leaf-insert` targeting that
  entry. Its body requires the full amended contract, source coverage, entry
  routes and visual consistency, and the set proof `book-second-half-k163`'s
  record of inventories 4 and 5.

## Notes

This pass fixes artifacts; it is not a report-only review or a claim of
new-reader comprehension. The owner's read 2 follows the final proof.

## Decisions (running log)

- 2026-10-19: All chapter units and both guide fold-ins precede this leaf
  as retired work. The clean jj working change starts at `53e1b29c`, after
  k165. Read 2 remains live. This pass keeps the agreed crisp Typst/CeTZ
  renderer, palette and notation. It changes layout within that contract.
- 2026-10-19: The fourteen sources already share fonts, colours, stroke
  weights and arrowheads. Wide layouts and unwrapped keys make the effective
  text scale differ. First try bounded key wrapping and narrower measured
  sequence labels, then adjust the three C4 layouts. Preserve every relation,
  participant, branch and lifetime marker. Split a sequence into labelled
  panels only if wrapping cannot give a legible column view.
- 2026-10-19: Graph generation `2026-10-18T13:53:21Z` is ready, Tier 2.
  Figure-symbol search returns no nodes. Coverage lists the fourteen SVGs as
  deliberately excluded. Direct reads of the Typst sources and layout records,
  followed by rendered inspection, supply the figure evidence.
- 2026-10-19: Keep all fourteen figures and the existing tables. Additional
  worker/environment/test-seam/installation diagrams and a separate Pages
  diagram would repeat relations already carried by tables or linked views.
  These are editorial choices, not contract restrictions or medium failures.
- 2026-10-19: Shared keys wrap at 640 units. Sequence names use the bold
  entity face and result labels use their arrows' green. Sequence messages
  can use their actual arrow span after column placement. Narrower columns
  and C4 layouts retain all relations, branches and lifetime markers. The
  full code sequence stays continuous rather than split across panels.
- 2026-10-19: A wrapped deployment key exposed a measurement/drawing mismatch:
  157.33 units measured versus 177.72 drawn. Explicit inner text-edge settings
  in `_style.typ` make both 157.33 and restore the intended padding. Complete
  final figure views confirm the correction across keys and notes.
- 2026-10-19: Final body text is 12.3–16.7 pixels at the 750-pixel column.
  The set is 3,184,770 SVG bytes. Retain the agreed glyph-path medium and all
  explanations; the first-half 2 MB / 300 KB benchmarks do not pass. The
  complete size assessment, deliberate visual differences, per-figure
  measurements and viewing limits are in `docs/book/figures.md`.
- 2026-10-19: The final coverage check reports generation
  `2026-10-18T14:03:10Z`, with all fourteen SVGs excluded and no additional
  scope gaps or pagination. Changed figure files require source fallback;
  direct source/record reads and rendered views supply it. All old and new
  layout element IDs, shapes, arrow flags and dash flags agree in order.
  Only documented wording, participant-face and result-label-role changes
  differ apart from geometry. This is bounded evidence, not graph completeness.

## Validation

- 2026-10-19: All seven documentation checks pass: `task docs:book:build`,
  `task docs:figures` (fourteen sources), `task docs:figures:test` (including
  regeneration, isolation and refusal controls), `task docs:examples`
  (fourteen passed; 27 illustrative fragments not run), `task docs:excerpts`
  (eight passed), `task docs:ste -- --strict` (zero problems in 28 files),
  and `task docs:breaks -- --strict` (zero overlong runs in eleven files).
  SHA-256 comparison confirms all 3,525 tracked inputs outside `.grove/`
  stayed unchanged during these checks. The full Rust suite was not rerun
  for this figure and documentation change.
- Chrome on macOS showed all fourteen final figures completely at 750 pixels,
  including every section of the long sequences and activity figures. Actual
  mdBook views checked chapter 2's process view and role text, chapter 8's
  code-sequence opening, path table and full-size link, and chapter 9's timing
  key and explanation. The detailed record names the uninspected surfaces;
  this pass claims neither other themes/widths/platforms nor human comprehension.
- Whole-book proof is the next stage, before the existing owner read 2. The
  book node remains open; no review leaf or parent close is due.
