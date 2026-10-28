#!/usr/bin/env bash
# Exact-manifest controls against the real Rhai delivery staging function.
set -euo pipefail
IFS=$'\n\t'
repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
scratch="$(mktemp -d)"
trap 'rm -rf "$scratch"' EXIT
# Release archive contents. release-build.sh packs every archive with
# release-common.sh's pack_archive and checks it against the manifest there;
# these cases show that check fires on each way an archive can be wrong, and
# that the manifest agrees with what ravel's build emits and with the
# formula, here, before any version is cut.
# shellcheck source=scripts/release-common.sh
source "$repo_root/scripts/release-common.sh"
fail() {
  echo "FAIL: $*" >&2
  exit 1
}
# Stage placeholder files in the manifest's layout under $scratch/archive/stage.
stage_archive() {
  local top="$1" entry
  rm -rf "$scratch/archive"
  mkdir -p "$scratch/archive/stage/$top"
  while IFS= read -r entry; do
    mkdir -p "$(dirname "$scratch/archive/stage/$top/$entry")"
    printf 'fixture %s\n' "$entry" >"$scratch/archive/stage/$top/$entry"
  done < <(archive_manifest)
  while IFS= read -r entry; do
    chmod +x "$scratch/archive/stage/$top/$entry"
  done < <(archive_executables)
}
pack_fixture() {
  pack_archive "$scratch/archive/stage" "$1" "$scratch/archive/$1.tar.xz"
}
# The check must refuse the archive, reporting exactly the line given.
refused_archive() {
  local top="$1" expected="$scratch/archive/$1.tar.xz: $2"
  if assert_archive "$scratch/archive/$top.tar.xz" "$top" >"$scratch/archive/output"; then
    fail "an archive with '$2' passed the manifest check"
  fi
  grep -Fxq -- "$expected" "$scratch/archive/output" ||
    fail "the manifest check did not report '$2': $(cat "$scratch/archive/output")"
}

# Independent contract: changing the manifest cannot quietly narrow delivery.
expected_manifest='LICENSE
README.md
bin/ravel
share/ravel/examples/llm.rhai
share/ravel/examples/sample.rhai
share/ravel/examples/static.rhai
share/ravel/notices/THIRD-PARTY-NOTICES.md'
[[ "$(archive_manifest)" == "$expected_manifest" ]] || fail 'archive manifest differs from the Rhai delivery contract'
[[ "$(archive_executables)" == bin/ravel ]] || fail 'bin/ravel must be the only executable'

# Stage actual built assets through the same entry point as installation and
# release-build; compare contents as well as the independent path contract.
task --dir "$repo_root" build notices
stage_installation "$repo_root" "$repo_root/target/debug/ravel" "$scratch/installed"
(cd "$scratch/installed" && find . ! -type d | sed 's|^\./||' | LC_ALL=C sort) >"$scratch/staged"
printf '%s\n' "$expected_manifest" | sed '/^[^/]*$/d' >"$scratch/expected"
diff -u "$scratch/expected" "$scratch/staged" || fail 'staged payload differs from the manifest'
for example in static sample llm; do
  cmp "$repo_root/crates/ravel/examples/$example.rhai" "$scratch/installed/share/ravel/examples/$example.rhai"
done
cmp "$repo_root/target/notices/THIRD-PARTY-NOTICES.md" "$scratch/installed/share/ravel/notices/THIRD-PARTY-NOTICES.md"

# Homebrew's real install/test is also exercised by task release:formula.
template="$repo_root/scripts/templates/ravel.rb.tmpl"
grep -Fxq '    bin.install "bin/ravel"' "$template" || fail 'formula does not install bin/ravel'
grep -Fxq '    (share/"ravel").install "share/ravel/examples", "share/ravel/notices"' "$template" || fail 'formula does not install the data directories'

