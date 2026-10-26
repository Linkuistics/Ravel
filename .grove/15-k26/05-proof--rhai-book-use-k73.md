# rhai-book-use-k73

## Goal

Read Parts I and II as one document and complete the proof charter for chapters 1–6.
Use `docs/book/contract.md` and the node brief's remaining handoffs.

## Context

- Draft and copy-edit describe the Ravel 2.0.0 design before the scheduled TypeScript removal.
  Keep that dependency explicit. The current checkout is not the finished release.
- Art `rhai-book-use-k72` updated seven figures with the existing Typst/CeTZ style.
  Its task file records source checks, viewing evidence, declined figures and validation.
- The process-authority view omits evaluation harness runs explicitly.
  The source-components view shows them, and the selection-cancellation sequence shows their cleanup before worker teardown.
  Harness-run data reaches the policy. It cannot complete the final session.
- The successful sequence follows chapter 3's policy, which makes no host call.
  Its former change to `/` is gone.
- The final-outcome key distinguishes failed harness cleanup during evaluation from later cleanup after an original failure.
- Chapter 6 shows one executable and three editable Rhai examples.
  The third-party notice path still belongs to the delivery-removal increment.
- Chapter 8's old cancellation example and alternative await its already scheduled draft.
  The Parts III and IV node brief records the mismatch with the shared figure.
  Do not expand this proof into that document's rewrite.

## Done when

- The proof reads every chapter, table, example, alternative, caption and linked figure in Parts I and II.
- Coverage inventories come from the source, as the contract requires.
  The remaining node handoffs close or pass to their named owners.
- The applicable book gates pass, including examples, excerpts, figures, strict prose checks and the build.
- The local book receives a visual inspection. State which other viewing environments remain unavailable or uninspected.
- Cut the required Read 1 requirements leaf after proof, with the bare document stem.
  The owner uses Read 1 to write a fake-agent policy and run a session.
  The owner interprets refusal and cancellation, then uses a direct reference entry.
  Both reader checkpoints remain required. This proof does not replace the owner's read.
