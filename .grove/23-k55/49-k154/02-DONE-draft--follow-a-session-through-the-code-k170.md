# follow-a-session-through-the-code-k170

## Goal

Complete part B of chapter 8 in
`docs/book/src/08-follow-a-session-through-the-code.md`, under the named
structure brief and contract `docs/book/contract.md`. Start from part A's
successful-path map. Teach the refusal and cancellation forks, error/signal
precedence, and where an independent change belongs.

## Context

- Load `linkuistics:writing-code-walkthroughs` at orientation depth. Source
  remains authoritative; the guides can be stale. No full-source fragment graph.
- Append these reserved level-two headings in order: “Follow a refusal or
  launch failure”, “Follow cancellation”, “Choose the final outcome”, “Find
  where a change belongs”. Part A's eight sections stand; fill the reserved
  scope without restructuring them or adding empty headings.
- Refusal/launch forks: policy refusal; refusal of the policy file, behaviour
  or data; selection timeout; missing/unexecutable selected program. Keep their
  classifications distinct, including timeout versus policy refusal. Locate
  the decision and the point where managed launch stops.
- Cancellation forks: during selection, during supervision and after a result
  was sent. Trace `run`, `settle`, `supervision_failure` in
  `crates/ravel/src/run.rs`, and `Handlers::finalise`/propagation in
  `crates/ravel/src/cancellation.rs`. **`join_cancellation` is test-only in
  this checkout**; do not teach it as the production join. Preserve the final
  decision versus later output-write window and no-atomic-output guarantee.
- Use the outcome-precedence activity figure from k153, checking its claims
  against the source. Reuse chapter 5's `selection-cancellation` and
  `supervision-cancellation` figures unless the code path needs another.
  Hand new figure work to art through the node brief.
- Verify “An independent-change scenario” in `docs/components.md` and teach
  where the change belongs. Do not present an undelivered component or public
  reuse contract. Chapter 10 belongs to future component work.
- K155's log holds the complete O1–O10 destination table, exact guide statement
  groups and successful-sequence art contract. Finish O3, O4, O5, O7 and O9
  where it marks part B. O6/O8 detail belongs to chapter 9. Enumerate the ten
  obligations again, with final section destinations and explicit limits.
- Part A teaches `ravel: <report>` on stderr in its final section. The
  second-half set proof still decides chapter 6's entry.
- Part A's narrow reviewer corrected worker frame ownership (`converse`
  sends frames returned by `load`/`select`) and terminal order (child pre-exec
  handover, parent `lend` inside `watch` after the start check). Preserve both.
  Its real chapter 3 run was nonterminal, and it claimed no runtime trace of
  every internal event. Guide native-entry status is historical, not current.

## Done when

- The reader can name the production function and rule that choose the final
  cancellation outcome, and can locate a concrete independent change.
- The whole chapter passes `task docs:book:build`, strict STE,
  `task docs:breaks -- --strict`, `task docs:examples` and `task docs:excerpts`.
  Every source excerpt names its file under `docs/book/excerpts.md`.
- Guide statements absorbed by part B and final obligation destinations are
  in the log; art needs are in the node brief, with participants and order.
- The successful path remains intact, scope exclusions remain explicit and
  no complete-source or reader-comprehension claim appears.

## Notes

Copy-edit k171 is already the later live sibling. Do not add another copy-edit
leaf. The ordinary one-reviewer allowance is this leaf's, with any escalation
through the editorial family. The node remains live through copy-edit/art/proof.

## Decisions (running log)

- The named structure brief is `docs/book/contract.md`. It names “a
  16-year-old software engineer with no background in Ravel”, with Rust,
  TypeScript, shell and general OS knowledge. Its ordered section plan puts
  chapter 8 between the component map and job mechanics. The chapter outcome
  includes “the refusal and cancellation forks and the error and signal
  precedence”, then locating a change. Its emphasis includes “What success
  means” and authority boundaries. Its exclusions keep evidence campaigns and
  undelivered contracts outside the book. These named statements discharge
  the draft preconditions and the orientation intake without another interview.
- Part B appends the four reserved headings in their prescribed order. Part A
  remains intact. Production source at parent `5ae641d5` is authoritative.
  Source excerpts are exact quotations, tests are evidence, and the guides
  supply migration inputs rather than current implementation authority.
- Production cancellation joins in `Handlers::finalise`, then `run::settle`
  chooses the primary outcome. The exact expression includes a second CLI
  latch read before the pending signal. Explain that detail in prose and an
  excerpt. The activity figure compresses it and needs an art handoff rather
  than a draft-stage figure edit. `join_cancellation` remains test-only.
- The independent-change example changes result storage while preserving the
  public behaviour. It identifies transport, appearance observation and CLI
  composition obligations without proposing an extraction or public contract.
