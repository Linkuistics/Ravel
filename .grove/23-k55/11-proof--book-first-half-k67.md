# book-first-half-k67

## Goal

Read chapters 1–6 of the book as one set, after each unit's own proof, and
leave Parts I and II ready for the guide migration and for read 1. This is the
`proof` stage for the set, not a repeat of the units' proofs: a unit's proof
reads only its unit, and the entry routes, the inventories and the cross-chapter
terms span the units. The contract is `docs/book/contract.md`; this leaf names
it as its contract, and its sections 1–3 are the classes in charter.

## Context

- Every unit under this node (chapters 1–2, 3, 4, 5, 6) has retired through
  draft, copy-edit, art and its own proof. Read the node brief's "First half"
  section and the units' nodes for what each handed forward.
- Run, and record, rather than recall: `task docs:book` (build), `task
  docs:examples`, `task docs:figures`, `task check`. A viewing environment the
  session cannot reach (GitHub's rendering before a push, the published site)
  is named as unchecked.

## Done when

- **Entry routes**, read in order through the real book: the learning route
  through chapter 6; "Running sessions from your own tool" (1–3, then 5, with 6
  for lookup); "Writing a policy" (1–3, then 4, with 6 for lookup). Each
  arrives with the terms and context it needs. A forward reference to a
  chapter 7–12 is plain text and a forward reference to chapter 6 is a
  page-level link; both resolve or are named as the second half's.
- **Direct entries**: every chapter 6 entry, every exit status, every error
  message the book explains and every figure is readable on arrival, with its
  local context and a link back. Sample at least one of each class; enumerate
  the exit statuses and error messages from the source and check each is
  present.
- **Coverage inventories 1 and 2**, re-enumerated from the subject (the CLI's
  help and argument definitions, the SDK's exports, the settings reader, the
  spec), not from the contract or the units' logs, and compared with the
  chapters. Inventory 3 (lifecycle and failure outcomes) is checked for what
  chapters 3 and 5 claim; inventory 4 (component obligations) and 5
  (contributor workflows) belong to the second half and are reported as not
  yet claimed, not as covered.
- **Technical truth**: every behaviour the chapters restate is checked against
  its rule in `docs/specs/session-execution.md`, `docs/specs/result-job-completion.md`
  and the three decision records; a disagreement is fixed in the chapter (the
  specifications are normative) or reported to the owner if the code contradicts
  a specification.
- **Terms and notation**: the term risks hold across chapters ("context" is
  never unqualified; "session" is never a POSIX session unannounced), each
  notation is explained once where it first appears, and `CONTEXT.md` is
  current.
- **House style** over the whole set (British spelling, no normative
  keywords, front-loading, example labels), and no placeholder text.
- Nothing is handed forward within Parts I and II; items for the second half
  are written into the node brief's "Handed forward" list.

## Notes

- This session may fix what its charter owns (the proof class) and must send
  what another stage owns back as a correction run under the editorial family
  file; a second re-run of one stage against one document is an escalation.
- Read 1 follows the guide migration. Do not hold read 1's tasks here: the
  owner's comprehension is not this session's to judge, and a model's read is a
  proxy reported separately.
