---
name: research-family
description: The procedure that the research-a and research-b session kinds share — what a survey owes and what the pair owes each other. Use when the grove-research-a or grove-research-b skill directs you here.
harnesses: [any]
---
- **research-a** and **research-b** (both AFK) — a citation-disciplined
  literature/prior-art survey, breadth-seeking. No grilling, no tree growth. The
  two kinds are identical in discipline and distinct only in what the owner's
  policy launches for each, so a single survey that needs no second corpus is
  `research-a`.

## What the survey owes

- **A citation per failure-mode claim.** "That tool had this problem" without an
  issue link, a thread quote or a post-mortem is mood, not evidence. Primary
  sources; those citations are what an ADR's rationale is later built from.
- **A walk-away check per system.** For each prior tool, answer: with the tool
  uninstalled, what is still legible? It is the cheapest invariant to demand and
  the most revealing — it separates the architectures that can be borrowed from
  the ones that cannot.
- **An explicit note where the search found silence.** "No primary source found"
  is a finding: the absence is a confidence signal, and recording it stops a
  later reader re-running the same fruitless search.

## What the pair owes each other

**Both researchers get the same brief.** The pair buys breadth, and breadth
comes from the corpora differing, not the questions; two briefs produce two
surveys you cannot union.

**Neither researcher is adversarial.** Framing the second as "find what the first
got wrong" discards the breadth you paid for and biases it toward the first
survey's frame. Both run breadth-seeking; the adversarial move belongs to
`combine-research`.

**The kind decides where you write**, because a pair's leaves share one slug and
would otherwise clobber each other: `research-a` writes
`docs/research/<slug>-a.md` and `research-b` writes `docs/research/<slug>-b.md`.
A solo `research-a` still writes `-a.md`, and a `-b` survey added later renames
nothing.
