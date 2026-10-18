# deliver-ravel-k162

## Goal

Draft **part A** of chapter 12, as the document node's brief describes, under
`docs/book/contract.md`, into `docs/book/src/12-deliver-ravel.md`, linked from
`docs/book/src/SUMMARY.md`: versions and identity, the archive layout and
manifest, the checks before a cut, the platform floors and the installed-layout
smoke. The headings for part B are not written here.

## Context

- Chapters 1–3, 5, 7 and 11 are proved before this runs. Chapter 11 teaches
  `task install` and `task smoke`; this chapter continues from them to archives
  and floors. Link back instead of re-teaching.
- Read `docs/RELEASING.md` with the node brief's warning: its first paragraph
  is stale about versions and tags. Check every statement about versions,
  baselines and floors against `Cargo.toml`, `git tag`, `CHANGELOG.md`, the
  scripts and `docs/specs/session-execution.md` (“Installed boundary”,
  “Archives and platform floors”).
- Run what the host allows and report the rest. `task release:test` needs no
  Docker. `task release:archives` with `TARGETS=aarch64-apple-darwin` builds one
  archive. State the host, the tools found and the tools missing.
  `task release:smoke` needs Docker with linux/arm64, Zig and cargo-zigbuild; if
  they are missing, say that the chapter describes those floors from the script
  and the evidence documents and that no run was made for it.
- Settle the heading structure for the whole of chapter 12 now, so that part B
  fills sections and does not reshape the file. Write down in this leaf's log
  which headings part B owns.

## Done when

- Part A's sections exist, build and pass `task docs:book:build`, the strict
  STE check, `task docs:breaks -- --strict`, `task docs:examples` and, if the
  chapter holds an excerpt, `task docs:excerpts`.
- The log lists which tasks part A teaches and which remain for part B, and the
  destination of each `docs/RELEASING.md` section that part A covers.
- Last act: `leaf-add` a `draft` leaf (bare stem `deliver-ravel`) whose body
  states part B's scope from the node brief, the headings reserved for it and
  anything part A learned that changes it; then `leaf-add` the `copy-edit` leaf
  if no live later sibling already holds it.

## Notes

- One fresh reviewer is allowed under the leaf-wide allowance. Spend it on the
  statements of the platform floors, because a wrong floor misleads a reader who
  installs on an old system.

## Decisions (running log)

- 2026-10-18: The named structure brief is `docs/book/contract.md`.
  “The reader” names a 16-year-old software engineer. “What the reader can do
  afterwards” includes “Build, test and deliver the project”. The ordered
  section plan gives chapter 12 its delivery purpose. “Emphasis” names the
  support limits, and “Exclusions” keeps history and unsupported reuse outside
  the teaching. The document node supplies the two-part division.
- Part A settles these headings, in order: “Keep the version and pair together”,
  “Build archives before a release”, “Check the archive contents”,
  “Test the installed archive”, “Read the platform checks correctly”, and
  “Checks made for this chapter”. Part B inserts “Prepare and test the Homebrew
  formula”, “Prepare a version cut”, “Cut and deliver a release”,
  “Keep the book with its release”, and “Recover a partial release without
  another bump” before the final checks section. It extends that final section
  with its own observations. Part A emits no empty headings.
- Local workspace version is 1.0.0. `jj tag list` shows v0.1.0 and v1.0.0.
  The guide's development 0.0.0 and planned first-cut statements are historical.
  Part A teaches version versus source identity, without claiming that the
  working-copy archive is the released v1.0.0 artifact.
- `task --list` enumerates ten public release tasks. Part A teaches
  `release:archives`, `release:test` and `release:smoke`. Chapter 11 already
  teaches `install` and `smoke`. Part B owns `release:formula`,
  `release:doctor`, `release:notes`, `release:lifecycle:test`, `release:patch`,
  `release:minor` and `release:major`. The internal `release` task is not a
  separate public task. No delivery task is unassigned.
