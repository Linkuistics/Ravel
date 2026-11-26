# Vertical-slice completion report review

The report contains all nine requested subjects, and its main research figures
are supported by the stored artifacts. It does not yet provide a consistent
completion account: the clean pipeline stops at corrections, while later
integration has changed the correction and the recovery evidence that it cites.
The extraction change also needs a complete source-identity check and stronger
verification of which manifest it selects.

This is the inspection-only review of `completion-report-k11`, producer commit
`46599e79fd9e18f17f7735e65a4d31ffa5e239af`, against the tree at `6ec572ea`.
No tests, builds, lint, formatting, production stages or model calls were run.
Production and test files were not edited. Artifact inspection below reads JSON,
checks recorded payload hashes and views an existing render; it does not rerun
the pipeline. Graph tools were unavailable. The CLI indexing attempt failed
with `CBM CLI could not start because a pre-coordination or unverified CBM generation is active; close all CBM sessions and commands, then retry`.
Source conclusions use direct reads, without graph coverage or exhaustive claims.

## R1: reconcile completion claims with the runs actually demonstrated

At `docs/milestones/vertical-slice-completion.md:7` the report says `task pipeline`
built the pipeline end to end. Line 30 says all six stages run from an empty
store through that command. The recorded R1 instead stops at corrections with
status 1; discovery and weighting run separately on uncorrected analyses.
R2 repeats that failure, so line 100's statement that every stage reported reuse
also overstates the record. The adjacent disclosure is useful but does not make
these completion statements true.

The report also needs reconciliation against later integration, rather than
silently treating producer-time evidence as current:

- Line 154 describes the current correction file as targeting
  `6IQTDYAW:0fbbac3a863e:i022`. The file now targets
  `6IQTDYAW:4c555fc18422:i023` after an explicit passage review.
- Line 168 says current transcript S7 corrects `PZZBPXX9` on a clone.
  Current S7 corrects `6IQTDYAW` in the real working store, runs the full pipeline
  successfully and records seven new artifacts, 99 to 106, with one inference
  call. S8 then succeeds unchanged at 106 with zero calls.
- Lines 205 and 208 cite S2's old count of 76 and S5's old damaged artifact
  `2b9bc2e5…`. Current S2 records 99; current S5 damages `c3de0bff…`.
- The next-step justification at line 264 says every recovery section failed
  corrections. This is false for the current linked transcript.

These are material to the requested distinction between implemented behavior
and verified integration. Preserve the historical failed run and its artifact
identifiers as historical evidence; connect the current conclusion to the newer
successful run. Do not infer that automatic correction targeting is required to
meet acceptance: manually reviewing and retargeting an immutable correction
already allowed the successful S7/S8 demonstration. Automatic matching by kind
and first evidence element may be future work, but that pair alone does not
establish that two interpretations have the same meaning or the same defect.

## R2: the reproduction script can finish successfully after unexpected failure

At `scripts/reproduction-transcript.sh:72` and line 82, any pipeline failure
enters the same fallback, including failures before corrections. Failures of
the fallback stages are not propagated. Line 88 only prints `THE STORE CHANGED`,
and `model_calls` at line 52 prints and clears its log without checking the
restart count. The final log reset normally returns success even after a
refused model call or a failed discovery/report stage.

The existing transcript still exposes its stage exit codes; this finding does
not erase that evidence. It means the documented reproduction entry point is
not a reliable acceptance check: its exit status can say success while the
demonstration failed. Expected correction refusal must be distinguished from
unexpected failures, and the required successful stages, unchanged restart and
zero restart calls need checked outcomes.

The run provenance at lines 63–64 is also only a short base commit plus a count
of dirty paths. It cannot identify the seven changed subjects used for R1.
Artifact dependencies preserve much of the stage provenance, but the script,
correction and other run inputs have no before/after digest inventory. The
Grove execution procedure requires that inventory for a measurement. A current
acceptance record should identify the exact subjects and establish that they
did not move during the run; a dirty path count does not do so.

## R3: latest-manifest resolution must preserve the complete source identity

`urg/extraction.py:150` deliberately drops the selection's manifest pin.
This is reasonable when catalogue code changes its artifact key while the
selected source and PDF remain unchanged. But `resolve_document` at line 170
compares only `item_key`, followed by attachment key and file hash. It does not
compare `library_id` or `library_type` from the selection's source identity.

The source-identity ADR defines a record by its Zotero library and item key.
Once any latest manifest is accepted, matching keys from a different library
must not establish the selected record's identity. Matching PDF bytes do not
repair that provenance error: two library records can refer to the same PDF.
The returned path is then used for an extraction whose dependency claims the
selection's library identity. With different bytes the same collision causes
a misleading hash failure instead. This is a source-derived risk, not a claim
that the supplied single-library corpus contains a collision.

Keep the latest-manifest behavior if desired, but require the complete selected
source identity before accepting its attachment. Verification should distinguish
another library with the same item/attachment keys, including identical PDF
content, from the intended library.

