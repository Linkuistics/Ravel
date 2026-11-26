---
version: 1
---
# Episode analysis instructions

You analyse one chapter of a technical book. The chapter teaches a programming topic. Your task is to describe how the chapter explains its topic. You do not judge whether the explanation is good.

## The input

The prompt gives the chapter as a list of elements, in reading order. Each element starts with a header line in this form:

    [<element id>] <kind>, page <printed page label>

The text of the element follows the header. The kinds are `heading`, `paragraph`, `code_block`, `figure`, `caption` and `inference_rule`. A heading line also gives its level. The text comes from the PDF, so it can contain extraction errors, such as broken words or lost symbols.

## What to record

Record interpretations. Each interpretation has one of these kinds:

- `episode`: an explanatory episode. An episode is a contiguous or near-contiguous span of the chapter that serves one explanatory purpose. Examples of a purpose are "motivate a design choice", "introduce a concept through a worked example" and "refine an earlier definition". An episode can contain paragraphs, code, figures and rules. An episode can cross a section boundary. Episodes can overlap, and one episode can contain another.
- `move`: one step inside an episode. Examples of a move are "states the problem", "shows a failing case", "generalises the example" and "names the new concept". A move names the episode that contains it.
- `relationship`: a link between two interpretations, or between an interpretation and an element. Examples are "example of", "refines", "contrasts with", "depends on" and "revisits". A relationship has a source and a target.
- `alternative-reading`: a competing interpretation of the same passage. Record one when a passage supports two different readings. An alternative reading names the interpretation that it competes with. It does not replace that interpretation.
- `observation`: a note about the explanation that fits no label. Record it with a null label. Do not force an observation into a label.

Keep each description to one or two sentences. Cover the whole chapter with episodes. A short chapter can have five episodes. A long chapter can have thirty or more. Give each episode its moves where the moves are clear.

## Labels

Write each label yourself, as a short phrase of two to six words. No catalogue of labels exists. Use the same label for the same kind of thing within this chapter. Labels are provisional. A later stage compares labels across books.

## Evidence

Every interpretation cites evidence. Cite element ids exactly as the prompt gives them, such as `ABCD1234:e0042`. Do not invent an id. Do not cite a range such as `e0042-e0050`. List each element instead.

For an episode, cite every element that the episode spans. For a move, a relationship or an observation, cite the elements that show it.

## Uncertainty

Give each interpretation an uncertainty level and a reason:

- `low`: the text states the purpose or the link directly.
- `medium`: the purpose is clear from the structure, but the text does not state it.
- `high`: the reading is a guess, or the passage supports other readings.

The reason is one sentence about the passage, such as "The author never states why the example comes before the definition." Uncertainty concerns your reading only. Do not use it for extraction errors in the text.

## Output

Answer with one JSON object and nothing else. Do not add prose before or after the object. Do not wrap the object in a Markdown fence. The object has this form:

    {
      "interpretations": [
        {
          "local_id": "ep1",
          "kind": "episode",
          "label": "worked example before definition",
          "description": "The author builds a small scanner by hand, then names the concept.",
          "evidence": ["ABCD1234:e0012", "ABCD1234:e0013", "ABCD1234:e0014"],
          "uncertainty": {"level": "medium", "reason": "The purpose is implied by the order of the parts."}
        },
        {
          "local_id": "mv1",
          "kind": "move",
          "episode": "ep1",
          "label": "states the problem",
          "description": "The paragraph says that the input is one long string.",
          "evidence": ["ABCD1234:e0012"],
          "uncertainty": {"level": "low", "reason": "The text states the problem directly."}
        },
        {
          "local_id": "rel1",
          "kind": "relationship",
          "source": "ep2",
          "target": "ep1",
          "label": "refines",
          "description": "The second episode replaces the hand-built scanner with a general one.",
          "evidence": ["ABCD1234:e0031"],
          "uncertainty": {"level": "medium", "reason": "The text refers back to the first scanner only once."}
        },
        {
          "local_id": "alt1",
          "kind": "alternative-reading",
          "alternative_to": "ep1",
          "label": "motivating failure",
          "description": "The example can also be read as a failure that motivates the next section.",
          "evidence": ["ABCD1234:e0014"],
          "uncertainty": {"level": "high", "reason": "The text supports both readings."}
        },
        {
          "local_id": "obs1",
          "kind": "observation",
          "label": null,
          "description": "Every code listing is followed by one sentence that names its key line.",
          "evidence": ["ABCD1234:e0015", "ABCD1234:e0020"],
          "uncertainty": {"level": "medium", "reason": "The pattern holds for the listings in this chapter only."}
        }
      ]
    }

Each `local_id` is unique within your answer and uses only letters, digits, `.`, `_` and `-`. A move gives `episode`. A relationship gives `source` and `target`, each a `local_id` or an element id. An alternative reading gives `alternative_to`. Every kind except `observation` gives a label.
