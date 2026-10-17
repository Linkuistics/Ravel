// Processes and authority — C4 container view. The ONLY editable source of
// this trial figure; geometry is placed by hand, sizes come from measured text.
// `--input variant=long` adds a long relation label and an extra container.
#import "/src/figures/_style.typ": *
#show: figure-page

#let long = sys.inputs.at("variant", default: "base") == "long"
#let G = pal.green

// An entity: C4 stereotype, name, technology, description, in a role.
#let ent-body(e, k) = {
  let parts = ()
  parts.push(text(size: sz.qual, fill: k.text, e.stereo))
  parts.push(text(size: sz.head, weight: "bold", fill: k.text, e.name))
  if e.at("tech", default: none) != none { parts.push(text(size: sz.qual, fill: k.text, [\[#e.tech\]])) }
  if e.at("desc", default: none) != none {
    parts.push(v(6pt))
    parts.push(text(size: sz.body, fill: k.text, e.desc))
  }
  box(width: e.w * 1pt - 24pt, stack(dir: ttb, spacing: 4pt, ..parts.map(x => align(center, x))))
}

#let ents = (
  caller: (kind: "external", cx: 520, top: 8, w: 300, stereo: [«external_system»], name: [caller],
    desc: [An orchestrator or person: requests work and interprets the opaque result]),
  worker: (kind: "ravel", cx: 117, top: 270, w: 200, stereo: [«container»], name: [policy worker],
    tech: [Separate process; compiled Bun],
    desc: [Evaluates trusted owner policy. No session-result context.]),
  cli: (kind: "ravel", cx: 520, top: 250, w: 270, stereo: [«container»], name: [Ravel CLI],
    tech: [Rust process: ravel start-session],
    desc: [Checks the selection and decides the final outcome. The runner executes inside this process and owns launch and supervision.]),
  managed: (kind: "external", cx: 930, top: 285, w: 230, stereo: [«external_system»], name: [managed process],
    desc: [Selected direct child: a harness running an LLM session, or another program]),
  channel: (kind: "temporary", cx: 520, top: 500, w: 270, stereo: [«temporary» «container»], name: [result channel],
    tech: [Temporary directory and completion file],
    desc: [Carries one opaque result; no persistent history], db: true),
) + if long {
  (audit: (kind: "ravel", cx: 520, top: 760, w: 270, stereo: [«container»], name: [audit sink],
    tech: [Owner-supplied executable],
    desc: [Receives a copy of the final outcome for the owner's own records.]),)
} else { (:) }

#context {
  // measure every entity from the pinned font, then derive its box
  let geo = (:)
  for (id, e) in ents {
    let k = kinds.at(e.kind)
    let s = size-of(ent-body(e, k))
    let h = s.h + 22 + if e.at("db", default: false) { 20 } else { 0 }
    geo.insert(id, (x0: e.cx - e.w / 2, x1: e.cx + e.w / 2, y0: e.top, y1: e.top + h, cx: e.cx, cy: e.top + h / 2, h: h))
  }
  // the CLI sets the row centre; its neighbours centre on it
  let rowc = geo.cli.cy
  for id in ("worker", "managed") {
    let b = geo.at(id)
    geo.insert(id, (..b, y0: rowc - b.h / 2, y1: rowc + b.h / 2, cy: rowc))
  }
  let g = geo
  canvas({
  let lbl(x, y, anchor, label, tech, color: pal.ink, w: auto, left: false) = {
    let body = box(width: len(w), fill: pal.white, inset: 2pt, align(if left { std.left } else { center }, stack(dir: ttb, spacing: 3pt,
      text(size: sz.body, fill: color, label),
      if tech != none { text(size: sz.qual, fill: color, tech) })))
    draw-text(x, y, body, anchor: anchor)
  }
  let rel(color, ..pts) = draw-arrow(..pts.pos(), color: color)

  // boundary first (behind everything)
  let cli = g.cli
  let bx0 = 0
  let bx1 = cli.x1 + 26
  let by0 = cli.y0 - 62
  let by1 = g.channel.y1 + 28
  draw-line((bx0, by0), (bx1, by0), (bx1, by1), (bx0, by1), (bx0, by0), color: pal.slate, dash: (4pt, 4pt))
  draw-text((bx0 + bx1) / 2, by0 + 22, ctext([Ravel software], size: sz.head))
  draw-text((bx0 + bx1) / 2, by0 + 40, ctext([\[System\]], size: sz.qual))

  // entities
  for (id, e) in ents {
    let k = kinds.at(e.kind)
    let b = g.at(id)
    if e.at("db", default: false) {
      draw-cylinder(b.x0, b.y0, b.x1, b.y1, k)
      draw-text(b.cx, b.cy + 8, ent-body(e, k))
    } else {
      draw-box(b.x0, b.y0, b.x1, b.y1, k)
      draw-text(b.cx, b.cy, ent-body(e, k))
    }
  }

  // relations
  let xr = cli.cx - 70
  rel(pal.rel, (xr, g.caller.y1), (xr, cli.y0))
  lbl(xr - 8, (g.caller.y1 + by0) / 2, "east", [Requests a session], [\[kind and prompt\]], left: true)
  let xw = cli.cx + 70
  rel(G, (xw, cli.y0), (xw, g.caller.y1))
  lbl(xw + 8, (g.caller.y1 + by0) / 2, "west", [Writes opaque result only after successful cleanup], [\[standard output\]], color: G, w: 170, left: true)
  let ya = cli.cy - 26
  let yb = cli.cy + 30
  rel(pal.rel, (g.worker.x1, ya), (cli.x0, ya))
  lbl((g.worker.x1 + cli.x0) / 2, ya - 4, "south", [Policy response], [\[selection or refusal; separate socket\]], w: 150)
  rel(pal.rel, (cli.x0, yb), (g.worker.x1, yb))
  lbl((g.worker.x1 + cli.x0) / 2, yb + 4, "north", [1\. Evaluates request], [\[bounded request; restricted environment\]], w: 150)
  rel(pal.rel, (cli.x1, cli.cy), (g.managed.x0, cli.cy))
  lbl((cli.x1 + g.managed.x0) / 2, cli.cy + if long { 4 } else { -4 }, if long { "north" } else { "south" },
    if long { [2\. Launches in its own process group, with the narrowed environment the owner policy granted] } else { [2\. Launches] },
    [\[selected command; session-result context\]], w: 150)
  let ch = g.channel
  rel(G, (g.managed.cx, g.managed.y1), (g.managed.cx, ch.cy), (ch.x1, ch.cy))
  lbl(g.managed.cx + 8, (g.managed.y1 + ch.cy) / 2 - 20, "west", [Publishes result via end-session], none, color: G, w: 120, left: true)
  rel(G, (ch.cx, ch.y0), (ch.cx, cli.y1))
  lbl(ch.cx + 8, (ch.y0 + cli.y1) / 2, "west", [Supplies result], [\[validate and read\]], color: G, left: true)
  if long {
    let au = g.audit
    rel(pal.rel, (cli.x0, cli.y1 - 18), (cli.x0 - 12, cli.y1 - 18), (cli.x0 - 12, au.cy), (au.x0, au.cy))
    lbl(cli.x0 - 20, (ch.y1 + au.y0) / 2, "east", [Writes a copy of the outcome], [\[owner-configured\]], left: true)
  }
  // key
  let kl = (
    [Blue: Ravel processes. Grey: external participants. Gold: temporary channel.],
    [Green arrows carry the opaque result.],
    [The LLM asks its harness to run end-session.],
    [The managed process's children inherit its session-result context.],
    [Publication asks Ravel to stop; return waits for successful cleanup.],
    [1 precedes 2: the worker and managed process are not concurrent phases.],
    [Before 2: worker ended and reaped; its private directory removed.],
    [A valid selection alone is insufficient for launch.],
    [The runner is library code inside the Ravel CLI, not another process.],
    [The enclosing outline is software membership, not a sandbox.],
    [Boundaries do not authenticate same-user code or contain detached children.],
  )
  legend-block(kl, g.managed.x1 / 2, (if long { g.audit.y1 } else { by1 }) + 18)
  })
}
