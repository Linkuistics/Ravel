# rhai-book-use-k75

## Goal

Apply the owner's Read 1 patterns to chapters 1–6, on every page where they apply.
Keep the examples, the facts in the figures and the technical truth correct.

## Context

- Read 1, `rhai-book-use-k74`, stopped in chapter 1 on 2026-10-27.
  Its running log keeps the owner's words for findings F1–F8.
  The owner asked for the patterns of those findings in all of Parts I and II before a re-review.
- Section 2 of `docs/book/contract.md` states the patterns, after “Term list”.
  “Register”, “Terms” and “Stock phrasing” changed with them.
  The chapter 1 row of the section plan and two rows of “Figure needs” are new.
- The six passages are at commit `cf28b89f`, in chapter 1, “From a request to a result”:
  - F1, lines 51–54: Rhai gets one sentence. Chapter 2, lines 36–37, and chapter 4, line 6, repeat it.
    No chapter links to Rhai's own documentation.
  - F2, lines 58–62: “a selection” and “the notice that Ravel writes” have no explanation.
    Chapter 2, line 55, first says what a selection contains. Chapter 3, line 155, first says where the routing notice goes.
  - <!-- tc: depart synonym-rotation because the quote keeps the owner's word "modify" --> F3, line 64: write that the caller can use the kind “to extend or modify the prompt”.
  - F4, lines 93–98: put the reader's goal first. You may want an LLM to choose the best harness, model, effort and prompt additions.
  - F5, lines 115–118: “inherited session-result context” does not say how `end-session` gets the path.
  - F6, line 120 and the “Send” row on line 139: name the program that Ravel launched.
    End with the result that `ravel start-session` returns.
- F7 and F8 apply to every page. Repeat and restate instead of DRY. Use a conversational tone within the STE rules.
- Rhai's own documentation gave status 200 on 2026-10-27:
  - The Rhai Book, <https://rhai.rs/book/>, for Rhai 1.27.0. `Cargo.lock` has the same version.
  - The language reference, <https://rhai.rs/book/ref/>, and the feature list, <https://rhai.rs/book/about/features.html>.
  - The online playground, <https://rhai.rs/playground/stable/>. It has no `ravel::` host capabilities and none of Ravel's limits.
  - Do not link to <https://rhai.rs/book/language/>. That address returns 404.
- These facts need explanations. Confirm each one against the source and the specifications, not against this list:
  - A selection is the map that `select` returns. Chapter 3, lines 158–164, gives `program`, `args`, `provider`, `model` and `effort`.
  - Before the launch, Ravel writes the text routing notice as one line on the caller's standard error.
    With `--json`, a schema 2 document replaces it after the supervision attempt. See chapter 5, lines 587–650.
  - Ravel sets `RAVEL_EXIT_FILE` for each managed launch and evaluation harness run. The policy worker never receives it.
    See chapter 3, lines 234–240, and chapter 6, lines 582–592.
  - A child process inherits the environment of its parent. A nested `start-session` gives only its own child a new path.
    See chapter 5, lines 351–362.

## Done when

- Chapters 1–6 follow each pattern in section 2 of the contract, at the six passages and on every other page.
- Chapter 1 introduces Rhai where it first names it. It says what Rhai is and gives its characteristics for a policy.
  It shows a short example and links to the documentation above.
- Chapter 1 explains a selection, the routing notice and the path of `RAVEL_EXIT_FILE`.
  Each explanation has a concrete example with real values.
- The node brief has a “Handed forward” list. It gives art the place and the question of each new figure.
- Each complete example runs, and each fragment has its label. `task docs:examples` and `task docs:excerpts` pass.
- `task docs:ste -- --strict` and `task docs:breaks -- --strict` pass for chapters 1–6. `task docs:book:build` passes.
- Each link from chapters 7–11 and the README into chapters 1–6 still resolves.
- Each new lesson about documentation or communication is in `docs/documentation-lessons.md`.

## Notes

- If the six chapters do not fit one session, decompose this leaf with `grove-llm leaf-decompose`. Do Part I first.
- Copy-edit `rhai-book-use-k76`, art `rhai-book-use-k77` and proof `rhai-book-use-k78` already follow this leaf. Cut no next stage.
- This run is the first correction run against this document. Another send-back against it goes to the owner.
- Keep the release placeholder in chapter 3. Keep the recorded dependencies on the format cutover and the notice path.
- Do not change chapters 7–11. `rhai-book-inside` owns them.
