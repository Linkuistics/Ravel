# Episode analysis

The `episode-analysis` stage reads each extracted chapter and asks a model for candidate interpretations. An interpretation is an explanatory episode, a move, a relationship, an alternative reading or an observation. Each interpretation cites evidence from the extraction. Each one has the review status `proposed`. The stage proposes. It does not decide.

## Commands

Analyse every extracted chapter:

```sh
task episode-analysis
```

Analyse one book:

```sh
uv run urg episode-analysis --book H3MVK7J5
```

Call the model again for one book, and keep the earlier answer:

```sh
uv run urg episode-analysis --book H3MVK7J5 --new-attempt
```

The stage reads the latest `chapter-extraction-run` report. It analyses each chapter that the report marks as extracted.

## Inputs

- `docs/analysis/instructions.md`: the analysis instructions. The front matter gives an integer `version`. The body is the system prompt of the model call.
- `docs/analysis/model.json`: the model configuration. It names the harness, the model, the effort and the timeout.
- The `chapter-extraction` artifact of each chapter.

The prompt lists each structure element of the chapter in reading order. Each element starts with its ID, its kind and its printed page label. The model cites these IDs. It can also cite the ID of one evidence line.

## The model boundary

Every model call passes through one `ModelBoundary` in `urg/model.py`. The boundary has two methods. `describe` returns the identity of the model for the dependency record. `call` sends one request and returns the raw output.

`ClaudeCliModel` runs `claude -p` with these settings:

- `--tools ""`: the model has no tools. It answers from the prompt alone.
- `--safe-mode`: no CLAUDE.md file, skill, plugin, hook or MCP server reaches the call.
- `--no-session-persistence`: the call stays out of the session history.
- The working directory is `/`.
- The child environment comes from an allowlist. The stage removes variables such as `ANTHROPIC_API_KEY`, which move the call to metered access.

A call that answers through a provider other than the subscription fails. The tests use `RecordedModel`, which replays recorded answers and makes no call.

A call can fail at the boundary after the harness produced output. The output can be text that is not JSON, or JSON that is not an object. The harness can also report an error, or omit the answer text. A timeout can leave partial output, and a provider other than the subscription can answer.

In each of these cases, the boundary raises `ModelError` and attaches a `FailedOutput`. That record holds the standard output, the standard error and the reason. The stage keeps it as an `episode-analysis-failed-response` artifact before it reports the failure.

## Artifacts

The stage writes four kinds of artifact.

`episode-analysis-response` keeps the raw model output for one chapter:

- `request.json`: the system prompt and the user prompt.
- `raw-response.json`: the output of the harness, byte for byte.
- `metadata.json`: the models that answered, the token use, the duration and the harness version.

`episode-analysis-failed-response` keeps the output of one failed call for one chapter:

- `request.json`: the system prompt and the user prompt.
- `stdout` and `stderr`: the output of the harness, byte for byte. After a timeout, these files hold the partial output.
- `failure.json`: the error message, the reason, the exit code or the timeout, and the harness version.

A failed call keeps its own artifact name. The stage therefore never reuses a failed call as an answer. A rerun calls the model again, and each failure gets a new ordinal in its dependency record. A call that never ran keeps nothing. Examples are a missing executable and an exhausted `RecordedModel`.

The review `episode-analysis-k15` found this gap as finding R1, and `episode-analysis-k24` repaired it. Pattern discovery calls the same boundary. The review `pattern-report-k16` found the same gap there, and `pattern-report-k25` repaired it with the artifact `pattern-discovery-failed-response`.

`episode-analysis` holds the checked interpretations of one chapter:

- `analysis.json`: the record.
- `summary.md`: the counts and a table of the interpretations.

`episode-analysis-run` records the outcome of each chapter in one run. The status of a chapter is `analysed`, `rejected` or `failed`. For a failed call, `failed_response_key` names the kept output, and `response_key` is null.

## The interpretation record

Each interpretation in `analysis.json` has these fields:

