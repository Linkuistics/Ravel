# book-read-one-k70

## Goal

Hold read 1 of the book: the owner's labelled stand-in read of Parts I and II,
recorded with its evidence class, its corrections routed to the stages that own
them, and the second half's planning leaf cut under `ravel-book-k55`.

## Context

- Contract, "Reader checkpoints": `docs/book/contract.md`. The reader is the
  owner, reading as a labelled stand-in for the intended reader, and the result
  can establish accuracy, coverage and the owner's own results, not whether a
  developer new to Ravel can find, understand and use the book.
- **Tasks**: write a policy that routes to a fake agent; run a session; interpret
  a refusal and a cancellation; and arrive at one direct entry, such as an
  exit code or an error message, without reading what precedes it.
- **Entry surfaces**: begin at chapter 1 or at the stated entry, in the locally
  built book (`task docs:book`), after confirming the figures load there. The
  README's short entry (`book-readme-and-authority-k69`) is the repository's own
  door; start there if the owner wishes.
- **Viewing environments** to offer, in order: the locally built book; the raw
  Markdown files in an editor preview; GitHub's view, which is unchecked until
  the branch is pushed (this workstream is not on GitHub) and is named as
  unchecked, not omitted; the published site, which comes only with a release.
- Record, as the contract requires: the reader's own words, how far they got,
  the locations they report and what they did not reach. Report separately and
  never as one another: the owner's results, task performance, mechanical
  checks (`task docs:book`, `docs:examples`, `docs:figures`, `check`) and any
  model-proxy result. An early comprehension failure stops the read and routes
  its correction to the stage that owns it before dependent drafting continues.

## Done when

- The read is held with the owner and recorded in this leaf's log with its
  evidence class.
- Each correction is routed to the owning stage: a `leaf-add` of the stage's
  re-run (kind `draft`, `copy-edit`, `art` or `proof`) under the unit's
  document node, in pipeline order ending at `proof`, with the specific defect
  in its body; or recorded as a contract change with the owner. A second
  re-run of one stage against one document is an escalation.
