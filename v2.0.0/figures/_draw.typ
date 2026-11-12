// The book's figure drawing: the only module that imports CeTZ and the only one
// that draws a stroke. A `_*.typ` module: it is imported, never rendered. A figure
// source builds a layout record with a layout module (_sequence.typ, ...) and
// hands it to `render` below, so a figure is an import and one call:
//
//   #import "_sequence.typ": *
//   #import "_draw.typ": render
//   #render(sequence([A title], parts, rows, key))
//
// `render` sets the page and font up, runs the layout, attaches the record to the
// page as `metadata(record) <layout>` (invisible) and draws it. `typst eval
// 'query(<layout>).first().value' --in FIG.typ` prints the record as JSON.
// scripts/figures.sh refuses any other module that imports CeTZ.
//
// ---- the layout record ------------------------------------------------------
// Plain data only (strings, numbers, booleans, arrays, dictionaries), so it
// prints as JSON. Geometry is y-down canvas units: 1 unit is 1pt, the origin is
// the canvas's top-left corner. The record has NO style: no fill, stroke weight,
// font size or roughness. Colours and weights enter only as roles, which this
// file looks up in _style.typ, so a restyle cannot change a record.
//
//   (
//     w: 480, h: 220,          // the layout's own extent, informational
//     elements: (...),         // drawn in order, so later ones sit on top
//   )
//
// Every element has an `id` (unique in the record), a `shape` and a `role`.
//
//   shape      role is one of          other fields
//   ---------  ----------------------  --------------------------------------
//   "line"     relation, result,       pts: ((x, y), ...) two or more points;
//              lifeline, rule, fold,   dashed: bool; arrow: bool (the shared
//              end, boundary           arrowhead on the last point)
//   "polygon"  note, control           pts: ((x, y), ...) closed; filled
//   "box"      ravel, external, owner, x0, y0, x1, y1
//              temporary, note, key,
//              divider, phase, outcome,
//              bar
//   "cylinder" a kinds role            x0, y0, x1, y1; cap: vertical radius
//                                      of the elliptical ends
//   "circle"   a kinds role            x, y: the centre; r: the radius; filled
//   "text"     a kinds role (colours   text: string, "\n" breaks a line;
//              the text; result is     face: title | head | name | qual | body;
//              green) or "label"
//              (plain ink)             x, y: the point; anchor: center | south
//                                      | west (the side of the text on the
//                                      point; east is also accepted);
//                                      align: center | left;
//                                      wrapped: bool, w: the wrapping width
//                                      if wrapped, else the text's own width;
//                                      h: wrapped height; w and h are measured
//                                      in the pinned font
//
// A worked example, a box with a label and an arrow to a second box:
//
//   (w: 230, h: 40, elements: (
//     (id: "a", shape: "box", role: "ravel", x0: 0, y0: 0, x1: 80, y1: 40),
//     (id: "a-label", shape: "text", role: "ravel", face: "head",
//      text: "CLI", x: 40, y: 20, anchor: "center", align: "center", wrapped: false, w: 27.5, h: 19),
//     (id: "go", shape: "line", role: "relation", pts: ((80, 20), (150, 20)),
//      dashed: false, arrow: true),
//     (id: "b", shape: "box", role: "external", x0: 150, y0: 0, x1: 230, y1: 40),
//   ))
//
// _layout.typ has a constructor for each shape; layout modules use those.
//
// ---- style ------------------------------------------------------------------
// The shipped style uses exact geometry. The owner retired the sketch style.
// Palette, fonts and stroke weights come from _style.typ. Layout records do not
// change with these drawing choices. An explicit --input style=crisp is optional.
#import "@preview/cetz:0.4.2"
#import "_style.typ": *

#let style = sys.inputs.at("style", default: "crisp")
#assert(style == "crisp", message: "--input style must be crisp, not " + style)

// Corner radius by box role; any other box is 3pt.
#let radii = (key: 6pt, divider: 0pt, phase: 12pt, outcome: 12pt, bar: 0pt)

// ---- drawing primitives -----------------------------------------------------
// Coordinates are (x, y) pairs written y-down. Colours are `pal` values.

// The CeTZ y-down flip.
#let flip(pt) = (pt.at(0), -pt.at(1))

// The one arrowhead every relation, message and reply shares.
#let head(color) = (end: (symbol: "stealth", fill: color, stroke: color, length: 9pt, width: 7pt))

#let stroke-of(color, weight, dash: none) = (paint: color, thickness: weight, dash: dash)

// A canvas of 1pt units; `body` is a block of `draw-*` calls.
#let canvas(body) = cetz.canvas(length: 1pt, body)

