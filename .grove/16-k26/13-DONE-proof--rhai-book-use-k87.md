# rhai-book-use-k87

## Goal

Prove chapters 1–6 after the complete correction run so the owner can proceed to Part II.
Read both parts as one document and resolve every remaining class of defect before the reader checkpoint resumes.

## Context

- The owner completed Part I, then requested all lessons before Part II on 2026-10-28.
  Findings F1–F16, their exact words and that timing decision are in the live checkpoint `rhai-book-use-k74`.
- Read the current contract, all of `docs/documentation-lessons.md` and the preceding stages' decisions.
  Draft `rhai-book-use-k84`, copy-edit `rhai-book-use-k85` and art `rhai-book-use-k86` form this authorised second correction run.
- The earlier proof `rhai-book-use-k78` provides historical checks and coverage methods, not evidence for the changed document.
  Build fresh inventories from the current chapters and sources. Check the claims that the new configuration terminology and worker explanation imply.
- The source of truth is the current code, the built CLI, the cited ADRs and the recorded harness smoke evidence.
  Resolve any outstanding cutover or installed-layout dependency explicitly rather than copying an earlier status claim.

## Done when

- The whole of chapters 1–6 follows the current contract and all applicable accumulated lessons.
  The record shows coverage of the lessons and the owner's findings, with any inapplicable rule explained.
- A concrete task teaches the caller's prompt, completion options, result transport and next action before the general model.
- Configuration terms, headings, contents, figures and links agree. Exact filenames, commands, fields and diagnostics still match the interface.
- The worker account explains the separate process, its purpose and its completion before the final managed launch, with source evidence.
- All complete examples, exact excerpts, figure reproductions, strict prose checks and the book build pass.
  Link checks cover the affected local pages, anchors and assets, and include a deliberately failing control.
- Fresh visual inspection covers affected figures and pages at the reading width. State its bounds and distinguish it from mechanical checks.
  Record the final subject and check that validation did not change it.
- The node brief has no unresolved hand-forward item for these parts. Each required later whole-book reconciliation has an owner.
- The proof leaves a concise correction recap for `rhai-book-use-k74`, including the general lessons applied and the locations of visible changes.
  Update the shared lessons record with any further transferable rule and its evidence.

## Return to the reader

Cut no successor and no new Read 1 checkpoint. The existing `rhai-book-use-k74` remains live immediately after this node.
It resumes at chapter 4 after it refreshes the local book and private reader environment.
Make amended Part I available for inspection without requiring a restart at chapter 1.

The owner's required practical tasks remain outstanding until the owner reports them.
Neither this proof nor the earlier Part I review substitutes for those tasks or for a new reader's evidence.
Read 2 remains required after the later whole-book proof. A further substantial correction run needs owner escalation.

## Decisions (running log)

- **Scope and source access, 2026-10-28.** Read chapters 1–6 as one document against the current contract and all of lessons L01–L66. The graph CLI refused access because a pre-coordination or unverified generation is active. Direct source reads replace graph evidence. No graph project, generation or coverage is claimed.
- **Refusal scope, 2026-10-28.** Chapter 3's outcome summary now names the final managed launch and allows earlier evaluation harness runs. This closes the same ambiguity that the detailed entries already resolve.
- **Environment terminology, 2026-10-28.** Chapter 6's `policyEnvironment` row now names the base variables that `ravel::env` permits. They are not variables in the empty environment of the Rhai worker. `environment.rs`, `Grants::lookup` and `to_json`, and `choice.rs`'s Rhai launch establish the distinction.

## Coverage of the contract and lessons

The subject at entry was `e589fed1`, the art commit. This proof read all six current chapters, not only the changed passages.
It also read the glossary, the contract, all of the lessons record, the five cited ADRs and the correction stages' decisions.
The mechanics notes supplied the checks. Historical proof results supplied no current pass evidence.

