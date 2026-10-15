# native-entry-linux-qualification-k49 — brief

## Goal

Qualify the repaired native-entry-to-settlement launcher on the declared GNU
Linux targets and at the declared minimum Rust 1.85, with the same frozen
153-row matrix, the same incumbent baseline and the same 310 protected
identities that k43 measured on aarch64-apple-darwin. k43 qualified one host
and one compiler; this leaf is the support remainder it was required to cut.

## Context

Read the k43 section of `docs/native-entry-development-evidence.md` ("Frozen
campaign and R01"), the k38 and k31 briefs, the Linux signal-domain paragraph
and the Linux v6.18 exec source note in `docs/native-entry-transition-design.md`,
`docs/RELEASING.md` (what the Linux Docker/QEMU smoke infrastructure already
establishes and what it explicitly does not), and `scripts/native-campaign.sh`
and `scripts/native-incumbent.sh` as k43 froze them. Use the driver as built; an
instrument edit that Linux genuinely needs (a Darwin-only assumption, a libc
constant, a PTY primitive) is the work of this leaf and invalidates the rows it
touches, so rerun them and never reason about them.

What is known to differ and must be checked against the target headers and the
running kernel, not remembered: the handled-signal domain (1 to 64 excluding
glibc-reserved 32 and 33 on GNU Linux), `exec` clearing action flags so the
private SA_NOCLDWAIT case is a post-entry fixture rather than an inherited flag,
`EPERM` for a group of unreaped zombies, `waitid` with WNOWAIT, controlling-
terminal and foreground-group behaviour of a PTY, and what the guest kernel
reports for pending and blocked sets. Record the measured kernel, glibc and
toolchain for each target the way `domain.json` does.

The existing Docker/QEMU archive smoke runs release archives, not this matrix,
and `docs/RELEASING.md` states that kernel floors are not measured by containers
or user-mode emulation. Reuse its pinned userlands and registration where they
fit; they do not by themselves qualify anything here.

## Done when

- Rust 1.85 itself builds and lints the workspace (`cargo +1.85 build`, test
  binaries and `clippy --all-targets`), checked against the toolchain the
  manifest declares (`rust-version = "1.85"`), with the compile errors or
  warnings it reveals listed. Any API newer than 1.85 that the repaired code or
  its instruments use is a finding to repair, with the source cited.
- The frozen matrix runs on aarch64-unknown-linux-gnu under a real Linux kernel
  (the arm64 Docker VM the release smoke already requires, kernel 6.7 or later)
  as repaired, broken and incumbent subjects, with digests before and after,
  one writer per receipt, and every row reconciled item by item. Rows whose
  Darwin premise does not exist on Linux (for example the raw executable-entry
  SA_NOCLDWAIT observation) are classified with a reason and a control, never
  dropped, and the 153-row identity set and the S01 to S05 counts 8/7/8/7/2 stay.
- The same matrix runs for x86_64-unknown-linux-gnu. Under user-mode emulation
  it is recorded as **emulated**: it can show that the code and instruments
  build and behave under that emulator, and it cannot establish the native
  kernel's signal, process-group or PTY semantics. Cut the native x86_64
  remainder as its own precise `requirements` leaf, naming the human choice
  (provide an x86_64 Linux host the campaign can use, or accept emulation as the
  stated limit) with a recommendation and the evidence for it. Do not narrow the
  matrix or call emulation qualification.
- R01's 310 identities and ignored flags are preserved on each Linux target
  through test:cli, test:session, test:runner, test:terminal-recovery, test and
  check, joined identity by identity as k43 did, with the join seen to fail on
  its wrong inputs. Failures and any new identities are reported separately.
- The completion spec's closing status paragraph and the evidence note are
  reconciled with what is now measured (k43 left the spec unedited because it is
  a digested subject), the support limits are stated per target, and the
  Darwin receipts stay immutable and untouched.
- Actual charges and balances are recorded in the brief chain.

## Notes

Pricing (a projection, not an allocation): P<=45/E<=60/RC<=4, D=X=0. The basis
is k44 to k46 (about P 22 each to host and classify a baseline family) for the
Linux port of the seams and instruments, k43's measured campaign and six
Taskfile runs for the execution (roughly nine minutes of wall time per target
plus container build and reruns), and k40's controller deadlines (30 s case, 5 s
acknowledgement) which a slower emulated target may force to be re-priced rather
than widened silently. Funding: k41's standing human direction ("I'm not
actually concerned about the budget - just do what is required.") removes
budget caps as a gate for required work, so no funding-question leaf is cut;
cumulative bounds after k43 are D<=115/RC<=102/P<=510/E<=187/X=0 against the
historical caps D<=135/RC<=88/P<=467/E<=177/X=0, kept as bookkeeping with the
D<=20 phase-closure reserve and zero production disruption intact.

