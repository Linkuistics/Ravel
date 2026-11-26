# Technical communication: publication review

Three residual similarities need triage before publication. Two concern example
scenarios. The third concerns the teaching order of four reference files. The
rewrites removed most of the earlier similarities, but the blanket claim that
an example must share its rule's structure does not settle these cases.

This is the report of `publication-k17`, reviewing `publication-k16` at commit
`30fdcaa70734`. The twelve files under review were unchanged between that
commit and this session's starting tree. All file coordinates below refer to
that commit. A path under `references/` means
`plugins/linkuistics/skills/technical-communication/references/`.

The contract is the spec's
[restatement discipline](specs/technical-communication.md#restatement-discipline)
and [the licence ADR](adr/technical-communication-carries-no-standard-text.md).
The findings concern that contract. They do not establish that publication
would infringe copyright.

## Findings

### F1 — The instrument example still substitutes software into the source's pattern

**File:** `references/practices.md:188`, with the surrounding explanation at
line 183. **PDF page:** 124 (printed page 1-9-10). **Kind:** adapted example.
**Seriousness:** medium; resolve before publication under this contract.

The before/after pair still changes a command to use an instrument for an
operation into a command to perform that operation with the instrument. The
source makes the same paired transformation at the end of its discussion.
The rewrite replaces the instrument and the operation with a script and a
data export. It retains the example's scenario, sentence frame and repair.

The ambiguity example at lines 172 to 180 also keeps the source's preceding
teaching device: a command whose final phrase could attach to the object or
to the operation, followed by alternative interpretations. The new example
has fewer branches and different wording. Taken alone, that grammatical
ambiguity is common. Its placement before the instrument pair makes the
continued dependence on the source's example sequence more apparent.

The idea that the principal action should be the verb can stay. The contract
requires an independently invented example, and does not require this pair.
Replacing the earlier software instrument with another software instrument
has not removed the concern. Rebuild this passage around an independent
software case, rather than another substitution into either source example.

### F2 — The replacement note retains the timed start-up scenario

**File:** `references/procedures.md:136`. **PDF page:** 91 (printed page
1-5-5). **Kind:** close example scenario. **Seriousness:** medium; a narrow
editorial repair, with less confidence than F1.

The first sample of a correct note still describes a system's initial
behaviour over a short time interval. The source's first correct note uses
the same start-up settling scenario. The previous version described the
duration of an initial sync. The new version describes a cache filling
during its first minute. The nouns, duration and wording changed, but the
choice of scenario remains close.

A note's defining property is that it supplies background. No part of that
rule requires a timed start-up example. Use a different background fact
whose explanatory job does not depend on an initial interval. This is a
conservative finding under the explicit ban on adapted scenarios, rather
than a claim that every short temporal statement is distinctive expression.

I do not extend this finding to line 137. A repeated sync that needs no new
token introduces a different dependency from the source's optional removal
action. Nor do I extend it to the new token-validity example at lines 121 to
125, which replaces an outcome limit with a prerequisite.

### F3 — The teaching outlines preserve more than numeric coverage

**Files and lines:** `references/procedures.md:13`,
`references/descriptions.md:15`, `references/sentences.md:7`, and
`references/punctuation.md:8`. **PDF pages:** 87, 95, 77 and 107 respectively
(printed pages 1-5-1, 1-6-1, 1-4-1 and 1-8-1). **Kind:** retained ordered
outline. **Seriousness:** medium; distinguish this from the permitted numeric
coverage before accepting the producer's trade-off.

These are teaching tables with prose summaries, and their body sections
follow the same sequence. They are not merely tables of coverage numbers.
The clearest case is `procedures.md`: length, action count, command form,
condition placement and notes appear in the source's order, both in the
table and at lines 21, 39, 67, 93 and 111.

`descriptions.md` keeps the sequence from gradual information through term
repetition and sentence length to paragraph organisation and paragraph
length. Its merger of two neighbouring paragraph rules leaves that outline
intact. `sentences.md` expands some source topics into several identifiers
but preserves their relative order. `punctuation.md` groups the counting
rules, while retaining the source's preceding sequence of punctuation topics.

The spec requires one reference file per source section. It also permits
rule numbers for coverage. Neither choice requires the teaching prose and
its summary list to follow the source's internal outline. The sweep
explicitly asks reviewers to detect lists in the same order. Giving the
prose its own teaching organisation would remove this remaining similarity
without losing the coverage map.

This is a contract finding about discretionary arrangement. It is not a
request to shuffle every pair of related ideas. The order of a warning's
label, preventive action and consequence has a direct functional reason,
so I raise no equivalent finding against `safety.md`.

## Verdict for each file

“Clean” means that this bounded comparison found no actionable overlap. It
does not mean that the file received a new runtime or checker verification.

| File | Verdict | Source pages compared |
|---|---|---|
| `references/words.md` | Clean | 45–62, 119–121 |
| `references/noun-clusters.md` | Clean | 63–66 |
| `references/verbs.md` | Clean | 67–76, 121–122 |
| `references/sentences.md` | F3 | 77–86 |
| `references/procedures.md` | F2, F3 | 87–94 |
| `references/descriptions.md` | F3 | 95–102 |
| `references/safety.md` | Clean | 103–106 |
| `references/punctuation.md` | F3 | 107–114 |
| `references/practices.md` | F1 | 115–128 |
| `SKILL.md` | Clean | 45–128 |
| `always-on/instruction.md` | Clean | 45–128 |
| `docs/TECHNICAL-COMMUNICATION.md` | Clean | 45–128 |

`SKILL.md` and `always-on/instruction.md` are under
`plugins/linkuistics/skills/technical-communication/`. The user guide is at
the repository root's `docs/` path, as shown.

## The three trade-offs

**Necessary grammar: accept the narrow claim; reject the blanket exemption.**
Adding an article, restoring a clause boundary, moving a condition or
expanding an abbreviation necessarily demonstrates that operation. Those
operations do not make an example an adaptation by themselves. The current
worker, ticket, branch, server and linter cases do not retain a distinctive
source scenario beyond that operation. F1 and F2 concern choices that the
rule does not require.

**Coverage numbers and order: accept the numbers; change the teaching outlines
in F3.** Numeric references can remain, including ascending numbers in a
coverage table. The required nine-file layout and `SKILL.md` rule index can
remain too. A complete prose outline in the source's order is a separate
arrangement, and the coverage exception does not expressly allow it.

**Generic identifiers: accept.** The identifiers `word-count`,
`vertical-list`, `false-friend` and `no-latin-abbreviation` name ordinary
language concepts. They do not reproduce a distinctive rule title. Replacing
them would add little protection and would disrupt the skill's identifier
conventions. A longer distinctive title would require a different verdict.

## Check of the producer's rewritten passages

I compared the producer's diff with the current files and the page images.
The following verdicts cover the substantive rewrites recorded by that leaf.

| Rewritten passage | Verdict |
|---|---|
| Long-name introduction and abbreviated queue example | Clean: a declaration and a software dependency replace the source's procedural scenario |
| Word-choice guidance for a short term | Clean: the prior file path supplies identity; the source's multi-part physical assembly does not remain |
| Unknown actor in the passive | Clean: the amendment example removes the source's causal-error demonstration |
| Release steps and merge-check list | Clean examples: no retained physical procedure or assembly list; F3 concerns the surrounding outline |
| Article guidance and examples | Clean: the smallest grammatical operation remains, without the source's particular assembly scenario |
| List punctuation guidance | Clean wording and regrouped list; the broader outline remains F3 |
| Comma guidance | Clean: the paired adverb-placement example is gone |
| Step limit and token-validity note | Clean: the new cases do not retain the source's particular measurement scenario |
| Correct notes | First note is F2; the second note is clean |
| Test that hides notes | Clean restatement of a checking method; no retained wording or example scenario |
| Description types | Clean: reordered, with model and review uses added |
| Safety label guidance, caution example and consequence explanation | Clean: the index writer, reversibility and cost cases are independent |
| Semicolon explanation and hyphenated verbs | Clean: ordinary punctuation concepts, with different examples |
| Parenthesis uses | Clean: seven source categories regrouped into four, with independent examples |
| Failed word swaps, pronouns and possessives | Clean: different examples and independent wording |
| Examples using “with” | F1 |
| Latin abbreviation example | Clean: a minimal language substitution in a different software scenario |
| Core summaries in the skill and instruction | Clean: independently worded summaries of ideas |

The recommendation order in the body of `practices.md` now differs from the
source. Its numeric coverage table is permitted. I found no additional
distinctive five-word phrase to report from the visual comparison. I did
not rerun the producer's phrase-search sweep or treat its reported phrase
count as proof of independent expression.

## The single change that would reduce risk most

Give the reference prose its own teaching organisation, while keeping the
numeric coverage map separate. That addresses F3 across four files and
reduces the tendency to invent examples while following the source page by
page. It also gives the two example findings a better repair context than
another round of noun substitutions.

## Evidence and limits

I read all of Part 1, PDF pages 45 to 128, as page images. This includes the
summary pages, explanations, examples and blank pages. The harness has no
PDF `Read` tool. `pdftoppm` rendered temporary images outside the repository,
and the image viewer displayed them. No source text was extracted or OCRed.
This report identifies pages and describes similarities in original words;
it carries no quotation from the standard.

The repository graph was initially indexed for this session at generation
`2026-10-18T08:18:49Z`. Coverage was checked for the reviewed paths and the
contracts. That fast index excluded `docs/`, so those documents were read
directly. A later coverage read reported a full index at generation
`2026-10-18T08:23:58Z`, with no recorded gap for the report and task file.
All twelve subjects were read directly: this review concerns prose, and
does not infer textual originality from a code graph.

This was an inspection-only review. No implementation file was edited and no
test, build, lint or format command ran. The producer records 19 of 20
repository checks passing, with a Bun version mismatch in the remaining
check. That is recorded producer evidence, not a result verified here.
The historical task-log naming question remains with the human as the
producer reported; it is outside this review's twelve-file comparison.