| Lessons | Application and bounds of this proof |
|---|---|
| L01–L03 | The agreed reader, tasks, exclusions and chapter order stand. Chapter 1's review precedes the general roles. Caller conventions stay labelled. |
| L04–L11 | Rhai has a local introduction, example and documentation links. Each chapter restates its terms. Mechanisms name their actors, streams and results. The same review connects the general statements. |
| L12–L18 | Read the prose for connected meaning and conditions. Compare repeated guarantees with the specs and source. Correct the refusal summary and environment recipient. Preserve the explicit release dependencies below. |
| L19–L25 | Derive fresh inventories below. Run the visible complete examples, convention sample and exact excerpts. Keep the version placeholder and private fake-agent environment. |
| L26–L34 | Compare the 13 figures used in these parts with their alternatives and surrounding explanations. Reproduce all 18 figures. Inspect the rendered figures at chapter width. No renderer, style or pin changes are necessary. |
| L35–L40 | Use broken-anchor, missing-image, semicolon and prose-length controls. Check rendered headings and raw Markdown. Classify all unmatched source tokens. Preserve literal help and diagnostics. |
| L41–L48 | Keep owner progress separate from checks and model inspection. Read 1 resumes at chapter 4. The recap names visible changes and remaining tasks. Extend L16 and L36 with this proof's observations. |
| L49–L51 | No borrowed source text or personal data enters the book. No live model call is necessary. The chapters already use the prose rules, so no separate prose conversion is necessary. |
| L52–L56 | Compare recorded fragments across chapters, including the 619-byte notice script and its digest. Preserve the measured harness vectors. Check heading ids and code fences after the edits. |
| L57–L62 | Check all F9–F16 patterns throughout both parts. These include concrete cases, complete caller tasks, configuration terms, JSON descriptions, worker reasons, links, headings and familiar commands. |
| L63–L65 | Enumerate old terminology outside code and link targets. Each remaining use explains a literal name or diagnostic. Check inbound anchors and link descriptions against their destinations. Recorded review fragments agree across chapters. |
| L66 | Work continues to completion. No condition requires an unfinished stop or another correction run. |

The owner's F1–F8 are covered by the Rhai introduction, selection and routing-notice explanation, reader goals, inherited-path mechanism and repeated local explanations.
F6 follows the sent value through cleanup to standard output. F7 and F8 govern the explanatory prose throughout, not only the opening chapter.

F9 is visible in chapter 1's prompt and outcome tables and chapter 5's executable orchestrator.
F10–F16 are covered by the last three rows above. No finding remains handed forward inside this node.
The pending practical tasks and new-reader comprehension remain outside mechanical proof.

## Fresh source inventories

The source comparison uses direct reads because the graph generation could not be established.
The inventory is task-directed verification, not an audit of every source line or a new platform qualification.

| Inventory | Derived subject and disposition |
|---|---|
| CLI | `cli.rs`, `main.rs`, `inputs.rs`, `refusal.rs`, cancellation and the built help for all three command lines. Both verbs, all fifteen session options, help/version options and the result argument have reference entries. The status tables cover 0, 1, 2, 3, 5, 124, 126, 127, 129, 130 and 143, including process-API signal propagation. |
| Configuration | `settings.rs`, `harness_entries.rs`, `limits.rs`, `policy.rs`, `context.rs`, and the Rhai engine, bridge, helpers, discovery, host and harness modules. Every settings key, nested key, request bound and request field has an entry. So do selection/refusal fields, context fields, measurements, schema keywords and registered `ravel::` functions. Read overloads remain distinct. |
| Engine and worker | `rhai_worker/mod.rs`, `engine.rs`, `role.rs`, `heap.rs`, `front.rs` and `worker.rs`. The constant engine limits, 1 GiB heap, 64 MiB evaluation stack, module count, message/read limits and stop order agree with chapter 6. `choice.rs` supplies an empty worker environment. `worker::supervise` checks reap and directory removal before it returns success. Only then does the Rhai choice judge the selection and resolve the final program. |
| Lifecycle | The session and result/job specs, the policy stop rules, `run.rs`, completion code and harness supervision. Chapters 3, 5 and 6 cover sent, empty, absent and repeated results, later nonzero child exit, refusal and invalid inputs. They cover worker/launch failures, deadlines, cancellation, escalation, cleanup failures, partial writes, nesting, confinement and terminal limits. Native-entry details remain with chapters 7–11. |
| Component obligations | The ten obligations cover inputs, the worker, selection validation, direct launch and first-result transport. They also cover reap/group cleanup, cancellation, terminal recovery, result-only output and same-binary execution. The figures preserve these boundaries. The private runner remains inside the CLI. |
| Contributor workflows | Fresh `task --list` and the Taskfile include book checks, harness smoke and Rhai upgrade proof, plus the current build/test/install/release and measurement tasks. Build/install tasks still mention the TypeScript worker. Delivery removal owns their reconciliation, and the later whole-book proof owns the final contributor inventory. This proof makes no claim that chapters 10–11 already describe that future layout. |

