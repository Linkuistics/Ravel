---
name: editorial-family
description: The procedure that the draft, copy-edit, art and proof session kinds share. It states what a stage is, the one-charter rule, how the chain is cut and how work moves forward or back. Use when the grove-draft, grove-copy-edit, grove-art or grove-proof skill directs you here.
harnesses: [any]
---

**Load the `grove` skill now** if this session has not loaded it. This is a
bare skill name. If your harness requires a plugin namespace, use the
namespace under which the skill is actually installed. Do not assume one. If
you cannot find or read the skill, follow the launch prompt's rule for a
missing kind skill.

- **draft**, **copy-edit**, **art** and **proof** (all AFK) — the four stages of
  a document's publishing pipeline. Each is a **producer, not a reviewer**: it
  reads the whole document as its predecessor left it and *fixes*, where a
  `review-*` kind only reports. The contract the document declares — its
  specification, its prose rules, its validator — supplies every class a charter
  names; the stage supplies the discipline of reading for it.

A document declares its contract as `DOCUMENT-CONTRACT-FORMAT.md` directs.
Each stage's skill names the parts of the contract that the stage reads.

## Whole document, one charter

A stage reads the whole document and changes only what its own charter owns.
Work outside the charter is recorded and passed on, **never absorbed** — a stage
that quietly does another's job leaves the next stage's empty result unreadable,
since nobody can then tell *found nothing* from *pre-empted*.

## The chain is lazy, and its membership is not optional

A document's leaf becomes a node with
`grove-llm leaf-decompose <leaf> <slug> --kind draft`, and each stage's **last
act** is `grove-llm leaf-add <node> <stem> --kind <next>` — **unless a live later
sibling under this node already holds that stage**, in which case you cut
nothing, because an earlier stage's correction run has already queued it. That
condition is read off the node's live entries; there is no flag and no field, and
a stage that cuts unconditionally regrows the tail of the pipeline at every hop.
`proof` cuts nothing in the ordinary case. Cutting late is what lets the cutting
session write the next stage's body with the specific case it has just met.

**Do not skip a stage because you judge it will find nothing.** Which stages
exist was measured, and a skipped stage is this session re-grading a stage that
measurement already graded. That is the one place this family differs from a
review chain, where the producer genuinely decides whether the next step is
warranted. Skipping and *finding it already queued* are different: in the second
the stage is already standing ahead of you in the walk, and will run in its turn.

**Every stage leaf under a document's node carries the document's bare stem as
its whole slug** — no stage word, no suffix. The kind field beside it is the
canonical statement of which stage this is, and a slug restating it would be a
second, unvalidated copy of a fact grove already parses and routes on.

## Handing work forward

A defect a *later* stage's charter owns goes in the document node's brief,
under a running `## Handed forward` list naming the owning stage and the
location. Every later stage reads it, because the brief chain is root-to-leaf —
which the next leaf's body cannot do for a stage two hops away. A stage
**clears** the entries it closes: a brief is current-state context, not a log.

## Sending work back is forward tree growth, and it is one ordered run

`pick` is a depth-first pre-order walk; it cannot re-enter a retired leaf, and
there is no leaf state that means *reopened*. A defect an **earlier** stage's
charter owns, that you may not fix, becomes a **re-run leaf of that stage** — and
one leaf is never enough, because the material it changes is read again by every
stage after it. Cut, with `leaf-add` and in pipeline order, a **contiguous run**
that begins at the owning stage and ends at `proof`: <!-- tc: depart synonym-rotation because "launch prompt" above is a Grove term, not a verb, and this sentence is pinned -->

- the run always contains the stage that would otherwise have run next after
  you, and always ends at `proof`;
- omit a stage lying between the owning stage and you only where the changed
  material cannot reach its charter, and say why in the run's first body;
- the run **replaces** your ordinary last act. Do not cut your normal successor
  a second time beside it.

The chain is lazy, so nothing is queued behind you and call order is walk order;
use `leaf-insert` only where a later sibling entry already holds live work. Write
the specific defect into the re-run leaf's body. Every leaf in the run then finds
its own successor already standing and cuts nothing, which is what makes the
correction terminate rather than regrow itself.

**A second re-run of one stage against one document is an escalation, not a
third leaf.** Stop and say so. An unbounded re-run rule in an unattended arm is
an oscillation that spends sessions without terminating.

## Your review allowance is the ordinary producer's

No stage takes a `review-*` leaf, so no stage is the spine's
*already has a review beside it* case: every stage, `proof` included, is a plain
producer with the ordinary leaf-wide allowance of one in-session reviewer, spent
under that procedure's four-step pass. **The next stage is not that review.** It
reads the whole document against a *different* charter and may not repair your
class, so it neither looks for nor may act on the defects an adversarial read of
your own obligations would report. Where a **second** need appears, the
escalation the execute procedure names has no kind in this family. Finish to a
coherent boundary and route the doubt through the machinery that exists: the next
stage's body if it is that stage's class, the node brief's `## Handed forward` if
a later stage owns it, a correction run if an earlier stage does.

## Leave it green

A stage ends with the document passing whatever gate the repository runs over it,
in one focused commit like any other task.
