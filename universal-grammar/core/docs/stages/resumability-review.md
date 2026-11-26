# Resumability implementation review

The fixture tests and source support the six reuse and invalidation claims.
Acceptance evidence remains incomplete in two places: the real-data transcript
never demonstrates a successful complete restart and correction regeneration,
and the requested mutation checks have no recorded results. These are findings
about acceptance evidence; this inspection found no additional runtime defect
in the store changes.

This review covers `resumability-k10`, producer commit
`e154748f08ee185d2dce3da882780ede615ab21e`, against source at `9b4d4c73`.
The producer's store, fixture harness, resumability tests, document and transcript
are unchanged. Later stage integrations were considered against the current
source. No production or test code changed during this review. No tests,
builds, lint, format commands or model calls were run.

## R1: complete the real-data acceptance demonstration

The root contract requires a correction to regenerate affected derived results
and a restart to reuse completed work. The recorded transcript supports parts
of those conditions, but does not complete that demonstration.

- S2 follows a run whose corrections stage failed. The transcript helper at
  `scripts/resumability-transcript.sh:58` deliberately runs the stages separately
  and continues after failures. The real `Taskfile.yml` pipeline stops at the
  failed correction. S2's unchanged 76 artifacts and zero model calls demonstrate
  reuse by the individually invoked stages, not a successful complete pipeline
  restart.
- At `docs/resumability-transcript.txt:507`, S7 refuses the old `c001` target,
  applies `c900` to `PZZBPXX9`, and exits 1. Exactly two new artifacts appear:
  the corrected analysis and the corrections run. Discovery then fails at
  `docs/resumability-transcript.txt:519` with
  `claude --version failed: b'model call refused by the resumability transcript\n'`.
  The artifact count stays at 78. No corrected discovery or report is produced.
- The reported one model call is a refused version probe. `urg/model.py:251`
  calls `_harness_version` before invoking inference, so that count cannot
  establish that one completed discovery call suffices.

The limitations are disclosed in the producer's documents. Disclosure does not
fulfil the end-to-end acceptance condition. Integration should obtain a valid,
reviewed correction target for the selected analyses, then record a successful
complete run and unchanged restart, followed by successful selective correction
regeneration through reports. Preserve evidence of upstream reuse and actual
inference calls. Keep the existing failed run as adverse evidence. The missing
demonstration does not justify an automatic correction-retargeting redesign.

## R2: record the requested tests against broken behavior

The review task explicitly requests a broken-behavior check for each claim,
including replacing `reuse` with `find`, bypassing payload hash verification in
a stage's load path, and omitting an input from a dependency record. Neither the
producer's record nor the later recorded full check supplies those results.

Inspection identifies useful sentinels: returning to earlier inputs checks
`latest` pointers, damage tests check both direct rejection and selective repair,
and dependency-role assertions detect missing required inputs. Those assertions
are evidence of intent, not evidence that each requested mutation was detected.
For example, the generic code-version assertion at
`tests/test_resumability.py:123` checks only an `urg ` prefix; it does not exercise
changed source content. The input-change cases exercise instructions, model
effort, discovery settings and report weights, rather than every version/input
category.

The loaded review procedure prohibits executing tests or modifying production
or test code. The mutation checks therefore remain unverified and belong to the
integration's verification work. Record each temporary mutation, the sentinel
that fails for the intended reason, restoration, and the clean resulting check.
If a mutation survives, judge whether it exposes a missing guard and repair the
necessary test. Do not infer a production defect solely from this evidence gap.

## Six claims: source, fixture and transcript

| Claim | Fixture and source assessment | Recorded real-data evidence |
|---|---|---|
| 1. Relevant inputs are named | `test_every_dependency_record_names_its_inputs` checks roles, configurations and schema versions. `test_a_change_to_one_input_redoes_only_its_dependants` checks exact new artifacts and model calls for four edits. The dependency builders below include the named inputs. | S3 lists dependency shapes, including historical artifacts. It shows categories, not all values or current selection; source and fixture inspection supply those checks. |
| 2. Restart repeats no valid work | The restart test compares the entire artifact-to-`written_at` map and checks zero calls. Returning to earlier instructions additionally checks restored `latest` pointers. `reuse` validates through `find` and then updates the pointer. | S2 preserves all 76 artifacts with zero calls, but corrections fails; R1 limits the complete-run claim. |
| 3. A killed write recovers | The parametrized kill test requires SIGKILL, checks staging or publication state, completes the expected artifact set on rerun, and preserves completed artifacts' timestamps. Three state classes are covered below. | S4 kills before the second extraction rename; the rerun builds three missing artifacts with zero calls. Only this kill class is demonstrated on real data. |
| 4. Damage is rejected and repaired | Six mutations cover truncated/flipped/missing payloads and missing/truncated/altered records. Direct `load` must raise `InterruptedWrite`, `find` must miss, and rerunning changes only the damaged artifact's timestamp with zero calls. `load` checks record identity, payload size and SHA-256. | S5 flips a stored analysis byte, observes rejection, and rebuilds the same key from its response without a model call. |
| 5. A correction affects only dependants | The correction test requires exactly seven new artifacts: corrected A analysis, corrections run, discovery response, discovery, pattern report, baseline report and weighted report. All old timestamps remain unchanged; discovery names corrected A and original B, with only one discovery call. | S7 proves exactly one new `episode-analysis`, for `PZZBPXX9`, and a new corrections run. Successful descendant regeneration remains unproved; R1. |
| 6. Weights preserve upstream work | The weight test requires exactly one new weighted report and zero calls. Baseline dependency excludes preference weights; weighted dependency includes them. | S6 grows 76 to 77 artifacts, adding only a weighted report with zero calls. The individually invoked corrections stage still fails. |