## R4: verify the two-manifest rule and the requested broken-behavior check

The new absent-pin test at producer `tests/test_chapter_extraction.py:249`
uses a single manifest and changes the selection key to `e` repeated 64 times.
It rejects the old pin-only implementation. It would still pass an
implementation that uses the pinned manifest when present and falls back to
latest only when absent. The second new test checks an empty store, not that
choice. Thus neither establishes the documented always-latest rule when both
manifests exist.

Inspection found a real multi-version store: `data/artifacts/source-manifest/`
holds `eccb9a1d…`, `8d58abdc…`, `53ec10ee…` and `8bc9f2fc…`, each recording a
different source-code digest. The selection pins `eccb9a1d…`; the latest pointer
names `8bc9f2fc…`. The source chooses the latter. This supports the intended
choice by inspection, but does not exercise it. `latest_key` reads a pointer;
reuse can reset that pointer, so latest means most recently selected upstream
output, not the greatest code version or newest creation time.

The review charter explicitly asks for two code versions and a temporary
violation that a test detects. No such execution evidence is recorded for this
change. The review-kind procedure prohibits performing those actions here.
Integration owns that finite verification, restoration and the resulting clean
check. The missing evidence alone is not a finding that current code chooses
the wrong manifest.

## Evidence checked

Every abbreviated artifact key in the report resolves in the named store.
Read-only inspection checked record keys and all payload sizes/SHA-256 values
for 30 relevant artifact directories (the 20 clean-run artifacts plus referenced
working-store artifacts), covering 2,005 payload files, without a mismatch.
This checks stored evidence integrity; it does not prove the producing code or
its scientific interpretations correct.

- The three chapters have 711, 406 and 1,245 evidence lines. Their recorded
  structure counts match the report, including 31 inference rules and four code
  blocks for `H3MVK7J5`. Interpretation counts match all three table rows.
- Discovery `219849511e73…` names response `176068ac96ab…` and records 26
  patterns, 104 sections, 254 interpretations and 203 unfitted observations.
  Fourteen patterns use one book. The recipe pattern has 10 supports and two
  exceptions, recipes 6.12 and 6.13.
- Pattern `p013` has eight supports, no exceptions and two unresolved entries.
  Its move `6IQTDYAW:976d6b678278:i027` names episode `i025` and five elements,
  all on PDF page index 113, printed page 101. Element `e0072` contains evidence
  lines `p0113-b010-l00` through `l05`. Its existing PNG was visually inspected
  and shows the cited truthiness paragraph. Alternative `i074` remains unresolved
  with high uncertainty.
- Historical change summary `4a3f921d…` records 594 reused files, four changed,
  76 added and 58 removed; 18 changed matched patterns, two removed and two new;
  and causes 3 correction, 155 rerun, 14 either. Its before/after discovery
  artifacts record 226 and 239 unfitted observations. These historical numbers
  remain valid despite the current correction file having changed.
- Baseline `a657a851…` and weighted report `7bb31fc4…` give equal thirds and
  shares 0.571428…, 0.285714…, 0.142857…. All six sampled rank/prevalence rows
  match, as do coverage 1.0 for `p008`, `p013`, `p015` and `p017`. Illustrative
  weights, relevance and uncertainty remain distinct; discovery is fixed.
- The clean R1/R2 transcript records 20 artifacts, four then zero model calls,
  with correction refusal in both runs. The evidence supports partial pipeline
  execution plus reuse, not a successful complete restart of that run.
- Report `b9030c4c…` contains 603 files. Its index has 378 anchor elements
  (344 distinct hrefs), rather than the stated 354 links. This incidental count
  can be corrected while reconciling R1; it is not a separate completion gate.

## The nine requested completion subjects

| Requested subject | Assessment |
|---|---|
| Implementation and locations | Present in section 1; stage and store boundaries are identified. |
| Reproduction commands | Present in section 2, with an explicit failed-correction workaround; R1/R2 limit the completion claim and reliability of that entry point. |
| Three books and chapters | Present in section 3; sampled counts agree with the store. |
| Examples linked to evidence | Present in section 4; the complete p013 chain and render agree. |
| Demonstrated correction and downstream effects | Present in section 5; historical summary figures agree, but the current correction and transcript need reconciliation under R1. |
| Equal-book versus supplied weights | Present in section 6; sampled arithmetic and ranks agree, and illustrative preferences and fixed discovery are labelled. |
| Recovery and reuse | Present in section 7; linked evidence has advanced since the report, as R1 explains. |
| Remaining limitations | Present; OCR, layout, model variation, unresolved fits, unverified Grove mid-stage resume and missing workbook mapping are distinguished. No new access limitation was invented. |
| Next smallest step | Present; the observed failure motivates investigation of correction portability, but the proposed automatic targeting is not established as the smallest necessary acceptance step after successful manual retargeting. |

Later integration records 20 passing resumability tests and nine of nine
recovery mutations caught. Those support the tree at that integration; they
do not substitute for the specific latest-manifest mutation/two-version
verification requested by this review. This review claims no new test result.