// Text placed at a point (the text itself is not stroked). `anchor` names which
// side of the text sits on the point, as CeTZ does.
#let draw-text(x, y, body, anchor: none) = {
  import cetz.draw: content
  if anchor == none { content(flip((x, y)), body) } else { content(flip((x, y)), anchor: anchor, body) }
}

// A straight line, or a polyline through more than two points; `arrow` puts the
// shared arrowhead, in the line's colour, on the last point.
#let draw-line(..pts, color: pal.rel, weight: w-main, dash: none, arrow: false) = {
  import cetz.draw: line
  let at = pts.pos().map(flip)
  let st = stroke-of(color, weight, dash: dash)
  if arrow { line(..at, stroke: st, mark: head(color)) } else { line(..at, stroke: st) }
}

// A closed filled polygon through the points, outlined.
#let draw-poly(..pts, fill: pal.pale, color: pal.slate, weight: w-main) = {
  import cetz.draw: line
  line(..pts.pos().map(flip), close: true, fill: fill, stroke: stroke-of(color, weight))
}

// A rounded box in a role: the role's fill, outlined in the role's line colour.
#let draw-box(x0, y0, x1, y1, k, radius: 3pt, weight: w-main) = {
  import cetz.draw: rect
  rect(flip((x0, y0)), flip((x1, y1)), fill: k.fill, stroke: stroke-of(k.line, weight), radius: radius)
}

// A filled circle (the initial marker): the role's fill, outlined in its line colour.
#let draw-circle(cx, cy, rad, k, weight: w-main) = {
  import cetz.draw: circle
  circle(flip((cx, cy)), radius: rad, fill: k.fill, stroke: stroke-of(k.line, weight))
}

// The shared cylinder: one primitive for sequence heads and container views.
#let draw-cylinder(x0, y0, x1, y1, k, ry: 9, weight: w-main) = {
  import cetz.draw: circle, line, rect
  let cx = (x0 + x1) / 2
  let rx = (x1 - x0) / 2
  let st = stroke-of(k.line, weight)
  circle(flip((cx, y1 - ry)), radius: (rx, ry), fill: k.fill, stroke: st)
  rect(flip((x0, y0 + ry)), flip((x1, y1 - ry)), fill: k.fill, stroke: none)
  line(flip((x0, y0 + ry)), flip((x0, y1 - ry)), stroke: st)
  line(flip((x1, y0 + ry)), flip((x1, y1 - ry)), stroke: st)
  circle(flip((cx, y0 + ry)), radius: (rx, ry), fill: k.fill, stroke: st)
}

// ---- record to strokes ------------------------------------------------------

#let role-of(table, el) = {
  assert(el.role in table, message: "element " + el.id + ": role " + el.role + " is not one of " + table.keys().join(", ") + " for a " + el.shape)
  table.at(el.role)
}

#let draw-element(el) = {
  if el.shape == "line" {
    let s = role-of(strokes, el)
    // A boundary's dashes are longer than a lifeline's, which CeTZ's preset draws as dots at its weight.
    let dash = if not el.dashed { none } else if el.role == "boundary" { (4pt, 4pt) } else { "dashed" }
    draw-line(..el.pts, color: s.color, weight: s.weight, dash: dash, arrow: el.arrow)
  } else if el.shape == "polygon" {
    let k = role-of(kinds, el)
    draw-poly(..el.pts, fill: k.fill, color: k.line)
  } else if el.shape == "box" {
    draw-box(el.x0, el.y0, el.x1, el.y1, role-of(kinds, el), radius: radii.at(el.role, default: 3pt))
  } else if el.shape == "cylinder" {
    draw-cylinder(el.x0, el.y0, el.x1, el.y1, role-of(kinds, el), ry: el.cap)
  } else if el.shape == "circle" {
    draw-circle(el.x, el.y, el.r, role-of(kinds, el))
  } else if el.shape == "text" {
    let args = faces.at(el.face)
    if el.role != "label" { args += (fill: role-of(kinds, el).text) }
    let block = if el.align == "left" { ltext } else { ctext }
    draw-text(el.x, el.y, block(label(el.text), w: if el.wrapped { el.w } else { auto }, ..args), anchor: if el.anchor == "center" { none } else { el.anchor })
  } else {
    panic("element " + el.id + ": unknown shape " + el.shape)
  }
}

// The one entry point a figure source calls. `layout` is a record, or a deferred
// layout (what `sequence` returns) that is run here, under the page's font, so
// the text it measures is the text that is drawn.
#let render(layout) = {
  show: figure-page
  context {
    let rec = if type(layout) == function { layout() } else { layout }
    [#metadata(rec) <layout>]
    canvas({
      for el in rec.elements { draw-element(el) }
    })
  }
}
