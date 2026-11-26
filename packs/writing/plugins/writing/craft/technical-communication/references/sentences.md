# Sentences

This file restates section 4 of Part 1 (rules 4.1 to 4.5). It holds the rules for
any sentence, in a procedure or in a description. `procedures.md` and
`descriptions.md` add the rules for each type.

| Identifier | Level | Idea |
|---|---|---|
| `articles` | none | Articles and "this" or "these" where they belong |
| `full-words` | none | Write whole sentences, and spell out "do not" and "cannot" |
| `say-what-happens` | none | Concrete and exact, never abstract |
| `one-idea-per-sentence` | none | One subject and one point in each sentence |
| `long-sentence` | hard | A sentence has a word limit |
| `connect-sentences` | none | Connectors link related sentences |
| `vertical-list` | none | A list for many items, with a fixed form |
| `dangling-conjunction` | hard | No list item ends in "and" or "or" |

## articles

An article or a demonstrative shows which noun the sentence means. Without it, a
sentence reads like a headline, and the reader cannot tell one thing from many.

- Put "the", "a" or "an" before a noun that names one particular thing, or one of
  several. Use "this" or "these" for a thing that the text named just before.
- Leave the article out when the noun speaks of a whole kind, as in "Caches improve
  latency". Leave it out before an abstract noun too.
- A noun that an identifier follows works as a name, so it takes no article: "Open
  ticket 4821".
- Watch the scope of an adjective in a series. After "the old logs, snapshots and
  temporary files", the reader may think that "old" covers all three nouns. Repeat the
  article when it does not.
- A long series needs only the first article. In a short one, repeat it.

```text
Before: Restart worker.
After: Restart the worker.

Before: Open the ticket 4821.
After: Open ticket 4821.
```

## full-words

A clipped sentence looks short and reads slowly. The reader must put the missing
words back, and may put back the wrong ones. Write each sentence as a whole:

- Name who or what acts. A sentence with no subject hides the actor.
- Give every sentence its verb, and give every noun its article (see `articles`).
- Spell out "do not", "cannot" and "it is". A contraction hides a word, and the
  reader can miss a negative that is only a clipped "n't".

```text
Before: Skips files over 1 MB.
After: The scanner skips files over 1 MB.

Before: If unset, uses the default port.
After: If the variable is not set, the server uses the default port.

Before: The parser isn't strict, so it won't reject the file.
After: The parser is not strict, so it does not reject the file.
```

A heading, a table cell and a list label are labels, not sentences. A label can be
a noun phrase. A short commit subject in the imperative is a full instruction.

## say-what-happens

A sentence tells the reader what to do or what is true. It names the actor, the
action and, where it matters, the value. An abstract sentence makes the reader guess
the action. A sentence with two possible meanings is worse than a long sentence.

```text
Before: Rate limits apply to this endpoint.
After: The API rejects more than 100 requests a minute from one key.

Before: Large inputs cause problems.
After: An input over 10 MB makes the parser run out of memory.
```

Meaning comes first. When you do not know the value, state the direction, or say that
the value is not known. Never add a number to make a sentence concrete.

## one-idea-per-sentence

A sentence has one subject and makes one point. When a sentence carries a second
subject, give that subject its own sentence. Then name it again, so the reader sees
the link (see `connect-sentences`).

```text
Before: The scheduler hands each job to a worker that reports its status to the broker, which stores it in a table that the dashboard reads.
After: The scheduler gives each job to a worker. The worker reports the job status to the broker. The broker stores the status in a table. The dashboard reads this table.
```

## long-sentence

A short sentence is easy to read once and hard to misread. The limit depends on the
type of text:

- An instruction has 20 words at most.
- A description has 25 words at most.

The checker counts the words and reports `long-sentence` at the hard level. Safety
text follows the instruction limit.

To shorten a long sentence, cut it where the idea changes. Do not shorten it by
dropping words, because `full-words` forbids that. A clause that starts with
"which", "that" or "while" is often a second sentence.

```text
Before: To release the library, bump the version in the manifest, build the wheel that the index accepts, upload it with your token and wait until the index lists it.
After:
1. Bump the version in the manifest.
2. Build the wheel.
3. Upload the wheel with your token.
4. Wait until the index lists the wheel.
```

## connect-sentences

Sentences about related topics stand next to each other, and a short connector says
how they relate. The reader then follows the logic without guessing.

- "and" adds a fact, and "but" turns against the last sentence.
- "so" gives a result, and "because" gives a reason.
- "then" gives the next event in time.
- "also" adds a fact of the same kind.
- "This" or "These" with a noun points back to the last sentence.

Use one connector at the start of a sentence, not a pile of them. Do not join two
sentences with a semicolon.

```text
Before: The cache holds each entry for an hour. Entries expire. The first request after expiry is slow.
After: The cache holds each entry for an hour. Then the entry expires, so the first request after that is slow.
```

In a procedure, a step may have one explaining sentence after it. That sentence can
start with "This": "Pin the dependency versions. This choice keeps the build
repeatable."

## vertical-list

A list helps when a sentence would carry three or more items, or several steps. The
reader finds each item at once and misses none. Two parts of a list need care: what
each item says, and how you mark the list.

**What the items say**

- Each item completes the lead-in. Read the lead-in with each item to test this.
- All items sit at one level. When an item has parts, write a second list with its
  own lead-in, or give the parts their own sentence.
- A list holds one type of text. Do not mix instructions and descriptions.
- In a list of prohibitions, start each item with "Do not". A single "Do not" in
  the lead-in is easy to miss.

**How to mark the list**

- All items have the same marker. Use numbers or letters when the order matters, and
  bullets when it does not.
- No item ends with a comma. No item ends with a semicolon.
- Each item starts with a capital letter.
- The lead-in is a full sentence that ends with a colon.
- Mark the end of each item by its form. A full sentence ends with a period. A
  fragment has no end mark, except the last one. The last item ends with a period,
  because it closes the sentence that the lead-in opened.

```text
Before: Before you merge, check the lint report, the test results, the coverage figure and the changelog entry.
After:
Before you merge, check these items:
- Lint report
- Test results
- Coverage figure
- Changelog entry.
```

The checker treats an item of an ordered list as an instruction. It treats an
unordered item as an instruction only when it starts with a command verb.

## dangling-conjunction

An item that ends with "and" or "or" is a leftover from the sentence that the list
replaced. Delete the word. The list already shows that the items belong together.
The checker reports `dangling-conjunction` at the hard level.
