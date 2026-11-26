# Extraction probe: candidate books and chapter selection

Task: `extraction-probe-k4`. Source manifest: `eccb9a1d42774e89b70a63300322d49a637607de174827155bdb7759c175d742`.
Extractor: PyMuPDF 2.1 with MuPDF 1.28.6.
The machine-readable selection is in `docs/probes/chapter-selection.json`.

## Method

The candidate pool holds reconciled Computing books with exactly one readable PDF. That pool has 834 books.
The probe opened ten books that cover three explanatory approaches.
For each book, the probe read the outline, the page-label rules and the producer metadata.
It then sampled six pages at 5, 15, 30, 50, 70 and 90 per cent of the page count.
For each sampled page, it recorded the following:

- Text-layer character count.
- Monospace character count, from font name or the fixed-pitch flag.
- Images, full-page images and vector drawings.
- Caption lines such as "Figure 3-1".
- Whether the stored block order differs from the geometric sort.

For the four finalists, the probe printed the outline around the chapter and the text of each boundary page.
It also rendered each boundary page to PNG.
The probe code was throwaway and stayed out of the repository.

## Probe notes per candidate

Page indexes are 0-based PDF page indexes. Outline targets are 1-based, as PyMuPDF reports them.

| Key | Book (year) | Approach | Text layer | Page labels | Outline | Code recognition | Figures and captions |
|---|---|---|---|---|---|---|---|
| 6IQTDYAW | Crafting Interpreters (2021) | Tutorial with code | Born-digital (InDesign). 1,500-2,600 characters per page. | 3 rules: roman, then decimal from index 13 | 253 entries, levels 1-2. Order is defective (see below). | Source Code Pro, 300-440 monospace characters on code pages | Vector drawings. No caption lines found. |
| GN263Y2E | Grokking Deep Learning (2019) | Tutorial, visual and conversational | Born-digital | 5 rules: decimal from index 21, offset 20 | 262 entries, levels 1-2. Chapters at level 1. | Courier, 470-1,431 monospace characters | Figures are vector drawings, 20-49 per page. |
| PZZBPXX9 | Regular Expressions Cookbook (2012) | Recipe cookbook | Born-digital (XSL Formatter) | Decimal from index 16, offset 15 | 1,479 entries, levels 1-5. Chapter titles use NBSP. | TheSansMonoCd, 128-765 monospace characters | "Figure 1-5." caption found |
| H3MVK7J5 | Practical Foundations for Programming Languages (2013) | Theory, rules and proofs | Born-digital (Distiller 7) | Malformed (see below) | 371 entries, levels 1-4: parts, chapters, sections | CMTT10, 20-76 monospace characters (inline syntax only) | No images. Inference rules are text. |
| 2NGS23JV | The Definitive ANTLR 4 Reference (2014) | Reference with tutorial parts | Born-digital | 26 rules, one per front-matter page | 124 entries, levels 1-3 | DejaVu Sans Mono, 175-833 monospace characters | Not probed further |
| N3EQESED | How to Design Programs (2001) | Tutorial with code | Text layer present (iTextSharp). Font names are obfuscated. | None | None | Not recognisable: no font names a monospace face | "Figure 63" and "Figure 118" captions found. One full-page image. |
| 4P4MSXAK | Types and Programming Languages (2002) | Theory, rules and proofs | Text layer present (dvips and Ghostscript, Type 3 fonts) | None | None | Not recognisable by font | "Figure 1-1" and "Figure 21-3" captions found. Up to 80 glyph bitmaps per page. |
| 85PL42K4 | Introduction to the Theory of Computation (2006) | Theory, rules and proofs | Scan: one full-page image per sampled page with a hidden Times text layer | None | None | Not applicable | Figures are inside the page image |
| GE97DAWJ | LaTeX: A Document Preparation System, user's guide (1994) | Reference manual | Scan with ABBYY FineReader OCR. One OCR font (AppleMyungjo). | None | 395 entries, but only two level-1 entries | Courier on one sampled page only | Figures are inside the page image |
| NAIX9LIB | An Introduction to Functional Programming Through Lambda Calculus (1989) | Theory, tutorial | Scan with ABBYY FineReader OCR | None | 171 entries. Titles are inconsistent and end with newlines. | Not recognisable | Figures are inside the page image |

### Defects found

