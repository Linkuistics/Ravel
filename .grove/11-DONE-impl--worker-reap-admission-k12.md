# worker-reap-admission-k12

## Goal

Correct C2 in one complete incumbent slice: successful policy conversation may
reach resolution, Selected and managed launch only after confirmed worker reap
and checked post-evaluation private-directory cleanup. Preserve failure and
cancellation precedence, all available operational reports and bounded policy
output. Deliver discriminating checks and a compatible working launcher.

## Context

Read `worker-reap-admission-k11`, `completion-failure-scope-k7`,
`completion-failure-contract-k8` and `cancellation-failure-transfer-k10` beyond
the root brief. The independent oracle is
`docs/specs/result-job-completion.md#worker-teardown-before-admission`;
source/check rationale and C1 receipts are in
`docs/completion-failure-design-evidence.md`. Fixed preservation reference:
`0bdcb1f9ae24db5e9dd499308dd791744e908eb0`; retain existing expectations.
K11 changes task artifacts only, with no behavioral observation.

Tier 2 graph coverage at generation `2026-10-14T06:08:15Z` reported matching
metadata and no recorded gaps for worker/choice/front/refusal/CLI source and
read task/spec/ADR/Taskfile paths. This is best effort, not completeness proof.
Refresh project/generation/coverage before discovery. The evaluator's both-way
depth-one trace completed (one inbound, 25 outbound); unrelated heuristic
same-name edges were excluded through exact source. Exact evaluate/stop and
choice evaluate-and-resolve snippets establish the material chain.

## Working increment and ownership

- `crates/ravel/src/worker.rs`, evaluator and stop: retain the actual wait result
  and available stop/poll/kill errors with operation labels and worker identity.
  A confirmed nonzero exit satisfies reap. Preserve bounded grace/escalation;
  never signal a possibly reused numeric PID after confirmed reap. Close the
  protocol endpoint, attempt existing stop/kill/wait, collect bounded policy
  diagnostics, explicitly attempt private-directory removal, then compose the
  outcome before admission. Preserve errno/error text and checked removal
  path/remedy; do not silently retry away a reported cleanup failure. A failed
  poll followed by confirmed wait is a recoverable report, not proof of missing
  reap. Do not infer alive/gone from failed wait or claim successful teardown.
- Keep the conversation result until composition is complete. Cancellation wins;
  otherwise existing output-bound, refusal, deadline and protocol precedence
  remains. Success plus failed wait becomes `worker_failed`, stage worker,
  exit 5; successful conversation plus failed required removal also refuses
  with operational cleanup detail. Do not replace either primary classification
  with a secondary teardown failure. Preserve original failure details when a
  cancellation supersedes it and include every available stop/wait/removal
  report. Captured policy stdout/stderr stays policy-only, with its existing
  aggregate byte bound. Correct unconditional “worker reaped/stopped” language
  on unconfirmed paths. Do not expand the detached-policy-child guarantee.
- `crates/ravel/src/choice.rs`: retain the evaluation-error short circuit before
  `program::resolve`. Its `overruled` currently replaces the original refusal;
  make the narrow preservation change needed for late cancellation to retain
  original and operational diagnostics. Keep first-signal semantics and existing
  JSON classification fields/text stream labels. Use existing refusal message/
  remedy reporting or a minimal internal carrier; no new public fault API or
  incompatible report schema. `crates/ravel/src/refusal.rs` and sibling front
  `run.rs` may change only as needed to carry/report these observations through
  actual selection-failure settlement. Do not fork a test-only admission path.
- Private tests adjacent to evaluator/choice/front; existing
  `crates/ravel/tests/cli.rs` for real-policy regressions. Justify the smallest
  test-only report transform after real teardown and before admission. Thread
  it through the same internal production composition with identity behavior
  in production; no env/CLI switch, owner-policy fault convention or general
  syscall abstraction. Use process-isolated fixtures for signal/global state.
- Existing root `Taskfile.yml` owns repeatable commands. Update only when a
  required reusable selector cannot use its existing CLI_ARGS tasks. Evidence
  belongs in the existing completion-failure document with a linked per-case
  receipt if useful; do not claim C3/C4/native or root closure.

## Fixed expected cases

These outcomes are fixed before implementation. For every failed case assert
no successful Evaluation/Selected, no managed-start sentinel, empty result
stdout, original classification/metadata when primary, separately labelled
bounded policy output, and every injected wait/removal error plus identity,
operation and useful recovery advice. Successful judgment alone is not admission.

