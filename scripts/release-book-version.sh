#!/usr/bin/env bash
#
# Guard the one line of the book that names the release version.
#
#   release-book-version.sh ready          the release can still replace the line
#   release-book-version.sh released TAG   the commit at TAG ships the right line
#
# release.toml writes the version into the line when cargo-release cuts the
# release. The pattern there and the pattern here must agree; the fixture in
# release-book-version.test.sh runs the real cargo-release, so drift fails it.

set -euo pipefail
IFS=$'\n\t'

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
readonly REPO_ROOT
readonly BOOK_PAGE='docs/book/src/03-a-complete-session.md'
readonly PLACEHOLDER='@RAVEL_BOOK_VERSION@'
# A whole line: the placeholder before the first release, a version after it.
readonly VERSION_LINE='^ravel (@RAVEL_BOOK_VERSION@|[0-9]+\.[0-9]+\.[0-9]+)$'

die() {
  echo "release-book-version: $*" >&2
  exit 1
}

# Require exactly one version line in the page text on stdin; print it.
the_version_line() {
  local page lines count
  page="$(cat)"
  lines="$(grep -E "$VERSION_LINE" <<<"$page" || true)"
  count="$(grep -c . <<<"$lines" || true)"
  [[ "$count" == 1 ]] ||
    die "$BOOK_PAGE has $count lines of the form 'ravel <version or $PLACEHOLDER>', but the release replaces exactly one"
  printf '%s\n' "$lines"
}

main() {
  case "${1:-}" in
  ready)
    [[ $# == 1 ]] || die 'usage: release-book-version.sh ready | released TAG'
    [[ -f "$REPO_ROOT/$BOOK_PAGE" ]] || die "$BOOK_PAGE is missing"
    the_version_line <"$REPO_ROOT/$BOOK_PAGE" >/dev/null
    echo "release-book-version: $BOOK_PAGE has the one line that the release replaces"
    ;;
  released)
    [[ $# == 2 && "$2" =~ ^v[0-9]+\.[0-9]+\.[0-9]+$ ]] || die 'usage: release-book-version.sh released vX.Y.Z'
    local tag="$2" page line
    # What ships is the commit at the tag, not the working copy.
    page="$(git -C "$REPO_ROOT" show "$tag:$BOOK_PAGE")" || die "cannot read $BOOK_PAGE at $tag"
    line="$(the_version_line <<<"$page")"
    [[ "$line" == "ravel ${tag#v}" ]] ||
      die "$BOOK_PAGE at $tag says '$line', but the release is 'ravel ${tag#v}'"
    if grep -Fq "$PLACEHOLDER" <<<"$page"; then
      die "$BOOK_PAGE at $tag still has the placeholder $PLACEHOLDER"
    fi
    echo "release-book-version: $BOOK_PAGE at $tag says '$line'"
    ;;
  *) die 'usage: release-book-version.sh ready | released TAG' ;;
  esac
}

main "$@"
