# native-entry-correctness-k18


## Goal

Price one complete, independently useful native-entry correctness increment on
the incumbent one-session launcher CLI, spanning entry, worker admission, runner
and front settlement. Deliver its bounded implementation/check mandate before
the book, or a precise funding/scope remainder if the complete increment cannot
be supported within the renewed allowance. Preserve the checked C1–C4 repairs;
select no component, package or migration.



## Context

- `native-entry-funding-k17` records the human's “approved” renewal. Phase caps
  are D≤100/RC≤40/P≤240/E≤120/X=0 minutes, with zero production disruption.
  Charged bounds after that agreement are D≤70/RC≤38/P≤140/E≤30/X=0.
  Remaining are D≤30, of which D≤20 is exclusively the phase-closure reserve,
  RC≤2/P≤100/E≤90/X=0. This leaf may use only the renewed D≤10 for discovery,
  planning and its closure. RC≤2 is shared with subsequent review/coordination
  and closure, not renewed per leaf. P=E=X=0 in this planning session.
- `completion-failure-contract-k8` and
  `docs/specs/result-job-completion.md#one-session-native-launcher-entry` own the
  required entry contract; the adjacent test-seams section and
  `docs/completion-failure-design-evidence.md#discriminating-checks` own the
  agreed boundaries and planned native cases. Use current source/toolchain
  verification to price the repair; old locations and source observations are
  navigation, not fresh runtime evidence.
- `failed-exec-terminal-recovery-k16` delivers scoped C4 checks, linked under
  `docs/completion-failure-design-evidence.md#c4-checked-observations`.
  Its pre-exec terminal-INT case does not establish the whole entry contract.
  Started is spawn-reported success, not confirmed exec. Failed-spawn handback
  still requires exclusive foreground authority and no unrelated reuse of the
  recorded group identity through handback; no recorded identity permits
  post-reap signalling. Retain these premises and C1–C4 evidence distinctions.

## Done when

- Verify the current ownership and transition path needed for one complete
  executable-entry-to-settlement repair: capture exec-surviving signal state
  before runtime changes (including Rust SIGPIPE startup), establish waitability
  before both policy worker and managed child, and check handler/mask changes
  or retain their failures. Keep child entry values separate from launcher
  repair state. Preserve ignored and blocked INT/TERM/HUP/SIGPIPE semantics,
  including pending blocked signals and child ignore/default dispositions.
- Account for auxiliary threads' signal eligibility, handlers retained through
  mask restoration, and joining pending/front/runner/late cancellation on both
  success and failure. Distinguish launcher-targeted INT from terminal INT after
  foreground handoff, and child signal death from launcher cancellation. Retain
  temporary handoff cleanup, required terminal recovery, original/secondary
  diagnostics, signal re-raise and exact-result settlement before CLI exit.
  The persistent runner/returning host remains a separate contract.
- Freeze discriminating before/after cases through the agreed public CLI with
  real TypeScript policy/fake jobs, transport/real-process and runner/PTY seams.
  Include ignored SIGCHLD and SA_NOCLDWAIT with confirmed worker and child waits,
  ignored/blocked signal inheritance, actual INT targeting, transition/after-reap
  arrival and no-ending failure settlement. Name justified owner-local controls
  for otherwise unreachable transition errors; keep reported errors separate
  from physical fixture operations and require bounded kill/wait/PTY cleanup.
  No public testing switches or unbounded syscall framework are authorised.
- Protect C1–C4, CLI/configuration/results and existing real-process/PTY behavior
  with untouched expectations and meaningful broken controls. Specify the
  supported host/target/toolchain domain and explicit unknowns. Use existing
  Taskfile workflows, immutable per-case receipts and subject provenance; this
  planning leaf constructs no controls and runs no behavioral measurements.
- Before growing a producer, price its complete source changes, instruments,
  red/green and protected CLI/C1–C4/runner/workspace checks, evidence, necessary
  review/coordination and retirement/commit against the actual unused P≤100,
  E≤90 and shared RC≤2. Record this leaf's actual D/RC charges first; allocate
  no reserved D and count no proposed allocation as spend. Grow only the exact
  next complete supported increment before `book-structure-k5`, with no hidden
  dependency on an unpriced repair. If material new mechanism, further discovery
  or review cannot fit, grow a precise human scope/funding continuation before
  the book rather than starting implementation or waiving an obligation.

## Notes

