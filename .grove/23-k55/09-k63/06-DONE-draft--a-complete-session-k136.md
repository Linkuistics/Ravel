# a-complete-session-k136

## Goal

Draft chapter 3 again for the owner's read-1 findings R17 and R11. The reader
installs Ravel with Homebrew and checks its version. No run of prose is
longer than the limit.

## Context

- The owner's words, the choice “Set at release time” and the locations at
  `1ef02398` are in `book-read-one-k70`'s log. The contract records the
  decision in the chapter 3 row of the section plan and in “The reader's
  Ravel” under the examples policy.
- **R17.** Replace the section “Obtain the build used by this chapter”
  (lines 17–66). Tell the reader to run `brew install linkuistics/taps/ravel`,
  then `ravel --version`. The output must name the version that the book
  names. Name that version in exactly one place in the chapter, as a
  placeholder, for example as the expected output of `ravel --version` in a
  `text` block.
- The later leaf `book-release-version-k145` makes the release write its own
  version into that place and check it. Thus, keep the placeholder on one
  short line that a text replacement can find. Write its exact form in this
  node's brief for k145. Tell the reader what to do when the installed
  version is different: the book on GitHub Pages for each release describes
  that release.
- Check these facts again and keep them. The formula installs from release
  archives for aarch64 macOS, aarch64 Linux and x86_64 Linux. The installed
  pair needs no Bun or Node on the host. The support statement stays: x86_64
  Linux has execution evidence only from emulation.
- The developer setup (`task setup`, `task install`) leaves chapter 3. The
  README's “Develop” section has it until chapter 11.
- The new heading changes the anchor `#obtain-the-build-used-by-this-chapter`.
  These pages link to it: chapter 4 (line 11), chapter 5 (line 13) and
  chapter 6 (lines 12 and 960, and the installed-layout table). Their drafts
  come later in this run and change those links. Write the new anchor in
  this node's brief for them.
- The complete examples still run through `task docs:examples` with the
  build of the checkout. A sentence that needs the old `PATH` setup, for
  example “In the shell where you set `PATH`”, changes.
- **R11.** Chapter 3 has a run of 1,745 words (lines 136–330). Measure with
  the check from `book-prose-breaks-check-k133`. Add tables where items
  compare on the same attributes. Write each figure need under `## Handed
  forward` in this node's brief, with its reader question, for set art
  `book-first-half-k144`.
- This is the first draft re-run on this document. Set art and set proof
  serve all six chapters, so this node gets no art or proof leaf.

## Done when

- Chapter 3 tells the reader to install Ravel with Homebrew and to check the
  version against one placeholder.
- The prose check finds no run that is longer than the limit in chapter 3.
- This node's brief has the new anchor, the exact form of the placeholder and
  each figure need.
- `task docs:book:build` and `task docs:examples` pass.

## Notes

### Source checks and validation

The whole chapter was read against its structure brief and subject. Its
order remains installation, the complete example, selection, launch, the
sent result and cleanup, then the other outcomes. The tables compare facts
at those points. They preserve the distinction between a sent value and
success after cleanup and the final write.

The source check used Tier 2 graph evidence from project
`Users-antony-Development-Ravel.simplify-and-componentize`, generation
`2026-10-18T06:15:24Z`. All checked paths had no recorded coverage issue and
matching metadata. This is a best-effort signal, not proof of completeness.
The bounded searches had no further page. The outbound trace of `run` had
43 callees, with no truncation. Exact source reads established these facts:

- `crates/ravel/src/run.rs`: selection precedes channel allocation and
  launch; the managed process receives the channel; cleanup and outcome
  selection precede the final write and flush.
- `crates/ravel/src/worker.rs`, `evaluate`: the private directory, cleared
  environment and socket, followed by confirmed reap and directory removal
  before a successful selection returns.
- `crates/ravel/src/choice.rs`, `choose_prepared`, and
  `crates/ravel/worker/sdk/index.ts`, `definePolicy`: selection composition
  and the unchanged typed policy object.
- `crates/ravel/src/completion.rs`, `end_session` and `publish_result`:
  inherited path, complete temporary value and first-value retention.
- `crates/keyed-launch/src/run.rs`, `watch` and `supervise_reported`: the
  grace stages, group stop before reap, terminal recovery, group confirmation
  and retained operation reports.

The session execution and result/job completion specifications, the three
ADRs and `CONTEXT.md` were read for the chapter's remaining claims and limits.
The local formula from `brew cat --formula linkuistics/taps/ravel` agrees with
`scripts/templates/ravel.rb.tmpl` on the three archive targets and paired
layout. The template, installed-boundary specification, release guide and
Taskfile confirm that the installed pair needs no host Bun or Node.
The release guide's old initial-version prose was not used as the book's
version. The direct web fetch of the formula failed; the local tap and
repository template supplied the evidence instead. No installation occurred.

Fresh checks passed on 2026-10-18:

- `task docs:breaks -- --strict docs/book/src/03-a-complete-session.md`:
  no run exceeds 450 words. The same check failed before the edit.
- `task docs:ste -- --strict docs/book/src/03-a-complete-session.md`:
  no reported problem. It first caught “matching” in new prose; that sentence
  was corrected before the final run.
- `task docs:book:build`: the HTML build completed without warnings. The
  rendered chapter has the new anchor and the placeholder line.
- `task docs:examples`: 13 passed, none failed or skipped, six fragments
  not run and no page errors. The chapter 3 complete example passed at line
  68. Homebrew installation is a labelled fragment, outside this check.

The 190 tracked subjects under `docs/book`, `crates` and `scripts`, plus the
Taskfile, Cargo manifests/lock and Cargo configuration, had identical SHA-256
records before and after validation. Local logs, the exact subject list and
the two digest records are under `target/k136-validation/`. No code, example
fixture or figure changed. This is no release, terminal-operation or reader
comprehension result. The inherited links in chapters 4–6 still need the
scheduled drafts. The node brief carries their new anchor and the release
placeholder. Copy-edit k137 is already live, so no successor is added. The
node remains open, and no parent cascade or ADR change is required.

## Decisions (running log)

- The structure brief is `docs/book/contract.md`. It names the reader as a
  “16-year-old software engineer with no background in Ravel”. Its chapter 3
  row requires installation, a version check, a real session with a fake,
  then an explanation of the stages. “What success means” supplies the
  emphasis. “Exclusions” excludes agent installation and evidence campaigns.
- Use the heading “Install Ravel and check its version”, with anchor
  `#install-ravel-and-check-its-version`. The one version line is
  `ravel @RAVEL_BOOK_VERSION@` in a `text` fence. The later release leaf owns
  its replacement and checks. A placeholder means an unreleased copy of the
  book, not an expected output from an installed release.
- Retain the chapter's conceptual order. Replace or group comparable facts
  in tables: selected fields, worker and managed-process roles, session and
  process-group terms, and shutdown stages. The initial prose check failed
  on one run of 1,721 words at lines 136–330. No new figure is required for
  these comparisons. The two existing figures keep their questions.
