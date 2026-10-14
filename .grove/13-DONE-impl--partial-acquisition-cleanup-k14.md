# partial-acquisition-cleanup-k14


## Goal

Correct and check C3 on the incumbent: each private-directory allocator owns
partial preparation, records identity before another process gains access,
returns the original failure plus every checked cleanup report, and transfers
ownership only on success. Deliver the complete allocator-to-consumer slice
with measured controls, preserving C1/C2 admission and failure settlement.



## Context

Read `partial-acquisition-cleanup-k13` for source discovery and pricing,
`completion-failure-scope-k7` for the human agreement and
`completion-failure-contract-k8` for the repair design. The independent contract
is `docs/specs/result-job-completion.md#cleanup-after-partial-acquisition`;
source/check rationale and C1/C2 receipts are in
`docs/completion-failure-design-evidence.md`. Fixed preservation reference remains
`0bdcb1f9ae24db5e9dd499308dd791744e908eb0`; protect its ordinary expectations
alongside the now-checked C1/C2 repairs. No new JSON schema or public fault API.

The working increment is complete checked private-directory ownership, not a
standalone guard or a cleanup helper waiting for migration. All three owners
and their error/admission consumers belong together. There is no supported
extraction or migration increment to split out. Successful owner configuration,
complete sessions and existing result-channel anchored reads remain usable.

## Ownership and enabling points

Refresh Tier 2 graph/generation/coverage before implementation. K13 source was
verified at generation `2026-10-14T06:54:50Z`, with matching coverage metadata and
no recorded gaps in the named Rust paths. Searches and both-direction depth-one
traces completed; exact source rejects unrelated heuristic edges. The graph
missed Confined::prepare's front caller, which direct front source establishes;
absence of that edge is not an absence-of-use result. No exhaustive inventory
or runtime proof follows from coverage.

- `Confined::prepare` in `crates/ravel/src/confinement.rs`: directory creation
  precedes root canonicalization, runtime-file/protected-path/system-grant
  validation, executable discovery and tmp subdirectory creation. Hold local
  identity immediately after creation; every returned error after that point
  explicitly settles the directory. On success transfer into Confined and keep
  that same identity for remove and best-effort Drop. Preserve
  confinement_unusable/confinement_overlap, CLI stage, malformed status 2,
  input/source/message/remedy; append operational path/operation/error reports.
- `SessionChannel::allocate` in `crates/ravel/src/completion.rs`: after directory
  creation, open_directory and keyed_launch::Channel::allocate can fail before
  the wrapper exists. Protect even failure of the first identity/held-reader
  open. Transfer the local owner only after directory and channel acquisition
  both succeed; keep anchored read and appearance semantics. Preserve original
  io error kind/source/errno information alongside cleanup context; do not
  manufacture a new generic error that discards it. The front still converts
  allocation failure to session_channel_unavailable/exec/5. Existing close/Drop
  disarms TempDir by keep() before checking held/current dev+ino and type.
- `worker::evaluate` in `crates/ravel/src/worker.rs`: directory creation precedes
  UnixStream::pair, above_stdio, held_descriptors and spawn. Private controls at
  those fallible operations must settle partial ownership before returning.
  Preserve worker_failed/worker/5 and descriptor source/remedy, and
  worker_missing/worker/5 for missing worker spawn. Record identity before spawn
  grants cwd access. Retain the owner through child teardown, bounded capture
  and checked removal; confirmed wait plus required removal still gates success.
  Preserve original conversation/overflow/cancellation precedence and all C2
  operational reports. No removal while a worker may still use the directory
  may be described as confirmed overall settlement.

Use a small private owner/report mechanism if shared behavior needs one; do not
introduce a generic syscall/filesystem abstraction or expose testing switches.
The representation is chosen against these contracts, not a mandated crate/type.
All acquired descriptors still close under their existing owners. Failures
before creation own no new directory and must not report a fictional removal.
Identity capture failure is an ordinary returned failure with an explicit
unknown-owned-directory report; never authorize deletion by pathname alone.

