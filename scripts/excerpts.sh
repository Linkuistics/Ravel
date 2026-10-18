#!/usr/bin/env bash
# Check that every source excerpt in the book still occurs in its file.
#
#   scripts/excerpts.sh check [PAGE-OR-DIR...]   (default: docs/book/src)
#
# An excerpt is a label paragraph that names a file ("Excerpt from `PATH`")
# and the one fenced block after it. The block must be one contiguous run of
# whole lines of that file. The convention is written in docs/book/excerpts.md;
# the controls for this script are scripts/excerpts.test.sh. The extractor is
# the one that scripts/examples.sh uses, so both checks read a page alike.
#
# Environment (used by the controls; callers normally set none):
#   EXCERPTS_ROOT   the checkout whose files the excerpts quote (default: this one)
set -euo pipefail
IFS=$'\n\t'

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly SCRIPT_DIR
readonly ROOT="${EXCERPTS_ROOT:-$(cd "$SCRIPT_DIR/.." && pwd)}"
readonly EXTRACT="$SCRIPT_DIR/examples-extract.awk"

die() {
  echo "excerpts: $*" >&2
  exit 1
}

usage() {
  die "usage: excerpts.sh check [PAGE-OR-DIR...]"
}

# git, with the variables that a hook or a worktree sets removed, so the
# question is always about the checkout in ROOT.
checkout_git() {
  env -u GIT_DIR -u GIT_WORK_TREE -u GIT_INDEX_FILE -u GIT_COMMON_DIR git -C "$ROOT" "$@"
}

