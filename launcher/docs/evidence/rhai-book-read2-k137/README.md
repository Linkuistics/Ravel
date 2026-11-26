# Owner Read 2

This record preserves the partial second owner checkpoint after the whole-book
proof. The owner ended the remaining Rhai-specific read on 14 November 2026;
the checkpoint is abandoned, not completed. The owner is a labelled stand-in
for the intended reader. Their responses are not evidence about a developer
new to Ravel.

## Checkout and reading surface

The prepared book is from the corrected checkout based on commit
`b18d7f8356ead43a874792c8349abba2b688fa8a`, change `uwvlxqwz`. The checkpoint
notes and evidence are being recorded in change `ynxulxul`; no book or Rust
source has changed during this checkpoint.

The stated entry is chapter 7 of the locally built book at
<http://127.0.0.1:8775/07-the-parts-of-ravel.html>, with chapters 1–6 available
for reference. For the package answer below, the owner reported a heading in
chapter 10. Their initial entry, browser and whether the figures displayed
remain unreported; the heading does not establish which other material they
read.

[Preparation evidence](preparation.log) records the assistant's local book
builds and HTTP checks. All eleven chapter pages and 21 figures were available
over HTTP. That is mechanical preparation evidence, not an owner task result or
a fresh browser-rendering observation. The unchanged before/after manifests
cover the generated HTML and SVG files during those checks. The earlier
whole-book proof retains its own attribution and limits.

## Owner response: separate worker

On 14 November 2026, after the assistant asked why Ravel uses a separate Rhai
worker, the owner replied:

> The worker is separate for process isolation, so that it cannot affect the main ravel process, especially in the case of failure.

This is the owner's exact response, without an assistant-supplied explanation
before the attempt. The owner supplied no heading, source location, command or
observed output with it.

The assistant compared this response with chapter 7, “The Rhai worker evaluates
the script in its own process” (`docs/book/src/07-the-parts-of-ravel.md:318`).
The response identifies process isolation and failure containment. No
comprehension failure was reported in it. The owner's broad wording is retained
as supplied; it is not a verified guarantee that a worker cannot affect the
CLI. This is one response, not a completed Read 2 outcome.

## Owner response: two Rust packages

On 14 November 2026, after the assistant asked why Ravel uses two Rust packages
and requested the book heading used, the owner replied:

> "Find the source for a change" - the runner is a separate package because it is decoupled from the other functions

This is the owner's exact response. They reported “Find the source for a
change”. The assistant located that heading at
`docs/book/src/10-develop-ravel.md:143`; it maps the two packages and their
source and test responsibilities. This records a location used for this
answer, not a confirmed initial entry or an observation about other pages.

The response identifies separation of the runner from other functions. It
does not supply a concrete example of an independent test or change. No
comprehension failure was reported. The explanation was not supplied by the
assistant before the attempt. No command, output or further source location
was supplied with it. At this point Read 2 remained open; the next owner task
would have been the code trace to the final cancellation decision.

## Clarification of the cancellation task

After the assistant asked how a session reaches its final cancellation
decision and requested a trace through files and functions, the owner asked:

> It's not clear what you are asking. By cancellation decision do you mean the final outcome, or a cancellation that fails?

The owner asked about the meaning of the checkpoint instruction. They did not
identify an unclear passage in the book or supply a code trace. This records
ambiguity in the assistant's question, not an established book comprehension
failure.

The assistant clarified that the target is the final outcome: follow a session
through the code to where Ravel decides whether cancellation prevents it from
returning a result to the caller, naming the files and functions followed. A
failed attempt to stop a process is not the specifically requested case. This
clarification supplies the task scope, not the code path or outcome rule.

## Owner's change of direction

Before supplying the cancellation trace, the owner said:

> Given that I now want to change the configuration to use lua (it is by far the most popular extensions/configuration language), how much value is this current process?

Lua is the owner's newly stated intended configuration language. The
popularity claim is their rationale; this checkpoint did not verify it. The
question changes the context for continuing the read and is not a reported
book comprehension failure. The assistant recommends preserving this partial
evidence and ending the remaining Rhai acceptance read in favour of Lua
requirements. Knowledge of the runner and cancellation remains useful, but
the new runtime and its documentation need fresh validation. No complete Read 2
outcome is claimed. At that point the leaf remained live pending an explicit
decision to abandon the remaining checkpoint.

## Disposition

After the assistant recommended ending the remaining Rhai acceptance read and
moving to Lua, the owner replied:

> please remove those two jj workspaces, and let's move to lua

They then clarified:

> I would like to put the move to lua as a new GH issue, because want to first merge some of the separate repos in AgenticOrchestration

The remaining reader checkpoint is abandoned on that accepted change of
direction. Lua migration is recorded separately in
[Ravel issue 5](https://github.com/Linkuistics/Ravel/issues/5), explicitly after
repository consolidation. No Lua migration leaf was added to this Grove.
This disposition retains the owner's two explanations and the instruction
clarification as partial evidence; it supplies no new comprehension result.

## Tasks without an observed outcome

The owner supplied no code trace to the final cancellation decision or checkout
build/test commands and outputs before ending the read. Those tasks were not
reached. No assistant test run substitutes for them. The actual extent of
material read remains unreported; the change of direction is not a book
comprehension failure.
