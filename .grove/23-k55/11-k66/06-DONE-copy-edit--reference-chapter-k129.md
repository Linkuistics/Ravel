# reference-chapter-k129

## Goal

Make chapter 6 conform to the amended prose contract: ASD-STE100, the word
decisions in `docs/book/ste.md` and “send” for the result.

## Context

- Cause: read 1 (`book-read-one-k70`, log of 2026-10-17). The owner read
  chapter 1, made ASD-STE100 the prose base for all documentation and chose
  “send” to replace “publish” for the result. The owner did not read chapter
  6 yet. The contract change causes this run. It is not a finding against
  chapter 6.
- Read section 2 of `docs/book/contract.md`, `docs/book/ste.md` and
  `CONTEXT.md`. The STE specification is at the contract's path. Keep it and
  every extract out of the repository.
- Chapter 6 is the lookup chapter. Each entry is a direct entry, so it keeps
  its own context after the change. Entries and tables are text, so they
  follow STE too. Keep code identifiers, codes, statuses and paths exact.
- The `end-session RESULT` entry describes the command. K131 changes the
  command's help text to “send” after you. Describe the behaviour, not the
  help wording.
- Change the chapter's mentions of other chapters' titles to the contract's
  new titles. Links to the renamed heading of chapter 3 change in k126.
- Set art k130 owns the figure labels, keys, captions and alt text, and the
  colour text before each figure (R8). Do not edit figures.

## Done when

- Every sentence in chapter 6 follows the STE writing rules and the
  `ste.md` decisions: the sentence and paragraph limits, the active voice,
  simple tenses, no “-ing” forms outside technical names, no semicolons, no
  contractions and no phrasal verbs. British spelling stays. The book does
  not use the STE dictionary (owner, 2026-10-18).
- `task docs:ste -- --strict docs/book/src/06-reference.md` passes. A word in “-ing” that it
  reports and that is part of a technical name goes into `ste.md` as a
  decision, with its reason.
- No “publish”, “publication”, “submit” or “submission” for the result stays
  in chapter 6. A search with a control seen to fail shows this.
- Every local link and anchor in chapter 6 resolves. A control with a broken
  anchor fails.
- `task docs:book:build` and `task docs:examples` pass.

## Notes

- Cut nothing. Set art k130 and set proof k132 already stand for this
  document.
- Chapter 6 is the longest chapter. If it does not fit one session,
  decompose this leaf along the chapter's own inventories: the command line,
  exit statuses, streams and environment (inventory 1), then policy,
  settings, bounds, SDK and installed layout (inventory 2).

## Decisions (running log)

- One session holds the whole chapter. Decomposition along the two
  inventories is not necessary: most of the 420 problems that the strict
  check found before this stage are mechanical. 180 are semicolons, and most
  of these are in the “Stage; status” column of the two diagnostic-code
  tables. 36 are “including” and 20 are the column heading “Teaching”.
- The diagnostic-code tables split the column “Stage; status” into two
  columns, “Stage” and “Status”. The rows, their order and their facts do not
  change. A comma in place of the semicolon is ambiguous in a cell such as
  “`validation` or `context`, 3”.
- The column heading “Teaching” becomes “Taught in” in every table. The
  column gives the section of the book that teaches the entry.
- No heading changes. Each heading of chapter 6 is a code name or a noun
  phrase with no word in “-ing”. Thus, no anchor changes, and no other
  chapter needs a link change. The twelve links to chapter 5's renamed
  heading (from k128) stay as they are.
- Link texts with the old result words change: “Publication and absence”
  becomes “The sent result and its absence”, and “publish and accept
  results” becomes “send and accept results”. Their targets do not change.
- The settings-validation paragraph (one sentence of 29 words for the
  `settings_invalid` causes) becomes a short vertical list of those causes
  (Rule 4.3). The facts do not change.
- Paragraphs over six sentences split at a topic change, in place (for
  example `--prompt`, `--param`, `policy.ts`, `DeliveredContext`,
  `readChoice`). No sentence moves to a different entry. One sentence moved
  inside its entry: “`--prompt` accepts a value that starts with `-`” is now
  in the first paragraph of the `--prompt` entry, which gives the inputs.
- “A valid flag does not rescue an invalid setting” (a figure of speech)
  becomes “An invalid setting fails even with a valid flag”, in each of its
  four places.
