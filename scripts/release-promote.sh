#!/usr/bin/env bash
#
# Publish an already complete ravel@2 release as unversioned ravel.
#
# Usage: release-promote.sh --version X.Y.Z [--readiness FILE] [--evidence FILE]
#
# docs/specs/release-channels.md, "Promotion without another release", owns
# the contract. This script cuts no version, builds no archive, creates no tag
# or GitHub Release and changes no book. Before any tap write it verifies,
# against the cut's release evidence (--evidence, by default
# target/release-evidence/vX.Y.Z.json):
#   - the public tag's commit and the complete GitHub Release;
#   - the SHA-256 of each downloaded asset;
#   - the Pages commit that last changed the release's book;
#   - the consumer readiness receipt (--readiness, by default
#     target/release-evidence/vX.Y.Z.readiness.json), its binding to this tag
#     and these digests, and the consumer formula's version in the tap;
#   - that the tap's ravel@2.rb is the rendering of the downloaded assets.
# It renders ravel.rb from the same assets with the template at the tag, and
# tests both classes and the deprecated ravel@2.rb through Homebrew.
#
# One jj tap commit then writes ravel.rb and adds one deprecate! line to
# ravel@2.rb. Installation upgrades unversioned ravel with cleanup suppressed
# and keeps every earlier keg. The script writes
# target/release-evidence/vX.Y.Z.promotion.json.
#
# A retry verifies and keeps a promotion that the tap already holds, pushes a
# local promotion commit that has not reached the remote and keeps a matching
# installation. Any other tap state is reported and left unchanged.
#
# Env: RAVEL_TAP_DIR (default ~/Development/homebrew-taps), and the book's
# RAVEL_BOOK_REMOTE (default origin) and RAVEL_BOOK_BRANCH (default gh-pages).
set -euo pipefail
IFS=$'\n\t'

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
readonly REPO_ROOT
# shellcheck source=scripts/release-common.sh
source "$REPO_ROOT/scripts/release-common.sh"
readonly OUT_DIR="$REPO_ROOT/target/release-evidence"
readonly TAP_DIR="${RAVEL_TAP_DIR:-$HOME/Development/homebrew-taps}"
readonly BOOK_REMOTE="${RAVEL_BOOK_REMOTE:-origin}"
readonly BOOK_BRANCH="${RAVEL_BOOK_BRANCH:-gh-pages}"
readonly RECEIPT_SPEC='docs/specs/release-channels.md, "Consumer readiness receipt"'
readonly RECOVERY='docs/book/src/11-deliver-ravel.md, "Recover a partial release without another bump"'

die() {
  echo "release-promote: $*" >&2
  exit 1
}

step() {
  echo "release-promote: $*"
}

usage() {
  die 'usage: scripts/release-promote.sh --version X.Y.Z [--readiness FILE] [--evidence FILE]'
}

# json_problems FILE FILTER [JQ ARGS...]: print each problem that FILTER, a
# jq expression that yields strings, finds in FILE. Refuse a file that is not
# exactly one JSON object: jq runs FILTER once per document, so an empty file
# would otherwise yield no problem.
json_problems() {
  local file="$1" filter="$2"
  shift 2
  if ! jq -es 'length == 1 and (.[0] | type == "object")' "$file" >/dev/null 2>&1; then
    echo "$file is not one JSON object"
    return 0
  fi
  # A missing or mistyped field yields false here, never an empty result.
  jq -r "$@" 'def matches($re): [strings | test($re)] | any; '"$filter" "$file" 2>/dev/null || echo "$file is not JSON"
}

# The cut's evidence, written by release-evidence.sh --channel versioned-2.
verify_evidence() {
  local file="$1" version="$2" problems expected
  [[ -f "$file" ]] ||
    die "no release evidence at $file. Promotion compares public state with the cut's evidence: pass EVIDENCE=FILE with the file that scripts/release-evidence.sh --channel versioned-2 wrote for v$version"
  expected="$(for target in "${TARGETS[@]}"; do printf 'ravel-v%s-%s.tar.xz\n' "$version" "$target"; done | jq -Rsc 'split("\n")[:-1] | sort')"
  # shellcheck disable=SC2016 # jq expands these variables
  problems="$(json_problems "$file" '
    [ (if .schemaVersion != 1 then "schemaVersion is not 1" else empty end),
      (if .channel != "versioned-2" then "channel is not versioned-2" else empty end),
      (if .tag != $tag then "tag is not \($tag)" else empty end),
      (if (.tagCommit | matches("^[0-9a-f]{40}$")) | not then "tagCommit is not a commit" else empty end),
      (if ([.assets[]?.name] | sort) != $names then "assets do not name the three archives of \($tag)" else empty end),
      (if [.assets[]? | .sha256 | matches("^[0-9a-f]{64}$")] | all | not then "an asset has no SHA-256" else empty end),
      (if (.book.commit | matches("^[0-9a-f]{40}$")) | not then "book.commit is not a commit" else empty end)
    ] | .[]' --arg tag "v$version" --argjson names "$expected")"
  [[ -z "$problems" ]] ||
    die "$file is not the versioned-2 release evidence of v$version: $(tr '\n' ';' <<<"$problems") Pass EVIDENCE=FILE with the cut's evidence"
}

