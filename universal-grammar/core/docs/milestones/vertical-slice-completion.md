# Vertical slice: completion report

This report closes the milestone in `docs/milestones/vertical-slice.md`. It answers the nine items that the milestone asks for at completion.

## Outcome

The pipeline runs end to end on real data, in two parts. On 18 November 2026, `task pipeline` started on an empty data directory and stopped at the corrections stage. The transcript script then ran the last two stages one at a time. Together the two parts read the Zotero library, extracted one chapter from each of three books and analysed each chapter with the model. It then discovered 26 candidate patterns and built an equal-book baseline report and a weighted report.

The run made four model calls and wrote 20 research artifacts. A second run over the same store made no model call and wrote no artifact.

One part of the slice does not work on a fresh store: the recorded correction. The correction names an interpretation by its identifier. A new analysis gives each interpretation a new identifier, so the corrections stage refuses the correction and exits with status 1. `task pipeline` stops at that stage, and the transcript then runs the last two stages one at a time.

The correction and its downstream effects work on the store in which they were written. Section "The demonstrated correction" gives that evidence. The last section proposes the step that closes this gap.

The run also found one fault in the extraction stage. The extraction stage looked up the source manifest by the key that the chapter selection recorded. A clean store held a manifest with another key, so extraction stopped. The stage now reads the latest manifest in the store. Section "What changed in this leaf" gives the details.

## How to read the evidence

Each claim below cites a research artifact by its name and key. An artifact lives in the store at `artifacts/<name>/<key>/`. Two stores hold the evidence:

- The **reproduction store** is `reproductions/2026-11-18/data/`. The run that this report cites built it. The transcript of that run is `docs/milestones/vertical-slice-reproduction.txt`.
- The **working store** is `data/`. The earlier leaves built it, and it holds the correction and the reuse demonstrations.

Both stores are outside version control, because they hold text from the library's books. Version control holds the code, the documents, the correction file and both transcripts.

## Implemented behaviour and unverified integration

The reproduction transcript shows the following behaviour on real data:

- Each of the six stages runs from an empty store and writes its outputs. `task pipeline` runs the first four stages and stops when the corrections stage refuses `c001`. The transcript script runs discovery and the report stage one at a time.
- A restart reuses each valid output and makes no model call.

The working store and `docs/resumability-transcript.txt` show more behaviour on real data:

- A correction regenerates the derived results without a new extraction.
- A damaged output is rejected and rebuilt.
- A killed write is never read as an output.
- A weight change builds one new weighted report and reuses everything upstream.

The following integration is not verified, or it fails:

- **Correction on a fresh store: fails.** Section "The demonstrated correction" explains the failure.
- **Grove resumes a stage: not verified.** Grove ran the leaves of this work, and each leaf ran the stages through the Taskfile. No session tested a Grove resume in the middle of a stage. The resume evidence comes from the store, at the level of each stage.
- **Corpus scale: not verified.** The catalogue stage reads all 2,723 records. The later stages ran on three chapters only.
- **Workbook mapping: not available.** The audit workbook `zotero_corpus_audit.xlsx` is missing, so no record has an R-ID.

## 1. What was implemented, and where

The code is the Python package `urg/`, with tests in `tests/` and recipes in `Taskfile.yml`. Each stage has a document in `docs/stages/` that states its inputs, outputs, reuse rule and limits.

| Stage | Task | Code | Output artifacts |
|---|---|---|---|
| Catalogue and source manifest | `catalogue-manifest` | `urg/zotero_rdf.py`, `urg/zotero_db.py`, `urg/manifest.py` | `export-snapshot`, `source-manifest`, `alias-table` |
| Chapter extraction | `chapter-extraction` | `urg/extraction.py`, `urg/chapter_layout.py` | `chapter-extraction` per book, `chapter-extraction-run` |
| Episode analysis | `episode-analysis` | `urg/analysis.py`, `urg/model.py` | `episode-analysis-response` and `episode-analysis` per book, `episode-analysis-run` |
| Corrections | `corrections` | `urg/corrections.py` | `analysis-corrections-run`, a corrected `episode-analysis` per corrected book |
| Pattern discovery | `pattern-discovery` | `urg/discovery.py`, `urg/report.py` | `pattern-discovery-response`, `pattern-discovery`, `pattern-report` |
| Baseline and weighted reports | `weighted-report` | `urg/weighting.py`, `urg/weighted_report.py` | `baseline-report`, `weighted-report` |
| Change summary | `change-summary` | `urg/changes.py` | `change-summary` |