The D≤10 ceiling includes closure; preserve enough to write the supported mandate
or precise remainder and retire/commit. Do not restart settled requirements or
the component comparison. K4 retains reference
`0bdcb1f9ae24db5e9dd499308dd791744e908eb0`, one iteration/seven attempts,
R=1/8,F=4/4,N=3/6,D≤45/RC≤15/P=E=X=0; k6 separately retains
D≤45/RC≤10/P=E=X=0. Carry G2 six-month support/upkeep/route cost, G3 delivery
and use beyond entry, G4 unread alternatives, G5 unexecuted runtime domains and
R-W compatible-runtime/new-lifecycle debt. No target, extraction, independent
adoption, migration, simplification gain or root completion follows. Root
delivery, unchanged configuration/complete sessions, later design decisions,
human book structure and practical post-proof reader checkpoint remain live.

## Decisions (running log)

- Charge conservative D≤10 minutes for this bounded source/toolchain read,
  planning, tree update, verification, retirement and commit including closure;
  RC=P=E=X=0. No reviewer or behavioral instrument is constructed or run.
  Cumulative correctness charged bounds are D≤80/RC≤38/P≤140/E≤30/X=0.
  Remaining are D≤20, exclusively phase-closure reserve, RC≤2/P≤100/E≤90/X=0.
  These are charged bounds, not proposed allocations. No reserve is borrowed.
- The complete useful increment remains executable entry through worker
  admission, managed supervision and front settlement, on the incumbent CLI.
  An early SIGCHLD edit alone would leave unchecked transitions and failure
  settlement owed. No smaller independently useful increment satisfying this
  entry contract was established by this bounded read; splitting by module
  would conceal those dependencies.
- Current mechanisms already include pre-runtime capture and thread shielding.
  Their checked failure/ownership protocol is not yet designed sufficiently to
  price the complete implementation and all discriminators. Do not commission
  implementation from a guessed P/E allocation or assume a complete adversarial
  transition review fits the shared RC≤2. Cut a precise human continuation,
  `native-entry-transition-funding-k19`, before the book, as this leaf permits.
  Proposed additional D≤20/RC≤10 is a bounded design/pricing/review allowance,
  not approval, spend or a claim that P≤100/E≤90 will suffice.

## Current-source ownership and transition evidence

Tier 2 graph discovery found `main.main`, `signal_state.record`, front `run.run`,
`Handlers.install`/`block_and_check`, `choice.choose_prepared`, `Capture.start`,
and runner `restore_child_watching`/`EntrySignals.reinstate`/`run_with_output`.
The main outbound trace at depth two returned all 47 reported reachable rows;
the focused searches were untruncated. Exact source and surrounding code were
read for the material paths. Coverage generation `2026-10-15T02:48:14Z` reports
matching metadata and no recorded gaps for all relied-on repository paths;
that is best-effort evidence, not a proof of completeness or runtime behavior.

The executable initializer in `signal_state.rs:52–90` captures ignored and
blocked bits before `main`. Failed action queries are currently treated like
nonignored dispositions; `RECORDED` can still become true. Platform-invalid
numbers must be distinguished from a failure to query a required valid signal.
The child entry record must remain immutable and separate from host repairs.
`choice.rs:61–98` installs selection handlers before evaluator invocation,
retains them on success and drops them before returning selection failure.
Front `run.rs:35–246` owns output/channel/confined cleanup, joins front/runner/
late cancellation and writes exact bytes only after settlement. Its Started
observer discards mask-restoration failure; later restoration does not report
that earlier failed transition. Selection failure takes a separate settlement
path, so success-only joining would be insufficient.

Worker `Capture.start` creates output threads under `cancellation::shielded`.
That helper blocks INT/TERM/HUP before thread creation and restores the calling
mask afterwards, but checks neither operation. Eligibility is conditional on
those calls succeeding. `Capture.collect` waits only until its deadline and
does not join thread handles; the design must account for still-live readers,
without inventing an unbounded join or widening the worker descendant guarantee.
`block_and_check` has unchecked mask/pending/set operations and excludes signals
blocked at entry from pending cancellation. Handler Drop and re-raise mask
restoration discard failures. A replacement must preserve that blocked-signal
distinction while retaining every transition failure and the original error.

Runner `run_with_output` installs handlers and repairs child watching only after
policy evaluation. `restore_child_watching` tests SIG_IGN, not a standalone
SA_NOCLDWAIT condition, and discards repair failure. Its live process-wide latch
is not the executable's captured child state. `EntrySignals.reinstate` sets
ignore/default dispositions before the child mask, but ignores action/set
failures. The repair must precede both worker and managed-child creation, retain
the existing child entry semantics and preserve C4's checked lease/record path.
This read does not establish SA_NOCLDWAIT survival across exec: distinguish a
real executable inherited case from an owner-local post-entry action fixture.

