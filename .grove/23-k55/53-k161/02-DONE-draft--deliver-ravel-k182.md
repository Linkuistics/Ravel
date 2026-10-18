# deliver-ravel-k182

## Goal

Complete part B of chapter 12, `docs/book/src/12-deliver-ravel.md`, under
`docs/book/contract.md` and this document node's brief. Preserve part A's
useful boundary: a reader can build and check unreleased archives. Teach the
formula, release tasks, book version and recovery to complete delivery.

## Context

- Read the whole chapter, node brief and part A's log in `deliver-ravel-k162`.
  Its source map, ten-task inventory and guide destinations are handoff inputs,
  not substitutes for checking the scripts.
- Insert these settled headings, in order, before “Checks made for this chapter”:
  “Prepare and test the Homebrew formula”, “Prepare a version cut”,
  “Cut and deliver a release”, “Keep the book with its release”, and
  “Recover a partial release without another bump”. Extend the final checks
  section with your dated results. Do not reshape or cut part A.
- Teach the seven remaining public tasks: `release:formula`, `release:doctor`,
  `release:notes`, `release:lifecycle:test`, `release:patch`, `release:minor`
  and `release:major`. Re-enumerate delivery tasks from `task --list`.
  Chapter 11 teaches `install` and `smoke`.
- Verify formula template, real Homebrew method tests, temporary prefixes,
  substituted probe identities, tap `linkuistics/taps/ravel`, cargo-release
  1.1.2, `release.toml`, release order, notes and every recovery branch against
  current scripts. Cover every remaining guide statement and README release
  statement, with destinations in the log for fold-in k165.
- The guide is stale beyond its first paragraph too: its formula URLs and
  first-cut prose still mention 0.0.0. The workspace is 1.0.0 and both local
  version tags exist. Working-copy archives can have the release's version
  without its source. Recheck tags/baselines rather than copy that prose.
- R17 is part B: the single chapter 3 version line, the release replacement,
  `ready` and `released vX.Y.Z` guards, what each refuses and how to recover.
  Verify what the release actually does with the book on GitHub Pages. No
  first book release occurs in this node. Preserve chapter 10's absent status.
- Keep the support statement in “Read the platform checks correctly”. It
  distinguishes aarch64 measurements from emulated-only x86_64 Linux and
  links the native qualification issue. Do not add a conflicting second one.
- Part A ran `release:doctor`, `release:test`, full `release:smoke` and
  `release:lifecycle:test` successfully on macOS 27.0.1 arm64, with Docker
  linux/arm64 29.8.1, kernel 7.0.14-linuxkit. All four direct/symlink routes
  and their glibc/CPU controls passed. Logs and archive digests are in k162.
  Local artifacts can be rebuilt. No formula task or project release ran.
- The toolchain script prefers rustup on PATH but honours an explicit `RUSTC`.
  Do not restore part A's corrected unconditional toolchain-coherence claim.
- Do not run the three version-cut tasks, push, tag or install a public release.
  Use the doctor, isolated lifecycle checks and dry runs. Formula preparation
  can run in its private prefixes. Record what ran and what did not.

## Done when

- The whole chapter teaches delivery through recovery and covers inventory 5's
  delivery tasks, with no unassigned item. The guide destination map is complete.
- The version-line teaching agrees with chapter 3 and the real guards.
- The chapter passes book build, strict STE, strict prose breaks, examples and
  any excerpt check it needs. Source facts and measured limits remain distinct.
- Art has the specific release activity question and the installed-figure reuse
  in the node brief's Handed forward list.
- Copy-edit already stands after this leaf. Under the family rule, cut no
  duplicate successor. Retire and commit this draft before the session signal.

## Decisions (running log)

- 2026-10-18: The named structure brief is `docs/book/contract.md`.
  It names a “16-year-old software engineer” and the outcome “Build, test and
  deliver the project”. Its ordered section plan places delivery after local
  development. “Emphasis” requires plain support limits, and “Exclusions”
  keeps evidence campaigns outside the book. The document node fixes part B's
  five headings. Preserve part A and add those headings before the checks.
