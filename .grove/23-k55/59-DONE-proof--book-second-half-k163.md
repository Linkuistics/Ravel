# book-second-half-k163

## Goal

Proof chapters 7, 8, 9, 11 and 12 as a set after their units are proved, and
before the guide fold-in removes the guides that these chapters replace. A
unit's proof reads only its unit. This leaf reads what spans the units: the
two inventories that the second half claims, the terms and facts that two
chapters state, the figures that two chapters share, and the links between the
chapters. Chapter 10 does not exist and is not read.

## Context

- Contract: `docs/book/contract.md`. The units' logs list the guide sentences
  that each chapter replaces. Those lists are the fold-in leaves' inputs, so
  this leaf checks that they are complete.
- **Inventory 4 (component obligations).** Enumerate the ten obligations from
  `docs/specs/session-execution.md` and `docs/components.md` again. Report for
  each of O1–O10 the section in chapters 7, 8 or 9 that teaches it, and what
  is missing. Coverage shows what the book covers, not that a reader understands
  it.
- **Inventory 5 (contributor workflows).** Enumerate every task from
  `task --list` again. Report for each the section in chapter 11 or 12 that
  names it, and the pointer to the chapter that teaches it. The native-entry,
  incumbent and Linux measurement tasks are evidence instruments: they are
  named, with a pointer to the evidence, and are not taught. Check that the
  list in `Taskfile.yml` is the list in the chapters, and not the list in this
  file.
- **Across the set**, check:
  - one fact has one value everywhere: the escalation times, the exit statuses,
    the support statement, the version and identity rules, the count of
    packages and processes;
  - one term has one meaning, and “context” is never unqualified;
  - chapter 3's promises of chapter 9 (“the exact times”, “the terminal rules”,
    “signal masks and failure guarantees”) and of chapter 8 are kept;
  - chapter 1's route table, entry routes and the contract's entry routes
    agree with the chapters as written, including “Changing Ravel” (chapters
    1–3, 7 and 8, then 11, with 9 and 12 as needed). The route “Reusing a
    component” cannot be completed until chapter 10 exists; the book says so;
  - the loose contract (R15) in chapters 1, 2 and 7 agrees, and R14, R12, R13
    and R16 agree wherever their concepts occur in chapters 7–12;
  - the figures that two chapters share are the same figures in the same style:
    the container view (chapters 2 and 7), the source and installation figure
    (chapters 6, 7 and 12), and the cancellation sequences (chapters 5 and 9).
- **Plain-text mentions of chapters 7–12 in chapters 1–6.** The first half
  wrote them as plain text with the number and the title. Turn each into a link
  to its target (with an anchor where a section is meant), and update titles
  that changed (chapter 7 is “The parts of Ravel” now). Enumerate the mentions
  again with a search that you first see find known ones
  (`docs/book/src/01-what-ravel-is-for.md` has several, and so do chapters 3, 4
  and 5). The README's reworded paragraph and `docs/book/src/SUMMARY.md` are
  checked here for the draft entries too: no empty `[…]()` entry remains for
  chapters 7, 8, 9, 11 and 12.
- **The non-`--json` form of an operation report** (`ravel: <report>` on
  standard error, `crates/ravel/src/run.rs`) is in no chapter and chapter 6's
  standard error row says only “operational reports”. Decide here whether
  chapter 6 gains an entry, using what chapters 8 and 9 teach. Record the
  decision with its reason. If you add the entry, it is a sentence-level change
  to chapter 6, and it follows the shared decisions.
- **Source claims.** Spot-check against the code the claims that the units
  marked as hardest to check. The unit proofs checked their own; this read
  checks the claims that two chapters make about one mechanism.
- Check the strict STE and prose-break checks on all of chapters 1–12 that
  exist, with `task docs:ste` run with no arguments now that
  `book-ste-all-chapters-k149` extends its default.

## Done when

