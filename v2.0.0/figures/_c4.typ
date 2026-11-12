// Declarative C4 layout (system context, container and, for the component view,
// the same pieces). A `_*.typ` module: imported by C4 figures, never rendered. It
// returns a layout record (shape: the header of _draw.typ) and draws nothing:
// `render` in _draw.typ draws it.
//
// Placement is MANUAL, and sizes are measured. A figure source gives each entity
// a centre and a top (or `cy-of`, to sit level with another entity) and a
// width; the module measures the entity's text in the pinned font and derives the
// height. A relation names two entities and the side of each it leaves and enters
// by, with an offset along the side; the module routes an orthogonal polyline
// between the two ports and places the label beside one segment. What the module
// does not do is choose positions or avoid collisions: moving an entity, a port
// or a label is an edit to the figure source and a render to look at.
//
//   #import "_c4.typ": *
//   #import "_draw.typ": render
//   #render(c4([Title], ents, bounds, rels, key-lines))
//
// Roles: an entity is in a kinds role (ravel, external, owner, temporary); a
// relation is "relation" or "result" (the arrow, and the green label, that carry
// the session result). Entities are placed by `cx` and `top` in figure
// coordinates, y-down; only their positions relative to each other matter, since
// the module puts the topmost element (a boundary's header, a label) 14 below the
// title.
#import "_layout.typ": *

// Entity: «stereotype», name, [technology] and a description, in a role.
// `w` is the box width; text wraps 24 points inside it. `db` draws the cylinder
// (the shared one, with its label inside). Give `top`, or `cy-of` another entity's
// id to centre on that entity's row.
#let ent(id, role, name, stereo, cx: 0, top: none, cy-of: none, w: 220, tech: none, desc: none, db: false) = (
  id: id, role: role, name: name, stereo: stereo, cx: cx, top: top, cy-of: cy-of, w: w, tech: tech, desc: desc, db: db,
)

// A dashed outline around entities, with a name and a tag (`[System]`) at the top.
#let boundary(id, name, members, tag: none, pad: 26) = (id: id, name: name, members: members, tag: tag, pad: pad)

// A relation from one entity to another. `out` and `into` are each a side
// ("top", "bottom", "left", "right") or (side, offset): the offset moves the port
// along that side from its middle, in points (down a left/right side, right along
// a top/bottom side). Omitted, the sides face each other across whichever gap the
// two boxes have. The route is straight when the ports line up; an L when one
// side is vertical and the other horizontal; otherwise a Z whose middle run is
// halfway, or `jog` points from the start port.
//
// The label sits beside one segment, `seg` (default: the longest), `t` of the way
// along it. `side` is "above" or "below" a horizontal segment, "right" or "left"
// of a vertical one (default above, right). `dx` and `dy` nudge it, `w` wraps it,
// and it is centred beside a horizontal segment and left-aligned beside a
// vertical one unless `align` says otherwise.
#let rel(from, to, label, tech: none, role: "relation", out: auto, into: auto, jog: none,
  seg: auto, t: 0.5, side: auto, dx: 0, dy: 0, w: auto, align: auto) = (
  from: from, to: to, label: label, tech: tech, role: role, out: out, into: into, jog: jog,
  seg: seg, t: t, side: side, dx: dx, dy: dy, w: w, align: align,
)

// ---- layout ---------------------------------------------------------------

