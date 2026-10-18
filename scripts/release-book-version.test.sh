#!/usr/bin/env bash
# The release writes its own version into the book, and a release whose book
# disagrees stops. Runs the real cargo-release (1.1.2) and the real release.toml
# in an isolated repository; nothing here touches this checkout.
set -euo pipefail
IFS=$'\n\t'

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
scratch="$(mktemp -d)"
# shellcheck disable=SC2064 # exact owned test directory
trap "rm -rf '$scratch'" EXIT
export GIT_AUTHOR_NAME='Release Test' GIT_COMMITTER_NAME='Release Test'
export GIT_AUTHOR_EMAIL='release@example.invalid' GIT_COMMITTER_EMAIL='release@example.invalid'

fail() {
  echo "FAIL: $*" >&2
  exit 1
}

page=docs/book/src/03-a-complete-session.md
fixture="$scratch/repo"
mkdir -p "$fixture/scripts" "$fixture/crates/ravel/src" "$fixture/docs/book/src"
cp "$repo_root/scripts/release-book-version.sh" "$fixture/scripts/"
cp "$repo_root/release.toml" "$fixture/"
cp "$repo_root/$page" "$fixture/$page"
printf '## Unreleased\n\n- Fixture change.\n' >"$fixture/CHANGELOG.md"
printf '[workspace]\nmembers = ["crates/ravel"]\nresolver = "2"\n\n[workspace.package]\nversion = "1.0.0"\nedition = "2021"\n' >"$fixture/Cargo.toml"
printf '[package]\nname = "ravel"\npublish = false\nversion.workspace = true\nedition.workspace = true\n' >"$fixture/crates/ravel/Cargo.toml"
echo 'fn main() {}' >"$fixture/crates/ravel/src/main.rs"
printf '/target/\n' >"$fixture/.gitignore"
(cd "$fixture" && cargo generate-lockfile --quiet --offline)
git init -q "$fixture"
git -C "$fixture" add .
git -C "$fixture" commit -qm fixture

check="$fixture/scripts/release-book-version.sh"
version_lines() { grep -E '^ravel (@RAVEL_BOOK_VERSION@|[0-9]+\.[0-9]+\.[0-9]+)$' "$fixture/$page"; }
cut() { (cd "$fixture" && cargo release "$@" >"$scratch/cargo" 2>&1) || { cat "$scratch/cargo" >&2; fail "cargo release $* failed"; }; }

# The book starts with the placeholder, and a release can replace it.
[[ "$(version_lines)" == 'ravel @RAVEL_BOOK_VERSION@' ]] || fail 'the fixture page lacks the placeholder line'
"$check" ready >/dev/null

# The dry run shows the replacement and changes nothing.
cut minor
grep -Fxq -- '+ravel 1.1.0' "$scratch/cargo" || fail 'the dry run does not show the placeholder replaced by 1.1.0'
[[ "$(version_lines)" == 'ravel @RAVEL_BOOK_VERSION@' ]] || fail 'the dry run changed the page'

# The first cut replaces the placeholder, and the tag ships that line.
cut minor --execute --no-confirm
[[ "$(version_lines)" == 'ravel 1.1.0' ]] || fail 'the first cut did not write 1.1.0'
"$check" released v1.1.0 >/dev/null

# The line is found again: the next cut replaces the previous version.
"$check" ready >/dev/null
cut patch --execute --no-confirm
[[ "$(version_lines)" == 'ravel 1.1.1' ]] || fail 'the second cut did not replace 1.1.0 by 1.1.1'
"$check" released v1.1.1 >/dev/null

# Control: the check refuses a version that does not agree with the release.
# The tag v1.2.0 here marks a commit whose page still says 1.1.1.
git -C "$fixture" tag v1.2.0 v1.1.1
if "$check" released v1.2.0 >"$scratch/output" 2>&1; then fail 'the check accepted a book from the previous release'; fi
grep -Fq "says 'ravel 1.1.1', but the release is 'ravel 1.2.0'" "$scratch/output" || fail 'the refusal does not name both versions'
if "$check" released v9.9.9 >"$scratch/output" 2>&1; then fail 'the check accepted a tag that does not exist'; fi

# Control: a release stops when the placeholder survives into the tag.
git -C "$fixture" checkout -q v1.1.0
sed -i.bak 's/^ravel 1.1.0$/ravel @RAVEL_BOOK_VERSION@/' "$fixture/$page"
rm "$fixture/$page.bak"
git -C "$fixture" commit -qam 'restore placeholder'
git -C "$fixture" tag v1.3.0
if "$check" released v1.3.0 >"$scratch/output" 2>&1; then fail 'the check accepted the placeholder in a release'; fi
grep -Fq "says 'ravel @RAVEL_BOOK_VERSION@', but the release is 'ravel 1.3.0'" "$scratch/output" || fail 'the placeholder refusal does not name both lines'

# Control: a page without the line, or with two, is refused before the cut.
cp "$repo_root/$page" "$fixture/$page"
"$check" ready >/dev/null
sed -i.bak '/^ravel @RAVEL_BOOK_VERSION@$/d' "$fixture/$page"
rm "$fixture/$page.bak"
if "$check" ready >"$scratch/output" 2>&1; then fail 'ready accepted a page with no version line'; fi
grep -Fq 'has 0 lines' "$scratch/output" || fail 'the missing-line refusal does not say 0'
printf 'ravel 1.0.0\nravel 1.0.0\n' >>"$fixture/$page"
if "$check" ready >"$scratch/output" 2>&1; then fail 'ready accepted a page with two version lines'; fi
grep -Fq 'has 2 lines' "$scratch/output" || fail 'the duplicate-line refusal does not say 2'

echo 'release book version: all passed'
