# Ravel

Ravel is one product made of peer components. Each component lives in a
directory of its own, with its own tasks, tools and documents. No component
owns the root of this repository. The decision record
[`the-product-components-are-peers`](docs/adr/the-product-components-are-peers.md)
explains the layout.

## Components

- **[Launcher](launcher/)**, in `launcher/`. The launcher is the `ravel`
  program. It runs LLM sessions in agent harnesses, such as Claude Code and
  Codex, as supervised parts of another program. Its
  [README](launcher/README.md) explains the installation and points to its
  book.
- **Skill packs**, in `packs/`. Each pack ships session kinds of
  [Grove](https://github.com/Linkuistics/grove) as agent skills. The four
  packs share their tool pins in `packs/mise.toml`.
  - **[Requirements](packs/requirements/)**, in `packs/requirements/`. It
    ships the requirements kinds.
  - **[Engineering](packs/engineering/)**, in `packs/engineering/`. It ships
    the `impl` and `prototype` kinds and engineering craft skills.
  - **[Research](packs/research/)**, in `packs/research/`. It ships the
    research kinds.
  - **[Writing](packs/writing/)**, in `packs/writing/`. It ships the
    editorial kinds and writing craft skills.
- **Universal Refinement Grammar**, in `universal-grammar/`. Its two
  components keep separate tools, because one is in Python and one is in
  Rust.
  - **[UniversalGrammar](universal-grammar/core/)**, in
    `universal-grammar/core/`. It discovers how authors explain technical
    subjects, from whole books down to explanatory episodes. Its pipeline
    reads a personal library, and version control excludes that library and
    the data derived from it.
  - **[Visualiser](universal-grammar/visualiser/)**, in
    `universal-grammar/visualiser/`. It is a workbench for hand-written
    grammars of steps. It traces sequences of steps through a grammar and
    draws the rules as diagrams.

## Work in a checkout

[mise](https://mise.jdx.dev/) supplies the tools. Each component declares the
programs that its tasks call in its own `mise.toml`. mise installs a missing
tool when a command runs through it.

To check every component, run the root `check` task at the root of the
checkout:

```sh
mise exec -- task check
```

The root `check` runs the `check` task of each component in that component's
directory and tool environment.

To run another task of a component, run it in that component's directory.
For example, this command builds the launcher from the root:

```sh
mise exec -C launcher -- task build
```

[`CONTEXT-MAP.md`](CONTEXT-MAP.md) lists the glossary of each component.

## Licence

Licensed under [Apache-2.0](LICENSE).
