// The book's shared figure style: what a figure looks like, as data. A `_*.typ`
// module: it is imported, never rendered, and has no SVG. Fonts, scale, palette
// roles, stroke weights and text measuring live here, so every figure matches and
// a change to the colours or weights happens in this one file. Layout and
// mechanics: docs/book/figures.md.
//
// Units: one canvas unit is 1pt, which is one SVG user unit. Geometry is written
// y-down (top-left origin).
//
// This module draws nothing and does not import CeTZ. A layout module (see
// _layout.typ) measures text here and returns a record of positions and roles;
// _draw.typ turns a record into strokes, reading the roles in this file. A check
// in scripts/figures.sh refuses any other module that imports CeTZ.

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

// Semantic roles for a box, cylinder or polygon: fill / text / border. `key` is
// the pale key box and `divider` the label on a section rule; both repeat the
// colours of `note` and `external` so a figure can say what the element is.
// `result` is for the text of a label on a result relation (green); it is not a
// box role.
#let kinds = (
  ravel: (fill: pal.blue, text: pal.white, line: pal.navy),
  external: (fill: pal.grey, text: pal.ink, line: pal.slate),
  owner: (fill: pal.owner, text: pal.white, line: pal.owner-line),
  temporary: (fill: pal.gold, text: pal.brown, line: pal.ochre),
  note: (fill: pal.pale, text: pal.ink, line: pal.slate),
  key: (fill: pal.pale, text: pal.ink, line: pal.slate),
  divider: (fill: pal.grey, text: pal.ink, line: pal.slate),
  result: (fill: pal.white, text: pal.green, line: pal.green),
  // State views: `phase` and `outcome` repeat the colours of `ravel` and
  // `external` and are drawn with rounded corners (see `radii` in _draw.typ);
  // `initial` is the filled marker of the entry transition.
  phase: (fill: pal.blue, text: pal.white, line: pal.navy),
  outcome: (fill: pal.grey, text: pal.ink, line: pal.slate),
  initial: (fill: pal.rel, text: pal.white, line: pal.rel),
  // Activity views: `control` is a decision or merge diamond and the ring of a
  // final node; `bar` is the fork or join bar. Both take the relation colour.
  control: (fill: pal.white, text: pal.ink, line: pal.rel),
  bar: (fill: pal.rel, text: pal.white, line: pal.rel),
)

// Text scale. Stereotype and technology lines («container», [Rust]) are the
// `qual` size and upright: DejaVu Oblique is not a pinned face, and Typst draws
// a missing italic upright, so the size step and the «» and [] marks carry the
// distinction that an italic would.
#let sz = (body: 14pt, head: 16pt, qual: 12pt)
#let w-main = 1.5pt // entities, boundaries, notes, keys, arrow shafts
#let w-life = 0.5pt // sequence lifelines
#let w-rule = 0.75pt // paired separator rules

// Semantic roles for a line: colour and weight. `relation` is a message or
// relation, `result` the one that carries the session result, `lifeline` a
// sequence lifeline, `rule` a paired separator rule, `fold` the folded corner of
// a note, `end` the X that ends a lifetime and `boundary` the dashed outline of a
// C4 boundary.
#let strokes = (
  relation: (color: pal.rel, weight: w-main),
  result: (color: pal.green, weight: w-main),
  lifeline: (color: pal.slate, weight: w-life),
  rule: (color: pal.slate, weight: w-rule),
  fold: (color: pal.slate, weight: w-main),
  end: (color: pal.rel, weight: w-main),
  boundary: (color: pal.slate, weight: w-main),
)

// Text faces. A label is plain text in one face; the face is the only way a
// record says how big or bold it is. `name` is a C4 entity's name and `qual` its
// stereotype and technology lines.
#let faces = (
  title: (size: sz.head, weight: "bold"),
  head: (size: sz.head),
  name: (size: sz.head, weight: "bold"),
  qual: (size: sz.qual),
  body: (:),
)

// Document setup for one figure: auto-sized white page, pinned font.
#let figure-page(body) = {
  set page(width: auto, height: auto, margin: 6pt, fill: pal.white)
  set text(font: font, size: sz.body, fill: pal.ink, lang: "en")
  set par(leading: 0.55em, spacing: 0pt)
  body
}

// A plain label string as content: one text run per line, joined by explicit
// line breaks. CeTZ sizes a string holding "\n" a little taller than the same
// text joined by `linebreak()`, so measuring and drawing both go through this.
#let label(s) = s.split("\n").map(l => [#l]).join(linebreak())

// Intrinsic-size text blocks; `w` fixes the wrapping width.
// Set edges inside the block: CeTZ wraps drawn content in its own text style.
// These explicit edges keep wrapped text identical in measurement and drawing.
#let len(w) = if type(w) == int or type(w) == float { w * 1pt } else { w }
#let ctext(body, w: auto, ..args) = box(width: len(w), align(center, text(top-edge: "cap-height", bottom-edge: "baseline", ..args, body)))
#let ltext(body, w: auto, ..args) = box(width: len(w), align(left, text(top-edge: "cap-height", bottom-edge: "baseline", ..args, body)))
#let fl(x) = x / 1pt
#let size-of(body, w: auto) = {
  let m = measure(box(width: len(w), body))
  (w: fl(m.width), h: fl(m.height))
}