- The host has every full archive-smoke prerequisite: macOS 27.0.1 arm64,
  Bash 5.3.20, rustup rustc 1.93.1 with all three target libraries, Zig 0.17.0,
  cargo-zigbuild, and Docker 29.8.1 linux/arm64, kernel 7.0.14-linuxkit.
  The release doctor passed. Run the full unreleased smoke task, rather than
  the selected-target substitute. No cut, push, tag or permanent install is
  part of this leaf.

## Source and coverage evidence

Tier 2 graph verification used project
`Users-antony-Development-Ravel.simplify-and-componentize`, ready generation
`2026-10-18T11:30:23Z`. The bounded worker query returned 25 functions without
pagination. `verify_hello` has one inbound caller at depth 1, `converse`.
Its exact snippet and the caller's source show the three identity checks before
the evaluate frame. `build.rs` and `dispatch.sh` independently list the identity
inputs. The graph's first broad identity search was exploratory and truncated.
It supplies no exhaustive claim.

Coverage checked all relied-on product, release-script, specification and
chapter paths. `scripts/release-smoke-qemu.sh` has a parse gap at line 185.
Direct source reading covered lines 95–215, including that gap and the address,
vDSO and CPU controls. Other checked evidence paths had matching metadata and
no recorded gap. This is best-effort coverage, not proof of completeness.
Scripts, manifests and prose were read directly under the non-code fallback.

The three ADRs retain their decisions. This chapter changes no architecture.
No platform requirement comes from a current upstream web page: the statements
are about this checkout's pins, scripts and dated repository evidence.

## Guide destinations for fold-in k165

| Source statement group | Destination or owner |
|---|---|
| RELEASING opening: one version and source identity, independent of caller | “Keep the version and pair together” |
| Opening: development 0.0.0 and planned first minor | Replaced by checked workspace 1.0.0 and both local tags |
| Opening: recheck remote tags, historical tag is not a baseline | Part B, “Prepare a version cut” |
| Unreleased archive checks: host/tool/target prerequisites and setup | “Build archives before a release” |
| Unreleased checks: rebuild, clear selected-target output, final-name validation and retry | “Build archives before a release” |
| Unreleased checks: verified Bun and declarations, no installed host runtime | Version and build sections |
| Unreleased checks: Docker, pinned userlands, glibc/CPU probes, native versus emulated routes | “Read the platform checks correctly” |
| Unreleased checks: guest addresses, vDSO, ELF registration, unprivileged/no-network inputs, kernel limit | Platform section |
| Unreleased checks: own-tar extraction, direct/symlink, isolated PATH and installed cases, no skip | “Test the installed archive” |
| Layout and manifest controls: one top directory, all assets and exclusions | “Check the archive contents” |
| Layout controls: fresh worker, omission/extra/type/mode/macOS metadata controls | Archive-contents section |
| Layout controls: probe substitution at the real worker path, matching worker, probes/check tasks | Archive-contents section and chapter 11 test tasks |
| Formula preparation, every paragraph | Part B, “Prepare and test the Homebrew formula” |
| Evidence: wrapper log locations, concise summary and exact source/archive/digest/route handoff | Part B final checks/evidence teaching; raw records stay outside the book |
| Evidence: local artifacts, rebuild after source/runtime change | Build and platform sections |
| Version cuts and publication, every paragraph | Part B cut, book and recovery sections |
| Notes, every paragraph | Part B, “Prepare a version cut” |
| Recover a partial release, all seven cases and final validation limits | Part B recovery section |
| README release paragraph: matched layout, release:smoke, release:test | Opening, archive-contents and installed-archive sections |
| README release paragraph: release:formula and delivery-guide pointer | Part B formula section, then fold-in k165 redirects the pointer |
| README notices line: checkout and installation paths | Archive-contents section |
| README licence line and retired-project pointer | Remain README entry/provenance material for fold-in k165 |

## Observed release checks

The starting source revision was `5591eac9` (empty successor `f881fbbe`).
Local logs are in `target/draft-k162/`. The subject list contains 3,424 tracked
files from `crates/`, `scripts/`, Cargo manifests/lock, Taskfile, README, LICENSE,
release configuration, specifications and evidence documents.
`before.sha256` and `after.sha256` compare equal, item by item. These are tracked
source inputs, not a digest claim over every external dependency or host tool.
Generated archives are identified separately below.

