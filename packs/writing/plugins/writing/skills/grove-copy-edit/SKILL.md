---
name: grove-copy-edit
description: The `copy-edit` session kind — sentences, terminology and consistency with the document's prose contract, without restructuring. Use when a grove mandate names this skill, or when running a `copy-edit` session in a grove working tree.
harnesses: [any]
---

# copy-edit

**Load the `grove` skill now.** Then load the `editorial-family` skill. These
are bare skill names. If your harness requires a plugin namespace, use the
namespace under which each skill is actually installed. Do not assume one. If
you cannot find or read a skill named here, follow the launch prompt's rule for
a missing kind skill.

`draft`, `copy-edit`, `art` and `proof` are one family with one procedure. The
`editorial-family` skill states it. It says what a stage is and states the
one-charter rule. It says how the chain is cut, how work is handed forward and
sent back, and what a stage's review allowance is. This skill states none of it a second time.

Everything else
— the constraints, the bootstrap, and the execute, decompose, retire and commit
procedures — is the spine's. What follows is `copy-edit`'s, and is stated
nowhere else.

<!-- tc: depart long-sentence,synonym-rotation because the conformance inventory pins this rule statement verbatim -->
**copy-edit** (AFK) — sentences, terminology, and consistency with the
document's prose contract and with whatever precedent that contract names as the
one to be uniform with. **It does not restructure**: a structural finding is sent
back as a `draft` re-run leaf, never fixed here.

`copy-edit` owns the wording of headings, openings, summaries, example labels
and all body prose except normative sentences. It owns the content of no text.

<!-- tc: depart synonym-rotation because the conformance inventory pins this rule statement verbatim -->
- **Missing teaching goes to `draft`.** An untaught relationship or missing
  model is sent back as a `draft` correction run, never repaired in sentences.
- **Normative wording stays with `draft`.**
  <!-- tc: depart semicolon because the conformance inventory pins this rule statement verbatim -->
  Reword teaching text only; a defect in a normative sentence's wording or
  force goes back as a `draft` correction run rather than being repaired here.
  `proof` still repairs an error in a normative sentence itself.
- **Layout above the paragraph goes to `draft`.** Split a paragraph that
  carries two ideas. Remove bold that the prose contract does not admit. A
  heading level, list or table that the layout convention calls for goes back
  as a `draft` correction run. Adding or removing one regroups the text.
- **Figure-text defects go forward to `art`.** `copy-edit` runs before `art`.
  Record a defect in an existing figure's text in the node brief's
  `## Handed forward` list for `art`.

**The rules in `references/execute.md` under *Teaching the reader* bind this
sentence work.** Read them before you edit.

## What it reads

The document as the draft left it, the project's glossary, and the precedent
the contract names.

<!-- tc: depart long-sentence because the conformance inventory pins this rule statement verbatim -->
Read the set's prose contract as the contract format resolves it (the house
style when the set declares none), the term risks and the declared expertise.
The declared expertise says what not to teach.

## The operative tests

Each one is a defect the measured stage actually closed. The tests cover
headings, first sentences, summaries including table summaries, and example
labels, not only body text.

- A term spelled one way throughout rather than two.
- A term the glossary owns linked to it rather than silently re-defined beside
  it.
- A figure of speech replaced by the mechanism it stood for.
- A claim the reader cannot check replaced by one they can.
- Where the contract fixes a set of questions a passage must answer, **all** of
  them answered rather than one.

**Last act:** `leaf-add` the `art` leaf, under the family file's rule — which is
conditional, and which a correction run discharges for you.
