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

## Decisions (running log)

**Mechanical checks before the read (2026-10-19 — evidence class:
mechanical, not the owner's result).** The subject is commit `23ef3928`
(`ravel-book-k195`) with an empty working copy. A SHA-256 manifest of the
3,537 tracked files outside `.grove/` was the same before and after the
checks (manifest digest `f2af649d…7ed9`). Against the input manifest of
k195, the only difference is the twelve evidence files that k195 added.
Thus, the comparison can find a difference.

The 3,525 shared inputs are byte for byte the inputs that k195 proved. The
leaf conversion `4df6e68d` changed only this file, outside the manifest. The
results:

- `task docs:book:build`: no mdBook error or warning.
- `task docs:figures`: 14 of 14 figures match their sources and layout
  records (Typst 0.15.1, CeTZ 0.4.2, DejaVu 2.37).
- `task docs:excerpts`: 8 passed, 0 failed.
- `task docs:ste -- --strict`: zero problems in 28 files.
- `task docs:breaks -- --strict`: no run longer than 450 words in the 11
  chapters.
- `task docs:examples`: 14 passed, 0 failed, 0 skipped. 27 fragments did not
  run.
- `task check`: 339 tests passed, 46 skipped.
- The book that `task docs:book` serves on `127.0.0.1:8775`, in a Playwright
  browser at 1280 by 900 pixels: each of the 14 figures is complete. Each has
  a nonzero natural size and shows at the 750-pixel column. The 14 figures
  have 18 places in chapters 1–3, 5, 6, 8, 9 and 12. Chapters 4, 7 and 11
  have no figure. A control image with no file gave a load error and a 0×0
  size. The only console error is the 404 of that control.

The browser tool wrote its page snapshot and console log into
`.playwright-mcp/` at the repository root, which Git does not ignore. Both
files moved to the session scratchpad before the next jj command. Thus, no
change in the working copy came from them. These checks support the read.
None of them stands in for it.

**Owner choices for read 2 (2026-10-19 — the owner's answers).**

- Entry: “Chapter 1, change route”. The owner reads chapters 1–3 again, then
  chapters 7, 8 and 11, with 9 and 12 as necessary. This is the route to
  change Ravel in chapter 1.
- Viewing environment: “Markdown in an editor preview”. The owner reads the
  files under `docs/book/src/` in an editor preview. The contract places each
  read in the locally built book. The owner chose the editor preview for this
  read, and the served book stays available.
- For the editor preview, each of the 11 image paths in chapters 1–3, 7–9,
  11 and 12 names an existing SVG. A missing-file control fails. This
  session cannot see the editor of the owner.
- Chapter 11 has 22 links to `main` on GitHub, and `main` does not have this
  workstream. Chapter 11 states this limit in its opening. This session told
  the owner the limit before the read.

**Read 2: the owner's result (2026-10-21 — evidence class: the owner's
labelled stand-in read).** The owner read the Markdown files in an editor
preview. The owner chose to begin at chapter 1, on the route to change Ravel.
The owner's verdict was “This is good work.” Then the owner wrote “Please
continue”.

This session asked how far the owner read. The owner answered:

<!-- tc: off because the owner's words are a quotation -->

> “I read enough to know that this is good enough for now - we will iterate on
> the details later as we codify all your learnings into grove's
> communications/writing skills”

<!-- tc: on -->

- **Findings.** The owner reported no finding and no location. Thus, no
  correction goes to a stage, and this session cut no correction run.
- **How far the owner read.** The owner did not name the chapters that they
  read. The owner also did not name the chapters that they did not reach.
  Thus, this record does not show that the owner read each chapter on the
  route.
- **Figures.** The owner reported no figure that failed to show in the
  preview. This session could not see the editor.
- **README and removal.** The owner answered “Yes, both read correctly”. Thus,
  the reduced README and the removal of the folded guides read correctly to
  the owner.
- **Later iteration.** The owner named no detail and no time for it. Thus, no
  session can state that work precisely yet, and it has no leaf. The root
  brief holds it as an “On the horizon” note.

**Task performance (evidence class: task performance).** The owner did no task
(“No task”). Thus, read 2 gives no result for its three tasks. In read 1, the
owner did not report its four tasks either.

**The other evidence classes.** The mechanical checks are the entry of
2026-10-19 above. No file outside `.grove/` changed after commit `23ef3928`.
Thus, those checks apply to the book that the owner read. This session ran no
model proxy.

**What read 2 cannot establish.** Read 2 does not establish new-reader
comprehension, GitHub's view of the book or the book on GitHub Pages. It also
does not establish real-agent behaviour, interactive-terminal operation or
native x86-64 qualification.

**Close of the book node `ravel-book-k55` (2026-10-21).** This leaf is the
last live leaf under the node. The check of the node's “Done when”:

- Chapters 1–9, 11 and 12 are in `docs/book/src/` in the contract's order.
  Each went through draft, copy-edit, art and proof, in unit runs or in set
  and whole-book runs.
- Whole-book art `ravel-book-k101` came before whole-book proof
  `ravel-book-k195`. Both came before read 2.
- The book's Taskfile tasks exist, and they passed on 2026-10-19. `task setup`
  runs `scripts/book.sh setup`, which fetches the pinned mdBook 0.5.4. The
  release adds the book to the `gh-pages` branch for GitHub Pages
  (`book-release-pages-k184`). The first publication comes with the next
  release, outside the node.
- The fold-in leaves removed the guides and changed the links to them. The
  link check of k195 found no broken local link. The README is a short entry
  into the book, and the owner said that both read correctly. The record
  `docs/configuration-handoff-2026-10-12.md` stays outside the book.
- The owner held read 1 (`book-read-one-k70`) and read 2, and each log
  records its evidence class. Read 1's corrections went to their stages.
  Read 2 has none.
- `ravel start-session --help` describes `--policy-env`
  (`crates/ravel/src/cli.rs:91`, from `policy-env-help-k61`).
- Chapter 10 stays with the component work (`component-work-scope-k57`).

Each condition holds. Thus, the node closes with no gap and no escalation.
The root brief now holds the outcome in “Notes” and the later iteration in “On
the horizon”. No live review names this leaf or the node, so no Creator line
changes. No ADR and no glossary term changes.
