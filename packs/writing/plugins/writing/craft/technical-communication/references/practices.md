# Practices

This file restates section 9 of Part 1: four rules (9.1 to 9.4) and eight
recommendations. These are habits of a careful writer. They help most when a rule
alone does not give the right sentence. Two of the four rules live in other files, so
each rule has one owner:

- Rule 9.2 is split. `one-meaning` and `word-class` in `words.md` cover the sense
  and the job of a word. `literal-sense` below covers the rest of it.
- Rule 9.3 is `phrasal-verb` in `verbs.md`.

The last eight identifiers are advice, not rules. Meaning comes first, so a
recommendation yields when it would cost meaning (see the precedence order in
`SKILL.md`). The checker enforces none of this file. Read your own text for each
identifier.

| Identifier | Level | Idea |
|---|---|---|
| `rewrite-not-swap` | none | When a word swap fails, rebuild the sentence |
| `literal-sense` | none | A word of position or motion keeps its physical sense |
| `consistent-wording` | none | The same job gets the same words every time |
| `clear-pronoun` | none | A pronoun, or "this", points to one thing |
| `keep-that` | none | Keep "that" after a verb that introduces a clause |
| `with-one-meaning` | none | Reread every "with", and rewrite a sentence that has two readings |
| `no-latin-abbreviation` | none | Write the English words, not "e.g." or "etc." |
| `false-friend` | none | Check the English sense of a word that looks foreign |
| `possessive-care` | none | Use the possessive only when it is clear |
| `neutral-people` | none | No gendered pronoun for a person whose gender is unknown |

## rewrite-not-swap

A rule often says that a word is wrong. Do not replace the word and move on. First
test the result with three questions:

- Does the sentence still mean the same?
- Does the new word do the same job?
- Does the sentence read as good English?

When one answer is no, a swap is not enough. Rebuild the sentence.

A swap fails in three ways:

1. **The nearest plain word changes the meaning.**
2. **No plain word exists.** The idea needs a different construction.
3. **The new word needs a different structure.** A verb needs a subject, and an
   adjective does not.

```text
Before: The release is blocked on review.
Bad swap: The release is stopped on review.
After: The release cannot start until a reviewer approves the change.

Before: The test barely passes.
Bad swap: The test almost passes.
After: The test passes. It finishes 2 ms before the time limit.

Before: The script is brittle.
After: The script fails when a file name has a space.
```

To rebuild, ask what the reader must know or do, and write that. Start with the fact
that you left out, if one is missing. Read the code, or ask the human, because a vague
word does not fill the gap. Then cut the detail that the reader does not need, and
split what remains into short sentences.

Reread the neighbouring sentences too. A new wording can break a term that a later
sentence still uses.

## literal-sense

A word of position, motion, sight or the body keeps its physical sense. Do not stretch
it to say something abstract. The reader meets the picture first, and then has to
work out what you mean. This also costs the reader who knows only the plain sense.
For a state, a limit or a result, name the state, the limit or the result.

```text
Before: The error lives in the parser.
After: The error is in the parser.

Before: Chase the error through the logs.
After: Find the first line of the error in the logs.

Before: Keep the latency under 200 ms.
After: Make sure that the latency is less than 200 ms.
```

A word that the field defines keeps the sense that the field gives it. "A thread
waits on a lock" is plain. `no-slang` in `words.md` covers idioms and private jargon,
and `one-meaning` covers a word with two senses in one text. This rule covers the
ordinary word that a writer stretches.

## consistent-wording

When the same job comes up again, use the same words. A reader of a procedure learns
the wording of the first step and then reads the later steps fast. A new wording
makes the reader stop and ask if the job is a new one.

This extends two rules. `synonym-rotation` fixes one word for one action, and
`repeat-key-terms` fixes one word for one thing in adjacent sentences. This rule fixes
the whole sentence. Choose one sentence pattern for each kind of step, and use it
every time the step comes back.

```text
Before:
2. Run the linter on `src/`.
5. Lint `docs/`.
7. Use the linter to check `scripts/`.

After:
2. Run the linter on `src/`.
5. Run the linter on `docs/`.
7. Run the linter on `scripts/`.
```

The rule applies to more than procedures. Sections of a document that do the same job
use the same pattern. A session that repeats an action, such as a report of a test
run, reports it in the same form each time. A template of the project, such as a
commit or ADR template, fixes the wording where it applies (see the precedence order
in `SKILL.md`).

When two steps differ in meaning, write them differently, so the reader sees the
difference.

## clear-pronoun

A pronoun stands for a noun that came before. "This", "it", "those" and "they" are
common ones. When a pronoun can stand for more than one noun, the sentence has two
readings. Name the noun again.

1. Find the pronoun.
2. Find the nearest noun that fits it.
3. Look for a second noun that fits. If one exists, write the noun in place of the
   pronoun.

```text
Before: The importer reads the queue and the cache, and then it logs the delay.
After: The importer reads the queue and the cache, and then the importer logs the delay.
```

The word "this" needs more care. Used alone, it can point to a thing, an action or a
whole fact. Put a noun after it: "this restore", "this change", "this limit". When no
noun is clear, rewrite the sentence.

