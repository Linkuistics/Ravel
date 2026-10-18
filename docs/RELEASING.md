# Independent Ravel delivery

Ravel's front and private worker have one Ravel version and source identity,
independent of any caller's version. Development is 0.0.0; the first minor
release is planned at 0.1.0. Recheck remote version tags before a cut. The
retired daemon bookmark and `structured-plan-migration` tag are historical
provenance, not Ravel release-version baselines.

## Unreleased archive checks

Run from the Ravel checkout on an Apple silicon Mac with Bash 4 or later,
rustup/Rust 1.85 or later, Task, curl, tar/xz, ShellCheck, Zig and
cargo-zigbuild. Install all supported Rust targets:

```sh
rustup target add aarch64-apple-darwin aarch64-unknown-linux-gnu x86_64-unknown-linux-gnu
task setup
task release:test
task release:smoke
task release:formula
task check
```

`task release:smoke` first builds fresh archives. `task release:archives`
builds them without running smoke. Either build clears `target/archives`,
including on a selected-target build (`TARGETS=aarch64-apple-darwin`), so
earlier target artifacts cannot fill in a failed run. Each archive is exposed
under its final filename only after exact-manifest validation. Rerun a failed
unreleased build from the task; it cuts no version and publishes nothing.

The worker build verifies each Bun 1.4.2 runtime against committed SHA-256
pins, builds with that verified private copy and emits generic declarations.
Runtime pins, source findings and upgrade rules are in
[runtime evidence](runtime-evidence.md) and the production
[build script](../crates/ravel/scripts/dispatch.sh). There is no host Bun/Node
requirement for the installed pair; the checkout-local runtime is a build tool.

Linux tests require an available **linux/arm64 Docker** with kernel 6.7 or
later. The scripts fetch pinned CentOS 7.9 userlands, QEMU and an Ubuntu helper.
The userlands contain glibc 2.17. Native arm64 Docker tests run first, then
both Linux targets run through a private binfmt_misc registration at their CPU
floors: Cortex-A53 for arm64, Nehalem for x86-64. Floor probes run, newer-glibc
probes refuse, above-floor instructions raise SIGILL at the floor and run
under the twin `-cpu max` interpreter. The guests must have a vDSO and match
the supported virtual-address-space bounds. Every archive ELF, including the
front and worker, must match the emulator registration. Guests have no network,
read-only inputs and unprivileged execution. Kernel-version floors are not
measured by containers or user-mode emulation.

The macOS archive runs natively. Every environment extracts with its own tar,
checks the version and runs the same [installed-layout seam](../crates/ravel/scripts/installed-smoke.sh),
directly and through a relative symlink. Its private utility PATH excludes
Bun, Node and Grove. It tests the SDK, static/sample/choice, exact ordinary,
empty/multiline/flag-like results, child output separation, TypeScript helpers,
declared packages, measured context, params and stale context refusal.
Missing tools or unrun targets are failures; there is no skip path.

## Layout and manifest controls

An archive's only top directory is `ravel-v<VERSION>-<TARGET>/`, an installation
prefix with `bin/ravel` and `libexec/ravel/ravel-policy`. It also carries the
generic SDK source/declarations, static example source/declarations, editable
sample, Bun notices, README and LICENSE. The complete file/directory manifest
is [release-common.sh](../scripts/release-common.sh). It permits no caller
binary, caller adapter, owner policy, choice, development dependency or probe.

`task release:test` compares the manifest to a fresh production worker build,
then fires omission controls for every file, extra-file/directory controls,
caller-asset controls, executable-mode and symlink/type controls and macOS
AppleDouble/xattr controls. The existing hostile runtime suite additionally
substitutes every probe at the real worker path, requires identity refusal
before policy execution, and exercises the matching shipped worker there as
the firing control. Probe tests require `task probes`; `task check` builds them.

## Formula preparation

