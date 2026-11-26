#!/usr/bin/env python3
"""Draw the rule-diagram mock-ups as SVG files.

Usage: python3 mockups.py OUTPUT_DIR

The script parses grammars in the workbench notation and lays out each rule by
the method of the spec `rule-diagrams`. It then draws the marks of a trace on the
diagrams. It is a sketch of the method for the design discussion. The layout of
the core is the work of `diagram-layout-k13`.

The script reads the example grammars from `examples/` in the repository. The
SVG files use system fonts and follow the light or dark scheme of the reader.
"""

import math
import re
import sys
import unicodedata
from pathlib import Path
from xml.sax.saxutils import escape

ROOT = Path(__file__).resolve().parents[3]

# The layout constants of the spec, in pixels.
FONT_SIZE = 13
CELL = 7.8  # The advance of one cell of the bundled monospace font: 0.6 em.
LINE = 16  # The height of one line of a label.
PAD_X = 10
PAD_Y = 8
MIN_W = 64
MIN_H = 36
MAX_CELLS = 18  # The widest line of a label, in cells.
BAND = 22  # The free band above each box, for reading markers and the data object.
DRILL = 12  # The band at the bottom of a reference box that holds its drill-down marker.
GW = 24  # The width and the height of a gateway.
EV = 24  # The diameter of an event.
FLOW = 24  # The length of a flow between two elements in a row.
PAIR = 10  # The length of a flow between two gateways of one repetition.
SKIP = 12  # The gap between the band of an item and its skip flow.
BACK = 18  # The gap between an item and its back flow.
VGAP = 14  # The gap between two branches of a choice.
LANE = 24  # The gap between two back-edge lanes.
MARGIN = 16

MONO_FAMILY = '"IBM Plex Mono",ui-monospace,Menlo,Consolas,monospace'
SANS_FAMILY = 'ui-sans-serif,system-ui,-apple-system,"Segoe UI",sans-serif'

STYLE = """
:root{--bg:#ffffff;--panel:#fbfbfc;--hdr:#f1f3f6;--line:#d5dae2;--t:#1d2330;--m:#677184;
--cur:#0e7c66;--curbg:#e3f4ef;--sel:#d99a00;--e:#b3261e;--ebg:#fdecea;--chip:#eceff4;
--note:#2b3445;--box:#ffffff;--boxl:#5f6878;--gw:#ffffff;--ref:#1f4fa3;--frame:#c9ced8;
--tok:#2f6fd6;--lbl:#f4f5f7;--code:#f4f5f7}
@media (prefers-color-scheme: dark){:root{--bg:#15171c;--panel:#1b1e25;--hdr:#232731;
--line:#363c48;--t:#e5e8ee;--m:#9aa3b3;--cur:#4fd1b5;--curbg:#173a33;--sel:#f2c94c;
--e:#ff7b72;--ebg:#3b1f22;--chip:#2a2f3b;--note:#d5dae3;--box:#1f232b;--boxl:#9aa3b3;
--gw:#1f232b;--ref:#8fb4ff;--frame:#3a404c;--tok:#6ea8ff;--lbl:#232731;--code:#20242c}}
svg{background:var(--bg)}
text{font-family:%SANS%;font-size:12px;fill:var(--t)}
.lab{font-family:%MONO%;font-size:13px;fill:var(--t)}
.labp{font-family:%SANS%;font-size:13px;fill:var(--t)}
.m{fill:var(--m)} .b{font-weight:700} .sm{font-size:11px} .e{fill:var(--e);font-weight:600}
.cur{fill:var(--cur);font-weight:600} .refc{fill:var(--ref);font-weight:600}
.h1{font-size:15px;font-weight:700} .h2{font-size:12px;font-weight:700;fill:var(--m);letter-spacing:.04em}
.panel{fill:var(--panel);stroke:var(--line)} .hdr{fill:var(--hdr)}
.task{fill:var(--box);stroke:var(--boxl);stroke-width:1.4}
.refbox{fill:var(--box);stroke:var(--boxl);stroke-width:3.2}
.gw{fill:var(--gw);stroke:var(--boxl);stroke-width:1.4} .gwx{stroke:var(--boxl);stroke-width:1.8}
.ev{fill:var(--box);stroke:var(--boxl);stroke-width:1.4} .evend{fill:var(--box);stroke:var(--boxl);stroke-width:3.6}
.flow{stroke:var(--boxl);stroke-width:1.4;fill:none}
.ah{fill:var(--boxl)} .drill{fill:var(--box);stroke:var(--boxl);stroke-width:1.2}
.lblbg{fill:var(--bg)} .lbl{font-family:%MONO%;font-size:11px;fill:var(--m)}
.tab{fill:var(--cur);stroke:var(--cur);stroke-width:1.2} .tabo{fill:var(--bg);stroke:var(--cur);stroke-width:1.4}
.tabt{font-family:%SANS%;font-size:11px;font-weight:700;fill:var(--bg)} .tabto{font-family:%SANS%;font-size:11px;font-weight:700;fill:var(--cur)}
.nextmark{fill:none;stroke:var(--cur);stroke-width:2} .nextmarko{fill:none;stroke:var(--cur);stroke-width:2;stroke-dasharray:5 3}
.nextbtn{fill:var(--cur)} .nextbtnt{font-size:10px;font-weight:700;fill:var(--bg)}
.passbg{fill:var(--curbg);stroke:var(--cur);stroke-width:1} .passt{font-family:%MONO%;font-size:11px;fill:var(--cur);font-weight:600}
.sel{fill:none;stroke:var(--sel);stroke-width:2.6} .selflow{fill:none;stroke:var(--sel);stroke-width:5;opacity:.55}
.backmark{fill:none;stroke:var(--cur);stroke-width:4;opacity:.45}
.data{fill:var(--box);stroke:var(--boxl);stroke-width:1.2} .assoc{stroke:var(--boxl);stroke-width:1.2;stroke-dasharray:2 3;fill:none}
.badge{fill:var(--chip)} .chip{fill:var(--curbg);stroke:var(--cur)} .chipt{font-family:%MONO%;font-size:11px;fill:var(--cur)}
.pop{fill:var(--bg);stroke:var(--line)} .banner{fill:var(--ebg);stroke:var(--e)}
.call{fill:var(--t)} .callt{fill:var(--bg);font-size:11px;font-weight:700}
.note{fill:var(--note)} .nest{fill:none;stroke:var(--m);stroke-width:1;stroke-dasharray:3 3}
.nestt{font-size:10px;fill:var(--m)} .tok{fill:var(--tok)} .tokt{font-size:10px;font-weight:700;fill:#fff}
.code{fill:var(--code);stroke:var(--line)} .mono{font-family:%MONO%;font-size:12px}
.crumb{fill:var(--ref)} .rejbox{fill:none;stroke:var(--e);stroke-width:2;stroke-dasharray:4 3}
.strike{fill:none;stroke:var(--m);stroke-width:1.3}
.loopbar{fill:none;stroke:var(--cur);stroke-width:2.5;stroke-linecap:round}
""".replace("%MONO%", MONO_FAMILY).replace("%SANS%", SANS_FAMILY)


# ----------------------------------------------------------------------------
# Text widths


def cells(text):
    """The width of a text in cells: 2 for a wide East Asian character, 0 for a combining mark."""
    n = 0
    for ch in text:
        if unicodedata.combining(ch):
            continue
        n += 2 if unicodedata.east_asian_width(ch) in ("W", "F") else 1
    return n


def mono_width(text):
    return cells(text) * CELL


# Approximate advances of a proportional sans font, in em. Only the font mock-up uses them.
PROP = {c: 0.28 for c in "ijl.,:;'!|"}
PROP.update({c: 0.33 for c in "frt -()"})
PROP.update({c: 0.83 for c in "mwMW"})
PROP.update({c: 0.5 for c in "abcdeghknopqsuvxyz_"})


def prop_width(text):
    return sum(PROP.get(ch, 0.62) for ch in text) * FONT_SIZE


class Font:
    def __init__(self, name, cls, width):
        self.name, self.cls, self.width = name, cls, width


MONO = Font("mono", "lab", mono_width)
SANS = Font("sans", "labp", prop_width)
font = MONO


def pieces(label):
    """Split a label after each space, `_` and `-`: the places where a line can break."""
    return re.findall(r"[^ _\-]+[ _\-]?|[ _\-]", label)


def greedy(parts, limit):
    lines, cur = [], ""
    for p in parts:
        if cur and font.width((cur + p).rstrip()) > limit:
            lines.append(cur.rstrip())
            cur = p
        else:
            cur += p
    lines.append(cur.rstrip())
    return lines


def wrap(label):
    """The fewest lines, and then the narrowest box with that many lines."""
    parts = pieces(label)
    limit = MAX_CELLS * CELL
    lines = greedy(parts, limit)
    n = len(lines)
    if n == 1:
        return lines
    low = max(font.width(p.rstrip()) for p in parts)
    w = low
    while w < limit:
        trial = greedy(parts, w)
        if len(trial) == n:
            return trial
        w += 1
    return lines


def even(v):
    v = math.ceil(v)
    return v + (v % 2)


# ----------------------------------------------------------------------------
# A small parser for the notation


