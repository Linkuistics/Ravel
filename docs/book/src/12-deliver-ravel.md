# Deliver Ravel

A release carries a matched [Ravel CLI and policy worker](07-the-parts-of-ravel.md#the-installed-pair-stays-together).
An archive check first establishes that this pair has the right files and can
run outside the checkout. A version number alone establishes neither fact.

Start with [chapter 11's install and smoke tasks](11-develop-ravel.md#install-and-test-the-installed-pair).
Those tasks test a local installation. This chapter extends that check to
archives and the environments where their programs run.

## Keep the version and pair together

The workspace has one Ravel version, independent of the version of any
[caller](02-actors-and-words.md#caller-and-owner).
`Cargo.toml` supplies it to both Rust packages, and the worker build reads the
Ravel package version. The checkout used for this chapter has version 1.0.0.
Its local tags include `v0.1.0` and `v1.0.0`, and `CHANGELOG.md` describes both.
The development version is no longer 0.0.0.

An unreleased archive uses the checkout's version even when its source differs
from that version's release tag. Thus, a local archive named with `v1.0.0`
can contain later changes. Its name does not make it the released artifact.

The CLI checks three worker values before it sends the policy for evaluation:

| Value | What it establishes |
|---|---|
| Protocol number | Both programs use protocol 1 |
| Package version | Both programs report the same Ravel version |
| Source identity | Both builds used the same worker source inputs |

The source identity is a SHA-256 digest. Relative to `crates/ravel`, its inputs
are the TypeScript files under `worker/src`, `worker/sdk` and `worker/examples`,
both worker tsconfigs, and `scripts/dispatch.sh`.
Both builds sort those paths, hash each file, then hash the
same list of paths and digests.

This identity pairs builds. It does not authenticate an installation or cover
every Rust source file. A worker with a different protocol, version or identity
causes `worker_identity_mismatch`, with exit status 5, before policy evaluation.
A test probe also has a different identity. Rebuild and install the pair
together to correct a mismatch.

The CLI finds `libexec/ravel/ravel-policy` relative to its own canonical path.
A symlink to the CLI therefore still reaches that CLI's worker. Another worker
on `PATH` cannot replace it. The worker contains Bun 1.4.2, so an installation
needs no separate Bun or Node runtime.

## Build archives before a release

`task release:archives` builds local archives without a version cut. With no
`TARGETS` argument, it builds these targets:

| Target | Archive contents run on |
|---|---|
| `aarch64-apple-darwin` | Apple silicon macOS |
| `aarch64-unknown-linux-gnu` | Arm64 GNU/Linux |
| `x86_64-unknown-linux-gnu` | x86-64 GNU/Linux |

Run the tasks from the checkout. The full workflow needs an Apple silicon Mac,
Bash 4 or later, Task, rustup with Rust 1.85 or later, curl, tar/xz and ShellCheck.
The Linux builds also need Zig, cargo-zigbuild and both Linux Rust target libraries.
The Linux smoke tests need an available `linux/arm64` Docker daemon with kernel
6.7 or later. That kernel requirement belongs to the test environment's private
emulator registration, not Ravel's minimum kernel.

`task setup` supplies the checkout's pinned Bun and development dependencies.
The release build prefers rustup's tools on `PATH` when its shims exist.
An explicit `RUSTC` override can still select another compiler. The release
doctor checks the compiler that the build will use and its target libraries.
The worker build verifies each target's Bun
runtime against a committed SHA-256 digest before compilation. It also generates
the SDK declarations.

Fragment: prepare the target libraries and build local archives. This recipe
omits tool installation and the command output.

```sh
rustup target add aarch64-apple-darwin aarch64-unknown-linux-gnu x86_64-unknown-linux-gnu
task setup
task release:archives
```

The build writes `target/archives/ravel-v<VERSION>-<TARGET>.tar.xz`.
It clears `target/archives` before each build, even for one target.
A previous archive therefore cannot fill a gap in a failed build. Each archive
first has a temporary filename. The build exposes its final filename only
after the manifest check passes, then prints its SHA-256 digest.

For a local macOS check, select only the native target. This selection replaces
the archive directory's contents and does not check Linux.

Fragment: build only the macOS archive. This recipe omits setup and output.

```sh
task release:archives TARGETS=aarch64-apple-darwin
```

If an unreleased build fails, correct the reported cause and repeat the task.
It creates no release tag or remote release. The later release procedure builds
fresh artifacts from the release tag into `target/dist`.

## Check the archive contents

An archive has one top directory, `ravel-v<VERSION>-<TARGET>/`. That directory
is an installation prefix. The manifest in `scripts/release-common.sh` permits
exactly these files and their directories:

| Path below the prefix | Purpose |
|---|---|
| `bin/ravel` | Ravel CLI executable |
| `libexec/ravel/ravel-policy` | Matched policy worker executable |
| `libexec/ravel/sdk/index.ts`, `sdk/index.d.ts` | SDK source and declarations, both under `libexec/ravel` |
| `libexec/ravel/examples/static.ts`, `examples/static.d.ts` | Static example source and declarations, both under `libexec/ravel` |
| `libexec/ravel/examples/sample.ts` | Sample that an owner can copy and edit |
| `libexec/ravel/notices/NOTICES.md`, `notices/bun-LICENSE.md` | Runtime notices, both under `libexec/ravel` |
| `README.md`, `LICENSE` | Project entry and licence |

[Chapter 6's installed-layout view](06-reference.md#installed-layout) shows
how the CLI, worker, SDK and examples fit together. In the checkout, the runtime
notices are under `crates/ravel/notices/`. In an installation, they are under
`PREFIX/libexec/ravel/notices/`.

The manifest rejects extra files and directories, missing files, symlinks in
place of files, and executables without execute permission. Caller programs,
caller adapters, owner configuration, choice files, development dependencies
and probe directories do not belong in an archive. The pack step also excludes
macOS extended attributes and their AppleDouble metadata files.

`task release:test` tests this validator. It compares the manifest with a fresh
production worker build, then uses deliberately incorrect archives. Each omitted
file causes a failure. Other controls add unwanted assets, change types or
permissions, and add macOS metadata. The task needs no Docker, but its metadata
controls use macOS tools.

Fragment: test the archive validator. This recipe omits setup and output.

```sh
task release:test
```

A manifest checks layout, types and execute permission. It does not prove that
the file at the worker path is the right program. The installed smoke test checks
the pair through a real policy. The hostile-runtime suite separately substitutes
probe workers at that path and checks for identity refusal. Those tests need
`task probes`, which `task check` builds.

## Test the installed archive

`task release:smoke` rebuilds the selected archives, then tests them. With no
selection, every target runs. A missing tool, failed target or unrun selected
target fails the task. There is no successful skip path.

Fragment: rebuild and test all target archives. This recipe omits setup and
per-target output.

```sh
task release:smoke
```

Each environment extracts its archive with its own tar into a fresh prefix.
It checks the CLI version and runs `crates/ravel/scripts/installed-smoke.sh`.
That script tests both the direct CLI path and a relative symlink from another
prefix. Its private utility `PATH` has no Bun, Node or Grove. It supplies a
fresh home and temporary directory through a cleared environment.

The test uses a shell fake agent, a real policy worker and installed assets:

| Check | What the test compares |
|---|---|
| SDK, static example and copied sample | The sample uses the selected choice and starts the fake agent |
| Result values | Ordinary, empty, multiline and option-like values return unchanged |
| Child output | Child standard output goes to diagnostics, separate from the result |
| TypeScript imports | A relative helper and a declared package load from the test policy |
| Policy inputs | Measured context and command parameters reach the policy |
| Missing session-result context | `end-session` refuses and writes no result |
| Temporary resources | The sample sessions leave no resources in their temporary directory |

The result is the [opaque result](02-actors-and-words.md#the-managed-process-and-its-result)
that the fake agent sends with `end-session`. No model or personal policy runs.
The test does not exercise an interactive terminal or all lifecycle failures.
[Chapter 9](09-jobs-signals-and-the-terminal.md) explains those separate obligations.

## Read the platform checks correctly

A platform floor is the oldest library or CPU capability that a check exercises.
The Linux CLI builds against glibc 2.17 through Zig. The worker uses its pinned
Bun runtime. Both executables then run in the same archive smoke test.

| Route | Environment and check |
|---|---|
| macOS arm64 | Native execution on the Mac that runs the task |
| Linux arm64, native CPU | Pinned CentOS 7.9 userland in arm64 Docker, with glibc 2.17 |
| Linux arm64, CPU floor | The same userland under QEMU 10.2.2 with the Cortex-A53 model |
| Linux x86-64, CPU floor | CentOS 7.9 under QEMU 10.2.3 with the Nehalem model |

Each Linux route checks that the userland reports glibc 2.17. A probe built for
that floor runs. A second probe needs `GLIBC_2.25`, and the userland refuses it.
The pair shows that the environment both runs valid programs and rejects a
newer library requirement.

The CPU controls use an instruction above each floor: an Armv8.1 LSE atomic
operation for arm64, and AVX2 for x86-64. At the floor, the instruction causes
SIGILL. The same probe runs under the same emulator with `-cpu max`. Thus, the
failure at the floor does not come from an invalid probe alone.

The emulated routes check every archive ELF executable against their emulator
registration. This check covers the CLI and worker. They also check guest address bounds
and the presence of the vDSO, a kernel-supplied interface mapped into a process.
The test programs run without network access, as an unprivileged user, with
read-only test inputs.

These checks establish different limits:

| Observation | Limit |
|---|---|
| Native macOS smoke passed | It checks the host macOS version, not an older macOS floor |
| glibc and CPU controls passed | They check these userlands and CPU models, not every distribution or processor |
| Linux container or user-mode QEMU passed | It uses the host kernel and does not measure a minimum kernel version |
| Archive smoke passed | It checks the installed workflow, not the full process and terminal campaign |

The native-entry campaign measured aarch64 macOS and aarch64 Linux.
x86_64 Linux is built and run only under emulation. Native x86-64 Linux remains
unqualified, with qualification tracked in [Linkuistics/Ravel#1](https://github.com/Linkuistics/Ravel/issues/1).
An archive smoke pass does not change that support statement.

[Chapter 11's evidence map](11-develop-ravel.md#read-the-evidence-for-a-change)
locates the separate runtime, archive and native-entry records. Those records
name their source revisions and measured environments. Rebuild and repeat the
relevant checks after a source or runtime change.

## Checks made for this chapter

The draft checks use macOS 27.0.1 arm64, Bash 5.3.20 and rustup rustc 1.93.1.
All three Rust target libraries are present. Zig 0.17.0, cargo-zigbuild and
Docker 29.8.1 are available. Docker reports `linux/arm64` with kernel
7.0.14-linuxkit. No archive-test prerequisite is missing.

The following checks passed on 2026-10-18:

| Task | Observed result |
|---|---|
| `release:doctor` | All reported prerequisites passed |
| `release:test` | Manifest and deliberately incorrect archive controls passed |
| `release:smoke` | Fresh archives passed all four routes above, direct and through a symlink |
| `release:lifecycle:test` | Release procedure, recovery, notes and book-version controls passed in isolated fixtures |

The smoke task ran `release:archives` as its build step. Both Linux CPU controls
failed at their floor and passed with `-cpu max`. Every Linux route passed the
glibc probe and refused the probe that needs glibc 2.25. No target was skipped.
The source and script digests stayed unchanged through these checks.

This draft did not run `release:formula`, `release:notes`, `release:patch`,
`release:minor` or `release:major` against the project. Formula preparation and
release-note changes belong to the next part of this chapter. The version-cut
tasks change public release state and the local installation. The lifecycle
test exercised its fixture cuts without a project release, push or tag.

The draft did not repeat `install` or `smoke`. Chapter 11 records those checks,
and this draft exercised the same installed test through fresh archives.
These observations establish no new native x86-64, interactive-terminal,
real-agent or reader-comprehension result.
