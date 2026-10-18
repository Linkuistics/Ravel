# what-ravel-is-for-and-its-actors-k134

## Goal

Draft chapters 1 and 2 again for the owner's read-1 findings R14, R15, R16
and R11. This is the fourth correction run on this document. The owner
authorised it on 2026-10-18 in `book-read-one-k70`. Another send-back on this
document goes to the owner.

## Context

- The owner's words and the locations at `1ef02398` are in
  `book-read-one-k70`'s log, “Owner findings R10–R17”. The contract records
  the decisions in place: “The intended domain”, “Emphasis”, the chapter 1
  row of the section plan and “Breaks in prose”.
- **R14, the reason for a harness.** Chapter 1, lines 1–30, introduce the
  harness without the reason. Where the chapter first describes Ravel, say
  that a harness can run under the subscription of its user. Thus, a caller
  can use that subscription instead of paid API access, which costs much
  more. Do not name a price or a plan. Point to the documentation of each
  harness for its plans and terms. The contract's “Exclusions” keep accounts
  and credentials out of the book.
- **R15, the loose contract.** Chapter 1, lines 60–63, say this in two
  sentences. Make it clear and easy to find. The policy selects a program,
  Ravel runs it, and a program in the session sends one value with
  `end-session`. Ravel needs nothing more from the program. `start-session`
  takes a prompt (`--prompt` or `--prompt-file`), and its help calls it “The
  harness prompt”. But the policy decides which program runs and whether that
  program receives the prompt. A table of what Ravel needs and what it only
  expects can carry this, and it also breaks a long run. Check the section of
  chapter 2 on the managed process against the same statement.
- **R16, the race.** Chapter 2, line 128: “A later `end-session` does not
  replace it.” Say that this is a race. The first `end-session` normally ends
  the harness. Ravel waits 2 seconds, sends TERM, waits up to 5 seconds, then
  sends KILL and reaps the group (`ravel start-session --help` and
  `docs/specs/session-execution.md`). A second `end-session` can arrive only
  in that short time, for example from another process in the group. Ravel
  keeps the first result. Chapters 5 and 6 have the same sentence, and their
  drafts in this run use your wording. Write that wording in this node's
  brief for them.
- **R11, breaks in prose.** Chapter 1 has runs of 900 words (lines 1–107)
  and 723 words (lines 183–266). Chapter 2 has a run of 1,329 words (lines
  1–155). Measure with the check from `book-prose-breaks-check-k133`. Add a
  table where items compare on the same attributes. Where a figure answers a
  reader question better, write the figure need under `## Handed forward` in
  this node's brief, with its reader question, for set art
  `book-first-half-k144`. Do not add decoration.
- **R12 and R13** are wording for the copy-edit (`docs/book/ste.md`, “The
  result”). Apply them only in a passage that you write again.
- Keep the earlier corrections R1–R9, the STE writing rules and “send”.
  Other pages link to anchors in these chapters. Before you change a heading,
  find its inbound links with a search that you see find a known link. Then
  change each one.
- Set art `book-first-half-k144` and set proof `book-first-half-k147` serve
  all six chapters. Thus, this node gets no art or proof leaf.

## Done when

- Chapters 1 and 2 state R14, R15 and R16 in the book's own words. Each
  technical statement agrees with the source and the specifications.
- The prose check finds no run that is longer than the limit in chapters 1
  and 2.
- This node's brief gives the race wording and each figure need, with its
  reader question.
- Every link to a changed anchor works. A search with a control shows it.
- `task docs:book:build` and `task docs:examples` pass.

## Notes

- Do not state facts about real agents or subscription terms beyond the
  documentation of each harness. Record the date and the source of each such
  fact that you use.
- Change `CONTEXT.md` only if R15 changes a definition, for example
  **Harness** or **Managed process**.

## Decisions (running log)

- 2026-10-18: The structure brief is `docs/book/contract.md`. “The reader”
  names a “16-year-old software engineer with no background in Ravel”.
  “The section plan” orders purpose before actors, then a complete session.
  “Emphasis” names the harness reason, loose contract, success and authority.
  “Exclusions” keeps accounts and credentials outside the book. The existing
  chapter order and headings serve those outcomes and stay unchanged.
