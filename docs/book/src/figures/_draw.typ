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
//              end                     arrowhead on the last point)
//   "polygon"  note (a kinds role)     pts: ((x, y), ...) closed; filled
//   "box"      ravel, external, owner, x0, y0, x1, y1
//              temporary, note, key,
//              divider
//   "cylinder" a kinds role            x0, y0, x1, y1; cap: vertical radius
//                                      of the elliptical ends
//   "text"     a kinds role (colours   text: string, "\n" breaks a line;
//              the text) or "label"    face: title | head | body;
//              (plain ink)             x, y: the point; anchor: center | south
//                                      | west (the side of the text on the
//                                      point); align: center | left;
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
// ---- styles -----------------------------------------------------------------
// `sketch` (the default) wobbles every outline, line, arrow shaft and cylinder.
// Roughness 1, 2 and 3 move points by up to 0.6, 1.2 and 1.8pt, and the shipped
// default is 2. `crisp` is roughness 0 and draws the
// exact geometry. Select with Typst inputs; a bare `typst compile` gives the
// shipped look:
//
//   --input style=crisp          roughness 0
//   --input style=sketch         the default
//   --input roughness=N          sketch at roughness N (0 is crisp)
//   --input figure=NAME          seeds the wobble; scripts/figures.sh passes the
//                                figure's file name
//
// The wobble is a pure function of the figure name and of each element's own
// geometry, never a clock or random source, so two renders are byte-identical.
// Text is never moved: it is laid out from measured metrics. Elements whose role
// is in `crisp-roles` are never sketched: a key is a legend, not part of the
// drawing.
#import "@preview/cetz:0.4.2"
#import "_style.typ": *

#let style = sys.inputs.at("style", default: "sketch")
#assert(style in ("crisp", "sketch"), message: "--input style must be crisp or sketch, not " + style)
#let roughness = if style == "crisp" { 0 } else { int(sys.inputs.at("roughness", default: "2")) }
#let figure-name = sys.inputs.at("figure", default: "")
#let seed = figure-name.codepoints().fold(7, (a, c) => calc.rem(a * 31 + str.to-unicode(c), 2147483629))
#let amp-unit = 0.6
#let crisp-roles = ("key",)

// Corner radius by box role; any other box is 3pt.
#let radii = (key: 6pt, divider: 0pt)

// Integer LCG scramble of (seed, a, b, k) to a number in [-1, 1].
#let noise(a, b, k) = {
  let m = 2147483629
  let h = calc.rem(seed + int(calc.round(a * 4)) * 73856 + int(calc.round(b * 4)) * 19349 + k * 83492, m)
  for _ in range(3) { h = calc.rem(h * 48271 + 11, m) }
  calc.rem(h, 20001) / 10000 - 1
}

// Points along a wobbly path from `p` to `q` (y-down) at roughness `r`. Ends move
// by half the amplitude, interior points sideways by the full amplitude.
#let wobble(p, q, k, r) = {
  let dx = q.at(0) - p.at(0)
  let dy = q.at(1) - p.at(1)
  let l = calc.sqrt(dx * dx + dy * dy)
  if l == 0 { return (p, q) }
  let amp = calc.min(r * amp-unit, l * 0.05)
  let nx = -dy / l
  let ny = dx / l
  let n = int(calc.max(2, calc.round(l / 45)))
  let out = ()
  for i in range(n + 1) {
    let t = i / n
    let a = if i == 0 or i == n { amp / 2 } else { amp }
    let o = noise(p.at(0) + dx * t, p.at(1) + dy * t, k + i) * a
    let along = if i == 0 or i == n { noise(p.at(1), q.at(0), k + i + 50) * amp / 2 } else { 0 }
    out.push((p.at(0) + dx * t + nx * o + dx / l * along, p.at(1) + dy * t + ny * o + dy / l * along))
  }
  out
}

// A wobbly polyline through y-down points.
#let wobble-path(pts, k, r) = {
  let out = ()
  for i in range(pts.len() - 1) {
    let seg = wobble(pts.at(i), pts.at(i + 1), k + i * 7, r)
    out += if i == 0 { seg } else { seg.slice(1) }
  }
  out
}

// A wobbly closed ellipse as a polygon, y-down.
#let wobble-ellipse(cx, cy, rx, ry, k, r) = range(20).map(i => {
  let t = i / 20 * 360deg
  let f = 1 + noise(cx, cy, k + i) * r * amp-unit * 0.3 / calc.min(rx, ry)
  (cx + rx * f * calc.cos(t), cy + ry * f * calc.sin(t))
})