Before explicit cleanup, disarm the pathname-based destructor on all outcomes.
Check recorded identity against symlink_metadata at the owned path; preserve
replaced directories and symlinks, report missing/moved/unknown identity and
provide useful advice to inspect the named path and locate the moved owned
directory. Do not invent its new location when not known. A removal error names
the owned path and operation; original acquisition failure remains primary in
the absence of cancellation. Best-effort Drop cannot retry an explicit failed
attempt or delete a replacement. Source comparison/check/remove is not an
atomic race guarantee: do not claim concurrent pathname-mutation safety from
snapshot fixtures. If the implementation requires a new ownership premise or
cannot satisfy the required replacement protection within scope, externalize
the precise unresolved contract and seek funding rather than weakening it.

## Fixed discriminating cases

Freeze independent expected classifications, messages/metadata and case names
before implementing. Use a shared `partial_acquisition_cleanup` token and
individually listed parameter cases; controls are cfg(test), allocator-local,
and isolated/serialized where they affect process signals or environment.

| Controlled boundary | Required observation |
|---|---|
| Each allocator: creation itself fails | Original classification; no fabricated local path/removal, no successor admitted. |
| Each allocator: identity capture fails after real creation | Original identity error retained; exact owned path and deletion uncertainty/advice reported; no unsafe pathname deletion or destructor retry. |
| Confined: post-creation canonicalization, representative runtime/protected validation and tmp creation failures | Original refusal metadata and detail retained; checked cleanup success or distinct removal report; no policy evaluation/Selected/job start. Include real validation failure without fault injection. |
| Session: held-reader open and appearance-channel allocation failures, separately | Original operation/kind/errno information survives conversion; no managed start/result; acquired confined owner settles too. Session allocation is after Selected, so do not falsely require Selected absence here. |
| Worker: protocol pair, above-stdio duplicate, descriptor discovery and missing/other spawn failure, separately | Original code/stage/status/source/remedy; checked local cleanup; no successful Evaluation/resolution/Selected/managed start. Confined outer cleanup is also attempted. |
| Every post-creation preparation failure above, with confirmed removal and reported removal error | Local checked attempt observed; original failure primary, every removal path/operation/error retained, no result stdout. Do not count successful fixture cleanup as production cleanup success. |
| Each allocator: real rename without replacement, directory replacement with contents, and symlink replacement to unrelated contents | Preserve unrelated contents/target and moved owned directory; return moved/replaced uncertainty, not success; keep residual physical fixture ownership separate. Repeat after successful preparation transfer and later explicit cleanup. |
| Each allocator: recorded identity becomes unreadable/unknown at cleanup | No unsafe deletion, all primary/cleanup information plus manual inspection advice; descriptor closure and destructor disarming remain checked. Distinguish capture failure from later check failure. |
| Otherwise successful worker evaluation + replaced/unknown/removal failure | Required removal gates admission; worker_failed/worker/5, no resolution/Selected/start. Actual worker stop/wait precedes identity/removal testing; confirmed nonzero exit still satisfies reap. |
| Refusal/deadline/protocol/overflow/cancellation + worker cleanup failure, with C2 wait reports | Original precedence retained, separately bounded policy stdout/stderr and every original/wait/cleanup report survive. Exercise first INT/TERM/HUP and late cancellation override using existing isolated choice/front fixture. |
| No fault: all successful owner transfers, later explicit cleanup | Real selection/start and exact empty/Unicode/multiline result after cleanup; both original and moved-path destructor controls are capable of detecting a retry. Preserve anchored session reads and first writer/nesting. |

