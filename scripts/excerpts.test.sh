#!/usr/bin/env bash
# Controls for scripts/excerpts.sh. Every refusal below is made to fire against
# a deliberately wrong subject, and the passing excerpts are the other half of
# each control: a checker that never failed would read the same as one that
# cannot.
#
#   scripts/excerpts.test.sh            run every control against the real checker
#   scripts/excerpts.test.sh mutants    break the checker one safeguard at a time and show the
#                                       control that notices each break
#
# A control that changes a source file does it in a scratch repository under a
# temporary directory, never in the real checkout. The fixtures and mutations
# below are literal Markdown and shell text, quoted on purpose; nothing in a
# single-quoted string here is meant to expand.
# shellcheck disable=SC2016
set -euo pipefail
IFS=$'\n\t'

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
readonly ROOT
WORK="$(mktemp -d "${TMPDIR:-/tmp}/ravel-excerpts-test.XXXXXX")"
readonly WORK
# shellcheck disable=SC2064 # exact owned test directory
trap "rm -rf '$WORK'" EXIT

# The checker and extractor under test: the real ones, or a mutated copy.
SCRIPT="$ROOT/scripts/excerpts.sh"
EXTRACT="$ROOT/scripts/examples-extract.awk"

die() {
  echo "excerpts-test: $*" >&2
  exit 1
}

indent() { sed 's/^/    /'; }

# Run the checker under test against the checkout CHECK_ROOT (default: this one).
checker() {
  EXCERPTS_ROOT="${CHECK_ROOT:-$ROOT}" bash "$SCRIPT" check "$@"
}

# expect_fail NAME PATTERN PAGE...: the checker exits non-zero and says PATTERN.
expect_fail() {
  local name="$1" pattern="$2" output status=0
  shift 2
  output="$(checker "$@" 2>&1)" || status=$?
  if ((status == 0)); then
    echo "control $name: the checker passed a wrong subject" >&2
    return 1
  fi
  if ! grep -qE -- "$pattern" <<<"$output"; then
    echo "control $name: failed, but not for the expected reason (wanted /$pattern/):" >&2
    printf '%s\n' "$output" | indent >&2
    return 1
  fi
}

# expect_pass NAME PATTERN PAGE...: the checker exits zero and says PATTERN.
expect_pass() {
  local name="$1" pattern="$2" output status=0
  shift 2
  output="$(checker "$@" 2>&1)" || status=$?
  if ((status != 0)); then
    echo "control $name: the checker failed a good subject:" >&2
    printf '%s\n' "$output" | indent >&2
    return 1
  fi
  if ! grep -qE -- "$pattern" <<<"$output"; then
    echo "control $name: passed, but the summary lacks /$pattern/:" >&2
    printf '%s\n' "$output" | indent >&2
    return 1
  fi
}

# The errors that the shared extractor reports for a page (the example check's
# side of the convention). Prints nothing for a page it reads cleanly.
extractor_errors() {
  local stage="$WORK/stage-$RANDOM$RANDOM"
  mkdir -p "$stage"
  awk -v out="$stage" -f "$EXTRACT" "$1"
  if [[ -f "$stage/errors" ]]; then cat "$stage/errors"; fi
}

# ------------------------------------------------------------------ fixtures

# excerpt_md PATH FIRST LAST [LANG] [SOURCE-ROOT]: Markdown holding one excerpt,
# lines FIRST to LAST of PATH. In the page that a control writes around it the
# label is line 3 and line K of the excerpt is page line 5 + K.
excerpt_md() {
  local path="$1" first="$2" last="$3" lang="${4:-}" source="${5:-$ROOT}" body
  body="$(
    sed -n "${first},${last}p" "$source/$path"
    printf x
  )"
  body="${body%x}"
  [[ "$body" =~ [^[:space:]] ]] || die "lines $first to $last of $path are blank; a control needs code there"
  printf 'Excerpt from `%s`: lines %s to %s.\n\n```%s\n%s```\n' "$path" "$first" "$last" "$lang" "$body"
}

# page NAME TEXT...: write a page headed by a title, and print its path.
page() {
  local name="$1" file="$WORK/$1.md"
  shift
  {
    printf '# T\n\n'
    printf '%s\n' "$@"
  } >"$file"
  echo "$file"
}

