#!/usr/bin/env bash
# The incumbent subject for the native-entry campaign: the pre-repair product,
# built as its own frozen tree outside the repaired tree.
#
# The incumbent is git revision REVISION, the parent of the first repair commit.
# It is exported from git, then given only private test seams (seams.patch, test
# builds only), the repaired instrument sources copied verbatim (plus the small
# compile adaptations in instruments.patch), and the baseline instruments under
# scripts/native-incumbent/files. Nothing here changes what the incumbent does in
# a non-test build, but for the package version it pairs with its worker on. Everything this script reads is a campaign subject: the patches
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

# Where the worker is the same program in both trees: its sources and the script
# that compiles it. If either differs, the repaired tree's compiled worker is not
# the incumbent's. build.rs is not here: it compiles nothing, it only computes the
# digest the front compares the worker against, and `verify_pairing` measures that
# digest on the built incumbent instead of reading build.rs.
worker_paths=(crates/ravel/worker crates/ravel/scripts)

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

# Fails unless the incumbent's worker is the repaired tree's: byte for byte.
worker_identity() {
  local pristine="$1" path
  for path in "${worker_paths[@]}"; do
    diff -r -x node_modules "$pristine/$path" "$root/$path" >/dev/null ||
      die "the incumbent's $path differs from the repaired tree; its worker would have to be built separately"
  done
}

# The digest that `dispatch.sh` stamps into the compiled worker, which the front
# must be built to expect. `build_id` in dispatch.sh and `source_files` in
# build.rs define it twice; this asks the one that compiled the worker.
worker_digest() {
  bash "$root/crates/ravel/scripts/dispatch.sh" build-id
}

# Fails unless FILE carries the literal DIGEST: the worker embeds it through a
# Bun --define, the front through `env!("RAVEL_WORKER_BUILD_ID")`.
carries_digest() {
  local file="$1" digest="$2"
  [ -n "$digest" ] || die "no worker digest to look for in $file"
  grep -aqF -- "$digest" "$file"
}

# After the build: the incumbent front expects the digest the shared compiled
# worker carries. This is what the pinned build.rs diff stood in for. The
# incumbent's own build.rs computes the digest, so a build.rs that listed other
# files than dispatch.sh does would show here and nowhere earlier.
verify_pairing() {
  local front="$1" worker="$2" digest
  digest="$(worker_digest)"
  carries_digest "$worker" "$digest" ||
    die "the shared compiled worker does not carry the repaired tree's worker digest $digest; rebuild it (task worker)"
  carries_digest "$front" "$digest" ||
    die "the incumbent front does not expect worker digest $digest; its build.rs computes another, so its worker would have to be built separately"
}

# The workspace package version in a Cargo.toml read from stdin.
workspace_version() {
  awk '/^\[/ { section = $0 }
       section == "[workspace.package]" && /^version = "/ { gsub(/^version = "|"$/, ""); print; exit }'
}

# The versions Cargo.lock records for the two workspace members, one per line.
locked_versions() {
  awk '/^name = "(ravel|keyed-launch)"$/ { member = 1; next }
       member && /^version = "/ { gsub(/^version = "|"$/, ""); print; member = 0 }'
}

