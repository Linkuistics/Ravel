#!/usr/bin/env bash
# Exercise the shipped pinned_runtime function against private corrupt caches.
# No network, altered real cache, worker compilation, or new test framework.
set -euo pipefail
IFS=$'\n\t'
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
work="$(mktemp -d)"
# shellcheck disable=SC2064 # exact owned test directory
trap "rm -rf '$work'" EXIT
mkdir -p "$work/crate/scripts" "$work/cache" "$work/out"
script="$root/crates/ravel/scripts/dispatch.sh"
[[ "$(tail -n 1 "$script")" == 'main "$@"' ]] || { echo 'runtime-integrity: script entry changed; update the instrument' >&2; exit 1; }
# Load the unchanged production definitions in a disposable location, omitting
# only the executable entry. No test hook or test-only API enters production.
sed '$d' "$script" >"$work/crate/scripts/dispatch.sh"
# shellcheck disable=SC1090,SC1091 # generated copy of the production script above
source "$work/crate/scripts/dispatch.sh"
case "$(uname -s)/$(uname -m)" in
  Darwin/arm64) compile_target=bun-darwin-arm64 ;;
  Linux/aarch64 | Linux/arm64) compile_target=bun-linux-arm64 ;;
  Linux/x86_64) compile_target=bun-linux-x64 ;;
  *) die 'unsupported host for the runtime integrity instrument' ;;
esac
package="$(runtime_package "$compile_target")"
verified="$root/target/tools/$package-$BUN_VERSION.tgz"
[[ -f "$verified" ]] || die 'run task setup before the runtime integrity instrument'
[[ "$(digest_of "$verified")" == "$(runtime_sha256 "$package")" ]] || die 'setup archive is not the committed runtime'
metadata_field() { printf '%s\n' "$work"; }
# Emulate only the fetch boundary; all digest, cache, copy and extraction logic
# is the real shipped function, using the real pinned archive from task setup.
curl() {
  local output=""
  while (($#)); do
    if [[ "$1" == --output ]]; then output="$2"; shift 2; else shift; fi
  done
  [[ -n "$output" ]] || return 1
  echo fetched >>"$work/fetches"
  if [[ "${bad_download:-no}" == yes ]]; then
    printf 'corrupt fresh runtime' >"$output"
  else
    cp "$verified" "$output"
  fi
}
cache="$work/bun-runtimes/$package-$BUN_VERSION.tgz"
mkdir -p "$(dirname "$cache")"
printf 'corrupt cached runtime' >"$cache"
runtime="$(pinned_runtime "$compile_target" "$work/out")"
[[ -x "$runtime" && "$(env -i PATH=/usr/bin:/bin "$runtime" --version)" == "$BUN_VERSION" ]] || die 'corrupt cache did not recover to the verified runtime'
[[ "$(wc -l <"$work/fetches" | tr -d ' ')" == 1 ]] || die 'corrupt cache did not refetch exactly once'
rm -rf "$work/out/package"
runtime="$(pinned_runtime "$compile_target" "$work/out")"
[[ -x "$runtime" && "$(wc -l <"$work/fetches" | tr -d ' ')" == 1 ]] || die 'valid cache unexpectedly fetched'
printf 'corrupt again' >"$cache"
rm -rf "$work/out/package"
bad_download=yes
if (pinned_runtime "$compile_target" "$work/out") >"$work/refused.out" 2>"$work/refused.err"; then
  die 'a corrupt fresh runtime was accepted'
fi
grep -Fq 'refusing to build against it' "$work/refused.err" || die 'corrupt download failed for the wrong reason'
[[ ! -e "$work/out/package/bin/bun" && ! -s "$work/refused.out" ]] || die 'corrupt download reached extraction or returned a fallback path'
printf 'runtime-integrity: verified archive, corrupt-cache recovery, valid-cache reuse and corrupt-download refusal passed (%s %s)\n' "$package" "$BUN_VERSION"
