#!/usr/bin/env bash
# Test scripts/harness-smoke.sh against fake harnesses, so that no test makes a
# real call. Each success carries a firing control: the same script with a
# fake that answers otherwise must fail, so a check that read clean for every
# input would fail here. The real calls are the opt-in tasks harness:smoke:*.
#
# Every run uses a fake real home, so no test links the owner's sign-in store.
# The isolation controls of the sandbox home (docs/specs/configuration-evaluation.md,
# "Sandbox home for real harness runs") prove that each inherited variable
# refuses before the fake harness launches.
set -euo pipefail
IFS=$'\n\t'

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
smoke="$root/scripts/harness-smoke.sh"
work="$(mktemp -d "${TMPDIR:-/tmp}/harness-smoke-test.XXXXXX")"
trap 'rm -rf "$work"' EXIT

(cd "$root" && cargo build --locked --quiet -p ravel)
export HARNESS_SMOKE_RAVEL="$root/target/debug/ravel"

# The caller's own values of the refused names must not decide these tests.
unset CLAUDE_CONFIG_DIR CLAUDE_SECURESTORAGE_CONFIG_DIR CLAUDE_CODE_PLUGIN_CACHE_DIR \
  CLAUDE_CODE_PLUGIN_DIRS CLAUDE_CODE_PLUGIN_SEED_DIR CODEX_HOME

fail() {
  printf 'harness-smoke.test: %s\n' "$*" >&2
  exit 1
}

# The fake real home: a sign-in store for each harness, beside configuration
# that a sandbox must never expose.
owner="$work/owner"
mkdir -p "$owner/Library/Keychains" "$owner/.claude/skills/real" "$owner/.codex/skills/real"
printf 'keychain\n' >"$owner/Library/Keychains/login.keychain-db"
printf '{"claudeAiOauth":{}}\n' >"$owner/.claude/.credentials.json"
printf '{"real":true}\n' >"$owner/.claude/settings.json"
printf -- '---\nname: real\n---\n' >"$owner/.claude/skills/real/SKILL.md"
printf '{"auth_mode":"chatgpt"}\n' >"$owner/.codex/auth.json"
printf 'model = "real"\n' >"$owner/.codex/config.toml"
printf -- '---\nname: real\n---\n' >"$owner/.codex/skills/real/SKILL.md"

# A digest of a tree: each path, its type, its link target and its contents.
digest() {
  (cd "$1" && find . -print | LC_ALL=C sort | while IFS= read -r path; do
    if [[ -L "$path" ]]; then
      printf 'L %s %s\n' "$path" "$(readlink "$path")"
    elif [[ -f "$path" ]]; then
      printf 'F %s %s\n' "$path" "$(cksum <"$path")"
    else
      printf 'D %s\n' "$path"
    fi
  done) | cksum
}
owner_digest="$(digest "$owner")"

if [[ "$(uname -s)" == Darwin ]]; then claude_store=Library/Keychains; else claude_store=.claude/.credentials.json; fi
codex_store=.codex/auth.json

# A fake harness that records each launch, its arguments, its CODEX_HOME and
# its standard input, and then runs $2, a shell fragment.
fake() {
  local path="$work/$1"
  cat >"$path" <<FAKE
#!/bin/sh
printf '%s\n' "\$*" >> "$path.launched"
if [ "\$1" = "--version" ]; then echo "fake 1.0"; exit 0; fi
printf '%s\n' "\$@" > "$path.argv"
printf '%s\n' "\${CODEX_HOME-unset}" > "$path.codex-home"
cat > "$path.stdin"
$2
FAKE
  chmod +x "$path"
  printf '%s' "$path"
}

# Run the smoke script: label, harness, program, then NAME=VALUE assignments.
smoke_run() {
  local label="$1" harness="$2" program="$3"
  shift 3
  env HOME="$owner" HARNESS_SMOKE_PROGRAM="$program" "$@" bash "$smoke" "$harness" >"$work/$label.log" 2>&1
}

expect_pass() {
  local label="$1" harness="$2" program="$3"
  shift 3
  smoke_run "$label" "$harness" "$program" "$@" || {
    cat "$work/$label.log" >&2
    fail "$label: the task failed and should pass"
  }
  grep -q "harness-smoke: $harness passed" "$work/$label.log" || fail "$label: no pass line"
}

