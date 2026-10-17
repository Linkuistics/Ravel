# book-readme-and-authority-k69

## Goal

Reduce the README's user-facing parts to a short entry into the book, and
reduce `docs/authority.md` to what Parts I and II do not absorb, leaving the
repository's front door honest while chapters 7–12 are still to come.

## Context

- `docs/book/contract.md`, "Relation to other documents": the README is reduced
  to a short entry into the book; the authority note is replaced by the book
  once the chapters that absorb it land. Chapter 2 and chapter 5 absorb its
  authority statements and limits. Its test-isolation guards (the forced-empty
  authority names in `.cargo/config.toml`, the `env_clear` fixtures) and its
  legacy `GROVE_*` and `HARNESS_DISPATCH_*` exceptions are contributor and
  provenance material that chapter 11 and `docs/caller-references.md` take
  later, so the file is reduced in place now and removed in the second half.
- The README ships in the release archives (`scripts/release-common.sh`
  lists it and `scripts/release-build.sh` copies it). It must stay a
  standalone file that makes sense inside an archive with no checkout: do not
  make its only content a link into `docs/` that an archive does not carry.
  Say what Ravel is in a few sentences and name the book's location and how to
  read it locally (`task docs:book`), and state that the repository checkout is
  where the book lives.
- Keep, unchanged, the README's "Develop" section, the "Earlier project" pointer
  and the licence line, which chapters 11 and 12 and the retired record absorb
  later. Move nothing out of them here.
- Everything the README says about the pair, the runner, the channel and
  delivery is checked against the book's chapters: if a statement is dropped, a
  chapter has it or the spec does; if it stays, it is still true.

## Done when

- The README's intro and "Use the pair" parts are replaced by a short entry
  (what Ravel is, where the book is and how to build it, the two verbs in one
  line), with no teaching and no duplicate of a chapter.
- `docs/authority.md` states only the test-isolation guards and the
  legacy-name exceptions, with its title and first sentence saying that. Its
  user-facing authority paragraphs are gone, each shown to be in chapter 2 or
  chapter 5 by anchor in this leaf's log. Its path and
  its role in `docs/caller-references.md` are unchanged, and that inventory's
  row for it is edited to match.
- Every inbound link to either file still resolves; the sweep is enumerated
  with a positive and a mutated control as in `book-guide-fold-in-k68`.
- `task check` and `task release:test` pass, and `task smoke` is run or its
  non-run explained, because the README is in the installed archives.
- The node brief's "Handed forward" is updated with what remains for chapters
  11 and 12.

## Notes

- Nothing the README says about unreleased archives, floors or the formula may
  become less accurate: those statements are chapter 12's, not this leaf's.

## Decisions (running log)

- README reduced to an entry that stands alone in an archive: what Ravel is (a
  caller asks by kind, owner policy picks the program, Ravel supervises and
  returns the published result), the two verbs in one line, the book's
  location (`docs/book/`, in the checkout; archives do not carry it), how to
  read it (`task setup`, `task docs:book`, or the Markdown from chapter 1).
  Checked against `scripts/release-common.sh`: the archive manifest carries
  `README.md` and `LICENSE` but no `docs/`.
- The README keeps one honest pointer paragraph for ground the book has not yet
  written (component contracts, runner boundary, executable specification,
  runtime evidence, notices) so that `docs/components.md` and
  `docs/local-runner.md` do not lose their only front-door link while Parts III
  and IV are unwritten. The notices path is given both for a checkout
  (`crates/ravel/notices/NOTICES.md`) and an installation
  (`libexec/ravel/notices/`), since the first does not exist in an archive.
- Develop, Earlier project (with the two release paragraphs under it) and the
  licence line are byte-unchanged; only "Use the pair" and the intro were
  replaced. `git`/`jj diff` shows no hunk inside those sections.
- Dropped README statements: "installed pair includes its own runtime, generic
  SDK, examples and notices; runs from a fresh prefix, directly or through a
  symlink" is in `docs/specs/session-execution.md` (lines on the fresh prefix
  and symlink seam), `docs/RELEASING.md` and ch6 `#installed-layout`; the
  `ravel start-session ...` / `end-session done` example and the stdout/stderr
  rule are in ch3 and ch6 `#standard-streams`; the `PATH` instruction is
  ch5 `#supply-the-prompt-and-caller-data`; the SDK/example/declaration paths
  are ch4 and ch6 `#installed-layout`; the copy-and-edit-sample guidance is ch4
  `#copy-an-existing-policy`; `task docs:components` (editable component views)
  is no longer in the README, and the views stay reachable from
  `docs/components.md`.
- `docs/authority.md` reduced in place: title "Test-isolation guards and
  legacy-name exceptions", first sentence says so, the two guard/exception
  paragraphs are unchanged except that the sentence "Production environment
  grants exclude HARNESS_DISPATCH_* as well as RAVEL_*; ordinary unrelated
  owner grants remain generic" is gone (user-facing; ch6 holds it). It now
  points to chapters 2 and 5 for the authority boundaries. Path unchanged.
  `docs/caller-references.md` row edited to "Explain the test-isolation guards,
  the legacy-name exceptions and this bounded sweep".
- `docs/book/contract.md` "Relation to other documents": the authority-note
  clause edited in place to say it is reduced now and removed when chapter 11
  absorbs the remainder (the clause "replaced by the book once chapters land"
  would otherwise be false of the tree). Nothing else in the contract touched.

