# Ravel.gh-issue-2 — brief

## Goal

Replace Ravel's TypeScript configuration scripts with embedded Rhai scripts, as GitHub
issue 2 asks. Rhai scripts ask a model through a harness run of Claude Code or
Codex, with the owner's own sign-in, and Ravel keeps execution authority. The
owner amended issue 2 for that in `llm-calls-through-harnesses-k58`, and Ravel
no longer uses Rig. The work ends with the Ravel 2.0.0 release and the owner's
local upgrade.

## Done when

- Every acceptance criterion in issue 2 holds, as the owner amended it, and
  tests through the agreed seam prove each one. The amendment is a comment on
  the issue and the running log of `llm-calls-through-harnesses-k58`.
- Rhai is the only configuration script format. The Bun worker, the
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
  - `rhai-route-table` makes a deterministic Rhai configuration script work, and it makes the
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
  - `typescript-removal` freezes the equivalence results, removes the
    TypeScript path and its delivery machinery, and reconciles the specs.
  - `configuration-names` gives the remaining interface consistent configuration names.
    It follows the removal so that the frozen observations precede the rename and the old worker needs no second conversion.
  - `rhai-book-use` revises Parts I and II for Rhai and completes the current setup and naming corrections.
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
- ADRs in play: `configuration-script-uses-rhai`,
  `configuration-evaluation-precedes-the-launch`,
  `a-configuration-script-asks-a-model-through-a-harness-run`, `ravel-supervises-the-session`
  and `the-launched-child-is-a-job`.
- Specs in play: `configuration-evaluation`, which is the 2.0.0 design, and
  `session-execution` and `result-job-completion`.
- Glossary terms in play: Configuration script, Rhai worker, Host capability, LLM
  call and Provider. Also Harness run, Evaluation budget, Selection context,
  Harness, Refusal, Caller and Owner. See `CONTEXT.md`.
- `task docs:rhai-configuration` serves the design views in `docs/design/rhai-configuration/`.
- The test seam: the real `ravel start-session` against a sandbox home. Fake
  harness scripts stand behind it, also for an LLM call.

## On the horizon

- On 2026-10-28 the owner proposed notebook-like tutorials and runnable examples, possibly using only a formal structure. Explore readable steps with executable blocks, explicit prerequisites and shared state, reset rules, and expected outputs or state checks. The existing Markdown example checker is a starting point. Mechanical execution and TestSubject's proposed assessment of explanatory completeness remain separate questions; no new format is selected yet.
- The owner intends TestSubject to use an LLM as the reader of a tutorial and to execute its steps through tools.
  It checks whether all required instructions, prerequisites and context are supplied, as well as the observed outcomes in the stated environment.
  `rhai-book-use-k74` records the feasibility assessment and chapter 4 as a proposed first case.
  TestSubject is pre-development in its own repository. Its implementation is not yet a dependency of this workstream.
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

- The release-layout node fixes the final Rust notice path at `share/ravel/notices/THIRD-PARTY-NOTICES.md`. `task notices` generates it as `target/notices/THIRD-PARTY-NOTICES.md`; `rhai-installed-layout-k108` installs and archives it. `rhai-book-inside-k34` uses that path in chapter 6 and the shared installed-file figure after k108 lands.

- The owner reached chapter 4's example-setup section and reported F17: its files and table lack context.
  They also requested removal of “policy” from filenames and arguments (F18), then asked the session to continue.
  `configuration-names-k92` owns the consistent public-interface rename. Its map is the requirements session's interpretation.
  The existing TypeScript-removal node now precedes it. The book correction follows as `rhai-book-use-k93` through `rhai-book-use-k96`.
  Read 1 stays live under `rhai-book-use-k74` and resumes at the interrupted setup section after proof.
  The practical tasks and Read 2 remain required. The versioned 1.1.0 ending command and live owner files stay with their existing handover.
