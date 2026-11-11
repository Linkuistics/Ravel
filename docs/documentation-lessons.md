# Lessons for documents and communication

This file records what this grove learnt about documents and communication through 2026-10-28.
Its rules apply to books, reference pages, specifications, decision records, help text and replies.
They also apply to questions, handovers, task files, briefs, commit messages and prompts for other models.
Another project can use the rules after it states its own readers, terms and evidence requirements.

Each later session adds its lessons here when they arise.
Keep the rule, its source, its evidence class and its application together.
The owner wants a skill for other groves. The skill waits until the owner names its home.

The skill will distil the feedback into reusable decisions for authors and editors.
For each finding, identify the underlying reader problem, when it occurs, the corrective action and how to check the result.
State the scope and exceptions when a rule depends on the document's purpose or the reader's task.
Keep product-specific names, commands and corrections in the supporting examples.

For example, the terminology feedback becomes “prefer familiar terms and qualify them where precision matters”.
It does not make “configuration” the right term in every product.
The JSON feedback becomes “introduce format details when the reader needs to construct or interpret the data”.
It does not remove required version fields from examples or reference entries.

Combine overlapping lessons into a short procedure when the skill takes shape.
Keep detailed evidence and examples in supporting references that need no Grove history to understand.
Check the skill on material from another project before claiming that its rules transfer successfully.

**Owner evidence** means a recorded quotation or choice. **Session evidence** means an observation, analysis or editorial decision by a session.
**Background** means evidence from the earlier book grove, rather than a finding made in this grove.
A later accepted decision replaces an incompatible earlier one.
No check in this file establishes that the pending chapter corrections are complete.

## Explain the subject to the reader

### L01. Agree the reader and the tasks before the structure

State what the reader knows, what the document teaches and what the reader can do afterwards.
Use those tasks to choose the chapter order, emphasis, exclusions and direct entry routes.
Keep the agreement in a contract that each stage can read without another checkout.

