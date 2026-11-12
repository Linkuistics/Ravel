// UML activity layout: actions, decisions and merges with guards, fork and join,
// partitions, notes, and initial and final nodes. A `_*.typ` module: imported by
// an activity figure, never rendered. It returns a layout record (shape: the
// header of _draw.typ) and draws nothing: `render` in _draw.typ does.
//
// Placement is AUTOMATIC, and a figure source holds no coordinate. The source is
// a structured flow, a list of nodes from top to bottom; the module measures each
// node's text in the pinned font and stacks the nodes on one spine, so adding a
// node or lengthening a label moves what is below it and edits nothing else.
//
//   * A decision has two outgoing flows. `down` continues on the spine. `right`
//     leaves by the right-hand vertex to a branch beside the diamond, whose first
//     node is level with it. A flow carries a guard unless the source gives `none`. A decision with
//     `merge: true` closes with a merge diamond that the live branches rejoin; a
//     decision without it needs its `right` branch to end (in an `outcome` or a
//     `final`), and its `down` branch is the rest of the flow.
//   * A `fork` has a bar, one column per branch and a join bar. A branch is a list
//     of nodes. A `partition` is a dashed outline with a title around a list of
//     nodes.
//   * `via:` labels the flow that enters a node. `note:` hangs a note beside an
//     action or an outcome on a dashed connector.
//   * A branch that ends the flow ends in an `outcome` (a grey box with text) or a
//     `final` (the filled ring). Nothing may follow either.
//
//   #import "_activity.typ": *
//   #import "_draw.typ": render
//   #render(activity([Title], nodes, key-lines))
//
// Roles: an action is `phase` (blue), an outcome is `outcome` (grey), the flow is
// "relation". Decision and merge diamonds and the ring of a final node are
// `control`, the fork and join bars `bar` (both in _style.typ).
//
// What the module does not do is avoid collisions between a long label and a
// neighbour: it makes room for each guard and each via label (a side flow is as
// long as its guard needs, a spine flow as tall as its label needs) and nothing
// more. A note is placed to the right of its node and widens the block it is in.
#import "_layout.typ": *

// ---- nodes ------------------------------------------------------------------

#let initial(id, via: none) = (kind: "initial", id: id, via: via)
#let final(id, via: none) = (kind: "final", id: id, via: via)

// An action: `body` is its text, `\` between lines. `limit` is the widest line, in
// points, before it wraps. An outcome is a grey box that ends the flow.
#let action(id, body, via: none, note: none, limit: 240) = (
  kind: "action", id: id, body: body, via: via, note: note, limit: limit,
)
#let outcome(id, body, via: none, note: none, limit: 240) = (
  kind: "outcome", id: id, body: body, via: via, note: note, limit: limit,
)

// One outgoing flow of a decision: its guard (`none` draws none) and the nodes it
// passes through.
#let branch(guard, ..nodes) = (guard: guard, body: nodes.pos())

// A decision. `question` is drawn left of the diamond (or `none`).
#let decision(id, question, down: none, right: none, merge: false, via: none) = (
  kind: "decision", id: id, question: question, down: down, right: right, merge: merge, via: via,
)

// A fork bar, one column per branch (a list of nodes) and a join bar, `join`.
#let fork(id, join, branches, via: none) = (
  kind: "fork", id: id, join: join, branches: branches, via: via,
)

#let partition(id, title, ..nodes, via: none) = (
  kind: "partition", id: id, title: title, items: nodes.pos(), via: via,
)

// ---- sizes ------------------------------------------------------------------

#let pad-x = 22 // an action's text to its edge, as in _state.typ
#let pad-y = 9
#let dot-r = 9
#let ring-r = 11
#let core-r = 6
#let dia-x = 22 // a decision or merge diamond's half width and half height
#let dia-y = 16
#let bar-h = 6
#let flow-gap = 30 // between a node and the next on the spine
#let side-gap = 34 // least length of a flow that leaves a decision to the right
#let col-gap = 40 // between the columns of a fork
#let note-gap = 28
#let part-pad = 16

