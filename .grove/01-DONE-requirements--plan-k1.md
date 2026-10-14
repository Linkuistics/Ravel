# plan-k1

## Goal

Establish requirements for making Ravel's code easier to understand, reassessing
its component and language boundaries, extracting useful generic capabilities,
and documenting the resulting components and complete tool.

## Context

The human identifies three current components, questions the TypeScript
component and the two Rust crates, and wants the work to draw on these ongoing
Grove workstreams:

- `/Users/antony/Development/grove.improve-initial-design-decomposition-skill`
- `/Users/antony/Development/grove.gh-issue-14-documentation-skills-for-editorial-pipeline`

## Done when

The intended outcomes, compatibility constraints, extraction expectations,
upstream dependency conditions, documentation audiences and acceptance criteria
are settled sufficiently for a subsequent planning/design session. The human has
agreed the verification seams. Remaining design questions are stated explicitly rather
than answered by an implementation in this requirements session.

## Notes

The work spans architectural choices and documentation, so it requires more
than one sitting. Do not infer a Rust-only requirement or a target crate count
from the human's questions about the existing structure.

Initial investigation: TypeScript evaluates executable owner policies, rather
than merely parsing static configuration. The native front retains session
authority and reaps the compiled Bun worker before launching the selected
program. See `docs/adr/policy-evaluation-precedes-the-launch.md`,
`crates/ravel/worker/src/main.ts` (`load`) and
`crates/ravel/src/choice.rs` (`evaluate_and_resolve`). The separate
`keyed-launch` crate owns the generic supervised-job mechanics; see
`docs/local-runner.md` and `docs/adr/the-launched-child-is-a-job.md`.

The decomposition workstream has authored an initial `decomposition-design`
skill at `plugins/linkuistics/skills/decomposition-design/SKILL.md`, with an
existing-system path in its references. Its root brief describes subsequent
trials and revision. The editorial workstream's root brief requires
reader-specific structure, the draft/copy-edit/art/proof pipeline, and reader
testing outside those stages. These observations identify future adoption
points; they do not establish completion or installed readiness.

The first compatibility question is settled in the running log below.

The continuation is `upstream-guidance-k2`: a focused intake and planning task
that must establish and absorb both final upstream outcomes before analysing
the component breakdown. No architectural choice or implementation tree is cut
in this requirements session. No separate requirements review is added: the
human confirmed the scope and acceptance seams, and the substantive architecture
remains the work of the subsequent design rather than a claim made here.

## Decisions (running log)

The human's primary problem is comprehensibility: “This tool is very
comprehensive, but it is difficult to follow the code.” The work should make the
responsibilities and relationships of the components clear.

The human questions why TypeScript is needed and why there are two Rust crates.
These are questions for investigation, not settled instructions to remove
TypeScript or merge crates.

The human wants generic capabilities, including possible capabilities currently
inside the `ravel` Rust crate, considered for extraction. The target packaging
and consumers are not yet settled.

The human wants to draw on the ongoing initial-design/decomposition workstream
to decompose this project, and on the ongoing editorial-pipeline workstream for
high-quality documentation of the components, tool, configuration and project.
Neither dependency should be represented as finished without checking it.

The human confirms that configuration must not change. Preserve existing
TypeScript owner policies, SDK usage, settings, configuration discovery and
configuration behaviour. Retain the CLI/session behaviour as recommended;
component extraction and internal restructuring must work within those
compatibility constraints. This does not prescribe the implementation language
or number of Rust crates.

The human confirms that learnings from both named Grove workstreams must be
absorbed before analysing Ravel's component breakdown. Reading their current
drafts does not satisfy that prerequisite, and the existing component map is
orientation rather than a selected target architecture.

The human agrees that useful generic components should be independently usable
by other projects. The subsequent analysis decides which boundaries justify
extraction and then chooses repository/package/publishing arrangements. There
is no requirement to extract every generic-looking helper or to reach a fixed
component count.

The human confirms the proposed acceptance checks and shared understanding of
the requirements: unchanged configuration and sessions through the existing
CLI/real-policy/fake-program boundary; real process/terminal checks for
supervision and cleanup; consumer-facing contract checks for extracted
components; and validated examples plus a short human read for documentation.
The root brief records these agreed seams. Detailed documentation structure,
reader profiles, format and packaging remain downstream choices informed by
the two upstream workstreams.
