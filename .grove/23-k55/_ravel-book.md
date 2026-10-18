# ravel-book-k55 — brief

## Goal

Write the Ravel book to its contract, `docs/book/contract.md`: a pedagogical
user guide that lets a software engineer new to Ravel's context use, modify,
maintain and reuse Ravel, drafted against this workstream's current code. The
owner settled the contract in `book-structure-k5` on 2026-10-16.

## Done when

- Chapters 1–9, 11 and 12 exist under `docs/book/src/` in the contract's order,
  each taken through draft → copy-edit → art → proof under the contract.
- A whole-book art pass unifies colours, fonts, type sizes, line weights,
  arrows and spacing across those chapters, followed by whole-book proof
  before read 2. Readers know the diagram types; only book-specific
  conventions need explanations.
- The book's Taskfile tasks exist and pass: the mdBook build and local serve,
  the figure regenerate-and-compare check, the complete-example check through
  the CLI/real-policy/fake-program seam, the source-excerpt check, the STE
  check and the check for runs of prose longer than the limit. `task
  setup` fetches the pinned mdBook. Publishing the built book to GitHub Pages is
  part of the release procedure; the first publication happens with the next
  release, outside this node.
- The guides the contract lists are folded in and removed, every inbound link
  redirected, and the README reduced to a short entry into the book. The
  owner's configuration handoff record is kept outside the book.
- Read 1 (after Parts I and II) and read 2 (after Parts III and IV) have been
  held as the owner's labelled stand-in reads, each recorded with its evidence
  class, with their corrections routed to the stages that own them.
- The empty `--policy-env` description in `ravel start-session --help` is
  fixed, or handed on with its reason.
- Chapter 10 is not this node's: the work that delivers a supported extracted
  component writes it, together with that component's interface and consumer.

## Decomposition

Lazy, in two halves separated by read 1, because chapters 7–12 build on the
model chapters 1–3 teach and the contract stops dependent drafting until an
early comprehension failure is corrected. The first child plans the first half:
the scaffold the drafting needs, the drafting units for chapters 1–6, the guide
migration they imply, and read 1 placed after their proof. The session that
holds read 1 cuts the second half's planning leaf under this node once read 1's
corrections are routed. Everything for the book stays inside this node, so the
book runs before `component-work-scope-k57`, which follows the node at the root.

## Pointers

- Contract: `docs/book/contract.md`, with the structure brief, prose contract,
  format and media, and reader checkpoints. A document node brief or draft leaf
  body names it by path; it carries all three items the draft precondition
  cites.
- Term list: `CONTEXT.md`, including its flagged ambiguities.
- STE word decisions: `docs/book/ste.md` (from `book-first-half-k123`).
  `task docs:ste` examines the text against them and the STE rules that a
  script can check. The STE specification is at the path that the contract
  gives. It stays out of the repository.
- Normative sources the book restates: `docs/specs/session-execution.md`,
  `docs/specs/result-job-completion.md` and the three decision records in
  `docs/adr/`.
- Guides to fold in: the contract's "Relation to other documents".
- Example seam: the existing CLI tests' real-policy/fake-program fixtures in
  `crates/ravel/tests/cli.rs`.
- Chapter 8's method: the `linkuistics:writing-code-walkthroughs` skill at
  orientation depth (no complete-source fragment graph).

## Notes

- The pinned upstream design's contract format, figure contract, house style
  and toolchain snapshot are not installed in this checkout's Grove. The
  contract carries everything the book needs from them; stage sessions read
  the contract instead of looking for those files.
- This workstream's code is 48 commits past the v1.0.0 release and is not on
  GitHub, so GitHub's rendering of the book can be checked only after a push.
- Versions pinned in the contract (mdBook 0.5.4 current on 2026-10-16; Typst
  0.15.1 and CeTZ 0.4.2 on 2026-10-17) were current when checked; recheck a dated
  fact before a choice turns on it.

## First half: units and shared conventions

Cut by `ravel-book-first-half-k56`: the scaffold and one small product fix,
the five drafting units, the set proof, guide migration and read 1. The owner's
later prose and diagram amendments add a set correction before migration.
Chapters are drafted from the code and the
normative specifications. The guides they replace are sources that may be
stale, so each chapter verifies a claim against the subject.

- **Scaffold first** (`book-toolchain`, `book-figures`, `book-examples`): each
  is verified, with controls seen to fail, before any chapter relies on it.
  `policy-env-help` follows so no chapter hedges the empty description.
- **Units**, each a document taking draft → copy-edit → art → proof, and each
  naming `docs/book/contract.md` as its contract. Grouping follows session
  size and dependency:
  - chapters 1 and 2 are one unit: both are short, 2 defines the words 1 uses,
    and the system-context figure belongs to both;
  - chapter 3 is its own unit: it carries the first complete example and the
    first sequence and state figures, and everything after it leans on it;
  - chapter 4 is one document drafted in two sessions, because it runs from
    writing a first policy to context, settings, grants, real routes and
    copying a policy: part A the anatomy of a policy and its validation with
    fake agents, part B context, settings, grants, module resolution, real
    routes and copying;
  - chapter 5 is its own unit and needs chapter 3, not chapter 4's part B;
  - chapter 6 is one document drafted in two sessions along the contract's own
    inventories: part A the CLI (inventory 1), part B configuration and
    installed paths (inventory 2). It comes last because its entries link
    back to where each subject is taught.
- **One set proof** (`book-first-half`) reads chapters 1–6 as a set after the
  last unit, because a unit's proof reads only its unit and the entry routes,
  the inventories and the cross-chapter terms span them.
- **Guide migration** follows the set proof and precedes read 1, so read 1's
  entry is the reduced README and what is removed is already proved absorbed.
