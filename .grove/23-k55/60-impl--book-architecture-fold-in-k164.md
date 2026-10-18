# book-architecture-fold-in-k164

## Goal

Remove the three architecture guides that chapters 7, 8 and 9 replace,
`docs/components.md`, `docs/local-runner.md` and `docs/session-channel.md`,
together with the editable Mermaid views under `docs/design/components/` that
the contract says become figure sources. Redirect every live inbound link, and
keep what is evidence outside the book. This follows the second half's set
proof (`book-second-half-k163`), so that “absorbed” is a statement about proved
chapters.

## Context

- Contract: `docs/book/contract.md` ("Relation to other documents"): the book
  replaces the component guide, the local runner guide and the result-channel
  guide, each removed once the chapters that absorb it land, with every inbound
  link redirected. The evidence documents, the caller-reference inventory and
  the retired-project record stay outside the book.
- The first half did the same for two guides in `book-guide-fold-in-k68`. Read
  its log for the method (the sentence-level classification, the table in the
  log, the search for inbound links with controls) and follow it.
- **Input: the units' logs.** Chapters 7, 8 and 9 each record the destination
  of the guide sentences they replace, and the set proof records which
  sentences no unit placed. Use them, then classify every sentence group of the
  three guides yourself from the proved chapters, not from the lists alone.
  Each group lands in one of: a chapter (name the section), an evidence
  document that stays outside the book, or deletion with the reason (stale,
  duplicate, history). A sentence with no home is not dropped: send it to the
  chapter that owns its subject by a correction run (`leaf-add` the owning
  stage and `proof` under that unit's node, or a `draft`, `copy-edit` and
  `proof` run, in pipeline order), remove nothing, and leave this leaf live so
  that it runs again after that run. A second send-back of one stage against
  one document goes to the owner.
- **Evidence in the guides.** `docs/components.md` ends with “Source and
  observation evidence” (E1–E7, the observed scenarios and their limits), and
  `docs/local-runner.md` ends with “Source provenance” (the source project's
  revision and URLs). Both are evidence, not teaching. Keep them in an evidence
  document outside the book (a new file such as `docs/component-evidence.md`, or
  an existing evidence document that fits), with a title and an opening that say
  what it is. `docs/caller-references.md` has a row that names
  `docs/local-runner.md` as historical attribution; update that row to the new
  home, and re-run the inventory's instrument with its controls as the file
  says.
- **Inbound references.** Enumerate them with a search that you first see find
  known hits (`README.md`, `docs/specs/session-execution.md`,
  `docs/adr/the-launched-child-is-a-job.md`, `docs/adr/ravel-supervises-the-session.md`,
  `docs/design/result-job/README.md`, `Taskfile.yml`). Classify each:
  - **live prose or link**: redirect to the chapter and anchor that teaches the
    subject, or to the evidence document (the specification and the ADRs are
    normative: change a link target only, never a statement; `ADR-FORMAT.md`
    says how to chase citations);
  - **frozen receipt** (the `subjects-*.json`, `source-paths.txt` and
    `*-checks.txt` files under `docs/` and `docs/evidence/`): these record
    digests and paths of files as they were. Do not edit them. State in the log
    that a frozen receipt names the removed files and why that is correct. Check
    that no script, test or task reads those names as live input (the earlier
    search found none in `scripts/`, `crates/` and `Taskfile.yml`; confirm it).
- **The Mermaid views.** The Taskfile task `docs:components` serves
  `docs/design/components/`. Once the chapters hold their own figures, the views
  have no reader. Remove the directory and the task, and make sure that
  `task --list` and the README no longer mention them. The sibling
  `docs/design/result-job/` views and the `docs:result-job` task are evidence
  views and stay.
- The README's reference paragraph for the unwritten Parts III and IV (it names
  `docs/components.md`, `docs/local-runner.md`, the specification and the runtime
  evidence) is the next leaf's, together with the rest of the README. Redirect
  only the links that this leaf breaks, and leave a working README.
- **Chapter 10.** Content that only a supported extracted component will use
  is not placed in a chapter. If a guide holds such content, keep it in the
  evidence document with a pointer in the log, and do not write chapter 10.

## Done when

