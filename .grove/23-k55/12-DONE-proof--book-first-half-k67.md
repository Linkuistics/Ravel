# book-first-half-k67

## Goal

Read chapters 1–6 of the book as one set, after each unit's own proof, and
leave Parts I and II ready for the guide migration and for read 1. This is the
`proof` stage for the set, not a repeat of the units' proofs: a unit's proof
reads only its unit, and the entry routes, the inventories and the cross-chapter
terms span the units. The contract is `docs/book/contract.md`; this leaf names
it as its contract, and its sections 1–3 are the classes in charter.

## Context

- Every unit under this node (chapters 1–2, 3, 4, 5, 6) has retired through
  draft, copy-edit, art and its own proof. Read the node brief's "First half"
  section and the units' nodes for what each handed forward.
- Run, and record, rather than recall: `task docs:book` (build), `task
  docs:examples`, `task docs:figures`, `task check`. A viewing environment the
  session cannot reach (GitHub's rendering before a push, the published site)
  is named as unchecked.

## Done when

- **Entry routes**, read in order through the real book: the learning route
  through chapter 6; "Running sessions from your own tool" (1–3, then 5, with 6
  for lookup); "Writing a policy" (1–3, then 4, with 6 for lookup). Each
  arrives with the terms and context it needs. A forward reference to a
  chapter 7–12 is plain text and a forward reference to chapter 6 is a
  page-level link; both resolve or are named as the second half's.
- **Direct entries**: every chapter 6 entry, every exit status, every error
  message the book explains and every figure is readable on arrival, with its
  local context and a link back. Sample at least one of each class; enumerate
  the exit statuses and error messages from the source and check each is
  present.
- **Coverage inventories 1 and 2**, re-enumerated from the subject (the CLI's
  help and argument definitions, the SDK's exports, the settings reader, the
  spec), not from the contract or the units' logs, and compared with the
  chapters. Inventory 3 (lifecycle and failure outcomes) is checked for what
  chapters 3 and 5 claim; inventory 4 (component obligations) and 5
  (contributor workflows) belong to the second half and are reported as not
  yet claimed, not as covered.
- **Technical truth**: every behaviour the chapters restate is checked against
  its rule in `docs/specs/session-execution.md`, `docs/specs/result-job-completion.md`
  and the three decision records; a disagreement is fixed in the chapter (the
  specifications are normative) or reported to the owner if the code contradicts
  a specification.
- **Terms and notation**: the term risks hold across chapters ("context" is
  never unqualified; "session" is never a POSIX session unannounced), each
  diagram type is assumed familiar (the owner's direction during this proof),
  book-specific figure conventions are identified, and `CONTEXT.md` is
  current.
- **House style** over the whole set (British spelling, no normative
  keywords, front-loading, example labels), and no placeholder text.
- Nothing is handed forward within Parts I and II; items for the second half
  are written into the node brief's "Handed forward" list.

## Notes

- This session may fix what its charter owns (the proof class) and must send
  what another stage owns back as a correction run under the editorial family
  file; a second re-run of one stage against one document is an escalation.
- Read 1 follows the guide migration. Do not hold read 1's tasks here: the
  owner's comprehension is not this session's to judge, and a model's read is a
  proxy reported separately.

## Decisions (running log)

- The set proof retains the chapters' current behavioural explanations: the
  learning and caller routes distinguish publication, process exit, cleanup
  and result delivery consistently with the two specifications. Convert the
  first-half's outstanding chapter mentions into working links now, keeping
  chapter 6 forward links page-level and chapters 7–12 plain text. This is the
  promised set-wide navigation correction, not a new teaching structure.
- Re-enumerate inventories from the current CLI/source and compare individual
  members rather than accepting the unit proofs' counts. Inventories 4 and 5
  remain unclaimed until the second half; chapter 10 and the owner's stand-in
  comprehension checks remain separate obligations.
- The whole-set term sweep found abbreviated policy-context wording in
  chapters 2–4 (loader, budget, schema, example and invalid-data phrases).
  Qualify those prose references without changing code identifiers or literal
  diagnostics. Both forms of context remain explicitly contrasted in chapter
  2. Recheck the examples and build against the final chapter bytes.
- The owner changed the diagram prerequisite during proof: “We can assume
  that readers are familiar with all of the diagram types we use. If not they
  can look them up.” Update the contract in place, remove generic notation
  tutorials from the first-half prose and legends, and retain figure-specific
  meaning (colour, responsibility boundaries, lifetimes and evidence limits).
  This supersedes the earlier requirement to teach each notation at first use.
  Earlier checks are development checks; freeze the revised subjects before
  the final build, examples and figure comparisons.
- The owner clarified that style consistency means colours, fonts, line
  weights and related visual properties, and set a prose target of a
  16-year-old software engineer. “Front” and “admits” were specifically
  unfamiliar/technical. The contract now names the command-line program the
  “Ravel executable”, keeps the existing programming prerequisites and calls
  for plain action words. CONTEXT.md records that term. Route the whole-set
  wording change through copy-edit, then art (including figure labels), then
  proof before guide migration. The later whole-book art pass owns visual
  consistency across all delivered chapters. This is a new owner requirement,
  not an unexplained second correction of the earlier unit art work.
- The owner suggested “cli” as the name for the command-line program. The
  settled term is **Ravel CLI**, defined once as the `ravel` command-line
  program, then used consistently in prose and diagrams. This supersedes
  “Ravel executable” above; the contract and glossary now use Ravel CLI.

## Set reading and source verification

Read all six raw chapters and the three entry routes as one set: 1–6 in
order; 1–3 then 5 with 6 for lookup; 1–3 then 4 with 6 for lookup. The caller
route has the domain, installation, policy evaluation, job and result model
before it reaches caller inputs and outcomes. The policy route has the same
foundation before request/selection shapes, loaders, settings and imports.
The links added here connect the early prose to existing teaching. Forward
chapter 6 links remain page-level; chapters 7–12 remain plain text.

Checked the behavioural explanations against both specifications and all
three ADRs. No technical disagreement was found. The cross-chapter joins
retain worker reap and directory cleanup before managed launch; publication
before job cleanup and result delivery; first-writer/empty/multiline/Unicode
and option-looking results; publication surviving a later nonzero child exit;
natural exit 0 without publication as failure; cancellation before/after
publication; original errors and secondary cleanup reports; final-write
prefix failure; and group, terminal, same-user and platform limits.
JSON notice emission remains before final outcome selection, with
operationReports conditional on a returned runner ending. Settings are checked
before overrides; a careful sample choice is exactly high, even for an xhigh
static route. Policy context is replacement data with measured reads, not
result authority or an automatic merge.

No in-session reviewer was needed or launched. The new reader/visual
requirements are routed to producers, not reported as already met.

## Inventories re-enumerated from the subjects

The sources were current executable help for all three command lines,
cli.rs, main.rs, refusal.rs, cancellation.rs, run.rs and its program/result
branches; environment.rs and confinement.rs; settings.rs and limits.rs;
context.rs, policy.rs, choice.rs and worker.rs; the SDK declarations,
worker host/main, static table and sample; dispatch.sh and the archive manifest
in scripts/release-common.sh. The two specifications supply the normative
rules, not the earlier unit logs.

| Inventory | Source-derived members compared with chapters 3–6 | Result |
|---|---|---|
| CLI | start-session and end-session; RESULT; -- separator; -h/--help on all three command lines; -V/--version at top level; 15 start options: kind, prompt, prompt-file, task-file, task-id, config, param, context, timeout-ms, context-bytes, policy-env, confine, runtime-read, exit-dir, json | All have reference entries and teaching destinations |
| Statuses | 0 and 1 from main/result handling; 2, 3, 5, 124, 126, 127 from constants and branches; HUP/INT/TERM re-raise plus 128+signal fallback, giving 129, 130, 143 in a shell | Each appears in both status tables with meaning and caller action |
| Streams and notices | stdout/stderr/stdin routing; 11 base routing fields plus conditional operationReports; six policy-context-report fields; failure fields and ten stages | Entries agree with chapters 3–5 and the run/refusal JSON construction |
| Environment | HOME/PATH/TMPDIR/LANG/LC_* worker base; cache variable set to 0; nine reserved exact-name/prefix grant classes; six exact managed scrub names; confined HOME/USER/LOGNAME/PATH/LANG/LC_* plus TMPDIR/TMP/TEMP and fresh RAVEL_EXIT_FILE; XDG_CONFIG_HOME not used for discovery | Each rule covered; worker grants are kept distinct from managed inheritance |
| Settings and bounds | timeoutMs, contextBytes, policyEnv; unknown-key refusal and validation before override; six limits and their values/ranges/origins, plus prompt/task-id/param/result bounds | Each covered; no missing key or bound |
| Shapes | Policy: schemaVersion/version/select/loadContext; SelectionRequest: nine fields; Selected: seven; Refused: four; Context: six; Assessment: two; SourceRecord: four with digest/version alternative; MeasuredSource: four; ReadSource: three; hosts: diagnostic/signal/readText/readJson | Every field and optional/validation rule has reference coverage and chapter 4 teaching |
| SDK exports | Json, SourceRecord, Assessment, Context, MeasuredSource, DeliveredContext, ReadSource, SelectHost, ContextHost, LoadContext, Policy, SelectionRequest, Limits, Selected, Refused, SelectionResult, CHOICE_FILE, readChoice, definePolicy | All 19 exported names present |
| Embedded/static/sample | Two exact embedded specifiers; literal versus package-alias resolution; relative imports versus host reads; package main/exports/autoload controls; Command/Route/agent/routes/selectRoute/policy; sample default/careful and refusals | All covered; no invented real-agent or effort guarantee |
| Installed files | Eleven archive manifest entries: bin/ravel, compiled worker, SDK source/declarations, static source/declarations, sample.ts, two notices, README.md and LICENSE; first nine installed by task install | Each entry present; linked runner distinguished from another executable |

Diagnostic discovery enumerated every simple snake-case string in the current
Rust source, rather than searching a preselected code list. Classification
retained 49 diagnostic codes, all present as reference table rows. Four other
tokens are not public diagnostics: prompt_file is a clap conflict identifier,
remove_dir_all is an operation label, and spawn_missing/spawn_other are private
test-control tags. SDK/static/sample refusal codes (choice_unreadable,
choice_unoffered, incomplete_mapping and one_choice) are separately taught
as owner codes carried beside policy_refused. End-session's plain messages
were read from completion.rs: missing/empty inherited path, invalid/stale
directory, result bound/type/encoding and publication I/O failures are covered
under publication failure, distinct from an empty result. Platform-dependent
OS error text is not claimed as an exhaustive fixed string list.

Inventory 3 is checked for the lifecycle and failure claims chapters 3 and 5
make, including confinement granted/refused and conditional terminal recovery.
Inventory 4 (component obligations) and inventory 5 (contributor workflows)
are not yet claimed; the second half owns them. Chapter 10 remains outside
this node until a supported component and consumer exist.

Graph project: Users-antony-Development-Ravel.simplify-and-componentize,
Tier 2. Initial indexed generation 2026-10-16T07:21:53Z; final coverage
generation 2026-10-16T07:25:31Z, recording complete. Relevant source paths
have no recorded coverage issue and matching metadata. PlantUML paths have
untracked freshness metadata; all eleven source/style files were read
directly. SVGs are deliberately excluded by the graph; raw source reading,
regeneration and the visual sample supply their evidence. A clean coverage
signal is not proof of completeness. Structural discovery used graph search,
the main call trace and exact snippets; literals, tables and config were
checked directly.

## Checks and their limits

- task docs:book PORT=8779 built and served the book. Final
  task docs:book:build wrote the HTML without warnings.
- Final task docs:examples: 13 passed, zero failed/skipped, six labelled
  fragments not run and zero page errors. The pre-wording-edit run is
  superseded; every final chapter example was checked again.
- Final task docs:figures: all nine figures match their sources with the
  existing pinned renderers and fonts. No figure or style was changed here;
  the owner's new visual/notation requirements remain the next art task's.
- task check exited 0: 339 tests passed, 46 skipped, with formatting, lint,
  worker/probe builds and typechecks. No product source or test changed.
- The complete first-half source-link inventory has 402 local chapter
  link/image destinations, all resolving against the final built HTML;
  289 belong to chapter 6. Eighteen rendered img elements (inline and zoom
  copies of nine figures) have nonempty alt text. Known-good links passed;
  missing file, anchor, image and empty-alt controls were rejected. Inventory
  comparators also rejected an added missing code, SDK export, path and option.
- Final before/after SHA-256 comparison: 180 enumerated files under crates
  (excluding node_modules), scripts, docs/book, both specs, ADRs, root
  Cargo/Taskfile/glossary and root dotfiles remained unchanged through the
  final documentation checks. Tools/dependency caches are not part of that
  source snapshot. Logs and individual link/inventory records are under
  target/book-proof-k67; this task records the durable result.
- Read all chapter prose, headings, table rows and example labels. Code and
  literal diagnostics keep their spelling. The context sweep caused the
  qualifications in chapters 2–4; metalinguistic context discussion and C4
  system-context names retain their separate meanings. No placeholder prose
  was found in chapters 1–6; SUMMARY's later draft entries are intentional.
- Independently viewed the local light-page book through Chrome: the reading
  routes and their caller link, caller input tables, chapter 3 outcome figure
  with adjacent explanation, end-session/status entries and diagnostic table.
  This samples a reference entry, exit status, error and figure as requested.
  The VS Code preview surface was blank, so no editor-preview evidence is
  claimed. Raw files were read. GitHub and the unpublished site were not
  reachable as views of this work. Other themes/viewports and every full-size
  diagram were not inspected by this set proof.

These checks establish no new-reader comprehension, actual agent execution,
interactive terminal operation or native x86-64 qualification. Read 1 remains
the owner's labelled stand-in read. The new prose and art correction is live
work; Parts I and II are not declared ready for guide migration until its
following proof passes. No ADR decision or product behaviour changed.

## Tree handoff

The owner's new requirements have a contiguous correction run before guide
migration: copy-edit book-first-half-k98, art book-first-half-k99, proof
book-first-half-k100. Their bodies name the changed contract and precise
wording/style/notation work. Each finds its successor already queued, so
it grows no duplicate tail. The new whole-book art leaf ravel-book-k101 is
after read 1; read 1's live task explicitly inserts second-half planning
before it, with chapter work before whole-book art and proof/read 2 after it.
The book brief carries these current requirements and the migration facts.
There is no unresolved first-half hand-forward list without an owner.
The book node remains open; no parent close or provider-review receipt is
earned by this leaf's retirement.
