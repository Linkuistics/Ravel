# writing-an-owner-policy-k64 — brief

## Goal

Chapter 4 (`docs/book/src/04-writing-an-owner-policy.md`), taken through draft
→ copy-edit → art → proof under `docs/book/contract.md`. After it the reader can
write, typecheck and validate a policy with fake agents, covering the SDK,
refusals, policy context, settings, environment grants, routes to real agent
programs and copying an existing policy.

## Decomposition

One document drafted in two sessions, then the three remaining stages once.
Part A is the anatomy of a policy and its validation with fake agents; part B
is everything a policy then needs to be used for real. The seam is the point
at which a reader has a policy that routes and refuses correctly under fake
agents. Part B starts from that and cuts nothing out of part A.

- **Part A**: the policy file and `definePolicy`; the request fields; a
  selection and a refusal with their fields; the starter `ravel/examples/static`
  table and the editable `sample/policy.ts` with `readChoice`; typechecking
  against the generated declarations, in a checkout and from an installed
  prefix; validating every route and every refusal with fake agents on a private
  PATH with a cleared environment; running a policy with `--config`.
- **Part B**: policy context (the version-1 document, `loadContext`, host
  reads and their bounds, measured sources); `settings.json` (`timeoutMs`,
  `contextBytes`, `policyEnv`, unknown keys refuse, the obsolete `stateDir`);
  environment grants and the names never grantable; module resolution (what
  imports resolve, embedded names, package.json autoloading); routes to Claude
  Code (`claude`) and Codex (`codex`) with their actual flags, validated
  through fake executables of the same name; copying an existing policy.

## Context

- Needs chapters 1–3 and the examples check: every complete example runs
  through `task docs:examples`. Sources: `docs/specs/session-execution.md`
  ("Request, context and selection", "Policy authority and runtime discovery"),
  `crates/ravel/worker/sdk/index.ts`, `examples/static.ts`, `sample/policy.ts`,
  `docs/owner-configuration.md` (being replaced; verify, and re-verify its
  typecheck recipe against the installed declarations), `crates/ravel/tests/cli.rs`
  for the settings, grants and context cases.
- Real agent flags (part B): check each agent's flags against its own
  `--help` at a recorded version and date. Claude Code 2.1.301 and codex-cli
  0.162.1 were installed on 2026-10-16; recheck, and record the versions and the
  date in the chapter. Which flags and modes the owner's own policy uses today
  may be read from the owner's policy, but its routing table, labels and model
  names are personal and must not be copied into the book; the book's example
  policies are the book's. Installing the agents, accounts and credentials are
  out of scope with a pointer to each agent's own documentation. An
  interactive agent takes the terminal; say so where a route depends on it
  and point at chapter 9.
- The ending instructions a caller's prompt gives the agent are shown, because
  Ravel injects none.

## Done when

- Chapter 4 exists, is linked from `SUMMARY.md` and has passed all four stages.
- Each complete example runs under the checker; every route and every refusal
  a chapter policy defines is shown validated with a fake agent.
- The chapter states which settings keys, bounds and refusal codes exist by
  enumeration from the settings reader, SDK and spec, and chapter 6 entries
  will be consistent with it (they are written later from the same sources).
- Each statement from `docs/owner-configuration.md` outside the dated handoff
  record is either in chapter 4 or noted as chapter 6's, in this unit's log, so
  the guide fold-in can verify.

## Notes

- Part A cuts part B as its last act, ahead of copy-edit: cut a `draft` leaf
  with the bare stem for part B and its scope, then cut copy-edit as the
  family file directs (it lands after part B). Part B's session cuts nothing if
  copy-edit already stands.

## Part A to part B

The chapter's first five sections are drafted. Append part B's sections in this
order (these headings are deliberately not empty placeholders in the page):

1. `Supply policy context` — version-1 data, loader/host operations and measured
   sources, with complete examples and their refusals.
2. `Set bounds and grant environment names` — all settings keys, defaults,
   ranges, CLI replacements/additions, unknown keys including `stateDir`, and
   never-grantable names, enumerated from the implementation.
3. `Resolve policy imports` — embedded names, relative helpers, package lookup
   and autoloading controls/limits.
4. `Route to Claude Code and Codex` — date/version/help-checked flags, fake
   executables of the same names, ending instructions and terminal implications.
