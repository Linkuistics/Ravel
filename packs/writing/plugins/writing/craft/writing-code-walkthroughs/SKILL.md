---
name: writing-code-walkthroughs
description: Concept-ordered, source-exact code walkthrough authoring for Markdown books and subsystem guides. Use when creating or revising a walkthrough of an existing codebase for a defined audience, especially when a bounded source corpus must be covered completely or reproduced exactly from explained fragments.
harnesses: [any]
---

# Writing code walkthroughs

A code walkthrough is a checked projection of an authoritative codebase into a
reader order. Its explanation follows conceptual dependencies; its source
inventory, fragments, and verification preserve exactness independently of that
order.

The method scales down. For an orientation, retain the intake, evidence,
ordering, prose, and review contracts and omit exact-source fragments. For a
complete-source walkthrough, every production byte belongs to the fragment
graph and final verification proves reconstruction.

> **Provenance.** This method combines program-comprehension, instructional,
> literate-programming, and developer-documentation evidence. The adopted
> sources and limits are recorded in
> [`../../PROVENANCE.md`](../../PROVENANCE.md#writing-code-walkthroughs).

## Establish the authoring contract

Complete the intake before inspecting the target. Ask exactly one question per
turn and record its answer before continuing. A broad answer does not discharge
a narrower field; ask the follow-up that freezes each field below.

1. **Target:** Which repository, package, executable, or bounded subsystem is
   the walkthrough about? Which subsystem boundaries are in scope?
2. **Authoritative source:** Which manifests and production files are included?
   Classify tests, fixtures, models, generated files, examples, and dependencies
   separately as included source, evidence, or excluded material.
3. **Corpus stability:** May those bytes change during authoring? If they may,
   which revision, snapshot, or other artifact is authoritative, and how is a
   change detected?
4. **Audience:** Establish language proficiency, systems and tooling
   proficiency, and familiarity with the target domain separately.
5. **Depth:** Is the deliverable an orientation, a subsystem explanation,
   complete production-source coverage, or another explicit level?
6. **Output and walk-away behavior:** What files and Markdown structure are
   required? What source and prose must remain usable if walkthrough-specific
   tooling disappears?
7. **Style:** Freeze terminology, prose, citation, heading, navigation,
   repetition, and cross-reference requirements.
8. **Assurance:** What must deterministic checks prove? Which claims require an
   independent technical review and which require an independent editorial
   review?

Do not ask the user to design the chapter sequence before the evidence is known.
Freeze the answers as an authoring contract containing the eight fields, the
source-change policy, and the acceptance checks. Treat later scope changes as
contract changes, not informal additions.

## Inventory evidence before choosing reader order

Create an exact inventory of the authoritative manifests and production files.
Record repository-relative paths and the revision or digest that freezes their
bytes. Keep evidence-only and excluded material explicit so tests, models, and
examples cannot silently become claimed source coverage.

Use the strongest available evidence for each technical claim:

- production source and manifests for implemented behavior and dependency
  versions;
- tests and formal models for exercised cases and stated invariants;
- repository architecture, specifications, and decision records for intent;
- official dependency documentation for version-dependent framework behavior;
- external research only for writing and communication decisions.

Build a claim ledger when the walkthrough is large: claim or invariant,
authoritative evidence, owning section, and verification method. Mark an
unverified claim as `UNVERIFIED`; never turn an inference into a source fact.

Extract the codebase's purpose, vocabulary, public seam, representative
operations, invariants, effects, concurrency boundaries, rollback behavior,
refusals, environmental failures, and defects. This inventory supplies the
concept graph and the review briefs.

## Order concepts, then map source into that order

Build a dependency graph over the extracted concepts. Choose a real operation
that crosses the important boundaries and can serve as a low-resolution whole.
Order the walkthrough whole-to-path-to-parts:

1. purpose, ownership boundary, and observable result;
2. only the vocabulary and public types needed to name the operation;
3. the complete operation at low resolution, with every major stage present;
4. the public seam and read path;
5. mutation decisions, plans, effects, concurrency, rollback, and reports along
   causal dependencies;
6. interpreters, adapters, reference domains, and CLIs after the core model is
   stable; and
7. cross-cutting invariants and trade-offs, returning to the opening operation
   at full resolution.

The exact sequence adapts to the target. The invariant is that conceptual and
causal dependencies determine reader order; directories, modules, and source
line order do not. Whole-then-elaborate ordering is supported by instructional
and program-comprehension evidence, while the exact chapter sequence remains a
contextual design choice ([Reigeluth and Stein](https://ocw.metu.edu.tr/pluginfile.php/9337/mod_resource/content/1/Reigeluth%201983%20Article.pdf),
[Pennington](https://www.sciencedirect.com/science/article/pii/0010028587900077),
[Letovsky and Soloway](https://www.cs.kent.edu/~jmaletic/cs69995-PC/papers/letovsky-1986-software.pdf)).

For each stage of a worked operation, state:

- the production actor: type, function, process, or command;
- its concrete input and output;
- the invariant it uses, preserves, or establishes;
- why the next layer cannot infer that fact itself; and
- the adjacent refusal or failure fork when it changes the stage's meaning.

End with observable state and the returned result. Add another fully worked
operation only when it introduces a materially different invariant, boundary,
rollback shape, or refusal class. This worked-example and self-explanation
guidance transfers evidence from instructional settings rather than claiming a
controlled reader outcome for code walkthroughs
([Sweller and Cooper](https://doi.org/10.1207/s1532690xci0201_3),
[Chi et al.](https://doi.org/10.1207/s15516709cog1302_1)).

## Own exact source with fragments

Use this contract when the requested depth includes complete or reproducible
source coverage. The production files remain authoritative; the walkthrough is
never a generator that overwrites them.

- Associate every in-scope file with exactly one root fragment.
- Give every fragment one globally unique, intent-expressive identifier and one
  definition.
- Store literal source bytes plus explicit child references at exact insertion
  points. A reference contributes no implicit indentation or transformation.
- Make expansion pure recursive concatenation. Reference edges determine source
  order; Markdown definition order determines reader order only.
- Give planned references syntax distinct from resolved references and name the
  slice that owns them. Permit no planned reference in the final artifact.
- Keep every insertion relationship legible in raw Markdown without a renderer.

Named recursive expansion separates explanation order from source order in
CWEB and noweb; it does not choose a good reader order by itself
([CWEB manual](https://tug.ctan.org/web/cweb/cwebman.pdf),
[Ramsey 1994](https://www.cs.tufts.edu/~nr/pubs/lpsimp.pdf)). The one-root,
single-definition, absolute-byte, and compare-with-source rules are this
method's deterministic exactness contract.

Maintain a fragment ledger containing, at minimum:

| Field | Meaning |
|---|---|
| fragment | unique identifier |
| definition | Markdown file and anchor |
| source ownership | repository-relative file and byte or line range |
| parents | every insertion site and insertion order |
| children | every referenced fragment |
| slice owner | authoring increment responsible for resolving it |
| invariant roles | cross-cutting claims in which it participates |

Introduce the notation before its first nontrivial use with one small root,
one child inserted elsewhere in reader order, its recursive expansion, and its
byte comparison with the real file.

## Leave every increment coherent

Before drafting a slice, declare the concepts, fragments, source ranges, links,
and deferred references it owns. After drafting it:

1. expand every root that is expected to be complete at this point;
2. validate resolved fragment identity, edges, reachability, and owned coverage;
3. compare every non-deferred byte against the authoritative source;
4. validate the slice's headings, anchors, links, and navigation; and
5. update the claim and fragment ledgers.

Deferred references keep incomplete work explicit; they do not justify a final
coverage claim. A source change should create a localized equality failure, not
silent regeneration.

## Write self-contained technical prose

Lead sections with direct claims. Use stable codebase vocabulary. Name the actor
for every effect. Explain why a boundary exists, not merely which call follows
which.
Distinguish a deliberate refusal from an environmental failure and an
implementation defect. Calibrate detail to the audience contract: explain the
unfamiliar domain and surprising language or operating-system behavior, while
omitting mechanics the reader already knows.

For every source-fragment introduction, answer five questions in prose: why the
fragment appears here, which actor owns the behavior, what input becomes what
output, which invariant it uses or establishes, and what role it has in the
current example.

Use direct, concise introductions and descriptive headings. These are maintained
developer-documentation conventions, not a claim that one prose style guarantees
comprehension ([RFC 7322](https://www.rfc-editor.org/rfc/rfc7322.html),
[Google developer documentation style](https://developers.google.com/style/highlights),
[Google heading guidance](https://developers.google.com/style/headings)).

### Decide between local repetition and a link

Apply these tests at each dependency boundary:

1. **Audience:** Omit ordinary knowledge granted by the audience contract.
2. **Interaction:** If a codebase-specific fact must be held with the current
   passage to understand one causal claim, continue; otherwise link only for
   useful optional depth.
3. **Independence:** If both passages are independently intelligible, remove a
   restatement that adds no local relation, condition, or consequence.
4. **Local form:** Repeat the shortest exact semantic statement that makes the
   current claim intelligible. For substantial material, state its current
   implication locally and link to the authoritative treatment.
5. **Artifact:** Keep production fragments, exhaustive variant lists, syntax
   contracts, and detailed procedures single-owned. Repeat their current
   meaning, not the artifact.
6. **Goal:** At every site in a delocalized plan, state the shared goal when
   local clues reveal only that site's role.

Links name their destination and purpose and carry optional lookup or depth.
The current causal claim must not require a chain of links. This rule combines
split-attention boundaries with documented lookup costs
([Ayres and Sweller](https://www.davidlewisphd.com/courses/EDD8121/readings/2006-AyersSweller.pdf),
[Letovsky and Soloway](https://www.cs.kent.edu/~jmaletic/cs69995-PC/papers/letovsky-1986-software.pdf),
[WCAG link purpose](https://www.w3.org/WAI/WCAG22/Understanding/link-purpose-in-context.html)).

## Verify before claiming completeness

Use deterministic tooling for properties a machine can decide. For an
exact-source walkthrough, the final validator rejects:

- missing or duplicate file roots and fragment definitions;
- unresolved references, cycles, any production fragment unreachable from its
  intended file root, and reachability from any unintended root;
- missing, duplicated, overlapping, or unintended source coverage;
- any byte difference between an expanded root and its authoritative file;
- fragment relationships absent from the ledger; and
- invalid Markdown structure, duplicate anchors, broken local links, and
  unreachable pages.

Diagnostics identify the source file, first differing byte or line, owning
fragment, and relevant reference path. Run the target repository's existing
format, build, lint, test, and model checks as separate evidence; compilation or
copied-snippet presence is not proof of source equality.

Then commission two independent judgment passes, using separate review briefs:

- **Technical review:** verify claims against authoritative source, tests,
  models, and versioned dependency documentation; check API contracts,
  invariants, control and data flow, effects, concurrency, rollback, errors, and
  refusals.
- **Editorial review:** check concept dependencies, local completeness, worked
  examples, stable terminology, ambiguity, unexplained fragments, repetition,
  link purpose, fragmentation, and bloat.

Use fresh reviewers or review contexts when the environment permits. Keep the
roles distinct even when only review briefs can be handed off. Finish with a
walk-away check: without custom walkthrough tooling, the authoritative source
and full Markdown explanation remain readable and all insertion relationships
remain visible; only automated proof is lost.

Report exact checks and observed results. Claim complete source coverage only
when inventory, reachability, coverage, and byte equality all pass. Claim
technical or editorial quality only to the extent established by the
corresponding independent review.
