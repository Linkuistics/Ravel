# what-ravel-is-for-and-its-actors-k104

## Goal

Read chapters 1 and 2 under the art charter after the framing correction.
Check every figure, figure key, role sentence, alt text and table against the
corrected teaching and the amended `docs/book/contract.md`.

## Context

This is the document's second art correction run, cut by read 1
(`book-read-one-k70`) and authorised by the owner on 2026-10-17. Draft k102
now presents Ravel as running LLM sessions in agent harnesses, such as Claude
Code and Codex, as supervised parts of another program, typically an
orchestrator, and gives the reason for `end-session`: an interactive harness
does not exit when its LLM finishes. Copy-edit k103 follows it.

Known before this run started:

- **The system-context figure is generic**
  (`docs/book/src/figures/system-context.puml`). The caller is
  `"Person or program: supplies the task and interprets the result"`, the
  managed process `"The selected program: performs the work"`, and the
  publishing relation `"Publishes opaque result; requests completion"` over
  `end-session`. The contract's "Figure needs" row now asks: “Where does Ravel
  sit between a caller, such as an orchestrator, and the harness or other
  program it launches?” The labels, legend and alt text need to answer that
  question, in the prose contract's plain language.
- **Text that goes with it**: chapter 1's figure introduction (it said “The
  figure separates the caller's choice of work from the owner's choice of
  program…” before k102), the figure's alt text, and chapter 2's pointer to
  the figure in "Caller and owner".
- **The container figure** (`process-authority.puml`, chapter 2) and its alt
  text describe the managed process; check that they still agree with the
  corrected harness definition.
- **A relation that may now live only in prose**: publication by the LLM, then
  Ravel stopping whatever is still running in the harness's group, then
  cleanup, then the result. The art charter's rule applies (a sequence carried
  only by running prose where a figure or table would carry it is a defect),
  and so does the unit's scope: chapter 3 owns the session's sequence figure,
  and chapter 1 gives only the reason.

Keep the book's visual vocabulary: the shared `_style.puml` and the
conventions in `docs/book/figures.md`. Regenerate a changed SVG with
`task docs:figures:update` and confirm it with `task docs:figures`; the figure
script finds a working Java 11+ runtime itself. Inspect the rendered figures in
the locally built book (`task docs:book`) and record which viewing
environments you reached, as k80 did.

Proof k105 already stands after this leaf; do not append another proof leaf.
A defect an earlier stage owns goes to the owner as an escalation, not as
another run.

## Done when

- Figures, keys, role sentences and alt text agree with the corrected text,
  and any figure you declined is classified under the art skill's three
  reasons.
- `task docs:figures`, `task docs:book:build` and `task docs:examples` pass.
- Handoffs show what remains; retire this leaf with proof k105 next.

## Decisions (running log)

- Keep both existing C4 views and their shared style. Name the orchestrator
  as a caller and the harness running an LLM session as the managed process's
  intended use, while retaining other selected programs. Align role sentences,
  keys, alt text and the fit/responsibility tables with those labels.
- Carry publication → stopping remaining group members → confirmed cleanup →
  return in a short chapter 1 table. Decline another sequence diagram here as
  an **editorial judgement**: chapter 3 owns the detailed lifecycle diagram,
  while this table answers chapter 1's question about why ending is explicit.
  This is neither a contract prohibition nor a limitation of the medium.
- Proof k105 is a live later sibling. The ordinary successor is already queued;
  no extra proof leaf is needed. No architecture decision changes.
- The full-size process view exposed result-arrow labels crossing the software
  outline. Move the harness to the right and shorten and wrap those labels;
  the legend and adjacent text retain inherited session-result context and
  validation before return.
- The owner accepted the Typst/CeTZ recommendation and specified a preference
  with a trial before migration. Prototype `book-figure-renderer-k107` carries
  the two-diagram comparison and maintenance edits before the resumed read.
  Their separate request to add this guidance to Grove's art skill and release
  is handled in the Grove repository, not as a Ravel product change.

## Validation

`task docs:figures` matched all nine figures after the final layout correction.
`task docs:examples` passed all 13 examples (six declared fragments not run,
zero page errors). `task docs:book:build` passed. Native Chrome viewing at
chapter width and the process SVG at full size is recorded with its limits in
`docs/book/figures.md`. Proof k105 remains live, so this retirement closes no
parent node.
