#!/usr/bin/env bash
# Controls for scripts/breaks.sh. Each case writes a test page and runs the
# strict check on it: a page at the limit passes, a page over the limit fails,
# and each thing that the contract says breaks a run or adds no words is made
# to change the result. Every "breaks" control is paired with the same text
# without the break, which must fail, so a check that never failed would not
# pass for it.
# With "mutants", it changes one rule at a time in a copy of scripts/breaks.ts
# and shows that a control then fails.
set -euo pipefail
IFS=$'\n\t'
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
work="$(mktemp -d)"
# shellcheck disable=SC2064 # exact owned test directory
trap "rm -rf '$work'" EXIT

die() {
  echo "breaks-test: $*" >&2
  exit 1
}
ok() { printf 'breaks-test: ok: %s\n' "$*"; }

strict() { bash "$root/scripts/breaks.sh" --strict "$@" >"$work/out" 2>&1; }

# words N: N words in one line.
words() { awk -v n="$1" 'BEGIN { for (i = 1; i < n; i++) printf "word "; print "word" }'; }

# repeat N TEXT: TEXT on N lines.
repeat() { awk -v n="$1" -v t="$2" 'BEGIN { for (i = 1; i <= n; i++) print t }'; }

# page NAME PART...: a page of the parts, with a blank line after each one.
page() {
  local name="$1" part
  shift
  : >"$work/$name.md"
  for part in "$@"; do printf '%s\n\n' "$part" >>"$work/$name.md"; done
}

expect_pass() {
  page "$@"
  strict "$work/$1.md" || die "$1: the strict run failed: $(cat "$work/out")"
  ok "$1 passes"
}

expect_fail() {
  page "$@"
  if strict "$work/$1.md"; then die "$1: the strict run passed"; fi
  grep -qF -- "1 runs longer than 450 words in 1 files" "$work/out" || die "$1: the output does not count one run: $(cat "$work/out")"
  ok "$1 fails"
}

# shellcheck disable=SC2016 # the backquotes are Markdown, not a command
controls() {
  local w225 w300 w449 fig="![A figure with some alt text.](figures/x.svg)"
  w225="$(words 225)"
  w300="$(words 300)"
  w449="$(words 449)"

  expect_pass at-limit "$w225" "$w225"
  expect_fail over-limit "$w225" "$w225 extra"

  # The report names the file, the lines, the count and the headings.
  page position "# Title" "$(words 451)"
  if strict "$work/position.md"; then die "position: the strict run passed"; fi
  grep -qF "position.md:3-3: 451 words (limit 450); headings: \"Title\"" "$work/out" || die "position: $(cat "$work/out")"
  ok "the report names the file, the lines, the count and the headings"
  page two-headings "# One" "$w225" "## Two" "$w225" "extra"
  if strict "$work/two-headings.md"; then die "two-headings: the strict run passed"; fi
  grep -qF "headings: \"One\", \"Two\"" "$work/out" || die "two-headings: $(cat "$work/out")"
  ok "the report names every heading in the run"

  # A figure, a table and a code block each break a run.
  expect_fail no-break "$w300" "$w300"
  expect_pass figure-breaks "$w300" "$fig" "$w300"
  expect_pass table-breaks "$w300" "$(printf '| A | B |\n|---|---|\n| 1 | 2 |')" "$w300"
  expect_pass code-breaks "$w300" "$(printf '```sh\nrun one\n```')" "$w300"
  expect_pass tilde-code-breaks "$w300" "$(printf '~~~\nrun one\n~~~')" "$w300"
  expect_pass code-words-not-counted "$w300" "$(printf '```\n%s\n```' "$(words 500)")" "$(words 100)"
  page code-heading "# Title" "$(printf '```\n# Not a heading\n```')" "$(words 451)"
  if strict "$work/code-heading.md"; then die "code-heading: the strict run passed"; fi
  grep -qF 'headings: "Title"' "$work/out" || die "code-heading: $(cat "$work/out")"
  ok "a line in a code block is not a heading"

  # A heading and a list do not break a run.
  expect_fail heading-keeps "$w300" "## Heading" "$w300"
  expect_fail list-keeps "$w300" "$(printf -- '- one\n- two')" "$w300"
  expect_fail numbered-list-keeps "$w300" "$(printf '1. one\n2. two')" "$w300"

  # Each run is counted by itself.
  expect_pass runs-count-apart "$w300" "$fig" "$w300" "$fig" "$w300"

  # A marker and a heading's text are not words.
  expect_pass list-markers-not-words "$(repeat 150 '- a b c')"
  expect_pass numbered-markers-not-words "$(repeat 150 '1. a b c')"
  expect_pass quote-markers-not-words "$(repeat 225 '> a b')"
  expect_pass heading-text-not-words "$w449" "# $(words 20)"
  expect_fail list-words-count "$(repeat 151 '- a b c')"
  expect_fail quote-words-count "$(repeat 226 '> a b')"

  # A link target and an HTML comment add no words.
  expect_pass link-target-not-words "$w300 [a link](page.md \"$(words 200)\") [two](x_(y).md \"$(words 40)\")"
  expect_pass reference-definition-not-words "$w449" "[ref]: page.md \"$(words 100)\""
  expect_fail link-text-counts "$w449 [a link text](page.md)"
  expect_pass comment-not-words "$w300 <!-- $(words 100) --> $(words 100)"
  expect_pass comment-lines-not-words "$w225" "$(printf '<!--\n%s\n-->' "$(words 200)")" "$(words 225)"
  expect_fail text-beside-comment-counts "$w300 <!-- x --> $(words 160)"

  # The check counts each file by itself, and a run without words is no run.
  page small "$w225"
  page large "$w225" "$w225 extra"
  if strict "$work/small.md" "$work/large.md"; then die "files: the strict run passed"; fi
  grep -qF "small.md: 0 runs" "$work/out" || die "files: $(cat "$work/out")"
  grep -qF "large.md: 1 runs" "$work/out" || die "files: $(cat "$work/out")"
  ok "a count for each file"
  expect_pass only-headings "# One" "## Two"

  # Without --strict the run is a report; with no file it examines each chapter.
  bash "$root/scripts/breaks.sh" "$work/large.md" >"$work/out" 2>&1 || die "report: the run without --strict failed"
  grep -qF "large.md:1-" "$work/out" || die "report: $(cat "$work/out")"
  ok "the report run exits 0 and names the run"
  bash "$root/scripts/breaks.sh" >"$work/out" 2>&1 || die "default: the report run failed: $(cat "$work/out")"
  local chapter n=0
  for chapter in "$root"/docs/book/src/[0-9][0-9]-*.md; do
    grep -qF "breaks: docs/book/src/$(basename "$chapter"):" "$work/out" || die "default: the report does not examine $chapter"
    n=$((n + 1))
  done
  ((n > 0)) || die "default: no chapter found"
  ok "the default run examines each of the $n chapters"
}

