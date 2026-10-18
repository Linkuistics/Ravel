#!/usr/bin/env bash
# Controls for scripts/examples.sh. Every refusal below is made to fire against
# a deliberately wrong subject, and the unchanged fixture pages are the passes:
# a checker that never failed would read the same as one that cannot.
#
#   scripts/examples.test.sh            run every control against the real checker
#   scripts/examples.test.sh mutants    break the checker one safeguard at a time and show the
#                                       control that notices each break
#
# Needs a built front and worker (task build); examples.sh refuses a stale pair.
# The fixtures and mutations below are literal Markdown and shell text, quoted
# on purpose; nothing in a single-quoted string here is meant to expand.
# shellcheck disable=SC2016
set -euo pipefail
IFS=$'\n\t'

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
readonly ROOT
readonly PAGES="$ROOT/scripts/examples-fixtures/pages"
WORK="$(mktemp -d "${TMPDIR:-/tmp}/ravel-examples-test.XXXXXX")"
readonly WORK
# shellcheck disable=SC2064 # exact owned test directory
trap "rm -rf '$WORK'" EXIT

# The checker under test: the real one, or a mutated copy.
SCRIPT="$ROOT/scripts/examples.sh"
EXTRACT="$ROOT/scripts/examples-extract.awk"
FRONT="${RAVEL_FRONT:-$ROOT/target/debug/ravel}"

die() {
  echo "examples-test: $*" >&2
  exit 1
}

indent() { sed 's/^/    /'; }

# Run the checker under test; output and status are left for the caller.
checker() {
  EXAMPLES_ROOT="$ROOT" EXAMPLES_EXTRACT="$EXTRACT" RAVEL_FRONT="${CONTROL_FRONT:-$FRONT}" \
    bash "$SCRIPT" check "$@"
}

