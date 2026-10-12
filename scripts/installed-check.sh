#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
mkdir -p target
prefix="$(mktemp -d "$root/target/installed café.XXXXXX")"
# shellcheck disable=SC2064 # remove only the private fresh prefix
trap "rm -rf '$prefix'" EXIT
bash scripts/worker-check.sh install "$prefix"
