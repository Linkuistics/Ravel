// Declarative UML-sequence layout, promoted from the k107 trial. A `_*.typ`
// module: imported by sequence figures, never rendered. Input: participants and
// rows. Layout (column gaps, wrapped label heights, row heights) is computed from
// MEASURED Typst text, so the pinned font's metrics drive the figure and no layout
// engine or extraction step is needed. It returns a layout record (shape: the
// header of _draw.typ) and draws nothing: `render` in _draw.typ draws it.
#import "_layout.typ": *

#let part(id, label, kind: "external", db: false) = (id: id, label: label, kind: kind, db: db)
// `role` is "relation" or "result" (the arrow that carries the session result).
#let msg(from, to, label, role: "relation", dashed: false, create: false) = (t: "msg", from: from, to: to, label: label, role: role, dashed: dashed, create: create)
#let div(label) = (t: "div", label: label)
#let destroy(id) = (t: "destroy", id: id)
#let note(a, b, label) = (t: "note", a: a, b: b, label: label)

// Returns a deferred layout: calling it, inside `render`'s context, gives the
// record. Pass it straight to `render`.
#let sequence(title, parts, rows, legend, max-label: 170, min-gap: 14) = () => {
  let n = parts.len()
  let idx = (:)
  for (i, pt) in parts.enumerate() { idx.insert(pt.id, i) }

  // --- head sizes (16pt, measured) -------------------------------------
  let hs = parts.map(pt => {
    let s = size-of(ctext(label(plain(pt.label)), ..faces.head))
    (w: s.w + 22, h: s.h + 12 + if pt.db { 26 } else { 0 })
  })

  // --- label measurements -----------------------------------------------
  let lab(body, w: auto) = size-of(ctext(label(plain(body))), w: w)
  let natural(body) = {
    let s = size-of(ctext(label(plain(body))))
    calc.min(s.w, max-label) + 2
  }

  // --- column gaps: each label must fit between its participants --------
  let gap = range(n - 1).map(i => (hs.at(i).w + hs.at(i + 1).w) / 2 + min-gap)
  let reqs = ()
  for r in rows {
    if r.t == "msg" {
      let a = idx.at(r.from)
      let b = idx.at(r.to)
      let lw = natural(r.label)
      if a == b {
        if a < n - 1 { reqs.push((a, a + 1, lw + 40)) }
      } else {
        let extra = if r.create { hs.at(b).w / 2 } else { 0 }
        reqs.push((calc.min(a, b), calc.max(a, b), lw + 28 + extra))
      }
    } else if r.t == "note" {
      let a = idx.at(r.a)
      let b = idx.at(r.b)
      if a != b {
        let lw = natural(r.label)
        reqs.push((a, b, lw + 24 - (hs.at(a).w + hs.at(b).w) / 2))
      }
    }
  }
  for q in reqs.sorted(key: q => q.at(1) - q.at(0)) {
    let (lo, hi, need) = q
    let have = range(lo, hi).map(k => gap.at(k)).sum()
    if have < need {
      let add = (need - have) / (hi - lo)
      for k in range(lo, hi) { gap.at(k) += add }
    }
  }
  let x = (hs.at(0).w / 2 + 6,)
  for i in range(1, n) { x.push(x.at(i - 1) + gap.at(i - 1)) }

  // --- vertical layout --------------------------------------------------
  let y = 0
  let items = ()
  let started = range(n).map(i => false)
  let top = ()
  let ends = range(n).map(i => none)
  let tit = size-of(ctext(label(plain(title)), ..faces.title))
  y = tit.h + 14
  let head-h = 0
  for (i, pt) in parts.enumerate() {
    let created = rows.any(r => r.t == "msg" and r.create and r.to == pt.id)
    if not created {
      head-h = calc.max(head-h, hs.at(i).h)
    }
  }
  let life-from = range(n).map(i => none)
  for (i, pt) in parts.enumerate() {
    let created = rows.any(r => r.t == "msg" and r.create and r.to == pt.id)
    if not created {
      items.push((t: "head", i: i, cy: y + hs.at(i).h / 2))
      life-from.at(i) = y + hs.at(i).h
    }
  }
  y += head-h + 10

  for r in rows {
    if r.t == "msg" {
      let a = idx.at(r.from)
      let b = idx.at(r.to)
      if a == b {
        let lw = natural(r.label)
        let l = lab(r.label, w: lw)
        let loop-h = 18
        let rh = calc.max(l.h, loop-h)
        let ya = y + 4 + (rh - loop-h) / 2
        items.push((t: "self", x: x.at(a), y: ya, lw: lw, lh: l.h, label: r.label, role: r.role, dashed: r.dashed, loop-h: loop-h))
        y += rh + 14
      } else {
        let x1 = x.at(a)
        let x2 = x.at(b)
        let dir = if b > a { 1 } else { -1 }
        if r.create { x2 = x2 - dir * hs.at(b).w / 2 }
        let span = calc.abs(x2 - x1)
        let lw = calc.min(natural(r.label), span - 12)
        let l = lab(r.label, w: lw)
        let hh = if r.create { hs.at(b).h / 2 } else { 0 }
        let ya = y + calc.max(l.h + 6, hh)
        items.push((t: "msg", x1: x1, x2: x2, y: ya, mx: (x1 + x2) / 2, lw: lw, lh: l.h, label: r.label, role: r.role, dashed: r.dashed))
        if r.create {
          items.push((t: "head", i: b, cy: ya))
          life-from.at(b) = ya + hs.at(b).h / 2
        }
        y = ya + calc.max(10, hh + 6)
      }
    } else if r.t == "div" {
      items.push((t: "div", y: y + 18, label: r.label))
      y += 38
    } else if r.t == "destroy" {
      let i = idx.at(r.id)
      items.push((t: "x", x: x.at(i), y: y + 4))
      ends.at(i) = y + 4
      y += 22
    } else if r.t == "note" {
      let a = idx.at(r.a)
      let b = idx.at(r.b)
      let x0 = x.at(a) - hs.at(a).w / 2
      let x1 = x.at(b) + hs.at(b).w / 2
      let w = x1 - x0
      let l = lab(r.label, w: w - 20)
      items.push((t: "note", x0: x0, x1: x1, y0: y + 2, h: l.h + 14, label: r.label, lw: w - 20))
      y += l.h + 14 + 14
    }
  }
  let y-end = y

  // --- extents ----------------------------------------------------------
  let right = x.at(n - 1) + hs.at(n - 1).w / 2
  for it in items {
    if it.t == "self" { right = calc.max(right, it.x + 28 + it.lw + 4) }
  }
  let dw = right + 6
  let lg = key-size(legend)
  let W = calc.max(dw, lg.w + 12, tit.w + 12)
  let ox = (W - dw) / 2

  // --- elements, in drawing order -----------------------------------------
  let els = ()
  let n-msg = 0
  els.push(text-el("title", title, W / 2, tit.h / 2, face: "title"))

  // lifelines first, so heads and messages sit above them
  for (i, pt) in parts.enumerate() {
    if life-from.at(i) != none {
      let e = if ends.at(i) != none { ends.at(i) } else { y-end }
      els.push(line-el("life-" + pt.id, "lifeline", ((ox + x.at(i), life-from.at(i)), (ox + x.at(i), e)), dashed: true))
    }
  }
  for it in items {
    if it.t == "head" {
      let pt = parts.at(it.i)
      let h = hs.at(it.i)
      let cx = ox + x.at(it.i)
      let id = "head-" + pt.id
      if pt.db {
        els.push(cylinder-el(id, pt.kind, cx - h.w / 2, it.cy - h.h / 2, cx + h.w / 2, it.cy + h.h / 2))
        els.push(text-el(id + "-label", pt.label, cx, it.cy + 7, role: pt.kind, face: "head"))
      } else {
        els.push(box-el(id, pt.kind, cx - h.w / 2, it.cy - h.h / 2, cx + h.w / 2, it.cy + h.h / 2))
        els.push(text-el(id + "-label", pt.label, cx, it.cy, role: pt.kind, face: "head"))
      }
    } else if it.t == "msg" {
      n-msg += 1
      let id = "msg-" + str(n-msg)
      els.push(line-el(id, it.role, ((ox + it.x1, it.y), (ox + it.x2, it.y)), dashed: it.dashed, arrow: true))
      els.push(text-el(id + "-label", it.label, ox + it.mx, it.y - 4, anchor: "south", w: it.lw))
    } else if it.t == "self" {
      n-msg += 1
      let id = "msg-" + str(n-msg)
      let x0 = ox + it.x
      els.push(line-el(id, it.role, ((x0, it.y), (x0 + 22, it.y), (x0 + 22, it.y + it.loop-h), (x0, it.y + it.loop-h)),
        dashed: it.dashed, arrow: true))
      els.push(text-el(id + "-label", it.label, x0 + 28, it.y + it.loop-h / 2, anchor: "west", align: "left", w: it.lw))
    } else if it.t == "x" {
      let s = 8
      let x0 = ox + it.x
      let id = "end-" + str(els.len())
      els.push(line-el(id + "-a", "end", ((x0 - s, it.y - s), (x0 + s, it.y + s))))
      els.push(line-el(id + "-b", "end", ((x0 - s, it.y + s), (x0 + s, it.y - s))))
    } else if it.t == "div" {
      let l = size-of(ctext(label(plain(it.label))))
      let id = "div-" + str(els.len())
      els.push(line-el(id + "-rule-a", "rule", ((0, it.y - 2), (dw, it.y - 2))))
      els.push(line-el(id + "-rule-b", "rule", ((0, it.y + 2), (dw, it.y + 2))))
      els.push(box-el(id, "divider", dw / 2 - l.w / 2 - 10, it.y - l.h / 2 - 3, dw / 2 + l.w / 2 + 10, it.y + l.h / 2 + 3))
      els.push(text-el(id + "-label", it.label, dw / 2, it.y))
    } else if it.t == "note" {
      // A page with its top-right corner folded over.
      let x0 = ox + it.x0
      let x1 = ox + it.x1
      let y0 = it.y0
      let y1 = y0 + it.h
      let f = 10
      let id = "note-" + str(els.len())
      els.push(poly-el(id, "note", ((x0, y0), (x1 - f, y0), (x1, y0 + f), (x1, y1), (x0, y1))))
      els.push(line-el(id + "-fold", "fold", ((x1 - f, y0), (x1 - f, y0 + f), (x1, y0 + f))))
      els.push(text-el(id + "-label", it.label, (x0 + x1) / 2, (y0 + y1) / 2, w: it.lw))
    }
  }
  els += key-elements(legend, W / 2, y-end + 6)
  record(W, y-end + 6 + lg.h, els)
}
