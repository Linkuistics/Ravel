#!/usr/bin/env bash
# Make one real LLM call through a harness, with the owner's sign-in, and prove
# the recipe of the book at the command-line seam (docs/specs/policy-evaluation.md,
# "LLM calls through a harness", "Credentials" and "Upgrades"). The real
# `ravel start-session` evaluates a Rhai policy in a sandbox home. The policy
# asks the harness one question with a fixed answer shape, parses the answer
# with ravel::json_parse, checks it with ravel::json_check, and selects a
# harmless fake program as the route. This script checks the route and the
# notice. A real call spends subscription usage, so no default task runs it.
#
#   harness-smoke.sh claude|codex
#
# Test seams, used by scripts/harness-smoke.test.sh and never by an owner:
#   HARNESS_SMOKE_PROGRAM  the harness program, instead of the one on PATH. It
#                          also skips the sign-in check, because a fake has none.
#   HARNESS_SMOKE_RAVEL    the Ravel binary, instead of a build of this checkout.
set -euo pipefail
IFS=$'\n\t'

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

fail() {
  printf 'harness-smoke: %s\n' "$*" >&2
  exit 1
}

[[ $# -eq 1 ]] || fail "usage: harness-smoke.sh claude|codex"
harness="$1"
case "$harness" in
  claude | codex) ;;
  *) fail "unknown harness '$harness' (expected claude or codex)" ;;
esac

# The program of the harness, and its sign-in.
program="${HARNESS_SMOKE_PROGRAM:-$(command -v "$harness" || true)}"
[[ -n "$program" && -x "$program" ]] ||
  fail "no '$harness' program found on PATH; install it, or put it on PATH, before this task can make a call"
program="$(cd "$(dirname "$program")" && pwd)/$(basename "$program")"
version="$("$program" --version 2>&1 | head -n 1)" || fail "$program --version failed"

# The sandbox. Ravel reads its settings from $HOME/.config/ravel and has no
# override, and a harness run receives the caller's HOME. The sandbox home
# therefore holds the settings and the policy, and a link to the sign-in store of
# each harness. A link is no read: the harness opens its own store.
sandbox="$(mktemp -d "${TMPDIR:-/tmp}/harness-smoke.XXXXXX")"
trap 'rm -rf "$sandbox"' EXIT
home="$sandbox/home" work="$sandbox/work" bin="$sandbox/bin"
mkdir -p "$home/.config/ravel" "$work" "$bin" "$sandbox/tmp"

owner_home="${HOME:?HOME is unset}"
link_store() {
  local relative="$1"
  if [[ -e "$owner_home/$relative" ]]; then
    mkdir -p "$(dirname "$home/$relative")"
    ln -s "$owner_home/$relative" "$home/$relative"
  fi
}
case "$harness" in
  claude)
    # macOS keeps the sign-in in the login keychain, which Claude Code finds
    # through HOME. Linux keeps it under ~/.claude.
    if [[ "$(uname -s)" == Darwin ]]; then link_store Library/Keychains; else link_store .claude; fi
    ;;
  codex) link_store .codex ;;
esac

# Run with the sandbox home, and with no API key in the environment, so that the
# call proves the sign-in and bills nothing else. The task may run inside a
# harness session of a driver, where the environment holds the channel of that
# session. RAVEL_EXIT_FILE would take the result of the nested session to it, and
# RAVEL_POLICY_RUN makes start-session refuse as policy_reentry_refused.
sandboxed() {
  env -u ANTHROPIC_API_KEY -u ANTHROPIC_AUTH_TOKEN -u OPENAI_API_KEY -u CODEX_API_KEY \
    -u RAVEL_EXIT_FILE -u RAVEL_POLICY_RUN \
    HOME="$home" TMPDIR="$sandbox/tmp" "$@"
}

if [[ -z "${HARNESS_SMOKE_PROGRAM:-}" ]]; then
  signed_in=0
  case "$harness" in
    claude) sandboxed "$program" auth status >/dev/null 2>&1 && signed_in=1 ;;
    codex) sandboxed "$program" login status >/dev/null 2>&1 && signed_in=1 ;;
  esac
  [[ "$signed_in" == 1 ]] ||
    fail "$harness has no sign-in; sign in to $harness as the owner and run this task again"
fi

# The Ravel binary that the task proves.
if [[ -n "${HARNESS_SMOKE_RAVEL:-}" ]]; then
  ravel="$HARNESS_SMOKE_RAVEL"
else
  (cd "$root" && cargo build --locked --quiet -p ravel) || fail "cargo build of ravel failed"
  ravel="$root/target/debug/ravel"
fi
[[ -x "$ravel" ]] || fail "no ravel binary at $ravel"

# The settings: one harness, one run.
jq -n --arg name "$harness" --arg program "$program" \
  '{ harnesses: { ($name): { program: $program } }, budgets: { harnessRuns: 1 } }' \
  >"$home/.config/ravel/settings.json"

