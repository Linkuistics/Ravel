# a-complete-session-k82


## Goal

Copy-edit the whole of `docs/book/src/03-a-complete-session.md` under the
prose contract in `docs/book/contract.md`, preserving the chapter's technical
teaching and runnable example.



## Context

Read chapters 1–2 for established terms and the document brief's handoffs.
The draft introduces the first complete example, then explains admission,
evaluation and judgement, worker teardown before launch, process groups and
terminal lending, publication, reaping and final settlement. Its source and
validation record is in `a-complete-session-k72`.

The example uses the checker convention in `docs/book/examples.md`: exact
stdout and status, with the explicitly labelled stable stderr excerpt. The
chapter explains the additional process-dependent operation reports observed
on macOS. Keep that distinction. The installation fragment uses the current
checkout in a private prefix, with Java 11+ and tar's bzip2 support named
because the shared setup prepares the book's figure tools too.

The document brief assigns the sequence and outcome state-machine figures
to art; do not pre-empt that stage. Forward chapter mentions remain plain
text until their targets exist, under the book node's established convention.

## Done when

- The whole chapter satisfies the prose contract, with consistent terms and
  first-use links, and no change to the example's behaviour or the source
  sequence.
- `task docs:examples` and `task docs:book:build` pass; any correction outside
  this stage's charter is routed under the editorial family procedure.
- Cut the ordinary art successor with the document's bare stem, unless a live
  later sibling already holds that stage, then retire and commit this leaf.

## Notes

## Decisions (running log)

- Charter read against the prose contract, `CONTEXT.md` and chapters 1–2 as
  precedent. No structural finding: the section plan, headings, example and
  source sequence stand, so no draft re-run is cut.
- One spelling per term: “the managed group” becomes “the managed process's
  group”; the worker “changes to `/`” and the selected program “starts in the
  caller's working directory”, as chapter 2 says; the worker “embeds the Bun
  runtime”; settings live at `$HOME/.config/ravel/settings.json`.
- First-use links: “confined” now links to chapter 2's trust and confinement
  section; the managed process's `RAVEL_EXIT_FILE` path is named as its
  session-result context. “Job” is defined at the process-group paragraph in
  its job-control sense, since both headings use it; it is an OS term, not a
  Ravel term, so `CONTEXT.md` is unchanged (its Session entry still forbids
  calling a session a job, and the chapter does not).
- Figures of speech replaced by mechanisms, each checked against source:
  “lends the foreground” is now `tcsetpgrp` in words (the managed group made
  the foreground group, only when Ravel's own group holds it, otherwise no
  lending — `keyed-launch/src/run.rs` `hold_before_spawn`, pre-exec
  handoff); “terminal-ownership rules” becomes the restoring action with the
  per-holder rule deferred to chapter 9 (`reclaim`); “the direct child's
  identity reserves the group's numeric identity” becomes group ID = leader
  PID, not reused before reap (`command.process_group(0)`); “once the
  conversation ends” becomes “after the response”; “abrupt uncatchable death”
  becomes SIGKILL, as chapter 2 says.
- Uncheckable claims made checkable: “supported” host dropped (the evidence
  paragraph follows); `task install` “verifies” now names its installed smoke
  test (`crates/ravel/scripts/dispatch.sh` `install_pair`); `definePolicy`
  is an identity function with the SDK type (`worker/sdk/index.ts`);
  “process-specific details” become process IDs that differ between runs;
  “retains that report” becomes kept on standard error; “taught later” names
  chapter 5; cancellation names its INT, TERM and HUP signals; the shell
  prompt sentence states its condition instead of “may”.
- No in-session reviewer: each technical rewording was checked directly
  against the source named above, and the example and its output are
  untouched. Allowance unspent.
- Gates on the final text: `task docs:examples` 1 passed, 0 failed, 0 skipped,
  1 fragment not run, 0 page errors; `task docs:book:build` built without
  warnings; `task docs:figures` 2 figures match. Remaining lines over 80
  columns are links, the command and output blocks, or curly quotes counted
  as bytes.
- Art successor cut: no live later sibling held it.