- **Crafting Interpreters outline order.** The first 16 level-1 entries are all "Design Note: …" entries, with targets from page 23 to page 578. "Welcome" (page 14) follows them. Each design note also appears again as a level-2 child of its chapter. A chapter-boundary rule must do two things. It must sort entries by target page. It must also skip each chapter-level entry whose title recurs as a deeper entry. Sorting alone is not enough for this outline. In page order, the level-1 entries around chapter 7 are "Evaluating Expressions" (page 110), "Design Note: Static and Dynamic Typing" (page 123) and "Statements and State" (page 124). A rule that ends the chapter at the next sorted level-1 entry stops at index 121. It loses index 122, which holds the chapter's design note.
- **Practical Foundations page labels.** The `/PageLabels` number tree is `<</Nums[0 4591 0 R 0<</S/D/St 11>>0<</S/D/St 465>>2 4592 0 R 20 4593 0 R]>>`. It has three entries for page 0. `Page.get_label()` raises `TypeError: '<' not supported between instances of 'PdfObj' and 'PdfObj'`. `Document.get_page_labels()` returns the rules without error. Printed folios are reliable only from the page text.
- **Practical Foundations inference rules.** The text layer flattens each rule into lines. It gives no fraction bar between premises and conclusion. Primes and subscripts split onto separate lines ("e′" / "2"). The transition arrow comes out as a control character before "→" (`\x0f→`, `\r→`). Ligatures stay as single code points ("ﬁ").
- **Grokking Deep Learning licence watermark.** Every page carries a purchaser line with a personal name and email address. One page (index 98) holds only this line. Extracted text must drop this line before it enters any committed or shared record.
- **How to Design Programs, Types and Programming Languages.** These books have no outline and no page labels. Their fonts give no signal for code. The extractor would need running heads or a visual model to find chapter boundaries and code.
- **Theory of Computation, LaTeX guide, Lambda Calculus.** These are scans with an OCR text layer. The OCR layer has no word-level uncertainty that PyMuPDF exposes. Code and figures are inside the page image.
- **Reading order.** In six of the ten books, the stored block order differs from the geometric sort on most sampled pages. The differences come from running heads, margin asides (Crafting Interpreters) and figure labels (Grokking). The Regular Expressions Cookbook mostly kept the same order.

## Selection

The three selected books use clearly different explanatory approaches. The choice rests on the observed page contents.

1. **Crafting Interpreters, chapter 7 "Evaluating Expressions"** (indexes 109-122, labels 97-110). Prose narrates incremental Java code listings. Margin asides and a closing design note add a second register. It was chosen over Grokking Deep Learning for three reasons. Its page labels are clean. It has no personal watermark. Its asides test reading order.
2. **Practical Foundations for Programming Languages, chapter 5 "Dynamics"** (indexes 55-63, printed folios 36-44). Numbered inference rules and rule-induction arguments carry the explanation. It was chosen over two other theory texts. Types and Programming Languages has no outline. Theory of Computation is a scan without an outline.
3. **Regular Expressions Cookbook, chapter 6 "Numbers"** (indexes 390-423, labels 375-408). Each recipe repeats a Problem, Solution, Discussion and See Also frame. It was chosen over the ANTLR 4 Reference, because the recipe frame is a stronger contrast with the tutorial.

The JSON record gives the boundary evidence for each chapter. That evidence is the outline entry, the opening heading, the closing content, the next chapter's opening and the page labels.
For Crafting Interpreters, the JSON names "Statements and State" as the next level-1 entry. That statement applies after the duplicate "Design Note: Static and Dynamic Typing" entry at page 123 is skipped. The selected range 109-122 includes the design note.

## Recommendation for chapter-extraction

- **Use PyMuPDF** (`get_text("dict")`) for born-digital books. It gives spans with font, size, flags and bounding boxes. Monospace detection by font name, plus the fixed-pitch flag, found code in three of the four finalists.
- **Take boundaries from the outline**, sorted by target page. The next chapter is the next entry at the chapter's level or above that is not a duplicate of a deeper entry. Confirm each boundary against the opening heading text on the start page, and against the next chapter's opening.
- **Take printed labels from `/PageLabels` when `get_label()` succeeds.** When it fails or the rules look wrong, read the folio from the page text. When a chapter opener has no folio, record its label as inferred.
- **Keep the stored block order and the geometric order as two evidence anchors.** Record a reading-order uncertainty when they differ. Do not choose silently.
- **Keep the raw text of inference rules and listings, with coordinates.** Rule structure (premises above a bar) is not in the text layer. To recover it, read the vector drawings (the bar) or confirm against the rendered page.
- **Strip licence watermark lines** before writing extracted text.

Limitations:

- The probe sampled six pages per book, so defects on other pages are possible.
- No caption pattern matched in Crafting Interpreters or Practical Foundations, so figure recognition there is not yet established.
- Scanned books need OCR uncertainty that this extractor cannot report. The probe did not select any of them.