- 2026-10-18: Compare the review scenario's participants, the required and
  expected program behaviour, the supervision limits and the reading routes
  in tables. Chapter 2 also compares caller/owner decisions, worker cleanup
  conditions and the steps after a result. These tables answer concrete
  reader questions and break the three long runs found by `docs:breaks`.
- 2026-10-18: Official authentication documentation confirms subscription
  access for both named harnesses. Describe the cost advantage conditionally,
  with no plan, price or universal saving claim. Sources checked today:
  <https://learn.chatgpt.com/docs/auth> (the Codex authentication URL redirects
  here) and <https://code.claude.com/docs/en/authentication>. Neither page
  establishes a measured cost comparison for the reader's work.
- 2026-10-18: The first result starts shutdown, so a second sender races
  shutdown and cannot replace that result. The source checks child exit
  before each escalation step. State the 2-second/TERM/5-second/KILL schedule
  conditionally, with early exit and cleanup before reap. No glossary
  definition changes. Copy-edit k135 already holds the next stage.
- 2026-10-18: One focused fresh-context review checked the loose contract
  and the result race against source and the official access documentation.
  It reported no findings. This was one aimed technical read, not a reader
  comprehension test or an exhaustive code audit. No second review is needed.

## Evidence and validation

The draft owns structure and technical truth. The chapter sequence still
teaches purpose, responsibilities, then the practical session in chapter 3.
The required/expected comparison makes the loose contract visible without
changing a term definition. The role, limit and route comparisons retain the
earlier corrections. The race text distinguishes a request to stop from
successful cleanup. The next copy-edit retains R12/R13 and its full charter.

Tier 2 graph discovery and exact source checks used project
`Users-antony-Development-Ravel.simplify-and-componentize`, initially at
generation `2026-10-18T05:59:24Z`. Narrowed code search returned all 16
matches without pagination. The outbound `end_session` trace led to
`publish_result`; one unrelated `filter` edge was not used as evidence.
Coverage reported matching metadata and no recorded issues for each source,
chapter, contract, specification, ADR, glossary and task/brief path used.
This is best-effort coverage, not proof that the graph is complete.

- `crates/ravel/src/inputs.rs:92–110` and `run.rs:130–134`: required caller
  prompt and the selected program's arguments. `docs/specs/session-execution.md`,
  “Request, context and selection”, gives the same boundary.
- `crates/ravel/src/completion.rs:160–213`: inherited path, complete result
  write, and a hard link that does not replace the first result. A repeated
  valid call can succeed without replacing the value.
- `crates/ravel/src/run.rs:20–23` and
  `crates/keyed-launch/src/run.rs:2033–2196`: the 2-second and 5-second
  waits, child exit before escalation, group stop before reap. The built
  `start-session --help` and the session specification agree.
- Both specifications and the three ADRs retain the cleanup, authority,
  failure and support limits of the chapters. The glossary stays unchanged.
- The dated official access sources are in the decision above and chapter 1.
  No real agent, personal account or model service was used in validation.

The initial strict prose check found 885- and 703-word runs in chapter 1
and a 1,302-word run in chapter 2. The final strict check finds none above
450 words. The strict STE check reports no problems in either chapter.
The local STE PDF matches the digest in the contract.

Both heading lists match their parent revision exactly, so no anchor changed.
A literal search found the known links from chapter 1 to chapter 2's
managed-process anchor. Tables retain the chapter destinations of the prose
they replace. The book build passes without warnings. All 13 complete
examples pass, with zero failures or skips; six labelled fragments do not run.
All VCS-tracked files were digested before and after the build/example run.
The digests match. The final active-voice correction to the dated source
sentence receives a fresh build and both strict prose checks.

The node brief supplies the shared race wording to drafts k140 and k142,
which already point there. It also records that no new figure is needed for
these chapters and preserves set art k144's existing scope. No successor
leaf is cut: live copy-edit k135 already holds that stage. No parent closes,
and no ADR decision changes.