Current compiler is Homebrew rustc 1.99.0, commit
`b940084d7eb6a299eb4bfeb8e34901bc051e7ac4`, host aarch64-apple-darwin.
Its installed std source confirms startup DEFAULT selects SIGPIPE ignore, and
Unix `do_exec` restores SIGPIPE default before invoking pre_exec callbacks while
preserving the calling mask. Read source subjects under that toolchain's
`lib/rustlib/src/rust/library/std/src/`: `sys/pal/unix/mod.rs` SHA-256
`6c706c9a79d6a6ca3870a605800d7a74ba98cf41764c12cd4538a37812bba6aa`, and
`sys/process/unix/unix.rs` SHA-256
`1c0f874f1835e5befa6e8bdd65450903f475248b525d3b7b5e1ce861311a21d0`.
This is fresh local source evidence, not execution qualification. Workspace
minimum Rust is 1.85; declared release targets are Apple arm64 and Linux
arm64/x86_64 GNU. Linux, minimum-toolchain, initializer ordering and actual
signal/wait semantics are unexecuted here. No new supported domain is declared.

## Frozen discriminator obligations and unresolved instrument design

The following expected distinctions stay fixed while implementation and
instrument design remain unfunded. All process/PTY fixtures require deadlines,
bounded kill/wait and terminal recovery; a substituted report never replaces
physical fixture cleanup. Use the public CLI, real TypeScript policy and fake
jobs, existing transport/real-process and runner/PTY seams. No public switches.

| Case family | Required discriminating outcome | Still to design/price |
|---|---|---|
| Executable entry ignores/defaults and masks | Child receives entry INT/TERM/HUP/SIGPIPE state despite startup/host repair; blocked pending signals do not cancel the launcher | Pre-exec controller and child state receipts; before/after broken controls for each state |
| Worker and managed-child waitability | Inherited SIGCHLD ignore and reachable SA_NOCLDWAIT cases produce confirmed waits at both boundaries; failed repair prevents unsafe admission | Determine exec-surviving flags versus post-entry fixture; record real waits separately from reported errors |
| Direct and terminal INT | Launcher-targeted INT cancels even with a terminal; terminal INT after handoff reaches the child; child signal death does not invent launcher cancellation | Actual PID/group/foreground receipts and result-before-child-death control |
| Transition and late arrivals | Pending/front/runner/late observations join on success and failure, including after reap and no-ending failure; ignored/blocked observations retain their meaning | Fixed transition barriers at owner boundaries, no timing-only sleep oracle; test still-live shielded readers |
| Transition errors and failed startup | Original and every secondary report survive; cleanup continues, no result escapes; selected cancellation re-raises or retains numeric fallback | Finite private owner-local reported failures for capture, waitability, shielding, handler install/restore, pending query, child reinstatement and mask restoration; select exact reachable operations after design |
| Protected behavior | Untouched CLI/configuration/results, C1–C4, real-process and PTY expectations remain green with meaningful failing controls | Freeze case manifest and subject digests before/after; immutable per-case receipts, rerun item by item |

Existing Taskfile workflows remain the execution entry point: `test:cli`,
`test:session`, `test:runner`, `test:terminal-recovery`, `test` and `check`.
The producer's price must include source, fixed controls, red/green measurements,
protected checks, evidence, review/coordination and retirement/commit. No such
complete supported price is asserted here. The next funded design/pricing step
must resolve valid-signal filtering, transaction rollback versus retained
handlers, thread-shield failure ownership and selection-failure joining first;
these are precise questions, not permission to build a generic syscall framework.

## Verification and stopping state

Full jj diff review covers only this task, the new k19 funding mandate, root
promotion and the book's filename move. The book's SHA-256 remains
`1ec62bb523ab56e47af33a7a3030e0605bc37924a6ecbfff88338f898b40d28a`
against the parent revision. Both installed std source digests are unchanged
after reading. Resolve and brief-chain identify k19 at the root before the book;
coverage also reports matching metadata/no recorded gaps for the four changed
or moved tree files, and their diff is read directly. Touched text has no
trailing whitespace. No runtime or protected behavior checks are run by this
planning-only session; prior C1–C4 receipts retain their existing scope.

No review chain is cut: this is a precise funding remainder, not a completed
implementation design/decomposition to be adopted. Its unresolved checked
transition questions and review allowance are explicit in k19. No architectural
decision changed, so the ADR set remains current. K19 and the book stay live;
the root cannot close. Retire k18 and seal its one focused jj change.