- The three guides and `docs/design/components/` are removed, the
  `docs:components` task is removed, and no live document or task points at
  them. The log holds the classification of every sentence group, the list of
  inbound references with their classes, and the controls that were seen to
  fail (a planted dead link to a removed file is found by the instrument).
- Every redirected link resolves, and a control with a broken anchor fails.
  `task docs:book:build`, the strict STE and prose-break checks on the files
  changed, `task docs:figures`, `task docs:examples`, `task docs:excerpts` and
  `task check` pass on subjects whose digests are the same before and after.
- The evidence document (if any) is in STE, states what it is for, and is
  linked from chapter 11's guide to the evidence documents.
- The log says which statements of the three guides survive, where, and which
  were deleted with the reason.

## Notes

- Removal is the point of this leaf. Do not rewrite chapters 7, 8 or 9 here,
  except for a link that the move makes wrong.
- Do not run any task that publishes or tags.

## Decisions (running log)

- Evidence placement: a new `docs/component-evidence.md`, "Component source and
  observation evidence". It holds `components.md`'s E1–E7 and its observed
  scenarios, the acceptance-seam column of the O1–O10 table, the three boundary
  questions the read left open, and `local-runner.md`'s source provenance. It is
  re-worded to the book's STE rules; every digest, commit, count, filter and line
  range is kept verbatim. Strict STE, prose-break and the skill's linter pass.
  Chapter 10 content: none of the guides holds content that only a supported
  extracted component would use (the runner is private by the guides' own words),
  so nothing is held back for chapter 10.
- **This run does not remove the guides.** The reviewer found a guide sentence
  with no chapter home (`session-channel.md` 64–67, finding 2 below). The brief
  says: remove nothing, send it back by a correction run, leave this leaf live.
  The run began by removing the three guides, the views and the task; it then
  restored them (`jj restore --from @-`) together with `Taskfile.yml`,
  `contract.md`, `native-entry-development-evidence.md` and the removal edits in
  chapter 11. What stays from this run is only what remains correct while the
  guides exist: the evidence document, the redirected links and the caller-
  references row.
- Correction run cut under `develop-ravel-k159`: `develop-ravel-k192` (draft),
  `k193` (copy-edit), `k194` (proof). Art is omitted: one prose passage with no
  figure or table (reason also written in the draft body).
- Stale ADR sentence routed to a new root leaf, `adr-native-waitability-status-k191`.
- The k68 method was followed: the sentence-level classification, the table in
  the log, the link sweep with controls.

## State left by this run, and what the next run does

Done and kept (in the working tree of this commit):

- `docs/component-evidence.md` (final form, STE-clean).
- Redirected links: `README.md` (session channel → ch7
  `#the-result-channel-carries-the-value`); `docs/specs/session-execution.md`
  (→ ch7; → ch9 `#apply-filesystem-confinement`);
  `docs/specs/result-job-completion.md` (→ ch8 `#follow-cancellation`,
  `#choose-the-final-outcome`; → CE `#source-reference-and-discovery`; one link
  label reworded); `docs/adr/ravel-supervises-the-session.md` (→ ch7);
  `docs/adr/the-launched-child-is-a-job.md` (→ CE
  `#local-runner-source-provenance`); `docs/design/result-job/README.md` (→ CE);
  the `docs/caller-references.md` row (→ `docs/component-evidence.md`).

Still to do when this leaf runs again, after `develop-ravel-k194`:

1. Re-run the classification below against the chapters as they stand (ch11 will
   have the worker-prerequisite statement; the row for `session-channel.md` 64–67
   then becomes A with the anchor).
2. Remove `docs/components.md`, `docs/local-runner.md`, `docs/session-channel.md`,
   `docs/design/components/`, and the `docs:components` task in `Taskfile.yml`.
3. Edit chapter 11 only where the removal makes it wrong: the paragraph that
   names `task docs:components` (near "Read the evidence for a change") and the
   `docs:components` entry in the inventory table. The row in the "Read the
   evidence for a change" table that links `docs/component-evidence.md` is already
   in place from this run. Keep chapter 11's strict STE green (it was, with the
   row). The public task count falls from 57 to 56.
