# what-ravel-is-for-and-its-actors-k77

## Goal

Supply the opening unit's figures, under `docs/book/contract.md` (figure needs
in section 1, figure notations, renderers and delivery in section 3, the layout
in `docs/book/figures.md`), and fix the book's visual vocabulary for later
units: the C4 system-context figure for chapters 1–2 and the C4 container
figure for processes and authority in chapter 2. The node brief's "Handed
forward" entry for art states each figure's reader question and what it must
and must not show.

## Context

- The copy-edited text is complete without figures and has no placeholders.
  A figure is placed where its question arises and carries its own notation
  key; the text it sits in stays as copy-edit left it, apart from the sentence
  that introduces the figure.
- Names in the figures use the chapters' spelling exactly: caller, owner,
  owner policy, policy worker, native front, runner (inside the native front),
  managed process, opaque result, result channel, session-result context.
  Copy-edit settled "managed process" (not "managed job/program") and "native
  front" (not "the front"); "job" is only the cleanup scope, the process group.
- The container figure shows the policy response path (worker to native front)
  apart from the result path (managed process via `end-session` and the result
  channel to the native front, then standard output). The worker ends, is reaped
  and has its private directory removed before the managed launch. Chapter 2's
  "Trust and confinement" states the same-user and moved-out-of-group limits;
  boxes must not suggest authentication or containment beyond the group.
- No renderer has yet produced a committed figure: `task docs:figures` and
  `task docs:figures:update` are verified only against their controls'
  fixtures, so this is their first real use.

## Done when

- Both figures exist as source beside generated SVG under
  `docs/book/src/figures/`, `task docs:figures` reports no difference, and each
  is referenced from its chapter with text alternatives a reader without the
  image can use.
- The notation (C4 levels, element and relation shapes) is explained where it
  first appears, and the visual vocabulary is recorded for later units.
- `task docs:book:build` and `task docs:examples` pass.
- As the last work act, add the `proof` leaf under this node with the same
  bare slug, unless a live later sibling already holds that stage.

## Notes

Copy-edit sent nothing back to draft and added nothing to the node brief's
"Handed forward" list; the proof entry there (forward chapter links, first-use
term links) stands.
