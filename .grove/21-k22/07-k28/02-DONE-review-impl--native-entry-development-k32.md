# native-entry-development-k32

**Reviews:** native-entry-development-k30

## Goal

Try to disprove the development artifact's partial-handler and common-finalisation
composition before the complete native qualification builds on it. Produce
findings only, under the review-impl skill; the qualifier still owes the full
153-row native campaign regardless of this review's outcome.

## Context

Read k30's committed artifact through its handle, the k28 brief, the required
completion spec and selected native transition design. The available development
failures/passes and explicit unqualified remainder are in
`docs/native-entry-development-evidence.md`. K29's human approved the renewal
verbatim (“Approve the renewal (Recommended)”); this review is allocated RC≤5,
D=P=E=X=0, from the shared aggregate rather than resetting the phase caps.

K28 already used one in-session reviewer. This is the tree-owned continuation
for the additional doubt, never a renewed spawned-review allowance. No current
GROVE_PROVIDER_EVIDENCE receipt was present when k30 finished, so no Creator line
is supplied; do not invent or copy one.

## Done when

- Inspect the artifact and contract for a counterexample where a partial
  selection/runner handler install or failed mask operation leaves the front's
  knowledge inconsistent with actual protection, permits an unsafe disposition
  release, loses cancellation, or admits a result beside uncertainty. Challenge
  the installed subset, immutable entry eligibility, main-versus-reader masks,
  signal joins and release ordering together.
- Inspect whether returned Ended reports and the single JSON notice preserve
  initiating/secondary diagnostics under the approved EPERM rule, including
  failure/no-ending paths. Trace real source ownership rather than trusting the
  development summaries or substituted reports as physical facts.
- Distinguish a concrete implementation/contract violation from the explicitly
  unconstructed qualification controls. The full remaining matrix is already
  live in k31; its absence is not evidence that any claimed qualified run passed.
  Identify any claimed development control whose construction cannot distinguish
  the failure it purports to test.
- Report findings with source evidence and applicable contract. Do not edit
  product code or spend an in-session reviewer. Charge actual RC within its
  RC≤5 allocation; do not borrow D or exceed the approved caps.
- If integration is warranted, cut the ordinary integrate-review-impl step
  before live qualification k31, naming this review's handle rather than
  transcribing its findings as the integration charter. Leave full acceptance,
  supported-target qualification and all parent/root obligations live.

## Findings

### P1 — Runner cancellation can arrive after its last latch read

At `crates/ravel/src/cancellation.rs:181–192`, `Handlers::finalise` consumes
the runner latch **before** `try_block_and_check` blocks the calling thread.
The runner's handlers remain installed after supervision
(`crates/keyed-launch/src/run.rs:1094–1096`, `1758–1794`); their callback writes
only `INTERRUPTED_BY` (`run.rs:295–297`), whereas the final snapshot reads the
front latch and pending set (`cancellation.rs:124–167`).

A concrete source interleaving starts with a successful published result, no
previous cancellation, entry-unblocked TERM and completed supervision. The
front restores its entry mask; line 181 reads an empty runner latch. Before
line 182's block takes effect, TERM runs the runner handler and fills that
latch. The signal is consumed, so it is absent from the pending set. The front
latch is also empty. The final join therefore returns no cancellation, releases
dispositions and the mask, and `crates/ravel/src/run.rs:270–274` can write the
result successfully. There is no later runner-latch read on this path.

This violates the completion spec's first-consumed-cancellation/common-
finalisation requirements (`docs/specs/result-job-completion.md:308–322`) and
T7/T8's final blocked latch/pending snapshot in the selected transition design.
It also loses a newly consumed first signal on partial runner-handler failure
paths, although their existing operational error still prevents success.
Integration should establish the final runner observation under confirmed
blocking and preserve the documented front/outcome/late precedence, including
the failed-block path. A deterministic acknowledged boundary control must put
TERM after the old latch read and before the block; a general after-reap test
or probabilistic signal loop does not discriminate this ordering.

## Review evidence and limits

Reviewed k30 commit `6b0d1136f3af395a978074692154e4bc438ebdec` against the
current source after k29's funding/evidence-only changes. Inspected front
partial-install ownership, immutable entry eligibility, reader shield interface,
runner handler ownership, common finalisation, and Ended/JSON/error report
composition. Successful stop reports reach `operationReports` even if result
extraction later fails; runner errors use their composed message on no-ending
paths. No additional concrete report-loss finding was established in this
bounded read. This is not an exhaustive certification of those paths.

Tier 2 graph evidence used project
`Users-antony-Development-Ravel.simplify-and-componentize`, generation
`2026-10-15T07:11:54Z`. Exact finalise/handler snippets and inbound traces were
read, with complete returned pages. Coverage checks for the nine relied-on
source/contract/evidence files reported metadata matches and no recorded gaps;
that best-effort result is not proof of completeness. Direct source supplied the
interleaving above. The existing development final-block control checks stale
mask knowledge, not the separate late-runner-latch gap found here. The evidence
note already excludes the unconstructed E/F/S controls and qualified campaign;
their absence is not an additional finding or a claimed failed measurement.

No tests, builds, lint or formatting commands were run, and no product/test
source was changed. Runtime reproduction and every fix/check belong to
integration and qualification.

## Decisions (running log)

- Record the late-runner-latch race as actionable and insert
  native-entry-development-k33, integrating this review, immediately before
  native-entry-qualification-k31. Keep k28's full acceptance and k31's entire
  matrix live; no parent closes and no reviewer allowance resets.
- Conservatively charge RC≤5 minutes including inspection, findings, tree,
  retirement and commit; D=P=E=X=0. Cumulative bounds become
  D≤115/RC≤72/P≤225/E≤60/X=0. Balances are D≤20 exclusively phase closure,
  RC≤2/P≤100/E≤70/X=0, shared by k33 and k31 under the approved caps.
