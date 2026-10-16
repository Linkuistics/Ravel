# book-examples-k60

## Goal

Make "every complete example runs" a checked fact: a Taskfile task extracts each
complete example from the book's Markdown, runs it through the seam Ravel's CLI
tests use, and compares its output exactly, with controls seen to fail.

## Context

- Contract, "Examples policy": `docs/book/contract.md`. The built `ravel`, real
  TypeScript policy evaluation, fake agent programs on a private PATH, a cleared
  environment and a temporary HOME and TMPDIR; no real agent or model; every
  example introduced by a label beginning "Complete example" or "Fragment"; a
  route to a real agent program is validated through a fake executable of the
  same name.
- The seam to reuse is `Fixture` in `crates/ravel/tests/cli.rs` (`env_clear`,
  temporary `home`, `work` and `tmp`, `PATH=/usr/bin:/bin`, the built front,
  `/bin/sh` fixtures that call `end-session`). Its `routed_result_is_exact_*`
  and `refused_policy_never_launches_*` tests are the model for what an example
  asserts.
- Examples appear in chapters as literal Markdown fenced blocks, because the
  book is also read on GitHub and in an editor, where an mdBook `{{#include}}`
  does not run. So the Markdown is the source, and the checker reads the
  Markdown.
- Chapters 3, 4 and 5 are the first consumers; none exists yet, so this leaf
  proves the checker on a fixture page that is not in the book.

## Done when

- A convention for marking a complete example in Markdown is fixed and
  recorded: how its files (policy, fake agents, prompt, arguments) and its
  expected stdout, stderr and exit status are identified in the Markdown so a
  reader sees ordinary labelled code blocks and the checker finds the same
  blocks. Anything the checker needs that a reader should not see goes in HTML
  comments, which render in neither mdBook nor GitHub.
- `task docs:examples` builds what it needs (the existing front and worker), runs
  every complete example under `docs/book/src/`, and compares stdout, exit
  status and the stderr text the example claims, byte for byte. Output that is
  not stable (temporary paths, PIDs, timing) has a stated normalisation rule
  that is visible to the reader or a block the example may omit; nothing is
  compared loosely by default.
- An example needing a platform feature (confinement needs `sandbox-exec` or
  `bwrap`) reports a skip with its reason, never a pass, and the summary line
  counts passed, failed and skipped separately.
- Controls seen to fail: a changed expected stdout, a changed exit status, an
  example block with no expected output, a missing fake program, a stale
  `ravel` build and a Markdown page with an unlabelled runnable block each make
  the task fail. A page with one passing example passes. Each control is run
  and its failure shown here.
- The implementation language is chosen and recorded with the reason. The
  seam's neighbours are Rust nextest cases in `crates/ravel/tests`; a shell
  script under `scripts/` with a `task` wrapper is the alternative. Whichever is
  chosen, `task check` is not made to depend on the book's chapters (a Part I
  edit must not turn `task check` red unless the owner asks for that), or the
  decision to include it is recorded as the owner's.
- Confirm the checker never reads `~/.config/ravel`, a real agent or a real
  model, by running it with `HOME` pointing at an empty directory and a PATH
  that holds only fakes, and by showing the examples still pass.

## Notes

- Also settle, and record for chapter 5, how the checker treats an example that
  must be cancelled by a signal during selection or supervision. If the seam
  cannot run it deterministically without a PTY, the answer is "label it a
  Fragment and say what it omits", and the convention says so.
- The source-excerpt check (exact walkthrough excerpts) serves chapter 8 and
  waits for the second half; it is not this leaf's.
- One in-session reviewer is allowed by the spine's rule, and this is the leaf
  where it is best spent: whether a control would have failed against the
  broken instrument.
