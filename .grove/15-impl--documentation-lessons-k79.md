# documentation-lessons-k79

## Goal

Record every lesson about documentation and communication that this grove learnt, in `docs/documentation-lessons.md`.
The owner wants these lessons as a skill for future documents and books in other groves.

## Context

- <!-- tc: depart long-sentence because the item quotes the owner's words verbatim --> The owner's words of 2026-10-27, in Read 1 (`rhai-book-use-k74`): “Also remember that we want to remember/record all of our learnings/corrections etc through this process so we can apply them to future docs and books in other groves (as a skill).”
  Then: “By learnings I mean *all* the things about documentation and communication that we have learnt in this grove.”
- Documentation covers the book, its contract and mechanics files, `CONTEXT.md`, the ADRs, the specs, the README and the CLI help.
  Communication covers the replies to the owner, questions, handovers, task files, briefs, commit messages and prompts for other models.
- Read these sources in full:
  - Every task file and brief under `.grove/`, live, `DONE` and `ABANDONED`. Look most at the running logs and the owner's words in them.
  - The messages of this grove's commits, from `plan-k1` to now.
  - `docs/book/contract.md`, with its dated owner decisions, and `docs/book/ste.md`, `figures.md`, `examples.md` and `excerpts.md`.
  - The first Read 1, `book-read-one-k70`, in the history at `fa7b73f`. It comes from the earlier book grove, so mark its lessons as background.
- Check these readings of `rhai-book-use-k74`. They are a session's analysis, not the owner's words, so keep or reject each one:
  - The STE limits alone produced compressed and abstract prose. Findings F1–F8 and the ten patterns in section 2 of the contract correct it.
  - The old “Terms” rule put a link at the first use of a term, and “Stock phrasing” forbade restatement. Both rules made pages depend on other pages.
  - Finding R8 of the first Read 1 (no repeated colour key) and F7 (repeat and restate) can look opposed. The session read F7 as a rule for explanations, not keys.
  - Both reads followed a proof that passed every gate, and both found problems that no gate finds.
- Candidate lessons about reads from `rhai-book-use-k74`:
  - Run the reader's commands in a separate, private shell. The session's own shell held the `RAVEL_EXIT_FILE` of Grove.
  - Check figures and links with a control that can fail. Check each external address: one address from a web search returned 404.
  - A browser tool wrote its snapshot into the working copy. Move such files out before a commit.

## Done when

- `docs/documentation-lessons.md` gives each lesson as a rule that another project can follow.
  Each rule has its source: the handle, the date and the owner's words where they exist.
  Each rule also says where Ravel applies it, with the contract section or the check.
- The file keeps the owner's decisions and findings apart from a session's analysis.
- A search of every running log for the owner's decisions about writing and communication has a count.
  Each decision that it finds is in the file, or the file says why it is not a lesson.
- `docs/book/contract.md` and the root brief point to the file. They say that each later session adds its lessons there.
- The technical-communication checker reports no hard finding in the file.

## Notes

- The skill waits until the owner names its home. The root brief keeps it under “On the horizon”.
- Do not change the chapters of the book. The correction run under `rhai-book-use-k26` follows this leaf.
