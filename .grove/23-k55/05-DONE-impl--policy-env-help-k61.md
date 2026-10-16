# policy-env-help-k61

## Goal

Give `ravel start-session --policy-env` the description every other option has,
so the book's chapters 5 and 6 can quote `--help` and need not carry the
contract's "owed separately" gap.

## Context

- `ravel start-session --help` prints no description for `--policy-env <NAME>`
  because its argument definition (`SelectionArgs::policy_env` in
  `crates/ravel/src/cli.rs`) has no doc comment. Every neighbour has one.
- Meaning, to be verified against the source and not taken from here: it adds
  an environment variable name to the grants the policy worker may receive,
  repeatable, alongside the `policyEnv` owner setting; the names never grantable
  are listed in `docs/specs/session-execution.md` ("Policy authority and
  runtime discovery"); diagnostics report names and presence, never values.
  Read `crates/ravel/src/environment.rs`, `settings.rs` and `inputs.rs` for what
  the flag actually does, and write the sentence from what you read.
- The contract's "Omissions and unknowns" carries the gap as a bullet. This leaf
  removes that bullet and the matching clause in the "Coverage inventories"
  text if one exists, because the omission stops being true; record the edit in
  the log. It is a statement of fact, not a change of structure.

## Done when

- `--policy-env` has a doc comment of the same register and length as its
  neighbours that states what it grants, where the granted value goes (the
  policy worker only, not the managed process), that it is repeatable, how it
  combines with the owner setting, and which names are refused.
- `ravel start-session --help` prints it; a command-boundary assertion in
  `crates/ravel/tests/cli.rs` (or the existing help case) fails on an empty
  description for any `start-session` option, and is seen to fail against the
  unmodified source.
- The behaviour is unchanged: no argument, parse, bound or exit status differs.
  Show it with the existing CLI cases and `task check`. The native-entry
  instruments name source files, so also run `task test:native-incumbent-controls`
  (k52 showed a behaviour-preserving edit can trip the incumbent driver's guard);
  if it trips on `cli.rs`, report the exact refusal and handle it as that
  leaf's finding did, never by weakening the guard.
- `docs/book/contract.md` no longer lists the gap; `docs/session-cli.md` and
  any other guide are unchanged (they are retired later by the guide fold-in).

## Notes

- X=0 binds: this is a help-text change only. Nothing about production
  behaviour may move.

## Decisions (running log)

- The flag's meaning was read from `environment.rs` (`Grants::read`,
  `worker_environment`, `excluded`), `inputs.rs`, `choice.rs` and `settings.rs`,
  not from the leaf: grants are added to the worker's `HOME`, `PATH`, `TMPDIR`,
  `LANG`, `LC_*` base with the caller's value, combine with `policyEnv` by union
  (each name once), and only the worker's environment is built from them. The
  refusal list in the help is `excluded`'s: `BUN_*`, `NODE_OPTIONS`, `NODE_PATH`,
  `NODE_PRESERVE_SYMLINKS`, `NODE_CHANNEL_*`, `LD_*`, `DYLD_*`, `RAVEL_*`,
  `HARNESS_DISPATCH_*`. Diagnostics (`policyEnvironment`) hold name and `set` only.
- The doc comment is one long paragraph like `--param`, `--timeout-ms` and
  `--context-bytes`, so `--help` prints it on one line as its neighbours do.
- Test: `every_start_session_option_prints_a_description` in
  `crates/ravel/tests/cli.rs` reads the `Options:` block of
  `start-session --help` and requires a nonempty description after the flag
  column for every line. Seen to fail against the unmodified `cli.rs`
  (`no description printed for: "      --policy-env <NAME>      "`), pass after.
- `docs/book/contract.md`: removed the "Omissions and unknowns" bullet about the
  empty `--policy-env` description. No "Coverage inventories" clause named the
  gap (inventory 1 lists environment variables refused as grants, still true).
  `docs/session-cli.md` and other guides untouched.
- Verification: `task check` exit 0 (339 passed, 46 skipped); `task
  test:native-incumbent-controls` exit 0, the guard did not trip on `cli.rs`.
  The only source change is a doc comment, so no argument, parse, bound or exit
  status moves.