# Say why PATH (already a plain relative path) cannot be quoted, and return 0;
# return 1 when it can. The checks run from the cheapest to the dearest.
path_problem() {
  local path="$1" part current="" parts=()
  case "/$path/" in
    */target/* | */node_modules/*)
      echo "the path is inside a build output directory (target or node_modules); the book does not quote generated files"
      return 0
      ;;
  esac
  case "$path" in
    .grove/*)
      echo "the path is inside .grove, which the finish cycle deletes; quote a file that stays"
      return 0
      ;;
  esac
  IFS=/ read -ra parts <<<"$path"
  for part in "${parts[@]}"; do
    current+="${current:+/}$part"
    if [[ -L "$ROOT/$current" ]]; then
      echo "the path is, or passes through, a symbolic link ($current); name the file that the link points to"
      return 0
    fi
  done
  if [[ ! -e "$ROOT/$path" ]]; then
    echo "the file does not exist in the checkout (it was moved or deleted, or the path is wrong)"
    return 0
  fi
  if [[ -d "$ROOT/$path" ]]; then
    echo "the path is a directory; name a file"
    return 0
  fi
  if [[ ! -f "$ROOT/$path" ]]; then
    echo "the path is not a regular file"
    return 0
  fi
  if ! checkout_git ls-files --error-unmatch -- "$path" >/dev/null 2>&1; then
    echo "git does not track the file; the book quotes only files that a reader finds in the repository"
    return 0
  fi
  return 1
}

# Compare the excerpt TEXT_FILE with SOURCE_FILE. Both are read as lines: a line
# is the bytes up to the next LF or the end of the file, and a final LF does not
# start another line. The excerpt must equal a run of consecutive lines. On a
# difference this prints the first differing line and returns 1.
compare_excerpt() {
  perl -e '
    use strict;
    use warnings;
    my ($source_file, $excerpt_file) = @ARGV;
    sub slurp {
      my ($file) = @_;
      open(my $handle, "<:raw", $file) or die "open $file: $!";
      local $/;
      my $text = <$handle>;
      close $handle;
      return $text;
    }
    sub lines {
      my @found = split /\n/, $_[0], -1;
      pop @found if @found && $found[-1] eq q{};
      return @found;
    }
    sub show {
      my ($text) = @_;
      $text =~ s/([\x00-\x1f\x7f])/sprintf("\\x%02x", ord($1))/ge;
      $text = substr($text, 0, 200) . "..." if length($text) > 200;
      return "\"$text\"";
    }
    my @source = lines(slurp($source_file));
    my @excerpt = lines(slurp($excerpt_file));
    my ($best_start, $best_run) = (-1, 0);
    for my $start (0 .. $#source) {
      next unless $source[$start] eq $excerpt[0];
      my $run = 0;
      $run++ while $run <= $#excerpt && $start + $run <= $#source && $source[$start + $run] eq $excerpt[$run];
      exit 0 if $run == @excerpt;
      ($best_start, $best_run) = ($start, $run) if $run > $best_run;
    }
    if ($best_start < 0) {
      print "  line 1 of the excerpt is not a whole line of the file: ", show($excerpt[0]), "\n";
      for my $start (0 .. $#source) {
        if (index($source[$start], $excerpt[0]) >= 0) {
          print "  it occurs inside line ", $start + 1, " of the file, but an excerpt holds whole lines\n";
          last;
        }
      }
      exit 1;
    }
    my $source_line = $best_start + $best_run;
    if ($source_line > $#source) {
      printf "  the file ends at line %d, and the excerpt goes on with its line %d: %s\n",
        scalar(@source), $best_run + 1, show($excerpt[$best_run]);
    } else {
      printf "  line %d of the excerpt differs from line %d of the file:\n    excerpt: %s\n    file:    %s\n",
        $best_run + 1, $source_line + 1, show($excerpt[$best_run]), show($source[$source_line]);
    }
    exit 1;
  ' "$1" "$2"
}

# Print the page path relative to the checkout when it is inside it.
display() {
  printf '%s' "${1#"$ROOT"/}"
}

# Check one extracted excerpt; print "pass WHERE" or "FAIL WHERE" with details.
check_excerpt() {
  local dir="$1" where label path problem detail
  where="$(display "$(cat "$dir/where")")"
  label="$(cat "$dir/label")"
  path="$(cat "$dir/path")"
  if problem="$(path_problem "$path")"; then
    printf 'FAIL %s\n  label: %s\n  %s\n' "$where" "$label" "$problem"
    return 0
  fi
  if detail="$(compare_excerpt "$ROOT/$path" "$dir/text")"; then
    printf 'pass %s\n' "$where"
  else
    printf 'FAIL %s\n  label: %s\n  the excerpt no longer occurs in %s:\n%s\n' "$where" "$label" "$path" "$detail"
  fi
  return 0
}

check() {
  local targets=("$@") pages=() target stage page
  ((${#targets[@]})) || targets=("$ROOT/docs/book/src")
  for target in "${targets[@]}"; do
    if [[ -d "$target" ]]; then
      while IFS= read -r page; do pages+=("$page"); done < <(find "$target" -name '*.md' -type f | LC_ALL=C sort)
    elif [[ -f "$target" ]]; then
      pages+=("$target")
    else
      die "no such page or directory: $target"
    fi
  done

  checkout_git rev-parse --git-dir >/dev/null 2>&1 || die "$ROOT is not a git checkout, so the check cannot tell which files are tracked"

  stage="$(mktemp -d "${TMPDIR:-/tmp}/ravel-excerpts.XXXXXX")"
  # shellcheck disable=SC2064 # exact owned stage directory
  trap "rm -rf '$stage'" EXIT

  if ((${#pages[@]})); then
    awk -v out="$stage" -f "$EXTRACT" "${pages[@]}"
  fi

  local errors=0 line
  if [[ -s "$stage/excerpt-errors" ]]; then
    errors="$(wc -l <"$stage/excerpt-errors" | tr -d ' ')"
    echo "excerpts: the pages do not follow the excerpt convention (docs/book/excerpts.md):" >&2
    while IFS= read -r line; do printf '  %s\n' "$(display "$line")" >&2; done <"$stage/excerpt-errors"
  fi

  local passed=0 failed=0 dir result
  for dir in "$stage"/xc-*; do
    [[ -d "$dir" ]] || continue
    result="$(check_excerpt "$dir")"
    case "$(head -n 1 <<<"$result")" in
      pass*) passed=$((passed + 1)) ;;
      FAIL*)
        failed=$((failed + 1))
        printf '%s\n' "$result" >&2
        ;;
    esac
  done

  printf 'excerpts: %d passed, %d failed; %d page error(s)\n' "$passed" "$failed" "$errors"
  ((failed == 0 && errors == 0))
}

main() {
  local command="${1:-}"
  shift || true
  case "$command" in
    check) check "$@" ;;
    *) usage ;;
  esac
}

# Sourcing the file for its definitions must not run main.
if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then
  main "$@"
fi
