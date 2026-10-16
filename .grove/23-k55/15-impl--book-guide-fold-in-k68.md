# book-guide-fold-in-k68

## Goal

Retire the two guides Parts I and II fully absorb, `docs/session-cli.md` and
`docs/owner-configuration.md`, with every inbound link redirected and the
owner's dated configuration handoff kept outside the book.

## Context

- `docs/book/contract.md`, "Relation to other documents" and "Exclusions": the
  guides are removed once the chapters that absorb them land, the handoff
  record is "kept, not deleted", and the book never depends on a link to a
  specification.
- Placement decided in planning: the handoff section ("Configuration handoff on
  2026-10-12") moves unchanged in substance to its own file,
  `docs/configuration-handoff-2026-10-12.md`, with a short heading that says
  what it is (a dated record of the owner's copy, not Ravel's registry or
  defaults) and a pointer from `docs/caller-references.md`. It is not in the
  book and not in `docs/evidence/` (which holds measured receipts).
- Inbound links, enumerated in planning from a tracked-file search that must be
  repeated with controls (do not rely on this list): `README.md`,
  `docs/components.md` (to `session-cli.md#inputs-and-exit-behavior` and
  `owner-configuration.md`), `docs/session-channel.md`,
  `docs/specs/session-execution.md`, `docs/native-entry-development-evidence.md`
  and `docs/caller-references.md`. Frozen evidence under `docs/evidence/` and
  the `*-checks.txt` logs record those files as digested subjects by name; they
  are receipts and are left alone, and the sweep classifies them as such.
- The fold-in's input is the proved chapters. For each guide, enumerate every
  sentence and classify it: absorbed in chapter N (name the anchor), a
  duplicate of a document that survives (the spec, the README's Develop
  section), moved to the handoff record, or contributor-facing with no home.
  A contributor sentence with no home moves into the README's Develop section
  (it stays until chapter 11) and is listed in the node brief's "Handed
  forward"; nothing is lost silently.
- `docs/authority.md` and the README's user-facing text are
  `book-readme-and-authority-k69`'s.

## Done when

- The classification table for both guides exists in this leaf's log, every
  sentence accounted for, and no row is "absorbed" without a chapter anchor
  that a reader can follow.
- `docs/session-cli.md` and `docs/owner-configuration.md` are removed;
  the handoff record exists at its new path; every inbound link in a surviving
  document names its new place (a chapter anchor, the handoff record, or the
  specification the sentence belonged to).
- The link sweep is enumerated, not pattern-listed: extract every Markdown
  link target in the tracked tree (excluding `docs/evidence/`, the checks
  logs, `retired/` and `.grove/`), resolve each, and classify; a positive
  control (a link that must resolve) and a mutated control (a deliberately
  broken link) are seen to fail and succeed as they should. `task docs:book`
  still builds.
- `task check` passes and `task test:native-incumbent-controls` is run, or its
  non-run is explained, since the incumbent instrument names documents.
  Release archives are unchanged in manifest (`task release:test`).
- `docs/book/contract.md`'s "Relation to other documents" is edited in place
  only to say these two guides are gone and where the handoff record lives.

## Notes

- Do not touch `docs/authority.md` or the README here; that is k69. Where a
  README link points at a removed guide, redirect it minimally and let k69
  reduce the README.
