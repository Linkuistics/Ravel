#!/usr/bin/env bash
# Release channel fixtures for docs/specs/release-channels.md: the channel
# rules, the two rendered classes, the Taskfile's required CHANNEL, the
# baseline against the target channel, publication and each resume, the
# installation through the versioned keg, and the release evidence. Local jj
# and Git remotes stand for GitHub and the tap; gh and brew are fixtures.
#
# With "mutants", it changes one rule at a time in a copy of the release
# scripts and the Taskfile and shows that a control then fails.
set -euo pipefail
IFS=$'\n\t'

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# The root whose scripts/ and Taskfile.yml are under test. Mutants set it.
subject="${RELEASE_CHANNEL_SUBJECT:-$repo_root}"
fixtures="$(mktemp -d)"
# shellcheck disable=SC2064 # remove precisely this private directory
trap "chmod -R u+w '$fixtures' 2>/dev/null; rm -rf '$fixtures'" EXIT

ok() { printf 'release-channel-test: ok: %s\n' "$*"; }
fail() {
  echo "release-channel-test: FAIL: $*" >&2
  exit 1
}

# with_env NAME=VALUE COMMAND...: COMMAND, which may be a function, with NAME set.
with_env() {
  (
    export "${1?}"
    shift
    "$@"
  )
}

# refuses NAME PATTERN COMMAND...: COMMAND fails and its output matches PATTERN.
refuses() {
  local name="$1" pattern="$2"
  shift 2
  if "$@" >"$fixtures/out" 2>&1; then fail "$name: accepted"; fi
  grep -Eq -- "$pattern" "$fixtures/out" || fail "$name: refused without /$pattern/: $(tail -n 3 "$fixtures/out" | tr '\n' ' ')"
}

controls() {
  export JJ_CONFIG="$fixtures/jj.toml"
  printf '[user]\nname = "Release Test"\nemail = "release@example.invalid"\n' >"$JJ_CONFIG"
  export GIT_AUTHOR_NAME='Release Test' GIT_COMMITTER_NAME='Release Test'
  export GIT_AUTHOR_EMAIL='release@example.invalid' GIT_COMMITTER_EMAIL='release@example.invalid'
  real_git="$(command -v git)"
  export RELEASE_TEST_GIT="$real_git"
  mkdir -p "$fixtures/bin"
  export PATH="$fixtures/bin:$PATH"
  section_library
  section_render
  section_taskfile
  section_prepare
  section_publish
  section_install
  section_evidence
  echo 'release channel tests: all passed'
}

section_library() {
  (
    # shellcheck source=scripts/release-channel.sh
    source "$subject/scripts/release-channel.sh"
    channel_select versioned-2
    [[ "$CHANNEL_FORMULA|$CHANNEL_FILE|$CHANNEL_CLASS|$CHANNEL_KEG_ONLY|$CHANNEL_MAJOR" == 'ravel@2|ravel@2.rb|RavelAT2|  keg_only :versioned_formula|2' ]] ||
      fail "versioned-2 maps to $CHANNEL_FORMULA|$CHANNEL_FILE|$CHANNEL_CLASS|$CHANNEL_KEG_ONLY|$CHANNEL_MAJOR"
    channel_select unversioned
    [[ "$CHANNEL_FORMULA|$CHANNEL_FILE|$CHANNEL_CLASS|$CHANNEL_KEG_ONLY|$CHANNEL_MAJOR" == 'ravel|ravel.rb|Ravel||' ]] ||
      fail "unversioned maps to $CHANNEL_FORMULA|$CHANNEL_FILE|$CHANNEL_CLASS|$CHANNEL_KEG_ONLY|$CHANNEL_MAJOR"
    local bad
    for bad in '' stable versioned versioned-3 'versioned-2 ' Unversioned; do
      if channel_select "$bad" >/dev/null; then fail "channel_select admitted '$bad'"; fi
    done
    local channel target unversioned expected actual
    while IFS=' ' read -r channel target unversioned expected; do
      [[ "$unversioned" == - ]] && unversioned=''
      actual=refused
      channel_admits "$channel" "$target" "$unversioned" >/dev/null && actual=admitted
      [[ "$actual" == "$expected" ]] || fail "channel_admits $channel $target over unversioned '${unversioned}' is $actual, not $expected"
    done <<'EOF'
versioned-2 2.0.0 1.1.0 admitted
versioned-2 2.0.1 1.1.0 admitted
versioned-2 2.3.0 - admitted
versioned-2 1.1.1 1.1.0 refused
versioned-2 3.0.0 2.4.0 refused
versioned-2 2.0.1 2.0.0 refused
versioned-2 2.0.1 3.0.0 refused
unversioned 2.0.0 1.1.0 refused
unversioned 2.0.0 - refused
unversioned 2.0.1 2.0.0 admitted
unversioned 1.1.1 1.1.0 admitted
unversioned 3.0.0 2.4.0 admitted
bogus 2.0.0 1.1.0 refused
EOF
    [[ "$(channel_baseline_file versioned-2 1.1.0)|$(channel_baseline_file versioned-2 2.0.0)|$(channel_baseline_file unversioned 2.0.0)|$(channel_baseline_file unversioned 1.1.0)" == 'ravel.rb|ravel@2.rb|ravel.rb|ravel.rb' ]] ||
      fail 'the baseline formula does not follow the channel that published the baseline'
  ) || exit 1
  ok 'channel mapping, admission and baseline rules'
}

