// UML state-machine layout: rounded states in a role, an initial marker and
// transitions with a guard label and an optional colour. A `_*.typ` module:
// imported by a state figure, never rendered. It returns a layout record (shape:
// the header of _draw.typ) and draws nothing: `render` in _draw.typ does.
//
// Placement is by GRID CELL, and sizes are measured. A figure source gives each
// state a `col` and a `row`; the module measures the state's text in the pinned
// font, makes each column as wide as its widest state and each row as tall as its
// tallest, and puts `col-gap` and `row-gap` between them. A cell number may be
// fractional (`col: 2.5` is halfway between the centres of columns 2 and 3), which
// is how one state sits over a fan of others. Nothing is given a coordinate.
// A transition names two states and the side of each it leaves and enters by; the
// module routes an orthogonal polyline between the two ports (`route` in
// _layout.typ, shared with _c4.typ) and places the label beside one segment. What
// the module does not do is avoid collisions: a label or a line in the wrong place
// is moved in the figure source (a side, a `t`, a `dx`) and rendered again.
//
//   #import "_state.typ": *
//   #import "_draw.typ": render
//   #render(state-machine([Title], states, trans, key-lines, col-gap: 30, row-gap: 60))
//
// Roles: a state is `phase` (blue) or `outcome` (grey); a transition is "relation"
// or "result" (the green route, with its label).
#import "_layout.typ": *

// A state: `body` is its name, `\` between lines. `limit` is the widest line, in
// points, before it wraps; `min-w` widens the box past its text, which spreads the
// ports of a fan of transitions.
#let state(id, role, body, col: 0, row: 0, limit: 240, min-w: 0) = (
  kind: "state", id: id, role: role, body: body, col: col, row: row, limit: limit, min-w: min-w,
)

// The initial marker: a small filled circle in a cell.
#let initial(id, col: 0, row: 0) = (kind: "initial", id: id, col: col, row: row)

// A transition. `out` and `into` are each a side ("top", "bottom", "left",
// "right"), (side, offset) as in _c4.typ, or "align": the port sits level with
// the other state's centre, kept inside its own side, so a fan of transitions
// leaves a wide state one at a time with no offset written. Omitted, the sides
// face each other across the gap between the two boxes. `jog` sets the middle run
// of a Z or U route that many points from the start port.
//
// The label sits beside one segment, `seg` (default: the longest), `t` of the way
// along it; `side` is "above" or "below" a horizontal segment, "right" or "left" of
// a vertical one (default above, right); `dx` and `dy` nudge it and `w` wraps it.
// Its lines are centred unless `align` says "left".
#let tr(from, to, label: none, role: "relation", out: auto, into: auto, jog: none,
  seg: auto, t: 0.5, side: auto, dx: 0, dy: 0, w: auto, align: auto) = (
  from: from, to: to, label: label, role: role, out: out, into: into, jog: jog,
  seg: seg, t: t, side: side, dx: dx, dy: dy, w: w, align: align,
)

// ---- sizes ------------------------------------------------------------------

#let state-pad = (x: 22, y: 9)
#let dot-r = 9
#let port-margin = 14 // an aligned port keeps this far from a corner

// The centres of `n` cells in a line: each cell as wide as its widest member
// (`sizes`, one per cell, 0 for an empty one), with `gap` between neighbours
// (a number, or one per gap).
#let centres(sizes, gap) = {
  let out = ()
  let pos = 0
  for (i, s) in sizes.enumerate() {
    let g = if i == 0 { 0 } else if type(gap) == array { gap.at(i - 1) } else { gap }
    out.push(pos + g + s / 2)
    pos += g + s
  }
  out
}

// A (possibly fractional) cell index's coordinate, between the centres of the
// cells either side.
#let at-cell(cs, i) = {
  let lo = calc.floor(i)
  let hi = calc.min(lo + 1, cs.len() - 1)
  cs.at(lo) + (cs.at(hi) - cs.at(lo)) * (i - lo)
}

// The port of `a` that faces `b` for "align": level with b's centre, kept inside
// a's own side.
#let aligned(a, side, b) = {
  if side == "top" or side == "bottom" {
    let room = (a.x1 - a.x0) / 2 - port-margin
    calc.max(-room, calc.min(room, b.cx - a.cx))
  } else {
    let room = (a.y1 - a.y0) / 2 - port-margin
    calc.max(-room, calc.min(room, b.cy - a.cy))
  }
}

// ---- layout -------------------------------------------------------------------

