# Ravel

Ravel is being extracted into an independent harness launcher. This development
increment contains its **local job runner**, `keyed-launch`, and private session
result transport at version 0.0.0. They build and test on their own.
The command front, compiled policy worker
and installation tasks are the next increment; this tree does not yet install a
`ravel` command.

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

Use Rust 1.85 or later, [Task](https://taskfile.dev/), ShellCheck and
[cargo-nextest](https://nexte.st/docs/installation/pre-built-binaries/).
Run from this checkout:

```sh
task build
task test
task test:session
task check
```

`task check` checks formatting, denies Clippy warnings across all targets and
runs the complete Rust suite with nextest's reporting visible. Formatting,
script lint, Clippy and nextest diagnostics are saved under `target/check.*`, with short
pass/fail summaries and failure details when needed. Checks use the committed
Cargo lockfile. On
macOS, confinement tests exercise `/usr/bin/sandbox-exec`; on Linux they require
the system bubblewrap executable at `/usr/bin/bwrap` or `/bin/bwrap`. No Bun,
Node, model account or other project checkout is needed for these runtime checks.

The private [session channel](docs/session-channel.md) composes the runner's
completion signal with atomic opaque-result publication. Real subprocess fixtures
verify nested inherited context, cancellation, reaping and empty/multiline
values. It does not yet provide the policy-routed public CLI.

## Earlier project

The former actor-daemon project, including the owner's README edits, is preserved
in this repository under `retired/actor-daemon-2026-10-12`.
[Retrieval instructions and the exact revision](docs/retired-project.md) remain
part of this tree. Its daemon sources, design documents and agent-state files are
retired history.

The extracted tool starts at 0.0.0; its first minor release will be 0.1.0.
Release preparation and publication follow the runnable front/worker increment.

Licensed under [Apache-2.0](LICENSE).
