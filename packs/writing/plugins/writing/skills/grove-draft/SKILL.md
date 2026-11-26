---
name: grove-draft
description: The `draft` session kind — produce a document from its sources and its human-authored structure brief, owning structure and technical truth, to a green final validation. Use when a grove mandate names this skill, or when running a `draft` session in a grove working tree.
harnesses: [any]
---

# draft

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
procedures — is the spine's. What follows is `draft`'s, and is stated nowhere
else.

**draft** (AFK) — produce the document from its sources and its structure brief,
to a green final validation. The pipeline's first stage: there is no document
without it and no alternative to it.

## A structure brief is a precondition, not an input you can proceed without

Audience, conceptual order and what deserves emphasis are nowhere in the
sources. **If this document has no structure brief, stop and say so** — do not
draft from the sources alone. This is not tidiness: two editorial stages were
folded into this one on the evidence of a document drafted from such a brief,
and without the brief the fold has no basis under it.

**How to tell whether you have one — three things you can cite, from a named
artifact.** The document declares its structure brief the way it declares the
rest of its contract: **this leaf's body or the document node's brief names it by
path**, and that artifact itself states

- **who the reader is** and what they should be able to do afterwards,
- **the ordered section plan** — the sequence, and what each section is for,
- **what deserves emphasis**, and what the document does not cover.

<!-- tc: depart long-sentence because the conformance inventory pins this rule statement verbatim -->
**The three citable items carry expertise per domain and representation, the
model-first plan with page purposes and entry routes, and the intended
domain.**

The reader item names the reader's expertise in each domain and
representation. It also names what needs no teaching. The plan puts the
model before the notation and gives each page its purpose. It also names the
learning route and the direct entries. The emphasis item also names the
intended domain. The other fields of the brief are optional, and an absent
field takes its default.

All three, in the artifact, quotable. **The existence of a node file is not the
test.** `leaf-decompose` gives every node one automatically and bootstrap reads
it, so an existence check answers yes for every draft — including exactly the
no-human-structure case that must stop. If the only candidate is the node's own
brief and it does not state all three, you do not have a structure brief: stop,
and name which of the three is missing. Do not infer one from a filename or from
a neighbouring leaf's slug; those are one repository's convention and this kind
installs everywhere.

## What `draft` reads from the contract

The document's contract follows `DOCUMENT-CONTRACT-FORMAT.md`. That file
resolves each part and the default of each absent part. `draft` reads these
parts:

- the whole structure brief,
- the format and media part,
- the keyword, example-label, normative-aid and layout conventions of the
  prose contract.

`draft` owns the content of the structure, the teaching and the examples. It
owns the subject of each heading, opening and summary. It owns the example
labels, the normative text, the sections, the lists and the tables. It owns
the wording of each normative sentence.

**The section plan outranks a layout convention.** The human decides the
sections, so the headings that the plan states stand. They stand even where
a layout convention would not add them. A layout convention governs what
`draft` adds within the plan: further heading levels, lists and tables.

**The rules in `references/execute.md` under *Teaching the reader* bind this
drafting.** Read them before you write.

## Entries, examples and reference entries

<!-- tc: depart long-sentence because the conformance inventory pins this rule statement verbatim -->
- **Each declared entry carries the local context and example it needs, and
  teaching support at a normative entry stays distinguishable from the
  normative text.** A reader who arrives at a rule, an anchor, an example or a
  figure can use it without reading the pages before it.
<!-- tc: depart long-sentence,semicolon because the conformance inventory pins this rule statement verbatim -->
- **Label each example complete or fragment and say what a fragment omits;
  keep canonical examples complete; place a local excerpt where each construct
  is first met.**
<!-- tc: depart long-sentence because the conformance inventory pins this rule statement verbatim -->
- **Reference entries share fields shaped by the thing described and carry
  identifiers that survive heading edits.** Entries of one kind have the same
  fields.

## Two folded charters, owned explicitly

Because structure and technical truth are not separate stages, the draft owns
both, and owns them as obligations to discharge rather than as things the brief
has already handled.

- **Structure** — conceptual order, what each section is for, and whether the
  document delivers the reader outcome its brief asked for. Structure, not
  sentences.
- **Technical truth** — whether what the document says about its subject is
  true, complete for its scope, and not silently stale against the sources it is
  written from. The operative test is that **every claim about the subject is
  checked against the subject**, not against what you remember of it or what an
  earlier page of the same document asserted.

A drafting session that treats these as discharged by the brief has left two
stages of the pipeline undone. Whether they survive the fold was never measured,
so nothing licenses assuming they do.

**Last act:** `leaf-add` the `copy-edit` leaf, under the family file's rule —
which is conditional, and which a correction run discharges for you.