# The page of one excerpt that a control then breaks.
good_page() { # NAME [PATH FIRST LAST LANG]
  page "$1" "$(excerpt_md "${2:-crates/ravel/src/run.rs}" "${3:-2}" "${4:-9}" "${5:-rust}")"
}

# edited NAME FROM-PAGE LINE SUBSTITUTION: a copy of a page with a perl
# substitution on one line. A copy that equals the original would make its
# control pass for the wrong reason, so that stops the run.
edited() {
  local name="$1" from="$2" line="$3" substitution="$4" out="$WORK/$1.md"
  perl -pe "$substitution if \$. == $line" "$from" >"$out"
  cmp -s "$out" "$from" && die "edit $name changed nothing"
  echo "$out"
}

# A scratch repository holding copies of the named checkout files (tracked, as a
# reader's clone would have them). Prints its path.
repo() { # NAME FILE...
  local name="$1" dir="$WORK/repo-$1" file
  shift
  mkdir -p "$dir"
  git -C "$dir" init -q
  for file in "$@"; do
    mkdir -p "$dir/$(dirname "$file")"
    cp "$ROOT/$file" "$dir/$file"
  done
  git -C "$dir" add -A
  echo "$dir"
}

SOURCE=crates/ravel/src/run.rs

# A content line of the excerpt page above (K of 2..8) that is not blank.
nonblank_k() {
  local k
  for k in 2 3 4 5 6 7 8; do
    if [[ -n "$(sed -n "$((k + 1))p" "$ROOT/$SOURCE")" ]]; then
      echo "$k"
      return 0
    fi
  done
  die "no non-blank line to change in $SOURCE"
}

# ---------------------------------------------------------------- the controls

control_languages_pass() {
  local ts rust toml sh
  rust="$(excerpt_md crates/ravel/src/run.rs 2 9 rust)"
  ts="$(excerpt_md crates/ravel/worker/sdk/index.ts 2 9 ts)"
  toml="$(excerpt_md Cargo.toml 2 9 toml)"
  sh="$(excerpt_md scripts/examples.sh 2 9 sh)"
  expect_pass languages_pass '^excerpts: 4 passed, 0 failed; 0 page error\(s\)$' \
    "$(page languages "$rust" "" "$ts" "" "$toml" "" "$sh")"
}

control_chapters_one_to_six_pass() {
  # The chapters that were drafted before the excerpt check hold no excerpt.
  expect_pass chapters_one_to_six '^excerpts: 0 passed, 0 failed; 0 page error\(s\)$' "$ROOT"/docs/book/src/0[1-6]-*.md
}

control_excerpt_byte_changed() {
  local k
  k="$(nonblank_k)"
  expect_fail excerpt_byte_changed "line $k of the excerpt differs from line $((k + 1)) of the file" \
    "$(edited changed-excerpt "$(good_page base)" "$((5 + k))" 's/^./Q/')"
}

control_source_byte_changed() {
  local k dir
  k="$(nonblank_k)"
  dir="$(repo changed-source "$SOURCE")"
  sed -i.bak "$((k + 1))s/^./Q/" "$dir/$SOURCE"
  rm -f "$dir/$SOURCE.bak"
  cmp -s "$dir/$SOURCE" "$ROOT/$SOURCE" && die "the source edit changed nothing"
  CHECK_ROOT="$dir" expect_fail source_byte_changed "line $k of the excerpt differs from line $((k + 1)) of the file" "$(good_page base-s)"
}

control_source_moved_or_deleted() {
  local moved deleted
  moved="$(repo moved "$SOURCE")"
  mkdir -p "$moved/crates/ravel/src2"
  mv "$moved/$SOURCE" "$moved/crates/ravel/src2/run.rs"
  git -C "$moved" add -A
  CHECK_ROOT="$moved" expect_fail source_moved 'the file does not exist in the checkout' "$(good_page base-m)" || return 1
  deleted="$(repo deleted "$SOURCE")"
  git -C "$deleted" rm -q -f "$SOURCE"
  CHECK_ROOT="$deleted" expect_fail source_deleted 'the file does not exist in the checkout' "$(good_page base-d)"
}

control_page_crlf() {
  local out="$WORK/crlf.md"
  sed 's/$/\r/' "$(good_page base-c)" >"$out"
  expect_fail page_crlf 'CRLF line endings' "$out"
}

