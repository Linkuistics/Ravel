#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'
trap 'echo "Error on line $LINENO" >&2' ERR

here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)
readonly here
repo_root=$(cd "$here/../../../../.." && pwd -P)
readonly repo_root
readonly prompt_file="$here/prompt.txt"
readonly fixture="$repo_root/docs/evaluations/writing-code-walkthroughs/baseline/fixtures/external-check-floor/targets/ocaml/check_floor.ml"
readonly skill="$repo_root/plugins/writing/craft/writing-code-walkthroughs/SKILL.md"
readonly expected_fixture_sha=2624183a8836364b5fdbcbeae7bf62de20d88550e6e2358aad13812da4cb0f0e
readonly expected_skill_sha=795846cb31237e20de5f24492dab4d1bce890d206225c306b6b4b0fee5cb8006

write_manifest() {
  local directory=$1 output=$2 file relative bytes digest
  : >"$output"
  while IFS= read -r file; do
    relative=${file#"$directory"/}
    bytes=$(wc -c <"$file" | tr -d ' ')
    digest=$(shasum -a 256 "$file" | awk '{print $1}')
    printf '%s\t%s\t%s\n' "$relative" "$bytes" "$digest" >>"$output"
  done < <(find "$directory" -type f -print | LC_ALL=C sort)
}

initialize() {
  local root=$1 auth_source=$2 actual_fixture_sha actual_skill_sha
  actual_fixture_sha=$(shasum -a 256 "$fixture" | awk '{print $1}')
  actual_skill_sha=$(shasum -a 256 "$skill" | awk '{print $1}')
  [[ "$actual_fixture_sha" == "$expected_fixture_sha" ]]
  [[ "$actual_skill_sha" == "$expected_skill_sha" ]]
  mkdir -p "$root/control-template" "$root/enabled-template/skills/writing-code-walkthroughs"
  cp "$auth_source" "$root/control-template/auth.json"
  cp "$auth_source" "$root/enabled-template/auth.json"
  cp "$skill" "$root/enabled-template/skills/writing-code-walkthroughs/SKILL.md"
  write_manifest "$root/control-template" "$root/control-template.manifest.tsv"
  write_manifest "$root/enabled-template" "$root/enabled-template.manifest.tsv"
  printf '%s\n' 'control enabled enabled control enabled control control enabled control enabled' >"$root/schedule.txt"
}

run_attempt() {
  local root=$1 arm=$2 repetition=$3 attempt=$4
  local template attempt_dir home run_dir prompt_value start_epoch end_epoch status
  template="$root/${arm}-template"
  attempt_dir="$root/${arm}-repetition-${repetition}-attempt-${attempt}"
  home="$attempt_dir/codex-home"
  run_dir="$attempt_dir/run-directory"
  mkdir -p "$home" "$run_dir/targets/ocaml"
  cp -R "$template"/. "$home"/
  cp "$fixture" "$run_dir/targets/ocaml/check_floor.ml"
  write_manifest "$template" "$attempt_dir/template-pre.manifest.tsv"
  write_manifest "$home" "$attempt_dir/home-pre.manifest.tsv"
  write_manifest "$run_dir" "$attempt_dir/run-pre.manifest.tsv"
  shasum -a 256 "$prompt_file" >"$attempt_dir/prompt.sha256"
  prompt_value="$(cat "$prompt_file"; printf x)"
  prompt_value=${prompt_value%x}
  [[ $(printf '%s' "$prompt_value" | shasum -a 256 | awk '{print $1}') == $(awk '{print $1}' "$attempt_dir/prompt.sha256") ]]
  date -u +%Y-%m-%dT%H:%M:%SZ >"$attempt_dir/start-utc.txt"
  start_epoch=$(date +%s)
  set +e
  env -u CODEX_CI -u CODEX_PERMISSION_PROFILE -u CODEX_SANDBOX \
    -u CODEX_SANDBOX_NETWORK_DISABLED -u CODEX_SESSION_ID \
    -u CODEX_THREAD_ID -u GROVE_SIGNAL_FILE -u HERDR_ENV \
    -u HERDR_PANE_ID -u HERDR_SOCKET_PATH -u HERDR_TAB_ID \
    -u HERDR_WORKSPACE_ID CODEX_HOME="$home" \
    timeout --signal=TERM --kill-after=30s 20m \
    codex exec --ignore-user-config --ignore-rules --ephemeral \
    --skip-git-repo-check --sandbox read-only --model gpt-5.4 \
    -c model_reasoning_effort='high' -c skills.bundled.enabled=false --json \
    --cd "$run_dir" "$prompt_value" \
    >"$attempt_dir/raw.jsonl" 2>"$attempt_dir/stderr.txt"
  status=$?
  set -e
  end_epoch=$(date +%s)
  date -u +%Y-%m-%dT%H:%M:%SZ >"$attempt_dir/end-utc.txt"
  printf '%s\n' "$status" >"$attempt_dir/exit-status.txt"
  printf '%s\n' "$((end_epoch - start_epoch))" >"$attempt_dir/wall-seconds.txt"
  write_manifest "$home" "$attempt_dir/home-post.manifest.tsv"
  write_manifest "$run_dir" "$attempt_dir/run-post.manifest.tsv"
  jq -rs '[.[] | select(.type == "item.completed" and .item.type == "agent_message")] | last | .item.text // ""' \
    "$attempt_dir/raw.jsonl" >"$attempt_dir/final.md"
  jq -c 'select(.type == "item.started" or .type == "item.completed") | select(.item.type == "command_execution" or .item.type == "mcp_tool_call" or .item.type == "web_search")' \
    "$attempt_dir/raw.jsonl" >"$attempt_dir/tool-events.jsonl"
  printf 'attempt=%s status=%s wall=%s final_bytes=%s tool_events=%s run_unchanged=%s\n' \
    "$attempt_dir" "$status" "$((end_epoch - start_epoch))" \
    "$(wc -c <"$attempt_dir/final.md" | tr -d ' ')" \
    "$(wc -l <"$attempt_dir/tool-events.jsonl" | tr -d ' ')" \
    "$(cmp -s "$attempt_dir/run-pre.manifest.tsv" "$attempt_dir/run-post.manifest.tsv" && printf yes || printf no)"
}

case ${1-} in
  init) initialize "$2" "$3" ;;
  run) run_attempt "$2" "$3" "$4" "$5" ;;
  *) echo "usage: $0 init ROOT AUTH_SOURCE | run ROOT ARM REPETITION ATTEMPT" >&2; exit 2 ;;
esac
