// UML timing layout: lifelines whose states change at named instants on one
// shared time axis, signals between lifelines, and duration constraints. A
// `_*.typ` module: imported by a timing figure, never rendered. It returns a layout
// record (shape: the header of _draw.typ) and draws nothing: `render` in _draw.typ
// does.
//
// Placement is AUTOMATIC, and a figure source holds no coordinate. The source is
// a list of panels. Each panel has its instants (the named points of the time
// axis, in order), its lifelines (one row each, top to bottom), its signals and
// its duration constraints. The module measures every label in the pinned font
// and spaces the instants from what must fit between them, so a longer label
// moves the instants after it and edits nothing else.
//
// **The time axis is not to scale.** The instants are in order, and the distance
// between two is what their labels need. A figure says so in its key.
//
//   * An `instant` is a point on the axis. Its label is drawn above the panel,
//     on a dashed guide that runs down through the rows.
//   * A lifeline is a row of states. A `state` is a box that runs from the
//     previous state's end to the instant named by `to` (`"end"`, the default,
//     is the right-hand end of the panel). Its text sits inside. A state that
//     ends the lifeline can be followed by an X (`destroy: true`).
//   * A `msg` is a signal at an instant, from the edge of one lifeline's row to
//     the edge of the row next to it. Its label sits beside the arrow.
//   * A `bound` is a duration constraint between two instants: a labelled
//     double arrow under the last row, as wide as the two instants are apart.
//
//   #import "_timing.typ": *
//   #import "_draw.typ": render
//   #render(timing([Title], (panel(...), ...), key-lines))
//
// Roles: a lifeline takes a box role of `_style.typ` (`ravel`, `external`,
// `temporary`, `note`) and its states use it unless a `state` names another.
// A signal is "relation" or "result" (the green arrow).
//
// What the module does not do is avoid a collision that the measured sizes do not
// prevent: it makes room for each state label, each instant label, each signal
// label and each constraint label, and nothing more. A signal runs between two
// rows that are next to each other.
#import "_layout.typ": *

// ---- source -----------------------------------------------------------------

// A point on the time axis. `label` (`\` between lines) is drawn above the panel.
#let instant(id, label: none) = (id: id, label: label)

// A state of a lifeline: it ends at the instant `to` (an instant id, or "end").
// `body` is its text, `\` between lines. A different `role` colours only this
// state.
#let state(body, to: "end", role: auto) = (body: body, to: to, role: role)

// A lifeline: a row named `name` whose states, in order, start at the left-hand
// end of the panel. `destroy: true` puts an X after the last state.
#let lifeline(id, name, role, states, destroy: false) = (
  id: id, name: name, role: role, states: states, destroy: destroy,
)

// A signal at the instant `at`, from the lifeline `from` to the lifeline `to`.
// `dx` moves it along the axis, away from its instant.
#let msg(from, to, at, label, role: "relation", dx: 0) = (
  from: from, to: to, at: at, label: label, role: role, dx: dx,
)

// A duration constraint from the instant `from` to the instant `to`. Two that
// overlap take different `row`s.
#let bound(from, to, label, row: 0) = (from: from, to: to, label: label, row: row)

// One timing diagram under a heading.
#let panel(heading, instants, lifelines, msgs: (), bounds: ()) = (
  heading: heading, instants: instants, lifelines: lifelines, msgs: msgs, bounds: bounds,
)

// ---- sizes ------------------------------------------------------------------

#let name-gap = 14 // between a lifeline's name and its first state
#let state-pad = 10 // each side of a state's text
#let min-gap = 30 // least distance between two instants
#let row-min = 36 // least height of a row
#let row-gap = 40 // least distance between two rows
#let dim-gap = 28 // from the last row to the first constraint
#let dim-step = 34 // from one constraint row to the next
#let panel-gap = 26 // between two panels
#let cross = 8 // half the width of the X

#let tsize(body, face: "body") = size-of(ctext(label(plain(body)), ..faces.at(face)))

#let moved(el, dx, dy) = {
  if el.shape == "line" or el.shape == "polygon" {
    (..el, pts: el.pts.map(p => (p.at(0) + dx, p.at(1) + dy)))
  } else if el.shape == "box" {
    (..el, x0: el.x0 + dx, x1: el.x1 + dx, y0: el.y0 + dy, y1: el.y1 + dy)
  } else {
    (..el, x: el.x + dx, y: el.y + dy)
  }
}

// ---- one panel --------------------------------------------------------------
// Coordinates are local: x = 0 at the panel's left edge, y = 0 at its top.

