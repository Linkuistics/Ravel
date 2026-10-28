# rhai-book-use-k86

## Goal

Reconcile every figure in chapters 1–6 with the corrected explanations before the owner reads Part II.
Use the contract's figure medium and check the result at the book's actual reading size.

## Context

- Read the current contract, all accumulated documentation lessons and the node brief's hand-forward list.
- Draft `rhai-book-use-k84` and copy-edit `rhai-book-use-k85` apply findings F1–F16 and their general patterns.
  Read both stages' decisions and the resulting chapters before changing their figures.
- The new wording must make the caller's prompt, allowed results and subsequent actions clear.
  Configuration terminology must agree between prose, figure labels, alternatives and captions.
  The worker explanation must show Ravel's own executable in a separate process and preserve the order before the final launch.
- Earlier corrections added figures for the routing notice, inherited result context and caller roles.
  Preserve their concrete values, source accuracy and readability as the terminology changes.
- Proof `rhai-book-use-k87` already follows. This stage cuts no successor.

## Done when

- All figures in chapters 1–6 agree with the current prose, exact interfaces and source behaviour.
- Each figure serves a reader need. Labels use familiar terms, and the prose supplies the local context needed to understand it.
- Every affected Typst source, generated SVG, layout record, alternative and caption agrees.
- Figure reproduction and the book build pass. Inspect the changed figures in the rendered book at its normal column width.
- The running log states the exact visual coverage, viewing surface and any remaining limit.
  Mechanical image loading alone is not evidence that labels or arrows are readable.
- The stage clears its completed hand-forward entries and records any unresolved item for proof.
- New figure lessons extend the shared lessons record as reusable actions, with the Ravel application kept as evidence.

## Notes

Apply the whole accumulated figure guidance, not only the latest wording changes.
Shared figures may also appear in Parts III and IV. Record any dependent prose reconciliation for their scheduled drafts.
This is the second authorised correction run. Escalate any further substantial send-back.

## Decisions (running log)

- **Configuration terminology, 2026-10-28.** The figure labels use “Rhai worker”, “configuration script” and “settings”, while retaining literal interface names. The process view names the second copy of `ravel`, the isolation it supplies and the deadline limit.
- **Review flow, 2026-10-28.** A sequence diagram connects the concrete review request, result and next action in chapter 1. The outcome table maps all four cases. Chapter 5 repeats that mapping beside an executable example and an outcome view. A second diagram there was declined as an editorial judgement: it would repeat the mapping without adding a reader distinction. This is not a limitation of the medium.

## Checks

- The changed figure sources and their layout records, alternatives and local captions use the term map. The chapter 1 review sequence gives the concrete `changes-needed` path; the tables give the other caller actions. The Parts III and IV brief carries the shared-figure prose reconciliation.
- The local mdBook in Safari on an isolated macOS guest was viewed in its light theme at a 1920 × 1080 display and a 750-pixel chapter column. The review, routing notice, inherited paths, system context, process authority, successful session, evaluation harness, selection cancellation, settled exit status, source components and installed files were inspected, including the keys and the lower parts of tall diagrams. Temporary scroll aids in the generated HTML placed the figures in view; they did not change the source or figures.
- At that column width, ordinary text in the inherited-path and evaluation-harness figures calculates to 11.7 and 11.6 pixels respectively, close to the approximate 12-pixel target. The other changed figures calculate to at least 12.7 pixels. This inspection does not establish readability in other browsers, themes or smaller viewports, or with a new reader.
- The proof sibling `rhai-book-use-k87` already stands. No successor was added.
- `mise exec -- task setup` completed. `task docs:figures` reproduced all 18 figures, `task docs:book:build` built the book, and strict `task docs:ste` reported zero problems in chapters 1–6.
