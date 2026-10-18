# Ravel 0.1.0 publication and installation — 2026-10-13

The release source is `bbab823803bd2f2796f8bd9570f732dce900efa2`, the
peeled `v0.1.0` tag on Linkuistics/Ravel. The annotated tag object is
`7a1415fdfb5d9cc87e1e0801d81f8ced43756a06`. GitHub's non-draft release and
the Homebrew formula are published. The owner explicitly approved making the
repository public, including its full history and retired branches. All three
anonymous downloads now succeed and match the validated archives exactly.
GitHub reports `immutable: false`; the version-specific URLs below pin identity
by their SHA-256 values rather than a repository-enforced release lock.

## Published artifacts

All three authenticated and anonymous GitHub downloads were compared byte-for-byte with
the fresh tagged archives after every installed smoke route passed. GitHub's
asset digests match the local SHA-256 values. Published release notes match
the versioned CHANGELOG.md section exactly.

| Target | Version-specific archive URL | SHA-256 |
|---|---|---|
| macOS arm64 | https://github.com/Linkuistics/Ravel/releases/download/v0.1.0/ravel-v0.1.0-aarch64-apple-darwin.tar.xz | `7e33ae0c56a93e6a5cff256c5a8a981dec4eb4d114c1f636ce8ee23d1f5d1059` |
| Linux arm64 | https://github.com/Linkuistics/Ravel/releases/download/v0.1.0/ravel-v0.1.0-aarch64-unknown-linux-gnu.tar.xz | `19a20dc895a09440cbe3d8da1ce80b96311fbc1c3e6be5e2bf1eb18bb6cc0f7e` |
| Linux x86-64 | https://github.com/Linkuistics/Ravel/releases/download/v0.1.0/ravel-v0.1.0-x86_64-unknown-linux-gnu.tar.xz | `b9e3443dc428923d6ab45dff7d3837def16bd36985ddd5900f75489a7a4b96e8` |

## Release and recovery

The validated `task release:minor` cut 0.0.0 → 0.1.0 after doctor/tap
preflight and all 134 runtime tests passed. Tagged archive manifests, the
actual Homebrew formula methods and all four substituted worker identity
controls passed. macOS ran natively; Linux arm64 ran in native CentOS 7.9
Docker and QEMU 10.2.2 Cortex-A53; Linux x86-64 ran in QEMU 10.2.3 Nehalem.
Every direct/symlink installed route passed. The Linux glibc 2.17 refusal and
CPU SIGILL/132 controls fired, with twin `-cpu max` controls passing. These
environments do not measure a kernel-version floor.

The task stopped at its first push because Ravel's existing `main@origin`
bookmark was untracked. After confirming that state, tracking it and resuming
only the main/tag pushes and publisher succeeded. No source fix or second
version bump ran. Installation first failed on the private repository's 404;
the exact verified archive was then placed in Homebrew's download cache.
`brew install linkuistics/taps/ravel` and `brew test ravel` passed from that
cache, with automatic cleanup disabled. After the approved visibility change,
all three anonymous curl downloads matched the tested bytes. Homebrew's
`brew fetch --force linkuistics/taps/ravel` independently downloaded the public
macOS archive, and a fresh `brew test ravel` passed. No second cut or publication
was needed.

The tap is sealed at `bd5aa37b721aaff4a72b01fb966785392812b6a8`; its only
change is `Formula/ravel.rb`. Formula SHA-256 is
`e30d1eddf1a138db56d0b78861681ebc9363e21b060f78e53a4eaba2da98c5f6`.
The synchronized tap has a clean empty successor. All previously existing
formulas, including ravel-lite.rb, retain their original bytes.

## Installed pair and owner policy

`/opt/homebrew/bin/ravel` resolves to
`/opt/homebrew/Cellar/ravel/0.1.0/bin/ravel`, reports `ravel 0.1.0` and finds
`/opt/homebrew/Cellar/ravel/0.1.0/libexec/ravel/ravel-policy` through its real
path. All nine installed bin/libexec files match the tagged archive bytes.
The formula test routes through its own symlink and round-trips ordinary,
empty, multiline and flag-like results with the matching real worker.

The copied owner policy passed 81 individually verified shell fake-agent
sessions and three refusals through the Homebrew-linked front. Every owner
route/model/effort/argument and plain/primary/secondary workspace grant was
checked, including the real Grove workspace. The test child PATH contains
only shell fake agents, with no Grove, Bun or Node. The checkout-local Bun
orchestrates the test externally; it is absent from the installed pair's
execution environment. The deliberate wrong-argument control failed at the
expected assertion before the full passing run. No real model launched.

The 14,184-subject baseline compared 14,033 unchanged files, with only the
three intended release version/changelog files and 148 Homebrew Ruby files
changed or removed by `brew update`. The per-file record names every change.
Owner Ravel/legacy configuration, dependency sources, release instruments,
existing tap formulas and the complete original Grove libexec tree retain
their bytes. PATH still resolves Grove, grove-llm and harness-dispatch to
the original `/opt/homebrew/Cellar/grove/22.1.0` keg; all report 22.1.0.

## Complete local records

Records are under Ravel's `target/`:

- `k11-release-minor.log`: full checks, cut, builds/formula, all platform
  smoke routes and first-push refusal; `worker-check.SnOzvV` records probes.
- `k11-publish-recovery.log`, `k11-public-release.json`,
  `k11-published-downloads/`: resumed publication and exact downloaded assets.
- `k11-installed-brew.log`, `k11-installed-cache.log`: anonymous-download
  failure and successful cached installation/formula test.
- `k11-anonymous-downloads.log`, `k11-anonymous-downloads/`,
  `k11-public-brew.log`, `k11-public-release-final.json`: all anonymous archive
  comparisons, forced public Homebrew fetch and repeated installed formula test.
- `k11-routing.mjs`, `k11-routing-control.log`, `k11-routing.log`,
  `k11-routing-results.json`: installed owner-policy instrument and per-case
  assertions, with no runtime dependency in the fake agents.
- `k11-subjects.py`, `k11-subjects.json`, `k11-subjects-after.json`,
  `k11-installed-evidence.json`: individual file hashes and installed layout.

The first release is complete. Retain the existing tag, archives and formula;
another bump is not a recovery step. See [release recovery](book/src/12-deliver-ravel.md#recover-a-partial-release-without-another-bump).
