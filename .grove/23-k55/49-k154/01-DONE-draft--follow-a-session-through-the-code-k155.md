# follow-a-session-through-the-code-k155

## Goal

Draft **part A** of chapter 8, as the document node's brief describes, under
`docs/book/contract.md`, into
`docs/book/src/08-follow-a-session-through-the-code.md`, linked from
`docs/book/src/SUMMARY.md`: the successful session from `main` to the result
write, with the `end-session` side of the channel. The headings for part B are
not written here.

## Context

- Chapters 1–3, 5 and 7 are proved before this runs. Chapter 3 gives the stages
  of a session in plain words; this chapter shows where each stage is in the
  code. Link back instead of re-teaching.
- Load the `linkuistics:writing-code-walkthroughs` skill first. Use it at
  orientation depth, as the node brief says.
- Run the chapter 3 example in a scratch directory with the checkout build and
  read the order from the run and the code together. A debugger, a trace or a
  temporary log in a scratch copy of the source is allowed, and nothing from
  that copy is committed.
- Settle the heading structure for the whole of chapter 8 now, so that part B
  fills sections and does not reshape the file. Write down in this leaf's log
  which headings part B owns.
- `task docs:excerpts` and the example check exist before this leaf runs. If
  either is missing, stop: `book-source-excerpts-k150` is not retired.

## Done when

- Part A's sections exist, build and pass `task docs:book:build`, the strict
  STE check, `task docs:breaks -- --strict`, `task docs:examples` and
  `task docs:excerpts`. Each source excerpt names its file under the
  convention.
- The full successful-session sequence is described in the prose and its figure
  need is in the log for the art stage (the participants, the messages and the
  order). Do not draw it here: art owns the figure.
- The log lists the obligations that part A teaches (some of O3, O4, O5 and
  O9) and which part of them remains for part B.
- Last act: `leaf-add` a `draft` leaf (bare stem
  `follow-a-session-through-the-code`) whose body states part B's scope from the
  node brief, the headings reserved for it and anything part A learned that
  changes it; then `leaf-add` the `copy-edit` leaf if no live later sibling
  already holds it.

## Notes

- One fresh reviewer is allowed under the leaf-wide allowance. Spend it on the
  order of the steps and on the claim, if the chapter makes it, about which
  function does each step.

## Decisions (running log)

- The named structure brief is `docs/book/contract.md`. Its reader is “a
  16-year-old software engineer with no background in Ravel”; its chapter 8
  outcome is to “follow one successful session from `main` to the result
  write”. Its ordered chapter plan places the parts before the code path and
  job mechanics after it. Its emphasis includes the authority boundaries and
  stated limits. It calls chapter 8 “an orientation” that “does not cover the
  complete source”. These statements discharge the three draft preconditions.
  The agreed contract supplies the walkthrough intake, without another interview.
- Part A owns these level-two headings, in order: “Trace the example”,
  “Prepare the request”, “Evaluate and check the policy”, “Prepare the managed
  launch”, “Send the result”, “Finish the managed job”, “Read the result”,
  “Clean up and write the result”. Each follows a causal boundary, not directory
  order. Part B appends, in order: “Follow a refusal or launch failure”,
  “Follow cancellation”, “Choose the final outcome”, “Find where a change
  belongs”. No empty part B heading appears in part A.
- Source is the unchanged checkout at parent `52e0223a`. Production bytes are
  authoritative. The guides and specifications supply intent, with direct
  source checks for each implemented claim. Tests and the example run are
  evidence, not included production source. This orientation uses exact
  excerpts and no complete-source inventory or fragment graph.
- The ordinary chapter 3 example passed through `task docs:examples --
  docs/book/src/03-a-complete-session.md`: one passed, no failures or skips.
  The task built the matching pair and ran the page's files in scratch, with
  a cleared environment, private HOME/TMPDIR and fake-agent PATH. It checks
  `finished` without a newline, status 0 and the two stable stderr lines.
  This observes the external path, not every internal event or terminal action.
- Source fixes the internal order. `choice::judge` checks the selection during
  `worker::evaluate`; worker reap and directory removal precede its successful
  return and executable lookup. `ResultOutput::reserve` precedes channel
  allocation. Runner `watch` can see child exit before channel appearance;
  `supervise_reported` checks appearance again after reap and recovery. Do not
  require an observed grace period on this example.
- `join_cancellation` in `run.rs` is test-only in this checkout. Part B must
  trace the production join through `Handlers::finalise` in `cancellation.rs`,
  rather than repeat the node brief's older function list as a production call.
- Part A teaches the non-JSON operation report in “Clean up and write the
  result”. It uses `ravel: <report>` on stderr and distinguishes a report from
  the final outcome. The second-half set proof still decides chapter 6's entry.

## Component obligation inventory

Re-enumerated O1–O10 from `docs/components.md`, “Scoped behavioural
obligations”, against the session and completion specifications. This table
records section ownership, not complete-source coverage or comprehension.