4. Edit `docs/book/contract.md`: the "Relation to other documents" bullet (say
   that the three guides and the views are gone and that their evidence is in
   `docs/component-evidence.md`), and line 314 ("the component guide tabulates
   today" → "tabulated before its removal").
5. `docs/native-entry-development-evidence.md` line 1066: change the link
   `[local runner](local-runner.md)` to plain text that says the guide was folded
   into the book's chapter 9, and add "(since folded into the book's chapters 7 to
   9)" after "the component guide's job row" (the k68 method).
6. Frozen receipts stay as they are (32 files, below). Re-confirm that no script
   reads them.
7. Re-run the link sweep (instrument saved in the session scratchpad; rebuild it
   from the "Verification" description below), the enumerate-then-classify of
   remaining mentions, and the caller-references instrument with controls
   (below), then the gates in "Done when".

## Verification so far

- **Link sweep (enumerated, not pattern-listed).** Every Markdown link, image and
  reference definition in the tracked tree except `docs/evidence/`, the checks
  logs, `retired/` and `.grove/` (code fences and spans stripped), file and
  heading anchor resolved (GitHub and mdBook slug). Final tree of this run: 53
  Markdown files, 1,061 links: 286 resolve to a file, 597 to a file and anchor, 178
  external (not resolved), 0 broken. **Controls seen to fail:** a scratch file
  that linked `components.md`, `session-channel.md#x` and a misspelt anchor of the
  evidence document was reported as 3 broken, exit 1, while its good anchors were
  counted as resolved; the scratch file was removed and the clean reading
  returned. **Cross-tree control:** the pattern for the removed names still finds
  them in 32 receipt files and in the guides themselves.
- **Enumerate-then-classify of the remaining mentions** of
  `components.md|local-runner|session-channel|design/components|docs:components`
  and of the words "component guide", "runner guide", "result-channel guide",
  "component views" gave: `Taskfile.yml` line 253 ("local-runner checks": the
  `test:runner` description, not the guide), `docs/book/figures.md` line 325
  (the C4 figure kind), CE (the history sentence), and the files in the table
  below. Nothing else.
- **Caller-reference inventory instrument, re-run.** Expression
  `grove|harness[-_]dispatch`, case-insensitive, with ripgrep `--hidden
  --no-ignore -n -i -e` over the enumerated active files (`-e`, because `-E` is
  `--encoding` in ripgrep; run with `-E` it errors "unknown encoding", which is
  the instrument-break control). Positive controls: found `.cargo/config.toml`
  (6 lines) and the source-project URLs (3 lines). The mutated expression
  `grovv|harness[-_]dispatcz` found 0 of them, so both assertions fail as they
  must. A clean fixture file gives exit 1 and the same file with `GROVE_X` gives
  exit 0. Cross-tree control: the real expression finds Grove references in the
  source project's checkout (`/Users/antony/Development/grove/crates`) and the
  mutated one finds none. Result: the only change in the match set against the
  parent revision is that the two source URLs moved from `docs/local-runner.md`
  to `docs/component-evidence.md`, which the inventory row now names, and that
  the one "Grove continuation" sentence of `components.md` leaves with the guide.
- Strict `task docs:ste` and `task docs:breaks` pass on `docs/component-evidence.md`.
- Not run in this run: `task docs:book:build`, `task check` and the other gates.
  This run changes no book text and no code, and the leaf is not done. The next
  run takes the gates on frozen digests.

## Classification of the three guides

Method: the units' destination maps (k152, k155/k170, k158/k174, proofs
k169/k173/k177, set proof k163) were the input. Each group below was then read
again in the guide as it stood at the parent revision and searched for in the
proved chapter text, not taken from a list. Short forms: ch7 = `07-the-parts-of-ravel.md`,
ch8 = `08-follow-a-session-through-the-code.md`, ch9 =
`09-jobs-signals-and-the-terminal.md`, ch11 = `11-develop-ravel.md`; CE =
`docs/component-evidence.md` (new). Dispositions: **A** a chapter teaches it,
**E** kept in CE, **D** duplicates a document or source comment that survives,
**X** deleted with the reason given. Anchors are heading slugs in the built book.

### `docs/components.md` (421 lines)

