#!/usr/bin/env python3
"""Draw the output-view mock-ups as SVG files.

Usage: python3 mockups.py OUTPUT_DIR

Each mock-up is a list of rows. Edit the rows, then run the script again.
The SVG files use system fonts and follow the light or dark scheme of the reader.
"""

import sys
from pathlib import Path
from xml.sax.saxutils import escape

STYLE = """
:root{--bg:#ffffff;--panel:#fbfbfc;--hdr:#f1f3f6;--line:#d5dae2;--t:#1d2330;--m:#677184;
--r:#1f4fa3;--s:#8a4500;--l:#6b3fa0;--fold:#eef0fb;--cur:#0e7c66;--curbg:#e3f4ef;
--sel:#e0a800;--e:#b3261e;--ebg:#fdecea;--chip:#eceff4;--link:#0b5fc0;--note:#2b3445;
--box:#ffffff;--boxl:#8d96a6;--code:#f4f5f7}
@media (prefers-color-scheme: dark){:root{--bg:#15171c;--panel:#1b1e25;--hdr:#232731;
--line:#363c48;--t:#e5e8ee;--m:#9aa3b3;--r:#8fb4ff;--s:#f0b37e;--l:#c9a8f7;--fold:#272b3e;
--cur:#4fd1b5;--curbg:#173a33;--sel:#f2c94c;--e:#ff7b72;--ebg:#3b1f22;--chip:#2a2f3b;
--link:#7cb7ff;--note:#d5dae3;--box:#1f232b;--boxl:#6c7484;--code:#20242c}}
svg{background:var(--bg)}
text{font-family:ui-sans-serif,system-ui,-apple-system,"Segoe UI",sans-serif;font-size:13px;fill:var(--t)}
.mono{font-family:ui-monospace,SFMono-Regular,Menlo,Consolas,monospace;font-size:12px}
.t{fill:var(--t)} .m{fill:var(--m)} .r{fill:var(--r);font-weight:600} .s{fill:var(--s)}
.l{fill:var(--l);font-weight:600} .p{fill:var(--cur);font-weight:600} .e{fill:var(--e);font-weight:600}
.lk{fill:var(--link)} .b{font-weight:700} .i{font-style:italic} .sm{font-size:11px}
.h1{font-size:15px;font-weight:700} .h2{font-size:12px;font-weight:700;fill:var(--m);letter-spacing:.04em}
.panel{fill:var(--panel);stroke:var(--line)} .hdr{fill:var(--hdr)} .rule{stroke:var(--line)}
.fold{fill:var(--fold)} .curbar{fill:var(--cur)} .curbg{fill:var(--curbg)} .errbg{fill:var(--ebg)}
.sel{fill:none;stroke:var(--sel);stroke-width:2.5} .chip{fill:var(--chip)}
.next{fill:var(--curbg);stroke:var(--cur)} .nexto{fill:none;stroke:var(--m);stroke-dasharray:3 3}
.tw{fill:var(--m)} .call{fill:var(--t)} .callt{fill:var(--bg);font-size:11px;font-weight:700}
.note{fill:var(--note)} .code{fill:var(--code);stroke:var(--line)}
.box{fill:var(--box);stroke:var(--boxl);stroke-width:1.3} .flow{stroke:var(--boxl);stroke-width:1.3;fill:none}
.back{stroke:var(--boxl);stroke-width:1.3;fill:none;stroke-dasharray:5 3}
.marker{fill:var(--cur)} .nextmark{fill:none;stroke:var(--cur);stroke-width:2;stroke-dasharray:5 3}
.ah{fill:var(--boxl)} .data{fill:var(--box);stroke:var(--boxl);stroke-width:1.2}
.assoc{stroke:var(--boxl);stroke-width:1.2;stroke-dasharray:2 3;fill:none}
.btn{fill:var(--hdr);stroke:var(--line)}
"""

ROW_H = 26
GUTTER = 58
INDENT = 18


def width(text, cls):
    """Estimate the drawn width of a text in pixels."""
    if "mono" in cls:
        return len(text) * 7.25
    if "sm" in cls:
        return len(text) * 6.0
    if cls.split()[0] in ("r", "l", "p", "e", "b"):
        return len(text) * 7.3
    return len(text) * 6.6


