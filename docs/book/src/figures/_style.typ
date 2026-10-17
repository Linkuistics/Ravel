// The book's shared figure style. A `_*.typ` module: it is imported, never
// rendered, and has no SVG. Fonts, scale, palette roles, strokes and the drawing
// primitives all live here, so every figure matches and a change to how the
// book draws happens in this one file. Layout and mechanics: docs/book/figures.md.
//
// Units: one canvas unit is 1pt, which is one SVG user unit. Geometry is written
// y-down (top-left origin); the primitives flip it for CeTZ, which is y-up.
//
// Every stroke and outline is drawn by one of the `draw-*` primitives below
// (line, arrow, poly, box, cylinder) and nothing else in the book calls CeTZ. A
// figure source or another module places geometry and text through them. That is
// what lets a later leaf change how all lines are drawn without touching a
// figure, and a check in scripts/figures.sh refuses a figure source that
// imports a package itself.
#import "@preview/cetz:0.4.2"

#let font = "DejaVu Sans"
#let mono = "DejaVu Sans Mono"

#let pal = (
  ink: rgb("#172B3A"),
  blue: rgb("#1E4F78"),
  navy: rgb("#163A59"),
  grey: rgb("#E8EDF2"),
  slate: rgb("#64748B"),
  owner: rgb("#475569"),
  owner-line: rgb("#334155"),
  gold: rgb("#FFF1CF"),
  brown: rgb("#3D321A"),
  ochre: rgb("#8A6B26"),
  pale: rgb("#F8FAFC"),
  rel: rgb("#334155"),
  green: rgb("#135E50"),
  white: rgb("#FFFFFF"),
)

// Semantic roles: fill / text / border. `note` is also the key box, and
// `external` is also the divider label.
#let kinds = (
  ravel: (fill: pal.blue, text: pal.white, line: pal.navy),
  external: (fill: pal.grey, text: pal.ink, line: pal.slate),
  owner: (fill: pal.owner, text: pal.white, line: pal.owner-line),
  temporary: (fill: pal.gold, text: pal.brown, line: pal.ochre),
  note: (fill: pal.pale, text: pal.ink, line: pal.slate),
)

// Text scale. Stereotype and technology lines («container», [Rust]) are the
// `qual` size and upright: DejaVu Oblique is not a pinned face, and Typst draws
// a missing italic upright, so the size step and the «» and [] marks carry the
// distinction that italics did in the PlantUML output.
#let sz = (body: 14pt, head: 16pt, qual: 12pt)
#let w-main = 1.5pt // entities, boundaries, notes, keys, arrow shafts
#let w-life = 0.5pt // sequence lifelines
#let w-rule = 0.75pt // paired separator rules

// Document setup for one figure: auto-sized white page, pinned font.
#let figure-page(body) = {
  set page(width: auto, height: auto, margin: 6pt, fill: pal.white)
  set text(font: font, size: sz.body, fill: pal.ink, lang: "en")
  set par(leading: 0.55em, spacing: 0pt)
  body
}

// Intrinsic-size text blocks; `w` fixes the wrapping width.
#let len(w) = if type(w) == int or type(w) == float { w * 1pt } else { w }
#let ctext(body, w: auto, ..args) = box(width: len(w), align(center, text(..args, body)))
#let ltext(body, w: auto, ..args) = box(width: len(w), align(left, text(..args, body)))
#let fl(x) = x / 1pt
#let size-of(body, w: auto) = {
  let m = measure(box(width: len(w), body))
  (w: fl(m.width), h: fl(m.height))
}

// ---- hand-drawn switch ------------------------------------------------------
// `roughness` 0 draws the crisp figure exactly as before. 1, 2 and 3 wobble every
// outline, line, arrow shaft and cylinder by up to 0.6, 1.2 and 1.8pt. The wobble
// is a pure function of the figure name (`--input figure=NAME`) and of the
// element's own geometry, never a clock or random source, so two renders are
// byte-identical. Text is never moved: it is laid out from measured metrics.
#let roughness = int(sys.inputs.at("roughness", default: "0"))
#let figure-name = sys.inputs.at("figure", default: "")
#let seed = figure-name.codepoints().fold(7, (a, c) => calc.rem(a * 31 + str.to-unicode(c), 2147483629))
#let amp-unit = 0.6

// Integer LCG scramble of (seed, a, b, k) to a number in [-1, 1].
#let noise(a, b, k) = {
  let m = 2147483629
  let h = calc.rem(seed + int(calc.round(a * 4)) * 73856 + int(calc.round(b * 4)) * 19349 + k * 83492, m)
  for _ in range(3) { h = calc.rem(h * 48271 + 11, m) }
  calc.rem(h, 20001) / 10000 - 1
}

