#!/usr/bin/env bash
# The release builds the tagged book and delivers it to the Pages branch, and
# each wrong state stops it. Runs the real scripts and the pinned mdBook in
# isolated repositories. A local bare repository stands in for GitHub's remote,
# a stub stands in for `gh`, and a loopback server stands in for the Pages
# site. Nothing here writes to a public service.
# shellcheck disable=SC2016 # the printf templates hold Markdown, not expansions
set -euo pipefail
IFS=$'\n\t'

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
scratch="$(mktemp -d)"
server_pid=''
cleanup() {
  [[ -z "$server_pid" ]] || kill "$server_pid" 2>/dev/null || true
  rm -rf "$scratch"
}
trap cleanup EXIT
export GIT_AUTHOR_NAME='Release Test' GIT_COMMITTER_NAME='Release Test'
export GIT_AUTHOR_EMAIL='release@example.invalid' GIT_COMMITTER_EMAIL='release@example.invalid'
export BOOK_TOOLS_DIR="$repo_root/target/tools"

fail() {
  echo "FAIL: $*" >&2
  exit 1
}

[[ -x "$BOOK_TOOLS_DIR/mdbook" ]] || fail "the pinned mdBook is missing; run 'task setup'"

# The stub answers the two `gh` calls that the scripts make.
mkdir -p "$scratch/bin"
cat >"$scratch/bin/gh" <<'SH'
#!/usr/bin/env bash
set -euo pipefail
if [[ "$1 $2" == 'release view' ]]; then
  version="${3#v}"
  if [[ "${RELEASE_TEST_RELEASE:-complete}" == draft ]]; then draft=true; else draft=false; fi
  printf '{"isDraft":%s,"assets":[{"name":"ravel-v%s-aarch64-apple-darwin.tar.xz","size":1},{"name":"ravel-v%s-aarch64-unknown-linux-gnu.tar.xz","size":1},{"name":"ravel-v%s-x86_64-unknown-linux-gnu.tar.xz","size":1}]}\n' "$draft" "$version" "$version" "$version"
elif [[ "$1" == api ]]; then
  cat "$RELEASE_TEST_PAGES"
  [[ "$(cat "$RELEASE_TEST_PAGES")" != 'gh: Not Found (HTTP 404)' && "$(cat "$RELEASE_TEST_PAGES")" != 'gh: Server Error (HTTP 500)' ]]
else
  exit 99
fi
SH
chmod +x "$scratch/bin/gh"
export PATH="$scratch/bin:$PATH"
export RELEASE_TEST_PAGES="$scratch/pages-reply"

# The fixture repository holds a small book and the real scripts.
fixture="$scratch/repo"
remote="$scratch/remote.git"
page=docs/book/src/03-a-complete-session.md
git init -q --bare "$remote"
mkdir -p "$fixture/scripts" "$fixture/docs/book/src"
for script in book.sh release-book.sh release-book-version.sh release-common.sh; do
  cp "$repo_root/scripts/$script" "$fixture/scripts/"
done
printf '[book]\ntitle = "Fixture book"\nsrc = "src"\n\n[build]\nbuild-dir = "../../target/book"\ncreate-missing = false\n' >"$fixture/docs/book/book.toml"
printf '# Summary\n\n- [One](01-one.md)\n- [Three](03-a-complete-session.md)\n' >"$fixture/docs/book/src/SUMMARY.md"
printf '# One\n\nThe first chapter.\n' >"$fixture/docs/book/src/01-one.md"
printf '/target/\n' >"$fixture/.gitignore"
git init -q "$fixture"
git -C "$fixture" remote add origin "$remote"

# Commit a book whose version line says $1 and whose body says $2, and tag it.
release() {
  local version="$1" body="$2"
  printf '# Three\n\n%s\n\n```text\nravel %s\n```\n' "$body" "$version" >"$fixture/$page"
  git -C "$fixture" add -A
  git -C "$fixture" commit -qm "release $version"
  git -C "$fixture" tag -a -f "v$version" -m "Release v$version"
}
script() { (cd "$fixture" && bash scripts/release-book.sh "$@"); }
refuses() {
  local label="$1" expected="$2"
  shift 2
  if "$@" >"$scratch/output" 2>&1; then fail "$label: accepted"; fi
  grep -Fq -- "$expected" "$scratch/output" || {
    cat "$scratch/output" >&2
    fail "$label: the refusal does not say '$expected'"
  }
}
remote_branch() { git --git-dir="$remote" rev-parse --verify --quiet refs/heads/gh-pages || true; }