# The consumer readiness receipt. The coordinating release owner writes it
# after the compatible consumer release and its installed verification.
verify_readiness() {
  local file="$1" evidence="$2" version="$3" problems
  [[ -f "$file" ]] ||
    die "no consumer readiness receipt at $file. Promotion waits for the coordinating release owner's receipt, written after the compatible consumer release and its installed verification. $RECEIPT_SPEC gives its shape; pass READINESS=FILE"
  # shellcheck disable=SC2016 # jq expands these variables
  problems="$(json_problems "$file" '
    def assets: [.[]? | {name, sha256}] | sort_by(.name);
    [ (if .schemaVersion != 1 then "schemaVersion is not 1" else empty end),
      (if .kind != "ravel-promotion-readiness" then "kind is not ravel-promotion-readiness" else empty end),
      (if .ravel.version != $version then "ravel.version is not \($version)" else empty end),
      (if .ravel.tagCommit != $evidence[0].tagCommit then "ravel.tagCommit is not the cut tag commit \($evidence[0].tagCommit)" else empty end),
      (if (.ravel.assets | assets) != ($evidence[0].assets | assets) then "ravel.assets are not the cut asset digests" else empty end),
      (if (.consumer.name | matches("^[a-z0-9][a-z0-9-]*$")) | not then "consumer.name is missing" else empty end),
      (if (.consumer.version | matches("^[0-9]+\\.[0-9]+\\.[0-9]+$")) | not then "consumer.version is not X.Y.Z" else empty end),
      (if (.consumer.formula | matches("^[a-z0-9][a-z0-9@._+-]*\\.rb$")) | not then "consumer.formula is not a formula file name" else empty end),
      (if (.consumer.verifiedAt | matches("^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9:.]+Z$")) | not then "consumer.verifiedAt is not a UTC time" else empty end),
      (if (.consumer.verifiedRoutes | type == "array" and length > 0 and all(.[]; type == "string" and length > 0)) | not
         then "consumer.verifiedRoutes is not a list of route names" else empty end)
    ] | .[]' --arg version "$version" --slurpfile evidence "$evidence")"
  [[ -z "$problems" ]] ||
    die "the readiness receipt $file does not admit promotion of v$version: $(tr '\n' ';' <<<"$problems") $RECEIPT_SPEC gives its shape"
}

# The public tag and Release are the cut's.
verify_public_release() {
  local version="$1" commit="$2" remote
  remote="$(git -C "$REPO_ROOT" ls-remote origin "refs/tags/v$version^{}" | cut -f1)"
  [[ -n "$remote" ]] || remote="$(git -C "$REPO_ROOT" ls-remote origin "refs/tags/v$version" | cut -f1)"
  [[ "$remote" == "$commit" ]] ||
    die "the public tag v$version is ${remote:-absent}, not the cut's $commit. A public tag is immutable; reconcile the release before promotion ($RECOVERY)"
  published_release_complete "$version" ||
    die "the GitHub Release v$version is missing, a draft or incomplete; resume the release ($RECOVERY), never cut again"
}

# Download the public assets into DIR and compare each digest with the cut's.
verify_assets() {
  local version="$1" evidence="$2" dir="$3" target name actual expected
  gh release download "v$version" --repo Linkuistics/Ravel --dir "$dir" --pattern '*.tar.xz' ||
    die "cannot download the assets of v$version"
  for target in "${TARGETS[@]}"; do
    name="ravel-v$version-$target.tar.xz"
    [[ -f "$dir/$name" ]] || die "the public release has no $name"
    actual="$(shasum -a 256 "$dir/$name" | cut -d' ' -f1)"
    expected="$(jq -r --arg name "$name" '.assets[] | select(.name == $name) | .sha256' "$evidence")"
    [[ "$actual" == "$expected" ]] ||
      die "the public $name has SHA-256 $actual, but the cut recorded $expected. The published release changed after the cut; reconcile it, because promotion never uploads or rebuilds"
  done
}

