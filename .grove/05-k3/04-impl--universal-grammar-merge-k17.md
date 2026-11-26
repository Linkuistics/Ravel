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
