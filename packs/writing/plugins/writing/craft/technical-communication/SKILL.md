---
name: technical-communication
description: Controlled-English writing rules, modelled on ASD-STE100, for all natural language a model writes. Use when the always-on technical-communication instruction is in context, or when the human asks for controlled English.
harnesses: [any]
---

# technical-communication

This skill keeps everything a model writes in natural language short, plain and
exact. It is modelled on ASD-STE100. It claims no conformance with that standard,
and it ships no dictionary. Its rules are in this skill's own words.

The rules aim at one result: the reader understands the text on the first read,
and cannot read it in two ways. A text is complete when it has one meaning. It need
not be the shortest text possible.

## Scope

The skill governs these texts:

- Replies to the human.
- Documents for people: READMEs, docs, ADRs, specs, changelogs, and the text of
  commit messages, pull requests and issues.
- Code comments.
- Text for other models: subagent prompts, skill files, instruction files such as
  `AGENTS.md`, and task files.

These are outside the skill, because they are not prose:

- Code, commands and tool output.
- Quoted material.
- File names and identifiers.
- Markdown formatting. House style governs it, and the rules govern the words
  inside it.
- The private reasoning of the model.

## Precedence

When rules conflict, this order decides. The highest item wins.

1. **The human's explicit request** for one output. It wins for that output only.
2. **Meaning.** No rule may cost information, a hedge or a caveat, or add a fact. First
   look for wording that keeps both the rule and the meaning. Depart from the rule
   only when no such wording exists.
3. **This skill's rules** for words and sentences.
4. **House style:** the project style guide, document templates, commit
   conventions and the output style in use. It governs what the rules leave alone.
   Examples are formatting, what to explain, and which units and abbreviations to use.

A hedge is content. The words "may", "might" and "could" stay when the claim is
uncertain.

## Spelling

Spelling is British, with the -ise forms: "organise", "behaviour", "colour",
"centre", "cancelled", "modelling". Code, identifiers and quoted text keep their
own spelling. The checker reports an American form as `spelling`.

## Text types and limits

Each sentence is an instruction or a description.

| Type | What it is | Sentence limit |
|---|---|---|
| Instruction | A command to the reader or to another model | 20 words |
| Description | Everything else, including a note in a procedure | 25 words |

- A paragraph has 6 sentences at most. A list item, a table cell and a heading
  are not paragraphs.
- The checker reads a commit message as a description. Its subject is a short label.
- In `auto` mode, a sentence is an instruction in two cases: it is in an ordered
  list, or it starts with a command verb. The checker skips a leading "if", "when",
  "before", "after" or "once" clause first.

## Core rules

Each rule has one identifier. The checker uses the same identifier when it enforces
the rule. A hard finding fails the check. An advisory finding never fails it.

| Identifier | Level | The rule |
|---|---|---|
| `long-sentence` | hard | An instruction has 20 words at most. A description has 25 |
| `long-paragraph` | hard | A paragraph has 6 sentences at most |
| `one-idea-per-sentence` | none | A sentence has one subject and one point |
| `one-action-per-step` | none | A step names one action. Two actions share a step only when they happen at once |
| `imperative-steps` | none | A step is a command, not a passive or a "should" |
| `condition-first` | none | The condition comes first, then a comma, then the command |
| `full-words` | none | Write whole sentences, and spell out "do not" and "cannot" |
| `passive-voice` | advisory | Name the actor. Use the passive when the actor is unknown or not important |
| `present-perfect` | advisory | Use the simple past or present, unless the link to now is the point |
| `nominalisation` | hard | Use the verb. Write "analyse", not "perform an analysis" |
| `phrasal-verb` | hard | Use one plain verb instead of a soft phrasal verb. Write "start", not "spin up" |
| `marketing-adjective` | hard | Delete the sales word, or give the measurement that supports it |
| `synonym-rotation` | hard | One word for one action, every time |
| `semicolon` | hard | No semicolon. Split the sentence |
| `dangling-conjunction` | hard | No list item ends in "and" or "or" |
| `spelling` | hard | British spelling |
| `marker` | hard | A marker has a valid verb, rules and reason |

The checker finds `passive-voice` and `present-perfect` by word shape, not by
grammar, so it gives false alarms. Judge each by reading.

## Rule index

| Rule | Reference file |
|---|---|
| `plain-word`, `one-meaning`, `word-class`, `domain-terms`, `no-coined-words`, `short-terms`, `no-slang`, `synonym-rotation`, `domain-verbs`, `marketing-adjective`, `spelling` | `references/words.md` |
| `noun-cluster`, `short-form-after-full`, `hyphen-unit` | `references/noun-clusters.md` |
| `simple-tenses`, `present-perfect`, `participle-adjective`, `ing-forms`, `passive-voice`, `verb-for-action`, `nominalisation`, `phrasal-verb` | `references/verbs.md` |
| `articles`, `full-words`, `say-what-happens`, `one-idea-per-sentence`, `long-sentence`, `connect-sentences`, `vertical-list`, `dangling-conjunction` | `references/sentences.md` |
| `imperative-steps`, `one-action-per-step`, `notes-inform-only`, `condition-first`, `long-sentence` | `references/procedures.md` |
| `paragraph-topic`, `long-paragraph`, `gradual-order`, `repeat-key-terms`, `long-sentence` | `references/descriptions.md` |
| `risk-label`, `command-first`, `state-the-risk` | `references/safety.md` |
| `parentheses`, `hyphen-compound`, `semicolon`, `word-count` | `references/punctuation.md` |
| `rewrite-not-swap`, `literal-sense`, `consistent-wording`, `keep-that`, `with-one-meaning`, `clear-pronoun`, `false-friend`, `no-latin-abbreviation`, `neutral-people`, `possessive-care` | `references/practices.md` |