Private post-creation controls must execute real acquisition before reporting
failure. Identity fixtures mutate real directory names/contents. Removal-report
substitution and physical cleanup are separate: store the reported operation,
identity/path and result, then use an uninjected fixture guard to remove actual
debris after assertions. Inspect residual contents before that guard runs. A
reported-failure test may perform real removal first only when its receipt
explicitly distinguishes physical removal from the substituted report; it must
not serve as the proof that destructor retry was suppressed. The retry control
must leave real observable debris/replacement until after the production owner
is dropped. Count attempts or use a discriminating residual fixture, then prove
an intentional retry mutant fails. Fixtures kill/wait all actual subprocesses,
close endpoints and bound polling/capture; no detached writers or unbounded wait.

Exercise actual `choice::choose_prepared` and front `run`, adapting the existing
private worker-admission subprocess roles without adding product env/CLI flags.
For worker failure use a missing-program choice to distinguish worker 5 from
resolution 127, and a valid managed program with a start sentinel. Assert actual
Selected absence as well as absent sentinel; the no-fault counterpart must reach
Selected/start. Confined errors happen before handlers/worker installation;
do not claim selection cancellation was captured there. Channel failures occur
with retained selection handlers: signal override must preserve the original
allocation plus local and outer cleanup reports. C1 front settlement controls
protect first-signal joining and no-ending behavior. Fixture input logs must be
excluded from assertions about product diagnostics, as in C2.

## Done when

1. Construct the fixed private cases first and list their individual names with
   `task test:list CLI_ARGS="-E 'test(partial_acquisition_cleanup)'"`. Review every
   expected row/parameter and ignored subprocess role. Use nextest list, not
   test:session --list. The additive-seam red against incumbent behavior must
   fail intended lost-report/identity/ownership/admission assertions, not setup,
   missing-worker/build/timeout errors. Make the minimal complete repair and run
   `task test:session CLI_ARGS="-E 'test(partial_acquisition_cleanup)' --no-fail-fast"`.
2. Demonstrate distinct diagnostic-loss, identity-bypass, destructor-retry and
   admission-gate mutants fail their intended assertions. Preserve replacement
   fixtures in an isolated disposable parent so a broken cleanup cannot affect
   unrelated user files. Restore all source before the final green measurement.
3. Retain immutable per-case receipts and SHA-256 before/after subjects for each
   measured run: every project input actually read, manifests/lock/config,
   Taskfile/scripts, real-policy/fake-job fixtures, fixed matrix/reference inputs
   and measurement wrapper. Finish edits before measuring, one writer per run;
   record command/status and every named case outcome. Name external/generated
   inputs and runtime domains not frozen. Store actual wait/removal/identity
   receipts separately from injected reports; match reruns item by item.
4. Run `task test:cli`, protected C2 via test:session, C1 front via test:session
   and wait_events via test:runner, then `task check`. List focused selectors
   first. Run suites sequentially and retain C2's scoped scheduling controls;
   do not widen deadlines/retry failed fixtures into success. Full workspace
   checks include process/PTY, formatting/lint/types/build/probes; task check
   already supplies task test, so do not duplicate an identical passing run.
   Preserve config/SDK/settings/discovery, exact bytes, first writer/nesting,
   combined failures, late signals, descendants and final-write expectations.
5. Update current contract/evidence prose with scoped checked C3 observations
   and immutable receipts; validate touched local links. No source/design or
   private report is evidence that a real OS syscall failed. Retire only when
   this complete slice and required checks pass, recording actual charges and
   carrying the remaining obligations/account upward. At the first cap or
   material uncertainty, preserve coherent reviewable work and the precise
   remainder instead of claiming the whole slice complete.

## Notes

Prospective allocation P≤60/E≤30/RC≤5 minutes, D=X=0, one focused session,
zero production disruption. P covers implementation/private controls/tests/
evidence; E covers measured red/mutants/green and regressions/workspace; RC covers
any one permitted narrow new-artifact reviewer and coordination/retirement/
commit. A second review need becomes tree work, explicitly priced; no second
in-session reviewer or invented RC funding. This is a local incumbent repair,
not new design discovery. No public fault API or generic filesystem framework.

