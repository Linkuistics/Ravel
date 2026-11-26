#!/usr/bin/env bash
# Checkout-local licence tool and atomic notices for the locked release graph.
set -euo pipefail
IFS=$'\n\t'

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
readonly ROOT
readonly VERSION=0.8.4
readonly TOOL_ROOT="$ROOT/target/tools/cargo-about"
readonly TOOL="$TOOL_ROOT/bin/cargo-about"
cd "$ROOT"

die() { echo "notices: $*" >&2; exit 1; }
tool_matches() {
  [[ -x "$TOOL" ]] && [[ "$("$TOOL" --version)" == "cargo-about $VERSION" ]]
}

case "${1:-}" in
  setup)
    [[ $# == 1 ]] || die 'usage: notices.sh setup | generate'
    if ! tool_matches; then
      cargo install --locked --version "=$VERSION" --root "$TOOL_ROOT" --force cargo-about
    fi
    tool_matches || die "cargo-about $VERSION is required at $TOOL"
    # Fetch locked source packages before offline licence scanning.
    cargo fetch --locked
    ;;
  generate)
    [[ $# == 1 ]] || die 'usage: notices.sh setup | generate'
    tool_matches || die "cargo-about $VERSION is required; run 'task notices:setup'"
    [[ -f Cargo.lock ]] || die 'Cargo.lock is missing; generation requires a locked dependency graph'
    # TARGETS is the release's single target list.
    # shellcheck source=scripts/release-common.sh
    source "$ROOT/scripts/release-common.sh"
    target_args=()
    for target in "${TARGETS[@]}"; do target_args+=(--target "$target"); done
    mkdir -p "$ROOT/target/notices"
    tmp="$(mktemp "$ROOT/target/notices/.notices.XXXXXX")"
    trap 'rm -f "$tmp"' EXIT
    # --fail also refuses unreadable/unknown licences. --frozen prevents graph
    # updates and remote licence enrichment. Use every unique licence text.
    # https://github.com/EmbarkStudios/cargo-about/blob/0.8.4/docs/src/cli/generate/README.md
    "$TOOL" generate --workspace --frozen --fail "${target_args[@]}" \
      --config "$ROOT/about.toml" "$ROOT/scripts/templates/notices.md.hbs" >"$tmp"
    [[ -s "$tmp" ]] || die 'cargo-about produced an empty notice'
    chmod 0644 "$tmp"
    mv "$tmp" "$ROOT/target/notices/THIRD-PARTY-NOTICES.md"
    echo 'notices: generated target/notices/THIRD-PARTY-NOTICES.md'
    ;;
  *) die 'usage: notices.sh setup | generate' ;;
esac