class Svg:
    def __init__(self, w, h):
        self.w, self.h = w, h
        self.maxy = 0
        self.items = []

    def add(self, s):
        self.items.append(s)

    def text(self, x, y, s, cls="t", anchor="start"):
        self.add(f'<text x="{x:.1f}" y="{y:.1f}" class="{cls}" text-anchor="{anchor}" xml:space="preserve">{escape(s)}</text>')

    def rect(self, x, y, w, h, cls, rx=0):
        self.add(f'<rect x="{x:.1f}" y="{y:.1f}" width="{w:.1f}" height="{h:.1f}" rx="{rx}" class="{cls}"/>')

    def line(self, x1, y1, x2, y2, cls="rule"):
        self.add(f'<line x1="{x1:.1f}" y1="{y1:.1f}" x2="{x2:.1f}" y2="{y2:.1f}" class="{cls}"/>')

    def path(self, d, cls):
        self.add(f'<path d="{d}" class="{cls}"/>')

    def spans(self, x, y, parts):
        """Draw a run of (class, text) parts as one text element. Return the estimated end x."""
        tspans = "".join(f'<tspan class="{cls}">{escape(s)}</tspan>' for cls, s in parts)
        self.add(f'<text x="{x:.1f}" y="{y:.1f}" xml:space="preserve">{tspans}</text>')
        return x + sum(width(s, cls) for cls, s in parts)

    def callout(self, x, y, n):
        self.add(f'<circle cx="{x:.1f}" cy="{y:.1f}" r="9" class="call"/>')
        self.text(x, y + 4, str(n), "callt", "middle")

    def save(self, path):
        self.h = max(self.h, int(self.maxy + 20))
        body = "\n".join(self.items)
        Path(path).write_text(
            f'<svg xmlns="http://www.w3.org/2000/svg" width="{self.w}" height="{self.h}" '
            f'viewBox="0 0 {self.w} {self.h}">\n<style>{STYLE}</style>\n'
            f'<defs><marker id="ah" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" '
            f'markerHeight="7" orient="auto-start-reverse"><path d="M0 0L10 5L0 10z" class="ah"/>'
            f'</marker></defs>\n'
            f'<rect width="100%" height="100%" style="fill:var(--bg)"/>\n{body}\n</svg>\n'
        )


# Parts of a row label.
def rule(s): return ("r", s)
def sym(s): return ("s mono", f'"{s}"')
def loop(s): return ("l", s)
def pas(s): return ("p", s)
def m(s): return ("m", s)
def t(s): return ("t", s)
def e(s): return ("e", s)
SEP = ("m", "  ·  ")


def row(depth, parts, gutter="", twisty=None, chips=(), right="", cur=False, last=False,
        fold=False, sel=False, err=False, pend=False, callout=None, kind="row"):
    return dict(depth=depth, parts=parts, gutter=gutter, twisty=twisty, chips=chips, right=right,
                cur=cur, last=last, fold=fold, sel=sel, err=err, pend=pend, callout=callout, kind=kind)


def nextrow(gutter, options, note="", callout=None, label="next"):
    """options: list of (symbol, solid). A solid option belongs to the shown reading."""
    return dict(kind="next", gutter=gutter, options=options, note=note, callout=callout, label=label)


def chip(svg, x, y, kind, s):
    """Draw one attribute chip at the row baseline y and return its end x."""
    icon = {"file": "▤ ", "link": "↗ ", "kv": "", "more": ""}[kind]
    label = icon + s
    cls = "mono sm" if kind != "more" else "m sm"
    w = width(label, "mono") * 0.92 + 14
    svg.rect(x, y - 13, w, 18, "chip", 9)
    svg.text(x + 7, y, label, ("lk " if kind in ("file", "link") else "m ") + cls)
    return x + w + 6


def twisty(svg, x, y, state):
    if state == "open":
        svg.path(f"M{x} {y-8} L{x+9} {y-8} L{x+4.5} {y-2.5}z", "tw")
    elif state == "closed":
        svg.path(f"M{x+1} {y-10} L{x+6.5} {y-5.5} L{x+1} {y-1}z", "tw")


def draw_rows(svg, x, y, w, rows, callouts):
    """Draw rows inside a panel from top y. Return the y below the last row."""
    for r in rows:
        base = y + 17
        if r["kind"] == "next":
            svg.line(x + 8, y + 2, x + w - 8, y + 2)
            svg.text(x + GUTTER - 10, base + 2, r["gutter"], "m mono sm", "end")
            cx = svg.spans(x + GUTTER + 6, base + 2, [m(r["label"])]) + 10
            for s, solid in r["options"]:
                label = f'"{s}"'
                bw = width(label, "mono") + 16
                svg.rect(cx, base - 11, bw, 20, "next" if solid else "nexto", 10)
                svg.text(cx + 8, base + 3, label, "s mono")
                cx += bw + 6
            if r["note"]:
                svg.text(cx + 4, base + 2, r["note"], "m sm")
            if r["callout"]:
                callouts.append((y + ROW_H / 2 + 2, r["callout"]))
            y += ROW_H + 6
            continue
        ix = x + GUTTER + 8 + r["depth"] * INDENT
        if r["err"]:
            svg.rect(x + 1, y, w - 2, ROW_H, "errbg")
        elif r["fold"]:
            svg.rect(ix - 4, y + 2, x + w - ix - 4, ROW_H - 4, "fold", 5)
        elif r["last"]:
            svg.rect(ix - 4, y + 2, x + w - ix - 4, ROW_H - 4, "curbg", 5)
        if r["cur"] or r["last"]:
            svg.rect(x + GUTTER - 2, y, 3, ROW_H, "curbar")
        if r["sel"]:
            svg.rect(ix - 6, y + 1, x + w - ix - 2, ROW_H - 2, "sel", 6)
        gcls = "m mono sm" if not r["err"] else "e mono sm"
        svg.text(x + GUTTER - 10, base, r["gutter"], gcls, "end")
        twisty(svg, ix, base, r["twisty"])
        lx = ix + (14 if r["twisty"] else 0)
        parts = r["parts"]
        if r["pend"]:
            parts = [(c + " i", s) for c, s in parts]
        ex = svg.spans(lx, base, parts)
        cx = ex + 10
        for kind, s in r["chips"]:
            cx = chip(svg, cx, base, kind, s)
        if r["right"]:
            svg.text(x + w - (34 if r["callout"] else 12), base, r["right"], "m sm", "end")
        if r["callout"]:
            callouts.append((y + ROW_H / 2, r["callout"]))
        y += ROW_H
    return y


