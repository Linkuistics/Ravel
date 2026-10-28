# configuration-names-k114


## Goal
Finish current filenames and terminology, and prove the full configuration-name
contract before the dependent book correction starts.



## Context
Follows `configuration-names-k113`. This child owns the remaining mechanical
reconciliation across the current repository surface; explanatory restructuring
and F17 belong to the later book draft. Read every parent done criterion.

## Done when
- Rename the retained `policy.rs`, affected spec/ADR slugs and design paths.
  Chapter 4 is `04-write-a-configuration-script.md`. Reconcile every current link,
  citation, module use, Taskfile entry and excerpt source path.
- Enumerate candidate old tokens over the whole current surface and classify by
  ownership/meaning. Keep historical evidence, frozen fixtures, old-input refusal
  cases, third-party names, caller-owned values and stable Grove handles exact.
- Current prose, examples, help, release checks and public names agree. Remove
  every temporary compatibility form if one was introduced.
- Run `task check`, `task test`, complete book examples, example-checker controls,
  exact excerpts, book build and relevant release/help checks. Record actual
  results and platform limits. Use firing controls for exhaustive search claims.
- Check the full parent contract against all four children before closing k92;
  promote surviving context and documentation lessons as appropriate.

## Notes
Do not rewrite historical fixture observations, installed 1.1.0, the active Grove
ending command or the owner's live files. Consult tree decomposition rules if
the remaining scope still exceeds a focused session; do not stop for incompleteness.

## Plan

1. Rename current source/document paths and reconcile consumers, then mechanically
   align current terminology while retaining historical and caller-owned values.
2. Enumerate old-name candidates across all tracked content and paths, classify
   each retained class, and exercise positive and cross-tree search controls.
3. Run the complete Rust, book and applicable release/help checks sequentially
   against digest-stable inputs. Check every parent criterion against k111–k114.
4. Promote surviving context, retire this leaf, close k92 if its contract holds,
   and seal the focused jj change before the mandated ending command.

## Decisions (running log)

2026-10-29: The graph CLI refused `list_projects` because a pre-coordination or
unverified generation is active. No graph project or generation is usable.
Use direct source reads and whole tracked-surface inventories; no graph
completeness claim is available. The existing parent brief settles the design,
and this leaf applies its mechanical rename in the driver-provided jj workspace.

2026-10-29: Use `config.rs`, `configuration-evaluation.md`,
`configuration-script-uses-rhai.md`, `configuration-evaluation-precedes-the-launch.md`,
`a-configuration-script-asks-a-model-through-a-harness-run.md`,
`docs/design/rhai-configuration/`, `docs:rhai-configuration`, and
`04-write-a-configuration-script.md`. Rename the Rhai configuration test target
and internal worker role consistently. Preserve explicit arbitrary test paths,
owner streams/codes/results, historical fixture vocabulary and Grove handles.
No compatibility module, path or task alias is needed. Later book leaves still
own explanatory restructuring and the remaining TypeScript-era narrative.

2026-10-29: The first check stopped at Clippy after the helper rename collided
with existing `config` directory bindings. Rename the shared test helper to
`rhai_script` and distinguish the CLI fixture's source text from its directory.
Clippy then passed. The next full check passed the preliminary tools but stopped
after 560 passing tests at an admission fixture that similarly shadowed its
script path with a settings directory. Give that path its own `script_path`
binding. All 35 enabled worker-reap admission tests then passed. A source-derived
scan of original functions found exactly those two pre-existing directory/script
name overlaps; both are corrected. No runtime condition or test assertion was
weakened. The failed full run read unchanged input digests.

2026-10-29: Spent the leaf's one in-session reviewer on Rust value preservation
and current documentation naming. All eight findings are actionable mechanical
corrections: preserve the historical JSON field and two literal fixture filenames
in comments, name the actual helper, retain the distinct cancellation-policy
meaning, correct the reference's explanation of `config`, and remove two obsolete
rename deferrals (one falsely claimed an anchor survived). Direct source reads
confirmed each. The same sweep also corrects the comparator parameter comment.
The reviewer found no functional or caller-literal changes in its inspected diff.
The executable admission check found the additional shadowing issue above.
These corrections need no second reviewer. L63 records the reusable lesson to
inspect replacement phrases as well as residual old tokens.

2026-10-29: Current book references to the removed worker executable, SDK and
build tasks cannot remain as current product names. Reconcile those bounded
paragraphs and tables with the single Rust executable and Rhai host capabilities.
The later Parts III–IV draft still owns the complete source walkthrough,
explanatory restructuring and its whole-book proof. The naming leaf does not
claim to complete that narrative work.

2026-10-29: Both full Rust suites now pass 611 tests with 48 skipped against
identical 3,587-file subject manifests. The renamed upgrade task's preflight
then correctly refused an outdated target inventory: it omitted the existing
harness-run, LLM-call and example-equivalence targets. Include those three in
the declared engine campaign and retain its refusal on any future missing target.
This is reconciliation of an existing consumer, not a new proof interface.
The final documentation pass also removes the remaining future-tense rename
note in `ste.md` and updates two current figure-tool descriptions; dated figure
viewing and porting records retain their historical words. Rerun the complete
check and the upgrade task against these final inputs.

## Validation

2026-10-29, aarch64 macOS, repository toolchain through `mise exec -- task`:

- Final `task check` passed formatting, ShellCheck, book Bun integrity, fake
  harness smoke, Clippy, terminal recovery and all 611 enabled Rust tests;
  48 tests were skipped. Separate `task test` also passed 611 with 48 skipped.
  The enabled test identities in both successful full checks and the independent
  test run compare equal item by item. The final check follows the upgrade-task
  inventory correction. Earlier failing runs are recorded above, not counted
  as successful measurements.
