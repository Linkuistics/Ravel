# Words

This file restates section 1 of Part 1 (rules 1.1 to 1.14). The skill ships no
dictionary, so it cannot list the words to use. It states the principles behind
the choice of a word instead. Control of vocabulary is therefore approximate. The
checker finds a few cases by word shape. The model judges the rest.

The aim is simple. The reader knows each word at once, and each word has one
meaning in the text. The reader may be a person who learned English as a second
language. The reader may also be another model that has none of your context.

| Identifier | Level | Idea |
|---|---|---|
| `plain-word` | none | Choose the word that more readers know at once |
| `one-meaning` | none | Use a word in one sense in a text |
| `word-class` | none | A word keeps its job: no noun as a verb, no verb as a noun |
| `domain-terms` | none | A rare word stays only as a term of the field or the project |
| `no-coined-words` | none | Do not invent a word or a form |
| `short-terms` | none | A name that you choose has three words at most |
| `no-slang` | none | No slang, no regional word, no private jargon |
| `synonym-rotation` | hard | One word for one thing or action, every time |
| `domain-verbs` | none | A verb that names a process of the field may stay |
| `marketing-adjective` | hard | A measurement replaces a sales word |
| `spelling` | hard | British spelling |

## plain-word

Between two words that say the same thing, choose the one that more readers know
at once. The short word is usually the plain one. When a word is in doubt, ask
three questions:

1. Does a reader outside this team know the word?
2. Does the word have one meaning here?
3. Does a plainer word say the same thing?

```text
Before: Subsequently, the script attempts to ascertain whether the cache is populated.
After: Then the script tries to find out whether the cache has entries.
```

A plain word that says less is not a gain. When only the exact word carries the
meaning, keep the exact word. Meaning comes before this rule (see the precedence
order in `SKILL.md`).

## one-meaning

Many common words have several senses. Use a word in one sense in a text, and
choose the sense that the reader expects first. When a text needs two senses, give
one of them another word.

```text
Before: The editor supports Markdown. Please support the migration by testing it.
After: The editor accepts Markdown files. Please help with the migration by testing it.
```

A word that you restrict keeps the restriction. When a text says "task" for one
unit of work, it never says "task" for anything else. A word with a defined sense
in the project keeps that sense in every text of the project.

## word-class

A word has one grammatical job in a text. A noun stays a noun, and a verb stays a
verb. The reader meets a known word in a new job, and the sentence then needs a
second read. Models make this change easily, so check for it.

```text
Before: Ticket the failure, then screenshot the error.
After: Open a ticket for the failure. Take a screenshot of the error.

Before: The main ask in the issue is a faster build.
After: The issue asks for a faster build.
```

Some words are real domain nouns and real domain verbs, such as "commit", "build",
"log" and "patch". Use each job in the sense that the field gives it, in its usual
form. When the field does not use the word in that job, write the plain
construction. A past participle can serve as an adjective: "the compiled binary".
See `participle-adjective` in `verbs.md`.

## domain-terms

Plain English is not enough for software. Some words are rare in ordinary
English and exact in the field, such as "mutex", "schema", "daemon" and
"regression". A rare word stays when it is a domain term. A domain term has
three properties:

- It names a concept, part, tool, format, protocol, artefact or process of the
  reader's field or of the project.
- It is the name that the field or the project uses. The reader meets the same
  word in the docs, the code and the tools.
- The text uses it in that sense only.

When the project has a glossary or a context file, its words are the domain terms.
Use them as the file defines them. Read the file first, because a model does not
remember it. Define a term at its first use when a reader outside the project may
not know it.

Prefer the name of the field to a description of the thing. A name that exists
saves the reader a sentence.

```text
Before: Wait until the thing that keeps the order of the incoming jobs is empty.
After: Wait until the queue is empty.
```

The reverse also holds. A rare word is not a domain term because you like it. When
a common word says the same thing as exactly, use the common word.

```text
Before: The patch ameliorates the lag.
After: The patch reduces the lag.
```

An ordinary word can be a domain term with a narrower sense, such as "branch",
"leaf" or "lock". Keep to the domain sense, and do not mix it with the ordinary
sense in one text (see `one-meaning`).

## no-coined-words

Do not make a word that the reader has never seen. Do not add a suffix to a noun
to make a verb or an adjective. Do not make a new form of an old word. Irregular
verbs keep their usual forms, so write "ran" and never "runned". Compare only the
words that compare.

```text
Before: The rollout is unrevertable, so we pre-validated the schema twice.
After: We cannot revert the rollout. So we checked the schema twice before it.
```

A word that the field already uses is a domain term, and it stays. A name
that the project gives to its own command, tool or type is a domain term too.

## short-terms

When the field has no name for a thing, make one from common words, and keep it to
three words. An extra adjective earns its place only when it tells two things apart. `noun-cluster` in `noun-clusters.md` states the
limit and the ways to keep it.

