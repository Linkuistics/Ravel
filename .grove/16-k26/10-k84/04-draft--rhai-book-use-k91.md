# rhai-book-use-k91

## Goal

Apply all lessons and the term map to chapter 6, “Reference”. Then run the mechanical gates for all of chapters 1–6.

## Context

- The node brief `rhai-book-use-k84` gives the required corrections and the term map.
- Chapter 6 is the lookup chapter. A reader can arrive at any entry, so each entry restates its terms.
- Chapter 6 holds most of the JSON version labels (F11). Keep exact `schemaVersion` values in the entries that teach them.
- Several headings use “Policy”, such as “Policy entry and settings”. Keep identifier headings, such as `SelectionRequest`, exact.
- Draft `rhai-book-use-k89` renamed twelve chapter 4 headings and changed their 97 anchors in this chapter. Link texts that quote an old title or use the old terms remain, for example “Policy behaviour”, “Policy-context shape” and “copy a policy”. Make each link text agree with the new chapter 4 heading or with the term map. The new titles are in the running log of `rhai-book-use-k89`.
- Draft `rhai-book-use-k90` renamed chapter 5's “Understand the operational limits” to “Know when Ravel cannot guarantee cleanup” and changed its six anchors in this chapter.
  Their link texts still say “operational limits”, “support limits”, “Signal and recovery limits”, “Cleanup limits” and “platform limits”. Make each one agree with the new heading or name the row of its table.
- Chapter 5 now links to the entries “`--context PATH`” and “JSON failure document” for the fields and values of those documents. Keep those entries and their anchors.

## Done when

- Chapter 6 follows the term map, the contract and all lessons, with the earlier corrections preserved.
- `task docs:examples`, `task docs:excerpts`, `task docs:figures` and `task docs:book:build` pass.
- `task docs:ste -- --strict` and `task docs:breaks -- --strict` pass for chapters 1–6.
- Each link from chapters 7–11 and the README into chapters 1–6 resolves.
- New lessons are in `docs/documentation-lessons.md`.

## Notes

- This is the last draft under the node. Its retirement closes `rhai-book-use-k84`.