- The workspace version is 1.0.0, and `jj tag list` confirms v0.1.0 and v1.0.0.
  The 0.0.0 first-cut and formula-URL prose in the old guide is historical.
  Preparation output can use the current release's URL with different local
  bytes. Teach why it cannot become the tap formula.
- `task --list` again lists ten public release tasks. This draft completes the
  seven assigned to part B. `install` and `smoke` stay with chapter 11.
- The current sequential release task and its scripts do not build or deploy
  the book to GitHub Pages. `scripts/book.sh` only sets up, builds and serves
  locally, and this checkout has no `.github/` directory. The book contract's
  release-delivery requirement remains owed. Record the present limit in the
  chapter and cut a precise implementation leaf before the second-half set
  proof. It must route the resulting teaching through draft and the later
  editorial stages before that proof. Do not invent a deployment procedure or
  perform the first public book release in this draft.
- Run the doctor, the isolated lifecycle suite and formula preparation from
  part A's three archives. Preserve source and archive digests around the runs.
  The formula test uses temporary prefixes. No project version cut, tag, push,
  tap edit or permanent installation is authorised by this leaf.
- `book-release-pages-k184` now stands before set proof k163. It owns the
  Pages implementation, isolated controls and a later editorial correction
  run. Part B's activity question goes to art in the document node brief.
  Copy-edit k183 already stands, so no duplicate successor is due.
- The doctor, formula and lifecycle tasks passed. All 217 enumerated tracked
  inputs and the three archive digests compare equal before and after.
  No source or archive changed during a measurement. The source list covers
  crates, scripts, the book, Taskfile, Cargo manifests/lock, release config,
  changelog, README and LICENSE. It does not claim to freeze host tools or
  external dependency trees.

## Source and coverage evidence

Tier 2 used graph project
`Users-antony-Development-Ravel.simplify-and-componentize`, full generation
`2026-10-18T11:36:23Z`. Project status was ready. The exact `verify_hello`
search returned one result, with no later page. Its inbound depth-one trace
returned `converse`, and its exact snippet checks protocol, package version
and build ID. This supports the retained identity teaching, not an exhaustive
claim about all worker paths.

Coverage checked the release scripts, template, Ruby test, manifests, book
pages, specifications and evidence files used here. The release QEMU script
has a parse gap at line 185. Direct reading covered lines 95–215, including
the gap. Cargo.lock has unavailable graph freshness metadata, so its two
workspace package versions were read directly. The other checked paths had
matching metadata and no recorded gap. Scripts, config and prose used direct
source reads under the documented fallback. Coverage is best effort.

The Pages finding follows the complete command list in Taskfile's `release`
task, its called scripts, `scripts/check.sh`, and all three modes of
`scripts/book.sh`. No command there builds or deploys the book. The directory
check also finds no `.github/` in this checkout. This is a finding about the
checked-in procedure, not an audit of remote repository settings. Broader
`scripts/` coverage reports unrelated example/native-Linux parse gaps, which
were text-searched directly. No broad graph absence claim relies on them.

The three ADRs keep their decisions. No architectural choice changes here.
The old release evidence remains dated evidence, not current release state.
Local version tags and manifest versions were rechecked, but no project
release, remote tag or public-site validation ran in this draft.

## Delivery inventory and guide destinations

The current `task --list` again has ten public release tasks. The internal
`release` task supplies their shared procedure, not an extra public entry.

| Task | Teaching destination |
|---|---|
| `release:archives` | “Build archives before a release” |
| `release:test` | “Check the archive contents” |
| `release:smoke` | “Test the installed archive” and the platform checks |
| `release:formula` | “Prepare and test the Homebrew formula” |
| `release:doctor` | “Prepare a version cut” and the book guards |
| `release:notes` | “Prepare a version cut” |
| `release:lifecycle:test` | Preparation, recovery and dated checks |
| `release:patch` | Preparation task table, release order and recovery |
| `release:minor` | Preparation task table, release order and recovery |
| `release:major` | Preparation task table, release order and recovery |
| `install`, `smoke` | Chapter 11, linked from chapter 12's opening |

No delivery task is unassigned. The complete statement map below joins part
A's map, rather than replacing its source evidence. Locations are chapter 12
unless the row says otherwise. Fold-in k165 keeps the guides until set proof.

