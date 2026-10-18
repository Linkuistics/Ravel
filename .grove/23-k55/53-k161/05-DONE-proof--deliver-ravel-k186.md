# deliver-ravel-k186


## Goal

Proof the whole chapter 12, `docs/book/src/12-deliver-ravel.md`, under
`docs/book/contract.md`. Check its technical truth, STE, figures, examples,
entry routes and inventory 5 delivery coverage. Leave all required checks green.



## Context

- Read the document brief and its remaining proof handoff. The current release
  task has no Pages delivery step. `book-release-pages-k184` owns that work and
  its later editorial correction run. This proof cannot discharge that separate
  acceptance requirement or treat the version guard as a Pages deployment.
- Draft k162 and k182 hold the source checks, guide-statement destinations and
  release-task observations. Copy-edit k183 holds its wording changes. Art k185
  holds the seven-row recovery mapping, source order and final checks.
- `release-delivery.typ` uses `_activity.typ` and the shared crisp style.
  The local region ends before the first push of `main`. The seven grey exits
  name manual recovery, not automatic retries. Each decision summarises the
  actions since the previous decision, and a failure stops at its own step.
  No post-cut path starts another version cut. Check the order against
  `Taskfile.yml`, including formula tests within `scripts/release-build.sh`.
- The installation view stays a link to chapter 6. The exact manifest table
  stays on this page. There is no second installation source.
- The final action checks Ravel's version at the Homebrew prefix. The caption
  explanation precedes the figure so that the next prose run stays within
  450 words. Existing prose teaching and examples are unchanged by art.
- `docs/book/figures.md` records the local Chrome views and their limits.
  The activity is 263,202 bytes. The full SVG set is 3,175,724 bytes, above its
  earlier benchmark. Whole-book art k101 owns the existing set-size issue.

## Done when

- The whole chapter passes its contract, with the release-task inventory
  enumerated again from the subject and any gap routed to its owner.
- The figure order, public-write boundary and recovery meanings agree with
  current source. The figure links and rendered chapter destinations resolve.
- Figure comparison, book build, strict STE, strict prose breaks, examples and
  source excerpts pass. Inspect the viewing surfaces that are available and
  record limits. No publishing release task, push, tag or permanent install runs.
- Reconcile the brief's acceptance with chapter-local delivery. Preserve k184's
  Pages responsibility and all set/whole-book work when closing this node.

## Notes

Art found no earlier-stage defect that needs a correction run. Proof cuts no
ordinary successor. The remaining Pages handoff belongs to work already live
outside this document node, before the second-half set proof.

## Decisions (running log)

- The whole chapter agrees with the current release procedure. No chapter
  correction or editorial re-run is needed. Retain its distinction between
  the implemented book-version guards and the absent Pages delivery step.
- Inventory 5 has ten public release tasks, re-enumerated from `task --list`.
  Each has teaching in chapter 12. `install` and `smoke` have teaching in
  chapter 11, reached from this chapter's opening. No delivery task is missing.
- Close the document node after validation. Clear its proof handoff by
  promoting the surviving Pages requirement to the book brief, where k184
  already owns implementation and the complete editorial correction run.
  This closes chapter-local proof without discharging Pages acceptance.
- Keep the existing whole-book owners: k163 checks the second-half set,
  k165 folds in the release guide and README, and k101 owns figure size and
  visual consistency. Chapter 10 and the first public book release remain
  outside this chapter node. No architectural decision changes.

## Proof evidence and inventory 5

The whole chapter, its tables, labelled fragments, alt text and activity record
were read against the book contract, shared word decisions and glossary.
The installed boundary and archive/platform sections of the session specification
agree with the chapter. The three ADRs retain their decisions. Version 1.0.0
is present in the workspace manifest and both workspace entries of Cargo.lock;
local tags include v0.1.0 and v1.0.0. The stale 0.0.0 prose in the release guide
is not repeated as current state.

Taskfile's complete sequential release body, release.toml and the build,
common, formula, preparation, publisher and book-version scripts supply the
release order and recovery facts. Formula testing occurs at the end of the
archive build. Every-target smoke follows it before the first push. The seven
recovery rows agree with the seven grey exits; the tagged-book guard shares
the local-failure exit. A grouped decision does not postpone an earlier
command's failure. No post-cut recovery selects another version.

Tier 2 graph verification used project
`Users-antony-Development-Ravel.simplify-and-componentize`, full generation
`2026-10-18T12:04:03Z`. The worker function search returned 25 entries without
another page. `verify_hello` has `converse` as its one direct caller; its exact
snippet compares protocol, package version and build ID. The both-direction
trace has one caller and four callees, without truncation. Direct source reads
of build.rs and dispatch.sh confirm the sorted digest inputs. This is bounded
source evidence, not new runtime qualification.

