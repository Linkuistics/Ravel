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
| `/usr/bin/sandbox-exec` on macOS | Run the [confinement](02-actors-and-words.md#trust-and-confinement) tests |
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
`target/tools`, then installs the development dependencies of the
[policy worker](02-actors-and-words.md#policy-evaluation-and-supervision). It does
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
The built [Ravel CLI](02-actors-and-words.md#policy-evaluation-and-supervision)
and policy worker need no host Bun or Node at run time.

## Build the matched pair

`task build` first compiles the policy worker and generates the SDK declarations.
Then it runs `cargo build --locked --workspace` for both Rust packages. The
Cargo lockfile pins the versions of the Rust dependencies. A first build can
need network access for dependencies that the host Cargo cache does not hold.

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
| `crates/ravel/src/` | CLI arguments, policy evaluation, [result](02-actors-and-words.md#the-managed-process-and-its-result) transport and final outcome choice |
| `crates/ravel/worker/src/` | TypeScript worker implementation |
| `crates/ravel/worker/sdk/` | Policy interface and helpers |
| `crates/ravel/worker/examples/` and `crates/ravel/worker/sample/` | Example modules and the sample policy |
| `crates/ravel/worker/typecheck/` | Consumers that check generated declarations |
| `crates/keyed-launch/src/` | Process groups, signals, waits, terminal recovery and the completion channel |
| `crates/ravel/tests/` | Tests across CLI, worker and environment boundaries |
| `crates/keyed-launch/tests/` | Runner tests with real processes and pseudo-terminals |
| `scripts/` and `Taskfile.yml` | Reusable checks, documentation tools and delivery recipes |

The private [result channel](07-the-parts-of-ravel.md#the-result-channel-carries-the-value)
combines the runner's completion signal with the contents of the result.
`end-session` sends the result, and the complete result file appears in one
atomic step. The CLI uses the result channel with the compiled policy
evaluator. Subprocess fixtures check
nested [session-result context](02-actors-and-words.md#three-different-kinds-of-information), cancellation, process
cleanup, and empty and multiline results. A change to this boundary needs more
than a test of a parser.

## Choose a test seam

A **test seam** is the boundary at which a test supplies inputs and observes
behaviour. The nearest useful seam depends on the behaviour that the change
affects.

| Behaviour to check | Seam and source | Focused task |
|---|---|---|
| A local conversion, parser or decision | Rust unit tests beside the CLI code | `task test:session -- --bin ravel` |
| A result, [refusal](02-actors-and-words.md#policy-evaluation-and-supervision) or policy request through the executable | `crates/ravel/tests/cli.rs`, with real workers, temporary policies and fixture processes | `task test:cli` |
| Group cleanup, escalation or child exit status | Runner with real subprocesses under `crates/keyed-launch/tests/` | `task test:runner` |
| Terminal foreground or modes | Runner process on a private pseudo-terminal in `tests/job.rs` | `task test:runner -- --test job` |
| Hostile runtime inputs | `crates/ravel/tests/hostile.rs`, with the real pair and controls that expose each unwanted effect | `task test:hostile`, after `task probes` |
| All tests in the Ravel package | Unit and integration tests, with the command boundary | `task test:session` |
| Both Rust packages | Workspace tests under nextest | `task test` |

The runner's terminal tests create a pseudo-terminal, or PTY. This private
terminal lets a test observe foreground ownership and modes without a human
terminal. The `in_pty` helper in `crates/keyed-launch/tests/job.rs` starts the
command as the leader of a new POSIX session. It reads the terminal output
throughout, so a full terminal buffer cannot stop the leader. At its deadline,
it kills the leader's process group and fails the test.

Tests that change process-wide signal state use a separate process. The job
tests run their own binary in a fixture role, a test function marked
`#[ignore]`. A parent test invokes that role with its required environment.
The fixture role is not a check when it runs alone. [Chapter 9](09-jobs-signals-and-the-terminal.md#lend-and-recover-the-terminal)
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
A narrow filter can exclude related cases. Read both counts in the nextest
summary. The passed count gives the cases that ran and passed. The skipped
count includes the cases that an `-E` filter excluded.

While you edit, run the focused checks. Before handoff, run the full relevant
suite.

## Run the checks before handoff

`task check` runs `scripts/check.sh`. It stops at the first failed stage. Its
last stage runs the workspace tests with nextest and shows their report. The
earlier stages run in this order:

| Stage | Work |
|---|---|
| `formatting` | `cargo fmt --all --check` checks Rust format without changes |
| `scripts` | ShellCheck checks shell scripts under `scripts/` and `crates/ravel/scripts/` |
| `worker-build` | Rebuilds the worker and generated SDK assets |
| `worker-probes` | Builds test-only workers with individual runtime controls removed |
| `runtime-integrity` | Checks runtime-cache recovery and the rejection of a corrupt download, with offline fixtures |
| `worker-types` | Typechecks the TypeScript source and consumers of generated declarations |
| `lint` | Clippy checks the workspace and all targets with the committed lockfile |
| `terminal-recovery` | Runner tests produce real-PTY receipts, then a CLI unit test consumes them |

`task runtime:integrity` and `task test:terminal-recovery` run those two checks
separately.

The workspace configuration sets Clippy's `all` lint group to `deny`, so a
Clippy report from that group fails the `lint` stage. Each stage prints a short
result. On failure, `scripts/check.sh` names its log and shows the last 60
lines. For the three worker stages, `scripts/worker-check.sh` also names its
own complete log.

`task check` creates a fresh `target/check.*` file for each run. It saves the
earlier stages' output and appends the nextest report. `scripts/worker-check.sh`
writes `target/worker-check.*`. Builds also update worker assets, probes and
Cargo artifacts under `target/`. The terminal-recovery check creates temporary
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
change the code and run the same test again. A test that passes before the repair
does not distinguish the old behaviour from the new behaviour.

### Test a real child process

In `crates/keyed-launch/tests/launch.rs`, the test type `Harness` writes a
shell script in a temporary directory. This type is not an agent harness. The
`launch` helper builds the runner request. `support::controls()` gives the
names of the inherited enclosing-session controls, and the runner removes
them from the child.
[Keep tests separate from live sessions](#keep-tests-separate-from-live-sessions)
lists these controls.

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

## Install and test the installed pair

`task install` builds a release CLI and its matched worker. It copies the pair,
SDK files, examples and notices into `PREFIX`, then tests that installation.
The default prefix is `~/.local`. With an explicit prefix, a contributor can
test a build and keep their usual installation.

1. Choose a disposable prefix for the trial.
2. Run `task install` with that prefix.
3. Check the command's status and the worker log that it names.

Fragment from this chapter's macOS arm64 trial: these commands omit setup,
output and removal of the disposable prefix.

```sh
task install PREFIX=/tmp/ravel-k178-checks/install-prefix
task smoke
task probes
```

The installation puts the CLI at `PREFIX/bin/ravel` and its assets under
`PREFIX/libexec/ravel/`. It does not add the prefix to `PATH`.
[Chapter 6](06-reference.md#installed-layout) describes the installed files.
Chapter 12, “Deliver Ravel”, covers release archives and their target checks.

| Task | What it builds or checks | Files it leaves |
|---|---|---|
| `task install PREFIX=…` | Release pair, then installed smoke checks | Installed files at the chosen prefix, plus build outputs and a worker log |
| `task smoke` | The same install workflow in a fresh prefix whose name contains spaces and Unicode | Build outputs and a worker log. The script removes its temporary prefix on exit |
| `task probes` | Four test workers with selected runtime safeguards removed | Probe executables under `target/probes/ravel/`, with the default Cargo target directory |

The installed smoke script tests both the direct CLI path and a relative
symlink from another prefix. It uses the installed SDK and sample policy with
a fake program. Its checks cover ordinary, empty, multiline and option-like
results, child output, TypeScript imports,
[policy context](04-writing-an-owner-policy.md#supply-policy-context), parameters and absent
session-result context. It also checks the required assets and temporary-file
cleanup. The private `PATH` has selected system utilities and fixture programs,
with no host Bun or Node.

The probes let hostile-runtime tests show that their unwanted effect can occur.
For example, the autoload probe permits dotenv and bunfig autoloading. Other
probes permit tsconfig autoloading, omit embedded imports or retain the worker's
initial directory. Each probe has a distinct identity, so no Ravel CLI
accepts a probe as its policy worker. After a worker change, rebuild the pair
and probes before `task test:hostile`.

These three tasks passed on macOS 27.0.1 arm64 in this chapter's trial.
The [part B check records](../../evidence/develop-ravel-k178/README.md) give its
scope. This local smoke check does not qualify release archives, Linux floors
or a different host.

## Check the book

Book checks are separate from `task check`. A source change can make a quoted
excerpt stale even when every Rust test passes. A documentation change can
break a command example without a change to product code.

`task docs:book` first builds the book, then serves it at
`http://127.0.0.1:8775`. The server rebuilds after a source change. `PORT=…`
selects another port. `task docs:book:build` only builds, under `target/book`.
Its script, `scripts/book.sh`, fails on an mdBook `ERROR` or `WARN` diagnostic,
even if mdBook exits with status 0. A successful build does not establish that
every link resolves or that a figure is legible.

| Change | Check | What the check establishes |
|---|---|---|
| Chapter or book configuration | `task docs:book:build` | mdBook builds without an `ERROR` or `WARN` diagnostic |
| Figure source or shared figure module | `task docs:figures` | Fresh SVGs and layout records match the committed files |
| Complete command example | `task docs:examples` | The built pair runs each complete example and meets its stated output and status claims |
| Source excerpt or its source file | `task docs:excerpts` | Each excerpt still occurs as consecutive whole lines in its named tracked file |
| Prose, table, heading or figure text | `task docs:ste -- --strict` | No violation of the mechanical STE rules or shared word decisions |
| Chapter layout | `task docs:breaks -- --strict` | No prose run exceeds 450 words under the check's rules |

Figure checks use the pinned Typst tool, packages and fonts from `task setup`.
They render into a scratch directory. `task docs:figures:update` instead writes
the SVGs and `.layout.json` files beside their `.typ` sources. After an intended
figure change, run the update task, inspect the diff and view the built page.
Then run the comparison again.

The [figure convention](../figures.md) describes
the shared modules and visual checks. Equality with a fresh render does not
establish that a diagram explains the right behaviour.

The example check rebuilds the pair. It evaluates real TypeScript policies and
runs fake programs with a private home, temporary directory and `PATH`. Standard
input is `/dev/null`. It compares the declared streams and exit status, with
only the scratch path normalised. Its summary reports a skipped example as a
skip, not as a pass.

Fragments do not run. The check also does not compare source excerpts.
The [example convention](../examples.md) defines labels, output claims and
platform requirements.

The excerpt check needs no build. Its comparison preserves spaces and
indentation. It rejects untracked sources, symbolic links and generated paths.
An exact quotation can still have an incorrect explanation. The
[excerpt convention](../excerpts.md) describes the label and comparison.

The STE and prose-break tasks report problems without failure unless you pass
`--strict`. By default, the STE check examines all chapter files,
`SUMMARY.md`, `CONTEXT.md`, `README.md` and the text of each figure layout
record. The prose-break check examines the
chapters. A table, figure or code block breaks a prose run. Headings and lists
do not.

Neither check can establish clear explanations or compliance with every
STE rule. A human read still checks meaning, order and use of active verbs.

The checkers have their own tests. A **control** gives a checker a known bad
input and requires it to reject that input. A **mutant** removes a safeguard
from a copy of the checker. A control then needs to fail against that copy.
These tests help show that a clean report has a useful meaning.

| Task | Controls it exercises |
|---|---|
| `docs:figures:test` | Unchanged figures and changes to sources, SVGs, records, layout pins, tools and packages |
| `docs:examples:test` | Good pages and wrong output, status, labels, build identity, environment and other example rules |
| `docs:examples:mutants` | Example-checker copies with individual safeguards removed |
| `docs:excerpts:test` | Changed source or quotation, invalid paths and labels, followed by checker mutants |
| `docs:ste:test` | Planted text violations and default-file selection, followed by rule mutants |
| `docs:breaks:test` | Prose at and above the limit, each kind of break, and rule mutants |
| `book:integrity` | Offline mdBook download fixtures, cache reuse, corrupt-cache recovery and bad-download rejection |

`book:integrity` needs the archive that `task setup` verified. It substitutes
the download boundary and uses private tool directories. It does not test
GitHub availability or the book's content. Run a checker's controls when you
change that checker. Run the relevant content checks when you change a chapter.

Fragment from this chapter's final validation: these commands omit setup and
the reports. They check the complete book, not only this chapter.

```sh
task docs:book:build
task docs:figures
task docs:examples
task docs:excerpts
task docs:ste -- --strict
task docs:breaks -- --strict
```

## Keep tests separate from live sessions

A test can run inside an LLM [session](02-actors-and-words.md#caller-and-owner)
that Ravel supervises. If a test inherits
that session's real completion path, its `end-session` call can stop the
developer's session. The test boundary therefore removes inherited authority
before it creates its own fixture inputs.

| Boundary | Guard | Source |
|---|---|---|
| Cargo commands | Force the listed session controls to empty values | `.cargo/config.toml` |
| Runner subprocess helpers | Remove those names from commands, or pass the names as runner scrub controls | `crates/keyed-launch/tests/support/mod.rs` |
| CLI and hostile-runtime fixtures | Use `env_clear()`, a private `HOME` and `TMPDIR`, and a fixture `PATH` | `crates/ravel/tests/cli.rs` and `crates/ravel/tests/support/` |
| Installed smoke | Start with `env -i`, then supply only fixture inputs | `crates/ravel/scripts/dispatch.sh` and `installed-smoke.sh` |

The Cargo guards cover these names:

- `GROVE_LAUNCH_DIR`, `GROVE_SIGNAL_FILE` and `GROVE_RUN_SIGNAL_FILE`.
- `HARNESS_DISPATCH_EXIT_FILE`, `HARNESS_DISPATCH_RUN_ID` and
  `HARNESS_DISPATCH_STATE_DIR`.
- `RAVEL_EXIT_FILE`, `RAVEL_RUN_ID` and `RAVEL_STATE_DIR`.

An empty value disables the inherited authority. It does not redirect authority
to another path. The runner helpers also protect direct subprocess invocations
that do not pass through Cargo. The CLI fixtures create temporary policies, so
tests do not need the developer's personal owner policy.

The `GROVE_*` and `HARNESS_DISPATCH_*` names deliberately remain in these
guards. They protect tests from older enclosing-session controls. They are
safety exceptions that remain from the extraction of Ravel, not product
dependencies. Paths in source provenance and the retired-project record give
historical attribution. Ravel does not load those projects or interpret their
task trees. The [caller-reference inventory](../../caller-references.md)
classifies these references in source files and dotfiles.

For a new subprocess test, reuse the fixture at the chosen seam. Supply only
the environment values that the case needs. If the case needs a result channel,
create that channel inside its temporary fixture. Never supply the enclosing
session's real completion path.

## Read the evidence for a change

Evidence notes explain what a particular source read or measured run establishes.
They sit outside the book with their receipts and logs. The tasks produce check
output. A contributor writes the note that connects that output to a claim and
states its limits. A task does not rewrite a dated evidence note.

| Question about a change | Evidence to read | Source of its observations |
|---|---|---|
| Does a runtime upgrade preserve policy isolation and installed execution? | [Runtime evidence](../../runtime-evidence.md) | `probes`, `test:hostile`, `runtime:integrity` and `smoke` |
| Does an archive contain and run the required pair? | [Archive evidence](../../archive-evidence.md) | `release:smoke`, `release:test` and `release:formula` |
| Did a particular release reach its remote and installed destinations? | [Release and installation record](../../publication-evidence.md) | Release tasks, download comparisons and installed Homebrew checks |
| Does the release procedure refuse an unsafe cut or resume the right step? | [Release lifecycle evidence](../../release-lifecycle-evidence.md) | `release:lifecycle:test` and the archive/formula checks |
| Which resource and failure paths motivated the later repairs? | [Result/job source evidence](../../result-job-source-evidence.md) | A source read at the revision named in the note, not a runtime task |
| Which checks distinguish the completion-failure repairs? | [Completion failure evidence](../../completion-failure-design-evidence.md) | Focused runner/CLI tests, `test:terminal-recovery` and `check`, with their recorded controls |
| Why does native entry follow this protocol? | [Native-entry design](../../native-entry-transition-design.md) | Source and design analysis, with later observations linked separately |
| What did the native-entry instruments measure? | [Native-entry evidence](../../native-entry-development-evidence.md) | The development, incumbent, campaign and Linux instruments below |
| Where are the run records? | [Evidence directory](../../evidence/) | The linked note identifies each receipt set, its command and source revision |

Start with the note's source revision and measured platform. Then read the
section that matches the claim. Earlier sections can describe work that later
sections complete. A design proposal is not an observed run, and an old failure
does not establish that the current source still fails.

The native-entry instruments examine executable entry state and cleanup. The
**incumbent** is the frozen product before the repair, with private test seams.
Its observations let a contributor distinguish a prior behaviour from a
repair. The table below names the instruments so that a contributor can find
their records.
Campaign plans and execution procedures remain in the evidence documents.

| Instrument tasks | Purpose and evidence |
|---|---|
| `native-entry:build`, `native-entry:front`, `native-entry:runner`, `native-entry:start`, `test:native-entry` | Build the private fixture and run the CLI cases, the runner cases and the `start_check` cases for the runner's public API. Read the development sections of the [native-entry note](../../native-entry-development-evidence.md) |
| `native-incumbent:build`, `test:native-incumbent`, `test:native-incumbent-controls` | Build and measure the pre-repair subject, and check its allowed differences. Read the [incumbent subject section](../../native-entry-development-evidence.md#incumbent-subject-and-front-baseline-k44) |
| `test:native-campaign`, `test:native-campaign-controls` | Freeze inputs, collect case records and reconcile the finite case set. Read the [campaign section](../../native-entry-development-evidence.md#frozen-campaign-and-r01-k43) |
| `native-linux:image`, `test:native-linux` | Build the Linux environment and collect its support measurements. Read the [Linux campaign section](../../native-entry-development-evidence.md#gnulinux-immutable-campaign-k52) |
| `test:native-r01-controls` | Check that the protected-test comparison rejects invalid evidence. Read the R01 account in the [campaign section](../../native-entry-development-evidence.md#frozen-campaign-and-r01-k43) |

The instrument tasks call `scripts/native-incumbent.sh`, `native-campaign.sh`,
`native-linux.sh` and `native-r01.sh` for their respective instrument families.
The native-entry tasks build and select Rust tests directly. A development
test pass alone does not establish the complete campaign or a new platform's
support. The measured support covers aarch64 macOS and aarch64 Linux.
x86_64 Linux is built and run only under emulation, with native qualification
still absent.

`task docs:result-job` serves the editable result/job views on localhost port
8774. `task docs:components` serves the current component views on port 8773.
Both accept `PORT=…` and need network access for Mermaid. These design views
are separate from the book's Typst figures and establish no runtime result.

The [retired-project record](../../retired-project.md) identifies the former
actor-daemon revision for historical attribution only.

## Find a development task

`task --list` shows the public task names and short descriptions in the current
checkout. The table below accounts for each name at this chapter's draft.
The sections above explain development tasks. Chapter 12, “Deliver Ravel”,
owns the release workflow. Its release tasks can create a release version, write remote
state and replace an installation, so they are not chapter-check commands.

| Task names | Destination |
|---|---|
| `setup` | [Prepare a checkout](#prepare-a-checkout) |
| `build`, `worker`, `typecheck`, `deps` | [Build the matched pair](#build-the-matched-pair) |
| `test`, `test:session`, `test:cli`, `test:runner`, `test:hostile`, `test:list` | [Choose a test seam](#choose-a-test-seam) |
| `check`, `runtime:integrity`, `test:terminal-recovery` | [Run the checks before handoff](#run-the-checks-before-handoff) |
| `install`, `smoke`, `probes` | [Install and test the installed pair](#install-and-test-the-installed-pair) |
| `docs:book`, `docs:book:build`, `docs:figures`, `docs:figures:update`, `docs:figures:test` | [Check the book](#check-the-book) |
| `docs:examples`, `docs:examples:test`, `docs:examples:mutants`, `docs:excerpts`, `docs:excerpts:test` | [Check the book](#check-the-book) |
| `docs:ste`, `docs:ste:test`, `docs:breaks`, `docs:breaks:test`, `book:integrity` | [Check the book](#check-the-book) |
| `docs:components`, `docs:result-job` | [Read the evidence for a change](#read-the-evidence-for-a-change) |
| `native-entry:build`, `native-entry:front`, `native-entry:runner`, `native-entry:start`, `test:native-entry` | [Read the evidence for a change](#read-the-evidence-for-a-change) |
| `native-incumbent:build`, `test:native-incumbent`, `test:native-incumbent-controls` | [Read the evidence for a change](#read-the-evidence-for-a-change) |
| `test:native-campaign`, `test:native-campaign-controls`, `native-linux:image`, `test:native-linux`, `test:native-r01-controls` | [Read the evidence for a change](#read-the-evidence-for-a-change) |
| `release:archives`, `release:smoke`, `release:test`, `release:formula` | Chapter 12, “Deliver Ravel”: archive and formula checks |
| `release:doctor`, `release:notes`, `release:lifecycle:test` | Chapter 12, “Deliver Ravel”: preparation and procedure checks |
| `release:patch`, `release:minor`, `release:major` | Chapter 12, “Deliver Ravel”: version cut and delivery |
