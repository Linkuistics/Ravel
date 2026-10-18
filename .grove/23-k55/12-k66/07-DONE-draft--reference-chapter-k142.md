# reference-chapter-k142

## Goal

Draft chapter 6 again for the owner's read-1 findings R16 (the race), R17
(the installation in chapter 3) and R11 (runs of prose that are longer than
the limit).

## Context

- The owner's words are in `book-read-one-k70`'s log, “Owner findings
  R10–R17”.
- **R16.** Line 266 at `1ef02398`: “A repeated valid call succeeds, but it
  does not replace that result.” Use the race wording that draft
  `what-ravel-is-for-and-its-actors-k134` wrote in the brief of node
  `what-ravel-is-for-and-its-actors-k62`, in the form of a reference entry.
- **R17.** Line 12 (“The entries describe the checkout build that this book
  uses”), line 960 and the installed-layout table link to
  `#obtain-the-build-used-by-this-chapter`. The entries now describe the
  version that the book names, installed with Homebrew. Use the new anchor
  from the brief of node `a-complete-session-k63`.
- The installed layout keeps its paths below `PREFIX` in a Homebrew
  installation. Check this against the formula (`bin.install "bin/ravel"` and
  `libexec.install "libexec/ravel"`). `task install PREFIX=…` is a
  contributor task. Keep it only where an entry needs it.
- **R11.** The command-line entries are a run of 2,200 words (lines 1–295).
  Measure with the check from `book-prose-breaks-check-k133`. A table of the
  options of each verb can carry the entries. Each entry stays a direct
  entry: it has the local context that it needs, and it links back to where
  the book teaches its subject.
- This is the first draft re-run on this document. Draft k94 was the second
  part of the first draft. Set art and set proof serve all six chapters, so
  this node gets no art or proof leaf.

## Done when

- Chapter 6 states the race in agreement with chapter 2.
- Every link to the installation in chapter 3 works. A search with a control
  shows it.
- The prose check finds no run that is longer than the limit in chapter 6.
- Every CLI input and installed path of inventories 1 and 2 still has its
  entry.
- `task docs:book:build` and `task docs:examples` pass.

## Notes

## Decisions (running log)

- The structure brief is `docs/book/contract.md`, section 1. It names “a
  16-year-old software engineer with no background in Ravel”, the outcome
  “Look up any CLI input, setting, bound, data shape, environment variable
  or installed path in chapter 6”, and the ordered chapter plan. Its
  “Emphasis” and “Exclusions” state the priorities and scope. The document
  node names this contract, so the draft precondition holds.
- Keep all entry headings and teaching links. Replace selected groups of
  prose with tables of comparable inputs, limits or effects. The initial
  strict prose check finds one run of 2,118 words, lines 3–295. This agrees
  with k133's count under the current counting rules.
- Use the race wording handed forward by k134. The source sends through a
  temporary file and a no-replace hard link. A second sender checks the
  existing result, but directory removal or process shutdown can stop it.
  Do not promise that a repeated call succeeds.
- Describe the Homebrew installation of the version named in chapter 3.
  The formula's `bin.install "bin/ravel"` and
  `libexec.install "libexec/ravel"` preserve the relative paths. Keep the
  contributor install task as a labelled note in the installed-layout
  entry. Change every old chapter-3 installation anchor in chapter 6.
- Copy-edit k143 is already live. Do not add a duplicate successor. Set
  art k144 and set proof k147 serve this document under the owner's
  authorised correction run. No architecture decision changes.
- The final structure keeps all 50 headings. Nine new page tables compare
  help/version requests, prompt sources, task fields, parameter rules,
  timeout values, environment grants, confined access, channel parents and
  `end-session` inputs. Each entry keeps its local explanation and teaching
  links. No new figure is needed.

## Validation

- Read the whole chapter and checked the changed claims against the CLI
  definitions, completion sender, settings, bounds, worker grants, both
  specifications, the formula and the contributor installation script.
  The result entry agrees with chapter 2 and its node's R16 handoff. It
  distinguishes the send from successful cleanup and return.
- Tier 2 graph project:
  `Users-antony-Development-Ravel.simplify-and-componentize`, generation
  `2026-10-18T06:49:36Z`. The bounded symbol searches ended without more
  pages. The `end_session` trace reaches `publish_result`; its source
  confirms the first-result rule and the possible I/O failure of a later
  call. Coverage checks report no recorded gap and matching metadata for
  the evidence paths. That signal is best effort, not proof of completeness.
  Direct source reads establish the claims. No in-session reviewer was used.
- Fresh help for `ravel`, `start-session` and `end-session`, with
  `crates/ravel/src/cli.rs`, enumerates two verbs, fifteen session options,
  the result argument and the help/version forms. All retain entries.
- Source enumeration of `worker/sdk/index.ts` gives 19 public exports.
  `worker/examples/static.ts` gives six. All have table entries. The
  three settings, six bounds and the SDK's data and host fields still
  match their reference entries. Every byte from “Exit statuses” through
  “Shipped sample” is unchanged, preserving the stream, status,
  environment, diagnostic, configuration and import entries.
- The archive manifest in `scripts/release-common.sh` enumerates eleven
  files. All retain entries: nine pair/assets paths plus README/LICENSE.
  The Homebrew formula keeps the `bin` and `libexec/ravel` layout.
- All seven installation links in chapter 6 use
  `#install-ravel-and-check-its-version`. The old-anchor search is clean
  there. The same `rg -n -F` search finds it in the saved parent revision,
  the book brief's handoff and a scratch copy with one old anchor restored.
  The rendered-link check rejects that obsolete anchor.
- Final `task docs:breaks -- --strict docs/book/src/06-reference.md`:
  zero runs over 450 words. Final `task docs:ste -- --strict
  docs/book/src/06-reference.md`: zero problems. Earlier STE checks found
  two new “-ing” words and a seven-sentence paragraph; both corrections
  preceded final validation.
- `task docs:book:build` passes without warnings. All 341 links and images
  in the rendered chapter resolve. Missing-file, missing-image and
  obsolete-anchor controls fail as expected.
- `task docs:examples`: 13 passed, zero failed, zero skipped, six labelled
  fragments not run, zero page errors. The final checks read unchanged
  inputs: SHA-256 before/after agrees for 197 tracked crate, script, book,
  root-file and build-configuration subjects. Scratch digests and the
  baseline copy are under `target/k142-checks/`.
- These are source and mechanical checks. They establish no new-reader
  comprehension, real-agent execution, interactive-terminal behaviour,
  Homebrew installation run or native x86-64 qualification. This draft
  makes no visual-inspection claim. Set art and proof retain those charters.