def panel(svg, x, y, w, title, left="", right="", rows=(), details=None, inner_callouts=True):
    """Draw an output-view panel. Return (bottom y, callouts)."""
    callouts = []
    head = 38
    svg.rect(x, y, w, head, "hdr")
    svg.text(x + 12, y + 24, title, "b")
    hx = x + 12 + width(title, "b") + 18
    if left:
        svg.rect(hx, y + 9, 22, 20, "btn", 4)
        svg.text(hx + 11, y + 24, "‹", "m b", "middle")
        svg.rect(hx + 26, y + 9, 22, 20, "btn", 4)
        svg.text(hx + 37, y + 24, "›", "m b", "middle")
        svg.text(hx + 56, y + 24, left, "t")
    if right:
        svg.text(x + w - 12, y + 24, right, "m sm", "end")
    yy = draw_rows(svg, x, y + head + 6, w, rows, callouts)
    if details:
        yy += 6
        svg.line(x, yy, x + w, yy)
        svg.text(x + 12, yy + 20, "DETAILS", "h2")
        svg.text(x + 76, yy + 20, details["title"], "t")
        if details.get("callout"):
            callouts.append((yy + 16, details["callout"]))
        yy += 30
        for key, ann, value, action in details["rows"]:
            svg.text(x + 24, yy + 16, key, "mono")
            svg.text(x + 110, yy + 16, ann, "m mono sm")
            vx = svg.spans(x + 236, yy + 16, [("lk mono" if action else "t mono", value)])
            if action:
                bw = width(action, "t") + 18
                svg.rect(vx + 10, yy + 2, bw, 20, "btn", 4)
                svg.text(vx + 19, yy + 16, action, "t sm")
            yy += 24
        yy += 6
    outline_h = yy - y
    svg.add(f'<rect x="{x}" y="{y}" width="{w}" height="{outline_h:.1f}" rx="6" '
            f'style="fill:none;stroke:var(--line)"/>')
    if inner_callouts:
        for cy, n in callouts:
            svg.callout(x + w - 16, cy, n)
    return yy, callouts


def notes(svg, x, y, w, items, title="Reading the mock-up"):
    """Draw numbered notes. items: list of (number, text). Return the y below."""
    svg.text(x, y, title.upper(), "h2")
    y += 22
    per_line = int(w / 6.6)
    for n, s in items:
        svg.callout(x + 9, y - 4, n)
        words, line, lines = s.split(), "", []
        for word in words:
            if len(line) + len(word) + 1 > per_line:
                lines.append(line)
                line = word
            else:
                line = (line + " " + word).strip()
        lines.append(line)
        for i, ln in enumerate(lines):
            svg.text(x + 26, y + i * 17, ln, "note")
        y += len(lines) * 17 + 10
    svg.maxy = max(svg.maxy, y)
    return y


def code(svg, x, y, w, lines, title):
    h = 30 + len(lines) * 17
    svg.rect(x, y, w, h, "code", 6)
    svg.text(x + 12, y + 19, title.upper(), "h2")
    for i, ln in enumerate(lines):
        svg.text(x + 12, y + 40 + i * 17, ln, "mono")
    return y + h


def heading(svg, x, y, s, sub=""):
    svg.text(x, y, s, "h1")
    if sub:
        svg.text(x, y + 20, sub, "m")


# ---------------------------------------------------------------- mock-ups

