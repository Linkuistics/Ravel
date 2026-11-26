# product-layout-k34

## Goal

Move the launcher into `launcher/`, and give the Ravel repository a product
root. After this leaf, no component owns the root of the repository.

## Context

- The running log of `merge-terms-k15`, from "The launcher is one part of the
  product" to "The launcher moves now". It holds the layout, the mise rule and
  the design of the root `check`.
- `CONTEXT.md` defines **Launcher**, and it flags that "Ravel" alone means the
  product.

## Done when

- The launcher's files are in `launcher/`. They include its crates, its Cargo
  workspace and lock file, and its Rust toolchain file. They also include its
  Cargo and nextest settings, its Taskfile, its scripts, its documents and its
  release tooling.
- The root holds only the product's files. They are `README.md`, `LICENSE`,
  `Taskfile.yml`, `mise.toml`, `CONTEXT-MAP.md`, `docs/adr/`, the ignore files
  and `.grove/`.
- `launcher/mise.toml` declares every program that the launcher's tasks call.
  No launcher task relies on a globally installed program.
- The root `Taskfile.yml` has a `check` task that runs
  `mise exec -C launcher -- task check`. The root `mise.toml` declares only
  `task`.
- The root `check` passes, and the launcher's other tasks still pass when they
  run in `launcher/`.
- A decision record in the root `docs/adr/` states the layout of the product.
  The components are peers, and no component owns the root.
- The launcher's glossary, decision records and specs sit in `launcher/`. The
  root `CONTEXT-MAP.md` points to the launcher's glossary.
- Every link into a moved file still resolves. This covers Ravel's own links
  that name `main`, and the paths in the live task files of this grove.
- The root `README.md` describes the product and points to each component.
- Ravel's checks pass, and no change goes into or from Grove or
  Ravel-Strategies.

## Notes

- The move renames the 3,795 tracked files of the launcher. A rename keeps
  each file's content, so history can follow each file.
- `scripts/release-prepare.sh` checks that it runs at the root of the default
  workspace. That check needs the new place of the launcher.
- Grove links to Ravel's files 97 times. Of those links, 94 name a tag or a
  commit, and they survive the move. The other 3 name the branch
  `skill-directory-delivery`.
- One link from PersonalSkills names `main`, and the move breaks it.
  PersonalSkills is outside this grove, so name the link in the report.
- `rust-toolchain.toml` can stay for rustup, beside the Rust pin in
  `launcher/mise.toml`. mise sets `RUSTUP_TOOLCHAIN`, which rustup ranks above
  the file. So the two pins must name the same release.
- The root also holds a tracked file `out.err`. It is the error output of one
  grove launch on 2026-11-14, committed without a description. It belongs to
  no component, so ask the owner before you delete it.
- The decision record meets the bar in `ADR-FORMAT.md`. A later move breaks
  each link into the moved files, so the layout is hard to reverse. A
  repository called Ravel with the `ravel` program in `launcher/` surprises a
  reader without context. The rejected alternatives kept the launcher at the
  root, or put each pack at the top level.
- Never publish the owner's configuration or secrets as fixtures.
