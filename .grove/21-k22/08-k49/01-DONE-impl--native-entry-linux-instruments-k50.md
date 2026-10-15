# native-entry-linux-instruments-k50

## Goal

Make the frozen native-entry matrix, its incumbent baseline and R01 runnable and
judgeable on GNU/Linux (aarch64 native, x86_64 emulated) at the declared minimum
Rust, without changing production behaviour, and take **development**
measurements that say what is true there before any immutable run freezes
subjects. Immutable receipts, the correction of any production finding, and the
native-x86_64 human choice are the node's later children.

## Context

The node brief carries the charter, the Done-when contract and the findings
carried forward. The evidence is in the new section of
`docs/native-entry-development-evidence.md` ("GNU/Linux and minimum-Rust
continuation: development increment") and under
`docs/evidence/native-entry-linux-development/`.

## Done when

- A Linux environment per target exists and is reproducible from the checkout:
  `scripts/native-linux/Dockerfile`, `scripts/native-linux.sh` (image, up, sync,
  exec, pull, campaign, down), `scripts/native-linux-qemu.sh` and its interpreter
  and pivot helpers for the emulated target, `scripts/native-linux/measure.sh`,
  and Taskfile entries `native-linux:image`, `test:native-linux` and
  `test:native-r01-controls`.
- The driver and plan carry the platform dimension and the emulation accounting,
  each guarantee seen to reject its wrong input (`native-campaign.sh selftest`,
  26 controls; `native-r01.sh selftest`, 7), with the 153 identities and the
  327 expected records unchanged and Darwin's expectations untouched.
- Every instrument edit Linux needed is a measured Darwin-premise replacement,
  and the Darwin development campaign with the edited instruments reproduces
  k43's immutable receipts item by item (0 of 153 differ).
- Development measurements exist for aarch64 Linux at rustc 1.85.0 (153 of 153
  reconciled, plain suite 338 passed, R01 joined identity by identity) and for
  x86_64 under emulation (146 reconciled, 7 limited, 74 protected identities
  that cannot run), and the findings that block the immutable run are recorded
  and cut as leaves.
- No production source changed.

## Notes

Production behaviour is unchanged: every edit is in a script, instrument,
overlay, plan, document or Taskfile. The four production-source lint sites are
not edited here, by the charter. The spec and design status statements were
edited to say only what is measured (immutable receipts on Darwin only; the
continuation open).

## Decisions (running log)

- **Rust 1.85 on the host first.** `rustup run 1.85` (rustc/cargo 1.85.1; the
  `cargo` on PATH is Homebrew's and does not parse `+1.85`) builds the workspace
  with `--all-targets` with no warning, `clippy --all-targets -D warnings` and
  `cargo fmt --check` are clean, and the isolated `--cfg test`/`native-faults`
  fixture builds. Clippy and rustfmt were added to that existing toolchain. The
  exact floor, 1.85.0, is the Linux image's toolchain (below), so the Linux
  campaign measures the declared minimum itself rather than a patch release.
- **The Linux subject environment is a Docker image per platform**
  (`scripts/native-linux/Dockerfile`, launcher `scripts/native-linux.sh`): Ubuntu
  26.04.1 pinned by digest, rustc 1.85.0 from rustup, cargo-nextest 0.9.148 and
  go-task 3.54.0 at the Darwin versions, bubblewrap. Each tool's SHA-256 is in
  `/opt/native-linux/tools.txt` and in `domain.json`. Not reused from the release
  smoke: its CentOS 7.9 userland is glibc 2.17 for archives, not a build
  environment; its Docker-VM facts (kernel >= 6.7, seccomp finding) are reused.
- **Four container settings were each measured, not assumed**, and each is a
  property of running the product's sandbox in a container rather than of the
  product: `seccomp=unconfined` (Docker's profile refuses `unshare(2)`; 89
  protected failures with it, bwrap `Can't mount proc` without the second);
  `systempaths=unconfined` (Docker masks parts of `/proc`, which the kernel refuses
  under a new procfs in a user namespace; 18 failures remained without it);
  `--init` (a `sleep` PID 1 never reaps reparented orphans, so every group-absence
  check saw zombies as present; 8 failures); non-root uid 1000, as release-smoke's
  floor container does. With all four the plain suite is 338 passed, 46 skipped,
  the same 384 identities as k43's Darwin run. `hostile` also needs `task probes`
  (as `task check` runs it); that is an existing prerequisite, not a Linux fact.
- **Three Darwin premises do not exist on this kernel (7.0.14-linuxkit), each
  repaired in an instrument and none in production code.** (1) E03's ignored-
  SIGCHLD row assumed skipping the worker repair is harmless because Darwin's exec
  clears the no-child-wait state; on Linux an ignored SIGCHLD is itself the
  no-zombie state and exec keeps SIG_IGN (flush_signal_handlers, v6.18, source
  read and measured), so the skip-repair subject fails ECHILD and the row is a
  discriminator of its own. (2) E04's raw executable-entry row now asserts the
  flag is clear after a read-back-confirmed pre-exec injection, so "cleared" can
  only mean exec cleared it. (3) The incumbent S03 driver's "group of unreaped
  zombies answers EPERM" is false here (measured: success for the probe and for
  SIGKILL, ESRCH only after reap), so it reads live members from /proc on Linux.
- **The plan gains a platform dimension, not a second plan**: `platforms.<name>
  .rows` replaces a base expectation for an exact row with its reason, selected
  by the kernel the driver runs on; the 153 identities and 327 expected records
  are unchanged, and the driver's selftest shows the two plans differ on exactly
  the two overridden rows and that an override is applied only on its platform.
- **The 1.85 lint step fails, and my first reading of it was wrong.** The host
  check "clippy clean at 1.85" ran Homebrew's clippy 0.1.99 against rustc
  1.85.1 (`rustup run` put the toolchain's `cargo` first but `cargo clippy`
  resolved the subcommand elsewhere). Naming the genuine driver
  (`.../toolchains/1.85-*/bin/cargo-clippy`, 0.1.85) reports four errors, which
  `task check` surfaced in the container at 1.85.0 as its lint step failing
  (exit 201, 0 of 290 identities through `check`). Recorded as a finding, not
  repaired: three sites are production source and one a protected test, and the
  charter says a production repair earns a failing-first correction run.
- **x86_64 runs under the release smoke's own mechanism, not Docker's default
  emulation.** Docker Desktop's QEMU maps a guest above 2^47 and rustc faults in
  its allocator before doing any work; the repo already documents the cause and
  the fix (guest base 2^47, private binfmt_misc, the pinned QEMU), which the
  emulated environment reuses, with the guest as the namespace's real root.
- **The emulator cannot run the product's sandbox, and says why**: every
  emulated process has a second host thread, so `unshare(CLONE_NEWUSER)` is
  EINVAL for all of them. That is a property of the instrument's substrate, so
  the 74 protected identities that need a namespace are reported separately and
  the matrix rows the emulator cannot reproduce are *limited with a reason and a
  control*, never reconciled and never dropped: 146 reconciled, 7 limited
  (E04 x3 on the emulator's missing SA_NOCLDWAIT, shown by a probe built for both
  targets; E07 x4, cause not isolated and said so).
- **The leaf decomposes.** The correction is a production edit and invalidates
  every digest the immutable run freezes, so it must precede that run; this
  session therefore delivers the instrument increment, records the finding and
  cuts the correction, the run and the human choice as their own leaves. The
  documents' status statements are edited to the truth of this commit and are to
  be finalised by the run leaf before it freezes.
- **Left behind on purpose.** The old ignored build directories under `target/`
  (`rust185*`, `k49-darwin-dev`, `native-linux-amd64-inputs`, the dev containers
  and images) were not removed by this session; a removal was refused by the
  environment's safety check and none was needed. They are gitignored.
