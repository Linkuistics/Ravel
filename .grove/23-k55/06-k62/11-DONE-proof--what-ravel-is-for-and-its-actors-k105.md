# what-ravel-is-for-and-its-actors-k105

## Goal

Proof the whole opening unit, chapters 1 and 2, under the amended
`docs/book/contract.md`, at the end of the document's second correction run:
technical truth, teaching, prose, terms, figures, text alternatives and the
viewing environments you can reach.

## Context

Read 1 (`book-read-one-k70`) cut this run from two owner findings, and the
owner authorised it on 2026-10-17. Draft k102's body has their words: R1, the
framing (Ravel runs LLM sessions in harnesses such as Claude Code and Codex,
which typically cannot exit themselves, hence `end-session`, so a harness can
be a component of another process, typically an orchestrator); and R2, the
sentence “Publication is the start of completion”, whose referent was unclear.
Draft k102, copy-edit k103 and art k104 have run before you.

Check in particular:

- **The domain statement** against the contract's "The intended domain",
  `CONTEXT.md` (**Caller**, **Harness**) and the rule in
  `docs/specs/session-execution.md` for what happens after publication.
- **What the book claims about real agents**: it can rest only on chapter 4's
  recorded help check of 2026-10-16 and the owner's statement, since no
  example runs a real agent.
- **R2**: the publication sentence names what is published, where it stands.
- **Links**: every inbound anchor still resolves. Chapters 3–6, `SUMMARY.md`,
  `README.md` and `docs/authority.md` link into chapter 2's five section
  anchors and chapter 1's `#from-a-request-to-a-result` and
  `#understand-the-evidence-limits`. `book-first-half-k100` checked all 403
  local chapter destinations; repeat a check of that kind across chapters 1–6,
  with a control seen to fail.
- **The figure** against its source, keys, alt text and the rendered book.

This is the last stage of the document's second correction run. If something
needs an earlier stage again, stop and put it to the owner as an escalation;
do not cut a third run. When you retire having sent nothing back, the
document node closes again: perform the ordinary close check against the
node brief's `Done when`, clear its `## Handed forward` list and promote what
remains to the book node brief. The book node still has live work (the README
leaf `book-readme`, read 1 and the whole-book art leaf), so the cascade
stops there.

## Done when

- The whole unit satisfies its contract and the document node's `Done when`;
  any viewing environment you could not reach is named with its limit.
- `task docs:figures`, `task docs:book:build` and `task docs:examples` pass,
  and the link check finds no broken destination.
- The handoff list is cleared and the node close is recorded. Proof cuts no
  successor.

## Decisions (running log)

- The whole opening unit agrees with the amended contract, term list and
  normative completion rules. R1 now places LLM harness sessions inside a
  caller's work; R2 names the result publication next to `end-session`.
  No further correction or earlier-stage re-run is indicated. Proof cuts no
  successor and needs no in-session reviewer.
- Keep the real-agent account bounded by the owner's statement and chapter
  4's dated help check: neither the examples nor this proof execute a real
  agent. The publication table gives the reason for explicit ending; chapter
  3 retains the grace/escalation mechanics and the distinction between a fake
  that exits and a harness that stays running.
- Close document node `what-ravel-is-for-and-its-actors-k62` after retirement:
  its two chapters, SUMMARY entries, terms, figures and editorial stages meet
  its Done when under the amended contract. Clear its handoff and promote
  this result and its limits to the book brief. The book's live README,
  renderer trial, reader checkpoint and whole-book art stop the cascade.

## Verification, 2026-10-17

Read both chapters completely, including the tables, keys and alternatives,
against `docs/book/contract.md`, `CONTEXT.md`, the session specification, the
completion specification's applicable completion/acquisition/failure rules and
the two cited ADRs. Compared the new publication sequence with chapter 3's
publication and cleanup explanation and the harness account with chapter 4's
2026-10-16 help record (Claude Code 2.1.301, codex-cli 0.163.0). These sources
agree on worker cleanup before launch, result publication before completion,
cancellation suppressing results, successful cleanup before return and the
separate final-write failure. No chapter or figure correction was needed.

Fresh required tasks all exited 0: `task docs:figures` matched nine figures;
`task docs:book:build` built without warnings; `task docs:examples` passed all
13 complete examples, with six labelled fragments not run, zero skipped
examples and zero page errors. No product source or example changed.

A link check enumerated the local href/src destinations in the six rendered
chapters and chapter-entry links from README, authority and SUMMARY: 538
destinations, zero broken files or anchors. All 18 rendered image elements
(inline and zoom copies) have nonempty alternatives. Known-good opening-unit
anchors passed; deliberately missing file, anchor and image destinations each
failed the same resolver. The first control invocation accidentally chose an
unordered chapter for its known-good chapter-1 anchor; the corrected invocation
names chapter 1 explicitly. That instrument error was not a document defect.

Read both PlantUML sources directly and compared source roles, relations and
legends with the chapter keys and alternatives. Independently inspected both
figures at chapter width in locally served mdBook through native Chrome on a
light page, the new publication table, result/status/work and fit tables, the
managed-process explanation and responsibility table, and the process figure
at full SVG size. VS Code's light Markdown preview in Restricted Mode showed
chapter 1's introduction, publication table, system-context figure and key,
and adjacent output teaching legibly. Raw Markdown supplies the complete local
checkout read. Other themes, narrower viewports, chapter 2's editor preview
and the full-size system-context link were not inspected. GitHub rendering
is unavailable for this unpublished branch; the published site awaits the
first release carrying the book. This is no new-reader comprehension result,
real-agent execution test, interactive-terminal check or platform qualification.

Tier 2 graph coverage was checked at generation `2026-10-17T01:06:35Z`:
the relied-on Markdown/config paths had matching metadata and no recorded
gap. PlantUML freshness is untracked and SVGs are excluded by suffix; direct
source reads, regeneration and the independent viewing above supply their
evidence. Coverage remains best-effort. No structural code claim or source
change required a code graph trace.