// Points along a wobbly path from `p` to `q` (y-down). Ends move by half the
// amplitude, interior points sideways by the full amplitude.
#let wobble(p, q, k) = {
  let dx = q.at(0) - p.at(0)
  let dy = q.at(1) - p.at(1)
  let l = calc.sqrt(dx * dx + dy * dy)
  if l == 0 { return (p, q) }
  let amp = calc.min(roughness * amp-unit, l * 0.05)
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
#let wobble-path(pts, k) = {
  let out = ()
  for i in range(pts.len() - 1) {
    let seg = wobble(pts.at(i), pts.at(i + 1), k + i * 7)
    out += if i == 0 { seg } else { seg.slice(1) }
  }
  out
}

// A wobbly closed ellipse as a polygon, y-down.
#let wobble-ellipse(cx, cy, rx, ry, k) = range(20).map(i => {
  let t = i / 20 * 360deg
  let r = 1 + noise(cx, cy, k + i) * roughness * amp-unit * 0.3 / calc.min(rx, ry)
  (cx + rx * r * calc.cos(t), cy + ry * r * calc.sin(t))
})

// ---- drawing primitives ---------------------------------------------------
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
  let raw = pts.pos()
  let at = (if roughness == 0 { raw } else { wobble-path(raw, 1) }).map(flip)
  let st = stroke-of(color, weight, dash: dash)
  if arrow { line(..at, stroke: st, mark: head(color)) } else { line(..at, stroke: st) }
}

// A relation, message or reply: a line that ends in the shared arrowhead.
#let draw-arrow(..pts, color: pal.rel, dashed: false) = draw-line(..pts, color: color, dash: if dashed { "dashed" } else { none }, arrow: true)

// A closed filled polygon through the points, outlined.
#let draw-poly(..pts, fill: pal.pale, color: pal.slate, weight: w-main) = {
  import cetz.draw: line
  if roughness == 0 {
    line(..pts.pos().map(flip), close: true, fill: fill, stroke: stroke-of(color, weight))
  } else {
    let raw = pts.pos()
    line(..raw.map(flip), close: true, fill: fill, stroke: none)
    draw-line(..raw, raw.first(), color: color, weight: weight)
  }
}

// A rounded box in a role: the role's fill, outlined in the role's line colour.
#let draw-box(x0, y0, x1, y1, k, radius: 3pt, weight: w-main) = {
  import cetz.draw: rect
  if roughness == 0 {
    rect(flip((x0, y0)), flip((x1, y1)), fill: k.fill, stroke: stroke-of(k.line, weight), radius: radius)
  } else {
    rect(flip((x0, y0)), flip((x1, y1)), fill: k.fill, stroke: none, radius: radius)
    draw-line((x0, y0), (x1, y0), (x1, y1), (x0, y1), (x0, y0), color: k.line, weight: weight)
  }
}

// The shared cylinder: one primitive for sequence heads and container views.
#let draw-cylinder(x0, y0, x1, y1, k, ry: 9, weight: w-main) = {
  import cetz.draw: circle, line, rect
  let cx = (x0 + x1) / 2
  let rx = (x1 - x0) / 2
  let st = stroke-of(k.line, weight)
  if roughness != 0 {
    let ell(cy, k0) = line(..wobble-ellipse(cx, cy, rx, ry, k0).map(flip), close: true, fill: k.fill, stroke: st)
    ell(y1 - ry, 100)
    rect(flip((x0, y0 + ry)), flip((x1, y1 - ry)), fill: k.fill, stroke: none)
    draw-line((x0, y0 + ry), (x0, y1 - ry), color: k.line, weight: weight)
    draw-line((x1, y0 + ry), (x1, y1 - ry), color: k.line, weight: weight)
    ell(y0 + ry, 200)
  } else {
    circle(flip((cx, y1 - ry)), radius: (rx, ry), fill: k.fill, stroke: st)
    rect(flip((x0, y0 + ry)), flip((x1, y1 - ry)), fill: k.fill, stroke: none)
    line(flip((x0, y0 + ry)), flip((x0, y1 - ry)), stroke: st)
    line(flip((x1, y0 + ry)), flip((x1, y1 - ry)), stroke: st)
    circle(flip((cx, y0 + ry)), radius: (rx, ry), fill: k.fill, stroke: st)
  }
}

// ---- composite pieces built only from the primitives ----------------------

// The pale key box listing semantic lines, centred on `cx` with its top at `y0`.
#let legend-size(lines) = {
  let s = size-of(ctext(lines.map(l => [#l]).join(linebreak())))
  (w: s.w + 28, h: s.h + 20)
}

#let legend-block(lines, cx, y0) = {
  let body = ctext(lines.map(l => [#l]).join(linebreak()))
  let g = legend-size(lines)
  draw-box(cx - g.w / 2, y0, cx + g.w / 2, y0 + g.h, kinds.note, radius: 6pt)
  draw-text(cx, y0 + g.h / 2, body)
}
