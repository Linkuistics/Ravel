# Independent archive validation — 2026-10-12

This is Ravel's own unreleased delivery evidence, collected on macOS 27.0.1
arm64 with Docker 29.8.1 (linux/arm64, kernel 7.0.14-linuxkit), Zig 0.17.0 and
Homebrew 7.0.9-64-gbe45bb1. It supersedes no source-project measurements.

The artifact commit is named `ravel-archives-k24: deliver independent archives,
floor controls and tested formula`, based on configured Ravel source
`51b73cbffaa46882d7f9ebad15058e17dbd7bec4`. Its production inputs were frozen
before the handoff run and individually compared afterward. This evidence
document was added after that comparison; no measured input changed afterward.
The exact sealed commit is also recorded in the cross-project k24 handoff.
The worker's source identity is
`fa130cc262cfb22541798debe937ecd9e81488ebef494d873d377978808c9d9c`, version 0.0.0.

## Local artifacts

These are local unreleased archives under
`/Users/antony/Development/Ravel/target/archives`. They are not published URLs.

| Archive | SHA-256 |
|---|---|
| ravel-v0.0.0-aarch64-apple-darwin.tar.xz | 591e93826b249744094cce3ac3903a4f7115a79941b3e5fca15b7fc89894683b |
| ravel-v0.0.0-aarch64-unknown-linux-gnu.tar.xz | 85d1968e8743acc257a4324b2b1ec2bf4d9279002abb36ce941cc7564c5ed9a9 |
| ravel-v0.0.0-x86_64-unknown-linux-gnu.tar.xz | c4341611ee2264d889b728c88deb2f3a2f85967c20ddacfae10e30643d54df32 |
| ravel.rb (unpublished preparation formula) | cfe1d980757c1601ec1a0ac82afcbb30f37abb40f26cdade1c8fe284c4cbdc1a |

Each archive passed exact file/directory/type/executable manifest validation.
The template SHA-256 is
`d37fbaf1e2a98a45b0a4a319eb9109925f6df07a231d8aa8afe387b54a530aa8`.
No caller binary, adapter, policy, development runtime/dependency or probe ships.

## Target executions

| Target/route | Installed layout | Firing controls |
|---|---|---|
| macOS arm64 native | Direct and relative symlink passed using bsdtar 3.5.3 | Matching worker routes and returns exact opaque results |
| Linux arm64 native Docker, CentOS 7.9 | Direct and relative symlink passed using GNU tar 1.26 | glibc 2.17 probe ran; GLIBC_2.25 probe refused with exit 1 |
| Linux arm64, QEMU 10.2.2 Cortex-A53 | Direct and relative symlink passed using GNU tar 1.26 | Both archive ELFs match registration; guest below 2^48 with vDSO; LSE probe SIGILL/132, twin -cpu max runs; glibc pair fires |
| Linux x86-64, QEMU 10.2.3 Nehalem | Direct and relative symlink passed using GNU tar 1.26 | Both archive ELFs match registration; guest below 2^47 with vDSO; AVX2 probe SIGILL/132, twin -cpu max runs; glibc pair fires |

Every route executes the production installed-layout seam on a private utility
PATH with no Bun, Node or Grove. It covers generic sample/SDK/static/choice,
ordinary/empty/multiline/flag-like results, child-output separation, typed helper
and declared-package imports, measured context, params and stale context refusal.
No target was skipped and no target prerequisite was missing. Containers and
user-mode emulation do not establish a kernel floor.

## Controls and full checks

The transferred archive instrument passed omission controls for every manifest
file, extra runtime/cache/probe and caller assets, incorrect executable mode,
symlink worker, stray top directory and AppleDouble/xattr controls. It compared
the manifest with a fresh production worker build and with formula installation.

The actual Homebrew formula install/test methods passed from a private prefix
with spaces/Unicode. The installed front reached through a symlink imported the
SDK/static example, launched only a shell fixture, returned ordinary/empty/
multiline/flag-like values exactly and left no session resources. All four
probe workers substituted at this archive-derived installation's worker path
refused with worker_identity_mismatch/exit 5. The production worker's successful
formula test supplied the matching-path firing control before substitution.
This is preparation validation; `brew test ravel` after an actual published
installation remains a separate delivery check.

`task check` passed formatting, ShellCheck, worker/probe builds, offline runtime
integrity controls, TypeScript checks, Clippy and nextest: **134 tests passed**.
Nextest reports 14 ignored subprocess roles in job.rs, noninteractive.rs and
session_process.rs; the real acceptance tests invoke those roles explicitly.
They are not skipped target checks or missing prerequisites.

All **14,171** recorded subjects retained matching individual SHA-256 values:
tracked Ravel source/config/docs/instruments, worker development dependencies,
Cargo dependency source files, the installed Homebrew Ruby API, owner config,
tap formulas and original installed Grove/dispatch binaries/libexec assets.
The subject map/instrument and outputs are saved under target/ below. Archive
digests also agree individually with the ones embedded in the rendered formula.
The tap remains clean at e7416e57, ravel-lite.rb is intact and ravel.rb is absent.
The owner policy digests remain 7ca627d265fb8b3d81a1f7f710aa42298eac7aa6369fec92e126f608efd3196e
(Ravel) and 59a54bfae00295a887ab87283e7ba0a794c456a437fb142ff19a6a3d87e2275d
(legacy dispatch). No public cut/tag/push/publication or permanent installation
occurred, and the original Grove/dispatch pair was preserved.

## Reproduce and hand off

Run the existing root tasks in this order:

```sh
task release:smoke
task release:test
task release:formula
task check
```

Complete logs in `/Users/antony/Development/Ravel/target/`:

- `k24-handoff-smoke.log`: build/digests and individual target/floor records.
- `k24-handoff-archives.log`, `release-archives.7aMcNf`: manifest controls.
- `k24-handoff-formula.log`, `release-formula.zJHaNX`: actual formula and probe controls.
- `worker-check.eyljnw`: required native probe build.
- `check.c8wwOj`: full relevant workspace checks and nextest report.
- `k24-subjects.py`, `k24-subjects.json`, `k24-subjects-after.json`: individual subject comparison.

Local artifacts/logs can be removed and reproduced; this document preserves
the outcomes and boundaries. Release-lifecycle preparation still must deliver
version-cut/notes/preflight/tap/publication tasks, dry-cut tests and complete
partial-release recovery. Remote tag reinspection found only
structured-plan-migration, so the planned first minor remains 0.0.0 to 0.1.0.
No version-cut or publication command has been validated by this increment.
Recheck tags/source/tap state, then rebuild tagged-source artifacts and rerun
all target/formula checks before any release is published.
