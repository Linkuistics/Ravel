#!/usr/bin/env bash
# Run an installed Ravel 1.1 release and a newer installation at the same time.
# The old release makes a native-only launch, and the new installation makes a
# directory launch. Each has its own temporary HOME and a fake managed program.
# A sandbox-exec guard denies both runs every read and write in the real HOME,
# so this check runs on macOS only.
set -euo pipefail
IFS=$'\n\t'
fail() { echo "installed-pair-smoke: $*" >&2; exit 1; }
[[ $# == 2 && -n "$1" && -n "$2" ]] || fail 'usage: installed-pair-smoke.sh OLD_EXECUTABLE NEW_PREFIX'
[[ "$(uname -s)" == Darwin ]] || fail 'this check runs on macOS only, because its HOME guard uses sandbox-exec'
command -v sandbox-exec >/dev/null || fail 'sandbox-exec is not available'

old="$1"
[[ "$old" == /* && -x "$old" ]] || fail "OLD must be the absolute path of an installed ravel executable: $old"
old_prefix="$(cd "$(dirname "$old")/.." && pwd -P)"
new_prefix="$(cd "$2" && pwd -P)"
new="$new_prefix/bin/ravel"
[[ -x "$new" ]] || fail "NEW holds no bin/ravel: $new_prefix"
real_home="$(cd "$HOME" && pwd -P)"
[[ "$real_home" != *[\"\\]* ]] || fail 'the real HOME path holds a quote or a backslash'
[[ "$("$old" --version)" == 'ravel 1.'* ]] || fail "OLD is not a Ravel 1.x release: $("$old" --version)"

work="$(mktemp -d)"
# Retained snapshots are read-only, so cleanup first restores write access.
# shellcheck disable=SC2064 # remove precisely the owned directory
trap "chmod -R u+w '$work' 2>/dev/null; rm -rf '$work'" EXIT
work="$(cd "$work" && pwd -P)"
for subject in "$old_prefix" "$new_prefix" "$work"; do
  case "$subject/" in
  "$real_home"/*) fail "$subject is in the real HOME, which the guard denies" ;;
  esac
done

guard="(version 1)(allow default)(deny file-read* file-write* (subpath \"$real_home\"))"
guarded() { sandbox-exec -p "$guard" "$@"; }

# Digest every file and link of a tree, or record its absence.
tree_digest() {
  if [[ ! -e "$1" ]]; then
    printf 'absent\n'
    return
  fi
  (
    cd "$1"
    find . \( -type f -o -type l \) -print | LC_ALL=C sort | while IFS= read -r entry; do
      if [[ -L "$entry" ]]; then
        printf 'link %s %s\n' "$(readlink "$entry")" "$entry"
      else
        printf 'file %s %s %s\n' "$(stat -f %Lp "$entry")" "$(shasum -a 256 <"$entry" | cut -d' ' -f1)" "$entry"
      fi
    done
  ) | shasum -a 256 | cut -d' ' -f1
}
subjects=("$old_prefix" "$new_prefix" "$real_home/.config/ravel" "$real_home/.cache/ravel")
digests() { for subject in "${subjects[@]}"; do printf '%s %s\n' "$(tree_digest "$subject")" "$subject"; done; }
digests >"$work/before"

# The guard must deny a read of the real HOME, or it proves nothing.
if guarded /bin/ls "$real_home" >/dev/null 2>&1; then
  fail 'control: the guard admitted a read of the real HOME'
fi

tools="$work/tools"
mkdir -p "$tools"
for utility in cat env grep ls mkdir rm sleep; do
  resolved="$(PATH=/usr/bin:/bin command -v "$utility")" || fail "missing system utility $utility"
  ln -s "$resolved" "$tools/$utility"
done
pair="$work/pair"
mkdir -p "$pair"
# The fake managed program records what it received. Each run waits until
# the other run has started, so the two sessions overlap.
# shellcheck disable=SC2016 # the fixture expands these when the child runs
printf '%s\n' '#!/bin/sh' 'set -eu' 'role="$1"; prompt="$2"; out="$PAIR/$role"' \
  'printf "%s" "$prompt" >"$out.prompt"' 'printf "%s" "$HOME" >"$out.home"' \
  'printf "%s" "${RAVEL_SKILL_DIRECTORIES-<absent>}" >"$out.directories"' \
  'if [ "$#" -ge 3 ]; then cat -- "$3" >"$out.mandatory"; printf "%s" "$3" >"$out.mandatory-path"; fi' \
  ': >"$out.started"' 'other=old; [ "$role" = new ] || other=new' 'waited=0' \
  'while [ ! -e "$PAIR/$other.started" ]; do' \
  '  waited=$((waited + 1)); [ "$waited" -le 600 ] || exit 9; sleep 0.1' 'done' \
  'exec "$PAIR_FRONT" end-session -- "$role result"' >"$work/agent"
chmod +x "$work/agent"

old_home="$work/old-home"
new_home="$work/new-home"
mkdir -p "$old_home/.config/ravel" "$new_home/.config/ravel" "$work/old-cwd" "$work/new-cwd" \
  "$work/old-tmp" "$work/new-tmp" "$work/source/craft"
cat >"$old_home/.config/ravel/policy.ts" <<'TS'
import { definePolicy } from "ravel/sdk";
export const policy = definePolicy({
  schemaVersion: 2,
  version: "pair-old",
  select(request) {
    return { status: "selected", program: request.params.fixture, args: ["old", request.prompt],
      provider: "fixture", model: "fixture", effort: "high", reason: "pair old" };
  },
});
TS
cat >"$new_home/.config/ravel/config.rhai" <<'RHAI'
export const SCHEMA_VERSION = 4;
export const VERSION = "pair-new";
fn select(request, context) {
    if request.skills == () { throw "skill delivery is disabled"; }
    #{ status: "selected", program: request.params.fixture,
       args: ["new", request.prompt, request.skills.mandatory[0].path],
       provider: "fixture", model: "fixture", effort: "high", reason: "pair new" }
}
RHAI
printf 'Read this mandatory guidance first.\n' >"$work/source/mandatory.md"
printf '%s\n' '---' 'name: craft' 'description: A pair-smoke craft skill.' '---' 'Craft guidance.' \
  >"$work/source/craft/SKILL.md"
skills_settings() {
  printf '{"skills":{"schemaVersion":1,"sources":[{"id":"pair","type":"local","path":"%s","roots":["."]}],"mandatory":[{"source":"pair","path":"mandatory.md"}]}}\n' "$work/source"
}
skills_settings >"$new_home/.config/ravel/settings.json"
caller=$'Pair task: \xc3\xbcn\xc3\xafcode\nCaller task\nend'
printf '%s' "$caller" >"$work/caller"

launch() {
  local role="$1" front="$2" home="$3"
  (cd "$work/$role-cwd" && guarded /usr/bin/env -i HOME="$home" TMPDIR="$work/$role-tmp" \
    PATH="$tools" PAIR="$pair" PAIR_FRONT="$front" \
    "$front" start-session --kind pair --param "fixture=$work/agent" --prompt "$caller" --json) \
    >"$work/$role.result" 2>"$work/$role.diagnostics"
}
launch old "$old" "$old_home" &
old_job=$!
launch new "$new" "$new_home" &
new_job=$!
old_status=0
wait "$old_job" || old_status=$?
new_status=0
wait "$new_job" || new_status=$?
[[ "$old_status" == 0 && "$new_status" == 0 ]] ||
  fail "a launch failed. The 1.1 native-only launch: status $old_status, $(cat "$work/old.diagnostics")
The directory launch: status $new_status, $(cat "$work/new.diagnostics")"

[[ "$(cat "$work/old.result")" == 'old result' ]] || fail 'the 1.1 result changed'
[[ "$(cat "$work/new.result")" == 'new result' ]] || fail 'the directory result changed'
cmp -s "$work/caller" "$pair/old.prompt" || fail 'the 1.1 launch did not receive the caller prompt unchanged'
[[ "$(cat "$pair/old.directories")" == '<absent>' ]] || fail 'the 1.1 launch received RAVEL_SKILL_DIRECTORIES'
[[ "$(cat "$pair/old.home")" == "$old_home" ]] || fail 'the 1.1 launch saw another HOME'
[[ "$(cat "$pair/new.home")" == "$new_home" ]] || fail 'the directory launch saw another HOME'
grep -Fq '"reason":"pair old"' "$work/old.diagnostics" || fail 'the 1.1 launch did not use its policy'
grep -Fq '"reason":"pair new"' "$work/new.diagnostics" || fail 'the directory launch did not use its script'

new_prompt="$(cat "$pair/new.prompt")"
[[ "$new_prompt" != "$caller" ]] || fail 'the directory launch received no preamble'
[[ "$new_prompt" == *"$caller"* ]] || fail 'the directory launch lost the caller bytes'
cache="$new_home/.cache/ravel/skills/v1/"
descriptor="$(cat "$pair/new.directories")"
mandatory="$(cat "$pair/new.mandatory-path")"
[[ "$descriptor" == "$cache"* && -f "$descriptor" ]] || fail "RAVEL_SKILL_DIRECTORIES is not a retained descriptor: $descriptor"
[[ "$mandatory" == "$cache"* ]] || fail "the mandatory file is not retained under the second HOME: $mandatory"
[[ "$new_prompt" == *"\"$descriptor\""* && "$new_prompt" == *"\"$mandatory\""* ]] ||
  fail 'the preamble does not name the descriptor and the mandatory file'
cmp -s "$work/source/mandatory.md" "$pair/new.mandatory" || fail 'the managed program read other mandatory bytes'
[[ ! -e "$old_home/.cache" ]] || fail 'the 1.1 launch created a cache'
grep -Eo '"buildId":"[0-9a-f]+"' "$work/new.diagnostics" | head -n 1 | cut -d'"' -f4 >"$work/build-id" ||
  fail 'the directory notice names no build identity'
for role in old new; do
  [[ -z "$(ls -A "$work/$role-tmp")" ]] || fail "the $role launch left temporary session resources"
done

# A settings file with skills makes the 1.1 release refuse before any launch.
skills_settings >"$old_home/.config/ravel/settings.json"
rm -f "$pair/old.started"
refusal=0
launch old "$old" "$old_home" || refusal=$?
[[ "$refusal" == 2 && ! -s "$work/old.result" ]] || fail "the 1.1 release did not refuse skills settings with status 2: $refusal"
grep -Fq 'settings_invalid' "$work/old.diagnostics" || fail 'the 1.1 refusal is not settings_invalid'
[[ ! -e "$pair/old.started" ]] || fail 'the refused 1.1 launch started its managed program'

digests >"$work/after"
cmp -s "$work/before" "$work/after" || fail "a guarded subject changed: $(diff "$work/before" "$work/after" || true)"
printf 'installed-pair-smoke: %s native-only and %s directory launches overlapped, each under its own HOME\n' \
  "$("$old" --version)" "$("$new" --version)"
printf 'installed-pair-smoke: directory build %s\n' "$(cat "$work/build-id")"
printf 'installed-pair-smoke: the 1.1 release refused skills settings with status 2\n'
printf 'installed-pair-smoke: unchanged subjects:\n%s\n' "$(cat "$work/after")"
