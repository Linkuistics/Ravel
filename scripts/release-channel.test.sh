#!/usr/bin/env bash
# Release channel fixtures for docs/specs/release-channels.md: the channel
# rules, the two rendered classes, the Taskfile's required CHANNEL, the
# baseline against the target channel, publication and each resume, the
# installation through the versioned keg, the release evidence and the
# promotion to unversioned ravel. Local jj and Git remotes stand for GitHub
# and the tap; gh and brew are fixtures.
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
  section_promote
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
    # The deprecation date is the commit's date at UTC-12, never the UTC date,
    # so Homebrew west of UTC also sees a date that is not in the future.
    local dates="$fixtures/dates" when expected_date
    git init -q "$dates"
    while IFS=' ' read -r when expected_date; do
      GIT_COMMITTER_DATE="$when" git -C "$dates" commit -q --allow-empty -m "$when"
      actual="$(promotion_deprecation_date "$dates" HEAD)"
      [[ "$actual" == "$expected_date" ]] || fail "a commit at $when has the deprecation date $actual, not $expected_date"
    done <<'EOF'
2026-11-12T00:15:00Z 2026-11-11
2026-11-12T11:59:59Z 2026-11-11
2026-11-12T12:00:00Z 2026-11-12
2026-11-12T23:59:59+14:00 2026-11-11
EOF
  ) || exit 1
  ok 'channel mapping, admission, baseline and deprecation date rules'
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

  refuses 'release:promote without VERSION' 'missing required variables: VERSION' task --dir "$repo" release:promote
  [[ ! -e "$repo/.jj/release-lock" ]] || fail 'release:promote took the lock before refusing'
  task --dir "$repo" --dry release:promote VERSION=2.0.0 >"$fixtures/dry" 2>&1
  grep -Fxq "task: [release:promote] bash scripts/release-promote.sh --version 2.0.0" "$fixtures/dry" ||
    fail "release:promote does not run the promotion: $(tr '\n' ' ' <"$fixtures/dry")"
  task --dir "$repo" --dry release:promote VERSION=2.0.0 'READINESS=/r d.json' EVIDENCE=e.json >"$fixtures/dry" 2>&1
  grep -Fxq "task: [release:promote] bash scripts/release-promote.sh --version 2.0.0 --readiness '/r d.json' --evidence e.json" "$fixtures/dry" ||
    fail "release:promote does not pass READINESS and EVIDENCE: $(tr '\n' ' ' <"$fixtures/dry")"
  if grep -E 'cargo|release-(build|publish|book|prepare)\.sh|git push' "$fixtures/dry"; then fail 'release:promote cuts, builds or publishes'; fi
  ok 'release:promote requires VERSION and runs only the promotion'
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
  jj -R "$tap" git push --remote origin -b main >/dev/null 2>&1
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
  # At this time the UTC date is 2026-11-12 and the date at UTC-12 is 2026-11-11.
  GIT_COMMITTER_DATE=2026-11-12T00:15:00Z "$real_git" -C "$source" commit -qm fixture
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
  # jj's settings select a second remote, but publication uses origin only.
  "$real_git" init -q --bare "$fixtures/tap-mirror.git"
  jj -R "$tap" git remote add mirror "$fixtures/tap-mirror.git"
  jj -R "$tap" config set --repo git.fetch mirror
  jj -R "$tap" config set --repo git.push mirror
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
  [[ -z "$("$real_git" --git-dir="$fixtures/tap-mirror.git" for-each-ref)" ]] || fail 'publication pushed to the remote of git.push'
  ok 'a versioned release writes only ravel@2.rb to origin, and each resume keeps the channel'

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
# With "fresh", the prefix holds neither ravel nor ravel@2.
make_brew_prefix() {
  local p="$1"
  mkdir -p "$p/opt" "$p/bin" "$p/share/ravel/skills"
  : >"$p/.log"
  [[ "${2:-}" != fresh ]] || return 0
  mkdir -p "$p/Cellar/ravel/1.1.0/bin" "$p/Cellar/ravel/1.1.0/share/ravel"
  printf '#!/bin/sh\necho "ravel 1.1.0"\n' >"$p/Cellar/ravel/1.1.0/bin/ravel"
  chmod +x "$p/Cellar/ravel/1.1.0/bin/ravel"
  printf '{"schemaVersion":1,"skillDirectory":"%s"}\n' "$p/share/ravel/skills" >"$p/Cellar/ravel/1.1.0/share/ravel/installation.json"
  # As in the real 1.1.0 keg: the private worker, its SDK and Homebrew's receipt.
  mkdir -p "$p/Cellar/ravel/1.1.0/libexec/ravel/sdk"
  printf '#!/bin/sh\necho worker\n' >"$p/Cellar/ravel/1.1.0/libexec/ravel/ravel-policy"
  chmod +x "$p/Cellar/ravel/1.1.0/libexec/ravel/ravel-policy"
  printf 'export {};\n' >"$p/Cellar/ravel/1.1.0/libexec/ravel/sdk/index.ts"
  printf '{"installed_on_request":true}\n' >"$p/Cellar/ravel/1.1.0/INSTALL_RECEIPT.json"
  ln -s ../Cellar/ravel/1.1.0 "$p/opt/ravel"
  ln -s ../Cellar/ravel/1.1.0/bin/ravel "$p/bin/ravel"
}

