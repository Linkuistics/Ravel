# the-parts-of-ravel-k167

## Goal

Copy-edit the whole of `docs/book/src/07-the-parts-of-ravel.md` under
`docs/book/contract.md` and `docs/book/ste.md`. Keep its technical meaning
and conceptual order. The title is “The parts of Ravel”.

## Context

- Draft `the-parts-of-ravel-k152` checked the source and recorded the O1–O10
  and R1–R8 destination tables and sentence groups from both architecture
  guides in its log. Preserve that map for the later guide migration.
- The chapter explains CLI, worker, SDK, runner and result channel, then the
  loose program contract and the installed pair. The future chapters 8 and
  9 own detailed trace, precedence and process mechanics. Do not pull their
  work into this chapter.
- The draft changed the title in the contract, contents, shared title table
  and chapter 1's route. Those changes are part of the agreed title decision.
- Preserve the qualifications: identity verification precedes the policy
  request, not worker execution; confirmed reap and directory removal gate
  launch; the separate worker is not a hostile-code sandbox; a matching
  `Ended` is the channel consumer's obligation; private packages do not
  establish supported independent use or host restoration.
- One fresh reviewer checked the worker boundary and found no concrete
  defect. That review checked technical claims, not this stage's prose.
- The draft passes strict STE and prose-break checks, book build and all 13
  complete examples. It has no source excerpt and no new runnable example.
  The task records digested subjects, link controls and evidence limits.
- Art's figure-reuse decision is in the document node's “Handed forward”
  list. The draft only links to the proved figures in chapters 2 and 6.

## Done when

- The whole chapter meets the prose contract, including active voice,
  simple verb forms, sentence/paragraph limits, first-use term links and
  plain explanations of the package interfaces.
- Strict STE, prose-break checks, book build and complete examples pass.
- The log records any changed guide destinations. Technical or structural
  defects follow the editorial correction procedure rather than a silent
  expansion of this charter.
- Last act: add the `art` leaf under this node, with the bare stem
  `the-parts-of-ravel`, unless a live later sibling already holds that stage.

## Notes

Forward mentions of chapters 8, 9, 11 and 12 stay plain text until those
targets exist. The second-half set proof owns the final cross-chapter links.

## Decisions (running log)

- 2026-10-18: Baseline. The draft passes strict STE and prose-break checks
  on chapter 7. The read against the prose contract, `docs/book/ste.md` and
  `CONTEXT.md` found these classes: the term “managed program” beside the
  book's “managed process”; the unqualified “Context” in the SDK table;
  “refusal”, “managed process”, “confinement” and “persistent runner” not
  linked at first use; figures of speech (“owns the connection”, “hides …
  behind”, “span three places”, “ambient inputs”, “join these
  observations”); claims the reader cannot check (“root-level package files
  remain relevant”, “a private file representation helps”); and “runtime
  notices”, which in this book reads as Ravel's standard-error notices but
  in the ADR means Bun's licence notices.
- 2026-10-18: Use “managed process” throughout, as chapters 1–6 do (over a
  hundred uses). Chapter 5's three uses of “managed program” stay with that
  chapter's own stages: they are outside this leaf's document.
- 2026-10-18: State Bun's import inputs in chapter 6's words: `/package.json`
  and `/node_modules` stay reachable inputs for the resolver, and package
  autoloading stays enabled (`06-reference.md`, “Import resolution”).
- 2026-10-18: “Runtime notices” becomes “the licence notices of the Bun
  runtime” (`crates/ravel/notices/NOTICES.md`, `bun-LICENSE.md`).
- 2026-10-18: Chapter 5's three uses of “managed program” go to the book
  node's handoff table, with the whole-book proof as owner. Chapter 5 is a
  proved document outside this leaf's charter.
- 2026-10-18: Every guide destination in the draft's log stays valid. This
  stage changed sentences and table cells only. No heading, anchor, section
  order, table row or code block changed, and no technical claim was added.
  One clause that would have been a new claim (that the runner's callers
  make no system calls) was removed before validation.
- 2026-10-18: No in-session reviewer. The edits replace words and link
  terms. Each changed fact comes from a source read in this session or from
  chapter 6's proved text.

## Validation

All checks ran after the last edit to the chapter.

| Check | Result |
|---|---|
| `task docs:ste -- --strict` | 19 files, zero problems |
| `task docs:breaks -- --strict` | Seven chapters, zero runs above 450 words |
| `task docs:book:build` | Build passed |
| `task docs:excerpts` | Zero excerpts, zero page errors |
| `task docs:examples` | 13 passed, zero failed or skipped, six fragments not run |
| Rendered local links in chapter 7 | 31 links resolve. A control with a missing anchor and a missing file failed with status 1. The four new term links occur on the rendered page. |

The read did not test new-reader comprehension, real-agent behaviour or the
page in a browser.
