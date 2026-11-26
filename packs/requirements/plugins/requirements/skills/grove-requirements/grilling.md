<!-- bundled in grove from mattpocock/skills@d574778f94cf620fcc8ce741584093bc650a61d3 (skills/productivity/grilling/SKILL.md + skills/engineering/domain-modeling/SKILL.md) — MIT licensed; see LICENSES/mattpocock-skills.LICENSE -->
<!-- intentionally fused — upstream split them; grove has no skill-to-skill invocation, so the split would be cosmetic -->
<!-- upstream's `<supporting-info>` wrapper is dropped: it spanned eight units, so every unit it wrapped shipped inside an unclosed tag. `<what-to-do>` is kept — it sits wholly inside `grilling-interrogate` and delivers intact -->
<!-- the glossary, ADR-placement and test-seam sections upstream carried here were second statements of rules grove owns elsewhere; the glossary rules now live in CONTEXT-FORMAT.md, ADR placement in ADR-FORMAT.md, and the test-seam section splits — its grilling-timing half in this skill's `SKILL.md`, the rest (what a seam is, and where the agreement is recorded) in SPEC-FORMAT.md. This file points rather than restates -->

# Grilling — the `requirements`-task interrogation procedure

**This is a procedure reached on a condition, not a standing instruction.**
This skill's `SKILL.md` decides whether this session's open questions meet the
threshold for a full interview; read it there. Below that threshold nothing
on this page runs. Above it, this is the procedure.

<what-to-do>

Interview me relentlessly about every aspect of this plan until we reach a shared understanding. Walk down each branch of the design tree, resolving dependencies between decisions one-by-one. For each question, provide your recommended answer.

Ask the questions one at a time, waiting for feedback on each question before continuing. Asking multiple questions at once is bewildering.

If a *fact* can be found by exploring the codebase, look it up rather than asking me. The *decisions*, though, are mine — put each one to me and wait for my answer.

Do not commit decisions or grow the tree until I confirm we have reached a shared understanding.

</what-to-do>

## Discuss concrete scenarios

When domain relationships are being discussed, stress-test them with specific scenarios. Invent scenarios that probe edge cases and force the user to be precise about the boundaries between concepts.

## What the interview reaches for, and where each rule lives

The interview writes into artifacts whose rules are stated once, in the file a
session about to write one opens:

- Resolving a term, challenging one that conflicts with the glossary, and
  sharpening a fuzzy one — `CONTEXT-FORMAT.md`.
- Offering an ADR, and where records live in a single- or multi-context repo —
  `ADR-FORMAT.md`.
- Putting the test seams to the human while they are here —
  this skill's `SKILL.md`; `SPEC-FORMAT.md` says where the agreement is
  recorded.
