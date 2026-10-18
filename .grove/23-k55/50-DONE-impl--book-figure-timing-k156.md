# book-figure-timing-k156

## Goal

Add the UML timing notation to the book's figure engine, and deliver the first
timing figure with it: the timeline after `end-session` sends the result. The
contract's figure list asks, for chapter 9: “What is the timing after
`end-session` sends the result (2-second grace, TERM, up to 5 seconds, KILL)?”
The engine has no timing module, and the chapter 9 art stage must not build a
layout module inside a figure stage.

## Context

- Contract: `docs/book/contract.md` ("Format and media" and the figure list).
  Mechanics: `docs/book/figures.md` ("Layout, then draw", "The layout modules",
  "Visual vocabulary"). The seam is: a layout module measures its text and
  returns a layout record with no colour, weight, size or roughness; `_draw.typ`
  draws any record under a named style; the figure's `.layout.json` is
  committed and compared. The crisp style (roughness 0) is the only style.
  `book-figure-activity-k153` added the previous module. Read what it did and
  follow it, including its entry in `figures.md`.
- **The reader question.** After the result appears in the channel (or the
  managed process exits on its own), what does Ravel do, and when? The
  schedule is in `crates/keyed-launch/src/run.rs` (the escalation: `grace` runs
  from the channel's appearance to SIGTERM, `kill_grace` runs from SIGTERM to
  SIGKILL; the defaults are the 2 and 5 seconds the specification states), in
  `docs/specs/session-execution.md` (“Managed process and result”) and in
  `docs/adr/the-launched-child-is-a-job.md` (descendants receive time during
  their parent's shutdown, and surviving group members are terminated before
  the direct child is reaped). The file also holds other bounded waits (for
  example the group confirmation and the error settlement). Read the code and
  decide which times belong on the figure: the figure answers the question
  above and does not become a list of every constant.
- **Notation to cover**: the lifelines the question needs (for example the
  result channel, the managed process, the rest of its group, and the runner),
  states on each lifeline, state changes at points on a shared time axis, a
  time constraint (a bracket with a bound such as “up to 5 seconds”), and the
  condition that ends a wait early (the process exits inside its grace, and is
  never touched). Verify each state and each bound against the code and the
  runner tests. Decide whether the time axis is drawn to scale. If it is not,
  the figure says so on the figure.
- Chapter 9 also shows the lending and recovery of the terminal as a sequence.
  That figure uses the existing sequence module and is not part of this leaf.
- The book teaches no notation. The figure's own conventions are on or beside
  the figure.
- Text rules apply to every label, key and note: ASD-STE100 writing rules,
  `docs/book/ste.md`, “Ravel CLI”, “send”. Check the figure's
  `.layout.json` with `task docs:ste -- --strict`.

## Done when

- A layout module (`_timing.typ`, or the name the existing modules suggest)
  measures its text and returns a layout record. `_draw.typ` draws it under the
  existing style, with palette roles, strokes and sizes from `_style.typ`. The
  new figure uses the same colours, fonts, weights and arrow style as the other
  figures.
- A figure source (name it for its subject, for example
  `escalation-timing.typ`), its SVG and its `.layout.json` are committed.
  `task docs:figures` passes for every figure, and `task docs:figures:test` has
  a control for the new module (a changed source, a changed label and a changed
  bound each fail the compare). Watch each new control fail before you credit
  it.
- The figure is accurate: each state, bound and ordering was checked against
  the code and a runner test, and the log names the function or test that
  supports each one. The log says which time constants are on the figure and why
  the others are not.
- `docs/book/figures.md` describes the module and records a dated viewing of
  the figure at the 750-pixel column in a browser, with the limits of that view.
- No chapter is edited. The chapter 9 art stage places the figure and may refine
  its labels through the shared module.

## Notes

- If a Typst or CeTZ limit makes the notation impractical, record the limit and
  ask the owner through the log before you change the notation.
