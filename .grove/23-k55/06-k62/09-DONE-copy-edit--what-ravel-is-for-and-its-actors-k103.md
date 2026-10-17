# what-ravel-is-for-and-its-actors-k103

## Goal

Copy-edit chapters 1 and 2 as one document under `docs/book/contract.md`,
after draft k102 corrects the account of what Ravel is for. Read the whole
unit and keep changes inside the copy-edit charter.

## Context

This is the document's second correction run, cut by read 1
(`book-read-one-k70`) and authorised by the owner on 2026-10-17. Draft k102's
body has the owner's findings in their words: R1, Ravel runs LLM sessions in
harnesses such as Claude Code and Codex, which typically cannot exit
themselves, hence `end-session`, so a harness can be a component of another
process, typically an orchestrator; and R2, “Publication is the start of
completion” did not make clear what was published.

Give the closest attention to the text k102 changed: chapter 1's opening,
"From a request to a result", "Decide whether Ravel fits", and chapter 2's
caller and harness definitions. Three things to check there:

- **Hedges.** The owner's words say “typically” twice. The prose contract
  allows no hedge without its condition, so the book states the condition
  (an interactive harness; a caller such as an orchestrator).
- **Terms.** "Harness", "LLM session", "orchestrator" and "interactive
  harness" are each spelled one way, match `CONTEXT.md` (**Caller**,
  **Harness**) and link at first use on each page to where chapter 2 defines
  them. "Ravel CLI" stays the term for the `ravel` program; no “front”, and
  no specialist substitutes such as “admits”.
- **Reader.** The contract's reader is a 16-year-old software engineer with
  the stated prerequisites. Do not assume they know how an agent harness
  behaves; the chapter says what they need.

Art k104 and proof k105 already stand after this leaf; do not append another
art or proof leaf. A structural or technical defect goes back to the owner as
an escalation, not as a third draft run.

## Done when

- The whole unit meets the prose contract without absorbing another charter.
- `task docs:book:build` and `task docs:examples` pass, and the document
  brief's handoffs are current.
- Retire this leaf with art k104 next.

## Decisions (running log)

- The prose contract is `docs/book/contract.md` §2 and the term list
  `CONTEXT.md`; neither names an outside precedent, so the first half's proved
  chapters are the uniformity reference.
- **Reader:** chapter 1's opening now says what a harness does (passes the
  prompt to the LLM and runs the commands it asks for, such as shell
  commands), so "the prompt tells the LLM to run `ravel end-session`" has its
  mechanism. Chapter 2's harness definition says the same. No claim about
  environment inheritance inside a real harness was added; no recorded check
  supports one.
- **Terms:** "orchestrator" is defined at first use on both pages (a program
  that runs LLM sessions as steps of its own work) and linked from chapter 1
  to `#caller-and-owner`; "LLM sessions in agent harnesses" links to
  `#the-managed-process-and-its-result`; chapter 1's first plain "session"
  is linked. Grove is "an orchestrator", not "it orchestrates".
- **Term risk:** "LLM session" and Ravel "session" are distinguished in
  chapter 1 ("From a request to a result") and in chapter 2's session
  definition; `CONTEXT.md`'s "Session" ambiguity gains the LLM-session case.
  The contract's term-risk list is the owner's and is left unchanged.
- **Hedges:** neither chapter contains "typically"; the owner's two
  conditions stand as "an interactive harness" and "a caller such as an
  orchestrator".
- Sentence fixes: "begin completion" became "a request to end the session",
  matching chapter 2's result-channel wording; process group is defined before
  it is used; "a published result can succeed" became "a session with a
  published result"; the fit paragraph's ambiguous "It" and undefined
  "completion protocol" became "Your program" and "the same ending command,
  `end-session`"; "If Ravel is killed" became "If the Ravel CLI is killed";
  the fake agent stands "in place of a harness running an LLM session".
- No structural or technical defect found; nothing escalated. Figures, keys,
  alt text, figure introductions and tables are art's and unchanged; art
  k104's handoff entry names the settled words.

## Verification and handoff

On 2026-10-17, after all edits: `task docs:book:build` passed without warnings
and `task docs:examples` passed all 13 complete examples (six fragments not
run, zero page errors). The two anchors the new links use exist in the built
chapter 2; a deliberately wrong anchor name was not found by the same check.
A whole-chapter enumeration of "LLM session", "orchestrator", "agent harness"
and "interactive harness" found one spelling each; "typically" occurs in
neither chapter, and the same search finds it in the contract. Headings are
unchanged, so every inbound anchor still resolves. This verifies build,
links, examples and spellings, not reader comprehension or real-harness
behaviour. Art k104 and proof k105 stand next; nothing was cut.