write_brew() {
  cat >"$fixtures/bin/brew" <<'SH'
#!/usr/bin/env bash
# A Homebrew fixture over FAKE_BREW_PREFIX. As Homebrew 7.0.13 does, an
# install of ravel@2 without --skip-link links its keg into the shared bin
# when neither ravel nor ravel@2 is installed
# (FormulaInstaller#auto_link_versioned_keg_only?). Faults: FAKE_BREW_LINK=1 links a
# versioned keg into the shared bin, FAKE_BREW_KEG_ONLY=false reports it as
# not keg-only, FAKE_BREW_RECORD=keg names the keg in its record,
# FAKE_BREW_BIN_VERSION makes its command report another version,
# FAKE_BREW_UPGRADE_UNVERSIONED=1 also upgrades and relinks unversioned ravel
# and FAKE_BREW_REMOVE_OLD=1 removes the 1.1.0 keg, as a cleanup would.
# FAKE_BREW_OLD_FAULT=remove-worker removes the 1.1.0 worker, change-sdk changes
# its SDK and receipt rewrites its INSTALL_RECEIPT.json, as Homebrew may. An
# upgrade of a formula whose version is installed changes nothing, as in
# Homebrew, which only warns.
set -euo pipefail
p="$FAKE_BREW_PREFIX"
log() { printf '%s\n' "$*" >>"$p/.log"; }
skip_link=0
install_keg() {
  local name="$1" v="$FAKE_BREW_VERSION" keg dir link="${FAKE_BREW_LINK:-0}"
  if [[ "$name" == ravel@2 && "$skip_link" == 0 && ! -d "$p/Cellar/ravel" && ! -d "$p/Cellar/ravel@2" ]]; then link=1; fi
  keg="$p/Cellar/$name/$v"
  mkdir -p "$keg/bin" "$keg/share/ravel"
  printf '#!/bin/sh\necho "ravel %s"\n' "${FAKE_BREW_BIN_VERSION:-$v}" >"$keg/bin/ravel"
  chmod +x "$keg/bin/ravel"
  dir="$p/share/ravel/skills"
  [[ "${FAKE_BREW_RECORD:-}" == keg ]] && dir="$keg/share/ravel/skills"
  printf '{"schemaVersion":1,"skillDirectory":"%s"}\n' "$dir" >"$keg/share/ravel/installation.json"
  ln -sfn "../Cellar/$name/$v" "$p/opt/$name"
  if [[ "$name" == ravel || "$link" == 1 ]]; then ln -sfn "../Cellar/$name/$v/bin/ravel" "$p/bin/ravel"; fi
  if [[ "${FAKE_BREW_REMOVE_OLD:-0}" == 1 ]]; then rm -rf "$p/Cellar/ravel/1.1.0"; fi
  case "${FAKE_BREW_OLD_FAULT:-}" in
    remove-worker) rm -f "$p/Cellar/ravel/1.1.0/libexec/ravel/ravel-policy" ;;
    change-sdk) printf 'changed\n' >>"$p/Cellar/ravel/1.1.0/libexec/ravel/sdk/index.ts" ;;
    receipt) printf '{"installed_on_request":false}\n' >"$p/Cellar/ravel/1.1.0/INSTALL_RECEIPT.json" ;;
  esac
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
  install)
    log "install ${*:2} cleanup=${HOMEBREW_NO_INSTALL_CLEANUP:-unset}"
    if [[ "$2" == --skip-link ]]; then skip_link=1; fi
    formula="${!#}"
    install_keg "${formula##*/}"
    ;;
  upgrade)
    log "upgrade $2 cleanup=${HOMEBREW_NO_INSTALL_CLEANUP:-unset}"
    [[ -d "$p/Cellar/$2/$FAKE_BREW_VERSION" ]] || install_keg "$2"
    ;;
  test) log "test $2" ;;
  info)
    name="$3" keg_only=false reason=null linked=null
    if [[ "$name" == ravel@2 ]]; then keg_only="${FAKE_BREW_KEG_ONLY:-true}" reason='{"reason":":versioned_formula","explanation":""}'; fi
    if [[ "$name" == ravel || "$(readlink "$p/bin/ravel" || true)" == "../Cellar/$name/"* ]]; then linked="\"$(basename "$(readlink "$p/opt/$name")")\""; fi
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

  # Without either formula, Homebrew links a plain install of ravel@2.
  p="$fixtures/brew-fresh" && make_brew_prefix "$p" fresh && export FAKE_BREW_PREFIX="$p"
  install --channel versioned-2 >"$fixtures/out" 2>&1 ||
    fail "the versioned install on a prefix without ravel refused: $(tail -n 3 "$fixtures/out")"
  [[ ! -e "$p/bin/ravel" && ! -L "$p/bin/ravel" ]] || fail "the versioned install linked $p/bin/ravel on a prefix without ravel"
  [[ "$(readlink "$p/opt/ravel@2")" == ../Cellar/ravel@2/2.0.0 ]] || fail 'the versioned install on a prefix without ravel has no opt link'
  ok 'ravel@2 installs without a shared command on a prefix without ravel or ravel@2'

  p="$fixtures/brew-versioned" && make_brew_prefix "$p" && export FAKE_BREW_PREFIX="$p"
  before="$(unversioned_links "$p")"
  install --channel versioned-2 >"$fixtures/out" 2>&1 || fail "the versioned install refused: $(tail -n 3 "$fixtures/out")"
  [[ "$(cat "$p/.log")" == $'update cleanup=1\ninstall --skip-link linkuistics/taps/ravel@2 cleanup=1\ntest ravel@2' ]] ||
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
FAKE_BREW_OLD_FAULT=remove-worker	earlier keg .*/1.1.0 changed: < ./libexec/ravel/ravel-policy file x
FAKE_BREW_OLD_FAULT=change-sdk	earlier keg .*/1.1.0 changed: < ./libexec/ravel/sdk/index.ts file -
EOF
  [[ ! -e "$fixtures/decoy-ran" ]] || fail 'a refused verification ran the command on PATH'
  ok 'a linked, non-keg-only, misrecorded or misversioned keg, a changed unversioned ravel or a changed earlier keg refuses'
  p="$fixtures/brew-receipt" && make_brew_prefix "$p" && export FAKE_BREW_PREFIX="$p"
  with_env FAKE_BREW_OLD_FAULT=receipt install --channel versioned-2 >"$fixtures/out" 2>&1 ||
    fail "a rewritten Homebrew receipt in the 1.1.0 keg refused: $(tail -n 3 "$fixtures/out")"
  ok "Homebrew's rewrite of an earlier keg's INSTALL_RECEIPT.json is accepted"
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
  printf '<a href="v2.0.0/">ravel 2.0.0</a>\n' >"$site/index.html"
  "$real_git" -C "$site" add -A
  "$real_git" -C "$site" commit -qm 'Book for v2.0.0'
  local book_commit
  book_commit="$("$real_git" -C "$site" rev-parse HEAD)"
  printf '<a href="v2.0.0/">ravel 2.0.0</a> (newest)\n' >"$site/index.html"
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

