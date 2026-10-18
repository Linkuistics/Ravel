# book-guide-fold-in-k68

## Goal

Retire the two guides Parts I and II fully absorb, `docs/session-cli.md` and
`docs/owner-configuration.md`, with every inbound link redirected and the
owner's dated configuration handoff kept outside the book.

## Context

- `docs/book/contract.md`, "Relation to other documents" and "Exclusions": the
  guides are removed once the chapters that absorb them land, the handoff
  record is "kept, not deleted", and the book never depends on a link to a
  specification.
- Placement decided in planning: the handoff section ("Configuration handoff on
  2026-10-12") moves unchanged in substance to its own file,
  `docs/configuration-handoff-2026-10-12.md`, with a short heading that says
  what it is (a dated record of the owner's copy, not Ravel's registry or
  defaults) and a pointer from `docs/caller-references.md`. It is not in the
  book and not in `docs/evidence/` (which holds measured receipts).
- Inbound links, enumerated in planning from a tracked-file search that must be
  repeated with controls (do not rely on this list): `README.md`,
  `docs/components.md` (to `session-cli.md#inputs-and-exit-behavior` and
  `owner-configuration.md`), `docs/session-channel.md`,
  `docs/specs/session-execution.md`, `docs/native-entry-development-evidence.md`
  and `docs/caller-references.md`. Frozen evidence under `docs/evidence/` and
  the `*-checks.txt` logs record those files as digested subjects by name; they
  are receipts and are left alone, and the sweep classifies them as such.
- The fold-in's input is the proved chapters. For each guide, enumerate every
  sentence and classify it: absorbed in chapter N (name the anchor), a
  duplicate of a document that survives (the spec, the README's Develop
  section), moved to the handoff record, or contributor-facing with no home.
  A contributor sentence with no home moves into the README's Develop section
  (it stays until chapter 11) and is listed in the node brief's "Handed
  forward"; nothing is lost silently.
- `docs/authority.md` and the README's user-facing text are
  `book-readme-and-authority-k69`'s.

## Done when

- The classification table for both guides exists in this leaf's log, every
  sentence accounted for, and no row is "absorbed" without a chapter anchor
  that a reader can follow.
- `docs/session-cli.md` and `docs/owner-configuration.md` are removed;
  the handoff record exists at its new path; every inbound link in a surviving
  document names its new place (a chapter anchor, the handoff record, or the
  specification the sentence belonged to).
- The link sweep is enumerated, not pattern-listed: extract every Markdown
  link target in the tracked tree (excluding `docs/evidence/`, the checks
  logs, `retired/` and `.grove/`), resolve each, and classify; a positive
  control (a link that must resolve) and a mutated control (a deliberately
  broken link) are seen to fail and succeed as they should. `task docs:book`
  still builds.
- `task check` passes and `task test:native-incumbent-controls` is run, or its
  non-run is explained, since the incumbent instrument names documents.
  Release archives are unchanged in manifest (`task release:test`).
- `docs/book/contract.md`'s "Relation to other documents" is edited in place
  only to say these two guides are gone and where the handoff record lives.

## Notes

- Do not touch `docs/authority.md` or the README here; that is k69. Where a
  README link points at a removed guide, redirect it minimally and let k69
  reduce the README.

## Decisions (running log)

- Handoff record placed at `docs/configuration-handoff-2026-10-12.md`: the
  section's body is moved verbatim (only the heading is new, with a three-line
  opening that says it is a dated record of the owner's copy, not Ravel's
  registry or defaults, and that the book is where configuration is taught).
  Pointer: the `docs/caller-references.md` inventory row that named
  `docs/owner-configuration.md` now names the handoff record.
- Inbound links redirected to a chapter anchor or the specification, never to a
  removed file. README (3 links, minimal; k69 reduces it), `docs/components.md`
  (2), `docs/session-channel.md` (1, to the specification it belongs to),
  `docs/specs/session-execution.md` (1, to ch. 6 `#command-line`),
  `docs/native-entry-development-evidence.md` (1; a past-tense evidence
  sentence, so the link becomes plain text naming where the guide went).
  `docs/caller-references.md` (the row above). Frozen receipts left alone.
- Two contributor sentences had no home and one was only partly covered; they
  moved into the README's Develop section (`task typecheck` line and sentence,
  the `task test:cli` description, "focused checks while editing, the full
  relevant suite at handoff", and "`task check` rebuilds and typechecks the
  pair"). Everything else in the Develop paragraph was already there.
