# follow-a-session-through-the-code-k173

## Goal

Proof the whole of chapter 8, `docs/book/src/08-follow-a-session-through-the-code.md`,
against every class in the document contract. Fix defects within proof's
charter and leave the complete chapter green. This is the final unit stage,
not a review limited to the art diff.

## Context

- Read the node brief and ancestor brief chain, `docs/book/contract.md`,
  `docs/book/ste.md`, `docs/book/figures.md` and `docs/book/excerpts.md`.
  The audience reads Rust and TypeScript; the chapter gives source orientation,
  not a language tutorial or complete-source walkthrough. Follow the current
  graph and exact-source evidence rules for technical claims.
- Draft parts k155 and k170, copy-edit k171 and art k172 are terminal siblings.
  Their logs record source coverage, guide-statement destinations and the
  component-obligation inventory. Reconcile those records with the final
  chapter rather than accepting them as proof. Check the node's ten-obligation
  inventory and ownership of O3, O4, O5, O7 and O9. O6/O8 belong to chapter 9.
- Art added `session-code-path` after the opening trace table. Read its `.typ`
  source, generated layout and SVG, alt text and adjacent explanation. The
  runner stays in the original CLI; the second CLI sends the result. Worker
  cleanup precedes launch. Child-side terminal handover and parent-side lending
  are distinct. Escalation is optional; the later appearance check finds an
  early-exit result. Group stop precedes reap, and output follows final cleanup
  and the final outcome decision. The second CLI's exit/reap is omitted openly.
- `outcome-precedence` now shows all five source priorities from
  `Handlers::finalise`, including another CLI latch read before the earlier
  `look(true)` snapshot. It separates report order from outcome precedence:
  under cancellation, an original failure is reported before cleanup/group
  problems. This is source priority, not arrival-time ordering. Check the
  production `finalise` and `settle`, not the test-only `join_cancellation`.
- The two cancellation figures are unchanged chapter 5 sources. Their direct
  INT examples and attempted-cleanup limits must still agree with both chapters.
  Art converted combined worker-failure prose into a three-row precedence table
  with a retained-reports column. Verify the table against `worker::evaluate`.
- All art acceptance checks passed on unchanged inputs; k172 records commands,
  counts and the input hash. Its served-book inspection covered four figures at
  chapter width, both edited/new figures at full size, the new failure table
  and an absent-image negative control. Viewing limits are in `figures.md`.
- Dense figure text remains explicitly assigned to whole-book art k101. The new
  sequence's body text is about 8.4 px at 750 px chapter width. Its SVG is
  299,179 bytes; the eleven-figure set is 2,240,789 bytes, above the earlier
  2 MB benchmark. k101 owns that complete-set assessment too. Do not turn these
  observations into a claim of human comprehension or a changed medium contract.

## Done when

- The whole chapter, all figures/tables, terminology, claims, examples, excerpts,
  links and source order have passed proof against the amended contract.
- The reader can follow one session to the function and rule that choose its
  final cancellation outcome, and locate a result-channel change's consumers.
  Scope omissions and the separation of sent, candidate and returned results
  remain explicit. Inventory and guide-migration records match the delivered text.
- Book build, figure regeneration comparison, strict STE for the chapter and
  relevant layout records, strict prose breaks, excerpts and complete examples
  pass against final unchanged inputs. Stop a live book server before the final
  standalone build: they share `target/book`. Inspect reachable viewing surfaces
  and record their limits without claiming an owner comprehension result.
- Resolve all proof findings and apply the editorial correction-run rule if
  required. No ordinary successor is cut. On retirement, check the node's
  `Done when`, promote any context that future book siblings need, and name a
  node close in the commit only if all its obligations are fulfilled.

## Notes

There is no remaining chapter-local art handoff. The independent art reviewer
found no semantic defect in the two diagram sources; proof still owns a fresh
whole-document read. The whole-book art task and later set proof remain live.

## Decisions (running log)

- Read the whole chapter, the brief chain, glossary, contract, STE and excerpt
  conventions, figure conventions, all four figure sources and predecessor
  records. This proof owns the whole document and every defect class.
- The first prose use of “result channel” is in the opening path table. The
  term link previously appeared only after the figure. Link the table's use
  to chapter 2, as the prose contract requires. No source excerpt or technical
  claim changes.
- Production source confirms the five-source cancellation priority in
  `Handlers::finalise`, its block-before-late-read order, and `settle`'s
  report and outcome priorities. The worker table preserves overflow,
  original-conversation and teardown precedence. The full sequence preserves
  worker cleanup before launch, distinct terminal handovers, optional
  escalation, appearance after early exit and group stop before reap.
- No structural, technical or figure correction is needed from that read.
  The node has no remaining Handed forward list. Dense text and the complete
  figure-set size remain with the already-live whole-book art k101.

## Final obligation and migration check

Re-enumerated O1–O10 from the component guide's ten rows, checked against the
session/completion specifications and current production source. These are
teaching destinations, not claims of complete source coverage or new runtime
qualification.

