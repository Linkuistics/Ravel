---
name: grove-requirements
description: The `requirements` session kind — establish what should be built, in the human's own words, and decide whether the open questions meet the threshold for a full grilling. Use when a grove mandate names this skill, or when running a `requirements` session in a grove working tree.
harnesses: [any]
---

<!-- adapted (paraphrased into grove's voice, not bundled verbatim) from
     mattpocock/skills@d574778f94cf620fcc8ce741584093bc650a61d3
     (skills/engineering/wayfinder/SKILL.md, "Chart the map" step 2's no-fog
     early exit) — MIT licensed; see LICENSES/mattpocock-skills.LICENSE. -->

# requirements

**Load the `grove` skill now.** It is the shared spine and holds everything
this kind does not own: the constraints, the bootstrap, and the execute,
decompose, retire and commit procedures. `grove` is the bare skill name. If your
harness requires a plugin namespace, use the namespace under which `grove` is
actually installed. Do not assume one. If you cannot find or read the `grove`
skill, follow the launch prompt's rule for a missing kind skill. What follows is
`requirements`'s, and is stated nowhere else.

**requirements** (HITL) — establish *what* should be built, in the human's own
words. That is unconditional; the interview below is not.

## Whether the journey needs a map

If this session surfaces no real fog — the path to done is already clear and the
whole thing fits one sitting — say so and do the work directly, rather than
wrapping it in a tree, a brief chain and a relaunch cycle it does not need. Bare
`grove` scaffolds a tree and mints a first leaf in any rootless working tree;
that mechanical fact is not the signal to keep one. Session-to-session fog —
work whose shape you cannot see past the next session or two — is.

## The threshold for a full grilling

The **full one-question-at-a-time procedure**, which `grilling.md` carries, runs
**only when three or more of this leaf's open questions have interdependent
answers**. Below that threshold, record the decisions and proceed.

Staging an interrogation over settled ground costs a human's attention and
returns nothing, so a pre-decided question is not a grilling question: if you
already know the answer and only want it executed, record it and move on. If the
brief's questions are *all* pre-decided, say so — the work standing behind them
is an `impl` leaf.

When the threshold is met, `grilling.md` is the procedure, and one move makes that
interview productive rather than ceremonial. **Never put two questions in one
prompt**, even closely related ones: the answers conflate. Where two truly
interdepend, ask the *foundational* one first, wait, and carry its answer into
the *derived* one, which is only meaningful once the foundation is settled.

## Agree the test seams while the human is here

When the increment covers code that will be tested, sketch the seams the work
will be tested through and put them to the human *before* the design is
committed: "these are the seams — do they match what you expected?" That check
is a grilling move, and this is the session with a human in the loop to make it.
`SPEC-FORMAT.md` says where the agreement is recorded.

## A leaf that holds a reader checkpoint

**When this leaf holds a reader checkpoint, `reader-test.md` is the
procedure.** A document contract declares the checkpoint. The session that
cut the document's leaf placed this leaf after the document's `proof`. In a
leaf without a reader checkpoint, do not read that file.

## The fresh grove's bootstrap leaf

A brand-new grove's first leaf is a `requirements` leaf the driver minted before
any agent existed, and its only input is the human's own words — nothing else is
on disk yet. A **small** workstream's bootstrap session may resolve
requirements, design and planning in that one leaf and cut the leaves itself; a
larger one records what it settled and adds a `planning` leaf for a fresh
session to decompose (fresh-grove-start-contract).