| Case | Required outcome |
|---|---|
| Successful conversation + reported final wait error | `worker_failed`, worker, 5. Neither resolution nor launch reached; no claim of confirmed reap. |
| Policy refusal + wait error | Existing `policy_refused`/selection/3, owner code/source/message/remedy retained; wait report secondary. |
| Deadline before readiness and after handover + wait error | Existing `selection_timeout`/evaluation/124 with original bound/input/source retained; wait report secondary. |
| Protocol failure + wait error | Original protocol code/worker/5 and protocol detail retained, plus wait report. Include premature EOF/no-result detail. |
| Output overflow + wait error | `output_limit`/evaluation/3 with aggregate bound and bounded captured bytes, plus wait report; retain existing overflow precedence. |
| Cancellation + wait error | First INT/TERM/HUP observation wins (existing cancellation signal/exit), all original conversation and wait/removal reports survive. Include cancellation during teardown and late `overruled` after a non-cancellation refusal. |
| Successful conversation + confirmed nonzero worker exit | Reap prerequisite satisfied; selection succeeds if required removal succeeds. Preserve captured output; do not equate nonzero worker exit with missing reap. |
| Successful conversation + failed checked removal, with and without failed wait | No resolution/Selected/launch; cleanup path/error/remedy present. With failed wait keep worker failure primary and both reports. |
| Refusal/timeout/protocol/overflow/cancellation + wait and removal errors | Same primary precedence as each row, every distinct operational report retained; no invented success. |
| Recoverable poll/stop error + actual confirmed wait | Keep recovered-operation diagnostic and actual status; no false unconfirmed-reap conclusion. |
| No fault | Successful selection, real managed start and exact published empty/Unicode/multiline result after settlement. Existing refusal/cancellation/deadline and C1 expectations retained. |

The control always closes endpoints and performs real stop/wait before replacing
its reported wait result. Assert/store real ExitStatus, elapsed bound, worker
PID, report supplied to production composition and physical fixture-directory
cleanup separately. Drain both diagnostic pipes within existing bounds; avoid
fixtures that leave detached writers holding them. Use a separate uninjected
fixture guard to remove physical debris after a reported removal failure.
Do not overwrite the injected report with that successful fixture cleanup.
If physical wait fails unexpectedly, the fixture fails and its bounded emergency
cleanup runs; it cannot claim a real reap from the injected report.

Prove the consumer boundary with two otherwise valid policy choices: a missing
program yields worker failure 5 rather than resolution's 127, and a valid fake
managed program writes a start sentinel that must remain absent under injected
wait/removal failure. Exercise the actual choice/front composition; a standalone
outcome helper or absent sentinel alone cannot prove resolution was skipped.
Record the intermediate Selected result/absence. The no-fault counterpart must
reach selection/start and complete normally, proving the fixture is capable.

## Execution and done when

1. Freeze the matrix and reference expectations; write the private controls and
   consumer assertions first. List each focused selector with
   `task test:session CLI_ARGS="--list -E 'test(worker_reap_admission)'"` after
   naming tests with that shared token, and confirm all expected cases/parameters
   appear. Use corresponding nextest listing for CLI/C1 selectors. Listing is
   required before crediting a green run; fixture-role ignores remain explicit.
2. Run `task test:session CLI_ARGS="-E 'test(worker_reap_admission)' --no-fail-fast"`
   against the incumbent admission behavior with the additive seam. Red must be
   the intended admitted-success, lost-report or false-reap assertion, not build,
   unavailable worker or fixture failure. Then implement the minimal complete
   slice and rerun green. Verify gate removal and diagnostic-loss mutants fail
   their distinct assertions; restore source before the final green measurement.
3. Record immutable SHA-256 before/after subjects for every measured run: all
   tracked project inputs actually read (source, manifests/lock/config, Taskfile,
   scripts, policy/fake-program fixtures, fixed expected matrix and preservation
   inputs), command/status and each parameter's outcome. Name generated caches,
   external tools/dependencies and runtime domains not frozen. Finish edits
   before measuring, one writer per receipt; retain real teardown receipts
   separately from injected reports. Matching totals alone are insufficient.
4. Run `task test:cli`, focused C1 runner/front controls via `task test:runner`
   and `task test:session`, then `task check`. Its full-workspace nextest run
   supplies `task test`; do not duplicate an identical passing full run without
   a reason. Required formatting/lint/types/build/probes and workspace
   process/PTY checks must actually pass. Preserve configuration/SDK/settings/
   discovery, exact bytes/first writer/nesting, combined failures, late signals,
   descendants and final write. These regressions do not establish unexecuted
   C4 or native-entry correctness. Verify documentation links touched here.