// Returns a deferred layout: calling it, inside `render`'s context, gives the
// record. Pass it straight to `render`.
#let state-machine(title, states, trans, legend, col-gap: 40, row-gap: 50) = () => {
  // --- measured boxes -----------------------------------------------------
  let sizes = (:)
  let parts = (:)
  for s in states {
    if s.kind == "initial" {
      sizes.insert(s.id, (w: 2 * dot-r, h: 2 * dot-r))
    } else {
      let t = text-el(s.id + "-text", s.body, 0, 0, role: s.role, w: fit(s.body, "body", s.limit))
      parts.insert(s.id, t)
      sizes.insert(s.id, (w: calc.max(s.min-w, t.w + 2 * state-pad.x), h: t.h + 2 * state-pad.y))
    }
  }

  // --- grid: a whole cell sizes its column and its row ----------------------
  let ncols = calc.max(..states.map(s => calc.ceil(s.col))) + 1
  let nrows = calc.max(..states.map(s => calc.ceil(s.row))) + 1
  let colw = range(ncols).map(_ => 0)
  let rowh = range(nrows).map(_ => 0)
  for s in states {
    if calc.fract(s.col) == 0 { colw.at(int(s.col)) = calc.max(colw.at(int(s.col)), sizes.at(s.id).w) }
    if calc.fract(s.row) == 0 { rowh.at(int(s.row)) = calc.max(rowh.at(int(s.row)), sizes.at(s.id).h) }
  }
  let xs = centres(colw, col-gap)
  let ys = centres(rowh, row-gap)
  let geo = (:)
  for s in states {
    let g = sizes.at(s.id)
    let (cx, cy) = (at-cell(xs, s.col), at-cell(ys, s.row))
    geo.insert(s.id, (x0: cx - g.w / 2, y0: cy - g.h / 2, x1: cx + g.w / 2, y1: cy + g.h / 2, cx: cx, cy: cy))
  }

  // --- transitions: ports, routes, labels -----------------------------------
  let outs = ()
  let seen = (:)
  for r in trans {
    let a = geo.at(r.from)
    let b = geo.at(r.to)
    let face = facing(a, b)
    let spec = (r.out, r.into).zip(face).map(p => if p.at(0) == auto { p.at(1) } else { p.at(0) })
    let (s0, o0) = side-and-offset(spec.at(0))
    let (s1, o1) = side-and-offset(spec.at(1))
    let o0 = if o0 == "align" { aligned(a, s0, b) } else { o0 }
    let o1 = if o1 == "align" { aligned(b, s1, a) } else { o1 }
    let pts = route(port(a, s0, o0), s0, port(b, s1, o1), s1, r.jog)

    let base = r.from + "-" + r.to
    seen.insert(base, seen.at(base, default: 0) + 1)
    let id = "tr-" + base + if seen.at(base) > 1 { "-" + str(seen.at(base)) } else { "" }
    let els = (line-el(id, r.role, pts, arrow: true),)
    let box = none
    if r.label != none {
      // the segment the label sits beside
      let lens = range(pts.len() - 1).map(i => calc.abs(pts.at(i + 1).at(0) - pts.at(i).at(0)) + calc.abs(pts.at(i + 1).at(1) - pts.at(i).at(1)))
      let si = if r.seg != auto { r.seg } else { lens.position(l => l == calc.max(..lens)) }
      let (p, q) = (pts.at(si), pts.at(si + 1))
      let at = (p.at(0) + (q.at(0) - p.at(0)) * r.t, p.at(1) + (q.at(1) - p.at(1)) * r.t)
      let flat = calc.abs(q.at(1) - p.at(1)) < calc.abs(q.at(0) - p.at(0))
      let side = if r.side != auto { r.side } else if flat { "above" } else { "right" }
      assert((side in ("above", "below")) == flat, message: "transition " + base + ": side " + side + " does not fit its segment")
      let align = if r.align != auto { r.align } else { "center" }
      let color = if r.role == "result" { "result" } else { "label" }
      let l = text-el(id + "-label", r.label, 0, 0, role: color, align: align, w: r.w)
      let cx = at.at(0) + r.dx
      let cy = at.at(1) + r.dy
      if side == "above" { cy -= 4 + l.h / 2 }
      else if side == "below" { cy += 4 + l.h / 2 }
      else if side == "right" { cx += 8 + l.w / 2 }
      else { cx -= 8 + l.w / 2 }
      els.push((..l, x: cx, y: cy))
      box = (x0: cx - l.w / 2, x1: cx + l.w / 2, y0: cy - l.h / 2, y1: cy + l.h / 2)
    }
    outs.push((els: els, box: box))
  }

  // --- extents --------------------------------------------------------------
  let boxes = geo.values() + outs.filter(o => o.box != none).map(o => o.box)
  let x0 = calc.min(..boxes.map(b => b.x0))
  let x1 = calc.max(..boxes.map(b => b.x1))
  let y0 = calc.min(..boxes.map(b => b.y0))
  let y1 = calc.max(..boxes.map(b => b.y1))
  let tit = size-of(ctext(label(plain(title)), ..faces.title))
  let lg = key-size(legend)
  let W = calc.max(x1 - x0, lg.w + 12, tit.w + 12)
  let cx = (x0 + x1) / 2
  // the topmost element sits 14 below the title
  let oy = tit.h + 14 - y0

  // --- elements, in drawing order -------------------------------------------
  let els = (text-el("title", title, cx, tit.h / 2, face: "title"),)
  for s in states {
    let b = geo.at(s.id)
    if s.kind == "initial" {
      els.push(circle-el(s.id, "initial", b.cx, b.cy + oy, dot-r))
    } else {
      els.push(box-el(s.id, s.role, b.x0, b.y0 + oy, b.x1, b.y1 + oy))
      els.push((..parts.at(s.id), x: b.cx, y: b.cy + oy))
    }
  }
  for o in outs {
    for el in o.els {
      els.push(if el.shape == "line" { (..el, pts: el.pts.map(q => (q.at(0), q.at(1) + oy))) } else { (..el, y: el.y + oy) })
    }
  }
  els += key-elements(legend, cx, y1 + oy + 18)
  record(W, y1 + oy + 18 + lg.h, els)
}
