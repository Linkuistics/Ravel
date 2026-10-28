#!/usr/bin/env bash
# Real-tool controls: narrowing the inventory, broadening licence acceptance,
# truncating a previous output on failure or bypassing the pin must fail here.
set -euo pipefail
IFS=$'\n\t'

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
scratch="$(mktemp -d)"
trap 'rm -rf "$scratch"' EXIT
fail() { echo "FAIL: $*" >&2; exit 1; }

# Fail before fixture construction if the generator has not been implemented.
[[ -f "$root/scripts/notices.sh" ]] || fail 'the Cargo notice generator is missing'
tool="$root/target/tools/cargo-about/bin/cargo-about"
[[ -x "$tool" ]] || fail "run 'task notices:setup' first"
fixture="$scratch/fixture café"
mkdir -p "$fixture/scripts/templates" "$fixture/src" "$fixture/target/tools/cargo-about/bin"
cp "$root/scripts/notices.sh" "$fixture/scripts/"
cp "$root/scripts/release-common.sh" "$fixture/scripts/"
cp "$root/scripts/templates/notices.md.hbs" "$fixture/scripts/templates/"
cp "$root/about.toml" "$fixture/"
ln -s "$tool" "$fixture/target/tools/cargo-about/bin/cargo-about"
printf 'pub fn fixture() {}\n' >"$fixture/src/lib.rs"
cat >"$fixture/Cargo.toml" <<'TOML'
[package]
name = "notice-fixture"
version = "0.1.0"
edition = "2024"
license = "Apache-2.0"
TOML
cp "$root/LICENSE" "$fixture/LICENSE"

# Distinct texts of the same SPDX licence must both survive rendering. The
# build-only dependency belongs in notices; the dev-only dependency does not.
for component in normal-fixture build-fixture dev-fixture; do
  mkdir -p "$fixture/$component/src"
  printf '[package]\nname = "%s"\nversion = "0.1.0"\nlicense = "MIT"\n' "$component" >"$fixture/$component/Cargo.toml"
  touch "$fixture/$component/src/lib.rs"
  cat >"$fixture/$component/LICENSE" <<EOF
MIT License

Copyright (c) 2026 $component <fixture@example.invalid>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
EOF
done
cat >>"$fixture/Cargo.toml" <<'TOML'
[dependencies]
normal-fixture = { path = "normal-fixture" }
[build-dependencies]
build-fixture = { path = "build-fixture" }
[dev-dependencies]
dev-fixture = { path = "dev-fixture" }
TOML
(cd "$fixture" && cargo generate-lockfile --offline)
out="$fixture/target/notices/THIRD-PARTY-NOTICES.md"
bash "$fixture/scripts/notices.sh" generate
grep -Fq 'notice-fixture 0.1.0' "$out" || fail 'the selected package is absent'
grep -Fq 'Apache License' "$out" || fail 'licence text is absent'
for component in normal-fixture build-fixture; do
  grep -Fq "$component 0.1.0" "$out" || fail "$component is absent"
  grep -Fq "Copyright (c) 2026 $component <fixture@example.invalid>" "$out" || fail "$component attribution was lost or escaped"
done
if grep -Fq dev-fixture "$out"; then fail 'a dependency used only for development leaked into notices'; fi
cp "$out" "$scratch/first.md"
bash "$fixture/scripts/notices.sh" generate
cmp "$scratch/first.md" "$out" || fail 'unchanged inputs produced different notices'

# The production allowlist must refuse an unapproved SPDX expression, even
# though a licence file is present. Check the licence-specific diagnostic too.
sed -i.bak 's/license = "Apache-2.0"/license = "GPL-3.0-only"/' "$fixture/Cargo.toml"
if bash "$fixture/scripts/notices.sh" generate >"$scratch/refusal" 2>&1; then
  fail 'an unapproved GPL licence passed'
fi
grep -Fq 'GPL-3.0-only' "$scratch/refusal" || fail 'refusal did not identify the licence'
grep -Fq 'failed to satisfy license requirements' "$scratch/refusal" || fail "refusal was not an allowlist failure: $(cat "$scratch/refusal")"
cmp "$scratch/first.md" "$out" || fail 'failed generation replaced the previous notice'
mv "$fixture/Cargo.toml.bak" "$fixture/Cargo.toml"

# MPL is a crate-specific approval: smartstring passes, another crate refuses.
sed -i.bak 's/notice-fixture/smartstring/; s/license = "Apache-2.0"/license = "MPL-2.0"/' "$fixture/Cargo.toml"
(cd "$fixture" && cargo generate-lockfile --offline)
bash "$fixture/scripts/notices.sh" generate
grep -Fq 'smartstring 0.1.0' "$out" || fail 'smartstring exception did not apply'
cp "$out" "$scratch/mpl.md"
sed -i.bak 's/smartstring/other-mpl-crate/' "$fixture/Cargo.toml"
(cd "$fixture" && cargo generate-lockfile --offline)
if bash "$fixture/scripts/notices.sh" generate >"$scratch/refusal" 2>&1; then
  fail 'MPL approval leaked to another crate'
fi
grep -Fq 'failed to satisfy license requirements' "$scratch/refusal" || fail 'MPL refusal was not an allowlist failure'
cmp "$scratch/mpl.md" "$out" || fail 'MPL refusal replaced the previous notice'

# A missing lockfile is a refusal, never an invitation to resolve new versions.
rm "$fixture/Cargo.lock"
if bash "$fixture/scripts/notices.sh" generate >"$scratch/refusal" 2>&1; then
  fail 'generation accepted a missing Cargo.lock'
fi
[[ ! -e "$fixture/Cargo.lock" ]] || fail 'generation created a lockfile'
cmp "$scratch/mpl.md" "$out" || fail 'missing-lock refusal replaced the previous notice'

# No wrong-version executable may produce replacement notices. This stub only
# models version discovery; all licence behavior above uses real cargo-about.
rm "$fixture/target/tools/cargo-about/bin/cargo-about"
printf '#!/bin/sh\necho "cargo-about 0.0.0"\n' >"$fixture/target/tools/cargo-about/bin/cargo-about"
chmod +x "$fixture/target/tools/cargo-about/bin/cargo-about"
if bash "$fixture/scripts/notices.sh" generate >"$scratch/refusal" 2>&1; then
  fail 'generation accepted the wrong cargo-about version'
fi
grep -Fq '0.8.4' "$scratch/refusal" || fail 'pin refusal omitted the required version'
cmp "$scratch/mpl.md" "$out" || fail 'pin refusal replaced the previous notice'
[[ -z "$(find "$fixture/target/notices" -name '.notices.*' -print)" ]] || fail 'temporary output leaked'

echo 'Cargo notice controls: all passed'