# Fixture archives of VERSION, real layout and harmless contents, in DIR.
make_archives() {
  local version="$1" dir="$2" target top entry
  (
    # shellcheck source=scripts/release-common.sh
    source "$subject/scripts/release-common.sh"
    mkdir -p "$dir" "$fixtures/stage-$version"
    for target in "${TARGETS[@]}"; do
      top="ravel-v$version-$target"
      mkdir -p "$fixtures/stage-$version/$top"
      while IFS= read -r entry; do
        mkdir -p "$(dirname "$fixtures/stage-$version/$top/$entry")"
        printf 'fixture %s %s\n' "$version" "$entry" >"$fixtures/stage-$version/$top/$entry"
      done < <(archive_manifest)
      while IFS= read -r entry; do chmod +x "$fixtures/stage-$version/$top/$entry"; done < <(archive_executables)
      pack_archive "$fixtures/stage-$version" "$top" "$dir/$top.tar.xz"
    done
  )
}

# render VERSION CHANNEL OUTPUT: the production rendering from $fixtures/archives.
render() {
  (
    # shellcheck source=scripts/release-common.sh
    source "$subject/scripts/release-common.sh"
    render_archive_formula "$fixtures/archives" "$1" "$subject/scripts/templates/ravel.rb.tmpl" "$3" "$2"
  )
}

section_render() {
  make_archives 2.0.0 "$fixtures/archives"
  render 2.0.0 versioned-2 "$fixtures/ravel@2.rb"
  render 2.0.0 unversioned "$fixtures/ravel.rb"
  grep -Fxq 'class RavelAT2 < Formula' "$fixtures/ravel@2.rb" || fail 'ravel@2.rb lacks class RavelAT2'
  grep -Fxq 'class Ravel < Formula' "$fixtures/ravel.rb" || fail 'ravel.rb lacks class Ravel'
  grep -A1 '^  license ' "$fixtures/ravel@2.rb" | grep -Fxq '  keg_only :versioned_formula' ||
    fail 'ravel@2.rb is not keg-only after its licence'
  if grep -Eq '^[[:space:]]*keg_only' "$fixtures/ravel.rb"; then fail 'ravel.rb is keg-only'; fi
  if grep -q '@[A-Z_0-9]*@' "$fixtures/ravel.rb" "$fixtures/ravel@2.rb"; then fail 'a placeholder remains'; fi
  # One release: the same version, URLs and digests, only class and keg_only differ.
  sed -e 's/^class RavelAT2 < Formula$/class Ravel < Formula/' -e '/^  keg_only :versioned_formula$/d' "$fixtures/ravel@2.rb" >"$fixtures/normalised"
  cmp -s "$fixtures/normalised" "$fixtures/ravel.rb" || fail 'the channel formulae differ beyond class and keg_only'
  local bad
  for bad in '' stable; do
    rm -f "$fixtures/bad.rb"
    refuses "render '$bad'" 'release channel' render 2.0.0 "$bad" "$fixtures/bad.rb"
    [[ ! -e "$fixtures/bad.rb" ]] || fail "render '$bad' wrote a formula"
  done
  ok 'both classes render from one release and only class and keg_only differ'
}

section_taskfile() {
  local repo="$fixtures/task" level
  mkdir -p "$repo/scripts"
  cp "$subject/Taskfile.yml" "$repo/"
  # The doctor is the first command after the lock; a stub marks that it ran.
  # shellcheck disable=SC2016 # evaluated by the stub
  printf '#!/bin/sh\ntouch "$(dirname "$0")/../doctor-ran"\n' >"$repo/scripts/release-doctor.sh"
  chmod +x "$repo/scripts/release-doctor.sh"
  jj git init --colocate "$repo" >/dev/null
  for level in patch minor major; do
    refuses "release:$level without CHANNEL" 'missing required variables: CHANNEL' task --dir "$repo" "release:$level"
    refuses "release:$level CHANNEL=stable" "CHANNEL has an invalid value : 'stable'" task --dir "$repo" "release:$level" CHANNEL=stable
    refuses "release:$level CHANNEL=''" "CHANNEL has an invalid value : ''" task --dir "$repo" "release:$level" CHANNEL=
    [[ ! -e "$repo/doctor-ran" && ! -e "$repo/.jj/release-lock" ]] || fail "release:$level ran a command before refusing its channel"
    task --dir "$repo" --dry "release:$level" CHANNEL=versioned-2 >"$fixtures/dry" 2>&1
    local line
    for line in "bash scripts/release-prepare.sh --channel versioned-2 --level $level" \
      'scripts/release-build.sh --channel versioned-2' 'scripts/release-publish.sh --channel versioned-2' \
      'scripts/release-install.sh --channel versioned-2' 'scripts/release-evidence.sh --channel versioned-2'; do
      grep -Fxq "task: [release] $line" "$fixtures/dry" || fail "release:$level does not run '$line'"
    done
    if grep -E 'brew (install|upgrade|test)|--channel unversioned' "$fixtures/dry"; then fail "release:$level bypasses the channel"; fi
  done
  ok 'the release tasks require CHANNEL and carry it to every step'
}

# A Cargo workspace at VERSION in REPO, committed by the caller.
set_version() {
  local repo="$1" version="$2"
  sed -i.bak "s/^version = .*/version = \"$version\"/" "$repo/Cargo.toml"
  rm "$repo/Cargo.toml.bak"
  cargo generate-lockfile --manifest-path "$repo/Cargo.toml" --offline 2>/dev/null
}

# tap_commit TAP FILE TEXT MESSAGE: write Formula/FILE (or remove it for an
# empty TEXT), then move main and push it, as a published tap change does.
tap_commit() {
  local tap="$1" file="$2" text="$3" message="$4"
  if [[ -n "$text" ]]; then printf '%s\n' "$text" >"$tap/Formula/$file"; else rm -f "$tap/Formula/$file"; fi
  jj -R "$tap" describe -m "$message" >/dev/null 2>&1
  jj -R "$tap" bookmark set main -r @ >/dev/null 2>&1
  jj -R "$tap" new main >/dev/null 2>&1
  jj -R "$tap" git push -b main >/dev/null 2>&1
}