// Returns a deferred layout: calling it, inside `render`'s context, gives the
// record. Pass it straight to `render`.
#let c4(title, ents, bounds, rels, legend) = () => {
  // --- entities: measured parts, then boxes ------------------------------
  let parts-of = e => {
    let limit = e.w - 24
    let mk(suffix, body, face) = text-el(e.id + "-" + suffix, body, 0, 0, role: e.role, face: face, w: fit(body, face, limit))
    let ps = (mk("stereo", e.stereo, "qual"), mk("name", e.name, "name"))
    if e.tech != none { ps.push(mk("tech", "[" + plain(e.tech) + "]", "qual")) }
    if e.desc != none { ps.push(mk("desc", e.desc, "body")) }
    ps
  }
  let geo = (:)
  let parts = (:)
  for e in ents {
    let ps = parts-of(e)
    let gap = ps.map(p => 4).sum() - 4 + if e.desc != none { 6 } else { 0 }
    let h = ps.map(p => p.h).sum() + gap + 22 + if e.db { 20 } else { 0 }
    parts.insert(e.id, (ps: ps, content-h: ps.map(p => p.h).sum() + gap))
    geo.insert(e.id, (x0: e.cx - e.w / 2, x1: e.cx + e.w / 2, cx: e.cx, h: h, y0: none, y1: none, cy: none))
  }
  // vertical position: `top`, or the centre of another entity's row
  let place(g, id, cy) = {
    let b = g.at(id)
    g.insert(id, (..b, y0: cy - b.h / 2, y1: cy + b.h / 2, cy: cy))
    g
  }
  for e in ents {
    if e.top != none { geo = place(geo, e.id, e.top + geo.at(e.id).h / 2) }
  }
  for e in ents {
    if e.cy-of != none { geo = place(geo, e.id, geo.at(e.cy-of).cy) }
  }
  for e in ents {
    assert(geo.at(e.id).y0 != none, message: "entity " + e.id + " has neither top nor cy-of")
  }

  // --- boundaries: members' extent, padded, with room for the name -------
  let bgeo = ()
  for b in bounds {
    let ms = b.members.map(id => geo.at(id))
    let nm = text-el(b.id + "-name", b.name, 0, 0, face: "head")
    let tg = if b.tag != none { text-el(b.id + "-tag", "[" + plain(b.tag) + "]", 0, 0, face: "qual") } else { none }
    let head = nm.h + if tg != none { tg.h + 4 } else { 0 }
    // wide enough for its own name, centred on its members when the name is wider
    let bx0 = calc.min(..ms.map(m => m.x0)) - b.pad
    let bx1 = calc.max(..ms.map(m => m.x1)) + b.pad
    let need = calc.max(nm.w, if tg != none { tg.w } else { 0 }) + 28
    if bx1 - bx0 < need {
      let mid = (bx0 + bx1) / 2
      bx0 = mid - need / 2
      bx1 = mid + need / 2
    }
    bgeo.push((
      b: b, nm: nm, tg: tg,
      x0: bx0,
      x1: bx1,
      y0: calc.min(..ms.map(m => m.y0)) - b.pad - head - 14,
      y1: calc.max(..ms.map(m => m.y1)) + b.pad,
    ))
  }

  // --- relations: ports, routes, labels -----------------------------------
  let rels-out = ()
  let seen = (:)
  for r in rels {
    let a = geo.at(r.from)
    let b = geo.at(r.to)
    let face = facing(a, b)
    let (s0, o0) = side-and-offset(if r.out == auto { face.at(0) } else { r.out })
    let (s1, o1) = side-and-offset(if r.into == auto { face.at(1) } else { r.into })
    let pts = route(port(a, s0, o0), s0, port(b, s1, o1), s1, r.jog)

    // the segment the label sits beside
    let lens = range(pts.len() - 1).map(i => calc.abs(pts.at(i + 1).at(0) - pts.at(i).at(0)) + calc.abs(pts.at(i + 1).at(1) - pts.at(i).at(1)))
    let si = if r.seg != auto { r.seg } else { lens.position(l => l == calc.max(..lens)) }
    let (p, q) = (pts.at(si), pts.at(si + 1))
    let at = (p.at(0) + (q.at(0) - p.at(0)) * r.t, p.at(1) + (q.at(1) - p.at(1)) * r.t)
    let flat = calc.abs(q.at(1) - p.at(1)) < calc.abs(q.at(0) - p.at(0))
    let side = if r.side != auto { r.side } else if flat { "above" } else { "right" }
    assert((side in ("above", "below")) == flat, message: "relation " + r.from + "-" + r.to + ": side " + side + " does not fit its segment")
    let align = if r.align != auto { r.align } else if flat { "center" } else { "left" }

    let text-color = if r.role == "result" { "result" } else { "label" }
    let lw = if r.w == auto { auto } else { r.w }
    let l1 = text-el("l1", r.label, 0, 0, role: text-color, w: lw)
    let l2 = if r.tech != none { text-el("l2", "[" + plain(r.tech) + "]", 0, 0, role: text-color, face: "qual", w: lw) } else { none }
    let bw = calc.max(l1.w, if l2 != none { l2.w } else { 0 })
    let bh = l1.h + if l2 != none { l2.h + 3 } else { 0 }
    let cx = at.at(0) + r.dx
    let cy = at.at(1) + r.dy
    if side == "above" { cy -= 4 + bh / 2 }
    else if side == "below" { cy += 4 + bh / 2 }
    else if side == "right" { cx += 8 + bw / 2 }
    else { cx -= 8 + bw / 2 }

    let base = r.from + "-" + r.to
    seen.insert(base, seen.at(base, default: 0) + 1)
    let id = "rel-" + base + if seen.at(base) > 1 { "-" + str(seen.at(base)) } else { "" }
    let x-of(w) = if align == "left" { cx - bw / 2 + w / 2 } else { cx }
    let els = (line-el(id, r.role, pts, arrow: true),
      text-el(id + "-label", r.label, x-of(l1.w), cy - bh / 2 + l1.h / 2, role: text-color, align: align, w: lw))
    if l2 != none {
      els.push(text-el(id + "-tech", "[" + plain(r.tech) + "]", x-of(l2.w), cy - bh / 2 + l1.h + 3 + l2.h / 2,
        role: text-color, face: "qual", align: align, w: lw))
    }
    rels-out.push((els: els, x0: cx - bw / 2, x1: cx + bw / 2, y0: cy - bh / 2, y1: cy + bh / 2))
  }

  // --- extents, then shift everything below the title --------------------
  let boxes = geo.values() + bgeo + rels-out
  let x0 = calc.min(..boxes.map(b => b.x0))
  let x1 = calc.max(..boxes.map(b => b.x1))
  let y1 = calc.max(..boxes.map(b => b.y1))
  let tit = size-of(ctext(label(plain(title)), ..faces.title))
  let lg = key-size(legend)
  let W = calc.max(x1 - x0, lg.w + 12, tit.w + 12)
  let cx = (x0 + x1) / 2
  // the topmost element, whatever it is, sits 14 below the title
  let oy = tit.h + 14 - calc.min(..boxes.map(b => b.y0))

  // --- elements, in drawing order -----------------------------------------
  let els = (text-el("title", title, cx, tit.h / 2, face: "title"),)
  for g in bgeo {
    let b = g.b
    let bx = (g.x0 + g.x1) / 2
    els.push(line-el(b.id, "boundary", ((g.x0, g.y0 + oy), (g.x1, g.y0 + oy), (g.x1, g.y1 + oy), (g.x0, g.y1 + oy), (g.x0, g.y0 + oy)), dashed: true))
    els.push(text-el(b.id + "-name", b.name, bx, g.y0 + oy + 14 + g.nm.h / 2, face: "head"))
    if g.tg != none {
      els.push(text-el(b.id + "-tag", "[" + plain(b.tag) + "]", bx, g.y0 + oy + 14 + g.nm.h + 4 + g.tg.h / 2, face: "qual"))
    }
  }
  for e in ents {
    let b = geo.at(e.id)
    let ps = parts.at(e.id)
    if e.db {
      els.push(cylinder-el(e.id, e.role, b.x0, b.y0 + oy, b.x1, b.y1 + oy))
    } else {
      els.push(box-el(e.id, e.role, b.x0, b.y0 + oy, b.x1, b.y1 + oy))
    }
    // the parts stack from the top of the content, which is centred in the box
    // (a cylinder's content sits 8 lower, clear of the top cap)
    let y = b.cy + oy + if e.db { 8 } else { 0 } - ps.content-h / 2
    for (i, p) in ps.ps.enumerate() {
      if i == ps.ps.len() - 1 and e.desc != none { y += 6 }
      els.push((..p, x: b.cx, y: y + p.h / 2))
      y += p.h + 4
    }
  }
  for r in rels-out {
    for el in r.els {
      els.push(if el.shape == "line" { (..el, pts: el.pts.map(q => (q.at(0), q.at(1) + oy))) } else { (..el, y: el.y + oy) })
    }
  }
  els += key-elements(legend, cx, y1 + oy + 18)
  record(W, y1 + oy + 18 + lg.h, els)
}