control_trailing_whitespace() {
  local k
  k="$(nonblank_k)"
  expect_fail trailing_whitespace "line $k of the excerpt differs from line $((k + 1)) of the file" \
    "$(edited trailing "$(good_page base-t)" "$((5 + k))" 's/$/ /')" || return 1
  # The first line is anchored on, so a space there leaves no line to compare.
  expect_fail trailing_whitespace_first 'line 1 of the excerpt is not a whole line of the file' \
    "$(edited trailing-first "$(good_page base-t1)" 6 's/$/ /')"
}

control_label_without_block() {
  # A paragraph between the label and its block.
  expect_fail label_paragraph_between 'the excerpt label has no block: a paragraph comes between' \
    "$(edited between "$(good_page base-l)" 4 's/^$/\nA paragraph that does not belong here.\n/')" || return 1
  # A heading between them.
  expect_fail label_heading_between 'the excerpt label has no block: a heading comes between' \
    "$(edited heading "$(good_page base-h)" 4 's/^$/\n## Next\n/')" || return 1
  # The page ends after the label.
  expect_fail label_page_ends 'the excerpt label has no block: the page ends' \
    "$(page label-only "Excerpt from \`$SOURCE\`: nothing follows.")"
}

control_two_blocks_under_one_label() {
  local code
  code="$(excerpt_md "$SOURCE" 2 9 rust)"
  expect_fail two_blocks 'a second fenced block follows the excerpt with no paragraph between' \
    "$(page two-blocks "$code" '```text' 'a second block' '```')"
}

control_path_forms() {
  local form text
  for form in '../etc/passwd' 'crates/../Cargo.toml' '/etc/hosts' './Cargo.toml' 'crates//ravel' 'crates/' 'a b.rs' 'Cargo*.toml'; do
    text="$(printf 'Excerpt from `%s`: a path that is not plain.\n\n```text\nx\n```\n' "$form")"
    expect_fail "path_form_$form" 'is not a plain repository-relative path' "$(page path-form "$text")" || return 1
  done
}

control_missing_path() {
  expect_fail missing_path 'the file does not exist in the checkout' \
    "$(page missing "$(excerpt_md "$SOURCE" 2 9 rust | sed "s#$SOURCE#crates/ravel/src/no-such-file.rs#")")"
}

control_directory() {
  local text
  text="$(printf 'Excerpt from `crates/ravel/src`: a directory.\n\n```text\nx\n```\n')"
  expect_fail directory 'the path is a directory' "$(page directory "$text")"
}

control_symlink() {
  local dir text
  dir="$(repo symlink "$SOURCE")"
  ln -s run.rs "$dir/crates/ravel/src/link.rs"
  ln -s src "$dir/crates/ravel/lib"
  git -C "$dir" add -A
  text="$(excerpt_md "$SOURCE" 2 9 rust | sed "s#$SOURCE#crates/ravel/src/link.rs#")"
  CHECK_ROOT="$dir" expect_fail symlink 'a symbolic link \(crates/ravel/src/link.rs\)' "$(page symlink "$text")" || return 1
  text="$(excerpt_md "$SOURCE" 2 9 rust | sed "s#$SOURCE#crates/ravel/lib/run.rs#")"
  CHECK_ROOT="$dir" expect_fail symlink_parent 'a symbolic link \(crates/ravel/lib\)' "$(page symlink-parent "$text")"
}

control_untracked() {
  local dir
  dir="$(repo untracked "$SOURCE")"
  cp "$dir/$SOURCE" "$dir/crates/ravel/src/scratch.rs"
  CHECK_ROOT="$dir" expect_fail untracked 'git does not track the file' \
    "$(page untracked "$(excerpt_md "$SOURCE" 2 9 rust | sed "s#$SOURCE#crates/ravel/src/scratch.rs#")")"
}

control_generated_and_grove_paths() {
  local dir path
  dir="$(repo generated "$SOURCE")"
  for path in target/debug/gen.rs crates/ravel/worker/node_modules/pkg/index.js .grove/notes.md; do
    mkdir -p "$dir/$(dirname "$path")"
    cp "$dir/$SOURCE" "$dir/$path"
    git -C "$dir" add -f "$path"
  done
  CHECK_ROOT="$dir" expect_fail generated_target 'a build output directory' \
    "$(page gen-target "$(excerpt_md "$SOURCE" 2 9 rust | sed "s#$SOURCE#target/debug/gen.rs#")")" || return 1
  CHECK_ROOT="$dir" expect_fail generated_node_modules 'a build output directory' \
    "$(page gen-node "$(excerpt_md "$SOURCE" 2 9 rust | sed "s#$SOURCE#crates/ravel/worker/node_modules/pkg/index.js#")")" || return 1
  CHECK_ROOT="$dir" expect_fail grove_path 'inside .grove' \
    "$(page grove "$(excerpt_md "$SOURCE" 2 9 rust | sed "s#$SOURCE#.grove/notes.md#")")"
}

control_not_a_git_checkout() {
  local dir="$WORK/plain"
  mkdir -p "$dir"
  GIT_CEILING_DIRECTORIES="$WORK" CHECK_ROOT="$dir" expect_fail not_git 'is not a git checkout' "$(good_page base-g)"
}

control_substring_of_a_line() {
  local line prefix
  # The first line starts in the middle of a line of the file.
  line="$(sed -n '3p' "$ROOT/$SOURCE")"
  prefix="${line:4}"
  expect_fail substring_first_line 'it occurs inside line 3 of the file, but an excerpt holds whole lines' \
    "$(page substring-first "Excerpt from \`$SOURCE\`: a part of a line." "" '```rust' "$prefix" '```')" || return 1
  # The last line stops in the middle of a line of the file.
  line="$(sed -n '3p' "$ROOT/$SOURCE")"
  expect_fail substring_last_line 'line 2 of the excerpt differs from line 3 of the file' \
    "$(page substring-last "Excerpt from \`$SOURCE\`: a part of a line." "" '```rust' "$(sed -n '2p' "$ROOT/$SOURCE")" "${line:0:10}" '```')"
}

control_final_newline_and_blank_lines() {
  local dir
  dir="$WORK/repo-final"
  mkdir -p "$dir"
  git -C "$dir" init -q
  printf 'alpha\nbeta' >"$dir/no-final-newline.txt"
  printf 'alpha\nbeta\n\n\n' >"$dir/blank-end.txt"
  git -C "$dir" add -A
  # The last line of a file that has no final newline can be quoted.
  CHECK_ROOT="$dir" expect_pass final_newline_absent '^excerpts: 3 passed, 0 failed; 0 page error\(s\)$' \
    "$(page final-ok \
      'Excerpt from `no-final-newline.txt`: the whole file.' '' '```text' 'alpha' 'beta' '```' '' \
      'Excerpt from `no-final-newline.txt`: the last line only.' '' '```text' 'beta' '```' '' \
      'Excerpt from `blank-end.txt`: a line and the blank line after it.' '' '```text' 'beta' '' '```')" || return 1
  # A final newline does not start another line: the file has three lines after
  # "alpha", so one more blank line than the file holds is not in the file.
  CHECK_ROOT="$dir" expect_fail beyond_the_end 'the file ends at line 2, and the excerpt goes on with its line 3' \
    "$(page beyond 'Excerpt from `no-final-newline.txt`: one line too many.' '' '```text' 'alpha' 'beta' '' '```')"
}

control_source_has_crlf() {
  local dir="$WORK/repo-crlf"
  mkdir -p "$dir"
  git -C "$dir" init -q
  printf 'alpha\r\nbeta\r\n' >"$dir/crlf.txt"
  git -C "$dir" add -A
  CHECK_ROOT="$dir" expect_fail source_crlf 'line 1 of the excerpt is not a whole line of the file: "alpha"' \
    "$(page source-crlf 'Excerpt from `crlf.txt`: a file with CRLF line endings.' '' '```text' 'alpha' 'beta' '```')"
}

control_empty_block() {
  expect_fail empty_block 'the excerpt block holds no code' \
    "$(page empty "Excerpt from \`$SOURCE\`: nothing." "" '```rust' '```')" || return 1
  expect_fail blank_block 'the excerpt block holds no code' \
    "$(page blank "Excerpt from \`$SOURCE\`: only a blank line." "" '```rust' '' '```')"
}

control_malformed_labels() {
  local label
  for label in 'Excerpt `crates/ravel/src/run.rs`: no "from".' \
    '**Excerpt from `crates/ravel/src/run.rs`**: emphasis around the label.' \
    'Excerpt from crates/ravel/src/run.rs: no code marks.' \
    'Excerpts from `crates/ravel/src/run.rs`: a plural.' \
    'Excerpt: from the source.'; do
    expect_fail "label_form_$label" 'an excerpt label begins: Excerpt from' \
      "$(page malformed "$label" "" '```rust' 'fn main() {}' '```')" || return 1
  done
}

control_fence_shapes() {
  local dir="$WORK/repo-shapes" tilde quote wrapped info
  mkdir -p "$dir/docs"
  git -C "$dir" init -q
  printf 'Intro\n```sh\nravel start-session\n```\nOutro\n' >"$dir/docs/notes.md"
  mkdir -p "$dir/$(dirname "$SOURCE")"
  cp "$ROOT/$SOURCE" "$dir/$SOURCE"
  git -C "$dir" add -A
  tilde="$(excerpt_md "$SOURCE" 2 5 rust | sed 's/^```rust$/~~~rust/; s/^```$/~~~/')"
  info="$(excerpt_md "$SOURCE" 2 5 rust | sed 's/^```rust$/```rust title="a title"/')"
  quote="$(excerpt_md "$SOURCE" 2 5 rust | sed 's/^/> /')"
  wrapped="$(printf 'Excerpt from `docs/notes.md`: a file that holds a fence.\n\n````markdown\nIntro\n```sh\nravel start-session\n```\nOutro\n````\n')"
  CHECK_ROOT="$dir" expect_pass fence_shapes '^excerpts: 4 passed, 0 failed; 0 page error\(s\)$' \
    "$(page shapes "$tilde" "" "$info" "" "$quote" "" "$wrapped")"
}

control_one_stale_among_good() {
  local good stale out
  good="$(excerpt_md "$SOURCE" 2 9 rust)"
  stale="$(excerpt_md "$SOURCE" 2 9 rust | sed '6s/^.*$/use std::nothing::Here;/')"
  out="$(page mixed "$good" "" "$stale")"
  expect_fail one_stale '^FAIL .*mixed\.md:16$' "$out" || return 1
  expect_fail one_stale_summary '^excerpts: 1 passed, 1 failed; 0 page error\(s\)$' "$out"
}

control_directory_argument_and_missing_page() {
  local dir="$WORK/pages"
  mkdir -p "$dir/sub"
  good_page base-dir >/dev/null
  cp "$WORK/base-dir.md" "$dir/one.md"
  cp "$WORK/base-dir.md" "$dir/sub/two.md"
  expect_pass directory_argument '^excerpts: 2 passed, 0 failed; 0 page error\(s\)$' "$dir" || return 1
  expect_fail missing_page 'no such page or directory' "$WORK/does-not-exist.md"
}

# The example check's side of the convention, read by the same extractor.
control_excerpt_blocks_are_not_examples() {
  local errors
  errors="$(extractor_errors "$(page agree \
    "$(excerpt_md crates/ravel/worker/sdk/index.ts 2 9 ts)" "" \
    "$(excerpt_md scripts/examples.sh 2 9 sh)" "" \
    "$(excerpt_md crates/ravel/src/run.rs 2 9 '')" "" \
    "$(excerpt_md scripts/examples.sh 2 9 console)")")"
  if [[ -n "$errors" ]]; then
    echo "control excerpt_blocks_are_not_examples: the extractor refused a labelled excerpt:" >&2
    printf '%s\n' "$errors" | indent >&2
    return 1
  fi
}

control_other_blocks_fail_closed() {
  local lang errors shown
  for lang in ts sh console shell-session rust toml ''; do
    errors="$(extractor_errors "$(page bare "Prose, not a label." "" "\`\`\`$lang" 'x' '```')")"
    shown="${lang:-untagged}"
    if [[ "$shown" == untagged ]]; then
      grep -q 'a fenced block has no language tag' <<<"$errors" || { echo "control other_blocks_fail_closed: an untagged block was accepted" >&2; return 1; }
    else
      grep -q "a $lang block sits outside a labelled example" <<<"$errors" || { echo "control other_blocks_fail_closed: a bare $lang block was accepted" >&2; return 1; }
    fi
  done
  # An excerpt label does not unlock the next block: a paragraph or a second block ends it.
  errors="$(extractor_errors "$(page next "$(excerpt_md "$SOURCE" 2 9 rust)" "" "The next block has no label." "" '```ts' 'x' '```')")"
  grep -q 'a ts block sits outside a labelled example' <<<"$errors" ||
    { echo "control other_blocks_fail_closed: the block after an excerpt was accepted" >&2; return 1; }
  errors="$(extractor_errors "$(page twin "$(excerpt_md "$SOURCE" 2 9 rust)" '```ts' 'x' '```')")"
  grep -q 'a ts block sits outside a labelled example' <<<"$errors" ||
    { echo "control other_blocks_fail_closed: the second block under one label was accepted" >&2; return 1; }
}

