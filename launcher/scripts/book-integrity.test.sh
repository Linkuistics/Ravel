#!/usr/bin/env bash
# Exercise the shipped setup_mdbook function against private tool directories:
# fresh fetch, verified-cache reuse, corrupt-cache recovery, and the refusal of a
# corrupt or wrong-digest download. No network, no change to the real tools
# directory; only the fetch boundary is emulated, with the real archive that
# `task setup` already verified.
set -euo pipefail
IFS=$'\n\t'
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
script="$root/scripts/book.sh"
work="$(mktemp -d)"
# shellcheck disable=SC2064 # exact owned test directory
trap "rm -rf '$work'" EXIT

die() { echo "book-integrity: $*" >&2; exit 1; }

verified_archive() {
  (
    # shellcheck disable=SC1090 # the production script, sourced for its definitions
    source "$script"
    echo "$root/target/tools/mdbook-v$MDBOOK_VERSION-$(mdbook_target).tar.gz"
  )
}
verified="$(verified_archive)"
version="$(bash -c 'source "$1"; echo "$MDBOOK_VERSION"' _ "$script")"
[[ -f "$verified" ]] || die 'run task setup before the integrity instrument'

# Run setup_mdbook from the production script in tools dir $1. Mode $2 says what
# the emulated fetch delivers: good, corrupt or none (a failed download). Mode $3,
# when "wrong-pin", replaces the pinned digest so even the real archive differs.
# Each fetch appends a line to $1/fetches.
run_setup() {
  local tools="$1" mode="$2" pin="${3:-real-pin}"
  (
    export BOOK_TOOLS_DIR="$tools"
    # shellcheck disable=SC1090
    source "$script"
    # shellcheck disable=SC2329 # shadows curl for the sourced setup_mdbook
    curl() {
      local output=""
      while (($#)); do
        if [[ "$1" == --output ]]; then output="$2"; shift 2; else shift; fi
      done
      [[ -n "$output" ]] || return 1
      echo fetched >>"$tools/fetches"
      case "$mode" in
        good) cp "$verified" "$output" ;;
        corrupt) printf 'corrupt fresh mdbook' >"$output" ;;
        none) return 22 ;;
      esac
    }
    if [[ "$pin" == wrong-pin ]]; then
      # shellcheck disable=SC2329 # replaces the pin in the sourced script
      mdbook_sha256() { echo 0000000000000000000000000000000000000000000000000000000000000000; }
    fi
    setup_mdbook
  )
}
fetches() { if [[ -f "$1/fetches" ]]; then wc -l <"$1/fetches" | tr -d ' '; else echo 0; fi; }

# 1. Nothing cached: fetch once, verify, install.
t="$work/fresh"; mkdir -p "$t"
run_setup "$t" good >/dev/null 2>"$work/err" || die "fresh fetch failed: $(cat "$work/err")"
[[ -x "$t/mdbook" && "$(fetches "$t")" == 1 ]] || die 'fresh setup did not fetch exactly once and install'
[[ "$("$t/mdbook" --version)" == "mdbook v$version" ]] || die 'the installed tool is not the pinned version'

# 2. Verified cache: no fetch, still installed.
rm -f "$t/mdbook"
run_setup "$t" none >/dev/null 2>"$work/err" || die "cache reuse failed: $(cat "$work/err")"
[[ -x "$t/mdbook" && "$(fetches "$t")" == 1 ]] || die 'a verified cache was fetched again'

# 3. Corrupt cache, good fetch: refetched exactly once and recovered.
t="$work/corrupt-cache"; mkdir -p "$t"
cp "$verified" "$t/$(basename "$verified")"
printf 'corrupt cached mdbook' >"$t/$(basename "$verified")"
run_setup "$t" good >/dev/null 2>"$work/err" || die "corrupt cache did not recover: $(cat "$work/err")"
[[ -x "$t/mdbook" && "$(fetches "$t")" == 1 ]] || die 'corrupt cache did not refetch exactly once'
cmp -s "$verified" "$t/$(basename "$verified")" || die 'the recovered archive is not the verified one'

# 4. Corrupt download, no cache: refused for the digest, nothing installed or cached.
t="$work/corrupt-download"; mkdir -p "$t"
if run_setup "$t" corrupt >"$work/out" 2>"$work/err"; then die 'a corrupt download was accepted'; fi
grep -Fq 'refusing to use it' "$work/err" || die 'corrupt download failed for the wrong reason'
[[ ! -e "$t/mdbook" && -z "$(ls "$t"/mdbook-v*.tar.gz 2>/dev/null || true)" ]] ||
  die 'a refused download was installed or cached'

# 5. Corrupt cache and corrupt download: the cache is not trusted as a fallback.
t="$work/corrupt-both"; mkdir -p "$t"
printf 'corrupt cached mdbook' >"$t/$(basename "$verified")"
if run_setup "$t" corrupt >"$work/out" 2>"$work/err"; then die 'a corrupt cache and download were accepted'; fi
[[ ! -e "$t/mdbook" ]] || die 'a corrupt cache reached installation'

# 6. A wrong pinned digest refuses even the real archive: the check compares
# against the committed pin, not against anything the download supplies.
t="$work/wrong-pin"; mkdir -p "$t"
if run_setup "$t" good wrong-pin >"$work/out" 2>"$work/err"; then die 'a wrong pin accepted the archive'; fi
grep -Fq 'refusing to use it' "$work/err" || die 'wrong pin failed for the wrong reason'
[[ ! -e "$t/mdbook" ]] || die 'a wrong pin reached installation'

# 7. A failed download is reported, not mistaken for success.
t="$work/no-network"; mkdir -p "$t"
if run_setup "$t" none >"$work/out" 2>"$work/err"; then die 'a failed download was accepted'; fi
grep -Fq 'cannot download' "$work/err" || die 'failed download failed for the wrong reason'

printf 'book-integrity: fresh fetch, verified-cache reuse, corrupt-cache recovery and the refusals of a corrupt download, a wrong pin and a failed download passed (mdbook %s)\n' \
  "$version"