**Source:** Owner decisions dated 2026-10-16 in the book contract, retained as background.
Session `rhai-book-use-k70`, 2026-10-26, records the reader and task interpretation.
The contract calls the book a guide with a “strong pedagogical focus”.
**Ravel application:** [Contract §1](book/contract.md#1-structure-brief) names a 16-year-old engineer, prerequisites, outcomes and chapter tasks.

### L02. Teach the current subject without an unnecessary migration story

Match prerequisites and chapter placement to the agreed reader.
Separate a particular owner's configuration conversion from teaching the product.
When the owner removes a planned guide, change its remedies, release notes and dependent tasks too.

**Source:** Owner, `rhai-book-use-k27`, 2026-10-26: “a manual migration of my configuration as a one-off manual task”.
The owner also said “rest is agreed”.
This replaces `plan-k1`'s migration-guide decision of 2026-10-24.
**Ravel application:** Contract §1 keeps chapter numbers and teaches Rhai within chapter 4, with no TypeScript prerequisite or migration chapter.
`typescript-worker-cutover-k31`, `release-ravel-2-0-0-k37` and `owner-policy-conversion-k39` carry the changed destinations.

### L03. Open with the intended use and its reason

Name the real actors and the problem they need to solve.
Explain why a command exists when you introduce it.
Keep the general interface distinct from its expected use and from one caller's conventions.

**Source:** Background owner findings R1, R9, R14 and R15, `book-read-one-k70`, 2026-10-16 to 2026-10-18.
The owner asked why harness subscriptions matter and why interactive harnesses need `end-session`.
**Ravel application:** Contract §1, “The intended domain”, explains supervised harnesses, subscriptions and the generic program/result interface.
Grove's kinds and result meanings remain labelled caller conventions.
The contract permits no price or plan claim.

### L04. Introduce a technology at its first mention

Say what the technology is and which characteristics matter here.
Show a short example and offer the technology's own manual or reference for further study.
Distinguish the upstream language from restrictions and capabilities supplied by the host.

**Source:** Owner F1, `rhai-book-use-k74`, 2026-10-27: “describe its characteristics, show an example, and link to some documentation”.
**Ravel application:** Contract §1 and §2 require the Rhai introduction in chapter 1.
`rhai-book-use-k75` owns the correction. The upstream playground has no `ravel::` capabilities or Ravel limits.
Draft `rhai-book-use-k80` put the introduction in chapter 1, “The owner policy chooses a program”, and a shorter restatement in chapter 2.
Draft `rhai-book-use-k81` restates the characteristics and the three links in chapter 4, “Know the Rhai language”.

Draft `rhai-book-use-k82` introduces `sandbox-exec` and bubblewrap in chapter 5, “Confine the managed process”, because no earlier chapter names them.
Draft `rhai-book-use-k83` restates Rhai, its links and both sandbox tools in chapter 6, because a reader can enter that chapter at any entry.

### L05. Explain a term before each page needs it

Give a short local explanation even when another chapter teaches the term.
Say what the thing is, where it comes from and why it matters here.
A link can then offer more detail.

**Source:** Owner F2 and F7, `rhai-book-use-k74`, 2026-10-27.
The owner found that “'a selection' is not described/contextualised”.
**Ravel application:** Contract §2, “Explain a term before you use it” and “Terms”, replace the earlier link-only rule.
Direct chapter 6 entries also need their own context under contract §1.
Draft `rhai-book-use-k83` opens chapter 6 with the words of every page and gives each entry a short local explanation of its terms.

### L06. Name the actor, action, destination and reader of an output

Describe a write as communication when another actor reads it.
Give the stream or file, the time of the write and the information it conveys.
Distinguish human text from a structured report when their timing differs.

**Source:** Owner F2, `rhai-book-use-k74`, 2026-10-27: “we haven't said where it writes”.
The same finding asks for a diagram and a concrete example.
**Ravel application:** Contract §2, “Say who does what, where and why”, applies to the routing notice.
`rhai-book-use-k75` and `rhai-book-use-k77` own the explanation and figure.
The text notice precedes launch, and the `--json` report follows the supervision attempt.
Draft `rhai-book-use-k80` explains the notice in chapter 1, “The selection and the routing notice”, as a who, when, where, reader and why list.

Draft `rhai-book-use-k82` restates that list in chapter 5, “Read notices separately from results”, with the different times of the text and JSON forms.
Draft `rhai-book-use-k83` restates the list in chapter 6, “Standard streams”, and adds a table of the first part of each line on standard error with its writer.

### L07. Explain the mechanism behind an abstract verb

Words such as “inherits”, “finds” and “sends” need a concrete mechanism when the reader does not know it.
Show the data, the transfer between actors and the condition that changes the transfer.
Do not replace an unexplained term with another unexplained term.

**Source:** Owner F5, `rhai-book-use-k74`, 2026-10-27: “inherited how? what is the mechanism.”
**Ravel application:** Contract §2, “Show the mechanism”, requires the environment-variable path through parents and children.
The correction shows `RAVEL_EXIT_FILE`, its inheritance and its replacement for a nested session's own child.
Draft `rhai-book-use-k80` shows it in chapter 1, “How `end-session` finds its session”, with a table of processes and the paths of a real nested run.
Draft `rhai-book-use-k82` restates the mechanism in chapter 5, “Keep nested sessions separate”, with a table of both layers and a second real run.

Draft `rhai-book-use-k83` restates it in chapter 6, “`end-session RESULT`” and “`RAVEL_EXIT_FILE`”. Chapter 6 also shows the mechanism of `ravel::env` and of a host call: the worker asks the Ravel CLI over a private socket.

Owner F12, `rhai-book-use-k74`, 2026-10-28, extends the rule to process creation and its purpose.
The owner asked why Ravel launches a separate copy of itself to evaluate the configuration.
The next correction must connect that mechanism to the parent's deadline enforcement and the isolation of a script failure.
The source and the process-boundary ADR support that explanation. They also keep worker evaluation before the final managed launch.

Draft `rhai-book-use-k88` explains it in chapter 1, “The configuration script chooses a program”.
The three reasons are the deadline, host requests and the isolation of a failure.
Chapters 2 and 3 restate the reasons, and each says that the worker ends before the launch.

### L08. State the reader's goal before the capability

Give the reason why the reader might want the operation.
Then explain the means and its effect on the reader's work.
Prefer an observable change to an abstract description of instructions.

**Source:** Owner F3 and F4, `rhai-book-use-k74`, 2026-10-27.
The owner preferred “to extend or modify the prompt” and described the desired harness/model/effort choice before the means.
**Ravel application:** Contract §2, “Describe the effect in the reader's terms” and “Put the reader's goal first”.
Draft `rhai-book-use-k81` opens each section of chapter 4 with the reason why the owner may want it. “Run a harness during evaluation” opens with the owner's F4 sentence.
Draft `rhai-book-use-k83` opens each chapter 6 entry with the reader's question or goal, then gives the facts.

### L09. Follow an action through to the caller's result

Name each actor in the order that it acts.
End the explanation with what the caller receives and the conditions that permit it.
Explain an empty result, an absent result and a process exit status separately.

**Source:** Owner F6, `rhai-book-use-k74`, 2026-10-27, and background R6, R12, R13 and R16, 2026-10-17 to 2026-10-18.
F6 asks that “the result becomes the value returned as the Ravel result”.
**Ravel application:** Contract §1 and §2 follow `end-session`, group cleanup and the return from `start-session`.
The first-result rule also needs the race and shutdown explanation.
Draft `rhai-book-use-k81` gives chapter 4 order tables for an evaluation and the first example.
Other order tables follow selection context, a harness run, the Claude Code recipe and an interactive route.

Draft `rhai-book-use-k82` opens chapter 5 with chapter 1's order table and a column for the section of each row. A table after each complete example ends with what the caller receives.
Draft `rhai-book-use-k83` gives chapter 6 order tables for `start-session` and a harness run. The `end-session` and refusal entries end with what the caller receives.

### L10. Repeat explanations where they help the reader

Restate a key fact where the reader needs it.
Use previews and summaries when they help a long section.
Keep the same terms in each restatement.
Do not force readers to follow links to reconstruct a page's explanation.

**Source:** Owner F7, `rhai-book-use-k74`, 2026-10-27:

<!-- tc: off because these are the owner's quoted words -->
> Good human-centered writing doesn't use DRY at all. Repetition and restatement are important techniques.
<!-- tc: on -->

**Ravel application:** Contract §2, “Repeat and restate”, replaces the old ban on a closing restatement.
The adjacent-text rule for colour keys remains narrower. L26 records that distinction.
Chapter 5 restates the words of every page, the three meanings of “context” and the selection. A closing section, “Check your caller before real use”, summarises the chapter.

### L11. Speak to the reader in connected prose

Use “you” where it helps. Give the reason before the detail.
Connect each sentence to the idea before it.
Short sentences still need enough explanation for a first reading.

**Source:** Owner F8, `rhai-book-use-k74`, 2026-10-27: “a more conversational tone where possible”.
**Ravel application:** Contract §2, “Talk to the reader” and “Register”, permit conversational prose within the agreed STE rules.
The session interpreted that limit as no contractions, slang or jokes. That interpretation is not a separate owner quotation.

## Keep words and contracts precise

### L12. Use writing limits without removing meaning

Split a long sentence at an idea boundary. Keep its conditions, uncertainty, actor and consequence.
Rewrite the explanation when a word substitution loses the meaning.
Do not use a clean prose report as proof that the reader understands the text.

**Source:** Background owner STE decisions in `book-read-one-k70`, 2026-10-17 to 2026-10-18.
The owner later specified “only the rules” rather than the dictionary.
Session evidence: `rhai-book-use-k73` and `rhai-book-use-k74`, 2026-10-26 to 2026-10-27.
**Ravel application:** Contract §2 sets 20-word instructions, 25-word descriptions, six-sentence paragraphs, British spelling and the writing rules.
`task docs:ste -- --strict` checks only part of that contract.

### L13. Give one concept one name

Choose shared terms before editing chapters independently.
Keep code identifiers exact while explaining them in plain words.
Resolve ambiguous domain words in the glossary and record shared wording decisions where all stages can find them.

**Source:** Owner choice in `rhai-policy-k2`, 2026-10-24 to 2026-10-25, resolves `plan-k1`'s concern that “Provider” needs one meaning.
Session evidence: `rhai-book-use-k69`, 2026-10-26.
**Ravel application:** `CONTEXT.md` keeps Provider as the owner's model-origin label.
[Word decisions](book/ste.md) keep “configuration script”, “Rhai worker”, “harness run” and the full standard-stream names consistent.
Its signal-name rule preserves literal identifiers and diagnostics.

The prose checker accepts the first word of a synonym group in a file and reports each other member. New text uses the verb that the page already uses. Chapter 4 says “start” for a process, where chapter 1 says “launch”.

Draft `rhai-book-use-k89`, 2026-10-28, found the reverse case. An edit removed the first “check” from the opening of chapter 4. Then “validate” became the first word of its group, and an unchanged-looking heading with “check” became a finding.
After an edit near the top of a file, run the checker on the whole file. A finding far from the edit can come from a change of the first occurrence.
When you copy an explanation from another chapter, use the verbs of the page that receives it.

### L14. Prefer a familiar verb and a clear referent

Name the actor when a pronoun could refer to a person or a program.
Explain a result with an example value and the actor that gives it meaning.
Keep an earlier word choice as history when a later decision replaces it.

**Source:** Background R2–R7 and R12–R13, `book-read-one-k70`, 2026-10-16 to 2026-10-18.
The owner chose “The caller” and replaced “publish”, first with “submit”, then with “send”.
**Ravel application:** Contract §2 and `ste.md`, “The result”, use “send”, “receive”, “keep”, “unchanged” and “return no result to the caller”.
Technical names remain in code font. Historical specification wording does not silently become the book's wording.

### L15. Scope each guarantee to what its mechanism can establish

Name the protected actor, resource, phase and failure conditions.
Distinguish a bound on one operation from a bound on the whole evaluation.
State accepted escape paths and the evidence that the available test cannot supply.

**Source:** Session findings and integrations `rhai-policy-k4`/`k5`, `rhai-measured-reads-k52`/`k53` and `llm-calls-through-harnesses-k59`/`k61`, 2026-10-25 to 2026-10-26.
**Ravel application:** The configuration evaluation spec separates run output limits from evaluation stops, and `env` grants from deliberate credential reads.
It limits cleanup to the supervised group and distinguishes abandonable reads from a promise to stop a kernel call.

### L16. Correct summaries and tables with their detailed rule

After a correction, inspect introductions, summaries, tables, requirements, captions and alternatives for the same claim.
Read neighbouring clauses again when you remove an incorrect clause.
A correct detailed section cannot correct a contradictory summary elsewhere.

**Source:** Session evidence, `llm-calls-through-harnesses-k61`, 2026-10-26, and `rhai-book-use-k70`, `k71`, `k73`, 2026-10-26.
**Ravel application:** The design integration found false summaries during its own sweep.
The chapter drafts corrected cleanup precedence, budget-check order and settlement time.
Proof corrected chapter 4's deadline summary after chapters 5 and 6 already stated the exception.
Draft `rhai-book-use-k82` found its own new order tables in chapter 5 saying “stops” where the detailed text says that Ravel attempts the stop. The tables now say “attempts”.

Proof `rhai-book-use-k78`, 2026-10-28, found new summaries that omitted the
JSON notice's later time, evaluation harness runs before refusal and partial
output after a failed final write. Repeated figure introductions also lost
the condition that a parent can change its child's environment.
These are session findings, not new owner decisions. Check each repeated
statement against its exceptions, even when the detailed entry is correct.

Proof `rhai-book-use-k87`, 2026-10-28, found another summary with the same refusal ambiguity in chapter 3.
Chapter 6 also called the environment helper's base variables the worker's variables, although the Rhai worker has an empty environment.
Check summaries against the current mechanism after a terminology change. A familiar old description can retain the wrong recipient.

### L17. Separate a new contract from delivered behaviour

Say which release a document describes and which implementation steps remain.
Give each temporary mismatch an owner and a check at its delivery boundary.
Preserve dated measurements when a current requirement changes.

**Source:** Session evidence, `rust-2024-toolchain-k7`, 2026-10-25, and `rhai-book-use-k27`, `k70`, `k71`, `k73`, 2026-10-26.
**Ravel application:** The book describes 2.0.0 while the checkout still supports the old format.
Cutover owns retired-format refusal, delivery owns the notice path, and release owns the version placeholder.
Past Rust 1.85.0 measurements retain their original version.

### L18. Keep decisions, specifications, teaching and evidence distinct

Put the design reason in a decision record and the interface rules in the specification.
Teach the behaviour in the book without requiring either document for comprehension.
Keep campaign receipts, personal handoffs and detailed measurement logs outside the teaching text.

**Source:** Contract §1, “Relation to other documents”, background agreement of 2026-10-16.
Session evidence: `rhai-policy-k2`, `llm-calls-through-harnesses-k58`/`k59`, 2026-10-25 to 2026-10-26.
**Ravel application:** The replacement harness-call ADR owns the sign-in/process-cost trade-off.
The spec owns request and result shapes. The book explains both in its own words.
The README is an entry into the book.

## Make examples and references trustworthy

### L19. Derive complete coverage from the subject

Enumerate the actual inputs, fields, codes, operations and outcomes before comparing a reference with them.
Classify every unmatched item. Distinguish an assigned future test from an executed result.
Repeat the inventory after the implementation changes.

**Source:** Session evidence, `rhai-policy-increments-k6`, 2026-10-25, and `rhai-book-use-k71`/`k73`, 2026-10-26.
**Ravel application:** Contract §1 requires source-derived CLI, configuration, lifecycle, component and contributor inventories.
Chapter 6's inventory included quoted source tokens and classified internal controls separately.
Parts III and IV must derive the final task inventory after delivery removal.

### L20. Check version-dependent instructions against their source

Read the project's pins and the installed program's help before documenting an API or argument.
Record the version, date and exact successful arguments.
Distinguish fake argument tests from a real program run.

**Source:** Session evidence, `rhai-harness-smoke-k66`, `rhai-example-policies-k25` and `rhai-book-use-k27`, 2026-10-26.
**Ravel application:** Chapter 4's recipes use the measured Claude Code and Codex vectors.
Complete examples use fakes. Opt-in smoke tasks establish the separate real-call result.
The contract requires dated agent facts and a recheck when a choice depends on their version.

### L21. Make the visible complete example the executable input

Run the commands and files that the reader sees.
Compare output, exit status and newline claims exactly where they are stable.
State partial diagnostic claims and the one path substitution explicitly.
Label fragments with what they omit.

**Source:** [Example convention](book/examples.md), retained background, revised by `rhai-book-use-k71` on 2026-10-26.
**Ravel application:** `task docs:examples` extracts the Markdown and runs the real CLI with fake programs.
Missing labels, unknown requirements and timeouts fail. Skips remain skips.
`<scratch>` substitution alone cannot prove that a path is correct.

When the text explains a diagnostic line by line, the example claims those lines. Chapter 4 claims the whole standard error of two refusals and a parse failure, which launch no program. A changed character in one of those lines failed the check.

### L22. Isolate examples and live reader exercises from the agent's session

Use a private home, temporary directory and program path with a cleared environment.
Give the reader an independent shell and state which build it runs.
Keep completion authority and personal settings out of that shell.

**Source:** Session evidence, `rhai-book-use-k27` and `rhai-book-use-k74`, 2026-10-26.
**Ravel application:** The examples create their own settings directories.
Read 1's shell runs through `env -i` outside the repository.
The agent's shell contains Grove's `RAVEL_EXIT_FILE`, so it cannot safely stand in for that reader shell.

### L23. Quote source exactly and keep its explanation separate

Give each quotation a tracked source path and a contiguous block of whole lines.
Use separate excerpts for separated passages. Explain omitted context in the label.
Correct a moved or changed excerpt in the same increment that changes its source.

**Source:** [Excerpt convention](book/excerpts.md), retained background. Session evidence: `rust-2024-toolchain-k7`, 2026-10-25.
**Ravel application:** `task docs:excerpts` compares bytes and rejects untracked, generated, linked and `.grove` paths.
Labels avoid line numbers that drift. Bare Rust and TOML blocks need a fragment, example or excerpt label.
The check never rewrites the page to make itself pass.

### L24. Keep book checks working at each implementation handoff

Move an executable example and its checker when their runtime dependency changes.
Do not leave a known failing book check for a later prose stage.
Reconcile fixtures, help text and scripts that still name the removed input.

**Source:** Session F1, `rhai-policy-increments-k6` and integration `k40`, 2026-10-25.
**Ravel application:** `typescript-worker-cutover-k31` owns chapter 9's mechanical policy port and the freshness checker.
`typescript-delivery-removal-k32` owns fixtures that quote the removed SDK.
The later book draft owns the teaching rewrite.

### L25. Install and check the version that the book describes

Give the reader the supported installation path and a version check.
Keep the release version in one place and write it during the release.
Do not imply that an older installed binary implements an unpublished example.

**Source:** Background owner R17, `book-read-one-k70`, 2026-10-18: “Set at release time”.
**Ravel application:** Contract §1, “The reader's Ravel”, uses Homebrew and one chapter 3 placeholder.
The release guards check that value. Read 1 explicitly uses the checkout build until Rhai ships.

## Use figures to explain and inspect them at reading size

### L26. Choose a visual for the reader's question

Use a diagram for a mechanism or communication path that needs spatial or sequential explanation.
Use a table for a comparison, mapping or short sequence.
Explain the takeaway beside the visual and give a concrete example where the mechanism needs one.
Keep each figure's colour key in one local place.

**Source:** Owner F2/F5, `rhai-book-use-k74`, 2026-10-27, and background R8/R11, `book-read-one-k70`, 2026-10-17 to 2026-10-18.
R8 identified the same colour description before the diagram and in its key.
**Ravel application:** Contract §1, “Figure needs”, and §2 require the notice and inheritance figures.
The contract interprets F7 as permission to repeat explanations while retaining R8 for keys.
That distinction is session analysis, not a new quotation from the owner.

A concrete example can need both a sequence and an outcome mapping. Draw the sequence when it connects an initial request to a later action across participants. Keep the table when it distinguishes alternatives that the sequence does not show. Check both against the same example.

**Source:** Art `rhai-book-use-k86`, 2026-10-28, applies owner findings F9 and F11.
**Ravel application:** Chapter 1 now follows the review through `changes-needed`.
Its table also states the caller’s action for `approved`, an unexpected value and a failed session.

### L27. Break a long prose run with useful information

Introduce a figure, table or code block before the agreed screen of prose becomes too long.
A heading or list does not itself supply that break.
Keep the visual relevant to what the reader needs to understand.

**Source:** Background owner R11, `book-read-one-k70`, 2026-10-18: “One screen, code counts”.
**Ravel application:** Contract §2 sets a 450-word threshold at the 750-pixel column.
`task docs:breaks -- --strict` reports longer runs. Its controls check the counting rules.

Draft `rhai-book-use-k89`, 2026-10-28, added two terms and a sentence about literal names to the opening of chapter 4. The run before the first table grew to 485 words.
The opening word list compared each term on the same attribute, so the draft made it a table. The table carries the same information and breaks the run.

A list of parallel items can make a run too long. Then prefer a table that the layout rules permit. Do not add a decorative break.
Draft `rhai-book-use-k90` made chapter 5's opening word list a table for the same reason. Its new column “In the review” carries information that the list did not have.

### L28. Keep one visual vocabulary and one editable source

Share colours, fonts, type scale, strokes, arrows and spacing across diagram types.
Record any reason for a notation-specific difference.
Give each figure one authoritative source and reproducible output.

**Source:** Background owner contract decisions of 2026-10-16 to 2026-10-18, including R10 in `book-read-one-k70`.
Contract §3 and [figure mechanics](book/figures.md) retain them.
The owner chose “Crisp” after comparing all nine figures.
**Ravel application:** Typst sources use shared layout and drawing modules, with one SVG and layout record per figure.
The earlier sketch style remains historical evidence.

### L29. Preserve meaning when changing a renderer or style

Compare entities, relations, branches, labels, keys and lifetime marks against the prior figure.
Check both the layout data and the rendered result.
A word-set comparison cannot establish position, multiplicity or visible text.

**Source:** Background `book-figure-viewing-check-k118` and `ravel-book-k101` in `figures.md`, 2026-10-17 to 2026-10-19.
Session evidence: `rhai-book-use-k72`, 2026-10-26.

**Ravel application:** Layout records separate geometry from drawing style.
The crisp restyle kept records unchanged. Later art checked semantic changes against the source and surrounding text.
Session `rhai-book-use-k77` found that Typst converted `--json` to an en dash in a figure.
Escape literal command punctuation and inspect the rendered label, not only the source.

### L30. Pin every input that can change a render

Pin the renderer, packages and fonts. Check their versions and digests before rendering.
Remove ambient fonts, package downloads and environment effects from the comparison.
Treat a fallback warning as evidence that the requested rendering did not occur.

**Source:** Background experiments recorded by `book-figure-retire-plantuml-k117` and `book-figure-viewing-check-k118` in `figures.md`, 2026-10-17.
Missing font families still rendered through fallback with exit 0.
**Ravel application:** `task setup` supplies the pins. `task docs:figures` renders offline from a private environment and fails on renderer diagnostics.
`task docs:figures:test` exercises corrupt pins, caches, imports and fonts.

### L31. Measure text where the reader sees it

Inspect the full figure and its key at chapter width, including every part of a tall figure.
Measure effective text size from the rendered width and viewBox.
Prefer wrapping and a taller sequence to shrinking labels or deleting explanations.

**Source:** Background `ravel-book-k101`, 2026-10-19, in `figures.md`. Session `rhai-book-use-k72`, 2026-10-26.
**Ravel application:** Figure mechanics target ordinary text near 12 pixels or larger at a 750-pixel column.
The full-size link supplies detail, but does not replace the chapter-width inspection.
The shared key needed a rendered measurement to expose padding that the layout measurement missed.

### L32. Keep figures understandable without selectable SVG text

Provide alt text, a caption, a prose explanation and a full-size link.
Keep these alternatives consistent with the figure's scope and omissions.
State book-specific conventions and departures from notation.

**Source:** Contract §3 and background `book-figure-viewing-check-k118`/`ravel-book-k101`, 2026-10-17 to 2026-10-19.
Session evidence: `rhai-book-use-k72`, 2026-10-26.
**Ravel application:** Typst writes glyph paths, so the figure cannot be the sole source of a fact.
The process-authority figure explicitly omits evaluation harness runs.
Cancellation arrows describe signal attempts rather than observed cleanup.

### L33. Report visual limits and size costs honestly

Name the browser, theme, viewport, page regions and full-size views actually inspected.
State the views that remain unchecked.
Record an exceeded benchmark as exceeded, with its accepted consequence.
Do not confuse compressed local bytes with measured network transfer.

**Source:** Background whole-book art, `ravel-book-k101`, 2026-10-19. Sessions `rhai-book-use-k72`/`k73`, 2026-10-26.
**Ravel application:** `figures.md` retains the glyph-path format despite the old 300 KB and 2 MB benchmarks.
Local Chrome observations do not establish GitHub rendering, all themes, new-reader comprehension or platform qualification.

### L34. Explain a decision to omit a visual

Record the reader question and what already answers it.
Distinguish an editorial choice from a restriction of the contract or a limitation of the medium.
Reconsider the choice when reader evidence shows that the existing explanation is insufficient.

**Source:** Background art records in `figures.md`, 2026-10-18 to 2026-10-19. Session `rhai-book-use-k72`, 2026-10-26.
**Ravel application:** Earlier passes retained tables instead of duplicate diagrams.
The owner's F2 and F5 later required new diagrams. That reader evidence governs the correction run.

## Prove what checks establish and collect reader evidence

### L35. Make every instrument demonstrate a failure

Check a known valid subject and a deliberately invalid subject with the same instrument.
For an absence claim, also find the class where it legitimately remains.
Do not trust a successful exit without checking what the command actually examined.

**Source:** Session evidence, `rhai-policy-k5`, `rhai-endpoints-removal-k64` and `rhai-book-use-k74`, 2026-10-25 to 2026-10-27.
Background figure porting records show a pattern that missed green arrows until a removed label exposed the omission.
**Ravel application:** Link, figure, digest and prose checks use planted failures.
Strict modes matter because the default book prose reports exit 0 despite findings.

### L36. Enumerate the whole claim before classifying matches

Use the scope of the claim rather than a remembered file list.
Include dotfiles, root files, help text and summary layers when they fall within that scope.
Do not truncate lines before matching or rely on a growing list of suspected words.

**Source:** Session evidence, `rhai-endpoints-removal-k64`, 2026-10-26.
Its first search cut lines at 200 characters and missed obsolete CLI help.
**Ravel application:** The later token inventory found that help text.
This file's audit enumerates every running log, then classifies all owner/human matches.

Proof `rhai-book-use-k87`, 2026-10-28, corrected its inventory pattern before it relied on the result.
The first pattern allowed letters and underscores, so it omitted the codes `source_not_utf8` and `env_not_utf8`.
Include every character that the subject's identifiers permit. Test the extractor with an identifier from each character class, including digits.
This is evidence about the inventory instrument, not a missing reference entry. Both codes were already in chapter 6.

### L37. Freeze inputs before measuring

Finish edits before a check. Give one measurement one writer.
Digest every input before and after, including manifests, fixtures, tools and instructions that the check reads.
Compare repeated runs item by item rather than comparing totals alone.

**Source:** Sessions `rhai-llm-removal-k63`, `rhai-book-use-k27`, `k70`, `k71`, `k73` and `k74`, 2026-10-26.
**Ravel application:** Book proofs recorded unchanged subject digests and per-example outcomes.
Source changes during a run require a fresh measurement.

Draft `rhai-book-use-k90`, 2026-10-28, found a digest that compared two errors. In zsh, an unquoted `$D` with several paths stays one word. Thus, `shasum` failed before and after the run, and `diff` reported no difference.
Read the digest lines before you trust their comparison. An array, such as `"${files[@]}"`, gives one argument for each path.

### L38. Keep an unexplained failure unexplained

Retain failed output before a cleanup removes it.
Do not assign a later discovered cause to an earlier failure whose bytes are lost.
Separate source observations, reproduced defects and unexecuted test designs in the report.

**Source:** Background lost selection render and later metadata-only failures in `figures.md`, 2026-10-16.
Session review records `rhai-try-boundary-limits-k47` and `rhai-measured-reads-k52`, 2026-10-25.
**Ravel application:** The figure evidence retains the lost-render uncertainty.
Review findings distinguish source-derived counterexamples from executed reproductions and hand verification to integration.

### L39. Check actual link destinations and image loading

Check local files and anchors, then check each external destination separately.
Use a missing page, anchor or image to show that the check fails.
Before you correct a page for a reported failure, open one reported link by hand. A false failure is a defect of the instrument, not of the page.
Inspect the visible image as well as its load status.

**Source:** Sessions `rhai-book-use-k27`, `k70`, `k71`, `k73` and `k74`, 2026-10-26 to 2026-10-27.
**Ravel application:** The Rhai address ending in `/book/language/` returned 404 despite appearing in a search result.
Read 1 checked nonzero image dimensions with an absent-image control.
The browser's hidden mdBook zoom copy does not count as a second visible figure.
Draft `rhai-book-use-k82`'s first link script reported every same-page anchor of chapter 5 as broken, because it resolved the page without its directory. The built HTML had each id.

Draft `rhai-book-use-k83` found that mdBook gives the heading “`--exit-dir DIR`” the id `--exit-dir-dir`, with the two dashes of the option.

Owner F13, `rhai-book-use-k74`, 2026-10-28, asks for a link when the book mentions Grove.
Link each page's introduction of an external project to the authoritative entry point that its reader needs, such as its documentation or repository.
Keep the local explanation of why the project appears.
The Grove repository is `https://github.com/Linkuistics/grove`. The session opened that destination successfully on the same date.
Draft `rhai-book-use-k88` links Grove in chapter 1, the only page of chapters 1–3 that names it.

### L40. Keep checker rules distinct from editorial judgement

Check what a rule actually governs. Preserve quotations, code identifiers and historical records as such.
Read advisories in context rather than changing meaning to remove them.
A checker needs an explicit scope and a test when that scope changes.

**Source:** Session `rhai-book-use-k69`, 2026-10-26.
The word `binding` appeared on a glossary `_Avoid_` line that names a rejected word.
**Ravel application:** `ste.md` records that the two word checks exempt those lines while other rules still apply.
The checker has a control and a mutation for that exemption.
Book-specific and general prose checkers cover different surfaces.

Draft `rhai-book-use-k88`, 2026-10-28, found that the general prose checker applies a departure marker to one line's scope.
A marker before a table covers only the header row. A problem in a later row stays reported. A reworded line moves the report to the next occurrence.
Put the marker at the reported line, with the reason, or reword where the meaning permits.

Proof `rhai-book-use-k78`, 2026-10-28, checked the figure layout records as
well as the chapters. The chapters passed, but the new figure labels still
had semicolons. The figure text also repeated a literal diagnostic whose
words the checker treated as prose. The correction kept the full diagnostic
in the chapter and put the concrete selection labels in the figure.
Apply the prose contract to every delivered medium, and preserve literal
output where the medium can identify it as code.

Proof `rhai-book-use-k96`, 2026-10-29, found a disallowed “-ing” word in the new installed-files key.
The chapter text passed, but the figure layout record failed. Include generated figure text in the strict prose gate after a label changes.

Session `rhai-executable-book-examples-k100`, 2026-10-28, applies this scope rule to an example checker's build precondition.
The retiring worker's digest and installed path blocked examples that used only the Rust binary.
Scanning all Rust files also treated a newer integration test as a stale production binary, although Cargo could reuse that binary.
Define freshness by the inputs of the executable that the example actually runs.
Check both sides: a newer production input must refuse, and an unrelated newer test must pass.
The controls use a scratch checkout with independent timestamps, so they do not alter the source they are measuring.
The fresh review found that a path pattern could include tests when the checkout had an ancestor named `src`.
Placing the scratch checkout under such an ancestor reproduced the false refusal.
Include the surrounding path in a control when a check uses path patterns; selecting source directories before traversal avoids this dependency.
The excerpt-checker controls also assumed that chapters 1–6 contained no excerpts after chapter 4 had gained one.
A control over a changing document should require valid excerpts without freezing their count.
Use a fixed fixture for the separate case of a document with no excerpts.

### L41. Keep reader evidence separate from mechanical and model evidence

Record the reader's words, location, progress, task performance and unreached material.
Label a stand-in reader and the limits of that substitution.
Stop at an early comprehension failure instead of treating a passed proof as a reason to continue.

**Source:** Background `book-read-one-k70`, 2026-10-16 to 2026-10-18. Owner agreement `rhai-book-use-k27`, 2026-10-26.
Session `rhai-book-use-k74`, 2026-10-27 to 2026-10-28.

**Ravel application:** Contract §4 requires both reads after proof.
The owner completed Part I, chapters 1–3, on 2026-10-28 and supplied findings F9–F16.
The owner reported no Part II progress, practical task performance or viewing surface.
It establishes no result for a developer new to Ravel.

### L42. Apply the pattern of a finding across its agreed scope

Extract the underlying explanation problem before correcting the quoted passage.
Check each page for the same problem.
Route corrections to the stage that owns them and finish the correction pipeline before the next dependent read.

**Source:** Owner, `rhai-book-use-k74`, 2026-10-27: “apply the patterns of these re-statements”.
The owner later extended those patterns to the entire book and chose “Parts I–II first”.
**Ravel application:** Contract §2 states the patterns from the owner's findings.
`rhai-book-use-k75` through `k78` correct chapters 1–6. `rhai-book-inside-k34` applies them to chapters 7–11 before Read 2.
Read 1 remains live for the re-review.

On 2026-10-28, the owner first chose to collect the remaining Parts I–II notes before one combined correction.
After the Part I review, the owner changed that choice: apply all lessons before the Part II read.
The second correction run now precedes that read. Keep the earlier choice as history and make the latest choice explicit in the handover.

## Communicate decisions and hand work to the next session

### L43. Record the owner's words separately from your interpretation

Quote the relevant words and date the decision.
Label what the session inferred, what the owner explicitly chose and what remains unknown.
Give changed requirements a durable home where future producers will read them.

**Source:** Sessions `plan-k1`, `llm-calls-through-harnesses-k58` and `rhai-book-use-k74`, 2026-10-24 to 2026-10-27.
**Ravel application:** The issue amendment, contract and running logs distinguish accepted choices from session readings.
“The result” read as “the rest”, tone within STE and the figure-key distinction remain interpretations.
L46 below assesses the proposed causal reading separately.

### L44. Make a question answerable and preserve its settled answer

State the trade-off, recommend an answer and give the evidence for it.
Ask for missing information that affects the decision.
Carry the answer into dependent artifacts instead of asking the same question again without new evidence.

**Source:** Session evidence, `plan-k1`, `rhai-llm-upgrade-proof-k21` and `llm-calls-through-harnesses-k58`, 2026-10-24 to 2026-10-26.
The abandoned upgrade proof records a second session asking again after the owner chose a design change.
**Ravel application:** Design task `rhai-policy-k2` explicitly consumes settled requirements without another interview.
The harness design states rejected alternatives and their consequences.
The repeated question supplies the lesson, not a claim that the owner endorsed it.

### L45. Give a handover enough context to act independently

Name the artifact, stable handle, source revision, concrete defect, owner and required check.
State accepted dependencies and the work that remains live.
Use the existing document pipeline when one already owns the whole document.

**Source:** Sessions `rhai-policy-increments-k40`, `llm-harness-replan-k60` and `rhai-book-use-k27` through `k78`, 2026-10-25 to 2026-10-27.
**Ravel application:** Task bodies carry specific findings and checks. Node briefs carry cross-chapter handoffs.
The split draft retained one copy-edit/art/proof pipeline.
Shared figure changes also identify the later chapter whose alternative becomes stale.

### L46. Report a causal claim only as strongly as its evidence permits

Separate an observed defect from a theory about why it arose.
Keep a useful hypothesis visible without presenting it as an owner decision or a measured cause.

**Source:** This session's assessment of `rhai-book-use-k74`, 2026-10-27.
**Ravel application:** Accept that short, compliant prose omitted explanations and that F1–F8 expose those omissions.
Reject “STE alone caused compressed, abstract prose” as an established cause. No controlled comparison supports “alone”.

The old “Terms” and “Stock phrasing” rules did encourage page dependence, as their text and F7 show.
Both recorded reads followed passing proofs and found comprehension problems. That supports L41, not a measured failure rate for all checks.

### L47. Keep the working copy and durable record reviewable

Inspect new tool output files before the commit and move incidental browser artifacts outside the working copy.
Use a focused commit message that names the task and the delivered change.
Record decisions when they settle rather than reconstructing a session narrative at the end.

**Source:** Session history from `plan-k1` through `rhai-book-use-k74`, 2026-10-24 to 2026-10-27.
The `documentation-lessons-k79` task lists a browser-written snapshot as a candidate lesson from k74.
**Ravel application:** The task's proposed snapshot lesson is precautionary session evidence, rather than an owner quotation.
The current commit history ranges from concise subjects to subjects that repeat long implementation inventories.
Put detailed evidence in task records, and preserve a useful subject and focused explanation for the next reader.

### L48. Capture lessons as they arise and keep later decisions current

Add each new documentation or communication lesson to the shared record when it arises.
Give it a transferable rule, a source, an evidence class and a concrete application.
Mark obsolete decisions beside their replacements.

**Source:** Owner, `rhai-book-use-k74`, 2026-10-27:

<!-- tc: off because these are the owner's quoted words -->
> By learnings I mean *all* the things about documentation and communication that we have learnt in this grove.
<!-- tc: on -->

**Ravel application:** The root brief and book contract require later sessions to extend this file.
The correction tasks and both later drafts repeat that duty.
The skill remains an “On the horizon” item until the owner chooses its home.

On 2026-10-28, the owner reiterated: “Remember that we want to generalise this feedback to construct a skill”.
The record must supply reusable authoring decisions, their scope and ways to check them, as well as the original observations.
The opening paragraphs state how to distil those decisions. The final skill covers the whole grove's lessons, including later reader findings.

### L49. Preserve source rights when recording a rule

State borrowed rules in your own words and cite their source.
Keep a source's distribution restriction distinct from permission to consult it.
Record the licence and the reason for fetching or retaining third-party material.

**Source:** Background `book-read-one-k70` and renderer records, 2026-10-17 to 2026-10-18, in contract §2 and `figures.md`.
**Ravel application:** The contract records the owner's local STE PDF, its digest and its distribution restriction.
It permits paraphrased rules with their numbers rather than committing the PDF or its text.
The contract also attributes the retained Google-style points and names their CC BY 4.0 licence.
The figure note records why the project fetches CeTZ and the licence of that dependency.

### L50. Use invented examples and explicit permission for live calls

Preserve the useful shape of an example without copying personal data or credentials.
Use dummy values to test sensitive output paths.
Before a live call, state its purpose and cost and ask for the required permission.

**Source:** Sessions `rhai-example-policies-k25`, `rhai-llm-call-scenarios-k67` and `rhai-harness-smoke-k66`, 2026-10-26.
The owner approved “Yes, one call per harness”.
**Ravel application:** Contract §1, “Examples policy”, keeps default examples on fakes.
The owner-shaped configuration script uses invented programs. The credential probes share a sentinel and read no real sign-in.
The real-call record preserves only the agreed answer shape and the information needed to reproduce the recipe.

### L51. Separate a prose conversion from a change of meaning

Convert existing prose to the agreed writing rules before the semantic edit.
Keep the conversion in its own change so a reviewer can assess meaning without unrelated wording changes.
For a large document, state which sections remain outside that conversion.

**Source:** Session commit messages `01654277d4f6`, 2026-10-24, `f24c5b3cc856`, 2026-10-25, and `3e9ee1002e42`, 2026-10-27.
Their task handles are `rhai-policy-k2`, `llm-calls-through-harnesses-k58` and `rhai-book-use-k74` respectively.
The first says that the separate change makes the later design diff show only meaning.
**Ravel application:** The glossary, job ADR, policy spec, root brief and contract prose section received separate conversions before semantic edits.
The prose checker and a read of each conversion diff establish the boundary.
Historical records and owner quotations retain their words.

### L52. Give real values before the reader can run the example

On a page before the reader can run the product, show real output as a labelled fragment of a recorded run.
Record the build, the date and the environment of that run in the task.
Choose environment values that keep the output short, and say in the label which parts change in each run.

**Source:** Session evidence, `rhai-book-use-k80`, 2026-10-27.
Contract §2, “Give a diagram and a concrete example”, asks for real values, and chapter 1 comes before the installation in chapter 3.
**Ravel application:** Chapter 1 shows the routing notice, the JSON `selection` member, two nested `RAVEL_EXIT_FILE` paths and the `end-session` failure from runs of the checkout build.
The nested run used `TMPDIR=/tmp`. Its label names the random parts of each path and the `/private/tmp` path that macOS shows.

### L53. Put the new explanation in the first draft of a correction run

When a reader finding applies to every page, size each draft by the pages that it reads in full.
Put the page that introduces the new explanation in the first draft.
Later drafts then restate that explanation in the same terms.

**Source:** Session evidence, `rhai-book-use-k75`, 2026-10-27.
Chapters 1–6 had about 4,870 lines, and chapter 4 alone had about 1,500 lines.
**Ravel application:** Draft `rhai-book-use-k80` takes Part I, because chapter 1 holds the new explanations of Rhai, a selection, the routing notice and `RAVEL_EXIT_FILE`.
Drafts `rhai-book-use-k81`, `rhai-book-use-k82` and `rhai-book-use-k83` take chapters 4, 5 and 6. Their task bodies name the passages that restate chapter 1.

### L54. Give a design reason only from its record

When an explanation needs the reason for a rule, find that reason in the specification or a decision record.
If no record states a reason, state the difference and its effect for the reader.
Do not write a likely motive as the design's reason.

**Source:** Session evidence, `rhai-book-use-k81`, 2026-10-28.
Three planned reasons in chapter 4 had no record.
They concerned the two bases of imports and measured reads, the capture of policy output and the prompt on standard input.
**Ravel application:** Chapter 4 says that an import and a measured read can find two different files from one relative name. It says that the arguments of a recipe stay the same for each prompt.
The draft's decision log names the specification sections that it read.

Draft `rhai-book-use-k83` removed its own motive for the reserved grant classes of chapter 6. The text now gives the description in the source: controls of a language runtime, of the dynamic loader or of Ravel.

### L55. Claim the order of two outputs when the order teaches the point

When the text explains that one output comes before another, make one claim cover both outputs.
A claim of two adjacent lines fails when their order changes. Two separate claims do not.
Show an order that the check cannot claim in a labelled fragment of a real run.

**Source:** Session evidence, `rhai-book-use-k82`, 2026-10-28.
The text routing notice precedes the launch, and the JSON notice follows the supervision attempt. Chapter 5 teaches that difference.
**Ravel application:** The notice example of chapter 5 claims the text notice and the fake agent's next line as one block. A changed character in either line failed the check.
The JSON notice holds values that change in each run, so a fragment of a real run shows the agent's line before the JSON document.

### L56. Inspect the rendered page after an edit beside a code block

An edit that ends inside a line can join the rest of that line to new text.
After a closing code fence, that join leaves the fence open, and the page shows the following prose as code.
Prose, break and example checks then read less text and still pass.
After such an edit, search the built page for raw Markdown, such as a fence or a heading marker.
Then compare the heading ids with the headings.

**Source:** Session evidence, `rhai-book-use-k83`, 2026-10-28.
**Ravel application:** In chapter 6, “`--json`”, a new fragment ended where an old sentence continued on the same line.
The built page showed the rest of the entry as code, and the heading “`end-session RESULT`” had no id. The STE, break and example checks passed.

The link check found the missing anchor.
The draft then searched the built HTML for lines with a fence, a heading marker or an escaped `tc` comment as their first text.
That search found the leak. After the edit that closed the fence, the same search found nothing.

### L57. Teach one concrete case before the general model

Give a particular task and real values first. Follow its actions to the result.
Then name the general roles and explain which parts apply to other cases.
Keep the same example through that transition, so the reader can connect each abstract statement to a known case.

**Source:** Owner F9, `rhai-book-use-k74`, 2026-10-28.
The owner explicitly preferred a concrete explanation followed by abstraction or generalisation.
**Ravel application:** Contract §2 now makes this order a whole-book requirement.
Draft `rhai-book-use-k88` opens chapter 1 with “Follow one code review from request to result”, before the roles.
The next section, “What the review shows about every session”, maps each general role to its part in the review. Chapter 2 opens with the same review.

Draft `rhai-book-use-k89`, 2026-10-28, applied the rule to a general model in a later chapter. Chapter 4's order of an evaluation now follows chapter 1's review request, with its real kind, program and routing notice.
Then one sentence says that every evaluation follows the same order. The interactive section carries the same review to its results, `approved` and `changes-needed`.

Reuse the earlier concrete case where a later page generalises it. The reader then meets known values, not a new example.
Explain each term that the concrete trace uses before the trace, even when a fuller explanation follows it.

Draft `rhai-book-use-k90`, 2026-10-28, opened chapter 5 with chapter 1's review: its command, its result, its status and the orchestrator's next action.
The chapter's term table and order table then have a column “In the review”. Each general word and each row keeps a known value beside it.
A column of the earlier case's values connects a general table to that case without a second explanation.

Draft `rhai-book-use-k91`, 2026-10-28, applied the rule to the lookup chapter. Chapter 6 opens with chapter 1's review request and its result, then gives its word table with the column “In the review”.
Its fragments now show the review's prompt file and result. Thus, a reader who enters at any entry meets the same values as in chapters 1 and 5.

### L58. Explain the task, its completion instruction and the use of its result together

When the reader needs to understand a caller's complete task, naming only the requested work leaves the result unexplained.
Show the instruction, the possible results, the condition for each result and the caller's next action.
Distinguish the task's chosen result words from meanings that the underlying tool enforces.

**Source:** Owner F9, `rhai-book-use-k74`, 2026-10-28.
**Ravel application:** The proposed review example names files and tells the LLM when to send `approved` or `changes-needed` with `ravel end-session`.
The explanation then says what the orchestrator does after each result. These two words are the session's proposed example, not the owner's required vocabulary.

Draft `rhai-book-use-k88` shows the prompt file, a table of its four parts and a real run with a fake agent.
A table then gives the orchestrator's action for each outcome. The outcomes also cover an unexpected value and a failure status.

Draft `rhai-book-use-k90`, 2026-10-28, made the caller's side executable in chapter 5.
A complete example runs an orchestrator script. The script checks the status, reads the result and requests a `fix` session for `changes-needed`.
When the example's command is the caller and not the product, say whose output the output block shows. The block then shows the orchestrator's lines, not Ravel's result stream.

### L59. Prefer familiar terms and qualify them where precision matters

Use the reader's familiar term when it describes the concept accurately.
Add a qualifier when two different artifacts would otherwise share one name.
Reconcile the glossary and dependent terms when the preferred name changes.
Keep literal interfaces exact, unless the product change also owns those interfaces.

**Source:** Owner F10, `rhai-book-use-k74`, 2026-10-28, prefers “configuration” to “owner policy”.
**Ravel application:** Contract §2 requires that replacement in reader-facing prose.
The session proposes “configuration script” for the Rhai file and “settings” for the JSON values.
The proposal preserves the distinction between executable choices and host limits.
Draft `rhai-book-use-k88` adopted those words, with “Rhai worker” and “selection context”. L63 gives the reasons and the scope.

### L60. Put format versions where the reader needs them

Do not attach a version label to each mention of a familiar data format.
Name the document's purpose in the explanation. Put exact version fields in examples and in the entry that teaches their required values.
This removes incidental detail without changing the interface that the reader must use.
Draft `rhai-book-use-k88` recorded the rule in `ste.md`, “JSON documents”. Chapters 1–3 now say “JSON routing notice” and “JSON data”.

**Source:** Owner F11, `rhai-book-use-k74`, 2026-10-28.
**Ravel application:** The next correction removes incidental “version-1 JSON” and “version-2 JSON” labels from prose.
Exact `schemaVersion` values remain in executable examples and the reference.
This boundary is the session's interpretation of the request, not a request to change the JSON protocol.

### L61. Use headings that state their subject in familiar words

A heading must help the reader predict the subject, question or outcome of its section.
Name that subject in familiar words. Avoid unusual shorthand or metaphors that need the section's explanation before they make sense.
For a section about limits, name the relevant checks and their scope.
Read headings without their body text to check whether their subjects remain clear.
When a heading changes, reconcile its contents entry and inbound links as part of the same edit.

**Source:** Owner F14 and F15, `rhai-book-use-k74`, 2026-10-28.
**Ravel application:** The chapter 1 section covers platform checks, reader observations and publication surfaces.
The session proposes “See what we tested and what remains unchecked”. The draft owns the final title and its links.

The owner also found “The actors and their words” unusual and called “their words” jarring.
The session proposes “Who does what in a Ravel session” for chapter 2.
Both examples support descriptive headings. They do not impose one grammatical form on every document.

Draft `rhai-book-use-k88` adopted both proposals. It also renamed “Policy evaluation and supervision” and three other headings for the new terms.
It changed every inbound anchor in the book and the spec's citation of the chapter 4 title. A link check with a planted old anchor showed the change.

### L62. Name a familiar command instead of an abstract paraphrase

When the stated reader already knows a tool, use its ordinary command name to give a precise instruction.
Check the prerequisite agreement before treating the command as familiar.
Keep the action, its object and any required condition clear. Explain a command when the reader does not yet know it.
Plain language uses the reader's known vocabulary. It need not replace precise commands with longer descriptions.

**Source:** Owner F16, `rhai-book-use-k74`, 2026-10-28.
**Ravel application:** Chapter 3 says “then make it the current directory”.
The owner prefers “and `cd` into it”. Everyday shell use is a stated prerequisite, so the command makes the instruction more direct.
The correction belongs to the draft and copy-edit stages.
Draft `rhai-book-use-k88` writes “Create an empty directory for the example and `cd` into it.”

### L63. Reconcile a term across prose and product names

When a name that the reader sees changes, find every form that names the concept. Include its short form.
In Ravel, the bare word “policy” named the same concept as “owner policy”. A change of the phrase alone would give one concept two names.

Choose each qualified term in one shared record before any chapter changes.
Then each later draft uses the same words.
For a prose-only change, keep literal file names, options, codes and help text exact. Explain a retained name where necessary.
When the owner also requests a product rename, coordinate that implementation with its documentation and remove obsolete retained-name explanations.

Route each other surface to its owner. A glossary, a specification and command help can have different owners.


**Source:** Session evidence, `rhai-book-use-k88`, 2026-10-28, after owner F10.
The owner's words were “configuration” in place of “owner policy”. The qualifiers are the session's choice.

**Ravel application:** “Policy” occurred about 1,250 times in chapters 1–6. `ste.md`, “Configuration words”, and `CONTEXT.md` hold the term map.
The CLI option `--config` already matched the new term. `rhai-spec-reconciliation-k33` owns the specs, ADRs and README.
The CLI help is part of the product interface. `rhai-book-inside` owns chapters 7–11.

Owner F18 later requested the rename in filenames and arguments. `configuration-names-k92` owns the product change before the next book correction.

**Check:** Search each changed page for the old words. Classify each match as a literal, a quotation, a record of the decision or an omission.

Draft `rhai-book-use-k89`, 2026-10-28, applied the map to text that the reader sees in an example's output. A refusal `message` in an example script appears on standard error. Thus, it is prose, and it follows the map. The expected output of the example changed with it.
That draft retained Ravel's own diagnostic words, `VERSION` strings and labels as literals or identifiers.

The later product rename changes Ravel-owned names. It does not rewrite arbitrary owner-supplied data.

Session `configuration-names-k114`, 2026-10-29, extends this rule to comments and overloaded words.
Classify what each occurrence names before replacing it. An operating-system cancellation policy is not an owner configuration script.
A comment that names a historical JSON field or a deliberately rejected filename must retain that exact identifier.
Check both the remaining old tokens and the new replacement phrases: an old-token search cannot find a wrong new phrase.
The session reviewer found these errors in comments, the reference opening and a cancellation table; the mechanical corrections retain the existing contract.

**Additional session evidence:** `configuration-names-k113`, 2026-10-29,
changed diagnostic codes and JSON fields. Retained-name explanations in
chapters 1, 2 and 5 then became false, including an example that used a new
code to illustrate the old word. The implementation removed those explanations
and reconciled the field names in summaries, tables and executable output.
CLI checks carry old words through owner-supplied values unchanged. The frozen
comparison translates named fields and exact known messages, rather than
replacing words throughout the captured data.

Draft `rhai-book-use-k118`, 2026-10-29, found two old-name explanations that the rename had made false. Each contained only new tokens.

Chapter 6 said that the words “the configuration script” in a diagnostic are Ravel's words for the configuration script. Before the rename, the diagnostic said “policy”.
It also said that Ravel does not select `config.rhai` as a default. Before the rename, the default was another file. Now `config.rhai` is the default.

An old-token search cannot find such a sentence. Search for a sentence that names the new term twice, or that denies a property of the new name.

The renamed headings of chapter 4 changed 102 inbound anchors in four other chapters. The draft changed only the anchors there. The link texts that quote old titles belong to the drafts that own those chapters.
**Check:** Search the old anchors in the whole repository. Run the same search on the parent revision as a control.

### L64. Run a recorded fragment again when its example changes

A fragment of a real run is not executed by the example check. Thus, a change to the example's files can make the fragment false, and every check still passes.
When you change an input that a recorded fragment shows, run the example again. Then copy the new values into the fragment and record the run.
Values such as a byte count, a digest, a duration or a source path change with the input.

**Source:** Session evidence, `rhai-book-use-k90`, 2026-10-28.
**Ravel application:** Chapter 5's notice example changed its refusal message and its prompt source for the review.
The real-run fragment of the JSON routing notice still showed the old script's byte count, its digest and `"promptSource": "argument"`. A new run gave the values that the fragment now shows.

**Check:** For each changed complete example, search the same section for a fragment that names a real run. Compare each value that depends on an input.

Draft `rhai-book-use-k91`, 2026-10-28, found the same defect in another chapter. Chapter 6 quoted chapter 5's review run in four fragments: the command, the script, the JSON routing notice and the failure document.
Draft `rhai-book-use-k90` changed that run and checked its own section. Chapter 6 still showed the old prompt, the result `continue`, 605 bytes and `"promptSource": "argument"`.
A reference page often quotes the examples of the teaching chapters. Thus, the search for a stale fragment covers the whole document, not only the changed section.

**Check:** Search the whole document for each name of the changed example. Search for its file names and its result too, for example “the review session of chapter 5”, `review-prompt.txt` and `changes-needed`.
Compare each fragment that a search finds.

### L65. Make a link text agree with what its destination now says

A link text names or describes its destination. When the destination changes, the link text can become false while the link still resolves.

After a heading changes, give each inbound link the new heading. Alternatively, name the part that it points to, such as a row of a table.
Replace a generic label that many links share, such as “Policy behaviour”, with the subject of each destination. The reader then knows where each link goes before following it.
When the text says that the destination describes something, read the destination and check that claim.

**Source:** Session evidence, `rhai-book-use-k91`, 2026-10-28.
**Ravel application:** Chapter 6 had six links to a renamed chapter 5 section with five different texts, such as “operational limits” and “platform limits”. Each now names the section or its row.

A note said that the linked section describes two cases. That section describes only one of them. The note now links each case to its own section.
Many table rows of chapter 6 used the label “Policy behaviour” for eight different sections of chapter 4. Each label now names its section.

**Check:** For each changed heading, list the inbound link texts. Classify each as the new heading, a named part or a stale description.
For a link text that states what the destination contains, read the destination.

The following checks apply the rules where a judgement has no standalone executable check.

| Rules | Contract section or check in Ravel |
|---|---|
| L01–L11 | Contract §1 and §2 define the reader and explanatory patterns. The owner's reads under §4 test the explanations. |
| L12–L18 | Contract §2 governs prose and terminology. Proof compares each behavioural summary with the normative documents named in §1. |
| L19–L25 | Contract §1 requires coverage inventories, runnable examples and exact excerpts. The example and excerpt tasks check their mechanics. |
| L26–L34 | Contract §1 names figure needs. Section 3 governs media. Figure comparison and visual inspection check different properties. |
| L35–L42 | Contract §1 requires coverage evidence. Section 4 separates reader evidence. The named checks need controls and unchanged inputs. |
| L43–L48 | The source audit below checks attribution. A read of task bodies and `jj diff` checks handoffs and incidental artifacts. |
| L49–L51 | Contract §2 records source restrictions. The prose checker and separate conversion diffs check writing changes before semantic edits. |
| L52–L53 | The draft's running log records each run that a fragment quotes. The node brief names each draft's pages and the passages that restate earlier ones. |
| L54 | The draft's decision log records the source of each reason that the text gives. Proof compares each reason with the normative documents. |
| L55 | The examples check compares each multi-line claim byte for byte. Proof reads each order that the text states against a claim or a labelled fragment. |
| L56 | A search of the built HTML for raw Markdown, and a count of headings against heading ids, follow an edit beside a code block. |
| L57–L62 | Contract §2 records the Part I findings of 2026-10-28. Draft applies them throughout the book, and proof checks the explanations, exact interfaces and links. |
| L63 | A search of each changed page for the old term, with each match classified. `ste.md` and `CONTEXT.md` hold the term map. |
| L64 | A search of the whole document for real-run fragments of each changed example, and a new run when an input that it shows changed. |
| L65 | A list of the inbound link texts of each changed heading, each classified, and a read of each destination that a link text describes. |
| L73 | For each observation of a demonstration, the value that the other role would give. |
| L74 | A list of the link targets outside `docs/book/src/`, each resolved from its page and checked against the built site. |
| L75 | For each complete example, the files that the product reads by the location of its executable, and the claimed lines that they can change. |
| L76 | For each excerpt and each named function of a walkthrough, the chain of calls from the entry point that reaches it. |
| L77 | For each question that a walkthrough answers about a runtime value, a claimed line of a complete example that shows the value. |

### L66. Explain why unfinished work must stop

Before stopping unfinished work, state the condition that prevents further progress, its evidence, and the work that remains.
An incomplete task explains why it is not complete. It does not explain why the session cannot continue.
Record the explanation where the next session can find it. A crash can prevent the explanation, so retain independent exit evidence too.

**Source:** Owner question in `rhai-book-use-k85`, 2026-10-28, after two copy-edit sessions requested `grove-force-stop` with incomplete reads.
The visible transcripts stated that the reads were incomplete but did not say why the sessions could not continue. The owner asked for a way to debug that decision and requested the added instruction for this continuation.

**Ravel application:** A future force-stop explanation names the preventing condition, evidence, remaining work and why continuation or decomposition cannot resolve it. The task's running log preserves it.

**Check:** Read the explanation without the preceding conversation. It must distinguish the observed condition from an inferred cause and identify the next action.

### L67. Make setup visible and check the reader's sequence

When a tutorial asks the reader to prepare an example, name that example.
Show its workspace, file paths, contents and complete command.
Explain the purpose of each file. Distinguish what the reader creates from what an execution command creates.
Show the complete command before a table explains its parts. Explain differences between examples where they occur.

A tutorial check must account for the preparation that its runner supplies.
Associate each reader action with an instruction already encountered or a declared prerequisite.
Record an unsupported inference, missing tool or missing input instead of silently supplying it.
Preserve state between dependent steps and reset only where the tutorial specifies a reset.

**Source:** Owner finding F17 and the owner's intended TestSubject use, `rhai-book-use-k74`, 2026-10-28.
The owner reported that chapter 4's setup section did not identify the files and that its table did not make sense.
The owner wants TestSubject to execute tutorial steps in the specified environment and expose undeclared tools or expertise.
They clarified that it must check all instructions, prerequisites and context, and that an LLM reads the tutorial and decides what to do.

**Evidence:** The chapter's file blocks and complete command occur after the setup table.
Direct inspection of `scripts/examples.sh`, `run_example`, shows automatic parent-directory creation, file copying and executable permissions before any example command.
These mechanics are declared in `docs/book/examples.md`. They do not test whether a reader was told how to prepare those inputs.

**Ravel application:** Contract §2 requires a concrete setup sequence throughout the book.
Chapter 4 is a proposed first TestSubject case. TestSubject itself is still pre-development.
Its intended learner is an LLM that reads the actual prose and uses tools to perform the steps.
A parser, hidden fixture or prewritten action sequence must not supply the understanding or preparation that the tutorial is meant to provide.

The LLM can use declared prerequisite knowledge and explicitly referenced material. It must expose unsupported assumptions instead of silently correcting the instructions.

The first bounded LLM trial completed the example and found no material information gap, although the owner had reported confusing setup.
It used the later file blocks and the declared shell prerequisites. Its execution result was correct, but it did not identify the owner's context complaint.
Thus, procedural executability and instructional sufficiency need separate judgements. One model's successful derivation does not settle the latter.

The trial also read beyond its stopping boundary during a later citation lookup. Supply a physically bounded corpus when that boundary matters.

Proof `rhai-book-use-k96`, 2026-10-29, used physically bounded copies for a second trial in a separate leaf.
The LLM ran the revised first example once and reported no instruction gap. The supervisor checked the captured result and unchanged inputs separately.
The reader's first report described standard error before it read the captured output. Its unedited report and separate correction preserve that error.
Read each observation before reporting it. A plausible diagnostic is not evidence of the actual stream.
This trial supplies one model judgement and one execution result. It does not replace the owner's resumed read or establish novice comprehension.

**Check:** Follow the visible instructions from the declared initial environment, recording the source of every action and the resulting state.
Prove that a validator notices a missing setup step, a reordered dependency, an absent required tool and an incorrect output claim.
Separate observed execution from model judgements about prerequisite knowledge and from human-reader evidence.

Draft `rhai-book-use-k115`, 2026-10-29, applied the rule to chapters 3 and 4.
Each example now has a named directory, a tree of its paths, and a table of who makes each path and why. Numbered steps and a labelled fragment for `mkdir` and `cd` follow.
The table of the command's parts now follows the complete command. Each later example of chapter 4 has a paragraph that names its directory and its differences from the first example.

That draft also followed the visible steps in zsh, the default macOS shell. It saved the files without the executable bit, as a text editor does, and ran each example's blocks in one shell process.
Twelve of 13 examples matched exactly. The other differed only in the `<scratch>/work` substitution, which the page now explains.

A control that omitted `chmod` failed with status 126. This mechanical walk shows that the visible steps are executable. It is not evidence that a reader understands them.

Draft `rhai-book-use-k116`, 2026-10-29, applied the rule to chapter 5, where each example belongs to a different section.
The setup section lists the eight examples, their directories and the files to save. The tree, the path table and the command-part table stay at the first example, because its files need that section's explanation.
A reader can enter chapter 5 after chapter 3, so the section restates chapter 4's steps instead of a link to them.

The same walk ran the visible steps of all eight examples, and each matched exactly. The control without `chmod` failed seven of them.
It could not fail the confinement example, which has no `bin` directory. Choose a control that reaches each subject, or name the subjects that it cannot reach.

### L68. Write each example command for the reader's shell

An example checker often runs each block in a noninteractive shell. A reader types the same lines into an interactive shell, where some commands behave differently.
For example, `|| exit "$?"` stops a failed `sh -c` block early. In a reader's interactive shell, the same `exit` closes the shell.

Write each command so that it is safe and correct when the reader types it. Keep the checker's failure detection through the expected output and status.
Name the reader's shell when its behaviour matters, such as a continuation prompt for a multi-line loop.

**Source:** Session evidence, `rhai-book-use-k115`, 2026-10-29, during the F17 correction.
**Ravel application:** Chapter 4 removed `|| exit "$?"` from eleven preparation lines. Its interactive-routes loop now uses `|| break`.
A failed preparation still changes the next command's output, so the check still fails.

**Check:** Run the visible commands in the reader's default shell, after only the preparation that the page states. Then plant one omitted step and see that run fail.

### L69. Check a qualifier against the source, because it implies its complement

A qualifier limits a statement, and the reader then infers the opposite for the excluded case.
Thus, “never reads X from a terminal” tells the reader that Ravel reads X from other sources.
Check each qualifier against the source. Remove it when the behaviour has no such limit.

**Source:** Session evidence, `rhai-book-use-k116`, 2026-10-29.
**Ravel application:** Chapter 5 said that Ravel “never reads a prompt from the standard input of a terminal”.
`crates/ravel/src/inputs.rs` never reads a prompt from standard input at all, so the chapter now says that. The specification `configuration-evaluation.md` has the same narrow words, “terminal stdin”.

**Check:** For each “never”, “only” or “always” statement with a qualifier, find the code that enforces it. Read whether the code has the qualifier too.

Draft `rhai-book-use-k117`, 2026-10-29, found the same defect in a fragment label without a qualifier. Chapter 1 said that Ravel “does not read the words” of the prompt file. Ravel reads the file and gives its text unchanged to the configuration script. The label now says that Ravel does not interpret the text.

Proof `rhai-book-use-k96`, 2026-10-29, found the same false label in chapter 5's repeated review prompt.
The chapter 1 correction had not reached that copy. Distinguish a byte read from semantic interpretation in every repetition, including fragment labels.

### L70. Say when a command is not yet for the reader to run

A conceptual page before the installation can show commands as fragments. A reader who sees a command can try to type it, and the command then fails for a reason that the page does not give.
Near the top of the page, say that it has nothing to run. Name the first procedure that the reader runs.
At a command that looks complete, say what it needs before the reader can type it.

**Source:** Session evidence, `rhai-book-use-k117`, 2026-10-29, during the F17 correction.
**Ravel application:** Chapters 1 and 2 now say that each code block is a fragment. They name chapter 3, “Run the first session”, and its directory `~/ravel-first-session`.
Chapter 1's review request now says what it needs: an installed Ravel, the prompt file and a script with a rule for `review`.
Chapter 2 says that `ravel end-session changes-needed` fails with status 1 in the reader's own shell, because that shell has no result path.

**Check:** Read each command block of a page that comes before the installation. For each block, find the sentence that says whether, where and when the reader runs it.

### L71. Name the inputs of a real run when its values differ from the story

A real-run fragment can replace a part of the story that the book cannot run. For example, a fake agent replaces a real harness.
Its output then shows values that differ from the story. Say which inputs produced the run, and say which value each input changed.

**Source:** Session evidence, `rhai-book-use-k117`, 2026-10-29.
**Ravel application:** Chapter 1's review story says that the configuration chooses Claude Code. The real run's routing notice names the provider `local` and the model `fake`.
The text now says that the run used the chapter's own short script at its default path, and that this script selects a fake agent.
A label that said “the same session” for a separate `--json` run now says “the same request, run again with `--json`”.

**Check:** For each real-run fragment, compare each value in it with the story around it. Where a value differs, find the sentence that names the input that changed it.

### L72. Derive a reference list again from the code after a component is removed

When a component leaves the product, a reference list can keep the entries that the component needed.
A stale restriction does not fail an example, because no example uses the name that it refuses. Thus, every check still passes.
After a removal, find the code that defines each list in the reference. Compare the list with that code, item by item.

**Source:** Session evidence, `rhai-book-use-k118`, 2026-10-29.

**Ravel application:** Chapter 6 listed nine classes of names that a grant cannot use. They included `BUN_*` and four `NODE_*` names of the removed Bun worker.
`excluded` in `crates/ravel/src/environment.rs` refuses only `LD_*`, `DYLD_*`, `RAVEL_*` and `HARNESS_DISPATCH_*`. Its unit test asserts that the `BUN_*` and `NODE_*` names are grantable.
The help and the specification already named the four classes. The chapter now names them too.

The same read found that the installed-layout table lacked the notice file that the installation now places.

**Check:** For each list in a reference entry, name the source that defines it, such as a function, a constant or a manifest. Compare each item. A test that asserts the complement, such as names that stay grantable, is direct evidence.

### L73. Observe the property that tells two roles apart

A demonstration that two roles are separate must observe a property that differs between them.
When one executable has two roles, its name and its program path are the same in both. Such an observation cannot show which role acted.

**Source:** Session evidence, `ravel-book-inside-k329`, 2026-11-11.
**Ravel application:** Chapter 7's example shows that the Ravel CLI, not the Rhai worker, starts a harness run.
`ps -o comm=` gave `ravel` for the parent. The worker is the same binary, so that value would be the same if the worker were the parent.
The fakes now print `ps -o args=`. The Ravel CLI shows `ravel start-session`, and the worker shows `ravel-rhai-config-worker`.

**Check:** For each observation that a demonstration uses, name the value that the other role would give. If the two values are equal, choose another property.

### L74. Name a repository file outside the book instead of a link to it

A relative link from a book page to a file outside the book's source tree works in a checkout and on GitHub.
The built site does not hold that file, so the same link fails there.
Name the file and its repository path in code font. Keep the explanation on the page, so that the reader need not open the file.

**Source:** Session evidence, `ravel-book-inside-k329`, 2026-11-11.
**Ravel application:** The first draft of chapter 7 linked three decision records with `../adr/…`. That path was also wrong: the records are in `docs/adr/`, two levels up.
Chapter 6 already named a specification by its path, without a link. Chapter 7 now names each record and its path in the same way, and it restates each reason.

**Check:** List each link target outside `docs/book/src/`. Resolve each one from the page's directory. Then decide whether the built site holds it.

### L75. Find the example inputs that the checker's build does not have

An example check runs one build in one environment. A reader runs an installed product in another.
An input that only the installation has can change the reader's output while the check passes.
For each example, list the inputs that the product reads outside the command's own environment. Then make the example's claims independent of them, or say how they change the output.

**Source:** Session evidence, `ravel-book-inside-k329`, 2026-11-11.
**Ravel application:** A Homebrew installation writes `share/ravel/installation.json`. With no `skills.sources`, Ravel then reads the default skill directory, also under `env -i` with an empty `HOME`.
If that directory holds a source, the prompt gains a preamble. The checkout build has no record, so the examples check cannot see this.
Chapter 7's example prints neither the prompt nor `RAVEL_SKILL_DIRECTORIES`, and the page says why its output is the same. Chapter 3's first session prints the prompt, and the proof of chapters 7 to 11 received that case.

**Check:** For each complete example, name each file that the product reads by the location of its executable, not by `HOME` or the working directory. Compare a claimed line that such a file can change.

### L76. Check that an excerpt is still on the path that the text describes

An excerpt check shows that the quoted lines still exist in the file. It does not show that the described path still runs through them.
A change can replace a component and keep its old functions in the file.
A change can also remove functions that the text names, while the excerpts beside them still pass.
Trace each excerpt and each named function from the entry point, through the calls that the text describes.

**Source:** Session evidence, `ravel-book-inside-k330`, 2026-11-12.
**Ravel application:** Before this draft, chapter 8's 12 excerpts passed. Its evaluation section still named `evaluate_and_resolve`, `choice::judge`, `worker::evaluate`, `config::selected` and a TypeScript worker file.
None of them exists in 2.0.0.

The chapter now follows `choose_prepared`, `evaluate_and_resolve_rhai`, `rhai_worker::front::evaluate` and `worker::supervise`. Its path table names the function of each step, in the order of the calls.
`main` also no longer calls `Cli::try_parse_from`. The same chapter named it, and no earlier reader reported it.

**Check:** For each excerpt and each named function, name its caller on the described path, up to `main`. A name without a caller on that path is stale, even when its excerpt passes.

### L77. Answer a question about a runtime value with a claimed line

A walkthrough often answers a question such as “what does this function receive here?”. A reading of the source gives the answer for the reader's case only when every branch is understood.
A complete example that prints the value turns the answer into a claim that the examples check repeats.

**Source:** Session evidence, `ravel-book-inside-k330`, 2026-11-12.
**Ravel application:** The earlier chapter 8 said that the worker supplies `undefined` without a context, which is TypeScript. The Rhai source sends null in `request.context`, and `evaluate_with` passes it on.
The refusal example of chapter 8 prints `type_of(context)`, and the check claims `config stdout: The context is ().`. A control copy that claimed `map` failed.

**Check:** For each runtime value that the text states, find the claimed line that shows it.
If no example shows it, add the print to the nearest complete example.
Otherwise, say that the value comes from a reading of the source.

### L78. Measure a claim that a chapter changes little

A plan can say that a chapter changes little because its subject did not change. That statement is a claim about the code, and the plan did not measure it.
Compare the subject's source at the chapter's last thorough revision with the current source. Then read each statement of the chapter against the change.

**Source:** Session evidence, `ravel-book-inside-k331`, 2026-11-12.
**Ravel application:** The paused charter said that the runner was unchanged, so chapter 9 would change least.

From the book's last whole copy-edit, `b8903f3a8537`, `crates/keyed-launch/` changed in 19 files. The runner gained a third launch mode, `run_captured`, for harness runs. That mode has its own POSIX session, pipes, stop request and poll interval, and it installs no signal handler.
Chapter 9 named none of it. Its cancellation table, its launch-mode table, its signal-state section and its guarantees needed changes.

**Check:** Before you mark a chapter as almost unchanged, show the diff of its subject since its last thorough revision. Name each statement of the chapter that the diff can reach.

### L79. Show a precedence rule with a pair of runs that differ in one line

A rule such as “a signal wins over the deadline” is a comparison. One run shows only one side of it.
Two complete examples that differ in one line show both sides, and each run is a control for the other. A program in the example can make the timing deterministic, so the example needs no test harness.

**Source:** Session evidence, `ravel-book-inside-k331`, 2026-11-12.
**Ravel application:** Chapter 9's `ravel-parts-deadline` has a fake harness that writes a note at TERM and stays. The deadline alone gives `selection_timeout`, status 124.

In the second example, the fake's `trap` also sends TERM to its parent, the Ravel CLI. That TERM arrives after the deadline and before the kill grace ends, which is the case of the specification's scenario. The result is `selection_cancelled`, status 143, with `selection_timeout` under “original failure:”.
A control copy that claimed 124 for the second example failed.

**Check:** For each precedence rule that the text states, find the two claimed runs that show it. Name the one line in which they differ.

### Disposition of the task's candidate readings

These dispositions concern session analysis recorded in the task mandate. They are not additional owner decisions.

| Candidate | Disposition and rule |
|---|---|
| STE alone caused compressed and abstract prose. | Reject the causal claim. Accept the observed missing explanations. L12/L46. |
| The old term-link and restatement rules encouraged page dependence. | Accept from the old contract text and F7. L05/L10/L46. |
| R8 and F7 differ because keys and explanations serve different purposes. | Retain as the contract's interpretation, not a fresh owner quotation. L26. |
| Both reads found problems after proofs passed their checks. | Accept the two recorded observations. Do not infer universal checker coverage or a measured failure rate. L41/L46. |
| Reader commands need a private shell. | Accept from the inherited completion path described in k74. L22. |
| Figure and link checks need failing controls, including checks of external addresses. | Accept from the absent-image and 404 observations. L35/L39. |
| Browser snapshots belong outside the working copy. | Retain as a precaution from k79's candidate report. This session did not reproduce that browser write. L47. |

## Owner-decision audit

The audit baseline is `0f16c8e38f8d`, before this leaf's own running log and this document.
It contains 80 Markdown task files and briefs, including live, DONE and ABANDONED records.
The source reading includes all 80 files. Of these files, 55 have a `Decisions (running log)` section.
The scan ends each log at its next level-two heading.

The reproducible search splits each log at every unindented `- ` line.
It also splits at lines with `**` as their first two characters.
This gives 575 segments, including list items within a larger decision.
A case-insensitive whole-word search for `owner` or `human` finds 97 segments.
These are search matches, not 97 owner decisions.
The complete reading also checks decisions without either word, including the “rest is agreed” consequences in k27.

The following 20 entries are the distinct owner decisions or findings about documentation and communication at the audit baseline.
One entry can settle several closely related requirements. Repeated reports of the same choice do not add entries.
This count excludes the earlier grove's background decisions and this document's own statements.

| Decision | Source and date | Recorded words or choice | Disposition |
|---|---|---|---|
| D01 | `plan-k1`, 2026-10-24. `rhai-policy-k2`, 2026-10-24 to 2026-10-25 | Provider keeps the model-origin meaning. No verbatim answer survives. | L13. |
| D02 | `plan-k1`, 2026-10-24 | A migration guide translates three policies and adds an LLM example. No verbatim answer survives. | Replaced by D05. L02 preserves the change. |
| D03 | `plan-k1`, 2026-10-24 | The owner approved the text of the separate Grove issue. | L03 and L43 keep caller conventions separate and preserve approved scope. |
| D04 | `rhai-policy-k2`, 2026-10-24 to 2026-10-25 | A design review precedes planning. No verbatim answer survives. | L18 and L45 retain document review and handoff ownership. |
| D05 | `rhai-book-use-k27`, 2026-10-26 | One-off manual migration, then “rest is agreed”. | L02. No migration chapter or TypeScript prerequisite. |
| D06 | `rhai-book-use-k27`, 2026-10-26 | “rest is agreed” accepts the retained prerequisites and Rhai teaching within chapter 4. | L01–L02. No separate Rhai tutorial chapter. |
| D07 | `rhai-book-use-k27`, 2026-10-26 | “rest is agreed” accepts unchanged chapter numbers. | L02. |
| D08 | `rhai-book-use-k27`, 2026-10-26 | “rest is agreed” retains both reader checkpoints. | L41. The owner retained both reads. |
| D09 | `rhai-book-use-k74` F1, 2026-10-27 | Characteristics, example and documentation at the first Rhai mention. | L04. |
| D10 | `rhai-book-use-k74` F2, 2026-10-27 | Explain selection and notice, with a diagram and example. | L05–L06, L26. |
| D11 | `rhai-book-use-k74` F3, 2026-10-27 | “to extend or modify the prompt” | L08. |
| D12 | `rhai-book-use-k74` F4, 2026-10-27 | Explain the desired harness/model/effort/prompt choice first. | L08. |
| D13 | `rhai-book-use-k74` F5, 2026-10-27 | Explain the inheritance mechanism with a diagram. | L07, L26. |
| D14 | `rhai-book-use-k74` F6, 2026-10-27 | Follow the launched program through termination to the returned result. | L09. |
| D15 | `rhai-book-use-k74` F7, 2026-10-27 | Repeat and restate. | L10. |
| D16 | `rhai-book-use-k74` F8, 2026-10-27 | “a more conversational tone where possible” | L11. |
| D17 | `rhai-book-use-k74`, 2026-10-27 | Apply the patterns before the re-review, later extended to the entire book. | L42. The later scope extends the same decision. |
| D18 | `rhai-book-use-k74`, 2026-10-27 | Record all lessons for a skill, clarified to include all documentation and communication. | L48. The clarification extends the same decision. |
| D19 | `rhai-book-use-k74`, 2026-10-27 | “Parts I–II first” | L42. The rest follows before Read 2. |
| D20 | `llm-calls-through-harnesses-k58`, 2026-10-25 to 2026-10-26 | “Also comment on issue 2” | L43. Publish the agreed amendment where the original requirements remain visible. |

Each row below accounts for the numbered search matches, in path order at the baseline.
A range includes every match in that range. “Session” denotes analysis or a report, not another owner decision.

| Match numbers | Source | Disposition |
|---|---|---|
| 1–7, 9–10 | `plan-k1` | Product cutover, capabilities, providers, test seam, confirmation, callers, release and task order. No separate writing lesson. L17/L21/L45 use their implications. |
| 8 | `plan-k1` | D02. |
| 11 | `plan-k1` | D01, raised here and settled in k2. |
| 12 | `plan-k1` | D03. |
| 13–14, 16–17 | `rhai-policy-k2` | Toolchain, isolation, run opt-in and settings. Product decisions, not separate writing lessons. |
| 15, 18 | `rhai-policy-k2` | D01 and D04. |
| 19–23 | `rhai-policy-increments-k3`/`k6` | Session planning and coverage statements. L17, L19, L24 and L45. |
| 24–27 | `rhai-policy-increments-k40`, `rhai-engine-limits-k45`, `rhai-try-boundary-limits-k46` | Session corrections to evidence requirements and guarantees. L15, L17, L21 and L41. No new owner approval follows from these reports. |
| 28–33 | `rhai-llm-anthropic-call-k55`, `rhai-llm-budgets-k18`, `rhai-llm-openai-types-k19` | API and accounting choices or proposed options. Product detail, with source discipline under L15/L20. |
| 34–36, 38–39 | `llm-calls-through-harnesses-k58` | Recursion, Rig removal and trust are product decisions. The catchable schema rule is explicitly a session choice. L15/L43 preserve their scope and attribution. |
| 37 | `llm-calls-through-harnesses-k58` | D20. |
| 40–45 | `rhai-llm-upgrade-proof-k21` | Product redirection and repeated reports of its fate. L44 records the communication lesson about the repeated question. |
| 46–54 | `llm-calls-through-harnesses-k59`/`k61` | Session review and triage of contracts. L15–L16 and L38. No new owner writing decision. |
| 55–57 | `llm-harness-replan-k60`, `rhai-llm-removal-k63` | Session smoke-task, tree and JSON-feature consequences. L17/L45 retain handoffs. |
| 58–59 | `rhai-endpoints-removal-k64` | Session glossary and search classifications. L13/L36. |
| 60–61 | `rhai-upgrade-task-k65` | Session test-seam statement and documentation handoff. L19/L45. |
| 62 | `rhai-llm-call-scenarios-k67` | Session correction of the notice guarantee. L15–L16. |
| 63 | `rhai-harness-smoke-k66` | Permission for two real calls. An operational choice, not a writing lesson. |
| 64–66 | `rhai-harness-smoke-k66` | Session measurements of sandbox home, sign-in and arguments. L20–L22. |
| 67–72 | `rhai-example-policies-k25` | Example placement, invented personal-policy shape, failure rules, translation and handoff. L02/L20/L21/L45. No new owner writing decision. |
| 73–75 | `rhai-book-use-k27` | D05–D08. The full reading also includes prerequisite and chapter-number consequences without an owner keyword. |
| 76–77 | `rhai-book-use-k69`/`k73` | Session terminology and read placement. L13/L41/L45. |
| 78–82 | `rhai-book-use-k74` | Mechanical checks, highlighter warning, private environment, handover and reader-evidence limits. L22/L39/L41/L45. |
| 83–88 | `rhai-book-use-k74` | D09–D14, the six passage findings. |
| 89–91 | `rhai-book-use-k74` | D15–D16 and their evidence-class introduction. |
| 92 | `rhai-book-use-k74` | D17. |
| 93 | `rhai-book-use-k74` | Session web checks, not an owner finding. L39. |
| 94–96 | `rhai-book-use-k74` | Session contract and routing reports. L10/L26/L42/L43/L45. |
| 97 | `rhai-book-use-k74` | D17–D19. The last segment contains the scope extension, skill clarification and sequencing answer. |

The full reading adds lessons that the keyword search cannot find by itself.
These include strict-check scope, stale references, source inventories, failed controls and visual evidence in the mechanics files.
L01–L51 identify their sources individually.

### Background audit and provenance

The earlier `book-read-one-k70` at `fa7b73f` is background throughout this file.
R1–R17 map respectively to L03, L14, L14, L04, L14, L09, L14, L26, L03, L28, L27, L09/L14, L09/L14, L03, L03, L09 and L25.
Its later choices resolve the caller pronoun, the result verb, STE scope, crisp style, prose-break threshold and release version.
It also records approved correction runs and acceptance of the reduced README and removed guides.
L12, L14, L18, L28, L41 and L42 retain those lessons.
The temporary “submit” choice has no current authority over “send”.

The source reading includes every current task and brief, the glossary, the five cited ADRs and all five book contract/mechanics files.
The full commit messages from `01bc4eec4416` through `0f16c8e38f8d` supply the current grove's history.
Current-grove dates follow the dated log where available. A two-day range preserves a commit/log date difference rather than inventing a precise conversation date.
Historical task paths remain available in version control after Grove removes its tree.

To reproduce the background read, run:

```sh
jj file show -r fa7b73f .grove/23-k55/39-DONE-requirements--book-read-one-k70.md
```

To inspect the current-grove messages, run:

```sh
jj --no-pager log -r '01bc4eec4416::0f16c8e38f8d' --no-graph -T 'commit_id.short() ++ " " ++ description ++ "\n"'
```

The audit search uses this algorithm against the baseline files:

```python
from pathlib import Path
import re
import subprocess

revision = "0f16c8e38f8d"
paths = subprocess.check_output(
    ["jj", "file", "list", "-r", revision, ".grove/**"], text=True
).splitlines()
counts = {"files": 0, "logs": 0, "segments": 0, "matches": 0}
for path in sorted(path for path in paths if Path(path).suffix == ".md"):
    source = subprocess.check_output(
        ["jj", "file", "show", "-r", revision, path], text=True
    )
    counts["files"] += 1
    heading = re.search(r"^## Decisions \(running log\)\s*$", source, re.M)
    if heading is None:
        continue
    counts["logs"] += 1
    body = re.split(r"^## ", source[heading.end():], maxsplit=1, flags=re.M)[0]
    starts = list(re.finditer(r"^(?:\*\*[^\n]+|- [^\n]+)", body, re.M))
    for index, start in enumerate(starts):
        end = starts[index + 1].start() if index + 1 < len(starts) else len(body)
        segment = body[start.start():end]
        counts["segments"] += 1
        if re.search(r"\b(?:owner|human)\b", segment, re.I):
            counts["matches"] += 1
            print(counts["matches"], path, index + 1)
print(counts)
```

The search count does not itself prove that the search found every semantic decision.
The full reading and explicit dispositions supply that additional check.
Future additions need their own date and audit scope rather than a silent change to this baseline count.

## Read 1 addition, 2026-10-28

This addition covers the owner's message after the resumed chapter 1 read in `rhai-book-use-k74`.
It preserves that message's six findings as F9–F14 in the task's running log.
It does not change the scope or counts of the baseline audit above.

| Finding | Owner evidence | Disposition |
|---|---|---|
| F9 | Explain the prompt, completion instruction, possible results and caller's next action. Teach a concrete case before the general model. | L57–L58 and contract §2. |
| F10 | Prefer “configuration” to “owner policy”. | L59. The session's proposed qualifiers remain distinct from the owner's words. |
| F11 | Remove version labels at incidental JSON mentions. | L60. Preserve required fields in exact examples and reference entries. |
| F12 | Explain the separate copy of Ravel and the reasons for it. | L07. Check the process mechanism and its purpose against the source and ADR. |
| F13 | Link Grove to its repository. | L39. The session checked that destination. |
| F14 | Replace the opaque heading about evidence limits. | L61. Preserve the section's distinct limits and reconcile its links. |

The owner also said, “I have read all of Chapter 1.” L41 records that progress separately from the unreported practical tasks.

The owner then chose “Collect the remaining notes first (Recommended)” before one combined correction run.
L42 records that sequence. It settles the correction timing while the reader checkpoint remains live.

The owner then reiterated the skill goal. L48 and this document's opening paragraphs now state the generalisation method and its limits.

The owner later completed Part I, chapters 1–3, and supplied two more findings.
F15 rejects the chapter 2 title's unusual shorthand. F16 prefers the familiar `cd` command to the abstract directory instruction.
L61 and L62 preserve those general lessons, with the Ravel corrections as examples.

The owner then said, “I would like you to apply all the lessons before I proceed to Part II.”
That instruction replaces the earlier timing choice. L42 records the current sequence: correct chapters 1–6, then resume the Part II read.
