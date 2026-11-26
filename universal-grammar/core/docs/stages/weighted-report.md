# Baseline and weighted reports

The `weighted-report` stage scores the fixed patterns of one discovery run twice. The first scoring is the equal-book baseline. The second scoring applies a preference weight file for books and sections. The stage then compares the two scorings.

The stage reweights fixed patterns. It does not run pattern discovery again, and it does not call the model. A pattern that a discovery run under other weights could find does not appear. Each page states this limitation and names the discovery run.

## Commands

Build both reports for the newest discovery run with the illustrative files:

```sh
task weighted-report
```

Build both reports with other files, or for another discovery run:

```sh
uv run urg weighted-report --weights <file> --relevance <file> --discovery <key>
```

The command prints the path of each report. It also says whether it built the report or reused a stored one. It then lists the upstream research artifacts that it read.

## Research artifacts

| Artifact | Payload | Inputs in the dependency record |
| --- | --- | --- |
| `baseline-report` | `baseline.html`, `scores.json` | The discovery run, the chapter extraction and episode analysis of each book, the relevance file |
| `weighted-report` | `weighted.html`, `comparison.html`, `scores.json` | The baseline report, the preference weight file, the same upstream artifacts, the relevance file |

The preference weight file is not an input of the baseline report. A change to the weight file therefore writes only a new `weighted-report`. The stage reuses the extractions, the analyses, the discovery run and the baseline report. The dependency record of the new `weighted-report` names the same upstream keys as before.

## Inputs

The stage keeps four inputs separate. Only the preference weights change a number.

| Input | Source | Where it shows |
| --- | --- | --- |
| Preference weights | The weight file, `docs/weights/illustrative-weights.json` by default | The prevalence, the book shares and the weight table |
| Relevance assessments | The relevance file, `docs/weights/illustrative-relevance.json` by default | A column of counts by level |
| Extraction uncertainty | The defects of each chapter extraction | A column of counts and defect IDs |
| Interpretation uncertainty | The uncertainty level and the alternative readings of each interpretation | A column of counts by level |

The report never combines these inputs into one score.

### Preference weight file

| Field | Content |
| --- | --- |
| `version` | `1` |
| `status` | `illustrative` or `human` |
| `statement` | A sentence that says who chose the weights |
| `default_book_weight` | The weight of a book that `books` does not name |
| `books` | A map from Zotero item key to `{"weight", "reason"}` |
| `default_section_weight` | The weight of a section that has no configured ancestor |
| `sections` | A map from section ID to `{"weight", "reason"}` |

A weight is a finite number that is zero or more. A section weight applies to the section and to its descendants. The nearest configured section wins.

The human supplied no preferences for this slice. The default file therefore has the status `illustrative`. Each page that uses it says that the weights are illustrative and are not the human's preferences.

### Relevance file

The relevance file has `version`, `status`, `statement`, `books` and `sections`. Each entry is `{"level", "reason"}`, with the level `high`, `medium` or `low`. The level of a section applies to its descendants. A section without a level takes the level of its book. A book without a level is `not assessed`.

## Aggregation formula

The stage works on the listed observations of the discovery run. A listed observation is an inventory entry in the layer `structure-section` or `interpretation`. These entries are the observations that a pattern can cite.

For book *b* and observation *o* in *b*:

- *w_b* is the book weight. The baseline sets *w_b* = 1.
- *v_o* is the observation weight. The baseline sets *v_o* = 1.
- Under preference weights, *v_o* is the mean section weight of the sections that hold the evidence of *o*. A section observation uses its own section.
- An interpretation whose evidence lies in no located section takes the default section weight.

The denominator of book *b* is *D_b* = Σ *v_o* over the listed observations of *b*.

The share of book *b* is *s_b* = *w_b* / Σ *w_c*. The sum runs over the counted books *c*. The shares of the counted books add up to 1.

For pattern *P*:

- *f_b(P)* = (Σ *v_o* over the supporting instances of *P* in *b*) / *D_b*.
- prevalence(*P*) = Σ *s_b* · *f_b(P)* over the counted books.
- exception prevalence(*P*) uses the exceptions of *P* in place of the supporting instances.
- book coverage(*P*) = Σ *s_b* over the books where *P* has a supporting instance with a positive weight.

The rank orders the patterns by prevalence. Equal prevalences share the better rank.

### Normalisation

Each book first becomes a fraction of its own listed observations. The shares then combine the fractions. A book with more observations therefore gains no influence from its length. Under the baseline, each counted book has the share 1 / *n*.

### Supporting instances

Only the observations that the pattern cites directly as supporting instances count. Discovery also lets a section fit a pattern through an ancestor section. The report does not count such indirect fits. Unresolved cases do not enter the prevalence.

## Missing and unanalysed material

- A book that the discovery run did not analyse has no observations. It has no share and no denominator. A weight or relevance entry that names it appears in the list of ignored entries.
- A section that is not a section of an analysed chapter appears in the list of ignored entries.
- A counted book needs a positive denominator and a positive book weight. Otherwise the report excludes the book, states the reason and renormalises the other shares. The stage refuses weights that exclude every book.
- A section weight of zero removes the observations of that section from the numerator and from the denominator.
- A cited observation that is not in the inventory does not count. The report lists it under the pattern as not counted.
- A defect in the extraction does not change a weight. The extraction-uncertainty column counts the supporting instances whose evidence lines or pages touch a defect. It also names the chapter-level defects of each book that the pattern cites.