- “Keep” is the result verb in `ste.md`. For diagnostics, an instruction
  says “keep” as chapter 5 does (“Keep the diagnostics”). For Ravel's own
  action on other data, the text says “retains” (reports, caller values,
  the diagnostics prefix) or “puts … unchanged” (the prompt bytes).
- A source-bound refusal reports `maxBytes` as the `from` of the bound when
  the read gave its own limit. The committed text said “a read's own limit
  is reported as `maxBytes`”. `crates/ravel/src/context.rs` (the
  `Origin::Set("maxBytes")` branch) confirms that `from` carries it. This is
  a clarification, not a new fact.
- The sample paragraph said “`policy.ts` becomes owner configuration only
  when copied”. Its referent is the installed sample, so the text now says
  “The sample becomes owner configuration only when you copy it”.
- As in k125–k128, this stage converts the words and sentences of the two
  alt texts and of the blockquotes before the figures, because the strict
  check reads them. Their facts stay. The colour sentences (R8) and every
  figure asset stay for set art k130.
- One in-session reviewer compared the committed and the new chapter for
  meaning changes and for prose-contract breaches that the script cannot
  find. It reported 28 items and no Markdown defect. Classification:
  - Valid and fixed, meaning: “implicitly” is back in the three stdin
    sentences (`--prompt-file /dev/stdin` is an explicit stdin prompt); the
    `start-session` return is again part of the invocation (“returns …,
    but only after”); `end-session` success means only that the result
    channel accepted the sent result, and the status-0 row says “confirms
    only”; “even if the loader catches the exception” again covers all
    three read-bound cases (now a short list); the `session_result_missing`
    remedy names the managed process as the caller of `end-session`; the
    installed `.d.ts` files are not called copies; the alt text repeats
    “The Ravel CLI” where “It” pointed at the runner; the notice note says
    “trusted policy code can still print a value itself”; the runtime-read
    overlap sentence no longer names overlap targets that the committed
    entry did not name; the stage table uses agentless noun phrases again,
    because the worker does `load`, `context`, `validation` and
    `selection`; “that the caller has set” again covers all five base
    variables; “the exhausted bound” and “ultimately” are back.
  - Valid and fixed, STE and contract: “keep” next to the result verb
    (“Quote it so that it stays one argument”, “Do not mix the result on
    stdout with stderr”); diagnostics and reports now use “retain” in every
    place, so the concept has one word; “the Ravel CLI” in place of a bare
    “the CLI”; “the policy-context loader” in place of “the load”, which
    clashed with the stage `load`; two passives with a known agent; “enter”
    in place of “go into”; a “must” that this stage added; “the other
    files” in a figure link now names the editable sources and
    declarations; “the usual outcome” in place of “the usual result”.
  - Visible trade-off: the sample's subject (“The sample”, a referent fix
    recorded above) and the one sentence moved inside the `--prompt` entry.
  - Noise: the dropped sentence “Absent if no such account was returned”.
    “Present only when …” in the same cell states it.

## Verification

- `task docs:ste -- --strict docs/book/src/06-reference.md`: 0 problems
  (420 before). The strict check on chapters 1–6 together: 0 problems.
  `task docs:ste:test` passes, with each mutant seen to fail a control. No
  “-ing” word needed a new `ste.md` decision.
- Result words: a case-insensitive search for publish, publication, submit
  and submission finds no line in chapter 6. The same search finds 20 lines
  in the committed chapter 6 and finds a planted “Submission”. No old
  chapter title, “front”, “admit” or “settlement” occurs in chapter 6.
- Links: a scratch checker resolved every local link, anchor and image in
  chapters 1–6 (408, of which 289 are in chapter 6) against the built HTML
  ids. A scratch copy of chapter 6 with one broken anchor
  (`#reserved-worker-grant`) failed it.
- `task docs:book:build` passed. `task docs:examples` passed all 13 complete
  examples (6 fragments not run), with the SHA-256 digests of chapters 1–6
  the same before and after the run. `task docs:figures` matched all nine
  figures. No figure asset, code block or product source changed. This is
  not a reader-comprehension result.

## Handoff

Cut nothing: set art k130 and set proof k132 already stand. No chapter 6
draft work was found. The parent brief's Read 1 section carries the art
handoff (the figure text of chapter 6 that still breaks the rules, and R8),
because k130 does not read this node's brief. This retirement leaves node
`reference-chapter-k66` with no live leaf, so k66 closes. No ADR or
`CONTEXT.md` change: this stage changed wording only.