- `docs/book/contract.md` "Relation to other documents" edited in place only.
  The "Exclusions" bullet "The owner's personal configuration" still speaks of
  "today's configuration guide" and of the guide being "folded in"; it is
  outside this leaf's permitted edit, still true as a rule, and listed under
  "Handed forward".

## Classification: `docs/session-cli.md` (84 lines; every sentence)

Chapter anchors are `docs/book/src/<file>#<slug>`. Short forms: ch1 =
`01-what-ravel-is-for.md`, ch2 = `02-actors-and-words.md`, ch3 =
`03-a-complete-session.md`, ch4 = `04-writing-an-owner-policy.md`, ch5 =
`05-running-sessions.md`, ch6 = `06-reference.md`. Dispositions: A absorbed
(anchor named), D duplicate of a surviving document, C contributor sentence,
H handed forward.

| # | Sentence (paraphrased by line) | Disp. | Home |
|---|---|---|---|
| 1 | L3 Ravel's boundary is a process manager and router | A, D | ch1 `#from-a-request-to-a-result`; README intro and `docs/specs/session-execution.md` (both survive) |
| 2 | L3-6 this checkout builds front, worker, runner, channel together | A | ch2 `#policy-evaluation-and-supervision`; ch3 `#obtain-the-build-used-by-this-chapter` |
| 3 | L5-6 `task install` builds and verifies an independent pair; `task smoke` uses a fresh prefix | A | ch3 `#obtain-the-build-used-by-this-chapter`; ch6 `#installed-layout` |
| 4 | L7-8 the executable specification describes policy, runtime, process, authority | D | `docs/specs/session-execution.md` (survives; README links it) |
| 5 | L10-12 the component guide connects promises to owners, packages vs processes, result/cancellation composition | D | `docs/components.md` (survives until the second half; README links it) |
| 6 | L14-19 table: `start-session` and `end-session RESULT` responsibilities | A | ch1 `#from-a-request-to-a-result`; ch6 `#start-session`, `#end-session-result` |
| 7 | L21-23 `end-session` finds its session by inherited context, no session ID or target; ephemeral per-session channel isolated from nested sessions | A | ch5 `#keep-nested-sessions-separate`; ch2 `#three-different-kinds-of-information`; ch6 `#end-session-result` |
| 8 | L23-26 Ravel terminates and reaps before returning the argument unchanged on stdout, distinguishable from output and diagnostics | A | ch2 `#the-managed-process-and-its-result`; ch3 `#publish-the-result-and-finish-the-job`; ch6 `#standard-streams` |
| 9 | L26 ordinary `--help` and `--version` remain | A | ch6 `#ravel-help-and-version` |
| 10 | L28-30 caller supplies the prompt with `end-session` instructions; Ravel passes it unchanged; `continue`/`force-stop` are the caller's | A | ch1 `#from-a-request-to-a-result`; ch5 `#supply-the-prompt-and-caller-data` |
| 11 | L31-32 task trees, continuation decisions, provenance stay outside the boundary | A | ch1 `#decide-whether-ravel-fits` |
| 12 | L34-35 no persistent store, record/observation commands, lookup API, importer, history migration | A | ch1 `#decide-whether-ravel-fits`; ch2 `#three-different-kinds-of-information`; ch4 `#copy-an-existing-policy` |
| 13 | L35-36 existing dispatch history stays with its original tool | D | README "Earlier project"; `docs/retired-project.md` (both survive) |
| 14 | L36-37 generic policy/SDK routes with applicable context, no database; format and selection choices remain | A | ch4 `#define-a-policy-and-read-the-request`, `#supply-policy-context` |
| 15 | L37-38 obsolete storage settings are removed during configuration copying | A | ch4 `#copy-an-existing-policy`, `#set-bounds-and-grant-environment-names` |
| 16 | L39-40 see the owner configuration guide for that boundary and the handoff | A, M | ch4 `#copy-an-existing-policy`; handoff record `docs/configuration-handoff-2026-10-12.md` |
| 17 | L42-44 development checks use real-worker/fake-agent and local process seams; test round trips, nested/stale context, cancellation, reaping, terminal recovery, confinement, then the installed pair | D | README Develop (the "private session channel" paragraph and the `task smoke` / `task install` lines) |
| 18 | L45 use focused checks during edits and the full relevant suite at handoff | C | moved to README Develop (new sentence) |
| 19 | L45-48 Taskfile commands use nextest with reporting visible, save formatting/lint logs with summaries, inspect failures only when needed | D | README Develop (`task check` paragraph, unchanged) |
| 20 | L48 no real model launches or new testing framework required | D | README Develop ("No model account or other project checkout is needed") |
| 21 | L52-53 `start-session` requires `--kind` and exactly one prompt option | A | ch5 `#supply-the-prompt-and-caller-data`; ch6 `#--kind-text`, `#--prompt-text-and---prompt-file-path` |
| 22 | L53-54 default `~/.config/ravel/policy.ts`; `--config` relative to the caller's cwd | A | ch6 `#--config-path`, `#policyts`; ch4 `#define-a-policy-and-read-the-request` |
| 23 | L54-55 optional `--param`, `--context`, `--task-file`, `--task-id` are caller data with no Ravel task conventions | A | ch6 `#--param-namevalue`, `#--context-path`, `#--task-file-path-and---task-id-id`; ch5 `#supply-the-prompt-and-caller-data` |
| 24 | L56 the policy receives the prompt unchanged, including empty strings and newlines | A | ch5 `#supply-the-prompt-and-caller-data` |
| 25 | L57 use `--help` for bounds, grants and confinement | A | ch6 `#ravel-help-and-version`, `#--timeout-ms-ms`, `#--confine` |
| 26 | L59 first `end-session` publication wins | A | ch3 `#publish-the-result-and-finish-the-job`; ch6 `#end-session-result` |
| 27 | L59-61 a supplied result is success after confirmed cleanup even if the process exits nonzero | A | ch5 `#accept-a-result-only-after-success` |
| 28 | L61-62 child exiting without a result is exit 1, including a natural 0 | A | ch5 `#interpret-the-exit-status`; ch6 `#exit-statuses` |
| 29 | L62 missing or stale enclosing context makes `end-session` exit 1 | A | ch6 `#end-session-result`, `#exit-statuses` |
| 30 | L62-63 INT/TERM/HUP cancellation returns that signal and no result | A | ch5 `#cancel-during-selection-or-supervision`; ch6 `#exit-statuses` |
| 31 | L63-64 cleanup/supervision failures go to stderr and never yield success | A | ch5 `#accept-a-result-only-after-success`; ch6 `#exit-statuses` |
| 32 | L64-66 exit codes 2, 3, 5, 124, 126/127 | A | ch5 `#interpret-the-exit-status`; ch6 `#exit-statuses` |
| 33 | L66-67 `--json` formats notices and refusals on stderr; stdout stays the opaque result | A | ch6 `#json-routing-notice`, `#standard-streams`; ch5 `#read-notices-without-confusing-them-with-results` |
| 34 | L67-70 notice written once after supervision; `operationReports` holds benign stop reports retained after confirmed cleanup; empty array when none, absent when supervision produced no ending | A | ch5 `#read-notices-without-confusing-them-with-results`; ch6 `#json-routing-notice` (the `operationReports` row); the retained-`EPERM` rule is in `docs/specs/result-job-completion.md` |
| 35 | L70-71 without `--json` each report is a `ravel:` line on stderr | A (partial), H | ch6 `#standard-streams` says reports reach stderr; the `ravel: ` line form is verified in `crates/ravel/src/run.rs` (the two `eprintln!("ravel: {report}")` sites) and is **not** in a chapter; handed forward |
| 36 | L71 managed-process stderr remains its own stream | A | ch6 `#standard-streams` |
| 37 | L73-74 the worker is paired by package version and source digest before any policy is sent | A | ch6 `#json-routing-notice` (`worker` row), `#diagnostic-codes` (`worker_identity_mismatch`); ch3 `#check-the-request-and-evaluate-the-policy` |
| 38 | L74-75 embedded imports `ravel/sdk` and `ravel/examples/static`; `readChoice` reads `.ravel-choice` | A | ch4 `#resolve-policy-imports`, `#offer-a-checkout-choice`; ch6 `#ravelsdk` |
| 39 | L75-76 context-host reads are measured, bounded regular-file reads | A | ch4 `#supply-policy-context`; ch6 `#host-operations` |
| 40 | L76-77 no run-lookup operation, embedded caller adapter or persistent state directory | A, D | ch1 `#decide-whether-ravel-fits`; the "no embedded caller adapter" clause is in `docs/caller-references.md` (survives) |
| 41 | L77-78 obsolete `stateDir` settings refuse as unknown keys without opening that directory | A | ch4 `#set-bounds-and-grant-environment-names`; ch6 `#settingsjson`, `#diagnostic-codes` (`settings_invalid`) |
| 42 | L80-81 `task setup` installs verified Bun 1.4.2 and the frozen worker dependencies | D | README Develop (unchanged) |
| 43 | L81 `task build` builds the matching pair | D | README Develop `task build`; ch3 `#obtain-the-build-used-by-this-chapter` |
| 44 | L81-82 `task typecheck` checks both source and generated declarations | C | moved to README Develop (new sentence and list line) |
| 45 | L82-83 `task test:cli` uses actual workers, temporary policies, fixture processes | C | moved to README Develop (new sentence) |
| 46 | L83-84 `task check` rebuilds and typechecks the pair, lints all targets, runs the full nextest suite | C (part), D | README Develop already said the lint/nextest part; "rebuilds and typechecks the pair" added |

