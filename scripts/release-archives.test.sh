#!/usr/bin/env bash
# Transferred exact-manifest controls; build the real worker, never a model.
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

# The layout the spec places ravel in, stated independently of the
# manifest so that dropping an entry cannot quietly narrow what ships.
for required in bin/ravel \
  libexec/ravel/ravel-policy \
  libexec/ravel/sdk/index.d.ts libexec/ravel/sdk/index.ts \
  libexec/ravel/examples/sample.ts \
  libexec/ravel/notices/NOTICES.md libexec/ravel/notices/bun-LICENSE.md; do
  archive_manifest | grep -Fxq -- "$required" || fail "the archive manifest omits $required"
done
archive_executables | while IFS= read -r entry; do
  archive_manifest | grep -Fxq -- "$entry" || fail "executable $entry is not in the archive manifest"
done

# The manifest's ravel files are exactly what `dispatch.sh build`
# emits, so a file that build starts shipping without a manifest entry, or
# stops shipping, fails here rather than at release time. This is the build
# itself, not a restatement of its rules.
bash "$repo_root/crates/ravel/scripts/dispatch.sh" build "$scratch/dispatch-out" >/dev/null
(cd "$scratch/dispatch-out" && find . ! -type d | sed 's|^\./||' | LC_ALL=C sort) >"$scratch/dispatch-built"
archive_manifest | sed -n 's|^libexec/ravel/||p' | LC_ALL=C sort >"$scratch/dispatch-manifest"
diff -u "$scratch/dispatch-built" "$scratch/dispatch-manifest" >&2 ||
  fail "the archive manifest's ravel files differ from what dispatch.sh build emits"

# The formula installs exactly the manifest's executables and its one libexec
# directory; everything else it ships is a top-level metafile.
template="$repo_root/scripts/templates/ravel.rb.tmpl"
formula_bins="$(sed -n 's/^ *bin\.install //p' "$template" | tr -d '"' | tr ',' '\n' | tr -d ' ' | LC_ALL=C sort)"
[[ "$formula_bins" == "$(archive_manifest | grep '^bin/' | LC_ALL=C sort)" ]] ||
  fail "the formula's bin.install ($formula_bins) differs from the manifest's bin/ entries"
grep -Fxq '    libexec.install "libexec/ravel"' "$template" ||
  fail 'the formula does not install libexec/ravel'
if archive_manifest | grep -v -e '^bin/' -e '^libexec/ravel/' | grep -q /; then
  fail 'the manifest ships a directory the formula does not install'
fi

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

# Caller binaries, adapters, policy and choice files never belong in the pair.
for extra in bin/grove bin/grove-llm bin/harness-dispatch \
  libexec/ravel/grove/index.ts libexec/ravel/policy.ts .ravel-choice; do
  stage_archive "$top"
  mkdir -p "$(dirname "$scratch/archive/stage/$top/$extra")"
  touch "$scratch/archive/stage/$top/$extra"
  pack_fixture "$top"
  refused_archive "$top" "unexpected $top/$extra"
done

stage_archive "$top"
touch "$scratch/archive/stage/$top/libexec/ravel/bun-darwin-aarch64-v1.4.2"
pack_fixture "$top"
refused_archive "$top" "unexpected $top/libexec/ravel/bun-darwin-aarch64-v1.4.2"

stage_archive "$top"
mkdir -p "$scratch/archive/stage/$top/libexec/ravel/node_modules/.cache"
pack_fixture "$top"
refused_archive "$top" "unexpected $top/libexec/ravel/node_modules/"

# A probe build (`dispatch.sh probes`) is a test instrument and never ships.
# Beside the worker, the manifest refuses it by path. In the worker's own
# place it passes the manifest but not the installed smoke test, whose front
# refuses a probe's identity (tests/hostile.rs,
# a_probe_build_is_never_accepted_as_an_installations_worker).
stage_archive "$top"
mkdir -p "$scratch/archive/stage/$top/libexec/ravel/probes/unregistered"
printf 'fixture probe\n' >"$scratch/archive/stage/$top/libexec/ravel/probes/unregistered/ravel-policy"
pack_fixture "$top"
refused_archive "$top" "unexpected $top/libexec/ravel/probes/"

stage_archive "$top"
chmod -x "$scratch/archive/stage/$top/libexec/ravel/ravel-policy"
pack_fixture "$top"
refused_archive "$top" "$top/libexec/ravel/ravel-policy is not executable"

stage_archive "$top"
ln -sf ../../bin/ravel "$scratch/archive/stage/$top/libexec/ravel/ravel-policy"
pack_fixture "$top"
refused_archive "$top" "$top/libexec/ravel/ravel-policy is not a regular file"

stage_archive "$top"
mkdir "$scratch/archive/stage/stray"
COPYFILE_DISABLE=1 tar -C "$scratch/archive/stage" --no-xattrs -cJf "$scratch/archive/$top.tar.xz" "$top" stray
refused_archive "$top" "top level is not exactly $top/:"

# A file with an extended attribute, as macOS marks most staged files. Packed
# as bsdtar does by default, the archive gains an AppleDouble member that
# bsdtar hides when it reads; the check must see it. pack_archive must leave
# both the member and the attribute out.
stage_archive "$top"
policy="libexec/ravel/ravel-policy"
xattr -w org.linkuistics.ravel.probe fixture "$scratch/archive/stage/$top/$policy"
tar -C "$scratch/archive/stage" -cJf "$scratch/archive/$top.tar.xz" "$top"
refused_archive "$top" "unexpected $top/libexec/ravel/._ravel-policy"
pack_fixture "$top"
assert_archive "$scratch/archive/$top.tar.xz" "$top" >"$scratch/archive/output" ||
  fail "pack_archive kept macOS metadata: $(cat "$scratch/archive/output")"
mkdir "$scratch/archive/restored"
tar -xJf "$scratch/archive/$top.tar.xz" -C "$scratch/archive/restored" --xattrs
if xattr -p org.linkuistics.ravel.probe "$scratch/archive/restored/$top/$policy" >/dev/null 2>&1; then
  fail 'pack_archive kept an extended attribute'
fi

echo 'release archive tests: all passed'
