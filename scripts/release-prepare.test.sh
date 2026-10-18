#!/usr/bin/env bash
# Real jj/local-remotes and cargo-release; no model or public service runs.
set -euo pipefail
IFS=$'\n\t'
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
scratch="$(mktemp -d)"
trap 'rm -rf "$scratch"' EXIT
export JJ_CONFIG="$scratch/jj.toml"
printf '[user]\nname="Release Test"\nemail="release@example.invalid"\n' >"$JJ_CONFIG"
export GIT_AUTHOR_NAME='Release Test' GIT_COMMITTER_NAME='Release Test'
export GIT_AUTHOR_EMAIL='release@example.invalid' GIT_COMMITTER_EMAIL='release@example.invalid'
fail() {
  echo "FAIL: $*" >&2
  exit 1
}
git init -q --bare "$scratch/remote.git"
jj git init --colocate "$scratch/repo" >/dev/null
mkdir -p "$scratch/repo/scripts" "$scratch/repo/crates/ravel/src" "$scratch/repo/crates/keyed-launch/src" "$scratch/bin"
cp "$root/scripts/release-prepare.sh" "$root/scripts/release-notes.sh" "$root/scripts/release-common.sh" "$scratch/repo/scripts/"
cp "$root/Taskfile.yml" "$root/release.toml" "$scratch/repo/"
printf '/target/\n' >"$scratch/repo/.gitignore"
cat >"$scratch/repo/Cargo.toml" <<'TOML'
[workspace]
members = ["crates/ravel", "crates/keyed-launch"]
resolver = "2"
[workspace.package]
version = "0.0.0"
edition = "2021"
TOML
printf '[package]\nname="ravel"\nversion.workspace=true\nedition.workspace=true\n' >"$scratch/repo/crates/ravel/Cargo.toml"
printf '[package]\nname="keyed-launch"\nversion.workspace=true\nedition.workspace=true\n[package.metadata.release]\nrelease=false\n' >"$scratch/repo/crates/keyed-launch/Cargo.toml"
printf 'fn main() {}\n' >"$scratch/repo/crates/ravel/src/main.rs"
printf 'pub fn runner() {}\n' >"$scratch/repo/crates/keyed-launch/src/lib.rs"
printf '# Changelog\n\n## Unreleased\n\n' >"$scratch/repo/CHANGELOG.md"
# The production release.toml also replaces the book's version line.
book_page=docs/book/src/03-a-complete-session.md
mkdir -p "$scratch/repo/$(dirname "$book_page")"
cp "$root/$book_page" "$scratch/repo/$book_page"
cargo generate-lockfile --manifest-path "$scratch/repo/Cargo.toml" --offline
jj -R "$scratch/repo" describe -m 'Introduce independent sessions' >/dev/null
jj -R "$scratch/repo" git remote add origin "$scratch/remote.git"
jj -R "$scratch/repo" git push --named main=@ >/dev/null
jj -R "$scratch/repo" new main >/dev/null
printf 'fn main() { println!("improved"); }\n' >"$scratch/repo/crates/ravel/src/main.rs"
jj -R "$scratch/repo" describe -m 'Improve session output' >/dev/null
jj -R "$scratch/repo" bookmark set main -r @ >/dev/null
jj -R "$scratch/repo" new main >/dev/null
printf 'unrelated work\n' >"$scratch/repo/owner-work.txt"
saved="$(jj -R "$scratch/repo" log --no-graph -r @ -T change_id)"
# Refuse any attempt to launch Grove or a model. gh only supplies release state.
for program in grove ravel codex claude; do
  printf '#!/bin/sh\nexit 89\n' >"$scratch/bin/$program"
done
cat >"$scratch/bin/gh" <<'SH'
#!/bin/sh
exit "${RELEASE_TEST_GH_STATUS:-1}"
SH
chmod +x "$scratch/bin/"*
export PATH="$scratch/bin:$PATH"
(cd "$scratch/repo" && bash scripts/release-prepare.sh)
grep -Fxq -- '- Introduce independent sessions' "$scratch/repo/CHANGELOG.md"
grep -Fxq -- '- Improve session output' "$scratch/repo/CHANGELOG.md"
[[ ! -e "$scratch/repo/owner-work.txt" ]]
[[ "$(jj -R "$scratch/repo" file show -r "$saved" root:owner-work.txt)" == 'unrelated work' ]]
[[ -z "$(git -C "$scratch/repo" status --porcelain)" ]]
cp "$scratch/repo/CHANGELOG.md" "$scratch/notes"
before="$(jj -R "$scratch/repo" log --no-graph -r main -T commit_id)"
(cd "$scratch/repo" && bash scripts/release-prepare.sh)
cmp "$scratch/notes" "$scratch/repo/CHANGELOG.md"
[[ "$(jj -R "$scratch/repo" log --no-graph -r main -T commit_id)" == "$before" ]]
# Explicit input can refresh notes, but bad input never modifies the changelog.
printf ' \n\t\n' >"$scratch/empty"
printf '## v9.0.0\n- wrong section\n' >"$scratch/heading"
for input in missing empty heading; do
  if task --dir "$scratch/repo" release:notes CLI_ARGS="--from-file $scratch/$input" >"$scratch/output" 2>&1; then
    fail "notes accepted $input input"
  fi
  cmp "$scratch/notes" "$scratch/repo/CHANGELOG.md"
