# rhai-book-use-k71

## Goal

Rewrite chapter 6, `docs/book/src/06-reference.md`, as the complete Ravel 2.0.0 lookup chapter.

## Context

- Structure brief and prose contract: `docs/book/contract.md`.
- Read the revised chapters 1–5 and derive the reference inventory from the spec, source and built CLI.
- Source files include `settings.rs`, `limits.rs`, `cli.rs`, `inputs.rs`, `context.rs`, `environment.rs`, `harness_entries.rs`, `refusal.rs`, and `rhai_worker/`.
- The document node is `rhai-book-use-k26`. Its live copy-edit leaf `rhai-book-use-k69` already follows this subtree.
- The checkout still supports the old format until `typescript-worker-cutover-k31`. Describe the 2.0.0 contract and retain that dependency.
- The narrative draft changed no chapter-6 anchor. Check links from the reference to the revised chapters and keep direct entries self-contained.
- Reconcile the example-convention note `docs/book/examples.md` with Rhai where it describes policy execution and selection cancellation.
  Its sample label and cancellation explanation still use the old worker, although the published chapter examples now use Rhai.

## Done when

- Every CLI input, setting, bound, request, selection, context field, host function, error code, exit code and notice schema-2 member has a self-contained entry.
- Bun and SDK entries are removed. No TypeScript prerequisite or migration chapter is added.
- Each complete example and excerpt passes the book checks. Links to and from the revised reference resolve.
- Chapter 6 passes strict prose checks and the book builds. Later-stage defects are in the document node's `## Handed forward` list.
- Reconcile this node's done conditions and promote relevant context to `rhai-book-use-k26` at closure.
  The queued copy-edit stage discharges the successor obligation. Do not cut a duplicate pipeline.

## Notes

## Decisions (running log)

- The structure brief is `docs/book/contract.md`, section 1. It names the reader, the ordered chapter plan, emphasis and exclusions.
  Chapter 6 remains the lookup chapter. Existing CLI, status, context and import anchors stay where their subjects remain.
- The reference describes the 2.0.0 contract. Default-entry refusal, delivery paths and CLI help still depend on `typescript-worker-cutover-k31` and delivery removal.
- Graph verification could not start. `list_projects` reported an active pre-coordination or unverified generation.
  No project, generation or coverage result was available. Direct source reads supply the evidence instead.
- The source inventory covers the CLI, inputs, settings, limits, environment, harness entries, context, selection checks, refusals and notice construction.
  It also covers the Rhai engine, bridge, helpers, resolver, host, harness runs and shipped Rhai examples.
  The built CLI confirms the public flags. Its old policy example and worker-grant wording remain cutover work.
- The example-convention note first needed a prose conversion under the controlled-English skill.
  A separate preparatory change preserves its mechanics. The following edit updates Rhai execution and selection cancellation.
- The inventory comparison extracted every quoted snake-case token from the Rust source before its test modules.
  All public Rhai codes have entries. The unmatched tokens are three TypeScript-only diagnostics, Rhai callback names, a Clap field identifier and test fault controls.
  The chapter's inline identifiers were also enumerated and classified. Retired-format extensions and reserved grant names remain contractual, not SDK entries.
- The single independent review found two actionable errors: budget enforcement follows program resolution, and harness settlement allows eight seconds before worker teardown.
  Exact source branches confirmed both corrections. The harness result fields matched their source.
- The read-reply ceiling also needs an entry: JSON escapes can exceed 19 MiB even when source bytes fit their own bound.
  This gives catchable `source_unreadable` after one source is consumed. Rhai message-bound failures use the selection stage.
- Final checks passed: 19 book examples and the extracted convention sample, nine exact excerpts, 14 reproducible figures, and the mdBook build.
  Chapter 6 has zero strict STE findings and zero prose runs above 450 words. Its 317 inbound and outbound rendered links resolve.
  The link check also accepted a known anchor and refused a missing anchor and missing page.
- Input digests stayed unchanged across verification. The manifest covers the source, scripts, docs, root build files, glossary, README, pinned tools and assets, built front and worker, convention sample and personal prose checker.
  The scratch manifests are under `target/verification/`; no generated report is a repository deliverable.
- The personal prose checker reports zero hard findings for the reference and example convention, with 11 passive-voice advisories for copy-edit judgement.
  An exploratory book-STE scan of the convention note reports 11 findings, including literal sample code treated as prose inside its nested fence.
  The book checker normally excludes author notes. The required strict chapter check passes; the normal whole-book scan retains only the previously handed-forward `CONTEXT.md` finding.
- Both reviewer findings were valid and actionable, and their exact source branches establish the corrections.
  No unclear contract, accepted trade-off or noise finding remained. No second review pass was needed.
- The draft preserves the approved section order and chapter numbers. The old SDK declarations are replaced by direct Rhai lookup entries with links to their teaching sections.
  No design decision changed, so the current ADR set needs no edit. The document node holds the art and proof handoffs.