The store is `urg/store.py`. Each artifact has a dependency record. The record names the stage, the schema version, the code version, the configuration and each input by role and hash. The artifact key is the SHA-256 of that record.

`docs/resumability.md` describes the store and its six claims. `CONTEXT.md` defines the research terms, and `docs/adr/` records the five design decisions. `docs/representation.md` describes how the observations, patterns and grammar fragments relate.

The model boundary is `urg/model.py`. It runs `claude` in print mode with no tools. `docs/analysis/model.json` and `docs/discovery/model.json` record the model configuration: `claude-opus-5-5` at high effort. The tests replace the boundary with recorded responses.

## 2. Commands to reproduce the milestone

The inputs are `inputs/Zotero Library Export.rdf` and the Zotero data directory `~/Zotero`. The catalogue stage copies `zotero.sqlite` and refuses a database with a non-empty write-ahead log. Close Zotero before a run.

1. Install the tools with `mise install`.
2. Run every stage with `task pipeline DATA_DIR=<empty directory>`.
3. When the corrections stage fails, run `task pattern-discovery DATA_DIR=<directory>`.
4. Then run `task weighted-report DATA_DIR=<directory>`.
5. Run the checks with `task check`.

The script `scripts/reproduction-transcript.sh RUN_DIR` performs steps 2 to 4 and then a restart, and it records both runs. It accepts a failure of `task pipeline` only at the corrections stage, and only when each refusal names an interpretation that no analysed book holds. It exits 1 when another check fails. It refuses a directory that exists.

A shim named `claude` logs each model call. In the restart the shim refuses each call, so a count of zero shows that the restart made no model call. This report cites the output of this command:

```
scripts/reproduction-transcript.sh reproductions/2026-11-18 > docs/milestones/vertical-slice-reproduction.txt
```

The script `scripts/resumability-transcript.sh WORK_DIR` records the reuse demonstrations on clones of the working store. `docs/resumability.md` maps each section of its output to a claim.

`task change-summary` compares two discovery runs and needs no model. `docs/stages/corrections.md` gives its arguments.

### What the reproduction run did

The run used commit `e154748` plus the changes of this leaf. The working copy differed from the commit in 7 paths when the run started. Section R0 of the transcript records both facts.

- **R1, first run.** Section R1 of the transcript records each stage:
  - The catalogue stage wrote source manifest `8d58abdc…`.
  - The extraction stage extracted the three chapters.
  - The analysis stage made three model calls, one for each chapter.
  - The corrections stage refused `c001` and exited with status 1.
  - The discovery stage made one model call and found 26 patterns.
  - The report stage built the baseline report `a657a851…` and the weighted report `7bb31fc4…`.

  The store went from 0 to 20 artifacts, and the shim counted 4 model calls.
- **R2, restart.** R2 repeats the same commands. `task pipeline` stopped again at the corrections stage, with the same refusal and status 1. Each stage that ran reported "reused", and the corrections stage refused `c001` without a write. The store stayed at 20 artifacts, and the listing of the store did not change. The shim counted 0 model calls.

The script at commit `46599e7` recorded this run. That version printed each outcome but did not check it. It also ran the last two stages after any failure of `task pipeline`, not only after the expected refusal.

The present script, from this leaf, checks each outcome and exits 1 when one check fails. Section "What changed in this leaf" lists its checks. No run repeated it with the present script. A new run calls the model four times and changes every key that this report cites.

## 3. The three books and chapters

`docs/probes/extraction-probe.md` records the probe of ten books. `docs/probes/chapter-selection.json` records the selection.

