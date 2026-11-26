// UML deployment layout: nested enclosures (a node, directories with a tab)
// holding artifacts, and dependencies between artifacts. A `_*.typ` module:
// imported by a deployment figure, never rendered. It returns a layout record
// (shape: the header of _draw.typ) and draws nothing: `render` in _draw.typ does.
//
// Placement is AUTOMATIC, because an enclosure is a container: a figure source is
// a tree, each enclosure laying its children out in a `row` or a `col`, and the
// module measures the text, sizes every box from the inside out and places them.
// Nothing is given a coordinate. A row aligns its children on the centre of the
// first artifact each holds, so a dependency between two artifacts in a row runs
// straight even when one sits inside a deeper enclosure.
//
//   #import "_deployment.typ": *
//   #import "_draw.typ": render
//   #render(deployment([Title], tree, deps, key-lines))
//
// Roles: an artifact is in a kinds role (ravel for an executable, external for a
// supporting file). An enclosure is the `note` role: outlined, and nearly white.
// An enclosure's name is never drawn across its outline: a directory's name sits
// in its tab, which is sized to the name, and a node's name below its top face.
#import "_layout.typ": *

// An artifact: a box with a page icon, holding `body` (one line per name, `\`
// between lines). `limit` is the widest line, in points, before it wraps.
#let artifact(id, role, body, limit: 240) = (kind: "artifact", id: id, role: role, body: body, limit: limit)

// An enclosure holding `children`, laid out as a `col` (stacked, centred) or a
// `row` (side by side, aligned as above) with `gap` between them and `pad` round
// them. `dir` is a directory (a tab holding its name); `node` is the UML node, a
// box seen from above and to the right, named below its top face.
#let enclosure(shape, id, name, flow, gap, pad, children) = (
  kind: "enclosure", shape: shape, id: id, name: name, flow: flow, gap: gap, pad: pad, children: children,
)
#let dir(id, name, ..children, flow: "col", gap: 26, pad: 18) = enclosure("dir", id, name, flow, gap, pad, children.pos())
#let node(id, name, ..children, flow: "col", gap: 26, pad: 18) = enclosure("node", id, name, flow, gap, pad, children.pos())

// A dependency: a dashed arrow from one artifact to the one on its right.
#let dep(from, to) = (from: from, to: to)

// ---- sizes ------------------------------------------------------------------

#let art-pad = (x: 16, y: 11)
#let icon = (w: 12, h: 16, cut: 5, inset: 7) // the page icon's corner, in the box's top right
#let tab-pad = 12 // either side of a directory's name
#let tab-slant = 8
#let node-depth = 14 // the node's top and right faces

// The box, measured: `w`, `h`, and `anchor`, the height of the first artifact's
// centre below the box's top. An enclosure also keeps its children, each with its
// offset inside the content area, and the content area's size.
#let measure-node(n) = {
  if n.kind == "artifact" {
    let t = text-el(n.id + "-text", n.body, 0, 0, role: n.role, w: fit(n.body, "body", n.limit))
    // room beside the text for the icon, so a long first line never meets it
    let w = t.w + 2 * art-pad.x + 2 * icon.inset
    let h = calc.max(t.h + 2 * art-pad.y, icon.h + 2 * icon.inset)
    return (..n, t: t, w: w, h: h, anchor: h / 2)
  }
  let kids = n.children.map(measure-node)
  let nm = text-el(n.id + "-name", n.name, 0, 0, face: "head")
  let row = n.flow == "row"
  assert(row or n.flow == "col", message: "enclosure " + n.id + ": flow is row or col, not " + repr(n.flow))
  let gaps = n.gap * (kids.len() - 1)
  let cw
  let ch
  let offs
  let anchor
  if row {
    let above = calc.max(..kids.map(k => k.anchor))
    cw = kids.map(k => k.w).sum() + gaps
    ch = above + calc.max(..kids.map(k => k.h - k.anchor))
    let x = 0
    offs = ()
    for k in kids {
      offs.push((x, above - k.anchor))
      x += k.w + n.gap
    }
    anchor = above
  } else {
    cw = calc.max(..kids.map(k => k.w))
    ch = kids.map(k => k.h).sum() + gaps
    let y = 0
    offs = ()
    for k in kids {
      offs.push(((cw - k.w) / 2, y))
      y += k.h + n.gap
    }
    anchor = kids.first().anchor
  }
  // the head sits above the content: a directory's tab, or a node's top face and name
  let tab-h = nm.h + 12
  let head = if n.shape == "dir" { tab-h } else { node-depth + 8 + nm.h }
  // the face the content sits on: a directory's whole box, a node's front
  let fw = calc.max(cw + 2 * n.pad, nm.w + tab-pad * 2 + tab-slant + if n.shape == "dir" { 0 } else { 8 })
  let top = head + n.pad
  (
    ..n, kids: kids, offs: offs, nm: nm, cw: cw, fw: fw, tab-h: tab-h, top: top,
    w: fw + if n.shape == "node" { node-depth } else { 0 },
    h: top + ch + n.pad,
    anchor: top + anchor,
  )
}