#let lay-panel(p, i, namew) = {
  let pre = "p" + str(i + 1) + "-"
  let m = p.instants.len()
  assert(m > 0 and p.lifelines.len() > 0, message: "a timing panel needs an instant and a lifeline")

  // Axis nodes: 0 is the left-hand end, 1..m the instants, m + 1 the right-hand end.
  let ix = (:)
  for (k, it) in p.instants.enumerate() {
    assert(it.id not in ix and it.id not in ("start", "end"), message: "two instants share the id " + it.id)
    ix.insert(it.id, k + 1)
  }
  let node(id) = {
    if id == "end" { return m + 1 }
    assert(id in ix, message: "no instant has the id " + repr(id))
    ix.at(id)
  }
  let row = (:)
  for (r, l) in p.lifelines.enumerate() {
    assert(l.id not in row, message: "two lifelines share the id " + l.id)
    row.insert(l.id, r)
  }
  let n = p.lifelines.len()

  // --- what must fit between the nodes ---------------------------------------
  let reqs = ()
  let segs = ()
  for l in p.lifelines {
    let cur = 0
    let mine = ()
    for s in l.states {
      let hi = node(s.to)
      assert(hi > cur, message: "a state of " + l.id + " ends at " + repr(s.to) + ", which is not after the previous state")
      reqs.push((cur, hi, tsize(s.body).w + 2 * state-pad))
      mine.push((lo: cur, hi: hi, s: s))
      cur = hi
    }
    assert(l.destroy or cur == m + 1, message: "the last state of " + l.id + " must run to the end, or the lifeline must be destroyed")
    segs.push(mine)
  }
  let il = p.instants.map(it => if it.label == none { (w: 0, h: 0) } else { tsize(it.label) })
  reqs.push((0, 1, il.at(0).w / 2 + 4))
  for k in range(m - 1) {
    reqs.push((k + 1, k + 2, (il.at(k).w + il.at(k + 1).w) / 2 + 16))
  }
  let x-tail = p.lifelines.any(l => l.destroy and segs.at(row.at(l.id)).last().hi == m + 1)
  reqs.push((m, m + 1, il.at(m - 1).w / 2 + 4 + if x-tail { 2 * cross + 8 } else { 0 }))
  for b in p.bounds {
    let (lo, hi) = (node(b.from), node(b.to))
    assert(hi > lo, message: "a constraint must run forward, from " + repr(b.from) + " to " + repr(b.to))
    reqs.push((lo, hi, tsize(b.label).w + 2 * 20))
  }
  for g in p.msgs {
    reqs.push((node(g.at), node(g.at) + 1, tsize(g.label).w + 16))
  }
  let gap = range(m + 1).map(k => min-gap)
  for q in reqs.sorted(key: q => q.at(1) - q.at(0)) {
    let (lo, hi, need) = q
    let have = range(lo, hi).map(k => gap.at(k)).sum()
    if have < need {
      let add = (need - have) / (hi - lo)
      for k in range(lo, hi) { gap.at(k) += add }
    }
  }
  let x0 = namew + name-gap
  let X = (x0,)
  for k in range(m + 1) { X.push(X.at(k) + gap.at(k)) }

  // --- rows -------------------------------------------------------------------
  let hd = tsize(p.heading, face: "name")
  let lab-h = calc.max(..il.map(l => l.h))
  let lab-bottom = hd.h + 14 + lab-h
  let guide-top = lab-bottom + 4
  let rh = p.lifelines.map(l => calc.max(
    row-min,
    calc.max(..l.states.map(s => tsize(s.body).h)) + 18,
    tsize(l.name, face: "head").h + 16,
  ))
  let gapv = range(n - 1).map(r => row-gap)
  for g in p.msgs {
    assert(g.from in row and g.to in row, message: "a signal names a lifeline that the panel does not have")
    let (a, b) = (row.at(g.from), row.at(g.to))
    assert(calc.abs(a - b) == 1, message: "a signal runs between two rows that are next to each other, not " + g.from + " and " + g.to)
    gapv.at(calc.min(a, b)) = calc.max(gapv.at(calc.min(a, b)), tsize(g.label).h + 16)
  }
  let top = ()
  let y = lab-bottom + 14
  for r in range(n) {
    top.push(y)
    y += rh.at(r) + if r < n - 1 { gapv.at(r) } else { 0 }
  }
  let rows-bottom = y
  let dim-rows = if p.bounds.len() == 0 { 0 } else { calc.max(..p.bounds.map(b => b.row)) + 1 }
  let dim-y(r) = rows-bottom + dim-gap + r * dim-step
  let bottom = if dim-rows == 0 { rows-bottom } else { dim-y(dim-rows - 1) + 12 }

  // --- elements, in drawing order ----------------------------------------------
  let els = (text-el(pre + "heading", p.heading, 0, hd.h / 2, face: "name", anchor: "west", align: "left"),)
  // guides first, so the rows sit above them
  for (k, it) in p.instants.enumerate() {
    let ends = p.bounds.filter(b => b.from == it.id or b.to == it.id).map(b => dim-y(b.row) + 6)
    let to = if ends.len() == 0 { rows-bottom } else { calc.max(rows-bottom, ..ends) }
    els.push(line-el(pre + "guide-" + it.id, "lifeline", ((X.at(k + 1), guide-top), (X.at(k + 1), to)), dashed: true))
    if it.label != none {
      els.push(text-el(pre + "when-" + it.id, it.label, X.at(k + 1), lab-bottom, anchor: "south"))
    }
  }
  for (r, l) in p.lifelines.enumerate() {
    els.push(box-el(pre + "name-" + l.id, l.role, 0, top.at(r), namew, top.at(r) + rh.at(r)))
    els.push(text-el(pre + "name-" + l.id + "-label", l.name, namew / 2, top.at(r) + rh.at(r) / 2, role: l.role, face: "head"))
    for (si, sg) in segs.at(r).enumerate() {
      let role = if sg.s.role == auto { l.role } else { sg.s.role }
      let id = pre + l.id + "-" + str(si + 1)
      let (xa, xb) = (X.at(sg.lo), X.at(sg.hi))
      els.push(box-el(id, role, xa, top.at(r), xb, top.at(r) + rh.at(r)))
      els.push(text-el(id + "-label", sg.s.body, (xa + xb) / 2, top.at(r) + rh.at(r) / 2, role: role))
    }
    if l.destroy {
      let (cx, cy) = (X.at(segs.at(r).last().hi) + cross + 4, top.at(r) + rh.at(r) / 2)
      els.push(line-el(pre + l.id + "-end-a", "end", ((cx - cross, cy - cross), (cx + cross, cy + cross))))
      els.push(line-el(pre + l.id + "-end-b", "end", ((cx - cross, cy + cross), (cx + cross, cy - cross))))
    }
  }
  for (gi, g) in p.msgs.enumerate() {
    let (a, b) = (row.at(g.from), row.at(g.to))
    let (y0, y1) = if b > a { (top.at(a) + rh.at(a), top.at(b)) } else { (top.at(a), top.at(b) + rh.at(b)) }
    let x = X.at(node(g.at)) + g.dx
    let id = pre + "msg-" + str(gi + 1)
    els.push(line-el(id, g.role, ((x, y0), (x, y1)), arrow: true))
    els.push(text-el(id + "-label", g.label, x + 7, (y0 + y1) / 2, anchor: "west", align: "left"))
  }
  for (bi, b) in p.bounds.enumerate() {
    let (xa, xb) = (X.at(node(b.from)), X.at(node(b.to)))
    let (xm, yd) = ((xa + xb) / 2, dim-y(b.row))
    let lw = tsize(b.label).w
    let id = pre + "bound-" + str(bi + 1)
    els.push(line-el(id + "-a", "relation", ((xm - lw / 2 - 6, yd), (xa, yd)), arrow: true))
    els.push(line-el(id + "-b", "relation", ((xm + lw / 2 + 6, yd), (xb, yd)), arrow: true))
    els.push(text-el(id + "-label", b.label, xm, yd))
  }
  (w: X.last() + 6, h: bottom, els: els)
}