- `task docs:book:build`, `task docs:figures`, `task docs:figures:test`,
  `task docs:examples`, `task docs:excerpts`, `task docs:ste -- --strict`,
  `task docs:breaks -- --strict` and `task check` pass on subjects that have
  the same digests before and after.
- Every local link destination of the rendered book and the entry documents
  resolves, with controls (a broken anchor and a missing file fail). Every
  figure of chapters 7–12 loads in the served book (a control image with no
  file fails). Report the viewing environments reached and not reached.
- Inventories 4 and 5 are reported as the contract requires, with what is
  missing, and each gap is closed or routed to the stage that owns it by a
  correction run. A second send-back of one stage against one document goes to
  the owner.
- Every plain-text mention of a chapter 7–12 in chapters 1–6 is a link, or is
  listed with its reason.
- The decision on the operation report is recorded.
- The node brief's list of second-half handoffs is cleared of what this leaf
  closes. No defect goes forward without an owner. Mark which guide sentences
  the units did not place, so that the fold-in leaves have no unplaced content.
- A guide is removed only after this leaf passes. Say in the log that
  “absorbed” is a statement about the chapters as proved here.

## Notes

- The whole-book proof after `ravel-book-k101` reads the finished book again,
  after the whole-book art pass. This leaf does not replace it. It exists
  because inventories 4 and 5 and the cross-chapter facts are set questions
  that a unit's proof cannot answer, and because the guides must not be
  removed before the chapters that absorb them are checked as a set.
- The read 2 tasks are the contract's: explain why Ravel has a TypeScript worker
  and two Rust packages, follow one session through the code to where its final
  cancellation outcome is chosen, and build and test Ravel from a checkout.
  Check, without holding the read, that the chapters make each task possible.

## Decisions (running log)

- 2026-10-19: Chapter 6 gains the literal non-`--json` runner-report format,
  `ravel: <report>`, and a link to chapter 8's explanation. Chapters 8 and 9
  already teach the mechanism and its limits. A direct reference entry should
  let a reader recognise that stderr line without a sequential read.
- 2026-10-19: Convert the first half's forward references into chapter or
  section links, including the abbreviated numbers in chapter 1's route table.
  Keep chapter 10's absent-text notices as plain text because no target exists.
  Reword the README's obsolete draft paragraph now; the fold-in leaves retain
  guide removal, the notices-line removal and the remaining README reduction.
- 2026-10-19: The guide-map audit found two unplaced details: synchronous,
  prompt, nonpanicking runner callbacks (`local-runner.md`) and explicit
  cleanup disabling destructor retries, unknown identity and moved-directory
  recovery (`session-channel.md`). The units assigned them to chapters 8/9,
  but neither chapter states them. Add both to chapter 8's existing mechanism
  explanations, checked against runner callback docs and `PrivateDirectory`.
  Proof owns every class, so these bounded corrections need no send-back.

## Inventory 4: component obligations

Re-enumerated from the current `docs/components.md` obligation table and the
rules in `docs/specs/session-execution.md`, rather than from the contract's
summary. Each row below has teaching in the proved chapter set. No obligation
is missing. Coverage establishes the presence of teaching, not comprehension.