## Classification: `docs/owner-configuration.md` (96 lines; every sentence)

| # | Sentence | Disp. | Home |
|---|---|---|---|
| 1 | L3 default `~/.config/ravel/policy.ts` | A | ch6 `#policyts`, `#--config-path` |
| 2 | L3-5 a sibling `settings.json` may set `timeoutMs`, `contextBytes`, `policyEnv`, each optional | A | ch6 `#settingsjson`; ch4 `#set-bounds-and-grant-environment-names` |
| 3 | L5-7 absent settings keep 30,000 ms, 262,144 bytes, no extra grants | A | ch4 `#set-bounds-and-grant-environment-names`; ch6 `#bounds` |
| 4 | L7-8 default source read 65,536 bytes, 256 sources, 1,048,576-byte message, 262,144 diagnostic bytes | A | ch6 `#bounds`; ch4 `#supply-policy-context`, `#set-bounds-and-grant-environment-names` |
| 5 | L8-9 CLI flags replace adjustable bounds and add grants | A | ch6 `#--timeout-ms-ms`, `#--context-bytes-bytes`, `#--policy-env-name` |
| 6 | L9-10 see the session CLI and the specification | D | `docs/specs/session-execution.md` (survives) |
| 7 | L12-14 why configuration is executable TypeScript; the worker is separated from session authority | A, D | ch2 `#trust-and-confinement`; `docs/components.md#why-typescript-exists` (survives) |
| 8 | L16 the policy imports `ravel/sdk` | A | ch4 `#define-a-policy-and-read-the-request`; ch6 `#ravelsdk` |
| 9 | L16-17 the SDK version string is the owner's label, not the executable version | A | ch4 `#define-a-policy-and-read-the-request` |
| 10 | L17-18 `readChoice` uses `.ravel-choice` only when the policy calls it; Ravel never chooses from that file itself | A | ch4 `#offer-a-checkout-choice` |
| 11 | L18-19 caller kinds, provider/model labels and command arguments belong to the owner policy | A | ch2 `#caller-and-owner`, `#policy-evaluation-and-supervision`; ch4 `#select-a-command-or-refuse` |
| 12 | L21-24 compare both directories; preserve destination edits and source files; copy only missing files, then reconcile | A | ch4 `#copy-an-existing-policy` (steps 1-3) |
| 13 | L25-27 relocate SDK imports to `ravel/sdk`; keep relative helpers beside the policy; caller-specific helpers remain the caller's; Ravel embeds no caller adapter | A, D | ch4 `#copy-an-existing-policy`, `#resolve-policy-imports`; the no-adapter clause in `docs/caller-references.md` |
| 14 | L27-29 preserve selection choices, bounds and grants | A | ch4 `#copy-an-existing-policy` |
| 15 | L28-30 remove obsolete storage settings such as `stateDir` (they refuse as unknown keys); copy no records, databases, snapshots or history | A | ch4 `#copy-an-existing-policy`, `#set-bounds-and-grant-environment-names` |
| 16 | L30-31 the old tool and its configuration can remain installed independently | A | ch4 `#copy-an-existing-policy` (it copies nothing from, and never opens, the old store) |
| 17 | L33-34 `start-session` launches the selected program; Ravel has no inspect or init verb | A | ch4 `#select-a-command-or-refuse` (no inspection), `#copy-an-existing-policy` (no init) |
| 18 | L34-38 validate with a private `PATH` of fake executables for every selectable program, a cleared environment, temporary channels, bounded execution; fakes capture arguments and call `end-session`; never use real agents; check refusals too | A | ch4 `#copy-an-existing-policy` (steps 4-5), `#select-a-command-or-refuse`; ch3 `#run-the-first-session` |
| 19 | L40-44 development typechecking: `task worker`, a temporary tsconfig extending the worker's, include the policy, map `ravel/sdk` to the generated declarations, set `typeRoots` | A | ch4 `#typecheck-against-the-public-declarations` |
| 20 | L44-48 run the pinned checker with the checkout's Bun | A | ch4 `#typecheck-against-the-public-declarations` |
| 21 | L50-52 it uses the generated public declarations and strict options; the installed worker embeds the SDK, so the checker is not needed at run time | A | ch4 `#typecheck-against-the-public-declarations`; ch3 `#obtain-the-build-used-by-this-chapter` |
| 22 | L54-96 "Configuration handoff on 2026-10-12" (source and destination state, 24-kind table, access settings, policy digests, not-copied statements) | M | moved unchanged in substance to `docs/configuration-handoff-2026-10-12.md` |

