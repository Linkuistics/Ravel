# book-first-half-k100


## Goal

Prove chapters 1–6 as one set after the owner-directed copy-edit and art run,
leaving Parts I and II ready for guide migration and read 1. The contract is
the amended `docs/book/contract.md`; the preceding proof is
`book-first-half-k67`, whose checks are earlier evidence, not this pass's.



## Context

- Read copy-edit `book-first-half-k98`, art `book-first-half-k99` and the book
  brief. This ordered run applies new reader/visual requirements: plain prose
  for a 16-year-old software engineer; the term **Ravel CLI**; known diagram
  notation; consistent colours/fonts/line weights and related visual styling.
- Read all six chapters, following the learning route and the two caller and
  policy-author routes. Sample direct reference/status/error/figure arrivals.
  Re-enumerate inventories 1 and 2 from source; check inventory 3 for chapters
  3/5's claims. Inventories 4/5 and chapter 10 remain unclaimed here.
- Verify every changed behavioural explanation against the two normative
  specifications and three ADRs. Preserve source/literal exceptions without
  letting unexplained specialist wording leak into the teaching.

## Done when

- The entire first half meets the amended contract, including prose, figure
  labels/keys/captions/alt text, entry routes and shared visual conventions.
  No unresolved first-half handoff remains.
- Terms, source inventories and cross-chapter claims agree; all local links
  resolve. Forward chapter 6 links remain page-level; chapters 7–12 remain
  plain text pending the second half.
- Fresh build/serve, examples, figures and task check run and are recorded;
  actual viewing coverage and evidence limits are explicit. A known good
  earlier reading is not a substitute for checking the changed document.
- Clear the first-half correction handoff in the book brief, then retire
  without growing a duplicate tail. Guide migration is already next.

## Notes

The later whole-book art `ravel-book-k101` is a separate consistency pass
across both halves, followed by whole-book proof and read 2. Do not claim
that unavailable second-half work or owner comprehension was checked here.

## Decisions (running log)

- Read chapters 1–6 as one document against the amended contract, both
  normative specifications and the three ADRs. The caller route (1–3, 5, 6)
  and policy-author route (1–3, 4, 6) retain the concepts each needs before
  using them. No behavioural correction or additional editorial run is
  indicated by that reading; the already-live guide migration remains next.
- Keep literal codes, source aliases and stable figure filenames exact.
  Their spelling is distinct from reader-facing prose and figure labels.
  Check inventories 1/2 afresh and inventory 3 for chapters 3/5; inventories
  4/5 and unavailable second-half work remain outside this proof.
- The final checks passed on unchanged subjects. Clear the correction
  handoff in the book brief without cutting a successor: guide migration and
  the later whole-book art already have live leaves. No parent closes and
  no architectural decision changes.

## Whole-set reading and source inventories

Read all six chapters, including headings, tables, example explanations,
figure keys, captions and alt text. Followed the learning route and the two
caller/policy-author routes. The revised prose explains Ravel CLI, worker,
runner, process groups, reap, terminal lending, publication and outcome
selection without requiring an ADR or specification detour. Figure keys
explain the book's colours, responsibilities, lifetimes and limits rather
than teaching generic notation. Direct lookup entries link back to teaching.

Compared the restated behaviour with `docs/specs/session-execution.md`,
`docs/specs/result-job-completion.md` and all three `docs/adr/` records.
Worker reap and private-directory removal precede managed launch; publication
precedes result validation, cleanup and delivery. Original failures and
secondary diagnostics survive, cancellation suppresses a candidate result,
and final writing can fail after cleanup. Empty publication, absence, later
nonzero child exit, nested paths, group escape and conditional terminal
recovery stay distinct. Policy context remains measured selection data,
with loader replacement rather than automatic merge. No behavioural or
cross-chapter contradiction was found, and no producer correction or
in-session reviewer was needed.

Re-enumerated from current subjects, rather than treating k67's lists as
this pass's evidence:

| Inventory | Sources and comparison | Result |
|---|---|---|
| CLI | Fresh executable help for the root and both verbs; `cli.rs`, `main.rs`, `refusal.rs`, `cancellation.rs`, `run.rs` and program/result branches | Both verbs, RESULT, separator, help/version forms and all 15 start options covered; statuses 0/1/2/3/5/124/126/127 and shell signal statuses 129/130/143 agree across both tables |
| Diagnostics and streams | Enumerated simple snake-case literals across Rust CLI source, then classified every candidate; main/refusal/run JSON construction and completion publication errors | All 49 public diagnostic codes have reference rows; `prompt_file` is a clap conflict ID, `remove_dir_all` an operation name, and `spawn_missing`/`spawn_other` private controls. Publication messages remain plain-text status-1 failures, distinct from the JSON codes. No exhaustive OS-message claim |
| Environment | `environment.rs`, `confinement.rs`, `choice.rs`, `run.rs`, completion and worker startup, plus runner launch/confinement | Base worker names, nine reserved grant classes, explicit grants, cache control, six exact managed scrub names, confined retained names and temporary-path replacements covered. Test-only K28/C4 controls are not public inputs |
| Settings and bounds | `settings.rs`, `limits.rs`, inputs and both specifications | Three settings, unknown-key refusal, validation before overrides, six effective limits and prompt/parameter/task-ID/result limits covered with their values and origins |
| Shapes and SDK | SDK source declarations, Rust context/policy validators and worker host/main | Policy's four fields; request's nine; Selected's seven; Refused's four; Context's six; assessment/source/measured/read records; loader/selection hosts; all 19 SDK exports covered, including optionality and validation conditions |
| Imports, sample and installation | Embedded registrations, SDK/static/sample source, dispatch installer and release archive manifest | Both exact imports, resolution limits, six static exports, default/careful semantics and all eleven archive paths covered; task installation distinguished from archive-only README/LICENSE |
| Lifecycle (inventory 3) | Both specifications against chapters 3/5 and their figures | Claims about exact/repeated results, refusal/failure/timeout, failed execution, cancellation in both phases, escalation, descendants, nesting, confinement, terminal and evidence limits agree |

The structural lookup used the Tier 2 graph project
`Users-antony-Development-Ravel.simplify-and-componentize`: symbol search,
the main outbound call trace (seven results, untruncated), and the exact run
snippet. Coverage generation `2026-10-16T08:21:15Z` was recorded complete;
relevant source metadata matched and bounded scopes had no recorded source
gap. Coverage is best-effort, not proof of completeness. PlantUML freshness
was untracked, so all eleven source/style files were read directly. SVGs are
excluded by suffix; all nine were parsed for text/fonts and regenerated, with
the visual sample below supplying separate viewing evidence. Literal/config
inventories used direct source reads and enumeration.

## Fresh verification and limits, 2026-10-16

- `task docs:book PORT=8780` built and served the book; final
  `task docs:book:build` passed without warnings.
- `task docs:examples` passed: 13 complete examples, zero failed/skipped,
  six labelled fragments not run and zero page errors.
- `task docs:figures` passed: all nine SVGs match their sources with the
  unchanged pinned renderers/fonts and single-worker rendering. Every SVG's
  text and font-size inventory was checked against the current conventions;
  no superseded front/admission/settlement label or renderer warning was
  found. A positive control matched `native front`.
- `task check` exited 0: formatting/lint, worker/probe builds, typechecking
  and 339 passing tests, 46 skipped. No product source or test was changed.
- Enumerated 403 local Markdown link/image destinations across all six
  chapters; all resolve against built HTML/files, including anchors. All
  18 rendered image elements (inline and zoom copies) have nonempty alt text.
  Known-good destinations passed; missing file, anchor, image and empty-alt
  controls failed as intended. Invented missing option/export/path controls
  were detected. Forward chapter-6 links remain page-level and unavailable
  chapters 7–12 remain plain text/draft SUMMARY entries.
- Independently viewed the local light-page book in Chrome: chapter 1's
  entry and routes; chapter 3's outcome explanation, figure and caption at
  chapter width; chapter 6's environment-grant reference entry, diagnostic
  table and complete exit-status table; and process-authority and installed
  files at full SVG size. This is a visual sample, not a fresh full-size
  inspection of every diagram. Raw Markdown and all figure sources were
  read. No editor preview, GitHub rendering, published-site view, other
  theme or alternate viewport is claimed by this pass.
- Before/after SHA-256 comparison found zero changes in 3,452 subjects:
  docs, crates excluding node_modules, scripts, root files and the pinned
  local renderer/font/mdBook/Bun inputs. The compact sorted path/hash-list
  digest is `071ec994f886d84e08dbff7665de1fb70c4d985245e5401e72b79f76cdfa6b69`.
  Individual manifests, inventory/link records and logs are under
  `target/book-proof-k100`; this task carries the durable conclusions.

These results establish no real-agent execution, interactive-terminal
operation, native x86-64 qualification or new-reader comprehension. Read 1
remains the owner's labelled stand-in read. Inventories 4/5, source excerpts
for the later walkthrough, second-half chapters and chapter 10 remain their
existing owners' work. The first-half correction handoff is closed; Parts I
and II are ready for the already-queued guide migration and read 1. No live
review names this producer, so no Creator line is due. The book node stays
open and no duplicate tail is grown.