`task release:formula` requires all three current-version archives. It checks
each exact manifest, renders all target SHA-256 values and rejects unexpanded
placeholders. It then loads the rendered formula through the installed Homebrew
Ruby library and executes its **actual install and test methods**. Only prefix
and testpath are redirected to owned temporary directories; this creates no
Homebrew keg and changes no tap. Homebrew is therefore a prerequisite for
formula validation. This preparation test is distinct from `brew test ravel`
after publication/installation.

The installation must match the archive's bin/libexec directories exactly.
The formula test reaches the front through a symlink, imports Ravel's SDK and
static example, routes to a shell fake agent and round-trips ordinary, empty,
multiline and flag-like opaque results on a scrubbed environment. It leaves
no temporary session resources. It launches no model and installs no host
TypeScript runtime. The formula has no Grove dependency.

The preparation task builds all four native probe workers and substitutes each
at the archive-derived installed worker path after the formula test. Every one
must refuse with `worker_identity_mismatch`; the normal formula test establishes
the matching production worker's success first. The private worker is restored
after the controls; no installed tool outside that temporary prefix is touched.

The rendered `target/archives/ravel.rb` is **unpublished preparation output**;
its 0.0.0 archive URLs are not downloadable releases. Do not copy this file into
a tap or publish these archives. Formula publication uses a version cut and
fresh tagged-source archives; `ravel-lite.rb` is a different command and must
remain intact. The lifecycle tasks below own cuts and publication; these archive/formula
preparation tasks perform none of those operations.

## Evidence

The check wrapper saves full logs under `target/release-archives.*` and
`target/release-formula.*` and reports a concise pass/fail summary. Build and
floor-smoke output is suitable for a saved full log, with one target/control
record per execution. A handoff must name the exact source revision, archives
and SHA-256 digests, individual target routes and firing controls, complete
logs and any unmet prerequisites. Archives are local artifacts, not durable
published URLs; rebuild and rerun after source or runtime changes.

## Version cuts and publication

The independent root tasks `release:patch`, `release:minor` and `release:major`
run the actual sequential release pipeline. The first cut is **0.0.0 → 0.1.0**
with `task release:minor`; preparation permits an absent `v0.0.0` only when
neither local nor remote version tags exist. Recheck tags immediately before
starting. The historical daemon tag is not a version baseline.

Additional prerequisites are jj, jq, Git, authenticated `gh` with release access
to `Linkuistics/Ravel`, cargo-release **1.1.2**, a clean synchronized clone of
`linkuistics/taps`, and Homebrew. Set `RAVEL_TAP_DIR` if that clone is not
`~/Development/homebrew-taps`. Put Homebrew Bash before `/bin` on PATH: release
builds require Bash 4 or later, and the doctor checks this before a cut. Run
`task setup` for the verified checkout-local Bun; a host Bun is unnecessary.

Run from Ravel's **default colocated jj workspace**, with intended changes
already described and on local `main`. Unrelated working changes remain in
jj: preparation prints their stable change ID before creating a clean child of
main. Restore them later with `jj edit <saved-change-id>`; never abandon them.
A failure after preparation also leaves the saved change intact. Preparation
fetches origin, rejects divergent/conflicted/undescribed main, version work
outside main, unintegrated version tags and unfinished prior publication.

```sh
task release:doctor
scripts/release-publish.sh --check-tap
git ls-remote --tags origin
task release:lifecycle:test
task release:minor
```

The task acquires `.jj/release-lock`, then runs doctor, tap preflight, a check
that the Pages remote answers, preparation, a second tap check, full check,
cargo-release dry preview and the local executed cut. Cargo-release is the documented Git mutation exception;
it creates the version commit/tag and never pushes or publishes crates.io.
Only `ravel` owns changelog replacement; the private runner is release-disabled.
The anchored replacement requires exactly one `## Unreleased` heading, closes
it under `## v<VERSION>` and seeds a fresh empty standing heading. Config and
replacement controls are tested with the actual 1.1.2 tool in isolated repos.