| Item key | Book | Chapter | PDF pages (from 0) | Evidence lines | Structure |
|---|---|---|---|---|---|
| `6IQTDYAW` | *Crafting Interpreters* (2021) | 7, "Evaluating Expressions" | 109–122 | 711 | 176 paragraphs, 17 headings, 68 code blocks, 3 figures |
| `H3MVK7J5` | *Practical Foundations for Programming Languages* | 5 | 55–63 | 406 | 96 paragraphs, 6 headings, 31 inference rules, 4 code blocks |
| `PZZBPXX9` | *Regular Expressions Cookbook* | 6 | 390–423 | 1,245 | 365 paragraphs, 81 headings, 124 code blocks |

The three chapters differ in kind. The first is a tutorial with running code and side notes. The second is a formal text with inference rules. The third is a set of recipes. `docs/stages/chapter-extraction.md` gives these figures and the defects that the extraction recorded for each chapter.

In the reproduction run, the analysis found these interpretations:

| Book | Episodes | Moves | Relationships | Alternative readings | Observations |
|---|---|---|---|---|---|
| `6IQTDYAW` | 24 | 38 | 10 | 3 | 5 |
| `H3MVK7J5` | 23 | 26 | 12 | 3 | 4 |
| `PZZBPXX9` | 43 | 39 | 15 | 3 | 6 |

The earlier analysis in the working store found 23, 21 and 34 episodes. The instructions, the model and the extraction method are the same. The difference shows how much one model call varies.

## 4. Examples that link candidate structures to source evidence

Each example names a pattern in discovery `219849511e73…` of the reproduction store. The pattern report `b9030c4c…` shows each pattern with links to its observations and to page renders. Its `index.html` holds 378 links to 344 distinct targets, and the artifact holds 603 files.

### A pattern found in all three books: "survey alternatives then choose"

Pattern `pattern:176068ac96ab:p013` states: "The author surveys or names alternative designs, often with other languages as precedent, and then rejects some and adopts one with a reason." It proposes the rule `DesignChoice -> Alternatives+ Rejection* Adoption Justification`. It cites 8 supporting observations from the three books, 0 exceptions and 2 unresolved observations. Its uncertainty is medium, because the instances vary from full surveys to single rejected variants.

One supporting observation is move `6IQTDYAW:976d6b678278:i027`, "surveys design space". The chain from the pattern to the page is as follows:

1. The pattern cites the move with the note "Truthiness options are surveyed before Ruby's rule is adopted."
2. The move belongs to episode `i025`, "digression to semantic concept". That episode cites elements `e0067` to `e0086`.
3. The move cites five paragraphs on printed page 101, which is PDF page index 113. One of them is element `6IQTDYAW:e0072`. Its text starts: "We could just say it's an error because we don't roll with implicit conversions, but most dynamically typed languages aren't that ascetic."
4. Element `e0072` is built from six evidence lines, `6IQTDYAW:p0113-b010-l00` to `l05`, in extraction `467c0617…`. Each line holds its page, bounding box and text.
5. The report holds a render of the element: `render-6IQTDYAW_e0072.png`.

The analysis also recorded an alternative reading of the same passage. Interpretation `i074` reads the section mainly as a language-design discussion, with the code secondary, and its uncertainty is high. The pattern lists `i074` as unresolved, not as support. In this way an alternative reading stays visible beside the pattern that it competes with.

### A pattern from document structure: the recipe form

Pattern `pattern:176068ac96ab:p001`, "Problem-Solution-Discussion-See Also recipe", rests on sections, not on model interpretations. It proposes `Recipe -> Problem Solution Discussion Variations? SeeAlso`. It cites 10 recipes of `PZZBPXX9` as support.

Recipe 6.1 is section `s002`, followed by sections `s003` to `s006`. It cites 2 exceptions. Recipe 6.12 adds a Variations section, and recipe 6.13 adds a section after Discussion. The extraction found each section from the headings of the PDF, so a reader can check each instance against the chapter's outline.

### What the patterns do not cover

