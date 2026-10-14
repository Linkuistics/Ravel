# upstream-guidance-k2

## Goal

Absorb the final learnings from the two upstream Grove workstreams, apply them
to the agreed Ravel requirements, and cut the first concrete design increment.
This intake precedes analysis of Ravel's component breakdown.

## Context

`plan-k1` records the human's decisions; the root brief carries the requirements
and agreed acceptance seams. Its pointers identify both upstream workstreams.

Both workstreams were unfinished during `plan-k1`. Establish their final
outcomes at the time of this task. Read the final decomposition method,
existing-system guidance, catalogue, trial findings and revisions, and the final
editorial guidance and evaluation outcome. Locate retained outputs if a finished
workstream's temporary `.grove/` tree has been removed. Pin the source revisions
and relevant guidance versions actually consumed, including any unmet or
inconclusive evaluation findings.

The presence of a draft skill does not establish that its workstream is finished.
If either upstream workstream is unfinished or its final outcome cannot be
established, report that specific prerequisite, leave this leaf live and end
with the launch prompt's mandated force-stop signal. Do not retire, analyse the
component breakdown, substitute existing assumptions or repeatedly poll. Resume
after the inputs become available.

## Done when

- Both upstream outcomes have been absorbed, with source provenance and relevant
  limits recorded in the brief that will guide the design.
- The adopted decomposition method is connected to Ravel's questions about
  responsibilities, language/runtime boundaries, entanglement, state/effects,
  interfaces, generic capabilities, independent use and composition costs.
- The documentation guidance is connected to the tool, configuration, component
  and project documentation. Carry forward the agreed reader tasks and checks,
  and identify the later human structure agreement before editorial drafting.
- The smallest useful next design increment has been cut with a concrete goal,
  inputs and completion conditions. Keep implementation and editorial leaves
  lazy until their design and structure inputs are settled.

## Notes

Use the final installed skills where available and confirm they incorporate the
named workstreams. Otherwise read and follow the verified final source guidance;
do not silently replace it with older guidance of a similar name.

Consume the upstream outcomes for this project; do not duplicate their surveys,
trials or skill authoring. Keep process-specific adoption notes in the appropriate
brief. Durable Ravel architecture and behavioural contracts belong in the normal
repository artifacts once established.

The initial component explanation in `plan-k1` is orientation, not a selected
architecture. Choose boundaries by their contracts and reasoning burden,
accounting for composition and migration as well as local simplicity, while
preserving the root brief's configuration and session constraints.
