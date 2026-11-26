# Pattern discovery and the pattern report

The `pattern-discovery` stage reads the analysed chapters and asks a model for candidate patterns. A pattern is a recurring structure across the observations. Each pattern names its supporting instances, its exceptions and its unresolved cases. The stage checks each citation against an inventory of observations.

The `pattern-report` stage then writes a static HTML view of the result. Both stages propose. They do not decide.

## Commands

Discover patterns and build the report:

```sh
task pattern-discovery
```

Call the model again, and keep the earlier answer:

```sh
uv run urg pattern-discovery --new-attempt
```

Build the report again for the latest discovery run, without a model call:

```sh
task pattern-report
```

Build the report for one discovery run:

```sh
uv run urg pattern-report --discovery <discovery key>
```

The stage reads the latest `episode-analysis-run` report. It uses each chapter that the report marks as analysed.

## Inputs

- `docs/discovery/settings.json`: the discovery settings. They give the minimum number of supporting instances, the preference weight of each book and the observation scope.
- `docs/discovery/instructions.md`: the discovery instructions. The front matter gives an integer `version`. The body is the system prompt of the model call.
- `docs/discovery/model.json`: the model configuration.
- The `episode-analysis` artifact and the `chapter-extraction` artifact of each chapter.

## Observations

An observation is a recorded instance that a pattern can cite. The stage builds an inventory of observations from the two lower layers:

- Document structure: each section of each chapter, with its heading, its level and its page label. The prompt lists each section and the kinds of element in it.
- Interpretations: each interpretation of each chapter, with its kind, its label, its description, its references and its evidence.

The inventory also holds each element and each evidence line. The prompt names them only as the evidence of an interpretation. A pattern can cite any observation in the inventory. The list of unfitted observations covers only the sections and the interpretations, because the prompt lists them one by one.

## The discovery settings

The settings are the dependency of every later discovery. `minimum_supporting_instances` is the smallest number of supporting instances that a pattern needs. The stage rejects a pattern with fewer instances.

`preference_weights` gives a default weight and an optional weight for each book. The prompt states the weight of each book. A changed weight changes the prompt. The stage then calls the model again and writes a new discovery run. A change of weight is a new discovery, not a reweighting of an old one. The weighted-reports stage reweights without a model call.

`observation_scope` names the interpretation kinds and the structure layers that the inventory holds.

## The model answer

The model answers with one JSON object. Each pattern has a local ID, a name, a description, the three lists of citations, the grammar fragment, the representation needs and an uncertainty. A citation names one observation ID and gives a note.

A grammar fragment is a list of refinement rules in the form `Name -> Part Part? Part* Part+`. A name starts with a letter and holds letters, digits, `_` and `-`. A part is a name with an optional modifier. `?` marks an optional part, `*` marks a part that repeats zero or more times and `+` marks a part that repeats one or more times. A representation need names one feature and the cases that justify it. The four features are `hierarchy`, `cross-reference`, `attribute` and `refinement-rule`.

## Validation

The stage checks each proposed pattern. It rejects a pattern for these problems:

- A citation names an ID that is not in the inventory.
- A citation names an observation that the pattern cites already.
- The pattern has fewer supporting instances than the minimum.
- A rule does not have the refinement-rule form. The check covers the whole rule. A rule with a second arrow, an unknown modifier, a part that is not a name or a second line fails.

The discovery record keeps each rejected pattern with its reasons. A pattern with no supporting instance is a rejected pattern, not a malformed answer. A malformed answer fails the run. A missing field or a field of the wrong type makes the answer malformed. The stage keeps the response, and `--new-attempt` calls the model again.

A failed model call fails the run. When the call produced output, the stage keeps the request, the output and the failure as a `pattern-discovery-failed-response` artifact. The stage never reads that artifact as a response. A rerun calls the model again, and each failed call gets its own artifact. A call that never ran, such as a call to a missing executable, keeps nothing.

The stage computes the basis of each pattern from the layers of its citations. The basis is `document-structure`, `interpretation` or `document-structure-and-interpretation`.

## Unfitted observations

An observation fits a pattern when an accepted pattern cites it as a supporting instance. A section also fits when an accepted pattern cites one of its ancestor sections as a supporting instance. An exception or an unresolved citation does not make an observation fit. The `unfitted` list holds each listed observation that no pattern supports. An unfitted observation that a pattern cites as an exception or as unresolved names that pattern and the role.

