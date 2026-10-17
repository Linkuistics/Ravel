# book-first-half-k132

## Goal

Prove chapters 1–6, the README, `CONTEXT.md`, `SUMMARY.md` and the
`end-session` help text as a set after read 1's corrections, before the
owner's resumed read.

## Context

- Cause: read 1 (`book-read-one-k70`, log of 2026-10-17): findings R3–R9,
  the change to ASD-STE100, “send” for the result and the command-form
  titles. The run: word decisions k123, then draft k124 and copy-edit k125
  (chapters 1–2), copy-edits k126–k129 (chapters 3–6), set art k130 and the
  README and help leaf k131.
- Read all of `docs/book/contract.md`, `docs/book/ste.md`, `CONTEXT.md` and
  the record of proof k100. The STE specification is at the contract's path.
  Keep it and every extract out of the repository.
- Five sessions converted the chapters to STE one at a time. Look for one
  concept in different words in different chapters, and for a shared word
  decision that `ste.md` does not hold.

## Done when

- Every chapter, the README, `CONTEXT.md` and the figure text follow the STE
  writing rules and `ste.md`. `task docs:ste -- --strict` passes on all of
  them, and `task docs:ste:test` passes (its controls fail on a planted
  problem for each rule).
- R3–R9 are corrected as the read 1 log records. R6 and R9 agree with the
  code and the specifications.
- No “publish”, “publication”, “submit” or “submission” for the result stays
  in the book, the README, `CONTEXT.md` or the help text. The search has a
  control seen to fail.
- The titles agree in the contract, `SUMMARY.md` and each chapter's H1. All
  local links and anchors resolve, and a broken-anchor control fails.
- The source inventories still agree with the CLI, the SDK and the
  specifications, and the reading routes still work.
- A fresh `task docs:book:build`, `task docs:figures`, `task docs:examples`
  and `task check` pass on unchanged inputs. The served book shows every
  figure.

## Notes

- Cut nothing. The owner's resumed read, `book-read-one-k70`, follows.
- Record what you could not view, as k100 did.

## Decisions (running log)

- Read chapters 1–6, the README, glossary, contents and figure text as one
  set against the contract, shared word decisions, both specifications and
  all three ADRs. R3–R9's corrections agree across these surfaces. The caller
  owns the result convention and skill instructions. The policy owns the
  selected command and its actual model and effort arguments.
- Correct three chapter 6 notes under STE Rule 5.5: notes give information,
  not instructions. Replace the prompt-file instruction and two navigation
  instructions with statements. Keep their facts and link destinations.
  The strict script does not check this rule. No new word decision is needed.
- Keep the second-half handoffs with their named owners. This first-half
  proof does not close the book node or cut another leaf. The resumed owner
  read remains next, and the whole-book art retains the dense-figure text
  issue already assigned to it.

## Whole-set proof, 2026-10-18

Read all six chapters, their tables, example explanations, captions, alt
text and figure labels. Read the README, `CONTEXT.md`, `SUMMARY.md`, the
contract, `ste.md`, proof k100, read 1's findings, both normative
specifications and the three ADRs. Consulted Part 1 of the owner's STE PDF
outside the repository and verified its SHA-256 against the contract.
The dictionary is not this proof's word authority.

R3's review referent says “mentioned above”. R4 identifies the TypeScript
policy as a configuration file. R5 says “The caller”. R6 identifies RESULT
as caller-chosen text, gives `continue` as an example and separates that
convention from Ravel's unchanged return after cleanup. R7 uses “send”. R8
removes the repeated colour descriptions before the figures and retains
each figure's key. R9 separates caller skill instructions from the owner
policy's actual command arguments and from the selection's notice labels.
The completion implementation, request delivery, selection execution and
both specifications support R6 and R9. No behavioural correction was needed.

The learning route, caller route (1–3, 5, 6) and policy-author route
(1–3, 4, 6) still supply their prerequisites. Direct reference, diagnostic,
status and figure entries retain local explanations and links to teaching.
The chapter H1s agree with the contract and SUMMARY. The part titles also
agree. Future chapters remain draft entries or plain-text references.

