# ravel-book-k55 — brief

## Goal

Write the Ravel book to its contract, `docs/book/contract.md`: a pedagogical
user guide that lets a software engineer new to Ravel's context use, modify,
maintain and reuse Ravel, drafted against this workstream's current code. The
owner settled the contract in `book-structure-k5` on 2026-10-16.

## Done when

- Chapters 1–9, 11 and 12 exist under `docs/book/src/` in the contract's order,
  each taken through draft → copy-edit → art → proof under the contract.
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
- Versions pinned in the contract (mdBook 0.5.4, PlantUML 1.2026.8 with
  C4-PlantUML 2.13.0, d2 0.9.0) were current on 2026-10-16; recheck a dated
  fact before a choice turns on it.

## First half: units and shared conventions

Cut by `ravel-book-first-half-k56`. Positions 02–05 are the scaffold and one
small product fix, 06–10 the five drafting units, 11 the set proof, 12–13 the
guide migration and 14 read 1. Chapters are drafted from the code and the
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
- `docs/authority.md` is reduced, not removed, by the first half: its
  test-isolation guards and legacy-name exceptions belong to chapter 11 and to
  `docs/caller-references.md`, and the file is removed when chapter 11 lands.
- Contributor sentences from `docs/session-cli.md` that have no other home
  move into the README's Develop section (kept until chapter 11 absorbs it).
- Contributor and delivery parts of the README stay until chapters 11 and 12.
- The visual vocabulary and the first figures' conventions are the first
  half's art output; chapters 7–12 follow them.
- The first figure for "packages, worker and SDK, in source and in an
  installation" appears in chapter 6; chapters 7 and 12 reuse it.

## Handed forward to the first-half set proof

- Chapter 4 completed all four stages. Proof re-enumerated its configuration
  coverage from the SDK, settings, bounds and validators. Its local correction
  adds `message_too_large` for an oversized loader failure report; chapter 6
  must cover that context-stage use as well as policy snapshots and selection
  results. A temporary loader confirmed exit 3 and empty stdout. All seven
  complete book examples pass (six in chapter 4), four figures match and the
  book builds without warnings. The node's guide-disposition table remains the
  fold-in input; its dated personal handoff still needs preservation outside
  the book. `docs/book/figures.md` records proof's separate local Chrome/raw
  viewing and the unavailable editor preview, remote views and other viewing
  limits. These checks establish no human comprehension, real-agent operation,
  agent permissions or terminal interaction. No correction run remains.

- Chapter 4 part B supplies context/settings/grants/imports/agent-route/copying
  teaching. Chapter 6 re-enumerates the SDK exports, configuration shapes and
  full admission/selection/context codes, all six bounds, settings error codes,
  every excluded grant class (including `HARNESS_DISPATCH_*`), managed versus
  worker environments, embedded literal imports/package alias limits, and
  installed paths. Its entries link back to chapter 4's task sections. The
  document-node brief now completes the owner-configuration guide disposition;
  the dated personal handoff still awaits the planned preservation/migration.
  Agent help was rechecked on 2026-10-16: Claude Code 2.1.301 and Codex 0.163.0.
  The fake-route evidence establishes argument delivery only, not real-agent
  execution, permissions, terminal interaction or human comprehension.

- Chapter 3 completed its four editorial stages without a proof correction.
  Its complete example passed (one example; installation remains a labelled
  fragment). Preserve its distinction between exact stdout/status and the
  stable stderr excerpt, which permits retained operational reports. Forward
  chapter mentions remain plain text until their pages exist; link chapters
  4–6 from chapter 3 in the set proof, with later targets handled by the
  second half. `docs/book/figures.md` records chapter 3's separate art/proof
  viewing coverage and limits. Neither supplies terminal-interaction or human
  comprehension evidence; the owner's stand-in read remains outstanding.
- Opening-unit proof checked the corrected worker-directory sequence against
  source and found no further chapter correction. `docs/book/figures.md`
  records art and proof's separate local viewing coverage and the remote,
  theme and viewport limits. Keep those limits distinct from the owner's
  later stand-in read and the release's publication checks.
- The opening unit was drafted before chapters 3–6 existed, so its forward
  mentions use chapter numbers and titles. Once those targets exist, link the
  mentions in chapters 1 and 2 as part of `book-first-half`; chapter 6 links
  stay page-level. Later-half targets follow the existing rule above.
- Guide fold-in: chapter 1 qualifies `session-cli.md`'s “without database
  access” as no Ravel-owned persistent store. Trusted owner policy can make
  direct filesystem/network reads. It also locates unchanged prompt delivery
  at the caller-to-policy boundary; the policy constructs the selected
  program's arguments. Preserve these distinctions when reconciling the guide.
