# jobs-signals-and-the-terminal-k174

## Goal

Complete part B of chapter 9 in
`docs/book/src/09-jobs-signals-and-the-terminal.md` under
`docs/book/contract.md`. Read the whole part A and preserve its section order.
Add the three reserved second-level sections, in this order:

1. “Lend and recover the terminal”.
2. “Preserve the entry signal state”.
3. “Know the guarantees and their limits”.

These sections deliver the terminal, native-entry and failure-limit scope in
the document node brief. Part A's job, escalation, cleanup, cancellation and
confinement sections are already present. No empty headings await completion.

## Context

- Read `jobs-signals-and-the-terminal-k158` for source evidence, obligation
  and outcome inventories, guide destinations, review scope and checks.
- Teach controlling terminal, foreground group and saved modes where needed.
  Explain ordinary recovery, delegated foreground after signal death, normal
  exit with another foreground holder, and failed exec. Preserve the exclusive
  foreground-authority and no-unrelated-ID-reuse conditions. A recorded group
  after failed spawn never grants authority to signal that number.
- Teach signal dispositions and masks across exec, the capture instant, the
  runner's persistent process-wide changes, and the CLI's checked restoration.
  Distinguish child entry values from restoration of a returning host. Explain
  what the runner does not restore. Started means spawn-reported success, not
  confirmed exec. Re-derive the facts from current source.
- State quoteable guarantees, each with its limit: same-user tampering, abrupt
  CLI death, descendants that leave the group, detached policy children,
  concurrent embedding, and unqualified native x86-64 Linux. Do not present a
  persistent runner or native x86-64 qualification as delivered functionality.
- Part A distinguishes ordinary unconfined CLI execution without a terminal
  from `run_noninteractive`, a separate detached runner API without a channel.
  Preserve that distinction. Its cancellation section links chapter 5's two
  sequences. Its normal cleanup table signals before reap and queries after
  reap. Only ESRCH confirms absence. EPERM retention needs actual reap,
  successful terminal recovery and group absence.
- Part A's complete fake sends `finished`, ignores TERM, and forces KILL.
  Fourteen complete book examples passed on Darwin arm64. Fifty-four selected
  runner cases passed, including PTY, noninteractive and Seatbelt cases. This
  is no Linux measurement, interactive CLI example or universal security proof.
- Part A's single review found no issue in its assigned pre-reap order scope.
  It did not review part B. Apply this leaf's own ordinary review allowance.

## Done when

- The whole chapter fulfils the node's scope. O8 is taught in full, O6 and O7
  agree across both halves, and the ten-obligation and lifecycle inventories
  are re-derived with their destinations and outstanding owners recorded.
- Every signal/terminal/cleanup claim is checked against the subject and the
  relevant tests. Each guide statement replaced by part B has a destination.
- The terminal sequence's participants, branches, modes and conditions are in
  the node brief's art handoff or this leaf's log. Art owns its drawing.
- Complete examples pass `task docs:examples`; terminal-only fragments state
  what they omit. Book build, strict STE and strict prose-break checks pass.
- Inspect later siblings before any cut. Copy-edit already follows this leaf,
  so do not add another copy-edit. Retire and seal this leaf when complete.

## Decisions (running log)

- The named structure brief is `docs/book/contract.md`, section 1. It names
  “a 16-year-old software engineer with no background in Ravel”, gives the
  ordered twelve-chapter plan and chapter 9's outcome (“reason about process
  groups, escalation, reaping, terminal lending and recovery, confinement,
  entry signal state, and the failure guarantees with their limits”), and
  emphasises “What success means” and “The stated limits, said plainly”. Its
  exclusions keep evidence campaigns and undelivered contracts out of the
  teaching. The node brief refines chapter 9 into the ordered A/B sections.
  All three structure-brief prerequisites are present.
- Preserve part A's heading order and append the three reserved sections.
  Explain the terminal's different recovery branches in a table. Explain
  signal state through its capture, child application and CLI cleanup, with
  the runner's persistent host changes separate. Finish with quoteable
  guarantees and a specific limit for each. No new example or figure is
  needed in this draft; art owns the terminal sequence.