| Obligation | Teaching destination | Remainder after part A |
|---|---|---|
| O1 — owner policy, SDK, settings and discovery compatibility | Chapter 7; chapter 8 “Prepare the request” locates the path | Full configuration teaching stays in chapters 4 and 6 |
| O2 — worker has no result authority; identity check and reap precede launch | Chapter 7; “Evaluate and check the policy” | Failure branches in part B |
| O3 — checked policy decisions without fallback, unchanged caller data | “Prepare the request”, “Evaluate and check the policy” | Part B “Follow a refusal or launch failure” owns refusals, exceptions, malformed data and limits |
| O4 — direct argv in caller environment, confinement or refusal, fresh nested channel | “Prepare the managed launch” | Part B locates failure paths; chapter 5 teaches confinement/nesting; chapter 9 explains mechanics |
| O5 — first complete result, exact empty/multiline/Unicode, invalid channels refuse | “Send the result”, “Read the result” | Part B relates result errors to final precedence |
| O6 — matching launch reap/group cleanup, later child status does not decide result | “Finish the managed job”, “Read the result” introduce the boundary | Chapter 9 owns detailed reap/group guarantees and limits |
| O7 — cancellation suppresses results and propagates after cleanup | “Clean up and write the result” locates finalisation | Part B “Follow cancellation” and “Choose the final outcome” own the full rule |
| O8 — terminal recovery and group stop before reap | “Finish the managed job” identifies the owner | Chapter 9 owns foreground, modes and conditional failure guarantees |
| O9 — result alone on stdout after cleanup, write failure stays failure | “Prepare the managed launch”, “Clean up and write the result” | Part B owns combined-error precedence; no further normal-path teaching deferred |
| O10 — matched installed CLI/worker and SDK | Chapter 7 | Chapter 12 owns delivery details |

## Guide destinations for migration

These are sentence groups absorbed by part A, for k164 to verify after proof.
No guide is removed here. Part B and chapter 9 still own the detailed forks.

| Source statement group | Chapter 8 destination |
|---|---|
| `components.md`, “Composition and coordination”, R1/R2/R3/R4 normal-path relations | “Trace the example”, “Prepare the request”, “Evaluate and check the policy” |
| Same section, R5/R6/R7 normal-path calls, shared result and order | “Prepare the managed launch” through “Clean up and write the result” |
| “A successful operation”, request, worker start/hello/import/judgment/reap and executable lookup | “Prepare the request”, “Evaluate and check the policy” |
| Same section, output reservation/channel/argv, first complete file, group cleanup/reap/recovery/read/close/write | “Prepare the managed launch” through “Clean up and write the result” |
| “Cancellation and partial failure”, last paragraph: distinct participant knowledge and channel/launch association | “Read the result”, “Clean up and write the result” |
| `session-channel.md`, allocation, absolute inherited path and fresh nested context | “Prepare the managed launch” |
| Same guide, `end_session`/`publish_result`, temporary file and no-clobber hard link, first/empty/multiline/Unicode result, invalid context | “Send the result” |
| Same guide, runner appearance only, `result_after`, absent versus empty, corresponding launch obligation | “Finish the managed job”, “Read the result” |
| Same guide, held-directory read, regular-file/bound/UTF-8 checks, symlink/FIFO refusal | “Read the result” |
| Same guide, explicit close, directory identity and concurrent-pathname limit | “Clean up and write the result” |

The guide's test commands and fixture isolation remain chapter 11's. Detailed
allocator-failure/Drop behaviour, manual debris recovery and failure composition
remain for parts B/chapter 9 or migration's sentence-level check. Existing
chapters 2, 5 and 6 own same-user limits, stale context, the 1 MiB bound and
public outcomes. The older guide's native-entry conformance status is not
copied. R8/build identity remains chapters 7 and 12's.

## Art sequence contract

Participants: caller, original Ravel CLI (runner is in-process), policy worker,
managed process/fake agent, its `end-session` CLI invocation, result channel.
Ordered messages and actions:

1. Caller invokes `main`/`start-session`; `run` prepares input and policy entry.
2. CLI starts worker; worker moves to `/`, registers modules, sends hello.
3. CLI checks identity, sends evaluate(path, request); worker imports policy.
4. Worker sends policy description; CLI judges it and requests select. No
   context exchange in this example. Worker calls policy and sends selection.
5. CLI judges selection, closes socket, ends/reaps worker, drains diagnostics,
   removes worker directory. Only then resolves `fake-agent`.
6. CLI reserves stdout, allocates channel, composes argv/environment, invokes
   runner. Runner creates group and spawns fake. Child-side terminal handover,
   when applicable, precedes exec. Parent start check precedes ordinary
   supervision; the parent's `lend` callback runs inside `watch`, after that
   check, not between spawn and `supervise_started`.