| Obligation | Delivered destination and remaining owner |
|---|---|
| O1: owner policy, SDK, settings and discovery compatibility | Chapter 7, with chapter 8 “Prepare the request”; chapters 4/6 hold configuration detail |
| O2: worker identity and no result authority; reap before launch | “Evaluate and check the policy”, refusal and cancellation sections; chapter 7 explains the boundary |
| O3: judged decisions and limits without fallback; unchanged caller data | Request, policy and refusal sections, with isolated and combined failure tables |
| O4: direct argv, caller environment, requested confinement and nested result isolation | “Prepare the managed launch”, refusal section and change-boundary table; chapter 9 owns mechanics |
| O5: first complete result, empty/multiline/Unicode and invalid channel failure | “Send the result”, “Read the result” and the change scenario |
| O6: matching launch reap/group cleanup, independent child status | “Finish the managed job” and “Read the result”; chapter 9 owns detailed guarantees |
| O7: cancellation suppresses sent results and propagates after attempts | “Follow cancellation” and “Choose the final outcome”, including error-carried and late signals |
| O8: terminal recovery and group stop before reap | Managed-launch/job and cancellation sections; chapter 9 owns the conditional terminal/entry contract |
| O9: result-only stdout after cleanup, with write failures | “Prepare the managed launch”, “Complete cleanup and write the result” and final-outcome section |
| O10: matched installed pair and shipped SDK | Chapter 7, with identity checks in the policy section; chapter 12 owns delivery |

K155 and k170's guide-statement destination tables match the final chapter.
K170's O9 destination still uses the former heading “Clean up and write the
result”; its current destination is “Complete cleanup and write the result”.
The guide fold-in k164 can use that correction without rewriting the terminal
draft record. Context protocol, backend/terminal mechanics, detailed resource
recovery, test commands and independent fixture use retain their named chapter
9/11 owners. Neither historical native-entry debt in the guides nor their
three-source shorthand overrides the chapter's current five-source expression.

The read 2 task has a concrete answer: `Handlers::finalise` selects a signal by
source priority, then `settle` makes cancellation primary while retaining the
original and cleanup reports. “Find where a change belongs” names storage,
appearance, matching launch and final cleanup/decision responsibilities. Scope
omissions and sent/candidate/returned results remain explicit.

## Verification and limits

- Tier 2 verification used graph project
  `Users-antony-Development-Ravel.simplify-and-componentize`, full generation
  `2026-10-18T09:27:28Z`. The corrected exact-name search returned all 52
  matches with no further page. An initial regex file filter returned zero;
  the unfiltered exact-name search supplied the symbols. Both-direction,
  depth-1 traces of `settle`, `finalise`, `evaluate_and_resolve` and
  `supervise_reported` were untruncated. Spurious graph edges were treated as
  navigation only; exact snippets and direct source establish operation order.
- Coverage checked all relied-on source, chapter, figure, convention, guide,
  spec, ADR, Taskfile and brief/task paths. Production paths had no recorded
  issue and matching metadata. The terminal art leaf had unavailable tracking
  metadata and was read directly. The four SVGs are deliberately excluded;
  their sources/layouts, regeneration comparison and rendered views supply
  their evidence. Coverage is best effort, not completeness proof.
- After the final non-Grove edit, all six checks exited 0:
  `task docs:book:build` without warnings, `task docs:figures` (11 matches),
  strict STE on chapter 8 and its four figure layout records (0 problems),
  strict prose breaks (0 long runs in eight chapters), `task docs:excerpts`
  (8 passed, no page errors) and `task docs:examples` (13 passed, 0 failed,
  skipped or page errors; six labelled fragments not run). The preview server
  was stopped before the final standalone build.
- All 32 local link occurrences, including images, resolve. Anchor checks
  used built HTML IDs. The same checker rejected an absent file and an absent
  anchor. Strict STE rejected a planted semicolon in a scratch file.
- All 3,476 sorted tracked non-Grove file paths and their contents had the
  same SHA-256 record before and after the checks. Digest:
  `a1a1b1dd10da20e3d9ee0907c60ecb7191b1b624def63d77001e5a1e3ff72f33`.
  The scratch record is `/tmp/ravel-k173-before.json`. Only Grove retirement
  and promotion bookkeeping follow that measurement.
- Chrome's light locally served mdBook showed the path/failure/carrier tables,
  all four chapter figures and adjacent text at about 750 px chapter width.
  The sequence and activity full-size opening regions were also inspected.
  VS Code's light Restricted Mode preview showed the introduction, complete
  path table, corrected link and opening sequence. Raw files supplied the
  whole-document read. `docs/book/figures.md` records the viewing limits.
- Other themes, narrower viewports, GitHub and GitHub Pages were not
  inspected. Browser DOM evaluation timed out; native accessibility and
  screenshots worked. No human comprehension, real-agent, interactive terminal,
  new platform qualification or complete-source claim follows.
- The chapter node's Done when holds: SUMMARY links the chapter, all four
  stages are complete, source order and the real fake-agent main path were
  checked, and inventories/migration destinations agree. No correction run,
  ordinary successor, in-session reviewer or ADR change is needed. No live
  review names this leaf or node, so no Creator line is due. Retire k173 and
  close `follow-a-session-through-the-code-k154`; promote the useful context
  to the still-live book parent. Chapter 9 and later book work keep k55 open.