# The route: a harmless program that records its arguments and ends the session.
cat >"$bin/smoke-route" <<EOF
#!/bin/sh
printf '%s\n' "\$@" > "$sandbox/route.argv"
exec "$ravel" end-session done
EOF
chmod +x "$bin/smoke-route"

# The question and its answer shape. 17 is prime, so the answer is fixed.
schema='{"type":"object","properties":{"prime":{"type":"string","enum":["yes","no"]}},"required":["prime"],"additionalProperties":false}'
question='Is 17 a prime number? Answer with the JSON object only.'

# What the policy calls differs by harness. The rest is the same.
case "$harness" in
  claude)
    # The schema travels as text in the argument. The value of the answer is in
    # `structured_output` of the one JSON document that --output-format json
    # writes. No --bare: that flag never reads the sign-in.
    call="ravel::run_harness(#{
        harness: \"claude\",
        args: [\"-p\", \"--model\", \"haiku\", \"--tools\", \"\", \"--no-session-persistence\",
               \"--output-format\", \"json\", \"--json-schema\", ravel::json_encode(schema)],
        stdin: \"$question\",
    })"
    answer='ravel::json_parse(run.stdout).structured_output'
    schema_source='let schema = ravel::json_parse(`'"$schema"'`);'
    ;;
  codex)
    # The schema travels as a file in the sandbox. An unconfined run can read it.
    # The final message, which is the answer, is all of stdout.
    printf '%s\n' "$schema" >"$work/answer-schema.json"
    call="ravel::run_harness(#{
        harness: \"codex\",
        args: [\"exec\", \"--ephemeral\", \"--skip-git-repo-check\", \"--sandbox\", \"read-only\",
               \"-c\", \"model_reasoning_effort=\\\"low\\\"\",
               \"--output-schema\", ravel::path_resolve(request.cwd, \"answer-schema.json\"), \"-\"],
        stdin: \"$question\",
    })"
    answer='ravel::json_parse(run.stdout)'
    schema_source='let schema = ravel::read_json("answer-schema.json").value;'
    ;;
esac

cat >"$home/.config/ravel/policy.rhai" <<EOF
export const SCHEMA_VERSION = 3;
export const VERSION = "harness-smoke-$harness";

fn select(request, context) {
    $schema_source
    let run = $call;
    if run.status != "exited" || run.exitCode != 0 {
        return #{ status: "refused", code: "smoke_call_failed",
            message: "$harness ended with status " + run.status + " and exit code " + run.exitCode,
            remedy: "stderr of the harness: " + run.stderr };
    }
    let reply = ravel::json_check($answer, schema);
    #{ status: "selected", program: "smoke-route", args: ["routed", reply.prime],
       provider: "$harness", model: "smoke", effort: "low", reason: "the fixed question" }
}
EOF

printf 'harness-smoke: %s (%s), real call, sandbox %s\n' "$harness" "$version" "$sandbox"
status=0
(cd "$work" && sandboxed env PATH="$bin:$PATH" \
  "$ravel" start-session --kind smoke --prompt "harness smoke" --timeout-ms 240000 --json \
  >"$sandbox/stdout" 2>"$sandbox/stderr") || status=$?

if [[ "$status" != 0 ]]; then
  printf 'harness-smoke: ravel exited %s\n' "$status" >&2
  cat "$sandbox/stderr" >&2
  fail "the call did not end in a session"
fi

# The checks: the session ended with the route's value, the route got the fixed
# answer, and the notice lists one run of this harness that exited.
[[ "$(cat "$sandbox/stdout")" == "done" ]] || fail "stdout is not 'done'"
[[ -f "$sandbox/route.argv" ]] || fail "the route never started"
[[ "$(cat "$sandbox/route.argv")" == $'routed\nyes' ]] || fail "the route got '$(tr '\n' ' ' <"$sandbox/route.argv")' and the fixed answer is 'routed yes'"
notice="$(head -n 1 "$sandbox/stderr")"
jq -e --arg h "$harness" '
  .harnessRuns | length == 1 and .[0].harness == $h and .[0].status == "exited" and .[0].exitCode == 0' \
  <<<"$notice" >/dev/null || fail "the notice does not list one exited run of $harness: $(jq -c '.harnessRuns' <<<"$notice")"
jq -e '.resolution | contains("/smoke-route (")' <<<"$notice" >/dev/null ||
  fail "the notice does not report the route as smoke-route: $(jq -c '.resolution' <<<"$notice")"
[[ "$(wc -l <"$sandbox/stderr" | tr -d ' ')" == 1 ]] || fail "stderr holds more than the notice"

printf 'harness-smoke: %s passed (%s)\n' "$harness" "$(jq -c '{route: "smoke-route", run: (.harnessRuns[0] | {harness, status, elapsedMs})}' <<<"$notice")"
