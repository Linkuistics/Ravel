# book-first-half-k147

## Goal

Proof chapters 1–6 as a set after the read-1 correction run for the owner's
findings R10–R17.

## Context

- The owner's findings and decisions are in `book-read-one-k70`'s log. The
  contract changed these sections: “The intended domain”, “Emphasis”, the
  section plan, “The reader's Ravel”, “Breaks in prose” and “Figure style”.
- Check these points across the set:
  - R12 and R13 wherever their concepts occur. When no chapter uses the old
    words, consider their addition to the checked table in
    `docs/book/ste.md`.
  - R14 and R15 agree in chapters 1, 2 and 4 and in the README.
  - R16 agrees in chapters 2, 5 and 6.
  - R17: each link to the installation in chapter 3 resolves. The version
    placeholder occurs one time and agrees with `book-release-version-k145`.
  - R11: the prose check passes on all six chapters.
  - R10: every figure uses the crisp style.
- This is the third proof run on the set. The owner authorised it on
  2026-10-18 in `book-read-one-k70`.

## Done when

- `task docs:book:build`, `task docs:figures`, `task docs:examples`,
  `task docs:ste -- --strict`, the strict prose check and `task check` pass
  on subjects that have the same digests before and after.
- Every local link destination resolves. Controls with a broken anchor and a
  missing file fail.
- Every figure loads in the served book. A control image with no file fails.
- No defect in the first half goes forward without an owner.

## Notes
