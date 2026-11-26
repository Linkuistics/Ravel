# Refinement rerun adjudication resolution

Scorers 1 and 2 are invalid infrastructure attempts: both returned a final
message saying nested shell sandbox setup prevented them from reading the blind
bundle. Scorers 3 and 4 received the same criteria and anonymous answers inline,
made all 240 atomic decisions, and completed normally.

The valid scorers disagree on six decisions. Resolve each under the frozen
score-1-only wording:

- `S02 C06=0`: `[source fragment]` is a generic label; it names neither the
  destination nor the link's purpose.
- `S02 C09=1`: a glossary link for the service-domain terms “canonical” and
  “tenant-qualified” does not teach ordinary implementation-language or storage
  mechanics granted by the audience contract.
- `S05 C03=0`: the paragraph states an unverified path precondition and does not
  explicitly placeholder or obligate all four named surfaces: actors, inputs,
  outputs, and invariants.
- `S06 C02=1`: “needs to be explained in terms of” is authoring guidance, while
  the paragraph marks every conditional implementation effect as a source-backed
  claim or verification obligation; it does not assert the named effects occur.
- `S08 C23=0`: “Every important sentence names the actor” is weaker than an
  explicit actor for every effect, matching the frozen campaign's prior
  resolution verbatim in substance.
- `S10 C03=0`: “the request is translated into a target identity” leaves the
  actor and transformation unmarked rather than making every unknown surface a
  placeholder or verification obligation.

The anonymous mapping is preserved in [`mapping.txt`](mapping.txt). Scorers 3
and 4 disagree on 6 of 240 decisions, including preservation-row decision
`S02 C09` and target decision `S08 C23`; all six are resolved above from the
criteria's exact wording.