| Obligation | What the subject requires | Teaching destination |
|---|---|---|
| O1 | Owner TypeScript policy, SDK, settings and discovery remain compatible | Chapter 7, “The worker executes TypeScript before the launch” and “The SDK describes the policy interface”; chapter 8, “Prepare the request”. Chapters 4/6 retain author inputs |
| O2 | No worker result authority; identity before policy handover; worker reap before managed launch | Chapter 7, worker section and “The installed pair stays together”; chapter 8, “Evaluate and check the policy” |
| O3 | Check policy decisions, refusals, exceptions, malformed data and limits without fallback; retain caller prompt/parameters | Chapter 8, “Prepare the request”, “Evaluate and check the policy” and “Follow a refusal or launch failure” |
| O4 | Direct argv in caller environment and cwd; requested confinement or refusal; fresh nested result context | Chapter 8, “Prepare the managed launch” and “Follow a refusal or launch failure”; chapter 9, “Separate noninteractive execution from confinement” and “Apply filesystem confinement” |
| O5 | First complete result wins; preserve empty, multiline and Unicode values; reject missing/stale context and invalid files | Chapter 7, “The result channel carries the value”; chapter 8, “Send the result” and “Read the result”, with the caller failure behaviour in chapters 5/6 |
| O6 | Result success needs the matching launch's reap and confirmed group cleanup; later child status is not decisive | Chapter 7, result-channel association; chapter 8, “Finish the managed job” and “Read the result”; chapter 9, “Stop the group before the reap” |
| O7 | INT/TERM/HUP cancellation suppresses a sent result and propagates after cleanup attempts | Chapter 8, “Follow cancellation” and “Choose the final outcome”; chapter 9, “Handle cancellation” and “Preserve the entry signal state” |
| O8 | Recover foreground and saved modes under the stated holder/identity conditions; stop the group before reap | Chapter 9, “Stop the group before the reap” and “Lend and recover the terminal” |
| O9 | Only result bytes reach stdout after explicit cleanup; child output and diagnostics use stderr; cleanup/write failures prevent success | Chapter 8, “Prepare the managed launch”, “Complete cleanup and write the result” and “Choose the final outcome” |
| O10 | Matched installed CLI/worker and shipped SDK; no other project or host Bun/Node | Chapter 7, “The installed pair stays together”; chapter 12, “Keep the version and pair together” and installed/archive checks |

## Inventory 5: contributor workflows

The new `task --list --json` output has 57 public names. The subject is the
current Taskfile, including its internal/public distinction. The internal
`release` recipe is not a public extra. Each public name occurs once in
chapter 11's inventory. The table below was derived from that inventory and
joined to the actual task list, then checked against the built section IDs.
No public name is missing, duplicated or extra.

