# the-parts-of-ravel-k152

## Goal

Draft chapter 7 as the document node's brief describes, under
`docs/book/contract.md`, into `docs/book/src/07-the-parts-of-ravel.md`, linked
from `docs/book/src/SUMMARY.md` under the new title “The parts of Ravel”.

## Context

- Chapters 1–6 are proved and the first half is closed. Read chapters 1 and 2
  first: this chapter uses their terms, the authority statement and the
  container figure, and it must agree with chapter 1's statement of the loose
  contract.
- `docs/book/ste.md` and `task docs:ste -- --strict` apply from the first
  sentence. Runs of prose have 450 words or fewer (`task docs:breaks`); a table
  of the parts and their responsibilities is the natural break. Use “Ravel
  CLI”, “send”, “unchanged” and “return no result to the caller” as the shared
  decisions say.
- Make the title change in all the places that the node brief lists. Search
  with a pattern that you first see find a known occurrence (chapter 1's route
  table has one).
- Read the node brief's “Sources” bullet. The guides are sources that may be
  stale; verify each claim against the code at the checkout.

## Done when

- The chapter exists, builds (`task docs:book:build`) and meets the node
  brief's “Done when” for the draft stage. It passes the strict STE and
  prose-break checks, and `task docs:examples` still passes.
- The log lists each obligation O1, O2 and O10 and the section that teaches it,
  and for O3–O9 the chapter (8 or 9) that is to teach it.
- The log lists the destination of each sentence group of
  `docs/components.md` and `docs/local-runner.md` that this chapter covers, and
  of each relation R1–R8.
- Last act: `leaf-add` the `copy-edit` leaf under this node with the bare stem
  `the-parts-of-ravel`, and a body that carries what this draft learned. Cut
  nothing if a live later sibling already holds it.

## Notes

- One fresh reviewer is allowed under the leaf-wide allowance. Spend it on the
  claim that is hardest to check: why the worker is a separate process that is
  reaped before the launch.
- Forward mentions of chapters 8, 9, 11 and 12 are plain text until those
  targets exist. The second half's set proof (`book-second-half`) turns them
  into links.

## Decisions (running log)

- 2026-10-18: The structure brief is `docs/book/contract.md`. Its reader is
  “a 16-year-old software engineer with no background in Ravel”. Its ordered
  section plan puts chapter 7 before the session trace and job mechanics,
  with the outcome “explain the Ravel CLI” and the reasons for the worker and
  runner. Its Emphasis includes “Who holds which authority”; its Exclusions
  keep undelivered contracts as limits. The document node supplies the local
  order: CLI, worker, SDK, runner, result channel. All three draft
  preconditions are present in the named contract.
- 2026-10-18: Keep this chapter at structure depth. Use responsibility and
  change tables, source locations and links to earlier teaching. Leave the
  detailed session trace to chapter 8 and job mechanics to chapter 9. The
  guides contain old native-entry status statements, so they supply a
  coverage inventory rather than current technical authority.
- 2026-10-18: Use Tier 2 graph verification, then exact source reads. The
  indexed project is `Users-antony-Development-Ravel.simplify-and-componentize`.
  `index_status` reports ready but exposes no generation field. The first
  file-pattern query treated a regex as a glob and returned no rows. The
  corrected `crates/*` query returned all 39 matches with no further page.
  Trace results contain false name-based edges, including worker evaluation
  to configuration and test helpers. Treat them as navigation only.
- 2026-10-18: The chapter follows the brief's part order, then explains the
  loose program contract and the installed pair. Tables compare owners,
  interfaces and retained obligations. The draft links to the proved figures
  in chapters 2 and 6. Art owns whether to repeat them on this page.
- 2026-10-18: Changed the chapter title in the contract row and note, contents,
  shared title table and chapter 1's route. The old-title search first found
  the known chapter 1 hit. The shared title table retains the old name as
  migration history. Grove briefs retain the decision's context.
- 2026-10-18: The one in-session review is assigned to the worker boundary:
  separation of authority, identity before the policy request, confirmed
  cleanup before launch, and the same-user/detached-child limits. No other
  reviewer is commissioned.

## Source verification

The table records current source reads, not new runtime qualification. The
chapter's sections use the source locations as navigation, without source
excerpts or new runnable examples.