done
printf '\n- Reviewed session release.\n\n' >"$scratch/reviewed"
task --dir "$scratch/repo" release:notes CLI_ARGS="--from-file $scratch/reviewed"
grep -Fxq -- '- Reviewed session release.' "$scratch/repo/CHANGELOG.md"
# Versioned sections must survive replacement byte-for-byte.
printf '\n## v0.0.0\n\n- Historical prose.\n\nFooter.\n' >>"$scratch/repo/CHANGELOG.md"
task --dir "$scratch/repo" release:notes CLI_ARGS="--from-file $scratch/reviewed"
sed -n '/^## v0.0.0/,$p' "$scratch/repo/CHANGELOG.md" >"$scratch/versioned"
printf '## v0.0.0\n\n- Historical prose.\n\nFooter.\n' >"$scratch/expected"
cmp "$scratch/versioned" "$scratch/expected"
# Missing/duplicate heading refuses, preserving every byte.
for mode in missing duplicate; do
  cp "$scratch/notes" "$scratch/repo/CHANGELOG.md"
  if [[ "$mode" == missing ]]; then sed -i.bak 's/^## Unreleased$/## Gone/' "$scratch/repo/CHANGELOG.md"; else printf '\n## Unreleased\n' >>"$scratch/repo/CHANGELOG.md"; fi
  cp "$scratch/repo/CHANGELOG.md" "$scratch/bad"
  if task --dir "$scratch/repo" release:notes >"$scratch/output" 2>&1; then fail "notes accepted $mode heading"; fi
  cmp "$scratch/bad" "$scratch/repo/CHANGELOG.md"
done
rm -f "$scratch/repo/CHANGELOG.md.bak"
cp "$scratch/notes" "$scratch/repo/CHANGELOG.md"
jj -R "$scratch/repo" describe -m 'Reviewed notes' >/dev/null
jj -R "$scratch/repo" bookmark set main -r @ >/dev/null
jj -R "$scratch/repo" new main >/dev/null
# Real dry cuts must change nothing and select only Ravel's replacement once.
(cd "$scratch/repo" && cargo release config --manifest-path crates/ravel/Cargo.toml) >"$scratch/config" 2>&1
grep -Fq 'tag-name = "v{{version}}"' "$scratch/config"
for level in patch minor major; do
  before="$(git -C "$scratch/repo" rev-parse HEAD)"
  cp "$scratch/repo/CHANGELOG.md" "$scratch/pre-dry"
  (cd "$scratch/repo" && cargo release "$level" -v) >"$scratch/dry-$level" 2>&1
  grep -Fq 'Unreleased' "$scratch/dry-$level"
  [[ "$(git -C "$scratch/repo" rev-parse HEAD)" == "$before" ]]
  [[ -z "$(git -C "$scratch/repo" tag --list 'v*')" ]]
  cmp "$scratch/pre-dry" "$scratch/repo/CHANGELOG.md"
done
grep -Fq '0.1.0' "$scratch/dry-minor"
grep -Fxq -- '+ravel 0.1.0' "$scratch/dry-minor"
# Empty standing notes still preview a correctly reseeded version heading.
printf '# Changelog\n\n## Unreleased\n' >"$scratch/repo/CHANGELOG.md"
jj -R "$scratch/repo" describe -m 'empty changelog fixture' >/dev/null
jj -R "$scratch/repo" new >/dev/null
(cd "$scratch/repo" && cargo release minor -v) >"$scratch/empty-dry" 2>&1
grep -Fq '0.1.0' "$scratch/empty-dry"
cp "$scratch/notes" "$scratch/repo/CHANGELOG.md"
# Production config guards the heading even when cargo-release is run directly.
for mode in missing duplicate unanchored; do
  cp "$scratch/notes" "$scratch/repo/CHANGELOG.md"
  cp "$root/release.toml" "$scratch/repo/release.toml"
  case "$mode" in
    missing) sed -i.bak 's/^## Unreleased$/## Gone/' "$scratch/repo/CHANGELOG.md" ;;
    duplicate) printf '\n## Unreleased\n' >>"$scratch/repo/CHANGELOG.md" ;;
    unanchored)
      printf '\nText mentioning ## Unreleased in prose.\n' >>"$scratch/repo/CHANGELOG.md"
      sed -i.bak 's/\^## Unreleased\$/## Unreleased/' "$scratch/repo/release.toml"
      ;;
  esac
  rm -f "$scratch/repo/"*.bak
  jj -R "$scratch/repo" describe -m "replacement control $mode" >/dev/null
  jj -R "$scratch/repo" new >/dev/null
  cp "$scratch/repo/CHANGELOG.md" "$scratch/pre-dry"
  if (cd "$scratch/repo" && cargo release minor) >"$scratch/output" 2>&1; then fail "cut accepted $mode replacement"; fi
  grep -Eq 'replacements expected|no matches|replacements' "$scratch/output"
  cmp "$scratch/pre-dry" "$scratch/repo/CHANGELOG.md"