| Lines | Group | Disp. | Home or reason |
|---|---|---|---|
| 1–9 | Three acts (evaluate, launch, return one result); glossary terms | A | ch7 opening; ch2 `#caller-and-owner` |
| 11–20 | Explains the current implementation; completion invariant; spec owns promises; chapters and guides own interfaces | A, D, X | ch7 last section (limits); spec survives. The cross-references to the removed guides are X: they point at removed files |
| 22–34 | Six responsibilities table and "not six packages" | A | ch7 part table, `#the-ravel-cli-coordinates-the-session`, `#the-worker-executes-typescript-before-the-launch`, `#the-runner-owns-the-local-job-mechanics`, `#the-result-channel-carries-the-value`, `#the-installed-pair-stays-together`; input checks in ch8 `#prepare-the-request`. The `[E1]`–`[E6]` links → CE `#source-anchors` |
| 36–55 | Two Rust packages, worker as build unit, SDK embedded, executable and library targets, no daemon | A, E | ch7 opening table and `#the-ravel-cli-coordinates-the-session`. `publish = false` → CE `#source-anchors` (after the table) |
| 57–74 | Why TypeScript; Bun; delivery cost; compatibility promise | A | ch7 `#the-worker-executes-typescript-before-the-launch`, `#the-sdk-describes-the-policy-interface`. The sentence "a Rust-only implementation is an open question" is X: it is the analysis's framing of an unselected alternative; ch7 claims no replacement runtime and says a replacement must keep the resolver rules; the ADR `policy-evaluation-precedes-the-launch` owns the accepted limits |
| 76–90 | Why two Rust packages; runner hides OS mechanics; `libc`; consumer obligations; not a verdict on future packaging | A | ch7 `#the-runner-owns-the-local-job-mechanics` (including the closing paragraph on private packages); ch9 `#the-managed-process-leads-a-job` |
| 92–98 | Scoped obligations; "Source" and "observed" labels; seam is not a claim | E | CE `#acceptance-seams-for-the-ten-obligations` (intro paragraph) |
| 100–111 | O1–O10 table | A, E | Required outcomes: set proof k163 inventory 4 (every O has a chapter section). The "Evidence and existing acceptance seam" column → CE, table "Acceptance seams for the ten obligations" |
| 113–117 | Bounds and exit codes live in the spec and the reference | D | spec and ch6 `#bounds`, `#exit-statuses` survive |
| 119–148 | Composition, relations R1–R8, contract owner, co-change, shared state | A, X | R1–R8: ch7 sections and tables (k152 relation map), ch8 for the details. The O/R labels are the guide's own index and no surviving document uses them (checked below), so they are X. "Contract owner is the human" and "historical co-change was not sampled" are X: analysis method notes; the book makes no co-change claim. "No durable store; shared state is the ephemeral channel and OS state": ch1 `#decide-whether-ravel-fits` and ch7 `#the-result-channel-carries-the-value`. C1/C2 evidence links → CE "Related evidence documents" |
| 150–171 | A successful operation | A, E | ch8 `#trace-the-example` to `#complete-cleanup-and-write-the-result`. The rerun observations (five result forms, prompt, stderr, empty directory, no forced grace) → CE "What the cases checked" |
| 173–188 | Cancellation phases; publish-then-TERM fixture; join of latches | A, E | ch8 `#follow-cancellation`, `#choose-the-final-outcome`; ch9 `#handle-cancellation`. The fixture observations → CE "What the cases checked" |
| 190–211 | C1–C4 repair history; "native entry remains conformance work"; selected-scope cleanup | A, X | Consumed signal survives an error: ch8 `#follow-cancellation` (table). Selected-scope cleanup on failure: ch8 `#follow-a-refusal-or-launch-failure`. The repair history and the "conformance work" status are X: stale and historical; ch9 `#preserve-the-entry-signal-state` and `#know-the-guarantees-and-their-limits` give the current qualified statement; the evidence documents (linked from CE) keep the history |
| 213–219 | What each participant knows | A | ch7 `#the-result-channel-carries-the-value` (table); ch8 `#read-the-result` |
| 221–251 | Error and signal precedence; combined failures; accepted limits | A, E, X | ch8 `#choose-the-final-outcome`; limits in ch5 "Understand the operational limits" and ch9 `#know-the-guarantees-and-their-limits`. The two unit-case names and the owed noncancelled check → CE "Combined-cancellation unit cases"; the replay requirement → ch8 `#find-where-a-change-belongs`. "This manual replay is not a proof of all interleavings" is X: a method note |
| 253–257 | Native-entry contract; no host restoration | A | ch9 `#preserve-the-entry-signal-state`, `#know-the-guarantees-and-their-limits` |
| 259–277 | Independent-change scenario; type does not pair `Ended` with channel; test fixture composes the pair (E7) | A, E | ch8 `#find-where-a-change-belongs`; ch7 `#the-result-channel-carries-the-value`. E7 → CE `#source-anchors` |
| 279–287 | Five retained costs | A | TypeScript: ch7 worker section; generic job reuse: ch7 runner section; result-aware reuse and state: ch7 channel section; worker versus job lifecycle: ch7 worker and runner sections |
| 289–300 | First bounded comparison; baseline; priority; unassessed use | X, A | The prioritisation is the plan of the analysis. The design workstream ran that comparison and `docs/specs/result-job-completion.md` states its outcome, so the sentence is history. The scope limits (use is unassessed) are in ch7's last paragraph |
| 302–421 | Source and observation evidence (E1–E7, graph notes, observed scenarios, digests, limits) | E | CE `#source-reference-and-discovery`, `#source-anchors`, `#observed-scenarios-and-limits` (re-worded to the book's STE rules; digests, commit, filter and counts kept verbatim) |

### `docs/local-runner.md` (131 lines)

| Lines | Group | Disp. | Home or reason |
|---|---|---|---|
| 3–9 | Private runner; no dependency on another project; not published; composed by Ravel; links to the transport and component guides | A, E, X | ch7 opening table and runner section (`libc` only); "not published" → CE `#source-anchors` note; the two guide links are X |
| 11–16 | `Argv`, `Launch` fields; no shell | A | ch7 `#the-runner-owns-the-local-job-mechanics` |
| 18–25 | Interface table | A | ch7 runner table; ch9 for `signal`, `reraise`, exact signal behaviour |
| 27–34 | Escalation order; reap; terminal restore; group result; `operation_reports`; EPERM; what lies above the boundary | A | ch9 `#follow-the-escalation-after-a-result`, `#stop-the-group-before-the-reap`; ch7 `Ended` row and the boundary paragraph |
| 36–38 | Pointer to `src/lib.rs` and `src/run.rs` | D | source |
| 40–58 | Persistent handlers; ignored SIGCHLD; entry signals; consumer duties; observers; foreground gates lending; confinement detaches | A | ch9 `#preserve-the-entry-signal-state`, `#lend-and-recover-the-terminal`, `#separate-noninteractive-execution-from-confinement`; observers ch8 `#finish-the-managed-job` |
| 60–69 | `Ended` versus failed supervision; `operationReports`; `consumed_signal()`; `raw_os_error()`; Display and Debug text; final wait drains the latch | A, D | `operationReports` ch6 `#json-routing-notice`, ch8 `#complete-cleanup-and-write-the-result`; consumed signal ch8 `#follow-cancellation`; errno classification ch8 `#follow-a-refusal-or-launch-failure`. The accessors' contracts are the doc comments in `crates/keyed-launch/src/error.rs` (D). "Final wait drains the latch; a later signal is the caller's late observation": ch8 `#choose-the-final-outcome` (the `late` row) and ch9 boundary table; the draining step itself is in `run.rs` `supervise` only |
| 70–80 | Failed exec lease; attempted-group pipe; handback premises; pointers; "not runtime-tested then" | A, X | ch9 `#lend-and-recover-the-terminal` (failed-spawn paragraphs, outcome table). The pointers and the "not runtime-tested in that observation" are X: history |
| 82–93 | First native consumer; repair contract; C1–C4; "conformance work" | A, X | ch9 entry-state and guarantee sections. The status sentences are X: stale |
| 95–106 | Transferred suite; ignored fixture roles; `wait_events.rs`; `test:terminal-recovery`; `task check` | A | ch11 `#choose-a-test-seam`, `#run-the-checks-before-handoff`; ch9 test table (`internal/wait_events.rs`) |
| 108–113 | Cargo guard; helper scrub list | A, D | ch11 task and guard table (`.cargo/config.toml`, `support/mod.rs`); `docs/authority.md` survives |
| 115–131 | Source provenance | E | CE `#local-runner-source-provenance` |

### `docs/session-channel.md` (67 lines)

| Lines | Group | Disp. | Home or reason |
|---|---|---|---|
| 3–7 | Transport in the private `ravel` package; spec is the boundary; link to the component guide | A, X | ch7 `#the-result-channel-carries-the-value`; the guide link is X |
| 9–13 | `allocate(parent)`; choosing the parent for a sandbox; no result file; `RAVEL_EXIT_FILE` | A | ch8 `#prepare-the-managed-launch`; parent choice and write access: ch5 (`--exit-dir` note), ch6 `--exit-dir`, ch9 confinement table |
| 15–18 | `Launch.channel`; scrub; nested launch replaces the path in the child only; worker never sees it | A | ch8 `#prepare-the-managed-launch`; ch7 worker section; ch5 `#keep-nested-sessions-separate` |
| 20–23 | `end_session`; no session id; `publish_result` is private | A | ch8 `#send-the-result`; ch6 `#end-session-result` |
| 25–31 | Temporary file and no-clobber link; first wins; empty, Unicode, newline; refusal; 1 MiB above OS limits; read bound | A | ch8 `#send-the-result`, `#read-the-result`; ch5 and ch6 for the bound and OS argument limit |
| 33–40 | Runner sees appearance only; `result_after`; `None` versus empty; the consumer supplies the matching `Ended`; front owns status and streams | A | ch7 `#the-result-channel-carries-the-value`; ch8 `#finish-the-managed-job`, `#read-the-result` |
| 42–48 | Held-descriptor read; symlink, FIFO, non-regular; `close()`; Drop; identity check; allocation failure settles with cause and cleanup context; explicit attempt disarms Drop; unknown identity | A | ch8 `#read-the-result`, `#complete-cleanup-and-write-the-result`, `#follow-a-refusal-or-launch-failure`. The clause that the returned allocation error keeps the original cause plus the cleanup text is in `completion.rs` (`allocate`, lines 74–80) and ch6's `message` row ("original and secondary operational details") |
| 49–56 | C3 checks; snapshot comparison; moved directory needs manual cleanup; same-user limit; killed process leaves files; no store | A, E | ch8 `#complete-cleanup-and-write-the-result`; ch1 and ch5 abrupt-death rows; C3 link → CE "Related evidence documents" |
| 58–62 | `task test:session`, `--lib`, fixture scrub, ignored roles | A | ch11 `#choose-a-test-seam` |
| 64–67 | `task check`; transport tests need no worker; package suite needs the built worker; no host Bun, model or installed launcher | **not placed** (partly) | ch11 `#run-the-checks-before-handoff` and the seam table say `cli.rs` uses real workers and that a worker change needs `task build`; the opening says no model account. No chapter says which seams need the built worker and which do not. Sent back: correction run `develop-ravel-k192`/`k193`/`k194` |

### Fresh reviewer's findings (the leaf's one allowance)

The reviewer got the three guides at the parent revision, the chapters and the
evidence document, and no classification. It reported five gaps and one
contradiction. Each one, classified:

| # | Finding | Class | Handling |
|---|---|---|---|
| 1 | Runner suite coverage list (`local-runner.md` 97–102) names no test file for channel isolation, argv and environment, signal forwarding | visible trade-off | The module comments and test names under `crates/keyed-launch/tests/` hold that list (D), and `task test:list` enumerates it. ch9 and ch11 name the seams. No chapter lists every test file, by design |
| 2 | Which tests need the built worker (`session-channel.md` 64–67) | valid, actionable | Sent back: `develop-ravel-k192` (draft), `k193` (copy-edit), `k194` (proof). Art is omitted: one prose passage, no figure or table |
| 3 | Boundary questions of `components.md` 279–287 (worker versus job lifecycle; compare before extraction; generic reuse needs an example) | valid, but analysis state, not teaching | Kept in CE, new section "Component questions that the read left open". The extraction decisions belong to the open component-work scope leaf (`component-work-scope-k57`) and to chapter 10 |
| 4 | `signal` and `reraise` missing from ch7's runner table | noise (low) | Both have doc comments in `crates/keyed-launch/src/channel.rs` and `run.rs` (D). Chapter 7 is a map, not an API list |
| 5 | Runner independence and the "above the runner" boundary | noise (low) | ch7 `#the-runner-owns-the-local-job-mechanics` (no policy, no result read, `libc` only) and the crate documentation in `lib.rs` lines 4–8 |
| 6 | ADR `policy-evaluation-precedes-the-launch` says native waitability "remains separate conformance work"; chapter 9 says measured support | valid, a stale normative sentence that already stood in the ADR | Not a statement this leaf may change (ADRs: link targets only). New leaf `adr-native-waitability-status-k191` owns it |
| 7 | CE row O8 said the CLI recovers the terminal | valid wording point | Fixed in CE: the runner, inside the CLI, recovers it |

The reviewer also checked every number, digest, line range, count and filter
name in CE against the guides and found no difference.

## Inbound references

Enumerated from `jj file list` with the receipts excluded from the link sweep and
classified, not pattern-listed. Positive control: the search found the known
hits listed in the brief (README, the specification, both ADRs, the result-job
README, `Taskfile.yml`). The classes:

| Reference | Class | Action |
|---|---|---|
| `README.md` line 86 (session channel) | live link | → ch7 `#the-result-channel-carries-the-value` |
| `docs/specs/session-execution.md` lines 8, 196 | live link (normative: target only) | → ch7; → ch9 `#apply-filesystem-confinement` |
| `docs/specs/result-job-completion.md` lines 19, 21, 428 | live link (normative: target only; one link label also named the removed guide) | → ch8 `#follow-cancellation`, `#choose-the-final-outcome`; → CE `#source-reference-and-discovery` |
| `docs/adr/ravel-supervises-the-session.md` line 24 | live link (normative: target only) | → ch7 `#the-result-channel-carries-the-value` |
| `docs/adr/the-launched-child-is-a-job.md` line 42 | live link (normative: target only) | → CE `#local-runner-source-provenance` |
| `docs/design/result-job/README.md` line 24 | live link | → CE (link text changed to "component evidence") |
| `docs/native-entry-development-evidence.md` line 1066 | past-tense evidence sentence | link removed; plain text now says where each guide went (same method as k68) |
| `docs/caller-references.md` line 19 | inventory row | → `docs/component-evidence.md`; instrument re-run (below) |
| `docs/book/contract.md` lines 314, 404–407 | live prose | "Relation to other documents" bullet edited in place to say the three guides and the views are gone and where the evidence is kept; line 314 ("the component guide tabulates today") now says "tabulated before its removal" |
| `docs/book/src/11-develop-ravel.md` lines 632, 657 | live prose | `task docs:components` removed; one row added to "Read the evidence for a change" for CE |
| `Taskfile.yml` task `docs:components` | task | removed |
| `Taskfile.yml` line 253 ("local-runner checks") | wording only | unchanged: it names the `test:runner` target, not the guide |
| `docs/book/figures.md` line 325 ("component views") | unrelated | unchanged: it describes the C4 figure kind |
| Receipts: 32 files, 30 under `docs/evidence/` (`subjects-*.json`, `*-source-*.json`, `source-paths.txt`, `inventory.txt`, `tasks.txt`) and two `*-checks.txt` logs in `docs/` | **frozen receipt** | not edited |

Status of the Action column: done in this run are the README, both specifications,
both ADRs, the result-job README and the caller-references row. The contract, chapter 11,
`Taskfile.yml` and `native-entry-development-evidence.md` rows are the next run's
(steps 2 to 5 above), because they presuppose the removal.

Why the frozen receipts stay as they are: they record the digests and paths of
the files as they were when a run took them, so the names are the evidence. A
rewrite would destroy the digest-to-path record. No script, test or task reads
them as input: the scripts only write `subjects-before/after.json` files into a
fresh output directory (`native-campaign.sh`, `native-r01.sh`), and the subject
list of `native-campaign.sh` names `docs/native-entry-transition-design.md` and
`docs/specs/result-job-completion.md`, not the removed files.
`scripts/`, `crates/`, `.cargo/` and `Taskfile.yml` hold no other mention.

No surviving document uses the guide's `O1`–`O10` or `R1`–`R8` labels as a
reference (the book states the obligations in words, and the specification has
its own rules).