| Guide statement group | Final destination or disposition |
|---|---|
| Opening: one version and identity, independent of callers | “Keep the version and pair together” |
| Opening: development 0.0.0 and first minor 0.1.0 | Historical prose replaced by checked 1.0.0 and local tags |
| Opening: remote tag recheck and historical non-baseline tag | “Prepare a version cut” |
| Unreleased checks: host/tools/targets/setup | “Build archives before a release” |
| Unreleased checks: task recipe | Each task has its own teaching in this chapter, with `check` in chapter 11 and the release order |
| Unreleased checks: clearing, selected targets, final-name validation, retry | “Build archives before a release” |
| Unreleased checks: verified Bun, declarations, no host runtime | Version and archive-build sections |
| Unreleased checks: Docker, userlands, glibc, CPU, vDSO, address bounds, registered ELFs, confinement and kernel limits | “Read the platform checks correctly” |
| Unreleased checks: own-tar extraction, version, direct/symlink, private PATH, installed cases and no skip | “Test the installed archive” |
| Layout: exact prefix, assets and exclusions | “Check the archive contents” |
| Layout controls: omissions, extras, callers, types, execute bits, metadata and production comparison | Archive-contents section |
| Layout controls: hostile workers, refusal and production control, probes requirement | Archive-contents section and formula controls |
| Formula: three archives, manifest, SHA values and placeholders | Formula opening |
| Formula: actual Homebrew methods, private paths, no keg/tap, separate final brew test | Formula procedure and final paragraph |
| Formula: exact layout, symlink, SDK/static, fake agent, cleared environment, result shapes and empty tmp | Formula table and explanation |
| Formula: four native probes, real worker path, mismatch, production success and restoration | Formula control paragraph |
| Formula: preparation output cannot enter tap or release | Formula output paragraph, corrected for current-version URLs and possibly different bytes |
| Formula: fresh tagged artifacts and retained ravel-lite | Formula ending and release-order section |
| Evidence: logs, exact revision/digests/routes/controls and unmet prerequisites | Final paragraph of “Cut and deliver a release” and dated checks |
| Evidence: local artifacts and repeat after changes | Part A build and platform sections |
| Version cuts: patch/minor/major, first-cut exception and tag baseline | Preparation task table and baseline paragraph, with historical first cut qualified |
| Version cuts: tools, auth, tap location, Bash and checkout Bun | Preparation prerequisites plus part A setup |
| Version cuts: default jj workspace, described main, saved work and refusals | Preparation workspace and baseline paragraphs |
| Version cuts: lock, doctor/tap/prepare/check/preview/cut order | Release-order table and lock paragraph |
| Version cuts: cargo-release Git mutation, no push/crates.io, private runner and changelog replacement | Release configuration paragraphs |
| Version cuts: book replacement, raw/GitHub/mdBook agreement, two guards and isolated actual-tool controls | “Keep the book with its release”, recovery and dated checks |
| Version cuts: clean-tag archives, fresh dist, real formula tests, smoke before public writes | Release-order table and build paragraph |
| Version cuts: exact three archives, formula/digests/tap recheck, only ravel.rb | Release validation paragraph |
| Version cuts: Homebrew install/upgrade/test, exact prefix version and assets, no replacement of Grove/legacy tools | Release final-check paragraphs |
| Notes: meaningful prose, preserve or derive first lines, initial interval, review internal detail, no harness/model | Preparation notes paragraphs |
| Notes: task, explicit input, nonempty body/no level-two heading, preserved versions, refusal cases and jj commit | Preparation notes fragment and text |
| Recovery opening: no second bump, inspect states, new-cut refusals and lock recovery | Recovery opening, inspection fragment and lock paragraph |
| Recovery 1: failure before writes | Recovery table's first row |
| Recovery 2: partial version/replacement/commit/tag, no repeated replacement | Local-cut recovery paragraphs, including book-only repair and exact identity |
| Recovery 3: clean exact source, main repair, build/formula/smoke, source-fix identity | Build-recovery paragraphs and fragment, with tagged book guard added |
| Recovery 4: missing main/tag push and remote commit agreement | Push-recovery paragraph |
| Recovery 5: absent/existing GitHub release, only missing assets, draft status, byte-checked tap-only | GitHub recovery, tap-only fragment and its checks |
| Recovery 6: preserve local tap work, jj/Git branches, only missing push and remote formula | Tap-recovery paragraph |
| Recovery 7: local Homebrew repair, exact version, layout and assets | Installation-recovery paragraph |
| Recovery ending: isolated fixture coverage and public-state limits | Final recovery paragraph and dated checks |
| Guide's stale k11 future-publication wording | Removed as historical task prose, not a reader procedure |
| README release teaching: matched layout, smoke/floors and manifest checks | Opening and part A sections |
| README release teaching: formula checks and delivery guide pointer | Formula section, with pointer replacement owned by fold-in k165 |
| README notices line | “Check the archive contents” |
| README licence and retired-project pointer | Remain entry/provenance material for k165, outside chapter teaching |