- The four appended sections now give the refusal/timeout/worker/program
  classifications, two runner signal carriers, the source-order cancellation
  join, primary-failure precedence and storage-change boundaries. Chapter 5's
  two cancellation sequences are reused. The activity is embedded with prose
  that expands its compressed signal step. Art has the precise handoff in the
  node brief. No product source or architectural decision changes.

## Component obligation inventory

Re-enumerated the ten rows of `docs/components.md`, “Scoped behavioural
obligations”, against the session and completion specifications and current
source. This is a teaching map, not complete-source or comprehension evidence.

| ID | Final teaching destination | Scope and limits |
|---|---|---|
| O1 — owner policy, SDK, settings and discovery compatibility | Chapter 7; chapter 8 “Prepare the request” | Configuration details remain chapters 4 and 6 |
| O2 — no result authority for policy; identity check and worker reap before launch | “Evaluate and check the policy”, “Follow a refusal or launch failure”, “Follow cancellation” | Chapter 7 explains the process boundary; detached policy children stay outside its guarantee |
| O3 — judged decisions, refusals, exceptions, malformed data and bounds without fallback; caller data unchanged | “Prepare the request”, “Evaluate and check the policy”, “Follow a refusal or launch failure” | Distinguishes policy refusal from timeout and worker failure; full context protocol excluded |
| O4 — direct argv in caller context, requested confinement or refusal, fresh nested context | “Prepare the managed launch”, “Follow a refusal or launch failure”, “Find where a change belongs” | Chapter 5 teaches confinement/nesting; chapter 9 owns backend/job mechanics |
| O5 — first complete result, exact empty/multiline/Unicode values, invalid channels refuse | “Send the result”, “Read the result”, “Choose the final outcome”, “Find where a change belongs” | Same-user authentication is not promised; chapter 5 owns public failure use |
| O6 — matching launch reap/group cleanup; later child status does not decide result | “Finish the managed job”, “Read the result”, “Choose the final outcome” | Chapter 9 owns detailed guarantees and resource/identity limits |
| O7 — cancellation suppresses sent results and propagates after cleanup attempts | “Follow cancellation”, “Choose the final outcome” | Includes error without Ended, late observations and output-write window; no atomic output guarantee |
| O8 — foreground/mode recovery and group stop before reap | “Finish the managed job”, “Follow cancellation” | Chapter 9 owns the conditional recovery contract and entry state |
| O9 — only result bytes on stdout after cleanup; write/cleanup failure prevents success | “Prepare the managed launch”, “Clean up and write the result”, “Choose the final outcome” | Operation reports use stderr; a failed final write can leave a prefix |
| O10 — matched installed CLI/worker and SDK | Chapter 7 | Chapter 12 owns delivery details; no new installed-layout claim |

## Guide destinations for migration

These part B statement groups complete k155's part A map for k164. No guide
is removed. Existing historical native-entry status is not copied as current
fact. Chapter 9 retains its job-mechanics and returning-host limits.

| Source statement group | Destination or owner |
|---|---|
| `components.md`, “Cancellation and partial failure”: selection cancellation, worker teardown, no launch | “Follow cancellation”, selection sequence and text |
| Same: consumed runner signal survives an error without Ended | “Follow cancellation”, two-carrier table; “Choose the final outcome”, source join |
| Same: selected-scope cleanup on output/channel/launch failure, local allocator cleanup | “Follow a refusal or launch failure”, final two paragraphs |
| Same: differing participant knowledge and channel/launch association | Part A “Read the result”; “Find where a change belongs” |
| “Error and signal precedence”: latch order, original/secondary diagnostics, cancellation priority, cleanup order, group and output failures | “Choose the final outcome”; exact source adds the current final snapshot details |
| Same: worker judgment prevents launch; invalid result, failed cleanup and output cannot succeed | “Follow a refusal or launch failure”, “Choose the final outcome” |
| Same: required replay of result/cleanup and cancellation/cleanup combinations | “Find where a change belongs”, check paragraph |
| Same: abrupt death, same-user code and escaped descendants | Chapter 5 “Understand the operational limits”; part A names group/path limits; chapter 9 owns detailed guarantees |
| Same: child entry transparency does not restore a continuing host | Chapter 7's private-runner limit and chapter 9; not a new public contract here |
| “An independent-change scenario”: result representation, first complete value, exact bytes, empty/absent, nested isolation, appearance, matching Ended, cleanup and unchanged policy | “Find where a change belongs”, boundary table and causal explanation |
| Same: independent process fixture demonstrates composition, not adoption | Chapter 11 owns the test seam; no external-consumer claim in chapter 8 |
| `session-channel.md`: partial allocation cleanup and retained original/secondary errors | “Follow a refusal or launch failure” locates allocator versus run ownership |
| Same: explicit cleanup disarms best-effort Drop, unknown identity refuses deletion, moved directories need manual cleanup | Chapter 9 owns detailed resource recovery; chapter 8 part A states the identity/concurrency limit |
| Same: test commands, fixture controls, suite prerequisites | Chapter 11, as k155 already records |

