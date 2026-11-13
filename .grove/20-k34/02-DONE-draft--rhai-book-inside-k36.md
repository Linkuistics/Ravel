# rhai-book-inside-k36

## Goal

Draft the changes to chapters 10 and 11 of the Ravel book. A reader can prepare a checkout and build Ravel 2.0.0. The reader can also test it, deliver it and recover a partial release.

## Context

- The structure brief is `docs/book/contract.md`. Read the running log of `rhai-book-inside-k35` first.
- Apply the owner's Read 1 patterns in section 2 of the contract on every page. The owner asked for them in the entire book on 2026-10-27.
- Read all of `docs/documentation-lessons.md` and the later Part I findings F9–F16 in `rhai-book-use-k74`.
  Apply the current rules throughout both chapters, including configuration terms, concrete examples, descriptive headings and familiar commands.
- Chapter 10 describes a checkout, the setup with Bun, the worker build, the typecheck, the test seams and the evidence documents. Chapter 11 describes the versions, the identity, the installation, the archives, the formula and the release tasks.
- The new facts come from the leaves `rhai-upgrade-task`, `rhai-harness-smoke`, `typescript-delivery-removal` and `rust-2024-toolchain`. They are the Rust 1.99.1 toolchain, the Rhai upgrade task, the opt-in smoke tasks, the notices task, and the new archive layout.
- The book tools keep a checkout-local Bun. Say that they do, and that the release carries no Bun.
- The release step `brew upgrade ravel` removes the old version unless `HOMEBREW_NO_INSTALL_CLEANUP` is set. A running Grove session needs the old version. Chapter 11 states the rule.
- Run every complete example through `task docs:examples`, and check every excerpt with `task docs:excerpts`.

## Done when

- Chapters 10 and 11 describe the tasks, the files and the release steps that exist. Each task name was run, or the running log says why it was not.
- The chapters cover the new test seam: the fake harness for an LLM call, the opt-in smoke tasks and the Rhai upgrade proof.
- The refusal of a lone `policy.ts` appears as a complete example somewhere in the book, and it runs.
- Chapters 10 and 11 follow each Read 1 pattern of the contract.
- Each new lesson about documentation or communication is in `docs/documentation-lessons.md`.
- The leaf names the defects that a later stage owns in the `## Handed forward` list of the node brief.
- The last act cuts the `copy-edit` leaf, unless a live copy-edit leaf already stands below the node.

## Notes

- Do not describe a release that has not happened. The release leaves run after the book is final.
- If the leaf proves too big, decompose it by chapter.

## Decisions (running log)

**The later grove delivered chapters 10 and 11, 2026-11-14.**
L82 asks a waiting draft to map its done-when list to the delivered work before it drafts.
The later grove drafted both chapters in `ca8c7d01` (`ravel-book-inside-k308`), with trials in `docs/evidence/develop-ravel-k308/README.md`.
It then copy-edited them in `8e07b803` (k309), made the figures in `7d9526e9` (k310) and proofed the whole book in `424af53d` (k311).
After `424af53d`, chapter 11 changed one command line, `bash scripts/release-evidence.sh`, and chapter 10 did not change.

| Done-when item | Delivered location or gap |
|---|---|
| The chapters describe tasks, files and release steps that exist | `task --list-all` defines 56 public tasks, and each one is in the chapters. Every other task-like token is a check stage, a script mode, the internal `release` task or an English word after “task”. A planted unknown name and a removed real name both appeared in the comparison. The archive table matches the manifest in `scripts/release-common.sh`. The Rust, Rhai, Bun, mdBook, Typst, CeTZ, oxifmt, DejaVu and cargo-about pins match their sources. |
| The new test seam | Chapter 10, “Choose a test seam” and “Test an LLM call with a fake model”: the fake harness in `rhai_llm_calls.rs`, the opt-in `harness:smoke:claude` and `harness:smoke:codex`, and `rhai:upgrade-proof`. |
| The lone `policy.ts` refusal as a complete example | Chapter 11, “See what promotion changes for an owner”. `task docs:examples` runs it. |
| Read 1 patterns | Proof k311 checked each pattern of contract section 2 in all eleven chapters. This session's edits keep the existing concrete cases and terms. |
| Lessons | Draft k308 added L80 and L81. This session adds L83 and L84. |
| Handed forward | Nothing. The figure text of `release-delivery` states the procedure generally, so it stays true after promotion. |
| Copy-edit | Copy-edit `rhai-book-inside-k119` already stands below the node, so this session cuts none. |

The chapters also deliver the leaf's context items. They cover Rust 1.99.1, the checkout-only book Bun with no Bun in a release, the notices task and the archive layout.
Chapter 11, “Keep each old keg for its active readers”, states the cleanup rule of `brew upgrade`.

**Ravel 2.0.0 is released and promoted, so chapter 11 had stale statements, 2026-11-14.**
The leaf's note said not to describe a release that had not happened. The release happened in `ed6cedbf`, and the tap promoted it.

The remote `main` of `linkuistics/taps` is `06c7198`, “ravel v2.0.0 (promote ravel@2 v2.0.0)”. `Formula/ravel@2.rb` carries `deprecate! date: "2026-11-11"`.
The readiness receipt in the main checkout names Grove 22.9.0, verified on 2026-11-13. The release log shows the cut with `CHANNEL=versioned-2`.

The L81 search found one stale sentence: “Until the cut of 2.0.0, the checkout's version is 1.1.0”. `Cargo.toml` is now 2.0.0.
The tap's state showed more. Chapter 11 said, in the present tense, that Grove accepts only Ravel 1.x and that unversioned `ravel` stays at 1.1.0.

The draft charter owns technical truth, so this session corrected chapter 11:

