# book-first-half-k99


## Goal

Make the figures and their surrounding text across chapters 1–6 follow the
owner's amended book contract: shared visual styling, plain Ravel CLI wording,
and no generic diagram-notation tutorials. Own the resulting artifacts and
fixes as an art producer; this is not an inspection-only review.



## Context

- Contract: `docs/book/contract.md`, including the new reader assumptions,
  plain-vocabulary rule and whole-book style requirement. Read the preceding
  copy-edit `book-first-half-k98` and set proof `book-first-half-k67`.
- The owner clarified “styles” as **colours, fonts, line weights etc.** Check
  palette roles, font families and type sizes, stroke widths, arrow styles,
  borders and spacing across diagram types. Shared `_style.puml` and
  `_uml-style.puml` are starting points, not proof that every rendered figure
  uses consistent values. Read effective defaults and per-source overrides.
- Scope: all nine SVG/PlantUML pairs in `docs/book/src/figures`, the two shared
  style files, figure-related prose/captions/alt text across chapters 1–6,
  and the current conventions in `docs/book/figures.md`. Page-native tables
  retain the book's normal table styling. No renderer-pin change is requested.
- Readers know C4/UML/SysML notation or can look it up. Remove generic
  tutorials from prose and legends, retaining keys specific to this book:
  colour roles, software/process boundaries, lifetimes ending after confirmed
  reap/removal, attempted versus successful cleanup and outcome precedence.
- Use **Ravel CLI**, not “front”/“native front”, in diagram labels and text.
  Carry the preceding copy-edit's plain-language choices into captions, keys,
  alt text and adjacent explanations. Do not replace exact code identifiers.
- Preserve `docs/evidence/book-figures-k92`'s bounded renderer correction:
  single-worker rendering, retained metadata-only failures, original lost
  selection render's cause unknown. A mismatch calls for diagnosis, not
  blind update/retry.

## Done when

- All first-half figures use a coherent palette, typography, line/arrow
  weights and spacing, with choices in shared definitions where possible.
  Any notation-specific difference has a clear reason. Record effective
  styling from the sources and compare the actual renderings across figures.
- Generic notation teaching is removed, while book-specific keys and the
  technical conditions remain clear. Labels, captions, alt text and prose
  use the contract's reader level and Ravel CLI term consistently.
- Regenerate SVGs with the pinned tools, then run build, examples and figure
  comparisons against final unchanged inputs. Inspect chapter-width and
  full-size figures in reachable declared environments and record limits.
- Reconcile `docs/book/figures.md`'s current conventions. Keep historical
  viewing records as evidence of those earlier artifacts, not current style
  instructions. Clear this run's handed-forward entries.

## Notes

Proof `book-first-half-k100` is already queued immediately after this leaf;
cut no duplicate. The separate whole-book art `ravel-book-k101` later checks
chapters 1–9, 11 and 12 together, using the shared style established here.
This new owner requirement is not a second retry of an earlier unit defect.