5. `Copy an existing policy` — preserve source and destination edits, reconcile
   imports/settings/helpers, and validate routes/refusals before real use.

Part A's policies are the chapter 3 fake extended with two explicit refusals,
the embedded static policy and the sample choice policy (expanded for reading).
The sample sets every careful route to `high`, including defaults at `xhigh`;
it is not a minimum-effort operation. The installed-declaration typecheck
recipe uses the checkout's pinned compiler as a development tool, with all
embedded-module mappings pointed at the installation. It does not claim an
installation bundles a compiler. All three policies are checked against both
declaration sets, and a numeric `program` is rejected by both recipes.

### Owner-configuration guide disposition

This is the completed sentence-group classification for the guide fold-in;
the fold-in leaf verifies absorption after this document's proof.
`docs/owner-configuration.md` itself remains unchanged here.

| Guide statements outside the dated handoff | Teaching destination |
|---|---|
| Default policy entry; `version` is the owner's label; explicit `readChoice`; caller kinds/labels/arguments | Part A: define/select/checkout-choice sections |
| Settings keys, absence defaults, all six bounds, CLI overrides and grants | Chapter 4: `Set bounds and grant environment names`; chapter 6 supplies lookup entries |
| Why executable TypeScript and why a separate worker | Chapter 2 already introduces the authority split; chapter 7 owns the architectural rationale (guide's optional-depth pointer) |
| Preserve both configuration directories, copy missing files, reconcile edits, SDK and relative imports, caller helpers, choices/bounds/grants, `stateDir`, no stored history, old installation independent | Chapter 4: `Copy an existing policy`, `Resolve policy imports` and `Set bounds and grant environment names` |
| `start-session` launches; no inspect/init; fake executables, cleared environment, temporary channels, bounded checks, captured arguments/exact result and refusals | Part A: select/validate sections teach the real-launch check and the book's bounded seam; chapter 6 owns the full verb inventory including absence of init |
| Generated public declarations, temporary absolute-path tsconfig, pinned checker, strict options, no checker needed for installed execution | Part A: typechecking; extends the guide to cover installed declarations and the static module mapping |

The dated personal handoff stays outside the book and is not copied into this
chapter. Part A establishes no human-comprehension result, real-agent run,
GitHub rendering or publication evidence. Those limits remain the contract's.

## Part B to copy-edit

The five reserved sections are now present in order. Part A's examples and
progression are retained; one sentence now scopes its no-runtime-types claim
to the policies introduced so far, because part B uses process.env.
Part B adds complete examples for a context loader, effective settings/grants
and agent routes with a relative helper. The context example tests its accepted
route with and without a caller document, its explicit not-ready refusal and
representative source/shape/bound failures. The grants example prints only
invented-name presence, not credential values. Both real-program routes and
their unknown-kind refusal run through fake `claude`/`codex` executables; their
returned argument vector tests delivery, not agent obedience or permissions.

Help provenance is 2026-10-16 AEDT: Claude Code 2.1.301 and codex-cli 0.163.0.
The latter differs from the earlier briefing snapshot. Model defaults and
Codex effort are labelled as configured defaults, not guessed resolved values.
The terminal/permissions paragraphs explicitly limit what fake tests establish.
The external setup links were opened in their official documentation.

One aimed reviewer read the context and import explanations against source.
Its two mechanical qualifications (reserved `$ravel` objects and the precise
unsupported-version case) were accepted after direct source verification.
No further review was materialised; copy-edit keeps its own charter.

The complete book's examples check passed (7 examples, none failed or skipped;
6 in this chapter); the final book build passed without warnings. Every chapter
policy and the relative helper typechecked against checkout and installed
declarations. The settings example uses process.env and needs runtime types
when checked; the standalone part A recipe's types: [] qualification still
applies to the policies it introduces. The leaf records the measurement and
unchanged-example checks, without promoting them to comprehension evidence.

Chapter 6's future lookup scope includes every SDK exported type/helper and
shape, all policy admission/selection/context refusal codes, all six bounds,
settings refusal distinctions, the exact never-grantable classes (including
`HARNESS_DISPATCH_*`), managed versus worker environments, literal embedded
imports and package-alias limits, and installed source/declaration paths.
Chapter 4 teaches the policy-context and settings refusal vocabulary here;
chapter 6 re-enumerates from source rather than treating this prose as authority.
The parent book brief carries this handoff to that sibling.

