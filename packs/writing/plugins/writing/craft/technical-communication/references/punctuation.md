# Punctuation

This file restates section 8 of Part 1 (rules 8.1 to 8.7). Punctuation shows how the
parts of a text connect. The skill does not teach general punctuation, because a style
manual does that. It states the few marks that change how a controlled text reads. It
also says how to count the words of a sentence.

| Identifier | Level | Idea |
|---|---|---|
| `parentheses` | none | Parentheses carry four kinds of extra information and no others |
| `hyphen-compound` | none | A hyphen joins words that act as one modifier or one term |
| `semicolon` | hard | No semicolon. Write two sentences |
| `word-count` | none | The counting method that `long-sentence` uses |

## parentheses

Parentheses carry four kinds of extra information, and nothing else:

1. **A gloss or a substitute.** A gloss explains a word. A substitute is an alternative
   that the reader may use instead. Examples: "Use the staging key (the one that
   starts with `stg_`)." "Pass the host name (or the IP address) of the server."
2. **An abbreviation** after the full name (see `short-form-after-full` in
   `noun-clusters.md`).
3. **A label or a pointer.** A label is an identifier or a step number. A pointer is a
   reference to another place. Examples: "(see section 3)", "(line 42 of
   `build.sh`)", "the two failing jobs (3 and 7)", "(1)". In Markdown, a list marker
   numbers a step better.
4. **Singular and plural at once:** "the file(s)". Write "one or more files" when this
   is not clear.

The words in parentheses belong to the sentence, and they count in its limit (see
`word-count`). A parenthesis that needs more than a few words is a sentence. Write it
as one. Do not put one parenthesis inside another. Do not hide a hedge, a joke or a
second point in parentheses, because the reader may skip it.

```text
Before: The job retries (sometimes, when the queue is not full, and only if the owner turned retries on) before it fails.
After: The job retries before it fails. It retries only when the queue is not full and the owner turned retries on.
```

## hyphen-compound

A hyphen joins words that act together as one unit. Without it, the reader may attach
a word to the wrong neighbour. A "case insensitive match" can be a match that is
insensitive, or an insensitive case. "Case-insensitive match" has one reading.
`hyphen-unit` in `noun-clusters.md` covers the hyphen in a noun cluster. These are
the other jobs:

1. **A modifier before a noun.** Join the words that describe the noun:
   "case-insensitive match", "ten-minute timeout", "two-pass build",
   "user-defined function", "4-byte header".
2. **A prefix before a vowel that repeats a vowel or could be misread.** Write
   "re-enter" and "re-examine", so the reader does not see a different word.
3. **A number that you write in words.** Join the two parts: "forty-two".
4. **A verb that the field writes with a hyphen.** The field decides, as in
   "cherry-pick" and "force-push". Do not make a new one (see `no-coined-words`
   in `words.md`).

Do not hyphenate an adverb that ends in "-ly" and the word after it: write "a newly
created file". Do not hyphenate a pair that is clear alone.

Do not use a hyphen as a dash. A dash divides a sentence into two ideas, and each idea
needs its own sentence (see `one-idea-per-sentence`). Never write a hyphen with
spaces around it as a dash.

```text
Before: The retry logic is a best effort - it gives up after three tries.
After: The retry logic is a best effort. It gives up after three tries.
```

## semicolon

A semicolon joins two sentences into one long one, and writers often place it badly.
Replace it with a full stop. When the second part gives a result or a reason, a connector
such as "so" or "because" shows the link (see `connect-sentences` in
`sentences.md`).

```text
Before: The importer skips empty rows; it logs each skipped row at debug level.
After: The importer skips empty rows. It logs each skipped row at debug level.

Before: The cache is cold; the first request is slow.
After: The cache is cold, so the first request is slow.
```

The checker reports `semicolon` at the hard level in any prose, including a heading,
a table cell and a list item. It does not read code spans or quoted text, so a
statement terminator in code is safe. A semicolon never separates the items of a
vertical list (see `vertical-list`).

## word-count

The limits in `long-sentence` need one way of counting. This is the way, and the
checker uses it. Count by hand in the same way.

- A word is a token that has at least one letter or digit. A mark alone is not a word.
- A hyphenated compound is one word: "cache-miss" counts one. This does not allow a
  long chain of hyphens (see `hyphen-unit`).
- A code span counts nothing, and so does text in double quotation marks. Both are
  outside the skill. "Run `jj split` now" has 3 words.
- A number and its unit are two words: "10 ms" counts 2. House style chooses how to
  write a unit. The count is the same for "10 ms" and "10 milliseconds".
- Words in parentheses count in their sentence.
- A list lead-in ends at its colon, as if the colon were a full stop. Each item is its
  own sentence and has its own count. A table cell is also its own count.

```text
Check these things before you merge the change:     (8 words)
- The tests pass.                                   (3 words)
- The build is green.                               (4 words)
```

**What the count means for a reply or a document.** The limits apply to each sentence.
A reply has no word limit, and a document has none. A text may be long and still
follow every rule. The rules ask for short sentences, not a short text, and meaning
comes before any rule (see the precedence order in `SKILL.md`). Never delete a
needed fact to meet a count.

The reply log applies the same counting to a whole reply. Its `words` field is the
number of prose words. The field `per_100` divides the hard findings by that number,
so a short reply and a long reply are comparable. Code and quoted text add no words.
A reply of mostly code has few words, so one finding gives a high `per_100`. Read
`per_100` beside `words`.

## Coverage

| Identifier | Rules |
|---|---|
| `semicolon` | 8.1 |
| `hyphen-compound` | 8.2 |
| `parentheses` | 8.3 |
| `word-count` | 8.4, 8.5, 8.6, 8.7 |
