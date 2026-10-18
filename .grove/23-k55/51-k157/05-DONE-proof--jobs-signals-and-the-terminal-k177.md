# jobs-signals-and-the-terminal-k177

## Goal

Read and prove the whole of chapter 9,
`docs/book/src/09-jobs-signals-and-the-terminal.md`, after art k176, under
`docs/book/contract.md`, `docs/book/ste.md` and `docs/book/figures.md`.
Check every charter, the entry routes, source truth and figure alternatives.

## Context

- Draft k158 and k174 hold the source evidence, inventory 3 and 4 mappings,
  guide-statement destinations and measured checks. Copy-edit k175 records
  the signal-name rule and preserves the failure limits. Derive the relevant
  inventories again rather than treating those records as proof.
- Art k176 embeds the revised `escalation-timing` and the new
  `terminal-lending-recovery`. Read their sole editable `.typ` sources, keys,
  layout records, captions and alt text. Existing timing and sequence modules
  draw them with the shared style. No runtime code or renderer changed.
- Timing A and B are alternatives, both continuing at C, the normal cleanup
  path. A signal attempt is not confirmed exit. The two final group KILL
  attempts precede reap; terminal recovery precedes the signal-0 query.
  Only ESRCH confirms absence. Poll intervals and bounds are not wall-clock
  guarantees. Early child exit gives descendants no new grace.
- The terminal sequence keeps the runner inside the CLI. Its labelled
  dividers mark alternatives, not consecutive takeovers. Early setup failure
  can enter B before the group record, handoff or exec. Started is not proof
  of exec; the X belongs only to A's confirmed child reap. B's two recovery
  conditions remain visible. Unknown status permits no wider takeover.
- Cancellation stays linked to chapter 5. The new adjacent sentence says that
  those sequences do not show confined cancellation. Their unchanged signal
  wording, and chapter 6's mixed names, remain with the whole-book proof as
  the book brief records.
- Art's checks pass: thirteen figures, fourteen complete examples, eight
  excerpts, book build, and strict STE/prose breaks for chapter 9. Its one
  reviewer found the early-failure qualification, which art added. These are
  prior checks, not this proof's evidence.
- `docs/book/figures.md` records Chrome views at the 750-pixel column and
  full-size views, with unavailable environments. It also records the new
  terminal SVG above the earlier 300 KB benchmark and the set above 2 MB.
  Whole-book art k101 already owns size and dense text. Keep that ownership
  visible; do not claim those benchmarks pass or infer reader comprehension.

## Done when

- The whole chapter, figure text, local links and entry routes agree with the
  contract and current source. Inventories 3 and 4 and guide destinations are
  reconciled for this chapter. Any defect outside proof's repair allowance is
  routed through the editorial family's correction rule.
- Fresh figure comparison, book build, examples, excerpts, strict STE on the
  chapter and its figures, and strict prose breaks pass. Inspect the reachable
  reading environments and record unavailable ones.
- Check node k157's Done when. If it closes, promote its enduring guide
  destinations, obligation coverage and limitations to the book brief, and
  name the node handle alongside this leaf in the commit. Do not close the
  book node while its later work remains live.

## Notes

No reader-comprehension, new runtime or platform-qualification result follows
from this proof. Ordinary proof cuts no successor.

## Decisions (running log)

- Proof reads the whole chapter, both editable figure sources, their keys and
  alternatives against the current contracts and production source. The graph
  reports ready with 47,549 nodes and 60,124 edges. Coverage later identifies
  generation `2026-10-18T10:26:38Z`, with matching complete metadata.
  This is Tier 2 task-directed verification, with direct source reads for the
  operations and tests that support the chapter's claims.
- The terminal sequence placed the attempted-group write before SIGTTOU
  suppression. `run_with_output` suppresses SIGTTOU first, then writes the
  record, then attempts the handoff. Proof restores that order. This is a
  mechanical figure correction within proof's whole-document charter.
- The chapter now links the first uses of result, session, confinement and
  policy worker to their definitions. No heading, example, runtime code or
  contract changes.
- Both figure alternatives, their captions and their text alternatives agree
  with the corrected sequence. No correction run or in-session reviewer is
  needed for the source-ordered move and term links. No ADR changes.

## Source verification

The graph searches enumerated functions in runner `run.rs`, runner
`confinement.rs`, CLI `signal_state.rs`, and the relevant CLI run/worker
functions and cancellation methods. The outbound depth-one trace from
`supervise_reported` returned eleven entries without truncation. Exact snippets
for `watch`, `error_settlement`, `supervise_reported` and `confirm_gone` were
checked against direct source. The trace includes heuristic cross-module
matches, so it supplies discovery rather than proof of call resolution.

