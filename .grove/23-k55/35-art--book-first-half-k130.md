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
