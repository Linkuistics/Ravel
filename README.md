# Ravel

Ravel is a policy router and session process manager. A caller asks for work
by kind; the owner's TypeScript policy chooses the program that does it; Ravel
runs that program under supervision and returns the result the program
publishes. The front, policy worker and local runner are built and versioned
together as one tool.

It has two verbs: `ravel start-session` and `ravel end-session RESULT`.

The book is the guide to Ravel: what it is for, how to write an owner policy,
how to run sessions, and the reference. It lives in this repository checkout
under `docs/book/`; a release archive does not carry it. From a checkout, run
`task setup` once and then `task docs:book` to build the book and serve it on
localhost, or read the Markdown in `docs/book/src/`, starting at
[chapter 1](docs/book/src/01-what-ravel-is-for.md).

The book's chapters on how Ravel is built, reusing its components, developing
it and delivering it are not written yet. Until they are, the
[component contracts](docs/components.md), the
[runner boundary](docs/local-runner.md), the
[executable specification](docs/specs/session-execution.md) and
[runtime evidence](docs/runtime-evidence.md) are the references for that
ground. The runtime notices are
[`crates/ravel/notices/NOTICES.md`](crates/ravel/notices/NOTICES.md) in a
checkout and `libexec/ravel/notices/` in an installation.

## Develop

Use Rust 1.85 or later, curl, tar, [Task](https://taskfile.dev/), ShellCheck and
[cargo-nextest](https://nexte.st/docs/installation/pre-built-binaries/).
Run from this checkout:

```sh
task setup # fetch the verified Bun 1.4.2 development toolchain and pinned types
task build
task typecheck
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

`task typecheck` checks the worker and SDK source and the generated declarations.
`task test:cli` runs the command-boundary tests against actual workers, temporary
policies and fixture processes. Use focused checks while editing and the full
relevant suite at handoff. `task check` rebuilds and typechecks the pair, checks
formatting, denies Clippy warnings across all targets and
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

Release preparation and publication use the matched front/worker layout.

Unreleased three-target archives and installed glibc/CPU-floor checks are
available through `task release:smoke`, with exact-manifest controls at
`task release:test` and temporary-prefix formula checks at
`task release:formula`. See [delivery prerequisites and layout](docs/RELEASING.md).

Licensed under [Apache-2.0](LICENSE).
