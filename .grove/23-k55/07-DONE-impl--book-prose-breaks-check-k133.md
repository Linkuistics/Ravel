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

## Decisions (running log)

- The check is `task docs:breaks` (`scripts/breaks.sh`, `scripts/breaks.ts`),
  with `task docs:breaks:test` (`scripts/breaks.test.sh`, its controls and
  mutants). It follows `docs:ste`. With no file it examines every
  `docs/book/src/NN-*.md`, so chapters 7–12 join when they exist.
- The one number is 450. A run is longer when it has more than 450 words. A run
  of exactly 450 passes, and a control proves it. The contract's “Breaks in
  prose” names the task, the number and the counting rules.
- Counting rules, from the contract: words are the white-space-separated pieces
  of paragraphs, list items and blockquotes, after a link is reduced to its
  text. A heading's text, a list marker, the `>` of a blockquote, a link target,
  a reference definition and an HTML comment add no words. A heading and a list
  do not break a run. A fenced block (``` or ~~~), a table row and an image line
  break a run.
- Cross-check at `1ef02398`: the check finds the same ten runs, in the same
  places. Its counts are lower than the prototype's: ch1 885 and 703 (900, 723),
  ch2 1,302 (1,329), ch3 1,721 (1,745), ch4 463, 542 and 473 (466, 549 and
  487), ch5 524 and 487 (529 and 499), ch6 2,118 (2,200). The reason: the
  prototype counted the `#` and the words of each heading, the `>` of each
  blockquote and the marker of each list item as words. The contract does not
  call these words, so the check does not count them. With those three counts
  restored in a scratch copy, the check gives the prototype's ten numbers
  exactly. The difference changes no verdict: each run is still longer than 450.
