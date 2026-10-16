#!/usr/bin/env bash
# Run the book's complete examples and compare their output exactly.
#
#   scripts/examples.sh check [PAGE-OR-DIR...]   (default: docs/book/src)
#
# Every example runs through the seam Ravel's CLI tests use (Fixture in
# crates/ravel/tests/cli.rs): the built `ravel` front and its paired worker,
# real TypeScript policy evaluation, fake agent programs, an empty
# environment, a private HOME and TMPDIR and a PATH of `/usr/bin:/bin` plus the
# one directory holding `ravel`. No real agent, model or personal policy is
# reachable. The convention is written in docs/book/examples.md; the controls
# for this script are scripts/examples.test.sh.
#
# Environment (used by the controls; callers normally set none):
#   RAVEL_FRONT                    the front to run (default target/debug/ravel);
#                                  its worker is found beside it as the product finds it
#   EXAMPLES_TIMEOUT               seconds one command may run (default 60)
#   EXAMPLES_ASSUME_MISSING        feature names to treat as absent on this host
#   EXAMPLES_ROOT, EXAMPLES_EXTRACT  the checkout and extractor of a mutated copy
set -euo pipefail
IFS=$'\n\t'

# The controls run mutated copies of this script and its extractor from
# elsewhere; EXAMPLES_ROOT and EXAMPLES_EXTRACT name the checkout and the
# extractor for those copies. Callers do not set them.
ROOT="${EXAMPLES_ROOT:-$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)}"
readonly ROOT
readonly EXTRACT="${EXAMPLES_EXTRACT:-$ROOT/scripts/examples-extract.awk}"

die() {
  echo "examples: $*" >&2
  exit 1
}

usage() {
  die "usage: examples.sh check [PAGE-OR-DIR...]"
}

# The seam's constants, kept in one place and matched to cli.rs by the controls.
readonly SEAM_PATH_TAIL="/usr/bin:/bin"

# Is the platform feature named by a `requires` directive absent here? Prints
# the reason and returns 0 when it is absent; returns 1 when it is present.
feature_missing() {
  local name="$1" assumed assumed_list=()
  IFS=' ' read -ra assumed_list <<<"${EXAMPLES_ASSUME_MISSING:-}"
  for assumed in ${assumed_list[@]+"${assumed_list[@]}"}; do
    if [[ "$assumed" == "$name" ]]; then
      echo "$name is assumed missing (EXAMPLES_ASSUME_MISSING)"
      return 0
    fi
  done
  case "$name" in
    confinement)
      case "$(uname -s)" in
        Darwin)
          [[ -x /usr/bin/sandbox-exec ]] || { echo "confinement needs /usr/bin/sandbox-exec, which is absent"; return 0; }
          ;;
        Linux)
          command -v bwrap >/dev/null 2>&1 || { echo "confinement needs bwrap on PATH, which is absent"; return 0; }
          ;;
        *) echo "confinement is not supported on $(uname -s)"; return 0 ;;
      esac
      ;;
  esac
  return 1
}

# Refuse a front and worker that are not both built from the current worker
# source. build.rs embeds the digest of that source in the front and
# dispatch.sh embeds it in the worker, so a front or worker built before the
# last source change does not contain today's digest. The product refuses a
# mismatched pair at run time as well; this refuses before any example runs,
# and names the cause.
require_fresh_build() {
  local front="$1" id dir worker
  [[ -f "$front" && -x "$front" ]] || die "no built ravel at $front; run task build (or use task docs:examples, which builds)"
  id="$(bash "$ROOT/crates/ravel/scripts/dispatch.sh" build-id)" || die "cannot compute the worker source digest"
  dir="$(cd "$(dirname "$front")" && pwd -P)"
  worker="$dir/../libexec/ravel/ravel-policy"
  [[ -f "$worker" ]] || die "no policy worker at $worker beside the front; run task worker"
  grep -aqF -- "$id" "$front" || die "stale build: the front $front was built from different worker source than the checkout's (digest $id); run task build"
  grep -aqF -- "$id" "$worker" || die "stale build: the worker $worker was built from different source than the checkout's (digest $id); run task worker"
  # The digest covers the worker's TypeScript. A front built before a Rust edit,
  # or a worker before a manifest edit, carries today's digest too, so the
  # sources are also compared by modification time with what was built from
  # them. A touched file makes this refuse, which is the safe direction.
  local newer
  newer="$(newer_source "$front" "$worker")"
  [[ -z "$newer" ]] || die "stale build: $newer is newer than the built front or worker; run task build"
}