- Typst 0.15.1 and CeTZ 0.4.2 are pinned by the contract.

## Decisions (running log)

**Module shape (2026-10-18).** `_timing.typ` places every instant itself. A figure
source is a list of panels; a panel has `instant`s, `lifeline`s of `state`s, `msg`s
and `bound`s, so it holds no coordinate. This follows `_activity.typ` and
`_deployment.typ`, not `_c4.typ`. The rows are the UML general value lifeline
(a band with the state name inside) and not a stepped state lifeline: one row per
lifeline keeps the figure about 890 units tall for two panels, and a stepped
lifeline needs a row per state. The module spaces instants from what the labels
need (state text, adjacent instant labels, signal labels, constraint labels), so
**the time axis is not to scale and the figure's key says so.** The 2-second and
5-second waits differ from the exit time of a real child, which is unbounded below
the bound, so no scale would be true for both panels. No Typst or CeTZ limit made
the notation impractical, so no owner question was needed. `_draw.typ` and
`_style.typ` did not change: the record uses boxes, lines and text. The eleven
existing SVGs and records are byte-for-byte unchanged (`git status` after
`task docs:figures:update` lists only the new files).

**Two panels, because one axis shows one course (2026-10-18).** Panel A is the full
schedule (the process and its group ignore TERM): result appears, runner sees it,
2 seconds, TERM, up to 5 seconds, KILL, the runner stops the rest of the group,
reaps, and confirms the group is gone. Panel B is the early exit: the process
exits inside the grace period, so TERM and KILL never go to it, and the runner
still sends KILL to the group (a descendant that still runs) before it reaps.
Drawing the early exit on panel A's axis would put an exit after a later step. The
brief asked for “the condition that ends a wait early”; panel B is that condition,
and the key says “A wait ends when the process exits. Then the runner sends no
signal to the process.” An exit after TERM (inside the second grace period) is
covered in words by that key line and by the “up to 5 seconds” label; it has no
panel of its own, to keep the figure to two panels.

**Each state, bound and ordering, checked against the code and a test
(2026-10-18).** Sources: `crates/ravel/src/run.rs` (`ESCALATION`: 2 s and 5 s),
`crates/keyed-launch/src/run.rs` (`watch`, `OwnedChild`, `supervise_reported`,
`confirm_gone`) and `docs/specs/session-execution.md` (“Managed process and
result”). The runner tests are in `crates/keyed-launch/tests/launch.rs`.
- The runner sees the result at a poll, not at once, and the grace starts then:
  `watch`, arm `Watch::Running if channel.is_some_and(Channel::appeared) =>
  Watch::Signalled(Instant::now())`, with `POLL_INTERVAL` 500 ms (its doc says it
  only bounds how late an escalation starts). Hence the figure's separate
  “end-session sends the result” and “Runner sees the result” instants, and the key
  line “A step can start up to 0.5 second late”. Test:
  `a_signalled_child_that_keeps_waiting_is_terminated_after_the_grace` (the grace
  elapses before anything is sent).
- Grace period of 2 seconds, then TERM: `watch`, arm `Watch::Signalled(at) if
  at.elapsed() >= escalation.grace`, which calls `child.signal(SIGTERM)`. Test:
  the same one (`status.signal() == Some(SIGTERM)`, `elapsed >= grace`).
- TERM and KILL go to the process and to its group: `OwnedChild::signal` calls
  `stop_target(-pid, signal)` then `stop_target(pid, signal)`. Test:
  `the_escalation_reaps_the_childs_descendants` (a grandchild is gone, and a
  bystander outside the group is untouched). The figure's “Each signal goes to the
  process and to the rest of its group” rests on that function and test.
- Second grace period of up to 5 seconds, then KILL: arm `Watch::Terminated(at) if
  at.elapsed() >= escalation.kill_grace`, `child.signal(SIGKILL)`. “Up to” because
  the arm is skipped when the child exits first. Test:
  `a_child_that_ignores_sigterm_is_killed_after_the_kill_grace` (`signal() ==
  Some(SIGKILL)`, `elapsed >= grace + kill_grace`).
