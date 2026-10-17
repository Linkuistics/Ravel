# Ravel

Ravel runs LLM sessions in agent harnesses, such as Claude Code and Codex, as
supervised parts of another program. A caller, typically an orchestrator, asks
for work by kind. The TypeScript policy of the owner chooses the harness
command. Ravel runs that command under supervision. After cleanup succeeds,
Ravel returns the result that the session sent.

An interactive harness cannot normally exit by itself. Thus, the LLM runs
`ravel end-session` to send its result and ask Ravel to end the session. This
lets an interactive harness serve as a sub-component of the process of the
orchestrator. The Ravel CLI, the policy worker and the local runner are built
and versioned together as one tool.

Ravel has two verbs: `ravel start-session` and `ravel end-session RESULT`.

The book is the guide to Ravel. It says what Ravel is for, how to write an
owner policy and how to run sessions. It also has the reference chapters.

The book is in this repository checkout under `docs/book/`. A release archive
does not have it. To read it from a checkout, run `task setup` one time. Then
run `task docs:book` to build the book and serve it on localhost. You can also
read the Markdown in `docs/book/src/`. Start at
[chapter 1](docs/book/src/01-what-ravel-is-for.md).

The book does not yet have the chapters on four subjects. These are how Ravel
is built, how to reuse its components, how to develop it and how to deliver it.
Until these chapters exist, use these references for that subject: the
[component contracts](docs/components.md), the
[runner boundary](docs/local-runner.md), the
[executable specification](docs/specs/session-execution.md) and the
[runtime evidence](docs/runtime-evidence.md). The runtime notices are
[`crates/ravel/notices/NOTICES.md`](crates/ravel/notices/NOTICES.md) in a
checkout and `libexec/ravel/notices/` in an installation.

## Develop

Use Rust 1.85 or later, curl, tar, [Task](https://taskfile.dev/), ShellCheck and
[cargo-nextest](https://nexte.st/docs/installation/pre-built-binaries/).
Run these tasks from this checkout:

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
`task test:cli` runs the command-boundary tests against actual workers,
temporary policies and fixture processes. Use focused checks while you change
the code. Run the full relevant suite at handoff.

`task check` rebuilds and typechecks the pair. It checks the format of the
code and denies Clippy warnings in all targets. It also runs the complete Rust
suite and shows the nextest report. Diagnostics from the format check, the
script lint, Clippy and nextest are saved under `target/check.*`. They come
with short pass/fail summaries and, when needed, failure details. The checks
use the committed Cargo lockfile.

On macOS, the confinement tests use `/usr/bin/sandbox-exec`. On Linux, they
need the system bubblewrap executable at `/usr/bin/bwrap` or `/bin/bwrap`. The
policy worker is built with the Bun 1.4.2 in the checkout from `task setup`.
The built Ravel CLI and worker use no host Bun or Node at run time. The checks
need no model account and no other project checkout.

The private [session channel](docs/session-channel.md) combines the completion
signal of the runner with an atomic send of the opaque result. Real subprocess
fixtures test nested inherited context, cancellation, the end of every process,
and empty and multiline values. The Ravel CLI uses the channel together with
the compiled policy evaluator.

## Earlier project

The former actor-daemon project is preserved in this repository under
`retired/actor-daemon-2026-10-12`. The preserved project includes the README edits of the owner.
[Retrieval instructions and the exact revision](docs/retired-project.md) remain
part of this tree. Its daemon sources, design documents and agent-state files
are retired history.

Release preparation and delivery use the matched layout of the Ravel CLI and
the worker. Use `task release:smoke` for the unreleased three-target archives
and the installed glibc and CPU-floor checks. Use `task release:test` for the
exact-manifest controls. Use `task release:formula` for the temporary-prefix
formula checks. Read [delivery prerequisites and layout](docs/RELEASING.md) for
the details.

Licensed under [Apache-2.0](LICENSE).
