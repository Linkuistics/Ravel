# Automated book evidence for k132

These measurements date from 14 November 2026 on native arm64 macOS.
They establish the results below for the frozen inputs. They do not complete the proof.

## Inputs and reconciliation

`reconciliation.json` compares every subject in seven earlier manifests with its current digest.
The five source inventories are k125–k129. The other manifests cover gates k123 and references k131.

| Manifest | Subjects | Changed subjects | Missing subjects |
|---|---:|---:|---:|
| CLI k125 | 196 | 4 | 0 |
| Configuration k126 | 296 | 2 | 0 |
| Lifecycle k127 | 369 | 1 | 0 |
| Components k128 | 372 | 1 | 0 |
| Workflows k129 | 379 | 0 | 0 |
| Gates k123 | 354 | 4 | 0 |
| References k131 | 8 | 0 | 0 |

All changed subjects are book chapters 4, 6, 8 and 10.
The earlier producers corrected the example location, host-read reply limit, result-directory precedence and notices-test explanation.
`book-changes-since-k125.diff` preserves those changes.
All measured implementation inputs match the earlier manifests.
Digest agreement does not establish semantic completeness of an inventory.

`subjects-before.sha256` names all 437 measurement inputs.
It includes the book, checker sources, fixtures, tools, pins, earlier evidence and measured implementation files.
It excludes this evidence directory and Grove notes.
After both runs completed, this command verified every input and returned 0:

```sh
shasum -a 256 -c docs/evidence/rhai-book-proof-k132/subjects-before.sha256
```

`subjects-verification.log` records each result.
`subjects-after.sha256` copies the verified manifest. No subject changed during these measurements.

## Commands and observations

Each command below returned 0. Each log has one writer.

```sh
mise exec -- task setup
mise exec -- task docs:book:build docs:examples docs:excerpts docs:figures docs:ste docs:breaks
mise exec -- task book:integrity docs:figures:test docs:examples:test docs:examples:mutants docs:excerpts:test docs:ste:test docs:breaks:test
```

`setup.log`, `checks.log` and `controls.log` retain the corresponding output.
The checks observed these results:

- The mdBook build completed.
- All 27 complete examples passed. The checker reported 73 fragments and no page errors.
- The chapter 11 example with only `policy.ts` refused with `config_format_retired`, empty standard output and exit status 3.
- All 22 exact source excerpts passed.
- All 21 figures matched their sources with Typst 0.15.1, CeTZ 0.4.2 and DejaVu 2.37.
- The STE report found no problems in its 35 input files.
- The break report found no prose run above 450 words in any of the eleven chapters.

The controls checked clean fixtures and deliberate defects.
The example controls killed all 30 mutants. The excerpt controls killed all 24 mutants.
The STE and break controls rejected their listed mutations.

The figure controls exercised tool and cache corruption, changed figures, changed layout records and environment isolation.
Their single-page mdBook fixture checked copied SVG bytes, the image reference and alternative text.
The mdBook integrity controls checked cache reuse, corrupt-cache recovery and refusal of corrupt downloads, failed downloads and wrong pins.

## Private writing-rule source

The owner restored access to `/Users/antony/Downloads/ASD-STE100_ISSUE9.pdf` on 14 November 2026.
An earlier checksum attempt timed out after 12 seconds.
The successful SHA-256 check matched the required digest:
`d1f4ea9e7cd6e46b47aa9057209f99e78c0e9cfc4e27a5b07895b05c1a166431`.

The consultation covered Part 1 rules for verbs, sentences, procedures, descriptions, safety, punctuation and word counts.
It also covered the advice about consistent wording and unambiguous references.
The PDF and extracted text remain outside this repository.
This consultation does not establish that the book follows those rules.
The book contract excludes the dictionary and requires British spelling.

## Limits and open proof work

The fresh whole-book read and full contract reconciliation remain open.
All figure surfaces remain unviewed in this session. Reproducible bytes do not establish visual quality.
The full local-link and built-destination checks remain open.
The single-page figure fixture does not establish all book links or published rendering.

Owner Read 2 remains required after proof. No model read substitutes for it.
These measurements do not establish Linux behaviour, real harness behaviour or novice comprehension.

`coverage.json` records the graph coverage checks for the frozen inputs and cited notes.
The graph reported partial shell parsing and intentional exclusions for generated figures and tool directories.
Direct source reads covered the reported partial shell ranges.
Hash verification treats excluded assets as bytes. It makes no structural or visual claim about them.