mutants() {
  local copy="$work/breaks.ts" name pattern
  while IFS=$'\t' read -r name pattern; do
    sed "$pattern" "$root/scripts/breaks.ts" >"$copy"
    cmp -s "$copy" "$root/scripts/breaks.ts" && die "mutant $name: the pattern changed nothing"
    if BREAKS_TS="$copy" bash "$0" >"$work/mutant.out" 2>&1; then die "mutant $name: every control passed"; fi
    ok "mutant $name: a control fails ($(grep -v ': ok: ' "$work/mutant.out" | grep -m1 'breaks-test:' | cut -c14-90))"
  done <<'EOF'
no limit	s/r.words > LIMIT/r.words > 99999/
limit one more	s/^const LIMIT = 450;/const LIMIT = 451;/
limit one less	s/r.words > LIMIT/r.words >= LIMIT/
figure does not break	s/ || \/^\\s\*!\\\[\/.test(raw)//
table does not break	s/\/^\\s\*\\|\/.test(raw) || //
code block does not break	/if (f) {/{n;s/end();//;}
heading breaks	s/if (run) run.headings.push(h\[1\]);/end();/
list breaks	s/if (!raw.trim()) return;/if (\/^\\s*([-*+]|\\d+[.)])\\s\/.test(raw)) end(); if (!raw.trim()) return;/
blank line breaks	s/if (!raw.trim()) return;/if (!raw.trim()) return end();/
list and quote markers counted	s/^      const words = .*/      const words = linkText(raw).split(\/\\s+\/).filter(Boolean).length;/
heading text counted	s/heading = h\[1\];/heading = h[1]; if (!run) run = { first: n, last: n, words: 0, headings: [] }; run.words += 20;/
link targets counted	s/^const NESTED_PARENS = .*/const NESTED_PARENS = "ZZZZ";/
reference definitions counted	s/^    \.replace(\/\^\\s\*\\\[\[\^\\\]\]+.*$/    ;/
comments counted	s/^const uncommented = .*/const uncommented = (text: string) => text;/
no strict exit	s/if (strict \&\& longRuns.length > 0) process.exit(1);//
no headings in the report	s/headings: \${r.headings.map/headings: ${[].map/
no line numbers	s/\${r.first}-\${r.last}/0-0/
EOF
}

case "${1:-}" in
  mutants) mutants ;;
  "") controls ;;
  *) die "usage: breaks.test.sh [mutants]" ;;
esac