- Source, not the guides' historical native-entry status, owns technical
  claims. The graph is ready for this checkout at Tier 2. The first search
  used a regex as a file glob and found nothing; corrected separate glob
  queries found the required symbols with no remaining pages. The reclaim
  trace has five inbound and thirteen outbound rows. Ambiguous heuristic
  edges are not authority for the recovery conditions, which direct source
  supplies. Coverage and exact source checks precede the narrow review.
- Normal-exit recovery preserves another holder. Known signal death permits
  wider recovery, except from the launcher's or POSIX session leader's group.
  Failed spawn and unknown status use the narrower recorded-child rule.
  Exclusive foreground authority and no unrelated ID reuse remain explicit
  premises. A recorded attempted group never permits a post-failure signal.
- `EntrySignals` supplies child values, not capture or host restoration.
  The CLI captures before Rust startup changes, repairs waitability before
  either child, and checks its own mask/disposition transitions. A separate
  continuing-host or concurrency contract is not delivered by these actions.
- Coverage for the 26 bootstrap/source/document paths at generation
  `2026-10-18T09:56:50Z` reports complete metadata, matching files and no
  recorded gap. No missed source range required fallback. This remains a
  best-effort signal. The worker's separate `child_watching` implementation
  was verified directly at `worker.rs:1050`, called before spawn at line 338;
  it is not an inbound call of the runner's similarly purposed function.
- Keep the POSIX SIGCHLD exception explicit rather than generalising every
  ignored disposition across exec. The primary POSIX.1-2024 exec reference
  permits either ignored or default SIGCHLD after exec. The chapter links that
  reference. Ravel's own pre-exec restoration and measured supported-target
  behaviour remain the subject, not a portable promise for every POSIX host.
- The narrow review claim is that the terminal branch table teaches exactly
  the recovery authority the implementation and test arrangements support.
  A mistaken branch could tell a consumer to take an unrelated terminal.
  One fresh reviewer receives only that section, its normative contract and
  evidence navigation, with an issues-only adversarial prompt. Its distinct
  instrument compares the table against the PTY arrangements and assertions.

## Structure and source verification

The whole chapter moves from the job and shutdown order through cancellation
and launch modes to terminal ownership, entry state and limits. The three new
sections preserve part A's order. Definitions precede the operations that need
them; the terminal table separates outcomes; the signal tables distinguish
child values from launcher lifetime; each guarantee has a visible limit.
This supplies the chapter's stated reader outcome, not measured comprehension.

| Subject claim | Current source and discriminating test |
|---|---|
| `/dev/tty`, ownership gate, saved modes, child-side handoff, later foreground lending | `run.rs`: `Terminal`, `Lease::hold_before_spawn`, `Lease::lend`, `run_with_output`; `tests/job.rs` background and raw-mode cases |
| Normal versus signal death, session-leader exclusion, failed-spawn/unknown-status authority | `Lease::reclaim`; `tests/job.rs` paired foreign-holder cases; `run/terminal_tests.rs` failed-exec and S04 cases |
| Attempt record before handoff, close-on-exec pipe, missing record, no numeric stop after failed spawn | `AttemptGroup`, `record_attempt`, failed-spawn branch of `run_with_output`; terminal record/report controls |
| Checked terminal operations, SIGTTOU restoration and error composition | `Terminal::with_ttou_ignored`, `recovery_report`, `supervise_reported`; terminal report controls and paired CLI conversion task |
| Early immutable capture, required-query failure, child dispositions then mask | `signal_state.rs:record/recorded/entry`; runner `EntrySignals::reinstate`; native E01/E02/F01 and runner F06 assertions |
| Worker and managed waitability repairs remain separate from child values | `worker.rs:child_watching/evaluate`, runner `restore_child_watching`; native E03/E04 assertions and job SIGCHLD case |
| Runner handlers persist, ignore respected, transparent INT policy | `install_termination_handler`, `run_with_output`; job ignored-HUP, transparent-state and INT cases, runner E05 direct/PTY cases |
| Started is spawn success, failed parent-mask transition cleans up | `LaunchEvent`, `supervise_started`, CLI `run` start check; terminal signal-death case, native F05 Started assertions |
| Reader shields, final blocked snapshot, saved handled actions, late latch and queued-signal rules | `worker.rs:Capture::start`, `cancellation.rs:Shield/Handlers::look/finalise/reraise`; cancellation unit controls and native F04/F05/E08 assertions |
| Error prevents result, only actual reap counts, pre-reap stops, ESRCH-only absence and conditional EPERM | Runner `OwnedChild`, `watch`, `supervise_reported`, `confirm_gone`; CLI `run/settle`, part A's existing tests and source record |
| Measured platform scope, no native x86-64 claim | Completion spec's conformance paragraph and `native-entry-development-evidence.md`, “Support limits, per target” (k52) |

