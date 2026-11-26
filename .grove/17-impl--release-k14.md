# release-k14

## Goal

Release the work of this grove, as `pack-shipping` settled, with migration
notes for installed users. Record how the outcome relates to issues 4, 5 and
7.

## Context

- The running log of `pack-shipping-k12`.
- The decision record `a-caller-transition-publishes-a-versioned-formula-first`.
- The existing Taskfile tasks under `release:`.

## Done when

- Each release that this work causes is published, and each ships migration
  notes for installed users.
- The packaging and the third-party notices match the delivered contract.
- The specs, the decision records and the examples match the delivered
  contract.
- A record states how the outcome relates to Linkuistics/Ravel#4, #5 and #7.
  It names what #4 still holds open: hooks on generic harnesses and the
  executable plugins for interface features.
- Each issue whose acceptance criteria are met is closed, and no other.

## Notes

- A push of the Ravel repository publishes the merged histories. First, read
  the owner's publication decision in `merge-terms-k15`.
- A release is outward-facing and hard to reverse. Ask the owner to approve
  each publication step.
