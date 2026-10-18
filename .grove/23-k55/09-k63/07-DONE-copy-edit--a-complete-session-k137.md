# a-complete-session-k137

## Goal

Copy-edit chapter 3 after draft `a-complete-session-k136`, under the STE
writing rules and the word decisions in `docs/book/ste.md`.

## Context

- The draft's new text for R17 and R11 has had no copy-edit.
- Apply “unchanged” and “return no result to the caller” (R12 and R13 in
  `docs/book/ste.md`, “The result”) wherever chapter 3 states those concepts.
- Do not change the version placeholder. The release replaces it as text.
  This node's brief gives its exact form.
- This is the second copy-edit re-run on this document. The owner authorised
  it on 2026-10-18 in `book-read-one-k70`.

## Done when

- `task docs:ste -- --strict` passes on chapter 3.
- The prose check finds no run that is longer than the limit in chapter 3.
- The version placeholder has the form that this node's brief gives.
- `task docs:book:build` and `task docs:examples` pass.

## Notes

### Verification

Fresh checks on 2026-10-18, after the last edit:

- `task docs:ste -- --strict docs/book/src/03-a-complete-session.md`: 0
  problems. The edits first made three paragraphs of seven sentences. The
  check reported them, and each paragraph was split at its change of topic
  or joined into fewer sentences.
- `task docs:breaks -- --strict docs/book/src/03-a-complete-session.md`: no
  run is longer than 450 words.
- `task docs:ste:test` and `task docs:breaks:test`: each mutant made a
  control fail, so both clean results come from checks that can fail.
- `task docs:book:build`: the HTML build completed, with no warning.
- `task docs:examples`: 13 passed, 0 failed, 0 skipped, six fragments not
  run, no page errors. The SHA-256 digests of chapter 3,
  `docs/book/examples.md`, `docs/book/ste.md` and `scripts/ste.ts` were the
  same before and after the run (`target/k137-validation/`).
- The placeholder `ravel @RAVEL_BOOK_VERSION@` is the only line in `docs`,
  `scripts`, `README.md` and `Taskfile.yml` that contains the token. It stays
  in its `text` fence. The prose does not repeat the token, so a text
  replacement by k145 changes only that line.

Two claims were checked against the subject before the text used them.
`brew --prefix ravel` prints `/opt/homebrew/opt/ravel` on this host, and that
directory holds `bin/ravel` and `libexec/ravel/ravel-policy`.
`crates/keyed-launch/src/run.rs` has two waits, `grace` (from the channel to
SIGTERM) and `kill_grace` (from SIGTERM to SIGKILL). No heading, anchor, code
block, example, figure or alt text changed. No in-session reviewer was used:
the changes are word-level, and the two new facts were checked at the source.
This is not a reader-comprehension or real-installation result.

## Decisions (running log)

- R12: the chapter states what Ravel writes in two places. Both now say that
  Ravel writes the result unchanged to standard output (the example output,
  and the “In words” paragraph after Figure 3.1). The example paragraph also
  says that Ravel does not interpret the result, as `ste.md` gives it.
- R13: the refusal, cancellation and natural-exit items of “Recognise the
  other outcomes” now say that the outcome returns no result to the caller.
  The failure item does not, because a failed final write can leave the
  start of the result on standard output. The figures and captions keep
  “suppress”, because figure text is the art stage's.
- “Prefix” becomes “the start of the result”, as in chapter 1.
- The instruction “Install Ravel, then check its version” repeated the
  heading and was removed. The instruction to check the version before the
  examples moved to the sentence that introduces the version line, so that
  the step comes before its explanation. “Unreleased” became “not from a
  release”.
- “The installation contains” became a claim that the reader can check:
  “The directory that `brew --prefix ravel` prints contains …”.
- “Keep” has the result meaning in `ste.md`. Three other uses changed:
  Ravel “writes” the EPERM report to standard error, “uses the caller's
  standard output only for” the result, and the outcome “is” the original
  refusal or failure.
- Figures of speech were replaced by mechanisms: “all together” (one signal
  to the group), “the shebang line makes this program a shell script” (the
  first line tells the operating system to run the file with `/bin/sh`),
  “the two boundaries that a caller can trust” (two orders that Ravel
  guarantees), “cancellation comes first” (has priority), “tamper with”
  (change).
- The grace-stage table names the second wait “a second grace period”, from
  `kill_grace`. The ambiguous “its `bin` subdirectory” after “the shell” is
  now “a `bin` subdirectory in it”. “Matched worker” is now “the worker that
  matches it”, as in the installation section. `end-session` “sends” the
  value into the channel, not “puts”.
- The routing notice is now defined before the table that uses the term, and
  “the owner's labels” names the three fields.
- Last act: cut nothing. The node brief says that set art
  `book-first-half-k144` and set proof `book-first-half-k147` serve this
  document, and both are live under the book node.
- Close of node `a-complete-session-k63`: its `Done when` holds for this
  run's node part. The four open handoffs in its `## Handed forward` address
  leaves under the book node, so they move to the book brief, and the node's
  list is cleared.
