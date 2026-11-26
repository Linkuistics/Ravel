# Independent release lifecycle validation — 2026-10-13

This increment completes the lifecycle work left open by
[archive evidence](archive-evidence.md). Its artifact is the Ravel jj commit
named `ravel-release-lifecycle-k25: deliver independent release lifecycle and
recovery controls`, based on k24's sealed
`454ecc817342860b645af06a3c42212fe72baeb9`. The cross-project handoff names the
exact sealed commit. The measured working tree before this evidence-only file
was added was `02613ab8dc1c5f1ea6959e31333d4a704ec063b1`.

No production version cut, tag, push, GitHub/tap publication, permanent install
or model launch ran here. Development remains 0.0.0. Read-only remote rechecks
before and after validation found only `structured-plan-migration`, with no
version tags. The first independent minor remains **0.1.0** in publication k11.
Public GitHub/Homebrew outcomes remain k11's work, rather than claimed fixture
results. No preparation prerequisite or target check is missing.

## Lifecycle instruments

`task release:lifecycle:test` passed using the actual Taskfile, scripts, jj,
local Git remotes, cargo-release 1.1.2 and harmless fixture archives. GitHub
responses/downloads and expensive build boundaries were replaced in fixtures;
there were no public-service writes. The instrument establishes:

- Prerequisite failure and an existing release lock stop the actual tasks before
  a cut; normal failure releases its own lock and preserves another owner's lock.
- Patch/minor/major dry Task execution orders archive build and installed smoke
  before main/tag/GitHub/tap publication. Actual task executions with failed
  smoke stop before every publishing boundary.
- Dirty, detached Git-only and diverged jj taps refuse before GitHub/tap writes.
  Successful local tap publication uses jj exclusively and retains unrelated
  tap work and the separate ravel-lite formula.
- Missing/extra archives or stale rendered formulas refuse before publication.
  Recovery `--tap-only` does not recreate the GitHub release; it verifies exact
  published asset names/state and compares all downloaded archives byte-for-byte.
  Corrupt downloaded contents refuse without moving the tap bookmark.
- First-release preparation accepts the absent v0.0.0 baseline, preserves the
  unrelated working change by identity and generates notes from commit descriptions.
  Existing meaningful notes remain byte-for-byte unchanged. Explicit reviewed
  bodies replace only Unreleased; missing/empty/forbidden-heading inputs and
  missing/duplicate standing headings refuse without writes to the changelog.
- Real cargo-release dry cuts/config select Ravel alone and preview patch,
  minor 0.1.0 and major without changing commits, tags or changelog. The same
  replacement accepts empty Unreleased, refuses missing/duplicate headings and
  fires on a deliberately unanchored pattern matching prose.
- Local-only/unintegrated version tags, draft/missing GitHub assets and an
  already tagged main refuse another bump. A complete prior published fixture
  baseline admits subsequent new work.
- The real doctor refuses denied GitHub authentication and macOS Bash 3.2.
  The live preparation host passes all prerequisites with Bash 5.3.20,
  cargo-release 1.1.2, rustup cargo/rustc 1.93.1, all three target stds,
  checkout-local Bun 1.4.2, Zig 0.17.0, authenticated gh, Homebrew and linux/arm64
  Docker 29.8.1 on kernel 7.0.14-linuxkit. It installs nothing.

The leaf's single adversarial reader found three actionable P2 issues: Bash
admission after cut, incomplete GitHub records counted as completed publication,
and wildcard publication of extras. Each was fixed and covered by executable
fixture controls. There were no unresolved findings or additional review pass.
Chapter 12 of the book documents individual recovery boundaries without another bump.

## Rebuilt archives and installed checks

All current local artifacts are under `target/archives/` and are **unpublished**.
These rebuilds supersede k24's local archive digests, while the runtime identity
and template are unchanged. The template SHA-256 remains
`d37fbaf1e2a98a45b0a4a319eb9109925f6df07a231d8aa8afe387b54a530aa8`.

