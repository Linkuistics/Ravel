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