def anatomy(out):
    svg = Svg(1240, 640)
    heading(svg, 24, 32, "The output view during a run of the tutorial grammar",
            "Agreed. 15 steps so far. The last step is step 15, in the second lesson.")
    rows = [
        row(0, [rule("tutorial")], twisty="open", cur=True),
        row(1, [rule("overview"), SEP, sym("overview")], gutter="1", callout=6),
        row(1, [rule("setup")], twisty="open"),
        row(2, [loop("install+"), SEP, m("2 passes  ·  2 steps")], gutter="2–3",
            twisty="closed", fold=True, callout=2),
        row(2, [rule("configure"), SEP, sym("configure")], gutter="4"),
        row(1, [loop("lesson+")], twisty="open", right="pass 2  ·  1 or more passes", cur=True, callout=3),
        row(2, [pas("completed"), SEP, m("1 pass  ·  8 steps  ·  1 file")], gutter="5–12",
            twisty="closed", fold=True, callout=4),
        row(2, [pas("pass 2"), SEP, rule("lesson")], twisty="open", cur=True, callout=5),
        row(3, [rule("introduce_concept"), SEP, sym("introduce_concept")], gutter="13"),
        row(3, [loop("show_example{1,3}"), SEP, m("1 pass  ·  1 step  ·  1 link")], gutter="14",
            twisty="closed", fold=True),
        row(3, [loop("practice*")], twisty="open", right="pass 1  ·  0 or more passes", cur=True),
        row(4, [pas("pass 1"), SEP, rule("practice")], twisty="open", cur=True),
        row(5, [rule("set_exercise"), SEP, sym("set_exercise")], gutter="15", last=True,
            chips=[("file", "exercises/recursion.py")], callout=7),
        nextrow("16", [("give_hint", True), ("show_solution", True)], callout=8),
    ]
    details = dict(title='Step 15  ·  set_exercise  ·  pass 1 of practice*, pass 2 of lesson+',
                   rows=[("file", "(url-reference)", "exercises/recursion.py", "Open folder…")],
                   callout=9)
    bottom, _ = panel(svg, 24, 70, 760, "Output view", "Reading 1 of 1",
                      "15 steps  ·  cannot end yet", rows, details)
    notes(svg, 812, 84, 404, [
        (1, "The gutter numbers each step from 1. A fold shows its range of steps, so a step keeps its number."),
        (2, "A loop that the reading has left folds into one row: the repetition as written, the count of passes, and the count of steps."),
        (3, "The row of an open loop names the repetition as written in its rule. It shows the current pass and the bound."),
        (4, "Completed passes fold into one row. Each count says what it counts: passes, steps, files."),
        (5, "The current pass stays open. The teal bar marks the path from the start rule to the last step."),
        (6, "A rule for one step shares a row with its step. A rule that can hold more steps never shares."),
        (7, "Attributes show as chips on the step row. References come first. The row keeps its height."),
        (8, "The next options of the state. A click appends that symbol, as in the grammar view."),
        (9, "The details panel shows the row in focus: the selected row, or else the row of the last step."),
    ])
    svg.callout(24 + 16, 70 + 38 + 6 + ROW_H * 4 + 13, 1)
    svg.h = int(max(bottom, 560) + 24)
    svg.save(out / "mock-anatomy.svg")


def folds(out):
    svg = Svg(1240, 520)
    heading(svg, 24, 32, "A fold opens on a click, and stays open while the run goes on",
            "Agreed. The same state as the anatomy view, with the lesson+ fold open.")
    left = [
        row(1, [loop("lesson+")], twisty="open", right="pass 2", cur=True),
        row(2, [pas("completed"), SEP, m("1 pass  ·  8 steps  ·  1 file")], gutter="5–12",
            twisty="closed", fold=True, sel=True, callout=1),
        row(2, [pas("pass 2"), SEP, rule("lesson")], twisty="open", cur=True),
        row(3, [rule("introduce_concept"), SEP, sym("introduce_concept")], gutter="13"),
        row(3, [m("…")], gutter=""),
    ]
    right = [
        row(1, [loop("lesson+")], twisty="open", right="pass 2", cur=True),
        row(2, [pas("completed"), SEP, m("1 pass  ·  8 steps  ·  open, a click folds it")], gutter="5–12",
            twisty="open", fold=True, callout=2),
        row(3, [pas("pass 1"), SEP, rule("lesson")], twisty="open"),
        row(4, [rule("introduce_concept"), SEP, sym("introduce_concept")], gutter="5"),
        row(4, [loop("show_example{1,3}"), SEP, m("2 passes  ·  2 steps")], gutter="6–7",
            twisty="closed", fold=True, callout=3),
        row(4, [loop("practice*"), SEP, m("1 pass  ·  4 steps  ·  1 file")], gutter="8–11",
            twisty="closed", fold=True),
        row(4, [rule("check_understanding"), SEP, sym("check_understanding")], gutter="12"),
        row(2, [pas("pass 2"), SEP, rule("lesson")], twisty="open", cur=True, callout=4),
        row(3, [rule("introduce_concept"), SEP, sym("introduce_concept")], gutter="13"),
        row(3, [m("…")], gutter=""),
    ]
    b1, _ = panel(svg, 24, 70, 580, "Folded (the rule)", rows=left)
    b2, _ = panel(svg, 636, 70, 580, "Open (after a click)", rows=right)
    notes(svg, 24, max(b1, b2) + 34, 1180, [
        (1, "Every completed pass folds by the same fixed rule. The fold never depends on the distance from the last step."),
        (2, "A click opens the fold. It stays open while the run goes on and new passes join it, until the person folds it again."),
        (3, "Inside an open fold, each completed pass shows its own content. Its loops fold by the same rule."),
        (4, "The current pass is never folded. A new pass folds the pass before it into the fold row above."),
    ])
    svg.h = 520
    svg.save(out / "mock-folds.svg")