done
# A current local tag absent remotely is a partial cut, even with newer changes.
cp "$root/release.toml" "$scratch/repo/release.toml"
cp "$scratch/notes" "$scratch/repo/CHANGELOG.md"
jj -R "$scratch/repo" describe -m 'Return to valid preparation' >/dev/null
jj -R "$scratch/repo" bookmark set main -r @ >/dev/null
jj -R "$scratch/repo" new main >/dev/null
git -C "$scratch/repo" tag v0.0.0
if (cd "$scratch/repo" && bash scripts/release-prepare.sh) >"$scratch/output" 2>&1; then fail 'accepted partial local cut'; fi
grep -Fq 'resume' "$scratch/output"
# Remote tag + a GitHub record alone are insufficient: draft/missing assets
# remain partial publication, and a local-only tap version cannot clear it.
git -C "$scratch/repo" push origin v0.0.0 >/dev/null
cat >"$scratch/bin/gh" <<'SH'
#!/bin/sh
case "${RELEASE_TEST_GITHUB_MODE:-missing}" in
  draft) printf '%s\n' '{"isDraft":true,"assets":[]}' ;;
  missing) printf '%s\n' '{"isDraft":false,"assets":[]}' ;;
  *) exit 1 ;;
esac
SH
for mode in missing draft; do
  if (cd "$scratch/repo" && RELEASE_TEST_GITHUB_MODE="$mode" bash scripts/release-prepare.sh) >"$scratch/output" 2>&1; then fail "accepted $mode GitHub release"; fi
  grep -Fq 'GitHub' "$scratch/output"
  grep -Fq 'resume' "$scratch/output"
done
# A cut/tag off main must be refused without hiding it in a fresh change.
jj -R "$scratch/repo" new main >/dev/null
printf 'version cut work\n' >"$scratch/repo/cut.txt"
jj -R "$scratch/repo" describe -m 'chore: release v0.1.0' >/dev/null
jj -R "$scratch/repo" new >/dev/null
git -C "$scratch/repo" tag v0.1.0
if (cd "$scratch/repo" && bash scripts/release-prepare.sh) >"$scratch/output" 2>&1; then fail 'accepted unintegrated cut'; fi
grep -Fq 'resume' "$scratch/output"
# Complete prior release admits genuinely new work, while an already tagged
# main refuses a repeated bump. Only fixture tags/remotes are changed.
git -C "$scratch/repo" tag -d v0.1.0 >/dev/null
sed -i.bak 's/version = "0.0.0"/version = "0.1.0"/' "$scratch/repo/Cargo.toml"
rm "$scratch/repo/Cargo.toml.bak"
cargo generate-lockfile --manifest-path "$scratch/repo/Cargo.toml" --offline
jj -R "$scratch/repo" describe -m 'chore: release v0.1.0' >/dev/null
jj -R "$scratch/repo" bookmark set main -r @ >/dev/null
jj -R "$scratch/repo" new main >/dev/null
git -C "$scratch/repo" tag v0.1.0
git -C "$scratch/repo" push origin v0.1.0 >/dev/null
git init -q --bare "$scratch/tap-remote.git"
jj git init --colocate "$scratch/tap" >/dev/null
mkdir "$scratch/tap/Formula"
printf '  version "0.1.0"\n' >"$scratch/tap/Formula/ravel.rb"
jj -R "$scratch/tap" describe -m 'ravel v0.1.0' >/dev/null
jj -R "$scratch/tap" git remote add origin "$scratch/tap-remote.git"
jj -R "$scratch/tap" git push --named main=@ >/dev/null
jj -R "$scratch/tap" new main >/dev/null
export RAVEL_TAP_DIR="$scratch/tap"
cat >"$scratch/bin/gh" <<'SH'
#!/bin/sh
printf '%s\n' '{"isDraft":false,"assets":[{"name":"ravel-v0.1.0-aarch64-apple-darwin.tar.xz","size":1},{"name":"ravel-v0.1.0-aarch64-unknown-linux-gnu.tar.xz","size":1},{"name":"ravel-v0.1.0-x86_64-unknown-linux-gnu.tar.xz","size":1}]}'
SH
if (cd "$scratch/repo" && bash scripts/release-prepare.sh) >"$scratch/output" 2>&1; then fail 'accepted repeated tagged-main cut'; fi
grep -Fq 'No changes since v0.1.0' "$scratch/output"
printf 'fn main() { println!("next"); }\n' >"$scratch/repo/crates/ravel/src/main.rs"
jj -R "$scratch/repo" describe -m 'Improve next session' >/dev/null
jj -R "$scratch/repo" bookmark set main -r @ >/dev/null
jj -R "$scratch/repo" new main >/dev/null
(cd "$scratch/repo" && bash scripts/release-prepare.sh) >"$scratch/output" 2>&1
cmp "$scratch/notes" "$scratch/repo/CHANGELOG.md"
echo 'release preparation tests: all passed'
echo 'release preparation tests: all passed'