# A gh fixture: complete releases for every tag, downloads from the dist.
write_gh() {
  cat >"$fixtures/bin/gh" <<'SH'
#!/usr/bin/env bash
set -euo pipefail
case "$1 ${2:-}" in
  'auth status') exit 0 ;;
  'release view')
    v="${3#v}"
    printf '{"isDraft":false,"assets":[{"name":"ravel-v%s-aarch64-apple-darwin.tar.xz","size":1},{"name":"ravel-v%s-aarch64-unknown-linux-gnu.tar.xz","size":1},{"name":"ravel-v%s-x86_64-unknown-linux-gnu.tar.xz","size":1}]}\n' "$v" "$v" "$v"
    ;;
  'release download')
    while (($#)); do
      if [[ "$1" == --dir ]]; then
        shift
        cp "$RELEASE_TEST_DIST/"*.tar.xz "$1/"
        if [[ "${RELEASE_TEST_DOWNLOAD_CORRUPT:-0}" == 1 ]]; then printf 'corrupt\n' >>"$1/ravel-v2.0.0-aarch64-apple-darwin.tar.xz"; fi
      fi
      shift
    done
    ;;
  'release create') printf 'published\n' >>"$RELEASE_TEST_LOG" ;;
  *) echo "gh fixture: unexpected $*" >&2; exit 97 ;;
esac
SH
  chmod +x "$fixtures/bin/gh"
}

section_prepare() {
  local repo="$fixtures/prepare" tap="$fixtures/prepare-tap" saved
  write_gh
  git init -q --bare "$fixtures/prepare-remote.git"
  git init -q --bare "$fixtures/prepare-tap-remote.git"
  jj git init --colocate "$repo" >/dev/null
  mkdir -p "$repo/scripts" "$repo/crates/ravel/src"
  cp "$subject/scripts/release-prepare.sh" "$subject/scripts/release-notes.sh" "$subject/scripts/release-common.sh" \
    "$subject/scripts/release-channel.sh" "$repo/scripts/"
  printf '/target/\n' >"$repo/.gitignore"
  printf '[workspace]\nmembers = ["crates/ravel"]\nresolver = "2"\n[workspace.package]\nversion = "0.0.0"\nedition = "2021"\n' >"$repo/Cargo.toml"
  printf '[package]\nname="ravel"\nversion.workspace=true\nedition.workspace=true\n' >"$repo/crates/ravel/Cargo.toml"
  printf 'fn main() {}\n' >"$repo/crates/ravel/src/main.rs"
  printf '# Changelog\n\n## Unreleased\n\n- Pending change.\n' >"$repo/CHANGELOG.md"
  set_version "$repo" 1.1.0
  jj -R "$repo" describe -m 'chore: release v1.1.0' >/dev/null
  jj -R "$repo" git remote add origin "$fixtures/prepare-remote.git"
  jj -R "$repo" git push --named main=@ >/dev/null 2>&1
  jj -R "$repo" new main >/dev/null
  git -C "$repo" tag -a v1.1.0 -m v1.1.0 main
  git -C "$repo" push -q origin v1.1.0
  printf 'fn main() { println!("next"); }\n' >"$repo/crates/ravel/src/main.rs"
  jj -R "$repo" describe -m 'Next change' >/dev/null
  jj -R "$repo" bookmark set main -r @ >/dev/null
  jj -R "$repo" new main >/dev/null
  jj -R "$repo" git push -b main >/dev/null 2>&1

  jj git init --colocate "$tap" >/dev/null
  mkdir -p "$tap/Formula"
  printf 'separate command\n' >"$tap/Formula/ravel-lite.rb"
  printf '  version "1.1.0"\n' >"$tap/Formula/ravel.rb"
  jj -R "$tap" describe -m 'ravel v1.1.0' >/dev/null
  jj -R "$tap" git remote add origin "$fixtures/prepare-tap-remote.git"
  jj -R "$tap" git push --named main=@ >/dev/null 2>&1
  jj -R "$tap" new main >/dev/null 2>&1
  export RAVEL_TAP_DIR="$tap"
  prepare() { (cd "$repo" && bash scripts/release-prepare.sh "$@"); }

  # The first 2.0.0 cut: baseline 1.1.0 in unversioned ravel, target ravel@2.
  prepare --channel versioned-2 --level major >"$fixtures/out" 2>&1 || fail "the first versioned cut refused: $(tail -n 3 "$fixtures/out")"
  grep -Fq 'baseline v1.1.0 in Formula/ravel.rb; planned v2.0.0 publishes Formula/ravel@2.rb (CHANNEL=versioned-2)' "$fixtures/out" ||
    fail 'the first versioned cut did not check its baseline in ravel.rb'
  refuses 'unversioned major before promotion' 'until release:promote' prepare --channel unversioned --level major
  refuses 'versioned-2 for a 1.x cut' 'publishes only major 2' prepare --channel versioned-2 --level minor
  # Refusals of the value come before any fetch or working-copy change.
  saved="$(jj -R "$repo" log --no-graph -r @ -T change_id)"
  refuses 'prepare without a channel' 'release channel is required' prepare --level major
  refuses 'prepare with an unknown channel' 'unknown release channel' prepare --channel stable --level major
  refuses 'prepare without a level' 'usage' prepare --channel versioned-2
  [[ "$(jj -R "$repo" log --no-graph -r @ -T change_id)" == "$saved" ]] || fail 'a refused channel changed the working copy'
  # The incomplete-baseline refusal stays.
  tap_commit "$tap" ravel.rb '  version "1.0.0"' 'ravel v1.0.0'
  refuses 'an unfinished unversioned baseline' 'Formula/ravel.rb is not at v1.1.0' prepare --channel versioned-2 --level major
  tap_commit "$tap" ravel.rb '  version "1.1.0"' 'ravel v1.1.0'
  ok 'the first 2.0.0 cut checks 1.1.0 in ravel.rb and targets ravel@2'

  # A later 2.x cut before promotion: baseline 2.0.0 in ravel@2.
  set_version "$repo" 2.0.0
  jj -R "$repo" describe -m 'chore: release v2.0.0' >/dev/null
  jj -R "$repo" bookmark set main -r @ >/dev/null
  jj -R "$repo" new main >/dev/null
  git -C "$repo" tag -a v2.0.0 -m v2.0.0 main
  git -C "$repo" push -q origin v2.0.0
  printf 'fn main() { println!("later"); }\n' >"$repo/crates/ravel/src/main.rs"
  jj -R "$repo" describe -m 'Later change' >/dev/null
  jj -R "$repo" bookmark set main -r @ >/dev/null
  jj -R "$repo" new main >/dev/null
  jj -R "$repo" git push -b main >/dev/null 2>&1
  refuses 'a missing versioned baseline' 'Formula/ravel@2.rb missing' prepare --channel versioned-2 --level patch
  tap_commit "$tap" ravel@2.rb '  version "2.0.0"' 'ravel@2 v2.0.0'
  prepare --channel versioned-2 --level patch >"$fixtures/out" 2>&1 || fail "the later versioned cut refused: $(tail -n 3 "$fixtures/out")"
  grep -Fq 'baseline v2.0.0 in Formula/ravel@2.rb; planned v2.0.1 publishes Formula/ravel@2.rb (CHANNEL=versioned-2)' "$fixtures/out" ||
    fail 'the later versioned cut did not check its baseline in ravel@2.rb'
  refuses 'an unversioned cut before promotion' 'Formula/ravel.rb is not at v2.0.0' prepare --channel unversioned --level patch
  ok 'a later 2.x cut before promotion checks 2.0.0 in ravel@2'

  # After promotion: unversioned ravel has major 2.
  tap_commit "$tap" ravel.rb '  version "2.0.0"' 'ravel v2.0.0'
  refuses 'versioned-2 after promotion' 'promotion is complete' prepare --channel versioned-2 --level patch
  prepare --channel unversioned --level patch >"$fixtures/out" 2>&1 || fail "the unversioned cut after promotion refused: $(tail -n 3 "$fixtures/out")"
  grep -Fq 'baseline v2.0.0 in Formula/ravel.rb; planned v2.0.1 publishes Formula/ravel.rb (CHANNEL=unversioned)' "$fixtures/out" ||
    fail 'the unversioned cut after promotion did not check its baseline in ravel.rb'
  ok 'after promotion versioned-2 refuses and unversioned checks ravel.rb'
  unset RAVEL_TAP_DIR
}

