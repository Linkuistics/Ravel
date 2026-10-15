# native-entry-settlement-k25

## Goal

Specify the missing bounded runner/worker error-settlement protocol and reprice
the complete native-entry repair. Preserve k22's one useful increment and frozen
behavior; this leaf delivers design and a supported complete price, not product
implementation or native conformance.

## Context

Read native-entry-settlement-prerequisite-k23, the verbatim agreement and charged
balances in native-entry-settlement-scope-k24, and the implementation-prerequisite
section of docs/native-entry-transition-design.md. The current completion spec
owns behavior; the transition design and k22 brief own T0–T8, the additive Started
API and the frozen E/F/R expansion. K23's reconciled findings are source evidence,
not measured failures or unquestionable new requirements. Reuse exact source
evidence where current and verify the mechanisms this protocol actually needs.

Approved phase caps are D≤130/RC≤55/P≤240/E≤120/X=0 minutes, zero production
disruption. After k24's RC≤4 charge, cumulative correctness charged bounds are
D≤100/RC≤53/P≤155/E≤30/X=0. Remaining are D≤30 (D≤10 for this continuation
and D≤20 exclusively phase-closure reserve), shared RC≤2/P≤85/E≤90/X=0.

This leaf allocates D≤10 including closure: provisionally 6 for bounded
ownership/error protocol, 2 for exact finite discriminators and 2 for complete
pricing/tree/closure. RC≤2 is shared across at most one focused review,
coordination and closure; P=E=X=0. These are allocations, not spend or new caps.
Full use leaves only the D≤20 reserve, RC=0/P≤85/E≤90/X=0. Record actual
charges before pricing or commissioning any successor; fund necessary future
planning/review/closure too. The old P≤95/E≤80/RC≤4 complete estimate is
suspended and confers no replacement producer entitlement.

## Done when

- Revise the existing design and required spec in place, distinguishing the
  agreed behavior, proposed mechanism and unobserved implementation. Select an
  explicit error-settlement bound and nonblocking observation strategy after
  failed Started and worker-reader admission. State when each deadline begins,
  its relation to existing grace/collection limits and what ownership remains
  on expiry. No path may rely on an unbounded physical wait to reach recovery.
- Specify checked stop/group-stop reports, pre-reap ordering and treatment of
  observation/wait errors, EINTR and ECHILD. Preserve the initiating error,
  first consumed cancellation and every original/secondary report. Distinguish
  attempted termination, confirmed reap and confirmed group absence; uncertainty
  is an explicit error, never invented success. Do not signal after confirmed
  reap or after the identity is uncertain and released. Reconcile protected
  wait-event traces individually rather than weakening them to fit the design.
- Specify unknown-status terminal recovery without granting arbitrary foreground
  takeover. Preserve C4's exclusive-authority/no-unrelated-ID-reuse premises;
  distinguish attempted from confirmed foreground/mode recovery and retain
  unconfirmed recovery reports. Do not lend a reclaimed terminal again on the
  Started-error path. Keep real child/group/lease ownership, actual-only Reaped,
  callback io source, spawn-only errno and T7/common-finalisation joins intact.
- Keep the existing CLI with real policy/fake job, real-process and PTY seams.
  Specify and justify any necessary finite cleanup-failure amendment explicitly,
  with exact parameters, owner-local private controls, meaningful broken controls,
  physical-operation versus substituted-report receipts and bounded fixture
  cleanup. Retain the frozen additive API and other T0–T8 decisions unless the
  human changes them. No blanket matrix expansion, installed fault switches or
  target infrastructure is authorised.
- Fit the discriminators and fixture cleanup within the existing 30-second
  outer/5-second acknowledgement/5-second cleanup limits and existing configured
  production grace/collection limits. If a legal case cannot fit, state and price
  the exact timing remainder before execution. Never infer a longer bound.
- Price the whole coherent repair: all source work, frozen E/F/R plus any
  explicitly justified finite amendment, immutable manifests, baseline/broken/
  repaired measurements, protected C1–C4/CLI/session/runner/terminal/workspace
  checks through the existing Taskfile, item-by-item receipt reconciliation,
  evidence and necessary review/planning/closure. Compare with actual remaining
  P/E/RC and unreserved D after this leaf's charges. Unused allocations are not
  spend. A complete price above those balances earns one precise human funding
  decision, never a narrower acceptance claim or reserve borrowing.
