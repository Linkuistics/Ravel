# develop-ravel-k179


## Goal

Read and copy-edit all of chapter 11, `docs/book/src/11-develop-ravel.md`,
after both draft parts. Apply `docs/book/contract.md`, its STE writing rules
and `docs/book/ste.md`. Preserve its technical meaning and section plan.



## Context

- The structure brief is `docs/book/contract.md`. The node and draft logs
  `develop-ravel-k160` and `develop-ravel-k178` hold the source checks,
  task inventory and README/authority destinations.
- The four part A Rust fragments exactly match scratch additions that passed.
  Their patch and logs are in `docs/evidence/develop-ravel-k160/`. Preserve
  code bytes unless a technical defect earns a correction under the family
  rule. Fragments are not executable examples in the book checker.
- The first full scratch check failed at a hostile-worker fixture time limit.
  The unchanged isolated case and full rerun passed. Keep the failed
  observation and the limit on its unknown cause. No product fix is claimed.
- Part B supplies installation, documentation tasks, isolation guards,
  evidence families and the complete task inventory. Read the entire combined
  chapter, including tables, first-use term links and dated observations.

## Done when

- The whole chapter follows the prose contract, with its technical facts
  unchanged, and passes build, strict STE, strict prose breaks, examples
  and excerpts.
- Work outside copy-edit's charter has an owner under the editorial family
  rule. Clear any handed-forward items that this stage closes.
- Last act: add the art leaf with bare stem `develop-ravel`, unless a live
  later sibling already holds it. Carry specific figure/table questions in
  its body. Do not skip art because the chapter uses tables and code.

## Notes

This is the first copy-edit of the complete chapter, after its two planned
draft sessions. It is not a review leaf and needs no Creator line.

## Decisions (running log)

- Inputs read: the brief chain, `docs/book/contract.md` (term risks, prose
  contract), `docs/book/ste.md`, `CONTEXT.md`, chapter 9's copy-edit k175 as
  the precedent, and the whole chapter.
- Before any edit, strict STE and strict prose breaks on chapter 11 passed.
  The defects are those that a script cannot find.
- First-use links. Confinement, result, refusal and session had no link at
  their first use. Each now links to its chapter 2 section, as in chapter 9.
  The policy-worker link moves to the first use of “worker”. The Ravel CLI
  and session-result context links gain their section anchors.
- Term risks. “Session leader” is now “the leader of a new POSIX session”
  (`in_pty` calls `setsid`). The test type `Harness` is stated not to be an
  agent harness. A checker now “rejects” bad input, so “refusal” keeps its
  glossary meaning. “Ignored fixture role” is a test function marked
  `#[ignore]`.
- One word per action: “fetch” is “download”, and “fix” is “repair”, as in
  the evidence rows. “Fixes the dependency choices” is “pins the versions”,
  as in the setup table. “The repair” in the evidence table is “the
  completion-failure repairs”, the next row's term.
- Figures of speech and vague words replaced by mechanism, each checked
  against source: “drains its output” and “deadline bounds the wait”
  (`job.rs` `in_pty`), “denies Clippy's all group” (`Cargo.toml`), “the
  wrapper” (`scripts/check.sh`, `scripts/worker-check.sh`, `scripts/book.sh`),
  “atomic send” (an atomic write, chapter 7), “promise”, “needs attention”
  (the two nextest counts), “a skip remains a skip”, “the contents, glossary”
  (the file names), “Started-check” (`start_check`, Taskfile), “cut
  versions”, “extraction safety exceptions” (`.cargo/config.toml` comment),
  and probe identity (“no Ravel CLI accepts”, `dispatch.sh`).
- Mood: the `task check` stage table mixed imperatives and indicatives in one
  column. All rows are now indicative.
- One long paragraph (8 sentences) after the nextest edit is split.
- No structural defect. The section plan, tables, fragments and dated
  observations are unchanged. No Taskfile description is quoted, so
  `Taskfile.yml` is unchanged.

## Review

The leaf's one fresh reviewer received the chapter diff and the rule that a
copy-edit changes no technical claim. It checked each hunk against source and
found no false sentence and no broken anchor. Classification of its twelve
items:

- Valid, fixed:
  - The session link on “controls of an enclosing session” covered the
    `GROVE_*` and `HARNESS_DISPATCH_*` names, which are not Ravel session
    controls. The text now says “enclosing-session controls”, the chapter's
    own phrase. The session link moves to the first Ravel session (“an LLM
    session that Ravel supervises”).
  - “The checker can reject the bad input that a clean report excludes” was
    unclear and a new claim. The draft sentence is back.
  - The skipped count claim holds for an `-E` filter, not for `--bin`
    selection. It now names the `-E` filter.
  - “Atomic write” was not exact: `publish_result` writes a temporary file and
    then hard-links it (`completion.rs`). The text now says that the complete
    result file appears in one atomic step, and uses “send”.
  - “Completion-failure repairs” could narrow the result/job note, which also
    records native-entry observations. It is now “the later repairs”.
  - `start_check` is a test binary, not an API. The text now says “the
    `start_check` cases for the runner's public API”.
  - The policy-worker term is defined in chapter 2, not in the chapter 7
    section. The first-use link now goes to chapter 2.
- Visible trade-off, kept: the `in_pty` mechanism, the Clippy `deny`
  consequence, `scripts/check.sh` naming its log, and “gives the names … the
  runner removes them”. Each replaces a figure of speech or jargon with the
  mechanism, and the reviewer confirmed each against source.
- Noise, kept: “reject” for a checker. `CONTEXT.md` avoids “rejection” only
  as a name for a refusal, and chapter 9 already says that the result channel
  “rejects invalid result files”.

No second reviewer is needed: each fix restores draft text or narrows a
sentence to its source line.

## Verification and limits

- After the last chapter edit (SHA-256 `fa076f41…a90d` before and after the
  runs), all exited 0: `task docs:book:build`, `task docs:figures` (13 match),
  `task docs:examples` (14 passed, no failures or skips, 0 page errors),
  `task docs:excerpts` (8 passed), `task docs:ste -- --strict` on chapter 11,
  `SUMMARY.md` and `CONTEXT.md` (0 problems), and `task docs:breaks --
  --strict` on chapter 11 (no long run).
- A local link script resolved all 50 local link occurrences and their
  anchors. In a scratch copy, a wrong anchor and a missing file were both
  reported, so the clean result is not a broken instrument.
- Only raw Markdown was read. No rendered view was inspected. That is art's
  and proof's work.
- Art k180 is cut with the figure and table questions. No item is handed
  forward.
