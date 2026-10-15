#!/usr/bin/env bash
# The incumbent subject for the native-entry campaign: the pre-repair product,
# built as its own frozen tree outside the repaired tree.
#
# The incumbent is git revision REVISION, the parent of the first repair commit.
# It is exported from git, then given only private test seams (seams.patch, test
# builds only), the repaired instrument sources copied verbatim (plus the small
# compile adaptations in instruments.patch), and the baseline instruments under
# scripts/native-incumbent/files. Nothing here changes what the incumbent does in
# a non-test build. Everything this script reads is a campaign subject: the patches
# and overlay live under scripts/, which the campaign driver digests, and the tree
# and its binaries are digested by `digest` before and after a run.
set -euo pipefail
IFS=$'\n\t'

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
revision="0515419edd29"
here="scripts/native-incumbent"
tree="$root/target/native-incumbent/tree"

# Repaired instrument sources the incumbent builds verbatim (then adapts through
# instruments.patch). Paths are the same in both trees.
instruments=(
  crates/ravel/src/native_entry_tests.rs
  crates/ravel/src/native_entry_tests/faults.rs
  crates/keyed-launch/src/native_barrier.rs
  docs/native-entry-cases.json
)

# Where the worker is the same program in both trees: its sources, its build
# script and the script that compiles it. If any differ, the repaired tree's
# compiled worker is not the incumbent's.
worker_paths=(crates/ravel/worker crates/ravel/scripts crates/ravel/build.rs)

die() {
  printf 'native-incumbent: %s\n' "$*" >&2
  exit 1
}

usage() {
  cat >&2 <<'USAGE'
usage: native-incumbent.sh build [--tree DIR]    export, patch and build the incumbent
       native-incumbent.sh digest --out FILE     digest every incumbent subject
       native-incumbent.sh test --rows DIR       run the baseline instruments, filing
                                                 one record per row under DIR
       native-incumbent.sh patches               rewrite seams.patch and instruments.patch
                                                 from the edited tree
       native-incumbent.sh selftest              failing controls for this script
USAGE
  exit 2
}

# The tree exactly as git has it, nothing applied.
export_pristine() {
  local into="$1"
  git -C "$root" cat-file -e "$revision^{commit}" 2>/dev/null ||
    die "revision $revision is not in this repository"
  mkdir -p "$into"
  git -C "$root" archive "$revision" | tar -x -C "$into"
}

# Fails unless the incumbent's worker is byte-for-byte the repaired tree's.
worker_identity() {
  local pristine="$1" path
  for path in "${worker_paths[@]}"; do
    diff -r -x node_modules "$pristine/$path" "$root/$path" >/dev/null ||
      die "the incumbent's $path differs from the repaired tree; its worker would have to be built separately"
  done
}

apply_patch() {
  local into="$1" patch_file="$2"
  # --fuzz=0: a hunk that moved is a failure to look at, never a silent misplacement.
  patch -p1 --fuzz=0 --no-backup-if-mismatch -s -d "$into" <"$patch_file" ||
    die "$patch_file does not apply to the incumbent tree"
}

# Everything up to, but not including, the compile.
assemble() {
  local into="$1" file
  [ ! -e "$into" ] || die "tree exists (a frozen tree is built once, then removed): $into"
  export_pristine "$into"
  worker_identity "$into"
  apply_patch "$into" "$root/$here/seams.patch"
  cp -R "$root/$here/files/." "$into/"
  for file in "${instruments[@]}"; do
    mkdir -p "$into/$(dirname "$file")"
    cp "$root/$file" "$into/$file"
  done
  apply_patch "$into" "$root/$here/instruments.patch"
}

