#!/usr/bin/env bash
# Exercise an installation alone, also under bash 3.2 and the Linux floors.
set -euo pipefail
IFS=$'\n\t'
fail() { echo "installed-smoke: $*" >&2; exit 1; }
[[ $# == 2 ]] || fail 'usage: installed-smoke.sh PREFIX VERSION'
prefix="$(cd "$1" && pwd -P)"
version="$2"
work="$(mktemp -d)"
# shellcheck disable=SC2064 # remove precisely the owned directory
trap "rm -rf '$work'" EXIT
work="$(cd "$work" && pwd -P)"
# Select only system utilities, so installed execution has no development runtime.
tools="$work/tools"
mkdir -p "$tools"
for utility in cat chmod cmp cp dirname env grep ln ls mkdir rm; do
  resolved="$(PATH=/usr/bin:/bin command -v "$utility")" || fail "missing system utility $utility"
  ln -s "$resolved" "$tools/$utility"
done
export PATH="$tools"
for runtime in bun node grove grove-llm harness-dispatch; do
  ! command -v "$runtime" >/dev/null 2>&1 || fail "isolated PATH contains $runtime"
done
[[ ! -e "$prefix/libexec/ravel" ]] || fail 'retired libexec/ravel is installed'
for asset in examples/static.rhai examples/sample.rhai examples/llm.rhai notices/THIRD-PARTY-NOTICES.md; do
  [[ -s "$prefix/share/ravel/$asset" && ! -x "$prefix/share/ravel/$asset" ]] || fail "missing or executable installed $asset"
done
# A relative link from a different prefix, as a package manager links a keg.
mkdir -p "$work/linked/bin"
up=""; directory="$work/linked/bin"
while [[ "$directory" != / ]]; do up="../$up"; directory="$(dirname "$directory")"; done
ln -s "$up${prefix#/}/bin/ravel" "$work/linked/bin/ravel"
for label in direct symlink; do
  front="$prefix/bin/ravel"
  [[ "$label" == direct ]] || front="$work/linked/bin/ravel"
  [[ "$("$front" --version)" == "ravel $version" ]] || fail "$label version mismatch"
  dir="$work/$label"
  mkdir -p "$dir/home/.config/ravel" "$dir/cwd" "$dir/tmp" "$dir/bin"
  # Publish the last argument and keep the child's stdout off the result stream.
  # shellcheck disable=SC2016 # fixture expansions happen in the managed child
  printf '%s\n' '#!/bin/sh' 'for prompt in "$@"; do :; done' \
    'printf "fixture stdout\n"' 'exec "$SMOKE_FRONT" end-session -- "$prompt"' >"$dir/bin/my-agent-wrapper"
  chmod +x "$dir/bin/my-agent-wrapper"
  cp "$prefix/share/ravel/examples/sample.rhai" "$dir/home/.config/ravel/policy.rhai"
  cp "$prefix/share/ravel/examples/static.rhai" "$dir/home/.config/ravel/static.rhai"
  printf 'careful\n' >"$dir/cwd/.ravel-choice"
  for result in 'exact result' '' $'first\nsecond\n' '--help'; do
    printf '%s' "$result" >"$dir/expected"
    (cd "$dir/cwd" && env -i HOME="$dir/home" TMPDIR="$dir/tmp" \
      PATH="$dir/bin:$tools" SMOKE_FRONT="$front" \
      "$front" start-session --kind feature --prompt "$result" --json) \
      >"$dir/result" 2>"$dir/diagnostics" || fail "$label routed sample failed: $(cat "$dir/diagnostics")"
    cmp -s "$dir/expected" "$dir/result" || fail "$label changed the result"
    grep -Fq '"reason":"careful choice"' "$dir/diagnostics" || fail "$label did not use the imported example and choice"
    grep -Fq 'fixture stdout' "$dir/diagnostics" || fail "$label lost child output"
    [[ -z "$(ls -A "$dir/tmp")" ]] || fail "$label left temporary session resources"
  done
  # Measured reads, relative Rhai imports and params need only installed Ravel.
  mkdir -p "$dir/policy"
  printf 'measured' >"$dir/cwd/context.txt"
  cat >"$dir/policy/helper.rhai" <<'RHAI'
export const SUFFIX = "relative";
RHAI
  cat >"$dir/policy/policy.rhai" <<'RHAI'
import "helper" as helper;
export const SCHEMA_VERSION = 3;
export const VERSION = "installed-rhai";
fn load_context(request) {
    let read = ravel::read_text("context.txt", 4096);
    #{ schemaVersion: 1, facts: #{ text: read.text }, sources: [read.source] }
}
fn select(request, context) {
    if context.measured.len() != 1 || context.facts.text != "measured" || request.params.mode != "exact" {
        throw "inputs changed";
    }
    #{ status: "selected", program: request.params.fixture,
       args: [helper::SUFFIX + "|" + request.prompt], provider: "fixture",
       model: "fixture", effort: "high", reason: "installed Rhai" }
}
RHAI
  (cd "$dir/cwd" && env -i HOME="$dir/home" TMPDIR="$dir/tmp" PATH="$tools" SMOKE_FRONT="$front" \
    "$front" start-session --kind check --config "$dir/policy/policy.rhai" --param mode=exact --param "fixture=$dir/bin/my-agent-wrapper" --prompt caller) \
    >"$dir/result" 2>"$dir/diagnostics" || fail "$label Rhai context failed: $(cat "$dir/diagnostics")"
  printf 'relative|caller' | cmp -s - "$dir/result" || fail "$label installed Rhai result changed"
  # Retired formats must refuse from the installation too, before any launch.
  printf 'retired input' >"$dir/policy/policy.ts"
  status=0
  env -i HOME="$dir/home" TMPDIR="$dir/tmp" PATH="$tools" \
    "$front" start-session --kind check --config "$dir/policy/policy.ts" --prompt caller --json \
    >"$dir/refused.out" 2>"$dir/refused.err" || status=$?
  [[ "$status" == 3 && ! -s "$dir/refused.out" ]] || fail "$label accepted retired TypeScript"
  grep -Fq '"code":"policy_format_retired"' "$dir/refused.err" || fail "$label wrong retired-format refusal"
  if env -i HOME="$dir/home" PATH="$tools" "$front" end-session stale >"$dir/stale.out" 2>"$dir/stale.err"; then
    fail "$label accepted missing session context"
  fi
  [[ ! -s "$dir/stale.out" ]] || fail "$label wrote a stale result"
  [[ -z "$(ls -A "$dir/tmp")" ]] || fail "$label left temporary evaluation resources"
  printf 'installed-smoke: %s Rhai imports/context/params, exact/empty/multiline/flag results, retired format and stale context passed\n' "$label"
done