| Field | Content |
|---|---|
| `id` | `<item key>:<response key prefix>:iNNN`. The ID names the response that produced it. |
| `version` | `1`. A correction records a new version in a later stage. |
| `kind` | `episode`, `move`, `relationship`, `alternative-reading` or `observation`. |
| `label` | The provisional label that the model chose. An observation has no label. |
| `description` | The reading, in one or two sentences. |
| `evidence` | Each cited ID, with its layer, its PDF page index and its printed page label. |
| `uncertainty` | The interpretation uncertainty: a level and a reason. |
| `author` | The harness, the model, the effort and the instructions version. |
| `review_status` | `proposed`. Review and correction change it in a new version. |
| `local_id` | The ID that the model used in its answer. |

A move names its `episode`. A relationship names its `source` and `target`. An alternative reading names the interpretation that it competes with in `alternative_to`. Each reference gives its layer: `interpretation`, `structure-element` or `evidence-line`.

The instructions give no move catalogue. The labels are the model's own phrases. A later stage compares them across chapters.

## Validation

The stage checks each answer in two steps.

1. The stage parses the answer against a strict schema. An answer with a missing field, an unknown field, an unknown kind or a duplicate ID is malformed. The stage rejects the whole answer, and the run report records the reason. The raw response stays in the store.
2. The stage resolves each citation and each reference. It rejects an interpretation that cites missing evidence. It also rejects an interpretation that names a missing or rejected interpretation. The other interpretations stay. `analysis.json` lists each rejected proposal with its reasons.

The stage accepts one Markdown fence around the JSON object. It accepts nothing else around the object.

## Reuse and invalidation

The dependency record of a response names the instructions, the extraction, the model identity and the SHA-256 of the request. The dependency record of an analysis names the instructions, the extraction and the response.

- A rerun with unchanged inputs reuses the stored response and the stored analysis. It makes no model call.
- A new instructions version, or any change to the instructions file, invalidates the response and the analysis. It does not invalidate the extraction.
- A change to the analysis code invalidates the analysis. The stage checks the stored response again and makes no model call. A code change that gives a new prompt gives a new request digest, so it does cause a new call.
- `--new-attempt` stores a new response as the next attempt. The earlier attempts stay in the store.

Each stage names the code version of its own module and the URG modules that it imports. Extraction does not import the analysis or the model module. A change to either module therefore leaves each extraction valid.

The dependency record does not guarantee identical model output on a rerun. The model is not deterministic, and the provider can change it under the same name. The record makes a rerun traceable. It does not make it reproducible. The stored raw response is the reproducible part.

## Results

The real run on 17 November 2026 used instructions version 1 and `claude-opus-5-5` at high effort. Each call answered through first-party subscription access. The run rejected no answer and no interpretation.

| Book | Episodes | Moves | Relationships | Alternative readings | Observations | Output tokens | Duration |
|---|---|---|---|---|---|---|---|
| Crafting Interpreters, chapter 7 | 23 | 49 | 12 | 3 | 4 | 20,295 | 149 s |
| PFPL, chapter 5 | 21 | 46 | 15 | 3 | 4 | 15,823 | 121 s |
| Regular Expressions Cookbook, chapter 6 | 34 | 50 | 18 | 3 | 5 | 31,197 | 232 s |

The episodes of each chapter cite every structure element of that chapter: 264, 137 and 570 elements. No interpretation cites a single evidence line. Episodes that cite elements from more than one section number 12, 0 and 25. That count includes subsections, such as the Problem, Solution and Discussion parts of one recipe.

A rerun with unchanged inputs reused all three responses and all three analyses. It made no model call.

A spot check compared four sampled episodes with their cited elements. Each description matched its passage. The review leaf makes the full comparison.

## Known limitations

- The prompt carries element text only. Figures reach the model as their captions and nearby text. The model cannot see a figure.
- Each chapter is one model call. A chapter longer than the context window of the model fails. The three selected chapters fit.
- The answer for the Regular Expressions Cookbook used 31,197 output tokens. A longer chapter can reach the output limit of the harness. The answer then ends early, and the stage rejects it as malformed.
- The model chooses the episode boundaries. Two attempts can divide one chapter in different ways. The attempts stay side by side in the store.
- The validation proves that each citation exists. It does not prove that the evidence supports the reading. The review step compares sampled episodes with their passages.
