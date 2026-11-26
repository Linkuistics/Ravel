# Chapter extraction

The `chapter-extraction` stage recovers one chapter from each book in `docs/probes/chapter-selection.json`. It stores each chapter as an immutable artifact with source anchors. Each record covers one chapter only. It makes no claim about the rest of its book.

## Commands

Extract every selected chapter into the store:

```sh
task chapter-extraction
```

Extract one book with a configuration file:

```sh
uv run urg chapter-extraction --book H3MVK7J5 --config extraction.json
```

Render the page region of one anchor:

```sh
uv run urg render-anchor H3MVK7J5 H3MVK7J5:e0001 --output anchor.png
```

The configuration file has two optional keys. `settings` replaces the default layout settings. `book_overrides` maps a Zotero item key to the settings that change for that book.

## Artifacts

Each book gets one `chapter-extraction` artifact with these payloads:

- `extraction.json`: the record.
- `summary.md`: the page table, the checks, the defect counts and the known limitations.
- `render-pNNNN.png`: one render for each boundary page and each representative page.

The record holds these parts:

- `coverage`: the statement that the record covers one chapter.
- `source` and `chapter`: the source identity, the file SHA-256 and the selected page range.
- `book_outline`: the outline sorted by target page, with the chapter entry, its siblings and its children.
- `pages`: for each page, the PDF page index and the printed page label as separate fields. A label also records its method.
- `evidence`: one item for each text line, with a stable ID, the page index, the printed label, the box and the text.
- `structure`: sections, and elements in reading order. An element is a heading, paragraph, code block, figure, caption or inference rule.
- `relationships`: caption to figure, caption to listing, paragraph to cited figure or listing, and a paragraph that continues on the next page.
- `defects`: one entry for each problem that the checks find.
- `checks`: a failed representative page names its defect in `defect`. The anchor check names its defects in `defects`.
- `checks`: the boundary check, the representative-page checks, the anchor check and the ink check.
- `method`: the extractor, its version, the code version, the settings and the known limitations.

The stage also writes one `chapter-extraction-run` artifact for each run. It records the outcome of each book, also a failure.

## Label methods

A printed label never comes from the page index without a record of the method.

| Method | Meaning |
|---|---|
| `page-labels-dictionary` | The PDF `/PageLabels` entry gives the label. |
| `folio-in-page-text` | The `/PageLabels` entry is missing or unreadable. A folio in the page text gives the label. |
| `inferred-from-neighbour` | The page shows no folio. The label continues from the next page. |
| `unknown` | No method gives a label. |

## Checks

- **Boundary.** The stage renders the page before the chapter, the first page, the last page and the page after the chapter. It checks that the outline entry targets the first page and that the opening heading is on that page. It also checks the next outline entry, the opening of the next chapter and the printed labels.
- **Representative pages.** The stage renders the first page, the last page, a page with code and a page with a figure. It prefers an inner page for code and figures. It compares each text line with the ink of the render. It also looks for ink that no text line, figure or drawing covers.
- **Anchors.** The stage reads the text inside each evidence box back from its page and compares it with the stored text.

A failed check adds a defect. Each kind of check adds its own kind of defect:

- The ink check over all pages adds `invisible-text`, `missing-figure` or `no-text-layer`.
- The boundary check adds `chapter-boundary`.
- A representative page that fails at the render resolution adds `render-mismatch`. The ink check at the check resolution can find the same lines. The two defects stay separate, because they record two checks.
- An evidence item whose box does not hold its stored text adds `unresolved-anchor`.

## Reuse

The stage resolves each book's document file through the latest source manifest in the store. The chapter selection also names a manifest key. That key records the manifest that the probe read, and the stage does not look it up. A change to the catalogue-manifest code gives a new manifest key, so a clean store does not hold the probe's manifest. The run report records the key of the manifest that the stage read.

The key of each book's artifact covers the file SHA-256, the selection entry, the settings for that book and the code version. A rerun with unchanged inputs reuses each artifact. A change to one book's override invalidates only that book. A change to the code invalidates every book.

A reused record keeps the location where the extraction read the file. A file that moves keeps its content hash, so the run reuses the record. To render an anchor, the stage first tries the recorded location. When that location does not hold the same content, the stage resolves the current location through the latest source manifest. The `render-anchor` command and the pattern report both do this.

## Results on the selected books

| Book | Chapter | Evidence lines | Elements | Defects |
|---|---|---|---|---|
| Crafting Interpreters | 7, indexes 109 to 122 | 711 | 176 paragraphs, 17 headings, 68 code blocks, 3 figures | outline order 1, duplicate outline entries 1, reading order 11 |
| Practical Foundations for Programming Languages | 5, indexes 55 to 63 | 406 | 96 paragraphs, 6 headings, 31 inference rules, 4 code blocks | page label 9, text layer 8, ligature 8, reading order 5 |
| Regular Expressions Cookbook | 6, indexes 390 to 423 | 1245 | 365 paragraphs, 81 headings, 124 code blocks | reading order 1 |

All boundary checks passed. All anchors resolved. All representative-page checks passed. The chapters of PFPL and the Regex Cookbook have no figure page, and the records say so.

## Known limitations

- Text comes from the PDF text layer. No OCR runs, so no OCR confidence exists.
- A paragraph is one text block of the text layer. A block can join or split paragraphs.
- Reading order is the stored block order. The defect list names pages where it differs from the geometric order.
- The stage records the margin notes of Crafting Interpreters as paragraphs after the body text of their page.
- The stage finds code by monospace fonts. Code set in a proportional font reads as a paragraph.
- A figure is an image or a cluster of vector drawings above a size threshold.
- An inference rule is a thin bar with a centred line below it. Fractions and subscripts inside a line are not recovered.
- The PFPL transition arrow keeps its control character, and ligatures stay single code points. The defect list records both.
