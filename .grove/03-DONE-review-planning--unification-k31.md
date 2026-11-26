# unification-k31

**Reviews:** unification-k2

## Goal

Try to break the decomposition that `unification-k2` cut. Report findings,
not fixes.

## Context

- The artifact is the task tree under `.grove/` as `unification-k2` committed
  it. It includes the node briefs, the leaf bodies and the Decomposition
  section of the root brief.
- The contract is the task file of `unification-k2`, its "Done when" list, and
  the `grove-planning` skill.
- The requirements are the root brief and the running log of `plan-k1`.

## Done when

The review reports on each doubt below, with evidence from the tree.

- Does every done criterion of the root brief have a live leaf that owns it?
  Is any criterion owned only in part?
- The driver walks the tree in pre-order. Can any live leaf block the walk
  before work that does not depend on it?
- Does each increment leave the product working at its last leaf?
- Does each node earn its level, by the placement test in the spine's
  decomposition procedure?
- Do the research briefs name the downstream questions that their consumers
  need? Is the shape of each research question justified?
- Does each owner decision that is still open have a leaf with the owner in
  the loop?
- Does any leaf body contradict a decision in the running log of `plan-k1`?

## Notes

- Read the tree before the Decomposition section of the root brief. That
  section states the producer's reasons, and the review must judge the tree
  without them first.

## Findings

Reviewed the planned subtree in producer commit
`9e1a7edbe97af8c212158a86e15d68da8f012397` (`unification-k2`). The working
copy was empty on that commit when inspection began. Paths and line numbers
below refer to that producer commit, before the integration leaf is inserted.
Handles remain the way to find tasks after positions move.

### F1 — Separate the pending Gemini trial from independent later work

**Priority: P2.** At
`.grove/10-k9/06-impl--gemini-cli-adapter-k27.md:31-34`, Gemini CLI is known to
be absent, and the task must stay live if the owner cannot complete its real
trial. In the pre-order walk, that live task prevents `pack-form-k28`, the
whole `corpus-review-k10` node, and every later root entry from running.

The fake adapter work and the real trial are distinct handoffs here. After the
fake work lands, mechanical conversion can use that implemented adapter and
the agreed skill spec; it does not need the owner's Gemini sign-in. At least
the corpus method decision and text review can also proceed without that
sign-in. The current body explicitly permits the trial to be the only
remaining work, but provides no ordered sibling for it, so a known external
prerequisite stops work that does not consume its result.

This fails the review's dependency-order criterion. Retain the real trial as
live acceptance work, while allowing work independent of the sign-in to run.
This finding does not propose waiving the trial or claiming the Gemini adapter
has been tried before it has.

### F2 — Assign completion of the Claude Code and Codex real trials

**Priority: P2; unclear acceptance contract.** The last done criterion at
`.grove/10-k9/04-impl--claude-code-adapter-k25.md:26-28` and
`.grove/10-k9/05-impl--codex-adapter-k26.md:26-28` requires an opt-in Taskfile
task. Neither body explicitly requires executing it and recording its real
result before retirement. Neither names a later leaf that owns that execution.
Gemini's body, by contrast, explicitly keeps its trial live at lines 33-34.

The distinction matters because a task that can run is not evidence that the
adapter works on the real harness. The root brief requires acceptance tests
at its four agreed seams (`.grove/_BRIEF.md:55-56,126-142`), and the glossary's
definition of **Supported harness** includes trying it in real trials
(`CONTEXT.md`, under Language). Fakes and trial tasks alone can satisfy the
two adapter bodies without delivering that evidence.

Make the owner of each actual trial and its recorded outcome unambiguous.
Keep simulated and real evidence separate, and leave an unavailable trial
live, as the Gemini body already requires. This is a gap in assigned
verification, not a claim that either unimplemented adapter is defective.

### F3 — Resolve the contradictory DesignSkills conversion contracts

**Priority: P2.** The skill design's done criteria say DesignSkills stays in
its current form throughout this grove
(`.grove/10-k9/01-design--skill-contract-k22.md:45-46`). The skill node repeats
that it stays unconverted
(`.grove/10-k9/_skill-delivery.md:73-75`). However,
`design-skills-merge-k11` explicitly requires conversion to the one form in
its goal and done criteria
(`.grove/12-impl--design-skills-merge-k11.md:5-7,17-21`), and `pack-form-k28`
also assigns that conversion to it
(`.grove/10-k9/07-impl--pack-form-k28.md:29-30`).

Both instructions cannot govern the delivered transition. The spec producer
can agree a contract that excludes this conversion, while the merge leaf is
required to perform it. That leaves the compatibility and shipping contract
unclear at a boundary used by Grove. Agree one conversion scope across those
contracts. The owner's exclusion of DesignSkills from the language review is
separate from mechanical form conversion; it does not resolve this conflict.

## Review coverage

### Root acceptance ownership

Every root done criterion has a named owner or an explicit lazy planning
handoff. This table records ownership, not completed implementation or passing
checks. F2 and F3 identify where the assigned work is incomplete or inconsistent.