- At most one focused reviewer may challenge boundedness, identity safety and
  uncertainty/report retention within the shared RC≤2. Classify every finding;
  material redesign or a further review need is a precise continuation, not a
  second allowance. Record the updated protocol, exact price, limitations and
  actual charges in the brief chain before retirement.
- Cut only the next precise supported continuation inside k22, before the book.
  If the whole repair and necessary handoff are supported, externalise planning
  with its price and actual allowance; a design session cuts no implementation
  leaf. Otherwise cut the exact human scope/funding remainder. No product code,
  native fixture construction, behavioral execution or implementation commission
  occurs here. Keep k22 live until its complete repair and checks are delivered.

## Notes

The returning-host contract, minimum Rust 1.85/Linux runtime qualification,
K4/K6 comparison history, G2–G5/R-W debt and all root delivery/documentation
obligations remain live through the parent brief. No target, extraction,
simplification gain, independent adoption, book acceptance or phase/root close
follows from a protocol design or a supported price.

## Decisions (running log)

- Select owner-local nonblocking wait with a 2-second absolute runner error/post-KILL budget and 2-second total worker teardown, retaining its 1-second grace and separate 1-second bounded reader collection. No physical blocking wait reaches recovery. Expiry reports unresolved ownership; it cannot establish reap or group absence.
- Preserve protected logical wait-event traces individually; emergency observation failure retains its single group-then-PID stop, while failed Started uses checked stop plus two pre-reap group stops. ECHILD/released identity forbids further signals. Unknown-status terminal recovery preserves foreign holders and reports unconfirmed foreground/modes under C4's authority/no-reuse premises.
- Propose exactly 32 S01–S05 cleanup-failure rows, retaining frozen E/F/R. They are priced but not commissioned, use owner-local controls and separate physical operations from reported substitutions within the 30/5/5 controller envelope.

- The one permitted focused reviewer returned “No concrete finding” against
  boundedness, identity safety and uncertainty/report retention. No findings need
  classification; no second review was launched. This is a narrow aimed read,
  not behavioral evidence. Protected logical traces were reconciled individually.
- Charge conservative D≤10/RC≤2, P=E=X=0 including protocol/source evidence,
  finite amendment/pricing, presentation, focused review, tree/verification and
  retirement/commit. Cumulative correctness bounds are D≤110/RC≤55/P≤155/
  E≤30/X=0; remaining D≤20 exclusively closure reserve, RC=0/P≤85/E≤90/X=0.
  The full future cycle needs D≤5/RC≤10/P≤125/E≤85, so exact added funding is
  D≤5/RC≤10/P≤40. Recommend caps D≤135/RC≤65/P≤280/E≤120/X=0. No unused
  allocation is spend and no reserve is borrowed. Cut only the precise funding
  continuation native-entry-complete-funding-k26; no producer is commissioned.

## Verification and stopping state

Read the complete nine-path jj diff. Mechanical checks pass for docs/tree-only
scope, whitespace, local link targets, manifest/source references, frozen additive
API and E/F/R byte equality against the parent, unchanged book SHA-256 and all
price/balance/amendment arithmetic. Product source and protected wait-event tests
are unchanged; no behavioral suite, native fixture or measurement ran.

Served the existing viewer through task docs:result-job PORT=8881. Chrome opened
the exact entry-failure deep link and visibly rendered the revised sequence,
current-discussion panel and updated marker. Corrected a Mermaid message separator
parse error and rechecked the actual layout. Desktop light appearance was checked;
mobile/dark appearance and human comprehension were not checked. Port 8879 was
occupied by an unusable stale viewer, so the dedicated presentation uses 8881.

Source Tier 2 coverage matched metadata with no recorded Rust gaps at generation
2026-10-15T03:45:25Z. Subsequent document coverage reports partial Mermaid line 18
and changed README/manifest metadata; those exact sources were read in full and
the diagram rendered in the browser. Best-effort coverage is not completeness.
The final self-read made the normal logical reap deadline explicit as well as
the failed-Started deadline, and disallowed unbounded terminal EINTR retry.
No architectural/package decision changes, so the ADR set remains current.

Retire only k25. The complete repair remains unimplemented in k22, whose next live
leaf is native-entry-complete-funding-k26 before the unchanged book. No parent
cascade, correctness-phase close or root close is earned. Seal this design and
funding handoff as one jj change; preserve the phase-closure reserve.
