# what-ravel-is-for-and-its-actors-k124

## Goal

Correct chapters 1 and 2 for the owner's read 1 findings R3–R7 and R9. Write
all new and changed text in STE, with the word decisions in
`docs/book/ste.md`.

## Context

- Cause: read 1, `book-read-one-k70` (its log, 2026-10-17). The owner read
  chapter 1 and paused the read so that these corrections land first. This
  is the third correction run on this document, after k78–k81 and k102–k105.
  The owner authorised it. A fourth run goes to the owner.
- The contract changed in the same session: ASD-STE100 is the prose base,
  “send” replaces “publish” for the result, and six chapter titles became
  commands. Read section 2 of `docs/book/contract.md`, `docs/book/ste.md`
  (from k123) and `CONTEXT.md`.
- The findings, in the owner's words. Lines are at commit `5bc411d1` in
  `docs/book/src/01-what-ravel-is-for.md`:
  - R3, line 30: “"In the review scenario ..." add "mentioned above". This
    makes it clear what we are talking about.” The scenario is the
    code-review orchestrator on line 15.
  - R4, lines 34–36: “"Their owner policy is Typescript ..." add a comment
    that this is a configuration file. That immediately contextualises it.”
    The default file is `$HOME/.config/ravel/policy.ts` (chapter 6,
    `policy.ts`).
  - R5, lines 27 and 30: “"It invokes ravel start-session ..." make that "The
    owner", because "it" doesn't work if it is a person.” The owner then
    confirmed “The caller”. Both pronouns stand for the caller.
  - R6, lines 51–61: “"If the caller chooses "continue" ..." make that 'For
    example, the caller may choose (the text) `continue` as the return value
    ...' and later mention that it means something to the caller, but not
    ravel. The notion of what the RESULT is, is not 100% clear and could do
    with a simpler more explicit explanation.”
  - R7, line 63: “"Publishing the result ..." - publishing is a confusing
    meaning here.” The owner chose “send” for every use.
  - R9, lines 26–32: “We should mention that the prompt supplied by the
    caller probably use the kind as a way of selecting skills, as well as way
    that ravel selects a harness with model/effort params.”
- R8 (the colour text before each figure) belongs to set art k130. Do not
  change figures or figure prose.
- Facts for R9. The owner policy, not Ravel, selects the harness and its
  arguments, for example `--model` and `--effort` (chapter 4, lines 406–418
  at `5bc411d1`). The `provider`, `model` and `effort` fields of a selection
  are labels for the starting notice (chapter 3, line 148). A caller can also
  use the kind when it writes the prompt, for example to tell the LLM which
  skill to load. That is a caller convention, not a Ravel feature. Grove can
  be the labelled example caller (contract, Exclusions). If the text names a
  harness's skill feature, make sure that the harness's own documentation
  supports it, and record the date. No session starts a real agent.
- Chapter 2 also explains the opaque result (`02-actors-and-words.md`, lines
  96–110 at `5bc411d1`). It must agree with the simpler R6 explanation.

## Done when

- R3, R4, R5, R6 and R9 are corrected where the owner found them. The result
  explanation is simple and explicit: what the result is, who selects its
  values (the caller, in the prompt's ending instructions), what Ravel does
  with it (it keeps it unchanged and gives it no meaning) and what the caller
  does with it. `continue` is an example value that has a meaning for the
  caller but not for Ravel.
- The passages that this run rewrites use “send” for the result. Copy-edit
  k125 converts the rest of the two chapters.
- New and changed text follows STE as far as a draft can. Copy-edit k125
  owns full conformance.
- Each claim agrees with the code and with `docs/specs/session-execution.md`.
  `task docs:book:build` and `task docs:examples` pass.

## Notes

- Cut nothing. Copy-edit k125 already stands under this node. Set art k130
  and set proof k132 serve this document too.