| Claim | Source checked directly |
|---|---|
| CLI graces, polls, retry bound, early exit | CLI `run.rs:ESCALATION`; runner constants, `watch`, `OwnedChild::bound/reap` |
| Group stops before reap, no numeric signal after identity loss, ESRCH-only confirmation, retained EPERM reports | Runner `OwnedChild`, `watch`, `supervise_reported`, `confirm_gone`; private `tests/internal/wait_events.rs` |
| Unconfined forwarding versus immediate confined KILL | `watch`, mode selection and `run_noninteractive`; the noninteractive cancellation test identities |
| Detached input/descriptors, grants, platform backends and no fallback | Runner `run_with_output`, `confinement.rs`; CLI `Confined::prepare/environment`, `run`; confinement tests |
| Foreground gate, modes, group record, suppression order and recovery rules | Runner `Terminal`, `Lease`, `AttemptGroup`, `run_with_output`; paired holder/raw-mode tests in `tests/job.rs`, failed-exec and unknown-status cases in `run/terminal_tests.rs` |
| Entry capture, exclusions and dispositions before mask | CLI `signal_state.rs`; runner `EntrySignals::reinstate`, handler installation and SIGCHLD repair |
| Waitability before worker, reader shields, Started check, final signal decision and output | CLI `worker.rs:child_watching/Capture::start`, `run.rs`, `cancellation.rs:Handlers::look/finalise` |
| Qualified native support | Completion specification and `native-entry-development-evidence.md`, “Support limits, per target” |

Coverage checks include the cited source, specifications, ADRs, chapter,
figure sources/records, glossary, briefs and viewing contract. All exact
source paths have matching metadata and no recorded parse gap. SVGs are
excluded by suffix. Direct browser views and regenerate-and-compare supply
their evidence. A clean coverage report is best-effort, not completeness proof.

The chapter's POSIX link returned HTTP 403 through the web reader. That
external page was not revalidated. The chapter retains its explicit SIGCHLD
portability qualification and the current supported-target implementation.
No new platform or terminal execution follows from this proof.

## Coverage inventories and guide destinations

Inventory 4 was derived again from the session specification's installed,
policy, request and managed-result obligations, then reconciled with the
guide's O1–O10 table. Every row has a teaching destination:

| Obligation | Final destination |
|---|---|
| O1: preserve owner configuration, SDK, settings and discovery | Chapters 4, 6 and 7 |
| O2: no worker result authority, identity check and reap before managed launch | Chapters 7–8; chapter 9 entry-state section supplies waitability |
| O3: caller data preserved, checked policy outcomes without fallback | Chapters 4–6 and 8 |
| O4: selected argv, caller context, requested confinement or refusal, fresh nested context | Chapter 8 launch preparation; chapter 9 mode and confinement sections |
| O5: first complete unchanged result, empty distinct from absent, invalid/stale refusal | Chapters 5 and 8; chapter 9 same-user limit |
| O6: matching reap and confirmed group absence, later child status not decisive | Chapter 9 escalation, fake, pre-reap order and guarantees |
| O7: cancellation suppresses even a sent result and propagates after cleanup | Chapter 9 cancellation and entry-state sections, chapter 5 sequences and chapter 8 final priority |
| O8: recover foreground/modes under holder rules, group stops before reap | Chapter 9 terminal and pre-reap sections, including the completed terminal sequence |
| O9: result-only stdout after cleanup, cleanup/output failure cannot succeed | Chapter 8 final decision; chapter 9 guarantees and possible output prefix |
| O10: matched installed pair without host runtime | Chapters 6–7; chapter 12 retains delivery work |

Inventory 3 was derived from the session specification and the completion
specification's one-decision, failure and native-entry sections. The map is:

