# Ravel.gh-issue-2 — brief

## Goal

Replace Ravel's TypeScript owner policy with embedded Rhai scripts, as GitHub
issue 2 asks. Rhai scripts ask a model through a harness run of Claude Code or
Codex, with the owner's own sign-in, and Ravel keeps execution authority. The
owner amended issue 2 for that in `llm-calls-through-harnesses-k58`, and Ravel
no longer uses Rig. The work ends with the Ravel 2.0.0 release and the owner's
local upgrade.

## Done when

- Every acceptance criterion in issue 2 holds, as the owner amended it, and
  tests through the agreed seam prove each one. The amendment is a comment on
  the issue and the running log of `llm-calls-through-harnesses-k58`.
- Rhai is the only format of owner policy. The Bun policy worker, the
  TypeScript SDK, their declarations, and their build and release steps are
  gone.
- The book, the README, the CLI help and the installation documents describe
  Rhai policies. The owner converts their configuration as a one-off manual task.
  The book assumes no TypeScript knowledge and has no migration chapter.
- The ADR set, the spec set and `CONTEXT.md` describe the new design.
- Ravel 2.0.0 is released.
- The owner's Homebrew installation runs Ravel 2.0.0, and the owner's local
  configuration is in Rhai.

## Decomposition

- `plan-k1` (requirements, done) settled what to build, with the owner.
- `rhai-policy-k2` (design) establishes how, as a spec and a reworked ADR set.
- `rhai-policy-increments-k3` (planning) cuts that design into increments.
  Each increment leaves main working. The increments follow, in order.
  - `rust-2024-toolchain` moves the workspace to Rust 1.99.1 and edition 2024.
  - `rhai-route-table` makes a deterministic Rhai policy work, and it makes the
    worker safe against a hostile script.
  - `rhai-local-capabilities` adds reads, imports, `load_context`, the data
    helpers, the three settings keys and discovery.
  - `rhai-llm-calls` built `ravel::llm` through Rig. The design leaf
    `llm-calls-through-harnesses-k58` then moved every LLM call to a harness
    run. The node now removes `ravel::llm`, Rig and the endpoint settings, and
    it keeps the schema check as `ravel::json_check`. It also adds the Taskfile
    task for a Rhai upgrade.
  - `rhai-harness-runs` adds `ravel::run_harness`, the re-entry refusal and the
    ordered stop. It tests an LLM call through a fake harness and adds smoke
    tasks for a real LLM call through each harness.
  - `rhai-example-policies` translates the example policies and proves their
    equivalence with the TypeScript originals.
  - `rhai-book-use` revises Parts I and II of the book for Rhai.
  - `typescript-removal` freezes the equivalence results, removes the
    TypeScript path and its delivery machinery, and reconciles the specs.
  - `rhai-book-inside` revises Parts III and IV and proves the whole book.
  - `release-ravel-2-0-0`, `homebrew-upgrade` and `owner-policy-conversion`
    end the workstream.
  A `review-planning` leaf reads this cut before the first increment runs.

The order is the dependency order. Planning needs the agreed design, and the
design needs the settled requirements.

## Pointers

- GitHub issue 2 states the work in the owner's words.
- The running log of `plan-k1` holds the settled requirements. Find its file
  with `grove-llm resolve plan-k1`.
- ADRs in play: `owner-policy-is-a-rhai-script`,
  `policy-evaluation-precedes-the-launch`,
  `a-policy-asks-a-model-through-a-harness-run`, `ravel-supervises-the-session`
  and `the-launched-child-is-a-job`.
- Specs in play: `policy-evaluation`, which is the 2.0.0 design, and
  `session-execution` and `result-job-completion`.
- Glossary terms in play: Owner policy, Policy worker, Host capability, LLM
  call and Provider. Also Harness run, Evaluation budget, Policy context,
  Harness, Refusal, Caller and Owner. See `CONTEXT.md`.
- `task docs:rhai-policy` serves the design views in `docs/design/rhai-policy/`.
- The test seam: the real `ravel start-session` against a sandbox home. Fake
  harness scripts stand behind it, also for an LLM call.

## On the horizon

- A verb that evaluates a policy without a launch. The owner kept it out of
  this workstream. With LLM calls in a policy, `start-session` is the only way
  to exercise the policy, and it launches the selected program.
- A skill for documents and books in other groves, made from
  `docs/documentation-lessons.md`. The owner names its home when the lessons
  are complete.
  It distils all documentation and communication feedback into reusable authoring decisions.
  Each decision gives its trigger, action, scope and a way to check the result.
  Ravel-specific corrections remain examples. Check transfer on material from another project.

## Notes

- Before requesting `grove-force-stop`, state the condition that prevents further work, the evidence, the remaining work, and why continuation or decomposition cannot resolve it. Record that explanation in the task's running log. An incomplete task alone is not a reason to force-stop. The owner requested this instruction on 2026-10-28 after two copy-edit sessions stopped without a visible reason to stop.
- `rhai-book-use-k26` closed after the whole-document proof `rhai-book-use-k73`.
  Chapters 1–6 teach Rhai policies and both harness-call recipes.
  All 19 complete examples, nine excerpts, fourteen figure reproductions, strict checks for these chapters and the book build passed.
  The proof corrected chapter 4's timeout summary and recorded source-derived coverage and local visual evidence in its task.
  Read 1 then opened the node again for a correction run.
