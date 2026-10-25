# rhai-book-inside-k36

## Goal

Draft the changes to chapters 10 and 11 of the Ravel book. A reader can prepare a checkout and build Ravel 2.0.0. The reader can also test it, deliver it and recover a partial release.

## Context

- The structure brief is `docs/book/contract.md`. Read the running log of `rhai-book-inside-k35` first.
- Chapter 10 describes a checkout, the setup with Bun, the worker build, the typecheck, the test seams and the evidence documents. Chapter 11 describes the versions, the identity, the installation, the archives, the formula and the release tasks.
- The new facts come from the leaves `rhai-llm-upgrade-proof`, `typescript-delivery-removal` and `rust-2024-toolchain`. They are the Rust 1.99.1 toolchain, the Rig and Rhai upgrade tasks, the opt-in smoke tasks, the notices task, and the new archive layout.
- The book tools keep a checkout-local Bun. Say that they do, and that the release carries no Bun.
- The release step `brew upgrade ravel` removes the old version unless `HOMEBREW_NO_INSTALL_CLEANUP` is set. A running Grove session needs the old version. Chapter 11 states the rule.
- Run every complete example through `task docs:examples`, and check every excerpt with `task docs:excerpts`.

## Done when

- Chapters 10 and 11 describe the tasks, the files and the release steps that exist. Each task name was run, or the running log says why it was not.
- The chapters cover the new test seam: the fake endpoint, the recordings and the Rig upgrade proof.
- The refusal of a lone `policy.ts` appears as a complete example somewhere in the book, and it runs.
- The leaf names the defects that a later stage owns in the `## Handed forward` list of the node brief.
- The last act cuts the `copy-edit` leaf, unless a live copy-edit leaf already stands below the node.

## Notes

- Do not describe a release that has not happened. The release leaves run after the book is final.
- If the leaf proves too big, decompose it by chapter.
