#!/usr/bin/env bash

set -euo pipefail
IFS=$'\n\t'

CRATE_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
readonly CRATE_DIR
readonly WORKER_DIR="$CRATE_DIR/worker"
readonly BUN_VERSION="1.4.2"
readonly WORKER_NAME="ravel-policy"
# A checkout-local pinned runtime takes precedence over the host runtime.
export PATH="$CRATE_DIR/../../target/tools:$PATH"

# WHY A TARGET BUILD FETCHES ITS OWN RUNTIME. A cross-compiled worker is a copy
# of the target's `bun` with the policy host appended, so that runtime is part
# of what ships. Bun fetches it from npm into its install cache with no
# integrity check, and reuses any cached file of the right name unverified
# (bun-v1.4.2 src/options_types/compile_target.rs, `to_npm_registry_url` and
# `exe_path`; src/standalone_graph/StandaloneModuleGraph.rs,
# `target_executable`). So a target build downloads the same tarball itself,
# checks it against the digest pinned below, and hands the `bun` inside to
# `--compile-executable-path`, which skips Bun's fetch entirely. Each digest's
# tarball also matched npm's published `dist.integrity` when it was pinned.
# A Bun upgrade re-pins all three, replaces notices/bun-LICENSE.md from the new
# tag, and reruns the runtime-evidence probes.
runtime_package() {
  case "$1" in
    bun-darwin-arm64) echo "bun-darwin-aarch64" ;;
    bun-linux-arm64) echo "bun-linux-aarch64" ;;
    bun-linux-x64) echo "bun-linux-x64" ;;
    *) die "no pinned Bun runtime for target '$1'; the pinned targets are bun-darwin-arm64, bun-linux-arm64 and bun-linux-x64" ;;
  esac
}

runtime_sha256() {
  case "$1" in
    bun-darwin-aarch64) echo "a9df486eaf7e9db9bdebb1fa425e8c9809abd783b1c518d1dbc5096a53b861ed" ;;
    bun-linux-aarch64) echo "9ab3970a19660b5cd089f17fb021d900e1ca1b988dafd461d66d0a0ff4d6eac4" ;;
    bun-linux-x64) echo "0c75e0b94e9d56cece77abb7d03e9995b47b79506df5dcca3046eeb888db5934" ;;
  esac
}

# Checkout-local development toolchain; the installed pair needs no host Bun.
setup_bun() {
  local target package expected tools scratch archive
  case "$(uname -s)/$(uname -m)" in
    Darwin/arm64) target=bun-darwin-arm64 ;;
    Linux/aarch64 | Linux/arm64) target=bun-linux-arm64 ;;
    Linux/x86_64) target=bun-linux-x64 ;;
    *) die "no pinned development runtime for $(uname -s)/$(uname -m)" ;;
  esac
  package="$(runtime_package "$target")"
  expected="$(runtime_sha256 "$package")"
  tools="$CRATE_DIR/../../target/tools"
  mkdir -p "$tools"
  archive="$tools/$package-$BUN_VERSION.tgz"
  if [[ ! -f "$archive" ]] || [[ "$(digest_of "$archive")" != "$expected" ]]; then
    scratch="$(mktemp "$tools/.download.XXXXXX")"
    if ! curl --fail --silent --show-error --location --output "$scratch" \
      "https://registry.npmjs.org/@oven/$package/-/$package-$BUN_VERSION.tgz"; then
      rm -f "$scratch"
      die "cannot fetch the pinned $package development runtime"
    fi
    if [[ "$(digest_of "$scratch")" != "$expected" ]]; then
      rm -f "$scratch"
      die "development runtime digest does not match the committed pin"
    fi
    mv "$scratch" "$archive"
  fi
  scratch="$(mktemp -d "$tools/.extract.XXXXXX")"
  # shellcheck disable=SC2064 # remove precisely this private extraction directory
  trap "rm -rf '$scratch'" EXIT
  tar -xzf "$archive" -C "$scratch" package/bin/bun
  install -m 0755 "$scratch/package/bin/bun" "$tools/bun"
  require_bun
  echo "ravel: Bun $BUN_VERSION development runtime in $tools"
}

die() {
  echo "ravel: $*" >&2
  exit 1
}

sha256() {
  if command -v shasum >/dev/null 2>&1; then
    shasum -a 256 "$@"
  else
    sha256sum "$@"
  fi
}

# The files whose bytes decide the compiled worker, relative to the crate.
# build.rs enumerates the same set; change both together.
source_files() {
  (
    cd "$CRATE_DIR"
    find worker/src worker/sdk worker/examples -type f -name '*.ts'
    printf '%s\n' worker/tsconfig.json worker/tsconfig.declarations.json scripts/dispatch.sh
  ) | LC_ALL=C sort
}