## Dependency inventory

The claim 1 categories are represented as follows. Artifact keys carry their
own dependency chains; a downstream record need not duplicate every transitive
input.

| Category | Inspected dependency construction |
|---|---|
| Source identity | `urg/manifest.py:143` names RDF, database and storage identity, plus the export snapshot; alias provenance names snapshot and manifest. `urg/extraction.py:185` names the PDF hash and canonical selection-entry hash, with stable source identity in configuration. |
| Extraction code and configuration | `urg/extraction.py:185` includes stage code/schema versions, settings, text flags, chapter and source identity. The extraction-run records selection, manifest and per-book results. |
| Schema and code versions | Stage builders set `SCHEMA_VERSION` and `code_version`. `urg/version.py:60` hashes the stage's transitive URG source closure. Raw model responses deliberately use the `urg.model` closure. |
| Analysis instructions and model | `urg/analysis.py:382` includes instructions, extraction key, model description, request hash and attempt. `urg/analysis.py:496` includes instructions/version, extraction, response and model configuration. |
| Interpretation versions and corrections | `urg/corrections.py:334` includes base analysis, correction file identities and applied correction configuration. File contents carry target/version. The run at `urg/corrections.py:348` includes base run, all correction files and outcomes. |
| Discovery settings | `urg/discovery.py:648` and `urg/discovery.py:762` include settings, instructions, request/model information, source extraction/analysis keys and response as applicable. Discovery selects corrected analyses only for the matching base run. |
| Reports and weights | `urg/report.py:519` names discovery, extraction keys and render configuration. `urg/weighted_report.py:474` builds baseline dependencies from discovery, extractions, analyses and relevance; weighted dependencies additionally name baseline and preference-weight content. |

## Kill windows and reuse

`ArtifactStore.write` writes payloads and the record in staging, renames the
directory into the artifact store, then replaces `latest`. The kill harness
models three distinct states visible after process death:

1. Incomplete staging, represented by a partial payload. Death while writing
   the record also leaves unpublished staging and follows the same recovery.
2. Complete staging before rename. It remains unpublished and the stage rebuilds.
3. Complete published artifact before `latest`. A rerun validates and reuses it,
   then repairs the pointer rather than rewriting valid work.

Death before staging exists leaves no new output. Atomic rename and pointer
replacement expose either side of their transitions; death while writing the
temporary pointer has the third state's recovery. `write` also updates `latest`
when an existing intact artifact wins. The state coverage is adequate for the
claimed single-process SIGKILL recovery. It does not establish power-loss
durability, concurrent-writer behavior or exactly-once external model calls.
Abandoned staging directories are a disclosed disk-space limit.

## Model response provenance and correction calls

Keeping the extraction key in an analysis response dependency preserves the
exact evidence version behind the response. The request hash also covers the
model input, but relying on it alone would change that provenance contract.
The broad invalidation after editing `urg/store.py` is conservative and
documented. It is an accepted trade-off, not a reason to remove the extraction
identity as a repair here.

A correction derives a new analysis from the stored base and correction files;
it does not request a new chapter analysis. The changed analysis key makes the
discovery response and its descendants stale. Source inspection and the fixture
test support one required discovery inference in a successful run, with no new
extraction or chapter-analysis inference. S7 confirms that only `PZZBPXX9` gains
an analysis key, but its refused version probe cannot verify completion.

## Verification limits and handoff

The corrections integration record reports the latest full `task check` as
156 tests passing, ruff clean and pyright with zero errors. This review relies
on that recorded result and does not claim a fresh run or completed mutations.

Codebase Memory indexing refused to start because a pre-coordination or
unverified generation was active. No project generation or coverage result was
available. This review used direct source, test, diff and document reads within
the stated scope.

R1 and R2 warrant an adjacent integration before the completion-report review.
That integration must triage the findings against the contract and record their
dispositions. The review itself is complete as an inspection; the unproved
acceptance work remains live in the handoff.