cmd_build() {
  if [ "${1:-}" = "--tree" ]; then
    [ -n "${2:-}" ] || usage
    tree="$2"
    shift 2
  fi
  [ $# -eq 0 ] || usage
  [ -f "$root/target/libexec/ravel/ravel-policy" ] ||
    die "the compiled worker is missing (run task worker): target/libexec/ravel/ravel-policy"
  # A tree is built once and never edited in place: rebuild from nothing.
  if [ -e "$tree" ]; then
    chmod -R u+w "$tree" 2>/dev/null || true
    rm -rf "$tree"
  fi
  assemble "$tree"
  mkdir -p "$tree/target/libexec/ravel"
  cp "$root/target/libexec/ravel/ravel-policy" "$tree/target/libexec/ravel/ravel-policy"
  (
    cd "$tree"
    # The same isolated target the repaired fixture uses: the extra cfg applies to
    # this one binary only and keeps libtest's signal-owning thread out of it.
    cargo rustc --locked -p ravel --bin ravel --profile dev \
      --target-dir target/native-entry-fixture -- --cfg test -A unused
    cargo test --locked --workspace --no-run
  )
}

# Digest map as one JSON object, keys prefixed so they never collide with the
# repaired tree's own subject keys.
cmd_digest() {
  [ "${1:-}" = "--out" ] && [ -n "${2:-}" ] || usage
  [ -d "$tree" ] || die "no incumbent tree to digest: $tree"
  local listing
  listing="$(mktemp)"
  {
    printf 'revision\t%s\t%s\n' "$revision" "$(git -C "$root" rev-parse "$revision^{tree}")"
    (cd "$tree" && find . -type f -not -path './target/*' -not -name '.DS_Store' | LC_ALL=C sort |
      while IFS= read -r path; do
        printf 'source\t%s\t%s\n' "${path#./}" "$(shasum -a 256 "$path" | cut -d' ' -f1)"
      done)
    local built
    for built in target/native-entry-fixture/debug/ravel target/libexec/ravel/ravel-policy; do
      [ -f "$tree/$built" ] || die "incumbent built subject missing: $built"
      printf 'built\t%s\t%s\n' "$built" "$(shasum -a 256 "$tree/$built" | cut -d' ' -f1)"
    done
    (cd "$tree" && cargo nextest list --locked --workspace --message-format json 2>/dev/null |
      jq -r '."rust-suites"[] | "\(."binary-id")\t\(."binary-path")"' | LC_ALL=C sort |
      while IFS=$'\t' read -r id path; do
        printf 'test-binary\t%s\t%s\n' "$id" "$(shasum -a 256 "$path" | cut -d' ' -f1)"
      done)
  } >"$listing"
  jq -Rn '[inputs | split("\t") | {kind: .[0], name: .[1], sha256: .[2]}]
          | map({key: "incumbent-\(.kind):\(.name)", value: .sha256}) | from_entries' \
    <"$listing" >"$2"
  rm -f "$listing"
}

# The baseline selections: the row instruments and the harness controls. Records
# go to the one directory the driver names; the incumbent's tests are the only
# writers of them for subject `baseline`.
cmd_test() {
  [ "${1:-}" = "--rows" ] && [ -n "${2:-}" ] || usage
  mkdir -p "$2"
  # A record is write-once: a directory that already holds baseline records
  # would refuse the second writer row by row, so refuse it once, up front. The
  # repaired instruments share the directory under other subject names.
  ! compgen -G "$2/*@baseline.json" >/dev/null ||
    die "the rows directory already holds baseline records (one writer): $2"
  [ -d "$tree" ] || die "no incumbent tree: $tree"
  (
    cd "$tree"
    # One invocation, so the campaign's one junit file names every test: the
    # front and worker instruments live in the ravel binary, the runner and
    # terminal instruments in the keyed-launch library.
    K40_ROW_DIR="$2" NEXTEST_PROFILE="${NEXTEST_PROFILE:-default}" \
      cargo nextest run --locked -p ravel --bin ravel -p keyed-launch --lib \
      -E 'test(native_baseline)' --run-ignored all --no-fail-fast
  )
}

# After editing the built tree: regenerate the two patches from it. Product files
# (everything git exports that is not an instrument) become seams.patch; the
# adapted copies of the repaired instruments become instruments.patch.
cmd_patches() {
  [ -d "$tree" ] || die "no incumbent tree: $tree"
  local scratch pristine file
  scratch="$(mktemp -d)"
  pristine="$scratch/pristine"
  export_pristine "$pristine"
  local seams="$scratch/seams"
  mkdir -p "$seams/a" "$seams/b"
  # Candidate seam files: every pristine file the tree changed.
  (cd "$pristine" && find . -type f | LC_ALL=C sort) | while IFS= read -r path; do
    path="${path#./}"
    if [ -f "$tree/$path" ] && ! cmp -s "$pristine/$path" "$tree/$path"; then
      case " ${instruments[*]} " in
        *" $path "*) continue ;;
      esac
      mkdir -p "$seams/a/$(dirname "$path")" "$seams/b/$(dirname "$path")"
      cp "$pristine/$path" "$seams/a/$path"
      cp "$tree/$path" "$seams/b/$path"
    fi
  done
  (cd "$seams" && { diff -ruN a b || true; } | perl -pe 's/^(---|\+\+\+) (\S+)\t.*/$1 $2/') \
    >"$root/$here/seams.patch"
  local adapted="$scratch/instruments"
  mkdir -p "$adapted/a" "$adapted/b"
  for file in "${instruments[@]}"; do
    mkdir -p "$adapted/a/$(dirname "$file")" "$adapted/b/$(dirname "$file")"
    cp "$root/$file" "$adapted/a/$file"
    cp "$tree/$file" "$adapted/b/$file"
  done
  (cd "$adapted" && { diff -ruN a b || true; } | perl -pe 's/^(---|\+\+\+) (\S+)\t.*/$1 $2/') \
    >"$root/$here/instruments.patch"
  rm -rf "$scratch"
  printf 'native-incumbent: wrote %s/seams.patch and instruments.patch\n' "$here"
}