| Artifact | SHA-256 |
|---|---|
| ravel-v0.0.0-aarch64-apple-darwin.tar.xz | 3a96143765421539d38878acc79675f7f99c39d5f992e5879727c5090ff664a5 |
| ravel-v0.0.0-aarch64-unknown-linux-gnu.tar.xz | ae920d63bf327f89c8bbab58e29b01c4efc47fef57f1c1a5d41cae1ba8a18288 |
| ravel-v0.0.0-x86_64-unknown-linux-gnu.tar.xz | 022df70515f7f817c4a02edc97c324e6e436ef7e8ad2ca4aa35645ac56c9f074 |
| ravel.rb (preparation only) | 5b10f0dceb53b92b44fcafb91b7c581ba937412636289bf1e304bb2402eca489 |

`task release:smoke` rebuilt all three and passed direct/symlink installed seams
without Grove/Bun/Node. macOS arm64 ran natively; Linux arm64 ran in native
CentOS 7.9 Docker and QEMU 10.2.2 Cortex-A53; Linux x86-64 ran in QEMU 10.2.3
Nehalem. Both Linux floors enforced glibc 2.17, refused GLIBC_2.25 with exit 1,
killed above-floor instructions with SIGILL/132, and ran the twin probes with
`-cpu max`. Both archive ELFs matched registration; mappings were bounded with
a vDSO. No target/route was skipped. These do not measure a kernel-version floor.

`task release:test` passed exact-manifest/omission/extra/type/mode/probe and
metadata controls against a fresh production worker. `task release:formula`
passed actual Homebrew install/test methods in private prefixes and all four
substituted worker identity controls. `task check` passed formatting, ShellCheck,
worker build/probes/types, runtime integrity, Clippy and **134 runtime tests**.
Its 14 ignored tests are subprocess fixture roles invoked by acceptance tests,
not missing target checks. The worker is still version 0.0.0 and source identity
`fa130cc262cfb22541798debe937ecd9e81488ebef494d873d377978808c9d9c`.

## Measurement and handoff

All **14,183 individual subjects** had matching before/after SHA-256 values.
The subjects include tracked source/manifests/instruments/docs, worker dependency
files, external Cargo dependency source, Homebrew's actual Ruby library, owner
Ravel/legacy-dispatch configuration, all tap formulas and the resolved original
Grove/grove-llm/harness-dispatch binaries and their libexec trees. Only this
new evidence document was added after the comparison. The tap stayed clean at
main `e7416e57`, with ravel-lite intact and ravel.rb absent. Original installed
commands still report 22.1.0; their bytes and the inherited exit contract remain.

Complete logs/instruments under Ravel `target/`:

- `release-lifecycle.aBzyix`, `k25-handoff-doctor.log`, `k25-handoff-tap.log`;
- `k25-handoff-smoke.log`, `k25-handoff-archives.log`, `release-archives.br0ybt`;
- `k25-handoff-formula.log`, `release-formula.jdIkis`, `worker-check.hneX4v`;
- `check.N5kpCt`, `k25-subjects.py`, `k25-subjects.json`, `k25-subjects-after.json`.

Ravel graph generation 2026-10-12T12:19:50Z is fast and excludes scripts/docs;
coverage calls identify those exclusions and direct source was read. Source
Grove's transferred pipeline had Tier 2 graph discovery/snippet/coverage evidence
at generation 2026-10-12T12:42:39Z. No graph completeness claim is made.

k11 must recheck the sealed source, notes, tags/tap/config and absence of a
partial cut, run `task release:minor`, rebuild fresh tagged 0.1.0 archives and
rerun all target/formula checks **before** publishing. It then records the
immutable URLs/digests and installed Homebrew fake-agent/result checks. Current
0.0.0 files must never be copied into the tap or uploaded as a public release.
