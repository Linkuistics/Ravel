#!/usr/bin/env bash
#
# Build the book from a release tag and deliver it to GitHub Pages.
#
#   release-book.sh build vX.Y.Z     export the tag's book source, build it, stage it
#   release-book.sh publish vX.Y.Z   add the staged book to the Pages branch
#   release-book.sh verify vX.Y.Z    compare the Pages branch with the staged book
#   release-book.sh live vX.Y.Z      read the book from the Pages site itself
#   release-book.sh check-remote     the Pages remote answers (changes nothing)
#   release-book.sh check-pages      the repository serves the Pages branch (changes nothing)
#
# Delivery layout. Branch gh-pages holds one directory per release, vX.Y.Z/,
# and a root index.html that lists them. A release adds its directory and never
# changes one that is there, so the book for an installed release stays at
# https://linkuistics.github.io/Ravel/vX.Y.Z/ for good. GitHub Pages publishes
# a branch as it stands, with no workflow:
# https://docs.github.com/en/pages/getting-started-with-github-pages/configuring-a-publishing-source-for-your-github-pages-site
# A workflow deployment would replace the whole site with one artifact and so
# drop earlier releases.
#
# Order, which release-publish.sh and the release task keep: `build` runs after
# the cut and before any archive build; `publish` runs only after the tag and
# the GitHub Release are public; `live` runs last.

set -euo pipefail
IFS=$'\n\t'

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
readonly REPO_ROOT
# shellcheck source=scripts/release-common.sh
source "$REPO_ROOT/scripts/release-common.sh"

readonly REMOTE="${RAVEL_BOOK_REMOTE:-origin}"
readonly BRANCH="${RAVEL_BOOK_BRANCH:-gh-pages}"
readonly SITE_URL="${RAVEL_BOOK_SITE_URL:-https://linkuistics.github.io/Ravel}"
readonly STAGE="${RAVEL_BOOK_STAGE:-$REPO_ROOT/target/book-release}"
# A published Pages site may be no larger than 1 GB:
# https://docs.github.com/en/pages/getting-started-with-github-pages/github-pages-limits
readonly SITE_LIMIT_KB="${RAVEL_BOOK_SITE_LIMIT_KB:-1000000}"
readonly BOOK_SOURCE='docs/book'
readonly VERSION_PAGE='03-a-complete-session'

die() {
  echo "release-book: $*" >&2
  exit 1
}

require_tag() {
  [[ "$1" =~ ^v[0-9]+\.[0-9]+\.[0-9]+$ ]] || die "not a release tag: $1"
}

# Read the rendered page on stdin: it shows the version line as the code block
# that chapter 3 has, `ravel X.Y.Z`, and not as a comment or other text.
page_says_version() {
  tr '\n' ' ' | grep -Eq "<pre><code class=\"language-text\">ravel ${1//./\\.} *</code></pre>"
}

tree_digest() {
  (
    cd "$1"
    find . -type f -print0 | LC_ALL=C sort -z | xargs -0 shasum -a 256
  ) | shasum -a 256 | cut -d' ' -f1
}

# The identity file records which source, tool and bytes a staged book has. It
# sits beside the staged directory and is never published.
identity_value() {
  sed -n "s/^$2=//p" "$STAGE/$1.identity"
}

# Refuse a tree that Pages would serve differently from how it was built:
# symbolic links need a workflow deployment, and Jekyll skips paths that start
# with `_`, `.` or `#` unless the site root has .nojekyll (written by `publish`).
refuse_unsafe_tree() {
  local dir="$1" links
  links="$(find "$dir" -type l)"
  [[ -z "$links" ]] || die "the book has symbolic links, which a branch deployment cannot serve: $links"
}