class Grammar:
    def __init__(self, text):
        self.text = text
        self.rules = {}  # name -> dict(doc, body, span)
        self.start = None
        self.offsets = [0]
        for ch in text:
            self.offsets.append(self.offsets[-1] + len(ch.encode()))
        self.toks = list(self.tokens())
        self.i = 0
        self.statements()
        self.analyse()

    def tokens(self):
        t, i = self.text, 0
        while i < len(t):
            c = t[i]
            if c.isspace():
                i += 1
            elif t.startswith("//", i):
                while i < len(t) and t[i] != "\n":
                    i += 1
            elif t.startswith('"""', i):
                j = t.index('"""', i + 3)
                body = t[i + 3 : j]
                lines = [ln.strip() for ln in body.strip("\n").split("\n")]
                yield ("str", " ".join(lines).strip(), i, j + 3)
                i = j + 3
            elif c == '"':
                j = i + 1
                while t[j] != '"':
                    j += 2 if t[j] == "\\" else 1
                yield ("str", t[i + 1 : j].replace('\\"', '"'), i, j + 1)
                i = j + 1
            elif c == "%":
                m = re.match(r"%[a-z]+", t[i:])
                yield ("dir", m.group(0), i, i + len(m.group(0)))
                i += len(m.group(0))
            elif c.isalpha():
                m = re.match(r"[A-Za-z][A-Za-z0-9_]*", t[i:])
                yield ("name", m.group(0), i, i + len(m.group(0)))
                i += len(m.group(0))
            elif c.isdigit():
                m = re.match(r"[0-9]+", t[i:])
                yield ("num", int(m.group(0)), i, i + len(m.group(0)))
                i += len(m.group(0))
            else:
                yield ("p", c, i, i + 1)
                i += 1

    def span(self, a, b):
        return (self.offsets[a], self.offsets[b])

    def peek(self, kind=None, value=None):
        if self.i >= len(self.toks):
            return None
        tok = self.toks[self.i]
        if kind and tok[0] != kind:
            return None
        if value is not None and tok[1] != value:
            return None
        return tok

    def take(self):
        tok = self.toks[self.i]
        self.i += 1
        return tok

    def statements(self):
        while self.peek():
            if self.peek("dir", "%doc"):
                self.take(), self.take(), self.take()
            elif self.peek("dir", "%start"):
                self.take()
                self.start = self.take()[1]
                self.take()
            else:
                name = self.take()
                doc = self.take()[1] if self.peek("str") else ""
                self.take()  # =
                body = self.choice()
                self.take()  # ;
                self.rules[name[1]] = dict(doc=doc, body=body, span=self.span(name[2], name[3]))

    def choice(self):
        alts = [self.sequence()]
        while self.peek("p", "|"):
            self.take()
            alts.append(self.sequence())
        if len(alts) == 1:
            return alts[0]
        return dict(kind="alt", items=alts, a=alts[0]["a"], b=alts[-1]["b"])

    def sequence(self):
        items = []
        while self.peek("name") or self.peek("str") or self.peek("p", "("):
            items.append(self.item())
        if len(items) == 1:
            return items[0]
        return dict(kind="seq", items=items, a=items[0]["a"], b=items[-1]["b"])

    def item(self):
        tok = self.take()
        if tok[0] == "name":
            node = dict(kind="ref", name=tok[1], a=tok[2], b=tok[3])
        elif tok[0] == "str":
            node = dict(kind="sym", value=tok[1], a=tok[2], b=tok[3])
        else:
            node = self.choice()
            node = dict(node, group=(tok[2], self.take()[3]))
        a = node.get("group", (node["a"],))[0]
        if self.peek("p", "?"):
            return dict(kind="opt", item=node, a=a, b=self.take()[3])
        if self.peek("p", "*") or self.peek("p", "+"):
            op = self.take()
            return dict(kind="rep", item=node, min=0 if op[1] == "*" else 1, max=None, a=a, b=op[3])
        if self.peek("p", "{"):
            self.take()
            lo = self.take()[1]
            hi = lo
            if self.peek("p", ","):
                self.take()
                hi = self.take()[1] if self.peek("num") else None
            end = self.take()
            return dict(kind="rep", item=node, min=lo, max=hi, a=a, b=end[3])
        return node

    # The facts that the drawing needs: rules for one step, tail and head references, loop links.

    def nullable(self, e):
        k = e["kind"]
        if k == "sym":
            return False
        if k == "ref":
            return self.null.get(e["name"], False)
        if k == "seq":
            return all(self.nullable(x) for x in e["items"])
        if k == "alt":
            return any(self.nullable(x) for x in e["items"])
        if k == "opt":
            return True
        return e["min"] == 0 or self.nullable(e["item"])

    def analyse(self):
        self.null = {}
        changed = True
        while changed:
            changed = False
            for name, r in self.rules.items():
                v = self.nullable(r["body"])
                if v != self.null.get(name, False):
                    self.null[name], changed = v, True
        self.tail_refs, self.head_refs = {}, {}
        for name, r in self.rules.items():
            self.walk(name, r["body"], True, True)
        self.links = {}
        for refs, kind in ((self.head_refs, "head-references"), (self.tail_refs, "tail-references")):
            edges = {}
            for key, (src, dst) in refs.items():
                edges.setdefault(src, set()).add(dst)
            for key, (src, dst) in refs.items():
                if self.reaches(edges, dst, src) and key not in self.links:
                    self.links[key] = kind

    def reaches(self, edges, a, b):
        seen, todo = set(), [a]
        while todo:
            x = todo.pop()
            if x == b:
                return True
            if x not in seen:
                seen.add(x)
                todo.extend(edges.get(x, ()))
        return False

    def walk(self, rule, e, tail, head):
        k = e["kind"]
        key = self.span(e["a"], e["b"])
        if k == "ref":
            if tail:
                self.tail_refs[key] = (rule, e["name"])
            if head:
                self.head_refs[key] = (rule, e["name"])
        elif k == "seq":
            first = True
            for i, x in enumerate(e["items"]):
                self.walk(rule, x, tail and i == len(e["items"]) - 1, head and first)
                first = first and self.nullable(x)
        elif k == "alt":
            for x in e["items"]:
                self.walk(rule, x, tail, head)
        elif k == "opt":
            self.walk(rule, e["item"], tail, head)
        elif k == "rep":
            self.walk(rule, e["item"], False, head)

    def one_step(self, e):
        k = e["kind"]
        if k == "sym":
            return True
        if k in ("alt",):
            return all(self.one_step(x) for x in e["items"])
        if k == "opt":
            return self.one_step(e["item"])
        return False

    def self_tail(self, rule, e):
        key = self.span(e["a"], e["b"])
        return e["kind"] == "ref" and e["name"] == rule and self.links.get(key) == "tail-references"


# ----------------------------------------------------------------------------
# Drawing primitives


def rounded(points, r=6):
    """An SVG path through orthogonal points, with rounded corners."""
    if len(points) < 3:
        return "M" + " L".join(f"{x:.1f} {y:.1f}" for x, y in points)
    d = [f"M{points[0][0]:.1f} {points[0][1]:.1f}"]
    for (x0, y0), (x1, y1), (x2, y2) in zip(points, points[1:], points[2:]):
        l1 = math.hypot(x1 - x0, y1 - y0)
        l2 = math.hypot(x2 - x1, y2 - y1)
        rr = min(r, l1 / 2, l2 / 2)
        ax = x1 - (x1 - x0) / l1 * rr if l1 else x1
        ay = y1 - (y1 - y0) / l1 * rr if l1 else y1
        bx = x1 + (x2 - x1) / l2 * rr if l2 else x1
        by = y1 + (y2 - y1) / l2 * rr if l2 else y1
        d.append(f"L{ax:.1f} {ay:.1f} Q{x1:.1f} {y1:.1f} {bx:.1f} {by:.1f}")
    d.append(f"L{points[-1][0]:.1f} {points[-1][1]:.1f}")
    return " ".join(d)


class Drawing:
    def __init__(self, grammar, rule):
        self.g, self.rule = grammar, rule
        self.out = []
        self.top = []  # Reading markers, drawn over every other mark.
        self.boxes = []  # dict(kind, key, label, x, y, w, h)
        self.loops = []  # dict(key, item, label, lx, ly, path)
        self.tails = []  # dict(key, name, sx, sy, path, lx, ly)
        self.gws = []  # dict(key, x, y)
        self.events = {}
        self.not_first = 0  # The count of choices around the current place, in a branch that is not their first.
        self.not_last = 0  # The count of choices around the current place, in a branch that is not their last.
        self.flows = []
        self.shapes = []  # (kind, data), in the order of placement

    def add(self, s):
        self.out.append(s)

    def flow(self, pts, arrow=True, cls="flow", role="sequence", key=None, label=None):
        end = ' marker-end="url(#ah)"' if arrow else ""
        self.add(f'<path d="{rounded(pts)}" class="{cls}"{end}/>')
        self.flows.append(dict(pts=pts, role=role, key=key, label=label))

    def gateway(self, x, y, key, role="split"):
        c = x + GW / 2
        h = GW / 2
        # An exclusive gateway without the X marker: BPMN makes the marker optional.
        self.add(f'<path d="M{x} {y} L{c} {y-h} L{x+GW} {y} L{c} {y+h}z" class="gw"/>')
        self.gws.append(dict(key=key, x=x, y=y, role=role))
        self.shapes.append(("gateway", dict(key=key, x=x, y=y - GW / 2, role=role)))

    def text_lines(self, lines, cx, cy):
        top = cy - len(lines) * LINE / 2
        for i, ln in enumerate(lines):
            self.add(
                f'<text x="{cx:.1f}" y="{top + i*LINE + 12:.1f}" class="{font.cls}" '
                f'text-anchor="middle" xml:space="preserve">{escape(ln)}</text>'
            )

    def label(self, x, y, text, cls="lbl"):
        w = mono_width(text) * 11 / 13 + 8
        self.add(f'<rect x="{x - w/2:.1f}" y="{y-8:.1f}" width="{w:.1f}" height="16" rx="3" class="lblbg"/>')
        self.add(f'<text x="{x:.1f}" y="{y+4:.1f}" class="{cls}" text-anchor="middle">{escape(text)}</text>')

    def find(self, kind, label, nth=0):
        hits = [b for b in self.boxes if b["kind"] == kind and b["label"] == label]
        return hits[nth]

    def find_loop(self, item, nth=0):
        return [lp for lp in self.loops if lp["item"] == item][nth]


# ----------------------------------------------------------------------------
# Layout: each element knows its width and its extents above and below the main line.


class Box:
    def __init__(self, d, node, kind, label, drill=False, link=None):
        self.node, self.kind, self.label, self.drill, self.link = node, kind, label, drill, link
        self.key = d.g.span(node["a"], node["b"])
        self.lines = wrap(label)
        tw = max(font.width(ln) for ln in self.lines)
        self.w = max(MIN_W, even(tw + 2 * PAD_X))
        self.h = max(MIN_H, even(len(self.lines) * LINE + 2 * PAD_Y)) + (DRILL if drill else 0)
        self.tail = False

    def size(self):
        return self.w, self.h / 2 + BAND, self.h / 2

    def place(self, d, x, y):
        top = y - self.h / 2
        cls = "task" if self.kind == "task" else "refbox"
        d.add(f'<rect x="{x}" y="{top:.1f}" width="{self.w}" height="{self.h}" rx="9" class="{cls}"/>')
        cy = y - (DRILL / 2 if self.drill else 0)
        d.text_lines(self.lines, x + self.w / 2, cy)
        if self.drill:
            mx, my = x + self.w / 2 - 6, top + self.h - 15
            d.add(f'<rect x="{mx:.1f}" y="{my:.1f}" width="12" height="12" class="drill"/>')
            d.add(f'<path d="M{mx+3:.1f} {my+6:.1f} L{mx+9:.1f} {my+6:.1f} M{mx+6:.1f} {my+3:.1f} L{mx+6:.1f} {my+9:.1f}" class="gwx"/>')
        if self.link:
            d.add(f'<text x="{x+7:.1f}" y="{top+self.h-6:.1f}" class="lbl">↻</text>')
        d.boxes.append(dict(kind=self.kind, key=self.key, label=self.label, x=x, y=top, w=self.w, h=self.h))
        d.shapes.append((self.kind, dict(d.boxes[-1], lines=self.lines, one_step=not self.drill, link=self.link)))


