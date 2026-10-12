# Ravel

Ravel is a policy router and session process manager. The development front,
compiled policy worker and local runner now build together at version 0.0.0.
The installed pair includes its own runtime, generic SDK, examples and notices.
It runs independently from a fresh prefix, directly or through a symlink.

The agreed public boundary is a process manager and router with two verbs:
`start-session` and `end-session RESULT`. The caller supplies the prompt and
ending instructions; Ravel manages the process and returns the opaque supplied
result. See [the session CLI contract](docs/session-cli.md). Persistence and
historical migration are outside this scope.

The runner starts a caller-supplied program as a supervised job. It owns process
groups, completion channels, cancellation, reaping, terminal recovery and optional
filesystem confinement. It selects no harness, reads no policy and opens no
record store. See [the runner boundary and source provenance](docs/local-runner.md).

## Develop

Use Rust 1.85 or later, curl, tar, [Task](https://taskfile.dev/), ShellCheck and
[cargo-nextest](https://nexte.st/docs/installation/pre-built-binaries/).
Run from this checkout:

```sh
task setup # fetch the verified Bun 1.4.2 development toolchain and pinned types
task build
task test
task test:session
task test:cli
task probes
task test:hostile
task runtime:integrity
task check
task smoke
task install PREFIX=/your/prefix
```

`task check` checks formatting, denies Clippy warnings across all targets and
runs the complete Rust suite with nextest's reporting visible. Formatting,
script lint, Clippy and nextest diagnostics are saved under `target/check.*`, with short
pass/fail summaries and failure details when needed. Checks use the committed
Cargo lockfile. On
macOS, confinement tests exercise `/usr/bin/sandbox-exec`; on Linux they require
the system bubblewrap executable at `/usr/bin/bwrap` or `/bin/bwrap`. The policy worker builds with the checkout-local Bun 1.4.2 from `task setup`.
The built front/worker uses no host Bun/Node at run time. No model account or
other project checkout is needed for these checks.

The private [session channel](docs/session-channel.md) composes the runner's
completion signal with atomic opaque-result publication. Real subprocess fixtures
verify nested inherited context, cancellation, reaping and empty/multiline
values. The command front composes it with the compiled policy evaluator.

## Earlier project

The former actor-daemon project, including the owner's README edits, is preserved
in this repository under `retired/actor-daemon-2026-10-12`.
[Retrieval instructions and the exact revision](docs/retired-project.md) remain
part of this tree. Its daemon sources, design documents and agent-state files are
retired history.

The extracted tool starts at 0.0.0; its first minor release will be 0.1.0.
Release preparation and publication follow the runnable front/worker increment.

Unreleased three-target archives and installed glibc/CPU-floor checks are
available through `task release:smoke`, with exact-manifest controls at
`task release:test` and temporary-prefix formula checks at
`task release:formula`. See [delivery prerequisites and layout](docs/RELEASING.md).

Licensed under [Apache-2.0](LICENSE).

## Use the pair

```sh
ravel start-session --kind feature --config ./policy.ts --prompt-file ./prompt.txt
# From inside its managed process:
ravel end-session done
```

Put PREFIX/bin (or target/debug in a checkout) on the managed process's PATH, or use its absolute path in
ending instructions. The caller supplies those instructions in the prompt.
Only the exact result bytes go to stdout, without an added newline. Session
output and policy diagnostics go to stderr. See [the CLI contract](docs/session-cli.md).
The generic SDK, static routing example and editable choice sample are under
`crates/ravel/worker`; declarations are generated beside the compiled worker and installed under
PREFIX/libexec/ravel. Copy examples/sample.ts into your owner configuration and
edit its routes; it launches only the wrapper you supply. See the
[executable specification](docs/specs/session-execution.md),
[authority boundary](docs/authority.md), [runtime evidence](docs/runtime-evidence.md)
and [runtime notices](crates/ravel/notices/NOTICES.md).
The [owner configuration guide](docs/owner-configuration.md) describes copying,
typechecking and fake-agent validation of an existing policy without history access.