- Early exit: the loop breaks on `child.exited()` before the grace arm runs, so a
  child that exits inside its grace is never signalled. Test:
  `a_child_that_signals_and_exits_inside_the_grace_is_never_touched` (`End::Exited`,
  `elapsed < grace`).
- The group stop precedes the reap, on every path: after the loop `watch` calls
  `child.kill_group()` (SIGKILL to `-pgid`, twice, 20 ms apart) and then
  `child.reap()`. The exit is seen with `waitid(... WNOWAIT)`, so the process is
  “ended, not reaped” until then (the comment in `OwnedChild::exited`). Test:
  `a_term_ignoring_descendant_is_gone_before_the_launch_returns`, which runs the
  three endings (exits on TERM, exits inside the grace, exits on its own) and
  expects `Group::Gone` each time. Panel B's group KILL is the second of those.
- After the reap, `supervise_reported` calls `confirm_gone`, which polls until the
  group is absent. That is the last runner state on both panels. The 1-second bound
  is not drawn (below).
- The default 2 and 5 seconds are the ones the specification states
  (`session-execution.md`: “2-second grace, then TERM, up to 5 seconds kill-grace,
  then KILL when needed”).

**Which times are on the figure, and why the others are not (2026-10-18).** On the
figure: the 2-second grace and the 5-second kill grace, which answer the
reader's question, and the 0.5-second poll as a key sentence, because it makes
every step up to 0.5 second late and the reader would otherwise read “2 seconds”
as exact. Not on the figure, each because it does not answer “what does Ravel do,
and when, after the result is sent”: `SECOND_KILL_PAUSE` (20 ms between the two
group KILLs, a race guard), `GROUP_CONFIRMATION` (1 s, how long the runner waits
for the system to answer that the group is gone, after the reap), `ERROR_SETTLEMENT`
(2 s, the bound of the failure path and of the wait for an exit after KILL),
`KILLED_POLL_INTERVAL` (10 ms), and the worker's `DRAIN_GRACE` and `CLEANUP_GRACE`
(they belong to the policy worker, which ends before this launch). The confined
runner's immediate KILL belongs to cancellation, not to this path. Chapter 9 can
add any of them in prose.

**Controls (2026-10-18).** `scripts/figures.test.sh` has a timing fixture (three
lifelines in three roles, an X, two signals, a constraint) that passes unchanged,
and mutated copies: an added instant, a changed state label and a changed bound
each fail both the record compare and the SVG compare. Five refusals each have
their own reason: an unknown instant, a state that does not end after the previous
one, a signal between rows that are not next to each other, a constraint that runs
backward and two instants with one id. Seen failing: (1) the bound control with its
sed turned into a no-op (`[2 seconds]` to `[2 seconds]`) stops the run with “the
timing control 'bound' did not change the source”; (2) the label control with its
expected text changed to `clock.layout.jsonX` stops the run, because the real
refusal no longer matches. Both edits were reverted, and the script is the one
committed.

**Text and a table row (2026-10-18).** `task docs:ste -- --strict` refused the
word “timing” in the title (“UML timing”). It is the name of a UML diagram type,
as “UML sequence” and “UML activity” are, so `docs/book/ste.md` gained a row in
its table of words in “-ing” (“UML timing”, a technical name) and no label changed.
`figures.md` changed in its module list (“Nine modules”, the file tree), one row
of “What each view means”, two module sections and a dated viewing record.

**Viewing (2026-10-18).** The figure was viewed at the 750-pixel column in the
Playwright browser from a scratch page. The first view found the name “Rest of the
process group” taller than its row; the row height now includes the name's height
(`rh` in `lay-panel`), and the final view showed no fault. The record and its limits
are in `docs/book/figures.md`. No chapter was edited; the chapter 9 art stage
places the figure and may refine its labels through the shared module.