class Seq:
    def __init__(self, items):
        self.items = items
        self.tail = items[-1].tail

    def size(self):
        w = up = down = 0
        for i, it in enumerate(self.items):
            iw, iu, idn = it.size()
            gap = FLOW if i and not isinstance(it, TailRef) else 0
            w += iw + gap
            up, down = max(up, iu), max(down, idn)
        return w, up, down

    def place(self, d, x, y):
        cx = x
        for i, it in enumerate(self.items):
            iw = it.size()[0]
            if i and not isinstance(it, TailRef):
                d.flow([(cx, y), (cx + FLOW, y)])
                cx += FLOW
            it.place(d, cx, y)
            cx += iw


class Choice:
    def __init__(self, d, node, branches):
        self.key = d.g.span(node["a"], node["b"])
        self.branches = branches
        self.tail = all(b.tail for b in branches)

    def size(self):
        sizes = [b.size() for b in self.branches]
        self.bw = max(s[0] for s in sizes)
        self.offs = [0]
        for (pw, pu, pd), (bw, bu, bd) in zip(sizes, sizes[1:]):
            self.offs.append(self.offs[-1] + pd + VGAP + bu)
        w = GW + FLOW + self.bw + FLOW + GW
        return w, max(GW / 2, sizes[0][1]), max(GW / 2, self.offs[-1] + sizes[-1][2])

    def place(self, d, x, y):
        w = self.size()[0]
        d.gateway(x, y, self.key, "choice-split")
        bx, jx = x + GW + FLOW, x + w - GW
        live = [b for b in self.branches if not b.tail]
        for i, b in enumerate(self.branches):
            by = y + self.offs[i]
            b.place(d, bx, by)
            if i == 0:
                d.flow([(x + GW, y), (bx, y)])
            else:
                d.flow([(x + GW / 2, y + GW / 2), (x + GW / 2, by), (bx, by)])
            if b.tail:
                continue
            ex = bx + b.size()[0]
            if len(live) < 2:
                d.flow([(ex, by), (jx + GW / 2, by), (jx + GW / 2, y), (x + w, y)], arrow=False)
            elif i == 0:
                d.flow([(ex, y), (jx, y)])
            else:
                d.flow([(ex, by), (jx + GW / 2, by), (jx + GW / 2, y + GW / 2)])
        if len(live) >= 2:
            d.gateway(jx, y, self.key, "choice-join")


class Option:
    def __init__(self, d, node, item):
        self.key = d.g.span(node["a"], node["b"])
        self.item = item
        self.tail = False

    def size(self):
        iw, iu, idn = self.item.size()
        return GW + FLOW + iw + FLOW + GW, iu + SKIP, max(GW / 2, idn)

    def place(self, d, x, y):
        w, up, _ = self.size()
        iw = self.item.size()[0]
        d.gateway(x, y, self.key, "option-split")
        d.flow([(x + GW, y), (x + GW + FLOW, y)])
        self.item.place(d, x + GW + FLOW, y)
        jx = x + w - GW
        d.flow([(x + GW / 2, y - GW / 2), (x + GW / 2, y - up), (jx + GW / 2, y - up), (jx + GW / 2, y - GW / 2)],
               role="skip", key=self.key)
        d.flow([(x + GW + FLOW + iw, y), (jx, y)])
        d.gateway(jx, y, self.key, "option-join")


def tail_route(d):
    """The lane of a back edge from the current place, or None.

    A self tail call stands at the end of each sequence around it. So only the
    later branches of a choice around it lie below it, and only the earlier
    branches lie above it. The route down is clear in the last branch of each
    choice, and the route up is clear in the first branch of each choice.
    """
    if d.not_last == 0:
        return "down"
    if d.not_first == 0:
        return "up"
    return None


class OptionTail:
    """An option around a self tail call: the gateway sends the back edge to its lane, and the main line goes on."""

    def __init__(self, d, node, ref):
        self.key = d.g.span(node["a"], node["b"])
        self.ref = ref
        self.tail = False
        self.up = tail_route(d) == "up"

    def size(self):
        return GW, GW / 2, GW / 2

    def place(self, d, x, y):
        d.gateway(x, y, self.key, "option-split")
        sy = y - GW / 2 if self.up else y + GW / 2
        d.tails.append(dict(key=d.g.span(self.ref["a"], self.ref["b"]), name=self.ref["name"],
                            sx=x + GW / 2, sy=sy, pts=[], up=self.up))


class TailRef:
    """A self tail call in a row: the flow turns into the back-edge lane, below or above the body."""

    def __init__(self, d, node):
        self.node = node
        self.key = d.g.span(node["a"], node["b"])
        self.tail = True
        self.up = tail_route(d) == "up"

    def size(self):
        return FLOW, 0, 0

    def place(self, d, x, y):
        d.tails.append(dict(key=self.key, name=self.node["name"], sx=x + FLOW, sy=y, pts=[(x, y)], up=self.up))


def bound_label(node):
    lo, hi = node["min"], node["max"]
    if lo in (0, 1) and hi is None:
        return None
    if hi is None:
        return f"↻ {lo}+"
    if hi == lo:
        return f"↻ {lo}"
    return f"↻ {lo}–{hi}"


class Repetition:
    def __init__(self, d, node, item):
        self.node = node
        self.key = d.g.span(node["a"], node["b"])
        self.item = item
        self.star = node["min"] == 0
        self.text = bound_label(node)
        self.tail = False

    def size(self):
        iw, iu, idn = self.item.size()
        if self.star:
            w = 4 * GW + 2 * PAIR + 2 * FLOW + iw
            up = iu + SKIP
        else:
            w = 2 * GW + 2 * FLOW + iw
            up = max(GW / 2, iu)
        return w, up, idn + BACK + (9 if self.text else 2)

    def place(self, d, x, y):
        w, up, _ = self.size()
        iw, iu, idn = self.item.size()
        if self.star:
            d.gateway(x, y, self.key, "skip-split")
            jx = x + GW + PAIR
            d.flow([(x + GW, y), (jx, y)])
        else:
            jx = x
        d.gateway(jx, y, self.key, "loop-join")
        ix = jx + GW + FLOW
        d.flow([(jx + GW, y), (ix, y)])
        self.item.place(d, ix, y)
        sx = ix + iw + FLOW
        d.flow([(ix + iw, y), (sx, y)])
        d.gateway(sx, y, self.key, "loop-split")
        by = y + idn + BACK
        back = [(sx + GW / 2, y + GW / 2), (sx + GW / 2, by), (jx + GW / 2, by), (jx + GW / 2, y + GW / 2)]
        lx = (sx + jx + GW) // 2
        d.flow(back, role="back", key=self.key, label=(self.text, lx, by) if self.text else None)
        if self.star:
            ex = sx + GW + PAIR
            d.flow([(sx + GW, y), (ex, y)])
            d.gateway(ex, y, self.key, "skip-join")
            d.flow([(x + GW / 2, y - GW / 2), (x + GW / 2, y - up), (ex + GW / 2, y - up), (ex + GW / 2, y - GW / 2)],
                   role="skip", key=self.key)
        if self.text:
            d.label(lx, by, self.text)
        d.loops.append(dict(key=self.key, item=getattr(self.item, "label", None), text=self.text,
                            lx=lx, ly=by, path=back))


def build(d, rule, e):
    g = d.g
    k = e["kind"]
    if k == "sym":
        return Box(d, e, "task", e["value"])
    if k == "ref":
        # A self tail call without a clear route to a lane draws as a reference box with the loop sign.
        if g.self_tail(rule, e) and tail_route(d):
            return TailRef(d, e)
        target = g.rules.get(e["name"])
        drill = not (target and g.one_step(target["body"]))
        return Box(d, e, "ref", e["name"], drill=drill, link=g.links.get(g.span(e["a"], e["b"])))
    if k == "seq":
        return Seq([build(d, rule, x) for x in e["items"]])
    if k == "alt":
        branches = []
        for i, x in enumerate(e["items"]):
            first, last = i == 0, i == len(e["items"]) - 1
            d.not_first += not first
            d.not_last += not last
            branches.append(build(d, rule, x))
            d.not_first -= not first
            d.not_last -= not last
        return Choice(d, e, branches)
    if k == "opt":
        if g.self_tail(rule, e["item"]) and tail_route(d):
            return OptionTail(d, e, e["item"])
        return Option(d, e, build(d, rule, e["item"]))
    return Repetition(d, e, build(d, rule, e["item"]))