CONTROLS=(
  languages_pass chapters_one_to_six_pass excerpt_byte_changed source_byte_changed
  source_moved_or_deleted page_crlf trailing_whitespace label_without_block
  two_blocks_under_one_label path_forms missing_path directory symlink untracked
  generated_and_grove_paths not_a_git_checkout substring_of_a_line
  final_newline_and_blank_lines source_has_crlf empty_block malformed_labels
  fence_shapes one_stale_among_good directory_argument_and_missing_page
  excerpt_blocks_are_not_examples other_blocks_fail_closed
)

run_controls() {
  local failed=0 name
  for name in "$@"; do
    if "control_$name"; then
      printf 'excerpts-test: ok: %s\n' "$name"
    else
      printf 'excerpts-test: FAILED: %s\n' "$name" >&2
      failed=$((failed + 1))
    fi
  done
  return $((failed > 0))
}

# ------------------------------------------------------------------- mutants
# Each mutant is the checker with one safeguard removed. The named control
# must fail against it, and each is run only against the mutant it should kill.

mutate_script() { # NAME FILE FROM TO: write a mutated copy under WORK/NAME
  local name="$1" file="$2" from="$3" to="$4" dir="$WORK/mutant-$1"
  mkdir -p "$dir"
  cp "$ROOT/scripts/excerpts.sh" "$dir/excerpts.sh"
  cp "$ROOT/scripts/examples-extract.awk" "$dir/examples-extract.awk"
  FROM="$from" TO="$to" perl -0777 -i -pe 'BEGIN { $f = $ENV{FROM}; $t = $ENV{TO} } $n += s/\Q$f\E/$t/; END { exit($n ? 0 : 42) }' "$dir/$file" ||
    die "mutant $name matched nothing in $file"
  echo "$dir"
}