Native-entry, incumbent, campaign, Linux and R01 tasks have evidence pointers
in chapter 11, “Read the evidence for a change”. Their execution procedures
remain outside the book. All other names have development or release teaching.
The release task table in chapter 12 and its specific procedures supply the
release teaching, in addition to chapter 11's name index.
| Public task | Named in | Explanation |
|---|---|---|
| `setup` | Chapter 11, “Find a development task” | Chapter 11, `prepare-a-checkout` |
| `build` | Chapter 11, “Find a development task” | Chapter 11, `build-the-matched-pair` |
| `worker` | Chapter 11, “Find a development task” | Chapter 11, `build-the-matched-pair` |
| `typecheck` | Chapter 11, “Find a development task” | Chapter 11, `build-the-matched-pair` |
| `deps` | Chapter 11, “Find a development task” | Chapter 11, `build-the-matched-pair` |
| `test` | Chapter 11, “Find a development task” | Chapter 11, `choose-a-test-seam` |
| `test:session` | Chapter 11, “Find a development task” | Chapter 11, `choose-a-test-seam` |
| `test:cli` | Chapter 11, “Find a development task” | Chapter 11, `choose-a-test-seam` |
| `test:runner` | Chapter 11, “Find a development task” | Chapter 11, `choose-a-test-seam` |
| `test:hostile` | Chapter 11, “Find a development task” | Chapter 11, `choose-a-test-seam` |
| `test:list` | Chapter 11, “Find a development task” | Chapter 11, `choose-a-test-seam` |
| `check` | Chapter 11, “Find a development task” | Chapter 11, `run-the-checks-before-handoff` |
| `runtime:integrity` | Chapter 11, “Find a development task” | Chapter 11, `run-the-checks-before-handoff` |
| `test:terminal-recovery` | Chapter 11, “Find a development task” | Chapter 11, `run-the-checks-before-handoff` |
| `install` | Chapter 11, “Find a development task” | Chapter 11, `install-and-test-the-installed-pair` |
| `smoke` | Chapter 11, “Find a development task” | Chapter 11, `install-and-test-the-installed-pair` |
| `probes` | Chapter 11, “Find a development task” | Chapter 11, `install-and-test-the-installed-pair` |
| `docs:book` | Chapter 11, “Find a development task” | Chapter 11, `check-the-book` |
| `docs:book:build` | Chapter 11, “Find a development task” | Chapter 11, `check-the-book` |
| `docs:figures` | Chapter 11, “Find a development task” | Chapter 11, `check-the-book` |
| `docs:figures:update` | Chapter 11, “Find a development task” | Chapter 11, `check-the-book` |
| `docs:figures:test` | Chapter 11, “Find a development task” | Chapter 11, `check-the-book` |
| `docs:examples` | Chapter 11, “Find a development task” | Chapter 11, `check-the-book` |
| `docs:examples:test` | Chapter 11, “Find a development task” | Chapter 11, `check-the-book` |
| `docs:examples:mutants` | Chapter 11, “Find a development task” | Chapter 11, `check-the-book` |
| `docs:excerpts` | Chapter 11, “Find a development task” | Chapter 11, `check-the-book` |
| `docs:excerpts:test` | Chapter 11, “Find a development task” | Chapter 11, `check-the-book` |
| `docs:ste` | Chapter 11, “Find a development task” | Chapter 11, `check-the-book` |
| `docs:ste:test` | Chapter 11, “Find a development task” | Chapter 11, `check-the-book` |
| `docs:breaks` | Chapter 11, “Find a development task” | Chapter 11, `check-the-book` |
| `docs:breaks:test` | Chapter 11, “Find a development task” | Chapter 11, `check-the-book` |
| `book:integrity` | Chapter 11, “Find a development task” | Chapter 11, `check-the-book` |
| `docs:components` | Chapter 11, “Find a development task” | Chapter 11, `read-the-evidence-for-a-change` |
| `docs:result-job` | Chapter 11, “Find a development task” | Chapter 11, `read-the-evidence-for-a-change` |
| `native-entry:build` | Chapter 11, “Find a development task” | Chapter 11, `read-the-evidence-for-a-change` |
| `native-entry:front` | Chapter 11, “Find a development task” | Chapter 11, `read-the-evidence-for-a-change` |
| `native-entry:runner` | Chapter 11, “Find a development task” | Chapter 11, `read-the-evidence-for-a-change` |
| `native-entry:start` | Chapter 11, “Find a development task” | Chapter 11, `read-the-evidence-for-a-change` |
| `test:native-entry` | Chapter 11, “Find a development task” | Chapter 11, `read-the-evidence-for-a-change` |
| `native-incumbent:build` | Chapter 11, “Find a development task” | Chapter 11, `read-the-evidence-for-a-change` |
| `test:native-incumbent` | Chapter 11, “Find a development task” | Chapter 11, `read-the-evidence-for-a-change` |
| `test:native-incumbent-controls` | Chapter 11, “Find a development task” | Chapter 11, `read-the-evidence-for-a-change` |
| `test:native-campaign` | Chapter 11, “Find a development task” | Chapter 11, `read-the-evidence-for-a-change` |
| `test:native-campaign-controls` | Chapter 11, “Find a development task” | Chapter 11, `read-the-evidence-for-a-change` |
| `native-linux:image` | Chapter 11, “Find a development task” | Chapter 11, `read-the-evidence-for-a-change` |
| `test:native-linux` | Chapter 11, “Find a development task” | Chapter 11, `read-the-evidence-for-a-change` |
| `test:native-r01-controls` | Chapter 11, “Find a development task” | Chapter 11, `read-the-evidence-for-a-change` |
| `release:archives` | Chapter 11, “Find a development task”; chapter 12 task table | Chapter 12, `build-archives-before-a-release` |
| `release:smoke` | Chapter 11, “Find a development task”; chapter 12 task table | Chapter 12, `test-the-installed-archive` |
| `release:test` | Chapter 11, “Find a development task”; chapter 12 task table | Chapter 12, `check-the-archive-contents` |
| `release:formula` | Chapter 11, “Find a development task”; chapter 12 task table | Chapter 12, `prepare-and-test-the-homebrew-formula` |
| `release:doctor` | Chapter 11, “Find a development task”; chapter 12 task table | Chapter 12, `prepare-a-version-cut` |
| `release:notes` | Chapter 11, “Find a development task”; chapter 12 task table | Chapter 12, `prepare-a-version-cut` |
| `release:lifecycle:test` | Chapter 11, “Find a development task”; chapter 12 task table | Chapter 12, `prepare-a-version-cut` |
| `release:patch` | Chapter 11, “Find a development task”; chapter 12 task table | Chapter 12, `cut-and-deliver-a-release` |
| `release:minor` | Chapter 11, “Find a development task”; chapter 12 task table | Chapter 12, `cut-and-deliver-a-release` |
| `release:major` | Chapter 11, “Find a development task”; chapter 12 task table | Chapter 12, `cut-and-deliver-a-release` |

