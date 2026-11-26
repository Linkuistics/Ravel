# Rhai book gate evidence

This record supports `rhai-book-gates-k123` on 14 November 2026.
It supplies executable evidence for the remaining whole-book proof under
`rhai-book-inside-k122`. It does not complete that proof.

## Subjects and environment

The declared subject set contains 354 files. `subjects.txt` records the paths.
The set includes tracked files under `docs/book/`, `scripts/`, `crates/`,
`share/` and `.cargo/`. It also includes `Taskfile.yml`, the Cargo manifests,
`rust-toolchain.toml`, `README.md`, `CONTEXT.md` and every file under
`target/tools/` after setup.

`inputs-before.sha256` and `inputs-after.sha256` contain the SHA-256 digests.
`subjects-after.txt` records a fresh enumeration with the same selection.
Both comparisons passed, with 354 unchanged subjects. `comparisons.txt` records
their zero exit statuses. The fresh enumeration retains jj's source-path order
and appends the sorted tool paths, as the initial enumeration did.
The logs and this evidence directory are outside the subject set.

The host is Darwin arm64. The tool versions are Rust and Cargo 1.99.1,
Task 3.54.0, mdBook 0.5.4, Bun 1.4.2 and Ravel 2.0.0.
The figure check uses Typst 0.15.1, CeTZ 0.4.2 and DejaVu 2.37.
`mise exec -- task setup` completed before the subject digests were taken.

## Commands and observations

These commands use the repository Taskfile:

```sh
mise exec -- task docs:book:build docs:examples docs:excerpts docs:figures docs:ste docs:breaks
mise exec -- task book:integrity docs:figures:test docs:examples:test docs:examples:mutants docs:excerpts:test docs:ste:test docs:breaks:test
```

`checks.log` records the first command. `checks.status` records its exit status.
The observed results are:

| Check | Result |
| --- | --- |
| Book build | Passed, with no reported warning or error |
| Complete examples | 27 passed, 0 failed, 0 page errors |
| Fragments | 73 identified and not executed |
| Source excerpts | 22 passed, 0 failed, 0 page errors |
| Figures | All 21 match the pinned renderer output |
| STE checker | 0 reported problems in 35 files |
| Prose-break checker | 0 runs over 450 words in 11 chapters |

The STE and prose-break commands report findings without a strict exit gate.
Their summaries, rather than their exit status alone, establish these results.

`controls.log` records the second command. `controls.status` records its exit
status. Both commands exited 0. The controls exercise corrupt downloads, changed figures, invalid example
pages, changed excerpts, planted STE problems and excessive prose runs.
The mutant runs remove safeguards from temporary copies of the checkers.
Each surviving safeguard must reject its corresponding wrong subject.
The run passed all 42 example controls. It killed all 30 example mutants,
24 excerpt mutants, 12 STE mutants and 17 prose-break mutants.

The control command first ran without a retained process exit status.
Its output reached all expected final markers. A second complete run replaced
that log and retained the exit status, so this record relies on the second run.
Each command has its own output file. Neither run changes the book or checker
sources.

## Limits and remaining work

These checks establish only the properties their instruments measure.
They do not establish reader comprehension, complete source-derived coverage,
current external links or the correctness of every statement in the book.
The final proof must read all eleven chapters, reconcile the release claims,
complete the contract inventories and arrange owner Read 2 outside the editorial
node. Historical evidence under `docs/evidence/ravel-book-k311/` remains frozen.

The digest set is explicit, rather than a claim to capture every environmental
input. It excludes system libraries, inherited environment variables and the
compiled debug binary. The example tasks rebuild that binary from the recorded
sources. No before-run digest of that binary was taken.

This run supplies native arm64 macOS evidence only. It adds no Linux execution,
real harness smoke, fresh-reader exercise or published GitHub rendering check.
It changes no book prose and no checker implementation.

The graph reported generation `2026-11-13T15:17:59Z` with matching generation
metadata. Coverage is best effort. The reported gap in
`scripts/examples.test.sh`, lines 1–779, was read directly before relying on its
controls. Excluded generated assets and unused QEMU scripts support no new
behavioural claim here.