# The Pages book of the release is the one the cut published.
verify_book() {
  local version="$1" commit="$2" actual
  git -C "$REPO_ROOT" fetch -q "$BOOK_REMOTE" "refs/heads/$BOOK_BRANCH" || die "cannot fetch $BOOK_REMOTE $BOOK_BRANCH"
  actual="$(git -C "$REPO_ROOT" log -1 --format=%H FETCH_HEAD -- "v$version/")"
  [[ "$actual" == "$commit" ]] ||
    die "the book of v$version on $BOOK_BRANCH was last changed by ${actual:-no commit}, not by the cut's $commit. A published book is final; reconcile it before promotion ($RECOVERY)"
  git -C "$REPO_ROOT" show FETCH_HEAD:index.html 2>/dev/null | grep -Fq "href=\"v$version/\"" ||
    die "the root index.html of $BOOK_BRANCH does not list v$version; resume the book ($RECOVERY)"
}

# tap_head REVISION: the commit of a tap revision, or nothing.
tap_head() {
  jj -R "$TAP_DIR" log --no-graph -r "$1" -T commit_id 2>/dev/null
}

# tap_changed REVISION: the tap files that REVISION changes, root-relative and
# sorted on one line. jj prints paths relative to its working directory.
tap_changed() {
  (cd "$TAP_DIR" && jj diff -r "$1" --name-only) | LC_ALL=C sort | tr '\n' ' '
}

# tap_holds REVISION DIR: REVISION's ravel.rb and ravel@2.rb are DIR's.
tap_holds() {
  local revision="$1" dir="$2" file
  for file in ravel.rb ravel@2.rb; do
    cmp -s <(jj -R "$TAP_DIR" file show -r "$revision" "root:Formula/$file" 2>/dev/null) "$dir/$file" || return 1
  done
}

# Commit the promotion in the tap as one change, or report why not.
commit_promotion() {
  local version="$1" dir="$2" changed
  jj -R "$TAP_DIR" new main >/dev/null
  cp "$dir/ravel.rb" "$dir/ravel@2.rb" "$TAP_DIR/Formula/"
  changed="$(tap_changed @)"
  if [[ "$changed" != 'Formula/ravel.rb Formula/ravel@2.rb ' ]]; then
    jj -R "$TAP_DIR" abandon @ >/dev/null
    die "the promotion would change '$changed' rather than ravel.rb and ravel@2.rb; the tap is left unchanged"
  fi
  jj -R "$TAP_DIR" describe -m "ravel v$version (promote ravel@2 v$version)" >/dev/null
  jj -R "$TAP_DIR" bookmark set main -r @ >/dev/null
  jj -R "$TAP_DIR" new main >/dev/null
}

push_promotion() {
  local commit
  commit="$(tap_head main)"
  tap_push "$TAP_DIR" ||
    die "the push of the tap's promotion commit $commit failed. It stays local; run release:promote again to verify and push it"
}

