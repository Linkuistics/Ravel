# ravel-book-second-half-k148

## Goal

Plan Parts III and IV of the book, chapters 7, 8, 9, 11 and 12, after the
corrections from read 1. Chapter 10 is not this node's work.

## Context

- The contract: the section plan rows for chapters 7–9, 11 and 12, the entry
  routes, the coverage inventories 4 (component obligations) and 5
  (contributor workflows), the figure needs for those chapters, the examples
  policy and the tasks of read 2.
- The book node's brief, “Handed forward to the second half”: the links to
  chapters 7–12, `docs/authority.md`, the README's “Develop” section, the
  operation report without `--json`, the shared figure conventions, the
  installed-files figure and the entry from `book-release-version-k145`.
- Read 1 added these rules for all chapters: the STE writing rules and
  `docs/book/ste.md`, “send” for the result, the crisp figure style, the
  prose limit and its check, the reason for a harness and the loose contract
  (R14 and R15).
- The examples policy needs a source-excerpt check: a Taskfile task checks
  that each walkthrough excerpt of chapter 8 still occurs in its file, byte
  for byte. It does not exist yet.
- Chapter 8's method: the `linkuistics:writing-code-walkthroughs` skill at
  orientation depth, with no complete-source fragment graph.
- Guides to fold in after their chapters land: `docs/local-runner.md`,
  `docs/session-channel.md`, `docs/components.md` (its views under
  `docs/design/components/` become figure sources), `docs/RELEASING.md` and
  the rest of `docs/authority.md`.

## Done when

- Units for chapters 7, 8, 9, 11 and 12 and their guide migration are cut
  before whole-book art `ravel-book-k101`. The source-excerpt check comes
  before the draft of chapter 8.
- Read 2, a `requirements` leaf with the contract's tasks, follows
  `ravel-book-k101`. The body of k101 tells it to cut the whole-book proof
  before read 2.
- Chapter 10 stays outside this node.

## Notes

## Decisions (running log)

- **Two scaffold increments come first** (`book-ste-all-chapters-k149`,
  `book-source-excerpts-k150`). The default `task docs:ste` file list stops at
  `0[1-6]-*.md`, so chapters 7–12 would be outside the plain report. The
  example check fails closed on an unlabelled `ts` or `sh` block, so the
  excerpt convention must make the two checks agree on one page; chapter 8's
  worker excerpts would fail the example check otherwise. The excerpt check
  precedes chapter 8's draft, as the brief requires. Neither enters `task check`.
- **Chapter 7's title is “The parts of Ravel”** (STE Rule 3.6: “How Ravel is
  built” is passive with a known agent). Files for chapters 7–12 take the final
  titles (`07-the-parts-of-ravel.md`, `08-follow-a-session-through-the-code.md`,
  `09-jobs-signals-and-the-terminal.md`, `11-develop-ravel.md`,
  `12-deliver-ravel.md`); the first half's names stay. The chapter 7 draft edits
  the contract row, `SUMMARY.md`, `ste.md` and chapter 1's route table.
- **Five units, with two-session drafts where the contract's own inventories or
  the subject divide the work**: 7 one session; 8, 9, 11 and 12 two sessions
  each, part A cutting part B lazily as in the first half. Seams: 8 (successful
  path | forks, precedence, where a change belongs); 9 (job and its end |
  terminal, entry state, guarantees); 11 (change code and check it | other
  workflows, guards, evidence documents); 12 (versions, archives, checks before
  a cut | formula, release tasks, the book's version line, recovery).
- **Two figure-module impl leaves precede the units that need them**
  (`book-figure-activity-k153` before chapter 8, `book-figure-timing-k156`
  before chapter 9). The engine has no activity or timing layout module, and an
  art stage must not build a module. Each leaf delivers its first real figure
  (outcome precedence; escalation timing) so that it is a working increment the
  figure check compares. The release activity figure waits for chapter 12's art.
- **A second-half set proof (`book-second-half-k163`) is cut now**, after the
  last unit and before the guide fold-in. Inventories 4 and 5, shared facts,
  shared figures, the link sweep and the operation-report decision span the
  units, and “absorbed” must be a statement about proved chapters. It does not
  replace the whole-book proof that `ravel-book-k101` cuts.
- **Guide migration is two impl leaves** (`book-architecture-fold-in-k164`,
  `book-contributor-delivery-fold-in-k165`) split by chapter owner. Frozen
  evidence receipts that name a removed file are not edited; the earlier search
  of `scripts/`, `crates/` and `Taskfile.yml` found no live reader of those
  names, and the leaves confirm it. Sentences with no home go back to the
  owning chapter by a correction run, and the fold-in leaf stays live.
- **Stale source found while planning**: `docs/RELEASING.md` says development is
  0.0.0 and the first minor is planned at 0.1.0, but `v0.1.0` and `v1.0.0` exist
  and the workspace is at 1.0.0. Chapter 12's brief names it.
- **`ravel-book-k101` was edited** to name read 2 (`book-read-two-k166`) as the
  `leaf-insert` target for the whole-book proof and to list the second half's
  new figures. Read 2 is cut after it with the contract's tasks.
- **No `review-planning` leaf and no in-session reviewer spent.** The tree was
  checked against this leaf's Done-when and the node brief's handoff list
  directly: dependency order (scaffold, modules, units, set proof, fold-ins,
  whole-book art, read 2) and the owners of each handoff, which the book node's
  brief now tabulates. A reader of the tree can still escalate.
- Not cut, and why: chapter 10 (outside this node); a research leaf (the code
  and the guides are the evidence); per-chapter `review-*` leaves (the editorial
  family takes none).
