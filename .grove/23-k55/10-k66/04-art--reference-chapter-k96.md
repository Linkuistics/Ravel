# reference-chapter-k96

## Goal

Produce chapter 6's figures (`docs/book/src/06-reference.md`) under the
figure contract in `docs/book/contract.md` and the layout and regeneration
procedure in `docs/book/figures.md`, reusing the visual vocabulary chapters
1–5 fixed.

## Context

- The node brief's `Handed forward` entry for art is this leaf's charter: the
  contract's "packages, worker and SDK in source and in an installation"
  figure, as a C4 component view and a UML deployment view, placed at
  "Installed layout". Chapters 7 and 12 reuse them, so draw them to stand
  outside this chapter.
- The relationships are already stated in the chapter's prose and table, and
  the figures add no product or extraction claim: two Rust packages
  (`crates/ravel` and `crates/keyed-launch`), the runner library linked into
  the native front, the separate compiled policy worker embedding Bun, the SDK
  and the static example, the editable sources, declarations, sample and
  notices installed beside it, worker discovery relative to the native
  front's canonical real path, and the worker reaped before the managed
  launch. `task install` installs nine paths; only a release archive adds
  `README.md` and `LICENSE` at its root.
- Copy-edit (`reference-chapter-k95`) fixed the chapter's spellings. Use them
  in captions and alt text: "native front" (never bare "front"), "policy
  worker", "runner" as linked library code (not a third executable or a
  service), "result channel", "session-result context" for `RAVEL_EXIT_FILE`,
  and "test-only probe builds" if probes appear at all.
- The chapter is a lookup chapter: keep the installed-layout table the direct
  entry a reader arrives at, with the figures beside it rather than replacing
  it. In chapter 6, a note or summary beside an entry is a labelled
  blockquote.

## Done when

- The figures exist with committed sources and SVGs, `task docs:figures`
  regenerates and matches them, and the chapter references them with alt
  text that carries the relationships for a reader who cannot see the image.
- `task docs:examples` and `task docs:book:build` pass, and every chapter-6
  link still resolves in the built HTML.
- The node brief's art entry is cleared, and the proof leaf is queued with
  bare stem `reference-chapter` unless a live later sibling already holds it.

## Notes

Mechanical checks establish no human comprehension, terminal or real-agent
operation, native x86-64 qualification, or remote viewing result.
