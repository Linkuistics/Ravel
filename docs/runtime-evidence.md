# Pinned runtime evidence

The active worker uses Bun **1.4.2**, with TypeScript **7.0.2** and @types/bun
**1.4.2** frozen in worker/package.json and bun.lock. The runtime and package
mechanics are in [dispatch.sh](../crates/ravel/scripts/dispatch.sh). This document
retains applicable source findings from the extracted evaluator and identifies
which instruments now exercise Ravel. It does not claim earlier archive runs
were runs of this independent product.

## Controls and firing configurations

`task probes` builds autoload, tsconfig, unregistered and unmoved workers from
the shipped source. Each differs by its named control and a probe-prefixed
identity; none is installed. `task test:hostile` runs real workers with temporary
policies and fake agents. Each controlled class has a firing arm in the same test,
so inert output cannot count a fixture that never worked. Tests require probes
rather than skip missing ones. The active suite has these observations:

| Class | Shipped front | Firing configuration |
|---|---|---|
| Cwd dotenv/local dotenv and bunfig preload | Neither variable nor sentinel appears | Autoload probe started there loads both/preloads; each control alone remains inert |
| Dotenv in a native Worker started later | Neither VM gains dotenv | Autoload probe: new VM loads it, starting VM remains inert |
| BUN_OPTIONS preload / BUN_BE_BUN | No preload; real policy routes | Direct shipped worker with each runs the preload / arbitrary -e code |
| Runtime transpiler cache under HOME or granted XDG_CACHE_HOME | No cache write; admitted source selects beside poisoned cache | Direct worker caches a >=4 KiB policy; patched cached output changes its version |
| NODE_PRESERVE_SYMLINKS | Real helper directory's dependency resolves; grants refuse | Direct worker resolves beside the link instead |
| NODE_CHANNEL_FD=3 | Private framed protocol remains intact; grants refuse | Direct worker adopts the fd and process.send writes malformed protocol bytes |
| Each literal embedded specifier beside files/exports/own-name shadows | Embedded module wins; an unregistered name loads its shadow | Unregistered probe loads every shadow |
| Imports alias to an embedded name | Alias is a package lookup: refuses without a package, loads its shadow with one | Literal import still resolves to embedded module in the same entry |
| Tsconfig paths beside admitted entry | Import refuses | Tsconfig probe loads sentinel module |
| Cwd package imports/self-name/dependency for policy elsewhere | Refuses all three imports | Same entry explicitly admitted in that directory loads each |
| Package above worker start directory, from data/blob/virtual module | Missing import; no sentinel | Unmoved probe loads each, shipped direct worker started there refuses |
| Package.json imports/self-name/dependency above worker start directory | Refuses without running sentinel | Unmoved probe loads each from no-file modules; file-relative imports remain refused |
| Package.json link to a never-yielding FIFO above worker start directory | Routes; no reader appears | Unmoved probe opens FIFO and remains running/silent until deadline; normal manifest baseline loads |
| Probe installed at actual worker path | Identity refusal before policy marker runs | Shipped worker at same path evaluates/launches successfully |
| Protocol-looking floods on both diagnostic streams | Actual selection and opaque result stay intact | Combined diagnostic output beyond 256 KiB refuses with output_limit |

HOME bunfig preloads, cwd-only tsconfig paths for an entry elsewhere, and startup
bunfig reloading after a process move have **no known firing configuration** on
this pinned runtime. The tripwire test reports those cases without treating them
as controls, and fails if they start firing. No root-owned /.env or /package.json
fixture is planted by the suite.

## Source findings retained with the implementation

