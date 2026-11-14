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

This task has not started. No owner words or Read 2 outcomes exist yet.
Use the corrected local checkout, not the immutable published 2.0.0 book.
The proof inspected all unique local figures, but it did not publish its two
prose corrections. Keep the historical `docs/evidence/ravel-book-k311/` evidence
and ancestor `out.err` unchanged.
