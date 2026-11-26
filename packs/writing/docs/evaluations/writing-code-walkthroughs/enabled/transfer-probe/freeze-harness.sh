#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'
trap 'echo "Error on line $LINENO" >&2' ERR

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
  local output=$1 auth_source=$2
  mkdir -p "$output/codex-home" "$output/run-directory"
  cp "$auth_source" "$output/codex-home/auth.json"
  write_manifest "$output/codex-home" "$output/home-pre.manifest.tsv"
  write_manifest "$output/run-directory" "$output/run-pre.manifest.tsv"
}

run_context() {
  local output=$1 prompt_file=$2 prompt_value start_epoch end_epoch status
  shasum -a 256 "$prompt_file" >"$output/prompt.sha256"
  prompt_value="$(cat "$prompt_file"; printf x)"
  prompt_value=${prompt_value%x}
  date -u +%Y-%m-%dT%H:%M:%SZ >"$output/start-utc.txt"
  start_epoch=$(date +%s)
  set +e
  env -u CODEX_CI -u CODEX_PERMISSION_PROFILE -u CODEX_SANDBOX \
    -u CODEX_SANDBOX_NETWORK_DISABLED -u CODEX_SESSION_ID \
    -u CODEX_THREAD_ID -u GROVE_SIGNAL_FILE -u HERDR_ENV \
    -u HERDR_PANE_ID -u HERDR_SOCKET_PATH -u HERDR_TAB_ID \
    -u HERDR_WORKSPACE_ID CODEX_HOME="$output/codex-home" \
    timeout --signal=TERM --kill-after=30s 20m \
    codex exec --ignore-user-config --ignore-rules --ephemeral \
    --skip-git-repo-check --sandbox read-only --model gpt-5.4 \
    -c model_reasoning_effort='high' -c skills.bundled.enabled=false --json \
    --cd "$output/run-directory" "$prompt_value" \
    >"$output/raw.jsonl" 2>"$output/stderr.txt"
  status=$?
  set -e
  end_epoch=$(date +%s)
  date -u +%Y-%m-%dT%H:%M:%SZ >"$output/end-utc.txt"
  printf '%s\n' "$status" >"$output/exit-status.txt"
  printf '%s\n' "$((end_epoch - start_epoch))" >"$output/wall-seconds.txt"
  write_manifest "$output/codex-home" "$output/home-post.manifest.tsv"
  write_manifest "$output/run-directory" "$output/run-post.manifest.tsv"
  jq -rs '[.[] | select(.type == "item.completed" and .item.type == "agent_message")] | last | .item.text // ""' \
    "$output/raw.jsonl" >"$output/final.md"
  jq -c 'select(.type == "item.started" or .type == "item.completed") | select(.item.type == "command_execution" or .item.type == "mcp_tool_call" or .item.type == "web_search")' \
    "$output/raw.jsonl" >"$output/tool-events.jsonl"
  printf 'status=%s wall=%s final_bytes=%s tool_events=%s run_unchanged=%s\n' \
    "$status" "$((end_epoch - start_epoch))" \
    "$(wc -c <"$output/final.md" | tr -d ' ')" \
    "$(wc -l <"$output/tool-events.jsonl" | tr -d ' ')" \
    "$(cmp -s "$output/run-pre.manifest.tsv" "$output/run-post.manifest.tsv" && printf yes || printf no)"
}

case ${1-} in
  init) initialize "$2" "$3" ;;
  run) run_context "$2" "$3" ;;
  *) printf 'usage: %s init OUTPUT AUTH_SOURCE | run OUTPUT PROMPT_FILE\n' "$0" >&2; exit 2 ;;
esac