# Bring the tap to the promotion: keep it, push it or make it. DIR holds the
# rendered ravel.rb and deprecated ravel@2.rb; VERSIONED the published one.
publish_promotion() {
  local version="$1" dir="$2" versioned="$3" main origin ahead unversioned_major
  [[ -d "$TAP_DIR/.jj" ]] ||
    die "promotion pushes its tap commit through jj, and $TAP_DIR is not a jj repository (set RAVEL_TAP_DIR, or run 'jj git init --colocate' there)"
  [[ -z "$(tap_changed @)" ]] ||
    die "the tap's working copy holds changes; preserve them before promotion"
  main="$(tap_head main)" || true
  origin="$(tap_head main@origin)" || true
  [[ -n "$main" && -n "$origin" ]] || die "the tap has no single main and main@origin; reconcile its bookmarks before promotion"

  if tap_holds main@origin "$dir"; then
    [[ "$main" == "$origin" ]] || die "the tap's remote holds the promotion, but local main differs; reconcile the tap"
    step "the tap already holds the promotion; kept"
    return
  fi
  if cmp -s <(jj -R "$TAP_DIR" file show -r main@origin root:Formula/ravel@2.rb 2>/dev/null) "$dir/ravel@2.rb"; then
    die "the tap's Formula/ravel@2.rb holds the promotion, but its Formula/ravel.rb is not this promotion's rendering; both are left unchanged. Reconcile the tap"
  fi
  # Anything but the published ravel@2 is not this release's state B.
  cmp -s <(jj -R "$TAP_DIR" file show -r main@origin root:Formula/ravel@2.rb 2>/dev/null) "$versioned" ||
    die "the tap's Formula/ravel@2.rb is neither the published formula of these assets nor its promotion; it is left unchanged. Reconcile it, or finish the versioned publication first ($RECOVERY)"
  unversioned_major="$(version_major "$(tap_unversioned_version "$TAP_DIR")")"
  if [[ -n "$unversioned_major" ]] && ((unversioned_major >= 2)); then
    die "the tap's Formula/ravel.rb already has major $unversioned_major and is not this promotion; it is left unchanged"
  fi

  if [[ "$main" != "$origin" ]]; then
    ahead="$(jj -R "$TAP_DIR" log --no-graph -r 'main@origin..main' -T 'commit_id ++ "\n"' | grep -c . || true)"
    [[ "$ahead" == 1 && "$(jj -R "$TAP_DIR" log --no-graph -r 'main-' -T commit_id)" == "$origin" ]] && tap_holds main "$dir" &&
      [[ "$(tap_changed main)" == 'Formula/ravel.rb Formula/ravel@2.rb ' ]] ||
      die "tap main is ahead of its remote with work that is not this promotion; integrate or push it before promotion"
    step "pushing the tap's local promotion commit $main"
  else
    commit_promotion "$version" "$dir"
    step "committed the promotion as $(tap_head main)"
  fi
  push_promotion
  jj -R "$TAP_DIR" git fetch --remote origin >/dev/null 2>&1 || die "cannot fetch the tap after the push"
  tap_holds main@origin "$dir" || die "the tap's remote does not hold the promotion after the push"
}

