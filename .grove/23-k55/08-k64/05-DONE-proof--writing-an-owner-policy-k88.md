# writing-an-owner-policy-k88

## Goal

Proof the whole chapter `docs/book/src/04-writing-an-owner-policy.md` against
`docs/book/contract.md`, under the editorial family's proof procedure. Check
its teaching, technical truth, prose, examples, tables and direct entries
together; re-enumerate the chapter's configuration coverage from the subject.

## Context

- The node brief carries the two-part draft, guide disposition and chapter 6
  handoff. Chapter 4 is one document, not only the figures changed by art.
- Art added page-native tables for evaluation order, caller/loader document
  combinations, worker versus unconfined managed-process environments, and
  import versus host-read resolution. No new SVGs. Each table has an adjacent
  takeaway. Keep timeout distinct from refusal, required worker cleanup before
  launch, loader replacement without automatic merging, and environment grants
  distinct from managed-process confinement.
- `docs/book/figures.md` records the local light-page inspection and its limits.
  Proof makes its own reachable viewing checks. Neither art nor mechanical
  validation establishes human comprehension, agent execution or terminal
  behaviour. GitHub and publication remain unverified for this unpublished book.
- Art's final figure, example and book checks pass: four SVGs match, seven
  complete examples pass (six in this chapter), no failures or skipped examples,
  four fragments not run, no book warning. All code blocks and example labels
  are unchanged from copy-edit. Re-run the repository gates on proof's output.
- The agent flags retain the draft's dated help provenance. The examples use
  fake programs and establish argument delivery, not agent permissions or
  obedience. The installed typechecking recipe uses a separate development
  checker; no installed execution dependency on it is claimed.
- No earlier-stage defect or correction run was identified by art. No later
  sibling is queued. This is the document's first proof run.

## Done when

- The complete chapter passes the contract's proof, inventory and direct-entry
  checks; fixes stay within the proof procedure, with any earlier-stage
  correction routed as its ordered run.
- `task docs:figures`, `task docs:examples` and `task docs:book:build` pass.
- Actual viewing coverage and remaining limits are recorded without claiming
  human comprehension. If the document node closes, check its done criteria
  and promote the relevant context to the book brief before retirement/commit.

## Decisions (running log)

- The whole-chapter read found a missing `message_too_large` entry in the
  table claiming complete policy-context diagnostics. `choice::context_breach`
  routes a context-stage message breach to `policy::message_too_large`, whose
  context remedy explicitly names an oversized loader error. Proof adds that
  row and includes failure reports in the protocol-bound explanation. This is
  a local correction within proof's unrestricted charter; no earlier-stage
  re-run, new design or ADR change is needed.

## Configuration inventory from the subject

The inventory was derived from the SDK, settings reader, limits constructor,
context validator, policy validator, choice judgement and worker assembly,
then compared with the whole chapter. Graph evidence used Tier 2 in project
`Users-antony-Development-Ravel.simplify-and-componentize`, generation
`2026-10-16T04:47:44Z`. Coverage returned no recorded issues with matching
metadata for the relied-on paths; this is a best-effort signal, not a claim
of graph completeness. The source was read directly as well. The settings
trace contains heuristic name collisions, so only exact source establishes
the settings-before-overrides order.