# Failing controls. A guarantee is credited only after the script is seen to
# refuse the input that should break it.
cmd_selftest() {
  local work
  work="$(mktemp -d)"
  trap 'chmod -R u+w "$work" 2>/dev/null; rm -rf "$work"' RETURN

  # Reproducible: two assemblies of the tree are identical file for file.
  assemble "$work/one"
  assemble "$work/two"
  diff -r "$work/one" "$work/two" >/dev/null ||
    die "selftest: two assemblies of the incumbent tree differ"
  printf 'selftest ok   two assemblies of the incumbent tree are identical\n'

  # Only seams: every file the assembly changed from git's export is named here.
  export_pristine "$work/pristine"
  local changed
  changed="$(cd "$work/one" && find . -type f -not -path './target/*' | LC_ALL=C sort |
    while IFS= read -r path; do
      [ -f "$work/pristine/$path" ] && cmp -s "$work/pristine/$path" "$path" || printf '%s\n' "${path#./}"
    done)"
  printf 'selftest ok   %s files differ from git revision %s (seams, overlay, instruments)\n' \
    "$(printf '%s\n' "$changed" | grep -c .)" "$revision"

  # A hunk that no longer applies is refused, not fuzzed into place.
  export_pristine "$work/stale"
  sed 's/pub fn shielded<T>/pub fn shielded_RENAMED<T>/' \
    "$work/stale/crates/ravel/src/cancellation.rs" >"$work/stale/c.rs" &&
    mv "$work/stale/c.rs" "$work/stale/crates/ravel/src/cancellation.rs"
  if (apply_patch "$work/stale" "$root/$here/seams.patch") 2>/dev/null; then
    die "selftest: a seam patch applied to a changed incumbent tree"
  fi
  printf 'selftest ok   a seam patch that no longer applies (rejected)\n'

  # A worker that differs from the repaired tree's is refused.
  export_pristine "$work/worker"
  printf '// changed\n' >>"$work/worker/crates/ravel/build.rs"
  if (worker_identity "$work/worker") 2>/dev/null; then
    die "selftest: a different worker source was accepted"
  fi
  printf 'selftest ok   a worker source that differs from the repaired tree (rejected)\n'

  # An existing tree is never assembled into.
  if (assemble "$work/one") 2>/dev/null; then
    die "selftest: an existing tree was assembled into"
  fi
  printf 'selftest ok   assembling into an existing tree (rejected)\n'
}

[ $# -ge 1 ] || usage
mode="$1"
shift
case "$mode" in
  build) cmd_build "$@" ;;
  digest) cmd_digest "$@" ;;
  test) cmd_test "$@" ;;
  patches) cmd_patches ;;
  selftest) cmd_selftest ;;
  *) usage ;;
esac