section_publish() {
  local source="$fixtures/source" tap="$fixtures/tap" before_unversioned
  export RELEASE_TEST_LOG="$fixtures/published" RELEASE_TEST_DIST="$source/target/dist"
  export RELEASE_TEST_BOOK_CALLS="$fixtures/book-calls" RELEASE_TEST_REMOTE="$fixtures/tap-remote.git"
  write_gh
  cat >"$fixtures/bin/git" <<'SH'
#!/usr/bin/env bash
set -euo pipefail
# Reject raw Git mutations in the jj tap, as the repository contract does.
if [[ "${1:-}" == '-C' && -d "$2/.jj" ]]; then
  case "${3:-}" in add|commit|push) exit 88 ;; esac
fi
exec "$RELEASE_TEST_GIT" "$@"
SH
  chmod +x "$fixtures/bin/git"
  mkdir -p "$source/scripts/templates" "$source/target/dist"
  cp "$subject/scripts/release-publish.sh" "$subject/scripts/release-common.sh" "$subject/scripts/release-channel.sh" \
    "$subject/scripts/release-install.sh" "$subject/scripts/release-evidence.sh" "$subject/scripts/installed-record.sh" "$source/scripts/"
  cp "$subject/scripts/templates/ravel.rb.tmpl" "$source/scripts/templates/"
  # The book step has its own suite. The stub records each call and the tap's
  # remote ravel@2.rb at that moment.
  cat >"$source/scripts/release-book.sh" <<'SH'
