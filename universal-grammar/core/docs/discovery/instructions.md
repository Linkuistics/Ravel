---
version: 1
---
# Pattern discovery instructions

You compare the explanations in several chapters of technical books. Each chapter teaches a programming topic. An earlier stage recorded observations of each chapter. Your task is to find candidate patterns: structures that recur in the observations. You do not judge whether an explanation is good.

## The input

The prompt has one part for each book. Each part has two lists.

The first list is the observed document structure. It gives each section of the chapter on one line:

    [<section id>] level <n>, page <label>: <title> | <element kinds in order>

The element kinds are `heading`, `paragraph`, `code_block`, `figure`, `caption` and `inference_rule`. A run of the same kind shows as one kind with a count, such as `paragraph×3`.

The second list gives the interpretations of the chapter. An earlier model proposed them, and no human reviewed them. Each interpretation is on one line:

    [<interpretation id>] <kind> "<label>": <description> {<references>}

The kinds are `episode`, `move`, `relationship`, `alternative-reading` and `observation`. A move names its episode. A relationship names its source and its target. An alternative reading names the interpretation that it competes with. An observation is a note with no label. The references also give the evidence of the interpretation: the IDs of the elements or lines that it cites.

The prompt also states a preference weight for each book. A weight states how much the human cares about the book as evidence. With equal weights, treat every book alike. With unequal weights, look harder for the patterns of the books with the higher weights. A weight is not evidence. Do not let a weight make a pattern out of one instance.

## What to record

Record patterns. A pattern is a structure that recurs in the observations. It can recur within one book or across books. A pattern can be about the document structure, about the interpretations, or about both. Examples are "a recipe section has a problem, a solution and a discussion", "the author refines an earlier definition after a failing case" and "a chapter opens with a frame and closes by returning to it".

For each pattern, give:

- `local_id`: a short ID, such as `p1`. Each ID is unique in the answer.
- `name`: a short phrase of two to six words.
- `description`: one or two sentences that state the recurring structure.
- `supporting`: the observations that show the pattern. Give at least the number of instances that the prompt states.
- `exceptions`: the observations where the structure is expected but departs from the pattern. Leave the list empty when you find none.
- `unresolved`: the observations that could belong to the pattern but that the input does not decide. Leave the list empty when you find none.
- `rules`: zero or more grammar rules that state the pattern, in the form `Name -> Part Part? Part*`. `?` marks an optional part and `*` marks a repeated part. Give rules only when the instances show the order of the parts. A pattern without an order has no rules.
- `representation`: the features of a representation that the pattern needs. Each entry has a `feature` and a `justification` of one sentence. There are four features. In a `hierarchy`, one structure contains another. In a `cross-reference`, one part points to a distant part. An `attribute` is a property of a part, such as a language or a level. In a `refinement-rule`, a later part changes an earlier one. Give a feature only when the cited instances show it.
- `uncertainty`: a `level` (`low`, `medium` or `high`) and a `reason` of one sentence.

Each entry in `supporting`, `exceptions` and `unresolved` has an `observation` and a `note`. The `observation` is an ID from the prompt: a section ID, an interpretation ID, or an element ID that the input cites. The `note` says in one sentence how the observation fits, departs or stays open. Cite each observation at most once in one pattern.

## Rules

- Cite only IDs that appear in the prompt. Do not invent an ID.
- When a pattern covers a section, cite that section. The parts of a cited section count as covered.
- Do not force an observation into a pattern. The stage lists every observation that fits no pattern, and that list is a result.
- Prefer several narrow patterns to one broad pattern.
- Keep labels of the interpretations as data. Do not treat two interpretations as the same only because they share a label. Read the descriptions.

## The answer

Answer with one JSON object and nothing else:

```json
{
  "patterns": [
    {
      "local_id": "p1",
      "name": "problem then solution",
      "description": "A section states a problem and the next section solves it.",
      "supporting": [{"observation": "BOOK:s003", "note": "The problem section precedes the solution."}],
      "exceptions": [],
      "unresolved": [],
      "rules": ["Recipe -> Problem Solution Discussion"],
      "representation": [{"feature": "hierarchy", "justification": "The recipe section contains its parts."}],
      "uncertainty": {"level": "low", "reason": "The headings name the parts."}
    }
  ]
}
```