build() {
  local tag="$1" version commit scratch exported expected page
  require_tag "$tag"
  version="${tag#v}"
  commit="$(git -C "$REPO_ROOT" rev-parse --verify --quiet "$tag^{commit}")" ||
    die "no tag $tag in this checkout"
  # The version line in the tag, before any build.
  "$REPO_ROOT/scripts/release-book-version.sh" released "$tag" >/dev/null

  # A rebuild starts from nothing, so a stale stage cannot stand in for it.
  rm -rf "${STAGE:?}/$tag" "$STAGE/$tag.identity"
  mkdir -p "$STAGE"
  scratch="$(mktemp -d "$STAGE/.build.XXXXXX")"
  # shellcheck disable=SC2064 # remove precisely this private directory
  trap "rm -rf '$scratch'" EXIT

  # The source is the commit at the tag, not the working copy.
  mkdir "$scratch/source"
  git -C "$REPO_ROOT" archive "$commit" "$BOOK_SOURCE" | tar -x -C "$scratch/source"
  expected="$(git -C "$REPO_ROOT" ls-tree -r --name-only "$commit" -- "$BOOK_SOURCE" | wc -l | tr -d ' ')"
  exported="$(find "$scratch/source/$BOOK_SOURCE" -type f | wc -l | tr -d ' ')"
  [[ "$expected" -gt 0 && "$expected" == "$exported" ]] ||
    die "exported $exported book files from $tag, but the tag holds $expected"

  bash "$REPO_ROOT/scripts/book.sh" build-tree "$scratch/source/$BOOK_SOURCE" "$scratch/out"

  page="$scratch/out/$VERSION_PAGE.html"
  [[ -f "$scratch/out/index.html" && -f "$page" ]] || die "the build lacks index.html or $VERSION_PAGE.html"
  # The rendered page, not only the Markdown, carries this release's version.
  page_says_version "$version" <"$page" ||
    die "the built $VERSION_PAGE.html does not say 'ravel $version'"
  # Other chapters name the placeholder when they teach the mechanism, so only
  # the version page is searched, as release-book-version.sh searches it.
  if grep -Fq '@RAVEL_BOOK_VERSION@' "$page"; then
    die "the built $VERSION_PAGE.html still has the placeholder @RAVEL_BOOK_VERSION@"
  fi
  refuse_unsafe_tree "$scratch/out"

  mv "$scratch/out" "$STAGE/$tag"
  {
    echo "tag=$tag"
    echo "commit=$commit"
    echo "mdbook=$(bash "$REPO_ROOT/scripts/book.sh" version)"
    echo "digest=$(tree_digest "$STAGE/$tag")"
  } >"$STAGE/$tag.identity"
  echo "release-book: staged the book for $tag (source $commit) in $STAGE/$tag"
}

# The staged book is the one built from this tag as it is now.
require_staged() {
  local tag="$1" commit now digest
  [[ -d "$STAGE/$tag" && -f "$STAGE/$tag.identity" ]] ||
    die "no staged book for $tag; run 'scripts/release-book.sh build $tag'"
  commit="$(identity_value "$tag" commit)"
  now="$(git -C "$REPO_ROOT" rev-parse --verify --quiet "$tag^{commit}")" || die "no tag $tag in this checkout"
  [[ "$commit" == "$now" ]] ||
    die "the staged book for $tag was built from $commit, but the tag is now $now; build it again"
  digest="$(tree_digest "$STAGE/$tag")"
  [[ "$digest" == "$(identity_value "$tag" digest)" ]] ||
    die "the staged book for $tag changed after its build; build it again"
  refuse_unsafe_tree "$STAGE/$tag"
}

# REMOTE is a remote name of this checkout or a URL; the scratch clones need a URL.
remote_url() {
  git -C "$REPO_ROOT" remote get-url "$REMOTE" 2>/dev/null || printf '%s\n' "$REMOTE"
}

# A fresh clone of the Pages branch (or an empty branch, if there is none).
open_site() {
  local site="$1"
  git init -q "$site"
  git -C "$site" remote add origin "$(remote_url)"
  if git -C "$site" ls-remote --exit-code --heads origin "$BRANCH" >/dev/null 2>&1; then
    git -C "$site" fetch -q --depth 1 origin "$BRANCH"
    git -C "$site" checkout -q -B "$BRANCH" FETCH_HEAD
  else
    git -C "$site" checkout -q --orphan "$BRANCH"
  fi
}

release_dirs() {
  local site="$1"
  (cd "$site" && find . -maxdepth 1 -type d -name 'v[0-9]*.[0-9]*.[0-9]*' | sed 's|^\./||') |
    sort -t. -k1.2,1nr -k2,2nr -k3,3nr
}