K13 conservatively charges D≤10/RC=P=E=X=0 including closure/commit.
Entering correctness cumulative bounds: D≤70/RC≤25/P≤70/E≤10/X=0.
Remaining D≤20 (all reserved for phase closure)/RC≤5/P≤170/E≤110/X=0.
Full use of this prospective allocation leaves D≤20/RC=0/P≤110/E≤80/X=0;
allocation is not spend. Record actual conservative charges at closure; no leaf
resets the aggregate D≤90/RC≤30/P≤240/E≤120 caps. No unreserved planning D
remains. Price only the next precise C4/native-entry continuation from existing
supported k8 design and remaining P/E, before `book-structure-k5`; if further
discovery/design/review is necessary or limits prevent a supported mandate,
grow a precise human scope/funding continuation instead of silently spending
the 20-minute closure reserve or resetting the account.

C4 checked failed-exec/terminal recovery and native entry/waitability/signal
transitions remain required. Started is spawn-reported success; exclusive
terminal lease/identity premises remain conditional. A continuing host/persistent
runner has its own contract. K4 stays closed with one iteration/seven attempts,
R=1/8,F=4/4,N=3/6,D≤45/RC≤15/P=E=X=0; k6 separately D≤45/RC≤10/P=E=X=0.
Carry G2 support/upkeep/route cost, G3 delivery/use beyond entry, G4 unread
alternatives, G5 runtime and R-W lifecycle/compatibility debt. No comparison,
catalogue/routing read, target/package/extraction/migration or gain is inferred.
Root independent-capability delivery, unchanged owner configuration/complete
sessions, human book structure and practical post-proof reader checkpoint remain.

## Decisions (running log)

- Bootstrap resolved the mandated leaf and root-only brief chain. Tier 2 graph
  discovery/traces/snippets and coverage refreshed at 2026-10-14T07:37:35Z; the
  six material Rust paths have matching metadata and no recorded gaps. Direct
  front source supplies the omitted Confined::prepare caller; unrelated heuristic
  edges are not evidence. Cargo.lock pins tempfile 3.27.0.
- The fixed cases precede repair and use ignored process-isolated roles. Product
  completion source is compiled privately into the binary test target so the
  actual front can receive allocator-local controls without a public fault API.
  C2 report instrumentation now acts only when its own report is armed; C3 must
  observe physical residual identity/removal independently of C2 substitutions.
- Follow the existing k8/k13 design and k14 plan inline. The small local directory
  owner will disarm TempDir immediately, record device/inode before process access,
  and disarm its own best-effort Drop before checked cleanup. Snapshot comparison
  supplies the agreed replacement protection, with no concurrent mutation claim.

- The corrected additive-seam red has intended diagnostic-loss, unsafe identity
  and failed admission assertions. The first run failed only fixture parsing
  (`--cwd` is not a public flag), so is retained but credited no red evidence.
  Initial green exposed erroneous fixture expectations for EIO's preserved kind,
  session_cancelled after selection, and nested debris removed by checked outer
  cleanup. Fix those against the existing public contract: session fixtures use
  an independent exit-dir, and compare original EIO kind/source/errno. No product
  expectation was weakened. Add real runtime validation and descriptor closure.

- The one fresh reviewer returned three actionable findings, verified against
  source: actual Channel allocation discarded its typed OS cause; worker controls
  omitted acquired outer confined ownership; conversation/cancellation + physical
  C3 cleanup + C2 wait-report combinations were missing. Add executable controls
  for each. Real missing-directory channel red reports Other instead of NotFound;
  retain Channel's original io::Error in LaunchError's source chain and select
  that kind at the SessionChannel conversion. Preserve LaunchError::raw_os_error's
  existing spawn-only semantics. This is local diagnostic retention, not a new
  public fault API, ownership premise or generic filesystem design. These fixes
  are conclusively checked through the funded seams; no second review is needed.
- Private receipt logging is separated from captured product stderr before
  assertions. Front settle already emits the superseded allocation error there;
  testing returned cancellation text alone missed that existing report boundary.