## Cross-chapter read and source checks

Chapters 7–9, 11 and 12 were read whole as raw Markdown, with the first-half
entry routes, chapter 3 promises, chapter 6 reference, contract, shared word
decisions, glossary and three ADRs. Chapter 10 has no text and was not read.

- The two Rust packages are source boundaries, not two service processes.
  The runner executes inside the CLI; the worker is a separate process, reaped
  before the job; `end-session` is another invocation of the CLI.
- Chapters 1/2/7 agree on the loose program contract. The policy controls
  whether the selected program receives the prompt. Subscription motivation
  stays at the intended-use level. There is no provider integration claim.
- Chapters 3/8/9 distinguish a sent candidate from a returned result. The
  result stays unchanged, the CLI's cancellation suppresses it, and another
  send races shutdown. A failed final write can leave a prefix on stdout.
- Chapter 9 fulfils chapter 3's promises: the exact escalation durations,
  terminal holder rules, signal masks and conditional failure guarantees.
  Chapter 8 supplies the code path and final cancellation/outcome decisions.
- Grace is 2 seconds, kill grace 5 seconds, ordinary polling 500 ms, killed
  polling 10 ms, final group-stop pause 20 ms, group confirmation up to 1 second,
  and post-KILL retries bounded by 2 seconds. These are separate intervals,
  not a seven-second session deadline. `run.rs` constants and runner `watch`
  agree with the chapter 9 tables and figure text.
- Chapter 8's final signal expression matches `Handlers::finalise`:
  `front.or(outcome).or(late).or(received()).or(pending)`. This is source
  priority, not arrival order. `settle` reports supplied failures and chooses
  cancellation, original failure, cleanup failure, surviving group, then
  result. `result_after` rejects a surviving group before it reads the file.
- `run` passes its channel to the runner and that runner's `Ended` back to
  the channel. The type cannot prove this association. Common cleanup and
  finalisation precede the unchanged write and flush. Plain runner reports
  and JSON `operationReports` agree with chapters 6/8/9.
- `worker::evaluate` requires confirmed wait, directory removal and no failed
  stop operation. Its output-limit/conversation/teardown order agrees with
  chapters 7/8. `verify_hello` checks protocol, package version and build ID.
- Chapters 7/12 agree on protocol 1 and the sorted worker-source identity
  inputs. The identity pairs builds and is not a signature. Chapter 3 retains
  its release placeholder, while chapter 12 distinguishes the 1.0.0 checkout
  from the tag. These are different facts, not competing version values.
- Chapters 1/9/11/12 agree on measured aarch64 macOS and aarch64 GNU/Linux,
  with native x86-64 Linux unqualified. Archive/QEMU smoke is not native
  kernel qualification. The dated 343-test scratch-checkout observation in
  chapter 11 includes its four added tests; it does not describe this tree's
  unchanged product suite.
