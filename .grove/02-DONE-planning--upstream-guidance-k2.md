# upstream-guidance-k2

## Goal

Absorb usable current learnings from the two upstream Grove workstreams, apply them
to the agreed Ravel requirements, and cut the first concrete design increment.
This intake precedes analysis of Ravel's component breakdown.

## Context

`plan-k1` records the human's decisions; the root brief carries the requirements
and agreed acceptance seams. Its pointers identify both upstream workstreams.

Both workstreams were unfinished during `plan-k1`. Establish their current
outcomes at the time of this task. Read the current decomposition method,
existing-system guidance, catalogue, trial findings and revisions, and current
editorial guidance and evaluation outcomes where available. Locate retained
outputs if a finished workstream's temporary `.grove/` tree has been removed.
Pin the source revisions
and relevant guidance versions actually consumed, including any unmet or
inconclusive evaluation findings.

The presence of a draft skill does not establish that its workstream is finished.
The human has authorised intake from unfinished workstreams. Pin the current
guidance actually used and carry unfinished evaluation and revision forward as
limits. Completion of the upstream workstreams is no longer a prerequisite.

## Done when

- Both current upstream outcomes have been absorbed, with source provenance
  and relevant limits recorded in the brief that will guide the design.
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

Use the current installed skills where available and confirm they incorporate the
named workstreams. Otherwise read and follow the pinned current source guidance;
do not silently replace it with older guidance of a similar name.

Consume the upstream outcomes for this project; do not duplicate their surveys,
trials or skill authoring. Keep process-specific adoption notes in the appropriate
brief. Durable Ravel architecture and behavioural contracts belong in the normal
repository artifacts once established.

The initial component explanation in `plan-k1` is orientation, not a selected
architecture. Choose boundaries by their contracts and reasoning burden,
accounting for composition and migration as well as local simplicity, while
preserving the root brief's configuration and session constraints.

## Decisions (running log)

On 2026-10-14 the human authorised extracting usable guidance from the two
workstreams' current state instead of waiting for completion. This supersedes
the final-outcome prerequisite agreed in `plan-k1`. Intake will pin the source
revisions and distinguish authored guidance from uncompleted evaluation; the
configuration, session and documentation acceptance constraints still apply.

The usable intake is recorded in the root brief with source commit identities,
the catalogue navigation identity, the installed editorial/source comparison,
live upstream evaluation/revision owners and the limits of the historical
reports. The current decomposition method is candidate, untested synthesis;
the editorial additions are an agreed source design rather than completed
installed delivery. No upstream effectiveness or comprehension claim is made.

The next increment is `component-contracts-k3`: a bounded current-contract and
design-frame investigation that delivers a useful component explanation before
selecting a target. Implementation and editorial work remain lazy; the
continuation must carry the later human structure agreement and reader tasks.
No review of this intake is cut: it settles source standing and a single
discovery handoff, and chooses no architecture or long-lived implementation
decomposition. The design producer decides review for its resulting artifact.