Read a reference file when the task needs it. Read `procedures.md` for steps,
`descriptions.md` for explanations, and `sentences.md` for lists, articles and
sentence form. Read `words.md` to choose a word. Read `noun-clusters.md` for a long
run of nouns, and `verbs.md` for tenses and voice. The rules in a file cover one
part of the standard each. Each file names that part.

Read `safety.md` before you write a warning. Read it also before you write a text
that proposes or reports a step that deletes, overwrites or publishes. Read
`punctuation.md` for hyphens, parentheses and word counts. Read `practices.md` when a
word swap fails or a sentence has two readings.

## Markers

A marker is an HTML comment. Rendered Markdown hides it. Its form is:

```text
<!-- tc: VERB [RULES] because REASON -->
```

| Verb | Use | Scope |
|---|---|---|
| `exempt` | The human asked for this output in another form | The checker skips a document. The log scores a reply and marks it exempt |
| `depart RULE[,RULE]` | No wording keeps both the rule and the meaning | The block that holds the marker. A marker alone on its line applies to the next block |
| `off` | Text that is not yours to change, such as a quotation or released history | To the next `on`, or to the end of the file |
| `pending` | Text of a large document that the conversion has not reached | To the next `on`, or to the end of the file |
| `on` | Ends the open `off` or `pending` region | It takes no rules and no reason |

A block is a paragraph, a list item, a table row or a heading. A departure accepts
only the findings of the rules it names. A marker with no reason, an unknown verb or
rule, or no region to close is a hard `marker` finding, and it does not apply. In a
reply, only `exempt` has an effect.

## The checker

The checker is `scripts/ste-lint.py` in this skill's directory. It needs Python 3.9
or later and no other package. It checks the rules that need no dictionary. It cannot
judge word choice, so read your own text too.

```bash
python3 scripts/ste-lint.py FILE...
python3 scripts/ste-lint.py --text-type commit < message.txt
python3 scripts/ste-lint.py --json FILE
python3 scripts/ste-lint.py --markers FILE
```

Each finding is `FILE:LINE:COL RULE: MESSAGE`, and a summary follows. Exit 0 means
no unaccepted hard finding. Exit 1 means findings. Exit 2 means a usage error or a
file that is not UTF-8.

Run the checker on these texts before you send them:

- A commit message: use `--text-type commit`.
- The text of a pull request or an issue. Pass it on standard input.
- A long code comment. Pass it on standard input. No tool scans comments.
- A prose file (`.md`, `.markdown`, `.txt`) that a shell command wrote.

After each write of a prose file, a gate may add the findings to the tool result.
Fix each finding, or mark a justified departure, before you finish. A gate sees only
the harness's own file tools. If no gate message came in this session, run
the checker yourself on each prose file that you write.

The checker does not read a reply, and no gate holds a reply back. The log scores
each reply after the model sends it. Write each reply by the rules as you compose it.

## Existing documents

When you edit an existing prose document, the document must follow the rules first.
Convert it as a separate change, and then make the edit. Never convert these:

- Licences and legal notices.
- Vendored or upstream text.
- Quotations and released history, such as past changelog entries.
- Generated files.
- Historical records, such as finished task files.

Put a quotation or released history inside a checked file in an `off` region.

1. **Check the document.** Run the checker on the whole document. If it is clean, or
   a departure accepts each finding, make the edit and stop here.
2. **Convert it.** Rewrite the whole document by the rules. Meaning wins over a
   rule. When no wording keeps both, mark a departure.
3. **Convert a large document in part.** A document is too large for one session
   in two cases. It has more than 3,000 prose words. Or it has more than 60 hard
   findings. Convert the sections that the edit touches. Put each other section in a `pending` region. A
   later edit converts more.
4. **Land the conversion first, as its own change.** Under jj, run
   `jj split -m "<summary>: convert <document> to controlled English" <document>`. This puts
   the converted document into its own parent change. Other work stays in the working
   copy on top. Under git, run `git commit -m "…" -- <document>`. In a grove session,
   the conversion is a preparatory commit before the commit of the task.
5. **Edit.** Make the requested edit in the next change.

Some sessions may not create commits. In such a session, convert the document and
stop. Ask the human to land the conversion, or to allow it, before you make the
edit.

## The switch

One file turns off every always-on layer: `technical-communication/off` in
`$XDG_STATE_HOME`, by default `~/.local/state`. When the file exists, the layers do
nothing. When it does not exist, they are on. The content of the file does not matter.

- To turn the layers off, create the file. To turn them on, delete it. The human can
  ask you to do either.
- The layers act at their next event. Text that a running session already holds stays
  until that session ends. When the human turns the switch off, stop applying the
  rules in that session too. The request of the human comes first.
- The skill stays installed. Its description loads it only when the always-on
  instruction is in context, or when the human asks for controlled English.
- `TECHNICAL_COMMUNICATION_STATE_DIR` moves the whole state directory for a test run.
  It is a location, not a second switch.
