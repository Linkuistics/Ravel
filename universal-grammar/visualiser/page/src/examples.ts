// The example grammars of the repository, bundled into the page.

// https://vite.dev/guide/features#glob-import: `query: "?raw"` imports each
// file as its text, and `eager` imports it at build time.
const files = import.meta.glob<string>("../../examples/*.ebnf", {
  query: "?raw",
  import: "default",
  eager: true,
});

export interface Example {
  /** The file name without its extension, such as `tutorial`. */
  readonly name: string;
  readonly text: string;
}

/** The example grammars, in the order of their names. */
export const examples: readonly Example[] = Object.entries(files)
  .map(([path, text]) => ({ name: path.replace(/^.*\//, "").replace(/\.ebnf$/, ""), text }))
  .sort((a, b) => a.name.localeCompare(b.name));