The native E01–E04/F01/F04/F05/E08 assertions are source checks here, not a
new execution of the complete native campaign. The support statement restates
the existing immutable measurements. The chapter carries no campaign counts,
receipt tutorial or claim of independent adoption. No production source or
architectural decision changes.

## Re-derived coverage inventories

Inventory 4 was enumerated again from the session specification's installed,
policy, request and managed-result sections, then reconciled with the guide's
O1–O10 table. These are the final destinations, not a list inferred from the
new headings. Unit proof and set proof still own their later verification.

| ID | Required behaviour | Destination and outstanding owner |
|---|---|---|
| O1 | Preserve TypeScript policy, SDK, settings and discovery | Chapters 4 and 6, chapter 7 worker/SDK sections. No chapter 9 remainder |
| O2 | Keep result authority out of policy, verify identity and reap worker before managed launch | Chapter 7 worker section, chapter 8 “Evaluate and check the policy”. Chapter 9 entry-state section adds the waitability prerequisite |
| O3 | Judge policy data/refusals/failures without fallback, preserve caller prompt/parameters | Chapter 8 request/evaluation/refusal sections, chapters 4–6. No chapter 9 remainder |
| O4 | Execute exact selected argv with caller context, requested confinement or refusal, fresh nested result context | Chapter 8 launch preparation; chapter 9 noninteractive and confinement sections |
| O5 | Keep first complete exact result, preserve empty/multiline/Unicode values, reject stale context and invalid files | Chapter 8 send/read sections and chapter 5 acceptance. Chapter 9 limits qualify same-user authority |
| O6 | Require the matching reap and confirmed group absence, independently of later child status | Chapter 9 escalation, fake example, pre-reap cleanup and guarantees. Both halves preserve ESRCH-only absence and EPERM's three prerequisites |
| O7 | Suppress result on applicable INT/TERM/HUP cancellation and propagate after cleanup | Chapter 9 cancellation and entry-state sections, chapter 5 sequences, chapter 8 final priority. Child terminal death alone is distinct from CLI cancellation |
| O8 | Recover foreground and saved modes under the holder rules, with group stops before reap | Chapter 9 pre-reap and terminal sections. Failed spawn and unknown status retain explicit authority/ID premises. Art still owes the terminal sequence |
| O9 | Reserve stdout for result after cleanup, report cleanup/output failures | Chapter 8 cleanup/final outcome, chapter 9 guarantees and output-prefix limit |
| O10 | Use the matched installed CLI/worker pair without host runtime | Chapters 6 and 7. Chapter 12 still owns delivery instructions |

Inventory 3 was re-derived from the session specification's request, selection
and managed-result outcomes and the completion specification's one-decision,
failure and native-entry sections. The final map includes the earlier half's
mechanisms so no second-half summary silently narrows the scope.