The Pages contract is a separate current gap with an owner, k184. No guide
statement establishes that missing deployment. The book-version tests check
the replacement and guard, not a rendered or deployed book.

## Observed checks

The source parent is `c4883ee8`, part A's sealed draft. Local records live in
`target/draft-k182/`: subject list, before/after digests, task list, doctor,
formula and lifecycle outputs. Archives retain k162's three digests unchanged.
The generated formula SHA-256 is
`6dd24212cc1963021133a084e6c29bee89f645cedb5434b66bc7653d810b4e21`.

- The doctor passed on Darwin arm64 with Bash 5.3.20, rustup cargo/rustc
  1.93.1, cargo-release 1.1.2, all three target libraries, Zig 0.17.0,
  checkout Bun 1.4.2 and arm64 Docker kernel 7.0.14-linuxkit.
- Formula preparation passed. Probe-build log: `target/worker-check.dOYrkn`.
  Full Homebrew method/control log: `target/release-formula.Wby6iU`.
  All four result shapes returned unchanged, and all four substituted
  workers refused with the expected identity code. The successful result
  sessions also emitted stop-attempt EPERM reports on stderr. These are
  reported observations, not failed results or a new platform qualification.
- Lifecycle checks passed. Full log: `target/release-lifecycle.KQGzHr`.
  This covers release order/refusal, preparation and notes, build checks,
  doctor controls, and actual cargo-release book-line previews and cuts.
  Tests use isolated repositories, local remotes and service substitutes.
- No project notes replacement, version cut, tag, push, tap edit, permanent
  install or Pages deployment ran. Part A's full archive smoke results remain
  dated part A results, not a new smoke run attributed to this leaf.

## Focused review and final validation

The one fresh reviewer inspected the formula, release order, book guards and
all recovery branches against the scripts and fixtures. The question was
whether an instruction could cause a repeated cut, incorrect release identity
or service change in the wrong order. The reviewer received the chapter and
contract without an author verdict. It returned no concrete findings, so no
finding classification or second review was needed. Its read-only source
inspection establishes neither public service state nor manual recovery.

The final whole-document read preserves part A's text before the final checks
section byte for byte. The five agreed part B headings follow its archive
boundary in order. The reader moves from local formula validation to release
preparation, the exact procedure, book identity and conditional recovery.
All ten delivery tasks have teaching. No guide is removed in this leaf.

Fresh checks passed: book build, strict STE, strict prose breaks, all fourteen
complete examples (zero failures/skips), and all eight source excerpts.
Part B adds six labelled shell fragments and no complete example or excerpt.
The example checker reports 26 fragments not run, with no page errors.
The enumerated book, script and Ravel source inputs compare equal before and
after the document checks. Local logs remain under `target/draft-k182/`.

All eight local chapter-body links resolve in the built HTML, and each
rendered href agrees with the Markdown target. A missing-file control and a
missing-anchor control both failed. Raw Markdown supplied the whole-document
view. No browser/editor preview, GitHub rendering, external-link check or Pages
view ran here. Art and proof retain their viewing work. No new reader or
platform result follows from these checks.

Art has the installed-layout reuse and release activity question in the node
brief. The specific Pages implementation gap has live owner k184 before set
proof, followed by its required editorial correction. Copy-edit k183 already
stands, so this draft cuts no successor. The document and book nodes stay
open. No ADR decision changes, and no parent cascade is due.
