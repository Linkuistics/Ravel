# The product's components are peers

Ravel names the product. Each component of the product lives in a directory
of its own, and no component owns the root of the repository:

- `launcher/` holds the launcher, the `ravel` program with its crates,
  documents and release tools.
- `packs/<name>/` holds each skill pack, named by the source ID of the pack.
- `universal-grammar/core/` holds UniversalGrammar, and
  `universal-grammar/visualiser/` holds its visualiser.

The root holds only the product's files: the README, the licence, the
Taskfile, `mise.toml`, the context map, the product's decision records and
the ignore files. Each component keeps its own Taskfile, `mise.toml`,
glossary, decision records and specs. Each component stays runnable alone in
its directory. The root `check` runs the `check` task of each component as a
separate run, with `mise exec -C <component> -- task check`.

On 2026-11-26 the owner stated that the launcher is one part of the product,
without privilege over the other parts. A component at the root holds that
privilege. Its tasks, its tools and its Cargo workspace become the defaults of
each command at the root, and the other components sit inside its tree.

The layout is hard to reverse. A later move breaks each link into the moved
files, and links from other repositories that name `main` break with them. A
repository called Ravel with the `ravel` program in `launcher/` also surprises
a reader without context. So the root README names each component and its
directory.

The design rejected three alternatives:

- **The launcher stays at the root, with the other components below it.** It
  keeps every link into the launcher, but it gives the launcher the privilege
  that the owner ruled out.
- **Each pack sits at the top level.** It shortens each path by one level, but
  it crowds the root with peers of one kind. The owner keeps the packs apart
  for clarity only, and they share one `packs/mise.toml`.
- **The root Taskfile includes each component's Taskfile.** In Task 3.54.0, an
  included Taskfile resolves `{{.ROOT_DIR}}` to the product root, so an
  included component resolves its paths there. Its tasks also take the root's
  tools. A separate run keeps each check exactly as it runs alone.