release 1.2.3 'Body of 1.2.3 as tagged.'

# --- build: the tag's source and version -----------------------------------
# The working copy changes after the tag. The book must show the tag.
printf '# Three\n\nWORKING COPY ONLY\n\n```text\nravel 1.2.3\n```\n' >"$fixture/$page"
script build v1.2.3 >/dev/null
grep -Fq 'Body of 1.2.3 as tagged.' "$fixture/target/book-release/v1.2.3/03-a-complete-session.html" ||
  fail 'the staged book does not hold the tagged text'
if grep -rFq 'WORKING COPY ONLY' "$fixture/target/book-release/v1.2.3"; then fail 'the staged book holds working-copy text'; fi
git -C "$fixture" checkout -q -- "$page"
grep -Fxq "commit=$(git -C "$fixture" rev-parse 'v1.2.3^{commit}')" "$fixture/target/book-release/v1.2.3.identity" ||
  fail 'the identity file does not name the tag commit'

# A tag whose page names another version cannot be built.
git -C "$fixture" tag -a -f v1.9.0 -m wrong "$(git -C "$fixture" rev-parse 'v1.2.3^{commit}')"
refuses 'wrong version' "but the release is 'ravel 1.9.0'" script build v1.9.0
[[ ! -e "$fixture/target/book-release/v1.9.0" ]] || fail 'a refused build left a staged book'
refuses 'unknown tag' 'no tag' script build v8.8.8
refuses 'not a tag name' 'not a release tag' script build main

# A version line that the page does not show as its code block is refused, even
# when the Markdown has the line (here inside an HTML comment).
printf '# Three\n\n<!--\nravel 1.6.0\n-->\n\nNo visible line.\n' >"$fixture/$page"
git -C "$fixture" commit -qam 'hidden version line'
git -C "$fixture" tag -a -f v1.6.0 -m hidden
refuses 'hidden version line' "does not say 'ravel 1.6.0'" script build v1.6.0
[[ ! -e "$fixture/target/book-release/v1.6.0" ]] || fail 'a refused build left a staged book'
git -C "$fixture" reset -q --hard v1.2.3
git -C "$fixture" tag -d v1.6.0 >/dev/null

# A failed build stages nothing, and clears an earlier stage of that tag.
printf '# Three\n\n{{#include missing.md}}\n\n```text\nravel 2.0.0\n```\n' >"$fixture/$page"
git -C "$fixture" add -A
git -C "$fixture" commit -qm 'broken book'
git -C "$fixture" tag -a v2.0.0 -m 'Release v2.0.0'
refuses 'failed build' 'not clean' script build v2.0.0
[[ ! -e "$fixture/target/book-release/v2.0.0" ]] || fail 'a failed build left a staged book'
cp -R "$fixture/target/book-release/v1.2.3" "$scratch/keep"
printf 'stale\n' >"$fixture/target/book-release/v2.0.0.identity"
refuses 'failed build over a stale identity' 'not clean' script build v2.0.0
[[ ! -e "$fixture/target/book-release/v2.0.0.identity" ]] || fail 'a failed build kept the old identity'
git -C "$fixture" reset -q --hard v1.2.3
git -C "$fixture" tag -d v2.0.0 v1.9.0 >/dev/null

# A placeholder in the tag is refused before any build.
printf '# Three\n\nwith placeholder\n\n```text\nravel @RAVEL_BOOK_VERSION@\n```\n' >"$fixture/$page"
git -C "$fixture" commit -qam 'placeholder'
git -C "$fixture" tag -a -f v1.5.0 -m placeholder
refuses 'placeholder' "but the release is 'ravel 1.5.0'" script build v1.5.0
git -C "$fixture" reset -q --hard v1.2.3
git -C "$fixture" tag -d v1.5.0 >/dev/null

# --- publish: nothing public until the release is public -------------------
script build v1.2.3 >/dev/null
echo '{"build_type":"legacy"}' >"$RELEASE_TEST_PAGES"
refuses 'unpushed tag' 'has no tag v1.2.3' script publish v1.2.3
[[ -z "$(remote_branch)" ]] || fail 'a premature publish created the Pages branch'
git -C "$fixture" push -q origin v1.2.3
RELEASE_TEST_RELEASE=draft refuses 'draft release' 'missing, a draft or incomplete' script publish v1.2.3
[[ -z "$(remote_branch)" ]] || fail 'a publish before the GitHub Release created the Pages branch'
refuses 'no staged book' 'no staged book for v1.3.0' script publish v1.3.0

