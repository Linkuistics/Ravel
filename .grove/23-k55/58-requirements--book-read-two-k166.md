# book-read-two-k166

## Goal

Hold read 2 of the book: the owner's labelled stand-in read of Parts III and IV
(chapters 7, 8, 9, 11 and 12), recorded with its evidence class and its
corrections routed to the stages that own them. It follows the whole-book proof
that comes after `ravel-book-k101`. Chapter 10 does not exist and is not read.

## Context

- Contract, "Reader checkpoints": `docs/book/contract.md`. The reader is the
  owner, reading as a labelled stand-in for the intended reader. The result can
  establish accuracy, coverage and the owner's own results. It cannot establish
  whether a developer new to Ravel can find, understand and use the book.
- **Tasks** (the contract's read 2): explain why Ravel has a TypeScript worker
  and two Rust packages; follow one session through the code to where its final
  cancellation outcome is chosen; and build and test Ravel from a checkout.
- **Entry surfaces.** Begin at chapter 1 or at the stated entry, in the locally
  built book (`task docs:book`, after `task setup`), with the figures confirmed
  to load there first. The routes in chapter 1 are the entries: “Changing Ravel”
  is chapters 1–3, 7 and 8, then 11, with 9 and 12 as needed. The README is the
  repository's own door; start there if the owner wishes. Read the new chapters
  from the first paragraph of chapter 7, as the contract directs for a read that
  begins at an entry, unless the owner chooses otherwise.
- **Viewing environments** to offer, in order: the locally built book; the raw
  Markdown in an editor preview; GitHub's view, which is unchecked until the
  branch is pushed (this workstream is not on GitHub) and is named as unchecked,
  not omitted; the book on GitHub Pages, which comes only with a release.
- Record, as the contract requires: the reader's own words, how far they got,
  the locations they report and what they did not reach. Report separately and
  never as one another: the owner's results, task performance, mechanical
  checks (`task docs:book:build`, `docs:examples`, `docs:figures`,
  `docs:excerpts`, `docs:ste`, `docs:breaks`, `check`) and any model-proxy
  result. An early comprehension failure stops the read and routes its
  correction to the stage that owns it before the read goes on.
- **Read 1 is the model.** `book-read-one-k70` holds the owner's way of reading
  (the owner reads, pauses, and says “Fix before reading on” or “Route now”) and
  the findings R1–R17 with their routes. Rules that read 1 made for the whole
  book apply to what the owner reads now: the STE writing rules and
  `docs/book/ste.md`, “send” for the result, the crisp figure style, the limit
  on runs of prose, the reason for a harness and the loose contract. A finding
  that restates one of those goes to the stage that owns the chapter.
- **What the earlier stages claim.** The log of `book-second-half-k163` records
  the inventory 4 and 5 coverage and the unknowns that the contract lists (new
  reader comprehension, GitHub's rendering, the book on GitHub Pages, real-agent
  behaviour, interactive-terminal operation and native x86-64 qualification).
  Do not present any of them as established by this read.
- The guides that Parts III and IV replace were removed by the fold-in leaves.
  Ask the owner whether the reduced README and the removal read correctly,
  as read 1 did, and record the answer.

## Done when

- The read is held with the owner and recorded in this leaf's log with its
  evidence class.
- Each correction is routed to the owning stage: a `leaf-add` of the stage's
  re-run (kind `draft`, `copy-edit`, `art` or `proof`) under the unit's
  document node, in pipeline order ending at `proof`, with the specific defect
  in its body; or a set or whole-book re-run under the book node; or recorded
  as a contract change with the owner. A second re-run of one stage against one
  document is an escalation to the owner. A correction run that changes a
  chapter is followed by a whole-book proof, because the book node cannot close
  on a chapter that was changed after the last proof.
- The book node's "Done when" is checked against what the subtree delivered
  and any gap is named as a leaf, or escalated: all chapters but chapter 10
  have passed their stages, the book's Taskfile tasks exist and pass, the guides
  are folded in, and both reads are held.

## Notes

- HITL. Stop and ask the owner for the reading; a model's proxy is not read 2.
- The owner's configuration handoff record stays outside the book.
- Chapter 10 belongs to the future work that delivers a supported extracted
  component. Its own read follows that chapter's proof.