#!/usr/bin/env bash
set -euo pipefail
versioned="$("$RELEASE_TEST_GIT" --git-dir="$RELEASE_TEST_REMOTE" show main:Formula/ravel@2.rb 2>/dev/null | head -n 1 || echo absent)"
printf '%s ravel@2=%s\n' "$*" "${versioned:-absent}" >>"$RELEASE_TEST_BOOK_CALLS"
[[ "${RELEASE_TEST_BOOK_FAIL:-0}" != 1 ]]
SH
  chmod +x "$source/scripts/release-book.sh"
  printf '## Unreleased\n\n## v2.0.0\n\n- Major release notes.\n' >"$source/CHANGELOG.md"
  printf '/target/\n' >"$source/.gitignore"
  "$real_git" init -q "$source"
  "$real_git" -C "$source" add .
  "$real_git" -C "$source" commit -qm fixture
  "$real_git" -C "$source" tag -a v2.0.0 -m v2.0.0
  cp "$fixtures/archives/"*.tar.xz "$source/target/dist/"
  cp "$fixtures/ravel@2.rb" "$source/target/dist/ravel@2.rb"

  "$real_git" init -q --bare "$fixtures/tap-remote.git"
  jj git init --colocate "$tap" >/dev/null
  mkdir -p "$tap/Formula"
  printf 'class Ravel < Formula\n  version "1.1.0"\nend\n' >"$tap/Formula/ravel.rb"
  printf 'separate command\n' >"$tap/Formula/ravel-lite.rb"
  jj -R "$tap" describe -m 'ravel v1.1.0' >/dev/null
  jj -R "$tap" git remote add origin "$fixtures/tap-remote.git"
  jj -R "$tap" git push --named main=@ >/dev/null 2>&1
  jj -R "$tap" new main >/dev/null 2>&1
  export RAVEL_TAP_DIR="$tap"
  before_unversioned="$("$real_git" --git-dir="$fixtures/tap-remote.git" show main:Formula/ravel.rb)"
  local tip
  tip="$("$real_git" --git-dir="$fixtures/tap-remote.git" rev-parse main)"
  publish() { bash "$source/scripts/release-publish.sh" "$@"; }
  remote_unchanged() {
    [[ "$("$real_git" --git-dir="$fixtures/tap-remote.git" rev-parse main)" == "$tip" ]] || fail "$1 changed the tap"
  }

  # No publication mode has a default channel.
  local mode
  for mode in '' --tap-only --book-only; do
    refuses "publish $mode without a channel" 'release channel is required' publish ${mode:+"$mode"}
    refuses "publish $mode with an unknown channel" 'unknown release channel' publish --channel stable ${mode:+"$mode"}
  done
  refuses 'publish --channel without a value' 'usage' publish --channel
  refuses 'publish --check-tap with a channel' 'usage' publish --check-tap --channel versioned-2
  [[ ! -e "$RELEASE_TEST_LOG" && ! -e "$RELEASE_TEST_BOOK_CALLS" ]] || fail 'a refused channel reached GitHub or the book'
  remote_unchanged 'a refused channel'
  # Unversioned 2.0.0 before promotion refuses before the GitHub Release.
  cp "$fixtures/ravel.rb" "$source/target/dist/ravel.rb"
  refuses 'unversioned 2.0.0 before promotion' 'until release:promote' publish --channel unversioned
  rm "$source/target/dist/ravel.rb"
  [[ ! -e "$RELEASE_TEST_LOG" ]] || fail 'the unversioned refusal came after the GitHub Release'
  remote_unchanged 'the unversioned refusal'
  ok 'publication requires a known, admitted channel before any write'

  # A failed book stops before the tap; each resume keeps the channel.
  if RELEASE_TEST_BOOK_FAIL=1 publish --channel versioned-2 >"$fixtures/out" 2>&1; then fail 'continued after a failed book'; fi
  [[ "$(cat "$RELEASE_TEST_LOG")" == published ]] || fail 'the GitHub Release did not precede the book'
  remote_unchanged 'a failed book'
  : >"$RELEASE_TEST_BOOK_CALLS"
  publish --channel versioned-2 --book-only >"$fixtures/out" 2>&1 || fail "--book-only refused: $(tail -n 3 "$fixtures/out")"
  [[ "$(cat "$RELEASE_TEST_BOOK_CALLS")" == 'publish v2.0.0 ravel@2=absent' ]] || fail '--book-only did not publish the book alone'
  remote_unchanged '--book-only'
  if RELEASE_TEST_DOWNLOAD_CORRUPT=1 publish --channel versioned-2 --tap-only >"$fixtures/out" 2>&1; then fail '--tap-only accepted a corrupt asset'; fi
  remote_unchanged 'a corrupt asset'
  : >"$RELEASE_TEST_BOOK_CALLS"
  publish --channel versioned-2 --tap-only >"$fixtures/out" 2>&1 || fail "--tap-only refused: $(tail -n 3 "$fixtures/out")"
  [[ "$(cat "$RELEASE_TEST_BOOK_CALLS")" == 'verify v2.0.0 ravel@2=absent' ]] || fail '--tap-only did not verify the book before the tap'
  [[ "$(cat "$RELEASE_TEST_LOG")" == published ]] || fail '--tap-only created a second GitHub Release'
  "$real_git" --git-dir="$fixtures/tap-remote.git" show main:Formula/ravel@2.rb | cmp -s - "$fixtures/ravel@2.rb" ||
    fail '--tap-only did not publish ravel@2.rb'
  [[ "$("$real_git" --git-dir="$fixtures/tap-remote.git" show main:Formula/ravel.rb)" == "$before_unversioned" ]] ||
    fail 'the versioned release changed ravel.rb'
  [[ "$("$real_git" --git-dir="$fixtures/tap-remote.git" show main:Formula/ravel-lite.rb)" == 'separate command' ]] ||
    fail 'the versioned release changed another formula'
  [[ "$("$real_git" --git-dir="$fixtures/tap-remote.git" diff --name-only main~1 main)" == 'Formula/ravel@2.rb' ]] ||
    fail 'the versioned tap commit changed more than ravel@2.rb'
  [[ "$("$real_git" --git-dir="$fixtures/tap-remote.git" log -1 --format=%s main)" == 'ravel@2 v2.0.0' ]] ||
    fail 'the tap commit does not name ravel@2'
  [[ -z "$("$real_git" -C "$tap" status --porcelain)" ]] || fail 'the tap is not clean after publication'
  ok 'a versioned release writes only ravel@2.rb, and each resume keeps the channel'

  # After promotion, versioned-2 refuses before the tap.
  tap_commit "$tap" ravel.rb 'class Ravel < Formula
  version "2.0.0"
end' 'ravel v2.0.0'
  tip="$("$real_git" --git-dir="$fixtures/tap-remote.git" rev-parse main)"
  refuses 'versioned-2 --tap-only after promotion' 'promotion is complete' publish --channel versioned-2 --tap-only
  remote_unchanged 'versioned-2 after promotion'
  ok 'versioned-2 refuses after promotion'
}

# A Homebrew prefix in DIR with unversioned ravel 1.1.0 installed and linked.
make_brew_prefix() {
  local p="$1"
  mkdir -p "$p/Cellar/ravel/1.1.0/bin" "$p/Cellar/ravel/1.1.0/share/ravel" "$p/opt" "$p/bin" "$p/share/ravel/skills"
  printf '#!/bin/sh\necho "ravel 1.1.0"\n' >"$p/Cellar/ravel/1.1.0/bin/ravel"
  chmod +x "$p/Cellar/ravel/1.1.0/bin/ravel"
  printf '{"schemaVersion":1,"skillDirectory":"%s"}\n' "$p/share/ravel/skills" >"$p/Cellar/ravel/1.1.0/share/ravel/installation.json"
  ln -s ../Cellar/ravel/1.1.0 "$p/opt/ravel"
  ln -s ../Cellar/ravel/1.1.0/bin/ravel "$p/bin/ravel"
  : >"$p/.log"
}