mutants() {
  local killed=0 total=0
  # A control that already fails would "kill" any mutant aimed at it, so every
  # control is first run against the unmutated checker.
  run_controls "${CONTROLS[@]}" >/dev/null 2>&1 || die "a control fails against the unmutated checker; run the controls first"

  # try_mutant NAME FILE FROM TO CONTROL
  try_mutant() {
    local name="$1" file="$2" from="$3" to="$4" control="$5" dir output status=0
    total=$((total + 1))
    dir="$(mutate_script "$name" "$file" "$from" "$to")"
    SCRIPT="$dir/excerpts.sh"
    EXTRACT="$dir/examples-extract.awk"
    output="$(run_controls "$control" 2>&1)" || status=$?
    if ((status == 0)); then
      printf 'excerpts-test: MUTANT SURVIVED: %s (control %s still passed)\n' "$name" "$control" >&2
    else
      killed=$((killed + 1))
      printf 'excerpts-test: mutant killed: %s by control %s\n' "$name" "$control"
      grep -E '^control ' <<<"$output" | head -n 1 | sed 's/^/    /' || true
    fi
  }

  try_mutant symlink-followed excerpts.sh \
    'if [[ -L "$ROOT/$current" ]]; then' 'if false; then' symlink
  try_mutant directory-read excerpts.sh \
    'if [[ -d "$ROOT/$path" ]]; then' 'if false; then' directory
  try_mutant missing-file-read excerpts.sh \
    'if [[ ! -e "$ROOT/$path" ]]; then' 'if false; then' missing_path
  try_mutant untracked-quoted excerpts.sh \
    'if ! checkout_git ls-files --error-unmatch -- "$path" >/dev/null 2>&1; then' 'if false; then' untracked
  try_mutant build-output-quoted excerpts.sh \
    '*/target/* | */node_modules/*)' '*/target-never/* | */node_modules-never/*)' generated_and_grove_paths
  try_mutant grove-quoted excerpts.sh \
    '.grove/*)' '.grove-never/*)' generated_and_grove_paths
  try_mutant git-not-required excerpts.sh \
    'checkout_git rev-parse --git-dir >/dev/null 2>&1 ||' 'true ||' not_a_git_checkout
  try_mutant lines-compared-by-substring excerpts.sh \
    'next unless $source[$start] eq $excerpt[0];
      my $run = 0;
      $run++ while $run <= $#excerpt && $start + $run <= $#source && $source[$start + $run] eq $excerpt[$run];' \
    'next unless index($source[$start], $excerpt[0]) >= 0;
      my $run = 0;
      $run++ while $run <= $#excerpt && $start + $run <= $#source && index($source[$start + $run], $excerpt[$run]) >= 0;' substring_of_a_line
  try_mutant prefix-counts-as-a-match excerpts.sh \
    'exit 0 if $run == @excerpt;' 'exit 0 if $run >= 1;' source_byte_changed
  try_mutant final-newline-makes-a-line excerpts.sh \
    "pop @found if @found && \$found[-1] eq q{};" '1;' final_newline_and_blank_lines
  try_mutant failures-do-not-fail-the-run excerpts.sh \
    '((failed == 0 && errors == 0))' '((errors == 0))' one_stale_among_good
  try_mutant page-errors-do-not-fail-the-run excerpts.sh \
    '((failed == 0 && errors == 0))' '((failed == 0))' label_without_block
  try_mutant bad-page-target-ignored excerpts.sh \
    'die "no such page or directory: $target"' 'true' directory_argument_and_missing_page

  try_mutant excerpt-block-is-an-example examples-extract.awk \
    '  if (fence_x) {
    record_excerpt(text)' '  if (fence_x && 0) {
    record_excerpt(text)' excerpt_blocks_are_not_examples
  try_mutant rust-and-toml-allowed-bare examples-extract.awk \
    'split("text yaml yml' 'split("text rust toml yaml yml' other_blocks_fail_closed
  try_mutant second-block-allowed examples-extract.awk \
    '    if (x_after_block)
' '    if (0)
' two_blocks_under_one_label
  try_mutant label-without-block-allowed examples-extract.awk \
    'if (x_pending) xfail(x_line, "the excerpt label has no block: " reason)' 'if (0) xfail(x_line, "the excerpt label has no block: " reason)' label_without_block
  try_mutant empty-block-allowed examples-extract.awk \
    'if (text ~ /^[ \t\n]*$/) {' 'if (0) {' empty_block
  try_mutant dotdot-allowed examples-extract.awk \
    '|| path ~ /(^|\/)\.\.(\/|$)/ || path ~ /(^|\/)\.(\/|$)/' '|| path ~ /(^|\/)\.(\/|$)/' path_forms
  try_mutant dot-part-allowed examples-extract.awk \
    ' || path ~ /(^|\/)\.(\/|$)/' '' path_forms
  try_mutant absolute-allowed examples-extract.awk \
    'path == "" || path ~ /^\// || path ~ /(^|\/)\.\.(\/|$)/ || path ~ /(^|\/)\.(\/|$)/' 'path == "" || path ~ /(^|\/)\.\.(\/|$)/ || path ~ /(^|\/)\.(\/|$)/' path_forms
  try_mutant odd-characters-allowed examples-extract.awk \
    '|| path !~ /^[A-Za-z0-9._\/-]+$/) {
    x_bad' '|| 0) {
    x_bad' path_forms
  try_mutant crlf-page-allowed examples-extract.awk \
    'if (!crlf_x_reported) {' 'if (0) {' page_crlf
  try_mutant malformed-label-allowed examples-extract.awk \
    'if (!match(first, /^Excerpt from `[^`]*`/)) {' 'if (0) {' malformed_labels

  printf 'excerpts-test: %d of %d mutants killed\n' "$killed" "$total"
  ((killed == total))
}

main() {
  case "${1:-}" in
    "") run_controls "${CONTROLS[@]}" ;;
    mutants) mutants ;;
    *) die "usage: excerpts.test.sh [mutants]" ;;
  esac
}

main "$@"
