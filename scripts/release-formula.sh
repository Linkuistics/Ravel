#!/usr/bin/env bash
# Render both channels' formulae from exact archives and exercise each through
# Homebrew's API. This does not install a keg or edit a tap. The caller must
# smoke all targets before using a rendered formula for publication.
#
#   release-formula.sh ARCHIVES_DIR            keep both formulae (unreleased check)
#   release-formula.sh ARCHIVES_DIR CHANNEL    keep only CHANNEL's formula (release)
#   release-formula.sh --test VERSION ARCHIVES_DIR VERSIONED UNVERSIONED DEPRECATED
#                                              test three rendered files, keep nothing
#
# docs/specs/release-channels.md: the channels share one release's archives,
# so the test checks that the two formulae differ only in class and keg_only.
# Each run also tests the promotion's deprecated ravel@2.rb through Homebrew's
# own loader. release-promote.sh uses --test for the files it publishes.
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
scratch="$(mktemp -d)"
# Skill snapshots are read-only; a failed formula test can leave them behind.
trap 'chmod -R u+w "$scratch" 2>/dev/null; rm -rf "$scratch"' EXIT

# homebrew_test VERSION ARCHIVES_DIR VERSIONED UNVERSIONED DEPRECATED: run the
# actual install and test methods of both classes from the macOS archive, and
# load the deprecated ravel@2.rb through Homebrew.
homebrew_test() {
  local version="$1" dir="$2" versioned="$3" unversioned="$4" deprecated="$5" work channel file groups=()
  [[ "$(uname -sm)" == 'Darwin arm64' ]] || die 'formula validation requires the macOS arm64 archive host'
  work="$scratch/homebrew"
  mkdir "$work"
  extract_exactly "$dir/ravel-v$version-aarch64-apple-darwin.tar.xz" "$work" ||
    die "cannot extract the macOS archive of $version from $dir"
  # The installed Homebrew library executes the actual install/test methods.
  # Prefix and testpath are redirected to private directories, no mock DSL.
  # https://docs.brew.sh/rubydoc/Formula.html#prefix-instance_method
  # Each argument group is CHANNEL FORMULA PREFIX TESTPATH.
  for channel in "${RELEASE_CHANNELS[@]}"; do
    case "$channel" in
      versioned-2) file="$versioned" ;;
      unversioned) file="$unversioned" ;;
    esac
    mkdir "$work/$channel" "$work/$channel/prefix café" "$work/$channel/test"
    groups+=("$channel" "$file" "$work/$channel/prefix café" "$work/$channel/test")
  done
  HOMEBREW_NO_AUTO_UPDATE=1 brew ruby "$root/scripts/release-formula.test.rb" \
    "$work/ravel-v$version-aarch64-apple-darwin" "$deprecated" "${groups[@]}"
}

if [[ "${1:-}" == --test ]]; then
  [[ $# == 6 ]] || die 'usage: release-formula.sh --test VERSION ARCHIVES_DIR VERSIONED UNVERSIONED DEPRECATED'
  [[ "$2" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]] || die "expected a stable version, got $2"
  homebrew_test "$2" "$(cd "$3" && pwd -P)" "$4" "$5" "$6"
  echo "release-formula: tested $4, $5 and $6 through Homebrew"
  exit 0
fi
[[ $# == 1 || $# == 2 ]] || die 'usage: release-formula.sh ARCHIVES_DIR [CHANNEL] | --test VERSION ARCHIVES_DIR VERSIONED UNVERSIONED DEPRECATED'
dir="$(cd "$1" && pwd -P)"
selected=''
if [[ $# == 2 ]]; then
  reason="$(channel_select "$2")" || die "$reason"
  selected="$2"
fi
id="$(cargo pkgid --quiet -p ravel)"
version="${id##*[#@]}"
[[ "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]] || die "expected a stable version, got $version"
# Both channels render and pass Homebrew's methods on every run, so the
# keg-only class is exercised before the first major-2 cut exists.
if [[ -n "$selected" ]]; then
  channel_select "$selected"
  [[ -z "$CHANNEL_MAJOR" || "$(version_major "$version")" == "$CHANNEL_MAJOR" ]] ||
    die "CHANNEL=$selected publishes only major $CHANNEL_MAJOR, not $version"
fi
for channel in "${RELEASE_CHANNELS[@]}"; do
  channel_select "$channel"
  render_archive_formula "$dir" "$version" "$root/scripts/templates/ravel.rb.tmpl" "$scratch/$CHANNEL_FILE" "$channel" ||
    die "archive/formula validation failed for CHANNEL=$channel"
done
# Any past date deprecates at once. The commit date of HEAD is one.
mkdir "$scratch/promoted"
reason="$(deprecate_versioned_formula "$scratch/ravel@2.rb" "$scratch/promoted/ravel@2.rb" \
  "$(TZ=UTC git -C "$root" log -1 --format=%cd --date=format-local:%Y-%m-%d)")" || die "$reason"
homebrew_test "$version" "$dir" "$scratch/ravel@2.rb" "$scratch/ravel.rb" "$scratch/promoted/ravel@2.rb"
for channel in "${RELEASE_CHANNELS[@]}"; do
  [[ -z "$selected" || "$channel" == "$selected" ]] || continue
  channel_select "$channel"
  mv "$scratch/$CHANNEL_FILE" "$dir/$CHANNEL_FILE"
  echo "release-formula: rendered and tested $dir/$CHANNEL_FILE (unpublished)"
done
