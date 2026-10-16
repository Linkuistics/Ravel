# reference-chapter-k95

## Goal

Copy-edit the whole completed chapter 6 (`docs/book/src/06-reference.md`)
under `docs/book/contract.md`, after both planned drafting parts.

## Context

- Part A is `reference-chapter-k75`; part B is `reference-chapter-k94`.
  The node brief carries shared context; each producer records its inventory
  and checks. The structure brief is `docs/book/contract.md`.
- This is the one lookup chapter: keep each entry independently useful,
  front-loaded and linked to anchored teaching. Notes/examples/summaries
  beside a statement are labelled blockquotes. Use British spelling and the
  contract's term discipline, especially qualified policy context versus
  session-result context and result versus exit status.
- Preserve the distinctions established in part A: settings validation precedes
  overrides; notice presence proves neither launch nor success; operationReports
  is conditional; worker grants and managed environments differ; cancellation
  signals may appear as signal termination or numeric fallback; failure can
  leave a partial stdout write. Do not shorten away those conditions.
- Read part B's final scope and any added handoff before starting. Structural
  or technical defects belong to draft under the editorial correction-run
  rule; this stage owns prose, not a second unrecorded technical draft.
- Part B adds the configuration fields and 49 diagnostic-code rows. Keep
  `message_too_large`'s context-stage loader failure case and the sample's
  exact `high` override (including static `xhigh` routes). Preserve the local
  distinction between caller `request.context` and delivered `measured` data,
  and between installed editable source and embedded runtime modules. The
  node's art handoff identifies the figure relationships now stated in prose.

## Done when

- The whole chapter meets the prose contract and passes its document checks.
- Clear any copy-edit handoffs closed; route out-of-charter findings under
  the editorial procedure, with precise locations.
- Last act: grow one `art` leaf with bare stem `reference-chapter`, unless a
  live later sibling already holds it. The art stage owns the reusable
  package/worker/SDK source-and-installation C4/UML views identified in the
  node brief and book contract.

## Notes

Part A's mechanical checks establish no human comprehension, terminal or
real-agent operation, native x86-64 qualification, or remote viewing result.

## Decisions (running log)

- One spelling per term, matching chapters 4 and 5: "the front" became the
  native front throughout; "managed program", "child" and "managed group"
  became the managed process and the managed process's group; bare "channel"
  became result channel; "harness" in the `model` row became the selected
  program; "fake programs" and "fake routes" became fake agents. "Result"
  meaning the policy's output is now "selection result" (exit status 3 row,
  `message_too_large`), since the term list reserves "result" for the opaque
  result.
- Session-result context versus result channel: the `RAVEL_*` grant row, the
  JSON notice's `context` field, `--context` and `SelectionRequest` now name
  the session-result context (the inherited `RAVEL_EXIT_FILE`) rather than a
  "result-channel name", "session-result channel" or "session result".
- Status 2 is not called a refusal: the reserved-grant paragraph now says the
  names are not accepted and a grant naming one reports `excluded_grant`.
- First-use links added for native front, refusal, runner and the worker's
  base variables; the policy-worker and session-result-context links moved to
  their first uses (deadline paragraph; `--context`).
- Figurative or uncheckable wording replaced by mechanism, each checked
  against source: JSON routing notice timing (`run.rs`: prepared after
  selection, result-output reservation and result-channel allocation; printed
  after the supervision attempt); "teardown grace" as one second after TERM
  before KILL (`worker.rs` `CLEANUP_GRACE`); "execution context" in
  `program_not_found` as missing path or bare name on no `PATH` entry
  (`program.rs`); "probe workers" as test-only probe builds
  (`worker/src/main.ts`, spec); "package shadow" and "embedded registration"
  as chapter 4's wording; "Version 1's catalogue" as a version-1 policy with
  `catalog` and `routes` (`policy.rs`); "Legacy dispatch controls" as names of
  the earlier `harness-dispatch` command (`docs/RELEASING.md`); "Ravel's own
  cache setting" as `BUN_RUNTIME_TRANSPILER_CACHE_PATH=0`; "Build identity
  pairs" as admitting only a worker with the front's own build identity;
  "stays primary" as deciding the code and status; "normally" and "commonly"
  hedges removed with their condition stated.
- About seventy slash pairs written out (chapter 5 precedent); `GNU/Linux`,
  `I/O` and the signal lists `129/130/143`, `HUP/INT/TERM` kept.
  `output_limit` states 262,144 bytes (256 KiB) like the bounds table.
- Bare demonstratives given nouns (reserved classes, disabled autoloading,
  the `careful` choice, the deadline).
- Installed-layout lead-in now says `task install` installs the first nine
  rows and only a release archive carries `README.md` and `LICENSE`
  (`scripts/release-common.sh` manifest); the rows already said so, the lead-in
  did not.
- Paragraphs whose edited lines ran past the file's wrap were rewrapped with
  links and code spans kept whole; the chapter has no code blocks or examples.
- No structural or technical-truth defect found; no correction run. The art
  handoff in the node brief is unchanged and still art's. No in-session
  reviewer was spent.
- Gates on the final text, with every chapter's SHA-256 unchanged across the
  run: `task docs:book:build` without warnings; `task docs:examples` 13
  passed, 0 failed, 0 skipped, 6 fragments not run, 0 page errors;
  `task docs:figures` 7 figures match; all 282 local links in the chapter (32
  unique) resolve to files and anchors in the built HTML, and a copy with one
  anchor mutated is reported bad. An earlier examples run overlapped an edit
  and is not credited. These checks establish no human comprehension.