# The first source file newer than FRONT (Rust, Cargo manifests and lock) or
# newer than WORKER (the worker's TypeScript, configuration, manifest, lock and
# its build script). Prints nothing when none is.
newer_source() {
  local front="$1" worker="$2" found
  found="$(find "$ROOT/crates" "$ROOT/Cargo.toml" "$ROOT/Cargo.lock" \
    -path '*/node_modules' -prune -o -path '*/target' -prune -o \
    -type f \( -name '*.rs' -o -name 'Cargo.toml' -o -name 'Cargo.lock' \) -newer "$front" -print 2>/dev/null | head -n 1)"
  if [[ -z "$found" ]]; then
    found="$(find "$ROOT/crates/ravel/worker" "$ROOT/crates/ravel/scripts" \
      -path '*/node_modules' -prune -o \
      -type f \( -name '*.ts' -o -name 'tsconfig*.json' -o -name 'package.json' -o -name 'bun.lock' -o -name 'dispatch.sh' \) -newer "$worker" -print 2>/dev/null | head -n 1)"
  fi
  printf '%s' "$found"
}

# Replace the example's scratch root with <scratch> in a stream file. The one
# normalisation the convention allows. The physical path goes first because the
# path as created (macOS's /var/folders) is a substring of it (/private/var/...).
normalise() {
  local physical="$1" created="$2" file="$3"
  PHYSICAL_ROOT="$physical" CREATED_ROOT="$created" \
    perl -0777 -pe 's/\Q$ENV{PHYSICAL_ROOT}\E/<scratch>/g; s/\Q$ENV{CREATED_ROOT}\E/<scratch>/g' "$file"
}

# Run COMMAND_FILE as one `sh` script under the seam's environment, in WORK.
# Writes stdout, stderr and the exit status to the three output files. The
# command runs in a process group of its own; one that outlasts the timeout has
# that group killed and leaves a marker file, so a timeout is never mistaken for
# a command that itself exits with the same status. The status is as a shell
# reports it: the exit code, or 128 plus the signal that ended the command.
run_command() {
  local scratch="$1" command_file="$2" out="$3" err="$4" statusfile="$5"
  local status=0
  (
    cd "$scratch/work"
    exec </dev/null
    exec perl -e '
      my ($limit, $marker, @command) = @ARGV;
      my $pid = fork;
      die "fork: $!" unless defined $pid;
      if (!$pid) { setpgrp(0, 0); exec @command or die "exec: $!"; }
      my $timed_out = 0;
      $SIG{ALRM} = sub { $timed_out = 1; kill "KILL", -$pid; open(my $f, ">", $marker); close $f; };
      alarm $limit;
      1 while waitpid($pid, 0) == -1 && $!{EINTR};
      my $s = $?;
      exit($s & 127 ? 128 + ($s & 127) : $s >> 8);
    ' "${EXAMPLES_TIMEOUT:-60}" "$scratch/timed-out" \
      env -i \
      HOME="$scratch/home" \
      TMPDIR="$scratch/tmp" \
      PATH="$scratch/ravel:$SEAM_PATH_TAIL" \
      /bin/sh -c "$(cat "$command_file")"
  ) >"$out" 2>"$err" || status=$?
  echo "$status" >"$statusfile"
}

# Compare one stream. Prints a description on mismatch and returns 1.
compare_stream() {
  local label="$1" mode="$2" expected="$3" actual="$4"
  case "$mode" in
    none) return 0 ;;
    exact)
      if ! cmp -s "$expected" "$actual"; then
        echo "  $label differs from the example's claim (< claimed, > actual):"
        diff "$expected" "$actual" | sed 's/^/    /' || true
        return 1
      fi
      ;;
    includes)
      # Byte for byte, trailing newlines included: command substitution would
      # strip them from both sides.
      if ! perl -0777 -e 'open(my $w, "<", $ARGV[0]); open(my $h, "<", $ARGV[1]); binmode $w; binmode $h; local $/; my ($want, $have) = (<$w>, <$h>); exit(index($have, $want) >= 0 ? 0 : 1)' "$expected" "$actual"; then
        echo "  $label does not contain the text the example claims:"
        sed 's/^/    claimed: /' "$expected"
        echo "  actual $label:"
        sed 's/^/    /' "$actual"
        return 1
      fi
      ;;
  esac
  return 0
}