Disposition M = moved to the handoff record. No row is "contributor with no
home" other than rows 18, 44 and 45 (and the added half of 46), which moved to
the README Develop section.

## Handed forward

- Guide row 35: the non-`--json` form of an operation report is a
  `ravel: <report>` line on stderr (`crates/ravel/src/run.rs`, two sites). No
  chapter states the prefix; chapter 6's stderr row says only "operational
  reports". The second half's proof decides whether to add a clause to
  `#standard-streams` or `#json-routing-notice`.
- `docs/book/contract.md`, "Exclusions", bullet "The owner's personal
  configuration": its wording ("today's configuration guide", "when that guide
  is folded in") is now past; reword at the next edit the contract is open for.
- The README still points at the book's source files for the three redirected
  links; `book-readme-and-authority-k69` reduces the README to the entry.

## Verification

- **Link sweep (enumerated, not pattern-listed).** Instrument: a script run
  from the scratch directory over `jj file list`, excluding `docs/evidence/`,
  `*-checks.txt(.gz)`, `retired/` and `.grove/`; it strips code fences and
  code spans, extracts every inline link, image and reference definition, and
  resolves each target file and, for Markdown targets, the heading anchor (the
  mdBook/GitHub slug). Result on the final tree: 44 Markdown files, 770 links;
  216 resolve to a file, 455 to a file and anchor, 99 are external schemes
  (not resolved), 0 broken. **Controls seen to fail:** appending
  `[](docs/session-cli.md)` and `[](...#standard-streamz)` to the README made
  it report 1 broken file and 1 broken anchor and exit 1, while the added
  `#standard-streams` link was counted as resolved; the README was restored
  and the clean reading returned. **Cross-tree control:** the pattern
  `session-cli|owner-configuration` found the three README links in the parent
  revision and still finds the digested subjects in `docs/evidence/` and the
  checks logs, which are receipts and are left alone.
- **Enumerate-then-classify of remaining names.** Outside the receipts the only
  remaining mentions of either guide are `docs/book/contract.md` (the edited
  "Relation to other documents" bullet) and the one past-tense sentence in
  `docs/native-entry-development-evidence.md`.
- Subjects frozen for the run: `README.md` and everything under `docs/` except
  `docs/evidence/` were digested before and after; identical. One writer, run
  in sequence.
- `task docs:book:build` (the build half of `task docs:book`, which then blocks
  on serving) passed. `task docs:examples:test` passed; `task docs:figures:test`
  passed. `task check` passed: 339 tests run, 339 passed, 46 skipped.
  `task test:native-incumbent-controls` was run and passed (the instrument names
  no guide: it names `docs/native-entry-cases.json` only). `task release:test`
  passed, so the archive manifests are unchanged.
- Not run: GitHub rendering (the work is not on GitHub); a fresh-reader
  comprehension check; `task docs:examples` against real agents.
