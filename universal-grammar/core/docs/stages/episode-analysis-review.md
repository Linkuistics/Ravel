# Episode-analysis implementation review

The sampled interpretations are faithful to their cited passages. One material
issue remains: the stage loses raw output when the model adapter rejects the
CLI response or times out. This prevents inspection of the failed attempt and
breaks the raw-response retention contract.

This review inspects producer commit
`7b830b91f0d56520a0fa17ede2d11c80bfdc7b10` (`episode-analysis-k6`) against the
source at parent `bec13663` of the review change. Later analysis reuse changes
were reconciled with the producer diff. No production or test code was changed,
and no tests, builds, lint, format commands or model calls were run.

## R1 — retain failed adapter output before rejecting it (P2)

In [urg/model.py](../../urg/model.py#L131), `parse_claude_result` raises
`ModelError` for non-JSON output, a non-object envelope, an error/nonzero exit,
or a missing result string. `ClaudeCliModel.call` also raises on timeout at
line 231, discarding any captured `TimeoutExpired.stdout` and `.stderr`, and
on an unacceptable provider at line 237.

All these failures occur before [urg/analysis.py](../../urg/analysis.py#L414)
writes the request, raw response and metadata: `response = model.call(request)`
must return before `store.write` is reached. `_analyse_one` catches the error
at line 590 and produces a failed outcome with no response artifact. The run
report consequently contains a null `response_key`. A truncated CLI envelope,
for example, leaves only a shortened error message rather than the captured
bytes. A timeout can lose the partial output entirely. The failed attempt
cannot be inspected or distinguished through a retained response record when
the stage is retried.

The contract says that a malformed answer is rejected while its raw response
is kept. The current retention test supplies malformed answer text through
`RecordedModel`, which wraps it in a successful Claude JSON envelope. It proves
retention after that envelope parses; it does not cover rejection by the real
adapter. `test_claude_result_parsing` expects an error for non-JSON output but
does not exercise persistence.

Integration should preserve the request, available stdout/stderr, and failure
metadata before rejecting these attempts. The preserved failure must remain a
failure and must not become a reusable successful answer. Adapter-level
recorded cases for malformed envelopes and partial timeout output should
establish retention and the retry behavior without calling a model. This is a
source-derived finding; the review did not induce a live CLI failure.

## Contract evidence

| Contract | Inspected evidence and result |
|---|---|
| One replaceable model boundary | `ModelBoundary`, `ClaudeCliModel`, `RecordedModel`, and `run_analysis`; the recorded-answer tests replace the boundary. |
| Interpretation identity and status | The answer schema and accepted-record construction supply IDs, version 1, kinds, evidence, descriptions, model authorship, uncertainty, and `proposed` status. The citation test checks these fields and the real records retain provisional labels and unlabeled observations. |
| Citation resolution and rejection | `evidence_index`, `_first_problems`, and `check_answer` resolve both evidence layers and cascade rejection through references. `test_every_citation_resolves_to_evidence` and `test_interpretation_citing_missing_evidence_is_rejected` cover this behavior. |
| Malformed-answer rejection and raw retention | The schema and recorded-answer test cover malformed answer text. Adapter failures have the retention gap in R1. |
| Dependency identity | `response_dependency`, `analysis_dependency`, and the dependency-record test name instructions, extraction, model identity and response. The stage notes explicitly disclaim deterministic model reproduction. |
| Invalidation and reuse | The instruction-change test retains the extraction key; module closures exclude analysis/model from extraction. Recorded reuse/new-attempt tests and the stage notes cover stored-response reuse. |
| Emergent categories | Version 1 instructions contain no move catalogue. The recorded fixture retains an observation with a null label; the real run retains 4, 4 and 5 observations. |
| Child authentication environment | `child_environment` copies an explicit allowlist and then removes the API key, token, base URL and alternative-provider variables. `ClaudeCliModel.call` uses it. All three original response envelopes record provider `firstParty`. |

The preceding chapter-extraction integration, commit `bec13663`, records
`task check` with ruff and format checks passing, pyright reporting zero errors,
and pytest reporting 125 passed. That is the available recorded suite result
for the reviewed source; the review makes no fresh execution claim.

## Passage comparison

For each entry below, the comparison read every cited structure element and
the elements next to it. It also read the indicated moves, relationships and
alternative readings against their cited and adjacent passages. The identifiers
in this table are the model's local IDs within the original response.

| Chapter | Five episodes | Other sampled interpretations | Result |
|---|---|---|---|
| Crafting Interpreters 7 | `ep3`, `ep7`, `ep13`, `ep19`, `ep23` | `mv1.1`, `mv23.3`, `rel1`, `rel12`, `alt1`, `alt3` | Supported: boxed value representation; unary evaluation; equality; public evaluation and stringification; static/dynamic tradeoffs. The payoff preview, tradeoff move, opening/closing frame and alternative boundaries also match. |
| PFPL 5 | `ep3`, `ep7`, `ep14`, `ep17`, `ep20` | `mv1.1`, `mv21.2`, `rel1`, `rel15`, `alt1`, `alt3` | Supported: multistep evaluation; the let-expression trace; structural/contextual equivalence; definitional-equality rule groups; symbolic evaluation. The overview, attribution, section relationships and alternative readings match the surrounding prose. |
| Regular Expressions Cookbook 6 | `ep5`, `ep16`, `ep25`, `ep31`, `ep34` | `mv1.1`, `mv33.3`, `rel1`, `rel18`, `alt1`, `alt3` | Supported: currency boundaries; graduated numeric ranges; empty-match failure; decimal/comma restrictions and repairs; Roman-numeral conversion. The character/quantity distinction, lookahead, recipe relationships and alternative readings also match. |

No unsupported interpretation was found among these 33 sampled
interpretations. This is a bounded sample, not a claim that every interpretation
in the three chapters is faithful.

Cross-section citations were inspected on both sides of the boundary. Long
Regex recipes are split into several episodes: recipe 6.7 has `ep13`–`ep18`,
6.10 has `ep23`–`ep26`, and 6.12 has `ep28`–`ep31`. The sample therefore gives
no evidence of a blanket one-episode-per-recipe rule.

PFPL inference-rule elements reach the prompt without mathematical image
content. The sampled descriptions are supported by surrounding prose and
textual traces; this comparison does not establish visual fidelity of every
displayed inference rule. That limitation is already stated in the stage
notes. Crafting Interpreters `ep7` was compared with the original response and
analysis: its unary-negation description is supported. The later correction
that adds logical-not citations was not treated as an original producer defect.

## Artifact provenance

The original run report is
`data/artifacts/episode-analysis-run/728ab633bf81981a8c5ca8aee9966226c8a67a8337a4629da2a4bb7b41755c8a/report.json`.
It records all three outcomes as analysed with zero rejected interpretations.
The corresponding response directories contain `request.json`,
`raw-response.json`, `metadata.json` and `record.json`. Each raw envelope
contains the original answer. The envelope and metadata together record
`claude-opus-5-5`, first-party access and harness version 2.1.322. These local
artifacts are excluded from version control; this table pins the generations
inspected rather than relying on the mutable latest pointers.

| Book key | Analysis key | Raw response key | Extraction key |
|---|---|---|---|
| `6IQTDYAW` | `8cda2b672900142a18749a81c8fbc6b847cb9eb9be99ac6f3fdb6b663eba31d9` | `0fbbac3a863e6718c6f5a7552bed6f0746312a1c0661fd204080780cb95c9e15` | `f7a9036c9b1380f4171f6033b56b6c466a5ba262c8cfbfe219de547e62c7acea` |
| `H3MVK7J5` | `822209747545e6ee920d0bd6202a1f69b7a2219f016b782289c939a56fd6028e` | `da01f8e04a43f6b1ef8899545c350f7f5058b50c09ff7f59393c74e7752bcb11` | `bc9d449704e095718cb74a3c7efc02097dfee50caf4d6a95489ed28f88850674` |
| `PZZBPXX9` | `9b65be05edd35899aabf3f54c6b3f299024c800bdeb617eb529e56ca9e03ed87` | `197d3c4b0f712bd869a0ec9dfcd34fbdecebc1b9313b757cf95c11d6b8ac6fd6` | `3f6220b42593ff8ed691aef2eafd980cd63dd10ec08b8cc7723f03d160c8b530` |

Graph indexing could not start because CBM reported an active incompatible
generation. This review used direct source and artifact inspection; no graph
coverage certification is available. The finding above follows explicit
adapter-to-store control flow and does not depend on an exhaustive caller
search.