The named complete source corpus is intentionally absent: this chapter is an
orientation. New-reader comprehension, new runtime/platform qualification,
real-agent behaviour and public reuse remain unestablished.

## Independent review

The leaf's one fresh reviewer read the whole chapter and contract, with a
narrow adversarial source check of the four new sections. Both findings are
valid and actionable:

- The refusal table initially stated only “without cancellation”, which left
  worker teardown's status 5 too broad. `worker::evaluate` preserves an
  earlier policy refusal or timeout when teardown also fails; diagnostic
  overflow has its own priority. The table now states isolated failures,
  separates teardown after a successful conversation, and the nearby paragraph
  gives the exact precedence from `worker.rs:598–635`. This is a source-exact
  qualification, not a redesign or a need for a second review.
- The activity figure omits the second CLI latch read. The existing art
  handoff already names this defect and `look(true)`'s mixed snapshot. The
  chapter's exact expression, table and figure caption preserve the complete
  rule while art retains ownership of the figure correction.

No unclear-contract, accepted-trade-off or noise finding remains. The reviewer
reported no other concrete finding in its assigned scope. Its inspection
establishes no runtime result or exhaustive correctness. Copy-edit is the
separate editorial pass already queued as k171.

## Verification and limits

- Tier 2 graph discovery used project
  `Users-antony-Development-Ravel.simplify-and-componentize`, ready/full,
  generation `2026-10-18T09:03:49Z`. Exact-name searches completed with
  `has_more: false`; both-direction depth-1 traces of `settle`,
  `Handlers::finalise` and `evaluate_and_resolve` were untruncated. A first
  search used a regex where `file_pattern` expects a glob and returned zero;
  the corrected searches, exact snippets and direct reads supply the evidence.
  Unrelated/test trace edges are navigation only, not call-order proof.
- Coverage checked every source and supporting path relied on, including
  `crates/ravel/src/{main,run,choice,worker,policy,cancellation,program,
  authority,confinement,completion,private_directory,refusal}.rs`, runner
  `src/{run,channel}.rs`, the component and channel guides, both specs, all
  three ADRs, contract/STE/excerpt notes, chapters 3/5/6/7/8 and SUMMARY,
  the three reused figure sources, Taskfile, glossary, build/example scripts
  and brief/task chain. Each returned `no_recorded_issue` with
  `metadata_match`; no missed range required fallback. The last metadata
  record was `2026-10-18T09:08:29Z`. This remains best-effort graph coverage.
- Exact snippets and direct reads checked each new technical claim, with
  special attention to worker failure composition, the runner's error-carried
  signal, final blocking before the late-latch read, queued-signal eligibility,
  propagation fallback and channel/launch association. The whole chapter was
  read, and the four new sections retain their reserved conceptual order.
  Part A compares byte for byte with the parent revision, unchanged.
- After the last chapter edit, all six tasks exited 0:
  `task docs:book:build` without warnings, strict STE on chapter 8 with zero
  problems, `task docs:breaks -- --strict` with no long run in eight chapters,
  `task docs:excerpts` with eight passes and no page errors,
  `task docs:examples` with thirteen passes and no failures/skips/page errors,
  and `task docs:figures` with ten matching figures. Six labelled fragments
  are not run. The example suite includes the chapter 3 successful session
  and chapter 5's selection/after-result cancellation cases, through the
  cleared-environment real-policy/fake-program seam, without a terminal.
- All 25 local chapter destinations resolve, with anchors checked against the
  built HTML. The same check rejected deliberately absent file and anchor
  controls. No rendering or reader-comprehension claim follows.
- The 203 measured subjects were tracked files under `crates/`, `scripts/`
  and `docs/book/`, plus `Cargo.toml`, `Cargo.lock`, `Taskfile.yml`, `CONTEXT.md`,
  `target/debug/ravel` and `target/libexec/ravel/ravel-policy`. SHA-256 lists
  before and after the checks compare equal. List digest:
  `61ca16be6bdf2fe791a6ad4225badeb3447b7b248e4e78493aa60228a5d36633`.
  Scratch lists are `/tmp/ravel-k170.tCGkVZ/{before,after}.sha256`.
  Generated book output and check logs are not committed.
- The checks do not establish every signal interleaving, terminal behaviour,
  platform qualification, real-agent operation, public reuse or new-reader
  comprehension. Art owns the figure correction and full successful sequence;
  copy-edit owns its existing prose handoffs. No new architecture or ADR
  decision follows, so the ADR set stays unchanged.
- The live later sibling `follow-a-session-through-the-code-k171` already
  holds copy-edit. The family rule therefore requires no successor cut.
  No live review names this producer. The document node remains live through
  copy-edit, art and proof, so no parent close or cascade is due.