Coverage was checked for 37 source, configuration, specification and document
paths. The QEMU script's recorded gap at line 185 was read directly, including
its address-bound check and neighbouring vDSO check. Cargo.lock lacks graph
freshness metadata; both workspace entries were read directly. SVGs are
excluded from the graph; regeneration comparison and the actual browser render
supply their evidence. Other checked paths have matching metadata and no
recorded gap. Coverage remains a best-effort signal.

The fresh public task list gives these destinations:

| Task | Teaching destination |
|---|---|
| `release:archives` | Build archives before a release |
| `release:test` | Check the archive contents |
| `release:smoke` | Test the installed archive; Read the platform checks correctly |
| `release:formula` | Prepare and test the Homebrew formula |
| `release:doctor` | Prepare a version cut; Keep the book with its release |
| `release:notes` | Prepare a version cut |
| `release:lifecycle:test` | Prepare a version cut; recovery and dated checks |
| `release:patch` | Preparation task table; cut and recovery |
| `release:minor` | Preparation task table; cut and recovery |
| `release:major` | Preparation task table; cut and recovery |
| `install`, `smoke` | Chapter 11's Install and test the installed pair, linked from the opening |

No delivery item is absent. The internal `release` task is also taught as the
shared procedure. The public list adds no other archive task. The guide's
statement groups were read again against k182's complete destination table
and the current chapter. That map remains valid for fold-in k165, including
the stale first-cut/formula-URL statements, the seven recovery branches and
the README's release and notices lines. No guide is removed by this proof.

## Verification and viewing limits

Fresh checks on 2026-10-18:

| Check | Result |
|---|---|
| `task docs:figures` | Fourteen SVGs and layout records match their sources |
| `task docs:book:build` | Passed without error or warning |
| `task docs:ste -- --strict` | Zero problems in 28 files |
| `task docs:breaks -- --strict` | Zero long runs in eleven chapters |
| `task docs:examples` | Fourteen passed, zero failed or skipped; 26 labelled fragments not run; no page errors |
| `task docs:excerpts` | Eight passed; no failures or page errors |
| `task release:lifecycle:test` | Passed in isolated fixtures, including actual cargo-release book-version controls |
| Rendered destinations | All 33 local chapter-body destinations resolve; all 21 Markdown targets agree with rendered href/src values |

The link check rejected deliberately missing file, anchor and image targets.
It checked built HTML IDs, not only Markdown filenames. The source digest
lists before and after the checks compare equal for all 219 subjects. They
cover tracked book files, scripts, crate source and fixtures, Cargo manifests
and lockfile, Taskfile, release configuration, glossary and README. Logs and
digests are under `target/k186-*`; the full lifecycle log is
`target/release-lifecycle.n7n1PP`. No measured input changed during a run.

Raw Markdown supplied the whole-document view. Chrome showed the local book
served by `task docs:book PORT=8786`, at the 750 px column in a 1467 × 1000
window. The release table, local cut and recovery branches, public-write
boundary, public actions, final installation and full key were inspected.
The release SVG opened separately at full size; its opening region was
inspected. VS Code supplied a light Restricted Mode Markdown preview of the
opening through the archive-build sections and target table. Quick Open
worked after the ordinary Open command reported a missing extension command.

GitHub rendering and the book on GitHub Pages cannot show this unpushed
chapter. They were not inspected. Other themes, narrower views and the
remainder of the full-size SVG were not inspected. The existing set-size and
dense-text assessments retain owner k101. No reader-comprehension, new target,
real-agent or interactive-terminal result follows.

The lifecycle test used only its isolated repositories and service substitutes.
No project release task, push, tag, tap write, Pages deployment or permanent
installation ran. The chapter's earlier archive and formula measurements stay
dated draft results; proof did not repeat them or rewrite their provenance.

## Node close

All four chapter stages are complete. The chapter exists in SUMMARY, inventory
5 has no chapter-local omission, current versions and limits agree with source,
and the guide-statement destinations and R17 teaching remain complete.
No chapter or figure correction was needed. No in-session reviewer was used.

Close deliver-ravel-k161 with this leaf. The proof handoff is cleared and its
surviving Pages obligation promoted to the book brief, under existing owner
k184 with its required later correction run. This does not close the book node:
Pages implementation, set proof, guide migration, whole-book art/proof and
read 2 remain live. No ADR change is warranted.
