#!/usr/bin/env bash
# Controls for scripts/ste.sh. The unchanged CONTEXT.md and SUMMARY.md are the
# one pass. Each other case puts one problem into a test copy and must fail
# the strict run with that rule's name: a check that never failed would read
# the same as a check that cannot.
# With "mutants", it removes one rule at a time from a copy of scripts/ste.ts
# and shows that a control then fails.
set -euo pipefail
IFS=$'\n\t'
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
work="$(mktemp -d)"
# shellcheck disable=SC2064 # exact owned test directory
trap "rm -rf '$work'" EXIT

die() {
  echo "ste-test: $*" >&2
  exit 1
}
ok() { printf 'ste-test: ok: %s\n' "$*"; }

strict() { bash "$root/scripts/ste.sh" --strict "$@" >"$work/out" 2>&1; }

# expect_fail NAME RULE TEXT: a copy of CONTEXT.md with TEXT as one more
# paragraph fails, and the output names RULE.
expect_fail() {
  local name="$1" rule="$2" text="$3" page="$work/$1.md"
  {
    cat "$root/CONTEXT.md"
    printf '\n%s\n' "$text"
  } >"$page"
  if strict "$page"; then die "$name: the strict run passed"; fi
  grep -qF -- "$rule" "$work/out" || die "$name: the output does not name '$rule': $(cat "$work/out")"
  ok "$name fails ($rule)"
}

# expect_pass NAME TEXT: a copy with TEXT added passes.
expect_pass() {
  local name="$1" text="$2" page="$work/$1.md"
  {
    cat "$root/CONTEXT.md"
    printf '\n%s\n' "$text"
  } >"$page"
  strict "$page" || die "$name: the strict run failed: $(cat "$work/out")"
  ok "$name passes"
}

controls() {
  strict "$root/CONTEXT.md" "$root/docs/book/src/SUMMARY.md" || die "unchanged CONTEXT.md and SUMMARY.md fail: $(cat "$work/out")"
  ok "unchanged CONTEXT.md and SUMMARY.md pass"

  local w26 w21
  w26="$(printf 'word %.0s' {1..25})end."
  w21="$(printf 'word %.0s' {1..20})end."
  expect_fail long-sentence "Rule 6.3" "$w26"
  expect_pass sentence-at-limit "$(printf 'word %.0s' {1..24})end."
  expect_fail long-step "Rule 5.1" "1. $w21"
  expect_fail long-paragraph "Rule 6.6" "One. Two. Three. Four. Five. Six. Seven."
  expect_fail semicolon "Rule 8.1" "Ravel stops; the caller waits."
  expect_fail contraction "Rule 4.2" "Ravel doesn't wait."
  expect_fail normative "normative keyword" "The caller MUST wait."
  expect_fail ing-word "Rule 3.5" "The caller is waiting."
  expect_fail decided-word "does not use" "The LLM publishes the result."
  expect_fail kept-phrase-part "Rule 3.5" "The working copy changes."
  expect_pass kept-phrase "The caller's working directory stays."
  # shellcheck disable=SC2016 # the backquotes are Markdown, not a command
  expect_pass code-span 'The command `publishing; don'"'"'t` is code.'
  # shellcheck disable=SC2016 # the backquotes are Markdown, not a command
  expect_pass code-block "$(printf 'Text.\n\n```sh\nrunning; publish; MUST\n```')"
  expect_pass avoid-line "_Avoid_: publish, submit"

  # A figure record is JSON: its text is examined, not its keys.
  local fig="$work/figure.layout.json"
  printf '{"elements": [{"id": "running", "text": "Ravel sends the result."}]}' >"$fig"
  strict "$fig" || die "figure-keys: a key in -ing failed the run: $(cat "$work/out")"
  ok "figure keys are not text"
  printf '{"elements": [{"text": "Ravel is sending; the LLM submits it."}]}' >"$fig"
  if strict "$fig"; then die "figure-text: the strict run passed"; fi
  for rule in "Rule 8.1" "Rule 3.5" "does not use"; do
    grep -qF "$rule" "$work/out" || die "figure-text: the output does not name '$rule': $(cat "$work/out")"
  done
  ok "figure text fails (Rule 8.1, Rule 3.5, a decided word)"

  # The decisions come from ste.md: without its row, a kept phrase fails.
  grep -v '^| working directory |' "$root/docs/book/ste.md" >"$work/ste.md"
  if STE_DECISIONS="$work/ste.md" strict "$root/CONTEXT.md"; then die "decisions: CONTEXT.md passed without the working directory row"; fi
  ok "a removed decision row fails CONTEXT.md"
  printf '# No tables\n' >"$work/empty.md"
  if STE_DECISIONS="$work/empty.md" strict "$root/CONTEXT.md"; then die "decisions: a file with no tables passed"; fi
  grep -qF "has no word tables" "$work/out" || die "decisions: $(cat "$work/out")"
  ok "a decisions file with no tables is refused"

  # Without --strict the run is a report.
  printf 'Ravel stops; the caller waits.\n' >"$work/report.md"
  bash "$root/scripts/ste.sh" "$work/report.md" >"$work/out" 2>&1 || die "report: the run without --strict failed"
  grep -qF "1 problems" "$work/out" || die "report: $(cat "$work/out")"
  ok "the report run exits 0 and counts the problem"
}

mutants() {
  local copy="$work/ste.ts" name pattern
  while IFS=$'\t' read -r name pattern; do
    sed "$pattern" "$root/scripts/ste.ts" >"$copy"
    cmp -s "$copy" "$root/scripts/ste.ts" && die "mutant $name: the pattern changed nothing"
    if STE_TS="$copy" bash "$0" >"$work/mutant.out" 2>&1; then die "mutant $name: every control passed"; fi
    ok "mutant $name: a control fails ($(grep -v ': ok: ' "$work/mutant.out" | grep -m1 'ste-test:' | cut -c11-90))"
  done <<'EOF'
no sentence limit	s/if (count(s) > limit)/if (false)/
no paragraph limit	s/ss.length > 6) add/ss.length > 99) add/
no semicolon check	s/if (semi >= 0)/if (false)/
no contraction check	s/(?:n\['’\]t|/(?:nXt|/
no normative check	s/MUST|SHOULD|MAY|SHALL|REQUIRED/XXXXXXXXXX/
no -ing check	s/p.endsWith("ing")/p.endsWith("ingX")/
no decided words	s/if (avoid.has(p) \&\&/if (false \&\&/
no kept phrases	s/!kept(parts, i)/true/
no code removal	s/" CODE ") \/\/ a code span/"$\&") \/\/ a code span/
no step limit	s/b.kind === "step" ? 20 : 25/25/
no fence removal	s/if (fence) {/if (false) {/
no avoid lines	s/b.kind !== "avoid"/true/
EOF
}

case "${1:-}" in
  mutants) mutants ;;
  "") controls ;;
  *) die "usage: ste.test.sh [mutants]" ;;
esac
