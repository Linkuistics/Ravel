#!/usr/bin/env bash
# Render from exact archives and exercise the template through Homebrew's API.
# This does not install a keg or edit a tap. The caller must smoke all targets
# before using a rendered formula for publication.
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
[[ $# == 1 ]] || die 'usage: release-formula.sh ARCHIVES_DIR'
dir="$(cd "$1" && pwd -P)"
id="$(cargo pkgid --quiet -p ravel)"
version="${id##*[#@]}"
[[ "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]] || die "expected a stable version, got $version"
scratch="$(mktemp -d)"
trap 'rm -rf "$scratch"' EXIT
formula="$scratch/ravel.rb"
render_archive_formula "$dir" "$version" "$root/scripts/templates/ravel.rb.tmpl" "$formula" || die 'archive/formula validation failed'
[[ "$(uname -sm)" == 'Darwin arm64' ]] || die 'formula validation requires the macOS arm64 archive host'
archive="$dir/ravel-v$version-aarch64-apple-darwin.tar.xz"
extract_exactly "$archive" "$scratch"
mkdir "$scratch/prefix café" "$scratch/test"
# The installed Homebrew library executes the actual install/test methods.
# Prefix and testpath are redirected to private directories, no mock DSL.
# https://docs.brew.sh/rubydoc/Formula.html#prefix-instance_method
HOMEBREW_NO_AUTO_UPDATE=1 brew ruby "$root/scripts/release-formula.test.rb" \
  "$formula" "$scratch/ravel-v$version-aarch64-apple-darwin" \
  "$scratch/prefix café" "$scratch/test" "$root/target/probes/ravel"
mv "$formula" "$dir/ravel.rb"
echo "release-formula: rendered and tested $dir/ravel.rb (unpublished)"
