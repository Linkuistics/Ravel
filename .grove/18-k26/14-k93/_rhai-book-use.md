# rhai-book-use-k93 — brief


## Goal

Make the procedures in chapters 1–6 supply the context, files and actions the reader needs.
Teach the renamed configuration interface.
Resolve F17 and F18 before the owner resumes the interrupted chapter 4 read.



## Context

- Read the whole contract and all documentation lessons, including L67.
  `rhai-book-use-k74` preserves the owner's findings, TestSubject intention and bounded LLM trial.
- `configuration-names-k92` has landed the product rename. Derive the interface from source and the built CLI.
  Do not retain old literal-name explanations or copy the previous proof's failure schema.
- Copy-edit `rhai-book-use-k94`, art `rhai-book-use-k95` and proof `rhai-book-use-k96` already follow.
  Cut no duplicate editorial tail or reader checkpoint.

## Done when

- Read all six chapters against the complete accumulated contract and lessons, especially instruction order and example preparation.
- Chapter 4 names the example and shows its workspace, directories, file paths, purposes and contents before asking the reader to act.
  Distinguish manual inputs from what the command creates. Show a complete command before explaining its parts.
  State each later example's differences where they occur. Keep fragments visibly distinct from complete procedures.
- Each procedure supplies or explicitly references its prerequisites and needed context.
  The reader can tell where to act, what should exist, what the action achieves and how to interpret its outcome.
  An example-checker pass does not establish that its automatically prepared inputs were explained to the reader.
- Files, arguments, settings, fields, diagnostics and help agree with the implementation.
  Current links and reference fragments agree across chapters. Re-run changed examples whose measured output a fragment records.
- Preserve the ordinary reader-facing prose and commands as the input for a future TestSubject trial.
  Add no hidden preparation plan that supplies missing instructions to an LLM.
- Complete examples, exact excerpts and the book build pass. Strict prose and break checks pass for changed chapters.
  Record figure work under Handed forward for art, with exact locations. Record new transferable lessons.

## Notes

- Read 1 resumes at the interrupted setup section after proof. The owner need not repeat Part I.
  The fake-agent, refusal, cancellation and direct-reference task results remain required.
- The first LLM trial ran the example but did not identify the owner's setup confusion.
  Its model evidence does not reject F17 or establish novice comprehension.

## Decomposition

Chapters 1–6 have about 7,640 lines and 417 KB. One session cannot read them in full, edit them and run the checks.
The two earlier correction drafts split at the same size. This node uses four drafts by chapter group.

- `rhai-book-use-k115` takes chapters 3 and 4. Chapter 3 has the book's first procedure, and chapter 4 has the F17 setup section.
  It writes the new setup pattern first: a named example, its workspace and files, then the complete command, then the parts.
- `rhai-book-use-k116` takes chapter 5. Its “Run the examples of this chapter” section has the same shape as chapter 4's F17 section.
- `rhai-book-use-k117` takes chapters 1 and 2. They have no complete example, but they show real-run fragments and configuration names.
- `rhai-book-use-k118` takes chapter 6, because it quotes the examples of the other chapters (L64).
  It also runs the mechanical gates for all of chapters 1–6, and it closes this node.

The four drafts share the stem `rhai-book-use`. Refer to each by key.
Copy-edit `rhai-book-use-k94` follows this node. No draft cuts a later stage.