# A stage that is not the tag as it stands, or that changed, is refused.
printf 'extra\n' >"$fixture/target/book-release/v1.2.3/extra.txt"
refuses 'changed stage' 'changed after its build' script publish v1.2.3
rm "$fixture/target/book-release/v1.2.3/extra.txt"
ln -s index.html "$fixture/target/book-release/v1.2.3/link.html"
refuses 'symbolic link' 'symbolic links' script publish v1.2.3
rm "$fixture/target/book-release/v1.2.3/link.html"
good_commit="$(git -C "$fixture" rev-parse 'v1.2.3^{commit}')"
printf 'changed\n' >>"$fixture/docs/book/src/01-one.md"
git -C "$fixture" commit -qam 'later change'
git -C "$fixture" tag -a -f v1.2.3 -m moved
refuses 'moved tag' 'build it again' script publish v1.2.3
git -C "$fixture" reset -q --hard "$good_commit"
git -C "$fixture" tag -a -f v1.2.3 -m 'Release v1.2.3' "$good_commit"
[[ -z "$(remote_branch)" ]] || fail 'a refused publish created the Pages branch'

# The site limit is checked before the push.
RAVEL_BOOK_SITE_LIMIT_KB=1 refuses 'site limit' 'over the Pages limit' script publish v1.2.3
[[ -z "$(remote_branch)" ]] || fail 'a publish over the limit created the Pages branch'

# A remote that rejects the push leaves the stage whole, and a repeat resumes.
mkdir -p "$remote/hooks"
printf '#!/bin/sh\nexit 1\n' >"$remote/hooks/pre-receive"
chmod +x "$remote/hooks/pre-receive"
refuses 'rejected push' 'nothing changed on the remote' script publish v1.2.3
[[ -z "$(remote_branch)" ]] || fail 'a rejected push left a branch'
rm "$remote/hooks/pre-receive"
script publish v1.2.3 >"$scratch/output"
grep -Fq 'holds the complete book for v1.2.3' "$scratch/output" || fail 'the resumed publish did not verify'

# --- the delivered branch --------------------------------------------------
view="$scratch/view"
git clone -q --branch gh-pages "$remote" "$view"
[[ -f "$view/.nojekyll" ]] || fail 'the branch has no .nojekyll'
diff -r "$fixture/target/book-release/v1.2.3" "$view/v1.2.3" >/dev/null || fail 'the branch holds a different book from the stage'
grep -Fq 'href="v1.2.3/"' "$view/index.html" || fail 'the root index does not list v1.2.3'
[[ ! -e "$view/v1.2.3.identity" && -z "$(find "$view" -name '*.identity' -not -path '*/.git/*')" ]] || fail 'the identity file was published'

# Publishing the same book again changes nothing.
tip="$(remote_branch)"
script publish v1.2.3 >"$scratch/output"
grep -Fq 'already holds the book for v1.2.3' "$scratch/output" || fail 'a repeat publish did not say so'
[[ "$(remote_branch)" == "$tip" ]] || fail 'a repeat publish added a commit'

# A published release is final: a different book for the same tag is refused.
git -C "$fixture" push -q -f origin v1.2.3
printf 'changed\n' >>"$fixture/docs/book/src/01-one.md"
git -C "$fixture" commit -qam 'different book'
git -C "$fixture" tag -a -f v1.2.3 -m moved
git -C "$fixture" push -q -f origin v1.2.3
script build v1.2.3 >/dev/null 2>&1 || true
refuses 'different book for a published tag' 'refusing to change a published release' script publish v1.2.3
[[ "$(remote_branch)" == "$tip" ]] || fail 'a refused publish moved the branch'
git -C "$fixture" reset -q --hard "$good_commit"
git -C "$fixture" tag -a -f v1.2.3 -m 'Release v1.2.3' "$good_commit"
git -C "$fixture" push -q -f origin v1.2.3