The same cut writes the release version into the one version line of the book,
`ravel @RAVEL_BOOK_VERSION@` in `docs/book/src/03-a-complete-session.md`
(`release.toml`). The line holds the placeholder until the first release, and
the previous version after it, so the next cut finds it again.
`task release:doctor` refuses a book that does not have exactly one such line.
After the cut, and before the build, the task runs
`scripts/release-book-version.sh released vX.Y.Z`. It reads the page at the tag
and stops the release if the line is not `ravel X.Y.Z`. Then the cut is local
and nothing is pushed. Recover as "Recover a partial release without another
bump" says. `scripts/release-book-version.test.sh` runs the real cargo-release
in an isolated repository: a dry run, two cuts and the controls that must fail.

The task imports the release commit onto main and builds the book from the tag
(see "The book on GitHub Pages"). It then builds all three clean-tagged
archives into freshly cleared `target/dist`, and renders/tests their formula
through the real Homebrew API in private prefixes. It then smokes **every target
at its floors before** pushing main, pushing the tag, creating GitHub's release,
adding the book to Pages or writing the tap. Publication goes in this order: the
GitHub Release, then the book, then the tap. It rechecks tap
cleanliness/synchronization, exact three archives and the formula's
version/digests, and it rejects extra archives. The formula changes only
`Formula/ravel.rb`; `ravel-lite.rb` stays intact.

After publication the task installs/upgrades and tests `ravel`, checks the
Homebrew-prefix binary's exact version and checks the GitHub assets. Ravel's
formula test exercises its real matching worker and opaque-result round trips.
This installs only Ravel; it does not replace Grove or legacy harness-dispatch.
Preparation leaves those existing tools and their live session authority alone.
The last step reads the book from the Pages site itself.

## The book on GitHub Pages

Each release publishes its own book. The book for release `X.Y.Z` is at
`https://linkuistics.github.io/Ravel/vX.Y.Z/`. The site root lists every
release, newest first. A reader finds the right book from the version that
`ravel --version` prints.

GitHub Pages serves branch `gh-pages` from its root. The branch holds one
directory per release, `vX.Y.Z/`. It also holds a root `index.html` and an empty
`.nojekyll`. A release adds its directory and never changes one that is there.

