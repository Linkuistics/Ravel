# rhai-book-use-k70

## Goal

Revise chapters 1, 2, 3 and 5 for Rhai, with runnable examples through the CLI seam.

## Context

The structure brief is `docs/book/contract.md`. Chapter 4 is the preceding draft.
Read `rhai-book-use-k27` for its settled owner decisions and verification limits.

## Done when

- The four chapters explain the Rhai policy worker, host capabilities and the distinction between evaluation harness runs and the managed session.
- Complete examples use Rhai and pass `task docs:examples`. Excerpts and the book build pass.
- Later-stage work is in the document node's `## Handed forward` list.
- The existing copy-edit leaf `rhai-book-use-k69` remains after the reference draft. Cut no duplicate.

## Decisions (running log)

- The structure brief names the reader as a 16-year-old software engineer and the outcome as the ability to configure and run a session.
  Its section plan assigns concepts to chapters 1–2, a complete session to chapter 3, and operational use to chapter 5.
  Its emphasis is authority and successful cleanup. Its exclusions include policy migration and agent installation.
- The graph CLI failed before it could list projects because an incompatible generation is active.
  Source verification uses direct reads. No graph freshness or coverage claim is available.
- The cancellation example uses a fake evaluation harness that sends INT to the Ravel CLI.
  Rhai cannot send the signal itself. This example exercises cancellation during a host call and leaves no final managed launch.
- The release-version line uses the existing placeholder until the release task writes 2.0.0.
  Leaving 1.1.0 there would direct the reader to an incompatible installation.
  The existing release guard accepts the placeholder and still owns the replacement.
- The leaf's one adversarial reviewer checked the authority, confinement, cancellation and notice passages against source.
  It found one actionable omission: failed evaluation-harness cleanup replaces a deadline with exit 5, before final cancellation precedence.
  `rhai_worker/host.rs::Host::settle` and the specification establish that exception.
  The chapter now states it beside the general rule, in the exit table and in the outcome explanation.
  This is a wording correction against an explicit source rule, with no implementation redesign or second reviewer needed.
- Technical checks used `choice.rs` and `rhai_worker/front.rs` for worker execution and its empty environment.
  `engine.rs` and the executed examples establish the Rhai exports and request interface.
  `host.rs`, `serve.rs`, `harness.rs` and `run.rs` establish the stop order, re-entry rule, result channels and notice fields.
  `limits.rs`, `inputs.rs`, `environment.rs`, `refusal.rs` and `confinement.rs` establish the bounds and caller interface.
  The 2.0.0 installed layout comes from the policy-evaluation specification and remains a delivery dependency.
- The final whole-book run passed 19 complete examples, with no failure, skip or page error.
  This includes the seven Rhai examples in the changed chapters. The run made no real model call.
  The examples log is `target/verification/rhai-book-use-k70.examples.log`, a local check output rather than a repository artifact.
- All nine book excerpts pass, the book builds without warnings, and all 14 figures match their committed sources.
  That figure check establishes regeneration, not technical correctness. The art handoff names the stale worker and cancellation figures.
- The four revised chapters pass strict STE and prose-break checks. The controlled-English check reports no hard finding in the changed documents.
  The whole-book STE scan retains the existing `binding` finding in `CONTEXT.md`, already assigned to copy-edit.
- The rendered link check examined 400 local links into or out of the four revised chapters and found no broken link.
  Its controls accepted a known heading and rejected a deliberately absent heading.
  The release-version guard accepts the placeholder.
- SHA-256 digests stayed unchanged across final validation for 325 input files.
  The subjects were `docs/book`, `scripts`, `crates`, `target/tools`, the root manifests and configuration, the glossary, README and built binaries.
  Dependency directories were excluded. Local before and after manifests are under `target/verification/rhai-book-use-k70.*.json`.
  The examples, excerpts, build, figure and prose checks all ran after the final document edits.

## Notes

The reference draft `rhai-book-use-k71` remains live, so this session closes no parent node.
The live copy-edit leaf `rhai-book-use-k69` is the document's successor after that draft.
No duplicate copy-edit leaf is necessary.
