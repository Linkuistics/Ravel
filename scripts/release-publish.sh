#!/usr/bin/env bash
#
# Publish the artifacts produced by release-build.sh:
#   1. Create a GitHub Release on Linkuistics/Ravel for v<ver> and
#      upload all tarballs from target/dist/.
#   2. Add the staged book to the Pages branch (release-book.sh publish).
#   3. Copy ravel.rb into $RAVEL_TAP_DIR/Formula/, commit, push.
#
# The book comes before the tap because the tap is what lets a reader install
# the release, and the reader then looks for that release's book.
#
# Prerequisite: ./scripts/release-build.sh and `release-book.sh build` have
# just run successfully, and the smoke tests have passed.
# Resume modes: --book-only (after step 1), --tap-only (after step 2).
# Env: RAVEL_TAP_DIR (default ~/Development/homebrew-taps).
#
set -euo pipefail
IFS=$'\n\t'

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
readonly REPO_ROOT
readonly DIST_DIR="$REPO_ROOT/target/dist"
# shellcheck source=scripts/release-common.sh
source "$REPO_ROOT/scripts/release-common.sh"
readonly TAP_DIR="${RAVEL_TAP_DIR:-$HOME/Development/homebrew-taps}"

die() {
  echo "release-publish: $*" >&2
  exit 1
}

check_tap() {
  [[ -d "$TAP_DIR/.git" ]] || die "tap clone not found at $TAP_DIR (set RAVEL_TAP_DIR)"
  local tap_status
  tap_status="$(git -C "$TAP_DIR" status --porcelain)"
  [[ -z "$tap_status" ]] ||
    die "tap working tree is dirty; preserve its changes before publishing"
  if [[ -d "$TAP_DIR/.jj" ]]; then
    command -v jj >/dev/null || die "jj tap requires jj on PATH"
    jj -R "$TAP_DIR" git fetch
    [[ -n "$(jj -R "$TAP_DIR" log --no-graph -r 'main & main@origin' -T commit_id)" ]] ||
      die "tap main differs from origin; integrate or push its work before publishing"
  else
    git -C "$TAP_DIR" symbolic-ref --quiet HEAD >/dev/null ||
      die "tap is on detached HEAD; switch to the branch intended for publication"
    local upstream tap_head upstream_head
    upstream="$(git -C "$TAP_DIR" rev-parse --abbrev-ref --symbolic-full-name '@{upstream}')" ||
      die "tap branch has no upstream; configure the intended publishing branch"
    git -C "$TAP_DIR" fetch
    tap_head="$(git -C "$TAP_DIR" rev-parse HEAD)"
    upstream_head="$(git -C "$TAP_DIR" rev-parse "$upstream")"
    [[ "$tap_head" == "$upstream_head" ]] ||
      die "tap branch differs from $upstream; integrate or push its work before publishing"
  fi
}

preflight() {
  command -v gh >/dev/null || die "gh CLI not on PATH"
  gh auth status >/dev/null 2>&1 || die "gh not authenticated; run 'gh auth login'"
  [[ -d "$DIST_DIR" ]] || die "no $DIST_DIR; run scripts/release-build.sh first"
  [[ -f "$DIST_DIR/ravel.rb" ]] || die "no rendered formula at $DIST_DIR/ravel.rb"
  compgen -G "$DIST_DIR/*.tar.xz" >/dev/null || die "no tarballs in $DIST_DIR"
  check_tap
}

read_version() {
  git -C "$REPO_ROOT" describe --tags --exact-match HEAD | sed 's/^v//'
}

