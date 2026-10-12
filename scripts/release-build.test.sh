#!/usr/bin/env bash
# Tagged-mode admission runs in a disposable source fixture; builds are refused
# at cargo's expensive boundary, never reaching runtime fetch or publication.
set -euo pipefail
IFS=$'\n\t'
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
scratch="$(mktemp -d)"
trap 'rm -rf "$scratch"' EXIT
export GIT_AUTHOR_NAME='Release Test' GIT_COMMITTER_NAME='Release Test'
export GIT_AUTHOR_EMAIL='release@example.invalid' GIT_COMMITTER_EMAIL='release@example.invalid'
mkdir -p "$scratch/repo/scripts" "$scratch/repo/target/dist" "$scratch/bin"
cp "$root/scripts/release-build.sh" "$root/scripts/release-common.sh" "$scratch/repo/scripts/"
printf '/target/\n' >"$scratch/repo/.gitignore"
printf '#!/bin/sh\nexit 0\n' >"$scratch/repo/scripts/release-doctor.sh"
chmod +x "$scratch/repo/scripts/release-doctor.sh"
printf 'preserve dist\n' >"$scratch/repo/target/dist/old"
git init -q "$scratch/repo"
git -C "$scratch/repo" add .
git -C "$scratch/repo" commit -qm fixture
export CARGO_HOME="$scratch/no-rustup"
cat >"$scratch/bin/cargo" <<'SH'
#!/bin/sh
case "$1" in
  pkgid) echo "path+file:///fixture#ravel@${RELEASE_TEST_VERSION:-0.1.0}" ;;
  build) echo 'build-boundary' >&2; exit 73 ;;
  *) exit 87 ;;
esac
SH
cat >"$scratch/bin/task" <<'SH'
#!/bin/sh
exit 0
SH
chmod +x "$scratch/bin/"*
export PATH="$scratch/bin:$PATH"
fail() {
  echo "FAIL: $*" >&2
  exit 1
}
for mode in untagged dirty wrong-version; do
  if [[ "$mode" == dirty ]]; then
    git -C "$scratch/repo" tag v0.1.0
    touch "$scratch/repo/unrelated"
  fi
  if [[ "$mode" == wrong-version ]]; then rm "$scratch/repo/unrelated"; fi
  if (cd "$scratch/repo" && RELEASE_TEST_VERSION=0.0.0 bash scripts/release-build.sh) >"$scratch/output" 2>&1; then fail "build admitted $mode"; fi
  case "$mode" in
    untagged) grep -Fq 'not tagged' "$scratch/output" ;;
    dirty) grep -Fq 'dirty' "$scratch/output" ;;
    wrong-version) grep -Fq 'version differ' "$scratch/output" ;;
  esac
  [[ "$(cat "$scratch/repo/target/dist/old")" == 'preserve dist' ]]
done
if (cd "$scratch/repo" && bash scripts/release-build.sh) >"$scratch/output" 2>&1; then fail 'build swallowed cargo failure'; fi
grep -Fq 'building ravel v0.1.0' "$scratch/output"
grep -Fq 'build-boundary' "$scratch/output"
[[ ! -e "$scratch/repo/target/dist/old" ]]
[[ -z "$(find "$scratch/repo/target/dist" -name '*.tar.xz' -print)" ]]
echo 'release build admission tests: all passed'