# Write a copy of a fixture page with one literal substitution, and refuse to
# go on if the substitution changed nothing: a mutation that matched nothing
# would make its control pass for the wrong reason.
mutated() {
  local name="$1" from="$2" to="$3" source="${4:-pass.md}"
  local out="$WORK/$name.md"
  FROM="$from" TO="$to" perl -0777 -pe 'BEGIN { $f = $ENV{FROM}; $t = $ENV{TO} } $n += s/\Q$f\E/$t/; END { exit($n ? 0 : 42) }' \
    "$PAGES/$source" >"$out" || die "mutation $name matched nothing in $source"
  cmp -s "$out" "$PAGES/$source" && die "mutation $name changed nothing"
  echo "$out"
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

# ---------------------------------------------------------------- the controls

control_pass() {
  expect_pass pass '^examples: 3 passed, 0 failed, 0 skipped; 1 fragment\(s\) not run; 0 page error\(s\)$' "$PAGES/pass.md"
}

control_stdout_changed() {
  local page
  page="$(mutated stdout 'without a final newline**

```text
continue' 'without a final newline**

```text
continued')"
  expect_fail stdout_changed 'standard output of command 1 differs' "$page"
}

control_status_changed() {
  local page
  page="$(mutated status '**Exit status**

```text
0
```

## A refusal' '**Exit status**

```text
1
```

## A refusal')"
  expect_fail status_changed 'exit status of command 1 is 0, the example claims 1' "$page"
}

control_no_expected_output() {
  local page
  page="$(mutated nooutput '**Standard output, without a final newline**

```text
continue
```

' '')"
  expect_fail no_expected_output 'has no expected standard output block' "$page"
}

control_no_exit_status() {
  local page
  page="$(mutated nostatus '**Exit status**

```text
3
```
' '')"
  expect_fail no_exit_status 'has no Exit status block' "$page"
}

control_missing_fake_program() {
  # The caller's PATH holds a real-looking `claude` that records being run; the
  # example, which no longer supplies its own, must fail rather than find it.
  local page canary="$WORK/canary-bin" marker="$WORK/canary-ran"
  mkdir -p "$canary"
  printf '#!/bin/sh\necho ran >%s\nexit 0\n' "$marker" >"$canary/claude"
  chmod +x "$canary/claude"
  page="$(mutated nofake '**`bin/claude`**

```sh
#!/bin/sh
pwd >&2
ravel end-session -- "continue"
```

' '')"
  PATH="$canary:$PATH" expect_fail missing_fake_program 'FAIL ' "$page" ||
    return 1
  if [[ -e "$marker" ]]; then
    echo "control missing_fake_program: the caller's own claude on PATH was run" >&2
    return 1
  fi
}

control_stale_front() {
  # A front copy whose embedded worker digest is another one: what a front
  # built before the last worker source change looks like.
  local dir="$WORK/stale-front" id other real
  real="$(cd "$(dirname "$FRONT")" && pwd -P)"
  id="$(bash "$ROOT/crates/ravel/scripts/dispatch.sh" build-id)"
  other="$(printf '%s' "$id" | tr '0-9a-f' '1-9a-f0')"
  [[ "$other" != "$id" ]] || die "cannot derive a different digest"
  mkdir -p "$dir/bin" "$dir/libexec/ravel"
  ID="$id" OTHER="$other" perl -0777 -pe 's/\Q$ENV{ID}\E/$ENV{OTHER}/g' "$real/ravel" >"$dir/bin/ravel"
  chmod +x "$dir/bin/ravel"
  cp "$real/../libexec/ravel/ravel-policy" "$dir/libexec/ravel/ravel-policy"
  CONTROL_FRONT="$dir/bin/ravel" expect_fail stale_front 'stale build: the front' "$PAGES/pass.md" || return 1

  # And the worker: the same refusal from its side.
  local wdir="$WORK/stale-worker"
  mkdir -p "$wdir/bin" "$wdir/libexec/ravel"
  cp "$real/ravel" "$wdir/bin/ravel"
  ID="$id" OTHER="$other" perl -0777 -pe 's/\Q$ENV{ID}\E/$ENV{OTHER}/g' "$real/../libexec/ravel/ravel-policy" >"$wdir/libexec/ravel/ravel-policy"
  chmod +x "$wdir/libexec/ravel/ravel-policy"
  CONTROL_FRONT="$wdir/bin/ravel" expect_fail stale_worker 'stale build: the worker' "$PAGES/pass.md"
}

control_missing_front() {
  CONTROL_FRONT="$WORK/no-such-ravel" expect_fail missing_front 'no built ravel at' "$PAGES/pass.md"
}

control_unlabelled_runnable_block() {
  local page="$WORK/unlabelled.md"
  { cat "$PAGES/pass.md"; printf '\nA command a reader will copy, with no label:\n\n```sh\nravel start-session --kind check\n```\n'; } >"$page"
  expect_fail unlabelled_runnable_block 'a sh block sits outside a labelled example' "$page"
}

control_unlabelled_block_in_example() {
  local page
  page="$(mutated nolabel '**`bin/claude`**

' '')"
  expect_fail unlabelled_block_in_example 'has no label paragraph directly before it' "$page"
}

control_unknown_label() {
  local page
  page="$(mutated badlabel '**Exit status**

```text
3' '**Return code**

```text
3')"
  expect_fail unknown_label 'unknown label "Return code"' "$page"
}

control_stderr_changed() {
  local page
  page="$(mutated stderr 'no route for this kind
  input' 'no route for these kinds
  input')"
  expect_fail stderr_changed 'standard error of command 1 differs' "$page"
}

control_stderr_partial_is_not_exact() {
  # Exact means the whole stream: dropping the last line is a difference.
  local page
  page="$(mutated partial '
  remedy: choose a kind the policy routes
```' '
```')"
  expect_fail stderr_partial_is_not_exact 'standard error of command 1 differs' "$page"
}

control_stderr_includes_absent() {
  local page
  page="$(mutated includes 'effort fixture for kind "check"' 'effort fixture for kind "other"')"
  expect_fail stderr_includes_absent 'does not contain the text the example claims' "$page"
}

control_newline_mode() {
  # `continue` is published with no newline; a block that claims one differs.
  local page
  page="$(mutated newline '**Standard output, without a final newline**' '**Standard output**')"
  expect_fail newline_mode 'standard output of command 1 differs' "$page" || return 1
  page="$(mutated newline2 '**Standard output**

```text
one' '**Standard output, without a final newline**

```text
one')"
  expect_fail newline_mode_2 '^examples: 2 passed, 1 failed' "$page"
}

control_only_the_scratch_root_is_normalised() {
  local page
  page="$(mutated rawpath '<scratch>/work
```' '/tmp/work
```')"
  expect_fail only_the_scratch_root_is_normalised 'does not contain the text the example claims' "$page"
}

control_skip_is_not_a_pass() {
  local output status=0
  output="$(EXAMPLES_ASSUME_MISSING=confinement checker "$PAGES/skip.md" 2>&1)" || status=$?
  if ((status != 0)) || ! grep -qE '^examples: 0 passed, 0 failed, 1 skipped;' <<<"$output" ||
    ! grep -qE '^skip .*skip.md:[0-9]+: confinement is assumed missing' <<<"$output"; then
    echo "control skip_is_not_a_pass: a skipped example was not reported as skipped with its reason:" >&2
    printf '%s\n' "$output" | indent >&2
    return 1
  fi
  # Where the feature is present the same example runs, and its wrong claim fails.
  # A separate shell: the checker's constants are readonly.
  if ! bash -c 'source "$1"; feature_missing confinement >/dev/null' _ "$SCRIPT"; then
    expect_fail skip_runs_where_available 'FAIL .*skip.md' "$PAGES/skip.md"
  fi
}

control_unknown_requirement() {
  local page
  page="$(mutated unknownreq 'requires confinement' 'requires gpu' skip.md)"
  expect_fail unknown_requirement 'unknown requirement "gpu"' "$page"
}

control_file_after_command() {
  local page="$WORK/late-file.md"
  { cat "$PAGES/leak.md"; printf '\n**`late.txt`**\n\n```text\nlate\n```\n'; } >"$page"
  expect_fail file_after_command 'comes after a Command block' "$page"
}

control_path_escape() {
  local page
  page="$(mutated escape '**`policy.ts`**

```ts
export const policy' '**`../policy.ts`**

```ts
export const policy')"
  expect_fail path_escape 'is not a plain relative path' "$page"
}

control_duplicate_file() {
  local page
  page="$(mutated duplicate '**`bin/claude`**' '**`policy.ts`**')"
  expect_fail duplicate_file 'is given twice' "$page"
}

control_timeout() {
  EXAMPLES_TIMEOUT=1 expect_fail timeout 'command 1 timed out after 1 seconds' "$PAGES/slow.md" || return 1
  # The status a killed command leaves is not a claim that can excuse it.
  local page
  page="$(mutated slowclaim '```text
0
```' '```text
137
```' slow.md)"
  EXAMPLES_TIMEOUT=1 expect_fail timeout_cannot_be_claimed 'command 1 timed out' "$page"
}

control_environment_is_cleared() {
  # Standard input is a pipe holding text, as it would be for a caller in a
  # pipeline; the fixture's `cat` must still read nothing.
  printf 'leaked stdin\n' | LEAK_CANARY=1 EXAMPLES_ROOT="$ROOT" EXAMPLES_EXTRACT="$EXTRACT" RAVEL_FRONT="$FRONT" \
    bash "$SCRIPT" check "$PAGES/leak.md" >"$WORK/leak.out" 2>&1 || {
    echo "control environment_is_cleared: the seam leaked the caller's environment or standard input:" >&2
    indent <"$WORK/leak.out" >&2
    return 1
  }
  grep -q '^examples: 3 passed, 0 failed' "$WORK/leak.out"
}

control_nothing_personal_is_read() {
  # A caller whose HOME holds an invalid ravel settings file (read, it refuses
  # every session as settings_invalid) and whose PATH leads with a `claude` and
  # a `ravel` that record being run. The passing fixture must still pass, and
  # neither program may have run.
  local home="$WORK/poison-home" bin="$WORK/poison-bin" marker="$WORK/poison-ran"
  mkdir -p "$home/.config/ravel" "$bin"
  printf '{ not json' >"$home/.config/ravel/settings.json"
  local tool
  for tool in claude codex ravel; do
    printf '#!/bin/sh\necho %s >>%s\nexit 99\n' "$tool" "$marker" >"$bin/$tool"
    chmod +x "$bin/$tool"
  done
  HOME="$home" PATH="$bin:$PATH" expect_pass nothing_personal_is_read '^examples: 3 passed, 0 failed' "$PAGES/pass.md" ||
    return 1
  if [[ -e "$marker" ]]; then
    echo "control nothing_personal_is_read: a program from the caller's PATH ran: $(cat "$marker")" >&2
    return 1
  fi
}

control_empty_home_and_fakes_only() {
  # The Done-when's own configuration: an empty HOME and the examples' PATH
  # holding only the fakes the pages supply. The checker's tools stay on the
  # outer PATH, which the examples never see.
  local home="$WORK/empty-home" fakes="$WORK/only-fakes"
  mkdir -p "$home" "$fakes"
  for tool in claude codex; do
    printf '#!/bin/sh\nexit 99\n' >"$fakes/$tool"
    chmod +x "$fakes/$tool"
  done
  HOME="$home" PATH="$fakes:/usr/bin:/bin" expect_pass empty_home_and_fakes_only '^examples: 3 passed, 0 failed' "$PAGES/pass.md"
}

control_directory_and_two_pages() {
  local good="$WORK/dir"
  mkdir -p "$good/sub"
  cp "$PAGES/pass.md" "$good/pass.md"
  cp "$PAGES/leak.md" "$good/sub/leak.md"
  expect_pass directory_pass '^examples: 6 passed, 0 failed' "$good" || return 1
  cp "$(mutated twopages 'no route for this kind
  input' 'no route for these kinds
  input')" "$good/sub/broken.md"
  expect_fail directory_one_bad 'FAIL .*broken.md' "$good"
}

control_a_page_with_no_examples() {
  local page="$WORK/prose.md"
  printf '# Prose\n\nFragment: nothing runs.\n\n```ts\nconst a = 1;\n```\n' >"$page"
  expect_pass a_page_with_no_examples '^examples: 0 passed, 0 failed, 0 skipped; 1 fragment' "$page"
}

# Page shapes the extractor must read the same way Markdown does, and shapes
# that would hide a runnable block and so must be refused.

control_shapes_are_read() {
  expect_pass shapes_are_read '^examples: 3 passed, 0 failed, 0 skipped; 1 fragment' "$PAGES/shapes.md"
}

control_fragment_covers_one_block() {
  local page="$WORK/fragment-two.md"
  printf '# T\n\nFragment: the first block only.\n\n```sh\nravel one\n```\n\n```sh\nravel two\n```\n' >"$page"
  expect_fail fragment_covers_one_block 'a sh block sits outside a labelled example' "$page" || return 1
  page="$WORK/fragment-prose.md"
  printf '# T\n\nFragment: a block, after some prose.\n\nProse in between.\n\n```sh\nravel one\n```\n' >"$page"
  expect_fail fragment_label_is_cleared_by_prose 'a sh block sits outside a labelled example' "$page"
}

control_prose_between_label_and_block() {
  local page
  page="$(mutated prose '**Exit status**

```text
3' '**Exit status**

Some prose between the label and its block.

```text
3')"
  expect_fail prose_between_label_and_block 'has no label paragraph directly before it' "$page"
}

control_unknown_and_bare_languages() {
  local lang page
  for lang in shell-session Bash fish rust toml ts ''; do
    page="$WORK/lang-$lang.md"
    printf '# T\n\nProse.\n\n```%s\nravel start-session\n```\n' "$lang" >"$page"
    if [[ -z "$lang" ]]; then
      expect_fail language_bare 'a fenced block has no language tag' "$page" || return 1
    else
      expect_fail "language_$lang" "a $(tr '[:upper:]' '[:lower:]' <<<"$lang") block sits outside a labelled example" "$page" || return 1
    fi
  done
  page="$WORK/indented-transcript.md"
  printf '# T\n\nRun it:\n\n    $ ravel start-session --kind check\n' >"$page"
  expect_fail indented_transcript 'an indented code block starting with "\$ "' "$page"
}

# The excerpt convention (docs/book/excerpts.md) shares this extractor: a block
# under an excerpt label is read by scripts/excerpts.sh and is not an example,
# so it needs no language on the never-an-example list. A scratch page holds an
# example, a fragment and a TypeScript excerpt of a real file; both checks pass
# it. It is built from the file as it is now, so it cannot go stale.
excerpt_page() {
  local out="$WORK/excerpt-page.md" source=crates/ravel/worker/sdk/index.ts
  cp "$PAGES/pass.md" "$out"
  {
    printf '\n## A source excerpt\n\nExcerpt from `%s`: the first lines of the file.\n\n```ts\n' "$source"
    sed -n '2,9p' "$ROOT/$source"
    printf '```\n'
  } >>"$out"
  echo "$out"
}

