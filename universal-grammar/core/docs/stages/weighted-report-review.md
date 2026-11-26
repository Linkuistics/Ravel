# Weighted-report implementation review

No material findings remain. The implementation meets the fixed-pattern
reweighting contract. The three sampled patterns reproduce the saved scores,
the pages keep preferences and uncertainty separate, and a weight change
preserves the baseline and every upstream artifact.

This review covers `weighted-reports-k9`, producer commit
`765e1d04fa731e0c18642c2214cef7e64489bae9`, against source at `7ea5937d`.
The weighting module and its tests are unchanged from the producer. The report
module's later changes replace the two `store.find` calls with `store.reuse`.
No production or test code was changed. No tests, builds, lint, format commands
or model calls were run during this review. Independent arithmetic read the
saved JSON without importing the production modules.

## Formula and observation weights

The [formula document](weighted-report.md) agrees with
[the scoring code](../../urg/weighting.py). For each book, the denominator is
the total weight of its listed `structure-section` and `interpretation`
observations. Direct supporting citations form the prevalence numerator;
direct exception citations form the exception numerator. Duplicate citations
count once. The normalized book shares combine those fractions. Book coverage
is the sum of shares of books with positive supporting weight.

The baseline uses observation weights of one and equal book shares. It
therefore normalizes chapter length within each book before combining books.
The preference configuration changes book shares and observation weights.
Zero book weights or zero denominators exclude a book and renormalize the
remaining shares; excluding every book is an error. Missing or unanalysed
material follows the documented exclusion rules.

An interpretation takes the mean weight of the distinct sections containing
its evidence. Section ownership is reconstructed through structure elements
and evidence lines. The nearest configured ancestor supplies an inherited
section weight. An interpretation with no located section takes the default
section weight, as `weighted_scores` explicitly implements. Every real
observation in this discovery has a located section, so the last branch was
checked in source rather than exercised by the real corpus.

The independent reconstruction gives these denominators. Book order in all
vectors below is Crafting Interpreters (`6IQTDYAW`), Practical Foundations for
Programming Languages (`H3MVK7J5`), and Regular Expressions Cookbook
(`PZZBPXX9`).

| Book | Baseline denominator | Illustrative denominator | Observation-weight reconstruction |
|---|---:|---:|---|
| `6IQTDYAW` | 108 | `5753/60 = 95.88333333333334` | `80×1 + 5×3/4 + 21×1/2 + 1×5/6 + 1×4/5` |
| `H3MVK7J5` | 95 | `181/2 = 90.5` | `90×1 + 4×0 + 1×1/2` |
| `PZZBPXX9` | 191 | 191 | `191×1` |

Baseline shares are `(1/3, 1/3, 1/3)`. Illustrative book weights `(2, 1, 1/2)`
give shares `(4/7, 2/7, 1/7)`. The nonintegral denominator and the means `5/6`
and `4/5` agree with the documented mean-section rule.

## Three real patterns

The full discovery IDs end in `:p001`, `:p007` and `:p005`. Their direct
support and exception weights, independently recovered from the discovery
citations and source observations, are:

| Pattern | Baseline support | Weighted support | Exceptions in both modes |
|---|---|---|---|
| `p001` | `(0, 0, 12)` | `(0, 0, 12)` | `(0, 0, 2)` |
| `p007` | `(7, 0, 1)` | `(19/3, 0, 1)` | `(0, 0, 0)` |
| `p005` | `(5, 1, 5)` | `(5, 1, 5)` | `(0, 0, 1)` |

For example, baseline `p007` is `(7/108 + 1/191)/3`; weighted `p007` is
`(4/7)×(19/3)/(5753/60) + (1/7)/191`. Applying the same formula to support
and exceptions gives:

| Pattern | Mode | Prevalence | Exception prevalence | Book coverage | Rank |
|---|---|---:|---:|---:|---:|
| `p001` | Baseline | 0.02094240837696335 | 0.0034904013961605585 | `1/3` | 5 |
| `p001` | Illustrative | 0.008975317875841436 | 0.0014958863126402393 | `1/7` | 12 |
| `p007` | Baseline | 0.023350138969685217 | 0 | `2/3` | 3 |
| `p007` | Illustrative | 0.03849222564247641 | 0 | `5/7` | 1 |
| `p005` | Baseline | 0.027666874185658056 | 0.0017452006980802793 | 1 | 1 |
| `p005` | Illustrative | 0.03669489746437384 | 0.0007479431563201197 | 1 | 2 |

Every sampled value agrees with the saved `scores.json`, allowing floating
point rounding. Ranks agree with the complete set of 20 saved patterns.

## Direct citations and inherited section fits