Production behaviour must not change for any non-test build. If a Linux finding
needs a production repair, it earns its own correction run, with the failing
broken subject first, rather than a quiet edit inside qualification.

Still live and outside this leaf: returning-host contract, independent adoption,
K4 (reference 0bdcb1f9ae24, one iteration of seven attempts, R=1/8, F=4/4, N=3/6,
D<=45/RC<=15) and k6 (D<=45/RC<=10) history, G2 six-month cost, G3 delivery and
use beyond entry, G4 unread alternatives, G5 target execution, R-W runtime and
lifecycle debt, and every root obligation. A pass here closes none of them and
does not advance the book.

## Decomposition

The first session ported the instruments, driver, plan and Linux environments,
took development measurements, and found that the repaired product's own lint
step fails at the declared minimum Rust on every platform (four sites, three in
production source). The charter requires a production repair to be its own
failing-first correction run, and a production edit invalidates every digest an
immutable run freezes, so the immutable runs must follow it. The leaf therefore
became this node. The children, in order:

1. `native-entry-linux-instruments-k50` (done): environments, ports, plan/driver
   platform and emulation accounting, development measurements, findings.
2. `native-entry-minimum-rust-lint-correction`: the failing-first correction of
   the four lint sites, with the behaviour-preservation evidence the charter
   asks for.
3. `native-entry-linux-campaign-run`: the immutable measurement on the corrected
   tree for aarch64 Linux (native, Rust 1.85.0) and x86_64 Linux (emulated),
   R01 on each through the six Taskfile tasks, the documents reconciled before
   the freeze, the evidence note, and the actual charges.

`native-entry-x86-64-linux-scope` (requirements, a sibling after this node)
carries the human choice the charter names for native x86_64.

## Findings carried into the remaining children

- **Declared-minimum lint (k50).** Clippy 0.1.85 reports four deny-level errors
  on the unmodified tree, on Darwin and on Linux alike: `clippy::precedence` at
  `crates/keyed-launch/src/run.rs:534` and `clippy::format_collect` at
  `crates/ravel/build.rs:72`, `crates/ravel/src/context.rs:723` and
  `crates/ravel/tests/cli.rs:451` (a protected test file). Clippy 0.1.86 is clean.
  A scratch copy with the four sites rewritten is clean at 1.85 for
  `--workspace --all-targets -D warnings` and for `native-faults`; the patch is
  kept unapplied in `docs/evidence/native-entry-linux-development/minimum-rust-clippy/`.
  A lint check at a pinned toolchain must name its clippy version: `rustup run 1.85
  cargo clippy` on the Darwin host resolved to Homebrew's clippy 0.1.99.
- **Emulation (k50).** x86_64 runs under QEMU user-mode with the guest base the
  release smoke documents. The emulator cannot create user namespaces (a second
  host thread per process, so `unshare(CLONE_NEWUSER)` is EINVAL) and does not
  honour SA_NOCLDWAIT. Under it 146 of 153 rows reconcile and 7 are
  emulation-limited with reasons; 74 of the plain suite's 338 identities need a
  namespace and fail. It cannot qualify an x86-64 kernel.
- **Environment (k50).** Docker needs `seccomp=unconfined`, `systempaths=
  unconfined`, `--init` and a non-root user for the product's sandbox and group
  checks to be meaningful; each is measured in the evidence note.
- **Digested documents.** The completion spec and the transition design carry
  status statements that are subjects of the campaign. Edit them before the
  freeze, never after, and state only what is measured.

## Outcome (k52)

The third child ran. After a first aarch64 run failed on the incumbent driver's
worker-identity guard (k51's `hex` rewrite in `crates/ravel/build.rs`, the one
difference, now pinned by the SHA-256 of its diff with controls), the three
targets were measured from one tree: aarch64-apple-darwin 153 of 153 and equal to
k43 receipt by receipt; aarch64-unknown-linux-gnu at Rust 1.85.0 natively 153 of
153 (two platform rows), R01 310 of 310 through six tasks including `check`;
x86_64-unknown-linux-gnu **emulated** 146 reconciled, 7 limited with reason and
control, 74 protected identities that cannot run. Support is stated per target in
the evidence note's k52 section; native x86_64 stays open in k53. The Done-when
items are met: Rust 1.85 builds and lints, the matrix runs per target with the
Linux-premise rows classified, the x86_64 remainder is its own requirements leaf
(no narrowing, emulation not called qualification), R01 is joined per target, the
spec and design are reconciled (before the freeze), and charges are recorded. The
node's three children are terminal.