```text
Before: The deploy retries after a failure and restores the old config. This can hide the real error.
After: The deploy retries after a failure and restores the old config. This restore can hide the real error.
```

A reader of a reply or of a prompt may lack the context that the writer has. A pronoun
that points to something only the conversation holds has no target. In text for another
model, and in a long reply, name the thing. Never point a pronoun at another pronoun.
`neutral-people` covers the pronouns for people.

## keep-that

After a verb that introduces a statement, keep the word "that". Verbs such as "check",
"confirm", "show", "note", "report" and "expect" do this. The word "that" marks where
the clause starts. Without it, the reader may take the noun after the verb as its
object. It also helps a reader who works through a translation, because many
languages cannot leave it out.

```text
Before: Check the server answers on port 8080.
After: Check that the server answers on port 8080.

Before: Note the config reloads without a restart.
After: Note that the config reloads without a restart.
```

## with-one-meaning

The word "with" can attach to a noun, or to the action. In "Restart the worker with
the saved queue", the worker may hold the queue, or the queue may be what the restart
loads. Reread each sentence that has "with". When the sentence has two readings,
rewrite it so that each part has one place to attach.

```text
Before: Restart the worker with the saved queue.
After: Restart the worker that holds the saved queue.
After: Restart the worker. Load the saved queue at start.
```

When "with" gives a condition, put the condition first (see `condition-first` in
`procedures.md`). When "with" gives a means, the action stays the verb of the
sentence. A weak verb such as "use" or "perform" must not carry the action (see
`verb-for-action` in `verbs.md`).

## no-latin-abbreviation

Do not use a Latin abbreviation in prose: "e.g.", "i.e.", "etc.", "cf.", "vs.",
"viz." or "N.B.". A reader may not know it, and a reader may mix up "e.g." and
"i.e.". Write the English: "for example", "that is", "and so on", "compare",
"against", "namely" or "note".

```text
Before: The linter reports style faults, e.g. long lines.
After: The linter reports style faults, for example long lines.

Before: The cache is per-user, i.e. not shared.
After: The cache belongs to one user. Other users cannot read it.
```

A list that ends with "etc." hides the last items. Choose one of two fixes. Give the
whole list when the reader needs every item. Otherwise write "for example" before the
items and leave the "etc." out. Code and quoted text keep their own form. The checker
does not enforce this rule.

## false-friend

A false friend is a word that looks like a word in the reader's first language and
means something else in English. A reader whose first language is not English can be
caught by it, and a model has many such readers. When you choose a word that resembles
a word from another language, check its English sense. When you are not sure,
choose a plain word.

```text
Before: Use the actual version of the schema.
After: Use the current version of the schema.
```

In English, "actual" means "real" and does not mean "current". The Spanish word
"actual" means "current". Likewise "eventually" means "after some time", and the French
"éventuellement" means "possibly". "Sensible" means "reasonable", and the French
"sensible" means "sensitive".

```text
Before: The job eventually fails when the disk is full.
After: The job may fail when the disk is full.
```

## possessive-care

The possessive ("the maintainer's review", "the user's files") is standard English,
and it is clear in short phrases. A reader whose first language marks ownership in a
different way may find it hard. When the sentence is in doubt, write "of" or a plain
phrase.

- A chain of two possessives is hard to read. Write it with "of" or "in".
- A long or technical owner reads better with "of" or "in".
- Write "its" with no apostrophe for a thing. "It's" is a contraction that `full-words`
  forbids.

```text
Before: The config's default's value is zero.
After: The default value in the config is zero.
```

## neutral-people

When the text mentions a person whose gender is unknown or does not matter, use no
gendered pronoun. Try these four ways in order:

1. Speak to the person. Write "you".
2. Name the role or the name: "the reviewer", "the maintainer".
3. Use the singular "they".
4. Rewrite in the plural or in the imperative.

When a person tells you their pronouns, use those. Never guess a person's gender
from a name. When "they" could also point to a plural noun nearby, name the role again
(see `clear-pronoun`).

```text
Before: Ask the reviewer to approve the change after he reads the diff.
After: Ask the reviewer to approve the change after they read the diff.

Before: The maintainer decides, and she owns the release.
After: The maintainer decides and owns the release.
```

Use neutral role words, as well as neutral pronouns. Write "person-hours" and not
"man-hours". Write "everyone" or "the team" and not "guys". The same care covers any
group. Describe a person by what matters for the task, and not by a trait that does not.

## Coverage

| Identifier | Rules |
|---|---|
| `rewrite-not-swap` | 9.1 |
| `literal-sense` | 9.2 |
| `consistent-wording` | 9.4 |
| `keep-that` | GR-1 |
| `with-one-meaning` | GR-2 |
| `clear-pronoun` | GR-3, GR-4 |
| `false-friend` | GR-5 |
| `no-latin-abbreviation` | GR-6 |
| `neutral-people` | GR-7 |
| `possessive-care` | GR-8 |

Rule 9.2 is also covered by `one-meaning` and `word-class` in `words.md`. Rule 9.3 is
covered by `phrasal-verb` in `verbs.md`.