// ---- layers -----------------------------------------------------------------
// A block's elements in drawing order: outlines at the back, then nodes, then
// flows and their labels.

#let layers(back, nodes, flows) = (back: back, nodes: nodes, flows: flows)

#let shift(el, dx, dy) = {
  if el.shape == "line" or el.shape == "polygon" {
    (..el, pts: el.pts.map(p => (p.at(0) + dx, p.at(1) + dy)))
  } else if el.shape == "box" or el.shape == "cylinder" {
    (..el, x0: el.x0 + dx, x1: el.x1 + dx, y0: el.y0 + dy, y1: el.y1 + dy)
  } else {
    (..el, x: el.x + dx, y: el.y + dy)
  }
}
#let shifted(ly, dx, dy) = layers(
  ly.back.map(e => shift(e, dx, dy)),
  ly.nodes.map(e => shift(e, dx, dy)),
  ly.flows.map(e => shift(e, dx, dy)),
)
#let joined(a, b) = layers(a.back + b.back, a.nodes + b.nodes, a.flows + b.flows)

// A laid-out node or run of nodes. Coordinates are relative to the block's top
// (y = 0) and its spine (x = 0).
//   l, r      how far the block reaches left and right of the spine
//   h         its height
//   top       where the flow that enters it ends
//   bottom    where the flow that leaves it starts, or `none` when it ends the flow
//   half      the half width of its own shape, for the alignment of side branches
//   need      the least distance from the spine to a side branch (a decision's)
//   fmid, fhalf   its first node's centre height and half width: a side flow enters there
//   first, last   the ids of its first and last nodes, to name the flows between blocks
//   out       a guard still to be drawn on the flow that leaves it, or `none`
#let blk(l, r, h, top, bottom, ly, half: 0, need: 0, fmid: 0, fhalf: 0, first: none, last: none, out: none) = (
  l: l, r: r, h: h, top: top, bottom: bottom, ly: ly, half: half, need: need,
  fmid: fmid, fhalf: fhalf, first: first, last: last, out: out,
)

#let flow-id(a, b) = "flow-" + a + "-" + b

// ---- layout -----------------------------------------------------------------
// One recursive function lays out any node and any run of nodes (kind "seq").
// `gut` is the least distance from the spine to a side branch, so the side
// branches of one run line up.

