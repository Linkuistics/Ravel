# Safety

This file restates section 7 of Part 1 (rules 7.1 to 7.3). Safety text warns the
reader that a step can cause harm. A model's text seldom risks injury. The usual harm
is lost data, a broken system, a leaked secret or a bill. The rules fit these cases.

Safety text appears in four places:

- A reply that proposes a destructive command or a step that cannot be undone.
- A step in a runbook, a README or an ADR that deletes, overwrites, migrates or
  publishes.
- A prompt for another model that lets it run such steps.
- A report of a step that the model already took.

This file governs the words of a warning. It does not decide whether the model may
run the step, or whether it asks first. The human's instructions and the permissions
of the harness decide that. A warning in a reply is never a substitute for asking.

| Identifier | Level | Idea |
|---|---|---|
| `risk-label` | none | A label word shows the level of risk before anything else |
| `command-first` | none | The first words of a warning are the order or the condition |
| `state-the-risk` | none | The warning says what goes wrong, and whether it can be undone |

Every sentence of safety text keeps to the instruction limit of 20 words. The checker
reads a sentence that starts with a label as a description, and a description has 25
words. Count the words of a warning yourself.

## risk-label

A label word tells the reader how much care the step needs. Use two levels:

- **Warning.** The step can cause harm that cannot be undone, or harm to people, or
  harm to things that belong to other people. Examples are a deletion with no backup,
  a published secret, a change to a live system and a payment.
- **Caution.** The step can cause damage that someone can repair. Examples are a
  broken local build, a lost hour and a reset of local state.

Decide the level with three questions. What is the worst result? Can the reader undo
it? Who carries the cost? A step that carries both levels gets "Warning".

Put the label at the start, before the instruction. A reader who sees the label
first slows down in time. The project style or a template may name other levels.
Then use its words, and keep the other two rules of this file.

```text
Before: Be careful with this command.
After: Warning: Do not run `git reset --hard` before you save your edits. The command deletes every change that you did not commit, and you cannot get these changes back.
```

Do not label a step that is safe or easy to undo. A label on every step teaches the
reader to skip every label. The text of a label is a matter of house style. Bold
type, a quotation block and capital letters belong to it, and this skill does not
regulate them.

## command-first

Start the warning with the action that prevents the harm. Write a prohibition as
"Do not" and the verb. Write a requirement as a command. The reader acts on the
first words, so the first words are the instruction.

```text
Before: The production database holds the data of every customer, so be aware that a manual change is dangerous.
After: Warning: Do not edit rows in the production database by hand. Use the migration tool.
```

Sometimes a condition decides whether the warning applies. Put the condition at the
front, and end it with a comma (see `condition-first` in `procedures.md`).

```text
Caution: Before you rebuild the index, stop the writer. A running writer corrupts the new index.
```

Background does not come first. A sentence of context before the command delays the
instruction, and the reader may stop before the command. Give a safe action in a
second sentence when one exists. Start it with "Instead".

## state-the-risk

After the command, say what goes wrong. Name the thing that suffers, the result and,
where the reader needs it, whether the harm can be undone. The result tells the
reader how much care to take. It also helps with a case that the warning did not
foresee.

```text
Before: Warning: Do not delete the volume. This may cause problems.
After: Warning: Do not delete the volume. It holds the only copy of the build logs, and a deleted volume cannot be restored.
```

State the real result and no more. "Can" says that the harm is possible. Use "will"
only when it is certain. A hedge stays in a warning, because meaning comes first
(see the precedence order in `SKILL.md`). Do not scare the reader, and do not
understate. When the harm can be undone, say how:

```text
Caution: Do not overwrite `config.toml` before you copy it. The new file replaces your settings. Restore them from the copy.
```

## In a reply and in a document

**Before the step.** In a reply, put the warning above the command that it governs,
in the same message. In a procedure, put it inside the numbered step, before the
action. A note at the end of a document is too late, and `notes-inform-only` in
`procedures.md` forbids a warning inside a note.

**After the step.** When the model already took a step that cannot be undone, say so
plainly in a description. Give four facts: what the model did, what changed, whether
it can be undone, and how to undo it.

```text
I deleted the `build/` folder. The next build creates it again, so nothing is lost.

I ran the migration on the staging database. The old column is gone. Restore it from the backup of 14:00.
```

**In a prompt for another model.** List each forbidden action as a warning with its
reason. The other model has no context and cannot guess what the damage is.

```text
Warning: Do not delete files outside `src/`. A deleted file in another folder cannot be restored, because that folder is not in version control.
```

## Coverage

| Identifier | Rules |
|---|---|
| `risk-label` | 7.1 |
| `command-first` | 7.2 |
| `state-the-risk` | 7.3 |