// ---- drawing primitives ---------------------------------------------------
// Coordinates are (x, y) pairs written y-down. Colours are `pal` values. `r` is
// the roughness the element is drawn at.

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
#let draw-line(..pts, color: pal.rel, weight: w-main, dash: none, arrow: false, r: roughness) = {
  import cetz.draw: line
  let raw = pts.pos()
  let at = (if r == 0 { raw } else { wobble-path(raw, 1, r) }).map(flip)
  let st = stroke-of(color, weight, dash: dash)
  if arrow { line(..at, stroke: st, mark: head(color)) } else { line(..at, stroke: st) }
}

// A closed filled polygon through the points, outlined.
#let draw-poly(..pts, fill: pal.pale, color: pal.slate, weight: w-main, r: roughness) = {
  import cetz.draw: line
  if r == 0 {
    line(..pts.pos().map(flip), close: true, fill: fill, stroke: stroke-of(color, weight))
  } else {
    let raw = pts.pos()
    line(..raw.map(flip), close: true, fill: fill, stroke: none)
    draw-line(..raw, raw.first(), color: color, weight: weight, r: r)
  }
}

// A rounded box in a role: the role's fill, outlined in the role's line colour.
#let draw-box(x0, y0, x1, y1, k, radius: 3pt, weight: w-main, r: roughness) = {
  import cetz.draw: rect
  if r == 0 {
    rect(flip((x0, y0)), flip((x1, y1)), fill: k.fill, stroke: stroke-of(k.line, weight), radius: radius)
  } else {
    rect(flip((x0, y0)), flip((x1, y1)), fill: k.fill, stroke: none, radius: radius)
    draw-line((x0, y0), (x1, y0), (x1, y1), (x0, y1), (x0, y0), color: k.line, weight: weight, r: r)
  }
}

// The shared cylinder: one primitive for sequence heads and container views.
#let draw-cylinder(x0, y0, x1, y1, k, ry: 9, weight: w-main, r: roughness) = {
  import cetz.draw: circle, line, rect
  let cx = (x0 + x1) / 2
  let rx = (x1 - x0) / 2
  let st = stroke-of(k.line, weight)
  if r != 0 {
    let ell(cy, k0) = line(..wobble-ellipse(cx, cy, rx, ry, k0, r).map(flip), close: true, fill: k.fill, stroke: st)
    ell(y1 - ry, 100)
    rect(flip((x0, y0 + ry)), flip((x1, y1 - ry)), fill: k.fill, stroke: none)
    draw-line((x0, y0 + ry), (x0, y1 - ry), color: k.line, weight: weight, r: r)
    draw-line((x1, y0 + ry), (x1, y1 - ry), color: k.line, weight: weight, r: r)
    ell(y0 + ry, 200)
  } else {
    circle(flip((cx, y1 - ry)), radius: (rx, ry), fill: k.fill, stroke: st)
    rect(flip((x0, y0 + ry)), flip((x1, y1 - ry)), fill: k.fill, stroke: none)
    line(flip((x0, y0 + ry)), flip((x0, y1 - ry)), stroke: st)
    line(flip((x1, y0 + ry)), flip((x1, y1 - ry)), stroke: st)
    circle(flip((cx, y0 + ry)), radius: (rx, ry), fill: k.fill, stroke: st)
  }
}

// ---- record to strokes ------------------------------------------------------

#let role-of(table, el) = {
  assert(el.role in table, message: "element " + el.id + ": role " + el.role + " is not one of " + table.keys().join(", ") + " for a " + el.shape)
  table.at(el.role)
}

#let draw-element(el) = {
  let r = if el.role in crisp-roles { 0 } else { roughness }
  if el.shape == "line" {
    let s = role-of(strokes, el)
    draw-line(..el.pts, color: s.color, weight: s.weight, dash: if el.dashed { "dashed" } else { none }, arrow: el.arrow, r: r)
  } else if el.shape == "polygon" {
    let k = role-of(kinds, el)
    draw-poly(..el.pts, fill: k.fill, color: k.line, r: r)
  } else if el.shape == "box" {
    draw-box(el.x0, el.y0, el.x1, el.y1, role-of(kinds, el), radius: radii.at(el.role, default: 3pt), r: r)
  } else if el.shape == "cylinder" {
    draw-cylinder(el.x0, el.y0, el.x1, el.y1, role-of(kinds, el), ry: el.cap, r: r)
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
