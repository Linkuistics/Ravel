# unification-k32

**Integrates:** unification-k31

## Goal

Triage the findings from `unification-k31` and apply the accepted repairs to
the planned subtree produced by `unification-k2`.

## Context

- Read the findings from the review's own commit, found by its stable handle.
  The review records its producer commit and uses that commit's coordinates.
- The root brief and the running log of `plan-k1` are the requirements. The
  producer task `unification-k2` and the `grove-planning` skill state the
  planning contract.

## Done when

- Each finding is verified against its source and classified as an unclear
  contract, a real issue, an accepted trade-off or noise, with the reason.
- Accepted repairs meet the agreed acceptance and material-risk threshold.
  The tree, node briefs and root decomposition agree after any repair.
- Every binding requirement retains an owner. Unfinished acceptance stays
  live, and any change to the owner's agreed scope has explicit confirmation.

## Notes

- This leaf owns triage, not an obligation to accept every finding. Do not
  copy the findings into this body's completion criteria.
- The review was inspection-only. It made no production or test changes and
  ran no test, build, lint or formatting commands.

## Decisions (running log)

**Source of the findings.** The findings come from commit `dad30819`, the
commit of `unification-k31`. Its line numbers refer to producer commit
`9e1a7edb`. That commit renumbered the positions only, so each cited line
still holds the cited text.

**F1, the Gemini trial blocks the walk: a real issue, repaired.** The leaf
`gemini-cli-adapter-k27` stayed live while the owner had no Gemini CLI
sign-in. The pre-order walk then stopped at it, before `pack-form-k28`, the
corpus review and every later root entry. None of that work uses the result
of the real trial.

The repair of F1 keeps the trial as live acceptance work, in the
new root leaf `gemini-cli-trial-k33`. That leaf sits before `book-k13` and
`release-k14`, which use its result. The adapter leaf keeps the fake-seam
work and the opt-in task. The node brief, `pack-form-k28` and the root
Decomposition now name the new leaf.

**The trial leaf sits at the root.** A last leaf inside `skill-delivery-k9`
would still block the corpus review node. So the leaf goes to the root, and
the node brief states that the root leaf owns the Gemini part of its trial
criterion. The kind is `impl`, because a failed trial can need an adapter
repair.

**F2, the real trials of Claude Code and Codex: an unclear contract,
repaired.** The two adapter bodies required a Taskfile task that runs the
trial. They did not require the session to run it and record the result. The
root brief requires the acceptance tests to run at all four seams, and the
glossary says a supported harness gets real trials. Each body now requires
the run and a recorded result. A trial that cannot run keeps the leaf live.

The owner's sign-ins for both harnesses already serve the tasks
`harness:smoke:claude` and `harness:smoke:codex`. So the risk that these
leaves block the walk is low, and this integration accepts it.

**F3, the DesignSkills conversion contradicts itself: a real issue,
repaired.** The design leaf and the node brief kept DesignSkills unconverted.
The merge leaf and `pack-form-k28` required its conversion. The running log of
`plan-k1` holds no decision to keep DesignSkills unconverted. The owner moved
only its language review to Linkuistics/Ravel#7.

The owner asked for one
skill form, so the repair keeps the conversion in `design-skills-merge-k11`.
The design leaf and the node brief now say that DesignSkills keeps its
current form until that leaf converts it. The other choice would drop an
obligation, and that needs the owner. This choice keeps every obligation, so
it changes no agreed scope.

**Acceptance after the repairs.** Every root done criterion keeps an owner.
The real trials now have explicit owners: the two adapters and
`gemini-cli-trial-k33`. No obligation moved out of the grove, and no scope
changed.