- The context terms remain qualified. Literal field names, source excerpts,
  diagnostic text and the glossary's explicit discussion of the word are not
  unqualified uses of a domain term.
- Chapter 1's Change Ravel route matches the contract: 1–3, 7/8, then 11,
  with 9/12 as needed. The reuse route explicitly awaits chapter 10.
  Chapters 7, 8 and 11 make the three read-2 tasks possible. This session did
  not hold the owner's read or infer comprehension from coverage.

Tier 2 graph evidence used project
`Users-antony-Development-Ravel.simplify-and-componentize`, ready with coverage
generation `2026-10-18T13:02:53Z`. Exact searches were complete at their stated
limits. The outbound depth-one trace from `run` returned 43 relationships,
without pagination, locating the composition calls. Exact snippets supplied
`run`, `settle`, `Handlers::finalise`, `SessionChannel::result_after`, runner
`watch`, and worker `evaluate`/`verify_hello`. Direct source supplied constants,
callback docs and `PrivateDirectory` cleanup. Coverage found no recorded gap
and matching metadata for those six Rust files. This is best-effort evidence,
not a completeness proof. Markdown, scripts, specifications and Taskfile were
read directly. SVGs are excluded from the graph by suffix; rendered browser
images, their layout text and regeneration checks supply that evidence.

## Guide-map audit for k164 and k165

The units' group maps were compared with the current guide sections and actual
chapter text: k152 (components and runner), k155/k170 (channel and composition),
k158/k174 (job and terminal), k181 (README development and authority),
k162/k182 (release and README delivery), and k187 (Pages additions).
Their final heading “Complete cleanup and write the result” replaces the
older map label “Clean up and write the result”.

Two groups lacked text despite assigned destinations. Both are now placed:

| Guide group previously unplaced | Proved destination |
|---|---|
| `local-runner.md`, “Native entry and return”: synchronous, prompt, nonpanicking observers | Chapter 8, “Finish the managed job”, including the start-check callback's fallible return |
| `session-channel.md`: explicit cleanup disarms best-effort destructor retries, unknown identity forbids deletion, moved owned directories can need manual cleanup | Chapter 8, “Complete cleanup and write the result” |

The remaining groups have the homes the unit maps name. Guide-only historical
measurements, E1–E7 provenance and comparison history stay outside teaching;
k164 owns their preservation/classification in the evidence document, as its
brief requires. Old claims that native entry is entirely unmeasured, the
release guide's 0.0.0 development/first-minor plan, its doctor/export claims
and its unqualified no-workflow wording are stale. The maps explicitly
replace them with checked current statements rather than silently discard
required behaviour. The callback and cleanup additions above leave no guide
teaching group without a home. The fold-ins still classify each group
independently before removal. No guide is removed by this leaf.

“Absorbed” is a statement about the chapters as proved here. It does not mean
an unmeasured independent component exists or that a new reader understands
these chapters. No correction run or ADR change is needed.

## Forward references and shared figures

The same mention search found known unlinked references in the parent versions
of chapters 1, 3, 4 and 5 before the clean current-source check was credited.
After Markdown links were removed from the search input, only three chapter-10
mentions remained: chapter 1's reuse route and absent-text paragraph, and
chapter 6's absent-text note. They remain plain text because chapter 10 has no
target. Abbreviated 8/11 and 7/9 numbers in chapter 1's route table were also
linked. SUMMARY has real targets for 7/8/9/11/12 and retains only chapter 10's
draft entry. The README now links the existing chapters and states the actual
chapter-10 gap.

There is one process-authority figure, reached from chapter 2 and linked by
chapter 7. Chapters 6/7/12 use the same source-components/installed-files
figures through chapter 6's installed-layout entry. Chapters 5/8 embed the
same two cancellation SVGs and chapter 9 links those sequences. No duplicate
editable figure or inconsistent variant was introduced. Shared crisp style
and renderer remain unchanged. Existing dense-text and size assessments stay
with whole-book art k101.

