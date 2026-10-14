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