## Classification: `docs/authority.md` before reduction (every sentence)

Ch short forms as in `book-guide-fold-in-k68`; `docs/book/src/` anchors. K =
kept in the reduced file.

| # | Sentence (original paragraph) | Disp. | Home |
|---|---|---|---|
| 1 | P1 owner policy is trusted executable configuration | absorbed | ch2 `#trust-and-confinement` |
| 2 | P1 Ravel chooses it only from explicit `--config` or the absolute-HOME default | absorbed | ch6 `#policyts`, `#--config-path`, `home_unset` row in the diagnostic codes; ch4 `#define-a-policy-and-read-the-request` |
| 3 | P1 the native front holds the caller's environment, terminal, result stdout and supervision authority | absorbed | ch2 `#policy-evaluation-and-supervision` |
| 4 | P1 the worker gets a bounded frozen request, measured context operations, explicit grants, no completion authority | absorbed | ch2 `#policy-evaluation-and-supervision`; ch4 `#supply-policy-context`, `#set-bounds-and-grant-environment-names`; ch6 `#reserved-worker-grants` |
| 5 | P1 the worker is reaped before the selected process starts | absorbed | ch2 `#policy-evaluation-and-supervision` |
| 6 | P1 owner-policy filesystem/network access is outside the confinement boundary | absorbed | ch2 `#trust-and-confinement`; ch5 `#understand-the-operational-limits` |
| 7 | P2 the managed process gets its own ephemeral `RAVEL_EXIT_FILE` | absorbed | ch6 `#managed-process-environment`, `#ravel_exit_file` |
| 8 | P2 nested sessions replace only the child's context; `end-session` addresses inherited context with no target or lookup | absorbed | ch5 `#keep-nested-sessions-separate` |
| 9 | P2 same-user code can tamper; neither identity matching nor context is authentication | absorbed | ch2 `#trust-and-confinement`; ch5 `#understand-the-operational-limits` (first row) |
| 10 | P2 detached policy children and abruptly orphaned managed processes are limits | absorbed | ch2 `#trust-and-confinement`; ch5 `#understand-the-operational-limits` (rows 2-4) |
| 11 | P3 every sentence (tests never act as the enclosing session; forced-empty Cargo controls; helpers scrub; fixtures `env_clear`; no inherited owner policy) | K | contributor material for ch11 and `docs/caller-references.md` |
| 12 | P4 legacy `GROVE_*`/`HARNESS_DISPATCH_*` spellings are intentional extraction-safety exceptions | K | as above |
| 13 | P4 production grants exclude `HARNESS_DISPATCH_*` and `RAVEL_*`; other grants generic | absorbed | ch6 `#reserved-worker-grants`; ch4 `#set-bounds-and-grant-environment-names` |
| 14 | P4 caller-specific paths in provenance are historical attribution; Ravel loads none of those projects | K | provenance, `docs/caller-references.md` |
| 15 | P4 the inventory classifies active references | K | link to `docs/caller-references.md` |

All anchors in rows 1-10 and 13 were checked to exist with the same slugger the
link sweep uses (script output: every listed anchor True).

## Verification

- **Link sweep (enumerated).** Script over `jj file list`, excluding
  `docs/evidence/`, `retired/` and `.grove/`, non-Markdown files, code fences
  and spans; resolves each link target file and heading anchor. Final tree: 44
  Markdown files, 767 links; 214 file, 454 file+anchor, 99 external (not
  resolved), 0 broken. The k68 reading was 770/216/455/99/0; the three-link
  difference is the README's dropped links less the new ones. **Controls seen
  to fail:** appending `docs/authority-gone.md` and
  `docs/authority.md#...-exceptionz` to the README gave 2 broken (file and
  anchor), exit 1, while the correct new-heading anchor in the same append was
  counted resolved (anchor-ok rose to 455); the README was restored and its
  sha256 verified before the clean 0-broken reading. **Cross-tree control:**
  `authority\.md` is found once in the parent revision's README and still found
  in `docs/caller-references.md` and `docs/book/contract.md`.
- **Enumerate-then-classify of remaining mentions of the authority note**
  (tracked tree minus receipts): only `docs/book/contract.md` (the edited
  clause) and `docs/caller-references.md` (the edited row). The README no
  longer links it, as the done-when requires. Non-Markdown files name neither
  `authority.md` nor README text, apart from the release scripts that copy
  `README.md`, and `docs/book/src/06-reference.md` / `installed-files.puml`,
  which say archives carry `README.md`, still true.
- **Gates (run after the last edit to README, `docs/` and the contract; only
  `.grove/` notes changed while they ran).** `task docs:book:build` passed
  (mdBook build, no warnings). `task check` passed: 339 tests run, 339 passed,
  46 skipped. `task release:test` passed (archive manifests unchanged, README
  still packaged). `task smoke` passed (fresh-prefix install; the README is
  packaged in the archives that `task release:test` builds, not in the smoke
  prefix itself, so smoke confirms the pair and release:test the README
  packaging).
- Not run: GitHub rendering of the README; a fresh-reader comprehension check
  of the new entry; `task release:smoke` (three-target archives; the README
  text is not read by it, and release:test already covers the manifest).
