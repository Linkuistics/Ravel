# rhai-book-use-k94


## Goal

Read chapters 1–6 as connected instructions after the setup and configuration-name corrections.
Correct sentences and terminology within the copy-edit charter.



## Context

- Draft `rhai-book-use-k93` owns structure and technical truth.
- Read the complete contract and all lessons, including L67 and F17–F18 in `rhai-book-use-k74`.
- Art `rhai-book-use-k95` and proof `rhai-book-use-k96` already follow. Cut no duplicate successor.

## Done when

- The whole of chapters 1–6 follows the prose contract and accumulated lessons.
- Instructions have clear referents. “These files”, “there” and similar words identify material already supplied or explicitly referenced.
  Command explanations follow the complete command and agree with its steps.
- Configuration vocabulary is consistent. Literals match the implementation, and quoted historical inputs remain accurate.
- Wording preserves the behavioural contract. No correction changes a result, bound or cleanup guarantee.
- Strict prose and break checks pass, as do affected examples, excerpts and the book build.
- Route structural or technical defects through the existing correction machinery, with their passage and consequence.
  Record new documentation lessons.

## Notes

- The remaining owner checkpoint is `rhai-book-use-k74`. No model result substitutes for it.

## Decisions (running log)

- The chapter 4 evaluation sequence describes a successful selection, not every
  evaluation. Refusals and failures can stop before the managed launch. The
  copy now states that scope without changing the sequence.
- The Codex example resolves its schema from the caller's working directory.
  The copy now names the example directory as the place from which the caller
  runs it, and distinguishes command blocks from individual command lines.
  These are applications of the existing setup lesson, not a new lesson.
- Chapter 5's setup now states the confined example's sandbox prerequisite at
  the first general prerequisite statement. Its environment comparison now
  distinguishes five controls that remain absent from the replaced result path.
  Both apply existing lessons on prerequisites and exact scope.
- Chapter 2 now includes outcomes before a program is selected in its session
  definition. Chapter 6 distinguishes host capabilities from the helpers that
  run inside the worker. These clarify existing glossary distinctions.

## Verification and handoff

- After the copy changes, strict STE and prose-break checks passed for chapters
  1–6. The book build passed. The excerpt check passed all nine excerpts.
  The example check passed all 21 complete examples in these chapters, with
  no failures or skips. It did not execute 39 labelled fragments. A final
  paragraph break in chapter 6 followed that example run; strict checks and
  the book build passed again after it.
- The installed-files figure remains handed forward to art `rhai-book-use-k95`.
  Art and proof `rhai-book-use-k96` already stand as live later siblings, so
  this correction run adds no successor. The owner checkpoint remains live.
- The corrections apply the existing setup, scope and terminology lessons;
  this read found no separate new lesson to add.
