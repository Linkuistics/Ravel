# Develop Ravel

Suppose that you change how Ravel returns a result. You want to be sure that a
result with spaces at its ends reaches the caller with those spaces. In this
chapter, you prepare a checkout and build Ravel. Then you find the code, choose
where a test observes the rule, add the test and run the checks before you
hand the change over. The same steps apply to every change to Ravel.

The tests run real processes. They use temporary
[configuration scripts](04-write-a-configuration-script.md), Rhai files that
select programs, and fake programs that stand in for harnesses. A harness is
an agent program, such as Claude Code or Codex, in which an LLM works.
The tests need no model account, no personal
configuration and no other project checkout.

[Chapter 7](07-the-parts-of-ravel.md) explains the parts of Ravel and why each
part exists.
[Chapter 8](08-follow-a-session-through-the-code.md#find-where-a-change-belongs)
follows a session through the code that a change can affect.

Ravel supports no separate component. Its private runner serves the Ravel
CLI, which supervises one session and then exits. The runner's return does
not restore process-wide signal state. A program that needs supervised
harness sessions invokes the Ravel CLI as a child process. This chapter
teaches changes to Ravel itself.

Ravel 1.x evaluated configuration scripts in a TypeScript worker. The checkout
built that worker with Bun, typechecked its SDK and installed it beside the
CLI. Ravel 2.0.0 removed the worker, its SDK, its build and its typecheck. The
Rhai engine is now a Rust dependency inside the one executable, `bin/ravel`.
Bun stays in the checkout only as a tool for two book checks.

Links to files outside the book open repository sources on GitHub.
Before a change reaches `main`, read its evidence and convention files from
the checkout.

## Prepare a checkout

A checkout needs tools that an installed Ravel does not need. The build uses
Rust 1.99.1 and a C linker. On macOS, the Command Line Tools supply the
linker. On Linux, the system development tools supply it. The table shows
which part of the work needs each tool.

| Prerequisite | Purpose |
|---|---|
| rustup, with Rust 1.99.1, rustfmt and Clippy | Compile, format and lint the Rust workspace |
| Task | Run the recipes in `Taskfile.yml` |
| cargo-nextest | List and run the Rust tests |
| ShellCheck | Check the shell scripts during `task check` |
| Bash, curl, and tar with gzip and xz | Run the scripts and unpack the downloaded book tools |
| `shasum` or `sha256sum` | Compare each download with its committed SHA-256 pin |
| Git | Obtain the checkout, and let scripts read the tracked files |

The file `rust-toolchain.toml` at the root of the checkout selects Rust
1.99.1, with rustfmt, Clippy and the three release target libraries. When
rustup's tools are first on `PATH`, `cargo` in the checkout uses that
toolchain. `rustup toolchain install`, run in the checkout, installs that
toolchain when rustup does not have it.

1. Obtain a checkout of the Ravel repository.
2. Use `cd` to enter its root directory, where `Taskfile.yml` is.
3. Run `rustup toolchain install`.
4. Run `task setup`.
5. Run `task build`.

Fragment from the checkout trial for this chapter: these commands omit tool
installation and the command output.

```sh
rustup toolchain install
task setup
task build
```

`task setup` needs network access on its first run. It downloads the book
tools into `target/tools`. It does not install Rust, Task, ShellCheck or
nextest. The table gives each download, its pin and the work that needs it.

| Download | Pin | Work that needs it |
|---|---|---|
| Bun | 1.4.2 | `task docs:ste` and `task docs:breaks`, which run `scripts/ste.ts` and `scripts/breaks.ts` |
| mdBook | 0.5.4 | The book build, `task docs:book` and the release book |
| Typst | 0.15.1 | The figure render, `task docs:figures` |
| CeTZ and its dependency oxifmt | 0.4.2 and 1.0.0 | The figure render |
| DejaVu fonts | 2.37 | The text of each figure |

The scripts compare each archive with its committed SHA-256 digest before
use. The figure setup also checks the package trees and the font files. A bad
archive in the cache causes another download. A fresh download with the wrong
digest stops setup.

Setup has tool pins for macOS arm64, Linux aarch64 and Linux x86_64. Native
x86_64 Linux remains unqualified. The checkout trial below covers only macOS
arm64.

The Bun of the checkout is a development tool for the book only. No build of
Ravel uses it, and no installation or archive holds it. The
[Ravel CLI](02-actors-and-words.md#evaluation-and-supervision) and its
[Rhai worker](02-actors-and-words.md#evaluation-and-supervision) need no Bun
or Node at run time.

## Build the executable

`task build` runs `cargo build --locked --workspace`. Cargo builds both Rust
packages and writes the executable to `target/debug/ravel`, with the default
Cargo target directory. The Rhai worker is not a separate build. The Ravel
CLI starts another copy of `target/debug/ravel` in an internal role to
evaluate a configuration script.

`Cargo.lock` pins every Rust dependency. `crates/ravel/Cargo.toml` pins the
Rhai engine exactly, at version 1.27.0. A first build can need network access
for the dependencies that the Cargo cache on the host does not hold.

The build script `crates/ravel/build.rs` puts a source identity into the
executable. That identity is a digest of the Rust sources of both packages,
the package manifest, the build script and `Cargo.lock`. The worker's `hello`
holds the identity, and the Ravel CLI compares it with its own. Both roles come
from one build, so a change to the worker always agrees with its CLI.
[Chapter 7](07-the-parts-of-ravel.md#the-installed-executable-supplies-both-roles)
explains this check.

| Task | Use it when | Output or check |
|---|---|---|
| `task build` | You change Rust source | `target/debug/ravel` and the runner library |
| `task check` | You hand a change over | Format, lint, script checks and all Rust tests |
| `task rhai:upgrade-proof` | You change the Rhai pin | The engine tests and the hostile classes through the CLI |
| `task notices` | You change a dependency | Licence notices for the release graph, under `target/notices/` |

`task notices` installs cargo-about 0.8.4 into the checkout on its first run,
through `task notices:setup`. It refuses a dependency whose licence is not on
the approved list. `task install` and the release builds run it, because each
installation carries the notice file.

## Find the source for a change

The workspace has two Rust packages. `ravel` holds the CLI, the Rhai worker
role and the result channel. `keyed-launch` holds the
[runner](07-the-parts-of-ravel.md#the-runner-owns-the-local-job-mechanics).
The package boundary does not promise a separately supported component. The
table maps each kind of source and test to its directory.

| Path from the checkout root | What a contributor finds there |
|---|---|
| `crates/ravel/src/` | CLI arguments, configuration script checks, the [result](02-actors-and-words.md#the-managed-process-and-its-result) channel and the choice of the final outcome |
| `crates/ravel/src/rhai_worker/` | The Rhai worker role and the CLI side of the host bridge |
| `crates/ravel/src/skills/` | Skill source resolution, the default skill directory and the preamble that Ravel adds to the prompt |
| `crates/ravel/examples/` | The Rhai examples that an installation copies to `share/ravel/examples/` |
| `crates/ravel/build.rs` | The source identity of the executable |
| `crates/keyed-launch/src/` | Process groups, signals, waits, terminal recovery and the completion channel |
| `crates/ravel/tests/` | Tests through the real `ravel` executable, with fake programs |
| `crates/keyed-launch/tests/` | Runner tests with real processes and pseudo-terminals |
| `scripts/` and `Taskfile.yml` | Reusable checks, book tools and delivery recipes |

[Chapter 7, “Find each part in the source”](07-the-parts-of-ravel.md#find-each-part-in-the-source)
gives each part with the process where it runs. In the example of this
chapter, the result goes through the
[result channel](07-the-parts-of-ravel.md#the-result-channel-carries-the-value).
`end-session` writes the complete result file in one atomic step, and the CLI
reads it after cleanup. Tests with real subprocesses check nested
[session-result context](02-actors-and-words.md#three-different-kinds-of-information),
cancellation, process cleanup, and empty and multiline results. Thus, a change
to this boundary needs more than a test of a parser.

## Choose a test seam

A **test seam** is the boundary at which a test supplies inputs and observes
behaviour. Choose the nearest seam that can observe the behaviour of your
change. For the spaces of a result, a test of the argument parser is too near.
It cannot see what `end-session` writes or what the caller receives. The
command boundary can see both.

The table connects each behaviour to a seam and a focused command.

| Behaviour to check | Seam and source | Focused command |
|---|---|---|
| A local conversion, parser or decision | Rust unit tests beside the CLI code | `task test:session -- --bin ravel` |
| A result, [refusal](02-actors-and-words.md#evaluation-and-supervision) or managed launch through the executable <!-- tc: depart synonym-rotation because launch names the runner's launch of a job, while start names other process creation --> | `crates/ravel/tests/cli.rs`, with temporary configuration scripts and fixture processes | `task test:cli` |
| A configuration script, its settings, helpers, reads or harness runs | `crates/ravel/tests/rhai_*.rs`, through the real `ravel start-session` | `task test:session -- --test rhai_config` |
| An LLM call in a configuration script | `crates/ravel/tests/rhai_llm_calls.rs`, with a fake harness that plays the model | `task test:session -- --test rhai_llm_calls` |
| Skill source resolution and delivery | `crates/ravel/tests/skills_*.rs`, with local source trees and fake programs | `task test:session -- --test skills_launch` |
| Hostile startup files, module shadows and diagnostics | `crates/ravel/tests/hostile.rs` | `task test:session -- --test hostile` |
| Group cleanup, escalation or child exit status | The runner, with real subprocesses under `crates/keyed-launch/tests/` | `task test:runner` |
| Terminal foreground or modes | The linked runner in a fixture process on a private pseudo-terminal, in `tests/job.rs` | `task test:runner -- --test job` |
| All tests in the Ravel package | Unit tests and every test file of `crates/ravel/tests/` | `task test:session` |
| Both Rust packages | Workspace tests under nextest | `task test` |

The test tasks build what they need. Cargo builds the `ravel` executable before
the integration tests of its package, and each test starts that executable.
Thus, a test always uses the worker of the same build. The tests need no host
Bun or Node, no model account and no installed Ravel.

A configuration script can ask a model through a harness run, as
[chapter 4](04-write-a-configuration-script.md) teaches. The tests never make
a real call. In `rhai_llm_calls.rs`, a fake harness on the private `PATH`
plays the model. It prints a fixed answer and records its arguments and its
standard input. The test then reads which route Ravel launched and what the
fake received. Each case has a control: the same script with another answer
must give the other route.

A real call is a separate, opt-in check. `task harness:smoke:claude` and
`task harness:smoke:codex` make one real call through Claude Code or Codex
with the owner's sign-in, and that call spends subscription usage. No default
task runs them.
[Chapter 4](04-write-a-configuration-script.md) explains them. `task check`
runs only `task harness:smoke:test`, which tests the smoke script against
fake harnesses.

When you change the Rhai pin, run `task rhai:upgrade-proof`. It prints the pin
and checks that `Cargo.lock` builds the same version. Then it runs each
`rhai_*` test target, in a fixed order. Last, it runs each hostile class of
`rhai_limits.rs`, such as an endless loop, a deep recursion and a full heap.
It fails when a `rhai_*` file is not on its list, or when a target runs no
test.

The runner's terminal tests create a pseudo-terminal, or PTY. This private
terminal lets a test observe foreground ownership and modes without a human
terminal. The `in_pty` helper in `crates/keyed-launch/tests/job.rs` starts the
command as the leader of a new POSIX session. It reads the terminal output all
the time, so a full terminal buffer cannot stop the leader. At its deadline,
it kills the leader's process group and fails the test.

Tests that change process-wide signal state use a separate process. The job
tests run their own binary in a fixture role, a test function marked
`#[ignore]`. A parent test invokes that role with its required environment.
The fixture role is not a check when it runs alone.
[Chapter 9](09-jobs-signals-and-the-terminal.md#lend-and-recover-the-terminal)
explains the terminal behaviour that these tests observe.

`task test:list` lists the cases before a run. The focused tasks pass the
arguments after `--` to nextest. A package selector chooses code, `--test`
chooses an integration-test binary, and `-E` chooses test names with a filter
expression.

Fragment from the checkout trial: list first, then run the added tests. The
fragment omits their source and the nextest output.

```sh
task test:list -- -E 'test(book_)'
task test:runner -- -E 'test(book_)'
task test:cli -- -E 'test(book_)'
task test:session -- --test rhai_llm_calls -E 'test(book_)'
task test:session -- --bin ravel -E 'test(book_)'
```

The `book_` cases are the additions in
[Add a test](#add-a-test). They exist only after you add them. A narrow filter
can exclude related cases. Read both counts in the nextest summary. The passed
count gives the cases that ran and passed. The skipped count includes the
cases that an `-E` filter excluded.

While you edit, run the focused checks. Before handoff, run the full suite.

## Run the checks before handoff

`task check` runs `scripts/check.sh`. Each stage prints one line with its
result. The script stops at the first stage that fails. Its last stage runs
the workspace tests with nextest and shows their report. The earlier stages
run in this order:

| Stage | Work |
|---|---|
| `formatting` | `cargo fmt --all --check` checks the Rust format and changes no file |
| `scripts` | ShellCheck checks the shell scripts under `scripts/` |
| `book-bun-integrity` | Offline fixtures check the book Bun's download, cache recovery and the refusal of a corrupt download |
| `harness-smoke-fakes` | The harness smoke script runs against fake harnesses and makes no real call |
| `installed-record` | Controls check the script that compares a Homebrew installation record |
| `lint` | Clippy checks the workspace and all targets with the committed lockfile |
| `terminal-recovery` | Runner tests produce real-PTY receipts, then a CLI unit test reads them |

`task test:terminal-recovery` runs the last of these checks alone.

The workspace configuration sets Clippy's `all` lint group to `deny`. Thus,
a Clippy report from that group fails the `lint` stage. When a stage fails,
`scripts/check.sh` names its log and shows the last 60 lines.

Each run of `task check` writes a new `target/check.*` log. The log holds the
output of the earlier stages and the nextest report. The terminal-recovery
check creates temporary receipts and removes them when its script exits. The
format stage does not change source files.

`task check` does not build the book or run its example, excerpt, figure or
prose checks. It also does not install Ravel or test release archives. Those
workflows have their own tasks. The workspace test run uses nextest's default
selection. Thus, a pass does not mean that every ignored fixture or native
measurement ran.

Fragment from the checkout trial: run the full check after the focused checks.
This fragment omits the output and the log paths, which change between runs.

```sh
task check
```

The [checkout trial record](https://github.com/Linkuistics/Ravel/blob/main/docs/evidence/develop-ravel-k308/README.md)
gives the source revision, tool versions, commands and results. It describes
one macOS machine with cached tools, not a clean-machine installation.

## Add a test

The following fragments come from tests added to a scratch copy of the
checkout for this chapter. They use the helpers and imports of the named
files. All five passed on macOS arm64. They show current behaviour and do not
change the product.

For a defect, first add an assertion that fails because of that defect. Then
change the code and run the same test again. A test that passes before the
code change cannot tell the old behaviour from the new behaviour.

### Test the command boundary

This is the test for the example of this chapter. In
`crates/ravel/tests/cli.rs`, `Fixture` gives the executable an empty
environment with a private `HOME`, `TMPDIR` and working directory. Its `PATH`
contains `/usr/bin:/bin`. The `selecting` helper writes a Rhai configuration
script that selects `/bin/sh` with a script of your choice. The real Rhai
worker evaluates that configuration script.

Fragment from the scratch copy of `crates/ravel/tests/cli.rs`: this test omits
the imports and the `Fixture` definition.

```rust
#[test]
fn book_result_keeps_spaces() {
    let fixture = Fixture::new();
    let config = fixture.selecting(r#""$1" end-session -- "$2""#, "  done  ");
    let output = fixture.start(&config, "Check spaces.");

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(output.stdout, b"  done  ");
    assert_eq!(fs::read_dir(fixture.root.join("tmp")).unwrap().count(), 0);
}
```

The helper gives the shell the built Ravel executable as `$1` and the result
as `$2`. The shell sends the result with `end-session`. The assertions check
success, the unchanged spaces and the cleanup of the fixture's temporary
directory. A test of the parser alone cannot check this path through both
`ravel` processes.

### Test an LLM call with a fake model

You may want to check that a configuration script launches the route that a
model's answer chooses. In `crates/ravel/tests/rhai_llm_calls.rs`, `SELECTION`
is a configuration script that asks the harness `classifier` to classify the
prompt. It maps each answer to a route. `fake_model` writes a fake
`classifier` on the private `PATH`, and `settings` names it as a harness.

Fragment from the scratch copy of `crates/ravel/tests/rhai_llm_calls.rs`: this
test omits the file's imports, constants and helpers.

```rust
#[test]
fn book_fake_model_answer_chooses_the_route() {
    let sandbox = Sandbox::new();
    let recorded = sandbox.recording_agent();
    fake_model(&sandbox, "classifier", &format!("echo {LIGHT}"));
    settings(&sandbox, &["classifier"], 1);
    sandbox.personal_rhai(&rhai_script(SELECTION));

    let notice = notice_of(&start(&sandbox, PROMPT, &[], &[]));

    assert_eq!(recorded.argv().unwrap(), ["routed", "quick-route"]);
    assert_runs(&notice, &[("classifier", "exited")]);
}
```

The fake model prints the light answer, so Ravel launches `quick-route`. The
routing notice lists one harness run of `classifier`. A copy of the test that
expected `deep-route` failed. Thus, the route comes from the answer and not
from a default. The file's own test of this script also checks the other
answer.

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
separate the exit status, the reason that supervision ended and the channel
state. This test needs no PTY, because it makes no claim about a terminal.

For a claim about a terminal, the helper in `job.rs` supplies the PTY. The
launcher role reports its foreground group before and after the child runs.

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

The parent checks that the launcher completed its report. Then it reads the
child's status from that report. The two statuses belong to different
processes. The table separates those observations in this example.

| Process | Work and observation in this example |
|---|---|
| Parent test | Starts the launcher role through `in_pty`. Checks the role's exit status, then reads the report |
| Launcher role | Runs the same test binary in a separate process on the private PTY. Its linked runner supervises the child. Reports the foreground values and the child's status |
| Child | Runs the shell script that exits with status 23. The report's `code` field holds this status |

This test checks the final foreground ownership. It does not check recovery
after a change of modes. The raw-mode test near it changes the modes and
checks that the change occurred before it checks recovery.

### Test a local rule

A CLI unit test can check the argument parser without a worker or a child
process. The separator `--` makes a result such as `--help` into data.

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
does not run `end-session` or need a result channel. The tests at the command
boundary cover the later effect of that command.

## Install and test the installed files

`task install` builds a release executable for the host and generates the
licence notices. It copies Ravel's files into `PREFIX`, then tests that
installation. The default prefix is `~/.local`. With an explicit prefix, you
can test a build and keep your usual installation.

1. Choose a disposable prefix for the trial.
2. Run `task install` with that prefix.
3. Read the last line, which names the installed version and the prefix.

Fragment from this chapter's macOS arm64 trial: these commands omit the output
and the removal of the disposable prefix.

```sh
task install PREFIX=target/install-trial
task smoke
```

The installation puts the executable at `PREFIX/bin/ravel`, the three Rhai
examples under `PREFIX/share/ravel/examples/` and the notice file under
`PREFIX/share/ravel/notices/`. It removes an old `PREFIX/libexec/ravel`
directory, which held the Ravel 1.x worker and SDK. It does not add the
prefix to `PATH`, and it writes no installation record. Only a Homebrew
formula writes `share/ravel/installation.json`, because only an installation
by Homebrew knows the shared skill directory.
[Chapter 6](06-reference.md#installed-layout) describes the installed files.
[Chapter 11, “Deliver Ravel”](11-deliver-ravel.md), covers release archives,
their target checks and the formula.

The table separates the installed files from temporary test files and build
outputs.

| Task | What it builds or checks | Files it leaves |
|---|---|---|
| `task install PREFIX=…` | Release executable and notices, then the installed smoke test | Installed files at the chosen prefix, plus build outputs |
| `task smoke` | The same installation in a fresh prefix under `target/`, whose name contains a space and a non-ASCII letter | Build outputs. The script removes its temporary prefix when it exits |
| `task smoke:pair OLD=… NEW=…` | On macOS, the installed Ravel 1.1 executable `OLD` and the installation under `NEW` at the same time. Each has its own temporary home, and a guard denies both runs the real home | Nothing. The script removes its temporary homes when it exits |

The guard of `smoke:pair` denies every path in your real home directory. Thus,
the script refuses an `OLD` or `NEW` prefix there before it starts a session. A
checkout under your home holds `target/install-trial`, so install the `NEW`
prefix elsewhere for this check.

Fragment: install into a prefix outside your home, then run both versions.
These commands omit the output. `OLD` is the absolute path of an installed
Ravel 1.1 executable.

```sh
task install PREFIX=/tmp/ravel-pair-trial
task smoke:pair OLD=/absolute/path/to/ravel-1.1/bin/ravel NEW=/tmp/ravel-pair-trial
```

Before `task smoke` installs, it puts an old `libexec/ravel/sdk/` file and an
unrelated `bin/other-tool` into its fresh prefix. After the installation, the
old Ravel 1.x payload is gone and the unrelated tool is unchanged. Thus, the
check shows what an upgrade over an old installation removes and keeps.

The installed smoke test, `scripts/installed-smoke.sh`, starts with `env -i`.
It tests both the direct path to the executable and a relative symlink from
another prefix, as a package manager links an installation. Its private
`PATH` has selected system utilities and fixture programs. It refuses to run
if that `PATH` finds `bun`, `node` or a Grove command. It checks:

- the version line of the executable.
- that no `libexec/ravel` and no installation record exist.
- the installed examples and the notice file.
- ordinary, empty, multiline and option-like results, which return unchanged.
- child output, which goes to diagnostics and not to the result.
- relative Rhai imports, measured reads,
  [selection context](04-write-a-configuration-script.md#supply-selection-context)
  and `--param` values.
- the refusal of a `policy.ts` entry as `config_format_retired`, with
  status 3.
- `end-session` without session-result context, which fails.
- the cleanup of temporary session files.

These tasks passed on macOS 27.0.1 arm64 in this chapter's trial. This local
smoke check does not qualify release archives, Linux floors or a different
host.

## Check the book

Book checks are separate from `task check`. A source change can make a quoted
excerpt stale even when every Rust test passes. A change to the book can break
a command example without a change to product code.

`task docs:book` first builds the book, then serves it at
`http://127.0.0.1:8775`. The server builds the book again after a source
change. `PORT=…` selects another port. `task docs:book:build` only builds,
under `target/book`. Its script, `scripts/book.sh`, fails on an mdBook `ERROR`
or `WARN` diagnostic, even if mdBook exits with status 0. A successful build
does not show that every link resolves or that a figure is legible.

The table matches each kind of book change to the check that can find a
related defect.

| Change | Check | What the check shows |
|---|---|---|
| Chapter or book configuration | `task docs:book:build` | mdBook builds without an `ERROR` or `WARN` diagnostic |
| Figure source or shared figure module | `task docs:figures` | Fresh SVGs and layout records match the committed files |
| Complete command example | `task docs:examples` | The built executable runs each complete example and meets its stated output and status |
| Source excerpt or its source file | `task docs:excerpts` | Each excerpt still occurs as consecutive whole lines in its named tracked file |
| Prose, table, heading or figure text | `task docs:ste -- --strict` | No violation of the STE rules that a script can check, or of the shared word decisions |
| Chapter layout | `task docs:breaks -- --strict` | No prose run is longer than 450 words under the check's rules |

The figure checks use the pinned Typst, packages and fonts from `task setup`.
They render into a scratch directory. `task docs:figures:update` instead
writes the SVGs and `.layout.json` files beside their `.typ` sources. After an
intended figure change, run the update task, read the diff and view the built
page. Then run the comparison again.

The [figure convention](https://github.com/Linkuistics/Ravel/blob/main/docs/book/figures.md)
describes the shared modules and the visual checks. Equality with a fresh
render does not show that a diagram explains the right behaviour.

The example check builds Ravel first. It evaluates real Rhai configuration
scripts and runs fake programs with a private home, temporary directory and
`PATH`. Standard input is `/dev/null`. It compares the declared streams and
exit status, with only the scratch path normalised. Every complete example
runs on every platform. An `<!-- example: ... -->` comment is a page error,
because the convention has no directive.

Fragments do not run. The example check also does not compare source
excerpts. The [example convention](https://github.com/Linkuistics/Ravel/blob/main/docs/book/examples.md)
defines the labels and the output claims.

The excerpt check needs no build. Its comparison keeps spaces and
indentation. It refuses untracked sources, symbolic links and generated paths.
An exact quotation can still have an incorrect explanation. The
[excerpt convention](https://github.com/Linkuistics/Ravel/blob/main/docs/book/excerpts.md)
describes the label and the comparison.

The STE and prose-break tasks report problems without failure, unless you
pass `--strict`. They run on the book Bun from `task setup`. By default, the
STE check examines all chapter files, `SUMMARY.md`, `CONTEXT.md`, `README.md`
and the text of each figure layout record. The prose-break check examines the
chapters. A table, figure or code block breaks a prose run. Headings and lists
do not.

Neither check can show clear explanations or agreement with every STE rule. A
human read still checks meaning, order and the use of active verbs.

The checkers have their own tests. A **control** gives a checker a known bad
input and requires it to refuse that input. A **mutant** removes a safeguard
from a copy of the checker. A control then must fail against that copy. These
tests help show that a clean report has a useful meaning. The table names the
controls to run after a change to a checker.

| Task | Controls it runs |
|---|---|
| `docs:figures:test` | Unchanged figures, and changes to sources, SVGs, records, layout pins, tools and packages |
| `docs:examples:test` | Good pages, and wrong output, status, labels, build identity, environment and other example rules |
| `docs:examples:mutants` | Copies of the example checker with one safeguard removed each |
| `docs:excerpts:test` | Changed source or quotation, invalid paths and labels, then checker mutants |
| `docs:ste:test` | Planted text violations and the default file selection, then rule mutants |
| `docs:breaks:test` | Prose at and above the limit, each kind of break, and rule mutants |
| `book:integrity` | Offline mdBook download fixtures, cache reuse, recovery from a corrupt cache and refusal of a bad download |
| `book:bun:integrity` | Offline book Bun download fixtures, cache recovery and refusal of a corrupt download |

`book:integrity` needs the archive that `task setup` checked. It replaces the
download boundary and uses private tool directories. It does not test the
availability of GitHub or the book's content. Run a checker's controls when
you change that checker. Run the related content checks when you change a
chapter.

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
that Ravel supervises. If a test inherits that session's real result path,
its `end-session` call can stop the developer's session. Thus, each test
boundary removes the inherited authority before it creates its own fixture
inputs. The table maps each test boundary to its guard and the source that
applies it.

| Boundary | Guard | Source |
|---|---|---|
| Cargo commands | Force the listed session controls to empty values | `.cargo/config.toml` |
| Runner subprocess helpers | Remove those names from commands, or pass the names as runner scrub controls | `crates/keyed-launch/tests/support/mod.rs` |
| CLI, Rhai, skill and hostile fixtures | Use `env_clear()`, a private `HOME` and `TMPDIR`, and a fixture `PATH` | `crates/ravel/tests/cli.rs` and `crates/ravel/tests/support/` |
| Installed smoke | Start with `env -i`, then supply only fixture inputs | `scripts/install.sh` and `scripts/installed-smoke.sh` |

The Cargo guards cover these names:

- `GROVE_LAUNCH_DIR`, `GROVE_SIGNAL_FILE` and `GROVE_RUN_SIGNAL_FILE`.
- `HARNESS_DISPATCH_EXIT_FILE`, `HARNESS_DISPATCH_RUN_ID` and
  `HARNESS_DISPATCH_STATE_DIR`.
- `RAVEL_EXIT_FILE`, `RAVEL_RUN_ID` and `RAVEL_STATE_DIR`.

An empty value disables the inherited authority. It does not send the
authority to another path. The runner helpers also protect direct subprocess
invocations that do not go through Cargo. The CLI fixtures create temporary
configuration scripts, so tests do not need the developer's personal
configuration script.

The `GROVE_*` and `HARNESS_DISPATCH_*` names stay in these guards on purpose.
They protect tests from older enclosing-session controls. They are safety
exceptions that remain from the extraction of Ravel, not product dependencies.
Paths in source provenance and the retired-project record give historical
attribution. Ravel does not load those projects or interpret their task
trees. The [caller-reference inventory](https://github.com/Linkuistics/Ravel/blob/main/docs/caller-references.md)
classifies these references in source files and dotfiles.

For a new subprocess test, use the fixture of the chosen seam again. Supply
only the environment values that the case needs. If the case needs a result
channel, create that channel inside its temporary fixture. Never supply the
enclosing session's real result path.

## Read the evidence for a change

An evidence note explains what a particular source read or measured run
shows. Evidence notes are outside the book, with their receipts and logs. The
tasks produce check output. A contributor writes the note that connects that
output to a claim and states its limits. A task does not change a dated
evidence note. The table connects each question to a note and the source of
its observations.

| Question about a change | Evidence to read | Source of its observations |
|---|---|---|
| Which observations support the skill directory contract and the rule for a package of a default source? | [Skill discovery design evidence](https://github.com/Linkuistics/Ravel/blob/main/docs/skill-discovery-design-evidence.md) | Source reads, local trials and the checks of each stage of the 2.0.0 line |
| Did a particular release reach its remote and installed destinations? | The release evidence that the release task writes, `target/release-evidence/vX.Y.Z.json`, and the [release and installation record](https://github.com/Linkuistics/Ravel/blob/main/docs/publication-evidence.md) | The release tasks, download comparisons and installed Homebrew checks |
| Does an archive contain and run the required files? | [Archive evidence](https://github.com/Linkuistics/Ravel/blob/main/docs/archive-evidence.md) | `release:smoke`, `release:test` and `release:formula`, at the revision named in the note |
| Does the release procedure refuse an unsafe cut or resume the right step? | [Release lifecycle evidence](https://github.com/Linkuistics/Ravel/blob/main/docs/release-lifecycle-evidence.md) | `release:lifecycle:test` and the archive and formula checks |
| Which resource and failure paths motivated the later repairs? <!-- tc: depart synonym-rotation because repairs names the recorded work of the completion-failure notes, while fixture names a test input --> | [Result/job source evidence](https://github.com/Linkuistics/Ravel/blob/main/docs/result-job-source-evidence.md) | A source read at the revision named in the note, not a runtime task |
| Which checks distinguish the completion-failure repairs? | [Completion failure evidence](https://github.com/Linkuistics/Ravel/blob/main/docs/completion-failure-design-evidence.md) | Focused runner and CLI tests, `test:terminal-recovery` and `check`, with their recorded controls |
| Why does native entry follow this protocol? | [Native-entry design](https://github.com/Linkuistics/Ravel/blob/main/docs/native-entry-transition-design.md) | Source and design analysis, with later observations linked separately |
| What did the native-entry instruments measure? | [Native-entry evidence](https://github.com/Linkuistics/Ravel/blob/main/docs/native-entry-development-evidence.md) | The current native-entry and Linux checks, and archived campaign receipts |
| What did the TypeScript worker of Ravel 1.x establish? | [Runtime evidence](https://github.com/Linkuistics/Ravel/blob/main/docs/runtime-evidence.md) and [component evidence](https://github.com/Linkuistics/Ravel/blob/main/docs/component-evidence.md) | Historical runs. Each note says that its worker, SDK and Bun controls are retired |
| Where are the run records? | [Evidence directory](https://github.com/Linkuistics/Ravel/tree/main/docs/evidence/) | The linked note names each receipt set, its command and its source revision |

Start with the note's source revision and measured platform. Then read the
section that matches the claim. Earlier sections can describe work that later
sections complete. A design proposal is not an observed run. An old failure
does not show that the current source still fails, and an old pass does not
show that it still passes.

The current native-entry instruments examine the state of the executable at
entry and its cleanup. The archived campaign compared a frozen product from
before the repair, the **incumbent**, with the repaired product. Its drivers
and the protected-test comparison, R01, were retired in the change to Rhai.
The historical
[campaign records](https://github.com/Linkuistics/Ravel/blob/main/docs/native-entry-development-evidence.md#frozen-campaign-and-r01-k43)
keep their measured source revisions, plans and receipts. Those observations
do not show that the current source has the same behaviour. The table below
names the available instruments.

| Instrument | Tasks, purpose and evidence |
|---|---|
| Native entry | `native-entry:build`<br>`native-entry:front`<br>`native-entry:runner`<br>`native-entry:start`<br>`test:native-entry`<br>Build the private fixture. Run the CLI cases, the runner cases and the `start_check` cases for the runner's public API. Read the development sections of the [native-entry note](https://github.com/Linkuistics/Ravel/blob/main/docs/native-entry-development-evidence.md) |
| Linux | `native-linux:image`<br>`test:native-linux`<br>Build the Linux environment and run the current Rust, Rhai and native-entry checks, with unchanged source digests. Earlier target measurements remain in the [Linux campaign section](https://github.com/Linkuistics/Ravel/blob/main/docs/native-entry-development-evidence.md#gnulinux-immutable-campaign-k52) |

The Linux tasks call `scripts/native-linux.sh` to stage and check the files of
the current checkout without the host's Git objects. The native-entry tasks
build and select Rust tests directly. A development test pass alone does not
show the complete campaign or the support of a new platform. The measured
support covers aarch64 macOS and aarch64 Linux. x86_64 Linux is built and run
only under emulation, and native qualification is still absent.

Three tasks serve editable design views on localhost: `task docs:result-job`
on port 8774, `task docs:rhai-configuration` on port 8776 and
`task docs:skills` on port 8778. Each accepts `PORT=…` and needs network
access for Mermaid. These design views are separate from the book's Typst
figures and show no runtime result.

The [retired-project record](https://github.com/Linkuistics/Ravel/blob/main/docs/retired-project.md)
names the former actor-daemon revision for historical attribution only.

## Find a development task

`task --list` shows the public task names and short descriptions in the
current checkout. The table maps each name at this chapter's draft to its
explanation. The sections above explain the development tasks.
[Chapter 11, “Deliver Ravel”](11-deliver-ravel.md), owns the release
workflow. Its release tasks can create a release version, write remote state
and replace an installation. Thus, they are not commands for a chapter check.

| Task names | Destination |
|---|---|
| `setup` | [Prepare a checkout](#prepare-a-checkout) |
| `build`, `notices`, `notices:setup`, `notices:test`, `rhai:upgrade-proof` | [Build the executable](#build-the-executable) |
| `test`, `test:session`, `test:cli`, `test:runner`, `test:list` | [Choose a test seam](#choose-a-test-seam) |
| `harness:smoke:claude`, `harness:smoke:codex`, `harness:smoke:test` | [Choose a test seam](#choose-a-test-seam) |
| `check`, `test:terminal-recovery` | [Run the checks before handoff](#run-the-checks-before-handoff) |
| `install`, `smoke`, `smoke:pair` | [Install and test the installed files](#install-and-test-the-installed-files) |
| `docs:book`, `docs:book:build`, `docs:figures`, `docs:figures:update`, `docs:figures:test` | [Check the book](#check-the-book) |
| `docs:examples`, `docs:examples:test`, `docs:examples:mutants`, `docs:excerpts`, `docs:excerpts:test` | [Check the book](#check-the-book) |
| `docs:ste`, `docs:ste:test`, `docs:breaks`, `docs:breaks:test`, `book:integrity`, `book:bun:integrity` | [Check the book](#check-the-book) |
| `docs:result-job`, `docs:rhai-configuration`, `docs:skills` | [Read the evidence for a change](#read-the-evidence-for-a-change) |
| `native-entry:build`<br>`native-entry:front`<br>`native-entry:runner`<br>`native-entry:start`<br>`test:native-entry` | [Read the evidence for a change](#read-the-evidence-for-a-change) |
| `native-linux:image`<br>`test:native-linux` | [Read the evidence for a change](#read-the-evidence-for-a-change) |
| `release:archives`, `release:smoke`, `release:test`, `release:formula` | [Chapter 11, “Build archives before a release”](11-deliver-ravel.md#build-archives-before-a-release) |
| `release:doctor`, `release:notes`, `release:lifecycle:test` | [Chapter 11, “Prepare a version cut”](11-deliver-ravel.md#prepare-a-version-cut) |
| `release:patch`, `release:minor`, `release:major` | [Chapter 11, “Cut and deliver a release”](11-deliver-ravel.md#cut-and-deliver-a-release) |
| `release:promote` | [Chapter 11, “Promote a versioned release”](11-deliver-ravel.md#promote-a-versioned-release) |
