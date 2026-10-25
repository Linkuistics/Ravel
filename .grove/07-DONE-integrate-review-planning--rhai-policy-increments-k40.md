# rhai-policy-increments-k40

**Integrates:** rhai-policy-increments-k6

## Goal

Triage the planning review and integrate supported corrections before the first
implementation increment runs. Preserve the agreed design and independently
working handoffs.

## Context

- Find the review commit by `rhai-policy-increments-k6` and read its findings,
  scenario ownership, acceptance coverage, source evidence and limits.
- The producer is `rhai-policy-increments-k3`, commit `8372228eefb7`. Its
  artifact is the planned subtree after this leaf, through
  `owner-policy-conversion-k39`.
- The root brief, the settled requirements in `plan-k1`, issue 2 and the
  integrated policy-evaluation design remain the contract. The current book
  contract also binds its documentation pipeline.

## Done when

- Each review finding has a reasoned disposition against its evidence and the
  contract.
- Supported corrections are integrated into the plan, and the brief chain and
  task bodies describe coherent inputs and verifiable handoffs.
- The integration records its verification and its limits under its kind's
  procedure.

## Notes

The review is evidence to triage, rather than a prescribed list of changes.
Read its committed artifact through the handle above. Do not treat its
conclusions as requirements.

## Decisions (running log)

**Method.** I read the findings from the review's own commit artifact, `rhai-policy-increments-k6`. I checked each claim against the source before I classified it. All five claims reproduce.

No finding needs a redesign, so this session cuts no new review chain. I spent no in-session reviewer. Every edit changes a task body or a brief. No code or spec changed.

**F1, keep the book checks green through the removal: a real issue, resolved.** The review located the break at the delivery leaf. The break starts earlier.

`crates/ravel/build.rs` embeds the worker digest. `scripts/examples.sh` (`require_fresh_build`, line 76) checks for that digest and for `libexec/ravel/ravel-policy`. The cutover leaf removes the digest, so the checker fails at the cutover. Chapter 9 line 150 also runs `--config policy.ts` and expects exit 0, and the first book node excludes chapters 7 to 11.

- `typescript-worker-cutover-k31` now owns the changes to `scripts/examples.sh` and `scripts/examples.test.sh`. It also ports each TypeScript complete example in chapters 7 to 11 to a Rhai policy, with no prose change. It re-anchors every excerpt it moves, and `task docs:examples` and `task docs:excerpts` pass at its end. Its decomposition hint keeps the Rust removal and the migration of tests, examples and checkers in one green boundary.
- `typescript-delivery-removal-k32` now also reconciles the fixtures that quote removed files: `scripts/examples.test.sh`, `scripts/excerpts.test.sh`, and the comments of `scripts/book.sh` and `scripts/figures.sh`. It cannot retire with a failing book check. Its done condition lists the four book tasks.
- The brief `typescript-removal-k29` drops the permission to record a failing check. It adds the book checks to its done condition.
- `rhai-book-inside-k35` reads the cutover log for the examples that already run Rhai.

**F2, no supervisor memory bound on the first LLM call: a real issue, resolved.** The Ravel CLI collects the body, so the limits of the worker cannot bound it.

Design review F2 got accepted, and the node brief already called the bounds "a safety property of the first call".

- `rhai-llm-anthropic-k16` now holds the body bounds. A successful body has at most 8 MiB, and a failure body at most 64 KiB. Ravel counts bytes as they arrive, handles a declared length, and decodes no compressed body.
- That leaf also holds the `output_limit` code and the scenarios "Oversized successful body" and "Large failure body".
- `rhai-llm-bounds-k17` keeps the answer text bound, the stall deadline, the cancellation and the check of the raw document copy. It adds a regression guard on the body bounds.
- The brief `rhai-llm-calls-k15` states the new split. The decomposition hint of k16 forbids a child that exposes a live call before the bounded collection exists.

**F3, synthetic fixtures cannot discharge the real-exchange proof: a real issue, resolved.** The spec says `rig-cassette` replays one recorded real exchange for each API type. `rhai-llm-upgrade-proof-k21` now keeps that item as real-only. 

When the owner gives no credentials, the session stops and asks. It recommends an `impl` leaf `rhai-llm-real-recordings` ahead of the release leaf, with the replay machinery proven on a fixture named `synthetic`. The other option is an owner-approved change of the spec. The deliverable stays visible in the tree either way.

**F4, reader checkpoints made optional: a real issue, resolved.** Section 4 of `docs/book/contract.md` requires Read 1 and Read 2 without a size condition. 

The briefs `rhai-book-use-k26` and `rhai-book-inside-k34` now make each read an obligation. 

The proof stage cuts the `requirements` read leaf unless section 4 no longer requires it. `rhai-book-use-k27` gets a fourth owner question on whether the reads stay, with the recommendation to keep both. If the owner waives a read, the first draft changes section 4 and names the read in its log. This is how the plan carries the contract decision.

**F5, a test seam that conflicts with the agreed seam: a real issue, resolved.** The note of `rhai-llm-structured-output-k20` allowed unit tests of the validator. The spec (lines 1126 and 1140) and `plan-k1` settle one CLI seam with no in-process seam. 

The note now says to test the validator only at the CLI seam. The done condition lists the edge cases. They are a missing required property, an extra property, `enum` and `items` mismatches, and nesting at 32 and 33 levels.

**Not changed.** The review's remarks on session size name no defect. They concern `rhai-policy-worker`, `rhai-llm-anthropic`, the first book draft and the delivery removal. 

Each body already says how to decompose. 

The release-leaf constraints reached their owners, as the review found. I verified the review's source citations at `build.rs`, `scripts/examples.sh:76`, `scripts/examples.test.sh:445`, chapter 9 line 150, `docs/book/contract.md` section 4, and the spec lines 614, 889, 1126 and 1140.

## Verification and limits

- The controlled-English linter (`scripts/ste-lint.py`) reports no hard finding on the changed plan files. Advisory findings remain, mostly on lines that were already there.
- No build, test or book task ran. The edits change plan text only. The corrected done conditions are not yet executed evidence, and the leaves that now own them take the first measurement.
- The scope of the F1 edits comes from a count of `policy.ts` mentions in chapters 7 to 11. The count found chapter 9 as the one complete example. The cutover leaf must search chapters 7, 8, 10 and 11 again, because this session did not run the book tasks.
- The code graph was unavailable, so I used direct reads and `grep`.
- The recommended answer to the F4 question is mine. The owner decides it in `rhai-book-use-k27`.
