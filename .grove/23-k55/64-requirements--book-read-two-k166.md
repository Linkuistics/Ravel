# book-read-two-k166

## Goal

Hold read 2 of the book. Read 2 is the owner's labelled stand-in read of Parts
III and IV (chapters 7, 8, 9, 11 and 12). Record it with its evidence class.
Route its corrections to the stages that own them. Read 2 follows the
whole-book proof that comes after `ravel-book-k101`. Chapter 10 does not
exist, and the read does not include it.

## Context

- Contract, "Reader checkpoints": `docs/book/contract.md`. The reader is the
  owner, who reads as a labelled stand-in for the intended reader. The result
  can establish accuracy, coverage and the owner's own results. It cannot
  establish whether a developer new to Ravel can find, understand and use the
  book.
- **Tasks** (the read 2 tasks of the contract):
  1. Explain why Ravel has a TypeScript worker and two Rust packages.
  2. Follow one session through the code to the place where the Ravel CLI
     chooses its final cancellation outcome.
  3. Build and test Ravel from a checkout.
- **Entry surfaces.** Begin at chapter 1 or at the stated entry, in the
  locally built book (`task docs:book`, after `task setup`). First, check
  that the figures load there. The routes in chapter 1 are the entries. The
  route to change Ravel is chapters 1–3, 7 and 8, then 11, with 9 and 12 as
  necessary. The README is the repository's own door. If the owner wishes,
  begin there. Unless the owner chooses otherwise, read the new chapters from
  the first paragraph of chapter 7. The contract directs this for a read that
  begins at an entry.
- **Viewing environments.** Offer these to the owner, in this order:
  1. The locally built book.
  2. The raw Markdown in an editor preview.
  3. GitHub's view. Nobody can check it until someone pushes the branch,
     because this workstream is not on GitHub. Name it as unchecked. Do not
     leave it out.
  4. The book on GitHub Pages. It comes only with a release.
- **Record** these, as the contract requires:
  - the reader's own words
  - how far the reader got
  - the locations that the reader reports
  - what the reader did not reach
- **Report** these separately, and never as one another:
  - the owner's results
  - task performance
  - mechanical checks (`task docs:book:build`, `docs:examples`,
    `docs:figures`, `docs:excerpts`, `docs:ste`, `docs:breaks`, `check`)
  - any model-proxy result
- An early comprehension failure stops the read. Its correction goes to the
  stage that owns it before the read continues.
- **Read 1 is the model.** `book-read-one-k70` holds the way that the owner
  reads, and the findings R1–R17 with their routes. The owner reads, pauses
  and says “Fix before reading on” or “Route now”. Read 1 made rules for the
  whole book, and these rules apply to what the owner reads now. A finding
  that restates one of these rules goes to the stage that owns the chapter.
  The rules are:
  - the STE writing rules and `docs/book/ste.md`
  - “send” for the result
  - the crisp figure style
  - the limit on runs of prose
  - the reason for a harness
  - the loose contract
- **What the earlier stages claim.** The log of `book-second-half-k163`
  records the coverage of inventories 4 and 5. Do not claim that this read
  establishes that coverage or any unknown that the contract lists. The log
  also records these unknowns:
  - the comprehension of a new reader
  - GitHub's view of the book
  - the book on GitHub Pages
  - real-agent behaviour
  - interactive-terminal operation
  - native x86-64 qualification
- The fold-in leaves removed the guides that Parts III and IV replace. As
  read 1 did, ask the owner whether the reduced README and the removal read
  correctly. Record the answer.

## Done when

- The session holds the read with the owner. This leaf's log records the
  read with its evidence class.
- Each correction goes to its owning stage in one of three ways:
  - A re-run of the stage (kind `draft`, `copy-edit`, `art` or `proof`), cut
    with `leaf-add` under the unit's document node. The run is in pipeline
    order and ends at `proof`. Its body names the specific defect.
  - A set or whole-book re-run under the book node.
  - A contract change that the owner agrees.
- A second re-run of one stage against one document is an escalation to the
  owner.
- A whole-book proof follows each correction run that changes a chapter. The
  book node cannot close on a chapter that changed after the last proof.
- The session checks the book node's "Done when" against what the subtree
  delivered. It names each gap as a leaf, or escalates it. The node's
  conditions are these:
  - all chapters except chapter 10 passed their stages
  - the book's Taskfile tasks exist and pass
  - the fold-in leaves moved the guides into the chapters and removed them
  - the owner held both reads

## Notes

- HITL. Stop and ask the owner to read. A model's proxy is not read 2.
- The owner's configuration handoff record stays outside the book.
- Chapter 10 belongs to the future work that delivers a supported extracted
  component. Its own read follows the proof of that chapter.
