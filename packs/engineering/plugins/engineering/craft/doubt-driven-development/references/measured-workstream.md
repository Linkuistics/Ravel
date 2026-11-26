# The measured workstream — where the body's figures come from

The rules in `SKILL.md` cite counts from **one** workstream: the Grove
formal-modelling campaign recorded in `Linkuistics/grove`'s
[`docs/review-yield.md`](https://github.com/Linkuistics/grove/blob/main/docs/review-yield.md)
and [`docs/candidate-lessons.md`](https://github.com/Linkuistics/grove/blob/main/docs/candidate-lessons.md). Read this file when you want to know how hard a
figure leans, or whether it transfers. Nothing here changes what you do; the
body carries the rules.

**One grove, one subject matter, one operator**, in a repository whose whole
purpose was rigour about evidence. **The rates are rates there.** What transfers
is the structure they expose, not the numbers.

## Two channels, two instruments

- **Tree-level (`T`)** — nine `review-*` → `integrate-review-*` chains. Derived:
  every heading, top-level list item and table row in the eighteen review and
  integration bodies is extracted from markdown block structure at a pinned
  revision, under a counting rule fixed before any count was taken, with
  coverage asserted in both directions.
- **In-session (`I`)** — the fresh-context reviewer a producer spawns, recorded
  only in that producer's own body. Those bodies are outside the derived set of
  eighteen, so the channel is counted by hand from each session's own four-way
  classification.

The two are never summed into one curve, and a comparison across them is a
comparison across instruments.

## What each channel found

| | tree-level | in-session |
|---|--:|--:|
| readers | 9 chains | 3 spends |
| findings raised | 45 | 24 |
| real issue / actionable | 43 | 17 |
| contract stated unclearly | 2 | 3 |
| trade-off accepted | 0 | 1 |
| noise | 0 | 3 |

**Nothing at tree level was graded a trade-off or noise; 4 of 24 (17%) in
session were.** State the contrast in those terms — *survived verification*
against *trade-off or noise* — and not as `valid` against `non-actionable`. Under
the skill's own four-way precedence, *something other than actionable* covers
contract-misread too, which is **7 of 24**, not 4.

**Why the asymmetry is structural rather than a compliment to the reviewers.** In
five of the nine chains, the integrating session's task body had been written by
the reviewer, so its charter *was* the finding list and `Done when` the same
list restated as obligations. Counter-evidence exists and is partial: four late
integrations state that they verified against the artifacts rather than the
review's summary, and one found the defect was **worse** than the review knew (a
Quint invariant true by construction, which no mutation could have moved); one
in-session reviewer **falsified** a finding an integration had raised itself;
both contract-unclear downgrades are real gradings. Nine chains cannot separate
the structural reading from reviewers who were simply right every time — which
is why the repair is the cheap structural one and not a claim about anyone's
accuracy.

## The three spends and the five declines

All three sessions that spent the allowance were **changed** by it, and not one
returned a second opinion:

- one reviewer broke a disposition its session had already landed, forcing a
  reversal across nine artifacts;
- one swept all **63** of a library's invariants against the candidate's module
  and found two violated, one nobody was looking for;
- one made its session withdraw a sentence it had already written.

Five sessions considered the allowance and declined, for four distinct reasons —
executable tests as the stronger evidence; a later leaf already chartered to
contest the verdict; a doubt a fresh read cannot discharge; and the harness
forbidding the spawn. Only the last is a constraint rather than a judgement, and
in both cases it hit, the session **cut a scheduled review leaf instead** — one
of them calling that the stronger instrument.

## The relay, and both of its weakenings

The most-quoted result is three readers of one artifact, each finding what the
previous two had not. It is **one node** — the node the loop had just selected
for review a third time, because it was the hardest, so this is selection rather
than sampling. And the three readers were **aimed**: the second's charter carried
the first's specific doubt, and the third's work list *was* the second's
findings. A chain of three *unaimed* readers was never run there, so the result
is no evidence that fresh contexts compound. The two findings nobody aimed at
came out of a **sweep the third reader ran to verify a finding** — an instrument,
not a reader.

## Why the finding count cannot be read as decay

Findings per review, in run order: **6, 6, 7, 6, 6, 3, 4, 2, 5** — first five
mean **6.2**, last four **3.5**. Four reasons that is not decay:

1. **The gap is inside the noise.** Against Poisson variation at this mean, and
   *before any confound*, the 2.7-finding difference is about **1.8σ**. Reaching
   **3σ on that effect** would need roughly twelve chains per group — about **25
   chains of matched subject size**. The 25 is a target for that specific effect
   at that specific confidence, not a general sample-size threshold.
2. **The subjects are not matched, and not randomly so.** The first five reviews
   read first-of-their-kind artifacts; the last four read incremental decisions
   on an artifact already reviewed two to five times.
3. **The review rate rose while the yield fell.** 42% of `design` leaves earned a
   review against 13% of `prototype` leaves: the loop reached further down its
   own value curve, and every review at the margin still found something.
4. **The unit is elastic.** Sub-corrections per finding run **1.0 to 3.3**, and
   consequence is not captured at all — two whole verdict reversals count the
   same as a corrected transition count.
