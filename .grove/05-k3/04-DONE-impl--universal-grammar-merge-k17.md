# universal-grammar-merge-k17

## Goal

Bring UniversalGrammar and UniversalGrammar-Visualiser into the Ravel
repository, each as one snapshot, on the terms of `merge-terms-k15`.

## Context

- The running log of `merge-terms-k15` holds the terms: the snapshot, the
  layout, the licence, the mise rule and the design of the root `check`.
- UniversalGrammar-ChatGPT is a folder of resources, not a repository. It is
  not part of the merge.

## Done when

- UniversalGrammar is in `universal-grammar/core/`, and the Visualiser is in
  `universal-grammar/visualiser/`. Each arrives as one commit that holds the
  files of its trunk. Each commit message names the source repository and the
  commit.
- The Visualiser's Cargo manifests declare `Apache-2.0`, and the root
  `LICENSE` covers both repositories. Third-party files, such as the IBM Plex
  Mono font, keep their own licences.
- The root `check` runs the check of each repository with
  `mise exec -C <directory> -- task check`, and both checks pass. The
  launcher's check and the pack checks still pass.
- The root `CONTEXT-MAP.md` also lists the glossaries of UniversalGrammar and
  the Visualiser.
- The root `README.md` lists UniversalGrammar and the Visualiser, each with
  its directory.
- No commit holds UniversalGrammar's `inputs/`, `data/` or `reproductions/`.

## Notes

- The Visualiser cleanup already happened. On 2026-11-26 between 18:13 and
  18:19, one operation forgot the workspace `gh-issue-1`, and four operations
  abandoned ten commits. Only the empty working copy stays visible outside the
  trunk. No snapshot carries the abandoned commits, and `merge-landing-k35`
  deletes the checkout that holds them.
- Leave both local checkouts in place. `merge-landing-k35` moves
  UniversalGrammar's personal data, and only then deletes both checkouts.
- Each repository keeps its own `mise.toml`. The Visualiser pins Rust 1.100.0
  with the WebAssembly target, and the launcher pins Rust 1.99.1. mise sets
  `RUSTUP_TOOLCHAIN`, so each run gets its own toolchain.
- UniversalGrammar's `.gitignore` ties `/inputs/`, `/data/` and
  `/reproductions/` to its own directory. Show that the patterns still exclude
  those directories in the new place.
- A research note of the Visualiser links to UniversalGrammar's files by
  absolute paths in the owner's home directory. Both repositories now sit in
  one tree, so relative links can replace those paths.
- Never publish the owner's configuration or secrets as fixtures.

## Decisions (running log)

**Each snapshot is exact.** `git archive` of each source trunk wrote the files
into its directory, and each repository took one commit. Neither source has a
`.gitattributes` file, so `git archive` changed no file. The Git tree of each
directory equals the tree of its source trunk:

| Directory | Source commit | Tree |
| --- | --- | --- |
| `universal-grammar/core` | `4ed57d8b97888ee9a85d8015dbe3b49dac0c66c3` | `587d5ffb3dab6040021dd70dfb578ce15d8983f2` |
| `universal-grammar/visualiser` | `215459f1d717b9f451423b55bf426aa957884844` | `96ebe780dc510f54c3072629c86f402a7a16e769` |

No snapshot commit touches a path outside its own directory. A control
compared the core snapshot with the Visualiser's tree, and it reported a
difference. Before the snapshots, each source `main` equalled `main@origin`,
and each working copy was empty.

**The snapshots hold no secrets and no owner configuration.** A scan looked
for key formats, configuration files, home paths and email addresses. A
planted token showed that the key search can find a hit. The scan found no
key and no configuration file. The only real personal data was six absolute
paths into the owner's home, in the Visualiser's research note
`docs/research/discourse-grammars.md`. The `/home/ann` paths and
`teacher@example.org` are invented example data.

UniversalGrammar is public on GitHub as `AntonyBlakey/UniversalGrammar`, not
under Linkuistics. The Visualiser is private as
`Linkuistics/UniversalGrammar-Visualiser`. Its files become public when the
stage lands, as the owner agreed in `merge-terms-k15`. `merge-landing-k35`
archives UniversalGrammar under its real owner.

**The ignore patterns still exclude the personal data.** The session put a
probe file in `inputs/`, `data/` and `reproductions/` under
`universal-grammar/core/`. `git check-ignore` tied each probe to its line in
`universal-grammar/core/.gitignore`, and jj tracked none of them. jj
tracked a control file in `universal-grammar/core/` itself. The session then
deleted the probes.

**The research note links to UniversalGrammar by relative paths.** Six
absolute paths became links to `../../../core/`. Two repository-relative
paths, `docs/milestones/vertical-slice.md` and
`docs/adr/emergent-categories-and-representation.md`, named UniversalGrammar's
files from inside the Visualiser. They became links too. Each link target
exists. The note is a finished report, so its other text stays unchanged.

**The context map flags two words.** "Reading" and "step" mean different
things in the two glossaries. A Visualiser **Reading** is one way in which a
grammar accepts a sequence. A URG **Alternative reading** is a competing
interpretation. A URG **Move** is "one step" of an episode, and a Visualiser
**Step** is one element of a traced sequence. `CONTEXT-MAP.md` names both
collisions under its relationships.

**The session trusted both `mise.toml` files.** mise refuses an untrusted
configuration file in a new place. The two files are the owner's own, and
`mise trust` records the trust on this machine only.

**The Visualiser declares `Apache-2.0`.** Its workspace `Cargo.toml` changed
from `MIT OR Apache-2.0`, and `cargo metadata` reports `Apache-2.0` for
`workbench-core` and `workbench-wasm`. No other file of the Visualiser
declares a licence. The root `LICENSE` covers it, and the IBM Plex Mono font
keeps its own licence file. UniversalGrammar keeps its own `LICENSE`, as the
packs and the launcher do.

**The root check passes.** `mise exec -- task check` at the root ran the
launcher's check, the four pack checks and the checks of
`universal-grammar/core` and `universal-grammar/visualiser`, with exit status
0. The tree of the working copy was `9ac27e19` before and after the run.
UniversalGrammar ran 158 tests, its lint and its type check. The Visualiser
ran its formatting check, Clippy, its Rust tests, the page check and 45
browser tests in Chromium. The 5 pytest warnings come from the SWIG bindings
of PyMuPDF. A control planted an unused import in `universal-grammar/core`,
and its lint failed with exit status 201. The lint passed again after the
session deleted the probe.

**A trust step waits for the landing.** mise records trust per path. When
`merge-landing-k35` updates the main checkout, both `mise.toml` files there
need `mise trust` before the root check runs.

**No adversarial review of this leaf.** This is the session's judgement, not
the owner's decision. The snapshots are byte-identical to their sources, and
the tree hashes prove it. The other changes are a licence field, six links
and lists of directories. The root check and its controls test them, and the
landing leaf runs the root check again.