| Outcome or mechanism | Destination and outstanding owner |
|---|---|
| Result sent and returned unchanged after cleanup, including empty/multiline/Unicode/option-looking values and later nonzero exit | Chapters 3, 5 and 8; chapter 9 fake extends the example to KILL death |
| Repeated send retains the first value, subject to shutdown race | Chapters 3, 5, 6 and 8 |
| Natural child exit without result, even status 0 | Chapters 3, 5 and 8; chapter 9 cleanup still stops descendants |
| Missing, empty or stale result context and invalid result file | Chapters 5, 6 and 8; chapter 9 limits distinguish validation from authentication |
| Malformed CLI/settings, policy file/data/refusal/exception and selection timeout | Chapters 4–6 and chapter 8 refusal path |
| Missing/stale/mismatched/failed worker | Chapters 6–8; chapter 9 entry-state repair makes reap possible |
| Selected program absent or unexecutable | Chapters 6 and 8; chapter 9 terminal section covers failed exec after handoff |
| INT/TERM/HUP in selection or supervision, including after result | Chapter 9 cancellation and entry-state sections; chapter 5 sequences; chapter 8 final priority |
| Grace, TERM, kill grace, KILL and early natural exit | Chapter 9 escalation and executable fake; timing figure remains art's |
| Group signals before reap, group survivor, failed absence query | Chapter 9 “Stop the group before the reap” and guarantees |
| Supervision/cleanup failure, partial acquisition, unconfirmed ownership and bounded retries | Chapter 8 failure/cleanup; chapter 9 pre-reap, terminal and guarantees sections |
| Unwritable output, failed final write/flush and possible prefix | Chapters 6 and 8; chapter 9 guarantees |
| Retained operation reports and conditional EPERM | Chapter 9 pre-reap table and prose, chapter 8 final output |
| Nested sessions replace only the new child's result context | Chapters 5 and 8 |
| Ordinary CLI without a terminal versus detached `run_noninteractive` | Chapter 9 mode table, preserved unchanged |
| Confinement granted/refused, absent/unusable backend, no fallback | Chapter 9 confinement, chapter 5 complete examples |
| Terminal lend, normal/signal/unknown status recovery, failed exec | Chapter 9 “Lend and recover the terminal”; art owns the sequence |
| Entry capture, child mask/dispositions, waitability, reader shields and checked transitions | Chapter 9 “Preserve the entry signal state”; chapter 8 owns detailed outcome precedence |
| Same-user attacks, abrupt CLI death, group escape, detached policy children, concurrent host and x86-64 limits | Chapter 9 “Know the guarantees and their limits” |

No chapter 9 teaching item remains assigned to a later draft. Copy-edit, art
and proof are still due. Chapter 11 owns contributor test commands, chapter 12
delivery, and the set proof owns cross-chapter links and inventory reconciliation.

## Guide destinations for part B

These sentence groups supplement k158's part A map for fold-in k164. They
authorise no removal now. Historical conformance statements in the guides are
replaced by the current source and qualified support statement, not repeated.

| Guide statement group | Final destination |
|---|---|
| `local-runner.md`, Boundary: `EntrySignals` and signal re-raise | Entry-state section, child-state table and final-condition table |
| Boundary: spawn/reap notifications | Entry-state `Started`/`Reaped` paragraph, with chapter 8's lifecycle teaching |
| Native entry and return: TERM/HUP/INT handlers, preserved ignores and persistent repair | Entry-state runner-handler paragraphs and boundary table |
| Caller-supplied entry values, capture instant, no host restoration | Entry-state definitions, initializer and boundary table |
| Consumer owns signal/terminal lifetime, transparency and late cancellation, no concurrent guarantee | Entry-state cleanup and guarantees table. Chapter 7 keeps the private-interface status |
| Prompt nonpanicking observers | Chapter 8's existing “Finish the managed job”, not duplicated here |
| Actual foreground gates lending and confined launches detach | Terminal opening/ownership paragraphs and unchanged part A mode table |
| Ended reports versus failed supervision's missing reap/group proof | Part A pre-reap error/report paragraphs, part B guarantees |
| Consumed cancellation retained through error; later observation belongs to caller | Part A cancellation, part B final cleanup, chapter 8 priority |
| Failed exec lease/pipe, original errno, checked recovery and authority/ID premises | Terminal sequence table, outcome table and failed-spawn paragraphs |
| CLI first native consumer, checked capture/waitability, undelivered continuing-host contract | Entry-state section and final guarantee/support paragraphs |
| `components.md`, O8 and R7 terminal recovery | Terminal section plus part A cleanup order |
| Cancellation and partial failure: C4 checked recovery, errors and retained cancellation | Terminal error paragraph, entry-state final-condition table, chapter 8 precedence |
| Accepted limits: abrupt death, same-user code and group escape | Guarantees table |
| Native-entry contract: child state distinct from returning host, saved INT/TERM/HUP only, no concurrency | Entry-state boundary table and final paragraph, guarantees table |
| Old statements that native entry remains entirely unmeasured | Guarantees support paragraph, scoped to existing aarch64 measurements and unqualified native x86-64 |