# The root page lists every release, newest first. It is a pure function of
# the directories, so a repeat publish writes the same bytes.
write_index() {
  local site="$1" dir first=1
  {
    echo '<!doctype html>'
    echo '<html lang="en-GB">'
    echo '<meta charset="utf-8">'
    echo '<meta name="viewport" content="width=device-width, initial-scale=1">'
    echo '<title>The Ravel book</title>'
    echo '<h1>The Ravel book</h1>'
    echo '<p>Each release of Ravel has its own book. Run <code>ravel --version</code>.'
    echo 'Then open the book with the same version.</p>'
    echo '<ul>'
    while IFS= read -r dir; do
      [[ -n "$dir" ]] || continue
      if ((first)); then
        echo "<li><a href=\"$dir/\">ravel ${dir#v}</a> (newest)</li>"
        first=0
      else
        echo "<li><a href=\"$dir/\">ravel ${dir#v}</a></li>"
      fi
    done < <(release_dirs "$site")
    echo '</ul>'
  } >"$site/index.html"
}

author_env() {
  local name email
  name="$(git -C "$REPO_ROOT" config user.name || true)"
  email="$(git -C "$REPO_ROOT" config user.email || true)"
  export GIT_AUTHOR_NAME="${GIT_AUTHOR_NAME:-${name:-Ravel release}}"
  export GIT_COMMITTER_NAME="${GIT_COMMITTER_NAME:-$GIT_AUTHOR_NAME}"
  export GIT_AUTHOR_EMAIL="${GIT_AUTHOR_EMAIL:-${email:-release@invalid}}"
  export GIT_COMMITTER_EMAIL="${GIT_COMMITTER_EMAIL:-$GIT_AUTHOR_EMAIL}"
}

# The tag is public: the same commit is on the remote under the tag's name.
require_public_tag() {
  local tag="$1" commit="$2" refs
  refs="$(git -C "$REPO_ROOT" ls-remote "$REMOTE" "refs/tags/$tag" "refs/tags/$tag^{}")" ||
    die "cannot read the tags of $REMOTE"
  grep -Eq "^${commit}[[:space:]]+refs/tags/${tag}(\\^\\{\\})?\$" <<<"$refs" ||
    die "$REMOTE has no tag $tag at $commit; push the tag before the book"
}

publish() {
  local tag="$1" version site commit existing
  require_tag "$tag"
  version="${tag#v}"
  require_staged "$tag"
  commit="$(identity_value "$tag" commit)"
  # Premature deployment: a book for a release that is not public yet.
  require_public_tag "$tag" "$commit"
  published_release_complete "$version" ||
    die "the GitHub Release for $tag is missing, a draft or incomplete; publish it before the book"

  site="$(mktemp -d)"
  # shellcheck disable=SC2064 # remove precisely this private directory
  trap "rm -rf '$site'" EXIT
  open_site "$site"
  existing="$site/$tag"
  if [[ -d "$existing" ]]; then
    # A release directory is final. The same bytes again is a resumed publish.
    diff -r "$STAGE/$tag" "$existing" >/dev/null ||
      die "$BRANCH already holds a different book for $tag; refusing to change a published release"
  else
    cp -R "$STAGE/$tag" "$existing"
  fi
  : >"$site/.nojekyll"
  write_index "$site"
  local size=0 entry
  while IFS= read -r entry; do
    size=$((size + $(du -sk "$entry" | cut -f1)))
  done < <(find "$site" -mindepth 1 -maxdepth 1 ! -name .git)
  ((size < SITE_LIMIT_KB)) ||
    die "the site would be ${size} KB, over the Pages limit of ${SITE_LIMIT_KB} KB"

  git -C "$site" add -A
  if git -C "$site" diff --cached --quiet 2>/dev/null && git -C "$site" rev-parse --verify --quiet HEAD >/dev/null; then
    echo "release-book: $BRANCH already holds the book for $tag"
  else
    author_env
    git -C "$site" commit -q -m "Book for $tag" -m "Built from $commit."
    git -C "$site" push -q origin "$BRANCH:refs/heads/$BRANCH" ||
      die "the push to $BRANCH failed; nothing changed on the remote. Run 'scripts/release-book.sh publish $tag' again"
    echo "release-book: pushed the book for $tag to $BRANCH"
  fi
  rm -rf "$site"
  trap - EXIT
  verify "$tag"
}