The `unfitted` list shows the coverage of the proposal. It is not an independent judgement that an observation fits no pattern. The model cites a sample of the instances of a pattern. An observation that the model did not cite stays in the list, also when it fits a pattern. The stage does not add the child moves of a supporting episode to the pattern. The report labels the list "Observations that no pattern supports".

## Artifacts

`pattern-discovery-response` keeps the raw model output:

- `request.json`: the system prompt and the user prompt.
- `raw-response.json`: the output of the harness, byte for byte.
- `metadata.json`: the models that answered, the token use, the duration and the harness version.

`pattern-discovery` holds one discovery run:

- `discovery.json`: the discovery record.
- `summary.md`: a short readable summary.

`pattern-report` holds the report of one discovery run:

- `index.html`: the patterns, with their grammar fragments, their citations, the unfitted observations, the rejected patterns and the discovery settings.
- `interpretations.html`: the interpretations of each chapter, with links to their evidence.
- `structure.html`: the sections, the elements and the cited evidence lines of each chapter, with page labels, page indexes and renders.
- `report.json`: the pages, the layer of each page and the outcome of each render.
- `render-<id>.png`: one render of each cited element or line.

## The discovery record

`discovery.json` holds these fields:

- `discovery_run`: the method, the settings and the resolved preference weight of each book. The settings carry their path, digest and content. The field also names the instructions version, the model, the response artifact and the attempt.
- `sources`: each chapter, with its analysis artifact, its extraction artifact and its preference weight.
- `counts`: the listed observations of each layer, the patterns, the rejected patterns and the unfitted observations.
- `inventory`: each observation by ID.
- `patterns`: the checked patterns. Each pattern has the review status `proposed`.
- `rejected`: the rejected patterns and their reasons.
- `unfitted`: the unfitted observations.

## The three layers in the report

Each page of the report holds one layer:

- `structure.html` holds the observed document structure.
- `interpretations.html` holds the candidate episodes, moves and relationships.
- `index.html` holds the inferred patterns.

A link from a pattern goes to the page of the cited observation. A link from an interpretation goes to its evidence on the structure page. Each cited element shows its page label, its page index and a render of its region. A render that fails leaves a note in place of the image. The report manifest records the failure.

## Reuse and invalidation

The response artifact depends on the settings, the instructions, the analyses, the extractions, the model and the attempt. The discovery artifact depends on the same inputs and on the response. A rerun with unchanged inputs reuses both, and it makes no model call. The report artifact depends on the discovery artifact and the render resolution. A rerun reuses it.

## Results

The first run covers the three analysed chapters. It used `claude-opus-5-5` at high effort, and each book had the weight 1.0.

- Discovery run: `e470dcc1112a8ab7adf9b832d13cd66df0dec35f4491752b1b71c88cff95e0dc`.
- Report: `88ca26a0241322d10a7e954c9f1a94cda56a2e80da5bd27cc2d360d82e7e6658`.
- Listed observations: 104 sections and 290 interpretations.
- Patterns: 20 accepted and 0 rejected. Thirteen patterns rest on two or three books, and seven rest on one book.
- Unfitted observations: 226, of which 199 are interpretations and 27 are sections. The list holds observations that fit a pattern but that the model did not cite. The review found two: `6IQTDYAW:0fbbac3a863e:i003` and `H3MVK7J5:da01f8e04a43:i001` fit `p008`, the opening preview that organises the sections.
- The report holds 2,297 links, and each link resolves. Each of the 652 renders succeeded.

A rerun with unchanged inputs reused the response, the discovery run and the report. `docs/representation.md` records the representation needs of the patterns.

## Known limitations

- One model call proposes every pattern. The stage checks citations, not whether a pattern is apt. A review compares sampled patterns with their passages.
- The minimum counts supporting instances, not books. A pattern can rest on one book.
- The prompt lists sections, not elements. A pattern about one element kind cites the sections that hold it.
- The report renders from the source PDF. A moved or changed PDF leaves the report without renders.
- The `unfitted` list includes the omissions of the model. No stage attaches an uncited observation to a pattern. The corrections stage corrects interpretations, not the citations of a pattern.
- The instructions version 1 explains `?` and `*` but not `+`. The checker accepts `+` because the first run used it with its usual meaning. A later instructions version can name it. That change calls the model again.