- `rhai-book-use-k74` is the required Read 1 checkpoint after that node.
  The owner writes a fake-agent policy, runs a session, interprets refusal and cancellation, and uses a direct reference entry.
  Corrections precede dependent drafting. Read 2 remains required after the later whole-book proof.
- Read 1 stopped in chapter 1 on 2026-10-27. The owner asked for the patterns of eight findings in chapters 1–6 before a re-review.
  Section 2 of `docs/book/contract.md` now states those patterns for the whole book. Thus, the drafts of Parts III and IV use them too.
  The run under `rhai-book-use-k26` is draft `rhai-book-use-k75`, copy-edit `rhai-book-use-k76`, art `rhai-book-use-k77` and proof `rhai-book-use-k78`.
  Read 1 resumes after that proof.
- The correction proof `rhai-book-use-k78` closed `rhai-book-use-k26` again on
  2026-10-28. Chapters 1–6 passed all 21 example checks, nine excerpts,
  seventeen figure reproductions, strict prose checks, the book build and
  762 local link targets. Its task records fresh source-derived inventories,
  a failing link control, unchanged subject digests and the bounded Safari
  visual inspection. Read 1 resumes from chapter 1 under `rhai-book-use-k74`.
  The proof extends documentation lessons L16 and L40. Read 2 remains required.
- On 2026-10-28, the owner completed Part I and supplied findings F9–F16 in `rhai-book-use-k74`.
  They require a concrete example before the general model and the complete caller task contract.
  They also prefer configuration terminology, fewer incidental JSON version labels, the reason for a separate worker, Grove links, descriptive headings and familiar commands.
  Section 2 of `docs/book/contract.md` records these requirements for the whole book.
  The owner then asked to apply all lessons before Part II, superseding the earlier choice to collect the remaining notes first.
  This authorises the second correction run under `rhai-book-use-k26`: draft `rhai-book-use-k84`, copy-edit `rhai-book-use-k85`, art `rhai-book-use-k86` and proof `rhai-book-use-k87`.
  Every stage reads chapters 1–6 against the full accumulated lessons. Read 1 stays live and resumes at chapter 4 after proof.
  The required practical tasks still need the owner's results. Parts III and IV apply the same current lessons before Read 2.
- Each later session adds its new documentation or communication lessons to [the shared record](../docs/documentation-lessons.md).
  Each rule keeps its source, evidence class and Ravel application.
  `documentation-lessons-k79` created the record from the whole grove. The owner wants these lessons as a skill for other groves.
- Parts I and II describe the agreed 2.0.0 design before the default-entry cutover and delivery removal.
  The release retains responsibility for replacing chapter 3's version placeholder before publication.
  `typescript-delivery-removal-k32` supplies the final notice path to `rhai-book-inside-k34`, which reconciles chapter 6 and the shared installed-file figure.
  `rhai-spec-reconciliation-k33` corrects the spec's broad environment-scrub claim to the six exact launch controls.
  The Parts III and IV brief also owns chapter 8's old cancellation example and alternative beside the revised figure.
- <https://github.com/Linkuistics/grove/issues/24> carries the port of Grove's
  policy helpers to Rhai. Ravel 2.0.0 does not wait for it.
- The Homebrew upgrade runs inside a Grove session that Ravel 1.1.0 launched.
  Keep the 1.1.0 version and `policy.ts` until the driver restarts. The running
  log of `plan-k1` gives the reason.
- Ravel 1.1.0 refuses an unknown key in `settings.json`, and both versions read
  the same file. The conversion of the local configuration adds no new settings
  key until the driver restarts.
- `task release:major` refuses outside Ravel's default colocated jj workspace.
  The work of this workstream lives in a secondary workspace, so the release
  leaf asks the owner how the work lands on main.
- The book is published with each release. Both book nodes finish before the
  release leaves run.
- Run `task docs:excerpts` and `task docs:examples` after a change to code that
  the book quotes. Correct a moved excerpt in the same leaf.
- `task test:native-linux` runs on a Linux image whose rustup holds every target
  that `rust-toolchain.toml` lists, because `/opt/rustup` is read-only to the
  campaign user. A new target in that file needs the same `-t` flag in
  `scripts/native-linux/Dockerfile`. The native-entry campaign selects only
  `native_` cases, so it runs no Rhai test. The `rhai-worker` step of
  `scripts/native-linux/measure.sh` runs the Rhai tests, and
  `native-incumbent-guard-build-rs-k51` corrects the campaign step.

- The second correction proof `rhai-book-use-k87` closed `rhai-book-use-k26` on 2026-10-28.
  The complete review task now precedes the general model, and chapters 1–6 use the shared configuration terms.
  The worker mechanism, descriptive headings, figures and reference fragments agree.
  All 22 examples, nine excerpts, 18 figure reproductions, strict prose checks and the book build passed.
  Link checks passed 2,075 local HTML targets and nine README links, with failing controls.
  Fresh Safari inspection covered the six openings and all 13 figures at chapter width. The proof records its inventories, subject digests and limits.
  `rhai-book-use-k74` has the correction recap and resumes at chapter 4 after it refreshes its local book and private shell.
  The practical tasks still need the owner's results. No new checkpoint or editorial tail was cut.
  The existing removal and later book leaves retain the final layout, help/spec terminology, whole-book reconciliation and Read 2 duties.
