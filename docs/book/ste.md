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
record). Thus it examines chapters 7–12 when they exist. A chapter that does
not exist, such as chapter 10 before its text is written, is not an error.
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

The chapter and part titles are commands, because Rule 3.5 permits “-ing”
forms only in technical names. These titles replace the earlier titles:

| Earlier title | Title |
|---|---|
| Part II: Using Ravel | Use Ravel |
| Writing an owner policy | Write an owner policy |
| Running sessions | Run sessions |
| Following a session through the code | Follow a session through the code |
| Part IV: Reusing and developing Ravel | Reuse and develop Ravel |
| Using Ravel's components in your project | Use Ravel's components in your project |
| Developing Ravel | Develop Ravel |
| Delivering Ravel | Deliver Ravel |

The chapter file names do not change. Thus, links to the chapters do not
change. A link text or a sentence that gives a chapter title uses the new
title.

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
| meaning | technical noun: the meaning that a caller or an owner policy gives to data |
| setting, settings | technical noun: an owner setting |
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

## Words for frequent phrases in -ing

These phrases occur in more than one chapter, and the table above does not
keep them. Use the same words in each chapter:

| Phrase | Use |
|---|---|
| ending instructions, ending instruction | the instructions to end the session |
| routing (a noun for the work of the owner policy), routing code, routing rule | the choice of command, the policy decisions (when refusals and arguments are part of it), the policy code, a rule in the owner policy, or a verb: “the policy selects a command” |
