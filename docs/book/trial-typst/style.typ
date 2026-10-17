// Shared book figure style (TRIAL, k107). One module owns fonts, scale,
// strokes, palette and the shared primitives (arrowhead, cylinder, box).
// Units: 1 canvas unit = 1pt = 1 SVG user unit; sizes match figures.md.
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

// Semantic roles: fill / text / border.
#let kinds = (
  ravel: (fill: pal.blue, text: pal.white, line: pal.navy),
  external: (fill: pal.grey, text: pal.ink, line: pal.slate),
  owner: (fill: pal.owner, text: pal.white, line: pal.owner-line),
  temporary: (fill: pal.gold, text: pal.brown, line: pal.ochre),
  note: (fill: pal.pale, text: pal.ink, line: pal.slate),
)

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

// y-down helper: all figure geometry is written y-down, CeTZ is y-up.
#let p(x, y) = (x, -y)

// Intrinsic-size centred text block; `w` fixes the wrapping width.
#let len(w) = if type(w) == int or type(w) == float { w * 1pt } else { w }
#let ctext(body, w: auto, ..args) = box(width: len(w), align(center, text(..args, body)))
#let ltext(body, w: auto, ..args) = box(width: len(w), align(left, text(..args, body)))
#let fl(x) = x / 1pt
#let size-of(body, w: auto) = {
  let m = measure(box(width: len(w), body))
  (w: fl(m.width), h: fl(m.height))
}

// The one arrowhead every relation, message and reply shares.
#let head(color) = (end: (symbol: "stealth", fill: color, stroke: color, length: 9pt, width: 7pt))

// A solid rounded box in a semantic role.
#let role-box(x0, y0, x1, y1, k, radius: 3pt) = {
  import cetz.draw: *
  rect(p(x0, y0), p(x1, y1), fill: k.fill, stroke: (paint: k.line, thickness: w-main), radius: radius)
}

// The shared cylinder: one primitive for sequence heads and container views.
#let cylinder(x0, y0, x1, y1, k, ry: 9) = {
  import cetz.draw: *
  let cx = (x0 + x1) / 2
  let rx = (x1 - x0) / 2
  let st = (paint: k.line, thickness: w-main)
  circle(p(cx, y1 - ry), radius: (rx, ry), fill: k.fill, stroke: st)
  rect(p(x0, y0 + ry), p(x1, y1 - ry), fill: k.fill, stroke: none)
  line(p(x0, y0 + ry), p(x0, y1 - ry), stroke: st)
  line(p(x1, y0 + ry), p(x1, y1 - ry), stroke: st)
  circle(p(cx, y0 + ry), radius: (rx, ry), fill: k.fill, stroke: st)
}

// Pale key/legend box listing semantic lines.
#let legend-block(lines, cx, y0) = {
  import cetz.draw: *
  let body = ctext(lines.map(l => [#l]).join(linebreak()))
  let s = size-of(body)
  let w = s.w + 28
  let h = s.h + 20
  rect(p(cx - w / 2, y0), p(cx + w / 2, y0 + h), fill: pal.pale,
    stroke: (paint: pal.slate, thickness: w-main), radius: 6pt)
  content(p(cx, y0 + h / 2), body)
}

#let legend-size(lines) = {
  let s = size-of(ctext(lines.map(l => [#l]).join(linebreak())))
  (w: s.w + 28, h: s.h + 20)
}