for target in "${TARGETS[@]}"; do
  top="ravel-v1.2.3-$target"
  stage_archive "$top"
  pack_fixture "$top"
  assert_archive "$scratch/archive/$top.tar.xz" "$top" >"$scratch/archive/output" ||
    fail "a complete $target archive failed the manifest check: $(cat "$scratch/archive/output")"
  [[ ! -s "$scratch/archive/output" ]] || fail 'a passing manifest check printed output'
done

# Positive controls: each file's omission is seen to fail, by name.
top="ravel-v1.2.3-${TARGETS[0]}"
while IFS= read -r entry; do
  stage_archive "$top"
  rm "$scratch/archive/stage/$top/$entry"
  pack_fixture "$top"
  refused_archive "$top" "missing $top/$entry"
done < <(archive_manifest)

# Caller binaries, adapters, configuration script and choice files never belong in the archive.
for extra in bin/grove bin/grove-llm bin/harness-dispatch \
  libexec/ravel/grove/index.ts libexec/ravel/policy.ts .ravel-choice; do
  stage_archive "$top"
  mkdir -p "$(dirname "$scratch/archive/stage/$top/$extra")"
  touch "$scratch/archive/stage/$top/$extra"
  pack_fixture "$top"
  refused_archive "$top" "unexpected $top/$extra"
done

# Retired delivery files and test instruments must never re-enter the archive.
for extra in libexec/ravel/ravel-policy libexec/ravel/notices/bun-LICENSE.md \
  libexec/ravel/sdk/index.ts libexec/ravel/probes/unregistered/ravel-policy; do
  stage_archive "$top"
  mkdir -p "$(dirname "$scratch/archive/stage/$top/$extra")"
  touch "$scratch/archive/stage/$top/$extra"
  pack_fixture "$top"
  refused_archive "$top" "unexpected $top/$extra"
done

stage_archive "$top"
chmod -x "$scratch/archive/stage/$top/bin/ravel"
pack_fixture "$top"
refused_archive "$top" "$top/bin/ravel is not executable"

stage_archive "$top"
chmod +x "$scratch/archive/stage/$top/share/ravel/examples/static.rhai"
pack_fixture "$top"
refused_archive "$top" "$top/share/ravel/examples/static.rhai is unexpectedly executable"

# Every payload/metafile must be regular, including data files.
while IFS= read -r entry; do
  stage_archive "$top"
  rm "$scratch/archive/stage/$top/$entry"
  ln -s /dev/null "$scratch/archive/stage/$top/$entry"
  pack_fixture "$top"
  refused_archive "$top" "$top/$entry is not a regular file"
done < <(archive_manifest)

stage_archive "$top"
mkdir "$scratch/archive/stage/stray"
COPYFILE_DISABLE=1 tar -C "$scratch/archive/stage" --no-xattrs -cJf "$scratch/archive/$top.tar.xz" "$top" stray
refused_archive "$top" "top level is not exactly $top/:"

# A file with an extended attribute, as macOS marks most staged files. Packed
# as bsdtar does by default, the archive gains an AppleDouble member that
# bsdtar hides when it reads; the check must see it. pack_archive must leave
# both the member and the attribute out.
stage_archive "$top"
executable="bin/ravel"
xattr -w org.linkuistics.ravel.probe fixture "$scratch/archive/stage/$top/$executable"
tar -C "$scratch/archive/stage" -cJf "$scratch/archive/$top.tar.xz" "$top"
refused_archive "$top" "unexpected $top/bin/._ravel"
pack_fixture "$top"
assert_archive "$scratch/archive/$top.tar.xz" "$top" >"$scratch/archive/output" ||
  fail "pack_archive kept macOS metadata: $(cat "$scratch/archive/output")"
mkdir "$scratch/archive/restored"
tar -xJf "$scratch/archive/$top.tar.xz" -C "$scratch/archive/restored" --xattrs
if xattr -p org.linkuistics.ravel.probe "$scratch/archive/restored/$top/$executable" >/dev/null 2>&1; then
  fail 'pack_archive kept an extended attribute'
fi

echo 'release archive tests: all passed'
