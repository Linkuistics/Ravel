// Fixture module for scripts/figures.test.sh: the smallest stand-in for the
// book's _draw.typ, the one module that may import CeTZ. It is imported by
// layers.typ and is never rendered. Like the book's, `render` takes a layout
// record (plain data, no style), attaches it as `metadata(record) <layout>` and
// draws it; every colour, weight and face lives here.
#import "@preview/cetz:0.4.2"

#let setup(body) = {
  set page(width: auto, height: auto, margin: 6pt, fill: white)
  set text(font: "DejaVu Sans", size: 14pt, fill: rgb("#172B3A"))
  body
}

// A labelled box whose top-left corner is (x0, y0), y-down.
#let framed(x0, y0, x1, y1, label) = {
  import cetz.draw: content, rect
  rect((x0, -y0), (x1, -y1), fill: rgb("#E8EDF2"), stroke: (paint: rgb("#64748B"), thickness: 1.5pt), radius: 3pt)
  content(((x0 + x1) / 2, -(y0 + y1) / 2), label)
}

#let arrow(pts) = {
  import cetz.draw: line
  line((pts.at(0).at(0), -pts.at(0).at(1)), (pts.at(1).at(0), -pts.at(1).at(1)), stroke: (paint: rgb("#334155"), thickness: 1.5pt), mark: (end: "stealth", fill: rgb("#334155")))
}

#let render(record) = {
  show: setup
  [#metadata(record) <layout>]
  cetz.canvas(length: 1pt, {
    for el in record.elements {
      if el.shape == "box" { framed(el.x0, el.y0, el.x1, el.y1, el.text) } else { arrow(el.pts) }
    }
  })
}
