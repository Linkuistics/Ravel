# Pattern-discovery implementation review

Four material findings remain: rule validation accepts malformed rules, an empty
support list prevents per-pattern rejection, the unfitted list contains plain
matches for existing patterns, and failed adapter calls lose their output.
The five sampled patterns otherwise match their passages and retain reasonable
exceptions and unresolved cases. Acceptance needs the findings triaged and any
accepted repairs verified.

This review covers producer commit
`d40b6a79aef28e0591d08317f41a525e79c21e8f` (`pattern-report-k7`) against source
at `cb317a5e`, the parent of this review change. Later corrections integration
already changes discovery source selection; the original discovery and report
were inspected through their pinned artifact keys below. No production or test
code was changed. No tests, builds, lint, format commands or model calls were run.

## R1 — validate the complete refinement rule (P2)

[urg/discovery.py:78](../../urg/discovery.py#L78) defines
`RULE = re.compile(r"^\s*[A-Za-z][\w-]*\s*->\s*\S")`.
The only rule check at [line 508](../../urg/discovery.py#L508) uses `RULE.match`.
It consumes the head, arrow and one non-space character, then ignores the rest.
Consequently `Recipe -> ???`, `Recipe -> Part -> Other`, and a valid prefix
followed by an arbitrary second line all pass this check. This follows directly
from the regex; these cases were not executed during review.

The contract requires malformed rules to be rejected with reasons, while the
instructions define a sequence of named parts with optional/repeated modifiers.
These accepted strings are subsequently presented as checked grammar fragments.
The rejection test covers too few instances but has no malformed-rule case.

Integration should define and validate the entire allowed rule form, preserving
valid existing fragments. The original response also uses `+` in fragments such
as `Variant+`, although the instructions explain only `?` and `*`; settle that
syntax explicitly rather than accidentally rejecting existing intended rules.
Recorded-answer cases should cover malformed suffixes and valid modifiers.

## R2 — reject zero-support proposals individually (P2)

[urg/discovery.py:448](../../urg/discovery.py#L448) declares
`supporting: list[Citation] = Field(min_length=1)`. An otherwise well-formed
proposal with `supporting: []` therefore fails `DiscoveryAnswer` parsing before
the minimum-support check at [line 503](../../urg/discovery.py#L503).
`run_discovery` rejects the entire answer before `build_record` can retain any
proposal in `rejected` or accept its valid neighbours.

Zero instances is fewer than the configured minimum of two. The explicit
contract says to reject that pattern and keep it with its reasons, as the code
already does for one instance. With a mixed answer containing valid patterns
and one zero-support proposal, all valid patterns are instead lost from the
discovery result. The raw response survives, but no discovery result records
the individual rejection. The existing fixture exercises one support, not zero.

Integration should allow an empty list through structural parsing and apply the
configured minimum during per-pattern checking. Missing or wrongly typed fields
can remain malformed-answer errors. Verify a mixed answer retains its valid
patterns and records the zero-support proposal and its reason.

## R3 — the unfitted sample contains observations that plainly fit (P2)

The review's explicit sampling condition fails in the original discovery:

| Unfitted observation | Existing pattern and passage evidence |
|---|---|
| `6IQTDYAW:0fbbac3a863e:i003`, framing the chapter task | `p008`, opening preview organises sections. The episode names the two guiding questions and cites `e0008`–`e0012`, printed page 98, PDF index 110. The pattern already supports `i005`, the move containing those same questions, and the later relationships `i073` and `i074`. |
| `H3MVK7J5:da01f8e04a43:i001`, chapter overview of methods | `p008`. The episode previews structural, contextual and equational dynamics. Its opening paragraph `e0002`, printed page 36, PDF index 55, is also the evidence of the pattern's supporting move `i003`. |

Both unfitted entries have an empty `cited_as` list. Their full descriptions
plainly instantiate the pattern; this is not a match inferred only from a label.
By contrast the sampled atmospheric opening `6IQTDYAW:0fbbac3a863e:i001` and its
payoff move `i002` do not preview section organisation and can reasonably remain
unfitted.

[urg/discovery.py:581](../../urg/discovery.py#L581) computes the complement of
explicit supporting citations, with ancestor coverage only for document
sections. It does not establish that an uncited interpretation fits no pattern.
Thus it faithfully reports omissions from the proposal, but the report labels
that complement as observations fitting no pattern. The pipeline's observation
granularity makes this distinction visible: an episode and its move can describe
the same pattern instance without both being cited.

There is also an unresolved definition mismatch: the stage document says an
exception counts as fitted, whereas `unfitted` and `METHOD.fit` count supporting
citations only. Integration should settle and consistently document coverage,
repair the sampled omissions or represent their coverage, and retain genuine
unmatched observations. Do not automatically classify every child move as a
complete instance of its parent's pattern. The acceptance condition concerns
plain semantic fits, not simply whether the set subtraction is correct.

## R4 — keep failed discovery-call output (P2)

[urg/discovery.py:670](../../urg/discovery.py#L670) calls `model.call(request)`
before the response `store.write`, without catching `ModelError`. The CLI
handler prints the error and exits. The shared boundary attaches captured
stdout, stderr and failure metadata to `ModelError.output` for malformed
envelopes, nonzero/error responses and timeouts
([urg/model.py:153](../../urg/model.py#L153),
[urg/model.py:263](../../urg/model.py#L263)). Discovery never stores that output
or its request. A partial or rejected model run consequently has no inspectable
response artifact or attempt record; retry starts from the last successful
response or the same unrecorded attempt number.

This is a material gap in the evidence-preserving discovery provenance, not a
request to make failures successful. The malformed-answer retention test feeds
answer text inside a successful recorded envelope and does not reach this path.
The preceding `episode-analysis-k24` integration explicitly identified this same
discovery gap and left it to this review; `docs/stages/episode-analysis.md`
records that limitation. The analysis stage now supplies a local precedent for
retaining failures separately from reusable successful responses.

Integration should retain the request, available bytes and failure metadata,
preserve failed status, and prevent reuse as a successful response. Recorded
boundary failures and partial timeout output should establish retention and
retry behavior without a live model call. Calls that never ran need not invent
output. This finding is derived from control flow, not an induced provider failure.

## Contract evidence

| Contract | Inspected evidence and result |
|---|---|
| Three citation roles resolve | `check_answer` checks each role against the inventory and constructs role-specific links. The original artifact's report links resolve. |
| Missing/repeated/insufficient citations and malformed rules reject individually | Missing and duplicate IDs have per-pattern checks. R1 and R2 identify rule and zero-support gaps. The fixture explicitly covers one-support rejection; missing/repeated/rule cases lack dedicated coverage. |
| Discovery provenance | Settings content and digest, resolved weights, instructions version/digest, model, response key and attempt are retained for successful calls. R4 covers failed calls. |
| Weight change creates new discovery | Response dependency includes settings and request digest; the changed-preference test checks a fresh response/discovery and changed prompt. |
| Unfitted observations | The artifact and report retain the computed list; R3 fails the semantic sampling condition. |
| Proposed status | `check_answer` assigns `proposed`; the original patterns and HTML display it. |
| Three layers | Report source and manifest map patterns, interpretations and document structure to separate pages. |
| Links and renders | Inspection of the stored HTML found 380 + 1,794 + 123 = 2,297 links resolving to existing files/anchors, no duplicate page IDs, and 652 existing image targets. The manifest records all 652 as rendered. Structure source supplies page labels/indexes; a fixture covers a missing-PDF failure note. |
| Unchanged-input reuse | Response scan, discovery dependency and report dependency return stored artifacts. Fixture tests cover reuse of all three and no additional model request. The stage notes record the original rerun. |

The preceding integration `cb317a5e` records `task check` passing with 133 tests.
That is the available recorded suite result for the reviewed source, not a fresh
execution claim. The integration of this review owns any new tests and post-fix
verification.

## Passage comparison

Pattern IDs below have prefix `pattern:3119a52fede4:`. The comparison read cited
elements and their neighbours from the discovery-pinned extractions, including
exceptions and unresolved cases. PDF indexes are zero based.

| Pattern | Compared evidence | Assessment |
|---|---|---|
| `p001`, fixed recipe section template | Cookbook recipe sections `s002`, `s007`, `s012`, `s017`, `s022`, `s027`, `s036`, `s042`, `s047`, `s052`, `s057`, `s062`, plus `i106`; recipe 6.1 headings `e0005`, `e0007`, `e0036`, `e0050` and adjacent prose, pages 375–378/indexes 390–393. | Problem, Solution, Discussion, See Also recur. Recipe 6.12 adds optional Variations. Exception `s076` correctly adds Convert Roman Numerals to Decimal (`s080`). No unresolved cases were proposed. |
| `p003`, failing case motivates fix | Cookbook `i014`, `i032`, `i053`, `i059`: currency boundaries, one-character numeric range, underscore failures and empty float matches; pages 378, 388–389, 396, 398/indexes 393, 403–404, 411, 413. CI `i042`, runtime errors, page 103/index 115. | The failure motivates a better mechanism. Some mechanisms continue beyond an episode's cited range, so adjacent prose matters. PFPL exception `i058`, pages 42–43/indexes 61–62, correctly defers semantic equivalence to chapter 47. Cookbook unresolved `i043` fairly leaves the alternative-order/anchor framing open; CI `i085` fairly records ambiguity at `e0119`'s episode boundary. |
| `p009`, formal rules then prose gloss | PFPL `i089`, `i018`, `i035`, `i038`, `i041`, `i053`; glosses `e0038`–`e0039`, `e0074`, `e0080`, `e0085`, `e0116`–`e0117`, pages 38–42/indexes 57–61. | Rule grouping, hole-filling gloss and the three-step paraphrase support the pattern. Unresolved `i008` is an induction principle rather than a direct gloss; `i032` has prose before instruction rules without a recorded following gloss. Both distinctions are justified. |
| `p012`, regex explained part by part | Cookbook `i029` (`e0191`–`e0192`), `i055` (`e0380`), `i072` (`e0496`–`e0497`), `i082` (`e0556`–`e0558`), pages 385–386, 396, 403, 407/indexes 400–401, 411, 418, 422. | Leading zeros, underscore groups, nested lookahead and Roman numeral parts are explicitly explained in sequence. No exceptions or unresolved cases were proposed. |
| `p017`, recurring not-all-optional principle | Cookbook `i059`, `i095`, `i097`, `i105`: `e0418`–`e0422`, `e0446`, `e0559`, pages 398, 400, 407/indexes 413, 415, 422. | The float principle recurs in separator and Roman numeral recipes; alternation and lookahead are different repairs. `i105` is another reading of the same float passage, not a fourth independent occurrence. The separator passage explicitly refers back; the Roman passage supplies thematic recurrence without an explicit backlink to that earlier principle. |

No sampled pattern was wholly unsupported. This is a bounded assessment of
faithfulness, not certification of every proposed pattern or visual fidelity of
every mathematical crop.

## One-book scope and representation

All seven one-book patterns describe plausible local style, not demonstrated
universal structures: cookbook `p001`, `p002`, `p012`, `p017` and PFPL `p009`,
`p010`, `p019`. Beyond the detailed sample, the variant descriptions in cookbook
`i004`, `i017`, `i031`, `i048`, `i051`, `i056`, `i079` support `p002`. PFPL's
short proofs of theorem 5.1 and lemmas 5.2–5.3 support `p010`, while theorem
5.4's worked cases justify its exception. PFPL's equivalence formulations and
theorems 5.1, 5.4 and 5.5 support the broad local tendency in `p019`.

`docs/representation.md` matches the named patterns: hierarchy
`p001/p002/p012/p013`, cross-references
`p003/p004/p005/p006/p007/p008/p017/p018/p019`, attributes
`p001/p002/p013/p015`, and revisions `p003/p005`. Its counts of 13 grammar
fragments, seven absent fragments and six patterns without representation needs
match the record. Keeping regex subject hierarchy unmodelled and grammar text
separate from revision edges is explicitly provisional. A typed cross-reference
need does not establish that every instance, particularly `p017`'s Roman case,
contains an explicit textual backlink.

## Artifact provenance and inspection limits

The original discovery is
`data/artifacts/pattern-discovery/e470dcc1112a8ab7adf9b832d13cd66df0dec35f4491752b1b71c88cff95e0dc/discovery.json`.
The original report directory is
`data/artifacts/pattern-report/88ca26a0241322d10a7e954c9f1a94cda56a2e80da5bd27cc2d360d82e7e6658/`.
These artifacts are locally available and excluded from version control. They
retain the original 20 patterns, zero rejections and 226 unfitted observations.
Their `sources` entries pin the analysis and extraction generations used for
the passage comparison; mutable latest pointers were not used for that sample.

CBM could not start: `codebase-memory-mcp: CBM CLI could not start because a pre-coordination or unverified CBM generation is active; close all CBM sessions and commands, then retry`.
This review used direct source and artifact inspection. No graph coverage
certification is available. The findings follow bounded validation/store control
flow and explicit artifact entries; they do not rely on exhaustive caller claims.
