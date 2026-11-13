# rhai-book-inside-k121


## Goal

Check the figures and their alternatives in chapters 7–11 against the book's contract and current text.


## Context

The later grove made figures in `7d9526e9` and proofed the book in `424af53d`. Compare those results with the changes since then, including the promotion and installation corrections in drafts `rhai-book-inside-k36` and `rhai-book-inside-k120`. Copy-edit `rhai-book-inside-k119` changed the README and chapter 10 prose only. The node brief names the figures and the whole-book proof that follows.

## Done when

- The figures, alternatives and text meet the art charter and the book contract.
- `task docs:figures` passes, and the stage records its checks and any work handed forward.
- Cut the proof leaf under the editorial-family rule.
## Notes

## Decisions (running log)

- After the full art read, `mise exec -- task docs:figures` reproduced all
  21 figures. The book build passed without warnings, and strict STE and
  prose-break checks passed on the two edited chapters. No additional
  hand-forward finding arose from this art read. Proof must still perform
  the whole-book checks and arrange Read 2 as the node brief requires.

- Completed the art read of chapters 3–6. Compared the successful-session,
  session-outcomes, evaluation-harness and settled-exit-status sources with
  their adjacent alternatives and text. The roles and principal conditions
  are stated. The other figures in these chapters are reused figures checked
  with their other placements. Declined further figures as an editorial
  judgement: the tables, examples and ordered paths carry the other relations.
- Completed the contract read and the remaining figure comparisons recorded
  above. The earlier note that chapter 2 and 7 sources still needed comparison
  describes an intermediate state; the later comparison closes it.

- Compared the four chapter 1 figure sources, chapter 2's process-authority
  source, and the three chapter 7 figure sources with adjacent alternatives and
  prose. Their roles are stated and the tables and explanations carry their
  principal facts, including nested result paths, worker cleanup, and separate
  evaluation and final result channels. Declined further figures in chapter 1
  as an editorial judgement: its tables carry the remaining relationships.

- Read chapters 10 and 11 in full. Declined additional figures as an
  editorial judgement: chapter 10's tables map checks, test seams and evidence;
  chapter 11's tables and ordered lists carry the channel, promotion and
  recovery alternatives. Compared the installed-files and release-delivery
  sources with the chapter's adjacent alternatives and explanations. Their
  roles are stated, and the prose and tables carry their main relationships,
  including promotion and the preservation of the selected release version.

- Read chapter 9 in full and compared its escalation, deadline and terminal
  figure sources with their adjacent alternatives and prose; the reused
  supervision-cancellation source was also checked with chapter 8. Their
  roles are stated beside them, and the prose and tables carry their main
  sequences, alternatives and limits. Declined an additional figure as an
  editorial judgement: the other relationships are carried by tables and
  ordered lists.
- Read chapter 8 in full against the art charter. Its ordered tables and
  numbered paths carry the remaining sequences and comparisons, so declined
  an additional diagram as an editorial judgement. Compared the four diagram
  sources used there with their adjacent alternatives and explanations: the
  prose and tables carry their principal facts, including conditional cleanup
  and outcome priority. The roles are stated beside the diagrams.
- Also read chapter 2 and chapter 7 for relationships. Their tables carry
  the principal comparisons and sequences, so declined additional figures as
  an editorial judgement. Their existing diagram alternatives still need a
  source-by-source comparison before those chapters are complete.
- The changes since proof `424af53d` leave the installed-files and
  release-delivery figures applicable: their layout and channel choice remain
  general, while the current chapter text records the completed promotion.
  No renderer change or additional figure was warranted for that correction.
- Added explicit reader takeaways beside the source-components figure in
  chapter 7 and three figures in chapter 8. The figure sources and alternatives
  did not need changes for these takeaways.
- `mise exec -- task setup` succeeded. After the edits,
  `mise exec -- task docs:figures` reproduced all 21 figures, strict STE passed
  chapters 7 and 8, and `task docs:book:build` built the book without warnings.
- The full-document art read remains incomplete. The large combined chapter
  read was truncated, and the earlier whole-book proof is not a substitute for
  this stage's own read. The remaining work is to read the entire book against
  the art charter, check all figure roles and alternatives, record any declined
  figures or hand-forward items, rerun the appropriate gates, and cut proof.
  This session has insufficient remaining context for that read. Leave this
  leaf live for a fresh session; continuation would incorrectly acknowledge
  retirement, and decomposition would duplicate the existing art stage.
- The resumed art read did not reach a complete chapter: the chapter 1 read
  was truncated by the tool output limit. The book has approximately 110,000
  words across eleven chapters, and this session cannot complete the required
  full-document read within its remaining context. Read each chapter in bounded
  ranges in a fresh session, then check figure alternatives and roles, record
  declined figures and hand-forward work, rerun the gates, and queue proof.
  The existing art leaf remains live. Decomposition would duplicate that stage,
  and continuation would falsely signal retirement; force-stop preserves the
  work for a fresh Grove run.
- This session read chapter 1 for relationships and figure roles, including the
  section omitted by the truncated first output. No additional figure was
  identified in that chapter. This is a preliminary prose read, not a complete
  assessment of table roles or a comparison of diagrams with their sources
  and alternatives.
- The book contains about 121,000 words, and the contract read and chapter 1
  read exceeded the tool's output limit even when requested together or as a
  whole chapter. The remaining context cannot support the required complete
  read and figure comparison in this session. The art leaf stays live: read
  the contract and chapters in smaller ranges, compare each diagram and its
  alternative, record declined figures and hand-forward items, run the gates,
  and queue proof. Continuation would acknowledge unfinished work; cutting
  another editorial stage would duplicate the stage's whole-document charter.
