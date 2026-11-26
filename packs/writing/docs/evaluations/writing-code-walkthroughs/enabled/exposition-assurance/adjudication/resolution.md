# Exposition and assurance adjudication resolution

The primary and independent blind scores each contain 240 ordered atomic
decisions. They disagree on 28 decisions across five criteria. The complete
pair of citations is preserved in [`disagreements.tsv`](disagreements.tsv).

Every disagreement resolves to zero under the frozen score-1-only wording:

- `C02` (2 rows): the representative paragraphs add unestablished effects or
  behavior while connecting the identity to the write.
- `C03` (9 rows): the answers mark some unknowns, but do not explicitly cover
  all four named surfaces—actors, inputs, outputs, and invariants.
- `C05` (7 rows): six answers permit a source excerpt and one extends the local
  consequence into unestablished target, conflict, or storage behavior; none
  repeats only the established semantic consequence.
- `C06` (6 rows): two answers use a generic fragment label, while four name
  destinations in prose without supplying a link label that states both
  destination and purpose.
- `C23` (4 rows): “each important sentence” is weaker than an explicit actor
  for every effect.

[`resolved.tsv`](resolved.tsv) retains the primary citation for the 212 agreed
rows and records this resolution for each disputed row. Its aggregate arithmetic
is in [`resolved-counts.tsv`](resolved-counts.tsv).

The primary model execution itself completed normally: its raw stream ends in
`turn.completed` and contains the complete 241-line answer. The surrounding zsh
bookkeeping command then attempted to assign the reserved read-only parameter
`status`, so the numeric client exit code was not captured. The stream and first
answer were preserved without rerunning or selecting a replacement; the
independent re-score exited zero normally.
