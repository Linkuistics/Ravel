#!/usr/bin/env bash
#
# Record the evidence of a published release of the selected channel.
#
# Usage: release-evidence.sh --channel CHANNEL
#
# Run at the tagged release commit after release-install.sh, and before the
# book's `live` check. It reads public state only and changes nothing outside
# target/release-evidence. docs/specs/release-channels.md names the fields:
# the channel, the tag's commit, the asset digests, the book commit, the tap
# commit and the installed result. It writes
# target/release-evidence/vX.Y.Z.json and prints the same JSON.
#
# Env: RAVEL_TAP_DIR (default ~/Development/homebrew-taps), and the book's
# RAVEL_BOOK_REMOTE (default origin) and RAVEL_BOOK_BRANCH (default gh-pages).
set -euo pipefail
IFS=$'\n\t'

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
readonly REPO_ROOT
# shellcheck source=scripts/release-common.sh
source "$REPO_ROOT/scripts/release-common.sh"
readonly DIST_DIR="$REPO_ROOT/target/dist"
readonly OUT_DIR="$REPO_ROOT/target/release-evidence"
readonly TAP_DIR="${RAVEL_TAP_DIR:-$HOME/Development/homebrew-taps}"
readonly BOOK_REMOTE="${RAVEL_BOOK_REMOTE:-origin}"
readonly BOOK_BRANCH="${RAVEL_BOOK_BRANCH:-gh-pages}"

die() {
  echo "release-evidence: $*" >&2
  exit 1
}

main() {
  [[ $# == 2 && "$1" == --channel ]] || die 'usage: release-evidence.sh --channel CHANNEL'
  local reason
  reason="$(channel_select "$2")" || die "$reason"
  channel_select "$2"

  local tag version tag_commit remote_tag
  tag="$(git -C "$REPO_ROOT" describe --tags --exact-match HEAD)" || die 'HEAD is not a release tag; run at the tagged release commit'
  version="${tag#v}"
  [[ "$tag" == "v$version" && "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]] || die "not a release tag: $tag"
  tag_commit="$(git -C "$REPO_ROOT" rev-parse "$tag^{commit}")"
  # An annotated tag lists its commit under ^{}; a lightweight tag only once.
  remote_tag="$(git -C "$REPO_ROOT" ls-remote origin "refs/tags/$tag^{}" | cut -f1)"
  [[ -n "$remote_tag" ]] || remote_tag="$(git -C "$REPO_ROOT" ls-remote origin "refs/tags/$tag" | cut -f1)"
  [[ "$remote_tag" == "$tag_commit" ]] || die "the public $tag is ${remote_tag:-absent}, not $tag_commit"

  # The public assets, byte for byte the archives that the smoke test checked.
  local scratch target name assets='[]' digest
  published_release_complete "$version" || die "the GitHub Release $tag is missing, a draft or incomplete"
  scratch="$(mktemp -d)"
  # shellcheck disable=SC2064 # remove precisely this private directory
  trap "rm -rf '$scratch'" EXIT
  mkdir "$scratch/assets"
  gh release download "$tag" --repo Linkuistics/Ravel --dir "$scratch/assets" --pattern '*.tar.xz' ||
    die "cannot download the assets of $tag"
  for target in "${TARGETS[@]}"; do
    name="ravel-v$version-$target.tar.xz"
    cmp -s "$scratch/assets/$name" "$DIST_DIR/$name" || die "the public $name differs from $DIST_DIR/$name"
    digest="$(shasum -a 256 "$scratch/assets/$name" | cut -d' ' -f1)"
    assets="$(jq -c --arg name "$name" --arg sha256 "$digest" '. + [{name: $name, sha256: $sha256}]' <<<"$assets")"
  done

  # The Pages commit that last changed this release's book directory.
  local book_commit
  git -C "$REPO_ROOT" fetch -q "$BOOK_REMOTE" "refs/heads/$BOOK_BRANCH" || die "cannot fetch $BOOK_REMOTE $BOOK_BRANCH"
  book_commit="$(git -C "$REPO_ROOT" log -1 --format=%H FETCH_HEAD -- "$tag/")"
  [[ -n "$book_commit" ]] || die "$BOOK_BRANCH holds no book for $tag"

  # The tap commit that last changed the selected formula, which must carry
  # this version and these digests.
  local tap_commit tap_text
  tap_fetch "$TAP_DIR" >/dev/null 2>&1 || die "cannot fetch the tap at $TAP_DIR"
  tap_text="$(tap_formula_text "$TAP_DIR" "$CHANNEL_FILE")" || die "the tap has no Formula/$CHANNEL_FILE"
  [[ "$(formula_version <<<"$tap_text")" == "$version" ]] || die "the tap's Formula/$CHANNEL_FILE is not at $version"
  cmp -s <(printf '%s\n' "$tap_text") "$DIST_DIR/$CHANNEL_FILE" || die "the tap's Formula/$CHANNEL_FILE differs from $DIST_DIR/$CHANNEL_FILE"
  tap_commit="$(git -C "$TAP_DIR" log -1 --format=%H "$(tap_remote_commit "$TAP_DIR")" -- "Formula/$CHANNEL_FILE")"
  [[ -n "$tap_commit" ]] || die "no tap commit changed Formula/$CHANNEL_FILE"

  # The installed result, read from the keg that release-install.sh verified.
  local keg installed prefix record
  keg="$(brew --cellar "$CHANNEL_FORMULA")/$version"
  [[ -x "$keg/bin/ravel" ]] || die "the keg $keg has no executable bin/ravel; run scripts/release-install.sh --channel $CHANNEL_ID"
  prefix="$(brew --prefix "$CHANNEL_FORMULA")"
  installed="$("$keg/bin/ravel" --version)"
  [[ "$installed" == "ravel $version" ]] || die "$keg/bin/ravel reports $installed"
  record="$(cat "$keg/share/ravel/installation.json")" || die "the keg $keg has no installation record"

  local evidence
  evidence="$(jq -n \
    --arg channel "$CHANNEL_ID" --arg formula "$CHANNEL_FORMULA" --arg formulaFile "Formula/$CHANNEL_FILE" \
    --arg class "$CHANNEL_CLASS" --argjson kegOnly "$([[ -n "$CHANNEL_KEG_ONLY" ]] && echo true || echo false)" \
    --arg tag "$tag" --arg tagCommit "$tag_commit" --argjson assets "$assets" \
    --arg bookBranch "$BOOK_BRANCH" --arg bookCommit "$book_commit" --arg tapCommit "$tap_commit" \
    --arg keg "$keg" --arg prefix "$prefix" --arg installed "$installed" --argjson record "$record" \
    '{
      schemaVersion: 1,
      channel: $channel,
      formula: {name: $formula, file: $formulaFile, class: $class, kegOnly: $kegOnly},
      tag: $tag, tagCommit: $tagCommit,
      assets: $assets,
      book: {branch: $bookBranch, commit: $bookCommit},
      tap: {commit: $tapCommit},
      installed: {keg: $keg, prefix: $prefix, version: $installed, record: $record}
    }')"
  mkdir -p "$OUT_DIR"
  printf '%s\n' "$evidence" >"$OUT_DIR/$tag.json.partial"
  mv "$OUT_DIR/$tag.json.partial" "$OUT_DIR/$tag.json"
  printf '%s\n' "$evidence"
  echo "release-evidence: wrote $OUT_DIR/$tag.json" >&2
}

main "$@"