verify_tag_matches_artifacts() {
  local version="$1"
  [[ "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ && "$version" != 0.0.0 ]] || die "not a release version: $version"
  [[ -z "$(git -C "$REPO_ROOT" status --porcelain)" ]] || die 'source tree is dirty; return to the exact release revision'
  local scratch count
  count="$(find "$DIST_DIR" -maxdepth 1 -name '*.tar.xz' | wc -l | tr -d ' ')"
  [[ "$count" == 3 ]] || die 'expected exactly three archives; rebuild and smoke before publishing'
  scratch="$(mktemp -d)"
  if ! render_archive_formula "$DIST_DIR" "$version" "$REPO_ROOT/scripts/templates/ravel.rb.tmpl" "$scratch/ravel.rb" ||
    ! cmp -s "$scratch/ravel.rb" "$DIST_DIR/ravel.rb"; then
    rm -rf "$scratch"
    die 'current-tag archives or formula mismatch; rebuild and smoke before publishing'
  fi
  rm -rf "$scratch"
}

create_github_release() {
  local version="$1"
  local tag="v${version}"
  local notes="$DIST_DIR/release-notes.md"
  awk -v heading="## $tag" '
    $0 == heading { section=1; next }
    section && /^## / { exit }
    section { print }
  ' "$REPO_ROOT/CHANGELOG.md" >"$notes"
  grep -q '[^[:space:]]' "$notes" || die "no changelog notes found for $tag"
  echo "release-publish: creating GitHub Release $tag"
  gh release create "$tag" \
    --verify-tag \
    --repo Linkuistics/Ravel \
    --title "Release $tag" \
    --notes-file "$notes" \
    "$DIST_DIR/ravel-v$version-aarch64-apple-darwin.tar.xz" \
    "$DIST_DIR/ravel-v$version-aarch64-unknown-linux-gnu.tar.xz" \
    "$DIST_DIR/ravel-v$version-x86_64-unknown-linux-gnu.tar.xz"
}

push_formula_to_tap() {
  local version="$1"
  echo "release-publish: pushing formula to $TAP_DIR"
  if [[ -d "$TAP_DIR/.jj" ]]; then
    jj -R "$TAP_DIR" new main
  fi
  mkdir -p "$TAP_DIR/Formula"
  cp "$DIST_DIR/ravel.rb" "$TAP_DIR/Formula/ravel.rb"
  if [[ -d "$TAP_DIR/.jj" ]]; then
    jj -R "$TAP_DIR" describe -m "ravel v${version}"
    jj -R "$TAP_DIR" bookmark set main -r @
    jj -R "$TAP_DIR" new main
    jj -R "$TAP_DIR" git push -b main
  else
    git -C "$TAP_DIR" add Formula/ravel.rb
    git -C "$TAP_DIR" commit -m "ravel v${version}"
    git -C "$TAP_DIR" push
  fi
}

verify_published_archives() {
  local version="$1" scratch target name
  published_release_complete "$version" || die 'GitHub release is incomplete; resume uploads before tap publication'
  scratch="$(mktemp -d)"
  if ! gh release download "v$version" --repo Linkuistics/Ravel --dir "$scratch" --pattern '*.tar.xz'; then
    rm -rf "$scratch"
    die 'could not verify uploaded archives; tap left untouched'
  fi
  for target in "${TARGETS[@]}"; do
    name="ravel-v$version-$target.tar.xz"
    if ! cmp -s "$scratch/$name" "$DIST_DIR/$name"; then
      rm -rf "$scratch"
      die "published $name differs from validated local archive; tap left untouched"
    fi
  done
  rm -rf "$scratch"
}

# The book for this release is on the Pages branch, whole.
publish_book() {
  "$REPO_ROOT/scripts/release-book.sh" publish "v$1"
}

main() {
  case "${1:-}" in
    --check-tap)
      check_tap
      return
      ;;
    --book-only)
      command -v gh >/dev/null || die "gh CLI not on PATH"
      publish_book "$(read_version)"
      return
      ;;
    '' | --tap-only) ;;
    *) die "usage: scripts/release-publish.sh [--check-tap | --book-only | --tap-only]" ;;
  esac
  preflight
  local version
  version="$(read_version)"
  verify_tag_matches_artifacts "$version"

  if [[ "${1:-}" == --tap-only ]]; then
    verify_published_archives "$version"
    # The tap never gets ahead of the book, on a resume either.
    "$REPO_ROOT/scripts/release-book.sh" verify "v$version"
  else
    create_github_release "$version"
    publish_book "$version"
  fi
  push_formula_to_tap "$version"

  echo
  echo "release-publish: done. Verify with:"
  echo "  brew update && brew install linkuistics/taps/ravel"
}

main "$@"