- Reviewer coverage gaps are filled by real front worker cases retaining acquired
  confined ownership, and fixed refusal/deadline/protocol/overflow × physical
  removal/identity-failure × reported wait/secondary poll/cancellation cases.
  INT/TERM/HUP preserve the first signal; the existing choice override is also
  exercised with residual C3 cleanup. Actual wait succeeds before substitution,
  including confirmed nonzero exit. Product stderr and physical/injected receipts
  are distinct. A moved-path retry control restores the owned inode to its original
  name after explicit failed cleanup, making an erroneously armed Drop destructive.

- All 97 C3 cases and the four distinct final mutants discriminate as required.
  Protected CLI 12/C2 35/C1 front 4/runner 11 and task check's 283 workspace cases
  pass. Focused outcomes match the workspace item by item. Before documentation
  closure, refine receipt labels to distinguish entering owner cleanup, actual
  identity/removal/wait operations and substitutions where no syscall was tried.
  Re-measure the changed private instrumentation, retaining the earlier runs.

- The refined physical/report receipt run again passes all 97 C3 cases, and the
  final task check passes all 283 selected workspace cases plus formatting,
  scripts, worker build/probes, runtime integrity, types and lint. Protected
  CLI/C1/C2 remain item-by-item confirmed. Product and private instrument edits
  preceded each measurement, and every before/after subject digest agrees.
- Conservatively charge P≤35/E≤10/RC≤5 minutes including remaining evidence,
  tree/retirement/commit, D=X=0. Cumulative correctness D≤70/RC≤30/P≤105/E≤20/X=0;
  remaining D≤20 (all closure reserve)/RC=0/P≤135/E≤100/X=0. Unused allocation
  is not spend. C4's existing design is prospectively priceable at P≤65/E≤35,
  but further review/coordination RC is exhausted. Grow terminal-entry-funding-k15
  before the book to obtain explicit RC renewal and any necessary native-entry
  discovery/design funding. No cap resets, parent completion or target follows.

## Verification and stopping state

The final C3 receipt run has 97 passing selected cases; the final workspace run
has 283 passing cases with 17 ignored fixture roles outside direct selection.
CLI 12, C2 35, C1 front 4 and runner wait-events 11 pass and match the final
workspace item by item. The four distinct mutants fail diagnostic-loss, identity,
destructor-retry and admission assertions; the final gate mutant distinguishes
worker failure 5 from missing-program resolution 127. All source was restored
before green. The receipt collector verifies final Rust/nextest hashes against
both final run manifests, round-trips every encoded log and the gzip payload,
and retains immutable per-run input digests/status/outcomes. Gzip keeps these
receipts below jj's normal new-file snapshot limit without changing repo settings.
The artifact is docs/partial-acquisition-cleanup-checks.txt.gz.

All 146 local paths/fragments in six touched documents validate with a stable
input manifest; the private measurement shell wrappers pass ShellCheck. The
continuation resolves with the root brief, and the book retains its body through
its rename. No current architectural choice changed, so no new ADR is needed;
the existing worker-order ADR now points to scoped C3 evidence.

Final Tier 2 coverage generation 2026-10-14T08:12:06Z has matching metadata and
no recorded gaps for the thirteen material Rust paths and touched prose/config.
The private-owner trace also omits session/worker cleanup edges; direct source
confirms their preparation and later explicit calls. This is best-effort graph
evidence, not an exhaustive call inventory. The compressed receipt is not graph
tracked; direct decompression, digest validation and item-by-item comparison
supply its evidence. No OS wait/removal fault is inferred from a substituted
report, and no atomic concurrent pathname protection is inferred from snapshots.

This completes the local incumbent C3 slice. C4/native entry, independent
capability delivery, later supported target/migration and the human book/reader
work remain. terminal-entry-funding-k15 and book-structure-k5 are live, so no
parent closes. Retire this leaf and seal its one focused jj change with its
artifact/evidence, funding continuation and promoted account.
