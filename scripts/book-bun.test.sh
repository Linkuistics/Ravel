#!/usr/bin/env bash
# Exercise book Bun setup offline against private caches. Only curl is replaced;
# verification, extraction and version checks use the real pinned archive.
set -euo pipefail
IFS=$'\n\t'
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
script="${BOOK_BUN_SCRIPT:-$root/scripts/book-bun.sh}"
work="$(mktemp -d)"
# shellcheck disable=SC2064 # exact owned test directory
trap "rm -rf '$work'" EXIT

die() { echo "book-bun-integrity: $*" >&2; exit 1; }
[[ -f "$script" ]] || die "no book Bun setup at $script"
verified="$(bash -c 'source "$1"; echo "$TOOLS/$(bun_package)-$BUN_VERSION.tgz"' _ "$script")"
version="$(bash "$script" version)"
[[ -f "$verified" ]] || die 'run task setup before the integrity instrument'

run_setup() {
  local tools="$1" mode="$2" pin="${3:-real-pin}"
  (
    export BOOK_TOOLS_DIR="$tools"
    # shellcheck disable=SC1090 # production definitions, scoped to this subshell
    source "$script"
    # shellcheck disable=SC2329 # fetch boundary used by the sourced setup
    curl() {
      local output=""
      while (($#)); do
        if [[ "$1" == --output ]]; then output="$2"; shift 2; else shift; fi
      done
      [[ -n "$output" ]] || return 1
      echo fetched >>"$tools/fetches"
      case "$mode" in
        good) cp "$verified" "$output" ;;
        corrupt) printf 'corrupt fresh Bun' >"$output" ;;
        none) return 22 ;;
      esac
    }
    if [[ "$pin" == wrong-pin ]]; then
      # shellcheck disable=SC2329 # wrong digest must reject the real archive
      bun_sha256() { echo 0000000000000000000000000000000000000000000000000000000000000000; }
    fi
    setup_bun
  )
}
fetches() { if [[ -f "$1/fetches" ]]; then wc -l <"$1/fetches" | tr -d ' '; else echo 0; fi; }

t="$work/fresh"
run_setup "$t" good >"$work/out" 2>"$work/err" || die "fresh fetch failed: $(cat "$work/err")"
[[ -x "$t/bun" && "$(fetches "$t")" == 1 ]] || die 'fresh setup did not fetch exactly once and install'
[[ "$("$t/bun" --version)" == "$version" ]] || die 'installed Bun has the wrong version'
cmp -s "$verified" "$t/$(basename "$verified")" || die 'fresh cache differs from the pinned archive'
rm -f "$t/bun"
run_setup "$t" none >"$work/out" 2>"$work/err" || die "cache reuse failed: $(cat "$work/err")"
[[ -x "$t/bun" && "$(fetches "$t")" == 1 ]] || die 'verified cache fetched again or failed to install'

t="$work/corrupt-cache"; mkdir -p "$t"
printf 'corrupt cached Bun' >"$t/$(basename "$verified")"
run_setup "$t" good >"$work/out" 2>"$work/err" || die "cache recovery failed: $(cat "$work/err")"
[[ -x "$t/bun" && "$(fetches "$t")" == 1 ]] || die 'corrupt cache did not recover with one fetch'
cmp -s "$verified" "$t/$(basename "$verified")" || die 'recovered cache differs from the pinned archive'

for scenario in corrupt-download corrupt-both wrong-pin no-network; do
  t="$work/$scenario"; mkdir -p "$t"
  mode=corrupt; pin=real-pin; reason='refusing to use it'
  case "$scenario" in
    corrupt-both) printf 'corrupt cached Bun' >"$t/$(basename "$verified")" ;;
    wrong-pin) mode=good; pin=wrong-pin ;;
    no-network) mode=none; reason='cannot download' ;;
  esac
  if run_setup "$t" "$mode" "$pin" >"$work/out" 2>"$work/err"; then die "$scenario was accepted"; fi
  grep -Fq "$reason" "$work/err" || die "$scenario failed for the wrong reason"
  [[ ! -e "$t/bun" && "$(fetches "$t")" == 1 ]] || die "$scenario installed Bun or fetched an unexpected number of times"
  if [[ "$scenario" != corrupt-both ]]; then
    [[ ! -e "$t/$(basename "$verified")" ]] || die "$scenario cached a refused download"
  fi
done

printf 'book-bun-integrity: fresh fetch, cache reuse/recovery and corrupt-download, wrong-pin and failed-download refusals passed (Bun %s)\n' "$version"
