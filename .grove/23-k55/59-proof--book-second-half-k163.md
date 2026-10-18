# book-second-half-k163

## Goal

Proof chapters 7, 8, 9, 11 and 12 as a set after their units are proved, and
before the guide fold-in removes the guides that these chapters replace. A
unit's proof reads only its unit. This leaf reads what spans the units: the
two inventories that the second half claims, the terms and facts that two
chapters state, the figures that two chapters share, and the links between the
chapters. Chapter 10 does not exist and is not read.

## Context

- Contract: `docs/book/contract.md`. The units' logs list the guide sentences
  that each chapter replaces. Those lists are the fold-in leaves' inputs, so
  this leaf checks that they are complete.
- **Inventory 4 (component obligations).** Enumerate the ten obligations from
  `docs/specs/session-execution.md` and `docs/components.md` again. Report for
  each of O1–O10 the section in chapters 7, 8 or 9 that teaches it, and what
  is missing. Coverage shows what the book covers, not that a reader understands
  it.
- **Inventory 5 (contributor workflows).** Enumerate every task from
  `task --list` again. Report for each the section in chapter 11 or 12 that
  names it, and the pointer to the chapter that teaches it. The native-entry,
  incumbent and Linux measurement tasks are evidence instruments: they are
  named, with a pointer to the evidence, and are not taught. Check that the
  list in `Taskfile.yml` is the list in the chapters, and not the list in this
  file.
- **Across the set**, check:
  - one fact has one value everywhere: the escalation times, the exit statuses,
    the support statement, the version and identity rules, the count of
    packages and processes;
  - one term has one meaning, and “context” is never unqualified;
  - chapter 3's promises of chapter 9 (“the exact times”, “the terminal rules”,
    “signal masks and failure guarantees”) and of chapter 8 are kept;
  - chapter 1's route table, entry routes and the contract's entry routes
    agree with the chapters as written, including “Changing Ravel” (chapters
    1–3, 7 and 8, then 11, with 9 and 12 as needed). The route “Reusing a
    component” cannot be completed until chapter 10 exists; the book says so;
  - the loose contract (R15) in chapters 1, 2 and 7 agrees, and R14, R12, R13
    and R16 agree wherever their concepts occur in chapters 7–12;
  - the figures that two chapters share are the same figures in the same style:
    the container view (chapters 2 and 7), the source and installation figure
    (chapters 6, 7 and 12), and the cancellation sequences (chapters 5 and 9).
- **Plain-text mentions of chapters 7–12 in chapters 1–6.** The first half
  wrote them as plain text with the number and the title. Turn each into a link
  to its target (with an anchor where a section is meant), and update titles
  that changed (chapter 7 is “The parts of Ravel” now). Enumerate the mentions
  again with a search that you first see find known ones
  (`docs/book/src/01-what-ravel-is-for.md` has several, and so do chapters 3, 4
  and 5). The README's reworded paragraph and `docs/book/src/SUMMARY.md` are
  checked here for the draft entries too: no empty `[…]()` entry remains for
  chapters 7, 8, 9, 11 and 12.
- **The non-`--json` form of an operation report** (`ravel: <report>` on
  standard error, `crates/ravel/src/run.rs`) is in no chapter and chapter 6's
  standard error row says only “operational reports”. Decide here whether
  chapter 6 gains an entry, using what chapters 8 and 9 teach. Record the
  decision with its reason. If you add the entry, it is a sentence-level change
  to chapter 6, and it follows the shared decisions.
- **Source claims.** Spot-check against the code the claims that the units
  marked as hardest to check. The unit proofs checked their own; this read
  checks the claims that two chapters make about one mechanism.
- Check the strict STE and prose-break checks on all of chapters 1–12 that
  exist, with `task docs:ste` run with no arguments now that
  `book-ste-all-chapters-k149` extends its default.

## Done when

- `task docs:book:build`, `task docs:figures`, `task docs:figures:test`,
  `task docs:examples`, `task docs:excerpts`, `task docs:ste -- --strict`,
  `task docs:breaks -- --strict` and `task check` pass on subjects that have
  the same digests before and after.
- Every local link destination of the rendered book and the entry documents
  resolves, with controls (a broken anchor and a missing file fail). Every
  figure of chapters 7–12 loads in the served book (a control image with no
  file fails). Report the viewing environments reached and not reached.
- Inventories 4 and 5 are reported as the contract requires, with what is
  missing, and each gap is closed or routed to the stage that owns it by a
  correction run. A second send-back of one stage against one document goes to
  the owner.
- Every plain-text mention of a chapter 7–12 in chapters 1–6 is a link, or is
  listed with its reason.
- The decision on the operation report is recorded.
- The node brief's list of second-half handoffs is cleared of what this leaf
  closes. No defect goes forward without an owner. Mark which guide sentences
  the units did not place, so that the fold-in leaves have no unplaced content.
- A guide is removed only after this leaf passes. Say in the log that
  “absorbed” is a statement about the chapters as proved here.

## Notes

- The whole-book proof after `ravel-book-k101` reads the finished book again,
  after the whole-book art pass. This leaf does not replace it. It exists
  because inventories 4 and 5 and the cross-chapter facts are set questions
  that a unit's proof cannot answer, and because the guides must not be
  removed before the chapters that absorb them are checked as a set.
- The read 2 tasks are the contract's: explain why Ravel has a TypeScript worker
  and two Rust packages, follow one session through the code to where its final
  cancellation outcome is chosen, and build and test Ravel from a checkout.
  Check, without holding the read, that the chapters make each task possible.