## Final validation and views

All eight required commands passed in one sequential run after the final
chapter edits. `target/k163-checks/subjects.txt` enumerates 3,535 tracked files
outside `.grove/` and `retired/`, including source, scripts, configuration,
pins, book conventions, guides and evidence inputs. `before.json` and
`after.json` contain identical per-file SHA-256 values. Each command also
compared those same subjects before the next command ran. No subject changed.
`results.json` gives each exit status, elapsed time and empty changed-path list.
The retained logs are named after each task with colons replaced by hyphens.

| Command | Fresh result |
|---|---|
| `task docs:book:build` | Passed with no mdBook warning or error |
| `task docs:figures` | Fourteen SVG/layout comparisons passed |
| `task docs:figures:test` | Fetch refusals, reproducibility, environment isolation, comparison/lint controls and page build passed |
| `task docs:examples` | Fourteen passed, zero failed or skipped; 27 labelled fragments not run; zero page errors |
| `task docs:excerpts` | Eight passed, zero failed |
| `task docs:ste -- --strict` | Zero problems in all 28 default files, including all eleven chapters |
| `task docs:breaks -- --strict` | Zero prose runs over 450 words in all eleven chapters |
| `task check` | All stages passed; 339 workspace tests passed, 46 skipped |

Before this final batch, the STE check rejected the two newly enlarged
chapter-8 paragraphs at seven/eight sentences. Paragraph splits corrected
those failures. The final batch above, not that earlier check, is the evidence
for completion. No source edit occurred during it.

The HTML link check enumerated every href/src in all fifteen built HTML pages,
including mdBook navigation and the combined print page. All 1,831 local
occurrences resolved against files and HTML IDs. Twenty-two local destinations
from README and SUMMARY also resolved, with chapter links checked against the
built page. Controls with a missing file, broken anchor and missing image
were rejected. The checker initially treated the 404 page's root `/` link as
a filesystem root. Correct URL-root resolution made it a book-root link;
the final run and all controls then passed. This was an instrument correction,
not a document defect. Remote source URLs and their content were not checked.

Chrome served the built files at `http://127.0.0.1:8899/`, in a light theme,
with a 1040 × 848 viewport and measured 750-pixel main column. Its print page
loaded all fourteen unique figure files, including shared first-half figures.
The images reported complete with positive natural widths. A separate scratch
page with an absent image reported complete with zero natural width, proving
that the same predicate rejects that failure. The scratch page was removed.
Chapter 8's four figures, chapter 9's two and chapter 12's release figure also
loaded on their own pages. Screenshots inspected chapter 7's part table,
chapter 8's cleanup section including the added details, chapter 9's escalation
table and figure opening, chapter 11's task inventory, and chapter 12's release
order. Tables reported no horizontal overflow on those chapter pages.

VS Code's existing light Restricted Mode Markdown preview supplied chapter
12's opening, identity table and archive-construction sections. Raw Markdown
supplied the whole-document reads. Other editor chapters, themes and narrower
viewports were not inspected. GitHub rendering and the public Pages site were
not inspected; the workstream is not pushed and no release carries this book.
No real harness, public release, new platform measurement or owner read ran.

## Closure

Inventories 4 and 5 have no gap. The two guide-map omissions are fixed, and
chapter 6's report-format decision and all first-half forward references are
settled above. No defect lacks an owner. The book brief now holds only the
remaining fold-in, whole-book art/proof and read-2 work. Existing dense-text,
figure-size, chapter-5 term and chapters-5/6 signal-name work retains its stated
later owner. The node's ordinary hand-forward list remains empty.

No review or correction leaf was cut, and no ADR or glossary decision changed.
No live review names this leaf as its producer. The parent `ravel-book-k55`
stays open for k164, k165, k101 and k166; retirement closes no ancestor.
The guides remain intact for their authorised fold-in leaves.
