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
- This leaf sits after read 1 only as a sequencing anchor: read 1 cuts the
  second-half planning **before this leaf**, and that planner inserts all
  chapter units and relevant guide migration before this leaf. Read 2 stays
  after the whole-book proof that follows this art pass. Do not bypass missing
  chapters or treat the current first half as a completed-book sample.
- Chapter 10 is outside this node until a supported extracted component and
  consumer exist. Its later art applies the same shared style.

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
  read 2. If read 2 already exists as a later live sibling, use leaf-insert
  targeting that entry; otherwise append. Its body requires the full amended
  contract, source coverage, entry routes and visual consistency.

## Notes

This pass fixes artifacts; it is not a report-only review or a claim of
new-reader comprehension. The owner's read 2 follows the final proof.