control_excerpt_blocks_agree() {
  local page output status=0
  page="$(excerpt_page)"
  expect_pass excerpt_page_examples '^examples: 3 passed, 0 failed, 0 skipped; 1 fragment\(s\) not run; 0 page error\(s\)$' "$page" || return 1
  output="$(bash "$ROOT/scripts/excerpts.sh" check "$page" 2>&1)" || status=$?
  if ((status != 0)) || ! grep -qE '^excerpts: 1 passed, 0 failed; 0 page error\(s\)$' <<<"$output"; then
    echo "control excerpt_blocks_agree: the excerpt check does not pass the page:" >&2
    printf '%s\n' "$output" | indent >&2
    return 1
  fi
}

control_excerpt_does_not_unlock_other_blocks() {
  local page="$WORK/excerpt-then-bare.md"
  {
    cat "$(excerpt_page)"
    printf '\nA paragraph, so the next block has no label.\n\n```ts\nconst bare = 1;\n```\n'
  } >"$page"
  expect_fail excerpt_then_bare_ts 'a ts block sits outside a labelled example' "$page" || return 1
  {
    cat "$(excerpt_page)"
    printf '\n```sh\nravel --version\n```\n'
  } >"$page"
  expect_fail excerpt_then_second_block 'a sh block sits outside a labelled example' "$page"
}