expect_fail() {
  local label="$1" harness="$2" program="$3" pattern="$4"
  shift 4
  if smoke_run "$label" "$harness" "$program" "$@"; then
    cat "$work/$label.log" >&2
    fail "$label: the task passed and should fail"
  fi
  grep -q "$pattern" "$work/$label.log" || {
    cat "$work/$label.log" >&2
    fail "$label: the failure does not match '$pattern'"
  }
}

# A refusal: the task fails with the pattern, and the fake never launched.
expect_refusal() {
  local label="$1" harness="$2" pattern="$3" program
  shift 3
  program="$(fake "$label" 'echo "{\"type\":\"result\",\"structured_output\":{\"prime\":\"yes\"}}"; echo "{\"prime\":\"yes\"}"')"
  expect_fail "$label" "$harness" "$program" "$pattern" "$@"
  [[ ! -e "$program.launched" ]] || fail "$label: the harness launched before the refusal: $(cat "$program.launched")"
}

# The link lines of a passing run: exactly one, for the store of the harness.
expect_links() {
  local label="$1" store="$2" log="$work/$1.log"
  [[ "$(grep -c '^harness-smoke: link ' "$log")" == 1 ]] || fail "$label: the run does not list exactly one link: $(grep '^harness-smoke: link ' "$log" | tr '\n' ' ')"
  grep -qx "harness-smoke: link $store -> $owner/$store" "$log" || fail "$label: the run does not list the link $store"
  grep -Eq '^harness-smoke: configuration directory .*/home/\.(claude|codex)$' "$log" ||
    fail "$label: the run does not record a sandbox configuration directory"
}

# Claude Code: the answer is in structured_output of one JSON document.
claude="$(fake claude-yes 'echo "{\"type\":\"result\",\"structured_output\":{\"prime\":\"yes\"}}"')"
expect_pass claude-yes claude "$claude"
expect_links claude-yes "$claude_store"
grep -q '^harness-smoke: configuration directory .*/home/\.claude$' "$work/claude-yes.log" || fail "claude: the configuration directory is not the sandbox .claude"
grep -qx -- '--json-schema' "$claude.argv" || fail "claude: --json-schema is not in the arguments"
grep -qx -- '--output-format' "$claude.argv" || fail "claude: --output-format is not in the arguments"
grep -q 'Is 17 a prime number' "$claude.stdin" || fail "claude: the question did not reach stdin"
[[ "$(cat "$claude.codex-home")" == unset ]] || fail "claude: the run set CODEX_HOME"
expect_fail claude-no claude \
  "$(fake claude-no 'echo "{\"structured_output\":{\"prime\":\"no\"}}"')" "fixed answer is 'routed yes'"
expect_fail claude-no-structured claude \
  "$(fake claude-none 'echo "{\"type\":\"result\",\"result\":\"yes\"}"')" "smoke_call_failed\|structured_output\|the call did not end"
expect_fail claude-exit-1 claude \
  "$(fake claude-bad 'echo "rate limited" >&2; exit 1')" "rate limited"

# Codex: the schema is a file that the harness is told to read, and the answer
# is the whole of stdout.
codex="$(fake codex-yes 'echo "{\"prime\":\"yes\"}"')"
expect_pass codex-yes codex "$codex"
expect_links codex-yes "$codex_store"
[[ "$(cat "$codex.codex-home")" == */home/.codex ]] || fail "codex: CODEX_HOME is '$(cat "$codex.codex-home")', not the sandbox .codex"
grep -qx -- '--output-schema' "$codex.argv" || fail "codex: --output-schema is not in the arguments"
schema_file="$(grep -A1 -x -- '--output-schema' "$codex.argv" | tail -n 1)"
[[ "$schema_file" == */answer-schema.json ]] || fail "codex: the schema argument is '$schema_file'"
expect_fail codex-no codex \
  "$(fake codex-no 'echo "{\"prime\":\"no\"}"')" "fixed answer is 'routed yes'"
expect_fail codex-bad-shape codex \
  "$(fake codex-shape 'echo "{\"prime\":\"yes\",\"extra\":1}"')" "json_mismatch\|did not end in a session"

# The isolation controls. An inherited CLAUDE_CONFIG_DIR that names a decoy real
# directory refuses before the harness launches, and the decoy is unchanged.
decoy="$work/decoy-claude"
mkdir -p "$decoy/skills/decoy"
printf '{"decoy":true}\n' >"$decoy/settings.json"
printf -- '---\nname: decoy\n---\n' >"$decoy/skills/decoy/SKILL.md"
decoy_digest="$(digest "$decoy")"
expect_refusal claude-config-dir claude "holds CLAUDE_CONFIG_DIR" CLAUDE_CONFIG_DIR="$decoy"
[[ "$(digest "$decoy")" == "$decoy_digest" ]] || fail "claude-config-dir: the decoy directory changed"
expect_refusal claude-config-dir-empty claude "holds CLAUDE_CONFIG_DIR" CLAUDE_CONFIG_DIR=
expect_refusal claude-securestorage claude "holds CLAUDE_SECURESTORAGE_CONFIG_DIR" CLAUDE_SECURESTORAGE_CONFIG_DIR="$owner/.claude"
expect_refusal claude-plugin-cache claude "holds CLAUDE_CODE_PLUGIN_CACHE_DIR" CLAUDE_CODE_PLUGIN_CACHE_DIR="$decoy"
expect_refusal claude-plugin-dirs claude "holds CLAUDE_CODE_PLUGIN_DIRS" CLAUDE_CODE_PLUGIN_DIRS="$decoy"
expect_refusal claude-plugin-seed claude "holds CLAUDE_CODE_PLUGIN_SEED_DIR" CLAUDE_CODE_PLUGIN_SEED_DIR="$decoy"

# An inherited CODEX_HOME in the real home refuses, also through an alias of
# the real home and as an empty value. One outside the real home gives way to
# the sandbox .codex.
ln -s "$owner" "$work/owner-alias"
expect_refusal codex-home-real codex "CODEX_HOME=.* is in the real home" CODEX_HOME="$owner/.codex"
expect_refusal codex-home-alias codex "CODEX_HOME=.* is in the real home" CODEX_HOME="$work/owner-alias/.codex"
expect_refusal codex-home-empty codex "CODEX_HOME=.* is in the real home" CODEX_HOME=
mkdir -p "$work/elsewhere/.codex"
codex_out="$(fake codex-elsewhere 'echo "{\"prime\":\"yes\"}"')"
expect_pass codex-home-elsewhere codex "$codex_out" CODEX_HOME="$work/elsewhere/.codex"
[[ "$(cat "$codex_out.codex-home")" == */home/.codex ]] || fail "codex-home-elsewhere: CODEX_HOME is '$(cat "$codex_out.codex-home")', not the sandbox .codex"
grep -q 'outside the real home; this run uses the sandbox .codex' "$work/codex-home-elsewhere.log" ||
  fail "codex-home-elsewhere: the run does not report the replaced CODEX_HOME"

# A harness that replaces the sign-in link with a file fails the run, with the
# remedy to sign in again. The real store is unchanged.
expect_fail claude-link-replaced claude \
  "$(fake claude-replace "rm \"\$HOME/$claude_store\"; mkdir \"\$HOME/$claude_store\"; echo '{\"type\":\"result\",\"structured_output\":{\"prime\":\"yes\"}}'")" \
  "replaced the sign-in link .*sign in to claude again in the real home"
expect_fail codex-link-replaced codex \
  "$(fake codex-replace "rm \"\$CODEX_HOME/auth.json\"; : > \"\$CODEX_HOME/auth.json\"; echo '{\"prime\":\"yes\"}'")" \
  "replaced the sign-in link .*sign in to codex again in the real home"

# A real home without the sign-in store names it.
mkdir -p "$work/bare"
if env HOME="$work/bare" HARNESS_SMOKE_PROGRAM="$claude" bash "$smoke" claude >"$work/bare.log" 2>&1; then
  fail "a home without a sign-in store passed"
fi
grep -q "no sign-in store at $work/bare/$claude_store" "$work/bare.log" || fail "a missing sign-in store is not named: $(cat "$work/bare.log")"

# A missing program, and an unknown harness, name what is missing.
expect_fail missing-program claude "$work/does-not-exist" "no 'claude' program found"
if bash "$smoke" nobody >"$work/unknown.log" 2>&1; then fail "an unknown harness passed"; fi
grep -q "unknown harness 'nobody'" "$work/unknown.log" || fail "an unknown harness is not named"

# No run changed the fake real home.
[[ "$(digest "$owner")" == "$owner_digest" ]] || fail "a run changed the real home"

printf 'harness-smoke.test: passed\n'