The discovery listed 104 sections and 254 interpretations. The 26 patterns leave 203 observations unfitted, and the report lists each of them. Fourteen patterns rest on one book only. `docs/representation.md` describes the patterns of the working store in the same terms.

## 5. The demonstrated correction

The correction is `corrections/c001-unary-not-ends-implementation.json`. It revises one episode in chapter 7 of *Crafting Interpreters*, section 7.2.3. The analysis ended the episode at element `e0066`. The correction ends it at `e0068`, where the text adds the second unary operator, logical not. The correction also raises the uncertainty from low to medium. The correction file records its author, its origin and its reason.

The correction first named episode `6IQTDYAW:0fbbac3a863e:i022`, in the analysis that built the working store. The evidence in section "Downstream effects in the working store" and the reproduction run use that target. Session `resumability-k27` later moved the target to `6IQTDYAW:4c555fc18422:i023`. That is the same episode in the analysis from the code of commit `38f81e0`, and it also ends at `e0066`. The present file names this second target.

### Downstream effects in the working store

`docs/stages/corrections.md` records the regeneration. Corrections run `43c0bff6…` wrote corrected analysis `d7f5326b…` for `6IQTDYAW` only. Change summary `4a3f921d…` compares discovery `f1e40eb0…` before the correction with discovery `801dacaa…` after it:

- The three extractions and the three model responses were reused. No chapter was extracted again.
- The analyses of `H3MVK7J5` and `PZZBPXX9` were reused. The analysis of `6IQTDYAW` changed from `8cda2b67…` to `d7f5326b…`.
- Discovery made one new model call. The pattern report changed from `7983c6f1…` to `1c462105…`.
- The report artifacts kept 594 files and changed 4. The renders gained 76 and lost 58.
- 18 patterns matched across the two runs, and each of them changed. 2 patterns disappeared and 2 appeared. Unfitted observations went from 226 to 239.

The change summary attributes 3 differences to the correction, 155 to the new discovery call and 14 to either cause. The correction changed no move of episode `i022`, so its causal link to the patterns is weak. One discovery call varies more than one correction changes. The record of a correction is therefore exact, but this data cannot measure its effect on the patterns.

Section S7 of `docs/resumability-transcript.txt` repeats the mechanism on the working store with the present target. The corrections stage applied `c001` to `6IQTDYAW:4c555fc18422:i023`. It wrote corrections run `d552be30…` and corrected analysis `6079772b…`. The pipeline reused the analyses of `H3MVK7J5` and `PZZBPXX9`.

Discovery made one new model call, because its input changed, and wrote discovery `b827a824…`. The report stage then wrote baseline `e748a6fb…` and weighted report `e8d22b50…`. `task pipeline` exited 0, and the store went from 99 to 106 artifacts. In S8 a restart with each model call refused wrote nothing.

### The correction on a fresh store

The reproduction run refused the correction, which then named its first target: "no analysed book holds interpretation 6IQTDYAW:0fbbac3a863e:i022". Corrections run `f910d75a…` records the refusal and the three analyses that it read. The identifier of an interpretation includes the key of the model response. A new response gives new identifiers, so a correction cannot follow its target into a new analysis.

The new analysis repeats the error that `c001` corrects. Episode `6IQTDYAW:976d6b678278:i020`, "incremental code with key insight", again cites elements `e0051` to `e0066`. A correction that named its target by evidence would find this episode.

## 6. Equal-book baseline compared with the supplied weights

The report stage scores the patterns of one discovery run twice. The baseline gives each analysed book the same share. The weighted report applies `docs/weights/illustrative-weights.json`. Neither report repeats discovery, and neither makes a model call. `docs/stages/weighted-report.md` states the formula, the normalisation, the denominators and the handling of missing material.

The supplied weights are illustrative. An agent chose them to show the arithmetic. They are not the human's preferences, and both the file and the report say so.

They give `6IQTDYAW` a book weight of 2.0 and `PZZBPXX9` a book weight of 0.5. They give section `H3MVK7J5:s006` a weight of 0 and section `6IQTDYAW:s013` a weight of 0.5. The relevance file `docs/weights/illustrative-relevance.json` is also illustrative, and the report shows it in its own column. A relevance level does not change a score.

