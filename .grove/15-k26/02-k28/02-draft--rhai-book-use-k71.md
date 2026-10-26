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
