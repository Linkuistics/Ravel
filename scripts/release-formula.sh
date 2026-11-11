#!/usr/bin/env bash
# Render both channels' formulae from exact archives and exercise each through
# Homebrew's API. This does not install a keg or edit a tap. The caller must
# smoke all targets before using a rendered formula for publication.
#
#   release-formula.sh ARCHIVES_DIR            keep both formulae (unreleased check)
#   release-formula.sh ARCHIVES_DIR CHANNEL    keep only CHANNEL's formula (release)
#
# docs/specs/release-channels.md: the channels share one release's archives,
# so the test checks that the two formulae differ only in class and keg_only.
set -euo pipefail
IFS=$'\n\t'
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
# shellcheck source=scripts/release-common.sh
source "$root/scripts/release-common.sh"
die() {
  echo "release-formula: $*" >&2
  exit 1
}
[[ $# == 1 || $# == 2 ]] || die 'usage: release-formula.sh ARCHIVES_DIR [CHANNEL]'
dir="$(cd "$1" && pwd -P)"
selected=''
if [[ $# == 2 ]]; then
  reason="$(channel_select "$2")" || die "$reason"
  selected="$2"
fi
id="$(cargo pkgid --quiet -p ravel)"
version="${id##*[#@]}"
[[ "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]] || die "expected a stable version, got $version"
scratch="$(mktemp -d)"
# Skill snapshots are read-only; a failed formula test can leave them behind.
trap 'chmod -R u+w "$scratch" 2>/dev/null; rm -rf "$scratch"' EXIT
# Both channels render and pass Homebrew's methods on every run, so the
# keg-only class is exercised before the first major-2 cut exists.
if [[ -n "$selected" ]]; then
  channel_select "$selected"
  [[ -z "$CHANNEL_MAJOR" || "$(version_major "$version")" == "$CHANNEL_MAJOR" ]] ||
    die "CHANNEL=$selected publishes only major $CHANNEL_MAJOR, not $version"
fi
channels=()
for channel in "${RELEASE_CHANNELS[@]}"; do
  channel_select "$channel"
  render_archive_formula "$dir" "$version" "$root/scripts/templates/ravel.rb.tmpl" "$scratch/$CHANNEL_FILE" "$channel" ||
    die "archive/formula validation failed for CHANNEL=$channel"
  channels+=("$channel")
done
[[ "$(uname -sm)" == 'Darwin arm64' ]] || die 'formula validation requires the macOS arm64 archive host'
archive="$dir/ravel-v$version-aarch64-apple-darwin.tar.xz"
extract_exactly "$archive" "$scratch"
# The installed Homebrew library executes the actual install/test methods.
# Prefix and testpath are redirected to private directories, no mock DSL.
# https://docs.brew.sh/rubydoc/Formula.html#prefix-instance_method
# Each argument group is CHANNEL FORMULA PREFIX TESTPATH.
groups=()
for channel in "${channels[@]}"; do
  channel_select "$channel"
  mkdir "$scratch/$channel" "$scratch/$channel/prefix café" "$scratch/$channel/test"
  groups+=("$channel" "$scratch/$CHANNEL_FILE" "$scratch/$channel/prefix café" "$scratch/$channel/test")
done
HOMEBREW_NO_AUTO_UPDATE=1 brew ruby "$root/scripts/release-formula.test.rb" \
  "$scratch/ravel-v$version-aarch64-apple-darwin" "${groups[@]}"
for channel in "${channels[@]}"; do
  [[ -z "$selected" || "$channel" == "$selected" ]] || continue
  channel_select "$channel"
  mv "$scratch/$CHANNEL_FILE" "$dir/$CHANNEL_FILE"
  echo "release-formula: rendered and tested $dir/$CHANNEL_FILE (unpublished)"
done
