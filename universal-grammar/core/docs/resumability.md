# Resumability

The pipeline resumes after any stop. Each stage reuses each output whose dependency record is unchanged. It redoes only the outputs whose inputs changed. Grove runs the sessions, and Taskfile runs the stages. No other orchestration layer exists.

## Commands

Run every stage in order:

```sh
task pipeline
```

`task pipeline` calls the model for each analysis and each discovery that the store does not hold. It stops at the first stage that fails. The corrections stage fails when a correction targets an interpretation that the latest analysis does not hold. Give that correction a new target, then run `task pipeline` again.

Record the real-data transcript:

```sh
scripts/resumability-transcript.sh WORK_DIR > docs/resumability-transcript.txt
```

The script needs a store in `data/` and the inputs of every stage. It calls the real model in sections S1 and S7 only. `task pipeline` reads the corrections in `CORRECTIONS_DIR`, which is `corrections` by default. The script gives it an empty directory before S7.

Record the mutation checks:

```sh
.venv/bin/python scripts/resumability-mutations.py > docs/resumability-mutations.txt
```

The script breaks the mechanism in one place at a time and runs `tests/test_resumability.py`. It records the tests that fail, then restores the source. It exits with status 1 when a break survives every test.

## How a stage decides to reuse an output

Each output is a directory in `data/artifacts/NAME/KEY/`. The key is the SHA-256 of the dependency record of the output. The record names these values:

- the stage, the artifact name and the schema version of the stage
- the code version, which is a hash of the URG modules that the stage imports
- the configuration of the stage
- each input, by role, with its content hash or its artifact key

A stage builds the dependency record of each output before it does any work. If the store holds an intact output with that record, the stage reuses it. Otherwise the stage does the work and writes a new output.

The store writes each output into `data/staging/` first. It writes the payload files, then `record.json`, and then it renames the directory into `data/artifacts/`. Last, it moves the `latest` pointer of the artifact name. A reused output also moves the pointer, because a later stage reads its upstream run through that pointer.

The store rejects an output when any of these is true:

- `record.json` is missing or is not valid.
- A payload file is missing, or its SHA-256 differs from the hash in `record.json`.
- The dependency record in `record.json` does not hash to the key of its directory.

The stage moves a rejected output to `data/staging/damaged-UUID/` and writes it again.

## Claims and their evidence

Each claim has a fixture test in `tests/test_resumability.py` and a section of `docs/resumability-transcript.txt`. The fixture tests run the stage commands on a fixture library with a scripted model. The kill tests run each stage command in a child process that dies with SIGKILL. The transcript runs the same commands on the real library.

| Claim | Fixture test | Transcript section |
| --- | --- | --- |
| 1. Each dependency record names each input that applies to it. | `test_every_dependency_record_names_its_inputs`, `test_a_change_to_one_input_redoes_only_its_dependants` | S3 |
| 2. A restart after a complete run repeats no valid work. | `test_a_restart_after_a_complete_run_repeats_no_work`, `test_a_return_to_earlier_inputs_reuses_their_results_downstream` | S2 |
| 3. A stage killed during a write leaves an output that the store rejects, and a rerun completes it. | `test_a_killed_stage_leaves_a_rejected_output_and_a_rerun_completes_it` | S4 |
| 4. The store rejects an output that is incomplete or corrupted. | `test_the_store_rejects_a_damaged_output_and_a_rerun_rewrites_only_it` | S5 |
| 5. A correction invalidates its dependants and nothing else. | `test_a_correction_invalidates_its_dependants_and_nothing_else` | S7 |
| 6. A change to the weights reuses extraction, analysis and discovery. | `test_a_weight_change_reuses_extraction_analysis_and_discovery` | S6 |

### How to read the transcript

The transcript records one run of the script on the real library. The sections run in this order:

- S0 names the commit and the number of stored artifacts.
- S1 runs `task pipeline` on `data/` with the real model and no correction. It makes each model call that the current code needs.
  In the recorded run S1 makes no model call. An earlier attempt in the same session made those calls: three analyses and one discovery, because commits after the first run changed `urg/`. That attempt stopped in S7, because the new analysis gave `c001` a missing target. After a check of the new episode, `c001` got its present target, and the script ran again.
- S2 runs `task pipeline` again with each model call refused. It must write nothing and make no model call.
- S3 lists the dependency records in `data/` by shape.
- S4 kills a stage in a clone and reruns it. S5 damages one output in a clone and reruns. S6 changes the weights in a clone.
- S7 runs `task pipeline` on `data/` with the files in `corrections/` and the real model. The corrections stage writes a corrected analysis for one book. Discovery runs again with the real model, because it reads the corrected analysis. The pipeline reuses each extraction and the analyses of the other books.
- S8 runs `task pipeline` again with each model call refused. It must write nothing.

The shim counts each inference call that passes to the real model as `called`. It does not count the version check of a passed call. In a refused section it counts every call, the version check included, as `refused`.