write_brew() {
  cat >"$fixtures/bin/brew" <<'SH'
#!/usr/bin/env bash
# A Homebrew fixture over FAKE_BREW_PREFIX. Faults: FAKE_BREW_LINK=1 links a
# versioned keg into the shared bin, FAKE_BREW_KEG_ONLY=false reports it as
# not keg-only, FAKE_BREW_RECORD=keg names the keg in its record,
# FAKE_BREW_BIN_VERSION makes its command report another version and
# FAKE_BREW_UPGRADE_UNVERSIONED=1 also upgrades and relinks unversioned ravel.
set -euo pipefail
p="$FAKE_BREW_PREFIX"
log() { printf '%s\n' "$*" >>"$p/.log"; }
install_keg() {
  local name="$1" v="$FAKE_BREW_VERSION" keg dir
  keg="$p/Cellar/$name/$v"
  mkdir -p "$keg/bin" "$keg/share/ravel"
  printf '#!/bin/sh\necho "ravel %s"\n' "${FAKE_BREW_BIN_VERSION:-$v}" >"$keg/bin/ravel"
  chmod +x "$keg/bin/ravel"
  dir="$p/share/ravel/skills"
  [[ "${FAKE_BREW_RECORD:-}" == keg ]] && dir="$keg/share/ravel/skills"
  printf '{"schemaVersion":1,"skillDirectory":"%s"}\n' "$dir" >"$keg/share/ravel/installation.json"
  ln -sfn "../Cellar/$name/$v" "$p/opt/$name"
  if [[ "$name" == ravel || "${FAKE_BREW_LINK:-0}" == 1 ]]; then ln -sfn "../Cellar/$name/$v/bin/ravel" "$p/bin/ravel"; fi
  if [[ "${FAKE_BREW_UPGRADE_UNVERSIONED:-0}" == 1 ]]; then
    cp -R "$p/Cellar/ravel/1.1.0" "$p/Cellar/ravel/1.1.1"
    ln -sfn ../Cellar/ravel/1.1.1 "$p/opt/ravel"
    ln -sfn ../Cellar/ravel/1.1.1/bin/ravel "$p/bin/ravel"
  fi
}
case "$1" in
  --prefix) if (($# == 1)); then echo "$p"; else echo "$p/opt/$2"; fi ;;
  --cellar) echo "$p/Cellar/$2" ;;
  update) log "update cleanup=${HOMEBREW_NO_INSTALL_CLEANUP:-unset}" ;;
  list)
    [[ -d "$p/Cellar/$3" ]] || exit 1
    echo "$3 $(ls "$p/Cellar/$3" | tr '\n' ' ')"
    ;;
  install) log "install $2 cleanup=${HOMEBREW_NO_INSTALL_CLEANUP:-unset}"; install_keg "${2##*/}" ;;
  upgrade) log "upgrade $2 cleanup=${HOMEBREW_NO_INSTALL_CLEANUP:-unset}"; install_keg "$2" ;;
  test) log "test $2" ;;
  info)
    name="$3" keg_only=false reason=null linked=null
    if [[ "$name" == ravel@2 ]]; then keg_only="${FAKE_BREW_KEG_ONLY:-true}" reason='{"reason":":versioned_formula","explanation":""}'; fi
    if [[ "$name" == ravel || "${FAKE_BREW_LINK:-0}" == 1 ]]; then linked="\"$(basename "$(readlink "$p/opt/$name")")\""; fi
    installed="$(ls "$p/Cellar/$name" 2>/dev/null | jq -Rsc 'split("\n")[:-1] | map({version: .})')"
    printf '{"formulae":[{"name":"%s","keg_only":%s,"keg_only_reason":%s,"linked_keg":%s,"installed":%s}]}\n' \
      "$name" "$keg_only" "$reason" "$linked" "$installed"
    ;;
  *) log "unexpected $*"; exit 97 ;;
esac
SH
  chmod +x "$fixtures/bin/brew"
}

