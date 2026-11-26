# Procedures

This file restates section 5 of Part 1 (rules 5.1 to 5.5). A procedure tells the
reader what to do, one step at a time. These are procedures:

- Setup guides, runbooks and checklists.
- Steps in a reply that the human will follow.
- A prompt that tells another model what to do, when it lists actions.
- A commit or pull request section that tells a reviewer how to test.

The rules for any sentence (`sentences.md`) also apply here.

| Identifier | Level | Idea |
|---|---|---|
| `imperative-steps` | none | A step is a command |
| `one-action-per-step` | none | A step names one action |
| `notes-inform-only` | none | A note gives background and no orders |
| `condition-first` | none | The condition comes before the command |
| `long-sentence` | hard | An instruction has 20 words at most |

## imperative-steps

Write each instruction as a command: "Restart the worker." Any other form leaves the
reader a question that the text should answer.

| Form | What the reader cannot tell |
|---|---|
| "The worker can be restarted" | Whether the step needs doing |
| "You might want to restart the worker" | Whether the step is optional |
| "The worker should be restarted" | Who restarts it |
| "The worker was restarted" | Whether the work is already done |

```text
Before: The configuration file can be reloaded with the admin tool.
After: Reload the configuration file with the admin tool.

Before: You will need to clear the cache first.
After: Clear the cache first.
```

A command needs no "must" in front of it. Keep "must" for a limit that a result has to
meet, such as "The build must finish in 10 minutes", and for safety text.

The same holds for a prompt to another model. Write "Read the failing test, then list
its inputs", not "You might want to read the failing test".

## one-action-per-step

A sentence in a procedure gives one instruction. Use as many steps as the task
needs. Number or letter the steps, so the order is clear.

```text
Before: Stop the worker and delete its lock file.
After:
1. Stop the worker.
2. Delete the lock file of the worker.
```

Two actions can share a sentence only when they happen at the same time:

```text
Start the stopwatch and run the query.
```

A step can also have a second sentence, when the result of the action follows at once.
This sentence can give a limit, or it can explain the step.

```text
Export the report. The file must be under 5 MB.
```

Test a step like this: if the reader could do the two parts at different times, they
are two steps.

## notes-inform-only

A note is background for the reader. It is descriptive text, so the rules of
`descriptions.md` apply to it. A note never carries any of these:

- An order. An imperative verb turns the note into a step.
- A requirement, a limit or a result that the step must reach. Put these in the step.
- A warning about damage or loss. Write it as safety text.

```text
Before:
3. Run the benchmark.
Note: The token must not be expired.
After:
3. Run the benchmark with a token that has not expired.

Before: Note: Check the free disk space first.
After:
2. Check the free disk space.
3. Start the import.
```

These are correct notes:

```text
Note: The export keeps the original row order.
Note: A second sync does not need a new token.
```

Hide the notes, and follow the steps exactly. Whatever then fails or comes out wrong
is a fact that a note holds and a step needs. Move it into a step, and test again.

Notes belong to procedures. A description needs one only beside a figure or a table
that it cannot explain alone.

## condition-first

A conditional step opens with its condition. A comma ends the condition, and the
command follows. The reader meets the condition before the action and cannot do the
action by mistake.

```text
Before: Delete the branch if the merge is complete.
After: If the merge is complete, delete the branch.

Before: Increase the timeout when the job fails twice.
After: When the job fails twice, increase the timeout.
```

The comma matters, because it marks where the condition ends. A comma in the wrong
place can attach a word to the wrong verb. Read the sentence aloud and check that the
comma sits at the end of the condition.

## long-sentence

An instruction has 20 words at most. Safety text in a procedure follows the same
limit.

When a step is too long, check whether it holds two actions. If it does, split it
into two steps (see `one-action-per-step`). If it holds one action with a long
setting, split the setting from the action. Write the setting first.

```text
Before: Set the timeout in the section of the settings file that the deploy tool reads when it starts, to a value of about 30 seconds.
After:
1. Open the settings file that the deploy tool reads at start.
2. Set the timeout to 30 seconds.
```

A note is a description, so a note has the higher limit of 25 words.