5. Retire only after the complete gate, diagnostic preservation, private cases,
   real-policy consumer controls and required workspace checks pass; record
   charges/limits and remaining obligations. If a material uncertainty or cap
   intervenes, leave coherent reviewable work, name the precise remainder and
   grow that narrower continuation before the book. No fresh funding is inferred.

## Funding and continuation

Prospective allocation P≤50/E≤25/RC≤5 minutes, D=X=0, one focused session,
zero production disruption. P includes repair, controls/tests and evidence;
E includes red/mutants/green/regressions/workspace checks; RC includes any one
permitted fresh reviewer and tree coordination/retirement/commit. Stop at the
first leaf or aggregate ceiling, carrying supported remainder for renewed human
funding rather than resetting a cap. No second in-session reviewer; substantive
new redesign/re-review becomes tree work with an explicit allocation.

K11 charges conservative D≤10/RC=0/P=E=X=0 including closure/commit.
Correctness cumulative charged bounds entering this leaf:
D≤60/RC≤20/P≤30/E≤5/X=0. Remaining D≤30 (20 reserved for phase closure),
RC≤10/P≤210/E≤115/X=0. Full use of this allocation would leave
D≤30/RC≤5/P≤160/E≤90/X=0; reserve is not actual spend. Record actual
conservative charges at closure and carry the account to the root/next mandate.

After C2 grow only the next precise supported continuation before
`book-structure-k5`, normally C3 partial-acquisition planning from the remaining
unreserved D≤10; keep the 20-minute closure reserve intact. C4 checked terminal/
failed-exec recovery and native entry remain required. Started is spawn-reported
success, not confirmed exec; exclusive terminal lease/identity premises remain
conditional. Persistent-host restoration is separate from the one-session CLI.

K4 remains closed: one iteration, seven attempts, R=1/8, F=4/4, N=3/6,
D≤45/RC≤15/P=E=X=0; k6 separately D≤45/RC≤10/P=E=X=0. Preserve G2 upkeep/
support/route cost, G3 delivery/use beyond entry, G4 unread alternatives,
G5 unexecuted runtime and R-W lifecycle/compatibility debt. No comparison,
variant/catalogue/routing read, target/package/extraction, migration or
simplification gain is authorized. Root independent-capability delivery,
unchanged owner config/complete sessions, human book structure and practical
post-proof reader checkpoint remain required.

## Decisions (running log)

- Execute the fixed k11/k12 plan and k8 contract in the mandated isolated jj
  workspace. No requirements, design campaign or generic planning restart is
  needed. Tier 2 discovery/coverage at generation `2026-10-14T06:14:28Z`
  refreshed evaluator/choice/front and relied-on documentation; no recorded
  source gaps. Cargo.lock is read directly because its graph freshness is
  untracked. Relevant evaluator both-direction depth-one trace completed (1
  inbound, 25 outbound); unrelated same-name heuristic edges were rejected by
  exact source. Choice resolves only after successful evaluation.
- The private control transforms a local teardown report after actual child
  wait, bounded capture and explicit directory close. It runs only in unit-test
  builds and is thread-local; ignored subprocess fixture roles supply actual
  choice/front composition, signal isolation and locate() layout. No product
  CLI/env switch or owner-policy failure convention is added. Physical wait and
  directory-removal receipts precede injected errors and remain separate.
- Keep the current refusal JSON schema and policy-only diagnostics. Operational
  context goes into existing refusal message/remedy; superseding cancellation
  retains the earlier refusal's complete existing text report. Confirmed nonzero
  exit satisfies admission. Partial preparation/identity cleanup remains C3.
  tempfile 3.27.0 close() source was read: it consumes and forgets its guard even
  on removal error, preventing silent destructor retry.
- Initial red was a fixture error (unknown diagnosticsBytes settings key), not
  conformance evidence. The corrected red reached actual wait/removal receipts
  and failed resolution/gate and lost-report assertions. Green then passed;
  gate and operational-report mutants failed their distinct expected assertions.
  Cancellation assertions exclude the input-receipt portion of stderr, preventing
  injected report logs from masquerading as product output. Named individual
  parameter tests now make every fixed case visible in nextest listing.