## Review

The single fresh reviewer checked the terminal section, outcome table and
terminal guarantee against production, the normative contract and the PTY
assertions. It returned no concrete finding in that scope. Thus there was no
finding to classify as unclear contract, actionable defect, visible trade-off
or noise, and no second reviewer ran. The reviewer checked all six of its
evidence paths at generation `2026-10-18T09:56:50Z` with no recorded gap.
Its read is source/assertion evidence, not an independent runtime result.
The added session-leader definition only explains the table's existing term.

## Validation and handoff

- `task test:runner -- -E 'binary(job) | test(native_s04) |
  test(native_unknown_status) | test(native_f06) | test(native_e05)'`:
  19 passed, 95 skipped by filter/fixture declaration, no failures.
- `task test:terminal-recovery`: six runner cases passed, then the one
  CLI conversion case passed against their real PTY receipts. The first
  filter skipped 108 cases and the second 223. This retains the distinction
  between physical recovery operations and private substituted reports.
- `task test:session -- --bin ravel -E 'test(cancellation::) |
  test(signal_state::)'`: eight passed, 216 skipped, no failures.
- The three runs used unchanged subjects: SHA-256 checks match all 148
  nonignored files enumerated under `crates`, `scripts` and `.cargo`, plus
  `Cargo.toml`, `Cargo.lock` and `Taskfile.yml`, before and after. Logs and
  the bounded input manifest are local scratch under `/tmp/ravel-k174/`.
  These are aarch64 macOS checks, not a Linux run, whole native campaign,
  interactive CLI example or universal security proof.
- Additional exact-path coverage for the native test sources, current support
  evidence, chapter 8 and paired terminal-check script reports matching
  metadata and no recorded gap, at the same generation, recorded complete at
  `10:02:19Z`. No reported range required fallback.
- `task docs:book:build`: passed with no warning or error.
- `task docs:ste -- --strict`: zero problems in 24 files.
- `task docs:breaks -- --strict`: zero long prose runs in nine chapters.
- `task docs:examples`: all 14 complete examples passed, zero failures or
  skips. Six labelled fragments elsewhere were not run. Part B adds no code
  example or fragment. After a final sentence clarification, the build and
  both strict checks passed again, and the chapter 9 example passed again
  through `task docs:examples -- docs/book/src/09-jobs-signals-and-the-terminal.md`.
- Both document-check runs froze 211 named inputs with SHA-256 before and
  after, with every digest unchanged within its run: nonignored files under
  `crates`, `scripts`, `.cargo` and `docs/book`, plus `Cargo.toml`, `Cargo.lock`,
  `Taskfile.yml`, `README.md` and `CONTEXT.md`. The second freeze follows the
  sentence edit. The manifests and logs are in `/tmp/ravel-k174/`.
- All 15 local chapter link occurrences resolve to files and rendered heading
  IDs. Missing-file and missing-anchor controls failed as intended. This is
  a bounded chapter check, not a whole-book link audit. The three edited files
  have no trailing whitespace.

The required structure, O6/O7/O8 teaching, lifecycle inventory and guide
destinations are complete for this draft. The local raw-file read and mechanical
checks establish no new reader result, browser/editor inspection, Linux
measurement, native x86-64 qualification or real-harness behaviour.

The existing copy-edit leaf k175 follows this draft, so the conditional last
act cuts nothing. The node brief carries both drafts' art handoffs and the
part B distinctions for copy-edit. The document node remains open for
copy-edit, art and proof. No parent closes, no guide is removed, and no ADR
decision changes.
