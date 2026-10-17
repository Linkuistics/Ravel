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

## Decisions (running log)

- Read the whole chapter against the art charter. Retain its lookup tables;
  add the contracted C4 component and UML deployment views after the installed
  table. The component view opens the native front to show the linked runner
  and keeps the policy worker a separate process. The deployment view groups
  the installed files and distinguishes embedded modules from editable copies.
- Reuse the book's PlantUML pin, Smetana and shared palette. Put notation,
  reader takeaway, meaningful alt text and full-size links beside each figure;
  keep chapter-6 figure explanations in labelled blockquotes. No renderer,
  contract, product, architecture or extraction decision changes.
- Source check: Tier 2 graph generation 2026-10-16T07:04:38Z; worker.locate's
  sole direct caller is choice.choose_prepared (complete one-row trace), and
  its exact snippet canonicalises current_exe before joining the worker path.
  Cargo manifests, dispatch build/install and release staging were read
  directly, as were worker main's embedded imports. Coverage reports no
  recorded issue on these evidence files; diagram sources have untracked
  freshness and were read directly. This is source evidence, not execution.
- A two-row page-native table now carries the sample's default/careful
  comparison, preserving the exact-high behaviour. Add role sentences to the
  existing stream and import tables. Other lookup tables retain their existing
  introductions. Declined a further session timeline and environment diagram:
  editorial judgement, because chapters 3–5 already carry those figures and
  this chapter's status, stream and environment tables supply direct lookup.
  No figure was declined because of a forbidden or inadequate medium.
- Local Chrome inspection found the deployment dependency label crowded a
  directory boundary; shortened the label and rechecked its final SVG at
  full size and chapter width. Both figure keys and captions remain legible.
  Viewing coverage and limits are recorded in docs/book/figures.md.
- Final checks: task docs:figures matched all nine sources/SVGs;
  task docs:examples passed 13 complete examples, zero failed/skipped, six
  labelled fragments not run; task docs:book:build completed without warnings.
  The link check resolved 289 chapter link/image destinations and verified
  alt text for both distinct figures in all four inline/zoom image elements.
  Missing-file/anchor/alt/image controls rejected known bad inputs. The first
  ad hoc image-count assertion expected two HTML img elements and failed:
  reading built HTML showed mdBook emits an inline and zoom copy per image.
  Corrected that assertion to test the distinct assets and every instance;
  no book or toolchain correction was needed.
- SHA-256 before/after comparison held for 173 files enumerated by rg under
  crates, scripts and docs/book, plus Cargo.toml, Cargo.lock, Taskfile.yml and
  CONTEXT.md. This bounds the stability claim to those inputs, not installed
  dependencies or renderer executables. Local scratch logs and the two digest
  maps are in target/k96-validation. The final sample-choice table was also
  inspected at chapter width after rebuilding and refreshing Chrome.
- No earlier-stage defect or medium limitation was found. No in-session
  reviewer was needed or spawned. Clear the art handoff and queue the bare-stem
  proof successor; the node stays open until proof. Existing ADRs still state
  the same decisions and need no reconciliation edit.
