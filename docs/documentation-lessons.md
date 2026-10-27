# Lessons for documents and communication

This file records what this grove learnt about documents and communication through 2026-10-28.
Its rules apply to books, reference pages, specifications, decision records, help text and replies.
They also apply to questions, handovers, task files, briefs, commit messages and prompts for other models.
Another project can use the rules after it states its own readers, terms and evidence requirements.

Each later session adds its lessons here when they arise.
Keep the rule, its source, its evidence class and its application together.
The owner wants a skill for other groves. The skill waits until the owner names its home.

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

### L05. Explain a term before each page needs it

Give a short local explanation even when another chapter teaches the term.
Say what the thing is, where it comes from and why it matters here.
A link can then offer more detail.

**Source:** Owner F2 and F7, `rhai-book-use-k74`, 2026-10-27.
The owner found that “'a selection' is not described/contextualised”.
**Ravel application:** Contract §2, “Explain a term before you use it” and “Terms”, replace the earlier link-only rule.
Direct chapter 6 entries also need their own context under contract §1.

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

### L07. Explain the mechanism behind an abstract verb

Words such as “inherits”, “finds” and “sends” need a concrete mechanism when the reader does not know it.
Show the data, the transfer between actors and the condition that changes the transfer.
Do not replace an unexplained term with another unexplained term.

**Source:** Owner F5, `rhai-book-use-k74`, 2026-10-27: “inherited how? what is the mechanism.”
**Ravel application:** Contract §2, “Show the mechanism”, requires the environment-variable path through parents and children.
The correction shows `RAVEL_EXIT_FILE`, its inheritance and its replacement for a nested session's own child.
Draft `rhai-book-use-k80` shows it in chapter 1, “How `end-session` finds its session”, with a table of processes and the paths of a real nested run.
Draft `rhai-book-use-k82` restates the mechanism in chapter 5, “Keep nested sessions separate”, with a table of both layers and a second real run.

### L08. State the reader's goal before the capability

Give the reason why the reader might want the operation.
Then explain the means and its effect on the reader's work.
Prefer an observable change to an abstract description of instructions.

**Source:** Owner F3 and F4, `rhai-book-use-k74`, 2026-10-27.
The owner preferred “to extend or modify the prompt” and described the desired harness/model/effort choice before the means.
**Ravel application:** Contract §2, “Describe the effect in the reader's terms” and “Put the reader's goal first”.
Draft `rhai-book-use-k81` opens each section of chapter 4 with the reason why the owner may want it. “Run a harness during evaluation” opens with the owner's F4 sentence.

### L09. Follow an action through to the caller's result

Name each actor in the order that it acts.
End the explanation with what the caller receives and the conditions that permit it.
Explain an empty result, an absent result and a process exit status separately.

**Source:** Owner F6, `rhai-book-use-k74`, 2026-10-27, and background R6, R12, R13 and R16, 2026-10-17 to 2026-10-18.
F6 asks that “the result becomes the value returned as the Ravel result”.
**Ravel application:** Contract §1 and §2 follow `end-session`, group cleanup and the return from `start-session`.
The first-result rule also needs the race and shutdown explanation.
Draft `rhai-book-use-k81` gives chapter 4 order tables for an evaluation and the first example.
Other order tables follow policy context, a harness run, the Claude Code recipe and an interactive route.

Draft `rhai-book-use-k82` opens chapter 5 with chapter 1's order table and a column for the section of each row. A table after each complete example ends with what the caller receives.

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
[Word decisions](book/ste.md) keep “policy”, “policy worker”, “harness run” and the full standard-stream names consistent.
Its signal-name rule preserves literal identifiers and diagnostics.

The prose checker accepts the first word of a synonym group in a file and reports each other member. New text uses the verb that the page already uses. Chapter 4 says “start” for a process, where chapter 1 says “launch”.

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
**Ravel application:** The policy spec separates run output limits from evaluation stops, and `env` grants from deliberate credential reads.
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

