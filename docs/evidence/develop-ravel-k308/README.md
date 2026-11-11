# Chapters 10 and 11 for 2.0.0: checkout, release and book checks

These records support the draft of `10-develop-ravel.md` and
`11-deliver-ravel.md` for Ravel 2.0.0, on 12 November 2026, by Grove leaf
`ravel-book-inside-k308`. The source is the content of
`47c6422effa0`, “Draft chapter 9 of the book for 2.0.0”. The trial of
`task check` ran at the working-copy commit `a1b9dda0e8c4`, which had the same
content.

The host is macOS 27.0.1, arm64, with Bash 5.3.20, Rust 1.99.1 through rustup,
Task 3.54.0 and cargo-nextest 0.9.151. The release tools are Homebrew
7.0.13-45, Zig 0.17.1, cargo-zigbuild 0.23.4, cargo-release 1.1.2 and
jj 0.47.1. Docker 29.9.2
reported `linux/arm64` with kernel 7.0.14-linuxkit. The tools of `task setup`
and the host Cargo cache were present. This was not a clean-machine trial.

## Chapter 10: the checkout

| Command | Elapsed | Result | Log |
|---|---|---|---|
| `rustup toolchain install` | Under 1 s | Rust 1.99.1 was already installed. Output not kept | None |
| `task setup` | 3 s | Checked the cached book tools. Downloaded nothing | `setup.log` |
| `task check` | 2 min 49 s | All seven stages passed. 713 tests passed and 48 were skipped | `check.log` |
| `task install PREFIX=target/install-trial` | 12 s | Installed `bin/ravel`, three examples and the notice file. The installed smoke test passed, direct and through a symlink | `install.log` |
| `task smoke` | 7 s | Passed, direct and through a symlink | `smoke.log` |

The five `book_` tests of the chapter's “Add a test” were added to the working
copy. The chapter's five focused commands listed them and ran each one. All
passed. `jj restore` then removed the five test edits. A copy of
`book_fake_model_answer_chooses_the_route` that expected `deep-route` failed
with `left: ["routed", "quick-route"]`. The command output was not kept,
because the chapter's trial table gives each count and time.

## Chapter 11: the release checks

`release-trials.txt` gives the status and elapsed time of each task, and a
digest of `scripts/`, `crates/`, the manifests, `release.toml` and chapter 3.
The digest was the same before and after.

| Command | Elapsed | Result | Log |
|---|---|---|---|
| `task release:doctor` | 3 s | All prerequisites met | `release-doctor.log` |
| `task release:test` | 5 s | Passed | `release-test.log` |
| `task release:smoke` | 53 s | All three targets passed at all four routes, with the glibc and CPU controls | `release-smoke.log` |
| `task release:formula` | 8 s | Both channels and the deprecated `ravel@2.rb` passed | `release-formula.log`, `release-formula-detail.log` |
| `task release:lifecycle:test`, first run | 2 min 7 s | Failed at the mutant “install keeps cleanup” | `release-lifecycle-test-first.log` |
| `task release:lifecycle:test`, second run | 8 min 5 s | Passed. Each of the 48 mutants failed a control | `release-lifecycle-test-second.log` |

The first lifecycle run inherited `HOMEBREW_NO_INSTALL_CLEANUP=1` from the
trial script. The fake `brew` of `scripts/release-channel.test.sh` logs that
variable from its environment. Thus, the mutant that removes the export from
`scripts/release-install.sh` passed every control. The second run removed the
variable with `env -u` and changed nothing else. The pair shows that the
suite needs an environment without that variable. Chapter 11 says so.

No check ran `release:patch`, `release:minor`, `release:major` or
`release:promote`. No push, tag, tap change, permanent installation or Pages
deployment occurred.

## The book

| Command | Result | Log |
|---|---|---|
| `task docs:book:build` | Passed with no warning | `final-docs-book-build.log` |
| `task docs:figures` | 18 figures match their sources | `final-docs-figures.log` |
| `task docs:examples` | 27 passed, 0 failed, 72 fragments not run | `final-docs-examples.log` |
| `task docs:excerpts` | 22 passed, 0 failed | `final-docs-excerpts.log` |
| `task docs:ste -- --strict` | 0 problems in 32 files | `final-docs-ste.log` |
| `task docs:breaks -- --strict` | 0 runs longer than 450 words in 11 files | `final-docs-breaks.log` |

The new complete example of chapter 11 shows the refusal of a lone
`policy.ts` as `config_format_retired`, status 3. A copy that claimed status 0
failed with “exit status of command 1 is 3, the example claims 0”.

These book logs come from the run after the last change to the chapters.
That change followed the one in-session review of chapter 11, which found
13 problems in its recovery, channel and formula statements.