main() {
  local version='' readiness='' evidence=''
  while (($#)); do
    case "$1" in
      --version | --readiness | --evidence)
        (($# >= 2)) || usage
        case "$1" in
          --version) version="$2" ;;
          --readiness) readiness="$2" ;;
          --evidence) evidence="$2" ;;
        esac
        shift 2
        ;;
      *) usage ;;
    esac
  done
  [[ "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]] || usage
  channel_select versioned-2
  [[ "$(version_major "$version")" == "$CHANNEL_MAJOR" ]] ||
    die "promotion moves the major-$CHANNEL_MAJOR release of ravel@2 to unversioned ravel, and v$version has major $(version_major "$version")"
  local tag="v$version"
  evidence="${evidence:-$OUT_DIR/$tag.json}"
  readiness="${readiness:-$OUT_DIR/$tag.readiness.json}"
  local tool
  for tool in gh jq jj brew shasum; do
    command -v "$tool" >/dev/null || die "$tool is not on PATH"
  done

  # Local inputs first, so a missing receipt refuses before any network read.
  verify_evidence "$evidence" "$version"
  verify_readiness "$readiness" "$evidence" "$version"
  local tag_commit book_commit consumer_formula consumer_version
  tag_commit="$(jq -r .tagCommit "$evidence")"
  book_commit="$(jq -r .book.commit "$evidence")"
  consumer_formula="$(jq -r .consumer.formula "$readiness")"
  consumer_version="$(jq -r .consumer.version "$readiness")"

  local scratch
  scratch="$(mktemp -d)"
  # shellcheck disable=SC2064 # remove precisely this private directory
  trap "chmod -R u+w '$scratch' 2>/dev/null; rm -rf '$scratch'" EXIT
  mkdir "$scratch/assets" "$scratch/versioned" "$scratch/promoted"

  verify_public_release "$version" "$tag_commit"
  verify_assets "$version" "$evidence" "$scratch/assets"
  verify_book "$version" "$book_commit"
  step "the public tag, the three asset digests and the book of $tag are the cut's"

  [[ -d "$TAP_DIR" ]] || die "tap clone not found at $TAP_DIR (set RAVEL_TAP_DIR)"
  tap_fetch "$TAP_DIR" >/dev/null 2>&1 || die "cannot fetch the tap at $TAP_DIR"
  local consumer_actual
  consumer_actual="$({ tap_formula_text "$TAP_DIR" "$consumer_formula" || true; } | formula_version)"
  [[ "$consumer_actual" == "$consumer_version" ]] ||
    die "the tap's Formula/$consumer_formula is at ${consumer_actual:-no version}, not the receipt's $consumer_version. Publish and verify the compatible consumer release before promotion"

  # Both classes from the downloaded assets and the template at the tag.
  local date reason
  git -C "$REPO_ROOT" show "$tag_commit:scripts/templates/ravel.rb.tmpl" >"$scratch/template" ||
    die "cannot read the formula template at $tag_commit"
  render_archive_formula "$scratch/assets" "$version" "$scratch/template" "$scratch/versioned/ravel@2.rb" versioned-2 ||
    die "cannot render ravel@2.rb from the public assets"
  render_archive_formula "$scratch/assets" "$version" "$scratch/template" "$scratch/promoted/ravel.rb" unversioned ||
    die "cannot render ravel.rb from the public assets"
  # The tag's date at UTC-12 is never in the future in Homebrew's local time
  # zone, and it keeps a retry's rendering byte for byte the same.
  date="$(promotion_deprecation_date "$REPO_ROOT" "$tag_commit")"
  reason="$(deprecate_versioned_formula "$scratch/versioned/ravel@2.rb" "$scratch/promoted/ravel@2.rb" "$date")" || die "$reason"
  bash "$REPO_ROOT/scripts/release-formula.sh" --test "$version" "$scratch/assets" \
    "$scratch/versioned/ravel@2.rb" "$scratch/promoted/ravel.rb" "$scratch/promoted/ravel@2.rb" ||
    die "the promoted formulae failed their Homebrew test; the tap is unchanged"

  publish_promotion "$version" "$scratch/promoted" "$scratch/versioned/ravel@2.rb"

  bash "$REPO_ROOT/scripts/release-install.sh" --channel unversioned --version "$version" ||
    die "the installation of unversioned ravel $version failed. The tap holds the promotion; repair the installation and run release:promote again, which keeps the published formulae"

  # The evidence of the promotion.
  local brew_prefix keg tap_commit kegs record evidence_json
  brew_prefix="$(brew --prefix)"
  keg="$(brew --cellar ravel)/$version"
  tap_commit="$(git -C "$TAP_DIR" log -1 --format=%H "$(tap_remote_commit "$TAP_DIR")" -- Formula/ravel.rb)"
  kegs="$(for k in "$brew_prefix"/Cellar/ravel/* "$brew_prefix"/Cellar/ravel@2/*; do [[ -d "$k" && "$k" != "$keg" ]] && printf '%s\n' "$k"; done | jq -Rsc 'split("\n")[:-1]')"
  record="$(cat "$keg/share/ravel/installation.json")"
  evidence_json="$(jq -n \
    --arg version "$version" --arg tag "$tag" --arg tagCommit "$tag_commit" \
    --slurpfile evidence "$evidence" --slurpfile readiness "$readiness" \
    --arg readinessFile "$readiness" --arg readinessSha256 "$(shasum -a 256 "$readiness" | cut -d' ' -f1)" \
    --arg tapCommit "$tap_commit" --arg date "$date" \
    --arg unversionedSha256 "$(shasum -a 256 "$scratch/promoted/ravel.rb" | cut -d' ' -f1)" \
    --arg versionedSha256 "$(shasum -a 256 "$scratch/promoted/ravel@2.rb" | cut -d' ' -f1)" \
    --arg keg "$keg" --arg prefix "$(brew --prefix ravel)" --arg installed "$("$keg/bin/ravel" --version)" \
    --argjson record "$record" --argjson retained "$kegs" \
    '{
      schemaVersion: 1,
      promotion: {from: "ravel@2", to: "ravel"},
      version: $version, tag: $tag, tagCommit: $tagCommit,
      assets: $evidence[0].assets,
      book: $evidence[0].book,
      readiness: {file: $readinessFile, sha256: $readinessSha256, consumer: $readiness[0].consumer},
      tap: {commit: $tapCommit, deprecationDate: $date, formulae: [
        {file: "Formula/ravel.rb", sha256: $unversionedSha256},
        {file: "Formula/ravel@2.rb", sha256: $versionedSha256}]},
      installed: {keg: $keg, prefix: $prefix, version: $installed, record: $record, retainedKegs: $retained}
    }')"
  mkdir -p "$OUT_DIR"
  printf '%s\n' "$evidence_json" >"$OUT_DIR/$tag.promotion.json.partial"
  mv "$OUT_DIR/$tag.promotion.json.partial" "$OUT_DIR/$tag.promotion.json"
  printf '%s\n' "$evidence_json"
  step "promoted $tag to unversioned ravel; wrote $OUT_DIR/$tag.promotion.json" >&2
}

main "$@"