control_list_item_fences() {
  local page="$WORK/list-fences.md"
  printf '# T\n\n- ```sh\n  ravel a\n  ```\n- ```sh\n  ravel b\n  ```\n\nProse.\n\n```sh\nravel UNLABELLED\n```\n' >"$page"
  expect_fail list_item_fences 'a fence opened on a list-item line' "$page"
}

control_crlf_is_refused() {
  local page="$WORK/crlf.md"
  sed 's/$/\r/' "$PAGES/pass.md" >"$page"
  expect_fail crlf_is_refused 'CRLF line endings' "$page"
}

control_duplicate_result_blocks() {
  local page
  page="$(mutated dupstatus '**Exit status**

```text
3
```' '**Exit status**

```text
9
```

**Exit status**

```text
3
```')"
  expect_fail duplicate_exit_status 'has two Exit status blocks' "$page" || return 1
  page="$(mutated dupstdout '**Standard output**

```text
one' '**Standard output**

```text
wrong
```

**Standard output**

```text
one')"
  expect_fail duplicate_standard_output 'has two standard output blocks' "$page" || return 1
  page="$(mutated dupstderr '**Standard error includes**' '**Standard error**

```text
```

**Standard error includes**')"
  expect_fail duplicate_standard_error 'has two standard error blocks' "$page"
}