| Root criterion | Live owners in the producer tree |
|---|---|
| Embedded Lua | `configuration-contract-k19`, `lua-routing-k20`, `configuration-parity-k21` |
| Route and skill selection | `lua-routing-k20`, `skill-contract-k22`, `skill-selection-k24` |
| Rhai compatibility and removable module | `configuration-contract-k19`, `lua-routing-k20`, `configuration-parity-k21` |
| One skill form, namespaces and locations | `skill-contract-k22`, `skill-sources-k23`, `pack-form-k28`, `pack-shipping-k12`; DesignSkills scope conflicts in F3 |
| Descriptions | `skill-contract-k22`, `skill-sources-k23` |
| Supported harnesses and hooks | `claude-code-adapter-k25`, `codex-adapter-k26`, `gemini-cli-adapter-k27`, `pack-form-k28` |
| Other harnesses and documented alternatives | `skill-contract-k22`, `skill-selection-k24` |
| Inspection and exact caller task | `skill-contract-k22`, `skill-sources-k23`, `skill-selection-k24`; adapters extend the preview |
| Protected paths | `skill-contract-k22`, `skill-sources-k23`, `lua-routing-k20` |
| First merge stage and DesignSkills | `merge-terms-k15`, `skill-packs-merge-k16`, `universal-grammar-merge-k17`, `design-skills-merge-k11` |
| Corpus semantics, comparisons and owner decisions | `review-method-k29`, then `review-plan-k30` cuts the concrete leaves against `corpus-review-k10`'s acceptance |
| Four test seams and existing checks | `lua-routing-k20`, `configuration-parity-k21`, `skill-sources-k23`, `skill-selection-k24`, the adapters, and merge/conversion leaves; real execution is only partly explicit, F2 |
| Releases and migration notes | `pack-shipping-k12`, `release-k14` |
| Book, specs, ADRs, examples, packaging, notices and issue disposition | `configuration-contract-k19`, `skill-contract-k22`, `lua-routing-k20`, `book-k13`, `release-k14` |

### Dependency order and working increments

F1 is a concrete pre-order obstruction. The separate DesignSkills wait also
remains an external prerequisite to completing the grove; its body preserves
the owner's acceptance and offers an explicit owner decision if it has not
landed. It does not silently drop the merge. The first merge's publication
decision is deliberately before any public push. No finding asks for either
obligation to be bypassed.

The planned implementation leaves otherwise name verifiable behaviour:
whole-repository merges with checks, Lua routing with its worker contract,
configuration parity, static skill inspection, selected prompt delivery,
one adapter at a time, and mechanical pack conversion. Static inspection is
useful before selection, so `skill-sources-k23` is not merely a dormant
internal layer. Research, design, prototype and owner decisions have their own
inspectable deliverables. The corpus leaves appropriately wait for the review
method rather than guessing comparison work now.

The first merge, Lua routing and skill delivery each state a working final
handoff in their node briefs. No claim that those future checks already pass
is warranted. In particular, the suggested split in `lua-routing-k20` remains
subject to its worker, bounds and cancellation contract; a future split must
keep a working slice rather than treat those safeguards as optional.

### Node placement

Each node supplies a distinct subgoal, joint acceptance and context beyond
the root brief. `first-merge-stage-k3` holds history import, source authority
and Visualiser cleanup. `lua-configuration-k8` holds runtime selection,
worker isolation and Rhai coexistence. `skill-delivery-k9` holds the skill
contract and harness research. `corpus-review-k10` holds the comparison method
and policy-change review. Each adds one brief to the root-to-leaf chain. The
research pair remains flat, as does the planning review chain. These levels
earn their context; their mere number of sessions is not the justification.

The owner explicitly chose one grove for the increments. The planning skill's
default of separate groves therefore does not justify undoing that choice.

### Research questions and shapes

`harness-delivery-k4` and `harness-delivery-k5` carry the same downstream
questions for skill design, adapter implementation, prompt-only delivery and
the configuration prototype. They ask about prompt limits, file access,
native mechanisms, hooks, collisions, compaction, delegation and trial
sign-ins, and ask for dated failure evidence. `harness-delivery-k6` preserves
the consumer sections and disagreements. The pair is justified by the
different vendor implementations and the load-bearing shared evidence, rather
than by a wish to repeat one survey.

`lua-runtime-k18` names questions for configuration design, implementation and
release notices. Its single survey compares runtime and binding candidates,
conversion traps, libraries, native calls, bounds and vendoring against an
explicit fallback. It is a defensible default shape with empirical routing
checks later. No missing downstream research question was established by this
inspection.

### Owner decisions and requirements consistency

`merge-terms-k15` owns private-history publication and merge terms.
`configuration-experience-k7` obtains the owner's reaction.
`configuration-contract-k19` and `skill-contract-k22` explicitly require spec
agreement and a restatement, even though their kinds normally run AFK.
`review-method-k29` owns the comparison method; `review-plan-k30` must retain
owner decisions for substantive policy changes. `pack-shipping-k12` owns the
formula move and release arrangement. The DesignSkills and Gemini bodies
explicitly seek owner input for their outstanding prerequisites. Owner input
is assigned; F1 concerns where a wait stops independent work.

Against `plan-k1`, the tree retains the staged merges, the Visualiser cleanup,
the prohibition on merging Grove and Ravel-Strategies, Rhai's unchanged 2.x
behaviour, namespaces without collision precedence, optional metadata,
description fallback, migration notes and the four test seams. It retains the
32-skill edit scope and read-only treatment of TestAnyware, superpowers and
Quint. The one-grove decision and merging the two UniversalGrammar repositories
are explicit owner choices. F3 is an internal plan contradiction rather than
evidence that the owner rejected mechanical DesignSkills conversion.

## Inspection limits

This was an inspection-only review of the producer's committed markdown tree,
its requirements log, glossary, cited ADRs and the current statements and
comments of Ravel issues 4 and 5 and the statement of issue 6. It inspected
the producer commit and its root-brief diff. It did not inspect implementation
symbols or run tests, builds, lint or formatting commands. It makes no claim
about actual current harness capabilities or future implementation results.

The bootstrap read the root brief, including Decomposition, and the producer
task before reading this review's instruction to inspect the tree first.
Consequently this was not the requested rationale-blind first pass. That
limitation was reported to the human. Findings above rest on the cited task
contracts, not on adopting the producer's explanations.
