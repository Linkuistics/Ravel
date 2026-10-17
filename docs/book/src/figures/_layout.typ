// What a layout module needs to build a layout record. A `_*.typ` module: it is
// imported, never rendered. The record's shape is written at the top of
// _draw.typ; the constructors here are the one place that spells it, so every
// layout module (_sequence.typ and the ones after it) produces the same shape.
// Nothing here draws or imports CeTZ.
//
// Everything here runs inside `render` (see _draw.typ), because measuring text
// needs Typst's `context` and the page's font.
#import "_style.typ": *

// A label as a plain string. Content is accepted only where it is text, spaces,
// line breaks (`\`) and the apostrophe smartquote, so the record holds data and
// not Typst content. Write a double quote or any other mark straight into a
// string instead. Typst drops the spaces on either side of a line break in
// content and keeps them in a string, so they are dropped here to draw the same.
#let flat(c) = {
  if type(c) == str { return c }
  if type(c) != content { panic("a label is a string or text content, not " + repr(c)) }
  if c.has("text") { return c.text }
  if c.has("children") { return c.children.map(flat).join("") }
  if c.func() == linebreak { return "\n" }
  if c.func() == [ ].func() { return " " }
  if c.func() == smartquote and not c.double { return "’" }
  panic("a label may hold only text, spaces and line breaks, not " + repr(c))
}
#let plain(c) = flat(c).replace(regex(" *\n *"), "\n")

// Labels joined one per line, for a key.
#let lines-of(lines) = lines.map(plain).join("\n")

#let line-el(id, role, pts, dashed: false, arrow: false) = (
  id: id, shape: "line", role: role, pts: pts, dashed: dashed, arrow: arrow,
)
#let poly-el(id, role, pts) = (id: id, shape: "polygon", role: role, pts: pts)
#let box-el(id, role, x0, y0, x1, y1) = (id: id, shape: "box", role: role, x0: x0, y0: y0, x1: x1, y1: y1)
// `cap` is the vertical radius of the elliptical ends.
#let cylinder-el(id, role, x0, y0, x1, y1, cap: 9) = (
  id: id, shape: "cylinder", role: role, x0: x0, y0: y0, x1: x1, y1: y1, cap: cap,
)

// A filled circle of radius `r` centred on (x, y): the initial marker of a state view.
#let circle-el(id, role, x, y, r) = (id: id, shape: "circle", role: role, x: x, y: y, r: r)

// A label placed at (x, y); `anchor` names which side of the text sits on the
// point ("center", "south", "west", as CeTZ does). `w` is the wrapping width, or
// `auto` for the text's own width; the record always holds the measured width and
// the wrapped height, in the pinned font, and says whether the label was wrapped
// at a fixed width (CeTZ sizes a fixed-width box a little taller than an
// auto-width one, so the drawing must know which it was). `role` colours the text: the role
// of the box it sits on, or "label" for plain ink.
#let text-el(id, body, x, y, role: "label", face: "body", anchor: "center", align: "center", w: auto) = {
  let s = plain(body)
  let m = size-of(ctext(label(s), w: w, ..faces.at(face)))
  (
    id: id, shape: "text", role: role, face: face, text: s, x: x, y: y, anchor: anchor, align: align,
    wrapped: w != auto, w: m.w, h: m.h,
  )
}

// The label's wrapping width for a part of a box: its own width when that fits
// the box, else the width that wraps it. Measured, so a one-line part is never
// drawn in a fixed-width box.
#let fit(body, face, limit) = {
  let s = size-of(ctext(label(plain(body)), ..faces.at(face)))
  if s.w > limit { limit } else { auto }
}

// The pale key box listing semantic lines. Its size is the text's plus a margin.
#let key-size(lines) = {
  let s = size-of(ctext(label(lines-of(lines))))
  (w: s.w + 28, h: s.h + 20)
}

// The key box centred on `cx` with its top at `y0`, then its text.
#let key-elements(lines, cx, y0, id: "key") = {
  let g = key-size(lines)
  (
    box-el(id, "key", cx - g.w / 2, y0, cx + g.w / 2, y0 + g.h),
    text-el(id + "-text", lines-of(lines), cx, y0 + g.h / 2),
  )
}

// A finished record: the layout's own extent and its elements, drawn in order.
#let record(w, h, elements) = (w: w, h: h, elements: elements)

// ---- orthogonal routes (shared by _c4.typ and _state.typ) --------------------

#let horizontal-side(s) = s in ("left", "right")

// The point on side `s` of box `b`, `off` along it from the middle.
#let port(b, s, off) = {
  if s == "top" { (b.cx + off, b.y0) }
  else if s == "bottom" { (b.cx + off, b.y1) }
  else if s == "left" { (b.x0, b.cy + off) }
  else if s == "right" { (b.x1, b.cy + off) }
  else { panic("a side is top, bottom, left or right, not " + repr(s)) }
}

#let side-and-offset(spec) = if type(spec) == array { spec } else { (spec, 0) }

// The sides that face each other: across the gap the boxes have, vertically when
// their columns overlap, otherwise horizontally.
#let facing(a, b) = {
  if a.x0 < b.x1 and b.x0 < a.x1 {
    if b.cy > a.cy { ("bottom", "top") } else { ("top", "bottom") }
  } else {
    if b.cx > a.cx { ("right", "left") } else { ("left", "right") }
  }
}

// The orthogonal polyline from port `p0` (leaving by `s0`) to `p1` (entering by `s1`).
#let route(p0, s0, p1, s1, jog) = {
  let same = (v, w) => calc.abs(v - w) < 0.5
  if horizontal-side(s0) == horizontal-side(s1) {
    if horizontal-side(s0) {
      if same(p0.at(1), p1.at(1)) { (p0, p1) } else {
        let xm = if jog != none { p0.at(0) + jog } else { (p0.at(0) + p1.at(0)) / 2 }
        (p0, (xm, p0.at(1)), (xm, p1.at(1)), p1)
      }
    } else {
      if same(p0.at(0), p1.at(0)) { (p0, p1) } else {
        let ym = if jog != none { p0.at(1) + jog } else { (p0.at(1) + p1.at(1)) / 2 }
        (p0, (p0.at(0), ym), (p1.at(0), ym), p1)
      }
    }
  } else if horizontal-side(s0) {
    (p0, (p1.at(0), p0.at(1)), p1)
  } else {
    (p0, (p0.at(0), p1.at(1)), p1)
  }
}
