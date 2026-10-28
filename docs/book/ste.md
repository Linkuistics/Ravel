# STE word decisions for the book

This file holds the word decisions that all chapters of the book use. The
base of the book's prose is the writing rules of ASD-STE100 Simplified
Technical English, Issue 9. Section 2 of [the book's contract](contract.md)
gives the base and its other points. The book does not use the STE
dictionary. The owner made this decision on 2026-10-18: “The skill I am going
to adopt does not use the word list, only the rules.”

Each stage of the book (draft, copy-edit, art and proof) uses these
decisions. If two chapters can put one concept into different words, decide
the words here, one time. A stage that needs a new decision adds it to this
file.

## The check

`task docs:ste` examines the book's text. This is every chapter file in
`docs/book/src/` (a name that matches `[0-9][0-9]-*.md`), `SUMMARY.md`,
`CONTEXT.md`, `README.md` and the text of each figure (its `.layout.json`
record). Thus it examines all eleven chapters. The script finds chapter
files by name, so it does not require a fixed chapter count.
The script is `scripts/ste.ts`. It removes code blocks and code spans first,
then it finds these problems:

- **Long sentences**: a sentence of more than 25 words (Rule 6.3). A step of
  a numbered list has 20 words or fewer (Rule 5.1). The script cannot find an
  instruction in other text. Thus, keep each instruction to 20 words.
- **Long paragraphs**: a paragraph of more than six sentences (Rule 6.6).
- **Semicolons** (Rule 8.1) and **contractions** such as “don't” (Rule 4.2).
- **Normative keywords**: MUST, SHOULD, MAY, SHALL and REQUIRED in capitals.
  The contract does not let the book use them.
- **Words in “-ing”**: a word that ends in “-ing” and is not in the table
  [Words in -ing that the book keeps](#words-in--ing-that-the-book-keeps)
  (Rule 3.5).
- **Words that the book does not use**: each word in the table
  [Words that the book does not use](#words-that-the-book-does-not-use).

An `_Avoid_` line of `CONTEXT.md` names the words that a term replaces. It
does not use those words. Thus, the check does not apply the two word
tables to that line. It applies the other rules.

The script cannot find all problems. Read the text for the other rules. For
example, use the active voice (Rule 3.6) and the simple tenses (Rule 3.2).
Do not use phrasal verbs (Rule 9.3). Give each paragraph one topic (Rule
6.5).

`task docs:ste` gives a report and stops with status 0. Use
`task docs:ste -- --strict FILE…` to make it stop with status 1 when it finds
a problem. A copy-edit that completes a chapter runs the strict form on that
chapter. `task docs:ste:test` shows that the check finds a problem that is
put into a test copy of a file. It also shows that a run with no file
arguments examines a chapter 7 that is put into a test copy of the book.

## The result

`end-session` sends the result to Ravel. Ravel receives the result and keeps
the first result that it receives. Use these words for the concept:

- the verb **send**: “The LLM sends the result with `end-session`.”
- the verb **receive**: “Ravel receives the result through the result
  channel.”
- the verb **keep**: “Ravel keeps the first result. A second `end-session`
  does not replace it.”
- the noun phrase **the sent result** where a noun is necessary.
- the word **unchanged** for what Ravel writes: “Ravel writes the result
  unchanged to standard output. Ravel does not interpret the result.” Do not
  write “the exact bytes of the result”.
- the phrase **return no result to the caller** for the outcomes without a
  result: “Some outcomes return no result to the caller.” Do not write
  “give no result”.

Do not use “publish”, “publication”, “submit” or “submission” for the result
(the owner's decision of 2026-10-17). The owner made the decisions for
“unchanged” and “return no result to the caller” on 2026-10-18, in read 1
(findings R12 and R13). The check does not find their old words yet.

The book on GitHub Pages is a different concept. Write “the book on GitHub
Pages”, not “the published site”. The release procedure puts the book on
GitHub Pages.

## Technical names

The technical names of the book are the terms in `CONTEXT.md`, code
identifiers, commands, file names, paths and product names. Rule 3.5 lets a
word in “-ing” be part of a technical name, for example “working directory”.

The titles below use commands where they name tasks. Rule 3.5 permits
“-ing” forms only in technical names. Chapter 7 uses a noun phrase.
These titles replace the earlier titles:

| Earlier title | Title |
|---|---|
| Part II: Using Ravel | Use Ravel |
| Writing an owner policy, Write an owner policy | Write a configuration script |
| The actors and their words | Who does what in a Ravel session |
| Running sessions | Run sessions |
| How Ravel is built | The parts of Ravel |
| Following a session through the code | Follow a session through the code |
| Part IV: Reusing and developing Ravel | Develop and deliver Ravel |
| Developing Ravel | Develop Ravel |
| Delivering Ravel | Deliver Ravel |

The file names for chapters 1–6 do not change. Chapter 7 uses
`07-the-parts-of-ravel.md`. Its title is a noun phrase, which avoids passive
voice with a known agent (Rule 3.6). A link text or a sentence that gives a
chapter title uses the new title.

The owner found “The actors and their words” unusual on 2026-10-28 (finding
F15 of Read 1). Chapter 2's title now names its subject. The chapter 4 title
follows the configuration words below. Chapter 1's last section is “See what
we tested and what remains unchecked” (finding F14).

## Configuration words

The owner prefers “configuration” to “owner policy” (finding F10 of Read 1,
2026-10-28): “I think the term "owner policy" is inferior to the term
"configuration", which is more common and specific.” The qualifiers in the
table are the choice of draft `rhai-book-use-k88`, not the owner's words. They
keep the executable script apart from the settings.

| Earlier words | Words to use | What the words name |
|---|---|---|
| owner policy, policy (the owner's code) | configuration script | The Rhai file that selects a program or refuses |
| owner settings | settings | The values in `settings.json` |
| the policy and the settings together | configuration | The configuration script and the settings |
| policy worker | Rhai worker | The second copy of the `ravel` executable that evaluates the configuration script |
| policy context | selection context | The JSON data that `select` receives as `context` |
| policy author | owner, or you | The person who writes the configuration script |

Literal names keep the word “policy”: `policy.rhai`, `--policy-env`,
`policyEnv`, `RAVEL_POLICY_RUN`, the `policy_*` diagnostic codes, the CLI help
and quoted diagnostics. A page that shows such a literal says one time that
Ravel's file names, options and codes use “policy” for the configuration
script.

The word “policy” alone never names a security or sandbox rule. A restriction
on a managed process is confinement.

## JSON documents

Name a JSON document by its purpose, for example “the JSON routing notice”,
“the failure document” or “the context document”. Do not put a version label,
such as “version-2 JSON”, on each mention (finding F11 of Read 1). Complete
examples keep their exact `schemaVersion` fields. The chapter 6 entry of each
document explains the values that a reader must write or read.

## Standard streams

In text, write “standard output”, “standard error” and “standard input”, not
stdout, stderr and stdin. Code font keeps an identifier exact, for example
the `stdout` field of a harness-run result and the `policy stdout:` prefix.
A table column of stream names uses the full names too. Chapters 1–3 used
these words first. Copy-edit of Parts I and II applied them to chapters 4–6
on 2026-10-26. The check does not find the short words.

## Signal names

Write INT, TERM, HUP and KILL without the prefix SIG, as the shell `trap`
command and the escalation figure do. Write the other signals with their full
names, for example SIGCHLD and SIGTTOU. A chapter that uses the short names
says this rule one time, before their first use. Literal code identifiers, API
values and diagnostic transcripts keep their exact spellings, such as `SIGINT`.
Chapter 9 made this decision on 2026-10-18. Whole-book proof applied it across
the delivered chapters on 2026-10-19.

## Words that the book does not use

Each row gives a word and the words to use in its place. The check finds
each word in the first column. A form of a word, for example “publishes”,
has its own row.

| Word | Use |
|---|---|
| publish, publishes, published, publishing | send (the result), or “put the book on GitHub Pages” (the book) |
| publication, publications | the sent result, or a sentence with “send” |
| submit, submits, submitted, submitting | send |
| submission, submissions | the sent result |
| front | Ravel CLI |
| admit, admits, admitted, admitting, admission | accept, check |
| settlement | the end of supervision, with the steps that it includes |

## Words in -ing that the book keeps

The check accepts a word in “-ing” only if this table has the word or a
phrase that contains it. Each other word in “-ing” is a verb form. Write the
sentence again without it, for example “while the program runs”, not “a
running program”.

| Word or phrase | Reason |
|---|---|
| string, strings | not a verb form |
| during | not a verb form |
| nothing, anything, something, everything | not a verb form |
| thing, things | not a verb form |
| ceiling | not a verb form |
| meaning | technical noun: the meaning that a caller or a configuration script gives to data |
| setting, settings | technical noun: a value in `settings.json` |
| heading, headings | technical noun: a heading of a page |
| spelling | technical noun: the spelling of a word |
| encoding | technical noun: a text encoding, such as UTF-8 |
| autoloading | technical noun: Bun's automatic load of configuration files |
| missing | adjective: a missing file or a missing field |
| the following | refers to the text that comes next |
| working directory | technical name |
| operating system, operating-system | technical name |
| controlling terminal | technical name |
| enclosing session | technical name |
| routing notice, routing diagnostics, routing diagnostic | technical names of Ravel's output |
| path mapping, module mapping, module mappings | technical names |
| trailing newline, trailing newlines | technical name |
| dangling symlink, dangling symlinks | technical name |
| word splitting | technical name of a shell operation |
| code signing | technical name |
| learning route | technical name of an entry route (contract) |
| writing rules | technical name of Part 1 of the STE specification |
| UML timing | technical name of a UML diagram type, in the title of a timing figure |

## Words for frequent phrases in -ing

These phrases occur in more than one chapter, and the table above does not
keep them. Use the same words in each chapter:

| Phrase | Use |
|---|---|
| ending instructions, ending instruction | the instructions to end the session |
| routing (a noun for the work of the configuration script), routing code, routing rule | The choice of command, the decisions of the configuration script (with refusals and arguments), or a rule in the configuration script. As a verb: “the configuration script selects a command” |
