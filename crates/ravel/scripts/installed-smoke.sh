#!/usr/bin/env bash
# Exercise an installed pair alone; reusable by later archive/release checks.
# Uses bash 3.2 and system utilities, including on the supported Linux floors.
set -euo pipefail
IFS=$'\n\t'
fail() { echo "installed-smoke: $*" >&2; exit 1; }
[[ $# == 2 ]] || fail 'usage: installed-smoke.sh PREFIX VERSION'
prefix="$(cd "$1" && pwd -P)"
version="$2"
work="$(mktemp -d)"
# shellcheck disable=SC2064 # clean precisely this owned directory
trap "rm -rf '$work'" EXIT
work="$(cd "$work" && pwd -P)"
# A distribution may have Node in /usr/bin. Use only explicitly selected
# system utilities so installed execution cannot accidentally rely on it.
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
for asset in ravel-policy sdk/index.ts sdk/index.d.ts examples/static.ts examples/static.d.ts examples/sample.ts notices/NOTICES.md notices/bun-LICENSE.md; do
  [[ -f "$prefix/libexec/ravel/$asset" ]] || fail "missing installed $asset"
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
  # A fixture wrapper supports the generic sample and publishes the prompt.
  # Its child output must stay separate from the opaque result stream.
  # shellcheck disable=SC2016 # fixture shell variables expand when the child runs
  printf '%s\n' '#!/bin/sh' 'for prompt in "$@"; do :; done' \
    'printf "fixture stdout\n"' 'exec "$SMOKE_FRONT" end-session -- "$prompt"' >"$dir/bin/my-agent-wrapper"
  chmod +x "$dir/bin/my-agent-wrapper"
  cp "$prefix/libexec/ravel/examples/sample.ts" "$dir/home/.config/ravel/policy.ts"
  printf 'careful\n' >"$dir/cwd/.ravel-choice"
  for result in 'exact result' '' $'first\nsecond\n' '--help'; do
    printf '%s' "$result" >"$dir/expected"
    (cd "$dir/cwd" && env -i HOME="$dir/home" TMPDIR="$dir/tmp" \
      PATH="$dir/bin:$tools" SMOKE_FRONT="$front" \
      "$front" start-session --kind feature --prompt "$result" --json) \
      >"$dir/result" 2>"$dir/diagnostics" || fail "$label routed sample failed: $(cat "$dir/diagnostics")"
    cmp -s "$dir/expected" "$dir/result" || fail "$label changed the result"
    grep -Fq '"reason":"careful choice"' "$dir/diagnostics" || fail "$label did not use the SDK/choice/example"
    grep -Fq 'fixture stdout' "$dir/diagnostics" || fail "$label lost child output"
    [[ -z "$(ls -A "$dir/tmp")" ]] || fail "$label left temporary session resources"
  done
  # Installed TypeScript, relative helpers, package main/exports, context reads
  # and params must work without any development runtime or source checkout.
  mkdir -p "$dir/policy/node_modules/declared/lib"
  printf '%s\n' '{"name":"declared","main":"./lib/value.js"}' >"$dir/policy/node_modules/declared/package.json"
  printf '%s\n' 'export const value = "package";' >"$dir/policy/node_modules/declared/lib/value.js"
  printf '%s\n' 'export const suffix: string = "relative";' >"$dir/policy/helper.ts"
  printf 'measured' >"$dir/cwd/context.txt"
  printf '%s\n' \
    'import { definePolicy, type SelectionRequest } from "ravel/sdk";' \
    'import { value } from "declared";' 'import { suffix } from "./helper.ts";' \
    'export const policy = definePolicy({' 'schemaVersion: 2, version: "installed-types",' \
    'loadContext(request, host) { const read = host.readText("context.txt"); return {schemaVersion: 1, facts: {text: read.text}, sources: [read.source]}; },' \
    'select(request: SelectionRequest, context) {' \
    'if (context.measured.length !== 1 || context.facts.text !== "measured" || request.params.mode !== "exact") throw new Error("inputs changed");' \
    "return {status: \"selected\", program: request.params.fixture, args: [value + \"|\" + suffix + \"|\" + request.prompt], provider: \"fixture\", model: \"fixture\", effort: \"high\", reason: \"installed types\"};" \
    '} });' >"$dir/policy/policy.ts"
  (cd "$dir/cwd" && env -i HOME="$dir/home" TMPDIR="$dir/tmp" PATH="$tools" SMOKE_FRONT="$front" \
    "$front" start-session --kind check --config "$dir/policy/policy.ts" --param mode=exact --param "fixture=$dir/bin/my-agent-wrapper" --prompt caller) \
    >"$dir/result" 2>"$dir/diagnostics" || fail "$label TypeScript policy failed: $(cat "$dir/diagnostics")"
  printf 'package|relative|caller' | cmp -s - "$dir/result" || fail "$label installed TypeScript result changed"
  if env -i HOME="$dir/home" PATH="$tools" "$front" end-session stale >"$dir/stale.out" 2>"$dir/stale.err"; then
    fail "$label accepted missing session context"
  fi
  [[ ! -s "$dir/stale.out" ]] || fail "$label wrote a stale result"
  printf 'installed-smoke: %s sample, exact/empty/multiline/flag results, TypeScript/context/package and stale context passed\n' "$label"
done