def readings(out):
    svg = Svg(1240, 560)
    heading(svg, 24, 32, "Two readings of five steps of the order-approval grammar",
            "Agreed. The grammar is unambiguous, but step 5 \"escalate\" opens both an approval and a rejection.")

    def rows_for(rule_name, mine, other):
        return [
            row(0, [rule("order")], twisty="open", cur=True),
            row(1, [sym("submit_order")], gutter="1"),
            row(1, [sym("check_order")], gutter="2", callout=5 if rule_name == "approval" else None),
            row(1, [loop("revision{0,3}"), SEP, m("1 pass  ·  2 steps")], gutter="3–4",
                twisty="closed", fold=True),
            row(1, [rule("outcome")], twisty="open", cur=True),
            row(2, [rule(rule_name)], twisty="open", cur=True, callout=3 if rule_name == "approval" else None),
            row(3, [sym("escalate")], gutter="5", last=True),
            nextrow("6", [("remind", True), ("escalate", True), (mine, True), (other, False)],
                    callout=4 if rule_name == "approval" else None),
        ]
    b1, c1 = panel(svg, 24, 70, 580, "Output view", "Reading 1 of 2", "5 steps  ·  cannot end yet",
                   rows_for("approval", "approve", "reject"))
    b2, _ = panel(svg, 636, 70, 580, "Output view", "Reading 2 of 2", "5 steps  ·  cannot end yet",
                  rows_for("rejection", "reject", "approve"))
    svg.callout(24 + 300, 70 + 19, 1)
    svg.callout(24 + 124, 70 + 19 + 22, 2)
    y = max(b1, b2) + 26
    y = code(svg, 24, y, 580, [
        "outcome  = approval \"ship\" | rejection \"notify_rejection\" ;",
        "approval  = \"remind\"{0,2} (\"approve\" | \"escalate\" approval \"hand_back\") ;",
        "rejection = \"remind\"{0,2} (\"reject\" | \"escalate\" rejection \"hand_back\") ;",
    ], "The rules in play")
    svg.rect(636, max(b1, b2) + 26, 580, 58, "hdr", 6)
    svg.text(648, max(b1, b2) + 50, "At the reading cap, the count reads", "m")
    svg.text(648, max(b1, b2) + 72, "Reading 1 of 50+  ·  the tracer stopped at the reading cap", "t")
    notes(svg, 24, y + 34, 1180, [
        (1, "The count says what it counts: the readings of the steps so far. Each reading is one distinct tree."),
        (2, "The switch shows one reading at a time. A new step keeps the reading that extends the one on view."),
        (3, "Here the readings differ in one rule node. A later step decides: \"approve\" keeps reading 1, and \"reject\" keeps reading 2."),
        (4, "A solid option belongs to the reading on view. A dashed option belongs only to other readings. Both stay clickable."),
        (5, "Every reading numbers the same steps in the same way, so a person can compare two readings row by row."),
    ])
    svg.h = 560
    svg.save(out / "mock-readings.svg")


def failure(out):
    svg = Svg(1240, 700)
    heading(svg, 24, 32, "A rejected step in the tutorial grammar",
            "Agreed. The sequence has 6 steps. Step 4 \"set_exercise\" comes before any example.")
    rows = [
        row(0, [rule("tutorial")], twisty="open", cur=True),
        row(1, [rule("overview"), SEP, sym("overview")], gutter="1"),
        row(1, [rule("setup")], twisty="open"),
        row(2, [loop("install+"), SEP, m("1 pass  ·  1 step")], gutter="2", twisty="closed", fold=True),
        row(1, [loop("lesson+")], twisty="open", right="pass 1", cur=True),
        row(2, [pas("pass 1"), SEP, rule("lesson")], twisty="open", cur=True),
        row(3, [rule("introduce_concept"), SEP, sym("introduce_concept")], gutter="3", last=True,
            callout=2),
        row(0, [e("✗  "), ("e mono", '"set_exercise"'), m("   rejected: no reading accepts this step here")],
            gutter="4", err=True, chips=[], callout=3),
        row(0, [m("The grammar uses "), ("s mono", '"set_exercise"'), m(" in practice, but not at this point.")],
            gutter="", err=True, callout=4),
        nextrow("4", [("show_example", True)], note="a click inserts the symbol before step 4", callout=5,
                label="next at 4"),
        row(0, [m("2 later steps, not read:  "), ("s mono i", '"show_solution"'), m(", "),
                ("s mono i", '"summarise"')], gutter="5–6", twisty="closed", pend=True, callout=6),
    ]
    bottom, _ = panel(svg, 24, 70, 760, "Output view", "Reading 1 of 1",
                      "rejected at step 4  ·  showing the state after step 3", rows)
    svg.callout(24 + 430, 70 + 19, 1)
    y = bottom + 24
    svg.rect(24, y, 760, 58, "hdr", 6)
    svg.text(36, y + 24, "When the grammar has no such symbol, line two reads", "m")
    svg.text(36, y + 46, 'The grammar has no symbol "set_excercise".', "t")
    gy = y + 84
    grammar_sketch_lesson(svg, 24, gy)
    notes(svg, 812, 84, 404, [
        (1, "The header names the failure point and the state on view: the state after the step before it."),
        (2, "The tree of the reading stays as it was after step 3, with its folds and its current pass."),
        (3, "The rejected step gets a row of its own at the end. It belongs to no node, so it has no indent."),
        (4, "A second line says why: the grammar uses the symbol elsewhere, or the grammar has no such symbol."),
        (5, "The next options of the state before the rejected step. A click inserts the symbol before it, and the tracer reads on."),
        (6, "Steps after the failure point fold into one row. The tracer has not read them, but their attributes stay visible."),
        (7, "The grammar view keeps its markers and next options of the state after step 3. A banner names the rejected step."),
    ])
    svg.h = int(gy + 190)
    svg.save(out / "mock-failure.svg")