A deployment by workflow replaces the whole site with one artifact. That would
drop the earlier releases. The branch needs no workflow, because Pages publishes
it as it stands
([publishing source](https://docs.github.com/en/pages/getting-started-with-github-pages/configuring-a-publishing-source-for-your-github-pages-site)).

`scripts/release-book.sh` does the work in four steps:

| Step | When | What it refuses |
|---|---|---|
| `build vX.Y.Z` | After the cut and its version guard. Before the archive build | An absent tag. A page at the tag whose version line is not `ravel X.Y.Z`. An mdBook error or warning. A built version page that does not show `ravel X.Y.Z` in its code block, or that has the placeholder. A symbolic link. Files that differ from the tag's `docs/book` |
| `publish vX.Y.Z` | Inside `scripts/release-publish.sh`. After the GitHub Release and before the tap | A missing staged book. A staged book built from another commit than the tag has now. A staged book that changed after its build. A tag that is not on the remote at that commit. A GitHub Release that is absent, a draft or incomplete. A `vX.Y.Z/` on the branch with different bytes. A site over the Pages limit |
| `verify vX.Y.Z` | At the end of `publish`. Before a resumed tap publication | A branch without `.nojekyll`. A branch without the directory. A directory that differs from the staged book. An index that omits the release |
| `live vX.Y.Z` | Last step of the release task | A Pages site that does not serve the version page with `ravel X.Y.Z`. A root page that does not list the release. It retries, because Pages shows a pushed branch after a delay |

`build` exports the book source from the commit at the tag with `git archive`.
The working copy takes no part. The book carries the version line that the cut
wrote.

The staged book is `target/book-release/vX.Y.Z/`, with
`vX.Y.Z.identity` beside it. The identity file names the commit, the mdBook
version and a digest of the tree. It is never published. `publish` pushes
without force, from a scratch clone. It never touches the checkout or its jj
state. The push takes the whole commit or nothing.

`check-remote` and `check-pages` change nothing. The doctor runs both. The
doctor accepts Pages that serves `gh-pages` from `/`. It records Pages that is
not enabled as a note, because that is the state before the first release. It
refuses any other source, because Pages would not serve the book.

### Set up Pages for the first release

The repository has no `gh-pages` branch and no Pages site until the first
release that carries the book. The release does not do the two steps that
remain. A person with administrator rights does them:

1. Let the first `publish` create the branch `gh-pages`. Pages cannot be
   enabled for a branch that does not exist.
2. Enable Pages with the source "Deploy from a branch", branch `gh-pages` and
   folder `/ (root)`. Use the settings page or the
   [REST endpoint](https://docs.github.com/en/rest/pages/pages):
   `gh api -X POST repos/Linkuistics/Ravel/pages -f build_type=legacy -f 'source[branch]=gh-pages' -f 'source[path]=/'`.

Until step 2, the release task ends at its last step with "did not serve".
Everything else is done by then. Do step 2. Then run
`scripts/release-book.sh live vX.Y.Z`. Do not publish the release again.

GitHub documents these Pages limits:

- A published site is at most 1 GB.
- The soft limit is 100 GB of bandwidth a month.
- The soft limit is 10 builds an hour.

Each release adds about 7 MB. `publish` refuses a site over the size limit.
Fixtures cannot establish three things, and they stay open until the first
release:

- The address above is the one that GitHub serves. A domain setting of the
  organisation would change it.
- The endpoint call above works with the rights of the release operator.
- Pages shows the branch within the time that `live` waits.

## Notes

Maintain meaningful prose under `## Unreleased`. Preparation preserves it
byte-for-byte. If the body is empty, it fills it from nonempty commit first
lines since the current version tag, then commits the notes to main through
jj. The initial absent-tag interval is repository history through main; review
that fallback because descriptions can contain internal maintenance detail.
There is no Grove, Ravel-session, model or credential-grant launch involved.

`task release:notes` fills only empty notes. For reviewed replacement prose:

```sh
scripts/release-notes.sh --to main --from-file /absolute/path/reviewed-body.md
```

The file must contain a nonempty section body with no level-two heading.
Versioned sections are preserved. Missing/whitespace-only/invalid input,
missing or duplicate Unreleased headings, an ambiguous endpoint and a
changelog differing from the fixed endpoint refuse without modifying it.
Commit reviewed notes through jj before cutting. A missing CHANGELOG.md is an
error rather than an implicit historical rewrite.

## Recover a partial release without another bump

**Never rerun `release:patch`, `release:minor` or `release:major` after any cut
step partly succeeds.** Inspect `jj st`, `jj log`, `git status --short`, local
and remote version tags, GitHub's draft/assets and tap state. Version changes
outside main, a local-only/unintegrated tag, missing/draft/incomplete GitHub
assets or an unpublished tap version refuse a new release. An acquired lock
blocks another task; on normal success/failure Task removes it. After a killed
Task, confirm no release process remains before `rmdir .jj/release-lock`.

1. **Preparation failed before version writes:** fix the named prerequisites,
   notes or main/tap synchronization. Confirm version/manifests/tags are still
   uncut before restarting the task. The saved working change remains in jj.
2. **Cargo-release partly wrote manifests/changelog or committed without a tag:**
   retain every write and inspect the dry preview/log. Finish the current version,
   never invoke the bump again. Use `cargo release replace --execute --no-confirm`
   only if the current version's changelog heading is absent and the standing
   Unreleased body still contains its notes. Do not run replacement twice.
   If the version write is incomplete, reconcile all workspace manifests and
   Cargo.lock to the already selected version, then check them. Commit the
   coherent release through jj with `jj describe -m 'chore: release v<VERSION>'`,
   `jj bookmark set main -r @`, `jj new main`; create only the missing local
   release tag with `git tag -a v<VERSION> -m 'Release v<VERSION>'` at its commit.
   Tag operations are the standing Git exception. Inspect an existing tag;
   never overwrite it to conceal a mismatch.
3. **Local cut/tag exists but build, formula or smoke failed:** return to a clean
   child of the exact release commit (`jj new <release-commit>`), move main there
   if the cut failed before that step (`jj bookmark set main -r <release-commit>`),
   fix environment prerequisites, then run:

   ```sh
   scripts/release-build.sh
   scripts/release-smoke.sh --archives target/dist
   ```

   The build validates tag/version/cleanliness before clearing dist. It rebuilds
   all archives, probes and formula; smoke executes every target, with no skips.
   If the book build failed, fix its cause as a described commit only when the
   release identity allows it, then run `scripts/release-book.sh build v<VERSION>`
   again. It always starts from nothing, so a stale staged book cannot stand in.
   A source fix needs its own described commit and an explicitly resolved release
   identity; do not silently move an already public tag. Nothing is pushed until
   all installed smoke routes pass.
4. **Build/smoke passed but a main/tag push failed:** retry only the missing
   `jj git push -b main` or `git push origin v<VERSION>`. Check the remote peeled
   tag matches the release commit before continuing; do not force a public tag.
5. **GitHub creation or uploads partly succeeded:** inspect
   `gh release view v<VERSION> --repo Linkuistics/Ravel --json isDraft,assets`.
   If absent, run `scripts/release-publish.sh` after the validated smoke. If it
   exists, upload only missing current archives with `gh release upload
   v<VERSION> <missing-archive> --repo Linkuistics/Ravel`; inspect existing files
   instead of overwriting them. Publish an intended draft with `gh release edit
   v<VERSION> --repo Linkuistics/Ravel --draft=false` only after all three assets
   are complete. Then run `scripts/release-publish.sh --book-only`, and after it
   `scripts/release-publish.sh --tap-only`. The second mode downloads all three
   assets and compares each byte to local validated archives, and it verifies the
   book on the Pages branch, before editing the tap. Neither creates a second
   GitHub release. If the book step failed and the release is complete, only
   `--book-only` and `--tap-only` remain. The book step pushes the branch whole
   or not at all. A repeat finds the same bytes and adds no commit. A different
   book under an existing `v<VERSION>/` is refused. Never replace a published
   release directory by hand to conceal a mismatch. If the staged book is gone,
   run `scripts/release-book.sh build v<VERSION>` first.
6. **Tap copying/commit/push partly succeeded:** inspect and retain the local tap
   work. If no tap write occurred, the tested `--tap-only` mode resumes it. If
   copying occurred, finish only that formula: in a jj tap describe the intended
   change, move main onto it, create its empty successor and `jj git push -b main`.
   If commit already succeeded, retry only its push. Git-only taps use the
   equivalent Git commit/push. Check remote `Formula/ravel.rb` and retain every
   unrelated formula; do not rerun the all-in-one publisher against a dirty tap.
7. **Publication passed but installation/verification failed:** retry only
   `brew update`, `brew install linkuistics/taps/ravel` (or `brew upgrade ravel`),
   `brew test ravel`, and `"$(brew --prefix)/bin/ravel" --version`. Inspect matching
   worker layout and GitHub's exact three assets. Do not cut another version to
   repair a missing local installation.
8. **Everything is published but the Pages site does not show the book:** the
   last step, `scripts/release-book.sh live v<VERSION>`, failed. Do not publish
   again. Check that the branch holds the book with `scripts/release-book.sh
   verify v<VERSION>`. Check that Pages serves `gh-pages` from `/`
   (`scripts/release-book.sh check-pages`, or the first-release setup above).
   Then repeat only `live` until the site answers.

Recovery admission, refusal/order and tap-only download comparison run against
isolated local repositories/services in `task release:lifecycle:test`; no real
cut, tag, GitHub/tap/Pages publication or permanent installation is part of
preparation validation. The book's build, staging, publication to a local bare
remote, verification and live read (from a loopback server) run there with the
pinned mdBook, so that suite needs `task setup`. Actual public recovery depends on the state that k11 observes.
