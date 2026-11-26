# Verbs

This file restates section 3 of Part 1 (rules 3.1 to 3.7). A verb tells the reader
who does what. The rules keep each verb in a form that has few readings, and they
name the actor.

| Identifier | Level | Idea |
|---|---|---|
| `simple-tenses` | none | Use the simple forms, with one auxiliary at most |
| `present-perfect` | advisory | Use the simple past or present, unless the link to now is the point |
| `participle-adjective` | none | A past participle can show the condition of a thing |
| `ing-forms` | none | An "-ing" word is a noun or a modifier, not a verb or a clause |
| `passive-voice` | advisory | Name the actor. Use the passive only when the actor is unknown or not important |
| `verb-for-action` | none | A verb names an action. A noun does not |
| `nominalisation` | hard | Use the verb. Write "analyse", not "perform an analysis" |
| `phrasal-verb` | hard | Use one plain verb instead of a soft phrasal verb |

The checker finds `passive-voice` and `present-perfect` by word shape, so it gives
false alarms. It never fails a check on those two. Judge each by reading.

## simple-tenses

Use these forms:

- The simple present for a fact, a behaviour or a state: "The parser rejects empty
  files."
- The simple past for an event: "The importer failed at step 3."
- "Will" with the base form for the future: "The next run will retry the job."
- The base form for a command, and after "to".
- The past participle as an adjective (see `participle-adjective`).

Do not use these forms:

- The progressive, such as "is running" and "was waiting".
- The past perfect, such as "had finished".
- A chain of auxiliaries, such as "would have been", "is going to be" and "is to be run".

Verbs keep their ordinary forms, and irregular verbs keep their irregular forms.

"Can", "must" and "will" take the base form of the verb. A second auxiliary starts
a chain. "Can be run" is a passive in disguise, so write "you can run" (see
`passive-voice`).

```text
Before: The job is running, so the lock has been held since noon.
After: The job runs now. It took the lock at noon, and it still holds the lock.

Before: The scheduler is going to retry the task when the queue is drained.
After: The scheduler will retry the task when the queue is empty.

Before: The config is to be reloaded after the file has been saved.
After: Reload the config after you save the file.
```

A hedge is not a tense fault. "May", "might" and "could" stay when the claim is
uncertain (see the precedence order in `SKILL.md`).

## present-perfect

Use the simple past for a finished event, and the simple present for a state that
holds now. "The build has failed" becomes "The build failed at 10:02" or "The build
fails now".

The perfect form stays in one case. The link to now is the point of the sentence,
and a simple form would lose it. "The cache has held no entries since the restart"
says that the cache is still empty.

The checker flags "has", "have" and "had" with a participle, at the advisory level.
It skips a hedge such as "may have failed".

## participle-adjective

A past participle can work as an adjective. It can stand before a noun ("the
disabled flag", "a failed job"), or after a linking verb ("is", "remains",
"looks"). It names the condition of a thing and does not name an actor.

The test is whether the sentence describes a state or an event. A state holds now
and has no actor in view. An event happened, and someone or something caused it.

- "The port is closed" gives a state.
- "The port is closed by the proxy at night" gives an event. It is a passive, so name
  the actor first (see `passive-voice`).

```text
Condition: The branch is merged, so delete it.
Passive: The branch was merged by the release bot.
Active: The release bot merged the branch.
```

Use a participle that readers know as an adjective. A made-up one is a coined word
(see `no-coined-words` in `words.md`). The checker flags "is closed" by word shape
even when it is a condition. Accept that false alarm when you judge the sentence to
be a condition.

## ing-forms

A word that ends in "-ing" has several jobs. It can name a thing, describe a noun, form
part of a verb or start a long clause. The reader must decide which, and the
sentences grow long. So do not use an "-ing" word as a verb, and do not start a
clause with one. Use the simple tense, or start a
second sentence.

```text
Before: When running the tests, make sure that the cache is warm.
After: When you run the tests, make sure that the cache is warm.

Before: The script, reading the whole file and building the index before it returns, uses much memory.
After: The script reads the whole file and builds the index. Then it returns. It uses much memory.
```