### L27. Break a long prose run with useful information

Introduce a figure, table or code block before the agreed screen of prose becomes too long.
A heading or list does not itself supply that break.
Keep the visual relevant to what the reader needs to understand.

**Source:** Background owner R11, `book-read-one-k70`, 2026-10-18: “One screen, code counts”.
**Ravel application:** Contract §2 sets a 450-word threshold at the 750-pixel column.
`task docs:breaks -- --strict` reports longer runs. Its controls check the counting rules.

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

### L37. Freeze inputs before measuring

Finish edits before a check. Give one measurement one writer.
Digest every input before and after, including manifests, fixtures, tools and instructions that the check reads.
Compare repeated runs item by item rather than comparing totals alone.

**Source:** Sessions `rhai-llm-removal-k63`, `rhai-book-use-k27`, `k70`, `k71`, `k73` and `k74`, 2026-10-26.
**Ravel application:** Book proofs recorded unchanged subject digests and per-example outcomes.
Source changes during a run require a fresh measurement.

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

### L40. Keep checker rules distinct from editorial judgement

Check what a rule actually governs. Preserve quotations, code identifiers and historical records as such.
Read advisories in context rather than changing meaning to remove them.
A checker needs an explicit scope and a test when that scope changes.

**Source:** Session `rhai-book-use-k69`, 2026-10-26.
The word `binding` appeared on a glossary `_Avoid_` line that names a rejected word.
**Ravel application:** `ste.md` records that the two word checks exempt those lines while other rules still apply.
The checker has a control and a mutation for that exemption.
Book-specific and general prose checkers cover different surfaces.

### L41. Keep reader evidence separate from mechanical and model evidence

Record the reader's words, location, progress, task performance and unreached material.
Label a stand-in reader and the limits of that substitution.
Stop at an early comprehension failure instead of treating a passed proof as a reason to continue.

**Source:** Background `book-read-one-k70`, 2026-10-16 to 2026-10-18. Owner agreement `rhai-book-use-k27`, 2026-10-26.
Session `rhai-book-use-k74`, 2026-10-27.
**Ravel application:** Contract §4 requires both reads after proof.
The latest read has findings from chapter 1 but no reported task performance or declared viewing surface.
It establishes no result for a developer new to Ravel.

### L42. Apply the pattern of a finding across its agreed scope

Extract the underlying explanation problem before correcting the quoted passage.
Check each page for the same problem.
Route corrections to the stage that owns them and finish the correction pipeline before the next dependent read.

**Source:** Owner, `rhai-book-use-k74`, 2026-10-27: “apply the patterns of these re-statements”.
The owner later extended those patterns to the entire book and chose “Parts I–II first”.
**Ravel application:** Contract §2 states ten patterns from eight findings.
`rhai-book-use-k75` through `k78` correct chapters 1–6. `rhai-book-inside-k34` applies them to chapters 7–11 before Read 2.
Read 1 remains live for the re-review.

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
The owner-shaped policy uses invented programs. The credential probes share a sentinel and read no real sign-in.
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

### L55. Claim the order of two outputs when the order teaches the point

When the text explains that one output comes before another, make one claim cover both outputs.
A claim of two adjacent lines fails when their order changes. Two separate claims do not.
Show an order that the check cannot claim in a labelled fragment of a real run.

**Source:** Session evidence, `rhai-book-use-k82`, 2026-10-28.
The text routing notice precedes the launch, and the JSON notice follows the supervision attempt. Chapter 5 teaches that difference.
**Ravel application:** The notice example of chapter 5 claims the text notice and the fake agent's next line as one block. A changed character in either line failed the check.
The JSON notice holds values that change in each run, so a fragment of a real run shows the agent's line before the JSON document.

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

The following 20 entries are the distinct owner decisions or findings about documentation and communication in this grove.
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