# The promotion of the published ravel@2 2.0.0 to unversioned ravel. It reuses
# the tag, the archives, the book and the evidence of the sections above, with
# a fresh tap in state B and a prefix that holds ravel 1.1.0 and ravel@2.
section_promote() {
  local source="$fixtures/source" tap="$fixtures/promote-tap" remote="$fixtures/promote-tap-remote.git" p
  local evidence="$source/target/release-evidence/v2.0.0.json" readiness="$source/target/release-evidence/v2.0.0.readiness.json"
  export RELEASE_TEST_FORMULA_CALLS="$fixtures/formula-calls" RELEASE_TEST_STRAY="$fixtures/stray"
  cp "$subject/scripts/release-promote.sh" "$source/scripts/"
  # The Homebrew test has its live check, task release:formula. The stub
  # records each call and refuses on request.
  cat >"$source/scripts/release-formula.sh" <<'SH'
#!/usr/bin/env bash
set -euo pipefail
[[ "$1" == --test && $# == 6 && -f "$4" && -f "$5" && -f "$6" ]] || exit 96
printf '%s %s %s %s\n' "$1" "$2" "${4##*/}" "${5##*/}" >>"$RELEASE_TEST_FORMULA_CALLS"
[[ "${RELEASE_TEST_FORMULA_FAIL:-0}" != 1 ]]
SH
  # Promotion builds nothing: cargo and the build script only record a call.
  mkdir -p "$fixtures/promote-bin"
  # shellcheck disable=SC2016 # evaluated by the stubs
  printf '#!/bin/sh\necho "cargo $*" >>"$RELEASE_TEST_STRAY"\nexit 1\n' >"$fixtures/promote-bin/cargo"
  # shellcheck disable=SC2016 # evaluated by the stubs
  printf '#!/bin/sh\necho "release-build $*" >>"$RELEASE_TEST_STRAY"\nexit 1\n' >"$source/scripts/release-build.sh"
  chmod +x "$fixtures/promote-bin/cargo" "$source/scripts/release-build.sh" "$source/scripts/release-formula.sh"

  # State B: ravel 1.1.0, the published ravel@2 2.0.0 and a consumer at 22.8.0.
  "$real_git" init -q --bare "$remote"
  # A marker file makes the remote reject a push.
  printf '#!/bin/sh\n[ ! -e "%s/reject-push" ]\n' "$fixtures" >"$remote/hooks/pre-receive"
  chmod +x "$remote/hooks/pre-receive"
  jj git init --colocate "$tap" >/dev/null
  mkdir -p "$tap/Formula"
  printf 'class Ravel < Formula\n  version "1.1.0"\nend\n' >"$tap/Formula/ravel.rb"
  cp "$fixtures/ravel@2.rb" "$tap/Formula/ravel@2.rb"
  printf 'class Grove < Formula\n  version "22.8.0"\nend\n' >"$tap/Formula/grove.rb"
  printf 'separate command\n' >"$tap/Formula/ravel-lite.rb"
  jj -R "$tap" describe -m 'state B' >/dev/null
  jj -R "$tap" git remote add origin "$remote"
  jj -R "$tap" git push --named main=@ >/dev/null 2>&1
  jj -R "$tap" new main >/dev/null 2>&1
  # jj's git.push selects a second remote, but promotion verifies and pushes origin.
  "$real_git" init -q --bare "$fixtures/promote-tap-mirror.git"
  jj -R "$tap" git remote add mirror "$fixtures/promote-tap-mirror.git"
  jj -R "$tap" config set --repo git.push mirror
  export RAVEL_TAP_DIR="$tap"

  p="$fixtures/brew-promote" && make_brew_prefix "$p" && export FAKE_BREW_PREFIX="$p" FAKE_BREW_VERSION=2.0.0
  brew install linkuistics/taps/ravel@2 >/dev/null
  : >"$p/.log"

  # The documented example receipt, bound to this cut.
  receipt() {
    jq --slurpfile ev "$evidence" --arg version "${1:-22.9.0}" \
      '.ravel.tagCommit = $ev[0].tagCommit | .ravel.assets = $ev[0].assets | .consumer.version = $version' \
      "$repo_root/docs/specs/release-channels-readiness.example.json"
  }
  # shellcheck disable=SC2031 # PATH is this test's own
  promote() { (cd "$source" && PATH="$fixtures/promote-bin:$PATH" bash scripts/release-promote.sh "$@"); }
  tip() { "$real_git" --git-dir="$remote" rev-parse main; }
  remote_file() { "$real_git" --git-dir="$remote" show "main:Formula/$1"; }
  dist_digests() { shasum -a 256 "$RELEASE_TEST_DIST/"*.tar.xz; }
  local tip_b dist_before gh_before book_before
  tip_b="$(tip)"
  dist_before="$(dist_digests)"
  gh_before="$(cat "$RELEASE_TEST_LOG")"
  book_before="$(cat "$RELEASE_TEST_BOOK_CALLS")"
  untouched() {
    [[ "$(tip)" == "$tip_b" ]] || fail "$1 changed the tap"
    [[ ! -s "$p/.log" ]] || fail "$1 ran Homebrew: $(tr '\n' ';' <"$p/.log")"
  }

  # Refusals before any tap write or Homebrew mutation.
  refuses 'promote without a version' 'usage' promote
  refuses 'promote a major-1 version' 'has major 1' promote --version 1.1.1
  refuses 'promote without evidence' 'no release evidence' promote --version 2.0.0 --evidence "$fixtures/absent.json"
  refuses 'promote without a readiness receipt' 'no consumer readiness receipt' promote --version 2.0.0
  : >"$readiness"
  refuses 'an empty readiness receipt' 'v2.0.0.readiness.json is not one JSON object' promote --version 2.0.0
  { receipt; receipt; } >"$readiness"
  refuses 'a receipt of two documents' 'v2.0.0.readiness.json is not one JSON object' promote --version 2.0.0
  printf ' \n' >"$fixtures/evidence-blank.json"
  refuses 'whitespace-only evidence' 'evidence-blank.json is not one JSON object' promote --version 2.0.0 --evidence "$fixtures/evidence-blank.json"
  [[ ! -e "$RELEASE_TEST_FORMULA_CALLS" ]] || fail 'a missing receipt reached the formula test'
  receipt | jq '.ravel.assets[0].sha256 = ("0" * 64)' >"$readiness"
  refuses 'a receipt for other archives' 'ravel.assets are not the cut asset digests' promote --version 2.0.0
  receipt | jq 'del(.consumer.verifiedRoutes)' >"$readiness"
  refuses 'a receipt without routes' 'consumer.verifiedRoutes' promote --version 2.0.0
  receipt >"$readiness"
  refuses 'promotion before the compatible consumer' "Formula/grove.rb is at 22.8.0, not the receipt's 22.9.0" promote --version 2.0.0
  untouched 'an unready promotion'
  tap_commit "$tap" grove.rb 'class Grove < Formula
  version "22.9.0"
end' 'grove v22.9.0'
  tip_b="$(tip)"
  refuses 'a mismatched archive' 'has SHA-256' with_env RELEASE_TEST_DOWNLOAD_CORRUPT=1 promote --version 2.0.0
  jq '.tagCommit = ("0" * 40)' "$evidence" >"$fixtures/evidence-tag.json"
  receipt | jq '.ravel.tagCommit = ("0" * 40)' >"$fixtures/readiness-tag.json"
  refuses 'evidence of another tag commit' 'public tag v2.0.0 is' promote --version 2.0.0 --evidence "$fixtures/evidence-tag.json" --readiness "$fixtures/readiness-tag.json"
  jq '.book.commit = ("0" * 40)' "$evidence" >"$fixtures/evidence-book.json"
  refuses 'evidence of another book' 'book of v2.0.0 on gh-pages was last changed' promote --version 2.0.0 --evidence "$fixtures/evidence-book.json"
  refuses 'a failed formula test' 'failed their Homebrew test' with_env RELEASE_TEST_FORMULA_FAIL=1 promote --version 2.0.0
  untouched 'a refused verification'
  tap_commit "$tap" ravel@2.rb "$(cat "$fixtures/ravel@2.rb")
# edited" 'ravel@2 edited'
  tip_b="$(tip)"
  refuses 'an unexpected ravel@2.rb' 'neither the published formula' promote --version 2.0.0
  untouched 'an unexpected ravel@2.rb'
  tap_commit "$tap" ravel@2.rb "$(cat "$fixtures/ravel@2.rb")" 'ravel@2 v2.0.0'
  tip_b="$(tip)"
  ok 'promotion refuses without evidence, a matching receipt, the compatible consumer, the cut identities or the published ravel@2'

  # A rejected push leaves one local commit; the retry pushes that commit.
  touch "$fixtures/reject-push"
  refuses 'a rejected tap push' 'stays local' promote --version 2.0.0
  rm "$fixtures/reject-push"
  untouched 'a rejected push'
  local local_commit
  local_commit="$(jj -R "$tap" log --no-graph -r main -T commit_id)"
  [[ "$local_commit" != "$tip_b" ]] || fail 'the rejected push left no local promotion commit'
  promote --version 2.0.0 >"$fixtures/out" 2>&1 || fail "the promotion retry refused: $(tail -n 3 "$fixtures/out")"
  grep -Fq "pushing the tap's local promotion commit $local_commit" "$fixtures/out" || fail 'the retry did not push the local commit'
  [[ "$(tip)" == "$local_commit" ]] || fail 'the retry published a commit other than the local promotion'
  [[ "$("$real_git" --git-dir="$remote" rev-parse main~1)" == "$tip_b" ]] || fail 'the promotion is not one commit on state B'

  # One commit: ravel.rb from the same assets, ravel@2.rb gains one line.
  # The tag commit is at 2026-11-12T00:15:00Z, which is 2026-11-11 at UTC-12.
  local date=2026-11-11
  # shellcheck disable=SC2016 # evaluated by the child shell
  bash -c 'source "$1" && deprecate_versioned_formula "$2" "$3" "$4"' _ "$subject/scripts/release-channel.sh" \
    "$fixtures/ravel@2.rb" "$fixtures/ravel@2.promoted.rb" "$date" >/dev/null || fail 'cannot derive the expected deprecated ravel@2.rb'
  remote_file ravel.rb | cmp -s - "$fixtures/ravel.rb" || fail 'the promoted ravel.rb is not the unversioned rendering of the release assets'
  remote_file ravel@2.rb | cmp -s - "$fixtures/ravel@2.promoted.rb" || fail 'the promoted ravel@2.rb is not the published one with its deprecation'
  [[ "$(diff <(cat "$fixtures/ravel@2.rb") <(remote_file ravel@2.rb) | grep -c '^[<>]')" == 1 ]] ||
    fail 'the promotion changed more than one line of ravel@2.rb'
  remote_file ravel@2.rb | grep -Fxq "  deprecate! date: \"$date\", because: :versioned_formula, replacement_formula: \"ravel\"" ||
    fail 'ravel@2.rb is not deprecated in favour of ravel'
  [[ "$("$real_git" --git-dir="$remote" diff --name-only main~1 main | tr '\n' ' ')" == 'Formula/ravel.rb Formula/ravel@2.rb ' ]] ||
    fail 'the promotion commit changed other tap files'
  [[ "$("$real_git" --git-dir="$remote" log -1 --format=%s main)" == 'ravel v2.0.0 (promote ravel@2 v2.0.0)' ]] ||
    fail 'the promotion commit is not named'
  [[ "$(remote_file ravel-lite.rb)" == 'separate command' ]] || fail 'the promotion changed another formula'
  [[ -z "$("$real_git" -C "$tap" status --porcelain)" ]] || fail 'the tap is not clean after promotion'
  [[ -z "$("$real_git" --git-dir="$fixtures/promote-tap-mirror.git" for-each-ref)" ]] || fail 'the promotion pushed to the remote of git.push'
  ok 'one tap commit pushed to origin renders ravel.rb from the release assets and deprecates ravel@2 in favour of ravel'

  # Nothing was cut, built, uploaded or rewritten.
  [[ "$(dist_digests)" == "$dist_before" ]] || fail 'the promotion changed an archive'
  [[ "$(cat "$RELEASE_TEST_LOG")" == "$gh_before" ]] || fail 'the promotion created a GitHub Release'
  [[ "$(cat "$RELEASE_TEST_BOOK_CALLS")" == "$book_before" ]] || fail 'the promotion called the book step'
  [[ ! -e "$RELEASE_TEST_STRAY" ]] || fail "the promotion built: $(cat "$RELEASE_TEST_STRAY")"
  grep -Fxq -- '--test 2.0.0 ravel@2.rb ravel.rb' "$RELEASE_TEST_FORMULA_CALLS" || fail 'the promotion did not test its formulae through Homebrew'
  ok 'promotion leaves the archives, the GitHub Release and the book unchanged and builds nothing'

  # Installation upgrades ravel and keeps every earlier keg.
  [[ "$(cat "$p/.log")" == $'update cleanup=1\nupgrade ravel cleanup=1\ntest ravel' ]] ||
    fail "the promotion's installation ran $(tr '\n' ';' <"$p/.log")"
  [[ "$("$p/Cellar/ravel/1.1.0/bin/ravel" --version)" == 'ravel 1.1.0' ]] || fail 'the 1.1.0 keg is gone'
  [[ -x "$p/Cellar/ravel@2/2.0.0/bin/ravel" ]] || fail 'the ravel@2 keg is gone'
  [[ "$(readlink "$p/opt/ravel")" == ../Cellar/ravel/2.0.0 && "$p/bin/ravel" -ef "$p/Cellar/ravel/2.0.0/bin/ravel" ]] ||
    fail 'unversioned ravel does not select the 2.0.0 keg'
  local record="$source/target/release-evidence/v2.0.0.promotion.json"
  jq -e --arg tap "$(tip)" --arg ravel "$(remote_file ravel.rb | shasum -a 256 | cut -d' ' -f1)" \
    --arg versioned "$(remote_file ravel@2.rb | shasum -a 256 | cut -d' ' -f1)" --arg date "$date" \
    --arg old "$p/Cellar/ravel/1.1.0" --arg versionedKeg "$p/Cellar/ravel@2/2.0.0" --slurpfile ev "$evidence" '
    .schemaVersion == 1 and .tagCommit == $ev[0].tagCommit and .assets == $ev[0].assets
    and .tap.commit == $tap and .tap.deprecationDate == $date
    and .tap.formulae == [{file: "Formula/ravel.rb", sha256: $ravel}, {file: "Formula/ravel@2.rb", sha256: $versioned}]
    and .readiness.consumer.version == "22.9.0" and .installed.version == "ravel 2.0.0"
    and (.installed.retainedKegs | sort) == ([$old, $versionedKeg] | sort)' "$record" >/dev/null ||
    fail "the promotion evidence is wrong: $(jq -c . "$record" | cut -c1-300)"
  ok 'installation upgrades ravel with cleanup suppressed, keeps the 1.1.0 and ravel@2 kegs and is recorded'

  # A retry keeps the published promotion and the matching installation.
  tip_b="$(tip)"
  : >"$p/.log"
  promote --version 2.0.0 >"$fixtures/out" 2>&1 || fail "the repeated promotion refused: $(tail -n 3 "$fixtures/out")"
  grep -Fq 'the tap already holds the promotion; kept' "$fixtures/out" || fail 'the repeated promotion did not keep the tap'
  [[ "$(tip)" == "$tip_b" ]] || fail 'the repeated promotion changed the tap'
  [[ "$(cd "$p/Cellar/ravel" && printf '%s ' *)" == '1.1.0 2.0.0 ' ]] || fail 'the repeated promotion changed the kegs'
  ok 'a retry keeps the published promotion and the matching installation'

  # A retry reports a mismatch and overwrites nothing.
  tap_commit "$tap" ravel.rb 'class Ravel < Formula
  version "2.0.0"
end' 'ravel v2.0.0 by another hand'
  tip_b="$(tip)"
  : >"$p/.log"
  refuses 'a promotion over another ravel.rb' 'ravel.rb is not this promotion' promote --version 2.0.0
  untouched 'a mismatched ravel.rb'
  [[ "$(remote_file ravel.rb)" == $'class Ravel < Formula\n  version "2.0.0"\nend' ]] || fail 'the mismatched ravel.rb was overwritten'
  tap_commit "$tap" ravel.rb "$(cat "$fixtures/ravel.rb")" 'ravel v2.0.0'
  p="$fixtures/brew-promote-mismatch" && make_brew_prefix "$p" && export FAKE_BREW_PREFIX="$p"
  mkdir -p "$p/Cellar/ravel/2.0.0/bin"
  printf '#!/bin/sh\necho "ravel 2.0.9"\n' >"$p/Cellar/ravel/2.0.0/bin/ravel"
  chmod +x "$p/Cellar/ravel/2.0.0/bin/ravel"
  ln -sfn ../Cellar/ravel/2.0.0 "$p/opt/ravel"
  ln -sfn ../Cellar/ravel/2.0.0/bin/ravel "$p/bin/ravel"
  refuses 'a promotion over another installation' 'does not report ravel 2.0.0' promote --version 2.0.0
  [[ "$("$p/Cellar/ravel/2.0.0/bin/ravel" --version)" == 'ravel 2.0.9' ]] || fail 'the mismatched installation was overwritten'
  p="$fixtures/brew-promote-cleanup" && make_brew_prefix "$p" && export FAKE_BREW_PREFIX="$p"
  refuses 'an installation that removes the 1.1.0 keg' 'earlier keg .*/1.1.0 changed: < \. dir' with_env FAKE_BREW_REMOVE_OLD=1 promote --version 2.0.0
  p="$fixtures/brew-promote-worker" && make_brew_prefix "$p" && export FAKE_BREW_PREFIX="$p"
  refuses 'an installation that removes the 1.1.0 worker' 'earlier keg .*/1.1.0 changed: < ./libexec/ravel/ravel-policy' with_env FAKE_BREW_OLD_FAULT=remove-worker promote --version 2.0.0
  ok 'a retry reports a mismatched formula or installation and overwrites neither, and a removed old keg or worker refuses'
  unset RAVEL_TAP_DIR
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
promotion deprecates nothing	scripts/release-channel.sh	'{ print } $0 == keg_only { print line }'	'{ print }'
promote skips readiness	scripts/release-promote.sh	  verify_readiness "$readiness" "$evidence" "$version"	  :
promote skips tag	scripts/release-promote.sh	  [[ "$remote" == "$commit" ]] ||	  true ||
promote trusts assets	scripts/release-promote.sh	    [[ "$actual" == "$expected" ]] ||	    true ||
promote skips book	scripts/release-promote.sh	  verify_book "$version" "$book_commit"	  :
promote skips consumer	scripts/release-promote.sh	  [[ "$consumer_actual" == "$consumer_version" ]] ||	  true ||
promote skips formula test	scripts/release-promote.sh	  bash "$REPO_ROOT/scripts/release-formula.sh" --test	  : "$REPO_ROOT/scripts/release-formula.sh" --test
promote any ravel@2	scripts/release-promote.sh	root:Formula/ravel@2.rb 2>/dev/null) "$versioned" ||	root:Formula/ravel@2.rb 2>/dev/null) "$versioned" || true ||
promote overwrites ravel.rb	scripts/release-promote.sh	    die "the tap's Formula/ravel@2.rb holds the promotion, but	    : "the tap's Formula/ravel@2.rb holds the promotion, but
promote never keeps	scripts/release-promote.sh	  if tap_holds main@origin "$dir"; then	  if false; then
promote recommits	scripts/release-promote.sh	  if [[ "$main" != "$origin" ]]; then	  if false; then
promote skips install	scripts/release-promote.sh	  bash "$REPO_ROOT/scripts/release-install.sh" --channel unversioned --version "$version" ||	  true ||
install links ravel@2	scripts/release-install.sh	    brew install --skip-link "$TAP/$CHANNEL_FORMULA"	    brew install "$TAP/$CHANNEL_FORMULA"
install drops old kegs	scripts/release-install.sh	    [[ "$(keg_manifest "$earlier")" == "${manifests[index]}" ]] ||	    true ||
install ignores contents	scripts/release-install.sh	      printf '%s file %s %s\n' "$rel" "$mode" "$(shasum -a 256 <"$path" | cut -d' ' -f1)"	      printf '%s file %s\n' "$rel" "$mode"
install compares receipts	scripts/release-install.sh	    elif [[ "$rel" == ./INSTALL_RECEIPT.json ]]; then	    elif false; then
tap push default remote	scripts/release-channel.sh	  jj -R "$1" git push --remote origin -b main	  jj -R "$1" git push -b main
deprecation date by UTC	scripts/release-channel.sh	  TZ=AOE12 git -C	  TZ=UTC git -C
promote dates by UTC	scripts/release-promote.sh	  date="$(promotion_deprecation_date "$REPO_ROOT" "$tag_commit")"	  date="$(TZ=UTC git -C "$REPO_ROOT" log -1 --format=%cd --date=format-local:%Y-%m-%d "$tag_commit")"
promote admits empty JSON	scripts/release-promote.sh	  if ! jq -es 'length == 1 and (.[0] | type == "object")' "$file" >/dev/null 2>&1; then	  if false; then
task promote unversioned	Taskfile.yml	bash scripts/release-promote.sh --version {{shellQuote .VERSION}}	bash scripts/release-promote.sh --version 2.0.0 --readiness x
EOF
}

case "${1:-}" in
  mutants) mutants ;;
  '') controls ;;
  *) fail 'usage: release-channel.test.sh [mutants]' ;;
esac
