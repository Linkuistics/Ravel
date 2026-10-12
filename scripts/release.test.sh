#!/usr/bin/env bash
# Local release fixtures; GitHub is the only publishing service replaced.
set -euo pipefail
IFS=$'\n\t'

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
scratch="$(mktemp -d)"
trap 'rm -rf "$scratch"' EXIT
export JJ_CONFIG="$scratch/jj.toml"
printf '[user]\nname = "Release Test"\nemail = "release@example.invalid"\n' >"$JJ_CONFIG"
export GIT_AUTHOR_NAME='Release Test' GIT_COMMITTER_NAME='Release Test'
export GIT_AUTHOR_EMAIL='release@example.invalid' GIT_COMMITTER_EMAIL='release@example.invalid'
real_git="$(command -v git)"
export RELEASE_TEST_GIT="$real_git" RELEASE_TEST_LOG="$scratch/published"
export RELEASE_TEST_DIST="$scratch/source/target/dist"
export RELEASE_TEST_NOTES="$scratch/release-notes"
mkdir -p "$scratch/bin" "$scratch/source/scripts" "$scratch/source/target/dist"
cat >"$scratch/bin/gh" <<'SH'
#!/usr/bin/env bash
set -euo pipefail
if [[ "$1 $2" == 'release view' ]]; then
  printf '%s\n' '{"isDraft":false,"assets":[{"name":"ravel-v1.2.3-aarch64-apple-darwin.tar.xz","size":1},{"name":"ravel-v1.2.3-aarch64-unknown-linux-gnu.tar.xz","size":1},{"name":"ravel-v1.2.3-x86_64-unknown-linux-gnu.tar.xz","size":1}]}'