- `release:doctor` passed: all reported tools, targets, host and book-line checks.
- `release:test` passed. Full wrapper log: `target/release-archives.zzXOQZ`.
- `release:lifecycle:test` passed. Full log: `target/release-lifecycle.5epT83`.
  It includes release order/refusal, preparation, build admission, doctor and
  actual cargo-release book-version controls in isolated repositories.
- `release:smoke` built all three archives and passed every direct/symlink route:
  macOS native, arm64 native Docker, arm64 QEMU Cortex-A53, x86-64 QEMU Nehalem.
  Both CPU probes died with status 132 at the floor and ran under `-cpu max`.
  All three Linux routes ran the glibc 2.17 probe and refused GLIBC_2.25.
  No selected target or route was skipped. Full log: `release-smoke.log`.
- No `release:formula`, project `release:notes`, version cut, push, tag or
  permanent installation ran. No extra local `install`/`smoke` run was needed:
  chapter 11 records those, and the archive run executes the same installed seam.

| Local archive in target/archives | SHA-256 |
|---|---|
| ravel-v1.0.0-aarch64-apple-darwin.tar.xz | a2af7200f342591df842aba2ad2fbe44eb0778f677c22503706b313f30875701 |
| ravel-v1.0.0-aarch64-unknown-linux-gnu.tar.xz | 7af502dc1d5ffc59953f083661beed8fdd6db70168ae947c1eaf352cdcc4a9f3 |
| ravel-v1.0.0-x86_64-unknown-linux-gnu.tar.xz | e0aec0bfc191c2bafb01b8b44b8586f3d76d37c03009f59ce7c9de98c1a24912 |

These are unreleased working-copy archives, not the public v1.0.0 assets.
No kernel floor, older macOS floor, native x86-64 kernel, real agent,
interactive-terminal campaign or new-reader comprehension was measured here.

## Focused reviewer

The one fresh reviewer inspected the platform-floor and installed-test claims
against scripts and controls. The claim under examination was that these
sections describe the environments actually exercised and do not imply a wider
support guarantee. A wrong floor could mislead an installer on an older host.
The reviewer received the artifact and contract, without the author's conclusion.

One finding was valid and actionable: the draft said Cargo and rustc always use
one toolchain. `pin_rust_toolchain` changes PATH, but `resolved_rustc` honours
`RUSTC`. The chapter now states that preference, the override and the doctor's
actual compiler/target-library check. This is a narrow factual qualification,
not a changed mechanism or substantive redesign. No second reviewer is needed.
No other finding was returned. The review establishes no new runtime support.

## Final validation and handoff

The whole chapter was read against its brief and source facts. The sequence
moves from identity to a local build, exact contents, installed execution and
measurement limits. A reader can build and test unreleased archives without
cutting a release. Part B's fixed headings remain in the node brief, not as
empty chapter headings. The final read clarified the common base directory of
the source-identity paths.

Fresh `task docs:book:build`, strict STE on chapter 12 and SUMMARY, and strict
prose breaks passed. The chapter has no over-limit prose run. The complete
example check passed 14 examples, with zero failures or skips and 20 fragments
not run. Part A adds four labelled shell fragments and no source excerpt.
The unchanged excerpt check also passed all eight excerpts. The final factual
and path qualifications changed no fence or example input. The release checks
above passed before the document checks.

The book-check snapshot compares equal for the enumerated files under
`docs/book`, `scripts` and `crates/ravel`. Build/STE/breaks ran again after the
final path clarification. Seven local chapter-body destinations resolve in
built HTML, and the rendered hrefs agree. Missing-file and missing-anchor
controls failed. No external link or browser view was checked in this draft.
Art and proof retain those viewing obligations. The source digests and local
logs stay under `target/draft-k162`, outside the book.

Part B and copy-edit are the next live stages under this document node. The
node remains open. No ADR changed, no guide was removed, and no parent closes.
The ten release tasks all have assigned teaching, but seven still require part
B. Coverage assignment is not completed teaching or reader comprehension.
