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
