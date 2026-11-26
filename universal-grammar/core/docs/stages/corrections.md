# Corrections and the change summary

The `corrections` stage records corrections to interpretations. A correction creates a new version of one interpretation. The stage never changes evidence or an earlier version.

The `change-summary` stage compares the derived results before and after the corrections. It lists the artifacts that changed and the artifacts that the pipeline reused. It also gives a cause for each difference in the patterns.

Neither stage calls a model or runs the extraction. Pattern discovery calls the model again when a correction changes its input.

## Commands

Apply the correction files in `corrections/`:

```sh
task corrections
```

Find the patterns again, and build the report for the corrected analyses:

```sh
task pattern-discovery
task pattern-report
```

Compare the results before and after the corrections:

```sh
task change-summary
```

If the stage cannot find the run before the corrections, name the runs:

```sh
uv run urg change-summary --before <discovery key> --after <discovery key>
```

The stage checks a named run in the same way as a run that it finds. The run before the corrections must read the uncorrected analyses of the corrections run. Both runs must have the same method, settings, instructions, model, preference weights and code version. The instructions must agree in version and in content hash. Otherwise the stage refuses the pair and gives each difference.

## Correction files

Each correction is one JSON file in `corrections/`. The stage applies the files in file-name order. A number at the start of each name therefore sets the order.

| Field | Content |
| --- | --- |
| `id` | The name of the correction, in lower case with hyphens. |
| `author` | `kind` is `human` or `reviewer`, and `name` names the author. |
| `origin` | Where the correction comes from, such as a review finding or a close reading. |
| `rationale` | Why the interpretation is wrong, with the passages that show it. |
| `target` | The `id` of one interpretation and the `version` that the correction changes. |
| `operation` | `revise` or `add-alternative-reading`. |
| `revise` | The fields that change: `label`, `description`, `evidence`, `uncertainty`, `episode`, `source` or `target`. |
| `reading` | For a new alternative reading: `label`, `description`, `evidence` and `uncertainty`. |

A change to the `evidence` of an episode moves its boundary. The list replaces the earlier list. A reference names an interpretation id, or, for a relationship, an evidence id.

## Validation

The stage refuses a correction in these conditions:

- The file is not valid JSON, or a field is missing or has the wrong type.
- No analysed book holds the target interpretation.
- The target version is not the current version of the interpretation.
- The correction cites evidence that the extraction does not contain.
- The correction adds a reference that the kind does not have, or removes a required reference.
- The `episode` reference of a move names an interpretation that is not an episode.

The stage applies the other corrections and records each refusal with its reason. The command exits with status 1 when it refuses a correction.

## Artifacts

A book with corrections gets a new `episode-analysis` artifact. A book without corrections keeps its original artifact.

The corrected record keeps the stable id of each interpretation. A corrected interpretation has these fields in addition to those of the analysis stage:

| Field | Content |
| --- | --- |
| `version` | The earlier version plus one. A new alternative reading has version 1. |
| `review_status` | `corrected`. The interpretations that no correction changed stay `proposed`. |
| `author` | The author of the correction. |
| `correction` | The `id`, `origin` and `rationale` of the correction, and the version that it revises. |

The record also holds `history`, the superseded versions with the correction that superseded each one. It holds `revision`, the original analysis artifact and the list of applied corrections.

The run report `analysis-corrections-run` has the result list of an episode-analysis run, with the original and the current analysis of each book. It also lists the outcome of each correction file. Pattern discovery reads this report when its base run is the latest episode-analysis run. When the analyses are newer than the corrections run, pattern discovery stops and asks for a new corrections run.

The `change-summary` artifact holds `changes.json` and `summary.md`. It lists the corrected interpretations, the artifacts that changed or that the pipeline reused, the report files that changed, and the differences in the patterns.

One corrections run can apply several corrections to one interpretation. The summary compares each corrected interpretation with its version in the base analysis, which the earlier discovery run read. It lists every correction in that interval with its author, origin and rationale. An alternative reading has no base version, so each of its fields counts as added.

