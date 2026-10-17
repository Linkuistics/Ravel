// Fixture module for scripts/figures.test.sh: the smallest stand-in for the
// book's _draw.typ, the one module that may import CeTZ. It is imported by
// layers.typ and is never rendered.
#import "@preview/cetz:0.4.2"

#let setup(body) = {
  set page(width: auto, height: auto, margin: 6pt, fill: white)
  set text(font: "DejaVu Sans", size: 14pt, fill: rgb("#172B3A"))
  body
}

#let canvas(body) = cetz.canvas(length: 1pt, body)

// A labelled box whose top-left corner is (x, y), y-down.
#let framed(x, y, w, h, label) = {
  import cetz.draw: content, rect
  rect((x, -y), (x + w, -(y + h)), fill: rgb("#E8EDF2"), stroke: (paint: rgb("#64748B"), thickness: 1.5pt), radius: 3pt)
  content((x + w / 2, -(y + h / 2)), label)
}

#let arrow(x0, y0, x1, y1) = {
  import cetz.draw: line
  line((x0, -y0), (x1, -y1), stroke: (paint: rgb("#334155"), thickness: 1.5pt), mark: (end: "stealth", fill: rgb("#334155")))
}
