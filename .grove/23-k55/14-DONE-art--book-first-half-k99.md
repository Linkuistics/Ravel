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

## Copy-edit handoff resolved

Updated the named alt text, caption, figure labels, keys and current figure
conventions. The superseded wording and generic notation tutorials are removed
from those reader-facing places; exact identifiers and stable filenames stay.
No unresolved entry from k98 remains for art.

## Decisions (running log)

- Keep the nine existing views and page-native tables: their partitions,
  sequences and mappings already carry the first half's relationships. A new
  diagram would duplicate those views (editorial judgement, not a contract
  restriction or evidence that the medium cannot carry the content).
- Make the existing palette explicit across C4 and UML, including sequence
  participant borders, temporary-channel text, boundary labels and state entry
  dots. Use 14-unit body/relationship/key text, 16-unit titles and entity heads,
  and 12-unit C4 technology/type qualifiers. Keep notation-specific arrowheads
  and dashed lines, with common 1.5-unit principal strokes. Shared styles own
  the choices; per-figure code owns content and semantic role tags.
- Retain the existing filenames and renderer pins. Remove generic notation
  tutorials while preserving each view's role, colour keys, software/process
  limits, confirmed-lifetime endings, attempted cleanup and outcome precedence.
  The already-live proof k100 discharges the ordinary successor-cut rule.

- Inspecting the effective output found that the attempted `padding` skinparam
  prints a deprecation warning inside the SVG. Scoped style padding replaces
  it. The lifeline thickness skinparam had no effect; a scoped style now
  explicitly retains the rendered 0.5-unit lifeline. Raw SVG attributes
  distinguish this from the 1.5-unit primary strokes.
- Fixed two visual collisions: the process-boundary heading now fits clear of
  the incoming arrow; the installed dependency runs horizontally, with its
  full canonical-path/build-identity meaning in the key. All nine final views
  were checked at chapter width and full size in local Chrome. The dated
  viewing record and environment limits are in `docs/book/figures.md`.

## Final verification, 2026-10-16

- `task docs:book:build` passed.
- `task docs:examples` passed: 13 complete examples, 0 failed, 0 skipped,
  6 intentionally non-executable fragments, 0 page errors. Its worker build
  and locked workspace build passed.
- `task docs:figures` passed: all nine committed SVGs match their sources
  with PlantUML 1.2026.8, d2 0.9.0 and DejaVu 2.37. Renderer pins and the
  single-worker correction are unchanged. No mismatch was discarded or
  normalised; the padding warning was a diagnosed style defect fixed before
  these checks.
- SHA-256 comparison before/after those three checks found zero changes in
  3,446 inputs: docs, crates (excluding node_modules), scripts, manifests,
  Taskfile, CONTEXT and pinned local renderer/font/mdBook/Bun tools. The
  sorted path/hash manifest digest is
  `0e3b6734c0ed0f5549eb8f88f6feee7871b6dfad5af5f5d19f7bea4f55da01e8`.
  The local manifest is `target/book-art-k99/final-inputs.json`; this result
  concerns these inputs on this host, not universal renderer reproducibility.
- Enumerated text nodes in all nine SVGs; no superseded front/admission/
  settlement labels or renderer warnings remain. A positive control matched
  `native front`. Source aliases and the stable `settled-exit-status` filename
  remain intentionally unchanged. Reviewed the prose/source diffs and each
  adjacent role/key; tables already carry the other relevant relationships.
- Tier 2 graph coverage used project
  `Users-antony-Development-Ravel.simplify-and-componentize`, generation
  `2026-10-16T08:00:07Z`. Markdown metadata had no recorded issue. PlantUML
  freshness was untracked and SVGs excluded by suffix, so all eleven sources
  were read directly and all nine SVGs inspected as raw attributes/text and
  rendered images. The graph is not evidence of figure completeness.
- No product or architectural decision changed; the three ADRs need no
  amendment. The copy-edit handoff above is resolved. Proof k100 is already
  the next live leaf; no duplicate tail was cut and the book parent remains
  open. No live review names this art producer, so no Creator line is due.