build_id() {
  local file
  (
    cd "$CRATE_DIR"
    source_files | while IFS= read -r file; do sha256 "$file"; done
  ) | sha256 | cut -d' ' -f1
}

require_bun() {
  command -v bun >/dev/null 2>&1 ||
    die "bun $BUN_VERSION is required to build the worker; install it (https://bun.sh/docs/installation) and retry"
  local found
  found="$(bun --version)"
  [[ "$found" == "$BUN_VERSION" ]] ||
    die "bun $BUN_VERSION is pinned, found $found; install bun $BUN_VERSION (upgrades rerun the runtime-evidence probes first)"
}

require_type_checker() {
  [[ -f "$WORKER_DIR/node_modules/typescript/bin/tsc" ]] ||
    die "the pinned type checker is not installed; run 'task deps'"
}

tsc() {
  bun "$WORKER_DIR/node_modules/typescript/bin/tsc" "$@"
}

# `cargo metadata` names the target directory and the workspace version; bun,
# already required, reads the JSON.
metadata_field() {
  cargo metadata --format-version 1 --no-deps --manifest-path "$CRATE_DIR/Cargo.toml" |
    bun -e "
      const metadata = JSON.parse(await Bun.stdin.text());
      const field = process.argv[1];
      if (field === 'target') console.log(metadata.target_directory);
      else console.log(metadata.packages.find(p => p.name === 'ravel').version);
    " "$1"
}

default_out_dir() {
  echo "$(metadata_field target)/libexec/ravel"
}

# Extract TARGET's pinned runtime into DIR and print the `bun` path. Tarballs
# are cached under the cargo target directory, but the digest is checked on a
# private copy in DIR, and that copy is what gets extracted, so the bytes
# verified are the bytes used whatever happens to the cache meanwhile. A cached
# tarball that fails the check is fetched again once, and a fresh download
# that still differs is refused.
#
# It runs inside a command substitution, which errexit does not reach, so every
# step that can fail exits explicitly: a failure must stop the build rather
# than hand Bun a path to nothing, which it would answer by fetching.
pinned_runtime() {
  local target="$1" dir="$2" package sha cache tarball copy download
  package="$(runtime_package "$target")" || exit 1
  sha="$(runtime_sha256 "$package")" || exit 1
  cache="$(metadata_field target)/bun-runtimes" || exit 1
  tarball="$cache/$package-$BUN_VERSION.tgz"
  copy="$dir/$package-$BUN_VERSION.tgz"
  if [[ ! -f "$tarball" ]] || ! cp "$tarball" "$copy" || [[ "$(digest_of "$copy")" != "$sha" ]]; then
    mkdir -p "$cache" || exit 1
    download="$(mktemp "$cache/.download.XXXXXX")" || exit 1
    curl --fail --silent --show-error --location --output "$download" \
      "https://registry.npmjs.org/@oven/$package/-/$package-$BUN_VERSION.tgz" ||
      { rm -f "$download"; die "cannot download the $package $BUN_VERSION runtime from npm"; }
    cp "$download" "$copy" || exit 1
    local found
    found="$(digest_of "$copy")"
    if [[ "$found" != "$sha" ]]; then
      rm -f "$download"
      die "the $package $BUN_VERSION runtime from npm has SHA-256 $found, but $sha is pinned; refusing to build against it"
    fi
    mv "$download" "$tarball" || exit 1
  fi
  tar -xzf "$copy" -C "$dir" package/bin/bun || die "cannot extract bun from $copy"
  echo "$dir/package/bin/bun"
}

digest_of() {
  sha256 "$1" | cut -d' ' -f1
}

