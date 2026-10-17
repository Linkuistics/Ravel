# reference-chapter-k97

## Goal

Proof the whole chapter 6 (`docs/book/src/06-reference.md`) under
`docs/book/contract.md`, including its lookup entries and the figures added by
art. Re-enumerate inventories 1 and 2 from the current subject and check every
entry against the source and the overlapping chapter 4/5 teaching.

## Context

- Both drafting parts and copy-edit are complete; their enumeration and
  decisions are in k75, k94 and k95. Art k96 adds reusable
  `source-components` (C4 component) and `installed-files` (UML deployment)
  beside the retained installed-path table, using the existing pinned
  PlantUML/Smetana/shared styles. Sources and SVGs live in src/figures.
- Read the whole chapter, not just the new art. Preserve settings validation
  before overrides, caller versus delivered policy context, notice emission
  before settlement, conditional operationReports, error/cancellation
  precedence and worker versus managed environments. The message_too_large
  entry includes oversized loader failure reports at context.
- Check the source/installation relationship in each view: two Rust packages;
  runner linked inside the native front; a separate compiled worker with Bun,
  SDK and static example embedded; adjacent editable sources/declarations,
  sample and notices; discovery from the native front's canonical path;
  confirmed worker reap and private-directory removal before managed launch.
  task install supplies nine files; archives additionally supply README/LICENSE.
- The new sample-choice table preserves careful's exact-high replacement
  even for xhigh routes and refusal for missing routes. Stream and import
  tables now have explicit reader takeaways. Figure explanations are labelled
  blockquotes, teach their notation locally and supply meaningful alt text
  and full-size links. No product or extraction claim was added.
- Art's checks passed: nine matching figures, thirteen complete examples,
  six labelled fragments not run, warning-free book build, and 289 chapter
  link/image destinations resolved. mdBook emits an inline and zoom img for
  each of the two figures; test the assets and every alt-text instance rather
  than assuming one HTML img per Markdown image. The node brief records the
  bounded stability check and the leaf records the initial ad hoc assertion
  failure. Art's evidence does not replace proof's checks.
- docs/book/figures.md records local Chrome/light-page and raw-file viewing,
  including final full-size and chapter-width diagrams; the final sample
  comparison was also viewed. Editor preview, other themes, narrower viewports,
  GitHub and the published site were not inspected in art. Inspect the
  declared surfaces you can reach and record proof's own coverage and limits.

## Done when

- The whole chapter passes proof, with inventories re-derived from CLI/source,
  gaps or disagreements resolved through proof's charter and bounded correction
  rule, and each entry self-contained with a working teaching destination.
- task docs:figures, task docs:examples and task docs:book:build pass; every
  chapter link resolves in built HTML. Figure relations, keys, alt text,
  captions and lookup tables remain correct and legible.
- Retire proof and, if the document node's Done when holds, close
  reference-chapter-k66 and promote the needed context to ravel-book-k55 for
  the first-half set proof. Do not grow a normal successor after proof.

## Notes

No correction run is queued. Mechanical, source and model viewing checks
establish no human comprehension, terminal or real-agent operation, native
x86-64 qualification or remote viewing. The owner's stand-in read remains
later, and chapters 7–12 remain unwritten except where their later work lands.

## Decisions (running log)

- Read the whole chapter, its two editable figure sources, the document
  contract, term list and cited ADRs. Re-derived inventories from the current
  CLI help, argument definitions, runtime source, public SDK, shipped sample
  and installation manifest. No chapter correction or correction run is
  needed; the lookup entries agree with the overlapping chapter 4/5 teaching.
- Keep the existing distinctions: settings validation before overrides;
  caller versus delivered policy context; notice emission before settlement;
  conditional operationReports; cancellation and original-error precedence;
  worker grants versus managed inheritance; exact-high sample replacement;
  linked runner versus separate worker and embedded versus editable modules.
- Tier 2 graph discovery found evaluate_and_resolve in choice.rs. Its complete
  depth-one trace has nine callees and one caller, choose_prepared; its exact
  snippet puts successful worker evaluation before command resolution. Direct
  worker/run/context/host reads establish the reap/removal admission gate,
  notice timing, outcome precedence and measurement rules. Literal enumeration
  complements graph evidence rather than treating graph coverage as proof of
  completeness.

## Subject-derived proof inventories

Enumerated on 2026-10-16 against this checkout after `task build`. Scratch
enumeration and help captures are under `target/k97-validation`.