Direct-citation prevalence is a sound measure of the support explicitly
recorded for a fixed proposal. It does not estimate every semantic instance in
a chapter. The producer settled that distinction, and the formula document
explicitly excludes fits inherited through an ancestor section.

Inspecting all 20 patterns finds inherited omissions only for `p001`: 49
descendant sections of `PZZBPXX9` inherit a fit from a cited ancestor. Adding
them by discovery's literal ancestor rule changes the support count from 12
to 61. Baseline prevalence would become `61/(3×191) = 0.10645724258289703`,
an increase of 0.08551483420593368. Illustrative prevalence would become
`61/(7×191) = 0.0456245325355273`, an increase of 0.03664921465968586.
Its rank would become 1 in both modes. Exception prevalence and book coverage
would stay unchanged. The other 19 patterns would be unchanged.

That expansion also includes `PZZBPXX9:s040`, which the pattern explicitly
cites as unresolved. Mechanical inheritance therefore cannot be treated as
additional direct evidence without changing the meaning of the measure.
The size of this alternative is material to interpretation, but it is a
documented choice rather than an acceptance defect. An in-page reminder about
direct citations would be optional clarification.

## Labels and separate inputs

The saved `baseline.html`, `weighted.html` and `comparison.html` each state
that they reweight fixed patterns and name the full discovery key. Their
titles, banners, column headings and comparison labels contain no claim to a
confidence or quality score.

Baseline and weighted tables keep preference effects, relevance, extraction
uncertainty and interpretation uncertainty distinct. The latter three supply
descriptive counts; none enters `score_patterns`. The columns note states
that only preferences change prevalence and points to the formula document.
The comparison clearly labels baseline and weighted prevalence, their
difference, ranks, coverage and book shares. Its numerical comparisons do not
combine uncertainty with preference weights.

The weighted page and comparison label the weights illustrative and explicitly
say they are not the human's preferences. Relevance has its own illustrative
status and statement. The report makes no claim about patterns that a new
discovery run under other weights might find.

## Reuse evidence

The inspected artifacts are under `data/artifacts/`:

- Discovery: `pattern-discovery/801dacaaafce1c4f35c2b44b580f26d2db20724f8bb64159bc387e03b370102f`.
- Baseline: `baseline-report/c64eeb119c4cca5b5af01b88d7111c9121166ae8e24d47b9407cc88970e2978c`.
- Illustrative: `weighted-report/baa62702a37926a3ce1cdaea9fdc59f7b0b191721c456d4d379b9ba6eb9aba23`.
- Changed weights: `weighted-report/ac4e3dc68a1bf0d614fa77c4842ca9a39a02560bbfcdcf22f1a52ffd8a873512`.

The two weighted `record.json` dependency envelopes have identical stage,
schema, code version, configuration and artifact declaration. Their ten ordered
inputs differ only in the preference-weight file locator and content identity:
`5b2888a16c7e262107bbc320b77db9a87b5df8126fe99481fb1ba66cc1924b5a`
versus `c26bab77610724785b7cae4b9530b93c1d937e0f2cfa01e4b2480917ac711ebf`.
Both name the same baseline, relevance input, discovery, three extractions and
three analyses. The baseline's eight inputs contain relevance and the seven
upstream artifacts, with no preference file.

The alternate file raises `PZZBPXX9`'s book weight from `1/2` to `3/2`, giving
shares `(4/9, 2/9, 1/3)` with unchanged observation denominators. The saved
records substantiate reuse rather than merely asserting that the stage makes
no model call.

## Recorded verification and limits

The 17 tests in [test_weighted_report.py](../../tests/test_weighted_report.py)
cover equal influence as observation counts grow from 4 to 40 and from 3 to
30, fixture arithmetic, preference weights, section exclusion and inheritance,
zero-book and all-book exclusion, invalid weights, ignored identifiers,
relevance independence, separate columns, illustrative and supplied-preference
labels, discovery banners, baseline and upstream reuse, identical-input reuse,
and the stage command. Source inspection and the arithmetic above supply the
remaining checks for distinct-section means, exception prevalence and
uncertainty independence. There is no dedicated test of the unlocated-section
default branch; its straightforward source agrees with the formula.

The [corrections integration record](corrections-review.md) reports the latest
full `task check`: ruff clean, pyright at zero errors and 156 tests passing.
This review relies on that recorded verification and does not claim a new run.

Graph indexing was attempted but refused because a pre-coordination or
unverified Codebase Memory generation was active. No graph generation or
coverage result was available. The review consequently used direct reads of
the scoped source, tests, documents and saved artifacts. No integration leaf
is needed because the inspection found no acceptance defect or unresolved
material risk.