# The shipped worker's autoload switches. All four are stated, even where
# 1.4.2's default already agrees, so that a Bun upgrade changing a default
# cannot change the build.
#
# WHY THE DOTENV SWITCH IS SOMETIMES THE ONLY CONTROL. For the files Bun reads
# as a process starts, the switch is one of two controls: the front also
# starts the worker in an empty directory of its own. Bun loads dotenv files
# again for each VM it starts, though, from the directory the process is then
# in (`start_vm`, and `clone_for_worker`, which marks no file loaded):
# https://github.com/oven-sh/bun/blob/bun-v1.4.2/src/jsc/web_worker.rs
# https://github.com/oven-sh/bun/blob/bun-v1.4.2/src/dotenv/env_loader.rs
# The worker has moved to `/` by the time a policy can start a native Worker
# (main.ts), so with this switch on such a Worker would load `/.env`. Seen on
# 2026-10-01 in a build with the switch on; see "A VM started later" under
# "The worker's directory" in the runtime evidence named below.
#
# WHY PACKAGE.JSON AUTOLOADING IS ON, AND WHAT MAKES THAT SAFE. Without it the
# resolver finds a package only by its file layout, so one whose entry `main`
# or `exports` declares, which is most published packages, does not load. The
# switch decides one resolver option, `load_package_json`
# (`apply_standalone_runtime_flags`), whose one reader records the
# package.json of each directory the resolver builds a record for
# (`dir_info_uncached`):
# https://github.com/oven-sh/bun/blob/bun-v1.4.2/src/bun.js.rs
# https://github.com/oven-sh/bun/blob/bun-v1.4.2/src/resolver/resolver.rs
# It builds one for more than the directories an import names. A module
# compiled into the worker, and any module with no file location, is resolved
# as though it sat in the directory the process is in
# (`resolve_and_auto_install`), and even an import of an absolute path builds
# that directory's record and each ancestor's (`resolve_without_symlinks`).
# The front starts the worker in a private directory under the caller's
# TMPDIR, so a worker that stayed there would read every package.json at or
# above TMPDIR whole on each evaluation. Seen on 2026-10-01: one past 4 GiB
# there stalled a plain policy to its deadline, and one with an `imports` map
# answered a `#` import from a `data:` module. So the switch is on only
# because the worker moves to `/` before it loads anything (main.ts). The
# `unmoved` probe, which does not move, still does both, and tests/hostile.rs
# holds the shipped worker to neither. `/package.json` is what remains, as
# `/node_modules` remained with the switch off.
#
# The switch also changes what an entry's own package.json does: its `imports`
# map applies, its own `name` resolves ahead of `node_modules`, and an
# `imports` alias to a registered specifier is a `node_modules` lookup that
# never reaches the embedded module (`load_package_imports`). The spec states
# each under *Policy authority and runtime discovery*. See "Package entry
# resolution", "The worker's directory" and "Package.json autoloading" in
# docs/design/harness-selection-and-execution/runtime-evidence.md.
readonly SHIPPED_SWITCHES=(
  --no-compile-autoload-dotenv
  --no-compile-autoload-bunfig
  --no-compile-autoload-tsconfig
  --compile-autoload-package-json
)

# Set `switches` to the shipped ones with autoloading of each class named in
# the arguments turned on: the control a probe build removes. A class the
# shipped build does not turn off is a mistake in the caller, since the probe
# would then be the shipped build under another name.
switches_enabling() {
  local class index found
  switches=("${SHIPPED_SWITCHES[@]}")
  for class in "$@"; do
    found=""
    for index in "${!switches[@]}"; do
      if [[ "${switches[index]}" == "--no-compile-autoload-$class" ]]; then
        switches[index]="--compile-autoload-$class"
        found=yes
      fi
    done
    [[ -n "$found" ]] || die "the shipped worker does not turn $class autoloading off, so no probe can turn it on"
  done
}

# Compile the worker to OUT_FILE, reporting build identity ID and package
# VERSION, with PROBE naming the control a probe build removes ("" for the
# shipped build) and TARGET a Bun cross-compile target or "". The remaining
# arguments are the autoload switches. Bun 1.4.2 leaves a copy of its ~60 MB
# compile template in its cwd after every compile, so it runs in a throwaway
# directory rather than the source tree, where jj would try to snapshot the
# copy.
compile_worker() {
  local out_file="$1" id="$2" version="$3" probe="$4" target="$5"
  shift 5
  (
    scratch="$(mktemp -d)"
    # shellcheck disable=SC2064 # expand now: the trap must remove this directory
    trap "rm -rf '$scratch'" EXIT
    cd "$scratch"
    cross=()
    if [[ -n "$target" ]]; then
      runtime="$(pinned_runtime "$target" "$scratch")"
      cross=(--target="$target" --compile-executable-path="$runtime")
    fi
    bun build --compile ${cross[@]+"${cross[@]}"} "$@" \
      --define "RAVEL_BUILD_ID=\"$id\"" \
      --define "RAVEL_PACKAGE_VERSION=\"$version\"" \
      --define "RAVEL_PROBE=\"$probe\"" \
      "$WORKER_DIR/src/main.ts" --outfile "$out_file"
  )
}