An "-ing" word keeps two jobs. It can be a domain noun, as in a heading
"Caching and invalidation". It can be a modifier in a domain noun, as in "logging
library", "testing framework" or "sorting key". There it names what the thing does
or what it is for. Some words end in "-ing" and are not verb forms, such as
"setting", "string" and "during". They are plain words and stay.

After a preposition, use a clause. Write "Before you run the tests", and not
"before running the tests".

## passive-voice

In an active sentence, the subject does the action. In a passive sentence, the
subject receives the action, and the actor comes after "by" or is missing. Prefer
the active voice. It says who does what. The passive hides the actor, and the reader
cannot tell which part of a system acts or what to do. Models write the passive
often, as in "The file is parsed" and "Errors are logged".

The test is to ask who or what does the action. When the text can answer, rewrite
the sentence. Three rewrites cover most cases:

1. **Use the imperative in a procedure.** "Clear the cache first."
2. **Use "you" or "I".** When the reader is the actor, write "you". In a report of
   your own work, the model is the actor, so write "I".
3. **Move the actor to the front.** "A reviewer approves the change." A frame such as
   "is applied by X to do Y" then shrinks to "X does Y".

```text
Before: The migration is run by the deploy job.
After: The deploy job runs the migration.

Before: The lock file must be deleted before the restart.
After: Delete the lock file before you restart.

Before: The retry limit can be changed in the settings file.
After: You can change the retry limit in the settings file.

Before: The fix was tested and pushed.
After: I tested the fix and pushed it.
```

When a passive has an auxiliary, such as "must be adjusted", decide first what the
sentence is. In a procedure, it is an instruction, so use the imperative. In a
description, name the actor and use "must" or "can" with it.

The passive stays when the actor is unknown or does not matter. "The commit was
amended after the review" is right when nothing shows who did it. Never invent an
actor only to avoid a passive, because a wrong actor changes the meaning. Meaning
comes before this rule. When you find the real actor, write it.

## verb-for-action

Name an action with a verb. Do not use a noun made from the verb with a weak verb
beside it. "Make a decision" becomes "decide". "Make use of" becomes "use".
"Provide a description of" becomes "describe". The noun form adds words and hides
the actor.

A noun for an action as the subject has the same fault.
Write "You must delete the lock", not "The deletion of the lock is required".

```text
Before: The reviewer made a decision to reject the change.
After: The reviewer decided to reject the change.

Before: Failure of the health check causes a restart.
After: The service restarts when the health check fails.
```

When a word names the thing and not the action, the noun is correct. "A commit" is
a thing. A verb that the field does not use is not a better choice than the noun
(see `domain-verbs` in `words.md`).

## nominalisation

The checker finds one pattern of this fault: "perform", "conduct" or "carry out",
then "a", "an" or "the", then a noun that ends in "-tion", "-sion", "-ment", "-ance",
"-ence" or "-ysis". It reports `nominalisation` at the hard level. Write the verb:
"analyse the logs", not "perform an analysis of the logs". Read for the other weak
verbs yourself, because the checker does not know them.

## phrasal-verb

A phrasal verb is a verb with a small word after it, and the whole has a meaning
that the parts do not give: "spin up", "reach out", "dive into", "circle back",
"touch base" and "kick off". A reader who learned English from books may not know
it. The small word also adds a second meaning. Use one plain verb: "start",
"contact", "read", "return to" or "begin".

```text
Before: Spin up a worker, then circle back with the numbers.
After: Start a worker. Then report the numbers.
```

The checker finds a short list of soft phrasal verbs and reports `phrasal-verb` at
the hard level. Some phrasal verbs are not soft. A field can name an action with
one, such as "roll back" or "log in", and no single verb is as clear. Such a verb
passes the tests of `domain-verbs` in `words.md`. Replace the others with one
verb when the verb says the same thing.

## Coverage

| Identifier | Rules |
|---|---|
| `simple-tenses` | 3.1, 3.2, 3.4 |
| `present-perfect` | 3.2, 3.4 |
| `participle-adjective` | 3.3 |
| `ing-forms` | 3.5 |
| `passive-voice` | 3.4, 3.6 |
| `verb-for-action` | 3.7 |
| `nominalisation` | 3.7 |

`phrasal-verb` comes from the checker, and it restates rule 9.3 too.