section_install() {
  local source="$fixtures/source" p before
  write_brew
  # A decoy on PATH: the verification must never run the command PATH finds.
  # shellcheck disable=SC2016 # evaluated by the decoy
  printf '#!/bin/sh\ntouch "%s/decoy-ran"\necho "ravel 2.0.0"\n' "$fixtures" >"$fixtures/bin/ravel"
  chmod +x "$fixtures/bin/ravel"
  export FAKE_BREW_VERSION=2.0.0
  install() { (cd "$source" && bash scripts/release-install.sh "$@"); }
  unversioned_links() { printf '%s %s %s\n' "$(ls "$1/Cellar/ravel")" "$(readlink "$1/opt/ravel")" "$(readlink "$1/bin/ravel")"; }

  p="$fixtures/brew-missing" && make_brew_prefix "$p" && export FAKE_BREW_PREFIX="$p"
  refuses 'install without a channel' 'usage' install
  refuses 'install with an unknown channel' 'unknown release channel' install --channel stable
  [[ ! -s "$p/.log" ]] || fail 'a refused channel ran Homebrew'

  p="$fixtures/brew-versioned" && make_brew_prefix "$p" && export FAKE_BREW_PREFIX="$p"
  before="$(unversioned_links "$p")"
  install --channel versioned-2 >"$fixtures/out" 2>&1 || fail "the versioned install refused: $(tail -n 3 "$fixtures/out")"
  [[ "$(cat "$p/.log")" == $'update cleanup=1\ninstall linkuistics/taps/ravel@2 cleanup=1\ntest ravel@2' ]] ||
    fail "the versioned install ran $(tr '\n' ';' <"$p/.log")"
  [[ "$(unversioned_links "$p")" == "$before" ]] || fail 'the versioned install changed unversioned ravel'
  [[ ! -e "$fixtures/decoy-ran" ]] || fail 'the verification ran the command on PATH'
  grep -Fq "ravel@2 2.0.0 installed and verified at $p/Cellar/ravel@2/2.0.0" "$fixtures/out" || fail 'the install did not report its keg'
  : >"$p/.log"
  install --channel versioned-2 >"$fixtures/out" 2>&1 || fail "the repeated versioned install refused: $(tail -n 3 "$fixtures/out")"
  grep -Fxq 'upgrade ravel@2 cleanup=1' "$p/.log" || fail 'a repeated install did not upgrade ravel@2'
  ok 'ravel@2 installs with cleanup suppressed and verifies through its own keg'

  local fault pattern
  while IFS=$'\t' read -r fault pattern; do
    p="$fixtures/brew-$fault" && make_brew_prefix "$p" && export FAKE_BREW_PREFIX="$p"
    refuses "install with fault $fault" "$pattern" with_env "$fault" install --channel versioned-2
  done <<'EOF'
FAKE_BREW_LINK=1	not an unlinked versioned keg-only formula|changed unversioned ravel
FAKE_BREW_KEG_ONLY=false	not an unlinked versioned keg-only formula
FAKE_BREW_RECORD=keg	installation.json holds
FAKE_BREW_BIN_VERSION=2.0.9	does not report ravel 2.0.0
FAKE_BREW_UPGRADE_UNVERSIONED=1	changed unversioned ravel
EOF
  [[ ! -e "$fixtures/decoy-ran" ]] || fail 'a refused verification ran the command on PATH'
  ok 'a linked, non-keg-only, misrecorded or misversioned keg, or a changed unversioned ravel, refuses'
}

section_evidence() {
  local source="$fixtures/source" site
  # The public source: the tag, and a Pages branch whose later commit leaves
  # the release directory unchanged.
  "$real_git" init -q --bare "$fixtures/source-remote.git"
  "$real_git" -C "$source" remote add origin "$fixtures/source-remote.git"
  "$real_git" -C "$source" push -q origin v2.0.0
  site="$fixtures/site"
  "$real_git" init -q "$site"
  "$real_git" -C "$site" checkout -q -b gh-pages
  mkdir "$site/v2.0.0"
  printf 'book\n' >"$site/v2.0.0/index.html"
  printf 'v2.0.0\n' >"$site/index.html"
  "$real_git" -C "$site" add -A
  "$real_git" -C "$site" commit -qm 'Book for v2.0.0'
  local book_commit
  book_commit="$("$real_git" -C "$site" rev-parse HEAD)"
  printf 'v2.0.0 later\n' >"$site/index.html"
  "$real_git" -C "$site" commit -qam 'Later index'
  "$real_git" -C "$site" push -q "$fixtures/source-remote.git" gh-pages
  local tap_commit
  tap_commit="$("$real_git" --git-dir="$fixtures/tap-remote.git" log -1 --format=%H main -- Formula/ravel@2.rb)"
  [[ "$tap_commit" != "$("$real_git" --git-dir="$fixtures/tap-remote.git" rev-parse main)" ]] ||
    fail 'the evidence fixture needs a later tap commit'
  export FAKE_BREW_PREFIX="$fixtures/brew-versioned"
  evidence() { (cd "$source" && bash scripts/release-evidence.sh "$@"); }

  refuses 'evidence without a channel' 'usage' evidence
  refuses 'evidence with an unknown channel' 'unknown release channel' evidence --channel stable
  refuses 'evidence over a corrupt asset' 'differs from' with_env RELEASE_TEST_DOWNLOAD_CORRUPT=1 evidence --channel versioned-2
  evidence --channel versioned-2 >"$fixtures/evidence.json" 2>"$fixtures/out" || fail "the evidence refused: $(tail -n 3 "$fixtures/out")"
  cmp -s "$fixtures/evidence.json" "$source/target/release-evidence/v2.0.0.json" || fail 'the printed evidence differs from the written file'
  local expected
  expected="$(jq -n --arg tagCommit "$("$real_git" -C "$source" rev-parse 'v2.0.0^{commit}')" \
    --arg book "$book_commit" --arg tap "$tap_commit" --arg keg "$fixtures/brew-versioned/Cellar/ravel@2/2.0.0" \
    --arg prefix "$fixtures/brew-versioned/opt/ravel@2" --arg skills "$fixtures/brew-versioned/share/ravel/skills" \
    --argjson assets "$(for f in "$source/target/dist/"*.tar.xz; do jq -n --arg name "${f##*/}" --arg sha256 "$(shasum -a 256 "$f" | cut -d' ' -f1)" '{name: $name, sha256: $sha256}'; done | jq -sc 'sort_by(.name)')" \
    '{schemaVersion: 1, channel: "versioned-2",
      formula: {name: "ravel@2", file: "Formula/ravel@2.rb", class: "RavelAT2", kegOnly: true},
      tag: "v2.0.0", tagCommit: $tagCommit, assets: $assets,
      book: {branch: "gh-pages", commit: $book}, tap: {commit: $tap},
      installed: {keg: $keg, prefix: $prefix, version: "ravel 2.0.0", record: {schemaVersion: 1, skillDirectory: $skills}}}')"
  [[ "$(jq -S '.assets |= sort_by(.name)' "$fixtures/evidence.json")" == "$(jq -S . <<<"$expected")" ]] ||
    fail "the evidence differs: $(diff <(jq -S '.assets |= sort_by(.name)' "$fixtures/evidence.json") <(jq -S . <<<"$expected") | head -n 12 | tr '\n' ' ')"
  ok 'the evidence names the channel, tag commit, asset digests, book commit, tap commit and installed keg'
}