# A second release adds its directory and leaves the first as it was.
before="$(cd "$view/v1.2.3" && find . -type f -print0 | LC_ALL=C sort -z | xargs -0 shasum -a 256 | shasum -a 256)"
release 1.3.0 'Body of 1.3.0.'
git -C "$fixture" push -q origin v1.3.0
script build v1.3.0 >/dev/null
script publish v1.3.0 >/dev/null
git -C "$view" pull -q
after="$(cd "$view/v1.2.3" && find . -type f -print0 | LC_ALL=C sort -z | xargs -0 shasum -a 256 | shasum -a 256)"
[[ "$before" == "$after" ]] || fail 'the second release changed the first'
[[ -d "$view/v1.3.0" ]] || fail 'the second release has no directory'
[[ "$(grep -o 'href="v[0-9.]*/"' "$view/index.html" | paste -sd' ' -)" == 'href="v1.3.0/" href="v1.2.3/"' ]] ||
  fail 'the index does not list the newest release first'

# --- verify: an incomplete delivery ----------------------------------------
script verify v1.3.0 >/dev/null
tamper="$scratch/tamper"
git clone -q --branch gh-pages "$remote" "$tamper"
git -C "$tamper" rm -q v1.3.0/03-a-complete-session.html
git -C "$tamper" commit -qm 'lose a page'
git -C "$tamper" push -q origin gh-pages
refuses 'a lost page' 'incomplete' script verify v1.3.0
git -C "$tamper" reset -q --hard HEAD~1
git -C "$tamper" push -q -f origin gh-pages
script verify v1.3.0 >/dev/null
git -C "$tamper" rm -q .nojekyll
git -C "$tamper" commit -qm 'lose .nojekyll'
git -C "$tamper" push -q origin gh-pages
refuses 'no .nojekyll' 'no .nojekyll' script verify v1.3.0
git -C "$tamper" reset -q --hard HEAD~1
git -C "$tamper" push -q -f origin gh-pages
git -C "$tamper" rm -q -r v1.3.0
git -C "$tamper" commit -qm 'lose the directory'
git -C "$tamper" push -q origin gh-pages
refuses 'no directory' 'has no directory v1.3.0' script verify v1.3.0
git -C "$tamper" reset -q --hard HEAD~1
git -C "$tamper" push -q -f origin gh-pages
script verify v1.3.0 >/dev/null

# --- live: the site itself -------------------------------------------------
git -C "$view" pull -q
port="$(python3 -c 'import socket; s = socket.socket(); s.bind(("127.0.0.1", 0)); print(s.getsockname()[1])')"
python3 -m http.server "$port" --bind 127.0.0.1 --directory "$view" >/dev/null 2>&1 &
server_pid=$!
for _ in 1 2 3 4 5 6 7 8 9 10; do
  curl --silent --fail "http://127.0.0.1:$port/" >/dev/null 2>&1 && break
  sleep 0.5
done
export RAVEL_BOOK_SITE_URL="http://127.0.0.1:$port" RAVEL_BOOK_LIVE_TRIES=2 RAVEL_BOOK_LIVE_INTERVAL=0
script live v1.3.0 >/dev/null
script live v1.2.3 >/dev/null
refuses 'live book not served' 'did not serve' script live v1.4.0
unset RAVEL_BOOK_SITE_URL RAVEL_BOOK_LIVE_TRIES RAVEL_BOOK_LIVE_INTERVAL

# --- preflight: the remote and the Pages setting ---------------------------
script check-remote >/dev/null
refuses 'unreachable remote' 'cannot read' env RAVEL_BOOK_REMOTE="$scratch/none.git" bash "$fixture/scripts/release-book.sh" check-remote
printf 'gh: Not Found (HTTP 404)' >"$RELEASE_TEST_PAGES"
status=0
script check-pages >"$scratch/output" 2>&1 || status=$?
[[ "$status" == 3 ]] || fail "Pages not enabled returned $status, not 3"
grep -Fq 'first release needs the one-time setup' "$scratch/output" || fail 'the not-enabled message does not point to the setup'
echo '{"build_type":"legacy","source":{"branch":"gh-pages","path":"/"}}' >"$RELEASE_TEST_PAGES"
script check-pages >/dev/null
echo '{"build_type":"workflow","source":{"branch":"main","path":"/"}}' >"$RELEASE_TEST_PAGES"
refuses 'workflow source' 'does not serve branch gh-pages' script check-pages
echo '{"build_type":"legacy","source":{"branch":"main","path":"/docs"}}' >"$RELEASE_TEST_PAGES"
refuses 'other branch' 'does not serve branch gh-pages' script check-pages
printf 'gh: Server Error (HTTP 500)' >"$RELEASE_TEST_PAGES"
refuses 'unreadable setting' 'cannot read the Pages configuration' script check-pages

echo 'release book: all passed'
