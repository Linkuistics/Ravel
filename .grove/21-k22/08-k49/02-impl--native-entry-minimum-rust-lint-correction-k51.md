# native-entry-minimum-rust-lint-correction-k51

## Goal

Make `task check`'s lint step pass at the declared minimum Rust, as its own
failing-first correction run, changing no behaviour. This is the production
repair the node's charter says earns its own run rather than a quiet edit inside
qualification.

## Context

Read the node brief's "Findings carried" and the evidence note's "Minimum Rust
1.85: the lint step fails" section. The red subject, the clippy log on the
unmodified tree, and an unapplied patch that makes a scratch copy clean are in
`docs/evidence/native-entry-linux-development/minimum-rust-clippy/`. The four
sites are `crates/keyed-launch/src/run.rs:534` (`clippy::precedence`),
`crates/ravel/build.rs:72`, `crates/ravel/src/context.rs:723` and
`crates/ravel/tests/cli.rs:451` (`clippy::format_collect`; the last is a
protected test file).

Name the clippy version every time: the genuine 1.85 driver is
`~/.rustup/toolchains/1.85-*/bin/cargo-clippy` (0.1.85); on this host
`rustup run 1.85 cargo clippy` resolves to Homebrew's 0.1.99 and reads clean. In
the Linux image rustc and clippy are 1.85.0. Do not add `#[allow]`: it hides the
finding the declared minimum exists to show.

## Done when

- **Red first.** On the unmodified tree, before any edit, clippy 0.1.85 at 1.85.0
  and 1.85.1 reports exactly these four errors (logs kept), and the same command
  at the current compiler and at 1.86.0 is clean, so the red is the declared
  minimum's and not the tree's.
- **The repair** rewrites the four sites with no change of behaviour. For the two
  production library sites and the build script, show it: a differential check
  that the new `hex` produces the old output for all 256 byte values (in the
  scratch or as a unit test already in the suite's style), and that the
  precedence site's compiled form is unchanged (`-Zunpretty=mir` of the function
  before and after, or an equivalent the session names). For the protected test
  site, show the identity, its assertion text and its fixture bytes are unchanged:
  only how the octal escape string is built differs.
- **Green**: clippy 0.1.85 clean for `--workspace --all-targets -D warnings` and
  for `native-faults`; the current compiler's clippy and `cargo fmt --check` clean.
- **Protected expectations unchanged**: `task test:cli`, `test:session`,
  `test:runner`, `test` and `check` on the host (Darwin) and, in the Linux image,
  through `scripts/native-r01.sh` on aarch64 (this is the first run where `check`
  can pass at 1.85.0). No suite expectation is weakened and no identity is lost.
- The k43 Darwin receipts describe the pre-edit tree; say so where the evidence
  note describes them, and do not touch them. The run leaf measures the corrected
  tree.
- No other production change. The incumbent subject (`0515419edd29`) is a frozen
  historical tree and is not edited.

## Notes

If a further declared-minimum finding appears once these four compile (clippy
stops dependents behind a failing crate), the scratch enumeration found none, but
a new one is the same kind of finding and is repaired here, not elsewhere.

## Decisions (running log)

