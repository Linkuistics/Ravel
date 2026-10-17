# book-first-half-k130

## Goal

Correct the figures and their text across chapters 1–6 for read 1. Remove the
colour text that repeats each figure's key (R8). Use “send” for the result in
figure labels. Make labels, keys, captions and alt text follow ASD-STE100.

## Context

- Cause: read 1 (`book-read-one-k70`, log of 2026-10-17). R8, in the owner's
  words: “We repeat the diagram colour description in the para before the
  diagram, and in the legend.” The owner found it before chapter 1's
  system-context figure (lines 88–91 at `5bc411d1`). The same repetition
  comes before every figure: chapter 2 line 122, chapter 3 lines 288 and
  337, chapter 5 lines 628, 648 and 708, and chapter 6 lines 935 and 950 (at
  `5bc411d1`, before the copy-edits moved lines).
- Keep the colours in the key. The key goes with the figure when a reader
  opens it at full size or arrives at it directly. Remove the repeated colour
  sentences from the prose, and keep prose that the key does not repeat.
- Some labels use “publish”, for example system-context's “Publishes opaque
  result; asks Ravel to stop”. Use the `ste.md` wording.
- Read section 2 of `docs/book/contract.md`, `docs/book/ste.md`,
  `docs/book/figures.md` and the records of art k99 and viewing check k118.
  The small text at chapter width that k118 found stays with
  `ravel-book-k101`, unless this work fixes it anyway.
- `task docs:figures` compares each SVG and its `.layout.json` record.
  `task docs:figures:update` regenerates them with the pinned renderer.

## Done when

- No paragraph repeats a figure's key. Each figure's key, caption, labels and
  alt text follow STE and `ste.md`, and use “send” for the result.
- `task docs:ste -- --strict docs/book/src/figures/*.layout.json` passes
  (the text in each figure). The book does not use the STE dictionary
  (owner, 2026-10-18).
- Each figure keeps its facts, unless a fact was wrong. Record any such
  change.
- `task docs:figures`, `task docs:book:build` and `task docs:examples` pass
  on unchanged inputs. The served book shows every figure, and a control with
  a missing figure fails. `docs/book/figures.md` is current.

## Notes

- Cut nothing. The README and help leaf k131 and set proof k132 already
  stand.

## Decisions (running log)

- Keep all nine figures, their notation, palette and renderer pins. Change
  their text in the authoritative `.typ` sources, then regenerate the SVGs
  and layout records. No technical fact needs a change at this point.
- Keep colour descriptions in each figure's key. Remove those descriptions
  from the adjacent prose, and retain the role and technical explanation of
  each figure. The existing diagrams and tables carry the required relations.
  Another figure would repeat them (editorial judgement, not a limit of the
  contract or the medium).
- Use the already-live proof k132 as the next editorial stage. Cut no leaf.

## Verification, 2026-10-18

- All nine authoritative figure sources now use the STE writing rules and
  “send” for the result. The six chapters and nine layout records pass
  `task docs:ste -- --strict` with zero problems. The STE dictionary is not
  used. The captions and alt text still describe the figures.
- Removed the repeated colour descriptions from the chapter prose. A scoped
  search finds no colour names in the six chapters and finds the keys in all
  nine sources. The same search finds the former duplication in chapter 1
  at the parent revision (positive control).
- Reviewed every source diff. The figure code outside its labels is unchanged,
  checked against the parent revision. No technical fact, relation, lifetime
  marker, colour role, renderer pin or shared module changed. No ADR changed.
- `task docs:figures`, `task docs:book:build` and `task docs:examples` pass.
  The examples report 13 passed, zero failed or skipped, six fragments not run
  and zero page errors. A before/after SHA-256 manifest of 4,564 inputs
  (tracked files outside `.grove/`, fetched tools, worker dependencies and
  the two scratch verification scripts) stayed identical through these runs:
  `09966c8cd818390d93bae2174dba18d85b5ad64ded33d140d08b69f7a6a8d9ef`.
- The served-book check traversed all six pages and checked all nine SVGs:
  HTTP 200, SVG content type, parseable SVG, nonempty alt text and exact source
  bytes. A deliberately missing figure failed the same check with HTTP 404.
- Inspected every figure at chapter width and full size in Chrome. The dated
  record in `docs/book/figures.md` gives the viewport, sizes and inspection
  limits. The small-text issue remains with k101, as this task specifies.
- Graph coverage generation `2026-10-17T18:53:57Z` recorded no gap for the
  inspected text paths. SVGs are excluded by suffix, so their sources and
  rendered output were inspected directly. This is task-scoped evidence,
  not a claim that the graph covers the repository completely.
- No live review names this producer. The parent retains live leaves k131,
  k132, k70 and k101, so this retirement closes no node.
