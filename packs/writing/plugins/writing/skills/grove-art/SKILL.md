---
name: grove-art
description: The `art` session kind — the document's figures, in the medium its contract admits and nothing else. Use when a grove mandate names this skill, or when running an `art` session in a grove working tree.
harnesses: [any]
---

# art

**Load the `grove` skill now.** Then load the `editorial-family` skill. These
are bare skill names. If your harness requires a plugin namespace, use the
namespace under which each skill is actually installed. Do not assume one. If
you cannot find or read a skill named here, follow the launch prompt's rule for
a missing kind skill.

`draft`, `copy-edit`, `art` and `proof` are one family with one procedure, and
the `editorial-family` skill is it: what a stage is, the one-charter rule, how
the chain is cut, how work is handed forward and sent back, and what a stage's
review allowance is. This skill states none of it a second time. Everything else
— the constraints, the bootstrap, and the execute, decompose, retire and commit
procedures — is the spine's. What follows is `art`'s, and is stated nowhere
else.

**art** (AFK) — the document's figures: what a table, a list figure or a diagram
can carry, and nothing else.

## What it reads

Read the contract's figure needs and the expertise it declares for each
representation. Read the admitted media and renderers, the viewing
environments and the prose contract's terms.

## Meet the figure contract, and own the figure's text

<!-- tc: depart long-sentence because the conformance inventory pins this rule statement verbatim -->
**Every figure drawn or kept meets `FIGURE-FORMAT.md`, and `art` owns source
and rendering agreement.**

<!-- tc: depart long-sentence because the conformance inventory pins this rule statement verbatim -->
**Figure titles, labels, keys, captions, role statements and text
alternatives are `art`'s, written in the prose contract's terms and read with
the adjacent prose.**

## Work in the medium the document's contract admits, and no other

Where the contract admits only the page's own bytes, a figure is a table, a list
figure or a drawing inside a text fence, and there is no file beside the page.
**Do not introduce a medium the contract does not name.**

## Prefer a shared renderer when diagrams need a unified look

Where the contract admits generated figures, prefer **Typst/CeTZ** for final
drawing when different diagram tools would otherwise produce inconsistent
fonts, line weights, colours or spacing. This is a preference, not a requirement
to migrate every diagram. Preserve the notation's meaning: common styling must
not erase distinctions such as arrow kinds, boundaries or sequence ordering.

Mermaid, PlantUML, Graphviz and similar tools may supply layout, but adopting
their coordinates is a pipeline to prove, not an assumed conversion. Measure
labels with the final fonts and sizes before layout, or recompute layout when
those metrics change. Share typography, strokes, colours, padding and arrow
styles in one drawing module. [CeTZ](https://typst.app/universe/package/cetz/)
provides the drawing layer; notation-specific libraries such as
[Fletcher](https://typst.app/universe/package/fletcher/) can build on it.

Before a broad migration, trial representative diagrams with different
notation and density — for example a dense architecture view and a sequence.
Compare them at publication width, then lengthen a label and add a participant.
Evaluate legibility, semantic fidelity, regeneration and maintenance effort,
and get the owner's reaction before choosing a document-wide migration.

Keep **one authoritative source per figure**: either generate the final drawing
from the layout source, or make the final drawing source authoritative and
retire the superseded source. Do not maintain two independent editable copies.
The figure contract states what a rendered figure records and how it is
checked. Keep the existing figures until the trial supports changing the
document's rendering contract.

## Two rules, and they are the whole charter

- A relation the reader would otherwise have to hold in their head — a sequence,
  a partition, a comparison, a mapping over a set of members — **carried only by
  running prose where a figure would carry it, is a defect**.
- **Every figure states its role**: adjacent to it, a sentence saying what the
  reader is to take from it, not merely what its subject is.

## Record a figure you declined to draw, and say which of three reasons it was

An editorial judgement, a rule in the contract that forbids the fix, and *the
medium could not carry it* are three different findings, and the third is the
only one that is evidence about the medium itself. Name it as such when it is
true, and do not reach for it when the real reason is one of the other two — a
contract's medium is reopened by that record and by nothing else. A host that
cannot render the admitted notation is a case of *the medium could not carry
it*.

**Last act:** `leaf-add` the `proof` leaf, under the family file's rule — which
is conditional, and which a correction run discharges for you.