control_empty_includes() {
  local page
  page="$(mutated emptyincludes '**Standard error includes**

```text
ravel: starting provider fixture, model fixture, effort fixture for kind "check"
<scratch>/work
```' '**Standard error includes**

```text
```')"
  expect_fail empty_includes 'an empty Standard error includes block claims nothing' "$page"
}

control_commands_run_in_numeric_order() {
  # Eleven commands, each appending its number to a file in the working
  # directory and the last printing the file. Run lexically (1, 10, 11, 2, …)
  # the file reads differently.
  local page="$WORK/eleven.md" n
  {
    printf '# T\n\nComplete example: eleven commands that share a directory.\n'
    for n in 1 2 3 4 5 6 7 8 9 10 11; do
      printf '\n**Command**\n\n```console\n$ printf '"'"'%s '"'"' >>order\n```\n\n**Standard output**\n\n```text\n```\n\n**Exit status**\n\n```text\n0\n```\n' "$n"
    done
    printf '\n**Command**\n\n```console\n$ cat order\n```\n\n**Standard output, without a final newline**\n\n```text\n1 2 3 4 5 6 7 8 9 10 11 \n```\n\n**Exit status**\n\n```text\n0\n```\n'
  } >"$page"
  expect_pass commands_run_in_numeric_order '^examples: 1 passed, 0 failed' "$page"
}