No art-owned defect was identified. Figures remain the art stage's decision
under the contract. Human comprehension, actual agent execution, terminal
interaction, editor/GitHub rendering and publication remain unmeasured by this
draft. The dated personal handoff remains untouched in owner-configuration.md
until the planned guide-migration leaf preserves it outside the book.

## Copy-edit to art

Copy-edit changed sentences and terms only: the chapter now uses chapter 1–3's
spellings (managed process, the managed process's group, native front,
session-result context for the inherited `RAVEL_EXIT_FILE` path) and links
each term-list word at its first use on the page. Every code block and example
label is unchanged, and the examples check and book build passed afterwards.
Nothing was handed forward to art or proof beyond the art leaf's own body.

## Art to proof

Art adds four page-native tables: evaluation order and stopping conditions,
caller/loader document combinations, the two process environments, and import
versus host-read resolution. Each has an adjacent takeaway; existing table
introductions now state their role where needed. No new SVG or renderer is
needed. The evaluation table distinguishes timeout from refusal and keeps
worker cleanup as a launch condition. The context table preserves replacement
without automatic merge, and the environment comparison is explicitly
unconfined and does not imply a policy sandbox.

`docs/book/figures.md` records the local light-page visual inspection and its
limits. Raw chapter reading and mechanical checks are not human comprehension
evidence. No earlier-stage defect was identified and no correction run was
cut. Proof still owns its whole-document, source-inventory and viewing checks.

## Handed forward

None. Copy-edit k139 closed this node on 2026-10-18. Its handoff to set art
k144 is now in the book brief.

## Proof closure

The document has completed draft, copy-edit, art and proof. Proof re-enumerated
configuration from source and added the missing context-stage
`message_too_large` code, clarifying that failure reports share the protocol
bound. A temporary throwing loader confirmed that outcome with exit 3 and no
stdout. All complete examples pass, every chapter policy's explicit route and
refusal is exercised, four figures match and the book builds without warnings.
`SUMMARY.md` links the chapter. The guide disposition above remains complete;
chapter 6's reference inventory and the dated-handoff preservation remain with
their existing siblings. The book brief receives these surviving obligations
and the viewing limits before this node closes. No ADR decision changed.

## Read 1 correction run (2026-10-17)

The owner's read 1 (`book-read-one-k70`) made ASD-STE100 the prose base for
all documentation, chose “send” to replace “publish” for the result, and
changed six chapter titles to commands. The owner did not read this
chapter yet, so these changes come from the contract, not from findings.
Copy-edit k127 converts chapter 4 to STE, with the word decisions in
`docs/book/ste.md` (from k123), and changes the title to “Write an owner
policy”. Chapter 4 has no figures, so set art has no work here.
Set art `book-first-half-k130` and set proof `book-first-half-k132` serve
this document. Thus, no stage under this node cuts an art or proof leaf.
Copy-edit k127 completed this conversion on 2026-10-18. Chapter 4 passes
the strict STE check. The heading “Copy an existing policy” is now “Copy a
policy from another configuration”, and chapter 6's three links use the new
anchor `#copy-a-policy-from-another-configuration`. Chapter 4 has no
figures, so set art k130 has no work in it.

## Read 1 correction run (2026-10-18)

The owner's full read 1 (`book-read-one-k70`) found R10–R17. Two reach this
document: its link to the installation in chapter 3 (R17) and runs of prose
that are longer than the new limit (R11). Draft k138 and copy-edit k139 run
under this node. They are the first draft re-run and the second copy-edit
re-run. The owner authorised both on 2026-10-18. Chapter 4 has no figures.
Set art `book-first-half-k144` draws a figure here only if draft k138 asks
for one. Set proof `book-first-half-k147` serves this document. Thus, no
stage under this node cuts an art or proof leaf.

Copy-edit k139 completed this run's node part on 2026-10-18. Chapter 4 now
says that Ravel writes the result unchanged (R12) and that refusals and an
agent exit without a sent result return no result to the caller (R13). It
passes the strict STE and prose-break checks. No heading, anchor or code
block changed.
