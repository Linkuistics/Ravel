# Descriptions

This file restates section 6 of Part 1 (rules 6.1 to 6.6). A description gives
information and no instructions, so it has no imperative verb. These are
descriptions:

- A report of what the model found, did or changed.
- A commit body, a pull request summary and the notes of a procedure.
- Why a design decision stands.
- How a system works and what parts it has.

The rules for any sentence (`sentences.md`) also apply here. A text that tells the
reader to act is a procedure, and `procedures.md` governs it.

| Identifier | Level | Idea |
|---|---|---|
| `paragraph-topic` | none | One topic in each paragraph, named first |
| `long-paragraph` | hard | A paragraph has 6 sentences at most |
| `gradual-order` | none | One new fact in each sentence, in a clear order |
| `repeat-key-terms` | none | The same word for the same thing |
| `long-sentence` | hard | A description has 25 words at most |

## paragraph-topic

A paragraph opens with a sentence that names its topic. That opening sentence links
to the text before it, often with a key word or a connector (see
`connect-sentences`). The rest of the paragraph supports the opening: it explains
the topic or adds detail. Each paragraph has one topic.

Skim only the opening sentence of each paragraph. The skim must tell the story of the
whole text. If a paragraph needs two opening sentences, it holds two topics, so
split it.

```text
A cache miss costs one round trip to the database. The round trip takes about 20 ms
on the production network. A miss also fills the cache, so the next request is fast.

The cache size is a fixed number of entries. ...
```

The first paragraph has the topic "the cost of a miss". The second paragraph starts
a new topic and says so in its first sentence.

Headings, lists and tables do not replace paragraphs. A list holds items of one
kind, and the sentence before the list names that group.

## long-paragraph

A paragraph has 6 sentences at most. A longer paragraph hides its topic and loses
the reader. Cut it where the topic shifts, and give the second part its own opening
sentence.

The checker counts the sentences of each paragraph and reports `long-paragraph` at
the hard level. It does not count a list item, a table cell or a heading as a
paragraph.

## gradual-order

Give the reader the information in small steps. Start with the subject in plain
terms. Then say what it is, what it has and what it does, one fact in each sentence.
A sentence with three new facts sends the reader back to its start.

```text
Before: The importer reads rows from the CSV, which a validator has already checked against the schema that the API publishes, and it sends each batch to the queue that the indexer drains.

After: The importer loads data from a CSV file. A validator checks each row against the API schema. The importer then sends the valid rows to a queue in batches. The indexer reads this queue.
```

Each sentence builds on the one before it. The new sentence names a thing that the
last sentence introduced, and it adds one fact about that thing.

## repeat-key-terms

Choose one word for each thing and use it every time. When the second sentence
repeats a key word of the first, the reader sees that they connect. A new word makes
the reader ask if there is a new thing.

```text
Before: The importer sends each job to the queue. The task waits there until a worker takes the unit of work.
After: The importer sends each job to the queue. The job waits there until a worker takes it.
```

Style does not justify a second word. "Job", "task" and "unit of work" are one thing
here, so the text uses "job". The checker finds some cases for actions
(`synonym-rotation`). It cannot find a rotation of nouns, so read for them.

A key phrase can connect sentences too. A phrase such as "when the queue is full" can
start one sentence and return in a later sentence that explains the result.

## long-sentence

A description has 25 words at most. The limit is higher than for a step, because a
description carries more context in each sentence. It is still a limit. Split a long
sentence where the subject changes.

```text
Before: A feature flag is a setting that turns a code path on or off at run time, so a team can ship the code early and enable it for users later, one group at a time.
After: A feature flag is a setting that turns a code path on or off at run time. A team can ship the code early. Later, the team enables it for one group of users at a time.
```