## Causes of pattern differences

Pattern discovery calls a model over all the observations. A new call can change patterns that no correction touched. The change summary therefore gives each difference a cause:

- `correction`: the difference involves an affected observation.
- `rerun`: the difference involves only observations that the corrections did not affect.
- `either`: a field of a pattern changed, and the pattern cites an affected observation. A changed note on an affected observation also has this cause.

The stage compares the name, description, basis, books, rules, representation and uncertainty of each pattern pair. It compares the observations in each role. For an observation that both runs cite in one role, it also compares the note.

An affected observation is a corrected interpretation, an interpretation that refers to one, or evidence that a correction added or removed. The stage pairs the patterns of the two runs by name, and then by the overlap of their citations. The rule is a heuristic. It cannot prove that the model would repeat a `rerun` difference.

## Reuse and invalidation

The corrections stage applies every correction again on each run. When the correction files and the analyses do not change, the stage reuses every artifact. A new correction changes only the analysis of its book. The extraction and the model response of the analysis stay the same.

## Results

The demonstration correction `c001-unary-not-ends-implementation` comes from close reading of Crafting Interpreters, section 7.2.3. Episode `6IQTDYAW:0fbbac3a863e:i022` implements the unary operators. The text announces two unary expressions in `e0056` and shows only negation. It gives logical not in `e0067` and `e0068`, in the same switch. The correction moves the end of the episode from `e0066` to `e0068`. It also revises the description and raises the uncertainty from low to medium.

Corrections run `43c0bff6` wrote the corrected analysis `d7f5326b` for this book. The change summary `c75a09b7` compares discovery run `f1e40eb0` with discovery run `801dacaa`. It uses schema version 2. It replaces the summary `4a3f921d`, which compared fewer pattern fields:

| Artifact | Result |
| --- | --- |
| Chapter extraction, three books | Reused |
| Episode-analysis response, three books | Reused |
| Episode analysis, `H3MVK7J5` and `PZZBPXX9` | Reused |
| Episode analysis, `6IQTDYAW` | Changed, `8cda2b67` to `d7f5326b` |
| Pattern-discovery response | Changed, `3119a52f` to `390ed486` |
| Pattern discovery | Changed, `f1e40eb0` to `801dacaa` |
| Pattern report | Changed, `7983c6f1` to `1c462105` |

The report reused 594 files, changed 4 files, added 76 renders and removed 58 renders. The renders follow the elements that the patterns cite.

The new model call changed all 18 patterns that matched by name or citations. Two patterns disappeared and two are new. The unfitted observations rose from 226 to 239.

The change summary gives 3 differences to the correction, 250 to the rerun and 20 to either cause:

- Pattern "deferral to later material" now lists move `i025` as unresolved. Move `i025` belongs to the corrected episode.
- Pattern "stepwise increase in complexity" no longer cites move `i026`. Move `i026` belongs to the corrected episode.
- Move `i026` is now unfitted.
- 18 field changes belong to four patterns that cite an affected observation.
- 2 note changes are on the affected observations `i023` and `i077`.

The earlier summary `4a3f921d` gave 155 rerun differences and 14 to either cause. The difference comes from the basis, books and representation fields and from the citation notes. 81 of the 95 added rerun differences are note changes.

The correction changed no move of episode `i022`. The three differences therefore follow the rule, but the evidence of a causal link is weak. The 250 rerun differences show that one model call varies far more than one boundary move changes. A comparison of two calls over unchanged inputs would measure that variation. This session did not make that comparison.

## Known limitations

- The cause of a difference is a heuristic, as the section on causes describes.
- A new discovery call varies more than one correction changes. The cause rule cannot separate the two with one call.
- A correction cannot remove an interpretation. It can lower the uncertainty level or add an alternative reading.
- The stage does not check that a revised episode stays inside one section.
- The report renders only the elements that patterns cite. An element that only a correction cites, such as `e0067`, has its text and no render.