The original source readings and experiments were made on macOS arm64 on
2026-09-29 through 2026-10-01. Their full provenance is the source repository's
[runtime evidence at the extraction revision](https://github.com/Linkuistics/grove/blob/639b9d63570ed87f81f3ced3d2851b7961006bf3/docs/design/harness-selection-and-execution/runtime-evidence.md).
Names, older feature claims and archive observations there describe that source
project only. The active table above names the transferred Ravel instruments.

The compile autoload flags feed resolver/runtime options in the pinned
[build command](https://github.com/oven-sh/bun/blob/bun-v1.4.2/src/runtime/cli/build_command.rs)
and [standalone startup](https://github.com/oven-sh/bun/blob/bun-v1.4.2/src/bun.js.rs).
[Resolver source](https://github.com/oven-sh/bun/blob/bun-v1.4.2/src/resolver/resolver.rs)
(`dir_info_uncached`, `resolve_and_auto_install`, `resolve_without_symlinks`,
`load_node_modules`, `load_package_imports`) explains whole package.json reads,
no-file importer directory substitution, ordinary package entry resolution and
imports aliases that do not return to embedded registration. Package.json
loading is enabled only after the worker's move removes start-directory ancestry.
A standalone executable does not invoke the package manager for missing imports.

[VirtualMachine.rs](https://github.com/oven-sh/bun/blob/bun-v1.4.2/src/jsc/VirtualMachine.rs)
updates resolver top_level_dir when process.chdir runs. The move closes no-file
module resolution through TMPDIR ancestors but leaves root-owned package files.
It is not a sandbox for admitted policy code. A policy can change cwd itself.

[web_worker.rs](https://github.com/oven-sh/bun/blob/bun-v1.4.2/src/jsc/web_worker.rs),
[env_loader.rs](https://github.com/oven-sh/bun/blob/bun-v1.4.2/src/dotenv/env_loader.rs)
and [transpiler.rs](https://github.com/oven-sh/bun/blob/bun-v1.4.2/src/bundler/transpiler.rs)
explain later VM dotenv loading. A private start directory protects startup
loads; only the compile switch protects dotenv when a policy starts another VM
in a different cwd. Bunfig is a process-start input, not reloaded by that move.

[RuntimeTranspilerCache.rs](https://github.com/oven-sh/bun/blob/bun-v1.4.2/src/jsc/RuntimeTranspilerCache.rs)
(`really_get_cache_dir`) reads the disabling environment value before cache-home
selection. The firing instrument pins cache layout version 28 and the output
metadata offsets; upgrades must rederive these fields. Its poisoned cached output
keeps the original input identity, illustrating why file admission alone is not
proof of which TypeScript ran.

The historical FIFO experiments observed a reader/stall on macOS arm64 and Linux
arm64. A writer leaving immediately let the Linux probe continue, so the test
holds its writer while observing the stall. This replaces the old >4 GiB sparse
manifest fixture: a dead probe can satisfy silence but cannot satisfy observed
open plus continued liveness. Root files and broader resolver layouts remain
outside that instrument's measured coverage.

## Runtime integrity and installation instruments

Cross-worker compilation extracts the target runtime from a committed SHA-256
pinned @oven tarball and passes the verified private copy to
--compile-executable-path. Bun's automatic npm/cache fetch is bypassed. The
[compile target source](https://github.com/oven-sh/bun/blob/bun-v1.4.2/src/options_types/compile_target.rs)
and [build command source](https://github.com/oven-sh/bun/blob/bun-v1.4.2/src/runtime/cli/build_command.rs)
explain that bypass; Bun's own cached fetch is not an integrity boundary.

| Runtime package, version 1.4.2 | Committed tarball SHA-256 |
|---|---|
| bun-darwin-aarch64 | a9df486eaf7e9db9bdebb1fa425e8c9809abd783b1c518d1dbc5096a53b861ed |
| bun-linux-aarch64 | 9ab3970a19660b5cd089f17fb021d900e1ca1b988dafd461d66d0a0ff4d6eac4 |
| bun-linux-x64 | 0c75e0b94e9d56cece77abb7d03e9995b47b79506df5dcca3046eeb888db5934 |

The original pinning compared each tarball with npm dist.integrity. task setup
verifies the native archive and installs checkout-local Bun. The offline
`task runtime:integrity` sources an unchanged private copy of the production
function, uses the real verified native archive, and substitutes only fetch and
target-directory boundaries. It proves corrupt-cache refetch, valid-cache reuse,
and refusal of a corrupt fresh download before extraction or fallback path return.
It never changes the real runtime cache, opens owner configuration or uses network.
It is a native integrity instrument, not execution of all target runtimes.

`task smoke` installs an independent release pair in a fresh temporary prefix.
Its direct and relative-symlink fronts run on a private PATH containing only
explicit system-utility links and fixture executables, with no host Bun/Node or
source-project binary. A system-installed Node therefore does not block installation.
The fresh prefix includes spaces/Unicode, and fixture paths travel as params. It exercises installed sample/choice/SDK/example, exact ordinary,
empty, multiline and flag-like result bytes, child-output separation, typed
relative imports, a package declared by main, measured context, params and stale
context refusal. Installed asset existence is checked. Hostile tests additionally
prove every probe is refused at that same worker layout. These are native checks;
Linux target glibc/CPU-floor/archive checks belong to the independent delivery
work and are not claimed here. The earlier source project's installed floor
experiments are provenance, not independent Ravel validation.

## Upgrade rule

Change Bun only with an explicit pinned upgrade: update all three tarball hashes,
worker dependencies/lockfile and the unchanged upstream licence notice; reread
the source functions above and rerun setup, worker/typecheck/probes, runtime
integrity, hostile/process/PTY/confinement tests and installed smoke. Regenerate
both identities together. A runtime upgrade needs target/archive checks before
publication. Do not silently replace the compiled worker with a host runtime.

Official [standalone documentation](https://bun.com/docs/bundler/executables) and
the pinned tag's [executable documentation](https://github.com/oven-sh/bun/blob/bun-v1.4.2/docs/bundler/executables.mdx)
describe compilation, switches and BUN_OPTIONS/BUN_BE_BUN. Current documentation
alone is not proof of the behavior of the pinned artifact. Runtime platform
floors must be verified by actual target delivery instruments.
