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
  the CLI/real-policy/fake-program seam, and the source-excerpt check. `task
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
instructions” and bare “routing”, which chapters 3–6 also use. For set art
k130: k125 converted both alt texts and the prose before both figures, and
kept their content. R8 remains in the paragraphs “The C4 system-context
view…” (chapter 1) and “This C4 container view…” (chapter 2). The figure
labels “Publishes opaque result; asks Ravel to stop” (`system-context`, also
a semicolon) and “Publishes result via end-session” (`process-authority`)
remain. If a label changes, make the alt text agree. This closes document
node `what-ravel-is-for-and-its-actors-k62`. Set art k130 and set proof
k132 in this node complete its third correction run.

Copy-edit `a-complete-session-k126` converted chapter 3 to STE and “send”.
It passes `task docs:ste -- --strict`. Its heading “Publish the result and
finish the job” is now “Send the result and finish the job”, and chapter 5's
one link to it changed. For set art k130: k126 converted both alt texts and
both captions, and kept their facts. R8 remains in the paragraphs “In this
UML sequence…” and “In this UML state machine…”. These labels and key lines
still use the old words: in `successful-session`, “Publish opaque result via
end-session”, “Validate and read published result”, “Launch and
publication”, “First complete publication wins. Publication does not wait
for cleanup.” and the key line “Publication precedes cleanup…”. In
`session-outcomes`, “publication and job ending, …” and the key lines with
“published value”, “absent publication”, “An empty publication is a value”
and “delivering”. If a label changes, make the alt text agree. This closes
document node `a-complete-session-k63` again.

`book-first-half-k123` wrote `docs/book/ste.md` (the result words, the
titles, the words that the book does not use and the “-ing” words that it
keeps), the check `task docs:ste` with its controls `task docs:ste:test`,
`CONTEXT.md` in STE and the new titles in `SUMMARY.md`. Each later stage of
the run passes the strict check on the files that it owns.

## Figure migration

The owner decided on 2026-10-17 to move every figure to Typst/CeTZ ("migrate
full") and later asked for a hand-drawn style. `book-figure-migration-k108`
planned it as ten leaves ahead of read 1: `book-figure-engine`,
`book-figure-sketch-style`, the five port leaves, the PlantUML retirement and
the viewing check. The plan, the decisions (SVG text, one source per figure,
`_*.typ` modules, pins) and the layout-cost table are in that leaf's `## Plan`.
`book-figure-retire-plantuml-k117` has landed: the contract and `figures.md` state
Typst/CeTZ only (pins, one `.typ` source per figure, `_*.typ` modules, SVG text as
paths with alt text on the page, sketched at roughness 2, the layout record,
`_draw.typ` and the `.layout.json` check), and the PlantUML and d2 mechanics, the
Java requirement and the `.puml` fixtures are gone. Chapters 7–12's figures are
drawn on the new renderer.

The owner then chose (2026-10-17, `layout-draw-separation-k119`) to separate layout
from drawing: each layout module returns a layout record, one `_draw.typ` draws any
record under a named style, and each figure's record is committed and compared as
`<figure>.layout.json`. The figures ship sketched at roughness 2 with keys and
tables crisp. `book-figure-layout-record-k120` re-cut the ports for that:
`book-figure-record-and-draw-k121` and `book-figure-layout-json-check-k122` build
the seam ahead of `book-figure-sequences-k111`, and the ports k111–k116 produce
records through their modules and draw only through `_draw.typ`. Rough.js on Bun
is held in reserve as a drawing backend at the same seam. Every handoff leaves
`task docs:figures` green.
