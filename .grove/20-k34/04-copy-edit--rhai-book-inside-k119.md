# rhai-book-inside-k119

## Goal

Copy-edit chapters 7 to 11 of the Ravel book after the two drafts of this node. Change only what the copy-edit charter owns.

## Context

- The structure brief and prose contract are in `docs/book/contract.md`. The word decisions are in `docs/book/ste.md`.
- The first draft, `rhai-book-inside-k35`, changed no chapter. A later grove had already drafted chapters 7–9 for 2.0.0 in `56dac027`, `f6ee3135` and `47c6422e`.
  That grove also drafted chapters 10–11 in `ca8c7d01`, copy-edited chapters 7–11 in `8e07b803`, made the figures in `7d9526e9` and proofed the whole book in `424af53d`.
  The running log of `rhai-book-inside-k35` maps each of its done-when items to that work.
- Read the running log of `rhai-book-inside-k36` for what the second draft changed.
  It changed chapter 11 after the promotion of 2.0.0, added the home rule of `smoke:pair` to chapter 10, and added lessons L83 and L84.
- The third draft, `rhai-book-inside-k120`, corrects the installation path in chapters 3 and 6, the contract and the README. Read its running log, and copy-edit those changes too.
- Compare the text with the copy-edit of `8e07b803` before you repeat it. Read each change after that commit in full, because it has had no copy-edit.
  After `424af53d`, chapter 11 changed one command line, and chapter 3 changed outside this node's chapters.
- Apply the current lessons in `docs/documentation-lessons.md`, including L82.

## Done when

- Each change to chapters 7–11 after `8e07b803` meets the prose contract, and the stage records what it read.
- `task docs:ste` reports no problem, and `task docs:examples` and `task docs:excerpts` pass.
- Each defect that a later stage owns is in the `## Handed forward` list of the node brief.
- The last act cuts the `art` leaf, unless a live art leaf already stands below the node.

## Notes

- The owner can judge the later grove's copy-edit, art and proof sufficient and prune this tail. The proof stage of this node still cuts Read 2, as the node brief requires.
