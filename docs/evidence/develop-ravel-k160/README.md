# Chapter 11 checkout trial

These records support the development commands and test fragments in
[chapter 11](../../book/src/11-develop-ravel.md). They describe one checkout
trial on 18 October 2026. They establish no new platform qualification or
reader-comprehension result.

## Subject and host

The scratch jj workspace `/tmp/ravel-k160-checkout` started at commit
`df241c28`, without a `target` directory or worker `node_modules`. The host
had macOS 27.0.1 arm64 (Darwin 27.0.0), Rust 1.99.1, Task 3.54.0 and
nextest 0.9.148. Host Cargo and Bun caches were available. No shared target
directory was copied into the checkout. The first setup fetched the pinned
tool archives from their configured sources.

Setup, build, typecheck and the initial test list ran before the scratch test
additions. Their logs record elapsed time from `/usr/bin/time -p`. These first
runs were exploratory and had no before/after digest capture.

`scratch-tests.patch` is the complete scratch source change. It adds the four
chapter fragments to their named files. It changes no product behaviour. The
focused commands passed, and `book-list.log` lists the four names that the
chapter's filter selects. No scratch test was added to the live product tree.

Before the first full check, all 3,662 tracked files were hashed with
`git ls-files -z | xargs -0 shasum -a 256`. The same enumeration after each
full check matched byte for byte. Each manifest's SHA-256 was
`281874bfa3c00b0a653b509cf2856373070a080e7956db4d7f5786c873bd3409`.
This freezes tracked source, scripts, manifests, fixtures and notes. It does
not claim an immutable host or hash the host caches and generated outputs.
The full check deliberately rebuilds the worker and probes.

## Commands and outcomes

| Record | Command | Elapsed seconds | Outcome |
|---|---|---|---|
| `setup.log` | `task setup` | 12.11 | Passed |
| `build.log` | `task build` | 4.23 | Passed |
| `typecheck.log` | `task typecheck` | 0.43 | Passed |
| `book-list.log` | `task test:list -- -E 'test(book_)'` | 0.20 | Four added cases listed |
| `runner.log` | `task test:runner -- -E 'test(book_)'` | 1.27 | Two passed |
| `cli.log` | `task test:cli -- -E 'test(book_)'` | 2.39 | One passed |
| `unit.log` | `task test:session -- --bin ravel -E 'test(book_)'` | 1.20 | One passed |
| `check.log` | `task check` | 29.33 | Earlier stages passed, workspace stopped after one failure |
| `hostile-recheck.log` | Exact failed case through `task test:hostile` | 7.57 | Passed without a change |
| `check-recheck.log` | `task check` | 109.29 | Earlier stages passed, 343 workspace tests passed, 46 skipped |

The initial unfiltered `task test:list` took 2.92 seconds. Its log is
`test-list.log`. The focused run selects four additions, whereas the full
workspace run includes the prior 339 tests and those four additions.

The failure was
`a_package_json_that_never_yields_above_the_workers_start_directory_stays_unopened_and_stalls_the_unmoved_probe`
at `crates/ravel/tests/hostile.rs:1315`: the shipped worker exceeded the
fixture's time limit. No source changed between the failure, isolated pass and
full pass. The cause remains unknown. The first run is not counted as passing.

The successful full run used the default nextest profile, not a native
qualification campaign. Ignored roles and measurements are not all included.
The terminal-recovery script separately exercised its ignored receipt consumer.
No Linux run or minimum-Rust run occurred in this trial.

After the successful full check, `du -sk` reported 942,540 KiB for `target`
and 37,480 KiB for worker `node_modules`. The source checkout occupied
30,032 KiB before setup. These figures omit host caches, shared repository
storage and peak temporary space. They are observations, not disk limits.
