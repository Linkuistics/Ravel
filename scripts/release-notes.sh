#!/usr/bin/env bash
# Deterministic fallback prose; explicit reviewed notes replace only Unreleased.
set -euo pipefail
IFS=$'\n\t'
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
die() {
  echo "release-notes: $*" >&2
  exit 1
}
to='@'
input=''
while (($#)); do
  case "$1" in
    --to | --from-file)
      (($# >= 2)) || {
        echo 'usage: release-notes.sh [--to REV] [--from-file BODY.md]' >&2
        exit 2
      }
      if [[ "$1" == --to ]]; then to="$2"; else input="$2"; fi
      shift 2
      ;;
    -h | --help)
      echo 'usage: release-notes.sh [--to REV] [--from-file BODY.md]'
      echo 'Fill empty Unreleased notes from commit descriptions; explicit reviewed prose replaces the body.'
      exit 0
      ;;
    *)
      echo 'usage: release-notes.sh [--to REV] [--from-file BODY.md]' >&2
      exit 2
      ;;
  esac
done
[[ -d .jj ]] || die 'run from a jj workspace'
rev="$(jj log --no-graph -r "$to" -T 'commit_id ++ "\n"')"
[[ "$rev" =~ ^[0-9a-f]+$ ]] || die 'endpoint must resolve to one revision'
mkdir -p target
scratch="$(mktemp -d "$root/target/release-notes.XXXXXX")"
trap 'rm -rf "$scratch"' EXIT
jj file show -r "$rev" root:CHANGELOG.md >"$scratch/original"
cmp -s "$scratch/original" CHANGELOG.md || die 'CHANGELOG.md differs from the endpoint; left unchanged'
awk '/^## Unreleased$/ { n++ } END { exit n != 1 }' CHANGELOG.md || die 'CHANGELOG.md must contain exactly one Unreleased heading'
if [[ -n "$input" ]]; then
  [[ -f "$input" && -r "$input" ]] || die "not a readable notes file: $input"
  cp "$input" "$scratch/body"
else
  if awk '/^## Unreleased$/ { section=1; next } section && /^## / { exit }
      section && /[^[:space:]]/ { found=1 } END { exit !found }' CHANGELOG.md; then
    echo 'release-notes: preserving existing Unreleased notes'
    exit 0
  fi
  id="$(cargo pkgid --quiet -p ravel)"
  version="${id##*[#@]}"
  tag="v$version"
  if [[ -n "$(jj log --no-graph -r "tags(exact:\"$tag\") & ::$rev" -T commit_id)" ]]; then
    interval="$tag..$rev"
  elif [[ "$version" == 0.0.0 && -z "$(git tag --list 'v[0-9]*')" ]]; then
    interval="root()..$rev"
  else
    die "missing baseline $tag; inspect history before writing notes"
  fi
  jj log --no-graph --reversed -r "$interval" \
    -T 'if(!empty && description.first_line() != "", "- " ++ description.first_line() ++ "\n")' >"$scratch/body"
fi
awk '/[^[:space:]]/ { found=1 } END { exit !found }' "$scratch/body" || die 'empty notes body; left unchanged'
awk '/^[[:blank:]]*##([[:space:]]|$)/ { bad=1 } END { exit bad }' "$scratch/body" || die 'notes body contains a forbidden ## heading; left unchanged'
awk -v notes="$scratch/body" '
  function emit(   line, pending, started) {
    print ""
    while ((getline line < notes) > 0) {
      if (line ~ /^[[:space:]]*$/) { pending++; continue }
      if (started) while (pending-- > 0) print ""
      pending=0; started=1; print line
    }
    close(notes)
  }
  /^## Unreleased$/ { print; emit(); section=1; next }
  section && /^## / { section=0; print "" }
  !section { print }
' CHANGELOG.md >"$scratch/changelog"
cmp -s "$scratch/original" CHANGELOG.md || die 'CHANGELOG.md changed during preparation; left unchanged'
mv "$scratch/changelog" CHANGELOG.md
echo "release-notes: filled Unreleased through $to; review prose before cutting"