# ------------------------------------------------------- grammar-view sketches

def task(svg, x, y, label, w=None, ref=False, cls="box"):
    w = w or width(label, "mono") + 26
    svg.rect(x, y, w, 40, cls, 7)
    svg.text(x + w / 2, y + 25, label, ("r" if ref else "s mono"), "middle")
    if ref:
        svg.rect(x + w / 2 - 6, y + 28, 12, 10, "box", 1)
        svg.text(x + w / 2, y + 37, "+", "m sm", "middle")
    return w


def arrow(svg, x1, y1, x2, y2):
    svg.add(f'<path d="M{x1:.1f} {y1:.1f} L{x2:.1f} {y2:.1f}" class="flow" marker-end="url(#ah)"/>')


def start_event(svg, x, y):
    svg.add(f'<circle cx="{x}" cy="{y}" r="9" class="box"/>')


def end_event(svg, x, y):
    svg.add(f'<circle cx="{x}" cy="{y}" r="9" class="box" style="stroke-width:3.2"/>')


def gateway(svg, x, y):
    svg.path(f"M{x} {y-14} L{x+14} {y} L{x} {y+14} L{x-14} {y}z", "box")
    svg.text(x, y + 4, "×", "m b", "middle")


def marker(svg, x, y):
    """A reading marker: a filled chevron above a box, distinct from a BPMN token."""
    svg.path(f"M{x-7} {y-14} L{x+7} {y-14} L{x+7} {y-6} L{x} {y-1} L{x-7} {y-6}z", "marker")


def sketch_frame(svg, x, y, w, h, crumbs, banner=None):
    svg.rect(x, y, w, h, "panel", 6)
    svg.rect(x, y, w, 38, "hdr")
    svg.text(x + 12, y + 24, "Grammar view", "b")
    svg.text(x + 120, y + 24, crumbs, "m")
    svg.text(x + w - 12, y + h - 10, "sketch: the rule-diagram design sets the drawing", "m sm i", "end")
    if banner:
        svg.rect(x + 1, y + 39, w - 2, 26, "errbg")
        svg.text(x + 12, y + 57, banner, "e sm")


def grammar_sketch_lesson(svg, x, y):
    sketch_frame(svg, x, y, 760, 168, "tutorial › lesson (pass 1)",
                 'Step 4 "set_exercise" rejected.  Next option: "show_example".')
    cy = y + 118
    start_event(svg, x + 30, cy)
    arrow(svg, x + 39, cy, x + 58, cy)
    w1 = task(svg, x + 60, cy - 20, "introduce_concept", ref=True, w=150)
    marker(svg, x + 60 + 75, cy - 20)
    arrow(svg, x + 210, cy, x + 232, cy)
    task(svg, x + 234, cy - 20, "show_example", ref=True, w=130)
    svg.rect(x + 230, cy - 24, 138, 48, "nextmark", 9)
    svg.text(x + 299, cy + 34, "{1,3}", "l sm", "middle")
    arrow(svg, x + 364, cy, x + 386, cy)
    task(svg, x + 388, cy - 20, "practice", ref=True, w=100)
    svg.text(x + 438, cy + 34, "*", "l sm", "middle")
    arrow(svg, x + 488, cy, x + 510, cy)
    task(svg, x + 512, cy - 20, "check_understanding", ref=True, w=170)
    svg.text(x + 597, cy + 34, "?", "l sm", "middle")
    arrow(svg, x + 682, cy, x + 714, cy)
    end_event(svg, x + 724, cy)
    svg.callout(x + 760 - 16, y + 52, 7)


