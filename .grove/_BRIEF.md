# Ravel.simplify-and-componentize — brief

## Goal

Make Ravel easier to understand and maintain by reassessing its component
boundaries using the learnings from the two named Grove workstreams. Extract
useful independently reusable capabilities where the analysis justifies them,
and provide high-quality documentation of the components, complete tool,
configuration and project.

## Done when

- The learnings from both upstream workstreams have been absorbed before the
  component breakdown is analysed. The adopted guidance and its evidence limits
  are identified, rather than treating an unfinished draft as a final result.
- The resulting design explains each component's responsibility, interface,
  dependencies and composition, including the reasons for language and package
  boundaries. It improves the ability to reason about and change components
  separately while retaining Ravel's required behaviour.
- Useful generic components selected by the analysis can be used independently
  by other projects through documented interfaces. Their packaging is chosen
  on the evidence; no fixed crate count or repository arrangement is prescribed.
- Existing owner configuration continues to work unchanged, including
  TypeScript policies, SDK usage, settings, discovery and behaviour. CLI and
  session semantics remain compatible.
- Documentation supports operating Ravel, authoring configuration, understanding
  the architecture, following the code and using extracted components. Project
  development and delivery instructions describe the resulting project.
- The agreed acceptance checks establish compatibility, component contracts,
  complete-session behaviour and practical usability of the documentation.

## Decomposition

`plan-k1` established the requirements, compatibility constraints and agreed
acceptance seams. `upstream-guidance-k2` absorbs the final upstream guidance
and shapes the next design increment. The concrete design,
implementation and documentation leaves will be cut when their inputs are ready.
Do not prescribe a target architecture or generate a detailed implementation
tree before the upstream learnings have been absorbed.

Documentation structure and editorial work follow the resulting component
contracts. Apply the upstream editorial guidance to both component documentation
and the complete tool/configuration/project documentation, with reader outcomes
and human structure decisions made before drafting.

## Pointers

- Existing behavioural contract: `docs/specs/session-execution.md` and
  `docs/session-cli.md`.
- Existing architectural decisions: `docs/adr/policy-evaluation-precedes-the-launch.md`,
  `docs/adr/the-launched-child-is-a-job.md` and
  `docs/adr/ravel-supervises-the-session.md`. These explain current obligations;
  their internal design choices can be reassessed within the agreed constraints.
- Configuration surface: `docs/owner-configuration.md`.
- Generic runner boundary: `docs/local-runner.md`.
- Upstream decomposition work:
  `/Users/antony/Development/grove.improve-initial-design-decomposition-skill`.
  Its first authored method is under
  `plugins/linkuistics/skills/decomposition-design/`; final trials, revisions
  and integration must be checked before relying on it.
- Upstream documentation work:
  `/Users/antony/Development/grove.gh-issue-14-documentation-skills-for-editorial-pipeline`.
  Consume its resulting guidance, evidence and declared limits, including
  requirements/structure planning, draft, copy-edit, art, proof and reader testing.
- Reusable development work uses the existing root `Taskfile.yml`.

## Agreed acceptance checks

Use the existing public CLI with real TypeScript policy evaluation and fake
launched programs to check unchanged configuration, routing, refusals and
session results. Keep real model services out of these regression checks.

Use the existing real-process and terminal-test boundaries for cancellation,
descendant cleanup, reaping and terminal recovery where the redesign touches
those responsibilities. Verify that complete Ravel sessions still satisfy their
contract after components are recomposed.

Exercise each extracted component through its consumer-facing contract, with
controlled dependencies only where an existing or justified seam is needed.
Show that a consumer can use it without depending on Ravel-specific policy or
session conventions unless those are genuinely part of its contract.

Validate documentation examples against the resulting interfaces and arrange a
short human read using practical tasks: configure and run a session; explain the
components and follow one session through the code; use an extracted component.
Determine the structure, admitted media and viewing environment using the
upstream guidance. Mechanical document checks support this read rather than
establishing comprehension by themselves.

The human confirmed these acceptance checks and shared understanding of the
requirements in `plan-k1` before the design and implementation work was cut.

## Notes

The primary problem is comprehensibility, not a shortage of features. TypeScript
removal, merging the Rust crates, additional crate splitting and extraction are
design possibilities to evaluate against the contract; none is preselected.
Do not expand the feature surface merely to justify a generic abstraction.

The human expects Rust to be capable of implementing the tool and needs the
purpose and role of the TypeScript component explained. Documentation should
expose the subject model and reasons for boundaries before asking readers to understand
language-specific internals. Precise reader profiles and structure are part of
the later documentation agreement.

If either upstream outcome is not ready to absorb, keep `upstream-guidance-k2`
live and stop rather than substituting assumptions or repeatedly polling. The
work remains resumable after the prerequisites become available.
