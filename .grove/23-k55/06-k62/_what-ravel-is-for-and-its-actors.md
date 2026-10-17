# what-ravel-is-for-and-its-actors-k62 — brief

## Goal

Chapters 1 and 2 of the book, as one document of two files
(`docs/book/src/01-what-ravel-is-for.md`, `docs/book/src/02-actors-and-words.md`),
taken through draft → copy-edit → art → proof. The contract is
`docs/book/contract.md`: its "The reader", "The intended domain", "The section
plan", "Emphasis", "Exclusions" and "Term risks" are the structure brief.

After chapter 1 the reader can say what Ravel solves, including why
`end-session` exists, what it leaves to callers (no store, no task meaning, no
model integration) and whether it fits their need. After chapter 2 the reader can name the caller, owner, owner policy,
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
- Read 1 (`book-read-one-k70`) found the account of what Ravel is for too
  generic: owner findings R1 and R2, quoted in draft k102's body. The
  contract's "The intended domain" now says that Ravel runs LLM sessions in
  agent harnesses as supervised parts of another program, and why
  `end-session` exists. The document's second correction run, draft k102 →
  copy-edit k103 → art k104 → proof k105, carries it; the owner authorised it
  on 2026-10-17. Any further send-back against this document goes to the
  owner.

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

## Figures delivered

`system-context.puml` and `process-authority.puml`, with generated SVGs, are
under `docs/book/src/figures/`. The shared `_style.puml` and
`docs/book/figures.md` record the visual vocabulary for later units. Chapters
carry role sentences, notation keys and text alternatives. The existing fit
and responsibility tables and new result/status/work, nested publication,
information-kind and platform-evidence tables carry the unit's comparisons.
The correction pass adds the worker-directory sequence table, distinguishing
startup location, the move before policy import and cleanup before launch.

## Handed forward

- **Art k104:** chapter 1 now leads with an orchestrator using an LLM session
  in a harness, and explains result publication followed by Ravel stopping
  remaining group members before cleanup. Align the system-context figure,
  its introduction and alt text with that account, and check the fit table
  under "Decide whether Ravel fits" and chapter 2's process-authority figure
  and responsibility table. The generic labels remain mechanically true but
  need to carry the document's intended domain. This is the already-queued
  R1 art correction; draft changed no figure, key, alt text or table.
  Copy-edit k103 fixed the prose words art should reuse: "LLM session",
  "harness" (the program that passes the prompt to the LLM and runs the
  commands it asks for), "interactive harness" and "orchestrator" (a program
  that runs LLM sessions as steps of its own work). An LLM session is not the
  Ravel session. Copy-edit changed no figure, key, alt text or table either.