| Surface | Enumeration from the subject | Coverage result |
|---|---|---|
| CLI | Two verbs; top-level help and version with short aliases; help on each verb; fifteen start options (eleven selection fields and four run fields); one end RESULT; option terminator. | Every item has local semantics and a teaching link. Only param, policy-env and runtime-read repeat. |
| Status and streams | Eleven normal numeric statuses: 0, 1, 2, 3, 5, 124, 126, 127, 129, 130, 143; last three also have process-level signal termination. Three streams. | Tables retain publication versus delivery, missing versus empty, partial writes, and help/version exceptions. |
| Notices | Twelve possible routing fields, including conditional operationReports; six delivered-context report fields; eleven error fields (five required, six optional); ten stages. | Fields and emission order match run.rs, context.rs and refusal.rs. |
| Environment | Four exact worker base names plus LC_; explicit open grants; one host cache override; nine excluded name/prefix classes; six exact managed scrub names; five exact confined retained names plus LC_, three temporary names and fresh RAVEL_EXIT_FILE. | Source rules cover arbitrary inherited/granted names. HOME, PATH, system temporary-directory selection and end-session's inherited path have entries; XDG_CONFIG_HOME is explicitly unused for discovery. Compile-time identity and private test/fault controls are not installed CLI inputs. |
| Entry and settings | Explicit/default canonical entry admission; fixed settings path; three keys timeoutMs, contextBytes and policyEnv, plus unknown-key refusal. | Every key, range, default and validation-before-override rule is covered. |
| Bounds and shapes | Six bounds; four policy fields; nine request fields; seven Selected and four Refused fields; six caller-context fields plus delivered measured. Nested Assessment/SourceRecord/MeasuredSource/ReadSource have 2/4/4/3 fields; four public host members and both read wrappers. | Names, presence, limits and consequences agree with source and chapters 4/5. |
| Diagnostics | Every lowercase underscore string literal in front Rust modules outside named test files and trailing unit-test modules was classified: 49 codes and four non-codes (prompt_file, spawn_missing, spawn_other, remove_dir_all). | Code-table rows equal the code set. A deliberately wrong message_too_large row fails the comparison; chapter 4 and source provide cross-tree controls. The context-stage oversized loader-failure case remains explicit. |
| SDK and resolution | Nineteen SDK exports (sixteen types/interfaces, two functions and CHOICE_FILE); two exact embedded imports; file/package/alias/no-file resolution and four explicit autoload choices. | Export names equal their rows; readChoice's byte/file/name rules and both helper codes are local. |
| Sample and layout | Six static exports, five routes, two sample choices; two Rust packages, two executables; eleven archive paths, of which nine are installed by task install. | All names have entries. Missing routes refuse; careful replaces even xhigh with high. Installed files, editable copies, declarations and canonical worker discovery agree with source and both figures. |

The first scratch CLI-field regex omitted the multiline prompt annotation;
it was replaced by struct-field enumeration, and the resulting fifteen names
equal the help options excluding help. Only that corrected result is credited.
No item is missing from these bounded inventories. The chapter repeats no
real-agent flags, so the dated agent checks remain chapter 4's evidence.

## Validation and closure

- All three required gates exit 0: `task docs:figures` matches nine figures;
  `task docs:examples` passes thirteen complete examples, zero failed/skipped,
  six labelled fragments not run and zero page errors; `task docs:book:build`
  completes without warnings. All 178 files enumerated under crates, scripts,
  docs/book, docs/specs and docs/adr, plus Cargo.toml, Cargo.lock, Taskfile.yml
  and CONTEXT.md, retain their SHA-256 before and after the completed gates.
  Installed tools/dependencies are outside this stability claim.
- Every chapter link/image destination resolves in built HTML: 289 occurrences,
  34 unique destinations. All 184 rows in tables with a Teaching column have
  anchored chapter 2–5 teaching links. Both figure assets load; all four
  inline/zoom img elements retain meaningful alt text. Deliberately missing
  file, anchor, alt and image inputs are rejected by the same checker. Its
  first ad hoc teaching assertion incorrectly applied to a two-column notice
  table's internal link; restricting it to tables declaring Teaching fixes
  the assertion without changing the chapter. Only the corrected result is
  credited.
- Coverage generation 2026-10-16T07:14:45Z reports no recorded gap and matching
  metadata on the relevant runtime/SDK/example scopes and source/spec/teaching
  files. SVGs are excluded and PlantUML freshness untracked: their sources
  were read directly and the two SVGs inspected through the local build.
  figures.md's changed metadata reflects this session's viewing note, read
  directly. The broad coverage request also included unrelated partial test
  and QEMU scripts; no structural claim relies on those files. The initial
  over-128-path request was refused; two bounded requests supplied the result.
- Local Chrome/raw viewing and its unavailable editor-preview/remote surfaces
  are recorded in docs/book/figures.md. These mechanical, source and visual
  checks establish no new-reader comprehension, real-agent execution or
  permissions, terminal interaction or native x86-64 qualification. Chapter 10
  remains intentionally unwritten and the owner's stand-in read remains later.
- No in-session reviewer or correction run was needed. No Handed forward
  defect remains in the document brief. Its Done when holds: SUMMARY links
  chapter 6, every inventory item has a lookup entry and teaching destination,
  and all four editorial stages are complete. Close reference-chapter-k66,
  promote the needed evidence and limits to ravel-book-k55, and leave that
  parent open for its existing set proof, migration and reader work. The ADRs
  require no change. No live review names this leaf or the closed node, so
  no Creator line is due.
