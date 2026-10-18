# the-parts-of-ravel-k169

## Goal

Take `docs/book/src/07-the-parts-of-ravel.md` through proof under
`docs/book/contract.md`. Read the whole chapter against every contract class,
verify its source claims and complete the chapter's editorial chain.

## Context

- Draft k152 records O1–O10, R1–R8 and the guide-sentence destinations.
  Derive the component obligations again from the specifications and source.
  Chapter 7 owns O1, O2 and O10; chapters 8 and 9 own the detailed O3–O9 work.
- Copy-edit k167 made the prose follow STE and standardised “managed process”.
  Art k168 kept the eight tables and added an adjacent takeaway to each.
  The decision table's final heading now says “Information at the boundary”
  because its cells include both received and sent information.
- Art declined repeated process, source and installed diagrams as editorial
  judgements. The tables and local prose carry the chapter's facts. The links
  to chapters 2 and 6 offer optional depth. Read the tables as direct entries
  and check that these links still lead to the intended views.
- `docs/book/figures.md` records the dated art inspection: all eight tables
  were legible in Chrome on a light page at the 750-pixel column. Other
  themes, narrower viewports, editor preview, GitHub and GitHub Pages were
  not inspected. The dense shared diagrams' small text stays with whole-book
  art k101.
- The final art checks passed: strict STE and prose breaks on chapter 7,
  book build, nine figure comparisons and 13 complete examples. Art did not
  reverify production source or claim reader comprehension.

## Done when

- The whole chapter meets the contract, including its direct-entry context,
  tables, terms, links, prose and evidence limits.
- The source confirms the stated package, language, process and result
  boundaries. O1, O2 and O10 have their required coverage. The chapter does
  not promise a supported extracted component or a persistent runner.
- Required book checks pass on the final files. Record the viewing surfaces
  reached and the limits, and reconcile the guide destinations with k152.
- Retire this leaf and check the document node's completion conditions. If
  it closes, promote the relevant outcome into the book brief. Cut no ordinary
  successor; route a defect only under the editorial family's correction rule.

## Notes

The scope is chapter 7. The book's second-half proof owns the wider sweep of
forward links and cross-chapter coverage. Whole-book proof owns the chapter 5
“managed program” wording already recorded in the book brief.

## Decisions (running log)

- 2026-10-18: Read the whole chapter against the document contract, shared
  word decisions, glossary, three ADRs and current source. Keep its structure
  and eight tables. They explain responsibilities without implying eight
  runtime components, a supported extraction or a persistent runner.
- 2026-10-18: Correct the worker-environment paragraph. Besides the base
  environment and permitted grants, `environment.rs::HOST` adds
  `BUN_RUNTIME_TRANSPILER_CACHE_PATH=0`. Name this control explicitly rather
  than saying that the worker receives only the base and grants. This is a
  local source correction within proof's charter; no correction run is needed.
- 2026-10-18: Re-enumerated the ten component obligations from the session
  specification and the component guide. O1, O2 and O10 have their required
  structural coverage here. Keep the detailed O3–O9 destinations with chapters
  8 and 9. The k152 sentence-group and R1–R8 maps still match this chapter.
  The cache-control correction extends its worker-startup explanation and
  changes none of those destinations.
- 2026-10-18: No architectural decision changes. The three ADRs stay as they
  are. No in-session reviewer was needed or commissioned. The node has no
  outstanding `Handed forward` entries, and this proof sends no work back.

## Source verification

Tier 2 graph navigation used the indexed project
`Users-antony-Development-Ravel.simplify-and-componentize`. Symbol search
returned all 22 matches without another page. The depth-one trace of
`choice::evaluate_and_resolve` supplies navigation; exact snippets and source
reads establish order and meaning. No graph edge alone establishes an order.
Coverage for the 36 checked source, manifest and document paths reported
generation `2026-10-18T08:31:34Z`, matching generation metadata and no recorded
gap. Chapter 7 had changed file metadata after the edit, so its actual source
and rebuilt HTML supplied the proof artifact. This best-effort coverage
signal does not prove completeness.

| Claim | Source rechecked |
|---|---|
| Two private Rust packages, executable/library split and exports | Workspace and both package manifests, both `src/lib.rs` files, Ravel `main.rs` |
| CLI owns launch pairing, result checks, cleanup and final output | Ravel `run.rs` 35–301 and `completion.rs` 1–225 |
| Policy-independent runner/channel composition | `tests/session_process.rs::supervisor_fixture`, 134–193 |
| Worker startup and checked teardown before resolution/managed launch | `worker.rs::evaluate`, 317–672; `choice.rs` 44–153; `run.rs` |
| Identity before request, canonical installed-worker lookup | `worker.rs::converse`, 742–798; `verify_hello`, 1282–1322; `locate`, 241 onward |
| Cleared environment, grants, result-path exclusion and cache control | `environment.rs`, especially `worker_environment`, `HOST`, `base` and `excluded`; worker command construction |
| TypeScript execution, embedded modules and initial directory | Worker `src/main.ts` 1–155; SDK `readChoice` and `definePolicy`; shipped compile switches and asset copy in `scripts/dispatch.sh` |
| Supported policy discovery and account settings | `authority.rs::resolve`, `settings.rs::read`, `choice.rs::prepare` |
| Direct command words and loose prompt/result contract | `policy.rs::selected`, 193–381; CLI `run.rs`; runner `run_with_output`; `end_session` and `publish_result` |
| Runner inputs, notifications, noninteractive mode and retained host effects | `keyed-launch/src/lib.rs`; `run.rs` launch/ending types, public run functions and `run_with_output`, 1184 onward |
| Shared identity inputs and compiled runtime delivery | Complete `ravel/build.rs`; worker build's `source_files`, `build_id`, compile command and installed asset copy |

