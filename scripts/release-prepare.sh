#!/usr/bin/env bash
# Prepare main and its changelog without incorporating unrelated working-copy work.
set -euo pipefail
IFS=$'\n\t'

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# shellcheck source=scripts/release-common.sh
source "$repo_root/scripts/release-common.sh"

die() {
  echo "release-prepare: $*" >&2
  exit 1
}

main() {
  cd "$repo_root"
  [[ -d .jj && -d .git && "$(pwd -P)" == "$(jj workspace root --name default)" ]] ||
    die "run from the default colocated jj workspace"
  local saved unmerged conflicts undescribed version tag changes pending
  saved="$(jj log --no-graph -r @ -T change_id)"
  jj status
  jj git fetch --remote origin
  unmerged="$(jj log --no-graph -r 'main@origin ~ ancestors(main)' -T commit_id)"
  [[ -z "$unmerged" ]] || die "main diverges from origin; integrate main@origin before releasing"
  conflicts="$(jj log --no-graph -r 'conflicts() & ancestors(main)' -T commit_id)"
  [[ -z "$conflicts" ]] || die "main contains conflicts; resolve them before releasing"
  undescribed="$(jj log --no-graph -r 'main@origin..main' -T 'if(description.first_line() == "", commit_id ++ "\n")')"
  [[ -z "$undescribed" ]] || die "main has undescribed changes; describe them before releasing"

  pending="$(jj log --no-graph -r 'tags(glob:"v[0-9]*") ~ ancestors(main)' -T commit_id)"
  [[ -z "$pending" ]] || die 'version tag outside main; resume the unfinished cut before another bump'
  local working_version main_version id
  id="$(cargo pkgid --quiet -p ravel)"
  working_version="${id##*[#@]}"
  main_version="$(jj file show -r main root:Cargo.toml | sed -n 's/^version *= *"\([^" ]*\)".*/\1/p' | head -n 1)"
  [[ "$working_version" == "$main_version" ]] || die 'version change outside main; reconcile or resume the unfinished cut before another bump'

  # Snapshotting and changing parents preserves every other line of work in jj.
  echo "release-prepare: saved working change $saved (return with: jj edit $saved)"
  jj new main
  version="$(cargo metadata --locked --no-deps --format-version 1 | jq -er '.packages[] | select(.name == "ravel") | .version')"
  [[ "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]] || die "expected a stable Ravel version, got $version"
  tag="v$version"
  local local_tag remote_tag version_tags
  local_tag="$(git rev-parse --verify "refs/tags/$tag^{commit}" 2>/dev/null || true)"
  remote_tag="$(git ls-remote --tags origin "refs/tags/$tag" "refs/tags/$tag^{}")"
  version_tags="$(git ls-remote --tags origin 'refs/tags/v[0-9]*')"
  if [[ "$version" == 0.0.0 && -z "$local_tag" && -z "$version_tags" && -z "$(git tag --list 'v[0-9]*')" ]]; then
    echo 'release-prepare: first release, no v0.0.0 baseline; planned minor is 0.1.0'
  else
    [[ -n "$local_tag" && -n "$remote_tag" ]] || die "partial cut or missing baseline $tag; resume using docs/RELEASING.md, never bump again"
    [[ "$(git ls-remote --tags origin "refs/tags/$tag^{}" | cut -f1)" == "$local_tag" || "$(git ls-remote --tags origin "refs/tags/$tag" | cut -f1)" == "$local_tag" ]] ||
      die "remote $tag differs; inspect and resume the unfinished release"
    git merge-base --is-ancestor "$local_tag" HEAD || die "$tag is not an ancestor of main"
    published_release_complete "$version" || die "GitHub $tag has draft/missing/incomplete assets; resume publication before another bump"
    local tap="${RAVEL_TAP_DIR:-$HOME/Development/homebrew-taps}"
    local tap_formula
    if [[ -d "$tap/.jj" ]]; then
      jj -R "$tap" git fetch --remote origin
      tap_formula="$(jj -R "$tap" file show -r main@origin root:Formula/ravel.rb)" || die 'tap publication missing; resume before another bump'
    else
      git -C "$tap" fetch origin
      tap_formula="$(git -C "$tap" show '@{upstream}:Formula/ravel.rb')" || die 'tap publication missing; resume before another bump'
    fi
    grep -Fxq "  version \"$version\"" <<<"$tap_formula" || die "tap $tag is unfinished; resume formula publication before another bump"
    changes="$(jj log --no-graph -r "$tag..main" -T commit_id)"
    [[ -n "$changes" ]] || die "No changes since $tag; resume any unfinished release using docs/RELEASING.md"
  fi
  awk '/^## Unreleased$/ { n++ } END { exit n != 1 }' CHANGELOG.md ||
    die "CHANGELOG.md must contain exactly one Unreleased heading"
  if awk '/^## Unreleased$/ { section=1; next }
      section && /^## / { exit }
      section && /[^[:space:]]/ { found=1 }
      END { exit !found }' CHANGELOG.md; then
    echo 'release-prepare: preserving existing Unreleased notes'
    return
  fi

  # Generation is shared with `task release:notes`; only the endpoint differs.
  bash scripts/release-notes.sh --to main
  jj describe -m 'docs: prepare release notes'
  jj bookmark set main -r @
  jj new main
  echo 'release-prepare: generated and committed release notes from commit descriptions'
}

main "$@"