| Chapter section or claim | Source checked |
|---|---|
| Opening map and two private Rust packages | Root `Cargo.toml`, both package manifests, both `src/lib.rs` files, `ravel/src/main.rs` |
| CLI coordination and output | `ravel/src/run.rs` 35–301: checked choice, channel allocation, `Launch`, `result_after`, explicit cleanup, final signal join, `settle`, final write |
| Policy-free channel/runner test composition | `ravel/tests/session_process.rs` 134–193: `supervisor_fixture` |
| Executable TypeScript and asynchronous policy | Worker `src/main.ts` 101–141, 196–223, 304–330; SDK `Policy` and request/selection types |
| Separate worker and restricted inputs | `ravel/src/worker.rs` 317–672: `Command`, socket, descriptor close flags, diagnostic pipes, teardown and success conditions; `environment.rs` base, grants and exclusions |
| Identity before policy request | `worker.rs` 742–798, 1282–1322; worker `src/main.ts` hello before `converse` |
| Worker before managed launch | `choice.rs` 44–153, `worker.rs` 317–672, `run.rs` 35–223; the successful evaluation needs confirmed wait, directory removal and no failed stop |
| Worker build/startup controls | `dispatch.sh` shipped switches, `compile_worker`, `build`; worker `src/main.ts` 1–141; `environment.rs` |
| SDK and compatibility | Whole SDK `index.ts`, worker embedded-module registration; `authority.rs` 34–117 and `settings.rs` 30–93; `choice.rs::prepare` reads settings independently of explicit entry |
| Runner input and output boundary | `keyed-launch/src/lib.rs`; `run.rs` 1–180, 1030–1150, 1184–1418; manifest has only `libc` as a production dependency |
| Channel result and pairing | `completion.rs` 1–213 and `run.rs` 35–301; the `Ended` argument has no type-level channel identity |
| Loose command contract | `policy.rs::selected` 193–381 validates shape and strings; `run.rs` constructs argv; runner `run_with_output` builds `Command` directly; `end_session` takes text and inherited path |
| Installed pair | `worker.rs::locate`, `verify_hello`; `ravel/build.rs` entire source; `dispatch.sh` source inventory, digest, compile and asset copy |
| Reasons and accepted limits | The three ADRs, current session specification, glossary and chapter 2; current code supports the described structure, without a new runtime or platform claim |

The source coverage check returned generation `2026-10-18T08:08:23Z` with
matching metadata and no recorded issue for all 19 production source and
manifest paths checked. A later check at `2026-10-18T08:12:51Z` did the same
for the test fixture and the document sources. The new chapter had changed
metadata, so its actual text, not the graph, supplies the review artifact.
No recorded gap is a best-effort signal, not proof of complete indexing.
Qualified snippets and direct reads supersede false graph edges. Relevant
searches and depth-1 traces had no further page.

## Inventory 4: obligations and destinations

Re-enumerated all ten rows in `docs/components.md` against the current
session specification's installed boundary, policy authority, request and
selection, and managed-process/result sections. Chapter 7 owns O1, O2 and
O10. The destinations for O3–O9 are obligations for the already-planned
chapters, not claims that those chapters exist.

| ID | Obligation | Destination |
|---|---|---|
| O1 | Supported TypeScript policy, SDK, settings and discovery stay compatible | Chapter 7, “The worker executes TypeScript before the launch” and “The SDK describes the policy interface”, linked to chapter 4 |
| O2 | No worker result authority, identity check before policy handover, worker reap before launch | Chapter 7, worker section and “The installed pair stays together” |
| O3 | Policy data, refusals, exceptions and limits are checked without fallback | Chapter 8: selection and refusal fork; chapter 7 only names the check |
| O4 | Direct argv in caller context, confinement or refusal, fresh nested context | Chapter 8: launch composition; chapter 9: confinement mechanics |
| O5 | First complete result wins, exact empty/multiline/Unicode values, invalid/stale channels refuse | Chapter 8: send/read path and failure forks; chapter 7 explains the representation and empty-versus-absent distinction |
| O6 | Matching job reap and confirmed group cleanup precede success, independent of later child status | Chapter 8: result/launch pairing and final outcome; chapter 9: physical cleanup; chapter 7 explains the consumer obligation |
| O7 | INT/TERM/HUP cancellation suppresses results and propagates after cleanup | Chapter 8: precedence; chapter 9: signal mechanics and limits |
| O8 | Foreground and saved modes recover, group cleanup precedes reap | Chapter 9: terminal and group mechanics, with authority and identity conditions |
| O9 | Only result bytes reach stdout after explicit cleanup; cleanup/write failures prevent success | Chapter 8: output reservation, cleanup and final write; chapter 7 names the owner |
| O10 | Matched installed CLI/worker and shipped SDK, no host runtime or other project | Chapter 7, installed-pair section; chapter 12 explains build/delivery checks |

