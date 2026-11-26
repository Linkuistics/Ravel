# Catalogue audit figures

The milestone request in `docs/milestones/vertical-slice.md` asks for a check of the earlier audit figures. It also asks for an explanation of any discrepancy. This document records the result of that check.

## Input

The check reads the Zotero RDF export in `inputs/`. Version control ignores that directory, because it holds personal library data. The RDF snapshot has this SHA-256:

`a37c41cea354d06ed375fdd2d7ff4fab7e66d9ae5c7d60e3e9121fcf73f22be7`

The RDF holds no attachment file paths. The Zotero database supplies them (ADR `source-identity`).

## Figures

The RDF alone gives the three audit figures:

| Figure | Count |
|---|---|
| Catalogue records | 2,723 |
| Books | 1,280 |
| Computing books | 841 |

A Computing book is a book in the top-level Computing collection or in one of its subcollections. The check found no other rule that gives the figure 841.

## The discrepancy

An earlier count gave 2,757 catalogue records. That count included 34 live items from a group library. The RDF export omits the group library, so the export holds 2,723 records.

## Document files

The 1,280 books have these document files:

| Document files per book | Books |
|---|---|
| 1 PDF | 1,247 |
| No PDF | 1 |
| More than 1 PDF | 32 |

The 32 books with more than one PDF were the first ambiguity cases for the source manifest.

## Workbook

The audit workbook `zotero_corpus_audit.xlsx` is not available. No catalogue record therefore has an R-ID. An audit that is rebuilt from the RDF must not reuse R-IDs, because an R-ID is export-scoped (`CONTEXT.md`).