When something else already identifies the thing, the bare noun is enough. A path,
a number, a label or an earlier step can do that. After a step that names
`build/out.json`, write "the output file", and not "the generated JSON build output
file".

## no-slang

Do not use slang, a regional word or a private jargon word, even when your team
uses it. A reader from another team or country, or a model with no context, may not
know it, and it has no entry to look up. Idioms and figures of speech belong here
too.

```text
Before: Nuke the cache, then dogfood the build.
After: Delete the cache. Then use the build in our own work.

Before: This flag is a footgun.
After: This flag causes data loss when a user sets it wrongly.
```

A domain term is not jargon. The test is the size of the audience. The whole field
knows "race condition", and the docs define it. A few people know a team's pet
name for a script, and no document defines it.

Private shorthand that depends on a past conversation, such as "the usual flow" or "the old way", fails the same test.
Name what you mean. This matters most in text for another model, which has no
memory of the conversation.

## synonym-rotation

One word names one thing or one action in a whole text, every time. A text that
says "job", "task" and "unit of work" makes the reader ask if there are three
things. The same holds for actions.

The checker knows groups of verbs that models often rotate, such as "start",
"launch" and "begin". It keeps the first word that it meets and flags each later
member of the group. It does not know nouns, so read for them. The neighbouring
case is `repeat-key-terms` in `descriptions.md`, which says the same for adjacent
sentences. The word that the code uses wins. When the code calls a thing `queue`,
the text says "queue".

When two words name two different things, keep both and make the difference clear.
A quotation and an identifier keep their own words, because they are outside the
skill.

## domain-verbs

A domain verb names a process of the reader's field. Software has many of them:
"compile", "parse", "serialise", "commit", "merge", "rebase", "deploy", "install",
"clone" and "revert". The model writes about code, commands and tools, so it uses
them often. The skill admits them under three tests. A verb must pass all three.

1. **The field names this process with this verb.** "Parse the file" names a
   precise process. "Look at the file" does not.
2. **No common verb says the same thing as exactly.** "Compile the module" has no
   plain equivalent. "Surface the error" has one: "show the error".
3. **The text uses the verb as a verb, in its domain sense.** `word-class` applies.

When a test fails, use a common verb or a plain construction. The admission does
not cover general business or fashion verbs, such as "leverage", "orchestrate" or
"unlock". It does not cover soft phrasal verbs either (see `phrasal-verb` in
`verbs.md`).

```text
Before: Leverage the schema to surface invalid rows.
After: Validate the rows against the schema. Show the invalid rows.

Keep: Parse the config file into a tree.
```

The name of a command is code, and the skill leaves code alone. In the text
"run `jj split`", the command is code. In the text "split the change", "split" is a
verb of prose, and the tests apply to it.

**The project's own verbs.** This repository's prose is the case to think about. It
describes a task tree, a version-control workflow and a Rust code base. It uses
verbs that only this project defines, such as "retire a leaf" and "decompose a
node". Such a verb is a domain verb when three conditions hold:

- The glossary or the context file of the project defines it for a process of the
  project.
- The text uses it in the defined sense only, and as a verb.
- A document for readers outside the project defines the verb at its first use or
  links to its definition.

When the glossary has no verb for a process, use plain words and do not invent a
verb. For example, write "mark the leaf as done". A project verb that is also an
ordinary word follows `one-meaning`. The word "retire" in a text about the task
tree does not also mean that a person leaves a job.

Some words are a domain noun and a domain verb at once. `word-class` states how to
use them.

## marketing-adjective

A sales word claims quality and gives no fact, such as "seamless", "robust" or
"blazing-fast". Delete it, or give the measurement that supports the claim. The
checker finds a short list of such words and reports `marketing-adjective` at the
hard level. Read for the others, such as "powerful", "elegant" and "intuitive". An
intensifier without a number is the same fault.

```text
Before: The cache gives seamless, blazing-fast lookups.
After: The cache answers a lookup in under 2 ms.

Before: This is a clean, elegant solution.
After: This solution removes the second lookup.
```

The rule covers the model's praise of its own work too. Say what the work does, and
let the reader judge it.

## spelling

Spelling is British, with the -ise forms. `SKILL.md` states the rule, and the
checker finds American forms as `spelling` at the hard level. Code, identifiers and
quoted text keep their own spelling. A user-interface string that the text quotes
in American spelling stays as it is.

## Coverage

| Identifier | Rules |
|---|---|
| `plain-word` | 1.1 |
| `one-meaning` | 1.3, 9.2 |
| `word-class` | 1.2, 1.7, 1.13, 9.2 |
| `domain-terms` | 1.1, 1.5, 1.6, 1.8 |
| `no-coined-words` | 1.4 |
| `short-terms` | 1.9 |
| `no-slang` | 1.10 |
| `synonym-rotation` | 1.11 |
| `domain-verbs` | 1.12 |
| `spelling` | 1.14 |

`marketing-adjective` comes from the checker and has no rule number of its own.