- A bounded fresh LLM trial manually performed chapter 4's first example successfully, but did not identify the owner's context complaint.
  It also exceeded its reading boundary during a later citation lookup. Its record in `rhai-book-use-k74` separates execution and model judgement.
  TestSubject remains an LLM-reader proposal. The trial does not establish that the proposed validator detects missing instructional context.
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
- `rhai-executable-book-examples-k100` mechanically ported chapter 9's “Run a fake that ignores TERM” complete example for `rhai-book-inside-k35` and `rhai-book-inside-k36`.
  Its Rhai selection preserves `finished` and status 0. Only its policy file/block and `--config` argument changed; prose still awaits those drafts.
  No other complete example in chapters 7–11 needed a policy port, and all nine current source excerpts still match.
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

- `rhai-delivery-cutover-k110` delivers `bin/ravel` as the only executable, with `share/ravel/examples/{static,sample,llm}.rhai` and `share/ravel/notices/THIRD-PARTY-NOTICES.md`. Archives also keep LICENSE/README. `task notices` generates the notice at `target/notices/THIRD-PARTY-NOTICES.md`, and installation, archives and the Homebrew formula copy those bytes unchanged. `task install` uses `scripts/install.sh`; `task smoke` and target-floor checks use `scripts/installed-smoke.sh`. The worker/SDK/dispatcher, delivery Bun pin/notices and probes are removed; book Bun stays in `scripts/book-bun.sh`. k92 still owns configuration names; k33 owns spec reconciliation.

- `rhai-spec-reconciliation-k33` closes `typescript-removal-k29`.
  `docs/specs/configuration-evaluation.md` now owns the current Rhai configuration rules and installed/archive delivery contract.
  `session-execution.md` retains managed launches, result transport and confinement, with links to configuration rules.
  `result-job-completion.md` retains detailed settlement and native-entry requirements and distinguishes historical receipts from current Rhai acceptance checks.
  The ADRs, README and design views use the glossary terms. `CONTEXT.md` already matched and needed no change.
  Existing literal interfaces, filenames and explicit old anchors remain for `configuration-names-k92`.
  The spec distinguishes reserved environment-grant classes from the six exact launch controls and explains fresh harness-run markers and result paths.
  Frozen comparison rules now state their exclusions and intentional language differences. Do not regenerate the original observations to remove those differences.
  Current macOS `task check`, all four book example/excerpt tasks, scoped local links and prose checks pass.
  k110 retains the three-target delivery and formula evidence. This leaf changes documentation only and does not rerun those delivery measurements.
  The book nodes retain their existing broader narrative duties.

- `configuration-names-k114` closes `configuration-names-k92` after checking the
  original contract against k111–k114. Current code uses `config.rhai`,
  `--config-env`, `configEnv`, `RAVEL_CONFIG_RUN`, configuration JSON/diagnostic
  names and failure schema 2 (routing schema 2 and script schema 3 stay).
  There are no old-name aliases. The retained Rust module is `config.rs`;
  the spec is `docs/specs/configuration-evaluation.md`; ADR slugs are
  `configuration-script-uses-rhai`, `configuration-evaluation-precedes-the-launch`
  and `a-configuration-script-asks-a-model-through-a-harness-run`.
  Design views are in `docs/design/rhai-configuration/`, served by
  `task docs:rhai-configuration`. Chapter 4 is
  `04-write-a-configuration-script.md`.
  Historical observations, rejected old inputs, arbitrary caller data and
  filenames, owner labels/refusal text, third-party names and stable Grove
  handles retain their exact spellings. The old-token inventory classifies
  every residual; firing content/path controls and unchanged frozen fixtures
  support the audit. L63 now also requires checking replacement phrases and
  distinguishing a generic cancellation policy from an owner script.
  Final macOS checks passed 611 enabled Rust tests, all 22 complete examples,
  checker controls/mutants, nine excerpts, 18 figures, the book build, local
  book links, installed smoke, the archive/formula checks and native E06.
  Fresh archives cross-compiled both Linux targets; no new Linux runtime/floor
  campaign or signed-in harness call ran. No live installation or owner file
  changed. The dependent book correction still owns F17 and explanatory
  restructuring. Parts III–IV retain their full source-walkthrough and delivery
  reconciliation, whole-book proof and Read 2; this naming close does not finish
  that narrative work or the live Read 1 checkpoint.