def selection(out):
    svg = Svg(1240, 700)
    heading(svg, 24, 32, "A selected step in the URG move grammar, in both views",
            "Agreed. Seven steps so far. The person selects step 6 \"identifies a gap\", in pass 2 of a self tail call.")
    rows = [
        row(0, [rule("episode")], twisty="open", cur=True),
        row(1, [sym("refers back")], gutter="1"),
        row(1, [sym("states the problem")], gutter="2"),
        row(1, [rule("development")], twisty="open", cur=True),
        row(2, [sym("proposes a solution")], gutter="3"),
        row(2, [loop("refinement ↻"), m("  self tail call")], twisty="open", right="pass 2", cur=True,
            callout=2),
        row(3, [pas("completed"), SEP, m("1 pass  ·  2 steps")], gutter="4–5", twisty="closed",
            fold=True),
        row(3, [pas("pass 2"), SEP, rule("refinement")], twisty="open", cur=True),
        row(4, [sym("identifies a gap")], gutter="6", sel=True, callout=1),
        row(4, [sym("revises the solution")], gutter="7", last=True),
        nextrow("8", [("identifies a gap", True), ("gives an example", True)], note="or end here"),
    ]
    bottom, _ = panel(svg, 24, 70, 600, "Output view", "Reading 1 of 1", "7 steps  ·  can end here", rows)
    # Grammar view sketch of the refinement rule.
    gx, gy, gw = 652, 70, 564
    sketch_frame(svg, gx, gy, gw, 300, "episode › development › refinement (pass 2)")
    svg.callout(gx + 120 + width("episode \u203A development \u203A refinement (pass 2)", "t") + 14, gy + 19, 3)
    cy = gy + 160
    start_event(svg, gx + 30, cy)
    arrow(svg, gx + 39, cy, gx + 66, cy)
    task(svg, gx + 68, cy - 20, "identifies a gap", w=150)
    svg.rect(gx + 63, cy - 25, 160, 50, "sel", 9)
    svg.callout(gx + 143, cy - 42, 4)
    arrow(svg, gx + 218, cy, gx + 244, cy)
    task(svg, gx + 246, cy - 20, "revises the solution", w=170)
    marker(svg, gx + 331, cy - 20)
    arrow(svg, gx + 416, cy, gx + 448, cy)
    gateway(svg, gx + 462, cy)
    arrow(svg, gx + 476, cy, gx + 520, cy)
    end_event(svg, gx + 530, cy)
    svg.add(f'<path d="M{gx+462} {cy+14} L{gx+462} {cy+64} L{gx+52} {cy+64} L{gx+52} {cy+8}" '
            f'class="back" marker-end="url(#ah)"/>')
    svg.text(gx + 257, cy + 82, "refinement: self tail call, a new pass", "m sm", "middle")
    svg.callout(gx + 470, cy + 64, 5)
    svg.text(gx + 16, gy + 262, "Teal chevron: the reading marker after step 7.  Yellow outline: the selection.",
             "m sm")
    y = max(bottom, gy + 300) + 30
    notes(svg, 24, y, 1180, [
        (1, "The person selects the row of step 6. The selection outline is yellow, and the teal marks of the run stay as they are."),
        (2, "The loop row of a self tail call names the rule with ↻. Each pass is one node of the rule."),
        (3, "The grammar view opens the diagram that holds the element. Its open path follows the ancestors of the node, so it shows only the current pass."),
        (4, "The task box of the symbol occurrence that step 6 matched gets the same yellow outline."),
        (5, "A selection of the back edge in the grammar view selects the pass rows that the tail call started."),
    ])
    svg.h = 700
    svg.save(out / "mock-selection.svg")


