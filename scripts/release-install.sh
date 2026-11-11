#!/usr/bin/env bash
#
# Install the release of the selected channel from the tap and verify the
# installation through its own keg, never through the command on PATH.
#
# Usage: release-install.sh --channel CHANNEL
#
# Run at the tagged release commit, after release-publish.sh. The channel
# (scripts/release-channel.sh) names the formula. For versioned-2 the formula
# is keg-only: its prefix holds the only command, and unversioned ravel, its
# keg and its shared links must stay exactly as they were.
#
# Every Homebrew mutation sets HOMEBREW_NO_INSTALL_CLEANUP=1, so no running
# reader loses an earlier keg (docs/specs/release-channels.md, "Recovery and
# existing processes"). The script never runs cleanup, uninstall or reinstall.
set -euo pipefail
IFS=$'\n\t'

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
readonly REPO_ROOT
# shellcheck source=scripts/release-channel.sh
source "$REPO_ROOT/scripts/release-channel.sh"
readonly TAP=linkuistics/taps

die() {
  echo "release-install: $*" >&2
  exit 1
}

# The installed formula as Homebrew reports it. The field names are those of
# Formula#to_hash in Homebrew 7.0.13:
# https://github.com/Homebrew/brew/blob/7.0.13/Library/Homebrew/formula.rb
formula_info() {
  brew info --json=v2 "$1" | jq -ec '.formulae[0] | {
    keg_only, linked_keg,
    keg_only_reason: (.keg_only_reason.reason // null),
    installed: [.installed[].version]
  }'
}

# What this script must not change in unversioned ravel: its installed
# versions, its linked keg and the shared prefix's command.
unversioned_state() {
  local brew_prefix="$1"
  brew list --versions ravel 2>/dev/null || true
  readlink "$brew_prefix/opt/ravel" 2>/dev/null || true
  readlink "$brew_prefix/bin/ravel" 2>/dev/null || true
}

main() {
  [[ $# == 2 && "$1" == --channel ]] || die 'usage: release-install.sh --channel CHANNEL'
  local reason
  reason="$(channel_select "$2")" || die "$reason"
  channel_select "$2"

  local tag version
  tag="$(git -C "$REPO_ROOT" describe --tags --exact-match HEAD)" || die 'HEAD is not a release tag; run at the tagged release commit'
  version="${tag#v}"
  [[ "$tag" == "v$version" && "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]] || die "not a release tag: $tag"
  [[ -z "$CHANNEL_MAJOR" || "$(version_major "$version")" == "$CHANNEL_MAJOR" ]] ||
    die "CHANNEL=$CHANNEL_ID installs only major $CHANNEL_MAJOR, not $version"

  local brew_prefix before=''
  brew_prefix="$(brew --prefix)"
  if [[ -n "$CHANNEL_KEG_ONLY" ]]; then
    before="$(unversioned_state "$brew_prefix")"
  fi

  export HOMEBREW_NO_INSTALL_CLEANUP=1
  brew update
  if brew list --versions "$CHANNEL_FORMULA" >/dev/null 2>&1; then
    brew upgrade "$CHANNEL_FORMULA"
  else
    brew install "$TAP/$CHANNEL_FORMULA"
  fi
  brew test "$CHANNEL_FORMULA"

  # Class, keg-only state, destination and prefix, together.
  local info keg prefix
  info="$(formula_info "$CHANNEL_FORMULA")" || die "cannot read Homebrew's record of $CHANNEL_FORMULA"
  jq -e --arg version "$version" '.installed | index($version)' <<<"$info" >/dev/null ||
    die "$CHANNEL_FORMULA $version is not installed: $info"
  keg="$(brew --cellar "$CHANNEL_FORMULA")/$version"
  [[ -x "$keg/bin/ravel" ]] || die "the keg $keg has no executable bin/ravel"
  prefix="$(brew --prefix "$CHANNEL_FORMULA")"
  [[ "$(cd "$prefix" && pwd -P)" == "$(cd "$keg" && pwd -P)" ]] ||
    die "the prefix $prefix does not resolve to the keg $keg"
  if [[ -n "$CHANNEL_KEG_ONLY" ]]; then
    jq -e '.keg_only == true and .keg_only_reason == ":versioned_formula" and .linked_keg == null' <<<"$info" >/dev/null ||
      die "$CHANNEL_FORMULA is not an unlinked versioned keg-only formula: $info"
    [[ "$(unversioned_state "$brew_prefix")" == "$before" ]] ||
      die "installing $CHANNEL_FORMULA changed unversioned ravel: before $before, after $(unversioned_state "$brew_prefix")"
  else
    jq -e --arg version "$version" '.keg_only == false and .linked_keg == $version' <<<"$info" >/dev/null ||
      die "$CHANNEL_FORMULA $version is not the linked keg: $info"
    # The shared prefix's command is this keg's command.
    [[ "$brew_prefix/bin/ravel" -ef "$keg/bin/ravel" ]] ||
      die "$brew_prefix/bin/ravel is not the command of $keg"
  fi
  [[ "$("$keg/bin/ravel" --version)" == "ravel $version" ]] ||
    die "$keg/bin/ravel does not report ravel $version"
  # The keg's installation record names the shared skill directory, not the
  # keg (docs/specs/skill-discovery.md, "The installation record").
  bash "$REPO_ROOT/scripts/installed-record.sh" "$keg" "$brew_prefix/share/ravel/skills"
  echo "release-install: $CHANNEL_FORMULA $version installed and verified at $keg (CHANNEL=$CHANNEL_ID)"
}

main "$@"
