# Corrections implementation review

The demonstrated correction is faithful to the passage. Four material software
findings need integration: summaries lose earlier revisions, baseline selection
can compare incompatible runs, corrected moves can point to a non-episode, and
pattern summaries omit changes to evidence explanations and representation.

This review covers producer commit `714e97c9` (`corrections-k8`) against source
at `90e978b2`. It inspected committed source, fixtures and stored artifacts. No
tests, builds, lint, format commands or model calls were run; no production or
test code was edited. Graph indexing failed with `CBM CLI could not start because
a pre-coordination or unverified CBM generation is active; close all CBM sessions
and commands, then retry`. Findings therefore use direct source inspection,
without graph or index-coverage evidence.

## R1 — compare the complete correction history with the selected baseline (P2)

[urg/changes.py:163](../../urg/changes.py#L163) chooses the final interpretation's
`revises_version` and compares only that immediate predecessor with the final
version. Discovery before the corrections instead reads the original analysis.

For a boundary correction from version 1 to 2 followed by a label correction
from version 2 to 3, the summary reports version 2 to 3 and only the label change.
The earlier evidence additions/removals disappear from both `changes` and the
affected-observation set. A pattern change citing that evidence directly is then
labelled `rerun`, despite satisfying the documented correction rule. The first
correction's authorship and rationale are also absent from the summary entry.
An alternative reading revised again has the same baseline problem: its creation
is lost behind its immediate predecessor. These cases follow from source; they
were not executed during this inspection.

The correction command explicitly supports sequential versions and reapplies all
files from the base analysis. Integration should make the summary cover the
selected comparison interval, preserving the relevant revision provenance and
affected evidence. Verify a boundary revision followed by a label revision, and
an added reading followed by a revision. Existing stale-version tests do not
check the resulting summary.

## R2 — enforce baseline provenance for explicit and automatic selection (P2)

[urg/changes.py:529](../../urg/changes.py#L529) validates an explicit `--before`
only by loading it and checking that its key differs from `after`. It bypasses
the base-analysis map, method and code checks used by `find_before`. A discovery
over already-corrected analyses, different weights or instructions therefore
receives the same correction-versus-rerun attribution. A different book inventory
can also reach the unguarded old-source lookup in `artifact_rows`.

Automatic selection is narrower but still incomplete:
[urg/changes.py:69](../../urg/changes.py#L69) compares instruction version,
settings content and weights, omitting instruction content hash and model
identity. Editing instructions without bumping their version, or changing model,
can produce an apparently compatible baseline. Its differences are then
attributed to correction or ordinary rerun although another input changed.

The producer describes `--before` as resolving ambiguity among comparable base
runs. Integration should validate every selected pair against that contract,
including complete instruction and model identity, and reject incompatible pairs
before building reports. Verify explicit incompatible runs and automatic
selection with changed instruction content/model. The real pinned comparison
below has matching settings, weights, instructions including hash, model,
attempt, method and discovery code, so this finding does not invalidate it.

## R3 — retain the episode-reference invariant when applying corrections (P2)

[urg/corrections.py:187](../../urg/corrections.py#L187) accepts any current
interpretation as a reference. `_revise` checks that a move has an `episode`
field and refuses self-reference, but accepts another move or a relationship as
that field's target. In contrast,
[urg/analysis.py:298](../../urg/analysis.py#L298) explicitly rejects an `episode`
reference whose target kind is not `episode`.

A valid correction file can thus turn a checked analysis into a structurally
invalid corrected analysis, which discovery and reporting consume directly.
Integration should preserve the analysis invariant and record a refusal for
non-episode targets. Verify a move referencing another move, a relationship,
and a valid episode. The existing fixture covers references on the wrong owner
kind, rather than the wrong target kind.

## R4 — include meaningful pattern fields and retained citation changes (P2)

[urg/changes.py:249](../../urg/changes.py#L249) compares role membership and four
text fields: name, description, rules and uncertainty. It ignores representation,
basis, books and metadata on citations retained in the same role. These fields
explain why the proposed grammar and its evidence deserve acceptance.

The real comparison contains omitted changes in 14 matched patterns'
`representation`, two patterns' `basis`, two patterns' `books`, and 83 retained
role citations' metadata, principally explanatory notes. For example, old
`p016` has no representation entries; new `p013` adds `refinement-rule` justified
by each later variant modifying the previous one by one requirement. The summary
lists no representation difference. In matched `p001`, the note for
`PZZBPXX9:s012` changes from a generic four-part account to an explanation of the
one-paragraph Discussion; retained membership hides that change.

The contract asks the summary to show changes to derived results. These are
substantive explanation changes, beyond identifier renumbering. Integration
should compare meaningful pattern/citation content and attribute its differences
consistently with the declared heuristic. Verify a representation-only change
and a note-only change with unchanged citation membership. Existing tests do
not cover those cases.

## Passage judgment

The review read every extraction element from `6IQTDYAW:e0050` through `e0072`.
Elements `e0051`–`e0057` introduce and begin implementing unary expressions;
`e0056` announces two operators, while the first switch implements minus.
Element `e0064` defers failed-cast handling and `e0066` explains post-order
traversal. Elements `e0067` and `e0068` then supply logical not in the same
`visitUnaryExpr` switch. The explicit side trip starts in `e0069`, followed by
the truthiness heading and explanation in `e0070`–`e0072`.

Extending episode `6IQTDYAW:0fbbac3a863e:i022` through `e0068` is well supported.
Its new description accurately covers negation, dynamic typing, traversal and
logical not. The rationale explains the missing second operator. Medium
uncertainty reasonably preserves the alternative boundary at `e0066` because
logical not also motivates truthiness. Keeping `e0067`/`e0068` in episode `i027`
is consistent with overlapping episodes. The running header in `e0065` is
pre-existing extraction noise, not introduced by this correction.

## Audit of every recorded cause

The audit reconstructed all 172 recorded cause labels from the pinned discovery
records and the stage's rule. All follow that rule: 3 `correction`, 155 `rerun`,
14 `either`. The affected set is `i022`–`i026`, `i077`, `e0067`, `e0068`, with
interpretations prefixed `6IQTDYAW:0fbbac3a863e:` and evidence prefixed
`6IQTDYAW:`. Pattern IDs below omit only their run prefix:
`pattern:3119a52fede4:` before and `pattern:390ed48621c7:` after.

| Before → after | correction | rerun | either |
| --- | ---: | ---: | ---: |
| p001 → p001 | 0 | 8 | 0 |
| p003 → p003 | 0 | 6 | 0 |
| p006 → p006 | 1 | 4 | 2 |
| p007 → p011 | 0 | 5 | 0 |
| p012 → p015 | 0 | 2 | 0 |
| p017 → p017 | 0 | 6 | 0 |
| p019 → p010 | 0 | 4 | 0 |
| p005 → p004 | 0 | 0 | 4 |
| p010 → p009 | 0 | 7 | 0 |
| p016 → p013 | 1 | 0 | 4 |
| p011 → p014 | 0 | 7 | 0 |
| p004 → p005 | 0 | 3 | 4 |
| p002 → p002 | 0 | 10 | 0 |
| p013 → p007 | 0 | 7 | 0 |
| p015 → p018 | 0 | 6 | 0 |
| p009 → p008 | 0 | 8 | 0 |
| p008 → p012 | 0 | 7 | 0 |
| p014 → p016 | 0 | 13 | 0 |
| Dropped p018, p020; new p019, p020 | 0 | 4 | 0 |
| Unfitted membership | 1 | 48 | 0 |
| Total | 3 | 155 | 14 |

All three `correction` labels are doubtful as causal claims:

- `i025` becomes unresolved in `p006`, deferral to later material. Its own
  text, evidence and version are unchanged; it refers to corrected episode
  `i022`. The new note questions whether its promise fits the within-chapter
  pattern, a question not explained by adding logical not to the episode.
- `i026` loses its supporting role in old `p016` → new `p013`, stepwise increase
  in complexity. Its traversal explanation at `e0066` is unchanged. The added
  `e0067`/`e0068` do not establish why this classification changed.
- The same `i026` becomes unfitted. This records the consequence of that lost
  support, with the same weak causal evidence.

These labels are mechanically correct under the documented dependency rule.
They do not demonstrate that the boundary change caused the model's decisions.
The 14 `either` labels in the four table rows with nonzero values appropriately
retain uncertainty. Every other recorded membership change involves observations
outside the affected set, and every `rerun` text change has no affected citation.
The dropped and new patterns likewise cite no affected observation. Global model
effects remain possible; a single call cannot establish causality for any label.
No further rule violations were found among the recorded labels. R4 concerns
differences the summary never recorded.

All 18 matched patterns changed; two were dropped and two added. Unfitted
observations rise from 226 to 239: 31 become unfitted and 18 become fitted.
Matching uses names followed by citation overlap. The lowest accepted overlap,
old `p014` → new `p016` at 0.30, retains the choices theme; it is a heuristic
identity, not a demonstrated mismatch.

## Reuse, report inspection and verification limits

The base and corrected records differ only in current `i022`, its superseded
history and correction provenance. Earlier evidence and version 1 remain
readable. The other two books keep their analysis keys. All three extraction
keys and all three episode-analysis-response keys are unchanged; their six
payload hashes match their stored record hashes. The correction code has no
extraction or model call path.

A scan of all 24 stored extraction records and six analysis-response records
(excluding `latest` aliases) found none written between the before discovery at
`2026-11-17T12:28:42.078330Z` and the after report at
`2026-11-17T12:35:23.620925Z`. The reused records predate that interval. This
supports the recorded no-rerun claim; a stored snapshot cannot establish the
absence of transient unrecorded activity. The summary reports 594 report files
reused, four changed, 76 added and 58 removed.

In the corrected report, `interpretations.html#6IQTDYAW_0fbbac3a863e_i022` shows
version 2, corrected status, description, medium uncertainty, author, rationale
and expandable version-1 evidence/description. Links to `e0067` and `e0068`
include printed page 101 and PDF index 113. Their `structure.html` sections show
the logical-not paragraph and the switch code including `BANG`/`!isTruthy`.
Neither has a render entry or image. The extracted text and code remain readable
without a render; the report supplies no separate missing-render notice.

Fixture inspection covers boundary/history preservation, labels, relationships,
alternative readings, stale and invalid corrections, reuse/invalidation,
summary causes, report history, stale correction runs and absent baselines.
The producer records an uncertainty regression that failed with `KeyError: 'id'`
and passed after its repair. The inspected producer/stage notes contain no
complete-suite passing transcript or count. This review therefore does not
certify a fresh test pass. Integration owns execution and verification of its
triage and repairs.

## Integration (corrections-k26)

<!-- tc: depart convert-first because the sections above are the recorded evidence of review corrections-k17, and integration adds only this section, which passes the checker -->

Integration `corrections-k26` found all four findings real in the source and repaired each one.

- R1: The summary compares each corrected interpretation with its version in the base analysis. It lists every correction in that interval with author, origin and rationale. An alternative reading has no base version, so its fields count as added. The record schema is version 2.
- R2: One check governs both the automatic search and an explicit `--before`. It compares the uncorrected analyses, method, settings content, instructions version and hash, model, preference weights and code version. It refuses a mismatch and gives each difference.
- R3: The corrections stage refuses an `episode` reference to an interpretation that is not an episode. The analysis stage applies the same rule.
- R4: The pattern comparison covers name, description, basis, books, rules, representation and uncertainty. It also compares the note of each observation that both runs cite in one role.

Each repair has a regression test in `tests/test_corrections.py`. The full `task check` passed with ruff clean, pyright at 0 errors and 156 tests.

The repaired summary `c75a09b7` replaces `4a3f921d`. It gives 3 `correction`, 250 `rerun` and 20 `either` differences. The three `correction` differences are the same as before. The added differences come from the basis, books, representation and note comparisons. The affected set stays the same.

The automatic search on the working store refused the pair, because the `latest` corrections run there is `24f6c9b1`, not `43c0bff6`. Integration therefore ran the summary on a copy of the store with the corrections pointer at `43c0bff6`. It then copied the summary artifact into the store. It did not move any `latest` pointer.

The missing render for `e0067` and `e0068` is a property of the report stage. The report renders only the elements that patterns cite. Integration accepts this trade-off and records it as a known limitation of the stage.

## Pinned artifacts

Keys refer to directories under `data/artifacts/<stage>/<key>/`; data is ignored
by version control and must be retained separately to reopen the demonstration.

| Stage/use | Key |
| --- | --- |
| analysis-corrections-run | `43c0bff66d6be5e2548c63202b3c601d503701cdd6efb910baeb4be93d217d36` |
| episode-analysis, base | `8cda2b672900142a18749a81c8fbc6b847cb9eb9be99ac6f3fdb6b663eba31d9` |
| episode-analysis, corrected | `d7f5326b98f1821578c83970242ee48ad38b87c82da7ccdebb2bfca2949be1dc` |
| pattern-discovery, before | `f1e40eb0cff08d106ad6b0e34570b952f830fd4c8a52fa5bd74889b60544469d` |
| pattern-discovery, after | `801dacaaafce1c4f35c2b44b580f26d2db20724f8bb64159bc387e03b370102f` |
| pattern-report, before | `7983c6f1407becb2f95b2a462c41e5b295db2d45a52b1e4183f1e0243736d3b1` |
| pattern-report, after | `1c462105bc29e1970bff5366e3903bf857482d4a421abf39bb013a8b476b7250` |
| change-summary | `4a3f921d5428d652a1dba228769edaea0e2c6a10a82f7cf232909768d198ed4d` |
| change-summary, after integration | `c75a09b79e035fc7d5d50fc84b8ca0e38b0fd8402a4373a1c751667a3c2ece2d` |

The source passage is in chapter-extraction
`f7a9036c9b1380f4171f6033b56b6c466a5ba262c8cfbfe219de547e62c7acea`.