elif [[ "$1 $2" == 'release download' ]]; then
  while (($#)); do
    if [[ "$1" == --dir ]]; then shift; cp "$RELEASE_TEST_DIST/"*.tar.xz "$1/"
      if [[ "${RELEASE_TEST_DOWNLOAD_CORRUPT:-0}" == 1 ]]; then printf 'corrupt\n' >>"$1/ravel-v1.2.3-aarch64-apple-darwin.tar.xz"; fi
    fi
    shift
  done
elif [[ "$1 $2" == 'release create' ]]; then
  while (($#)); do
    if [[ "$1" == '--notes-file' ]]; then
      shift
      cat "$1" >"$RELEASE_TEST_NOTES"
    fi
    shift
  done
  printf 'published\n' >>"$RELEASE_TEST_LOG"
fi
SH
cat >"$scratch/bin/git" <<'SH'
#!/usr/bin/env bash
set -euo pipefail
# Reject raw Git mutations in the jj tap, as the repository contract does.
if [[ "${1:-}" == '-C' && -d "$2/.jj" ]]; then
  case "${3:-}" in add|commit|push) exit 88 ;; esac
fi
exec "$RELEASE_TEST_GIT" "$@"
SH
chmod +x "$scratch/bin/gh" "$scratch/bin/git"
export PATH="$scratch/bin:$PATH"

fail() {
  echo "FAIL: $*" >&2
  exit 1
}
cp "$repo_root/scripts/release-publish.sh" "$repo_root/scripts/release-common.sh" "$scratch/source/scripts/"
mkdir -p "$scratch/source/scripts/templates"
cp "$repo_root/scripts/templates/ravel.rb.tmpl" "$scratch/source/scripts/templates/"
printf '## Unreleased\n\n## v1.2.3\n\n- Meaningful release notes.\n' >"$scratch/source/CHANGELOG.md"
printf "/target/\n" >"$scratch/source/.gitignore"
"$real_git" init -q "$scratch/source"
"$real_git" -C "$scratch/source" add .
"$real_git" -C "$scratch/source" commit -qm fixture
"$real_git" -C "$scratch/source" tag v1.2.3
# Real manifest/rendering, only the binaries are harmless fixture files.
# shellcheck source=scripts/release-common.sh
source "$repo_root/scripts/release-common.sh"
for target in "${TARGETS[@]}"; do
  top="ravel-v1.2.3-$target"
  mkdir -p "$scratch/stage/$top"
  while IFS= read -r entry; do
    mkdir -p "$(dirname "$scratch/stage/$top/$entry")"
    printf 'fixture %s\n' "$entry" >"$scratch/stage/$top/$entry"
  done < <(archive_manifest)
  while IFS= read -r entry; do chmod +x "$scratch/stage/$top/$entry"; done < <(archive_executables)
  pack_archive "$scratch/stage" "$top" "$scratch/source/target/dist/$top.tar.xz"
done
render_archive_formula "$scratch/source/target/dist" 1.2.3 "$repo_root/scripts/templates/ravel.rb.tmpl" "$scratch/source/target/dist/ravel.rb"
cp "$scratch/source/target/dist/ravel.rb" "$scratch/expected-formula"

"$real_git" init -q --bare "$scratch/remote.git"
jj git init --colocate "$scratch/tap" >/dev/null
mkdir -p "$scratch/tap/Formula"
printf 'old formula\n' >"$scratch/tap/Formula/ravel.rb"
printf 'separate command\n' >"$scratch/tap/Formula/ravel-lite.rb"
jj -R "$scratch/tap" describe -m fixture >/dev/null
jj -R "$scratch/tap" git remote add origin "$scratch/remote.git"
jj -R "$scratch/tap" git push --named main=@ >/dev/null
jj -R "$scratch/tap" new main >/dev/null
export RAVEL_TAP_DIR="$scratch/tap"

# A dirty tap must be refused before creating a GitHub Release or copying files.
printf 'unrelated work\n' >"$scratch/tap/notes.txt"
if bash "$scratch/source/scripts/release-publish.sh" >"$scratch/output" 2>&1; then
  echo 'FAIL: publishing accepted a dirty tap' >&2
  exit 1
fi
if [[ -e "$RELEASE_TEST_LOG" ]]; then
  echo 'FAIL: dirty tap was detected only after GitHub publication' >&2
  exit 1
fi
[[ "$(cat "$scratch/tap/Formula/ravel.rb")" == 'old formula' ]]
[[ "$(cat "$scratch/tap/notes.txt")" == 'unrelated work' ]]
rm "$scratch/tap/notes.txt"

# Diverged jj tap refuses before any publication, preserving its local work.
jj -R "$scratch/tap" describe -m 'local tap work' >/dev/null
printf 'local commit\n' >"$scratch/tap/local.txt"
jj -R "$scratch/tap" bookmark set main -r @ >/dev/null
jj -R "$scratch/tap" new main >/dev/null
if bash "$scratch/source/scripts/release-publish.sh" >"$scratch/output" 2>&1; then fail 'accepted diverged tap'; fi
grep -Fq 'differs from origin' "$scratch/output"
[[ ! -e "$RELEASE_TEST_LOG" ]]
# Seal and push this fixture-only unrelated change; publishing must retain it.
jj -R "$scratch/tap" git push -b main >/dev/null
# Missing target and stale formula must refuse before GitHub or tap writes.
archive="$scratch/source/target/dist/ravel-v1.2.3-x86_64-unknown-linux-gnu.tar.xz"
mv "$archive" "$scratch/hidden.tar.xz"
if bash "$scratch/source/scripts/release-publish.sh" >"$scratch/output" 2>&1; then fail 'accepted missing target'; fi
[[ ! -e "$RELEASE_TEST_LOG" ]]
mv "$scratch/hidden.tar.xz" "$archive"
printf 'stale formula\n' >"$scratch/source/target/dist/ravel.rb"
if bash "$scratch/source/scripts/release-publish.sh" >"$scratch/output" 2>&1; then fail 'accepted stale formula'; fi
[[ ! -e "$RELEASE_TEST_LOG" ]]
cp "$scratch/expected-formula" "$scratch/source/target/dist/ravel.rb"
# A fourth archive is unsmoked input and must never reach GitHub.
printf 'stale archive\n' >"$scratch/source/target/dist/stray.tar.xz"
if bash "$scratch/source/scripts/release-publish.sh" >"$scratch/output" 2>&1; then fail 'accepted extra archive'; fi
[[ ! -e "$RELEASE_TEST_LOG" ]]
rm "$scratch/source/target/dist/stray.tar.xz"
# A jj tap must publish through jj, update the intended bookmark, and end clean.
bash "$scratch/source/scripts/release-publish.sh"
"$real_git" --git-dir="$scratch/remote.git" show main:Formula/ravel.rb >"$scratch/remote-formula"
cmp "$scratch/expected-formula" "$scratch/remote-formula"
[[ -z "$("$real_git" -C "$scratch/tap" status --porcelain)" ]]
[[ "$(cat "$RELEASE_TEST_LOG")" == 'published' ]]
grep -Fxq -- '- Meaningful release notes.' "$RELEASE_TEST_NOTES"

# Recovery resumes only tap publication, retaining GitHub and unrelated formula.
tap_before="$(jj -R "$scratch/tap" log --no-graph -r main -T commit_id)"
if RELEASE_TEST_DOWNLOAD_CORRUPT=1 bash "$scratch/source/scripts/release-publish.sh" --tap-only >"$scratch/output" 2>&1; then fail 'accepted corrupt published archive'; fi
grep -Fq 'differs from validated local archive' "$scratch/output"
[[ "$(jj -R "$scratch/tap" log --no-graph -r main -T commit_id)" == "$tap_before" ]]
bash "$scratch/source/scripts/release-publish.sh" --tap-only
[[ "$(cat "$RELEASE_TEST_LOG")" == published ]]
[[ "$(cat "$scratch/tap/Formula/ravel-lite.rb")" == 'separate command' ]]
[[ "$(cat "$scratch/tap/local.txt")" == 'local commit' ]]
# A Git-only tap on detached HEAD must be refused before GitHub publication.
"$real_git" clone -q --branch main "$scratch/remote.git" "$scratch/git-tap"
"$real_git" -C "$scratch/git-tap" checkout -q --detach
cp "$RELEASE_TEST_LOG" "$scratch/published-before"
if RAVEL_TAP_DIR="$scratch/git-tap" bash "$scratch/source/scripts/release-publish.sh" >"$scratch/output" 2>&1; then
  echo 'FAIL: publishing accepted a detached Git-only tap' >&2
  exit 1
fi
cmp "$scratch/published-before" "$RELEASE_TEST_LOG"
grep -Fq 'tap is on detached HEAD' "$scratch/output"

# Failed prerequisites stop the real Task pipeline before cutting a version.
mkdir -p "$scratch/task/scripts"
cp "$repo_root/Taskfile.yml" "$scratch/task/"
printf '## Unreleased\n\n- Fixture change.\n' >"$scratch/task/CHANGELOG.md"
cat >"$scratch/task/scripts/release-doctor.sh" <<'SH'
#!/usr/bin/env bash
exit "${RELEASE_TEST_DOCTOR_FAIL:-0}"
SH
chmod +x "$scratch/task/scripts/release-doctor.sh"
jj git init --colocate "$scratch/task" >/dev/null
jj -R "$scratch/task" describe -m fixture >/dev/null
"$real_git" init -q --bare "$scratch/task-remote.git"
jj -R "$scratch/task" git remote add origin "$scratch/task-remote.git"
jj -R "$scratch/task" git push --named main=@ >/dev/null
jj -R "$scratch/task" new main >/dev/null
before="$("$real_git" -C "$scratch/task" rev-parse HEAD)"

# Task's dry run checks preconditions but must never execute a release command.
# The installed smoke test runs over the built archives after the build and
# before each of the three steps that publish: the pushes of main and the tag,
# and the GitHub Release and tap.
dry_line() {
  { grep -Fxn -- "task: [release] $1" "$scratch/dry" || true; } | head -n 1 | cut -d: -f1
}
for level in patch minor major; do
  task --dir "$scratch/task" --dry "release:$level" >"$scratch/dry" 2>&1
  grep -Fq "cargo release $level --execute --no-confirm" "$scratch/dry"
  build="$(dry_line scripts/release-build.sh)"
  smoke="$(dry_line 'scripts/release-smoke.sh --archives target/dist')"
  if [[ -z "$build" || -z "$smoke" ]] || ((build > smoke)); then
    fail "release:$level does not smoke-test target/dist after scripts/release-build.sh"
  fi
  # shellcheck disable=SC2016 # the Taskfile's own text, unexpanded
  for publishing in 'jj git push -b main' 'git push origin "$(git describe --tags --exact-match HEAD)"' \
    scripts/release-publish.sh; do
    line="$(dry_line "$publishing")"
    if [[ -z "$line" ]] || ((line < smoke)); then
      fail "release:$level runs '$publishing' before the installed smoke test, or not at all"
    fi
  done
done
[[ "$(cat "$RELEASE_TEST_LOG")" == 'published' ]]

if RELEASE_TEST_DOCTOR_FAIL=73 task --dir "$scratch/task" release:patch >"$scratch/output" 2>&1; then
  echo 'FAIL: release continued after failed prerequisites' >&2
  exit 1
fi
grep -Fq 'exit status 73' "$scratch/output"
[[ "$("$real_git" -C "$scratch/task" rev-parse HEAD)" == "$before" ]]
[[ -z "$("$real_git" -C "$scratch/task" tag --list)" ]]
[[ ! -e "$scratch/task/.jj/release-lock" ]]

# A second invocation must not enter an in-progress release.
mkdir "$scratch/task/.jj/release-lock"
if task --dir "$scratch/task" release:patch >"$scratch/output" 2>&1; then
  echo 'FAIL: a second release acquired the active release lock' >&2
  exit 1
fi
grep -Fq 'Another release holds .jj/release-lock' "$scratch/output"
[[ -d "$scratch/task/.jj/release-lock" ]]

# Execute the actual task, replacing expensive build/service boundaries only.
# A failed smoke must stop before even the first publishing command.
rmdir "$scratch/task/.jj/release-lock"
RELEASE_TEST_CARGO="$(command -v cargo)"
export RELEASE_TEST_CARGO
RELEASE_TEST_JJ="$(command -v jj)"
export RELEASE_TEST_JJ
export RELEASE_TEST_ORDER="$scratch/order"
cat >"$scratch/bin/cargo" <<'SH'
#!/usr/bin/env bash
set -euo pipefail
if [[ "$1" == release ]]; then
  printf 'cut %s\n' "$*" >>"$RELEASE_TEST_ORDER"
  exit 0
fi
exec "$RELEASE_TEST_CARGO" "$@"
SH
cat >"$scratch/bin/jj" <<'SH'
#!/usr/bin/env bash
set -euo pipefail
if [[ "$1 $2" == 'git push' ]]; then
  echo publish-main >>"$RELEASE_TEST_ORDER"
fi
exec "$RELEASE_TEST_JJ" "$@"
SH
chmod +x "$scratch/bin/cargo" "$scratch/bin/jj"
for stage in prepare build; do
  # shellcheck disable=SC2016 # evaluated by the fixture script
  printf '#!/bin/sh\necho %s >>"$RELEASE_TEST_ORDER"\n' "$stage" >"$scratch/task/scripts/release-$stage.sh"
  chmod +x "$scratch/task/scripts/release-$stage.sh"
done
printf '#!/bin/sh\nexit 0\n' >"$scratch/task/scripts/check.sh"
cat >"$scratch/task/scripts/release-publish.sh" <<'SH'
#!/bin/sh
if [ "$1" = --check-tap ]; then echo tap-preflight >>"$RELEASE_TEST_ORDER"; else echo publish-services >>"$RELEASE_TEST_ORDER"; fi
SH
cat >"$scratch/task/scripts/release-smoke.sh" <<'SH'
#!/bin/sh
echo smoke >>"$RELEASE_TEST_ORDER"
exit 74
SH
chmod +x "$scratch/task/scripts/release-"*.sh
for level in patch minor major; do
  : >"$RELEASE_TEST_ORDER"
  if task --dir "$scratch/task" "release:$level" >"$scratch/output" 2>&1; then fail 'continued after failed smoke'; fi
  grep -Fq 'exit status 74' "$scratch/output"
  grep -Fxq smoke "$RELEASE_TEST_ORDER"
  if grep -q '^publish-' "$RELEASE_TEST_ORDER"; then fail 'publication preceded successful smoke'; fi
  [[ ! -e "$scratch/task/.jj/release-lock" ]]
  [[ -z "$("$real_git" --git-dir="$scratch/task-remote.git" tag --list)" ]]
done

echo 'release tests: all passed'
