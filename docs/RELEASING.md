# Independent Ravel delivery

Ravel's front and private worker have one Ravel version and source identity,
independent of any caller's version. Development is 0.0.0; the first minor
release is planned at 0.1.0. Recheck remote version tags before a cut. The
retired daemon bookmark and `structured-plan-migration` tag are historical
provenance, not Ravel release-version baselines.

## Unreleased archive checks

Run from the Ravel checkout on an Apple silicon Mac with Bash 4 or later,
rustup/Rust 1.85 or later, Task, curl, tar/xz, ShellCheck, Zig and
cargo-zigbuild. Install all supported Rust targets:

```sh
rustup target add aarch64-apple-darwin aarch64-unknown-linux-gnu x86_64-unknown-linux-gnu
task setup
task release:test
task release:smoke
task release:formula
task check
```

`task release:smoke` first builds fresh archives. `task release:archives`
builds them without running smoke. Either build clears `target/archives`,
including on a selected-target build (`TARGETS=aarch64-apple-darwin`), so
earlier target artifacts cannot fill in a failed run. Each archive is exposed
under its final filename only after exact-manifest validation. Rerun a failed
unreleased build from the task; it cuts no version and publishes nothing.

The worker build verifies each Bun 1.4.2 runtime against committed SHA-256
pins, builds with that verified private copy and emits generic declarations.
Runtime pins, source findings and upgrade rules are in
[runtime evidence](runtime-evidence.md) and the production
[build script](../crates/ravel/scripts/dispatch.sh). There is no host Bun/Node
requirement for the installed pair; the checkout-local runtime is a build tool.

Linux tests require an available **linux/arm64 Docker** with kernel 6.7 or
later. The scripts fetch pinned CentOS 7.9 userlands, QEMU and an Ubuntu helper.
The userlands contain glibc 2.17. Native arm64 Docker tests run first, then
both Linux targets run through a private binfmt_misc registration at their CPU
floors: Cortex-A53 for arm64, Nehalem for x86-64. Floor probes run, newer-glibc
probes refuse, above-floor instructions raise SIGILL at the floor and run
under the twin `-cpu max` interpreter. The guests must have a vDSO and match
the supported virtual-address-space bounds. Every archive ELF, including the
front and worker, must match the emulator registration. Guests have no network,
read-only inputs and unprivileged execution. Kernel-version floors are not
measured by containers or user-mode emulation.

The macOS archive runs natively. Every environment extracts with its own tar,
checks the version and runs the same [installed-layout seam](../crates/ravel/scripts/installed-smoke.sh),
directly and through a relative symlink. Its private utility PATH excludes
Bun, Node and Grove. It tests the SDK, static/sample/choice, exact ordinary,
empty/multiline/flag-like results, child output separation, TypeScript helpers,
declared packages, measured context, params and stale context refusal.
Missing tools or unrun targets are failures; there is no skip path.

## Layout and manifest controls

An archive's only top directory is `ravel-v<VERSION>-<TARGET>/`, an installation
prefix with `bin/ravel` and `libexec/ravel/ravel-policy`. It also carries the
generic SDK source/declarations, static example source/declarations, editable
sample, Bun notices, README and LICENSE. The complete file/directory manifest
is [release-common.sh](../scripts/release-common.sh). It permits no caller
binary, caller adapter, owner policy, choice, development dependency or probe.

`task release:test` compares the manifest to a fresh production worker build,
then fires omission controls for every file, extra-file/directory controls,
caller-asset controls, executable-mode and symlink/type controls and macOS
AppleDouble/xattr controls. The existing hostile runtime suite additionally
substitutes every probe at the real worker path, requires identity refusal
before policy execution, and exercises the matching shipped worker there as
the firing control. Probe tests require `task probes`; `task check` builds them.

## Formula preparation

`task release:formula` requires all three current-version archives. It checks
each exact manifest, renders all target SHA-256 values and rejects unexpanded
placeholders. It then loads the rendered formula through the installed Homebrew
Ruby library and executes its **actual install and test methods**. Only prefix
and testpath are redirected to owned temporary directories; this creates no
Homebrew keg and changes no tap. Homebrew is therefore a prerequisite for
formula validation. This preparation test is distinct from `brew test ravel`
after publication/installation.

The installation must match the archive's bin/libexec directories exactly.
The formula test reaches the front through a symlink, imports Ravel's SDK and
static example, routes to a shell fake agent and round-trips ordinary, empty,
multiline and flag-like opaque results on a scrubbed environment. It leaves
no temporary session resources. It launches no model and installs no host
TypeScript runtime. The formula has no Grove dependency.

The preparation task builds all four native probe workers and substitutes each
at the archive-derived installed worker path after the formula test. Every one
must refuse with `worker_identity_mismatch`; the normal formula test establishes
the matching production worker's success first. The private worker is restored
after the controls; no installed tool outside that temporary prefix is touched.

The rendered `target/archives/ravel.rb` is **unpublished preparation output**;
its 0.0.0 archive URLs are not downloadable releases. Do not copy this file into
a tap or publish these archives. Formula publication uses a version cut and
fresh tagged-source archives; `ravel-lite.rb` is a different command and must
remain intact. Release cut/preflight/publication/recovery recipes are a separate
delivery increment; these preparation tasks perform none of those operations.

## Evidence

The check wrapper saves full logs under `target/release-archives.*` and
`target/release-formula.*` and reports a concise pass/fail summary. Build and
floor-smoke output is suitable for a saved full log, with one target/control
record per execution. A handoff must name the exact source revision, archives
and SHA-256 digests, individual target routes and firing controls, complete
logs and any unmet prerequisites. Archives are local artifacts, not durable
published URLs; rebuild and rerun after source or runtime changes.