In the reproduction store the book shares change from 1/3 each to 0.571, 0.286 and 0.143. Weighted report `7bb31fc4…` names baseline `a657a851…` and discovery `219849511e73…`. A selection of its rows follows:

| Pattern | Books | Baseline rank | Weighted rank | Baseline prevalence | Weighted prevalence |
|---|---|---|---|---|---|
| `p013` survey alternatives then choose | All three | 3 | 1 | 0.0253 | 0.0392 |
| `p011` forward reference bounds scope | `6IQTDYAW`, `H3MVK7J5` | 1 | 3 | 0.0273 | 0.0357 |
| `p001` recipe form | `PZZBPXX9` only | 7 | 17 | 0.0178 | 0.0076 |
| `p003` solution catalogue by variant | `PZZBPXX9` only | 11 | 20 | 0.0143 | 0.0061 |
| `p014` side note exposes semantic decision | `6IQTDYAW` only | 8 | 4 | 0.0172 | 0.0286 |
| `p018` special case handled last | `6IQTDYAW` only | 23 | 11 | 0.0069 | 0.0118 |

The changes follow from the weights. Patterns that rest on `PZZBPXX9` alone fall or stay last. The three patterns that rest on `6IQTDYAW` alone rise. Patterns `p008`, `p013`, `p015` and `p017` have weighted support in all three books, so each keeps a book coverage of 1.0. The comparison page `comparison.html` in the weighted report shows each of the 26 rows.

The comparison reweights one unchanged set of patterns. It does not show which patterns a discovery under these weights would find, and both reports state that limit. The discovery artifact records its settings, instructions, model configuration and inputs. A later discovery under other weights can use that record.

## 7. Interruption recovery and reuse

`docs/resumability.md` states six claims and names a test for each. `docs/resumability-transcript.txt` shows each claim on real data. Its sections S2 to S7 ran on the working store or on clones of it, with each model call refused unless the section says otherwise. The table cites the run of that transcript that session `resumability-k27` recorded at commit `38f81e0`.

| Claim | Evidence | Result |
|---|---|---|
| A restart repeats no valid work | S2 and reproduction section R2 | The store stayed at 99 and at 20 artifacts. 0 model calls. |
| Each record names its inputs and versions | S3 | Every dependency record, listed by shape |
| A killed write is not an output | S4 | The kill came before the rename of the second chapter. One extraction reached the store, and the second stayed in `staging/`. The rerun built the 3 missing artifacts with 0 model calls. The clone then held no artifact that the working store lacks. |
| A damaged output is rejected and rebuilt | S5 | One flipped byte in analysis `8343a0ea…` of `H3MVK7J5` made the store reject it. The rerun moved it aside and rebuilt it from the stored response. 0 model calls. |
| A correction invalidates the right results | S7 and section 5 | `c001` applied on the working store. The corrected analysis and corrections run were new. Discovery needed a new model call. The store went from 99 to 106 artifacts. |
| A weight change reuses upstream work | S6 | One new weighted report. 0 model calls. |

The reuse rule depends on content, not on the presence of a file. A change to any input, setting or code version gives a new key, so the old output does not match. The reproduction run shows the same rule from the other side. The code of this leaf differs from the code that built the working store. No output of the working store was therefore valid for the reproduction run.

## Remaining limitations

### Extraction

- The extraction reads the PDF text layer only. It does no OCR.
- A paragraph is a text block of the PDF, and reading order is block order. *Crafting Interpreters* had 11 reading-order defects, and *Practical Foundations* had 5.
- Side notes of *Crafting Interpreters* become paragraphs after the body text of the page.
- A code block is found by its monospace font. A figure is a cluster of images or vectors over a size threshold.
- An inference rule is a bar and a centred line. Fractions and subscripts are lost. *Practical Foundations* also had 8 text-layer defects and 8 ligature defects.

### Representation and analysis