# Each mutant: FILE, then the exact text it replaces, then its replacement,
# separated by tabs. Each must change its file and make a control fail.
mutants() {
  local work name file old new copy
  work="$(mktemp -d)"
  # shellcheck disable=SC2064 # remove precisely this private directory
  trap "chmod -R u+w '$fixtures' '$work' 2>/dev/null; rm -rf '$fixtures' '$work'" EXIT
  while IFS=$'\t' read -r name file old new; do
    copy="$work/$name"
    mkdir -p "$copy"
    cp -R "$repo_root/scripts" "$copy/scripts"
    cp "$repo_root/Taskfile.yml" "$copy/Taskfile.yml"
    python3 -I -c '
import sys
path, old, new = sys.argv[1:]
text = open(path).read()
if old not in text:
    sys.exit("mutant pattern not found: " + old)
open(path, "w").write(text.replace(old, new))
' "$copy/$file" "$old" "$new" || fail "mutant $name: the pattern changed nothing"
    if RELEASE_CHANNEL_SUBJECT="$copy" bash "$0" >"$work/$name.out" 2>&1; then fail "mutant $name: every control passed"; fi
    ok "mutant $name: a control fails ($(grep -m1 'FAIL' "$work/$name.out" | cut -c1-110))"
    rm -rf "$copy"
  done <<'EOF'
versioned file is ravel.rb	scripts/release-channel.sh	CHANNEL_FILE=ravel@2.rb	CHANNEL_FILE=ravel.rb
versioned class is Ravel	scripts/release-channel.sh	CHANNEL_CLASS=RavelAT2	CHANNEL_CLASS=Ravel
versioned not keg-only	scripts/release-channel.sh	CHANNEL_KEG_ONLY='  keg_only :versioned_formula'	CHANNEL_KEG_ONLY=''
omission is unversioned	scripts/release-channel.sh	    unversioned)	    unversioned | '')
versioned any major	scripts/release-channel.sh	      if [[ "$target_major" != 2 ]]; then	      if false; then
no promotion refusal	scripts/release-channel.sh	((unversioned_major >= 2))	((unversioned_major >= 99))
unversioned major 2 early	scripts/release-channel.sh	      if [[ "$target_major" == 2 ]] && {	      if false && {
baseline always ravel.rb	scripts/release-channel.sh	    echo ravel@2.rb	    echo ravel.rb
render ignores keg_only	scripts/release-common.sh	  if [[ -n "$CHANNEL_KEG_ONLY" ]]; then	  if false; then
render keeps placeholder	scripts/release-common.sh	    sed -i.bak '/^@KEG_ONLY@$/d' "$output" || return 1	    sed -i.bak 's/^@KEG_ONLY@$/  keg_only :versioned_formula/' "$output" || return 1
task channel unchecked	Taskfile.yml	          enum: [versioned-2, unversioned]	          enum: [versioned-2, unversioned, stable]
task prepare unchannelled	Taskfile.yml	bash scripts/release-prepare.sh --channel {{.CHANNEL}} --level {{.LEVEL}}	bash scripts/release-prepare.sh --channel unversioned --level {{.LEVEL}}
task install unchannelled	Taskfile.yml	scripts/release-install.sh --channel {{.CHANNEL}}	scripts/release-install.sh --channel unversioned
prepare skips admission	scripts/release-prepare.sh	    reason="$(channel_admits "$channel" "$target" "$unversioned")" || die "$reason"	    :
prepare fixed baseline	scripts/release-prepare.sh	    baseline_file="$(channel_baseline_file "$channel" "$version")"	    baseline_file=ravel.rb
publish default channel	scripts/release-publish.sh	  reason="$(channel_select "$channel")" || die "$reason"	  channel="${channel:-unversioned}"
publish skips admission	scripts/release-publish.sh	  check_channel_admitted "$version"	  :
publish fixed file	scripts/release-publish.sh	  cp "$DIST_DIR/$CHANNEL_FILE" "$TAP_DIR/Formula/$CHANNEL_FILE"	  cp "$DIST_DIR/$CHANNEL_FILE" "$TAP_DIR/Formula/ravel.rb"
publish skips the book	scripts/release-publish.sh	    "$REPO_ROOT/scripts/release-book.sh" verify "v$version"	    :
install keeps cleanup	scripts/release-install.sh	  export HOMEBREW_NO_INSTALL_CLEANUP=1	  :
install reads PATH	scripts/release-install.sh	  [[ "$("$keg/bin/ravel" --version)" == "ravel $version" ]] ||	  [[ "$(ravel --version)" == "ravel $version" ]] ||
install ignores keg-only	scripts/release-install.sh	.keg_only == true and .keg_only_reason == ":versioned_formula" and
install ignores unversioned	scripts/release-install.sh	    [[ "$(unversioned_state "$brew_prefix")" == "$before" ]] ||	    true ||
install skips record	scripts/release-install.sh	  bash "$REPO_ROOT/scripts/installed-record.sh" "$keg" "$brew_prefix/share/ravel/skills"	  :
evidence any formula	scripts/release-evidence.sh	-- "Formula/$CHANNEL_FILE")"	-- "Formula/ravel.rb")"
evidence trusts assets	scripts/release-evidence.sh	    cmp -s "$scratch/assets/$name" "$DIST_DIR/$name" ||	    true ||
evidence any book commit	scripts/release-evidence.sh	FETCH_HEAD -- "$tag/")"	FETCH_HEAD)"
EOF
}

case "${1:-}" in
  mutants) mutants ;;
  '') controls ;;
  *) fail 'usage: release-channel.test.sh [mutants]' ;;
esac