The diagnostic inventory enumerated every snake-case string literal, including digits, in the non-test source scope before comparing chapter 6.
It found 97 distinct tokens, of which 84 occur in the reference. Those tokens are not all diagnostic codes.
Every unmatched token received this disposition:

- `selection_unsettled`, `context_loader_unsettled` and `context_source_unreadable` belong to the TypeScript path. They are not Rhai diagnostics.
- `spawn_missing` and `spawn_other` are private fault controls. `remove_dir_all` names an operation, and `prompt_file` is a clap field reference.
- `find_map`, `for_each`, `index_of`, `reduce_rev`, `order_by` and `sort_by` are Rhai built-ins, not missing `ravel::` exports.

The reference has 54 CLI diagnostic rows, including the assigned future `policy_format_retired`, and 17 host-data-error rows.
The helper inventory comes from registration in `bridge::namespace` and `helpers::register`, not from the reference's own list.
The camel-case comparison separately classified old TypeScript fields, internal protocol fields and private native-entry evidence fields.
No unmatched public Rhai field was found in that comparison.

The environment inventory covers base reads, explicit grants, all nine reserved classes, all six scrubbed launch controls and the confined additions.
Compiler build identities, the old Bun cache control and private fixture controls are internal or retained-path details, not new user inputs.
`choice.rs`'s empty Rhai launch, `environment::Grants::lookup`, `run.rs` and the harness launch establish the different recipients.

The worker mechanism is source-backed: `front::executable` uses `/proc/self/exe` on Linux and canonical `current_exe` on macOS.
`front::launch` sets the internal role. `worker::supervise` constructs and spawns the child with its private directory and cleared environment.

The process-boundary ADR records the deadline and failure-isolation reasons. Host calls return through the private socket.
Every successful host call and worker teardown completes before the final managed launch. An evaluation harness can run earlier.

## Accepted delivery dependencies

- `typescript-worker-cutover-k31` owns the retired-format refusal and default-entry cutover. The current checkout still accepts the TypeScript path. The book teaches the agreed 2.0.0 interface.
- `typescript-delivery-removal-k32` owns the final notices path and installed archives. `rhai-book-inside-k34` already owns reconciliation of chapter 6 and `installed-files` against that final layout.
- `rhai-spec-reconciliation-k33` owns the specs, ADRs, README and the disposition of literal CLI help wording. Its brief already names the configuration term map.
- `rhai-book-inside-k34` owns the shared-figure prose in chapters 7–11, chapter 8's old cancellation example, the final whole-book proof and Read 2.
- `release-ravel-2-0-0-k37` owns replacement of chapter 3's version placeholder before publication.

The recorded real smoke evidence in `rhai-harness-smoke-k66` agrees with both chapter 4 recipes.
It names Claude Code 2.1.310 and Codex CLI 0.164.0, the exact argument vectors and answer extraction.
This proof uses that dated evidence. It makes no new live call and no claim about later harness releases.

## Validation evidence, 2026-10-28

`mise exec -- task setup` completed before the checks. The final checks used the source after both proof corrections and the lessons additions.

| Check | Final result |
|---|---|
| `task docs:examples` | 22 passed, 0 failed, 0 skipped. All 57 fragments remain outside this execution claim. |
| Example-convention sample | The visible sample extracted from `docs/book/examples.md` passed separately through the same checker. |
| `task docs:excerpts` | All nine excerpts passed. |
| `task docs:figures` | All 18 SVGs and layout records reproduced from the pinned sources. |
| Strict `task docs:ste` | Zero problems in chapters 1–6 and all 18 figure layout records. |
| Strict `task docs:breaks` | No prose run above 450 words in chapters 1–6. |
| `task docs:book:build` | Built without a warning or error. |
| Local links | All 2,075 local href/src targets in the built HTML and nine local README links passed. This includes links from later chapters into these parts. |
| External destinations | All eleven distinct external URLs in chapters 1–6 returned HTTP 200 after redirects, including Grove and the Rhai documentation. This checks reachability, not all linked content. |
| Rendered structure | Chapters 1–6 have 14, 6, 7, 19, 11 and 53 source headings respectively, with matching rendered heading ids. No raw fence, heading marker or escaped prose-control comment appeared at the beginning of a line in the rendered HTML. |

