# rhai-book-use-k85

## Goal

Copy-edit all of chapters 1–6 after the owner-authorised corrections, before the Part II read resumes.
Make the prose clear, connected and consistent with the reader's knowledge and the current contract.

## Context

- Read `docs/book/contract.md`, all of `docs/documentation-lessons.md`, and the node brief's current hand-forward list.
- Draft `rhai-book-use-k84` applies the accumulated lessons and findings F1–F16 from `rhai-book-use-k74`.
  Read its completed work and decisions before editing. The owner completed Part I but did not report the required practical tasks.
- The owner prefers a concrete explanation before its general model, familiar configuration terminology, descriptive headings and familiar shell commands.
  Check these patterns throughout both parts, together with the earlier lessons about repetition, tone, local context and reader goals.
- Art `rhai-book-use-k86` and proof `rhai-book-use-k87` already follow. This stage cuts no successor.

## Done when

- Every page follows the prose contract. The meaning is easy to follow across sentences, as well as within each sentence.
- Terms agree across prose, headings, contents and references. The configuration script and settings remain distinguishable.
  Necessary interface tokens stay exact, while narrative JSON descriptions omit incidental version labels.
- Each heading describes its subject, action or reader question in familiar language.
  Procedures name familiar commands where that is clearer than an abstract paraphrase.
- Examples, explanations and their observed outcomes agree. Changed runnable examples and exact excerpts pass their checks.
- Strict prose checks and the book build pass for this stage's changes.
- The node brief records only unresolved work that later charters own. Art has each required figure wording or layout change.
- New documentation lessons retain their evidence and explain how the rule transfers to other documents.

## Notes

Read the whole document under the copy-edit charter. Art owns figure changes, and draft owns structural or technical rewrites.
This is the second correction run that the owner authorised. Escalate a further substantial send-back rather than growing an unattended repetition.

## Decisions (running log)

- **Deadline and process-group wording, 2026-10-28.** Four explanations said the parent can always stop a process at the deadline, but the reference also says a kernel stall can delay exit. They now describe the parent's control and its limit. The chapter 1 and 4 process-group explanations now say that descendants can leave the group, consistent with the operational limits.
- **Harness answer and final result, 2026-10-28.** Chapter 4 said the caller sees only the selection after a harness run. It now also names the final session result, which the caller receives after cleanup.
- **Cancellation condition, 2026-10-28.** Chapter 5's notice section implied that the cancellation status follows every harness cleanup failure. It now states that cancellation sets that status when it occurs.

## Checks

- Strict prose and prose-break checks passed for chapters 1–6, and the book built.
- All nine exact excerpts passed. All 22 runnable examples passed with none skipped; 57 fragments were not run. The final edits after that run changed prose wrapping only.
- The art and proof leaves already follow this stage. The document brief's art handoff now includes the limit on deadline enforcement.
