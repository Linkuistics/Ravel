# book-readme-k106

## Goal

Bring the README's opening into line with the corrected chapter 1 and the
glossary before read 1 resumes, so the repository's own door describes Ravel
the way the book now does.

## Context

- Read 1 (`book-read-one-k70`) paused for the owner's finding R1: “We should
  be explicit that the programs we are running are LLM sessions in harnesses
  such as codex or claude code. We should also point out that these harnesses
  typically cannot exit themselves, which is why we have the `end-session`
  command. Thus ravel allows you to use such interactive harness as 'sub
  components' of another process, typically an orchestrator.” The contract's
  "The intended domain" and `CONTEXT.md` now carry it, and the chapters 1–2
  correction run (draft k102 → proof k105) runs before this leaf.
- The routing session found the same generic framing in the README's first
  paragraph (lines 3–7 at commit `34c901ed`): “Ravel is a policy router and
  session process manager. A caller asks for work by kind; …” Rewrite that
  paragraph from chapter 1's corrected opening as proof k105 left it, at
  README length; chapter 1 is the authority, and the README restates it.
- **A model-proxy observation, not the owner's finding:** the README says
  “front” for the Ravel CLI four times (the entry paragraph's “The front,
  policy worker and local runner…”, “The built front/worker…” and “The
  command front composes it…” under "Develop", and “the matched front/worker
  layout” under "Earlier project"). `CONTEXT.md`'s **Ravel CLI** lists “front”
  and “native front” as words to avoid. Change the term in all four places.
  Leave the "Develop" and "Earlier project" sections otherwise as they are:
  chapters 11 and 12 absorb them later (the book node brief's "Handed forward
  to the second half" says what moves where).
- Keep the README a short entry into the book (`book-readme-and-authority-k69`
  reduced it): what Ravel is, the two verbs, where the book is and how to
  build it.

## Done when

- The README's first paragraph agrees with chapter 1's corrected account of
  what Ravel is for, and no use of “front” for the Ravel CLI remains. Check
  with a search that finds all four in the pre-edit revision.
- Every relative link in the README still resolves.
- Retire this leaf. Read 1 (`book-read-one-k70`) is next in the walk.