The link instrument parses HTML `href`, `src` and `id` attributes, resolves local paths and checks fragments against destination ids.
It rejected a planted missing chapter-2 anchor and a planted missing SVG. The strict prose instruments rejected a semicolon and a 460-word run in scratch copies.
The link targets and final-run results were checked individually. Agreement of totals alone was not the comparison.

Before the final gates, SHA-256 covered 3,666 files: all tracked non-Grove files, regular non-symlink files under `target/tools`, and `target/debug/ravel`.
This includes the chapters, examples, excerpts, figures, contract, glossary, lessons, mechanics, source, scripts, manifests, lockfiles and pinned tools.
The manifest is in `/tmp/ravel-proof-k87/final-before.json`. Its sorted-map digest is `dfa9716bd3a1d97a215bcc1bbced8810045f496f5e7ebc896237a08470097768`.
Every subject digest was unchanged after validation. Grove evidence records are excluded because they record the results after the run.

| Final chapter | SHA-256 |
|---|---|
| 01 | `5fd0b2487b09880bd04713d2a81d08b09748664f95128e3cdcb84338d2a4a059` |
| 02 | `b11aa7277d9a2f30a1c32463196cb86891750a1e4133d4fdc6f45ea9ac8a1229` |
| 03 | `be9ec3132579dd30898d923581c288701bd8ba7b9d796183a65e8bc91076cf6c` |
| 04 | `d9112dbe3e38e7afef1df9c0a1bea0599bba8d5458bac964ede89b01d9f8a00c` |
| 05 | `3a8a6825502cafcb9406ba3b014798120d129e48b5428aea09868adf1cbc8870` |
| 06 | `8f443663b0cca08a9f2f2c644dececea30aab8748e47f4674eced5b8c5042870` |

## Return to Read 1

The existing checkpoint `rhai-book-use-k74` receives the correction recap. It resumes at chapter 4 and refreshes its private shell and local server.
The amended Part I stays available through the same contents entries. The owner need not repeat the completed Part I read.

The fake-agent session, refusal, cancellation and direct-reference exercises still need the owner's reported results.
Read 2 remains owned by the later whole-book pipeline. No successor, new checkpoint or correction run is necessary here.

No in-session reviewer was used. The two corrections rest on direct source comparisons and the existing interface checks.

## Visual evidence and limits

Fresh inspection used Safari in an isolated macOS 27 guest, in the light theme, on a 1920 × 1080 screen.
The page column was 750 pixels wide. Screenshots covered all six chapter openings, their contents entries, and all 13 distinct figures in chapters 1–6.
The inspection covered the lower parts and keys of the tall figures, with overlapping views.

The figures were review flow, routing notice, inherited paths, system context, process authority, successful session and session outcomes.
It also covered evaluation harness, exit-status precedence, both cancellation sequences, source components and installed files.
The corrected chapter 3 refusal summary was visible below the successful-session figure.

The effective ordinary text sizes calculate to about 11.7 pixels for inherited paths and 11.6 for evaluation harness.
They remain close to the approximate 12-pixel target. The other figures calculate to at least 12.5 pixels at this width.
The labels, arrows and keys remained readable in the inspected views. This is the session's judgement, not a new reader's result.
The inspection establishes neither other browsers/themes/widths nor GitHub rendering or the release site.

A scratch copy of the built HTML had scroll aids for figure selection. The source, figures and final built output had no such edits.
Screenshots and checker logs remain under `/tmp/ravel-proof-k87`, outside the working copy.
One guest navigation through AppleScript timed out. VNC navigation completed the remaining views, and the screenshots' page titles were checked before crediting them.
The guest and temporary server are stopped before the session ends.