O1 remains in the worker and SDK sections: executable TypeScript, embedded
SDK, supported settings and default/explicit discovery. O2 remains in the
worker and installed-pair sections: no supplied result path, checked identity
before request, confirmed reap and removal before managed launch. O10 remains
in the installed-pair section: canonical pair lookup, matching identities,
shipped SDK and compiled runtime. Trusted owner code and detached children
remain explicit limits.

O3 (judgment without fallback) goes to chapter 8. O4 (direct launch,
confinement and nested context) goes to chapters 8–9. O5 (first complete
result and invalid channels) goes to chapter 8. O6 (matching reap and group
cleanup) and O7 (cancellation precedence) go to chapters 8–9. O8 (terminal
recovery) goes to chapter 9. O9 (stdout and explicit cleanup) goes to chapter
8. This chapter teaches their owners and interfaces, not the full mechanics.

R1–R2 still land in CLI ownership, SDK compatibility and the loose contract.
R3–R4 land in the worker protocol and before-launch table. R5–R6 land in
runner inputs and result-channel pairing, appearance and contents. R7 lands
in the CLI/channel success conditions, with precise lifecycle order in
chapters 8–9. R8 lands in the installed-pair section, with delivery in chapter
12. Every guide sentence group allocated to chapter 7 by k152 has a home in
the final chapter. Its detailed and historical exclusions retain the owners
and dispositions recorded there; no guide is removed in this leaf.

## Validation and viewing

The final whole-chapter read checked all contract classes, including table
entries, local role explanations, term links, simple verb forms, paragraph
topics, optional-depth links and the evidence limits. The owner's local STE
Issue 9 writing rules supplied the prose reference, with the dictionary
excluded as agreed. No new term or shared word exception is needed.

| Fresh check | Result |
|---|---|
| `task docs:ste -- --strict` | Zero problems in 19 files |
| `task docs:breaks -- --strict` | Zero long prose runs in seven chapters |
| `task docs:book:build` | Passed without warnings |
| `task docs:figures` | All nine SVGs and layout records match their pinned sources |
| `task docs:excerpts` | Zero excerpts, failures or page errors |
| `task docs:examples` | 13 passed, zero failed or skipped; six labelled fragments not run, zero page errors |
| Chapter 7 rendered-body links | All 31 resolve; missing-file and missing-anchor controls both fail |

All 3,470 measured subject files have identical per-file SHA-256 values before
and after the checks. These are all tracked non-Grove files, plus
`target/debug/ravel` and `target/libexec/ravel/ravel-policy`. This includes the
final chapter, contract, word decisions, sources, manifests, scripts and
fixtures. Both sorted digest maps have SHA-256
`e2ab83559a2daf764bc4548d6c0442ebd1c8b6e040f031796501d45b87699304`.
Generated book output and tool/dependency caches are outside that digest.

Chrome on macOS displayed the final locally served chapter on a light page
at chapter width. All eight tables and their adjacent explanations were
legible, including the source paths, runner identifiers and the decision
table's mixed input/output heading. Raw Markdown supplied the whole-file
read. The links into chapters 2 and 6 resolve to the intended process and
source/installed views, with alternatives and full-size links present.
This pass does not extend the earlier visual qualification of the shared
diagrams. Their dense small text remains with whole-book art k101.

The first serve attempt at port 8784 found that port occupied. The normal
`task docs:book PORT=8799` then served successfully. The Code application
could not supply a window for editor-preview inspection. Other themes,
narrower viewports and other browsers were not inspected. GitHub cannot show
this unpushed chapter, and GitHub Pages cannot show it before the release
that first carries the book. These checks establish neither new-reader
comprehension, real-agent behavior nor new platform/runtime qualification.

## Node close

The document node's completion conditions hold: all four stages are done,
the chapter builds and has its new title in the contents, its structural
claims were rechecked, and the boundary reasons and guide destinations are
present. There are no outstanding handoffs in this node. Retire k169 and
close `the-parts-of-ravel-k151`, promoting this outcome and its limits to the
book brief. The book node retains live chapters and later set checks, so the
cascade stops there. No ordinary successor or correction run is cut.
