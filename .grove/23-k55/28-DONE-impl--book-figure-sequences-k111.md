# book-figure-sequences-k111

## Goal

Replace the three PlantUML sequence figures with Typst sources in the shared
sequence engine, and prove the engine covers the notation features they use.

## Context

- Figures: `successful-session` (trial source exists), `selection-cancellation`,
  `supervision-cancellation` under `docs/book/src/figures/`.
- The trial covered messages, replies, create, destroy, dividers, notes and one
  self-call. The cancellation figures add notes spanning participants, a
  `database` participant (cylinder head) and multi-line self-calls; check each
  against `_sequence.typ` and extend the engine, not the figure, if one is
  missing.
- Record design (`book-figure-layout-record-k120`, survey option B): `_sequence.typ` already returns a layout record (plain data, no style) and the figure source passes it to `_draw.typ`'s render; nothing here draws. Each regenerated figure commits `<figure>.layout.json` beside its SVG (`book-figure-layout-json-check-k122`), and it is shipped sketched at roughness 2 with keys crisp (`book-figure-record-and-draw-k121`).
  k121 moved the draw loop out; this leaf proves the record covers the features below, extending the layout side only.
- Notation facts to preserve: `docs/book/figures.md`, "What each view means"
  (X ends a lifetime only after confirmed removal; cancellation figures use
  direct INT; no X certifies teardown there). Palette and roles come from
  `_style.typ`; the style (sketch at roughness 2, keys crisp) from `_draw.typ`,
  per `book-figure-sketch-style`'s decision. A result arrow is
  `msg(..., role: "result")`, not a colour.

## Done when

- Each of the three `.puml` files is replaced by a `.typ` whose regenerated SVG
  and `.layout.json` are committed (the first real ones: the check from k122 is seen
  to pass on them and to fail on a moved element); no `.puml` for them remains. Every label, note, key line, colour
  role and ordering from the old figure is present; list any difference with its
  reason in the decision log.
- Adding a participant and lengthening a label in one cancellation figure needs
  no coordinate edits (the trial's claim, checked on a figure it did not cover).
- The pages that embed the figures still carry their alt text and captions; edit
  them only where the figure's facts changed.
- `task docs:figures`, `task docs:book:build` and `task check` pass.

## Decisions (running log)

- **Engine coverage checked first, against the old figures.** `_sequence.typ` already
  handled everything the cancellation figures use except one defect: section dividers
  (rules, label box and label) were placed without the horizontal offset `ox` that
  centres the drawing when the key or title is wider than the participants, so in
  `selection-cancellation` the rules and the divider label sat left of the content.
  Fixed in `_sequence.typ` (layout side only); `successful-session` has `ox = 0` and is
  unchanged by it. Notes over three or more participants, a `database` head, a note
  before any message, and multi-line self-calls needed no change.
- **Ported with no per-figure geometry.** Each `.typ` is a participant list, a row list
  and a key. `.puml` removed in the same change; `.svg` and `.layout.json` regenerated
  by `task docs:figures:update` and committed. Pages carry the same `figures/<name>.svg`
  links, alt text and captions, which stay accurate (no fact changed), so no page edit.
- **Label width.** `sequence`'s default `max-label` (170pt) split labels at a hyphen
  ("re-/raise", "process-/group", "end-/session") and left one-word orphan lines in
  self-calls. The figures pass `max-label: 300` (`successful-session`,
  `selection-cancellation`) and `340` (`supervision-cancellation`); the engine default is
  unchanged. Found by viewing the rendered PNGs, not by a check.
- **Differences from the PlantUML figures** (every label, note, key line, divider,
  participant and their order is present, checked mechanically below; arrow directions,
  dashed or solid style and the green result arrows were compared by eye against the old
  SVGs):
  - apostrophes are typographic (`worker’s`, `process’s`, `session’s`) because the engine
    flattens the apostrophe smartquote to `’`; the PlantUML SVGs had a straight one;
  - the old figure wrapped by PlantUML's `maxMessageSize`, so line breaks inside a long
    label differ; the sources keep the old explicit breaks and wrap only at `max-label`;
  - the figures are now sketched at roughness 2 with keys crisp (the k110/k121 decision),
    not the PlantUML look; no wording changed anywhere.
- **Mechanical content check.** A script pulled every participant, message, divider,
  note and key line from the old `.puml` (kept in the scratchpad) and required each in the
  new record's text elements, with messages, dividers and notes in order: 31, 15 and 18
  items, no problems. Controls seen to fail: swapping two messages' texts in a copy of a
  record, and dropping "numeric" from a key line, were each reported.
- **k122's check passes on the first real records and fails on a moved element.**
  `task docs:figures` matched all nine figures. Moving `head-worker`'s `x0` by 10 in the
  committed `selection-cancellation.layout.json` failed with "layout.json differs from the
  layout record … evaluates to" and left the SVG comparison alone; renaming the worker
  head's label in the source changed both record and SVG, each reported. Both restored.
- **No-coordinate-edit claim, on a figure the trial did not cover.** A scratch copy of
  `supervision-cancellation.typ` (scratchpad `scal/`) with one added `audit sink
  (owner-supplied)` participant and one four-line message to it, and nothing else changed,
  rendered with columns, notes and key re-laid out (`scal/out.png`); not committed.
- **Docs.** `docs/book/figures.md` now says the three sequence figures are `.typ` with
  committed records and records what the sequence layout covers, `max-label` and the
  apostrophe difference. The PlantUML-side text and the ".puml and .typ" rules stay for
  `book-figure-retire-plantuml-k117`.
- **Verification** (finished edits first, then measured): `task docs:figures`, `task
  docs:figures:test`, `task docs:book:build` and `task check` (339 passed) pass. Not
  checked: GitHub rendering, the published site, a browser view of the built pages (the
  SVGs were viewed as PNGs through rsvg-convert, not in the book's page).