## Guide sentence groups and destinations

Ranges refer to the guides read at parent commit `fafb11f0`. The guides stay
in place until the scheduled architecture fold-in after set proof. Each row
covers the sentences in the named group, with technical details allocated
explicitly when chapter 7 gives only the structural explanation.

| `docs/components.md` source group | Destination and disposition |
|---|---|
| 3–20: opening acts, current implementation, normative/source navigation | Chapter 7 opening and final evidence paragraph. The unselected comparison remains outside the teaching. Normative documents stay. |
| 24–34: six responsibilities and why they do not imply six packages | Chapter 7 part map, CLI section, worker/runner/channel sections and installed pair. Detailed input, failure and cleanup rules belong to chapters 8–9. |
| 38–42: packages, private status, worker, SDK and shipped assets | Chapter 7 opening map, SDK, runner limit and installed pair |
| 44–49: executable/library targets and dependency roles | Chapter 7 CLI section and part map. The channel's `keyed-launch` types appear in the channel section. |
| 51–55: runner in the CLI, worker before managed process, `end-session` invocation, no daemon | Chapter 7 opening map, worker and result sections |
| 59–65: executable TypeScript, Bun, Rust authority, no result context | Chapter 7 worker section |
| 67–74: runtime/protocol/control costs, compatibility, no selected Rust-only alternative | Chapter 7 worker and SDK sections. No replacement runtime is claimed. |
| 78–82: runner's hidden decisions, absent policy knowledge, tests/consumer boundary, `libc` | Chapter 7 runner section. Private reuse differs from supported independent use. |
| 84–90: consumer-supplied launch obligations, extra Ravel checks, split not a future verdict | Chapter 7 CLI, runner and channel sections |
| 94–117: scoped obligation table and normative links | Inventory 4 above distributes every row. Historical observed-test labels belong to evidence, not chapter 7. |
| 121–132: composition overview and older C1/C2 evidence limits | Chapter 7 owners and worker table. Historical repair status is not copied as current status. Chapters 8–9 verify current failure paths. |
| 134–143: R1–R8 | Relation destinations below |
| 145–148: owner of contract, unsampled co-change, temporary shared state | Chapter 7 channel/CLI division. No historical co-change claim. Source/evidence limits remain explicit. |
| 152–171: successful-session trace and old measured results | Chapter 8 owns the trace. Chapter 7 covers worker-before-job and channel/runner composition only. Measurement receipts stay outside the book. |
| 175–211: cancellation, partial failure, C1–C4 and old native-entry status | Chapters 8–9 own current mechanics and guarantees. No old “conformance work” status is promoted from this guide. |
| 213–219: each participant's knowledge and the need to combine observations | Chapter 7 result-channel table and the paragraphs after it |
| 223–250: error/signal precedence, combined failures and limits | Chapter 8 owns precedence, chapter 9 owns process/terminal limits. Chapter 7 states only that the CLI combines them. |
| 252–257: continuing host, dispositions and no embedding guarantee | Chapter 7 runner's final paragraph and final limits; chapter 9 owns details |
| 261–277: independent storage-change scenario and channel/launch pairing | Chapter 7 result-channel section, including lack of type-level pairing. Test composition appears in the CLI section. |
| 281–287: five retained costs/questions | TypeScript: worker section. Generic job reuse: runner section. Result-aware reuse and state: channel section. Worker/job differences: worker and runner sections. No generic-worker extraction is claimed. |
| 289–300: comparison priority, baseline, scope and unassessed use | Outside this chapter: current comparison/spec/evidence and later component requirements. Chapter 7 states the applicable private-use and evidence limits. |
| 304–421: source provenance, E1–E7 and historical observations | Source inventory above supports this draft. Historical evidence remains outside the book and is not represented as a new measurement. |