# The front pairs with its worker on three things: the protocol, the build digest
# and the package version. The compiled worker is the repaired tree's, so it
# reports the repaired tree's version, while the incumbent's workspace says its
# own (1.0.0 at REVISION). Set the incumbent's version, and the two lock entries
# that follow it, to the repaired tree's, and fail unless both then agree. The
# version is read, not pinned: a release changes it.
align_version() {
  local into="$1" version
  version="$(workspace_version <"$root/Cargo.toml")"
  [ -n "$version" ] || die "no workspace package version in $root/Cargo.toml"
  awk -v version="$version" '
    /^\[/ { section = $0 }
    section == "[workspace.package]" && /^version = "/ { print "version = \"" version "\""; next }
    { print }' "$into/Cargo.toml" >"$into/Cargo.toml.aligned" &&
    mv "$into/Cargo.toml.aligned" "$into/Cargo.toml"
  awk -v version="$version" '
    /^name = "(ravel|keyed-launch)"$/ { member = 1; print; next }
    member && /^version = "/ { print "version = \"" version "\""; member = 0; next }
    { print }' "$into/Cargo.lock" >"$into/Cargo.lock.aligned" &&
    mv "$into/Cargo.lock.aligned" "$into/Cargo.lock"
  [ "$(workspace_version <"$into/Cargo.toml")" = "$version" ] ||
    die "the incumbent's workspace version is not $version after alignment"
  [ "$(locked_versions <"$into/Cargo.lock" | sort -u | tr '\n' ' ')" = "$version " ] ||
    die "the incumbent's Cargo.lock does not record $version for both workspace members"
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
  align_version "$into"
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
    # --features native-faults: the off-by-default seam that makes the runner's
    # child-restoration operations reachable from this fixture (F06); an
    # installed or release build never enables it.
    cargo rustc --locked -p ravel --bin ravel --profile dev --features native-faults \
      --target-dir target/native-entry-fixture -- --cfg test -A unused
    verify_pairing target/native-entry-fixture/debug/ravel target/libexec/ravel/ravel-policy
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

  # A worker that differs from the repaired tree's is refused: a TypeScript
  # source, and the script that compiles it.
  local edited
  for edited in crates/ravel/worker/src/main.ts crates/ravel/scripts/dispatch.sh; do
    export_pristine "$work/worker"
    printf '// changed\n' >>"$work/worker/$edited"
    if (worker_identity "$work/worker") 2>/dev/null; then
      die "selftest: a changed $edited was accepted"
    fi
    rm -rf "$work/worker"
  done
  printf 'selftest ok   a worker source or compile script that differs from the repaired tree (rejected)\n'

  # build.rs is not a worker input: the unmodified incumbent passes, and so does
  # any build.rs, because the digest it computes is measured after the build.
  export_pristine "$work/buildrs"
  (worker_identity "$work/buildrs") ||
    die "selftest: the incumbent's worker was refused although only build.rs differs"
  printf '// changed\n' >>"$work/buildrs/crates/ravel/build.rs"
  (worker_identity "$work/buildrs") ||
    die "selftest: a changed build.rs refused the worker, which build.rs does not compile"
  printf 'selftest ok   a build.rs that differs from the repaired tree (accepted: the digest is measured, not read)\n'

  # The measurement refuses a front or a worker that lacks the digest, including
  # one that differs from it in the last character only.
  local digest
  digest="$(worker_digest)"
  printf 'x%sx' "$digest" >"$work/has"
  printf 'x%sx' "${digest%?}0" >"$work/lacks"
  : >"$work/empty"
  carries_digest "$work/has" "$digest" ||
    die "selftest: a file carrying the worker digest was refused"
  if (carries_digest "$work/lacks" "$digest") 2>/dev/null; then
    die "selftest: a file carrying another worker digest was accepted"
  fi
  if (carries_digest "$work/empty" "$digest") 2>/dev/null; then
    die "selftest: an empty file was accepted as carrying the worker digest"
  fi
  if (carries_digest "$work/has" "") 2>/dev/null; then
    die "selftest: an empty digest was looked for"
  fi
  if (verify_pairing "$work/lacks" "$work/has") 2>/dev/null; then
    die "selftest: a front that does not expect the worker digest was paired"
  fi
  if (verify_pairing "$work/has" "$work/lacks") 2>/dev/null; then
    die "selftest: a worker that lacks the digest was paired"
  fi
  (verify_pairing "$work/has" "$work/has") 2>/dev/null ||
    die "selftest: a front and a worker that both carry the digest were refused"
  printf 'selftest ok   a front or worker without the repaired tree'"'"'s worker digest (rejected), both with it (accepted)\n'

  # The incumbent pairs with the shared worker on the repaired tree's version.
  local version
  version="$(workspace_version <"$root/Cargo.toml")"
  [ "$(workspace_version <"$work/pristine/Cargo.toml")" != "$version" ] ||
    printf 'selftest note the incumbent already has version %s; the alignment below proves nothing\n' "$version"
  [ "$(workspace_version <"$work/one/Cargo.toml")" = "$version" ] &&
    [ "$(locked_versions <"$work/one/Cargo.lock" | sort -u | tr '\n' ' ')" = "$version " ] ||
    die "selftest: the assembled incumbent does not carry the repaired tree's version $version"
  printf 'selftest ok   the incumbent carries the repaired tree'"'"'s version %s in Cargo.toml and Cargo.lock\n' "$version"
  mkdir -p "$work/noversion"
  printf '[workspace.package]\nedition = "2021"\n' >"$work/noversion/Cargo.toml"
  cp "$work/one/Cargo.lock" "$work/noversion/Cargo.lock"
  if (align_version "$work/noversion") 2>/dev/null; then
    die "selftest: an incumbent without a workspace version was aligned"
  fi
  printf 'selftest ok   an incumbent whose workspace version cannot be set (rejected)\n'

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