#let lay(n, gut) = {
  let k = n.kind
  if k == "initial" {
    let d = 2 * dot-r
    blk(dot-r, dot-r, d, 0, d, layers((), (circle-el(n.id, "initial", 0, dot-r, dot-r),), ()),
      half: dot-r, fmid: dot-r, fhalf: dot-r, first: n.id, last: n.id)
  } else if k == "final" {
    let d = 2 * ring-r
    let nodes = (circle-el(n.id, "control", 0, ring-r, ring-r), circle-el(n.id + "-core", "initial", 0, ring-r, core-r))
    blk(ring-r, ring-r, d, 0, none, layers((), nodes, ()),
      half: ring-r, fmid: ring-r, fhalf: ring-r, first: n.id, last: n.id)
  } else if k == "action" or k == "outcome" {
    let role = if k == "action" { "phase" } else { "outcome" }
    let t = text-el(n.id + "-text", n.body, 0, 0, role: role, w: fit(n.body, "body", n.limit))
    let w = t.w + 2 * pad-x
    let h = t.h + 2 * pad-y
    let nodes = (box-el(n.id, role, -w / 2, 0, w / 2, h), (..t, x: 0, y: h / 2))
    let flows = ()
    let r = w / 2
    if n.note != none {
      let nt = text-el(n.id + "-note-text", n.note, 0, 0, w: fit(n.note, "body", 200))
      let (nw, nh, f) = (nt.w + 28, nt.h + 24, 10)
      let x0 = w / 2 + note-gap
      let (x1, y0) = (x0 + nw, h / 2 - nh / 2)
      nodes.push(poly-el(n.id + "-note", "note", ((x0, y0), (x1 - f, y0), (x1, y0 + f), (x1, y0 + nh), (x0, y0 + nh))))
      nodes.push(line-el(n.id + "-note-fold", "fold", ((x1 - f, y0), (x1 - f, y0 + f), (x1, y0 + f))))
      nodes.push((..nt, x: (x0 + x1) / 2, y: h / 2))
      flows.push(line-el(n.id + "-note-link", "boundary", ((w / 2, h / 2), (x0, h / 2)), dashed: true))
      r = x1
    }
    blk(w / 2, r, h, 0, if k == "outcome" { none } else { h }, layers((), nodes, flows),
      half: w / 2, fmid: h / 2, fhalf: w / 2, first: n.id, last: n.id)
  } else if k == "decision" {
    assert(n.down != none and n.right != none, message: "decision " + n.id + " needs a down and a right branch")
    assert(n.right.body.len() > 0, message: "decision " + n.id + ": its right branch needs a node")
    let (hx, hy) = (dia-x, dia-y)
    let gd = if n.down.guard == none { none } else { text-el(n.id + "-guard-down", n.down.guard, 8, 0, anchor: "west") }
    let gr = if n.right.guard == none { none } else { text-el(n.id + "-guard-right", n.right.guard, 0, 0, anchor: "south") }
    let q = if n.question == none { none } else {
      text-el(n.id + "-question", n.question, 0, 0, anchor: "east", w: fit(n.question, "body", 200))
    }
    let side = lay((kind: "seq", items: n.right.body), 0)
    let down = if n.down.body.len() > 0 { lay((kind: "seq", items: n.down.body), 0) } else { none }
    let cy = calc.max(hy, side.fmid) // the diamond's centre; the side flow runs level with it
    let gap-down = calc.max(flow-gap, if gd == none { 0 } else { gd.h + 26 })
    let sg = calc.max(side-gap, if gr == none { 0 } else { gr.w + 24 })
    let xe = calc.max(gut, hx + sg) // the side branch's near edge
    let sx = xe + side.l // its spine
    let x-end = sx - side.fhalf // where the side flow enters its first node
    let y-side = cy - side.fmid
    let y-down = cy + hy + gap-down - if down == none { 0 } else { down.top }

    let nodes = (poly-el(n.id, "control", ((0, cy - hy), (hx, cy), (0, cy + hy), (-hx, cy))),)
    if q != none { nodes.push((..q, x: -hx - 8, y: cy)) }
    let flows = (line-el(flow-id(n.id, side.first), "relation", ((hx, cy), (x-end, cy)), arrow: true),)
    if gr != none { flows.push((..gr, x: (hx + x-end) / 2, y: cy - 4)) }
    nodes += shifted(side.ly, sx, y-side).nodes
    flows += shifted(side.ly, sx, y-side).flows
    let back = shifted(side.ly, sx, y-side).back
    if down != none {
      let ly = shifted(down.ly, 0, y-down)
      back += ly.back
      nodes += ly.nodes
      flows += ly.flows
      flows.push(line-el(flow-id(n.id, down.first), "relation", ((0, cy + hy), (0, cy + hy + gap-down)), arrow: true))
      if gd != none { flows.push((..gd, y: cy + hy + gap-down / 2)) }
    }
    let spine-end = if down == none { cy + hy } else { y-down + down.h }
    let side-end = y-side + side.h
    let l = calc.max(hx + if q == none { 0 } else { 8 + q.w }, if down == none { 0 } else { down.l })
    let r = calc.max(sx + side.r, if down == none { 0 } else { down.r })
    let common = (half: hx, need: hx + sg, fmid: cy, fhalf: hx, first: n.id)

    if n.merge {
      let live-down = down == none or down.bottom != none
      assert(live-down or side.bottom != none, message: "decision " + n.id + ": both branches end the flow, so there is nothing to merge")
      let m-y0 = calc.max(spine-end, side-end) + flow-gap
      let m-cy = m-y0 + hy
      let mid = n.id + "-merge"
      nodes.push(poly-el(mid, "control", ((0, m-y0), (hx, m-cy), (0, m-y0 + 2 * hy), (-hx, m-cy))))
      if down == none {
        flows.push(line-el(flow-id(n.id, mid), "relation", ((0, cy + hy), (0, m-y0)), arrow: true))
        if gd != none { flows.push((..gd, y: (cy + hy + m-y0) / 2)) }
      } else if down.bottom != none {
        flows.push(line-el(flow-id(down.last, mid), "relation", ((0, y-down + down.bottom), (0, m-y0)), arrow: true))
      }
      if side.bottom != none {
        flows.push(line-el(flow-id(side.last, mid), "relation", ((sx, y-side + side.bottom), (sx, m-cy), (hx, m-cy)), arrow: true))
      }
      let h = m-y0 + 2 * hy
      blk(l, r, h, cy - hy, h, layers(back, nodes, flows), ..common, last: mid)
    } else {
      assert(side.bottom == none, message: "decision " + n.id + ": its right branch must end the flow (in an outcome or a final), or the decision needs merge: true")
      let bottom = if down == none { cy + hy } else if down.bottom == none { none } else { y-down + down.bottom }
      blk(l, r, calc.max(spine-end, side-end), cy - hy, bottom, layers(back, nodes, flows), ..common,
        last: if down == none { n.id } else { down.last },
        out: if down == none and gd != none { (id: n.id + "-guard-down", body: n.down.guard) } else { none })
    }
  } else if k == "fork" {
    assert(n.branches.len() >= 2, message: "fork " + n.id + " needs two branches or more")
    let cols = n.branches.map(b => lay((kind: "seq", items: b), 0))
    let total = cols.map(c => c.l + c.r).sum() + col-gap * (cols.len() - 1)
    let wide = calc.max(total, 60)
    let xs = ()
    let x = -total / 2
    for c in cols {
      xs.push(x + c.l)
      x += c.l + c.r + col-gap
    }
    let y0 = bar-h + flow-gap
    let nodes = (box-el(n.id, "bar", -wide / 2, 0, wide / 2, bar-h),)
    let back = ()
    let flows = ()
    let bottoms = ()
    for (i, c) in cols.enumerate() {
      let ly = shifted(c.ly, xs.at(i), y0 - c.top)
      back += ly.back
      nodes += ly.nodes
      flows += ly.flows
      flows.push(line-el(flow-id(n.id, c.first), "relation", ((xs.at(i), bar-h), (xs.at(i), y0)), arrow: true))
      bottoms.push(y0 - c.top + c.h)
    }
    assert(cols.any(c => c.bottom != none), message: "fork " + n.id + ": every branch ends the flow, so there is nothing to join")
    let jy = calc.max(..bottoms) + flow-gap
    nodes.push(box-el(n.join, "bar", -wide / 2, jy, wide / 2, jy + bar-h))
    for (i, c) in cols.enumerate() {
      if c.bottom != none {
        flows.push(line-el(flow-id(c.last, n.join), "relation", ((xs.at(i), y0 - c.top + c.bottom), (xs.at(i), jy)), arrow: true))
      }
    }
    blk(wide / 2, wide / 2, jy + bar-h, 0, jy + bar-h, layers(back, nodes, flows),
      half: wide / 2, fmid: bar-h / 2, fhalf: wide / 2, first: n.id, last: n.join)
  } else if k == "partition" {
    let body = lay((kind: "seq", items: n.items), 0)
    let t = text-el(n.id + "-title", n.title, 0, 0, face: "head", anchor: "west")
    // The title sits left of the spine, so the flow that enters crosses no text.
    let l = calc.max(body.l + part-pad, part-pad + t.w + 14)
    let r = body.r + part-pad
    let y-body = t.h + 22 - body.top
    let h = y-body + body.h + part-pad
    let outline = line-el(n.id, "boundary", ((-l, 0), (r, 0), (r, h), (-l, h), (-l, 0)), dashed: true)
    let ly = shifted(body.ly, 0, y-body)
    let title = (..t, x: -l + part-pad, y: 8 + t.h / 2)
    blk(l, r, h, t.h + 22, if body.bottom == none { none } else { y-body + body.bottom },
      layers((outline, title) + ly.back, ly.nodes, ly.flows),
      half: r, fmid: y-body + body.fmid, fhalf: body.fhalf, first: body.first, last: body.last, out: body.out)
  } else if k == "seq" {
    assert(n.items.len() > 0, message: "a run of nodes needs a node")
    // First lay the run out with no alignment, to learn the widest node and the
    // widest side flow, then again with every side branch at one distance.
    let g = 0
    for b in n.items.map(m => lay(m, 0)) { g = calc.max(g, b.need, b.half + side-gap) }
    let bls = n.items.map(m => lay(m, g))
    let ly = layers((), (), ())
    let (l, r, half, need) = (0, 0, 0, 0)
    let prev = none
    let prev-y = 0
    for (i, b) in bls.enumerate() {
      let m = n.items.at(i)
      let y = 0
      let lt = none
      if prev != none {
        assert(prev.bottom != none, message: "node " + b.first + " follows a node that ends the flow")
        let label = if m.via != none { m.via } else if prev.out != none { prev.out.body } else { none }
        let id = flow-id(prev.last, b.first)
        if label != none { lt = text-el(id + "-label", label, 8, 0, anchor: "west") }
        // the flow is as long as the gap, and as its label needs
        y = calc.max(prev-y + prev.h + flow-gap, prev-y + prev.bottom + if lt == none { 0 } else { lt.h + 10 } - b.top)
        let (y0, y1) = (prev-y + prev.bottom, y + b.top)
        ly.flows.push(line-el(id, "relation", ((0, y0), (0, y1)), arrow: true))
        if lt != none { ly.flows.push((..lt, y: (y0 + y1) / 2)) }
      }
      ly = joined(ly, shifted(b.ly, 0, y))
      (l, r, half, need) = (calc.max(l, b.l), calc.max(r, b.r), calc.max(half, b.half), calc.max(need, b.need))
      prev = b
      prev-y = y
    }
    let first = bls.first()
    blk(l, r, prev-y + prev.h, first.top, if prev.bottom == none { none } else { prev-y + prev.bottom }, ly,
      half: half, need: need, fmid: first.fmid, fhalf: first.fhalf, first: first.first, last: prev.last, out: prev.out)
  } else {
    panic("unknown activity node kind " + repr(k))
  }
}

// ---- the figure -----------------------------------------------------------------

// Returns a deferred layout: calling it, inside `render`'s context, gives the
// record. Pass it straight to `render`.
#let activity(title, nodes, legend) = () => {
  let flow = lay((kind: "seq", items: nodes), 0)
  let tit = size-of(ctext(label(plain(title)), ..faces.title))
  let lg = key-size(legend)
  let W = calc.max(flow.l + flow.r, lg.w + 12, tit.w + 12)
  // the content is centred; the topmost element sits 14 below the title
  let (ox, oy) = ((W - flow.l - flow.r) / 2 + flow.l, tit.h + 14)
  let ly = shifted(flow.ly, ox, oy)
  let els = (text-el("title", title, W / 2, tit.h / 2, face: "title"),) + ly.back + ly.nodes + ly.flows
  els += key-elements(legend, W / 2, oy + flow.h + 18)
  let ids = els.map(e => e.id)
  for (i, id) in ids.enumerate() {
    assert(id not in ids.slice(0, i), message: "two activity nodes or flows share the id " + id)
  }
  record(W, oy + flow.h + 18 + lg.h, els)
}