| `docs/local-runner.md` source group | Destination and disposition |
|---|---|
| 3–9: private runner, local dependencies, Ravel composition, transport | Chapter 7 opening, runner and result-channel sections |
| 13–16: argv and Launch inputs | Chapter 7 runner's first paragraph |
| 18–25: public interface table | Chapter 7 runner table gives structural roles. Chapter 9 owns `signal`, `reraise` and exact observer/signal behaviour. |
| 27–38: escalation, reap/recovery, group checks, retained reports and decisions above runner | Chapter 7 runner boundary and CLI ownership. Chapter 9 owns timing and the EPERM conditions. |
| 42–93: native entry, continuing host, errors, observers, failed exec, old conformance status | Chapter 7 states the host-restoration limit. Chapter 9 owns the mechanics and current conformance statement. No old status is copied. |
| 97–113: tests, fixture roles, task workflow and isolation guards | Chapter 11 owns this material. |
| 117–131: transferred provenance and source decision pointers | Historical provenance stays outside chapter 7. Migration may retain one provenance pointer under the book contract. |

## Relation destinations

| Relation | Chapter 7 destination | Detailed continuation |
|---|---|---|
| R1: caller request and returned bytes | CLI decision table and loose-contract section | Chapter 8: entry and final write |
| R2: owner files to checks and evaluation | SDK compatibility paragraph and worker section | Chapter 8: request preparation; chapters 4/6 already teach author inputs |
| R3: private worker protocol, identity, no result path | Worker section and table | Chapter 8: evaluation path |
| R4: worker wait before managed spawn | Worker section and table | Chapter 8: selection-to-launch transition |
| R5: CLI composes channel and runner | CLI section, runner inputs, channel pairing | Chapter 8: actual `Launch` construction and failure exits |
| R6: send to shared file, appearance versus contents | Result-channel section | Chapter 8: send/read path |
| R7: group cleanup, reap, terminal/group confirmation, CLI outcome, stdout | CLI and channel sections state owners and success conditions | Chapter 8: final outcome; chapter 9: precise physical order and failure limits |
| R8: shared identity inputs pair both builds | Installed-pair section and table | Chapter 12: build and install checks |

## Review and validation

The single fresh reviewer returned no findings in the assigned worker
boundary. Its source read covered restricted inputs and descriptors, the
identity/request order, the cleanup predicate, TypeScript bootstrap, SDK
registration and build identity. There was no finding to classify as an
unclear contract, actionable defect, accepted trade-off or noise. This was
inspection only, not runtime, platform, prose or art evidence. The review
allowance is spent. Later changes added local term links and clarified the
hard-link sentence without changing a technical claim.

The initial strict STE check found seven disallowed verb forms in “-ing”.
The draft rephrased them, and the strict check passed. The prose-break check
also passed. No shared vocabulary exception was added. This used the local
STE specification's writing rules, with its dictionary excluded as agreed.

Final validation used completed before/after SHA-256 lists for all 3,468
tracked non-Grove files, plus `target/debug/ravel` and
`target/libexec/ravel/ravel-policy`. This covers chapter text, contract,
word decisions, build/check scripts, manifests, source, fixtures and the
built pair. Both lists have digest
`da1fb0b0198f06a2b85c08d889f292b443109e761c4502bd2e737e8030ec23d1`.
The per-file lists compare equal. Generated book output and dependency/tool
caches were not subjects of that digest.

| Final check | Result |
|---|---|
| `task docs:ste -- --strict` | 19 files, zero problems |
| `task docs:breaks -- --strict` | Seven chapters, zero runs above 450 words |
| `task docs:book:build` | Build passed without warnings |
| `task docs:excerpts` | Zero excerpts, zero page errors. Chapter 7 contains no source excerpt. |
| `task docs:examples` | 13 passed, zero failed or skipped, six labelled fragments not run, zero page errors |
| Rendered local links in chapters 1 and 7 | 31 links per page resolve. The same checker rejects a missing file and a missing anchor. |
| Title search | The earlier title remains only in the shared old/new title table and Grove decision context. The known pre-edit chapter 1 hit proved that the search detects it. |

The whole-chapter read checked order against the brief, direct-entry context,
term links and each technical claim against the source table. The chapter
states its evidence limits. This draft did not measure every failure path,
run a real agent, qualify a platform, inspect a browser layout or test new-reader
comprehension. Those are not results of these document checks.

No architectural decision changed, so the three ADRs remain unchanged.
The document node stays open for copy-edit, art and proof. Its art handoff
names the figures to consider. The parent brief points later chapters and
the architecture fold-in at this task's obligation and guide maps.