Re-enumerated inventories 1 and 2 from the current executable help, CLI
definitions, settings, bounds, environment code, SDK, static example, sample
and archive manifest. All 15 start options, both verbs, RESULT, help/version
forms, status classes, 19 SDK exports, six static exports and eleven archive
paths have reference entries. All 49 diagnostic codes have rows. The five
other snake-case candidates are classified: `owner_declined` is fixture
policy data, `prompt_file` is a clap conflict ID, `remove_dir_all` names an
operation, and `spawn_missing`/`spawn_other` are private controls. Settings,
six bounds, data shapes and worker/managed environment rules agree with
their source and teaching. Inventory 3's first-half lifecycle explanations
agree with both specifications. Inventories 4/5 and the second half are
not claimed here.

The Tier 2 graph project was
`Users-antony-Development-Ravel.simplify-and-componentize`. Exact symbol
search found main, run and end_session. Main's outbound trace returned all
seven rows without truncation. Exact run and end_session snippets support
the completion checks. Coverage generations `2026-10-17T19:04:08Z` and
`2026-10-17T19:11:31Z` reported complete metadata with matching source
freshness and no recorded source gap in the checked files. SVGs are excluded
from the graph: their layout text was read directly, and all nine were
regenerated. The attempted `scripts/release-layout.sh` path does not exist
and supplied no evidence. The manifest is in `scripts/release-common.sh`,
read directly and coverage-checked. Coverage remains a best-effort signal.

## Fresh checks and limits

- `task docs:ste -- --strict`: zero problems in 18 files, including all
  nine figure layout records. `task docs:ste:test` passed its planted-error
  and mutation controls. The three manual Rule 5.5 corrections are beyond
  that script's checks.
- `task docs:book:build`: passed with no warning. `task docs:figures`:
  all nine match the pinned Typst/CeTZ/font sources and layout records.
- `task docs:examples`: 13 passed, zero failures/skips/page errors. Six
  labelled fragments were not run. `task check`: formatting, scripts,
  worker/probe builds, runtime integrity, types and lint passed. The Rust
  suite passed all 339 tests, with 46 skipped.
- The link audit checked 429 local source destinations across the chapters,
  README, glossary and SUMMARY against the files and built chapter anchors.
  All resolve. Known-good anchors passed. Planted broken-anchor,
  missing-page and missing-image destinations failed. All 18 rendered image
  elements have nonempty alt text. Each figure has an inline image and a
  hidden zoom copy.
- The old result-word search is clean on the chapters, README, SUMMARY,
  figure records and end-session help. The glossary's `_Avoid_` line keeps
  the forbidden words solely to identify them. A planted “publishes the
  result” sentence matched, and the same search still matches the normative
  specification, which intentionally keeps that terminology.
- Served the fresh build on `127.0.0.1:8783`. Chrome confirmed all nine
  images complete with nonzero natural dimensions and a visible copy at
  the 750 px chapter column. Inspected full-size installed-files and the
  upper part of successful-session as visual samples. This is not a new
  complete visual inspection of every figure. The dense-text issue remains
  with whole-book art k101, as k130 records.
- Read the raw Markdown. VS Code inspection timed out, so its preview is
  unchecked. GitHub rendering is unavailable for this unpushed workstream,
  and the book on GitHub Pages is unavailable before its release. No other
  theme, real-agent execution, interactive-terminal test, native x86-64
  qualification or new-reader comprehension is claimed.
- The final before/after SHA-256 manifests agree for all 3,519 subjects:
  docs, crates excluding node_modules, scripts, root files and pinned tools
  under `target/tools`. The sorted compact manifest digest is
  `dbf5e65804d04458562ba5422391d136048fd1ed0b9652fb449d193b18e89ea8`.
  Generated build outputs and dependency installations are outside that
  digest scope. Logs, manifests and per-item link/inventory records are in
  `target/book-proof-k132`.

No unresolved first-half handoff remains. The book node stays open: read 1,
second-half drafting, whole-book art/proof and read 2 still have their own
owners. Its second-half handoff list remains relevant for those sessions,
including guide migration, future links and the report-format question.
No live review names this producer, no Creator line is due, and no ADR
decision changed.
