# rhai-book-read2-k137

## Goal

Conduct required owner Read 2 after the whole-book proof. Record the owner's
explanation of the worker and packages, trace of final cancellation, and actual
checkout build and test results. The owner is a labelled stand-in for the
intended reader, not evidence about a developer new to Ravel.

## Context

`docs/book/contract.md` section 4 defines this checkpoint. The final proof is
`rhai-book-inside-k136`; its evidence is
`docs/evidence/rhai-book-proof-k136/README.md`. All eleven chapters received a
fresh whole-document read. Two corrections clarified chapter 3's saved-directory
setup and chapter 10's private write/flush followed by atomic hard-link
publication. No executable example or figure changed.

This leaf is outside editorial node `rhai-book-inside-k34`, after its proof.
Read 1 is already owner-confirmed in `rhai-book-use-k74`; do not repeat it or
substitute it for Read 2. Mechanical checks, inventory mapping and the proof's
model review do not complete this checkpoint.

## Done when

Build the local book using its documented workflow and confirm that its figures
load before the owner starts. State the entry: chapter 7 in the locally built
book, with chapters 1–6 available for reference. Honour another entry the owner
chooses and record it.

Ask the owner to explain, in their own words, why Ravel uses a separate Rhai
worker and two Rust packages. Ask them to follow a session through the code to
its final cancellation decision. Then ask them to build and test Ravel from a
checkout using the book. Do not provide an answer before their attempt or
replace an attempt with a model-produced explanation or test run.

Keep the owner's exact words, progress, reported page/heading/source locations,
commands, observed outputs and places not reached. Attribute each observation.
Separate owner explanations, actual task performance, mechanical checks and
model-proxy results. Record the checkout revision and rendering surface.

Stop this read at the first comprehension failure. Preserve the failure and
unreached material; do not coach through it and report the later tasks as passed.
Route the specific correction to the stage that owns it, with the required
editorial correction run and a later owner checkpoint before dependent drafting
continues. If no failure occurs, record each task's actual outcome and any
remaining uncertainty. A read outcome can be failure; it cannot be an assumed
success. Keep this leaf live while required owner input is pending.

## Notes

The owner ended the remaining Rhai-specific read on 14 November 2026. Preserve
this leaf as abandoned, not completed. Their explanations of the separate
worker and package boundary are partial evidence; the cancellation trace and
checkout build/test were not reached. The owner's words and the limits of
these observations are preserved in
`docs/evidence/rhai-book-read2-k137/README.md`.
Lua migration is separate follow-up work in
<https://github.com/Linkuistics/Ravel/issues/5>, after the owner consolidates
some of the separate repositories in AgenticOrchestration. Do not add a Lua
migration leaf to this Grove.
Use the corrected local checkout, not the immutable published 2.0.0 book.
The proof inspected all unique local figures, but it did not publish its two
prose corrections. Keep the historical `docs/evidence/ravel-book-k311/` evidence
and ancestor `out.err` unchanged.

## Decisions (running log)

On 14 November 2026, use the corrected checkout based on commit
`b18d7f8356ead43a874792c8349abba2b688fa8a` (change `uwvlxqwz`), not the published
2.0.0 book. The initial working copy was empty at
`72af4789dfc6311e4c3daa1ca7af0b597d9ec867` (change `ynxulxul`). Only checkpoint
notes and preparation evidence have changed in this session. The stated entry
is chapter 7, with chapters 1–6 available for reference; the owner may choose
another entry, which must be recorded when supplied.

The assistant ran `mise exec -- task docs:book:build` and then
`mise exec -- task docs:book`. Both builds returned successfully. The latter
command remains serving at `http://127.0.0.1:8775/` in tool session `84134`.
Entry URL: `http://127.0.0.1:8775/07-the-parts-of-ravel.html`.
`docs/evidence/rhai-book-read2-k137/preparation.log` retains commands and
observed results. HTTP GETs returned 200 for all eleven chapter pages and
all 21 figures, with the expected HTML/SVG content types and nonempty bodies.
The three entry-chapter figures are `session-authority.svg`,
`source-components.svg` and `host-bridge.svg`.

This is assistant preparation evidence, not owner performance or comprehension.
The optional headless-browser initialisation failed with
`The requested module './index.js' does not provide an export named 'default'`.
No fresh browser-decoding or visual-quality result is claimed. The previous
proof's visual inspection keeps its original attribution. Before the first
reading attempt, ask the owner to stop if a figure does not display. An unchanged
36-file digest records the generated HTML and SVG subjects before and after
the HTTP checks; it does not measure a new Rust build or test run.

The sequence is the worker explanation, the two Rust packages, the cancellation
trace, then checkout build/test. Proceed only while no comprehension failure has
occurred. Record the owner's words verbatim with their reported heading, source
locations, commands and outputs. Keep this leaf and interactive session live
while awaiting required owner input; no retirement or completion signal is due.

On 14 November 2026, the owner supplied the first explanation:

> The worker is separate for process isolation, so that it cannot affect the main ravel process, especially in the case of failure.

This response identifies process isolation and failure containment, which are
reasons in chapter 7, “The Rhai worker evaluates the script in its own process”.
That heading is the assistant's comparison location, not a location reported by
the owner. The owner has not reported their actual entry, heading, browser or
figure display. No comprehension failure was reported in this response; this
does not complete Read 2 or establish novice comprehension. Preserve the broad
wording as supplied, without treating it as a verified isolation guarantee.
The next required response is the owner's explanation of the two Rust packages.
No package explanation, code trace, command or checkout build/test output has
been supplied, so those parts have no observed outcome yet.

