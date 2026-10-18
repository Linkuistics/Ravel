# writing-an-owner-policy-k139

## Goal

Copy-edit chapter 4 after draft `writing-an-owner-policy-k138`, under the STE
writing rules and the word decisions in `docs/book/ste.md`.

## Context

- The draft's new text for R17 and R11 has had no copy-edit.
- Apply “unchanged” and “return no result to the caller” (R12 and R13 in
  `docs/book/ste.md`, “The result”) wherever chapter 4 states those concepts.
- This is the second copy-edit re-run on this document. The owner authorised
  it on 2026-10-18 in `book-read-one-k70`.

## Done when

- `task docs:ste -- --strict` passes on chapter 4.
- The prose check finds no run that is longer than the limit in chapter 4.
- `task docs:book:build` and `task docs:examples` pass.

## Notes

## Decisions (running log)

- Precedent: copy-edit `a-complete-session-k137` (chapter 3) for R12, R13,
  the Ravel-subject “keep” and the checkable installation claim.
- R12: “without a change” becomes “unchanged” for the helper's return value,
  the request's `prompt`, the grants fake's report and the instructions to
  end the session in the agent transcript. The first example's paragraph now
  says that Ravel writes the opaque result `finished` unchanged to standard
  output.
- R13: the first example's refusals and the agent exit without a sent result
  now “return no result to the caller”. The agent sentence said “Ravel
  reports no result”, but Ravel reports a diagnostic in that case.
- “Keep” has the result meaning in `ste.md`. Worker output after
  `output_limit`: Ravel “captures” the first 262,144 bytes, as the
  diagnostics capture in `crates/ravel/src/worker.rs` does.
- “An installation also contains” becomes the checkable claim “The directory
  that `brew --prefix ravel` prints also contains”, as in chapter 3. The
  three declaration and source paths are in `scripts/release-common.sh` and
  `crates/ravel/scripts/installed-smoke.sh`.
- “Harness” is a term in `CONTEXT.md`. Its first use on the page (k138's
  route introduction) now links to chapter 2. “Lend the terminal” links to
  chapter 3, where the book defines it, as chapter 5 does.
- One spelling: “bad UTF-8” and “bad JSON” become “invalid UTF-8” and
  “invalid JSON”, as in the choice-file and settings lists.
- Figures of speech replaced by mechanisms: “stays free for the result”
  (contains only the result), “a complete policy around this table” (that
  uses this table), “a promise keeps work alive” (pending work continues),
  “carries the instructions” (contains), “really launches” (launches).
- “Finds an own entry” becomes “finds the table's own entry, not an
  inherited property”, from `Object.hasOwn` in `examples/static.ts`.
- “In usual operation” was a hedge without a condition. The condition is
  “To run a policy”.
- The repeated sentence “They hold facts and attributed judgements” before
  the context-document table was removed. The paragraph before it states it.
- Last act: cut nothing. The node brief says that set art
  `book-first-half-k144` and set proof `book-first-half-k147` serve this
  document, and both are live under the book node.
- Close of node `writing-an-owner-policy-k64`: its `Done when` holds for
  this run's node part. Its one open handoff (set art k144) addresses a leaf
  under the book node, so it moves to the book brief, and the node's list
  is cleared. No live review names this leaf or the node.
  `GROVE_PROVIDER_EVIDENCE` is unset, so no Creator line is written.

## Verification

- `task docs:ste -- --strict` on chapter 4: 0 problems. The first run after
  the edits failed on two problems in the new text (an “-ing” word and a
  26-word sentence), which were corrected. `task docs:ste:test` passes.
- `task docs:breaks -- --strict` on chapter 4: 0 runs longer than 450 words.
  `task docs:breaks:test` passes.
- `task docs:book:build` passes without a warning. `task docs:examples`:
  13 passed, 0 failed, 0 skipped, 6 fragments not run, 0 page errors.
- A comparison with `@-` shows that all fenced code blocks and all headings
  are unchanged.
- The old R12/R13 words (“without a change”, “exact bytes”, “give(s) no
  result”, “reports no result”, a bare “returns no result.”, “bad UTF-8/JSON”)
  match five lines in the parent version of chapter 4 and none now.
- The two new links resolve to IDs in the built HTML
  (`02-actors-and-words.html#the-managed-process-and-its-result`,
  `03-a-complete-session.html#launch-the-managed-job`). A missing-anchor
  control is reported missing.

No in-session reviewer was used. No ADR decision changed. These checks do
not establish a visual read, real-agent behaviour or new-reader
comprehension.