- The opening and the reason for `ravel@2` now tell the 2.0.0 transition in the simple past. They name Grove 22.9.0 as the caller that accepted 2.0.0.
- A new paragraph says that both steps are complete and that each later cut selects `CHANNEL=unversioned`. The decision record supports the statement that a later refused major can need the same two steps.
- The version paragraph says that, between two cuts, the checkout's version is that of the last release. It names no value that a cut changes.
- The baseline rules, the cut and promotion fragment labels, the promotion rule and the recovery channel now separate the 2.0.0 history from later cuts.


**The installation path outside chapters 10–11 goes to a new draft leaf, 2026-11-14.**
A search for `ravel@2` and `1.1.0` found the same pre-promotion state in chapter 3, chapter 6, the contract's examples policy and the README.
Those files are outside this leaf's goal, so the decomposition rule externalises them. Draft `rhai-book-inside-k120` stands before copy-edit k119, so this node's copy-edit, art and proof cover it.
The `## 2.0.0` section of `CHANGELOG.md` records the release as cut, so it stays.
The node brief, the root brief and the copy-edit body now name the promoted state and k120.

**L83 records the lesson, 2026-11-14.**
A transition's present-tense statements contain no “until”, so L81's search cannot find them. Read each external system that the procedure changes, then search for the values of the temporary state.

**Prose checks of the files this session wrote, 2026-11-14.**
The checker reports no new finding in the changed chapter 11 text, L83 or the new leaf. The lessons record keeps 11 earlier findings and the root brief keeps 15. This leaf does not convert those files.
This session split two semicolon sentences in the node brief.

**Every runnable task ran, and one run exposed a gap in chapter 10, 2026-11-14.**
One script ran 42 task invocations in order on macOS 27, arm64, with one log per task. It removed `RAVEL_EXIT_FILE`, its two companions and `HOMEBREW_NO_INSTALL_CLEANUP` from each task's environment.
A SHA-256 digest of 289 tracked subjects was the same before and after. The subjects were `docs/book`, `scripts`, `crates`, the Taskfile, the Cargo files, `rust-toolchain.toml`, `.cargo/config.toml` and `release.toml`.

| Tasks | Result |
|---|---|
| `setup`, `build`, `check`, `test`, `test:list`, `test:runner`, `test:cli`, `test:session`, `test:terminal-recovery` | Passed. `check` ran 713 tests, all passed, and skipped 48 |
| `rhai:upgrade-proof`, `notices`, `notices:setup`, `notices:test`, `harness:smoke:test` | Passed |
| `install PREFIX=target/install-trial-k36`, `smoke` | Passed |
| `smoke:pair` with that prefix | Refused with status 201: the prefix “is in the real HOME, which the guard denies” |
| `docs:book:build`, `docs:figures`, `docs:examples`, `docs:excerpts`, `docs:ste -- --strict`, `docs:breaks -- --strict` | Passed: 21 figures, 27 examples, 22 excerpts, no prose problem in 35 files, no long run |
| `docs:figures:test`, `docs:examples:test`, `docs:examples:mutants`, `docs:excerpts:test`, `docs:ste:test`, `docs:breaks:test`, `book:integrity`, `book:bun:integrity` | Passed |
| `native-entry:build`, `native-entry:front`, `native-entry:runner`, `native-entry:start`, `test:native-entry` | Passed |
| `release:doctor`, `release:test`, `release:archives`, `release:formula`, `release:smoke` | Passed. Docker reported `linux/aarch64` |
| `release:lifecycle:test` | Passed in 477 seconds, without `HOMEBREW_NO_INSTALL_CLEANUP`, as L80 requires |

`scripts/installed-pair-smoke.sh` refuses an `OLD` prefix, a `NEW` prefix or its temporary directory inside the real home. Chapter 10 did not say so, and its install example uses `target/install-trial` in the checkout.
Chapter 10 now states the rule after the installed-files table, with a fragment that installs `NEW` outside the home. The script reads `OLD --version` before this check, so the text says “before it starts a session”.

A second script then ran the changed paths with an unchanged digest of the same subjects:

- `install` into a scratch prefix under `/private/tmp`, outside the home. The session's rules keep its files out of `/tmp`, and both paths are outside the home.
- `smoke:pair` with the 1.1.0 keg as `OLD` and that prefix as `NEW`. It passed: the two launches overlapped, and the 1.1.0 keg and the owner's configuration did not change.
- `docs:book:build`, `docs:examples`, `docs:excerpts`, `docs:ste -- --strict`, `docs:breaks -- --strict` and `docs:figures` passed, with the same counts and 73 fragments.

The release lifecycle suite ran before the chapter 10 edit. The edit changes no file that the suite builds or reads as a fixture.
A search of the built chapters 10 and 11 found no raw fence, heading marker or `tc` comment, and each heading has its id. A planted fence line was found.

These tasks did not run:

| Tasks | Reason |
|---|---|
| `harness:smoke:claude`, `harness:smoke:codex` | Each makes a real call with the owner's sign-in and spends subscription usage. L50 requires the owner's permission for a live call |
| `release:patch`, `release:minor`, `release:major`, `release:promote` | Each pushes, tags, publishes or changes the tap. The 2.0.0 cut and promotion are complete |
| `release:notes` | It rewrites the Unreleased notes in `CHANGELOG.md` |
| `docs:figures:update` | It overwrites the committed figures. `docs:figures` compared them |
| `docs:book`, `docs:result-job`, `docs:rhai-configuration`, `docs:skills` | Each serves a page until it stops. `docs:book:build` checked the book build |
| `native-linux:image`, `test:native-linux` | Each builds a Linux image and runs the platform campaign. Neither chapter changed a platform claim |
