# Chapter 4 saved-file inventories — k135

Measured on native arm64 macOS, 14 November 2026. This preparatory repair counts
`settings.json` in the complete examples and corrects claims about unchanged
contents. Nine introductions changed. All twelve complete examples were reconciled
against their labelled saved input blocks, before the first command block.

| Example | Saved input files |
| --- | ---: |
| fixed-route | 3 |
| refusals | 3 |
| print-and-parse | 4 |
| route-table | 3 |
| imports | 4 |
| checkout-choice | 3 |
| selection-context | 4 |
| data-helpers | 3 |
| settings-grants | 3 |
| claude-question | 4 |
| codex-question | 5 |
| interactive-routes | 4 |

`inventories.json` records each labelled input, its content digest and its
comparison with fixed-route. Manual comparison of those records with the
introductions establishes the prose reconciliation; example execution alone does
not check prose inventories. `fenced-blocks.sha256` records identical digests of
all chapter 4 fenced blocks before and after the repair, compared with the parent
commit. No executable block changed.

## Frozen inputs and results

The 437 paths from k132 were hashed after the last prose edit and before the
final runs. `subjects-before.sha256` and `subjects-after.sha256` agree.
`subjects-verification.log` records every path passing after the controls ended.
`prior-subjects.log` compares the previous k132 manifest: 436 inputs match;
chapter 4 alone differs. Evidence files here and Grove notes are outside that
inherited subject set. The private STE PDF and extracts were not added.

These commands each completed with status 0:

```sh
mise exec -- task docs:book:build docs:examples docs:excerpts docs:figures docs:ste docs:breaks
mise exec -- task docs:ste -- --strict docs/book/src/04-write-a-configuration-script.md
mise exec -- task docs:ste:test docs:breaks:test docs:examples:test docs:examples:mutants
shasum -a 256 -c docs/evidence/rhai-book-inventories-k135/subjects-before.sha256
```

`checks.log` records the successful book build, each of 27 complete examples,
73 fragments left unexecuted, zero page errors, 22 exact excerpts and 21 matching
figure builds. The lone `policy.ts` example retains its demonstrated refusal.
The figure toolchain is Typst 0.15.1, CeTZ 0.4.2 and DejaVu 2.37. STE reports
zero problems in 35 files; all eleven chapters have no prose run over 450 words.
`strict-ste.log` records chapter 4 passing strict STE. An earlier paragraph-limit
finding was corrected before the frozen final runs.

`controls.log` records the STE, prose-break and example controls, including all
30 example mutants killed. These controls use scratch mutations; production
inputs stayed fixed. Unchanged integrity, figure-control and excerpt-control
measurements remain those documented by k132, rather than new runs here.

`coverage.json` records best-effort graph coverage for the 437 paths. Partial
shell ranges in `scripts/examples.test.sh` (1–779),
`scripts/native-linux-qemu.sh` (116) and `scripts/release-smoke-qemu.sh` (184)
were read directly. Excluded, generated and unsupported files are byte-digest
subjects here; these records establish no structural completeness or visual
quality claim for them.

## Scope still open

This repair does not complete the whole-book read, semantic reconciliation of
all five source inventories, local and built links, figure viewing, published
rendering or owner Read 2. Those remain in the enclosing k134 brief and its live
proof child. No Linux, real-harness or novice-comprehension result is inferred.
Historical `docs/evidence/ravel-book-k311/` and the ancestor `out.err` are unchanged.