control_a_label_cannot_run_a_command() {
  # A quote in a file label once reached a shell during extraction.
  # The temporary directory's own path can hold "//", which the path check
  # would refuse for that reason alone; the injected command names a clean one.
  local victim="${WORK//\/\//\/}/pwned" page="$WORK/inject.md"
  {
    printf '# T\n\nComplete example: a hostile file label.\n\n'
    printf '**`d'"'"'$(touch %s)'"'"'/f`**\n\n```text\nx\n```\n' "$victim"
    printf '\n**Command**\n\n```console\n$ true\n```\n\n**Standard output**\n\n```text\n```\n\n**Exit status**\n\n```text\n0\n```\n'
  } >"$page"
  expect_fail a_label_cannot_run_a_command 'is not a plain relative path' "$page" || return 1
  if [[ -e "$victim" ]]; then
    echo "control a_label_cannot_run_a_command: the label's command ran" >&2
    return 1
  fi
}

control_stale_source() {
  # A front and worker with today's digest but older than the Rust and worker
  # sources: a build made before the last edit that did not touch the digest.
  local dir="$WORK/old-build" real
  real="$(cd "$(dirname "$FRONT")" && pwd -P)"
  mkdir -p "$dir/bin" "$dir/libexec/ravel"
  cp "$real/ravel" "$dir/bin/ravel"
  cp "$real/../libexec/ravel/ravel-policy" "$dir/libexec/ravel/ravel-policy"
  touch -t 200001010000 "$dir/bin/ravel" "$dir/libexec/ravel/ravel-policy"
  CONTROL_FRONT="$dir/bin/ravel" expect_fail stale_source 'stale build: .* is newer than the built front or worker' "$PAGES/pass.md"
}

