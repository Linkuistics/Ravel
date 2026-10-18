# book-prose-breaks-check-k133

## Goal

Add a Taskfile check that reports each run of prose that is longer than the
contract's limit, in each chapter of the book. The drafts of this correction
run use it to measure the owner's rule R11. The sessions for chapters 7–12
use it later.

## Context

- The rule is in `docs/book/contract.md`, section 2, “Breaks in prose”. A run
  of prose has about 450 words or fewer. A figure, a table or a code block
  breaks a run. Headings and lists do not. The owner made the decision and
  chose the limit on 2026-10-18 (R11 in `book-read-one-k70`'s log).
- Read 1 measured the chapters with a prototype outside the repository. It
  separated blocks at blank lines. Fenced code blocks, tables (lines that
  start with `|`) and images (lines that start with `![`) were breaks. It
  ignored HTML comments. It counted words in paragraphs, lists and
  blockquotes, after it removed link targets.
- At `1ef02398`, the prototype found ten runs longer than 450 words: chapter 1
  (900 and 723), chapter 2 (1,329), chapter 3 (1,745), chapter 4 (549, 487
  and 466), chapter 5 (529 and 499) and chapter 6 (2,200). Use these counts
  as a cross-check. Build the check from the contract, not from the
  prototype.
- Follow the pattern of `task docs:ste` (`scripts/ste.ts`): a report that
  stops with status 0, a strict form, and a test task with controls.

## Done when

- A Taskfile task, for example `docs:breaks`, reports each run that is longer
  than the limit: the file, the lines, the word count and the headings in
  the run. It examines every chapter under `docs/book/src/`, so it also
  examines chapters 7–12 when they exist.
- The report form stops with status 0. The strict form stops with status 1
  when it finds a run that is longer than the limit.
- A test task has controls. A planted long run fails. A figure, a table and
  a code block each split a run. A heading and a list do not split a run.
  Link targets and HTML comments add no words. Each control fails on a
  mutant of the check.
- The contract's “Breaks in prose” names the task and the exact number that
  it uses.
- The report on the text at this leaf's commit agrees with the ten runs
  above, or this leaf records why a count is different. This leaf changes no
  chapter text. The drafts fix the runs.
- `task check` and the other book checks pass.

## Notes

- The word count stands for one screen at the 750-pixel column. The owner
  chose “about 450 words”. The check needs one number, so state it.