// ---- the figure -------------------------------------------------------------

// Returns a deferred layout: calling it, inside `render`'s context, gives the
// record. Pass it straight to `render`.
#let timing(title, panels, legend) = () => {
  let tit = tsize(title, face: "title")
  let lg = key-size(legend)
  let namew = calc.max(..panels.map(p => calc.max(..p.lifelines.map(l => tsize(l.name, face: "head").w)))) + 22
  let lays = panels.enumerate().map(((i, p)) => lay-panel(p, i, namew))
  let W = calc.max(..lays.map(l => l.w), lg.w + 12, tit.w + 12)
  let ox = (W - calc.max(..lays.map(l => l.w))) / 2
  let els = (text-el("title", title, W / 2, tit.h / 2, face: "title"),)
  let y = tit.h + 14
  for (i, ly) in lays.enumerate() {
    if i > 0 {
      els.push(line-el("sep-" + str(i), "rule", ((0, y - panel-gap / 2), (W, y - panel-gap / 2))))
    }
    els += ly.els.map(e => moved(e, ox, y))
    y += ly.h + panel-gap
  }
  els += key-elements(legend, W / 2, y - panel-gap + 14)
  let ids = els.map(e => e.id)
  for (i, id) in ids.enumerate() {
    assert(id not in ids.slice(0, i), message: "two timing elements share the id " + id)
  }
  record(W, y - panel-gap + 14 + lg.h, els)
}