# Run one extracted example; echo "pass", "skip" or "fail" on the last line of
# the report it prints, with detail lines before it.
run_example() {
  local exdir="$1" front="$2" where scratch real_tmp mode
  where="$(cat "$exdir/where")"

  if [[ -f "$exdir/requires" ]]; then
    local requirement reason
    while IFS= read -r requirement; do
      if reason="$(feature_missing "$requirement")" && [[ -n "$reason" ]]; then
        printf 'skip %s: %s\n' "$where" "$reason"
        return 0
      fi
    done <"$exdir/requires"
  fi

  real_tmp="$(mktemp -d "${TMPDIR:-/tmp}/ravel-example.XXXXXX")"
  scratch="$(cd "$real_tmp" && pwd -P)"
  mkdir "$scratch/home" "$scratch/work" "$scratch/tmp" "$scratch/ravel"
  ln -s "$(cd "$(dirname "$front")" && pwd -P)/$(basename "$front")" "$scratch/ravel/ravel"
  if [[ -d "$exdir/files" ]]; then
    (cd "$exdir/files" && find . -type f | LC_ALL=C sort) | while IFS= read -r path; do
      mkdir -p "$scratch/work/$(dirname "$path")"
      cp "$exdir/files/$path" "$scratch/work/$path"
    done
    if [[ -f "$exdir/executable" ]]; then
      while IFS= read -r path; do chmod 755 "$scratch/work/$path"; done <"$exdir/executable"
    fi
  fi

  local failed=0 run report="" total n
  total="$(find "$exdir" -maxdepth 1 -type d -name 'run-*' | wc -l | tr -d ' ')"
  # In numeric order: a lexical glob would run command 10 before command 2.
  for ((n = 1; n <= total; n++)); do
    run="$exdir/run-$n"
    local out err status
    out="$scratch/out.$n"
    err="$scratch/err.$n"
    status="$scratch/status.$n"
    run_command "$scratch" "$run/command" "$out" "$err" "$status"
    normalise "$scratch" "$real_tmp" "$out" >"$out.n"
    normalise "$scratch" "$real_tmp" "$err" >"$err.n"
    local detail=""
    detail+="$(compare_stream "standard output of command $n" "$(cat "$run/stdout.mode")" "$run/stdout" "$out.n")" || failed=1
    mode="$(cat "$run/stderr.mode")"
    detail+="$(compare_stream "standard error of command $n" "$mode" "$run/stderr" "$err.n")" || failed=1
    if [[ -e "$scratch/timed-out" ]]; then
      detail+="  command $n timed out after ${EXAMPLES_TIMEOUT:-60} seconds and was killed"$'\n'
      rm -f "$scratch/timed-out"
      failed=1
    elif [[ "$(cat "$run/status")" != "$(cat "$status")" ]]; then
      detail+="  exit status of command $n is $(cat "$status"), the example claims $(cat "$run/status")"$'\n'
      failed=1
    fi
    [[ -z "$detail" ]] || report+="$detail"$'\n'
  done
  rm -rf "$real_tmp"
  if ((failed)); then
    printf 'FAIL %s\n%s' "$where" "$report"
  else
    printf 'pass %s\n' "$where"
  fi
  return 0
}

check() {
  local front="${RAVEL_FRONT:-$ROOT/target/debug/ravel}"
  local targets=("$@") pages=() target stage
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

  require_fresh_build "$front"

  stage="$(mktemp -d "${TMPDIR:-/tmp}/ravel-examples.XXXXXX")"
  # shellcheck disable=SC2064 # exact owned stage directory
  trap "rm -rf '$stage'" EXIT

  if ((${#pages[@]})); then
    awk -v out="$stage" -f "$EXTRACT" "${pages[@]}"
  else
    printf 'fragments 0\n' >"$stage/counts"
  fi

  local errors=0
  if [[ -s "$stage/errors" ]]; then
    errors="$(wc -l <"$stage/errors" | tr -d ' ')"
    echo "examples: the pages do not follow the example convention (docs/book/examples.md):" >&2
    sed 's/^/  /' "$stage/errors" >&2
  fi

  local passed=0 failed=0 skipped=0 exdir line
  for exdir in "$stage"/ex-*; do
    [[ -d "$exdir" ]] || continue
    # A page that breaks the convention runs nothing: what an example claims
    # is not known until its blocks are read as the convention says.
    ((errors == 0)) || break
    local result
    result="$(run_example "$exdir" "$front")"
    line="$(head -n 1 <<<"$result")"
    case "$line" in
      pass*) passed=$((passed + 1)) ;;
      skip*) skipped=$((skipped + 1)) ;;
      FAIL*) failed=$((failed + 1)) ;;
    esac
    printf '%s\n' "$result" >&2
  done

  local fragments
  fragments="$(cut -d' ' -f2 "$stage/counts")"
  printf 'examples: %d passed, %d failed, %d skipped; %d fragment(s) not run; %d page error(s)\n' \
    "$passed" "$failed" "$skipped" "$fragments" "$errors"
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

# Sourcing the file for its definitions (the controls do) must not run main.
if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then
  main "$@"
fi