The damage, weight and kill sections use copy-on-write clones of `data/`. The kill section uses a clone without the chapter extractions, so that the killed stage has work to do. Each section stops the script when `task pipeline` fails.

The transcript of commit 38f81e0 is adverse evidence that this version replaces. In that run, correction `c001` targeted an interpretation of an earlier analysis, so the corrections stage failed in each section. The script of that commit ran each stage by itself, so the later stages ran after the failure. That run showed no complete restart and no regeneration from a correction.

### Mutation checks

`docs/resumability-mutations.txt` records nine breaks of the mechanism. Each one must make at least one claim test fail:

| Mutation | Break | Claim |
| --- | --- | --- |
| M1 | A reused output does not move the `latest` pointer, so `reuse` acts as `find`. | 2 |
| M1b | The analysis run report calls `find` instead of `reuse`. | 2 |
| M2 | `load` skips the size and hash check of each payload file. | 4 |
| M3 | The corrected analysis omits its correction files. | 1 |
| M3b | A chapter extraction omits its chapter-selection entry. | 1 |
| M3c | The analysis run report omits its instructions. | 1 |
| M4 | The code version omits the hash of the module closure. | 1 |
| M5 | `find` never finds a stored output. | 2, 3 |
| M6 | The baseline report also depends on the weights file. | 6 |

The first run of these checks, in session resumability-k27, found two survivors. M3c survived because the dependency test did not check the run reports. M4 survived because the test checked only the prefix `urg ` of the code version. The test now checks the inputs of each run report and the full form of each code version, and both breaks fail it.

### Claim 1: inputs in each dependency record

The first test checks the dependency record of each output of a complete run, with one correction. These inputs appear:

- Source identity: the RDF export, the Zotero database and the storage files. Each chapter extraction also names its PDF file and records the source identity of that PDF.
- Extraction code and configuration: the code version, the settings, the text flags and the selected chapter.
- Schema version: each record carries the schema version of its stage.
- Analysis instructions and model configuration: the instructions file, the instructions version and the model configuration, with its effort.
- Interpretation versions and corrections: the corrected analysis names its base analysis and each correction file. Each correction file names the interpretation version that it targets. The record also lists the corrections that the stage applied.
- Discovery settings: the settings file, the instructions, the preference weights, each analysis and each extraction that discovery reads. Discovery names the corrected analysis, not its base analysis.
- Weight configuration: each weighted report names its preference weight file.
- Run reports: the extraction run names the chapter selection and the source manifest. The analysis run names the instructions and the extraction run. The corrections run names each correction file and the analysis run.
- Code version: each record names the package version and the SHA-256 of the modules that its stage imports.

The second test edits one input at a time. It checks that the rerun writes new outputs for that input's dependants only, and that it calls the model only where a dependant needs it.

### Claim 2: restart

The restart test reruns every stage after a complete run. The store does not change, and the model gets no call. The second test returns to the original analysis instructions after a run with edited ones. Each stage then reuses its earlier outputs, and each `latest` pointer names the same output as after the first run.

### Claim 3: a killed stage

The kill test stops a stage with SIGKILL at three points:

- `mid-payload`: during the write of a payload file. The directory in `data/staging/` has no `record.json`.
- `before-rename`: after `record.json` and before the rename. The directory in `data/staging/` is complete, but the store never reads `data/staging/`.
- `before-latest`: after the rename and before the `latest` pointer moves. The output is complete, but no later stage sees it until a rerun reuses it and moves the pointer.

In each case the rerun completes the run. It reuses each output that the killed run completed. It calls the model only for the work that the killed run did not store.

A directory that a kill leaves in `data/staging/` stays there. It costs disk space only.

### Claim 4: a damaged output

The damage test damages one stored analysis in six ways. It truncates a payload, flips one byte, deletes a payload, deletes `record.json`, cuts `record.json` short, and changes the recorded dependency. In each case `load` raises `InterruptedWrite` and `find` returns nothing. The rerun rewrites that one output from the stored model response, so it makes no model call.

### Claim 5: a correction

The correction test adds one correction to book A after a complete run. The rerun writes a new corrected analysis for book A, a new corrections run, a new discovery and new reports. It keeps the analysis of book B, and discovery reads that analysis unchanged. The only model call is the one discovery call, because discovery reads the corrected analysis.

### Claim 6: a weight change

The weight test changes the book weights after a complete run. The rerun writes one new weighted report. It reuses each extraction, each analysis, the discovery and the baseline report, and it makes no model call.

## Known limits

- Any change to a module that every stage imports, such as `urg/store.py`, makes every stored output stale. The rule is conservative by design. A rerun then calls the model again for each analysis and each discovery.
- A new analysis gives new interpretation ids. A correction to an earlier interpretation then fails, and `task pipeline` stops at the corrections stage until the correction gets a new target. A person must check that the new interpretation has the same fault before giving the correction its new target. Session resumability-k27 moved `c001` to `6IQTDYAW:4c555fc18422:i023` after that check.
- Nothing deletes a directory that a kill leaves in `data/staging/`.
