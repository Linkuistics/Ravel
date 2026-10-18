# deliver-ravel-k190


## Goal

Proof the whole chapter 12 again, and chapter 3 if the draft changed it, after the
Pages correction. Check technical truth, STE, figures, examples, entry routes and
the release-task inventory. Leave all required checks green.

## Context

- Read the document brief, the run's logs in `deliver-ravel-k187`,
  `deliver-ravel-k188` and `deliver-ravel-k189`, and `book-release-pages-k184`.
- Check the Pages teaching against the scripts, `Taskfile.yml` and
  `docs/RELEASING.md`, and the figure order against the Taskfile. Enumerate the
  release tasks again from `task --list`.
- Check that nothing claims a deployed site, and that the first-release setup and
  the open remote checks are stated. The first public book release stays outside
  this node. This proof does not discharge that.
- This is the chapter's second proof. A defect that a stage owns goes to a re-run
  leaf only if this is that stage's first re-run against chapter 12. A second
  re-run of one stage is an escalation.

## Done when

- The whole chapter passes its contract and the required checks, with the task
  inventory enumerated again from the subject.
- The figure, the caption, the recovery counts and chapter 3's statement agree.

## Decisions (running log)

- The whole-document read found one recovery sentence that needs a condition.
  After a failed first push, a retry must still create and send the book's
  branch commit. Only a branch that already holds the same book needs no new
  commit. Correct that sentence in proof's whole-document charter. This is a
  local clarification against `publish` in `scripts/release-book.sh`, not a
  second draft correction run or a change to release behaviour.

### Whole-document checks

The whole chapters 12 and 3 were read against `docs/book/contract.md`, the
shared STE decisions, glossary, specifications and three cited ADRs. The
release scripts and Taskfile establish the source facts. The archive
manifest, pair identity, platform routes, formula checks, version cut,
book modes, first-release setup and recovery agree. The one correction
above leaves the six-sentence paragraph within the STE limit.

The figure's tagged-book build precedes archives and public writes. Its
GitHub release precedes the Pages branch push and `verify`, then the tap.
Installation precedes the last `live` check. The eight grey exits agree with
the recovery table, alternative and key. Chapter 3 retains its one version
line, installation anchor and release-list route, with the same explicit
pre-first-release limit as chapter 12.

A fresh `task --list` supplies this complete public release inventory:

| Task | Chapter 12 destination |
|---|---|
| `release:archives` | Build archives before a release |
| `release:smoke` | Test the installed archive |
| `release:test` | Check the archive contents |
| `release:formula` | Prepare and test the Homebrew formula |
| `release:doctor` | Prepare a version cut |
| `release:notes` | Prepare a version cut |
| `release:lifecycle:test` | Prepare a version cut; recovery and dated checks |
| `release:patch` | Prepare a version cut; Cut and deliver a release |
| `release:minor` | Prepare a version cut; Cut and deliver a release |
| `release:major` | Prepare a version cut; Cut and deliver a release |

`install` and `smoke` retain chapter 11's teaching, linked in the opening.
The internal `release` task is not public. No release task is missing.
K187's added guide-statement destinations remain valid for k165. The guide
still differs from source about doctor preflights, export byte comparison
and custom workflows. The chapter teaches the implemented split, file-count
guard and absence of a custom workflow file. This proof does not migrate
the guide ahead of its owner.

Fresh official GitHub pages on 2026-10-18 UTC support the branch setup,
`.nojekyll`, GitHub-managed workflow, limits and create-endpoint parameters:
[source settings](https://docs.github.com/en/pages/getting-started-with-github-pages/configuring-a-publishing-source-for-your-github-pages-site),
[limits](https://docs.github.com/en/pages/getting-started-with-github-pages/github-pages-limits),
and [Pages REST](https://docs.github.com/en/rest/pages/pages#create-a-github-pages-site).
The book already links these sources. The book brief's obsolete `.nojekyll`
uncertainty is reconciled. This establishes no deployed site or successful
administrator call. The local `v1.0.0` tree has `Cargo.toml` and no `docs/book`.
The `ready` guard passes on chapter 3's unchanged version line.

### Verification and limits

The Tier 2 graph was ready. Coverage generation `2026-10-18T12:53:52Z`
reported matching metadata and no recorded gap for the ordinary evidence
paths. The SVG is excluded by suffix, Cargo.lock has no tracked graph hash,
and `scripts/release-smoke-qemu.sh` has a parse gap at line 185. Direct source
reads cover the lockfile's package versions and the reported arithmetic line.
Figure regeneration and browser inspection cover the SVG. Coverage remains
a best-effort signal, not a completeness guarantee.

Final validation after all chapter edits passed on 2026-10-19
(Australia/Melbourne):

| Check | Result |
|---|---|
| `task docs:book:build` | Exit 0, no error or warning |
| `task docs:ste -- --strict` | Zero problems in 28 files |
| `task docs:breaks -- --strict` | No long prose run in eleven chapters |
| `task docs:figures` | Fourteen figures match |
| `task docs:examples` | Fourteen passed, zero failed or skipped; 27 labelled fragments not run |
| `task docs:excerpts` | Eight passed, zero failed |
| `task release:lifecycle:test` | Passed; full log `target/release-lifecycle.fyROlp` |

The 222 measured inputs are enumerated in
`target/k190-checks/subjects.txt`: tracked files under `docs/book`, `scripts`,
`crates`, `.cargo`, Cargo manifests/lockfile, Taskfile, release configuration,
README and CONTEXT. The last two cover the additional STE inputs.
Their `verified-before.sha256` and `verified-after.sha256` lists are identical.
The final build log is `verified-book-build.log`. The other final logs use
`verified-` followed by the check name in that directory. One earlier
batch crossed the prose edit and its digest comparison rejected it. Another
stopped at STE because the first clarification made seven sentences. The
final single-sentence clarification restores six. Only the finished,
unchanged-input batch above is the final measurement.

A separate check enumerated every Markdown link/image destination in both
chapters and checked local destinations against built HTML files and IDs.
All 26 local occurrences in chapter 3 and 21 in chapter 12 resolve. The same
check rejected a missing file and a missing anchor for each page. It does
not establish remote content or GitHub rendering.

Chrome showed the locally served book on a light page with a measured
750-pixel main column and 1227 × 919 viewport. Inspected views include the
release table, complete activity and key, mode table, first-release setup,
recovery-table opening and chapter 3's installation entry. The full-size SVG
opened and its upper region was inspected. Raw files supplied whole-document
reads. Editor preview, other themes, narrower views, GitHub and public Pages
were not inspected. The public address, administrator's create call, Pages
delay and GitHub rendering remain unobserved. No new runtime, platform or
reader-comprehension result follows.

No project cut, push, tag, tap edit, permanent installation or public Pages
deployment ran. No archive or formula test was repeated. The existing
lifecycle controls exercise rejected pushes and unchanged repeat delivery,
which support the corrected sentence. No in-session reviewer was needed.

Clear the node's chapter-local hand-forward list. The existing owners table
retains whole-book art k101's figure-size assessment. Set proof k163, guide
fold-ins k164/k165, whole-book art/proof and read 2 retain their owners. No
new correction run is needed and no ADR decision changes. Retirement closes
no ancestor because the book node still has live work.

## Notes
