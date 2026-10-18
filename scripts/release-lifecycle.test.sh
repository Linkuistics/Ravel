#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
bash "$root/scripts/release.test.sh"
bash "$root/scripts/release-prepare.test.sh"
bash "$root/scripts/release-build.test.sh"
bash "$root/scripts/release-doctor.test.sh"
bash "$root/scripts/release-book-version.test.sh"
bash "$root/scripts/release-book.test.sh"
