# native-entry-campaign-tooling-k40

## Goal

Build the measurement instrument the immutable campaign needs and nothing it does
not: every native instrument emits one record per manifest row identity, and one
driver freezes subjects, digests them before and after, runs the finite cases
with a single writer and reconciles the emitted identities against
`docs/native-entry-cases.json` individually. Validate the driver with failing
controls on a *development* run. This leaf takes no immutable measurement.

## Context

Child of k38, which proved bigger than one session: the instruments run whole
families inside one test function and emit no row identity, so a per-row receipt
did not exist to freeze, and no incumbent subject exists. Read the k38 node brief
(its decomposition record), `docs/native-entry-cases.json` and the k37 table in
`docs/native-entry-development-evidence.md` that maps manifest ids to tests.
Read the "provenance of a measurement" rules in the Grove execute reference.

Tests construct rows inline (integer signals, booleans) and never read the
manifest, so the mapping from a judged iteration to a manifest id is a thing to
build and to check by set equality, not to assume.

## Done when

- Each native instrument records, at the point it judges a row against a
  subject, one private record naming the manifest id, the subject role
  (repaired or the row's named broken subject), the verdict and the facts it
  already holds. Production code and installed builds gain no switch.
- One driver script, wrapped by a Taskfile task, enumerates and digests every
  read subject (source, instruments, helpers, manifests, reference expectations,
  exclusion notes, toolchain/libc/kernel domain) before and after a run, writes
  receipts under one writer, refuses a second writer or a changed subject, and
  reconciles emitted ids to the manifest by set equality in both directions.
- Each driver guarantee is seen to fail against a deliberately broken input
  (digest drift, duplicate writer, missing id, unknown id) before a clean
  reconciliation is credited.
- A development run reconciles the 153 identities; any id that cannot be emitted
  by the repaired product is named with its reason rather than papered over.
- Existing expectations are preserved: R01's 310 identities and ignored flags
  through `test:native-entry` and `check`.

## Notes

No baseline subject, immutable measurement, documentation reconciliation or
support continuation is this leaf's. Charge only new effort and record it in the
brief chain. A product or instrument defect found is repaired and recorded; do
not widen the matrix.

## Decisions (running log)

- Decompose k38 rather than run it as one session: instruments carried no row
  identity, no incumbent subject exists and every instrument edit must precede the
  frozen campaign. Only the tooling child runs here; the immutable campaign waits
  for the incumbent so that it is run once.
- Row identity is emitted by the instruments themselves through a test-only
  write-once recorder (`<id>@<subject>.json`, mode 0444, create-exclusive) rather
  than inferred from nextest names, because most rows are iterations inside one
  test function. A plan file, not the instruments, says which subjects each row
  must have; the driver reconciles in both directions by identity.
- `conforms` is the raw judgement of a subject against the row's claims. A broken
  subject is recorded non-conforming only when it violates the claim it names;
  failing another way is recorded as conforming, i.e. not a detection.
- Existing assertions are wrapped, not rewritten, where one subject has many
  (`judged` / `detected`), so every prior expectation is unchanged and a violated
  claim is a recorded verdict before the test fails.
- Ten rows have no failing broken subject and each carries its reason in the
  plan: E02 pending=no (3), E03 (2), E04 raw, E05 (3) and F07's real-death
  control. Recorded as such, not invented.
- Controls found two instrument defects before any run was credited: a jq
  `select` that shadowed `.`, and a second-writer refusal lost inside a piped
  loop under `if`. Both are fixed and seen failing before passing.
- Development run: 59 tests, 327 records, 153/153 rows, 310/310 identities
  (74 new), `task check` 338 run/46 skipped. A development run, not a receipt.
- Charge P<=25/E<=10/RC<=1, D=X=0, no reviewer spawned. The remaining price
  exceeds the balance, so k41 asks rather than k42 starting short.
