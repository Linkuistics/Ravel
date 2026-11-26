---
name: coding-style-bash
description: Bash coding standards — strict mode, shellcheck/shfmt, quoting, local-scoped functions, bats-core testing. Use when writing or refactoring Bash scripts.
harnesses: [any]
paths:
  - "**/*.sh"
  - "**/*.bash"
---

# Bash Coding Style Guidelines

**The repo's own configuration wins.** `.shellcheckrc`, `.editorconfig`, an `shfmt` invocation in CI or a `Makefile`, and the existing scripts' own shebang and strict-mode conventions are the authority wherever they exist. What follows is the default for a repo that has not decided.

## Header
- **Shebang:** `#!/usr/bin/env bash` — bash 4+ assumed; do not target POSIX sh
- **Strict mode:** every script begins with `set -euo pipefail` and `IFS=$'\n\t'`

## Formatting & Linting
- **Linter:** shellcheck — gate in CI, treat warnings as errors
- **Formatter:** shfmt with `-i 2 -ci` (2-space indent, indent case branches)

## Style
- Always quote variable expansions: `"$var"`, `"$@"`, `"${array[@]}"`
- Use `[[ ... ]]` over `[ ... ]`; use `(( ... ))` for arithmetic
- Use `$(...)` over backticks for command substitution
- Prefer `mapfile -t` over `read` loops for slurping command output
- Lowercase variable names except for exported environment variables

## Functions
- Declare every variable inside a function with `local`
- Communicate via exit codes (0 success, non-zero failure); do not echo-and-parse status
- Pass data through stdin and arguments, not global variables
- Name functions verb-first: `fetch_repo`, not `repo`

## Error Handling
- Trap errors: `trap 'echo "Error on line $LINENO" >&2' ERR`
- When `set -e` is locally suppressed, check return codes explicitly
- Write user-facing errors to stderr (`>&2`) and exit with a non-zero code

## Testing
- **Write the test first.** A new behaviour or a bugfix starts as a failing test; implement until it passes.
- Use **bats-core** for any script with non-trivial logic
- Trivial one-liners do not need tests

## When to Stop Using Bash
- If a script exceeds ~100 lines, or needs structured data (JSON, multi-dimensional state, etc.), rewrite in Python or Rust
