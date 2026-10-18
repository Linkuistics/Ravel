# reference-chapter-k75

## Goal

Draft **part A** of chapter 6, as the document node's brief describes, under
`docs/book/contract.md`, into `docs/book/src/06-reference.md`, linked from
`docs/book/src/SUMMARY.md`: the CLI inventory (verbs, options, arguments, exit
statuses, streams and notices, environment variables).

## Context

- Chapters 1–5 are proved before this runs.
- Settle the chapter's whole heading structure now, so part B fills sections
  rather than reshaping the file, and write down in this leaf's log which
  headings part B owns and how an entry is shaped.

## Done when

- Part A's entries exist and build; the enumeration it was written from and
  the counts are recorded in this leaf's log.
- Last act: `leaf-add` a `draft` leaf (bare stem `reference-chapter`) whose
  body states part B's scope from the node brief, the headings reserved for it
  and anything part A learnt that changes it; then `leaf-add` the `copy-edit`
  leaf if no live later sibling already holds it.

## Decisions (running log)

- The structure precondition is met by `docs/book/contract.md`: “The reader
  is a software engineer with no background in Ravel's context”; the ordered
  section plan makes chapter 6 the place to “look up any CLI input, setting,
  bound, request, selection or context shape, environment variable or installed
  path”; Emphasis requires success after cleanup, authority, fake-agent
  validation and stated limits, while Exclusions keeps history, evidence
  campaigns, agent installation and personal configuration outside the book.
- Part A establishes these whole-chapter second-level headings: Command line;
  Exit statuses; Streams and notices; Environment variables; Policy entry and
  settings; Bounds; Policy and request; Selection and refusal; Policy context
  and host operations; SDK and module resolution; Shipped sample; Installed
  layout. Part B owns the last eight. It fills the reserved headings without
  restructuring the CLI entries. Those sections remain explicitly marked as
  awaiting the configuration draft rather than pretending complete coverage.
- A lookup entry starts with its literal name, states meaning, defaults,
  constraints and failure consequences locally, and links to the anchored
  teaching in chapter 4 or 5 (chapter 3 for help/install orientation). Parallel
  fields use tables; an aside is a labelled blockquote. Part A adds no runnable
  examples: existing chapters carry the complete checked examples.
- Use the checkout's matched `target/debug/ravel`, built by `task build`, for
  all three help inventories. Its version string is 1.0.0, but these entries
  describe this workstream's source, not a claim about the released binary.
- CLI, source and environment lookup use Tier 2 verification. The graph is
  ready; coverage generation `2026-10-16T06:29:44Z` has matching metadata and
  no recorded issue on the 18 queried production source paths or the three
  runtime scopes. This is best-effort coverage, not completeness proof.
  The `run` trace has 43 outbound and two inbound rows, untruncated; its exact
  snippet and direct source reads establish emission/cleanup order. Literal
  environment enumeration complements the graph and excludes compile-time
  definitions, test-only controls and contributor scripts from the public CLI
  inventory.
- No overlapping CLI, environment, status or notice statement was found to
  disagree with the relevant teaching in chapters 4 and 5. Settings validation
  precedes valid flag overrides; JSON routing emission precedes final
  settlement and does not certify success; worker exclusions are distinct
  from the managed process's exact six-name scrub. These distinctions are
  stated locally rather than left to links.

## Subject-derived inventory

Enumerated on 2026-10-16 after `task build`, against this checkout. This is
part A's inventory, not a claim that all chapter 6 configuration entries exist.

