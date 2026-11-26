# Universal Refinement Grammar

Universal Refinement Grammar (URG) discovers and represents how authors explain technical subjects. It works at four levels: whole books, chapters, sections and explanatory episodes.

The aim is descriptive. URG recovers explanatory structures, relationships and recurring patterns from real books. It does not train a model, and it does not measure how well a book teaches.

## Principles

- **Evidence stays separate from interpretation.** Each interpretation cites the source passages that support it, with their page and coordinate anchors.
- **The corpus helps to reveal the categories.** Each label is provisional. URG keeps the observations that do not fit the current categories.
- **Derived results never replace observations.** A pattern names its supporting instances, its exceptions and its unresolved cases.
- **Weights and uncertainty stay distinct.** URG records preference weights, relevance assessments, extraction uncertainty and interpretation uncertainty as separate inputs. It has no combined confidence score.
- **Reuse depends on dependency records.** A stage reuses an output only when the identities and versions of all its inputs are unchanged.

`CONTEXT.md` defines the vocabulary. The decision records in `docs/adr/` give the reasons.

## Status

The first milestone is a vertical slice. It imports a Zotero catalogue, extracts one chapter from each of three books and analyses each chapter with a model. It then derives candidate patterns and builds an equal-book baseline report and a weighted report.

`docs/milestones/vertical-slice-completion.md` reports what works and what does not. One known gap is that the recorded correction applies only in the store in which it was written.

## Pipeline

| Stage | Task | Calls the model |
|---|---|---|
| Catalogue import and source manifest | `task catalogue-manifest` | No |
| Chapter extraction | `task chapter-extraction` | No |
| Episode analysis | `task episode-analysis` | Yes |
| Corrections | `task corrections` | No |
| Pattern discovery | `task pattern-discovery` | Yes |
| Baseline and weighted reports | `task weighted-report` | No |

`task pipeline` runs the six stages in order. Each stage has a document in `docs/stages/`.

## Requirements

- [mise](https://mise.jdx.dev/), which installs the tools that `mise.toml` pins: Python, uv, task, ruff and pyright.
- A Zotero library in `~/Zotero`, with its PDF attachments.
- A Zotero RDF export of that library at `inputs/Zotero Library Export.rdf`.
- The Claude Code command `claude`, for the two stages that call the model.

## Use

1. Install the pinned tools.

   ```sh
   mise install
   ```

2. Install the locked Python environment.

   ```sh
   task sync
   ```

3. Run the lint, the type check and the tests. The tests need no Zotero library and no model.

   ```sh
   task check
   ```

4. Run the pipeline. This step reads the Zotero library and calls the model.

   ```sh
   task pipeline
   ```

To see each task, run `task --list`.

## Data and privacy

The repository holds the code, the documents, the correction file and the run transcripts. It does not hold library data.

Version control ignores `inputs/`, `data/` and `reproductions/`. These directories hold the personal library export and the text that URG derives from the books.

## Layout

| Path | Contents |
|---|---|
| `urg/` | The pipeline stages, the artifact store and the command-line interface |
| `tests/` | The tests, which build their own fixture documents |
| `docs/adr/` | The decision records |
| `docs/stages/` | One document for each stage, with its review |
| `docs/milestones/` | The milestone request, the completion report and the reproduction transcript |
| `corrections/` | The recorded corrections to interpretations |
| `scripts/` | The scripts that write the transcripts |

## Licence

URG has the Apache License, Version 2.0. The file `LICENSE` holds the full text.