def layout(g, rule):
    """Lay out one rule diagram. Return the drawing with its size."""
    d = Drawing(g, rule)
    body = build(d, rule, g.rules[rule]["body"])
    bw, bu, bd = body.size()
    tails = [x for x in walk_elements(body) if isinstance(x, (TailRef, OptionTail))]
    has_tail = bool(tails)
    n_up = sum(1 for t in tails if t.up)
    y = MARGIN + max(bu, EV / 2) + (LANE * n_up + 8 if n_up else 0)
    top = y - max(bu, EV / 2)
    x = MARGIN
    d.add(f'<circle cx="{x + EV/2}" cy="{y}" r="{EV/2}" class="ev"/>')
    d.events["start"] = (x + EV / 2, y)
    d.shapes.append(("start-event", dict(x=x, y=y - EV / 2)))
    x += EV
    d.flow([(x, y), (x + FLOW, y)])
    x += FLOW
    jt = None
    if has_tail:
        jt = x
        d.gateway(x, y, g.rules[rule]["span"], "back-edge-join")
        x += GW
        d.flow([(x, y), (x + FLOW, y)])
        x += FLOW
    body.place(d, x, y)
    x += bw
    if not body.tail:
        d.flow([(x, y), (x + FLOW, y)])
        x += FLOW
        d.add(f'<circle cx="{x + EV/2}" cy="{y}" r="{EV/2 - 1.2}" class="evend"/>')
        d.events["end"] = (x + EV / 2, y)
        d.shapes.append(("end-event", dict(x=x, y=y - EV / 2)))
        x += EV
    bottom = y + max(bd, EV / 2)
    ups = downs = 0
    for t in d.tails:
        if t["up"]:
            ups += 1
            lane = top - LANE * ups
            end = (jt + GW / 2, y - GW / 2)
        else:
            downs += 1
            lane = bottom + LANE * downs
            end = (jt + GW / 2, y + GW / 2)
        pts = t["pts"][:1] + [(t["sx"], t["sy"]), (t["sx"], lane), (jt + GW / 2, lane), end]
        t["pts"] = pts
        t["lx"], t["ly"] = (t["sx"] + jt + GW // 2) // 2, lane
        d.flow(pts, role="back-edge", key=t["key"], label=(f"↻ {t['name']}", t["lx"], lane))
        d.label(t["lx"], lane, f"↻ {t['name']}")
    if downs:
        bottom += LANE * downs + 8
    d.w = x + MARGIN
    d.h = bottom + MARGIN
    d.main_y = y
    return d


def walk_elements(el):
    yield el
    for attr in ("items", "branches"):
        for x in getattr(el, attr, []):
            yield from walk_elements(x)
    if hasattr(el, "item"):
        yield from walk_elements(el.item)


# ----------------------------------------------------------------------------
# Marks of a trace


def tabs(d, box, readings):
    """Numbered tabs on the top edge of a box: one for each reading that stands there."""
    x = box["x"] + 6
    for n, on_view in readings:
        y = box["y"] - 8
        cls, tcls = ("tab", "tabt") if on_view else ("tabo", "tabto")
        d.top.append(f'<path d="M{x} {y-3} h20 v13 l-10 5 l-10 -5z" class="{cls}"/>')
        d.top.append(f'<text x="{x+10}" y="{y+8.5}" class="{tcls}" text-anchor="middle">{n}</text>')
        x += 23


def event_tab(d, which, readings):
    cx, cy = d.events[which]
    box = dict(x=cx - 15, y=cy - EV / 2 - 2)
    tabs(d, box, readings)


def next_mark(d, box, on_view=True, button=True):
    cls = "nextmark" if on_view else "nextmarko"
    d.add(f'<rect x="{box["x"]-4}" y="{box["y"]-4:.1f}" width="{box["w"]+8}" height="{box["h"]+8}" rx="12" class="{cls}"/>')
    if button:
        bx, by = box["x"] + box["w"] - 14, box["y"] - 9
        d.add(f'<rect x="{bx}" y="{by:.1f}" width="18" height="15" rx="4" class="nextbtn"/>')
        d.add(f'<text x="{bx+9}" y="{by+11:.1f}" class="nextbtnt" text-anchor="middle">▸</text>')


def end_mark(d, which="end", text=None, on_view=True):
    cx, cy = d.events[which]
    cls = "nextmark" if on_view else "nextmarko"
    d.add(f'<circle cx="{cx}" cy="{cy}" r="{EV/2 + 5}" class="{cls}"/>')
    bx, by = cx + 6, cy - EV / 2 - 10
    d.add(f'<rect x="{bx}" y="{by:.1f}" width="18" height="15" rx="4" class="nextbtn"/>')
    d.add(f'<text x="{bx+9}" y="{by+11:.1f}" class="nextbtnt" text-anchor="middle">▸</text>')
    if text:
        d.add(f'<text x="{cx + EV/2 + 10}" y="{cy + 4:.1f}" class="cur sm">{escape(text)}</text>')
        d.w = max(d.w, cx + EV / 2 + 10 + text_w(text, 11) + 8)


def back_mark(d, pts):
    d.add(f'<path d="{rounded(pts)}" class="backmark"/>')


def pass_label(d, lx, ly, text):
    w = mono_width(text) * 11 / 13 + 12
    d.add(f'<rect x="{lx - w/2:.1f}" y="{ly-9:.1f}" width="{w:.1f}" height="18" rx="9" class="passbg"/>')
    d.add(f'<text x="{lx:.1f}" y="{ly+4:.1f}" class="passt" text-anchor="middle">{escape(text)}</text>')


def select(d, box):
    d.add(f'<rect x="{box["x"]-7}" y="{box["y"]-7:.1f}" width="{box["w"]+14}" height="{box["h"]+14}" rx="14" class="sel"/>')


def select_loop(d, loop):
    d.add(f'<path d="{rounded(loop["path"])}" class="selflow"/>')
    for gw in d.gws:
        if gw["key"] == loop["key"]:
            c = gw["x"] + GW / 2
            d.add(f'<path d="M{gw["x"]-4} {gw["y"]} L{c} {gw["y"]-16} L{gw["x"]+GW+4} {gw["y"]} L{c} {gw["y"]+16}z" class="sel"/>')


def data_object(d, box, count):
    x = box["x"] + box["w"] / 2 - 7
    y = box["y"] - BAND + 2
    d.add(f'<path d="M{x} {y} h10 l4 4 v13 h-14z" class="data"/>')
    d.add(f'<path d="M{x+10} {y} v4 h4" class="data"/>')
    d.add(f'<text x="{x + 20}" y="{y + 13}" class="m sm">{count}</text>')
    d.add(f'<path d="M{x+7} {y+17} L{x+7} {box["y"]}" class="assoc"/>')


def popover(d, x, y, title, chips):
    w = max(mono_width(title) * 0.9, sum(mono_width(c) * 11 / 13 + 20 for c in chips)) + 16
    d.h = max(d.h, y + 52)
    d.add(f'<rect x="{x}" y="{y}" width="{w:.1f}" height="44" rx="6" class="pop"/>')
    d.add(f'<text x="{x+8}" y="{y+15}" class="m sm">{escape(title)}</text>')
    cx = x + 8
    for c in chips:
        cw = mono_width(c) * 11 / 13 + 14
        d.add(f'<rect x="{cx:.1f}" y="{y+21}" width="{cw:.1f}" height="17" rx="8" class="chip"/>')
        d.add(f'<text x="{cx+7:.1f}" y="{y+33}" class="chipt">{escape(c)}</text>')
        cx += cw + 6


def callout(d, x, y, n):
    d.add(f'<circle cx="{x:.1f}" cy="{y:.1f}" r="9" class="call"/>')
    d.add(f'<text x="{x:.1f}" y="{y+4:.1f}" class="callt" text-anchor="middle">{n}</text>')


# ----------------------------------------------------------------------------
# Panels and pages


def text_w(s, size=12):
    return len(s) * size * 0.55


def wrap_note(text, width, size=12):
    words, lines, cur = text.split(), [], ""
    for wd in words:
        if cur and text_w(cur + " " + wd, size) > width:
            lines.append(cur)
            cur = wd
        else:
            cur = (cur + " " + wd).strip()
    if cur:
        lines.append(cur)
    return lines


class Page:
    def __init__(self, width):
        self.w = width
        self.items = []
        self.y = 0

    def add(self, s):
        self.items.append(s)

    def reach(self, x):
        self.w = max(self.w, int(x + 16))

    def title(self, text, sub=None):
        self.add(f'<text x="16" y="{self.y + 26}" class="h1">{escape(text)}</text>')
        self.y += 34
        if sub:
            for ln in wrap_note(sub, self.w - 32):
                self.add(f'<text x="16" y="{self.y + 12}" class="m">{escape(ln)}</text>')
                self.y += 17
        self.y += 6

    def notes(self, x, y, notes, width):
        """Numbered notes. Return the bottom y."""
        for n, text in notes:
            callout(self, x + 9, y + 9, n) if n else None
            for i, ln in enumerate(wrap_note(text, width - 26)):
                self.add(f'<text x="{x + 26}" y="{y + 13 + i*16}" class="note">{escape(ln)}</text>')
            y += 16 * len(wrap_note(text, width - 26)) + 10
        return y

    def panel(self, d, crumbs=None, badge=None, sub=None, banner=None, notes=(), x=16, width=None,
              notes_w=300, advance=True, y=None):
        """Draw one rule diagram in a frame, with its breadcrumbs, header and notes."""
        y0 = self.y if y is None else y
        width = width or (d.w + 24)
        head = 30 + (17 if sub else 0)
        bh = 0
        if banner:
            blines = wrap_note(banner, width - 30)
            bh = 16 * len(blines) + 14
        h = head + bh + d.h + 8
        self.add(f'<rect x="{x}" y="{y0}" width="{width}" height="{h}" rx="8" class="panel"/>')
        crumbs = crumbs or [d.rule]
        cx = x + 12
        for i, c in enumerate(crumbs):
            last = i == len(crumbs) - 1
            cls = "b" if last else "crumb"
            self.add(f'<text x="{cx}" y="{y0 + 20}" class="{cls}">{escape(c)}</text>')
            cx += text_w(c, 12.5) + 6
            if not last:
                self.add(f'<text x="{cx}" y="{y0 + 20}" class="m">›</text>')
                cx += 14
        if badge:
            bw = text_w(badge, 11) + 14
            bx = x + width - bw - 10
            self.add(f'<rect x="{bx}" y="{y0 + 8}" width="{bw}" height="17" rx="8" class="badge"/>')
            self.add(f'<text x="{bx + 7}" y="{y0 + 20}" class="m sm">{escape(badge)}</text>')
        if sub:
            self.add(f'<text x="{x + 12}" y="{y0 + 38}" class="m sm">{escape(sub)}</text>')
        if banner:
            by = y0 + head
            self.add(f'<rect x="{x + 8}" y="{by}" width="{width - 16}" height="{bh - 6}" rx="5" class="banner"/>')
            for i, ln in enumerate(blines):
                self.add(f'<text x="{x + 16}" y="{by + 15 + i*16}" class="e">{escape(ln)}</text>')
        gy = y0 + head + bh
        self.add(f'<g transform="translate({x + 12},{gy})">' + "\n".join(d.out + d.top) + "</g>")
        bottom = y0 + h
        self.reach(x + width)
        if notes:
            nb = self.notes(x + width + 14, y0 + 4, notes, notes_w)
            bottom = max(bottom, nb)
            self.reach(x + width + 14 + notes_w)
        if advance:
            self.y = bottom + 16
        return bottom

    def save(self, path):
        h = int(self.y + 10)
        body = "\n".join(self.items)
        Path(path).write_text(
            f'<svg xmlns="http://www.w3.org/2000/svg" width="{self.w}" height="{h}" viewBox="0 0 {self.w} {h}">\n'
            f"<style>{STYLE}</style>\n"
            '<defs><marker id="ah" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" '
            'orient="auto-start-reverse"><path d="M0 0L10 5L0 10z" class="ah"/></marker></defs>\n'
            f'<rect width="100%" height="100%" style="fill:var(--bg)"/>\n{body}\n</svg>\n'
        )


def load(name):
    return Grammar((ROOT / "examples" / f"{name}.ebnf").read_text())


# ----------------------------------------------------------------------------
# The mock-ups


CATALOGUE = [
    ("Symbol and sequence: task boxes in a row", 'r = "submit_order" "check_order" "ship" ;', "r"),
    ("Choice: a pair of exclusive gateways", 'r = "approve" | "escalate" | "reject" ;', "r"),
    ("Optional part: a skip flow above the item", 'r = "remind"? "approve" ;', "r"),
    ("One or more: a back flow below the item", 'r = "install"+ ;', "r"),
    ("Zero or more: a skip flow and a back flow", 'r = "set_exercise" "give_hint"* "show_solution" ;', "r"),
    ("Bound: the loop marker with its numbers", 'r = "show_example"{1,3} ;', "r"),
    ("Bound from 0: the skip flow and the numbers", 'r = "remind"{0,2} "approve" ;', "r"),
    ("Reference: a thick box with a drill-down marker", 'r = "submit_order" revision "ship" ;\nrevision = "request_changes" "revise_order" ;', "r"),
    ("Reference to a rule for one step: a thick task box", 'r = overview lesson ;\noverview = "overview" ;\nlesson = "introduce_concept" "show_example" ;', "r"),
    ("Self tail call: a back edge to the start", 'refinement = "identifies a gap" "revises the solution" refinement? ;', "refinement"),
    ("Self tail call in a choice", 'menu = "show_menu" ("choose_item" menu | "quit") ;', "menu"),
    ("Self tail call in a middle branch: a link with the loop sign",
     'menu = "show_menu" ("choose_item" menu | "search" menu | "quit") ;', "menu"),
    ("Loop through two rules: a link with the loop sign", 'question = "asks" answer ;\nanswer = "answers" question? ;', "answer"),
    ("Left recursion: a link with the loop sign", 'list = list "item" | "first_item" ;', "list"),
]


def mock_constructs(out):
    global font
    font = MONO
    p = Page(1180)
    p.title(
        "One drawing for each construct",
        "Each panel shows one rule and its text. Flows run from left to right. The first alternative stays on the "
        "main line, skip flows run above an item and back flows below it. Agreed on 2026-11-20.",
    )
    x = 16
    row_top = row_bottom = p.y
    for title, text, rule in CATALOGUE:
        g = Grammar("%start " + rule + " ;\n" + text)
        d = layout(g, rule)
        lines = text.split("\n")
        width = max(d.w + 24, max(len(ln) for ln in lines) * 7.3)
        if x > 16 and x + width > 1180:
            x = 16
            row_top = row_bottom + 22
        p.add(f'<text x="{x}" y="{row_top + 12}" class="b">{escape(title)}</text>')
        for j, ln in enumerate(lines):
            p.add(f'<text x="{x}" y="{row_top + 30 + j*15}" class="mono m">{escape(ln)}</text>')
        ty = row_top + 22 + 15 * len(lines)
        bottom = p.panel(d, crumbs=[rule], x=x, y=ty, width=width, advance=False)
        row_bottom = max(row_bottom, bottom)
        x += width + 24
    p.y = row_bottom + 10
    p.save(out / "mock-constructs.svg")


def mock_examples(out):
    global font
    font = MONO
    sets = [
        ("order-approval", ["order", "outcome", "approval"], {"approval": "may need a stack"}, "mock-order.svg",
         "The order-approval grammar: one diagram for each rule"),
        ("tutorial", ["tutorial", "setup", "lesson", "practice"], {}, "mock-tutorial.svg",
         "The tutorial grammar: references to rules for one step look like tasks"),
        ("urg-moves", ["episode", "development", "progression", "fix", "refinement", "generalisation"],
         {"refinement": "no self-embedding"}, "mock-urg.svg",
         "The URG move grammar: a self tail call draws as a back edge"),
    ]
    for name, rules, badges, file, title in sets:
        g = load(name)
        p = Page(1100)
        p.title(title, f"Drawn by the agreed layout from examples/{name}.ebnf. The breadcrumbs show the open path from the start rule.")
        for r in rules:
            d = layout(g, r)
            doc = g.rules[r]["doc"]
            sub = doc if len(doc) < 120 else doc[:117] + "…"
            p.panel(d, crumbs=open_path(g, r), badge=badges.get(r), sub=sub)
        p.save(out / file)


GROVE_SOURCES = {
    "grove": "the grove spine", "work": "the grove spine", "planning": "the grove spine",
    "requirements": "the requirements source", "research": "the research source",
    "design": "the design source", "prototype": "the engineering source", "impl": "the engineering source",
}


def load_grove():
    return Grammar((Path(__file__).resolve().parent / "grove-sessions.ebnf").read_text())


def short(text, n=110):
    return text if len(text) <= n else text[: n - 1] + "…"


def mock_grove(out):
    """A guess at the grammar of the sessions of a grove, for the look of a workflow grammar."""
    global font
    font = MONO
    g = load_grove()
    p = Page(1100)
    p.title(
        "Sketch: a grammar of the sessions of a grove",
        "Drawn by the agreed layout from grove-sessions.ebnf in this directory. The grammar is a guess from the live "
        "groves, for the look of a workflow grammar only. Each panel names the skill source that could contribute its rule.",
    )
    notes = {
        "grove": [(1, "work is the point of extension. The loop runs one pass for each piece of work, and finish closes the grove.")],
        "work": [(2, "Each skill source adds the alternatives of its own kinds. A new kind skill adds one reference box here, and no other rule changes.")],
        "design": [
            (3, "X, then review-X, then integrate-review-X. The loop holds each review that found something, with its integration."),
            (4, "planning, requirements, prototype and impl repeat this shape with their own kinds, so each chain draws the same way."),
        ],
        "research": [(5, "The pair of surveys. The research source cuts the three leaves at once, so the second survey cannot inherit the framing of the first.")],
    }
    for r in ["grove", "work", "design", "research"]:
        d = layout(g, r)
        if r == "grove":
            b = d.find("ref", "work")
            callout(d, b["x"] - 18, b["y"] - 4, 1)
        elif r == "work":
            b = d.find("ref", "requirements")
            callout(d, b["x"] + b["w"] + 14, b["y"] + 4, 2)
        elif r == "design":
            b = d.find("task", "review-design")
            callout(d, b["x"] - 18, b["y"] - 4, 3)
            ex, ey = d.events["end"]
            callout(d, ex + 4, ey - 24, 4)
        else:
            b = d.find("task", "research-b")
            callout(d, b["x"] - 18, b["y"] - 4, 5)
        sub = short(f"From {GROVE_SOURCES[r]}. {g.rules[r]['doc']}")
        width = max(d.w + 24, int(text_w(sub, 11)) + 30)
        p.panel(d, crumbs=open_path(g, r), sub=sub, notes=notes[r], width=width)
    p.save(out / "mock-grove.svg")


def mock_grove_trace(out):
    """This grove as a sequence of the kinds of its sessions, at its current session."""
    global font
    font = MONO
    g = load_grove()
    p = Page(1160)
    p.title(
        "Sketch: this grove as a trace, at session 32",
        "The sequence is the kind of each session of this grove so far: requirements · planning · review-planning · "
        "design · review-design · … · impl · review-impl · integrate-review-impl · design. The last step is this session.",
    )
    d = layout(g, "grove")
    w = d.find("ref", "work")
    tabs(d, w, [(1, True)])
    next_mark(d, w)
    loop = d.find_loop("work")
    back_mark(d, loop["path"])
    pass_label(d, loop["lx"], loop["ly"], "pass 12")
    next_mark(d, d.find("task", "finish"))
    callout(d, w["x"] - 18, w["y"] - 4, 1)
    callout(d, loop["lx"] - 52, loop["ly"], 2)
    p.panel(d, crumbs=["grove"], sub="One level up from the view that follows the reading.", notes=[
        (1, "The reading stands inside work. The mark on work says that the next session can stay in this piece of work."),
        (2, "Pass 12 of work: eleven pieces of work came before it. A new pass can start any kind of work, or finish can end the grove."),
    ])
    d = layout(g, "design")
    ds = d.find("task", "design")
    tabs(d, ds, [(1, True)])
    r1 = d.find("task", "review-design", 0)
    r2 = d.find("task", "review-design", 1)
    next_mark(d, r1)
    next_mark(d, r2)
    end_mark(d, text="then the next work, or finish")
    callout(d, ds["x"] - 18, ds["y"] - 4, 3)
    callout(d, r2["x"] + r2["w"] / 2, r2["y"] - 30, 4)
    p.panel(d, crumbs=["grove", "work", "design"], sub="The view that follows the reading, at this session: rule-diagram-design-k12.", notes=[
        (3, "The grammar view follows the reading into design. Its marker sits on the task box of this session."),
        (4, "Both review-design boxes carry a mark. The next review can start a round, or it can be the last review. The end event says that new work can also come next."),
    ])
    d = layout(g, "design")
    r1 = d.find("task", "review-design", 0)
    r2 = d.find("task", "review-design", 1)
    tabs(d, r1, [(1, True)])
    tabs(d, r2, [(2, False)])
    integ = d.find("task", "integrate-review-design")
    next_mark(d, integ)
    end_mark(d, on_view=False, text="reading 2: the next work, or finish")
    callout(d, r1["x"] - 18, r1["y"] - 4, 5)
    callout(d, r2["x"] - 18, r2["y"] - 4, 6)
    p.panel(d, crumbs=["grove", "work", "design"], sub="One session later, after the review of this design: two readings.", notes=[
        (5, "In reading 1, the review found something, so integrate-review-design comes next. The output view shows this reading."),
        (6, "In reading 2, the review found nothing and ended the chain. The next session decides between the two readings."),
    ])
    p.save(out / "mock-grove-trace.svg")


def mock_grove_abandoned(out):
    """Abandoned leaves as withdrawn steps: a real case, and a review that a new review replaces."""
    global font
    font = MONO
    g = load_grove()
    p = Page(1160)
    p.title(
        "Sketch: abandoned work as a withdrawn step",
        "A leaf that the human abandoned stays in the sequence as a withdrawn step. The views show it, and the tracer "
        "does not read it, so the grammar needs no rule for abandonment. Withdrawn steps wait until after the slice.",
    )
    p.add(f'<text x="16" y="{p.y + 12}" class="b">A real case: one node of the BookScanBuilder grove</text>')
    p.y += 20
    outline_rows(p, [
        ("1", 0, 'impl · "impl"', "step", None, "stream-inventory-k387"),
        ("2", 0, 'impl · "impl"', "step", None, "availability-k388"),
        ("3", 0, 'impl · "impl"', "step", None, "readers-k391"),
        ("4", 0, 'impl · "impl"', "step", None, "checker-k392"),
        ("5", 0, '"impl"', "withdrawn", "withdrawn", "semantic-controls-k389"),
        (None, 0, "impl", "rule", None, None),
        ("6", 1, '"impl"', "step", None, "angular-costs-k390"),
        (None, 1, '("review-impl" "integrate-review-impl")*', "loop", "pass 1", None),
        ("7", 2, '"review-impl"', "step", None, "angular-costs-k393"),
        ("8", 2, '"integrate-review-impl"', "step", None, "angular-costs-k394"),
    ], caption="Output view, roughly: the node level-music-paper-arms-angular-costs, with its folds open", width=600, notes=[
        (1, "Each chip is the leaf of the step, an attribute that the grammar never reads. The rows number the steps from the start of the node, and the leaf names drop the prefix of the node."),
        (2, "The node holds nine leaves in this order. The human abandoned the fifth, semantic-controls, and the sixth, angular-costs, took its place with a review."),
    ], marks={0: 1, 4: 2})
    p.add(f'<text x="16" y="{p.y + 12}" class="b">A review that a review with changed content replaces</text>')
    p.y += 20
    outline_rows(p, [
        (None, 0, "work*", "loop", "pass 12", None),
        ("2–31", 1, "completed · 11 passes · 30 steps", "fold", None, None),
        (None, 1, "pass 12", "pass", None, None),
        (None, 2, "design", "rule", None, None),
        ("32", 3, '"design"', "step", None, "rule-diagram-design-k12"),
        ("33", 3, '"review-design"', "withdrawn", "withdrawn", "rule-diagram-design-k37"),
        (None, 3, '("review-design" "integrate-review-design")*', "loop", "pass 1", None),
        ("34", 4, '"review-design"', "step", None, "rule-diagram-design-k38"),
        ("35", 4, '"integrate-review-design"', "step", None, "rule-diagram-design-k39"),
    ], caption="Output view, roughly: this grove, if the human abandons the review of this design", width=600, notes=[
        (3, "Step 33 keeps its number and its place, struck through. It belongs to no node, as the rejected step at a failure point does."),
        (4, "The replacement review, step 34, fits the grammar, so the tree stays well formed. A click on row 33 outlines the boxes that it matched in the state before it."),
    ], marks={5: 3, 7: 4})
    d = layout(g, "design")
    integ = d.find("task", "integrate-review-design")
    tabs(d, integ, [(1, True)])
    r1 = d.find("task", "review-design", 0)
    r2 = d.find("task", "review-design", 1)
    loop = d.loops[0]
    back_mark(d, loop["path"])
    pass_label(d, loop["lx"], loop["ly"], "pass 1")
    next_mark(d, r1, button=False)
    next_mark(d, r2)
    end_mark(d, text="then the next work, or finish")
    callout(d, integ["x"] - 18, integ["y"] - 4, 5)
    p.panel(d, crumbs=["grove", "work", "design"], sub="The grammar view of the same state, after step 35.", notes=[
        (5, "The grammar view shows the state of the steps that stand. The withdrawn review leaves no mark here."),
    ])
    p.save(out / "mock-grove-abandoned.svg")


def mock_text_mode(out):
    """The tutorial in a text mode of the grammar view, with one part open as a diagram."""
    global font
    font = MONO
    g = load("tutorial")
    p = Page(1200)
    p.title(
        "Sketch: a rule as text, with parts that open as diagrams",
        "The tutorial after overview · install · introduce_concept · show_example · set_exercise · give_hint. Each item of "
        "the rule is one line. Text mode comes after the slice. The human described it as program text: whole constructs "
        "with implicit structure, and embedded diagrams for some parts. This first sketch draws more structure than that.",
    )
    x0, y0, W = 16, p.y, 1010
    doc = lambda r: re.sub(r"\s+", " ", g.rules[r]["doc"]).strip()
    lines = [
        (0, None, "overview", "", doc("overview"), False),
        (0, "▸", "setup", "", doc("setup"), "closed"),
        (0, "▾", "lesson", "1 or more · pass 1", doc("lesson"), "text"),
        (1, None, "introduce_concept", "", doc("introduce_concept"), False),
        (1, None, "show_example", "1 to 3", doc("show_example"), False),
        (1, "▾", "practice", "0 or more · pass 1", doc("practice"), "diagram"),
        (1, None, "check_understanding", "optional", doc("check_understanding"), False),
        (0, None, "summarise", "", doc("summarise"), False),
        (0, None, "suggest_next_steps", "optional", doc("suggest_next_steps"), False),
    ]
    body = []
    ly = y0 + 70
    doc_x = x0 + 420
    rows_y = {}
    island = None
    for i, (depth, toggle, name, quant, text, opened) in enumerate(lines):
        rows_y[name] = ly
        tx = x0 + 40 + 20 * depth
        if toggle:
            body.append(f'<text x="{tx}" y="{ly}" class="m">{toggle}</text>')
        nx = tx + 16
        body.append(f'<text x="{nx}" y="{ly}" class="{"mono b" if opened else "mono"}">{escape(name)}</text>')
        if quant:
            body.append(f'<text x="{nx + len(name) * 7.2 + 10:.1f}" y="{ly}" class="lbl">{escape(quant)}</text>')
        room = int((x0 + W - 110 - doc_x) / 6.1)
        body.append(f'<text x="{doc_x}" y="{ly}" class="m sm">{escape(short(text, room))}</text>')
        if opened:
            bx = x0 + W - 92
            active = opened == "diagram"
            body.append(f'<rect x="{bx}" y="{ly - 13}" width="76" height="18" rx="9" class="{"chip" if active else "badge"}"/>')
            body.append(f'<text x="{bx + 38}" y="{ly}" class="{"chipt" if active else "lbl"}" text-anchor="middle">◇ diagram</text>')
        ly += 26
        if opened == "diagram":
            d = layout(g, name)
            st = d.find("ref", "give_hint")
            tabs(d, st, [(1, True)])
            next_mark(d, st)
            lp = d.find_loop("give_hint")
            back_mark(d, lp["path"])
            pass_label(d, lp["lx"], lp["ly"], "pass 1")
            next_mark(d, d.find("ref", "show_solution"))
            ix, iy = nx, ly - 8
            body.append(f'<rect x="{ix}" y="{iy}" width="{d.w + 8}" height="{d.h + 26}" rx="8" class="panel"/>')
            body.append(f'<text x="{ix + 10}" y="{iy + 17}" class="m sm">{escape(name)} · diagram</text>')
            body.append(f'<g transform="translate({ix + 4},{iy + 22})">' + "\n".join(d.out + d.top) + "</g>")
            island = (ix, iy, st)
            ly = iy + d.h + 26 + 26
    # The bar of the open loop lesson+: from its first line to its last line.
    top, bottom = rows_y["introduce_concept"] - 16, rows_y["check_understanding"] + 6
    bar_x = x0 + 46
    body.append(f'<path d="M{bar_x} {top} V{bottom}" class="loopbar"/>')
    h = ly - y0 - 6
    p.add(f'<rect x="{x0}" y="{y0}" width="{W}" height="{h}" rx="8" class="panel"/>')
    p.add(f'<text x="{x0 + 12}" y="{y0 + 20}" class="b">tutorial</text>')
    p.add(f'<text x="{x0 + 12}" y="{y0 + 38}" class="m sm">{escape(doc("tutorial"))}</text>')
    sx = x0 + W - 150
    p.add(f'<rect x="{sx}" y="{y0 + 8}" width="138" height="20" rx="10" class="badge"/>')
    p.add(f'<rect x="{sx + 2}" y="{y0 + 10}" width="58" height="16" rx="8" class="chip"/>')
    p.add(f'<text x="{sx + 31}" y="{y0 + 22}" class="chipt" text-anchor="middle">text</text>')
    p.add(f'<text x="{sx + 98}" y="{y0 + 22}" class="lbl" text-anchor="middle">diagrams</text>')
    p.add("\n".join(body))
    callout(p, x0 + 14, rows_y["overview"] - 4, 1)
    callout(p, x0 + 14, rows_y["lesson"] - 4, 2)
    callout(p, x0 + 14, rows_y["practice"] - 4, 3)
    callout(p, x0 + 14, rows_y["setup"] - 4, 4)
    callout(p, sx - 14, y0 + 18, 5)
    p.reach(x0 + W)
    nb = p.notes(x0 + W + 14, y0 + 4, [
        (1, "Each item of the rule is one line, in the order of the text. A rule for one step shows its doc string, so the rule reads like the plan of a tutorial."),
        (2, "lesson is open as text, like a fold in a code editor. A repetition shows its bound in words and its current pass. The bar marks the lines of the loop."),
        (3, "practice is open as a diagram: a part to inspect. Its marks are those of the diagram view. The diagram button of a line opens or closes its diagram."),
        (4, "setup stays closed. Its ▸ opens it as text, and its diagram button opens it as a diagram."),
        (5, "Text or diagrams: whether the grammar or the person chooses stays open, as for the text presentation of the output view."),
        (None, "The reading marker sits on the innermost element in view that holds the reading. Here that is give_hint, inside the diagram of practice."),
    ], 330)
    p.reach(x0 + W + 14 + 330)
    p.y = max(y0 + h, nb) + 16
    p.save(out / "mock-text-mode.svg")


def outline_rows(p, rows, caption, width=560, notes=(), marks=None):
    """A rough sketch of output-view rows: (gutter, depth, label, kind, right side, chip)."""
    marks = marks or {}
    x0, y0 = 16, p.y + 14
    h = 12 + 22 * len(rows) + 8
    p.add(f'<rect x="{x0}" y="{y0}" width="{width}" height="{h}" rx="8" class="panel"/>')
    p.add(f'<text x="{x0 + 12}" y="{y0 - 6}" class="m sm">{escape(caption)}</text>')
    for i, (gutter, depth, label, kind, right, chip) in enumerate(rows):
        y = y0 + 26 + 22 * i
        if gutter:
            p.add(f'<text x="{x0 + 44}" y="{y}" class="lbl" text-anchor="end">{escape(gutter)}</text>')
        tx = x0 + 56 + 16 * depth
        cls = {"rule": "mono b", "pass": "mono m", "fold": "mono m", "withdrawn": "mono m"}.get(kind, "mono")
        p.add(f'<text x="{tx}" y="{y}" class="{cls}">{escape(label)}</text>')
        lw = len(label) * 7.2
        if kind == "withdrawn":
            p.add(f'<path d="M{tx} {y - 4} h{lw:.1f}" class="strike"/>')
        if chip:
            cx = tx + lw + 10
            cw = mono_width(chip) * 11 / 13 + 14
            p.add(f'<rect x="{cx:.1f}" y="{y - 12}" width="{cw:.1f}" height="17" rx="8" class="chip"/>')
            p.add(f'<text x="{cx + 7:.1f}" y="{y}" class="chipt">{escape(chip)}</text>')
        if right:
            rcls = "lbl" if kind != "withdrawn" else "cur sm"
            p.add(f'<text x="{x0 + width - 12}" y="{y}" class="{rcls}" text-anchor="end">{escape(right)}</text>')
        if i in marks:
            callout(p, x0 + 14, y - 4, marks[i])
    bottom = y0 + h
    if notes:
        bottom = max(bottom, p.notes(x0 + width + 14, y0 + 4, notes, 340))
    p.reach(x0 + width + 14 + 340)
    p.y = bottom + 22


def open_path(g, rule):
    """The first shortest chain of references from the start rule."""
    paths = {g.start: [g.start]}
    todo = [g.start]
    while todo:
        cur = todo.pop(0)
        for ref in refs_of(g.rules[cur]["body"]):
            if ref not in paths and ref in g.rules:
                paths[ref] = paths[cur] + [ref]
                todo.append(ref)
    return paths.get(rule, [rule])


def refs_of(e):
    if e["kind"] == "ref":
        yield e["name"]
    for x in e.get("items", []):
        yield from refs_of(x)
    if "item" in e:
        yield from refs_of(e["item"])


def mock_markers(out):
    """Order approval after submit_order, check_order and remind: two readings."""
    global font
    font = MONO
    g = load("order-approval")
    p = Page(1160)
    p.title(
        "Reading markers, after submit_order · check_order · remind",
        "Two readings: the reminder can belong to an approval or to a rejection. The output view shows reading 1. "
        "Each panel is one state of the grammar view, with a different open path.",
    )
    d = layout(g, "outcome")
    tabs(d, d.find("ref", "approval"), [(1, True)])
    tabs(d, d.find("ref", "rejection"), [(2, False)])
    next_mark(d, d.find("ref", "approval"), on_view=True)
    next_mark(d, d.find("ref", "rejection"), on_view=False)
    b = d.find("ref", "approval")
    callout(d, b["x"] - 18, b["y"] - 4, 1)
    b2 = d.find("ref", "rejection")
    callout(d, b2["x"] - 18, b2["y"] - 4, 2)
    p.panel(d, crumbs=["order", "outcome"], notes=[
        (1, "Reading 1 stands inside approval. Its marker, a numbered tab, sits on the reference box. The filled tab is the reading on view."),
        (2, "Reading 2 stands inside rejection. An outlined tab is a reading that the output view does not show. A dashed outline marks next options that only other readings allow."),
    ])
    d = layout(g, "approval")
    remind = d.find("task", "remind")
    tabs(d, remind, [(1, True)])
    next_mark(d, remind)
    loop = d.find_loop("remind")
    back_mark(d, loop["path"])
    pass_label(d, loop["lx"], loop["ly"], "pass 1 · ↻ 0–2")
    next_mark(d, d.find("task", "approve"))
    next_mark(d, d.find("task", "escalate"))
    callout(d, remind["x"] - 18, remind["y"] - 4, 3)
    callout(d, loop["lx"] - 70, loop["ly"], 4)
    a = d.find("task", "approve")
    callout(d, a["x"] + a["w"] + 14, a["y"] + 4, 5)
    p.panel(d, crumbs=["order", "outcome", "approval"], badge="may need a stack",
            sub="Reading 2 stands outside this diagram.", notes=[
                (3, "The grammar view follows the reading on view into approval. Its marker sits on the task box of the step just read."),
                (4, "The loop shows only its current pass: pass 1 of 0 to 2. Another reminder starts pass 2, so the back flow carries the next-option colour."),
                (5, "Next options of reading 1: remind, approve and escalate. The ▸ button on a mark appends that symbol."),
            ])
    # Marker and token.
    y = p.y
    p.add(f'<text x="16" y="{y + 14}" class="b">A reading marker is not a BPMN token</text>')
    box = dict(x=40, y=y + 40, w=90, h=36)
    p.add(f'<rect x="40" y="{y+40}" width="90" height="36" rx="9" class="task"/>')
    p.add(f'<text x="85" y="{y+62}" class="lab" text-anchor="middle">remind</text>')
    sub = Drawing(g, "x")
    tabs(sub, box, [(1, True), (2, False)])
    p.add("\n".join(sub.out + sub.top))
    p.add(f'<text x="150" y="{y+55}" class="note">Reading marker: a numbered tab on the top edge. One tab for each reading that stands here.</text>')
    p.add(f'<text x="150" y="{y+71}" class="note">The number is the number of the reading in the output view.</text>')
    p.add(f'<rect x="40" y="{y+96}" width="90" height="36" rx="9" class="task"/>')
    p.add(f'<text x="85" y="{y+118}" class="lab" text-anchor="middle">remind</text>')
    p.add(f'<circle cx="44" cy="{y+128}" r="9" class="tok"/><text x="44" y="{y+132}" class="tokt" text-anchor="middle">2</text>')
    p.add(f'<text x="150" y="{y+111}" class="note">BPMN token, not used: a dot or a count badge in a corner. It counts running instances,</text>')
    p.add(f'<text x="150" y="{y+127}" class="note">and readings are alternatives of one run. The grammar view draws no dot.</text>')
    p.y = y + 150
    p.save(out / "mock-markers.svg")


def mock_next_options(out):
    """Tutorial after overview and install."""
    global font
    font = MONO
    g = load("tutorial")
    p = Page(1160)
    p.title(
        "Next options at each level of the open path, after overview · install",
        "One reading. The grammar view follows it into setup, the innermost rule that is not a rule for one step. "
        "A breadcrumb click opens tutorial, which shows the same state one level up.",
    )
    d = layout(g, "setup")
    inst = d.find("ref", "install")
    tabs(d, inst, [(1, True)])
    next_mark(d, inst)
    loop = d.find_loop("install")
    back_mark(d, loop["path"])
    pass_label(d, loop["lx"], loop["ly"], "pass 1")
    next_mark(d, d.find("ref", "configure"))
    end_mark(d, text="then introduce_concept")
    callout(d, inst["x"] - 18, inst["y"] - 4, 1)
    ex, ey = d.events["end"]
    callout(d, ex + 22, ey - 24, 2)
    p.panel(d, crumbs=["tutorial", "setup"], sub=g.rules["setup"]["doc"], notes=[
        (1, "install is a rule for one step, so its reference box looks like a task, and the marker sits on it. Another install starts pass 2."),
        (2, "The end event carries a mark when a next option lies after this rule. Its popover names the symbols, here introduce_concept."),
    ])
    d = layout(g, "tutorial")
    setup = d.find("ref", "setup")
    tabs(d, setup, [(1, True)])
    next_mark(d, setup)
    lesson = d.find("ref", "lesson")
    next_mark(d, lesson)
    popover(d, lesson["x"] - 10, lesson["y"] + lesson["h"] + 30, "next inside lesson", ["introduce_concept"])
    callout(d, setup["x"] - 18, setup["y"] - 4, 3)
    callout(d, lesson["x"] + lesson["w"] + 12, lesson["y"] + 4, 4)
    p.panel(d, crumbs=["tutorial"], sub=g.rules["tutorial"]["doc"], notes=[
        (3, "One level up, the marker sits on the reference box that holds the reading. The mark on setup says that the next step can stay inside it: install or configure."),
        (4, "The mark on lesson says that the next step can enter lesson. Its ▸ button opens a popover with the symbols that pass through the box."),
    ])
    p.save(out / "mock-next-options.svg")


def mock_current_pass(out):
    global font
    font = MONO
    g = load("urg-moves")
    p = Page(1160)
    p.title(
        "The current pass of a self tail call",
        "The URG grammar, after states the problem · proposes a solution · identifies a gap · revises the solution · identifies a gap. "
        "The reading is in pass 2 of refinement.",
    )
    d = layout(g, "refinement")
    gap = d.find("task", "identifies a gap")
    tabs(d, gap, [(1, True)])
    rev = d.find("task", "revises the solution")
    next_mark(d, rev)
    t = d.tails[0]
    pass_label(d, t["lx"], t["ly"], "↻ refinement · pass 2")
    callout(d, gap["x"] - 18, gap["y"] - 4, 1)
    callout(d, t["lx"] - 110, t["ly"], 2)
    p.panel(d, crumbs=["episode", "development", "refinement ↻ 2"], badge="no self-embedding", notes=[
        (1, "Pass 2 of refinement. The marker sits on the step just read, and the one next option is revises the solution."),
        (2, "The back edge carries the number of the current pass. Earlier passes leave no mark: the output view folds them."),
    ])
    d = layout(g, "refinement")
    rev = d.find("task", "revises the solution")
    tabs(d, rev, [(1, True)])
    t = d.tails[0]
    back_mark(d, t["pts"])
    next_mark(d, d.find("task", "identifies a gap"))
    pass_label(d, t["lx"], t["ly"], "↻ refinement · pass 2")
    end_mark(d, text="then gives an example · end")
    callout(d, t["lx"] - 110, t["ly"], 3)
    ex, ey = d.events["end"]
    callout(d, ex + 22, ey - 24, 4)
    p.panel(d, crumbs=["episode", "development", "refinement ↻ 2"], badge="no self-embedding",
            sub="One step later, after revises the solution.", notes=[
                (3, "identifies a gap would start pass 3. Its mark covers the back edge and the task box at the start of the new pass."),
                (4, "After this rule, the reading can go on with gives an example, or the episode can end."),
            ])
    g2 = load("tutorial")
    d = layout(g2, "lesson")
    se = d.find("ref", "show_example")
    tabs(d, se, [(1, True)])
    next_mark(d, se)
    lp = d.find_loop("show_example")
    back_mark(d, lp["path"])
    pass_label(d, lp["lx"], lp["ly"], "pass 2 · ↻ 1–3")
    next_mark(d, d.find("ref", "practice"))
    next_mark(d, d.find("ref", "check_understanding"))
    end_mark(d, text="then introduce_concept · summarise")
    callout(d, lp["lx"] - 80, lp["ly"], 5)
    p.panel(d, crumbs=["tutorial", "lesson"], sub="The tutorial, after a second show_example in the first lesson.", notes=[
        (5, "A repetition shows its current pass on its back flow, with its bound: pass 2 of 1 to 3."),
    ])
    p.save(out / "mock-current-pass.svg")


def mock_failure(out):
    global font
    font = MONO
    g = load("order-approval")
    p = Page(1160)
    p.title(
        "A failure point keeps the state before the rejected step",
        "Order approval: submit_order · check_order · escalate · approve · ship. Step 5 is rejected, because the "
        "escalated decision needs its hand_back first.",
    )
    banner = ('Step 5 "ship" is rejected. The grammar uses "ship" in outcome, but not at this point. '
              "Showing the state after step 4. Next: hand_back.")
    d = layout(g, "approval")
    ap = d.find("task", "approve")
    tabs(d, ap, [(1, True)])
    end_mark(d, text="then hand_back")
    callout(d, ap["x"] - 18, ap["y"] - 4, 1)
    p.panel(d, crumbs=["order", "outcome", "approval", "approval"], badge="may need a stack", banner=banner, notes=[
        (1, "The open path follows the reading into the inner approval, one level up from the first. The banner of the output-view spec names the rejected step."),
        (None, "The marks are those of the state after step 4. The only next option, hand_back, lies after this rule, so the end event carries it."),
    ])
    d = layout(g, "approval")
    inner = d.find("ref", "approval")
    tabs(d, inner, [(1, True)])
    hb = d.find("task", "hand_back")
    next_mark(d, hb)
    callout(d, hb["x"] + hb["w"] + 12, hb["y"] + 4, 2)
    p.panel(d, crumbs=["order", "outcome", "approval"], badge="may need a stack", banner=banner, notes=[
        (2, "A click on the breadcrumb approval shows the outer level. There the marker sits on the inner reference box, and hand_back carries the next-option mark."),
    ])
    p.save(out / "mock-failure.svg")


def mock_selection(out):
    global font
    font = MONO
    g = load("tutorial")
    p = Page(1160)
    p.title(
        "The selection and the data object",
        "The tutorial, after a step set_exercise with the attribute file=(url-reference)\"exercises/loop.py\". "
        "The selection style of the output-view spec is an amber outline.",
    )
    d = layout(g, "practice")
    se = d.find("ref", "set_exercise")
    tabs(d, se, [(1, True)])
    data_object(d, se, 1)
    next_mark(d, d.find("ref", "give_hint"))
    next_mark(d, d.find("ref", "show_solution"))
    callout(d, se["x"] + se["w"] / 2 + 34, se["y"] - 14, 1)
    p.panel(d, crumbs=["tutorial", "lesson", "practice"], sub=g.rules["practice"]["doc"], notes=[
        (1, "The data object of the step in focus sits in the free band above its box, joined by a dotted association. It shows the count of attributes, and a click selects the step."),
    ])
    d = layout(g, "lesson")
    pr = d.find("ref", "practice")
    select(d, pr)
    lp = d.find_loop("show_example")
    select_loop(d, lp)
    callout(d, pr["x"] - 14, pr["y"] - 8, 2)
    callout(d, lp["lx"] - 60, lp["ly"] + 2, 3)
    p.panel(d, crumbs=["tutorial", "lesson"], sub="Two selections, shown together here only to compare them.", notes=[
        (2, "A selected reference box gets the amber outline. In the output view, the rule rows of practice are selected."),
        (3, "A selected repetition outlines its gateways and its back flow. The output view selects its loop rows and fold rows."),
    ])
    p.save(out / "mock-selection.svg")


def mock_layout(out):
    global font
    font = MONO
    g = load("order-approval")
    p = Page(1160)
    p.title(
        "Each box takes its size from its children",
        "The layout is a fold over the expression of the rule. Each dashed frame is the box of one expression: the "
        "core measures the children first, then places them in the order of the text.",
    )
    d = layout(g, "approval")
    nest_frames(d, g, "approval")
    p.panel(d, crumbs=["approval"], notes=[
        (None, "Sequence: the children in a row, joined by flows of 24 px."),
        (None, "Repetition {0,2}: two gateway pairs, a skip flow above and a back flow below."),
        (None, "Choice: the first alternative on the main line, the next ones below it, in the order of the text."),
        (None, "An edit inside one frame changes the size of that frame and of its ancestors. Elements after it move, but none changes its order."),
    ])
    g2 = Grammar(g.text.replace('("approve" | "escalate" approval "hand_back")', '("approve" | "approve_with_conditions" | "escalate" approval "hand_back")'))
    d2 = layout(g2, "approval")
    p.panel(d2, crumbs=["approval"], sub='After an edit: a new alternative "approve_with_conditions".', notes=[
        (None, "The new branch adds height below approve. The task remind keeps its place, and the branch of escalate moves down by one branch."),
    ])
    p.save(out / "mock-layout.svg")


def nest_frames(d, g, rule):
    """Draw the box of each expression as a dashed frame. Only the layout mock-up uses this."""
    body = build(Drawing(g, rule), rule, g.rules[rule]["body"])
    x0 = MARGIN + EV + FLOW
    frames = []

    def visit(el, x, y, depth):
        w, up, down = el.size()
        name = {"Seq": "sequence", "Choice": "choice", "Repetition": "repetition", "Option": "optional part"}.get(
            type(el).__name__, type(el).__name__.lower())
        if not isinstance(el, Box):
            frames.append((x, y - up, w, up + down, name, depth))
        if isinstance(el, Seq):
            cx = x
            for i, it in enumerate(el.items):
                if i and not isinstance(it, TailRef):
                    cx += FLOW
                visit(it, cx, y, depth + 1)
                cx += it.size()[0]
        elif isinstance(el, Choice):
            el.size()
            for i, b in enumerate(el.branches):
                visit(b, x + GW + FLOW, y + el.offs[i], depth + 1)
        elif isinstance(el, Repetition):
            ix = x + (2 * GW + PAIR if el.star else GW) + FLOW
            visit(el.item, ix, y, depth + 1)

    visit(body, x0, d.main_y, 0)
    for x, y, w, h, name, depth in frames:
        pad = 4 - depth
        d.add(f'<rect x="{x - pad}" y="{y - pad:.1f}" width="{w + 2*pad}" height="{h + 2*pad:.1f}" rx="4" class="nest"/>')
        d.add(f'<text x="{x - pad + 3}" y="{y + h + pad + 11:.1f}" class="nestt">{name}</text>')


def mock_fonts(out):
    global font
    g = load("urg-moves")
    p = Page(1160)
    p.title(
        "Decision: a bundled monospace font or a proportional font with a width table",
        "The same two rules in both fonts. The layout constants are equal, and only the text widths differ. "
        "Agreed: the bundled monospace font.",
    )
    for f, name in ((MONO, "Bundled monospace font, 0.6 em for each cell (agreed)"),
                    (SANS, "Proportional font, widths from a table of the bundled font")):
        font = f
        p.add(f'<text x="16" y="{p.y + 12}" class="b">{escape(name)}</text>')
        p.y += 22
        for r in ("fix", "generalisation"):
            d = layout(g, r)
            lines = max(len(b["label"]) and len(wrap(b["label"])) for b in d.boxes)
            p.panel(d, crumbs=["episode", "…", r], notes=[
                (None, f"Diagram: {d.w:.0f} by {d.h:.0f} px. The longest label takes {lines} lines."),
            ], notes_w=320)
    font = MONO
    p.y = p.notes(16, p.y, [
        (None, "Both fonts use the same limit of 140 px for one line. The proportional font fits a whole phrase on "
               "one line, and the monospace font wraps it. So the monospace diagrams are narrower and taller."),
        (None, "Measured in Chromium on macOS, as em for each character: Menlo 0.602, SF Mono 0.618, Courier New "
               "0.600, Monaco 0.600. Chromium has no ui-monospace, and Safari maps it to SF Mono. A system font "
               "therefore differs from the width that the core assumes by up to 3 % on this machine. Fonts on other "
               "systems were not measured. A bundled font removes the difference."),
        (None, "A monospace font gives each character its own cell. Symbols that differ only by case, spaces, _ or - "
               "stay easy to tell apart, and the rule checks warn about exactly those symbols."),
    ], 900) + 10
    p.save(out / "mock-fonts.svg")


def kdl_string(text):
    return text if re.fullmatch(r"[A-Za-z_][A-Za-z0-9_\-]*", text) else '"' + text.replace('"', '\\"') + '"'


def write_kdl(out):
    """Write the diagram KDL of one rule, as the spec proposes it."""
    global font
    font = MONO
    g = load("tutorial")
    rule = "setup"
    d = layout(g, rule)
    n = lambda v: f"{v:.0f}"
    lines = [
        '// The diagram KDL of the rule setup of examples/tutorial.ebnf, as the spec rule-diagrams proposes it.',
        '// mockups.py writes this file. The hash is shortened here.',
        f'diagrams format-version=1 grammar-sha256="…" font-size={FONT_SIZE} cell-width={CELL} line-height={LINE} {{',
        f'    diagram rule={rule} width={n(d.w)} height={n(d.h)} {{',
    ]
    for kind, sh in d.shapes:
        if kind in ("start-event", "end-event"):
            lines.append(f'        {kind} x={n(sh["x"])} y={n(sh["y"])} size={EV}')
        elif kind == "gateway":
            lines.append(f'        gateway role={sh["role"]} start={sh["key"][0]} end={sh["key"][1]} '
                         f'x={n(sh["x"])} y={n(sh["y"])} size={GW}')
        else:
            name = "task" if kind == "task" else "reference"
            attr = f'symbol={kdl_string(sh["label"])}' if kind == "task" else f'rule={sh["label"]}'
            extra = ""
            if kind == "ref":
                extra = f' one-step={"#true" if sh["one_step"] else "#false"}'
                if sh["link"]:
                    extra += f' loop-link={sh["link"]}'
            text = " ".join(kdl_string(ln) for ln in sh["lines"])
            lines.append(f'        {name} start={sh["key"][0]} end={sh["key"][1]} {attr} x={n(sh["x"])} '
                         f'y={n(sh["y"])} width={sh["w"]} height={sh["h"]}{extra} {{')
            lines.append(f'            label {text}')
            lines.append('        }')
    for f in d.flows:
        head = f'        flow role={f["role"]}'
        if f["key"]:
            head += f' start={f["key"][0]} end={f["key"][1]}'
        if f["label"]:
            text, lx, ly = f["label"]
            head += f' label="{text}" label-x={n(lx)} label-y={n(ly)}'
        pts = " ".join(f"{n(x)} {n(y)}" for x, y in f["pts"])
        lines.append(f'{head.replace("flow ", "flow " + pts + " ", 1)}')
    lines.append("    }")
    lines.append("}")
    (out / "setup.diagram.kdl").write_text("\n".join(lines) + "\n")
    old = out / "approval.diagram.kdl"
    if old.exists():
        old.unlink()


def main():
    out = Path(sys.argv[1] if len(sys.argv) > 1 else ".")
    mock_constructs(out)
    mock_examples(out)
    mock_grove(out)
    mock_grove_trace(out)
    mock_grove_abandoned(out)
    mock_text_mode(out)
    mock_markers(out)
    mock_next_options(out)
    mock_current_pass(out)
    mock_failure(out)
    mock_selection(out)
    mock_layout(out)
    mock_fonts(out)
    write_kdl(out)


if __name__ == "__main__":
    main()