| Surface | Enumeration and count | Source checked |
|---|---|---|
| Commands | Two: `start-session`, `end-session`. No help subcommand. | All three help outputs and `cli.rs::Command` |
| Top-level controls | Two option identities: help (`-h`, `--help`) and version (`-V`, `--version`). Help also occurs on each verb; version does not. `--` terminates option parsing. | Help outputs, CLI derives and `main.rs::parse_failure`; direct parse probes refuse verb-level version |
| Start options | Fifteen: kind, prompt, prompt-file, task-file, task-id, config, param, context, timeout-ms, context-bytes, policy-env, confine, runtime-read, exit-dir, json. Eleven selection fields plus four run fields; only three are repeatable vectors. | `cli.rs::SelectionArgs`, `RunArgs`, built start help; all fifteen found in chapter; duplicate `--json` directly refuses |
| End arguments | One positional `RESULT`, plus verb help; no target. | Built end help, `Command::EndSession`, `completion.rs::end_session`/`publish_result` |
| Statuses | Eleven distinct normal numeric values: 0, 1, 2, 3, 5, 124, 126, 127, 129, 130, 143. Last three represent HUP/INT/TERM; process APIs can receive signal termination. | `refusal.rs` constants, `main.rs` parse/end paths, `run.rs` settlement/supervision/write, `program.rs`, `cancellation.rs` signal mapping; the two specs |
| Streams | Three entries: stdin, stdout, stderr; help/version and end-session exceptions are local. | `main.rs`, `run.rs::ResultOutput`, worker spawn, keyed-launch confined/ordinary launch |
| Routing notice | Twelve possible top-level fields: schemaVersion, selection, policy, worker, selectionMs, context, bounds, policyEnvironment, resolution, promptSource, diagnostics, operationReports. Last one conditional. Six policy-context report fields, six named bounds, two diagnostic streams. | `run.rs::run`, `context.rs::Delivered::to_json`, `limits.rs::to_json`, `environment.rs::to_json`, `refusal.rs::Diagnostics` |
| Failure document | schemaVersion/error plus optional diagnostics; eleven error fields (five required, six optional), ten stage values. | `refusal.rs::Failure::to_json`, `Stage::as_str` |
| Worker base/grants | Four exact base names HOME/PATH/TMPDIR/LANG plus the LC_ prefix; open explicit-grant set; one host override BUN_RUNTIME_TRANSPILER_CACHE_PATH=0. | `environment.rs`, `worker.rs` spawn |
| Excluded grants | Nine exact-name/prefix entries: BUN_*, NODE_OPTIONS, NODE_PATH, NODE_PRESERVE_SYMLINKS, NODE_CHANNEL_*, LD_*, DYLD_*, RAVEL_*, HARNESS_DISPATCH_*. | Complete `environment.rs::excluded` branch enumeration |
| Managed environment | Open inheritance for unconfined launches, six exact scrub names, then fresh RAVEL_EXIT_FILE. Confined launches retain five exact names HOME/USER/LOGNAME/PATH/LANG plus LC_*; set three temporary names TMPDIR/TMP/TEMP, then fresh RAVEL_EXIT_FILE. All other caller names absent under confinement. | `run.rs`, `confinement.rs::environment`, keyed-launch `run_with_output` and confined env_clear |
| Front environment reads | HOME (entry/settings), PATH (program lookup), TMPDIR through system temporary-directory selection, RAVEL_EXIT_FILE on end-session; arbitrary names enumerated for grants/inheritance. XDG_CONFIG_HOME deliberately not used for discovery. | `choice.rs`, `authority.rs`, `settings.rs`, `program.rs`, `completion.rs`, worker/run/confinement allocation; literal environment-access sweep across crates |

The literal sweep also finds compile-time build identity/metadata and
cfg(test)-only K28_NATIVE_ENTRY_MAIN/C4_RECEIPTS; these are not installed CLI
inputs. No finite list purports to enumerate arbitrary caller variable names:
the inheritance and grant rules cover that open set. The follow-up coverage
check at generation `2026-10-16T06:36:54Z` reports matching metadata and no
recorded issue on the contract, normative/teaching documents and worker paths,
or SDK/example scopes. Source enumeration, rather than a clean graph alone,
supports coverage.

## Validation and handoff

- `task docs:book:build`: exit 0, no warnings.
- `task docs:examples`: 13 complete examples passed, 0 failed, 0 skipped;
  six labelled fragments not run; zero page errors. Part A adds no examples.
- `task docs:figures`: all seven existing figures match their sources. Part A
  changes no figure; chapter 6's package/deployment views remain the art stage's.
- Checked every link written in chapter 6 against built HTML: 98 links to
  four chapter pages and four same-page links, no missing target/anchor.
- Before/after SHA-256 lists compare equal for all 3,442 files enumerated by
  `rg --files --hidden -g '!.jj/**' -g '!.git/**' -g '!.grove/**'`, sorted
  before hashing. This includes the chapter, contract, source, scripts and
  manifests; ignores exclude build/cache output. No subject changed during
  the three checks. No product source was edited.
- Structure read: CLI inputs → statuses → output interpretation → environment
  rules, followed by the eight reserved configuration/layout sections. Each
  entry has local semantics and a teaching link. Part A has no missing item
  within the inventory above; part B still owes inventory 2, installed paths
  and diagnostic-code lookup entries, and removal of the draft placeholders.
- No in-session reviewer was used; this is source-grounded reference prose,
  with no architectural decision or ADR change. No chapter 4/5 correction is
  requested. Copy-edit still owns prose, art the figures, and proof the whole
  chapter and fresh inventory. Human comprehension, real-agent operation,
  terminal interaction, native x86-64 qualification and remote rendering are
  not established by these checks. Part A repeats no real-agent flags, so
  their dated checks remain chapter 4's; part B preserves or rechecks any it
  brings into scope.