7. Fake prints prompt (caller stderr), invokes second CLI with `finished`.
   Second CLI reads inherited path, writes/flushes temporary file, adds
   completion name with hard link. Its return does not wait for job cleanup.
8. Fake exits. Optional branch if it continues: appearance starts grace,
   then TERM and KILL as needed. Do not promise the runner observes appearance
   before child exit, or that the fake always reaches natural exit before stop.
9. Runner observes exit without releasing identity, stops group, reaps direct
   child, recovers lent terminal, confirms group gone and checks channel
   appearance in `Ended`. CLI receives the ending.
10. CLI checks corresponding ending, reads result through held directory,
    reports operations, restores entry mask, closes channel, removes any
    confined workspace (absent here), restores stdout, finalises signals.
11. `settle` decides from result/cleanup/group/signal observations. Only success
    writes and flushes unchanged `finished`; `main` returns status 0.

An X means confirmed reap/removal, as in chapter 3, not merely exit. The
normal-path figure makes no platform/terminal runtime measurement claim.

## Independent review

The leaf's one fresh reviewer inspected the chapter against the source for
step order and function ownership. Both findings are valid and actionable:

- Worker `load` and `select` return frames; `converse` sends them. The table
  now names `converse` via each helper. Source: worker `main.ts`, 114–116
  and 138–140. This is a local actor-label correction.
- The first terminal description compressed child-side pre-exec handover and
  parent-side `lend` into an incorrect order. It now distinguishes the child
  handover during spawn from the parent's callback inside `watch`, after
  `Started` and the start check. Source: runner `run.rs`, 1408–1451 and
  `watch`'s `lend()`. The art sequence above carries the same correction.

The reviewer reported no further finding in its assigned scope. No unclear
contract, accepted trade-off or noise finding remains to classify. These
source-exact attribution corrections need no second reviewer or redesign.
Copy-edit remains the independent prose pass, followed by art and proof.

## Verification and limits

- Tier 2 graph work used project
  `Users-antony-Development-Ravel.simplify-and-componentize`, ready/full,
  generation `2026-10-18T08:58:51Z`, metadata recorded complete at
  `2026-10-18T08:59:20Z`. Search pages were complete. Trace pages for `main`
  (outbound depth 2), `choose_prepared` and `supervise_reported` (both
  directions, depth 1) were not truncated. Some unrelated/test edges occur,
  so exact snippets and direct reads establish the prose's causal claims.
- Coverage was checked for every source and supporting artifact relied on:
  `crates/ravel/src/{main,cli,run,choice,inputs,settings,authority,program,
  worker,completion,private_directory,cancellation,signal_state}.rs`, worker
  `src/main.ts`, runner `src/{run,channel}.rs`, `scripts/examples.sh`,
  Taskfile, glossary, book contract/excerpts/STE notes, chapter 3, chapter 8,
  SUMMARY, both guides, both specifications and all three ADRs. All returned
  `no_recorded_issue` with `metadata_match`; no missed range needed fallback.
  This best-effort result is not a claim of graph completeness.
- After the final chapter edit, `task docs:book:build` passed without warnings;
  strict STE passed for chapter 8 and SUMMARY; `task docs:breaks -- --strict`
  passed all eight present chapters; `task docs:examples` passed all 13 examples
  with no failures/skips/page errors; `task docs:excerpts` passed all five
  excerpts with no failure or page error. Six labelled fragments are not run.
- All 15 chapter links resolve to source pages and, for anchors, built HTML
  IDs. A deliberately absent path and absent anchor both failed the same check.
- The measured subjects were all tracked paths under `crates/`, `scripts/`
  and `docs/book/`, plus `Cargo.toml`, `Cargo.lock`, `Taskfile.yml`, `CONTEXT.md`,
  `target/debug/ravel` and `target/libexec/ravel/ravel-policy`: 203 files.
  SHA-256 lists before and after the five checks compare equal. The list digest
  is `fb4fda1dc2a5b51b9d0cd80fd096d302e8487aeb7fdcd942b7b5bb78ee5e7271`.
  Scratch lists are `/tmp/ravel-k155-{before,after}.sha256`; no evidence log
  or generated book output is committed.
- The whole chapter was read after the corrections. Structure follows the
  successful operation from the whole path through its boundaries and back to
  observable output. The narrow technical review is complete. Copy-edit owns
  the two wording/link matters in the node brief, art the figure. No independent
  editorial or visual proof, new-reader comprehension, terminal run, platform
  qualification or complete-source coverage is claimed by this draft.
- No architectural decision changed, so the three ADRs remain unchanged.
  Part B and copy-edit keep document node k154 live; no parent closes here.
- The final work act created part B `follow-a-session-through-the-code-k170`,
  then copy-edit `follow-a-session-through-the-code-k171`, both with the bare
  document stem. Their bodies carry the reserved headings, source corrections,
  scope and validation obligations. No duplicate successor or review leaf was
  present. This leaf is retired; the document node remains live.
