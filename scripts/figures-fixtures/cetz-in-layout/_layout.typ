// Refused fixture for scripts/figures.test.sh: a layout module that draws with
// CeTZ itself. Only _draw.typ may import CeTZ, so adding this file to a figures
// directory must fail the check. It is not part of scripts/figures-fixtures/figures,
// whose files must all pass.
#import "@preview/cetz:0.4.2"

#let ruler(x0, x1) = cetz.canvas(length: 1pt, {
  import cetz.draw: line
  line((x0, 0), (x1, 0))
})
