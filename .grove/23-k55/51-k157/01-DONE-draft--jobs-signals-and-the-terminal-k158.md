# jobs-signals-and-the-terminal-k158

## Goal

Draft **part A** of chapter 9, as the document node's brief describes, under
`docs/book/contract.md`, into
`docs/book/src/09-jobs-signals-and-the-terminal.md`, linked from
`docs/book/src/SUMMARY.md`: the managed process as a job, the escalation after
Ravel receives the result, group cleanup before the reap, noninteractive
isolation and confinement. The headings for part B are not written here.

## Context

- Chapters 1–3, 5 and 7 are proved before this runs. Chapter 3 introduced the
  process group and reaping only as far as a first session needs, and it points
  here for the rest; keep its words for the same concepts and link back.
- Run the escalation yourself on a scratch copy before you write the times: a
  fake agent that sends a result and then ignores TERM shows the grace and the
  kill grace on a real run. The runner's tests under
  `crates/keyed-launch/tests/` (`job.rs`, `interrupt.rs`, `confinement.rs`,
  `noninteractive.rs`) show what the project already observes. State in the
  chapter only what the code or a test supports.
- The macOS stop-report behaviour that chapter 3 describes (accepted `EPERM`
  stop reports, retained after confirmed reap) belongs here too. Keep the
  explanation consistent with chapter 3 and with `supervise_reported` in
  `crates/keyed-launch/src/run.rs`.
- Settle the heading structure for the whole of chapter 9 now, so that part B
  fills sections and does not reshape the file. Write down in this leaf's log
  which headings part B owns.
- Confinement on Linux needs bubblewrap and on macOS `sandbox-exec`. Say what
  the checkout's tests can and cannot show on the host you use, and what no
  test shows.

## Done when

- Part A's sections exist, build and pass `task docs:book:build`, the strict
  STE check, `task docs:breaks -- --strict` and `task docs:examples`. The
  figure needs (the participants, states and bounds for the timing figure) are
  in the log for the art stage. Do not draw them here.
- The log lists the obligations that part A teaches (O6 and the process side of
  O7) and what remains for part B (O8).
- Last act: `leaf-add` a `draft` leaf (bare stem
  `jobs-signals-and-the-terminal`) whose body states part B's scope from the
  node brief, the headings reserved for it and anything part A learned that
  changes it; then `leaf-add` the `copy-edit` leaf if no live later sibling
  already holds it.

## Notes

- One fresh reviewer is allowed under the leaf-wide allowance. Spend it on the
  claim about the order of group cleanup and reap, because a wrong order is the
  kind of mistake the compiler cannot find.

## Decisions (running log)

- The named structure brief is `docs/book/contract.md`, section 1. Its reader
  is “a 16-year-old software engineer with no background in Ravel”. Its chapter
  9 outcome is to “reason about process groups, escalation, reaping, terminal
  lending and recovery, confinement, entry signal state, and the failure
  guarantees with their limits”. The ordered chapter plan and the node's
  ordered A/B scope supply the section purposes. The contract emphasises
  “What success means” and “The stated limits, said plainly”, and excludes
  evidence campaigns and undelivered contracts. All three draft prerequisites
  are present.
- Whole-chapter heading order: “The managed process leads a job”, “Follow the
  escalation after a result”, “Stop the group before the reap”, “Handle
  cancellation”, “Separate noninteractive execution from confinement”, “Apply
  filesystem confinement”, then part B's “Lend and recover the terminal”,
  “Preserve the entry signal state” and “Know the guarantees and their limits”.
  The escalation section includes “Run a fake that ignores TERM”. Part B owns
  the final three headings and adds no empty placeholders in part A.
- Noninteractive execution needs an explicit distinction: `run_noninteractive`
  is a runner API with no completion channel and a regular output file.
  An ordinary unconfined `start-session` without a terminal does not call that
  API or acquire its detached-session and descriptor-isolation rules.
- Technical truth rests on current source, at Tier 2. The graph was ready at
  bootstrap. It returned 27 targeted runner symbols, 32 targeted test symbols
  and 32 confinement symbols, with no remaining pages. `supervise_reported`
  has three inbound and eleven outbound rows in its one-hop trace. Some call
  edges resolve ambiguous names to unrelated methods, so direct source owns
  the order and conditions. The status tool supplies no generation identifier.
  Exact-path coverage and source verification precede the narrow review.