control_cancellation_is_deterministic() {
  # A cancellation by signal that the fake agent or the policy itself sends the
  # front runs without a terminal, and gives the same claim every time. The
  # signal number is part of the claim: SIGINT is 130, not 143.
  local i
  for i in 1 2 3 4 5; do
    expect_pass "cancellation_is_deterministic (run $i)" '^examples: 2 passed, 0 failed' "$PAGES/cancel.md" || return 1
  done
  local page
  page="$(mutated sigterm '```text
130
```

Complete example' '```text
143
```

Complete example' cancel.md)"
  expect_fail cancellation_status_is_claimed 'exit status of command 1 is 130, the example claims 143' "$page"
}

control_seam_matches_the_cli_tests() {
  # The seam is written twice, in cli.rs's Fixture and here. Both must still say
  # the same thing, and this fails when either side is edited alone.
  local cli="${1:-$ROOT/crates/ravel/tests/cli.rs}" path_tail
  path_tail="$(sed -n 's/^readonly SEAM_PATH_TAIL="\(.*\)"$/\1/p' "$SCRIPT")"
  [[ -n "$path_tail" ]] || { echo "control seam: no SEAM_PATH_TAIL in the checker" >&2; return 1; }
  local line
  for line in '.env_clear()' \
    '.env("HOME", self.root.join("home"))' \
    '.env("TMPDIR", self.root.join("tmp"))' \
    ".env(\"PATH\", \"$path_tail\")"; do
    if ! grep -qF -- "$line" "$cli"; then
      echo "control seam: $cli's Fixture no longer has \`$line\`, which the checker's seam repeats" >&2
      return 1
    fi
  done
}

control_seam_drift_is_noticed() {
  local drifted="$WORK/cli-drifted.rs"
  sed 's#"/usr/bin:/bin"#"/usr/bin:/bin:/opt/bin"#' "$ROOT/crates/ravel/tests/cli.rs" >"$drifted"
  cmp -s "$drifted" "$ROOT/crates/ravel/tests/cli.rs" && die "the drift mutation changed nothing"
  if control_seam_matches_the_cli_tests "$drifted" 2>/dev/null; then
    echo "control seam_drift_is_noticed: a drifted Fixture was accepted" >&2
    return 1
  fi
}

CONTROLS=(
  pass stdout_changed status_changed no_expected_output no_exit_status
  missing_fake_program stale_front missing_front unlabelled_runnable_block
  unlabelled_block_in_example unknown_label stderr_changed
  stderr_partial_is_not_exact stderr_includes_absent newline_mode
  only_the_scratch_root_is_normalised skip_is_not_a_pass unknown_requirement
  file_after_command path_escape duplicate_file timeout environment_is_cleared
  nothing_personal_is_read empty_home_and_fakes_only directory_and_two_pages
  a_page_with_no_examples shapes_are_read fragment_covers_one_block
  prose_between_label_and_block unknown_and_bare_languages list_item_fences
  crlf_is_refused duplicate_result_blocks
  excerpt_blocks_agree excerpt_does_not_unlock_other_blocks
  empty_includes commands_run_in_numeric_order
  a_label_cannot_run_a_command stale_source
  cancellation_is_deterministic seam_matches_the_cli_tests seam_drift_is_noticed
)

run_controls() {
  local failed=0 name
  for name in "$@"; do
    if "control_$name"; then
      printf 'examples-test: ok: %s\n' "$name"
    else
      printf 'examples-test: FAILED: %s\n' "$name" >&2
      failed=$((failed + 1))
    fi
  done
  return $((failed > 0))
}

# ------------------------------------------------------------------- mutants
# Each mutant is the checker with one safeguard removed. The named controls
# must fail against it, and each is run only against the mutant it should kill.

mutate_script() { # NAME FILE FROM TO: write a mutated copy under WORK/NAME
  local name="$1" file="$2" from="$3" to="$4" dir="$WORK/mutant-$1"
  mkdir -p "$dir"
  cp "$ROOT/scripts/examples.sh" "$dir/examples.sh"
  cp "$ROOT/scripts/examples-extract.awk" "$dir/examples-extract.awk"
  FROM="$from" TO="$to" perl -0777 -i -pe 'BEGIN { $f = $ENV{FROM}; $t = $ENV{TO} } $n += s/\Q$f\E/$t/; END { exit($n ? 0 : 42) }' "$dir/$file" ||
    die "mutant $name matched nothing in $file"
  echo "$dir"
}

mutants() {
  local killed=0 total=0
  # A control that already fails would "kill" any mutant aimed at it, so each
  # named control is first run against the unmutated checker.
  local baseline=(status_changed stdout_changed stderr_includes_absent stale_front stale_source
    unknown_and_bare_languages environment_is_cleared skip_is_not_a_pass timeout
    unlabelled_runnable_block pass commands_run_in_numeric_order fragment_covers_one_block
    prose_between_label_and_block shapes_are_read
    list_item_fences duplicate_result_blocks crlf_is_refused a_label_cannot_run_a_command
    excerpt_blocks_agree excerpt_does_not_unlock_other_blocks)
  run_controls "${baseline[@]}" >/dev/null 2>&1 || die "a control fails against the unmutated checker; run the controls first"

  # try_mutant NAME FILE FROM TO CONTROL
  try_mutant() {
    local name="$1" file="$2" from="$3" to="$4" control="$5" dir output status=0
    total=$((total + 1))
    dir="$(mutate_script "$name" "$file" "$from" "$to")"
    SCRIPT="$dir/examples.sh"
    EXTRACT="$dir/examples-extract.awk"
    output="$(run_controls "$control" 2>&1)" || status=$?
    if ((status == 0)); then
      printf 'examples-test: MUTANT SURVIVED: %s (control %s still passed)\n' "$name" "$control" >&2
    else
      killed=$((killed + 1))
      printf 'examples-test: mutant killed: %s by control %s\n' "$name" "$control"
      grep -E '^control ' <<<"$output" | head -n 1 | sed 's/^/    /' || true
    fi
  }

  try_mutant status-ignored examples.sh \
    'elif [[ "$(cat "$run/status")" != "$(cat "$status")" ]]; then' 'elif false; then' status_changed
  try_mutant stdout-ignored examples.sh \
    'if ! cmp -s "$expected" "$actual"; then' 'if false; then' stdout_changed
  try_mutant includes-always examples.sh \
    'if ! perl -0777 -e' 'if false && ! perl -0777 -e' stderr_includes_absent
  try_mutant stale-front-accepted examples.sh \
    'grep -aqF -- "$id" "$front" ||' 'true ||' stale_front
  try_mutant stale-source-accepted examples.sh \
    '[[ -z "$newer" ]] ||' 'true ||' stale_source
  try_mutant unlabelled-allowed examples-extract.awk \
    'else if (!(lang in nonrunnable))' 'else if (0)' unknown_and_bare_languages
  try_mutant rust-and-toml-allowed-bare examples-extract.awk \
    'split("text yaml yml' 'split("text rust toml yaml yml' unknown_and_bare_languages
  try_mutant excerpt-block-is-an-example examples-extract.awk \
    $'  if (fence_x) {\n    record_excerpt(text)' $'  if (fence_x && 0) {\n    record_excerpt(text)' excerpt_blocks_agree
  try_mutant excerpt-label-unlocks-every-block examples-extract.awk \
    $'    if (x_pending) {\n      fence_x = 1' $'    if (1) {\n      fence_x = 1' excerpt_does_not_unlock_other_blocks
  try_mutant bare-fence-allowed examples-extract.awk \
    $'    if (lang == "")\n' $'    if (0)\n' unknown_and_bare_languages
  try_mutant environment-inherited examples.sh \
    $'env -i \\' $'env \\' environment_is_cleared
  try_mutant home-removed examples.sh \
    $'      HOME="$scratch/home" \\\n' '' environment_is_cleared
  try_mutant tmpdir-removed examples.sh \
    $'      TMPDIR="$scratch/tmp" \\\n' '' environment_is_cleared
  try_mutant stdin-inherited examples.sh \
    'exec </dev/null' 'true' environment_is_cleared
  try_mutant skip-counted-as-pass examples.sh \
    'skip*) skipped=$((skipped + 1)) ;;' 'skip*) passed=$((passed + 1)) ;;' skip_is_not_a_pass
  try_mutant no-timeout examples.sh \
    'alarm $limit;' 'alarm 0;' timeout
  try_mutant page-errors-ignored examples.sh \
    '((failed == 0 && errors == 0))' '((failed == 0))' unlabelled_runnable_block
  try_mutant normalisation-disabled examples.sh \
    $'perl -0777 -pe \'s/\\Q$ENV{PHYSICAL_ROOT}\\E/<scratch>/g; s/\\Q$ENV{CREATED_ROOT}\\E/<scratch>/g\'' \
    $'perl -0777 -pe \'1\'' pass
  try_mutant lexical-run-order examples.sh \
    'run="$exdir/run-$n"' 'run="$(ls -d "$exdir"/run-* | sed -n "${n}p")"' commands_run_in_numeric_order
  try_mutant fragment-covers-every-block examples-extract.awk \
    'if (fragment_pending) { fragment_pending = 0; return }' 'if (fragment_pending) { return }' fragment_covers_one_block
  try_mutant prose-keeps-label examples-extract.awk \
    $'  } else {\n    pending = ""\n  }\n  para = ""' $'  } else {\n  }\n  para = ""' prose_between_label_and_block
  try_mutant blockquote-ignored examples-extract.awk \
    'while (match(s, /^ ? ? ?> ?/)) {' 'while (0) {' shapes_are_read
  try_mutant quote-depth-not-kept examples-extract.awk \
    'for (i = 0; i < n; i++)' 'for (i = 0; i < 9; i++)' shapes_are_read
  try_mutant tilde-fence-ignored examples-extract.awk \
    'if (match(line, /^ ? ? ?(```+|~~~+)/)) {' 'if (match(line, /^ ? ? ?(```+)/)) {' shapes_are_read
  try_mutant closer-length-ignored examples-extract.awk \
    'length(closer) >= fence_len' 'length(closer) >= 3' shapes_are_read
  try_mutant list-fence-allowed examples-extract.awk \
    'if (line ~ /^ ? ? ?([-*+]|[0-9]+[.)])[ \t]+(```|~~~)/) {' 'if (0) {' list_item_fences
  try_mutant duplicate-status-allowed examples-extract.awk \
    'if (has_status[runs]) {' 'if (0) {' duplicate_result_blocks
  try_mutant crlf-allowed examples-extract.awk \
    'if (!crlf_reported) {' 'if (0) {' crlf_is_refused
  try_mutant label-path-unchecked examples-extract.awk \
    $'|| path !~ /^[A-Za-z0-9._\\/-]+$/) {\n      fail(start, "file label' $'|| 0) {\n      fail(start, "file label' a_label_cannot_run_a_command

  printf 'examples-test: %d of %d mutants killed\n' "$killed" "$total"
  ((killed == total))
}

main() {
  [[ -x "$FRONT" ]] || die "no built ravel at $FRONT; run task build"
  case "${1:-}" in
    "") run_controls "${CONTROLS[@]}" ;;
    mutants) mutants ;;
    *) die "usage: examples.test.sh [mutants]" ;;
  esac
}

main "$@"
