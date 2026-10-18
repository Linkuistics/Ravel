# book-release-pages-k184


## Goal

Implement the book contract's release-to-GitHub-Pages path. Build the book from
the exact release source, preserve its checked version, and make deployment
and recovery part of the release procedure. Test the path without a public
release. The first public book release remains outside this book node.



## Context

- Found by draft `deliver-ravel-k182`. Read `docs/book/contract.md`, section 3,
  the book node brief and chapter 12's “Keep the book with its release”.
  This is an agreed delivery requirement, not a new hosting choice.
- At discovery, `Taskfile.yml`'s sequential release task checks chapter 3's
  tagged version line but does not build or deploy the book. Its script calls
  cover doctor, preparation, product checks, version cut, archive build/smoke,
  GitHub assets, tap and local installation. `scripts/book.sh` has setup, build
  and local serve modes only. The checkout has no `.github/` directory.
- Chapters 3 and 12 must agree with the delivered mechanism. Chapter 3 already
  directs readers to the book for their installed release on GitHub Pages.
  Chapter 12 currently states the implementation gap. Decide and test how a
  reader reaches the right release's book, with the contract retained.
- Keep the pinned mdBook and figure toolchain. The book stays out of product
  archives and worker identity. Keep ordinary `task check` independent of the
  book toolchain, as toolchain leaf k58 established.
- Use the existing Taskfile for reusable work. Check current GitHub Pages
  requirements against official sources. Preserve the order that puts every
  archive smoke route before public release changes. Define the deployment
  boundary and partial-failure recovery explicitly.
- No public version cut, project tag, push, tap edit, Pages deployment or
  permanent installation is authorised by this leaf. Use isolated fixtures and
  dry runs. Report separately any remote configuration needed at first release.
- This leaf precedes set proof `book-second-half-k163`. After implementation,
  insert a contiguous chapter-12 correction run before that proof: draft,
  copy-edit, art, proof, each with bare stem `deliver-ravel`. The draft updates
  the release order, Pages instructions, recovery and task inventory from the
  actual implementation. Reconcile chapter 3's statement through its editorial
  owner if the mechanism changes its teaching. Do not let set proof absorb
  missing drafting. Observe the editorial family's re-run escalation rule.

## Done when

- The release builds the exact tag's book and delivers it under the agreed
  Pages contract. A reader can identify and reach the book for their release.
- Executable controls reject an incorrect version/source, a failed build,
  premature deployment and an incomplete delivery. Recovery resumes the
  intended release without another bump or a concealed identity change.
- Tests run without public-service writes and preserve the current archive,
  formula, release-order and book-version controls. Record any remote checks
  that fixtures cannot establish.
- The implementation's teaching and figure changes have a complete editorial
  correction run before set proof. The book node brief carries any exact
  first-release setup or external limitation, without claiming a deployed site.

## Decisions (running log)

- Delivery layout: branch `gh-pages` served from `/`, one immutable directory
  `vX.Y.Z/` per release, a root `index.html` listing releases newest first, and a
  root `.nojekyll`. The reader reaches the book at
  `https://linkuistics.github.io/Ravel/vX.Y.Z/` from the version `ravel --version`
  prints. Why a branch: Pages publishes a branch as it stands
  (docs.github.com/en/pages/getting-started-with-github-pages/configuring-a-publishing-source-for-your-github-pages-site),
  and a workflow deployment replaces the whole site with one artifact, so it would
  drop the earlier releases' books. The contract says the site matches a released
  Ravel, and an installed older release still needs its own book. The repository
  has no `.github/`, and none is added.
- The book is built from `git archive` of the tag's commit, not from the working
  copy, with `book.sh build-tree` (the pinned mdBook, the same error and warning
  refusal) and `--dest-dir` (mdBook 0.5.4 `guide/src/cli/build.md`). The staged
  book has an identity file (commit, mdBook version, tree digest) beside it, never
  published. `publish` refuses a stage built from another commit than the tag has
  now, or changed after its build.
- Order: `build` after the cut's version guard and before the archive build, so a
  book failure stops with the cut local. `publish` inside `release-publish.sh`
  after the GitHub Release and before the tap, because the tap is what lets a
  reader install and the reader then looks for the book. `--tap-only` verifies the
  book first, so a resume keeps the order. `--book-only` is the new resume mode.
  `live` is the last Taskfile step because Pages shows a pushed branch after a
  delay. No Taskfile task was added: the public ten stay, chapter 12's inventory
  does not change, and recovery calls the scripts as it does for the tap.
- Immutability: a `vX.Y.Z/` already on the branch with the same bytes is a resumed
  publish (no commit); with other bytes it is refused. Recovery needs no bump and
  hides no identity change. The push is a plain push, never forced.
- Premature deployment is refused by two gates in `publish`: the remote has the tag
  at the staged commit, and the GitHub Release is complete (`published_release_complete`).
- Pages limits from docs.github.com/en/pages/getting-started-with-github-pages/github-pages-limits:
  1 GB site, soft 100 GB/month, soft 10 builds/hour. `publish` refuses a site over
  1 GB. A real book is about 7 MB per release (measured in the rehearsal).
  Symbolic links need a workflow deployment per the publishing-source page, so the
  build refuses them.
- The `.nojekyll` file: the official pages fetched do not mention it. The Jekyll
  page says Jekyll skips paths that start with `_`, `.` or `#`. mdBook writes its
  own `.nojekyll` in each output and the script writes one at the site root.
  Training-data knowledge says the root file turns Jekyll off. Not verified against
  an official source, and `verify` requires the file.
- Doctor: checks the pinned mdBook, and reads the Pages setting with
  `GET /repos/{owner}/{repo}/pages` (docs.github.com/en/rest/pages/pages). Not
  enabled (404) is a note, the state before the first release. A source other than
  `gh-pages` at `/` is a failure.
- First-release remote setup (stated in `docs/RELEASING.md`, done by the owner, not
  by the release): the first `publish` creates `gh-pages`; Pages cannot be enabled
  before the branch exists; enable it as "Deploy from a branch", `gh-pages`, `/`.
  Observed on 2026-10-18 by read-only calls: the repository is public, has no Pages
  site and no `gh-pages` branch. Remote checks that fixtures cannot make: the
  served address (an organisation domain setting would change it), the create
  endpoint call with the operator's rights, the Pages delay, and GitHub's rendering.
- 404.html in each release directory has `<base href="/">` (mdBook default
  site-url). Pages serves only a root 404.html on a project site, so these files are
  never served as error pages. Left as built.
- Mutation controls seen to fail: public-tag gate, release gate, stale commit,
  stage digest, immutability, size limit, verify directory, rendered version block,
  root `.nojekyll`, and the three Taskfile order assertions. The first sort of the
  release list was ascending because `-r` does not apply to keys with their own
  options; the newest-first control caught it.
- Chapter 12 and the book node brief: a correction run `deliver-ravel` (draft,
  copy-edit, art, proof) stands before set proof k163. Chapter 3's sentence "Each
  release puts its own book there" gains the address through chapter 3's owner
  inside that run's draft, if the draft judges that the teaching changed.

## Notes
