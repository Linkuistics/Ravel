#!/usr/bin/env bash
#
# The formula channel of a release. SOURCED, never executed:
#
#   source "$(dirname "${BASH_SOURCE[0]}")/release-channel.sh"
#
# docs/specs/release-channels.md owns the contract. One release has one tag,
# one book and three archives. The channel selects only how those artifacts
# reach Homebrew. This file is the one owner of the mapping from a channel to
# its formula name, file, class, keg-only line and prefix, and of the rules
# that admit a channel for a version. A caller gives the channel alone.

if [[ -n "${RAVEL_RELEASE_CHANNEL_SOURCED:-}" ]]; then
  return 0
fi
RAVEL_RELEASE_CHANNEL_SOURCED=1

# shellcheck disable=SC2034 # consumed by the scripts that source this file
readonly RELEASE_CHANNELS=(versioned-2 unversioned)
readonly RELEASE_CHANNEL_USAGE='CHANNEL=versioned-2 or CHANNEL=unversioned'

# channel_select CHANNEL: set the channel's identities, or print why the value
# is refused and return 1. An empty value is refused like an unknown one:
# omission never selects unversioned ravel.
#   CHANNEL_ID            the value itself
#   CHANNEL_FORMULA       Homebrew formula name, also the brew argument
#   CHANNEL_FILE          formula file in the tap and in target/dist
#   CHANNEL_CLASS         Ruby class of the rendered formula
#   CHANNEL_KEG_ONLY      the formula's keg_only line, empty for none
#   CHANNEL_MAJOR         the only major the channel publishes, empty for any
# Versioned naming and keg_only :versioned_formula: https://docs.brew.sh/Versions
# shellcheck disable=SC2034 # consumed by the scripts that source this file
channel_select() {
  case "${1:-}" in
    versioned-2)
      CHANNEL_ID=versioned-2
      CHANNEL_FORMULA=ravel@2
      CHANNEL_FILE=ravel@2.rb
      CHANNEL_CLASS=RavelAT2
      CHANNEL_KEG_ONLY='  keg_only :versioned_formula'
      CHANNEL_MAJOR=2
      ;;
    unversioned)
      CHANNEL_ID=unversioned
      CHANNEL_FORMULA=ravel
      CHANNEL_FILE=ravel.rb
      CHANNEL_CLASS=Ravel
      CHANNEL_KEG_ONLY=''
      CHANNEL_MAJOR=''
      ;;
    '')
      echo "the release channel is required: $RELEASE_CHANNEL_USAGE"
      return 1
      ;;
    *)
      echo "unknown release channel '$1': $RELEASE_CHANNEL_USAGE"
      return 1
      ;;
  esac
}

# The major of a stable version X.Y.Z, or nothing for an empty argument.
version_major() {
  [[ -n "${1:-}" ]] && printf '%s\n' "${1%%.*}"
}

# channel_admits CHANNEL TARGET_VERSION UNVERSIONED_VERSION: return 0 when the
# selected CHANNEL may publish TARGET_VERSION while the tap's Formula/ravel.rb
# has UNVERSIONED_VERSION (empty when the tap has no unversioned formula).
# Otherwise print the reason and its remedy, and return 1.
channel_admits() {
  local channel="$1" target="$2" unversioned="${3:-}" target_major unversioned_major
  target_major="$(version_major "$target")"
  unversioned_major="$(version_major "$unversioned")"
  case "$channel" in
    versioned-2)
      if [[ "$target_major" != 2 ]]; then
        echo "CHANNEL=versioned-2 publishes only major 2, and v$target has major $target_major; select CHANNEL=unversioned"
        return 1
      fi
      if [[ -n "$unversioned_major" ]] && ((unversioned_major >= 2)); then
        echo "unversioned ravel already has major $unversioned_major ($unversioned), so promotion is complete and ravel@2 receives no later release; select CHANNEL=unversioned"
        return 1
      fi
      ;;
    unversioned)
      if [[ "$target_major" == 2 ]] && { [[ -z "$unversioned_major" ]] || ((unversioned_major < 2)); }; then
        echo "v$target has major 2 and unversioned ravel is ${unversioned:-absent}; until release:promote moves unversioned ravel, major 2 publishes only through CHANNEL=versioned-2"
        return 1
      fi
      ;;
    *)
      channel_select "$channel"
      return 1
      ;;
  esac
}

