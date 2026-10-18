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

`system-context.typ` and `process-authority.typ`, with their generated SVGs
and layout records, are under `docs/book/src/figures/`. The shared `_*.typ`
modules and `docs/book/figures.md` record the visual vocabulary. Chapters
carry role sentences, notation keys and text alternatives. The existing fit
and responsibility tables and new result/status/work, nested publication,
information-kind and platform-evidence tables carry the unit's comparisons.
The correction passes add the worker-directory sequence table, distinguishing
startup location, the move before policy import and cleanup before launch,
and the chapter 1 publication/stop/cleanup/return table. The two C4 views,
their text alternatives and the fit/responsibility tables now name the
orchestrator and LLM harness while preserving the generic process boundary.

## Read 1 correction run (2026-10-17)

The owner's read 1 (`book-read-one-k70`) found R3–R7 and R9 in chapter 1,
made ASD-STE100 the prose base for all documentation and chose “send” to
replace “publish” for the result. The owner authorised this third
correction run on this document. A fourth run goes to the owner. Draft
k124 corrects the findings. Copy-edit k125 converts chapters 1 and 2 to
STE, with the word decisions in `docs/book/ste.md` (from k123).
Set art `book-first-half-k130` and set proof `book-first-half-k132` serve
this document. Thus, no stage under this node cuts an art or proof leaf.

## Read 1 correction run (2026-10-18)

The owner's full read 1 (`book-read-one-k70`) found R10–R17. Four reach this
document: the reason for a harness (R14), the loose contract (R15), the race
of a second `end-session` (R16) and runs of prose that are longer than the
new limit (R11). Two wording decisions (R12 and R13) are in
`docs/book/ste.md`. The owner authorised this fourth correction run on
2026-10-18. Another send-back on this document goes to the owner. Draft
k134 and copy-edit k135 run under this node. Set art `book-first-half-k144`
(the crisp figure style, R10) and set proof `book-first-half-k147` serve this
document. Thus, no stage under this node cuts an art or proof leaf.

## Handed forward

- Drafts k140 and k142, chapters 5 and 6, R16: use this wording, with the
  local explanation of the result channel:

  > Ravel keeps the first complete result that it receives. That first
  > `end-session` requests shutdown, which normally ends the harness. A
  > second `end-session` races that shutdown. For example, another process
  > in the group can send a result before Ravel stops it. Ravel keeps the
  > first result, even if the second command succeeds.

  State the schedule where the page needs it: after Ravel detects the result,
  it gives the managed process 2 seconds to exit. If it still runs, Ravel
  sends TERM and waits up to 5 seconds, then sends KILL if necessary. These
  waits end early if the managed process exits. Ravel stops any group
  members that remain before it reaps the managed process. A result requests
  shutdown, not proof that cleanup succeeded. Cancellation and cleanup failure
  still prevent a successful result.
- Set art k144: no new figure is needed for R11 in chapters 1 and 2. The
  tables answer these questions: who chooses what in a review, what the
  selected program needs to do, which guarantees have limits, which chapter
  route serves each reader, what caller and owner decide, what cleanup
  precedes launch, and when a second sender can race shutdown. The existing
  C4 figures keep their questions and content. Apply the queued crisp style.
  R8 stays closed: the prose does not repeat the figure colour keys.
- Copy-edit k135: R14's conditional cost statement expresses the purpose of
  subscription access, not a measured comparison or a universal saving.
  The linked official authentication pages confirm the access options as of
  18 October 2026. Preserve the distinction between a required caller prompt
  and the policy's choice to pass it to the selected program (R15).