The owner's next response on 14 November 2026 was:

> "Find the source for a change" - the runner is a separate package because it is decoupled from the other functions

The owner reports the heading “Find the source for a change”, which the
assistant located at `docs/book/src/10-develop-ravel.md:143`. Record chapter 10
as the location used for this answer; do not infer that it was their initial
entry or that they read the intervening material. Their explanation identifies
separation of the runner from other functions. It supplies no concrete example
of an independent test or change. No comprehension failure was reported, and
neither explanation completes the checkpoint. Continue with the owner's code
trace to the final cancellation decision, without providing that trace first.
Checkout build/test remains pending after the trace. The browser and figure
display remain unreported.

Before attempting the cancellation trace, the owner asked:

> It's not clear what you are asking. By cancellation decision do you mean the final outcome, or a cancellation that fails?

This question concerns the assistant's checkpoint wording. It does not identify
an unclear book passage or establish a book comprehension failure. Clarify that
the target is the final outcome: trace a session to where Ravel decides whether
cancellation prevents it from returning a result to the caller, naming the
files and functions followed. The requested case is not specifically a failed
attempt to stop a process. Do not supply the code path or outcome rule before
the owner's attempt. Record the question and clarification in the evidence.
The trace and checkout build/test still have no observed outcome. Keep the leaf
and interactive session live for the owner's response.

The owner then changed the intended configuration language and asked about
the remaining value of this process:

> Given that I now want to change the configuration to use lua (it is by far the most popular extensions/configuration language), how much value is this current process?

Record Lua as the owner's intended direction. The popularity statement is the
owner's rationale, not a comparison verified in this session. This is a change
of direction, not a reported book comprehension failure. No cancellation trace
or owner checkout build/test has been supplied. Preserve the two explanations
and the instruction clarification as partial Read 2 evidence. The assistant
recommends ending the remaining Rhai acceptance read and opening Lua
requirements: runner and cancellation knowledge remains useful, while the
configuration language, runtime-specific rationale and documentation need
fresh validation. The owner has not explicitly authorised pruning this leaf;
it remains live and no completed Read 2 outcome is claimed.

The owner also asked which sibling Ravel workspaces are obsolete. Read-only
history comparisons found `coordinated-release-design` and
`skill-directory-delivery` at empty working-copy changes, with both named
bookmarks already ancestors of `main` (`d6d817654daf229f3b2edc3d51cfeee52ec565c3`).
`main..@` in each contains only its empty working copy. These are removable
workspace candidates; their old live Grove task files do not establish unique
unfinished work. In contrast, `main..@` in `gh-issue-2` contains the later
Read 1, book corrections/proof and this checkpoint's evidence. Keep this
workspace until that work is preserved elsewhere. All workspaces share the
repository stored in `Ravel/.jj/repo`; keep the main `Ravel` directory.
Only `.jj` metadata and, in `skill-directory-delivery`, generated `target/`
were listed as ignored entries in the removable candidates. No workspace,
bookmark or task was deleted or pruned.

The owner accepted the recommendation to end the remaining Rhai read:

> please remove those two jj workspaces, and let's move to lua

They then clarified the sequencing:

> I would like to put the move to lua as a new GH issue, because want to first merge some of the separate repos in AgenticOrchestration

File Lua as a separate workstream, not a new requirements leaf in this Grove.
The assistant created and read back the open issue
<https://github.com/Linkuistics/Ravel/issues/5>, “Replace Rhai configuration
scripts with Lua”. It explicitly places repository consolidation before the
migration and leaves the runtime, interface and transition decisions for that
work. The partial reader checkpoint is abandoned on the owner's accepted
change of direction. Its two answers are retained; no cancellation trace,
checkout build/test result or completed Read 2 is claimed. No book or Rust
source was changed by this disposition.

Before removal, both named workspaces were checked again: their working
copies were empty, `(main..@) & ~empty()` returned no commits, and no process
had a working directory in either target. Their only ignored material was
`.jj` metadata and, for `skill-directory-delivery`, generated `target/`.
`jj workspace forget coordinated-release-design skill-directory-delivery`
removed their jj registrations and linked Git worktree registrations. The
assistant then deleted the two checkout directories. Fresh jj and read-only
Git inventories list only `default` and `gh-issue-2`; both deleted paths and
their Git worktree administration directories are absent. The shared
repository in `Ravel/.jj/repo`, the main directory, the active `gh-issue-2`
workspace, their commits and the historical bookmarks are retained.

Stop the current Grove sequence after sealing this disposition. The owner
wants repository consolidation before the Lua work recorded in issue 5.
Automatic continuation would enter the remaining Rhai release, upgrade and
configuration-conversion tail instead. Those historical tasks still need
reconciliation with the already-delivered 2.0.0 artifacts if this Grove is
resumed; this session neither reruns nor prunes them. The pending work is the
owner's separate repository consolidation, then the Lua issue. Continuing or
decomposing this reader leaf cannot choose that consolidation's scope or make
Lua work timely. Preserve the tree and the active workspace, then end through
`grove-force-stop`; do not tear down or report this workstream completed.