# channel_baseline_file CHANNEL BASELINE_VERSION: the tap file that must carry
# the baseline. The first 2.0.0 cut reads Formula/ravel.rb for 1.1.0. A later
# versioned 2.x cut reads Formula/ravel@2.rb. After promotion an unversioned
# cut reads Formula/ravel.rb.
channel_baseline_file() {
  if [[ "$1" == versioned-2 && "$(version_major "$2")" == 2 ]]; then
    echo ravel@2.rb
  else
    echo ravel.rb
  fi
}

# The deprecate! line that promotion adds to ravel@2.rb, dated DATE
# (YYYY-MM-DD). Homebrew 7.0.13 requires date: and names the reason and the
# replacement formula:
# https://github.com/Homebrew/brew/blob/7.0.13/Library/Homebrew/formula.rb
# (deprecate!) and Library/Homebrew/deprecate_disable.rb (:versioned_formula).
promotion_deprecation() {
  printf '  deprecate! date: "%s", because: :versioned_formula, replacement_formula: "ravel"\n' "$1"
}

# deprecate_versioned_formula INPUT OUTPUT DATE: write INPUT, a rendered
# ravel@2.rb, to OUTPUT with the promotion's deprecate! line directly after
# its keg_only line. Homebrew's component order puts deprecate! after
# keg_only (Library/Homebrew/ast_constants.rb). Nothing else changes. Refuse
# input without exactly one keg_only line or with a deprecate! line.
deprecate_versioned_formula() {
  local input="$1" output="$2" date="$3" keg_only line
  [[ "$date" =~ ^[0-9]{4}-[0-9]{2}-[0-9]{2}$ ]] || {
    echo "not a deprecation date: $date"
    return 1
  }
  keg_only='  keg_only :versioned_formula'
  [[ "$(grep -cFx -- "$keg_only" "$input")" == 1 ]] || {
    echo "$input has no single '$keg_only' line"
    return 1
  }
  if grep -Eq '^[[:space:]]*deprecate!' "$input"; then
    echo "$input is already deprecated"
    return 1
  fi
  line="$(promotion_deprecation "$date")"
  awk -v keg_only="$keg_only" -v line="$line" '{ print } $0 == keg_only { print line }' "$input" >"$output"
}

# tap_fetch TAP: fetch the tap's remote main, through jj in a jj tap.
tap_fetch() {
  local tap="$1"
  if [[ -d "$tap/.jj" ]]; then
    jj -R "$tap" git fetch --remote origin
  else
    git -C "$tap" fetch -q origin
  fi
}

# tap_formula_text TAP FILE: print Formula/FILE as the tap's remote main had it
# at the last tap_fetch. Return 1 when the remote has no such file.
tap_formula_text() {
  local tap="$1" file="$2"
  if [[ -d "$tap/.jj" ]]; then
    jj -R "$tap" file show -r main@origin "root:Formula/$file" 2>/dev/null
  else
    git -C "$tap" show "@{upstream}:Formula/$file" 2>/dev/null
  fi
}

# The tap's remote main commit at the last tap_fetch.
tap_remote_commit() {
  local tap="$1"
  if [[ -d "$tap/.jj" ]]; then
    jj -R "$tap" log --no-graph -r main@origin -T commit_id
  else
    git -C "$tap" rev-parse '@{upstream}'
  fi
}

# The version line of formula text on stdin, or nothing.
formula_version() {
  sed -n 's/^  version "\([^"]*\)"$/\1/p' | head -n 1
}

# The version of the tap's unversioned formula on its remote main at the last
# tap_fetch, or nothing when the remote has none.
tap_unversioned_version() {
  { tap_formula_text "$1" ravel.rb || true; } | formula_version
}

# The version that cargo-release selects for LEVEL from a stable CURRENT.
# release-prepare.test.sh compares it with real cargo-release 1.1.2 previews.
next_version() {
  local current="$1" level="$2" major minor patch
  IFS=. read -r major minor patch <<<"$current"
  case "$level" in
    patch) echo "$major.$minor.$((patch + 1))" ;;
    minor) echo "$major.$((minor + 1)).0" ;;
    major) echo "$((major + 1)).0.0" ;;
    *) return 1 ;;
  esac
}