- The one fresh reviewer found three actionable gaps: timeout wording claimed
  stopping without confirmation; fixture output/wait had no effective deadline;
  overflow could discard original EOF/deadline information before cancellation.
  All were reproduced/inspected and classified valid/actionable. Convert every
  conversation failure before applying overflow precedence and retain it as
  context; use neutral teardown-attempt wording; fixture processes use file-backed
  capture, bounded polling and emergency group kill before direct fixture reap.
  New mixed EOF/deadline + overflow + cancellation controls and a physical
  fixture timeout/reap control cover the fixes. No second reviewer is commissioned;
  these fixes are conclusively covered by the executable seams, not a new design.
- The initial public CLI run exposed a stimulus collision: its existing
  unwritable-stdout/confined-cleanup fixture chmodded the shared TMPDIR, also
  making the now-required worker removal fail earlier. Preserve every original
  assertion; narrow that fixture to its owned confined directory and create a
  blocker there. A separate real-permission CLI case retains the former shared-
  TMPDIR stimulus and requires worker_failed before admission plus outer cleanup
  diagnostics. Both controls restore fixture permissions independently. This
  changes the failure stimulus to keep testing its intended boundary, never the
  protected expected code/output/diagnostic assertions.
- Running the 35 individual C2 parameters alongside the separate CLI suite
  overloaded fixture startup: several intended-success/overflow/refusal cases
  reached the real one-second pre-readiness timeout instead. That run is not
  credited green. Preserve the one-second expected bound and all assertions;
  nextest's scoped threads-required override reserves the active run's slots
  for each worker-admission case (verified in installed 0.9.148 help and official
  docs). Subsequent measured suites run sequentially. No retries or timeout
  widening normalize this scheduling failure. The first workspace check found
  one mechanical Clippy single-character push_str lint; change it to push.
- Closure conservatively charges P≤40/E≤5/RC≤5 minutes, D=X=0, including
  repair/test/evidence authoring, execution, the one fresh reviewer and tree/
  retirement/commit. Correctness cumulative bounds are D≤60/RC≤25/P≤70/E≤10/X=0;
  remaining D≤30 (20 reserved)/RC≤5/P≤170/E≤110/X=0. Grow only C3 planning
  `partial-acquisition-cleanup-k13` before the book, allocated D≤10 and
  RC=P=E=X=0. C3/C4/native and root/G2–G5/R-W obligations remain unchanged.
  No glossary meaning or ADR choice changed; the existing worker-ordering ADR
  now points to checked C2 evidence. The reversible local repair earns no new ADR.

## Verification and stopping state

All 35 explicitly listed C2 cases pass after the final gate and diagnostic-loss
mutants were restored. Item-by-item selected names match the workspace run;
complete input manifests are identical between those two final measurements.
Every measured run retained immutable before/after subjects and per-case receipts
in `docs/worker-reap-admission-checks.txt`; repetition encoding round-tripped to
raw log digests. Uncredited fixture/scheduling/lint failures are retained too.
Twelve real-policy CLI cases, 11 C1 wait-event cases and four C1 front cases pass.
`task check` passes formatting, scripts, worker build/probes, runtime integrity,
types, lint and all 177 selected workspace tests, including process/PTY cases;
16 fixture roles are ignored as direct selections. No redundant full run was
added. The later focused rerun follows intentional mutants, using identical
restored project inputs. No product or test expectation edits followed it.

Current specification/component/ADR/evidence prose carries C2 while retaining
C3/C4/native limits and fixed-reference history. Local file/anchor validation
passed 104 links; missing-file and missing-anchor controls failed as intended.
HTML dynamic fragment IDs were checked against source and diagram manifests;
no rendering/layout check is claimed. The new ledger was read and losslessly
validated directly; graph coverage remains best effort, not completeness proof.

The useful stopping state is a working admission/diagnostic repair with measured
controls and the next funded planning mandate. No live review names this producer,
so no Creator receipt is due. K13 and the book remain live; the root's Done when
is incomplete. No parent close or finish teardown is earned.

Final coverage at generation `2026-10-14T06:48:05Z`, recorded 06:49:42 UTC,
checked 55 source/document/task/configuration paths with matching metadata and
no recorded issue. The two plain-text receipts are not graph-tracked and were
read/validated directly. The documentation measurement held 146 project/wrapper
subjects identical before/after; its controls/validator and receipts are appended
after measurement. Evidence publication and final tree notes are outside the
behavioral measurements. Book bytes match the parent through renumbering.
Changed product/test/configuration inputs still match the passing workspace
measurement. Root and k13 carry the same conservative budget account.
