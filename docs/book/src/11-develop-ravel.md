# Develop Ravel

A contributor changes the code and checks its behaviour through real processes.
The tests use temporary [owner policies](04-writing-an-owner-policy.md) and
fixture programs. They need no model account
or other project checkout.

This chapter starts with a checkout, then shows where to change code and add a
test. [Chapter 7](07-the-parts-of-ravel.md) explains the reasons for the parts.
[Chapter 8](08-follow-a-session-through-the-code.md#find-where-a-change-belongs)
traces the code that a change can affect. A contributor who adds a supported
extracted component writes chapter 10 with that component.

## Prepare a checkout

The checkout needs tools that an installed Ravel does not need. The build uses
a C linker and Rust 1.85 or later. On macOS, the Command Line Tools supply the
linker. On Linux, the system development tools supply it.

| Prerequisite | Purpose |
|---|---|
| Rust and Cargo, with rustfmt and Clippy | Compile, format and lint the Rust workspace |
| Task | Run the recipes in `Taskfile.yml` |
| Bash, curl and tar with gzip, bzip2 and xz support | Run scripts and unpack the downloaded tools |
| `shasum` or `sha256sum` | Compare downloads with committed SHA-256 pins |
| ShellCheck | Check shell scripts during `task check` |
| cargo-nextest | List and run Rust tests |
| Git | Obtain the checkout and support scripts that inspect tracked files |
| `/usr/bin/sandbox-exec` on macOS | Run the confinement tests |
| `/usr/bin/bwrap` or `/bin/bwrap` on Linux | Run the confinement tests where the host permits user namespaces |

1. Obtain a checkout of the Ravel repository.
2. Change to its root directory, where `Taskfile.yml` is.
3. Run `task setup`.

Fragment from the checkout trial for this chapter: these commands omit tool
installation and the command output.

```sh
task setup
task build
task typecheck
```

Setup needs network access on its first run. It downloads the tools below into
`target/tools`, then installs the worker's development dependencies. It does
not install Rust, Task, ShellCheck or nextest.

Setup has tool pins for macOS arm64, Linux aarch64 and Linux x86_64. Native
x86_64 Linux remains unqualified. The checkout trial below covers only macOS
arm64.

| Download | Pin | Source |
|---|---|---|
| Bun | 1.4.2 | npm runtime archive |
| mdBook | 0.5.4 | GitHub release archive |
| Typst | 0.15.1 | GitHub release archive |
| CeTZ and its dependency oxifmt | 0.4.2 and 1.0.0 | Typst package registry |
| DejaVu fonts | 2.37 | GitHub release archive |
| TypeScript and Bun types | 7.0.2 and 1.4.2 | Worker package manifest and lockfile |

The scripts compare tool archives with committed SHA-256 digests before use.
The figure setup also checks package trees and font files. A bad cached archive
causes another download. A fresh archive with the wrong digest stops setup.
The worker dependency install uses `bun install --frozen-lockfile --ignore-scripts`
under `crates/ravel/worker`. These dependencies go into that directory's
`node_modules`.

The checkout-local Bun takes precedence over a host Bun during worker builds.
The built [Ravel CLI](02-actors-and-words.md) and
[policy worker](07-the-parts-of-ravel.md#the-worker-executes-typescript-before-the-launch)
need no host Bun or Node at run time.

## Build the matched pair

`task build` first compiles the policy worker and generates the SDK declarations.
Then it runs `cargo build --locked --workspace` for both Rust packages. The
Cargo lockfile fixes the Rust dependency choices. A first build can need network
access for dependencies that the host Cargo cache does not hold.

| Task | Use it when | Output or check |
|---|---|---|
| `task build` | You need the matched CLI and worker | `target/debug/ravel` and `target/libexec/ravel/` |
| `task worker` | You change worker or SDK source | Compiled `ravel-policy`, SDK declarations, examples and notices under `target/libexec/ravel/` |
| `task typecheck` | You change TypeScript or its interface | Worker, SDK, examples, sample policy and consumers of generated declarations |
| `task deps` | The worker dependency manifest or lockfile changes | Dependencies from the frozen Bun lockfile |

These paths assume the default Cargo target directory. `task typecheck` needs
the declarations from `task worker` or `task build`. It does not generate them.
After a worker change, run `task build` before command-boundary tests. Ravel
checks the worker identity, so a new CLI with an old worker can refuse to run.
[Chapter 7](07-the-parts-of-ravel.md#the-installed-pair-stays-together)
explains this identity check.

## Find the source for a change

The workspace has two Rust packages. `ravel` holds the CLI and its private
TypeScript worker. `keyed-launch` holds the
[runner](07-the-parts-of-ravel.md#the-runner-owns-the-local-job-mechanics).
The package boundary does not promise an independently supported component.

| Path from the checkout root | What a contributor finds there |
|---|---|
| `crates/ravel/src/` | CLI arguments, policy evaluation, result transport and final outcome choice |
| `crates/ravel/worker/src/` | TypeScript worker implementation |
| `crates/ravel/worker/sdk/` | Policy interface and helpers |
| `crates/ravel/worker/examples/` and `crates/ravel/worker/sample/` | Example modules and the sample policy |
| `crates/ravel/worker/typecheck/` | Consumers that check generated declarations |
| `crates/keyed-launch/src/` | Process groups, signals, waits, terminal recovery and the completion channel |
| `crates/ravel/tests/` | Tests across CLI, worker and environment boundaries |
| `crates/keyed-launch/tests/` | Runner tests with real processes and pseudo-terminals |
| `scripts/` and `Taskfile.yml` | Reusable checks, documentation tools and delivery recipes |

The private [result channel](07-the-parts-of-ravel.md#the-result-channel-carries-the-value)
combines the runner's completion signal with an atomic send of the result.
The CLI uses it with the compiled policy evaluator. Subprocess fixtures check
nested [session-result context](02-actors-and-words.md), cancellation, process
cleanup, and empty and multiline results. A change to this boundary needs more
than a test of a parser.

## Choose a test seam

A **test seam** is the boundary at which a test supplies inputs and observes
behaviour. The nearest useful seam depends on the promise that the change
affects.

| Behaviour to check | Seam and source | Focused task |
|---|---|---|
| A local conversion, parser or decision | Rust unit tests beside the CLI code | `task test:session -- --bin ravel` |
| A result, refusal or policy request through the executable | `crates/ravel/tests/cli.rs`, with real workers, temporary policies and fixture processes | `task test:cli` |
| Group cleanup, escalation or child exit status | Runner with real subprocesses under `crates/keyed-launch/tests/` | `task test:runner` |
| Terminal foreground or modes | Runner process on a private pseudo-terminal in `tests/job.rs` | `task test:runner -- --test job` |
| Hostile runtime inputs | `crates/ravel/tests/hostile.rs`, with the real pair and controls that expose each unwanted effect | `task test:hostile`, after `task probes` |
| All tests in the Ravel package | Unit and integration tests, with the command boundary | `task test:session` |
| Both Rust packages | Workspace tests under nextest | `task test` |

The runner's terminal tests create a pseudo-terminal, or PTY. This private
terminal lets a test observe foreground ownership and modes without a human
terminal. The `in_pty` helper in `crates/keyed-launch/tests/job.rs` starts a
separate session leader and drains its output. Its deadline bounds the wait
for that leader.

Tests that change process-wide signal state use a separate process. The job
tests run their own binary in an ignored fixture role. A parent test invokes
that role with its required environment. The ignored role alone is not a
standalone check. [Chapter 9](09-jobs-signals-and-the-terminal.md#lend-and-recover-the-terminal)
explains the terminal behaviour that these tests observe.

`task test:list` lists the cases before a run. The focused tasks pass arguments
after `--` to nextest. A package selector chooses code, `--test` chooses an
integration-test binary, and `-E` chooses test names with a filter expression.

Fragment from the checkout trial: list first, then run the added tests. The
fragment omits their source and the nextest output.

```sh
task test:list -- -E 'test(book_)'
task test:runner -- -E 'test(book_)'
task test:cli -- -E 'test(book_)'
task test:session -- --bin ravel -E 'test(book_)'
```

The `book_` cases are the additions below. They exist only after you add them.
A narrow filter can exclude related cases, so the nextest summary needs
attention: passed and skipped counts describe different sets. While you edit,
run the focused checks. Before handoff, run the full relevant suite.

## Run the checks before handoff

`task check` runs `scripts/check.sh`. It stops at the first failed stage. Its
last stage runs the workspace tests with nextest and shows their report. The
earlier stages run in this order:

| Stage | Work |
|---|---|
| `formatting` | `cargo fmt --all --check` checks Rust format without changes |
| `scripts` | ShellCheck checks shell scripts under `scripts/` and `crates/ravel/scripts/` |
| `worker-build` | Rebuild the worker and generated SDK assets |
| `worker-probes` | Build test-only workers with individual runtime controls removed |
| `runtime-integrity` | Check runtime-cache recovery and corrupt-download refusal with offline fixtures |
| `worker-types` | Typecheck the TypeScript source and consumers of generated declarations |
| `lint` | Clippy checks the workspace and all targets with the committed lockfile |
| `terminal-recovery` | Runner tests produce real-PTY receipts, then a CLI unit test consumes them |

`task runtime:integrity` and `task test:terminal-recovery` run those two checks
separately.

The workspace configuration denies Clippy's `all` lint group. Each stage prints
a short result. On failure, the wrapper shows the last 60 lines of its log.
Worker build, probe and typecheck wrappers also name their own complete logs.

`task check` creates a fresh `target/check.*` file for each run. It saves the
earlier stages' output and appends the nextest report. Worker wrappers write
`target/worker-check.*`. Builds also update worker assets, probes and Cargo
artifacts under `target/`. The terminal-recovery check creates temporary
receipts and removes them when its script exits. The format stage does not
rewrite source files.

`task check` does not build the book or run its example, excerpt, figure or prose
checks. It also does not install Ravel or test release archives. Those workflows
have their own tasks. The workspace test run uses nextest's default selection,
so a pass does not mean every ignored fixture or native measurement ran.

Fragment from the checkout trial: run the full check after the focused checks.
This fragment omits output and the log paths that vary between runs.

```sh
task check
```

The checkout trial on 18 October 2026 used macOS 27.0.1 arm64, Rust 1.99.1,
Task 3.54.0 and nextest 0.9.148. It started without `target/` or worker
dependencies. Host download caches were available. These elapsed times describe
one machine, not a performance limit.

| Task in the trial | Elapsed time | Result |
|---|---|---|
| `task setup` | 12.11 seconds | Tool downloads, verification and dependency install passed |
| `task build` | 4.23 seconds | Passed |
| `task typecheck` | 0.43 seconds | Passed |
| `task test:list` | 2.92 seconds | Listed the workspace cases |
| Focused `test:runner`, `test:cli` and `test:session` commands above | 1.27, 2.39 and 1.20 seconds | All four added tests passed |
| First `task check` | 29.33 seconds | A hostile-runtime test exceeded its fixture time limit |
| Unchanged `task check` rerun | 109.29 seconds | 343 tests passed, 46 skipped |

The failed case passed alone before the full rerun. Its cause remains unknown.
The [checkout trial records](../../evidence/develop-ravel-k160/README.md) retain both
runs. After the checks, `target/` occupied about 920 MiB and worker dependencies
about 37 MiB. Allow additional space for the checkout, host caches and temporary
build files. Setup needs network access for missing downloads.

## Add a test

The following fragments came from tests added to a scratch checkout for this
chapter. They use the helpers and imports of the named files. All four passed
on macOS arm64. They demonstrate current behaviour and do not change the
product.

For a defect, first add an assertion that fails because of that defect. Then
change the code and run the same test again. A test that passes before the fix
does not distinguish the old behaviour from the new behaviour.

### Test a real child process

In `crates/keyed-launch/tests/launch.rs`, `Harness` writes a shell script in a
temporary directory. The `launch` helper builds the runner request.
`support::controls()` removes inherited session controls from the child.

Fragment from the scratch copy of `crates/keyed-launch/tests/launch.rs`: this
test omits the file's imports and helper definitions.

```rust
#[test]
fn book_child_exit_keeps_its_status() {
    let harness = Harness::new();
    let script = harness.script("exit 23\n");
    let argv = harness.argv(&script);
    let channel = Channel::allocate(&harness.control()).unwrap();

    let ended = run(launch(&argv, &channel, support::controls(), None)).unwrap();

    assert_eq!(ended.status.code(), Some(23));
    assert_eq!(ended.end, End::Exited);
    assert!(!ended.signalled);
}
```

The child exits with status 23 without a completion signal. The assertions
separate the exit status, the reason supervision ended, and the channel state.
This test needs no PTY because it makes no terminal claim.

For a terminal claim, the helper in `job.rs` supplies the PTY. The launcher
role reports its foreground group before and after the child runs.

Fragment from the scratch copy of `crates/keyed-launch/tests/job.rs`: this test
omits the imports, `Scratch`, the launcher role and the PTY helper.

```rust
#[test]
fn book_normal_exit_restores_foreground() {
    let scratch = Scratch::new();
    let child = scratch.script("exit 23\n");
    let (status, output) = in_pty(
        role_command(
            "launcher",
            &[("JOB_CHILD", &child), ("JOB_REPORT", &scratch.os("report"))],
        ),
        Duration::from_secs(20),
        |_| {},
    );
    assert!(status.success(), "{status:?}\n{output}");
    let report = report(&scratch.path("report"));
    assert_eq!(report["code"], "23");
    assert_eq!(report["before_foreground"], report["group"]);
    assert_eq!(report["after_foreground"], report["group"]);
}
```

The parent checks that the launcher completed its report, then reads the child
status from that report. The two statuses belong to different processes.
This test checks final foreground ownership. It does not check recovery after
a mode change. The nearby raw-mode test changes the modes and verifies that
the change occurred before it checks recovery.

### Test the command boundary

In `crates/ravel/tests/cli.rs`, `Fixture` gives the executable an empty
environment with a private `HOME`, `TMPDIR` and working directory. Its `PATH`
contains `/usr/bin:/bin`. The `selecting` helper creates a TypeScript policy
that selects `/bin/sh`. The real compiled worker evaluates that policy.

Fragment from the scratch copy of `crates/ravel/tests/cli.rs`: this test omits
the imports and `Fixture` definition.

```rust
#[test]
fn book_result_keeps_spaces() {
    let fixture = Fixture::new();
    let policy = fixture.selecting(r#""$1" end-session -- "$2""#, "  done  ");
    let output = fixture.start(&policy, "Check spaces.");

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(output.stdout, b"  done  ");
    assert_eq!(fs::read_dir(fixture.root.join("tmp")).unwrap().count(), 0);
}
```

The helper supplies the built Ravel executable as `$1` and the result as `$2`.
The shell sends the result with `end-session`. The assertions check success,
unchanged spaces and cleanup of the fixture's temporary directory. A parser
test alone cannot check this path through both executables.

### Test a local rule

A CLI unit test can check the argument parser without a worker or child process.
The separator `--` makes a result such as `--help` into data.

Fragment from the scratch copy of `crates/ravel/src/cli.rs`: add this test
module at file scope. The fragment omits the argument definitions above it.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn book_result_after_separator_is_data() {
        let cli = Cli::try_parse_from(["ravel", "end-session", "--", "--help"]).unwrap();
        let Command::EndSession { result } = cli.command else {
            panic!("expected end-session");
        };
        assert_eq!(result, "--help");
    }
}
```

The parser returns `Command::EndSession` with the unchanged value. This test
does not call `end-session` or need a result channel. The command-boundary
tests cover the later effect of that command.
