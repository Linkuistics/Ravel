# WritingSkills provenance

The skills use independent prose, but several disciplines and examples were
adapted from public prior art. This file keeps the claim-level source keys used
inside `SKILL.md` files without retaining the large point-in-time survey that
originally discovered them.

The sections below come from `plugins/linkuistics/PROVENANCE.md` in Grove, at
Grove commit `31ec9117c9dacfe993e80313addd432f2930d679`. They cover the two
craft skills of this pack. The four kind skills and `editorial-family` are
Grove's own text, and the repository's README names their source. In the
section of `technical-communication`, the paths of the checker and of its
licence now start with `craft/`, the root of the skill in this pack. In both
sections, "this repository" means Grove, where the skills were written.

## Individual skills

### `writing-code-walkthroughs`

The method was synthesized in this repository from two independent surveys and
then exercised by the complete, technically reviewed, and editorially reviewed
`ordinal-fs-tree` book. Its external foundations are primary or authoritative
sources: Reigeluth and Stein, Pennington, and Letovsky and Soloway for
whole-to-parts concept ordering; [Sweller and Cooper](https://doi.org/10.1207/s1532690xci0201_3)
plus [Chi et al.](https://doi.org/10.1207/s15516709cog1302_1) for worked
examples and self-explanation; Ayres and Sweller plus Letovsky and Soloway for
local integration and delocalized-plan goals; CWEB and Ramsey's noweb paper for
named recursive source expansion; RFC 7322, Google's developer-documentation
guide, and WCAG for direct prose, headings, links, and navigation. The exact
fragment grammar, byte-comparison contract, scoped-progress protocol, and
separate technical/editorial review briefs are Linkuistics' project synthesis;
the skill does not claim controlled reader-outcome evidence for them. The
[behavioral evaluation campaign](../../docs/evaluations/writing-code-walkthroughs/README.md)
did not establish its predeclared acceptance rubric; the report distinguishes
the retained judged outputs from the stronger deterministic checks.
The proposed replacement campaign did not run: the behavioral-acceptance
conjunct is closed as unmet after no inspected hosted provider documented the
required final-boundary attestation and controlled self-hosting was declined.
The current decision and its reopening boundary are recorded in
[`behavioral-acceptance-is-closed-unmet`](../../docs/adr/behavioral-acceptance-is-closed-unmet.md).

### `technical-communication`

The skill is modelled on ASD-STE100, Issue 9. Its
rules are restated in our own words from the standard's Part 1. The standard is
licensed to the maintainer and may not be redistributed, so nothing in this
repository quotes it: no sentence, no rule title, no example and no dictionary.
Every example is invented, about software and model work. The skill names the
standard only to say what it is modelled on, and it does not claim to conform
to it. The decision is
[technical-communication carries no text from the standard](../../docs/adr/technical-communication-carries-no-standard-text.md).

The checker, `craft/technical-communication/scripts/ste-lint.py`, is vendored
code.

| Item | Detail |
|---|---|
| Upstream | [`danyuchn/asd-ste100-skill`](https://github.com/danyuchn/asd-ste100-skill), file `scripts/ste-lint.py` |
| Commit | `3944283c183eea13fe5bd902b572da3b4de8bf44` |
| Licence | MIT, copyright (c) 2026 Dustin Yuchen Teng. The notice is `craft/technical-communication/LICENSES/danyuchn-asd-ste100-skill.LICENSE`, copied unchanged from upstream at that commit |
| Entered | Unchanged in `vendored-checker-k3`, apart from an attribution comment that names the commit |

What we changed, after the checker came in unchanged. Every changed or added
hunk in the script carries a comment that starts with `ours:`, and code without
one is as upstream wrote it. The spec section *The checker's contract* in
`docs/specs/technical-communication.md` states the result.

- **Whole-document checks.** Rules for what counts as prose in Markdown, front
  matter, commit messages and plain text; British spelling by suffix patterns,
  with no dictionary; and the `tc:` markers that record a departure.
- **Reply scoring.** The `--log-start` and `--log-reply` modes, which append
  counts to the reply log and store no reply text.

Only the checker came from upstream. The upstream skill text, its word lists
and its rule wording were not adopted. The hook core (`scripts/tc_hook.py` and
`scripts/tc-hook`), the harness entries, the skill text and its reference files
are original to this repository. The survey of what each harness allows is
`docs/research/harness-hooks-a.md`.
