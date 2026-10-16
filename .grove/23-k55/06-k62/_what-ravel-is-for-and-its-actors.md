# what-ravel-is-for-and-its-actors-k62 — brief

## Goal

Chapters 1 and 2 of the book, as one document of two files
(`docs/book/src/01-what-ravel-is-for.md`, `docs/book/src/02-actors-and-words.md`),
taken through draft → copy-edit → art → proof. The contract is
`docs/book/contract.md`: its "The reader", "The intended domain", "The section
plan", "Emphasis", "Exclusions" and "Term risks" are the structure brief.

After chapter 1 the reader can say what Ravel solves, what it leaves to callers
(no store, no task meaning, no model integration) and whether it fits their
need. After chapter 2 the reader can name the caller, owner, owner policy,
policy worker, managed process and opaque result, and say who holds which
authority.

## Context

- Beyond the brief chain: `CONTEXT.md` (the term list, including its flagged
  ambiguities), `docs/session-cli.md` and `docs/authority.md` (being replaced:
  verify each claim against the spec), `docs/specs/session-execution.md`,
  `docs/adr/ravel-supervises-the-session.md` and
  `docs/adr/policy-evaluation-precedes-the-launch.md` for the reasons, and
  `docs/caller-references.md` for how Grove may appear (one labelled caller).
- This is the first chapter set: it teaches the domain, not mechanics. Process
  groups, the terminal and reaping are named at most; chapter 3 introduces
  them as a session needs them and chapter 9 teaches them.
- Chapter 1 carries the fit test: situations Ravel suits and situations it does
  not (it routes and supervises; it does not store, schedule, meter or choose a
  model). The retired actor daemon, dispatch and the Grove extraction get one
  pointer line at most.
- Chapter 2 carries the first statement of authority: owner policy is trusted
  code with no result authority, the worker is reaped before anything launches,
  only the native front supervises, each managed process holds only its own
  result context. It also states, plainly, that same-user code can tamper and
  that the context is isolation, not authentication.

## Done when

- Both chapters exist, are linked from `SUMMARY.md`, and the unit has passed
  draft, copy-edit, art and proof under the contract.
- Each term on the contract's term list is defined once, where first needed,
  and `CONTEXT.md` is updated for any term the chapters add.
- Figures from the contract's list for these chapters exist (system context for
  chapters 1–2, container in chapter 2) with notation explained where first
  used; the unit's art stage fixes the book's visual vocabulary.
- Every fragment says what it omits; no complete example is required here.

## Notes

- The reader is the contract's reader. Do not teach shell, Git, JSON, Rust,
  TypeScript or the general OS concepts it assumes.

## Handed forward

- **Art** — supply the contract's system-context figure for chapters 1–2 and
  process/authority container figure in chapter 2. The text is complete without
  figure placeholders. The question for the first is where caller, owner, Ravel
  and selected program meet; the second must show the worker ending before the
  managed launch, the runner inside the native front, and the separate policy
  response and result paths. Explain the notation locally and establish the
  visual vocabulary for later units. Same-user and detached-child limits remain
  explicit in the text; boxes do not claim authentication or universal containment.
- **Proof** — chapters 3–6 do not exist at this draft, so their forward mentions
  are plain chapter numbers/titles. Link them when targets exist during the
  first-half set proof; chapter 6 remains page-level. The parent already carries
  the second-half conversion for chapters 7–12. Check every first-use term link
  against the final wording and figures.