// ---- elements, in drawing order ----------------------------------------------

// The page icon: a sheet with its top right corner folded, in the artifact's role.
#let icon-elements(id, role, x1, y0) = {
  let (ix, iy) = (x1 - icon.inset - icon.w, y0 + icon.inset)
  let c = icon.cut
  (
    poly-el(id + "-icon", role, ((ix, iy), (ix + icon.w - c, iy), (ix + icon.w, iy + c), (ix + icon.w, iy + icon.h), (ix, iy + icon.h))),
    line-el(id + "-icon-fold", "fold", ((ix + icon.w - c, iy), (ix + icon.w - c, iy + c), (ix + icon.w, iy + c))),
  )
}

// Places measured `n` with its top left at (x, y): its elements and a geometry
// entry (box, centre) per id.
#let place(n, x, y) = {
  if n.kind == "artifact" {
    let b = (x0: x, y0: y, x1: x + n.w, y1: y + n.h, cx: x + n.w / 2, cy: y + n.h / 2)
    return (
      els: (box-el(n.id, n.role, b.x0, b.y0, b.x1, b.y1), ..icon-elements(n.id, n.role, b.x1, b.y0), (..n.t, x: b.cx, y: b.cy)),
      geo: ((n.id): b),
    )
  }
  let b = (x0: x, y0: y, x1: x + n.w, y1: y + n.h, cx: x + n.w / 2, cy: y + n.h / 2)
  let els = ()
  if n.shape == "dir" {
    let tw = n.nm.w + 2 * tab-pad
    els.push(poly-el(n.id, "note", (
      (x, y), (x + tw, y), (x + tw + tab-slant, y + n.tab-h), (b.x1, y + n.tab-h), (b.x1, b.y1), (x, b.y1),
    )))
    els.push((..n.nm, x: x + tab-pad + n.nm.w / 2, y: y + n.tab-h / 2))
  } else {
    let (d, fx, fy) = (node-depth, x + n.fw, y + node-depth)
    // the whole solid, then its front face and the edge between the faces
    els.push(poly-el(n.id, "note", ((x, fy), (x + d, y), (b.x1, y), (b.x1, b.y1 - d), (fx, b.y1), (x, b.y1))))
    els.push(poly-el(n.id + "-front", "note", ((x, fy), (fx, fy), (fx, b.y1), (x, b.y1))))
    els.push(line-el(n.id + "-edge", "boundary", ((fx, fy), (b.x1, y))))
    els.push((..n.nm, x: x + n.fw / 2, y: fy + 8 + n.nm.h / 2))
  }
  let geo = ((n.id): b)
  let (ox, oy) = (x + (n.fw - n.cw) / 2, y + n.top)
  for (k, o) in n.kids.zip(n.offs) {
    let r = place(k, ox + o.at(0), oy + o.at(1))
    els += r.els
    geo += r.geo
  }
  (els: els, geo: geo)
}

// ---- layout -------------------------------------------------------------------

// A dependency's route: straight across the gap when the target's centre falls
// within the source's side, else a Z with its middle run halfway.
#let dep-route(a, b) = {
  assert(a.x1 <= b.x0, message: "a dependency runs to an artifact on the right: " + repr(a) + " " + repr(b))
  let reach = (a.y1 - a.y0) / 2 - 10
  if calc.abs(b.cy - a.cy) <= reach { ((a.x1, b.cy), (b.x0, b.cy)) } else {
    let xm = (a.x1 + b.x0) / 2
    ((a.x1, a.cy), (xm, a.cy), (xm, b.cy), (b.x0, b.cy))
  }
}

// Returns a deferred layout: calling it, inside `render`'s context, gives the
// record. Pass it straight to `render`.
#let deployment(title, tree, deps, legend) = () => {
  let root = measure-node(tree)
  let tit = size-of(ctext(label(plain(title)), ..faces.title))
  let lg = key-size(legend)
  let W = calc.max(root.w, lg.w + 12, tit.w + 12)
  let top = tit.h + 14
  let placed = place(root, (W - root.w) / 2, top)
  let els = (text-el("title", title, W / 2, tit.h / 2, face: "title"),) + placed.els
  for d in deps {
    let (a, b) = (placed.geo.at(d.from), placed.geo.at(d.to))
    els.push(line-el("dep-" + d.from + "-" + d.to, "relation", dep-route(a, b), dashed: true, arrow: true))
  }
  els += key-elements(legend, W / 2, top + root.h + 18)
  record(W, top + root.h + 18 + lg.h, els)
}