build() {
  local target=""
  if [[ "${1:-}" == "--target" ]]; then
    target="${2:?--target needs a Bun target}"
    shift 2
  fi
  local out_dir
  out_dir="${1:-$(default_out_dir)}"
  require_bun
  require_type_checker
  grep -Fq "Bun $BUN_VERSION" "$CRATE_DIR/notices/NOTICES.md" ||
    die "notices/NOTICES.md does not describe Bun $BUN_VERSION; update the notices with the pin"
  local id version
  id="$(build_id)"
  version="$(metadata_field version)"
  mkdir -p "$out_dir/sdk" "$out_dir/examples" "$out_dir/notices"
  # Bun compiles from a scratch directory, where a relative OUT_DIR would name
  # somewhere inside that directory.
  out_dir="$(cd "$out_dir" && pwd)"

  compile_worker "$out_dir/$WORKER_NAME" "$id" "$version" "" "$target" "${SHIPPED_SWITCHES[@]}"

  tsc -p "$WORKER_DIR/tsconfig.declarations.json" --outDir "$out_dir"
  cp "$WORKER_DIR/sdk/index.ts" "$out_dir/sdk/index.ts"
  cp "$WORKER_DIR"/examples/*.ts "$out_dir/examples/"
  cp "$WORKER_DIR/sample/policy.ts" "$out_dir/examples/sample.ts"
  cp "$CRATE_DIR"/notices/*.md "$out_dir/notices/"
  echo "ravel: worker $version ($id)${target:+ for $target} in $out_dir"
}

typecheck() {
  local out_dir
  out_dir="${1:-$(default_out_dir)}"
  require_bun
  require_type_checker
  [[ -f "$out_dir/sdk/index.d.ts" ]] ||
    die "no SDK declarations in $out_dir; run 'task worker'"
  # The generated tsconfig below lives in a scratch directory, which a
  # relative OUT_DIR would be resolved against.
  out_dir="$(cd "$out_dir" && pwd)"
  tsc -p "$WORKER_DIR/tsconfig.json"

  # Fixtures import `ravel/sdk`, the examples as an
  # owner's policy does, resolved to the shipped declarations rather than the
  # sources.
  local scratch
  scratch="$(mktemp -d)"
  # shellcheck disable=SC2064 # expand now: the trap must remove this directory
  trap "rm -rf '$scratch'" EXIT
  cat >"$scratch/tsconfig.json" <<EOF
{
  "extends": "$WORKER_DIR/tsconfig.json",
  "compilerOptions": {
    "types": [],
    "paths": {
      "ravel/sdk": ["$out_dir/sdk/index.d.ts"],
      "ravel/examples/*": ["$out_dir/examples/*.d.ts"]
    }
  },
  "include": ["$WORKER_DIR/typecheck/*.ts"]
}
EOF
  tsc -p "$scratch/tsconfig.json"
  echo "ravel: worker, SDK and fixtures type-check"
}

# WHY PROBE BUILDS, AND WHY NONE CAN SHIP. Each hostile class a command-seam
# test proves inert has a firing configuration, which must be seen to fire so
# that a clean result cannot come from a fixture that never could have
# (docs/specs/harness-selection-and-execution.md, the firing-configuration
# table under *Agreed test seams and acceptance*). Some of them need the same
# worker source with one control removed:
#
#   autoload      dotenv and bunfig autoloading on, as in Bun's defaults
#   tsconfig      tsconfig autoloading on
#   unregistered  the shipped switches, and no embedded-module registration
#   unmoved       the shipped switches, and no move out of the directory the
#                 worker starts in
#
# main.ts reads RAVEL_PROBE for the last two.
#
# Each takes its switches from the shipped set, so a probe differs from the
# shipped worker by its one control and by nothing else.
#
# Each reports the identity probe-<name>-<source digest>, which no front
# accepts, so a probe is refused with exit 5 wherever an installation's worker
# belongs, and an archive carrying one as its worker fails the installed smoke
# test before a release publishes. Tests drive a probe directly. They live in
# target/probes, outside the libexec layout, and nothing else builds them.
probes() {
  local out_dir
  out_dir="${1:-$(metadata_field target)/probes/ravel}"
  require_bun
  local id version name switches
  id="$(build_id)"
  version="$(metadata_field version)"
  for name in autoload tsconfig unregistered unmoved; do
    mkdir -p "$out_dir/$name"
  done
  out_dir="$(cd "$out_dir" && pwd)"
  switches_enabling dotenv bunfig
  compile_worker "$out_dir/autoload/$WORKER_NAME" "probe-autoload-$id" "$version" autoload "" "${switches[@]}"
  switches_enabling tsconfig
  compile_worker "$out_dir/tsconfig/$WORKER_NAME" "probe-tsconfig-$id" "$version" tsconfig "" "${switches[@]}"
  for name in unregistered unmoved; do
    compile_worker "$out_dir/$name/$WORKER_NAME" "probe-$name-$id" "$version" "$name" "" "${SHIPPED_SWITCHES[@]}"
  done
  echo "ravel: probe builds of worker $version ($id) in $out_dir; test instruments, never shipped"
}

main() {
  local command="${1:-}"
  shift || true
  case "$command" in
    setup) setup_bun ;;
    deps) require_bun; (cd "$WORKER_DIR" && bun install --frozen-lockfile --ignore-scripts) ;;
    build-id) build_id ;;
    build) build "$@" ;;
    typecheck) typecheck "$@" ;;
    probes) probes "$@" ;;
    *) die "usage: dispatch.sh build-id | build [--target T] [OUT_DIR] | typecheck [OUT_DIR] | probes [OUT_DIR]" ;;
  esac
}

main "$@"