- Coverage supplies generation `2026-10-18T09:42:01Z`, recorded complete at
  `09:45:52Z`. All 28 requested source, test, contract and guide paths have
  `no_recorded_issue` and `metadata_match`. This is a best-effort check, not
  proof of completeness. No missed range required fallback. Direct reads
  verified the relevant production and test paths in any case.
- The scratch example passed through `task docs:examples` before the times
  were written. It sent `finished`, ignored TERM and waited for KILL. The
  9.75-second command measurement includes the build and example runner, so
  the chapter states no measured shutdown duration. Source supplies the CLI's
  2/5-second graces and 500/10-millisecond polls. The example claims exact
  result bytes and status only.
- The selected runner command passed 54 tests on Darwin arm64, with 60 tests
  excluded or marked as subprocess fixtures. It selected launch, job,
  interrupt, confinement, noninteractive and private wait-event cases. The
  command and full output are retained in the local scratch log. It does not
  exercise the Linux backend or establish new platform qualification.
- “Run a fake that ignores TERM” is a separate second-level heading, directly
  after “Follow the escalation after a result”. This avoids an unpaired
  subsection. The other headings and part B's three reserved headings retain
  their agreed order.

## Source and destination record

The subject is the current checkout. No production code or architectural
choice changed. The section order moves from the unit of control, through
normal completion and cancellation, to the distinct launch restrictions.
This sequence supplies the job half of the contract's reader outcome.
The final three reserved sections supply the terminal and signal half.

### Component obligations (inventory 4)

Re-enumerated from `docs/specs/session-execution.md` and checked against the
O1–O10 table in `docs/components.md`, not inferred from chapter coverage:

| ID | Required behaviour | Teaching destination |
|---|---|---|
| O1 | Preserve owner policy, SDK, settings and discovery | Chapters 4 and 6, chapter 7 “The worker executes TypeScript before the launch” and “The SDK describes the policy interface” |
| O2 | Keep result authority out of policy execution, verify worker identity, and reap the worker before the managed launch | Chapter 7 “The worker executes TypeScript before the launch”, chapter 8 “Evaluate and check the policy” |
| O3 | Judge decisions and failures without fallback, preserve caller prompt and parameters | Chapter 8 “Prepare the request”, “Evaluate and check the policy”, “Follow a refusal or launch failure” |
| O4 | Execute selected argv directly with caller context, requested confinement or refusal, and fresh nested result context | Chapter 8 “Prepare the managed launch”, this chapter “Separate noninteractive execution from confinement” and “Apply filesystem confinement” |
| O5 | Keep the first complete result exact, and reject invalid or stale result files | Chapter 8 “Send the result” and “Read the result”, chapter 5 “Accept a result only after success” |
| O6 | Require matching child reap and group absence before success, independently of later child status | This chapter “Follow the escalation after a result”, “Run a fake that ignores TERM” and “Stop the group before the reap” |
| O7 | Suppress results on INT/TERM/HUP and propagate cancellation after cleanup | This chapter “Handle cancellation” teaches the process actions, chapter 5 supplies both sequences, chapter 8 teaches final priority. Part B teaches entry state and the CLI's final signal restoration |
| O8 | Recover foreground and saved modes with group stops before reap | This chapter teaches the pre-reap order. Part B owns “Lend and recover the terminal” and its explicit premises |
| O9 | Reserve stdout for exact result bytes after cleanup, and report cleanup/write failures | Chapter 8 “Complete cleanup and write the result” and “Choose the final outcome” |
| O10 | Execute the matched installed CLI and worker without a host runtime | Chapter 7 “The installed pair stays together”, chapters 6 and 12 |

Part A delivers O6 and the process side of O7. It does not claim completion
of O8 or of the chapter as a whole. Reader comprehension remains unknown.

### Lifecycle and failure outcomes (inventory 3)

Re-derived from “Managed process and result” in the session specification and
“One completion decision”, “Required failure settlement” and “One-session
native launcher entry” in the completion specification:

| Outcome or mechanism | Destination |
|---|---|
| Exact sent result, empty/multiline/Unicode/option-looking values, later nonzero child status | Chapters 3, 5 and 8. Part A's fake adds the KILL death case |
| First result retained on repetition, subject to shutdown race | Chapters 3, 5, 6 and 8 |
| Natural exit without a result, even status 0 | Chapters 3, 5 and 8. Part A explains group stops on every child exit |
| Missing/empty/stale result context and invalid result file | Chapters 5, 6 and 8 |
| Malformed CLI/settings, policy refusal/file/data/exception, selection timeout | Chapters 4–6 and chapter 8 refusal path |
| Worker absent/stale/mismatched or failed | Chapters 6–8 |
| Selected executable absent or cannot execute | Chapters 6 and 8. Part B owns terminal recovery after failed exec |
| INT/TERM/HUP during selection/supervision, even after a sent result | Part A “Handle cancellation”, with chapter 5 sequences and chapter 8 priority |
| Grace, TERM, kill grace, KILL | Part A “Follow the escalation after a result” and executable fake |
| Group stops before reap, survivor and absence-query failure | Part A “Stop the group before the reap” |
| Cleanup error or unwritable result stream | Chapter 8 final cleanup and outcome sections. Part B states failure guarantees |
| Retained operation reports and macOS EPERM conditions | Part A “Stop the group before the reap”, consistent with chapter 3 |
| Nested sessions and replaced result context | Chapter 5 “Keep nested sessions separate”, chapter 8 launch preparation |
| Noninteractive isolation | Part A “Separate noninteractive execution from confinement” |
| Confinement granted, denied, missing or unusable backend | Part A “Apply filesystem confinement”, with chapter 5's complete example |
| Terminal lend/recovery and failed-exec recovery | Part B “Lend and recover the terminal” |
| Entry masks/dispositions and CLI/runner lifetime distinction | Part B “Preserve the entry signal state” |
| Same-user tampering, abrupt CLI death, group escape, detached policy children, concurrent embedding and native x86-64 limits | Part B “Know the guarantees and their limits”. Part A introduces group escape and limits of the host checks |

### Guide statements this chapter replaces

These destinations are inputs to `book-architecture-fold-in-k164` after proof.
They do not authorise guide removal now. A range names a sentence group whose
individual claims share a destination, not a claim that the whole guide moved.

| Guide statement group | Destination and ownership |
|---|---|
| `local-runner.md`, Boundary: direct argv, launch cwd/environment/channel/durations | Chapter 7's runner interface table and chapter 8 launch preparation. Part A explains durations and the group |
| Boundary: `run` returns status, completion observation, group result and retained stop reports | Part A “Stop the group before the reap”, chapter 8 result checks |
| Boundary: `run_observed` gives synchronous spawn/reap notifications | Chapter 8 “Finish the managed job”. Part B distinguishes spawn report from confirmed exec |
| Boundary: `run_noninteractive` isolates input/descriptors/terminal and writes a regular output file | Part A “Separate noninteractive execution from confinement” |
| Boundary: confined launch limits writable roots and system/explicit reads | Part A “Apply filesystem confinement” |
| Boundary: completion appearance starts grace/TERM/kill-grace/KILL | Part A “Follow the escalation after a result” |
| Boundary: reap, terminal restoration and group confirmation precede action on completion | Part A “Stop the group before the reap”, with terminal mechanics reserved to part B |
| Boundary: EPERM is retained only after three confirmations and does not prove absence | Part A “Stop the group before the reap” |
| Boundary: selection, meaning, storage and next work remain outside the runner | Chapter 7 “The program contract stays loose” |
| Native entry and return: persistent handler changes, child entry state, capture instant, host lifetime and concurrency | Part B “Preserve the entry signal state” and “Know the guarantees and their limits” |
| Native entry and return: actual foreground gates lending, confinement uses a detached session | Part A mode distinction, part B terminal mechanism |
| Native entry and return: successful Ended can carry stop reports, CLI JSON/non-JSON forms | Part A “Stop the group before the reap”, chapter 8 final output |
| Native entry and return: error can lack reap/group proof, consumed cancellation survives failure, late latch belongs to caller | Part A error paragraph and cancellation section, chapter 8 priority, part B entry-state guarantees |
| Native entry and return: failed exec lease/attempt record, checked handback, exclusive-authority/no-ID-reuse premises | Part B “Lend and recover the terminal” |
| Tests and authority: escalation, descendants, noninteractive and confinement tests | Part A final test table. Chapter 11 owns test commands and isolation guards |
| `components.md`, two-package rationale: runner hides OS job mechanics but leaves result and policy composition to Ravel | Part A mode comparison, chapter 7 package explanation |
| O4/O6/O7/O8 and R5/R7: direct launch, completion observation, group stops/reap/terminal/group confirmation and final CLI outcome | Part A job, escalation, pre-reap and cancellation sections. Part B owns terminal details |
| Successful operation: received result starts escalation, natural exit also stops group before reap, later child status does not invalidate value | Part A escalation/example/order, chapter 8 result and final output |
| Cancellation and partial failure: selection stops worker without a managed launch; supervision forwards cancellation, ends group, reaps and suppresses stdout | Part A cancellation table with chapter 5 sequences |
| Cancellation and partial failure: no result on cleanup failure, partial resource ownership and error composition | Chapter 8 final outcome. Part B states the quoteable limits |

