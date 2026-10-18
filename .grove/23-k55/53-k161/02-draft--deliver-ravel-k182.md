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