- The second half is planned: a `planning` leaf for Parts III and IV is cut
  under `ravel-book-k55`, carrying chapters 7–9, 11 and 12 (chapter 10 is not
  this node's), the units' handed-forward items from the node brief, the
  examples policy's source-excerpt check (chapter 8), and read 2's tasks from
  the contract. If corrections are still open when the session ends, the
  planning leaf is cut after them in the walk.
- Insert that planning leaf before the live whole-book `ravel-book` art leaf,
  using its stable handle from the node brief/tree. Its planning body requires
  chapter units and guide migration before whole-book art, then whole-book
  proof and read 2. Chapter 10 remains outside this node. The final art pass
  checks colours, fonts, line weights, arrows and spacing across the complete
  book; the prose target and Ravel CLI term are the amended contract's.
- The owner's word on whether the migrated README and removed guides read
  correctly is recorded.

## Notes

- HITL. Stop and ask the owner for the reading; a model's proxy is not read 1.

## Decisions (running log)

**Mechanical checks before the read (2026-10-16; evidence class: mechanical,
not the owner's result).** Subject: commit `34c901ed`
(`book-readme-and-authority-k69`) with an empty working copy. All 3,546 tracked
files were digested before and after the checks, unchanged (whole tree
`cd8014e5…0234`, `docs/book/` `9c0bc35b…34cb`). Neither migration commit
(`k68`, `k69`) changed `docs/book/src/`, so the chapters read are the ones
`book-first-half-k100` proved. `task docs:book:build` built with no mdBook
error or warning. `task docs:figures`: 9 of 9 figures match their sources
(PlantUML 1.2026.8, d2 0.9.0, DejaVu 2.37; the script used Homebrew OpenJDK 27
because this host's default Corretto 16 is x86-64 and does not start).
`task docs:examples`: 13 passed, 0 failed, 0 skipped, 6 fragments not run.
`task check`: 339 passed, 46 skipped. In the book served by `task docs:book` on
`127.0.0.1:8775`, a headless browser found every figure on chapters 1, 2, 3, 5
and 6 loaded (complete, non-zero natural size; chapter 4 has none). Each figure's
`<img>` occurs twice in the HTML because mdBook 0.5.4 adds a click-to-zoom copy
inside a `display: none` wrapper; one copy renders. These checks support the
read; none stands in for it.

**Owner finding R1, framing (2026-10-16; evidence class: the owner's stand-in
read).** The owner's words: “We should be explicit that the programs we are
running are LLM sessions in harnesses such as codex or claude code. We should
also point out that these harnesses typically cannot exit themselves, which is
why we have the `end-session` command. Thus ravel allows you to use such
interactive harness as 'sub components' of another process, typically an
orchestrator.” Located: chapter 1's opening (a generic “tool asks for work”
with a code-review example), “From a request to a result” (“An AI coding agent
is one possible program”; the `end-session` paragraph says what the command
does, not why it exists), and chapter 2's harness definition under “The managed
process and its result”. Outside the chapters, the same framing is in the
contract's “The intended domain” (“often AI coding agents (harnesses)… but
neither defines the domain”), `CONTEXT.md`'s **Harness** and the README's first
paragraph. Chapter 4's “Route to Claude Code and Codex” already starts the
interactive CLIs and puts the ending instructions in the caller's prompt, so it
agrees with the finding; only the reason is missing. Owning stage: draft
(teaching and technical truth), with a contract change for the domain
statement.

**Owner finding R2, the publication sentence (2026-10-16; evidence class: the
owner's stand-in read).** The owner's words: “"Publication is the start of
completion" wasn't clear that publication refers to the result value from the
para before the diagram.” Located: chapter 1, the first sentence after the
system-context figure. Its referent, the `end-session` publication, is two
paragraphs and a figure earlier. The owner resolved the referent, so this did
not stop the read. Owning stage: wording, but it sits in the passage R1
rewrites, so the draft re-run that carries R1 takes it too.

**Owner decision: correct before reading on (2026-10-17; the owner's words).**
“I would like you to apply those fixes you already know about before I
continue reading. I prefer to read the entire thing from start to finish
rather than isolated sections.” This answers the escalation put to the owner
on 2026-10-16. A new draft → copy-edit → art → proof run against
`what-ravel-is-for-and-its-actors` is that document's second correction run;
the first was draft k78 → proof k81, sent back by art k77. The editorial
family makes a second run an escalation, and the owner authorises this one.
Its cause is new input from the reader checkpoint and an owner framing
decision, not stages sending work back and forth. Consequences: the read
pauses after R1 and R2 and this leaf stays live, because the read is still
this leaf's work, just not next. R1 and R2 go through one run in the unit's
node, and the README's opening follows the corrected chapter 1 in its own
leaf before the read resumes. The resumed read starts again at the README or
chapter 1 and runs to the end of chapter 6, with the contract's four tasks.
The corrections are not edited into the chapters in this session: each stage
reads the whole document under its own charter, and the owner's full read
should meet proofed text that no later stage will rewrite behind it.

**Contract change for R1 (2026-10-17; the owner's finding, carried into the
contract in place).** `docs/book/contract.md`, "The intended domain", now
opens with the owner's R1 words and states the domain they describe: Ravel runs
LLM sessions in agent harnesses such as Claude Code and Codex as supervised
parts of another program; an interactive harness does not exit when its LLM
finishes, so the prompt has the LLM run `end-session`, after which Ravel stops
whatever is still running in the harness's process group and returns the
result after cleanup; a caller such as an orchestrator can therefore use the
harness as one component. The generic mechanism stays stated (any program
running `end-session` can stand in, hence fake agents; no meaning given to
kinds or results, no model provider, no store), and "neither defines the
domain" is gone. The same edit adds the harness to the managed-process role,
"including why `end-session` exists" to chapter 1's row of the section plan,
the orchestrator and harness to the system-context row of "Figure needs", and
names Claude Code, Codex and the LLM session in the "Harness" term risk.
`CONTEXT.md` follows: its opening line, **Caller** (such as an orchestrator)
and **Harness** (an interactive harness does not end itself, which is why the
LLM publishes with `end-session`). The post-publication behaviour was checked
against `docs/specs/session-execution.md` ("Publication requests the
supervisor's 2-second grace, then TERM, up to 5 seconds kill-grace, then KILL
when needed"). The wording is this session's, built from the owner's words;
the resumed read is where the owner checks it. Also done while the contract
was open, as the node brief handed forward: "Exclusions", "The owner's personal
configuration", now names `docs/configuration-handoff-2026-10-12.md` instead of
"today's configuration guide".

**Routing of R1 and R2, and the pause (2026-10-17).** One correction run under
`what-ravel-is-for-and-its-actors-k62`, cut in pipeline order in one call:
draft `what-ravel-is-for-and-its-actors-k102` → copy-edit `-k103` → art
`-k104` → proof `-k105`. The draft's body carries both findings in the owner's
words, their line locations at `34c901ed`, the anchors chapters 3–6, `SUMMARY`,
the README and `docs/authority.md` link to (their counts came from a search
seen to find them), the limits on claims about real agents, and the
escalation bound: another send-back against this document goes to the owner.
The art body names the generic system-context labels; the proof body asks for
a repeat of k100's local link check with a control seen to fail. The document
brief records the run and the amended reader outcome. R1 also reaches the
README's first paragraph, so `book-readme-k106` (impl) was inserted ahead of
this leaf. It rewrites that paragraph from the corrected chapter 1 and, as a
labelled model-proxy observation rather than the owner's, replaces the four
uses of “front” the glossary's **Ravel CLI** avoids. The insert moved this
leaf to position 18 and `ravel-book-k101` to 19; a search, with a control
seen to find `.grove/` references, found no position-prefixed reference to
either. Chapters 3–6 were read for statements R1 makes false and none was
found; chapter 4's Claude Code and Codex routes already agree. The second
half's planning leaf is not cut yet. It belongs to the end of the read, after
the owner's remaining findings, and is still owed by this leaf's `Done when`.
The resumed session holds the full read from the start, the four tasks, the
owner's word on the README and the removed guides, and that planning leaf.

**Mechanical checks before the resumed read (2026-10-17; evidence class:
mechanical, not the owner's result).** Subject: commit `5bc411d1`
(`book-figure-viewing-check-k118`) with an empty working copy. All 3,578
tracked files were digested before and after the checks, unchanged (digest
list `c150c882…824a`; `docs/book/` `75d98c99…730b`). Since the 2026-10-16
checks the subject has changed in chapters 1 and 2 (k102–k105), the README
(k106) and every figure, now drawn by Typst 0.15.1 with CeTZ 0.4.2, sketched
at roughness 2 (k107–k118). `task docs:book:build` built with no mdBook error
or warning. `task docs:figures`: 9 of 9 figures match their sources and layout
records (Typst 0.15.1, CeTZ 0.4.2, DejaVu 2.37). `task docs:examples`: 13
passed, 0 failed, 0 skipped, 6 fragments not run. `task check`: 339 passed, 46
skipped. In the book served by `task docs:book` on `127.0.0.1:8775`, a headless
browser found each of the nine figures on chapters 1, 2, 3, 5 and 6 loaded
(complete, non-zero natural size, displayed at the 750 px column; chapter 4 has
none), and a missing figure, the control, came back with width 0 and a load
error. Each figure's `<img>` still occurs twice because of mdBook's hidden
click-to-zoom copy. These checks support the read; none stands in for it.

**Owner findings R3–R9, chapter 1 (2026-10-17; evidence class: the owner's
stand-in read).** The resumed read began at chapter 1 in the locally built
book; the owner reported these together: “This is just chapter 1.” Lines are
at `5bc411d1` in `docs/book/src/01-what-ravel-is-for.md`.

- **R3**, in the owner's words: “"In the review scenario ..." add "mentioned
  above". This makes it clear what we are talking about.” Line 30; the
  referent is the code-review orchestrator of the opening's second paragraph
  (line 15). Wording, in the passage R4 and R9 rewrite, so the draft that
  carries them takes it, as the R1 draft took R2.
- **R4**: “"Their owner policy is Typescript ..." add a comment that this is a
  configuration file. That immediately contextualises it.” Lines 34–36. The
  default entry is `$HOME/.config/ravel/policy.ts` (chapter 6, `policy.ts`).
  Draft (teaching).
- **R5**: “"It invokes ravel start-session ..." make that "The owner", because
  "it" doesn't work if it is a person.” Line 27, with “the sort of work it
  wants” on line 30; both pronouns stand for the caller, “the person or program
  making the request”, so “The owner” is put to the owner as a possible slip
  for “The caller”. Wording, in the passage the draft rewrites.
- **R6**: “"If the caller chooses "continue" ..." make that 'For example, the
  caller may choose (the text) `continue` as the return value ...' and later
  mention that it means something to the caller, but not ravel. The notion of
  what the RESULT is, is not 100% clear and could do with a simpler more
  explicit explanation.” Lines 51–61, the `end-session RESULT` paragraph. Draft
  (teaching): what the result is, who chooses its values, and what Ravel does
  and does not do with it.
- **R7**: “"Publishing the result ..." - publishing is a confusing meaning
  here.” Line 63. This is chapter 1's second stumble on the word (R2 was
  “Publication is the start of completion”). The publish family runs through
  chapters 1–6, a chapter 3 heading and its anchor, example kinds and prompts,
  figure labels, `CONTEXT.md` and the CLI's help (“Publish exactly one opaque
  result to the enclosing session and request its end”, “The first publication
  wins”); the runtime diagnostics do not use it. Whether the fix is local or a
  term change is put to the owner.
- **R8**: “We repeat the diagram colour description in the para before the
  diagram, and in the legend.” Lines 88–91 repeat the key of
  `figures/system-context.typ` (“Blue: Ravel. Grey: caller and managed
  process. Slate: human owner.”, “Green arrows carry the opaque result.”). The
  same repetition precedes every figure in chapters 2, 3, 5 and 6 (chapter 2
  line 122; chapter 3 lines 288 and 337; chapter 5 lines 628, 648 and 708;
  chapter 6 lines 935 and 950), found by a search for colour words. Art
  (figures and their text).
- **R9**: “We should mention that the prompt supplied by the caller probably
  use the kind as a way of selecting skills, as well as way that ravel selects
  a harness with model/effort params.” Lines 26–32, where the kind is
  introduced. The book never mentions skills. Chapter 4 already has a policy
  passing model and effort arguments chosen by kind (lines 406–418), and the
  `provider`, `model` and `effort` selection fields are labels for the starting
  notice (chapter 3, line 148). The owner policy, not Ravel, chooses the
  harness and its arguments, and using the kind to pick skills is a caller's
  convention that the book does not present as Ravel's (contract,
  Exclusions). Draft (teaching and technical truth).

**Owner decisions on R5, R7, the timing and ASD-STE100 (2026-10-17; the
owner's answers and words).**

- R5: the owner chose “The caller”. The two pronouns on lines 27 and 30
  become the caller.
- R7: the owner chose “Replace it, CLI help too”, then the verb “submit”.
  “Submit” replaces “publish” and “publication” in chapters 1–6, the README,
  `CONTEXT.md` and the contract. This includes headings, anchors, example
  kinds and prompts, and figure labels. The two `end-session` help strings in
  `crates/ravel/src/cli.rs` change too. The specifications and the decision
  records keep their words.
- Timing: the owner chose “Fix before reading on”, then “Route now”. The
  corrections go to their stages now. The read continues after they land.
  This authorises the third correction run on
  `what-ravel-is-for-and-its-actors`. A fourth run goes to the owner again.
- STE, in the owner's words: “I intend to add a global instruction to use
  ADE-STE100 for all LLM communication (such as when you talk to me here) as
  well as in all documentation. Apply that rule now.” This session reads
  “ADE-STE100” as ASD-STE100, Simplified Technical English, Issue 9 (January
  2025). From this point, the messages to the owner use STE. The book's prose
  contract takes ASD-STE100 as its base. Rule 1.14 permits another spelling
  when an official directive gives one, so the contract's British spelling
  stays. Rule 3.5 permits “-ing” forms only in technical names, so the six
  chapter titles with “-ing” forms become commands. Their file names stay, so
  no link breaks. Rule 1.13 does not permit a technical verb as a noun, so the
  book uses “submit” and “submitted result”, not “submission”. The rule
  applies to the book, its README, `CONTEXT.md`, the contract text this
  session writes and the two changed help strings. Other documents get STE
  when later work changes them.

**R7 verb under STE, and the dictionary (2026-10-17; the owner's answers).**
The owner downloaded ASD-STE100 Issue 9 (2025-01-15) to
`~/Downloads/ASD-STE100_ISSUE9.pdf` (434 pages, SHA-256
`d1f4ea9e7cd6e46b47aa9057209f99e78c0e9cfc4e27a5b07895b05c1a166431`). Its
copyright notice prohibits unauthorised distribution, so the PDF and text from
it stay out of the repository. The dictionary does not approve “submit (v)”
and gives SEND as its alternative. Rule 1.12 permits a technical verb only
when no approved verb gives the same information. The owner then chose “send”.
This replaces the earlier “submit” decision above. End-session sends the
result, and Ravel keeps the first result that it receives. The dictionary also
gives TELL for “ask” and “request”, STOP for “end” as a verb, and ACCURATE for
“exact”. “Opaque” is not approved, but Rule 1.6 permits it inside the
technical noun “opaque result”.

**Contract change and routing, second pause (2026-10-17).** This session
changed the contract and the glossary. In `docs/book/contract.md`, the prose
contract's base is ASD-STE100 Issue 9. Section 2 now gives the owner's words,
the local copy of the specification and its distribution limit, the
word-decision file `docs/book/ste.md`, British spelling under Rule 1.14,
technical names, “send” in the plain vocabulary, the STE limits for sentences
and paragraphs, no semicolons or contractions, and the active voice with
simple tenses. The section plan has the command-form titles. The domain
paragraph, the term risks, inventories 3 and 4 and the timing figure question
use “send”. The “published site” (the book on GitHub Pages) is a different
concept and stays. `CONTEXT.md` uses “send” and lists “publish”,
“publication” and “submit” under the opaque result's _Avoid_. K123 converts
the rest of `CONTEXT.md` to STE.

The run, in walk order:

1. `book-first-half-k123` (copy-edit), inserted before the chapter units:
   the shared STE word decisions. The insert renumbered 30 siblings, and the
   verb reported no cross-reference to fix. A search for position-prefixed
   names, with a control that found both planted forms, found two. Both stay
   correct after the insert.
2. Under `what-ravel-is-for-and-its-actors-k62`: draft k124 (R3–R7, R9),
   then copy-edit k125.
3. Copy-edits under their unit nodes: k126 (chapter 3), k127 (chapter 4),
   k128 (chapter 5) and k129 (chapter 6).
4. Inserted before this leaf: `book-first-half-k130` (art, with R8),
   `book-readme-and-help-k131` (impl) and `book-first-half-k132` (proof).

Set art and set proof serve all six chapters, so the unit nodes cut no art or
proof leaf. Each unit brief and the book brief say so. The read stays this
leaf's work. It starts again from the beginning after k132. The second half's
planning leaf and the owner's word on the README and the removed guides are
still owed.

**Mechanical checks before the second resumed read (2026-10-18; evidence
class: mechanical, not the owner's result).** The subject is commit `1ef02398`
(`book-first-half-k132`) with an empty working copy. A SHA-256 manifest in
k132's scope has 3,519 subjects: the root files, `docs/`, `crates/` without
`node_modules`, `scripts/` and `target/tools/`. It is equal to k132's final
record. The same comparison against k132's first record found one difference,
`docs/book/src/06-reference.md`, which k132 changed. Thus, the comparison can
find a change. The manifest did not change during the checks. The results:

- `task docs:book:build`: no mdBook error or warning.
- `task docs:figures`: 9 of 9 figures match their sources and layout records
  (Typst 0.15.1, CeTZ 0.4.2, DejaVu 2.37).
- `task docs:examples`: 13 passed, 0 failed, 0 skipped. Six fragments did not
  run. There were zero page errors.
- `task docs:ste -- --strict`: zero problems in 18 files. In
  `task docs:ste:test`, each control fails as it must.
- `task check`: 339 tests passed, 46 skipped.
- The book that `task docs:book` serves on `127.0.0.1:8775`, in a Playwright
  browser at 1280 by 900 pixels: each of the nine figures in chapters 1, 2, 3,
  5 and 6 is complete, has a nonzero natural size and shows at the 750-pixel
  column. Chapter 4 has no figure. mdBook adds a hidden zoom copy of each
  image. A control image with no file gave a load error and a 0×0 size. The
  only console error is the 404 of that control.

The first STE run failed because of the instrument, not the book: zsh gave
`docs:ste -- --strict` to `task` as one word. The direct run above replaced
it. These checks support the read. None of them stands in for it.

**Owner findings R10–R17, the full read (2026-10-18; evidence class: the
owner's stand-in read).** The owner read in the book that `task docs:book`
serves. In the owner's words: “I have read part one completely and part two
I have skimmed. Both look correct excepting the above issues.” The owner did
not report the four tasks of read 1. Lines are at `1ef02398`.

- **R10**, in the owner's words: “I've changed my mind about the hand-drawn
  look. I don't think it looks friendly, I think it just looks wobbly, so I'd
  like to change that.” This applies to all nine figures (chapters 1, 2, 3, 5
  and 6), the contract's “Figure style”, `docs/book/figures.md` and the
  shipped style of `_draw.typ`. Art, with a contract change.
- **R11**: “I also want to make sure that we don't have too long expanses of
  prose without a diagram or a table to break it up.” A measurement of each
  chapter found these longest runs of prose between figures and tables: 900
  words in chapter 1 (lines 1–107), 1,329 in chapter 2 (lines 1–155), 2,305
  in chapter 3 (lines 1–330), 862 in chapter 4 (lines 164–443), 793 in
  chapter 5 (lines 74–333) and 2,200 in chapter 6 (lines 1–295). If code
  blocks also count as breaks, chapters 4 and 5 fall to 549 and 529 words.
  The others do not change. Draft (tables and structure) and art (figures),
  with a contract change.
- **R12**: “Rather than saying writes the exact bytes of the result to
  standard output, we should say something like writes the result unchanged
  to standard output. It does no interpretation of this result.” Chapter 1,
  line 128. Chapter 5, line 189, says that values “keep their exact bytes”.
  The `start-session --help` text says “Only the exact result is written to
  stdout”. Copy-edit (wording).
- **R13**: “Rather than saying these outcomes give no result, we should say
  some outcomes return no result to the caller.” Chapter 1, line 138: “These
  outcomes give no successful result:”. Copy-edit (wording).
- **R14**: “We should make it obvious that the reason we want to use an LLM
  harness is because it allows us to use a subscription rather than an API
  session which is much cheaper.” Chapter 1, lines 1–30, introduce the
  harness without this reason. No chapter says “subscription”. The contract's
  “The intended domain” and the README's first paragraph also lack it. Draft
  (teaching), with a contract change.
- **R15**: “We should emphasize that the fact that the configuration runs a
  harness is not fully required. The contract interface between the Revel
  CLI and the configuration is that it runs something and returns a value.
  Now it is the case that the Revel CLI specifically talks about prompts and
  we expect that it will be a harness that it runs, but this is a very loose
  contract.” Chapter 1, lines 60–63, say this in two sentences. The
  `start-session --help` text calls `--prompt` “The harness prompt”. Draft
  (teaching), with a contract change.
- **R16**: “When we say that a later end session does not replace the first
  result value, we should make it clear that this is a form of race condition
  and in fact end session should terminate the harness execution.” Chapter
  2, line 128, chapter 5, line 192, and chapter 6, line 266. The help text
  gives the schedule after `end-session`: 2 seconds, TERM, up to 5 seconds,
  then KILL and the reap of the group. Draft (technical truth).
- **R17**: “In chapter 3 where we say obtain the build by used by this
  chapter we should instead simply refer them to installing Ravel using
  Homebrew and then checking that the version is the same as the book
  describes.” Chapter 3, lines 17–66 (“Obtain the build used by this
  chapter”). Chapter 4 (line 11), chapter 5 (line 13) and chapter 6 (lines
  12, 960 and the installed-layout table) link to it. The formula
  `linkuistics/taps/ravel` installs 1.0.0 for aarch64 macOS, aarch64 Linux and
  x86_64 Linux. The workspace version is still `1.0.0`. Version 1.0.0 does
  not have the `operationReports` notice field or the reworded `end-session`
  help. Draft (teaching and technical truth), with an owner decision on the
  version.

Crisp copies of all nine figures were rendered for the owner with the pinned
Typst (`--input style=crisp`) outside the repository. They use the same
layout records.

**Owner decisions on R10, R11 and R17 (2026-10-18; the owner's answers).**

- R10: the owner chose “Crisp”, after a side-by-side view of all nine
  figures. The figures use the crisp style of `_draw.typ` (roughness 0,
  exact geometry). The layout, colours, fonts and keys do not change. This
  replaces the sketched style of 2026-10-17.
- R11: the owner chose “One screen, code counts”. A run of prose has about
  450 words or fewer (one screen at the 750-pixel column) before a figure, a
  table or a code block breaks it. A break carries information that the
  reader needs. It is not decoration. A Taskfile check reports each longer
  run, in chapters 1–6 and in chapters 7–12. At `1ef02398`, ten runs are
  longer: two in chapter 1, one each in chapters 2, 3 and 6, three in
  chapter 4 and two in chapter 5.
- R17: the owner chose “Set at release time”. Chapter 3 tells the reader to
  install Ravel with Homebrew and to run `ravel --version`. The book names
  the version in one place. The release writes its own version there and
  checks it. Until that release, the book shows a placeholder. The release
  machinery comes now, before the version is known.

**Owner decisions on the correction run and the README (2026-10-18; the
owner's answers).**

- The owner chose “Authorise the run”. Every unit had a re-run of the stages
  that it now needs, so the editorial rules make this run an escalation. Its
  cause is the reader checkpoint, not stages that send work back and forth.
  Another send-back on a unit after this run goes to the owner again. The
  approved plan, in walk order: an impl leaf for the prose check, then a
  draft and a copy-edit for each unit, then set art, an impl leaf for the
  release version, an impl leaf for the README, set proof and the planning
  leaf for Parts III and IV. The book names no price or plan for a harness
  subscription. Each harness's own documentation covers its plans and
  terms. The `start-session --help` text keeps “Only the exact result is
  written to stdout”.
- README and removed guides: the owner chose “Yes, they read correctly”. This
  is the owner's word on the reduced README and on the removal of
  `docs/session-cli.md` and `docs/owner-configuration.md`. R14 and R15 still
  reach the README's opening through the README leaf.

**Contract changes and routing (2026-10-18).** This session wrote the
owner's decisions into `docs/book/contract.md` in place: “The intended
domain” (R14 and R15, with the owner's words), “Emphasis”, the section plan
rows for chapters 1 and 3, “The reader's Ravel” in the examples policy
(R17), “Breaks in prose” in the prose contract (R11) and “Figure style”
(R10). `docs/book/ste.md` has R12 and R13 under “The result”. The STE check
does not find their old words yet, so that the check stays green until the
copy-edits land. The root brief has a note on R14 and R15 for the later
component work.

The run, in walk order, with each body carrying its defects and locations:

1. `book-prose-breaks-check-k133` (impl), inserted before the unit nodes.
   The insert moved 33 siblings. A search for references by position, with
   a control that found both planted forms, found two. Both stay correct.
2. A draft and a copy-edit in each unit node: k134 and k135
   (`what-ravel-is-for-and-its-actors-k62`), k136 and k137
   (`a-complete-session-k63`), k138 and k139 (`writing-an-owner-policy-k64`),
   k140 and k141 (`running-sessions-k65`), k142 and k143
   (`reference-chapter-k66`).
3. Inserted before `ravel-book-k101`: set art `book-first-half-k144`,
   `book-release-version-k145` (impl), `book-readme-k146` (impl), set proof
   `book-first-half-k147` and `ravel-book-second-half-k148` (planning).

Set art and set proof serve all six chapters, as in the run of 2026-10-17.
Each unit brief and the book brief say so. The chapters 1–2 brief also lost
two stale statements: the R8 entry that set art k130 closed, and the
PlantUML file names. After these edits, `task docs:ste -- --strict` (zero
problems in 18 files), `task docs:ste:test` and `task docs:book:build`
passed. The four Done-when items of this leaf hold: the read is recorded
with its evidence class, each correction has its stage, the planning leaf
precedes whole-book art, and the owner's word on the README and the removed
guides is recorded.