| Outcome or mechanism | Final destination |
|---|---|
| Unchanged result, empty/multiline/Unicode/option-looking values, later nonzero status | Chapters 3, 5 and 8; chapter 9 fake demonstrates KILL death |
| Repeated send, natural exit without result, stale/missing context and invalid file | Chapters 3, 5, 6 and 8; chapter 9 defines the validation/authentication limit |
| Malformed CLI/settings, policy refusal/exception/data, timeout, absent/mismatched/failed worker | Chapters 4–8; chapter 9 adds the waitability prerequisite |
| Absent or unexecutable selected program | Chapters 6 and 8; chapter 9 covers failed exec after terminal handoff |
| Selection/supervision cancellation, including after a sent result | Chapter 9 cancellation and entry state; chapter 5 examples and chapter 8 priority |
| Grace, TERM, kill grace, KILL, early exit and descendant cleanup before reap | Chapter 9 escalation and pre-reap sections, timing alternatives and fake |
| Survivor, failed group query, supervision/cleanup failure, partial acquisition and expired retry bound | Chapter 8 failure/cleanup; chapter 9 pre-reap, terminal and guarantee sections |
| Operation reports and conditional EPERM | Chapter 9 pre-reap prose; chapter 8 output forms |
| Unwritable output, failed write/flush and possible prefix | Chapters 6 and 8; chapter 9 guarantees |
| Nested sessions | Chapters 5 and 8 |
| Noninteractive isolation and confinement granted/refused without fallback | Chapter 9 mode/confinement sections and chapter 5 complete example |
| Terminal lend/recovery, failed spawn, normal/signal/unknown child state | Chapter 9 terminal section and sequence |
| Entry capture, masks/dispositions, worker/managed waitability, reader shields and final transitions | Chapter 9 entry-state section; chapter 8 source priority |
| Same-user tampering, abrupt CLI death, escaped descendants, detached policy children, continuing/concurrent hosts and native x86-64 limit | Chapter 9 guarantee table and support paragraphs |

The complete guide sentence-group maps in draft k158 (“Guide statements this
chapter replaces”) and k174 (“Guide destinations for part B”) were reconciled
against both guides and the final chapter. Their destinations hold. The final
O8 and lifecycle rows no longer owe art: both figures exist, and proof checks
their alternatives and order. No chapter-local teaching gap remains.

For fold-in k164, `local-runner.md` Boundary maps to chapters 7–8's interfaces
and chapter 9's escalation, mode, confinement and report sections. Native entry
and return maps to chapter 9's terminal, entry-state and guarantee sections.
Observers remain chapter 8's; test commands and authority guards remain chapter
11's. Provenance stays outside the book. The guides' old entirely-unmeasured
native-entry claims give way to the qualified existing aarch64 evidence.

For `components.md`, O4/O6/O7/O8, R5/R7, successful operation, cancellation,
C4 recovery and native-entry limits map to the chapter 9 sections above,
with chapter 8 retaining result association, partial allocation and detailed
failure priority. No guide is removed in this leaf. Set proof k163 and fold-in
k164 remain the owners of cross-chapter reconciliation and removal.

## Verification and viewing limits

- Fresh `task docs:figures`: all thirteen SVG/layout pairs match their sources.
- Fresh `task docs:book:build`: no error or warning.
- Fresh `task docs:examples`: fourteen passed, zero failed or skipped, six
  labelled fragments not run. This seam has no terminal or real harness.
- Fresh `task docs:excerpts`: eight passed, zero failed.
- Strict STE: zero problems in chapter 9 and both local figure records.
- Strict prose breaks: no run above 450 words.
- All forty chapter-body local link/image occurrences resolve in the built
  book. Missing-file and missing-anchor controls are rejected.

The measurement subjects were the 213 files enumerated from `docs/book`,
`crates`, `scripts` and `.cargo`, plus Taskfile, Cargo manifests/lock and
CONTEXT. SHA-256 digests before and after each finished run agree item by
item. The last link edits prompted a fresh build/example/excerpt run and
strict prose checks; figure subjects were unchanged. Temporary manifests and
logs are in `/tmp/ravel-k177-proof.3QeimE/`. These are development checks, not
new immutable runtime-campaign receipts.

Chrome showed both full chapter-width figures, their keys, the adjacent
tables and the guarantee table on a light local mdBook page. Both full-size
SVG links opened, with the opening regions inspected there. VS Code's light
Restricted Mode preview showed the introduction, process-group comparison,
escalation table and timing opening. The whole chapter and editable figure
sources were read as raw files. The viewing record in `docs/book/figures.md`
states the environments and limits. No other theme, narrower viewport,
GitHub rendering or book on GitHub Pages was inspected.

## Node close

Node `jobs-signals-and-the-terminal-k157` meets its Done when: SUMMARY lists
chapter 9, all four stages have delivered, the complete example passes, and
the source, inventories, figures and guide destinations agree. Its handed
forward work is closed or already owned above the node. No correction run is
needed and no chapter-local entry is left without an owner. Dense figure text
and size remain whole-book art k101's existing work, not a passed benchmark.

The enduring coverage, guide destinations and limits are promoted to the book
brief. The book node remains open for later chapters, set proof, guide fold-in,
whole-book art/proof and read 2. No architecture or support decision changes.