- The analysis reads text only. It knows a figure through its caption.
- Each chapter takes one model call, so the chapter must fit in the context window and the output limit.
- The model chooses the episode boundaries, and two calls give different boundaries. The episode counts in section 3 show this.
- Validation proves that each citation exists. It does not prove that the evidence supports the interpretation.
- One discovery call proposes every pattern. The checks prove that each citation resolves, not that the pattern is apt. The minimum support counts instances, not books.
- The discovery prompt lists sections and interpretations, not the elements of the text.
- The renders in a report need the source PDF.
- No human reviewed a pattern or an interpretation. Each one has the status "proposed".

### Integration

- A correction names its target by an identifier that changes with each new analysis. On a store whose analyses differ from the target's, the corrections stage refuses `c001`, and `task pipeline` stops before discovery. The reproduction store is such a store.
- After that failure, discovery runs on the analyses without the correction. Nothing in the transcript warns about it except the refusal.
- A change to `urg/store.py` makes every stored output stale, because the code version covers the import closure of each stage.
- A killed write leaves a directory in `staging/`, and that directory stays there.
- The cause labels in a change summary come from a rule, not from a controlled comparison.
- The weights and relevance levels are illustrative. No human preference has been applied.

## What changed in this leaf

The first clean run failed at extraction with this message:

```
error: the source manifest eccb9a1d42774e89b70a63300322d49a637607de174827155bdb7759c175d742 named by the chapter selection is not in the store
```

The chapter selection recorded the key of the manifest that the probe read. Each change to the catalogue code gives the manifest a new key, so a clean store held manifest `8d58abdc…` only. The working store still held the old manifest, so the earlier runs passed.

The extraction stage now reads the latest source manifest in the store. It still checks each document file against the SHA-256 in the selection, and the run report records the key of the manifest that it read. Two tests cover the change, in `tests/test_chapter_extraction.py`.

One shows that a selection whose manifest key is absent still extracts. The other shows that an empty store names the stage to run first. `docs/stages/chapter-extraction.md` describes the rule. `scripts/reproduction-transcript.sh` is new.

The integration of the review in `docs/milestones/vertical-slice-completion-review.md` added these changes:

- The extraction stage matches a manifest record only when the whole source identity is equal: the library and the item key. An equal item key in another library no longer serves the selection.
- Two new tests store two manifests from two catalogue code versions. In the first, the selection pins the older manifest, and the stage must read the latest. In the second, the latest manifest records the item key in another library, and the stage must refuse. A temporary change that preferred the pinned manifest failed both tests. A temporary change that compared the item key only failed the second test.
- The reproduction script checks each outcome. Each stage must exit 0, except the corrections stage when each refusal names an absent interpretation. The restart must make no model call and must leave the store unchanged. The SHA-256 inventory of the changed paths, the correction files and the run's own code must be the same before and after the run. The script prints the full commit, then exits 1 and lists each failed check when one fails.

The change to `urg/extraction.py` changes the code version of the stages that import it. The next run on the working store will therefore repeat the analysis and discovery model calls.

## The next smallest step

**Make a correction name its target by evidence, so that it survives a new analysis.** Two observed cases justify this step:

- A new analysis gives the target a new identifier, so the correction fails and `task pipeline` stops. This happened in the reproduction run. It also happened in the first attempt of session `resumability-k27`, until a person moved the target by hand.
- The new analysis repeats the error that `c001` corrects. Episode `i020` cites elements `e0051` to `e0066`, the same span as the corrected episode `i022`. A target named by the interpretation kind and its first evidence element finds this episode. The current identifier cannot.

The step is small. The corrections stage would resolve a target by kind and first element when its identifier is absent. It would refuse a target that matches no interpretation or more than one, and it would record the match in the corrections run. A test can use the two analyses in hand: `d7f5326b…` and `b7844c06…`.

The step after that is to measure how much one analysis call varies. Two calls on chapter 7 of *Crafting Interpreters* gave 23 and 24 episodes, and two calls on the recipe chapter gave 34 and 43. 155 pattern differences came from one new discovery call, and 3 came from the correction. Until that variation is measured, no comparison between two runs can show what a correction or a weight changed.