### Figure handoff to art

The art stage owns the illustration, caption and alt text. No figure changed
in this draft. Place the scaffold's `escalation-timing` figure after the
escalation table, once art has checked it against the final chapter.

- Participants: managed process, runner within the Ravel CLI, completion
  channel, and the rest of the job's process group. Show the result sender
  separately if its lifetime helps explain the observation delay.
- States: child alive with no result, result observed/grace, TERM/kill grace,
  KILL/exit observation, group-stop attempts while identity is reserved, reap,
  terminal recovery when applicable, and group-absence confirmation.
- Bounds: CLI grace 2 seconds from observation, kill grace 5 seconds from TERM,
  normal poll 500 milliseconds, killed poll 10 milliseconds, post-KILL retry
  bound 2 seconds, two final group KILL attempts separated by 20 milliseconds,
  group confirmation up to 1 second after recovery. These are not an exact
  seven-second session wall time or a guarantee against kernel/scheduler stalls.
- Distinguish natural early child exit from the illustrated full escalation.
  Do not give descendants a new grace after their parent's exit. Show final
  group signals before reap and the post-reap query as observation only.
- Reuse/link chapter 5's selection and supervision cancellation sequences.
  Confined cancellation uses immediate KILL. Part B supplies the terminal
  sequence's participants, saved modes, failed-spawn branch and conditions.

## Review

The leaf's single fresh reviewer inspected the cleanup-order section and its
related timing/cancellation statements against the contract and exact source.
It returned no finding within that scope. It confirmed the order in `watch`
and `supervise_reported`, the reservation guard in `OwnedChild`, ESRCH-only
absence in `confirm_gone`, conditional EPERM retention, and the retry limits.
There were no findings to classify as unclear contract, actionable defect,
visible trade-off or noise. No second reviewer ran. This source review is not
an independent runtime measurement or a review of the unwritten part B.

The reviewer checked all five of its evidence paths at generation
`2026-10-18T09:50:35Z`, recorded complete `09:51:04Z`, and found matching
metadata with no recorded coverage gap.

## Validation and handoff

- `task docs:book:build`: passed with no mdBook warning or error.
- `task docs:ste -- --strict`: passed, zero problems in 24 files.
- `task docs:breaks -- --strict`: passed, zero long runs in nine chapters.
- `task docs:examples`: passed, 14 complete examples, zero failures or skips.
  The new chapter's TERM-ignore example is the fourteenth. It ran without a
  terminal, personal policy or real harness.
- The chapter's 14 local Markdown link occurrences resolve to files and
  headings. This bounded check is not a whole-book link audit.
- `task test:runner -- -E 'binary(launch) | binary(job) | binary(interrupt) |
  binary(confinement) | binary(noninteractive) | test(wait_events)'`: 54 passed,
  60 skipped by filter or fixture declaration, no failures. No Linux run.
- Before the final four document checks, SHA-256 digests froze 209 files:
  the nonignored files under `crates`, `scripts`, `docs/book` and `.cargo`, plus
  `Cargo.toml`, `Cargo.lock` and `Taskfile.yml`. After all four checks completed,
  every digest matched. No test or check input changed during its run.
  The earlier focused run separately checked 13 named source/test/example
  subjects before and after. These are bounded observations, not a new native
  qualification campaign. Local scratch logs and manifests are under
  `/tmp/ravel-k158/`; the durable results and scope are this task record.
- Final coverage checks on the chapter, SUMMARY, this leaf and the document
  brief report matching metadata with no recorded gap. This check remains
  best effort.

Part B is next, followed by one copy-edit of the whole chapter. Art needs the
escalation handoff above and part B's terminal sequence. No parent closes:
the document still needs its second draft and all three later stages. The
existing ADR decisions remain unchanged. No guide removal, whole-book proof,
reader-comprehension result or publication follows from this leaf.