verify() {
  local tag="$1" site problems=0
  require_tag "$tag"
  require_staged "$tag"
  site="$(mktemp -d)"
  # shellcheck disable=SC2064 # remove precisely this private directory
  trap "rm -rf '$site'" EXIT
  git -C "$REPO_ROOT" ls-remote --exit-code --heads "$REMOTE" "$BRANCH" >/dev/null 2>&1 ||
    die "$REMOTE has no branch $BRANCH"
  open_site "$site"
  [[ -f "$site/.nojekyll" ]] || {
    echo "release-book: $BRANCH has no .nojekyll at its root" >&2
    problems=1
  }
  [[ -d "$site/$tag" ]] || {
    echo "release-book: $BRANCH has no directory $tag" >&2
    problems=1
  }
  if [[ -d "$site/$tag" ]] && ! diff -r "$STAGE/$tag" "$site/$tag" >"$site/.diff" 2>&1; then
    echo "release-book: $BRANCH holds a $tag that differs from the staged book:" >&2
    head -n 10 "$site/.diff" >&2
    problems=1
  fi
  grep -Fq "href=\"$tag/\"" "$site/index.html" 2>/dev/null || {
    echo "release-book: the root index.html of $BRANCH does not list $tag" >&2
    problems=1
  }
  ((problems == 0)) || die "the delivery of the book for $tag is incomplete"
  echo "release-book: $BRANCH holds the complete book for $tag"
}

# The Pages site itself serves the book: the one check a fixture cannot make
# against GitHub. A branch publish takes a while to appear, so it retries.
live() {
  local tag="$1" version tries interval n=0 url
  require_tag "$tag"
  version="${tag#v}"
  tries="${RAVEL_BOOK_LIVE_TRIES:-30}"
  interval="${RAVEL_BOOK_LIVE_INTERVAL:-10}"
  url="$SITE_URL/$tag/$VERSION_PAGE.html"
  while ((n < tries)); do
    if curl --fail --silent --location "$url" 2>/dev/null | page_says_version "$version" &&
      curl --fail --silent --location "$SITE_URL/" 2>/dev/null | grep -Fq "href=\"$tag/\""; then
      echo "release-book: $SITE_URL/$tag/ serves the book for $tag"
      return 0
    fi
    n=$((n + 1))
    ((n >= tries)) || sleep "$interval"
  done
  die "$url did not serve 'ravel $version' after $tries tries. If Pages is not enabled for branch $BRANCH, enable it once (chapter 12, \"Set up Pages for the first release\": docs/book/src/12-deliver-ravel.md, #set-up-pages-for-the-first-release), then run 'scripts/release-book.sh live $tag' again"
}

check_remote() {
  git -C "$REPO_ROOT" ls-remote "$REMOTE" HEAD >/dev/null 2>&1 ||
    die "cannot read $REMOTE, which holds branch $BRANCH"
  echo "release-book: $REMOTE answers"
}

# Exit 0 when the repository serves branch gh-pages from its root, 3 when Pages
# is not enabled yet (the first release), and 1 for any other state.
# https://docs.github.com/en/rest/pages/pages#get-a-apiname-pages-site
check_pages() {
  local repo="${RAVEL_BOOK_REPO:-Linkuistics/Ravel}" reply
  if ! reply="$(gh api "repos/$repo/pages" 2>&1)"; then
    if grep -Eq 'Not Found|HTTP 404' <<<"$reply"; then
      echo "release-book: Pages is not enabled for $repo; the first release needs the one-time setup in chapter 12, \"Set up Pages for the first release\" (docs/book/src/12-deliver-ravel.md, #set-up-pages-for-the-first-release)"
      return 3
    fi
    die "cannot read the Pages configuration of $repo: ${reply%%$'\n'*}"
  fi
  if jq -e --arg branch "$BRANCH" '.build_type == "legacy" and .source.branch == $branch and .source.path == "/"' <<<"$reply" >/dev/null; then
    echo "release-book: Pages serves branch $BRANCH from /"
    return 0
  fi
  die "Pages for $repo does not serve branch $BRANCH from /: $(jq -c '{build_type, source}' <<<"$reply")"
}

main() {
  local command="${1:-}"
  case "$command" in
    build | publish | verify | live)
      [[ $# == 2 ]] || die "usage: release-book.sh $command vX.Y.Z"
      "$command" "$2"
      ;;
    check-remote)
      [[ $# == 1 ]] || die 'usage: release-book.sh check-remote'
      check_remote
      ;;
    check-pages)
      [[ $# == 1 ]] || die 'usage: release-book.sh check-pages'
      check_pages
      ;;
    *) die 'usage: release-book.sh build TAG | publish TAG | verify TAG | live TAG | check-remote | check-pages' ;;
  esac
}

main "$@"