- `task rhai:upgrade-proof` passed all nine current Rhai test targets and all
  15 hostile classes. Its missing-target preflight fired before reconciliation.
- `task docs:examples` passed all 22 complete examples, with no failed or skipped
  example. The 57 labelled fragments are not executed. Example controls passed,
  and `task docs:examples:mutants` killed all 30 safeguards removed in turn.
- `task docs:excerpts` passed all nine exact excerpts. Excerpt controls passed
  and detected all 24 removed safeguards.
- `task docs:figures` reproduced all 18 figures; figure controls passed.
  The changed session-code-path source, SVG and layout record agree. The book
  build passed without a warning. Strict prose checks passed chapters 1–6.
- An offline check of the 15 built HTML pages passed all 2,061 local link/src
  targets. Its missing-file and missing-fragment controls both fired.
  Current help matches the reference's exact help block. Start-session help
  names `config.rhai`, `--config-env` and `configEnv`, with no old spellings.
  Task listing exposes `docs:rhai-configuration` and no old task alias.
- `task smoke` passed real CLI checks through direct and symlinked installations
  in a fresh private prefix, including imports, context, parameters, exact,
  empty, multiline and flag-shaped results, retired formats and stale context.
- `task release:test` passed the exact archive-manifest controls. Fresh unreleased
  archives built for aarch64 macOS, aarch64 GNU/Linux and x86-64 GNU/Linux.
  `task release:formula` passed the actual Homebrew install/test methods in a
  redirected private prefix. No release, tap or live installation was changed.
- `task native-entry:build` and the targeted native E06 boundary/join control
  passed (one selected test, 314 filtered out). The earlier focused admission
  retry passed all 35 enabled cases. This is not a full native-entry campaign.

The final campaign read identical SHA-256 manifests before and after its work.
They enumerate every tracked file outside `.grove/`: all 3,587 source,
manifest/lockfile, script, documentation, generated figure, fixture and historical
inputs. Their shared manifest digest is
`2e2f7d2c162af6b5189f90bb7e0eb93185dbf0c9b8b216db16a2df98e8a18cc0`.
The first two passing full Rust suites also had matching before/after manifests;
subsequent edits were the documented script inventory and documentation changes,
followed by the final complete check. Disposable logs, manifests, per-test
identities and inventories are under `target/k114-*`.

The old-name inventory enumerates candidate tokens and candidate filenames from
the entire tracked surface, including root files and dotfiles. Each occurrence
is classified in `target/k114-classified-candidates.jsonl`. Retained classes are:

- stable Grove paths/handles, requirements and session history;
- released history, dated evidence, frozen observations/discriminator labels,
  and dated figure porting/viewing records;
- old-input rejection/absence assertions, the literal retired-format probe and
  explicit historical comparison translation;
- arbitrary explicit test paths, opaque caller data, script-owned refusal
  text and owner `VERSION` labels;
- old-word explanations, term maps, quotations and authoring evidence;
- operating-system confinement, signal/cancellation and terminal-holder rules,
  third-party Seatbelt identifiers, and the general examples policy.

The same case-insensitive search and flags read a scratch subject of new names
clean, then found deliberately inserted singular, plural and uppercase old
forms in a hidden file. The same pattern finds the legitimate class in the
unchanged changelog and frozen fixtures. A wrong-pattern control failed to find
the planted subject. Filename controls distinguish `config.rs` from `policy.rs`
and a hidden plural-name path. Residual tracked filenames occur only in stable
Grove paths and historical evidence. No temporary compatibility form remains.
The frozen equivalence fixtures, `docs/evidence/` and released changelog have
empty jj diffs. Previously terminal Grove task records also remain unchanged.

This is macOS runtime evidence plus cross-compilation/archive evidence for both
Linux targets. No Linux runtime/floor campaign, full native-entry campaign or
signed-in harness call ran. No installed 1.1.0 executable, active ending command
or live owner file changed. The graph generation remained unavailable; the
inventory and exact source reads are the discovery evidence.

## Parent contract audit

All four children were checked against the original k92 brief, rather than only
against their own reduced goals:

| Child | Delivered contract and final evidence |
|---|---|
| k111 | Default `config.rhai`, no old-default alias, explicit arbitrary `.rhai` basenames and the literal historical `policy.ts` refusal probe. `rhai_config` covers all these through the real CLI, including exact old-token result bytes. |
| k112 | Only `--config-env`/`configEnv`; both retired spellings refuse. `RAVEL_CONFIG_RUN` preempts evaluation, reaches evaluation harnesses and is scrubbed where required. Grant ordering/privacy, confinement, re-entry, cancellation and cleanup cases passed in the final suite. |
| k113 | `config`, `configEnvironment`, `configCode`, `config_`/`config.` and both config stream prefixes. Failure schema 2, routing schema 2 and script schema 3 remain distinct. Diagnostic tests preserve owner values; frozen comparison translation and mutation controls passed unchanged. |
| k114 | Current Rust/test, spec/ADR/design and chapter paths use configuration names, with current consumers reconciled. Old tokens are classified by meaning/ownership, controls fire, all required full/book checks pass, and relevant installed/archive/formula/help/native-boundary checks pass. |

The naming contract is satisfied and k92 closes when this last child retires.
Promote the final paths and the explicit historical/caller-value exceptions to
the root brief for the dependent book work. L63 records this session's additional
naming lesson. The glossary already defines the final terms; the ADR set was
renamed and its current citations reconciled, with no new architectural decision.
The existing later book leaves retain explanatory restructuring, F17, the full
Parts III–IV reconciliation and the required human reads. The root remains live.