def attributes(out):
    svg = Svg(1240, 760)
    heading(svg, 24, 32, "Step attributes and file references",
            "Agreed. The output view shows chips and a details panel. The grammar view shows a data object.")
    rows = [
        row(0, [rule("show_example"), SEP, sym("show_example")], gutter="14",
            chips=[("link", "doc.rust-lang.org/book/\u2026"), ("kv", "level=2")], callout=1),
        row(0, [rule("set_exercise"), SEP, sym("set_exercise")], gutter="15", last=True, sel=True,
            chips=[("file", "exercises/recursion.py"), ("more", "+2")], callout=2),
    ]
    details_before = dict(title='Step 15  ·  set_exercise  ·  3 attributes', callout=3, rows=[
        ("file", "(url-reference)", "exercises/recursion.py", "Open folder…"),
        ("minutes", "", "20", None),
        ("solution", "(url-reference)", "/home/ann/solutions/r.py", None),
    ])
    b1, _ = panel(svg, 24, 70, 600, "Before the folder is open", rows=rows, details=details_before)
    rows2 = [dict(r) for r in rows]
    rows2[0]["callout"] = None
    rows2[1]["callout"] = None
    details_after = dict(title='Step 15  ·  set_exercise  ·  folder "tutorial-run" open', callout=4,
                         rows=[
                             ("file", "(url-reference)", "exercises/recursion.py", "Open"),
                             ("minutes", "", "20", None),
                             ("solution", "(url-reference)", "/home/ann/solutions/r.py", None),
                         ])
    b2, _ = panel(svg, 636, 70, 580, "After the folder is open", rows=rows2, details=details_after)
    svg.text(648, b2 + 18, "solution: a path that starts with / stays text. The page opens only", "m sm")
    svg.text(648, b2 + 34, "web addresses and paths relative to the opened folder.", "m sm")
    svg.callout(1216 - 16, b2 + 22, 5)
    # Grammar view decoration sketch.
    gy = max(b1, b2) + 56
    sketch_frame(svg, 24, gy, 600, 170, "… › practice (pass 1)")
    cy = gy + 116
    start_event(svg, 24 + 30, cy)
    arrow(svg, 24 + 39, cy, 24 + 70, cy)
    task(svg, 24 + 72, cy - 20, "set_exercise", w=140)
    marker(svg, 24 + 142, cy - 20)
    svg.rect(24 + 67, cy - 25, 150, 50, "sel", 9)
    svg.path(f"M{24+250} {cy-66} L{24+276} {cy-66} L{24+284} {cy-58} L{24+284} {cy-30} L{24+250} {cy-30}z", "data")
    svg.path(f"M{24+276} {cy-66} L{24+276} {cy-58} L{24+284} {cy-58}", "assoc")
    svg.text(24 + 267, cy - 42, "3", "m sm b", "middle")
    svg.add(f'<path d="M{24+250} {cy-40} L{24+212} {cy-18}" class="assoc"/>')
    svg.text(24 + 294, cy - 44, "3 attributes of step 15", "m sm")
    arrow(svg, 24 + 212, cy, 24 + 250, cy)
    svg.text(24 + 260, cy + 4, "…", "m")
    svg.callout(24 + 600 - 16, gy + 52, 6)
    notes(svg, 652, gy + 4, 564, [
        (1, "A web address shows its host and path. A click opens it in a new tab."),
        (2, "A relative file reference shows its path. Other attributes follow as key=value chips while they fit, and \"+2\" counts the rest."),
        (3, "Before any folder is open, the page cannot read the file. The panel offers the folder picker."),
        (4, "After the person opens the folder of the trace file, the page resolves each relative reference against it."),
        (5, "Other forms stay text, and the page never runs a value as a script."),
        (6, "The grammar view draws a data object beside the task box of the step in focus, with the attribute count. It never moves a box."),
    ])
    svg.h = int(gy + 200)
    svg.save(out / "mock-attributes.svg")


def entry_rule(out):
    svg = Svg(1240, 600)
    heading(svg, 24, 32, "Where a pass starts in a loop through several rules",
            "Agreed. Six steps of an invented interview grammar, drawn by each rule for passes.")
    code(svg, 24, 58, 1192, [
        "dialogue = \"opens\" question | \"resumes\" answer ;",
        "question = \"asks\" answer ;",
        "answer   = \"answers\" question? ;        // question and answer form a loop of tail references",
    ], "Grammar   ·   sequence: opens asks answers asks answers asks")
    a = [
        row(0, [rule("dialogue")], twisty="open", cur=True),
        row(1, [sym("opens")], gutter="1"),
        row(1, [loop("question ↻"), m("  entered at question")], twisty="open", right="pass 3", cur=True,
            callout=1),
        row(2, [pas("completed"), SEP, m("2 passes  ·  4 steps")], gutter="2–5", twisty="closed",
            fold=True, callout=2),
        row(2, [pas("pass 3"), SEP, rule("question")], twisty="open", cur=True),
        row(3, [sym("asks")], gutter="6", last=True),
        nextrow("7", [("answers", True)]),
    ]
    b = [
        row(0, [rule("dialogue")], twisty="open", cur=True),
        row(1, [sym("opens")], gutter="1"),
        row(1, [loop("question ↻ answer")], twisty="open", right="pass 5", cur=True),
        row(2, [pas("completed"), SEP, m("4 passes  ·  4 steps")], gutter="2–5", twisty="closed",
            fold=True, callout=3),
        row(2, [pas("pass 5"), SEP, rule("question")], twisty="open", cur=True),
        row(3, [sym("asks")], gutter="6", last=True),
        nextrow("7", [("answers", True)]),
    ]
    b1, _ = panel(svg, 24, 160, 580, "Agreed: each pass starts at the pass rule", rows=a)
    b2, _ = panel(svg, 636, 160, 580, "Rejected: every tail call starts a pass", rows=b)
    notes(svg, 24, max(b1, b2) + 34, 1180, [
        (1, "The reading entered the loop at question, from dialogue, so question is the pass rule. Each pass starts at a tail reference to it."),
        (2, "One pass is one trip around the loop: \"asks\" then \"answers\". The count matches the repetition (\"asks\" \"answers\")+ that the loop is equal to."),
        (3, "If every tail call starts a pass, passes alternate between question and answer, and the count doubles. A pass no longer means one trip."),
    ])
    svg.h = 600
    svg.save(out / "mock-entry-rule.svg")


def main():
    if len(sys.argv) != 2:
        sys.exit("usage: mockups.py OUTPUT_DIR")
    out = Path(sys.argv[1])
    out.mkdir(parents=True, exist_ok=True)
    for draw in (anatomy, folds, readings, failure, selection, attributes, entry_rule):
        draw(out)


if __name__ == "__main__":
    main()
