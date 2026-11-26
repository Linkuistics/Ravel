#!/usr/bin/env bash
#
# Load check for the technical-communication Pi extension. It spends no model
# quota: it starts Pi in RPC mode, which loads the extension and fires
# `session_start` without a prompt, and then reads what the extension did.
#
# WHY IT EXISTS. A broken extension stops Pi from starting in every mode, and
# the install links this file into every Pi session on the machine. Pi also
# accepts an unknown event name without a word, so a renamed event would leave a
# dead handler and no error. Loading alone proves neither of those things, so the
# check asserts the one effect that needs the extension to load, to find the hook
# core through its own path and to register a working `session_start` handler:
# a start record in the reply log.
#
# THE CONTROLS. A clean run proves little when the instrument may be broken, so
# each claim has a run that must fail:
#   - an extension that throws must print Pi's load error (Pi exits 0 in RPC
#     mode even then, so the message on standard error is the only signal);
#   - the same extension with the off file present must write nothing.
#
# It does not test the other four layers, which need a model request. Those were
# shown in a real session (.grove, `pi-always-on-k13`).
#
# Run: bash scripts/pi-extension-load.test.sh
# Skipped, with exit 0 and a visible message, when `pi` is not installed.

set -euo pipefail
IFS=$'\n\t'

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
extension="${repo_root}/plugins/writing/craft/technical-communication/harnesses/pi/technical-communication.ts"

if ! command -v pi >/dev/null 2>&1; then
  echo "skip   pi is not installed; the Pi extension was not loaded"
  exit 0
fi

scratch="$(mktemp -d)"
trap 'rm -rf "${scratch}"' EXIT

failures=0
fail() {
  echo "FAIL: $1" >&2
  failures=$((failures + 1))
}

# run_pi <state-dir> <extension> <stderr-file>: start Pi in RPC mode with only
# that extension. Pi leaves when its standard input ends, and it does not wait
# for the extension's child process, so the input stays open for a few seconds.
run_pi() {
  (sleep 3) | TECHNICAL_COMMUNICATION_STATE_DIR="$1" \
    pi --mode rpc --no-session -ne -e "$2" >/dev/null 2>"$3" || true
}

# 1. The extension loads and its session_start handler runs the hook core.
state="${scratch}/state-on"
mkdir -p "${state}"
run_pi "${state}" "${extension}" "${scratch}/err-on"
if [[ -s "${scratch}/err-on" ]]; then
  fail "the extension printed on standard error: $(head -c 300 "${scratch}/err-on")"
fi
records="$(cat "${state}"/sessions/pi--*.jsonl 2>/dev/null || true)"
if [[ "${records}" != *'"kind":"start"'* || "${records}" != *'"harness":"pi"'* ]]; then
  fail "no start record from the extension (log: ${records:-empty})"
fi

# 2. Control: with the off file present, the same extension writes nothing.
state="${scratch}/state-off"
mkdir -p "${state}"
: >"${state}/off"
run_pi "${state}" "${extension}" "${scratch}/err-off"
if compgen -G "${state}/sessions/*" >/dev/null; then
  fail "the extension wrote a record while the off file existed"
fi

# 3. Control: Pi reports an extension that cannot load.
bad="${scratch}/throws.ts"
echo 'export default function () { throw new Error("load check control"); }' >"${bad}"
run_pi "${scratch}/state-bad" "${bad}" "${scratch}/err-bad"
if ! grep -q 'Failed to load extension' "${scratch}/err-bad"; then
  fail "Pi did not report a throwing extension, so the load check proves nothing"
fi

if ((failures > 0)); then
  echo "pi-extension-load: FAILED — ${failures} problem(s)" >&2
  exit 1
fi
echo "pi-extension-load: the extension loads, logs a start record, obeys the off file"