| Source-derived items | Chapter coverage |
|---|---|
| Policy: `schemaVersion`, `version`, `select`, `loadContext`; named plain-object export and unknown-field refusal | Define a policy; select/refuse; supply policy context |
| Request: `schemaVersion`, `kind`, `prompt`, `cwd`, `params`, `taskFile`, `taskId`, `context`, `limits` | Request table; caller data distinguished from enforced limits |
| Selection: `status`, `program`, `args`, `provider`, `model`, `effort`, `reason`; refusal: `status`, `code`, `message`, `remedy` | Selection/refusal tables and fake-route cases |
| Context: `schemaVersion`, `summary`, `acceptanceCriteria`, `facts`, `assessments`, `sources`; assessment `by`/`value`; source `name`/`sha256`/`bytes`/`version`; attached measurement `name`/`via`/`bytes`/`sha256` | Supply policy context; replacement cases, attribution versus measurement, reserved marker and both source-count limits |
| Host: `diagnostic`, `signal`, `readText`, `readJson`, optional `maxBytes`; read returns `text` or `value` beside `source` | Define a policy and supply policy context; reads only during loading, relative to caller cwd, sticky bound breaches |
| Settings: `timeoutMs`, `contextBytes`, `policyEnv`, with defaults/ranges and refusal of every other key | Settings table and prose; invalid settings precede CLI replacements, grants add |
| Limits: `selectionMs`, `contextBytes`, `sourceBytes`, `sources`, `messageBytes`, `diagnosticsBytes` | Six-row bound table; timeout separate from refusal and managed-process runtime |
| Worker base names, host cache value, exact excluded names/prefixes | Environment prose, exclusion table and worker/managed-process comparison |
| Literal embedded modules, relative imports, package lookup/aliases, autoload switches and root-resolution residue | Resolve policy imports |
| SDK exports: `Json`, `SourceRecord`, `Assessment`, `Context`, `MeasuredSource`, `DeliveredContext`, `ReadSource`, `SelectHost`, `ContextHost`, `LoadContext`, `Policy`, `SelectionRequest`, `Limits`, `Selected`, `Refused`, `SelectionResult`, `CHOICE_FILE`, `readChoice`, `definePolicy` | Shapes and operations are taught; named type-by-type lookup remains chapter 6's existing mandate |
| Static exports: `Command`, `Route`, `agent`, `routes`, `selectRoute`, `policy`; sample choice logic and its refusals | Starter and choice sections; default/careful across every route, unknown kind, count, unoffered and unreadable cases |
| Context codes from context validation and choice judgement, including context-stage message overflow | Context diagnostic table, with proof's missing-row correction; global output overflow and timeout explained under bounds |

Full policy-admission/selection code lookup, CLI input constraints and installed
path lookup remain the already-planned chapter 6 inventory, not a claim that
chapter 4 supplies those reference entries. The guide-disposition table in the
node brief was checked against the unchanged guide outside its dated handoff;
the chapter 6/7 destinations remain as recorded there.

## Verification and closure

- A temporary loader throwing a 1 MiB error produced `message_too_large`,
  stage `context`, exit 3 and empty stdout. Its diagnostic reports a
  1,048,640-byte failure frame against the 1,048,576-byte bound and asks for a
  shorter loader error. Policy and built-pair hashes were unchanged across
  this check. Files are in `/tmp/ravel-k88-proof.HjJ7sR` (disposable local
  evidence, not a durable dependency).
- After the final chapter edits, `task docs:examples` passed all seven complete
  examples, none failed or skipped, four fragments not run, no page errors.
  Per-example records identify chapter 3 at line 69 and chapter 4 at lines
  152, 430, 553, 867, 1212 and 1465. All fenced blocks and example/fragment
  labels compare byte-for-byte with the predecessor chapter.
- `task docs:figures` passed: four figures match. `task docs:book:build`
  passed without warnings. The checks used one writer per output. A manifest
  of 162 subject files (all files under docs/book, scripts and crates excluding
  node_modules, plus Taskfile/Cargo manifests and the built front/worker) was
  digested before these final runs and verified unchanged afterwards. Logs and
  the subject manifest are in that same temporary directory.
- Proof inspected the local Chrome light-page tables and surrounding text,
  including the corrected row after rebuild, and read the whole raw chapter.
  `docs/book/figures.md` records the exact viewing scope. VS Code did not open
  this chapter; no editor preview, other themes, narrower viewports, GitHub or
  published-site evidence is claimed. No human comprehension, real-agent,
  permissions or terminal-interaction evidence follows from these checks.
- No in-session reviewer was materialised. No handed-forward defect remains
  and no correction run is needed. Node k64 meets its done criteria: chapter
  present and linked, four stages complete, route/refusal checks pass,
  source-derived settings/bounds/context coverage reconciled, guide disposition
  complete. Its reference and guide-migration obligations and viewing limits
  are promoted to the book brief; close `writing-an-owner-policy-k64`. The book
  node still has live siblings; no further parent closes. No review names this
  leaf or the document node, and no provider line needs updating.