- **Chapter file names** (set by `book-toolchain`; the contract's order):
  `01-what-ravel-is-for.md`, `02-actors-and-words.md`,
  `03-a-complete-session.md`, `04-writing-an-owner-policy.md`,
  `05-running-sessions.md`, `06-reference.md`. SUMMARY lists chapters 7–12 as
  mdBook draft entries until the second half writes them.
- **Links**: a chapter links back to earlier chapters freely, with anchors. A
  forward reference to chapter 6 is a page-level link (no anchor), because its
  entries are written last; chapter 6 links back to anchors. A reference to a
  chapter 7–12 is plain text with its number and title, and the second half
  turns each into a link when its target exists (see below).
- **Examples**: every complete example runs through `task docs:examples`; the
  convention (labels, the `$` command, the one `<scratch>` normalisation, skips)
  is `docs/book/examples.md`. A fragment says what it omits and covers one block.
  A cancellation caused by the fake agent or policy (`kill -INT "$PPID"`) is a
  complete example claiming `Standard error includes`; only a cancellation that
  needs a terminal is a fragment. A route to Claude Code or Codex is a fake
  executable of the same name.
- **Figures**: the art stage of each unit owns that unit's figures from the
  contract's figure list and may refine, merge or add. The first unit's art
  fixes the visual vocabulary; later units reuse and extend it. No figure
  relies on Mermaid.
- **Term and unknown discipline**: the term risks and the omissions list in
  the contract are read by every stage. "Context" is never unqualified.

## Guide migration: where content lands

Planned for the first half; the sentence-level classification is done by the
migration leaves from the proved chapters, not from this table.

| Guide content | Lands in | Disposition |
|---|---|---|
| `session-cli.md`: the boundary, the two verbs, the prompt and result meanings, what Ravel has no store for | chapters 1, 2, 3 | file removed by `book-guide-fold-in` |
| `session-cli.md`: inputs, exit behaviour, `--json` notices, `operationReports`, worker pairing, embedded imports, `readChoice` | chapters 4, 5 and the 6 entries | removed with the file |
| `session-cli.md`: development checks, task setup/build/typecheck/check | README "Develop" until chapter 11 | any sentence with no other home moves there |
| `owner-configuration.md`: defaults, settings, bounds, SDK import, copying a policy, fake-agent validation, development typechecking | chapter 4 (parts A and B) and the 6 entries | file removed by `book-guide-fold-in` |
| `owner-configuration.md`: "Configuration handoff on 2026-10-12" | `docs/configuration-handoff-2026-10-12.md`, outside the book | kept, not deleted |
| `authority.md`: who holds which authority, trusted policy, worker reaped, same-user and detached-child limits | chapters 2 and 5 | those paragraphs removed by `book-readme-and-authority` |
| `authority.md`: test-isolation guards, legacy-name exceptions | chapter 11 and `docs/caller-references.md` | file reduced now, removed in the second half |
| README: what Ravel is, the pair, the runner, the channel, "Use the pair" | chapters 1–3, 5, 6 | reduced to a short entry into the book |
| README: Develop, release, earlier project, licence | chapters 11 and 12 | kept until those land |

Removal waits for the set proof (`book-first-half`) so that "absorbed" is a
statement about proved chapters, and precedes read 1 so read 1's entry is the
reduced README.

## Handed forward to the second half

- Plain-text mentions of chapters 7–12 in chapters 1–6 become links when each
  target exists (a sweep by the second half's proof).
- `docs/authority.md` is reduced, not removed, by the first half
  (`book-readme-and-authority-k69` did it): it now states only the
  test-isolation guards and the legacy-name exceptions, under that title, and
  points to chapters 2 and 5 for the authority boundaries. Chapter 11 takes
  those two paragraphs (forced-empty Cargo controls, scrubbing helpers,
  `env_clear` fixtures, installed smoke's `env -i`, the `GROVE_*` and
  `HARNESS_DISPATCH_*` extraction-safety exceptions); the file is removed when
  chapter 11 lands, with its row in `docs/caller-references.md` and the
  `docs/book/contract.md` "Relation to other documents" clause edited then.
- Contributor sentences from `docs/session-cli.md` that had no other home are
  now in the README's Develop section (kept until chapter 11 absorbs it): the
  `task typecheck` description and list line, what `task test:cli` runs, "focused
  checks while editing, the full relevant suite at handoff", and that `task
  check` rebuilds and typechecks the pair. Chapter 11 should take them from
  there. `book-guide-fold-in-k68` removed both guides and holds the
  sentence-level classification.
- The non-`--json` form of an operation report (`ravel: <report>` on stderr,
  `crates/ravel/src/run.rs`) is in no chapter; chapter 6's stderr row says only
  "operational reports". The second half's proof decides whether to add it.
- Contributor and delivery parts of the README stay until chapters 11 and 12.
  The README is now an entry (what Ravel is, the two verbs, where the book is
  and `task docs:book`) plus the untouched "Develop" section, "Earlier
  project" section (carrying the two release/archive paragraphs and the
  licence line). Chapter 11 absorbs "Develop" (including the "private session
  channel" paragraph and the contributor sentences listed above); chapter 12
  absorbs the two release paragraphs under "Earlier project", whose heading
  does not describe them; the retired record's pointer stays. The README's
  short paragraph naming `docs/components.md`, `docs/local-runner.md`, the
  specification and runtime evidence as references for the unwritten Parts III
  and IV is to be reworded to point at the chapters when they land. The notices
  line in that paragraph (checkout path versus installation path) moves to
  chapter 12.
- The visual vocabulary and the first figures' conventions are the first
  half's art output; chapters 7–12 follow them.
- The first figure for "packages, worker and SDK, in source and in an
  installation" appears in chapter 6; chapters 7 and 12 reuse it.
- Read 1's rules apply to chapters 7–12: the crisp figure style (R10) and
  the limit on runs of prose, with the check from
  `book-prose-breaks-check-k133` (R11).
- Chapter 7 explains the parts of Ravel. It states the loose contract
  between the Ravel CLI and the configuration (R15) in agreement with
  chapter 1.
- Chapter 12 teaches the release tasks. It includes how the release writes
  its version into chapter 3 and checks it (R17).
  `book-release-version-k145` adds the details here. What chapter 12 must say
  about the mechanism (done 2026-10-18):
  - The book has one version line, `ravel @RAVEL_BOOK_VERSION@` in a `text`
    fence in `docs/book/src/03-a-complete-session.md`. A checkout between
    releases shows the placeholder or the last release's version, and the
    reader of chapter 3 compares it with `ravel --version`.
  - `release.toml` has a second `pre-release-replacements` entry beside the
    changelog one. Its pattern matches the whole line with the placeholder or
    any `X.Y.Z`, and `exactly = 1`. So the next cut finds the line again.
    `cargo release` writes the version into the line in the same release commit
    as the manifests, and the tag carries it. The path is relative to the
    package directory, `../../docs/book/src/03-a-complete-session.md`, as the
    changelog path is. The book uses no include that only mdBook understands,
    so GitHub and raw views show the same text.
  - Two guards in `scripts/release-book-version.sh`. `ready` (run by `task
    release:doctor`) refuses a book without exactly one such line. `released
    vX.Y.Z` (run by the release task after the cut and before the build) reads
    the page at the tag and refuses a line that is not `ravel X.Y.Z`, or any
    surviving placeholder. A refusal leaves the cut local with nothing pushed;
    the recovery is "Recover a partial release without another bump" in
    `docs/RELEASING.md`.
  - `scripts/release-book-version.test.sh` (part of `task release:lifecycle:test`)
    runs the real cargo-release 1.1.2 in an isolated repository: a dry run
    that shows the replacement, two cuts, and the controls that fail. The
    published book for each release is on GitHub Pages, and the placeholder
    text in chapter 3 tells the reader to use it when versions differ.

## First-half set proof and owner amendments

`book-first-half-k67` read chapters 1–6 as a set and checked the learning,
caller and policy-author routes. It links the outstanding first-half chapter
mentions and qualifies abbreviated policy-context prose. Its source-derived
inventories find no missing CLI option, status, diagnostic code, SDK export,
setting, bound or installed path. The task records sources, scope, checks and
limits; this is not a comprehension result.

The owner changed the contract during that proof:

- Readers already know the diagram types, or can look them up. Remove generic
  notation tutorials; retain each figure's own keys and meaning.
- Unify visual style across the whole book: colours, fonts, type sizes, line
  weights, arrows and spacing. Shared definitions carry the style. A final
  whole-book art pass follows the chapter units and precedes final proof and
  read 2. This is visual consistency, not a new `art-review` kind.
- Write for a 16-year-old software engineer with the contract's programming
  prerequisites. Use **Ravel CLI**, first defined as the `ravel` command-line
  program, instead of “front” or “native front”. Use plain actions such as
  “checks”, “accepts” and “allows” instead of specialist wording such as
  “admits”. Simplify the whole prose, rather than replacing only those words.
  Figure labels, captions, keys and alt text follow the same language.

These requirements are in `docs/book/contract.md`, with the chosen term in
`CONTEXT.md`. The correction run `book-first-half-k98` (copy-edit),
`book-first-half-k99` (art) and `book-first-half-k100` (proof) precedes guide
migration. Art has applied the notation decision and plain terms to all nine
figures and their surrounding text, and made palette, typography, principal
strokes and spacing explicit in the shared definitions. Its current conventions
and dated viewing limits are in `docs/book/figures.md`. The whole-set proof
`book-first-half-k100` has closed this correction handoff: all six chapters,
the source inventories and the reading routes agree with the amended contract,
both specifications and the ADRs. Fresh build/serve, 13 complete examples,
nine figure comparisons and `task check` (339 passing tests) passed on unchanged
subjects; all 403 local chapter destinations resolve. Its task records the
independent local viewing sample and the limits of those checks. Parts I and II
are ready for the already-live guide migration and read 1; no first-half entry
is handed forward unresolved. The later `ravel-book-k101`
art leaf checks the completed book across chapter and diagram boundaries.
Read 1 and its planning continuation insert the second-half work before that
whole-book art leaf; read 2 stays after its proof. No parent closes here.

The earlier unit handoffs have owners and no unresolved first-half list
remains. Preserve these facts through guide migration: no Ravel-owned store
is not a restriction on trusted policy's filesystem/network access; the
unchanged prompt promise ends at caller-to-policy delivery and policy builds
program arguments. Unit guide-disposition tables remain migration inputs.
Preserve the dated personal handoff outside the book. The art correction's
retained metadata-only failures and thirty serial matches do not establish
the cause of the discarded selection render or universal reproducibility.

All second-half handoffs above remain live. Inventories 4 (component
obligations) and 5 (contributor workflows) are not yet claimed. GitHub rendering,
the published site, real-agent behaviour, interactive-terminal operation,
native x86-64 qualification and new-reader comprehension remain separate
unknowns. Chapter 10 belongs to the future supported-component work.

## Read 1

Read 1 (`book-read-one-k70`) paused on 2026-10-17, after the owner's first two
findings and at the owner's request, so that the corrections land before a
full read from the start. The framing finding changed the contract's "The
intended domain": Ravel runs LLM sessions in agent harnesses, such as Claude
Code and Codex, as supervised parts of another program, typically an
orchestrator, and `end-session` exists because an interactive harness does not
end itself. Chapters 7–12 follow that framing too. The chapters 1–2
correction run (draft k102 → proof k105) and the README's alignment
(`book-readme-k106`) run first. The owner then authorised a two-diagram
Typst/CeTZ trial, `book-figure-renderer-k107`, before the resumed read, and
clarified "Prefer Typst/CeTZ; trial before migration". It compares the dense
process-authority view and a sequence, including a longer label and an added
participant, before any decision to migrate the book's figures. The read then
resumes and, once its corrections are routed, cuts the second half's planning leaf before
`ravel-book-k101`.

The opening-unit correction closes with proof `what-ravel-is-for-and-its-actors-k105`
on 2026-10-17, closing document node `what-ravel-is-for-and-its-actors-k62` again.
The whole unit agrees with the amended contract: harness sessions are parts
of a caller's work, `end-session` exists because interactive harnesses wait
for more input, and result publication requests ending before confirmed cleanup
and return. Both C4 views, keys, alternatives and tables agree. No further
correction was needed and the document's handoff list is cleared. Fresh build,
nine figure comparisons and 13 complete examples passed; 538 rendered/entry
local destinations resolved, with missing-file/anchor/image controls rejected.
Local Chrome viewing and chapter 1's VS Code preview supplied independent
visual checks; GitHub and the published site remain unavailable. This is no
real-agent execution or new-reader comprehension result. The README alignment,
accepted Typst/CeTZ trial and resumed owner read remain their existing live
leaves; the current media contract still governs until a migration is agreed.

The read resumed on 2026-10-17 and paused again after chapter 1, at the
owner's choice (“Fix before reading on”, then “Route now”). The owner gave
findings R3–R9 (the read 1 log) and two decisions for the whole book:

- ASD-STE100 Simplified Technical English, Issue 9, is the prose base for all
  documentation (contract, section 2). On 2026-10-18 the owner limited it to
  the writing rules: the book does not use the STE dictionary. British
  spelling stays under STE Rule 1.14. Six chapter titles and two part titles became commands, because Rule
  3.5 permits “-ing” forms only in technical names. The file names stay.
- “Send” replaces “publish”, “publication” and “submit” for the result, in
  the book, the README, `CONTEXT.md`, the contract and the `end-session` help
  text. The specifications and decision records keep their words.

The correction run comes before the resumed read: the shared STE word
decisions `book-first-half-k123` (`docs/book/ste.md`), the chapters 1–2 draft
k124, one copy-edit for each unit (k125–k129), set art k130 (R8: the colour
text that repeats each figure's key), the README and help leaf k131, and set
proof k132. The read then starts again from the beginning. The second half's
planning leaf is still owed at the end of the read. Chapters 7–12 follow the
STE writing rules and `docs/book/ste.md`.

Copy-edit `what-ravel-is-for-and-its-actors-k125` converted chapters 1 and 2
to STE and “send”. Both pass `task docs:ste -- --strict`. Chapter 1's
heading “Choose a reading route” is now “Choose a route through the book”.
No link used its anchor. `ste.md` now gives the words for “ending
instructions” and bare “routing”, which chapters 3–6 also use. Both alt texts and the prose before the figures keep their content. This closes document
node `what-ravel-is-for-and-its-actors-k62`. Set art k130 and set proof
k132 in this node complete its third correction run.

Copy-edit `a-complete-session-k126` converted chapter 3 to STE and “send”.
It passes `task docs:ste -- --strict`. Its heading “Publish the result and
finish the job” is now “Send the result and finish the job”, and chapter 5's
one link to it changed. Both alt texts and captions keep their facts. This closes
document node `a-complete-session-k63` again.

Copy-edit `writing-an-owner-policy-k127` converted chapter 4 to STE and
“send”. It passes `task docs:ste -- --strict`. Its title is “Write an owner
policy”. Its heading “Copy an existing policy” is now “Copy a policy from
another configuration” (“existing” is an “-ing” form), and chapter 6's three
links to it changed. The agent example's prompt now says “When you finish,
run … as your last action. Then do nothing else.” Chapter 4 uses page-native tables and has no separate diagram. This closes document node
`writing-an-owner-policy-k64` again.

Copy-edit `running-sessions-k128` converted chapter 5 to STE and “send”. It
passes `task docs:ste -- --strict`. Its title is “Run sessions”. Its heading
“Read notices without confusing them with results” is now “Read notices
separately from results” (“confusing” is an “-ing” form). Chapter 6's twelve
links to it changed, and their link texts did not. The three alt texts and the prose before the figures keep their facts. This closes document node
`running-sessions-k65` again.

Copy-edit `reference-chapter-k129` converted chapter 6 to STE and “send”.
It passes `task docs:ste -- --strict`. No heading or anchor changed. The
column “Teaching” is now “Taught in”, and the diagnostic-code tables split
“Stage; status” into two columns. Both alt texts and the blockquotes before the figures keep their facts. This closes document node
`reference-chapter-k66` again.

`book-first-half-k123` wrote `docs/book/ste.md` (the result words, the
titles, the words that the book does not use and the “-ing” words that it
keeps), the check `task docs:ste` with its controls `task docs:ste:test`,
`CONTEXT.md` in STE and the new titles in `SUMMARY.md`. Each later stage of
the run passes the strict check on the files that it owns.

## Read 1 outcome and its correction run (2026-10-18)

Read 1 is held. The owner read Part I completely and skimmed Part II, in the
book that `task docs:book` serves: “Both look correct excepting the above
issues.” The owner did not report the four tasks. The owner's words for
findings R10–R17, the decisions and the mechanical checks are in
`book-read-one-k70`'s log. The owner said that the reduced README and the
removal of the two guides read correctly. The contract and
`docs/book/ste.md` record the decisions in place:

| Finding | Decision | Where it lands |
|---|---|---|
| R10 | Crisp figures, not sketched | set art k144 |
| R11 | About 450 words of prose, then a figure, a table or a code block | check k133, then every unit draft |
| R12, R13 | “unchanged”, and “return no result to the caller” | unit copy-edits |
| R14 | A harness lets a caller use a subscription instead of paid API access | draft k134, README k146 |
| R15 | The contract with the configuration is loose | draft k134, README k146 |
| R16 | A second `end-session` is a race | drafts k134, k140, k142 |
| R17 | Homebrew install, and a version that the release writes | draft k136, impl k145, links in k138, k140, k142 |

The owner authorised the correction run on 2026-10-18. Each stage in it is
a re-run that the editorial rules make an escalation. In walk order, it is:
the prose check `book-prose-breaks-check-k133`, then a draft and a copy-edit
in each unit node (k134–k143), then set art `book-first-half-k144`,
`book-release-version-k145`, `book-readme-k146`, set proof
`book-first-half-k147` and the second half's planning
`ravel-book-second-half-k148`. The planning leaf comes before whole-book art
`ravel-book-k101`. Another send-back on a unit goes to the owner.

Draft `what-ravel-is-for-and-its-actors-k134` and copy-edit
`what-ravel-is-for-and-its-actors-k135` complete the chapters 1–2 part of
this run. Chapter 1 now says that Ravel writes the result unchanged to
standard output and does not interpret it (R12), and that some outcomes
return no result to the caller (R13). Both chapters pass the strict STE and
prose-break checks. No heading or anchor changed. This closes document node
`what-ravel-is-for-and-its-actors-k62` again. Set art k144 and set proof k147
complete its fourth correction run.

Draft `a-complete-session-k136` and copy-edit `a-complete-session-k137`
complete the chapter 3 part of this run. The reader installs Ravel with
Homebrew under the heading “Install Ravel and check its version” (anchor
`#install-ravel-and-check-its-version`). Chapter 3 now says that Ravel writes
the result unchanged (R12) and that refusal, cancellation and a natural exit
without a sent result return no result to the caller (R13). It passes the
strict STE and prose-break checks. This closes document node
`a-complete-session-k63` again. Set proof k147 closed its installation and
version handoffs on 2026-10-18. Chapter 3 has one version line with the
placeholder, and the release replacement and guards agree with it. The
inbound installation links use `#install-ravel-and-check-its-version` and
resolve. K144 closed the figure handoffs. The example check still uses the
checkout pair and fake agents, without a terminal.

Draft `writing-an-owner-policy-k138` and copy-edit
`writing-an-owner-policy-k139` complete the chapter 4 part of this run.
Chapter 4 links to `#install-ravel-and-check-its-version` (R17) and gives
the checkout prerequisites for its typecheck recipes. It says that Ravel
writes the result unchanged (R12) and that refusals and an agent exit
without a sent result return no result to the caller (R13). It passes the
strict STE and prose-break checks. No heading, anchor or code block
changed. This closes document node `writing-an-owner-policy-k64` again.
Set art k144 found the three new tables sufficient for the reader questions.

Draft `running-sessions-k140` and copy-edit `running-sessions-k141` complete
the chapter 5 part of this run. Chapter 5 links to
`#install-ravel-and-check-its-version` (R17) and names the race of a second
`end-session` call and the shutdown (R16). It says that Ravel writes the
result unchanged and does not interpret it (R12). It says that a natural
exit without a sent result, a cancellation and a failure of required cleanup
return no result to the caller (R13). It passes the strict STE and
prose-break checks. No heading, anchor or code block changed. This closes
document node `running-sessions-k65` again. Set art k144 closed its figure handoffs.

Draft `reference-chapter-k142` and copy-edit `reference-chapter-k143`
complete the chapter 6 part of this run. All seven installation links in
chapter 6 use `#install-ravel-and-check-its-version` (R17), and the
installed layout gives the Homebrew `PREFIX`. The `end-session` entry
names the race of a second call and the shutdown (R16). Chapter 6 says
that Ravel writes the result unchanged and does not interpret it (R12).
It says that cancellation, a failure of required cleanup and the signal
statuses return no result to the caller (R13). Nine page tables break the
command-line prose (R11). It passes the strict STE and prose-break checks.
No heading changed. This closes document node `reference-chapter-k66`
again. Set art k144 closed its figure handoff.

Set proof `book-first-half-k147` completes this correction run. It read all
six chapters, their tables and figure text together against R10–R17, the
contract, shared word decisions, glossary, specifications and ADRs. No
chapter correction was needed. It cleared the opening unit's stale R16
handoff and repaired a broken sentence in the chapter 3 brief. All required
checks passed on unchanged inputs, including 339 project tests, 13 complete
examples and nine figure comparisons. All 429 local destinations resolve,
and all nine figures load in Chrome. The task records the controls, digest,
source inventories and viewing limits.

No first-half defect is handed forward. The second-half handoffs above
retain their owners: k148 plans that half, k101 checks the completed book's
visual style and dense figure text, and the later proof checks the finished
book. Chapter 12 owns the release-version teaching. This proof closes no
parent node and cuts no correction run. No ADR decision changed.

## Figure migration

Art `book-first-half-k130` corrected all nine figure sources to STE and
“send”, regenerated their SVGs and layout records, and removed the repeated
colour descriptions from the chapter prose (R8). Each key retains its colours
and meaning. No technical fact, notation or renderer pin changed. The strict
STE check passes on all six chapters and nine layout records. The figure
comparison, book build and examples pass on unchanged inputs (13 examples,
zero failures or skips). All nine figures were inspected in the served book
at chapter width and full size, and a missing-figure control failed. The
dated record in `docs/book/figures.md` states the inspection limits. The dense
figures' small text remains with k101. README/help k131 and set proof k132
are complete. No new leaf is needed.

Set proof `book-first-half-k132` read the six chapters, README, glossary,
contents and figure text together against the STE writing rules, shared word
decisions, both specifications and the three ADRs. It corrected three chapter
6 notes under Rule 5.5: notes give information, not instructions. R3–R9 agree
across the set and with the source. Fresh strict STE and its controls, book
build, nine figure comparisons, 13 complete examples and `task check` all
passed on unchanged digested subjects (339 tests passed, 46 skipped).
All 429 local source destinations resolve, with broken-anchor/file controls
rejected. Chrome confirmed every figure loaded at chapter width. The task
records the source inventories, visual sample and unavailable views.
No first-half defect is handed forward. The second-half handoffs above keep
their owners, and the book node stays open. The owner's resumed read k70
then held read 1 from the beginning (see “Read 1 outcome” below).

The owner decided on 2026-10-17 to move every figure to Typst/CeTZ ("migrate
full") and later asked for a hand-drawn style. `book-figure-migration-k108`
planned it as ten leaves ahead of read 1: `book-figure-engine`,
`book-figure-sketch-style`, the five port leaves, the PlantUML retirement and
the viewing check. The plan, the decisions (SVG text, one source per figure,
`_*.typ` modules, pins) and the layout-cost table are in that leaf's `## Plan`.
`book-figure-retire-plantuml-k117` has landed: the contract and `figures.md` state
Typst/CeTZ only (pins, one `.typ` source per figure, `_*.typ` modules, SVG text as
paths with alt text on the page, the drawing style, the layout record,
`_draw.typ` and the `.layout.json` check), and the PlantUML and d2 mechanics, the
Java requirement and the `.puml` fixtures are gone. Chapters 7–12's figures are
drawn on the new renderer.

The owner then chose (2026-10-17, `layout-draw-separation-k119`) to separate layout
from drawing: each layout module returns a layout record, one `_draw.typ` draws any
record under a named style, and each figure's record is committed and compared as
`<figure>.layout.json`. On 2026-10-18, in read 1 (R10), the owner replaced the
sketched style (roughness 2) with the crisp style (roughness 0), after a view
of all nine figures in both styles. Set art `book-first-half-k144` applied
it. The nine layout records did not change. `book-figure-layout-record-k120` re-cut
the ports for that:
`book-figure-record-and-draw-k121` and `book-figure-layout-json-check-k122` build
the seam ahead of `book-figure-sequences-k111`, and the ports k111–k116 produce
records through their modules and draw only through `_draw.typ`. The reserve
sketch backend, Rough.js on Bun, has no use with the crisp style. Every
handoff leaves `task docs:figures` green.

## Crisp art boundary (k144)

All nine figures in chapters 1–6 use the crisp style. The sketch code and
its controls are gone. Each SVG matches the former renderer's explicit
crisp output, and the nine layout records remain byte-for-byte unchanged.
The five units request no new diagram: their R11 tables carry the named
relations and have adjacent role text. Art read all six chapters and closed
its handoffs. The figure captions and alt text in chapters 3 and 5 use R12
and R13. The unchanged records retain their short diagram labels.

The figure comparison and controls, strict STE and prose-break checks, book
build and ShellCheck pass. Chrome loaded every figure at the 750-pixel
column, and an absent-image control failed. `docs/book/figures.md` records
the dated view, sizes and limits. Small text in dense figures stays with
whole-book art k101. Proof k147 is already live. It reads the whole set,
including the retained label wording and its adjacent explanation. No new
proof leaf is due, and the book node stays open.

## Second half: units and shared conventions

Cut by `ravel-book-second-half-k148` on 2026-10-18, after read 1's corrections:
two scaffold increments, five drafting units with two figure modules, a set
proof, two guide fold-in leaves, whole-book art `ravel-book-k101` and read 2
`book-read-two-k166`. Everything stays inside this node. Chapter 10 is not here.

- **Scaffold first, each seen to fail before any chapter relies on it.**
  `book-ste-all-chapters-k149` makes the default `task docs:ste` examine chapters
  7–12 (the default list stopped at `0[1-6]`). `book-source-excerpts-k150` builds
  the source-excerpt check (`task docs:excerpts`) and its convention. The
  example check fails closed on an unlabelled `ts` or `sh` block, so the
  convention must make both checks agree on one page. The excerpt check comes
  before the draft of chapter 8.
- **Units**, each a document taking draft → copy-edit → art → proof under
  `docs/book/contract.md`, in the book's order and with file names taken from
  the final chapter titles (the first half kept the former names, so these are
  the new convention for chapters 7–12):
  - chapter 7, “The parts of Ravel”: `the-parts-of-ravel-k151`, one draft
    session. The plan's title “How Ravel is built” is passive with a known agent
    (STE Rule 3.6), so the planning leaf changed it. The draft edits the
    contract row, `SUMMARY.md`, `docs/book/ste.md` and chapter 1's route table;
  - chapter 8: `follow-a-session-through-the-code-k154`, two draft sessions (A
    the successful session; B the forks, the precedence and where a change
    belongs), method `linkuistics:writing-code-walkthroughs` at orientation
    depth, source excerpts under the excerpt check;
  - chapter 9: `jobs-signals-and-the-terminal-k157`, two draft sessions (A the
    job and its end; B the terminal, the entry state and the guarantees);
  - chapter 11: `develop-ravel-k159`, two draft sessions (A change code and
    check it; B the other workflows, the guards and the evidence documents);
  - chapter 12: `deliver-ravel-k161`, two draft sessions (A versions, archives
    and the checks before a cut; B the formula, the release tasks, the book's
    version line and recovery).
  The two-session split is cut lazily by part A as its last act, as in the first
  half. Each unit's art stage owns that unit's figures; the figure modules for
  the two notations the engine lacks come first:
  `book-figure-activity-k153` (with the outcome-precedence figure, before
  chapter 8) and `book-figure-timing-k156` (with the escalation timing figure,
  before chapter 9). The release activity figure of chapter 12 uses the activity
  module and is drawn by that unit's art stage.
- **One set proof** (`book-second-half-k163`) reads chapters 7–12 as a set
  after the last unit, because inventories 4 (component obligations) and 5
  (contributor workflows), the facts two chapters state, the shared figures and
  the links from chapters 1–6 span the units. It turns the plain-text mentions
  of chapters 7–12 in chapters 1–6 into links and decides whether chapter 6
  gains an entry for the non-`--json` operation report.
- **Guide migration** follows the set proof and precedes the whole-book art
  pass, so that “absorbed” is a statement about proved chapters:
  `book-architecture-fold-in-k164` (the component, local runner and result
  channel guides and the Mermaid views) and
  `book-contributor-delivery-fold-in-k165` (the release guide, the authority
  note, the README reduction, the contract clause and the live script and task
  references). Frozen evidence receipts that name a removed file are not
  edited. A sentence with no home goes to the owning chapter by a correction run.
- **Whole-book art** `ravel-book-k101` follows the migration. Its body tells it
  to cut the whole-book proof before read 2 with `leaf-insert` at read 2.
- **Read 2** `book-read-two-k166` is a `requirements` leaf after the whole-book
  proof, carrying the contract's read 2 tasks.
- **Source facts the units must not take from the guides.** `docs/RELEASING.md`
  says that development is 0.0.0 and the first minor release is planned at
  0.1.0, but the tags `v0.1.0` and `v1.0.0` exist and the workspace is at 1.0.0.
  The guides use “front”, “admission”, “publication” and “settle”; the book does
  not. `Taskfile.yml` descriptions still say “front” in places, and chapter 11 may
  reword them when it quotes them.
- **Conventions for chapters 7–12** are those of the first half, with read 1's
  additions: STE writing rules and `docs/book/ste.md`; “Ravel CLI”; “send”;
  “unchanged” and “return no result to the caller”; runs of prose of 450 words
  or fewer; the crisp figure style; the reason for a harness and the loose
  contract. A command listing is a fragment or a complete example. Examples
  never launch a real agent. A figure follows `docs/book/figures.md`.

### Owners of the second-half handoffs

Chapter 11 completes all four stages with proof `develop-ravel-k181`, closing
`develop-ravel-k159`. The fresh inventory has all 57 public task names once,
with no omissions, extras or duplicates. Development teaching is complete.
Chapter 12 still owns the ten release tasks and their teaching. The proof log
maps every README Develop statement, the private-channel paragraph and both
authority-note paragraphs for fold-in k165. All nine evidence families and the
instrument families have destinations. The guides stay until the planned fold-in.

Proof corrected 21 links outside the book source directory. mdBook rewrites
relative links to those files into absent HTML pages. The chapter now uses
repository source URLs on `main`, with a checkout instruction before the change
reaches `main`. Future link checks must examine rendered paths as well as source
paths. All thirty local chapter destinations resolve against built HTML IDs.
The repository paths and anchors exist in the checkout. Their remote content
and GitHub rendering remain unchecked until push and integration.

All six required checks pass on unchanged digested inputs: build, thirteen
figure comparisons, fourteen complete examples, eight excerpts, strict STE and
strict prose breaks. The nine fences retain their content and the four Rust
fragments match the preserved scratch patch. The earlier timeout and unchanged
passing rerun remain dated observations with an unknown cause.

Chrome supplied local light-page views at the 750-pixel column, including the
evidence and task tables. VS Code supplied a light Restricted Mode preview of
the opening through the build table. Raw files supplied the whole-document
read. Other themes, narrower views, GitHub and GitHub Pages were not inspected.
No reader-comprehension or platform qualification follows. No chapter-local
defect is handed forward and no ADR changes. The later set proof, fold-in,
whole-book art/proof and read 2 keep their existing owners.


Chapter 9 completes draft, copy-edit, art and proof on 2026-10-18, closing
`jobs-signals-and-the-terminal-k157` with proof k177. The chapter teaches O6
(matching reap and confirmed group absence), O8 (conditional foreground/mode
recovery) and the process and entry-state side of O7. It distinguishes child
signal death from CLI cancellation, detached execution from confinement, and
spawn-reported success from confirmed exec. Proof corrected the terminal
figure's SIGTTOU suppression/write order and added four first-use term links.

All six required checks pass: thirteen figure comparisons, fourteen complete
examples, eight excerpts, build, strict STE and prose breaks. All forty local
chapter-body link/image occurrences resolve, with missing-file/anchor controls
rejected. The measured subjects stayed unchanged during each finished run.

Fold-in k164 retains the full guide sentence-group destinations in draft
k158 and k174, reconciled by proof k177. `local-runner.md` Boundary maps to
chapters 7–8's interfaces and chapter 9's escalation, modes, confinement and
reports. Native entry and return maps to chapter 9's terminal, signal-state
and guarantee sections. Chapter 8 retains observers, result association and
failure priority; chapter 11 retains test commands and authority guards.
Provenance stays outside the book. `components.md` O4/O6/O7/O8, R5/R7,
successful operation, cancellation, C4 and native-entry limits have the same
chapter 9 destinations. Old entirely-unmeasured native-entry claims are
replaced by the qualified existing aarch64 support statement. No guide is
removed by the chapter proof, and no independent-use contract is inferred.

Chrome supplied complete chapter-width views of both figures and keys, plus
their adjacent tables and the guarantee table. Both SVGs opened at full size,
with opening regions inspected. VS Code supplied a light Restricted Mode
preview of the opening, group comparison, escalation table and timing opening.
Raw files supplied the whole-document read. Other themes, narrower views,
GitHub and GitHub Pages were not inspected. The external POSIX exec link
returned HTTP 403 to the web reader and was not revalidated. No new runtime,
platform or reader-comprehension result follows.

No chapter-local defect is handed forward. Dense figure text and the existing
378,003-byte terminal SVG / 2,912,522-byte set assessment remain whole-book art
k101's work. The earlier size benchmarks are not claimed to pass. The book
node remains open for its later units, set proof, migration, whole-book art
and proof, and read 2. No ADR changes or correction run are needed.

Chapter 8 completes draft, copy-edit, art and proof on 2026-10-18, closing
`follow-a-session-through-the-code-k154` with proof k173. Its successful code
path, refusal/cancellation forks, five-source signal priority and final
failure priority are checked against production source. The full sequence
keeps the runner inside the CLI and shows the second result-sender CLI.
Proof added the missing first-use result-channel link in the opening table.
All six required checks pass: eleven figure comparisons, eight excerpts,
thirteen complete examples, build, strict STE and prose breaks. All 32 local
link occurrences resolve, with missing-file/anchor controls rejected.

K155/k170 retain the guide-statement destinations for fold-in k164. Proof
k173 re-enumerates all ten obligations and reconciles their final destinations.
K170's former “Clean up and write the result” heading is now “Complete cleanup
and write the result”. O3/O4/O5/O7/O9 are taught here; chapter 9 retains O6/O8
detail, terminal/backend mechanics and resource-recovery limits, and chapter 11
retains test commands and the independent process fixture. The set proof still
decides chapter 6's non-JSON operation-report entry and cross-chapter links.

Chrome supplied light-page views of all four figures and the key tables at
chapter width; VS Code supplied a light Restricted Mode preview of the opening,
path table and sequence opening. Full-size sequence/activity opening regions
were inspected. Raw files supplied the whole-document read. Other themes,
narrower views, GitHub and GitHub Pages were not inspected. No reader result
or new runtime/platform qualification follows. No chapter-local defect remains
unowned. Dense figure text and the eleven-figure set's size remain with k101.
The book node remains open for its later chapters, set proof, migration, whole-
book art/proof and read 2. No architectural decision changed.

Chapter 7 completes draft, copy-edit, art and proof on 2026-10-18, closing
`the-parts-of-ravel-k151` with proof `the-parts-of-ravel-k169`. The chapter
explains the CLI, separate TypeScript worker, SDK, private Rust runner and
result channel, with the loose program contract and matched installed pair.
Proof rechecked source and corrected the worker environment to include the
CLI's explicit Bun cache control. O1, O2 and O10 are covered here; k152's
complete obligation, R1–R8 and guide sentence-group destinations still hold.
Chapters 8–9 retain the detailed O3–O9 work, chapter 11 the test/isolation
material, and k164 the architecture guide fold-in after set proof.

All six required book checks pass, including nine figure comparisons and
13 complete examples, with unchanged digested subjects. All 31 chapter-body
links resolve. Chrome showed the eight tables legibly on a light page at
chapter width; raw Markdown supplied the whole-file read. Editor preview
could not be reached, and other themes, narrower views, GitHub and GitHub
Pages were not inspected. No reader-comprehension or new runtime/platform
claim follows. No defect is handed forward from this node. Dense shared
figure text remains with k101, and the set/whole-book proofs retain their
existing cross-chapter work. No ADR changes or correction run are needed.

Chapter 7's draft (`the-parts-of-ravel-k152`) records the complete O1–O10 and
R1–R8 destination tables and the sentence groups of both architecture guides
in its task log. Chapters 8 and 9 take their detailed obligations from that
map and verify them against current source. The map also names chapter 11's
runner-test and isolation material. The architecture fold-in uses these
destinations after the unit stages and set proof, without treating the old
guides' native-entry status as current fact.

| Handoff (from “Handed forward to the second half”) | Owner |
|---|---|
| Plain-text mentions of chapters 7–12 become links | `book-second-half-k163` |
| `docs/authority.md` guards and exceptions into chapter 11; file removed; caller-references row and contract clause edited | chapter 11 (`develop-ravel-k159`), then `book-contributor-delivery-fold-in-k165` |
| README “Develop”, the “private session channel” paragraph and the contributor sentences | chapter 11, then `book-contributor-delivery-fold-in-k165` |
| Non-`--json` operation report | chapters 8 and 9 teach it if the path meets it; `book-second-half-k163` decides chapter 6 |
| README reference paragraph reworded to the chapters; the notices line moves to chapter 12; the “Earlier project” heading | `book-contributor-delivery-fold-in-k165` (chapter 12 holds the notices line) |
| Release version teaching (R17) and the two release paragraphs | chapter 12 (`deliver-ravel-k161`) |
| The loose contract (R15) in agreement with chapter 1 | chapter 7 (`the-parts-of-ravel-k151`) |
| The packages, worker and SDK figure reused | chapters 7 and 12 (art stages) |
| The dense figures' small text | `ravel-book-k101` |
| Chapter 9 art k176 records a 378,003-byte terminal SVG and a thirteen-figure set of 2,912,522 bytes. These exceed the earlier 300 KB per-figure and 2 MB set benchmarks. Its new text is about 11.3 px at the 750-pixel column | `ravel-book-k101`, with the existing size and dense-text assessment |
| Chapter 5 says “managed program” three times (in “Read notices separately from results” and the exit-status table) where the rest of the book says “managed process”. Copy-edit `the-parts-of-ravel-k167` changed chapter 7 to “managed process” | the whole-book proof that `ravel-book-k101` cuts |
| Chapters 5 and 6 write one signal two ways (SIGINT and INT, SIGTERM and TERM). Copy-edit `jobs-signals-and-the-terminal-k175` set the short-name rule in `docs/book/ste.md` (“Signal names”) and applied it to chapter 9 | the whole-book proof that `ravel-book-k101` cuts |
| Guides `components.md`, `local-runner.md`, `session-channel.md` and `design/components/` | chapters 7–9, then `book-architecture-fold-in-k164` |
| Guide `RELEASING.md` | chapter 12, then `book-contributor-delivery-fold-in-k165` |
