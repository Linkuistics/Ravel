// The grammar view: one rule diagram at a time, with breadcrumbs for the open
// path, a header and a rule index. The core lays out each diagram, and the
// page computes no positions of its own: docs/adr/layout-in-the-core.md.
// The section "The grammar view" of docs/specs/rule-diagrams.md states the
// behaviour. The section "Marks of a trace" states the marks of the state on
// view, which the core computes for each open path.

import {
  type Selection,
  select,
  type ZoomBehavior,
  type ZoomTransform,
  zoom,
  zoomIdentity,
} from "d3";
import type {
  CheckedGrammar,
  Diagram,
  DiagramsOutcome,
  Flow,
  MarksOutcome,
  Shape,
  Span,
} from "./core.ts";
import {
  type ElementKey,
  type ElementMarks,
  type MarksView,
  marksView,
  marksWords,
  outsideText,
} from "./marks.ts";
import { indexPath, isOpenPath, type RuleGraph, ruleGraph } from "./open-path.ts";
import { spanKey } from "./outline.ts";

type Badge = CheckedGrammar["recursiveGroups"][number]["badge"];
type Box = Extract<Shape, { type: "task" | "reference" }>;
type Point = Flow["points"][number];

const svgNs = "http://www.w3.org/2000/svg";

const badgeLabels: Readonly<Record<Badge, string>> = {
  "no-self-embedding": "no self-embedding",
  "may-need-a-stack": "may need a stack",
};

// The drawing constants of the spec that the coordinates of the core leave to
// the page: the band of the drill-down marker and the height of a label line.
const drillBand = 12;
const lineHeight = 16;
const cornerRadius = 6;
const labelPadding = 4;
// The marks: a numbered tab in the free band of 22 px above a box, an outline
// around a marked element and a ▸ button at its upper right corner.
const tabWidth = 18;
const moreTabWidth = 26;
const tabHeight = 13;
const tabPoint = 4;
const tabGap = 4;
const tabInset = 10;
const outlineGap = 4;
const buttonSize = 14;
// After three tabs, a tab "+n" counts the rest.
const maxTabs = 3;
// The selection: an amber outline further out than a next-option mark. The
// data object: a page of 14 by 17 px in the free band above a box.
const selectionGap = 7;
const freeBand = 22;
const dataWidth = 14;
const dataHeight = 17;
const dataFold = 4;
// The gateways of a repetition. A click on one of them selects the repetition.
const repetitionGateways: ReadonlySet<string> = new Set([
  "loop-join",
  "loop-split",
  "skip-split",
  "skip-join",
]);

/** The elements of the page that the grammar view fills. */
export interface GrammarViewElements {
  readonly root: HTMLElement;
  readonly note: HTMLElement;
  readonly index: HTMLUListElement;
  readonly crumbs: HTMLOListElement;
  readonly title: HTMLElement;
  readonly badge: HTMLElement;
  readonly doc: HTMLElement;
  readonly frame: HTMLElement;
  readonly zoomIn: HTMLButtonElement;
  readonly zoomOut: HTMLButtonElement;
  readonly fit: HTMLButtonElement;
  /** The banner of a failure point. */
  readonly banner: HTMLElement;
  /** The note that names the readings outside the open diagram. */
  readonly outside: HTMLElement;
  /** The next options that no listed reading allows, with a ▸ button each. */
  readonly unlisted: HTMLElement;
  /** The popover of the next options through a reference box or the end event. */
  readonly popover: HTMLElement;
}

/** The state on view of a trace, as the grammar view marks it. */
export interface TraceMarks {
  /** The number of the reading on view, from 1. It is 0 when the state has no reading. */
  readonly onView: number;
  /** The number of listed readings. */
  readonly count: number;
  /** The follow path of the reading on view. */
  readonly followPath: readonly string[];
  /** The symbols of the next options of the state, in their order. */
  readonly order: readonly string[];
  /** The operation Marks of the core, for an open path. */
  readonly marks: (openPath: readonly string[]) => MarksOutcome;
  /** The banner of a failure point, when the last read failed. */
  readonly banner: string | undefined;
  /** Appends a next option to the sequence, and reads it. */
  readonly choose: (symbol: string) => void;
  /**
   * The step in focus and its count of attributes. The operation Marks of
   * the core places its data object.
   */
  readonly focus: { readonly step: number; readonly attributes: number } | undefined;
  /** The span keys of the selected grammar elements. */
  readonly selected: ReadonlySet<string>;
  /** Selects a grammar element, by its span. */
  readonly selectElement: (span: Span) => void;
  /** Selects a step: a click on the data object. */
  readonly selectStep: (step: number) => void;
}

/**
 * What an operation does to the open path: it keeps it, it follows the
 * reading on view, or it opens the given path.
 */
export type OpenChange = "keep" | "follow" | readonly string[];

export interface GrammarView {
  /**
   * Shows the diagrams of a new grammar text. After a text with errors, the
   * view keeps the diagrams of the last text without errors, dimmed.
   */
  readonly update: (checked: CheckedGrammar, outcome: DiagramsOutcome) => void;
  /** The open path, from its first rule to the open rule. */
  readonly openPath: () => readonly string[];
  /**
   * Marks the state on view of a trace, or nothing. After an operation on the
   * trace, `open` is "follow", and the open path becomes the follow path of
   * the reading on view: the section "Following a reading" of the spec. A
   * selected row of the output view gives the open path of its elements.
   */
  readonly showTrace: (trace: TraceMarks | undefined, open: OpenChange) => void;
}

/** The diagrams of the last grammar text without errors, with its facts. */
interface Model {
  readonly checked: CheckedGrammar;
  readonly diagrams: ReadonlyMap<string, Diagram>;
  readonly graph: RuleGraph;
  readonly badges: ReadonlyMap<string, Badge>;
}

function badgesOf(checked: CheckedGrammar): Map<string, Badge> {
  const badges = new Map<string, Badge>();
  for (const group of checked.recursiveGroups) {
    for (const member of group.members) {
      badges.set(member.rule, group.badge);
    }
  }
  return badges;
}

/** An SVG path through the corners of a flow, with rounded corners. */
function roundedPath(points: readonly Point[]): string {
  const first = points[0];
  if (first === undefined) {
    return "";
  }
  const parts = [`M${first.x} ${first.y}`];
  for (let i = 1; i < points.length - 1; i++) {
    const [a, b, c] = [points[i - 1], points[i], points[i + 1]] as [Point, Point, Point];
    const before = Math.hypot(b.x - a.x, b.y - a.y);
    const after = Math.hypot(c.x - b.x, c.y - b.y);
    const r = Math.min(cornerRadius, before / 2, after / 2);
    const inX = before === 0 ? b.x : b.x - ((b.x - a.x) / before) * r;
    const inY = before === 0 ? b.y : b.y - ((b.y - a.y) / before) * r;
    const outX = after === 0 ? b.x : b.x + ((c.x - b.x) / after) * r;
    const outY = after === 0 ? b.y : b.y + ((c.y - b.y) / after) * r;
    parts.push(`L${inX} ${inY}`, `Q${b.x} ${b.y} ${outX} ${outY}`);
  }
  const last = points[points.length - 1] as Point;
  parts.push(`L${last.x} ${last.y}`);
  return parts.join(" ");
}

const pointKey = (point: Point): string => `${point.x},${point.y}`;

/** The name of a box for assistive technology, such as "task remind". */
function boxLabel(box: Box): string {
  return box.type === "task" ? `task ${box.symbol}` : `reference ${box.rule}`;
}

/** The key of the marks of a box, as `elementKey` of marks.ts gives it. */
const boxKey = (box: Box): ElementKey =>
  `${box.type === "task" ? "task-box" : "reference-box"}:${box.span.start}-${box.span.end}`;

/** The key of the marks of a back flow or a back edge. A flow of another role has none. */
function flowKey(flow: Flow): ElementKey | undefined {
  if (flow.span === undefined) {
    return undefined;
  }
  const kind = flow.role === "back" ? "back-flow" : flow.role === "back-edge" ? "back-edge" : "";
  return kind === "" ? undefined : `${kind}:${flow.span.start}-${flow.span.end}`;
}

/** The middle of the longest segment of a flow, for a label that the core did not place. */
function middleOfLongest(points: readonly Point[]): Point | undefined {
  let best: { at: Point; length: number } | undefined;
  for (let i = 1; i < points.length; i++) {
    const [a, b] = [points[i - 1], points[i]] as [Point, Point];
    const length = Math.hypot(b.x - a.x, b.y - a.y);
    if (best === undefined || length > best.length) {
      best = { at: { x: (a.x + b.x) / 2, y: (a.y + b.y) / 2 }, length };
    }
  }
  return best?.at;
}

/** The label of a flow with the current pass, as in `pass 2 · ↻ 1–3` or `↻ refinement · pass 2`. */
function passLabel(flow: Flow, pass: number): string {
  const label = flow.label?.text;
  if (label === undefined) {
    return `pass ${pass}`;
  }
  return flow.role === "back-edge" ? `${label} · pass ${pass}` : `pass ${pass} · ${label}`;
}

/** The bounds of a marked shape: a box, or the circle of an event. */
interface Bounds {
  readonly x: number;
  readonly y: number;
  readonly width: number;
  readonly height: number;
  readonly round: boolean;
}

export function createGrammarView(elements: GrammarViewElements): GrammarView {
  let model: Model | undefined;
  let stale = false;
  let path: string[] = [];
  // The zoom of the diagram on view, so that a redraw of the same rule keeps it.
  let drawn: { rule: string; transform: ZoomTransform } | undefined;
  let zoomBehavior: ZoomBehavior<SVGSVGElement, unknown> | undefined;
  let svgSelection: Selection<SVGSVGElement, unknown, null, undefined> | undefined;
  let fitTransform: ZoomTransform = zoomIdentity;
  // The trace on view, and its marks for the open path.
  let trace: TraceMarks | undefined;
  let marks: MarksView | undefined;

  const open = (next: string[], focus = false): void => {
    path = next;
    render();
    if (focus) {
      svgSelection?.node()?.focus();
    }
  };

  const drillDown = (rule: string, focus = false): void => {
    open([...path, rule], focus);
  };

  const renderIndex = (m: Model): void => {
    const openRule = path[path.length - 1];
    const start = m.graph.start;
    const items = select(elements.index)
      .selectAll<HTMLLIElement, string>("li")
      .data(m.graph.rules)
      .join("li")
      .attr("data-rule", (d) => d)
      .classed("start", (d) => d === start);
    items.selectAll("*").remove();
    items
      .append("button")
      .attr("type", "button")
      .attr("class", "rule-name")
      .attr("aria-current", (d) => (d === openRule ? "true" : null))
      .text((d) => d)
      .on("click", (_event, d) => openFromIndex(d));
    items
      .filter((d) => d === start)
      .append("span")
      .attr("class", "start-mark")
      .text("start rule");
    items.each(function (d) {
      const badge = m.badges.get(d);
      if (badge !== undefined) {
        select(this)
          .append("span")
          .attr("class", `badge ${badge}`)
          .attr("data-badge", badge)
          .text(badgeLabels[badge]);
      }
    });
  };

  const openFromIndex = (rule: string): void => {
    if (model !== undefined) {
      open(indexPath(model.graph, rule));
    }
  };

  const renderCrumbs = (): void => {
    const items = select(elements.crumbs)
      .selectAll<HTMLLIElement, string>("li")
      .data(path)
      .join("li")
      .attr("data-rule", (d) => d);
    items.selectAll("*").remove();
    items.each(function (rule, i) {
      const item = select(this);
      const crumb =
        i === path.length - 1
          ? item.attr("aria-current", "location").append("span").attr("class", "crumb")
          : item
              .attr("aria-current", null)
              .append("button")
              .attr("type", "button")
              .attr("class", "crumb")
              .on("click", () => open(path.slice(0, i + 1)));
      crumb.append("span").attr("class", "crumb-rule").text(rule);
      // The breadcrumb of a pass rule shows the loop sign and the current pass,
      // such as `refinement ↻ 2`.
      const pass = marks?.crumbPasses.get(i);
      item.attr("data-pass", pass ?? null);
      if (pass !== undefined) {
        crumb.append("span").attr("class", "crumb-pass").text(` ↻ ${pass}`);
      }
    });
  };

  const renderHeader = (m: Model, rule: string): void => {
    elements.title.textContent = rule;
    const badge = m.badges.get(rule);
    elements.badge.hidden = badge === undefined;
    elements.badge.className = badge === undefined ? "badge" : `badge ${badge}`;
    if (badge === undefined) {
      delete elements.badge.dataset.badge;
      elements.badge.textContent = "";
    } else {
      elements.badge.dataset.badge = badge;
      elements.badge.textContent = badgeLabels[badge];
    }
    const doc = m.checked.rules.find((r) => r.name === rule)?.doc?.text;
    elements.doc.replaceChildren();
    if (doc === undefined) {
      return;
    }
    const [first = "", ...rest] = doc.split("\n");
    if (rest.length === 0) {
      const line = document.createElement("p");
      line.textContent = first;
      elements.doc.append(line);
      return;
    }
    // A click on the first line shows the whole doc string.
    const details = document.createElement("details");
    const summary = document.createElement("summary");
    summary.textContent = first;
    const more = document.createElement("p");
    more.textContent = rest.join("\n");
    details.append(summary, more);
    elements.doc.append(details);
  };

  const drawDiagram = (diagram: Diagram): void => {
    const frame = elements.frame;
    frame.replaceChildren();
    const svg = select(frame)
      .append<SVGSVGElement>(() => document.createElementNS(svgNs, "svg"))
      .attr("class", "diagram")
      .attr("role", "graphics-document")
      .attr("aria-label", diagram.rule)
      .attr("tabindex", "-1")
      .attr("data-width", diagram.width)
      .attr("data-height", diagram.height);
    svgSelection = svg;
    svg
      .append("defs")
      .append("marker")
      .attr("id", "diagram-arrowhead")
      .attr("viewBox", "0 0 10 10")
      .attr("refX", 9)
      .attr("refY", 5)
      .attr("markerWidth", 7)
      .attr("markerHeight", 7)
      .attr("orient", "auto-start-reverse")
      .append("path")
      .attr("d", "M0 0L10 5L0 10z")
      .attr("class", "arrowhead");
    const viewport = svg.append("g").attr("class", "viewport");

    const shapes = viewport.append("g").attr("class", "shapes");
    for (const shape of diagram.shapes) {
      drawShape(shapes, shape);
    }

    // The page draws no arrowhead where a flow ends at the first point of
    // another flow: the section "The diagram KDL" of the spec.
    const starts = new Set(
      diagram.flows.flatMap((f) => (f.points[0] === undefined ? [] : [pointKey(f.points[0])])),
    );
    const flows = viewport.append("g").attr("class", "flows");
    for (const flow of diagram.flows) {
      const last = flow.points[flow.points.length - 1];
      const key = flowKey(flow);
      const flowMarks = key === undefined ? undefined : marks?.elements.get(key);
      const pass = key === undefined ? undefined : marks?.flowPasses.get(key);
      // A back flow selects its repetition, and a back edge selects itself.
      const span = key === undefined ? undefined : flow.span;
      const selected = span !== undefined && (trace?.selected.has(spanKey(span)) ?? false);
      const group = flows
        .append("g")
        .attr("class", `flow ${flow.role}`)
        .attr("data-role", flow.role)
        .attr("data-start", flow.span?.start ?? null)
        .attr("data-end", flow.span?.end ?? null)
        .attr("data-option", flowMarks?.outline ?? null)
        .attr("data-pass", pass ?? null)
        .attr("data-selected", selected ? "" : null);
      if (selected) {
        group.append("path").attr("class", "selection-line").attr("d", roundedPath(flow.points));
      }
      if (span !== undefined) {
        // A wide path without paint takes the clicks near the thin line.
        group
          .classed("selectable", true)
          .on("click", () => trace?.selectElement(span))
          .append("path")
          .attr("class", "hit")
          .attr("d", roundedPath(flow.points));
      }
      group
        .append("path")
        .attr("class", "line")
        .attr("d", roundedPath(flow.points))
        .attr(
          "marker-end",
          last === undefined || starts.has(pointKey(last)) ? null : "url(#diagram-arrowhead)",
        );
      // A loop that holds the reading on view shows its current pass in the
      // label of its back flow or back edge. A back flow without a label of
      // the core gets one in the middle of its longest segment.
      const at = flow.label ?? (pass === undefined ? undefined : middleOfLongest(flow.points));
      if (at !== undefined) {
        const label = pass === undefined ? (flow.label?.text ?? "") : passLabel(flow, pass);
        // The label stands on a box of the background, which hides the flow
        // behind it. The box takes the measured width of the text.
        const background = group.append("rect").attr("class", "flow-label-background");
        const text = group
          .append("text")
          .attr("class", pass === undefined ? "flow-label" : "flow-label pass-label")
          .attr("x", at.x)
          .attr("y", at.y)
          .text(label);
        const width = text.node()?.getComputedTextLength() ?? 0;
        background
          .attr("x", at.x - width / 2 - labelPadding)
          .attr("y", at.y - lineHeight / 2)
          .attr("width", width + 2 * labelPadding)
          .attr("height", lineHeight)
          .attr("rx", 3);
      }
    }

    // At first the diagram fits the width of the view, at a scale of 1 at most.
    const available = frame.clientWidth;
    const scale = available > 0 ? Math.min(1, available / diagram.width) : 1;
    fitTransform = zoomIdentity.scale(scale);
    svg.attr("height", Math.max(Math.ceil(diagram.height * scale), 48));
    const behavior = zoom<SVGSVGElement, unknown>()
      .scaleExtent([0.1, 4])
      .clickDistance(4)
      // A wheel zooms only with Ctrl or Cmd, as a pinch on a trackpad sends,
      // so a plain wheel still scrolls the page. Otherwise d3's own filter.
      .filter((event: MouseEvent | WheelEvent) =>
        event.type === "wheel"
          ? event.ctrlKey || event.metaKey
          : !event.ctrlKey && !(event as MouseEvent).button,
      )
      .on("zoom", (event: { transform: ZoomTransform }) => {
        viewport.attr("transform", event.transform.toString());
        drawn = { rule: diagram.rule, transform: event.transform };
      });
    zoomBehavior = behavior;
    // A double click on a reference box opens its rule, so it never zooms.
    svg.call(behavior).on("dblclick.zoom", null);
    const keep = drawn?.rule === diagram.rule ? drawn.transform : fitTransform;
    svg.call(behavior.transform, keep);
  };

  const drawShape = (
    parent: Selection<SVGGElement, unknown, null, undefined>,
    shape: Shape,
  ): void => {
    switch (shape.type) {
      case "start-event":
      case "end-event": {
        const group = parent
          .append("g")
          .attr("class", "event")
          .attr("data-event", shape.type)
          .attr("role", "graphics-symbol");
        group
          .append("circle")
          .attr("class", shape.type)
          .attr("cx", shape.x + shape.size / 2)
          .attr("cy", shape.y + shape.size / 2)
          .attr("r", shape.size / 2);
        const name = shape.type === "start-event" ? "start event" : "end event";
        decorate(
          group,
          { x: shape.x, y: shape.y, width: shape.size, height: shape.size, round: true },
          shape.type,
          name,
          undefined,
        );
        return;
      }
      case "gateway": {
        const { x, y, size } = shape;
        const half = size / 2;
        const ofRepetition = repetitionGateways.has(shape.role);
        const span = shape.span;
        const selected = ofRepetition && (trace?.selected.has(spanKey(span)) ?? false);
        if (selected) {
          const cx = x + half;
          const cy = y + half;
          const r = half + selectionGap;
          parent
            .append("path")
            .attr("class", "selection")
            .attr("d", `M${cx - r} ${cy} L${cx} ${cy - r} L${cx + r} ${cy} L${cx} ${cy + r}Z`);
        }
        const gateway = parent
          .append("path")
          .attr("class", ofRepetition ? "gateway selectable" : "gateway")
          .attr("data-role", shape.role)
          .attr("data-start", span.start)
          .attr("data-end", span.end)
          .attr("data-selected", selected ? "" : null)
          .attr(
            "d",
            `M${x} ${y + half} L${x + half} ${y} L${x + size} ${y + half} L${x + half} ${y + size}Z`,
          );
        // A gateway of a repetition selects the repetition. A choice gateway
        // and an optional part select nothing.
        if (ofRepetition) {
          gateway.on("click", () => trace?.selectElement(span));
        }
        return;
      }
      case "task":
      case "reference":
        drawBox(parent, shape);
        return;
    }
  };

  const drawBox = (parent: Selection<SVGGElement, unknown, null, undefined>, box: Box): void => {
    const drill = box.type === "reference" && !box.oneStep;
    const group = parent
      .append("g")
      .attr("class", `box ${box.type}`)
      .attr("role", "graphics-symbol")
      .attr("aria-label", boxLabel(box))
      .attr("data-name", boxLabel(box))
      .attr("tabindex", "0")
      .attr("data-start", box.span.start)
      .attr("data-end", box.span.end);
    group
      .append("rect")
      .attr("class", "frame")
      .attr("x", box.x)
      .attr("y", box.y)
      .attr("width", box.width)
      .attr("height", box.height)
      .attr("rx", 9);
    // The lines of a label stand in the centre of the box, above the band of
    // its drill-down marker.
    const centreX = box.x + box.width / 2;
    const centreY = box.y + (box.height - (drill ? drillBand : 0)) / 2;
    const top = centreY - (box.label.length * lineHeight) / 2;
    box.label.forEach((line, i) => {
      group
        .append("text")
        .attr("class", "label")
        .attr("x", centreX)
        .attr("y", top + i * lineHeight + lineHeight / 2)
        .text(line);
    });
    decorate(group, { ...box, round: false }, boxKey(box), boxLabel(box), box);
    // A click on a box selects it. The amber outline stands behind every mark.
    group.on("click", () => trace?.selectElement(box.span));
    if (trace?.selected.has(spanKey(box.span)) === true) {
      group
        .attr("data-selected", "")
        .attr("aria-label", `${group.attr("aria-label")}, selected`)
        .insert("rect", ":first-child")
        .attr("class", "selection")
        .attr("x", box.x - selectionGap)
        .attr("y", box.y - selectionGap)
        .attr("width", box.width + 2 * selectionGap)
        .attr("height", box.height + 2 * selectionGap)
        .attr("rx", 14);
    }
    const focus = trace?.focus;
    if (focus !== undefined && marks?.dataObject.includes(boxKey(box)) === true) {
      drawDataObject(group, box, focus);
    }
    if (box.type !== "reference") {
      return;
    }
    const rule = box.rule;
    group
      .attr("data-rule", rule)
      .attr("data-one-step", String(box.oneStep))
      .attr("data-loop-link", box.loopLink ?? null)
      .on("dblclick", (event: MouseEvent) => {
        event.preventDefault();
        drillDown(rule);
      })
      .on("keydown", (event: KeyboardEvent) => {
        if (event.key === "Enter") {
          event.preventDefault();
          drillDown(rule, true);
        }
      });
    if (drill) {
      // The marker of a collapsed sub-process in BPMN: a square with a plus.
      const mx = centreX - 6;
      const my = box.y + box.height - 15;
      const marker = group
        .append("g")
        .attr("class", "drill-down")
        .on("click", (event: MouseEvent) => {
          event.stopPropagation();
          drillDown(rule);
        });
      marker.append("title").text(`Open ${rule}`);
      marker.append("rect").attr("x", mx).attr("y", my).attr("width", 12).attr("height", 12);
      marker
        .append("path")
        .attr(
          "d",
          `M${mx + 3} ${my + 6} L${mx + 9} ${my + 6} M${mx + 6} ${my + 3} L${mx + 6} ${my + 9}`,
        );
    }
    if (box.loopLink !== undefined) {
      // The loop sign of a link of a loop through rules, in the lower left corner.
      group
        .append("text")
        .attr("class", "loop-sign")
        .attr("x", box.x + 7)
        .attr("y", box.y + box.height - 6)
        .text("↻");
    }
  };

  /**
   * Draws the marks of one shape: the reading markers as numbered tabs in the
   * free band above it, the outline of a next-option mark and its ▸ button.
   * The label of the shape names its marks. `box` is the box, for a box.
   */
  const decorate = (
    group: Selection<SVGGElement, unknown, null, undefined>,
    bounds: Bounds,
    key: ElementKey,
    name: string,
    box: Box | undefined,
  ): void => {
    const shapeMarks = marks?.elements.get(key);
    const words = marksWords(shapeMarks);
    group.attr("aria-label", words === "" ? name : `${name}, ${words}`);
    if (shapeMarks === undefined) {
      return;
    }
    group
      .attr("data-marker", shapeMarks.markers.length > 0 ? shapeMarks.markers.join(" ") : null)
      .attr("data-option", shapeMarks.outline ?? null);
    if (shapeMarks.outline !== undefined) {
      // The outline stands behind the shape, a few pixels outside it.
      if (bounds.round) {
        group
          .insert("circle", ":first-child")
          .attr("class", "option-mark")
          .attr("data-style", shapeMarks.outline)
          .attr("cx", bounds.x + bounds.width / 2)
          .attr("cy", bounds.y + bounds.height / 2)
          .attr("r", bounds.width / 2 + outlineGap);
      } else {
        group
          .insert("rect", ":first-child")
          .attr("class", "option-mark")
          .attr("data-style", shapeMarks.outline)
          .attr("x", bounds.x - outlineGap)
          .attr("y", bounds.y - outlineGap)
          .attr("width", bounds.width + 2 * outlineGap)
          .attr("height", bounds.height + 2 * outlineGap)
          .attr("rx", 12);
      }
    }
    drawTabs(group, bounds, shapeMarks.markers);
    // A next-option mark on a box or on the end event carries a ▸ button.
    if (shapeMarks.outline !== undefined && (box !== undefined || key === "end-event")) {
      drawNextButton(group, bounds, shapeMarks, name, box);
    }
  };

  /**
   * Draws the data object of the step in focus in the free band above its
   * box, joined to the box by a dotted association. It shows the count of
   * attributes, and a click on it selects the step.
   */
  const drawDataObject = (
    group: Selection<SVGGElement, unknown, null, undefined>,
    box: Box,
    focus: { readonly step: number; readonly attributes: number },
  ): void => {
    const x = box.x + box.width / 2 - dataWidth / 2;
    const y = box.y - freeBand + 2;
    const name = `step ${focus.step}, ${focus.attributes} ${focus.attributes === 1 ? "attribute" : "attributes"}`;
    const data = group
      .append("g")
      .attr("class", "data-object")
      .attr("role", "button")
      .attr("aria-label", `Select ${name}`)
      .attr("data-step", focus.step)
      .attr("data-count", focus.attributes)
      .on("click", (event: MouseEvent) => {
        event.stopPropagation();
        trace?.selectStep(focus.step);
      });
    data.append("title").text(`Select ${name}`);
    data
      .append("path")
      .attr("class", "association")
      .attr("d", `M${x + dataWidth / 2} ${y + dataHeight} L${x + dataWidth / 2} ${box.y}`);
    data
      .append("path")
      .attr("class", "page")
      .attr(
        "d",
        `M${x} ${y} h${dataWidth - dataFold} l${dataFold} ${dataFold} v${dataHeight - dataFold} h${-dataWidth}z`,
      );
    data
      .append("path")
      .attr("class", "page-fold")
      .attr("d", `M${x + dataWidth - dataFold} ${y} v${dataFold} h${dataFold}`);
    data
      .append("text")
      .attr("class", "data-count")
      .attr("x", x + dataWidth + 4)
      .attr("y", y + dataHeight / 2 + 2)
      .text(String(focus.attributes));
  };

  const drawTabs = (
    group: Selection<SVGGElement, unknown, null, undefined>,
    bounds: Bounds,
    markers: readonly number[],
  ): void => {
    if (markers.length === 0) {
      return;
    }
    const shown = markers.slice(0, maxTabs);
    const rest = markers.length - shown.length;
    const widths = [...shown.map(() => tabWidth), ...(rest > 0 ? [moreTabWidth] : [])];
    const total = widths.reduce((a, b) => a + b, 0) + tabGap * (widths.length - 1);
    // On a box, the tabs start near its left corner. Above an event, they stand in the middle.
    let x = bounds.round ? bounds.x + bounds.width / 2 - total / 2 : bounds.x + tabInset;
    const bottom = bounds.y;
    const top = bottom - tabPoint - tabHeight;
    const tab = (
      width: number,
      text: string,
      filled: boolean,
      reading: number | undefined,
    ): void => {
      const g = group
        .append("g")
        .attr("class", reading === undefined ? "marker-tab marker-more" : "marker-tab")
        .attr("data-reading", reading ?? null)
        .attr("data-filled", String(filled));
      g.append("path")
        .attr("class", "tab-shape")
        .attr(
          "d",
          `M${x} ${top} h${width} v${tabHeight} L${x + width / 2} ${bottom} L${x} ${top + tabHeight}Z`,
        );
      g.append("text")
        .attr("class", "tab-number")
        .attr("x", x + width / 2)
        .attr("y", top + tabHeight / 2)
        .text(text);
      x += width + tabGap;
    };
    for (const reading of shown) {
      tab(tabWidth, String(reading), reading === trace?.onView, reading);
    }
    if (rest > 0) {
      tab(moreTabWidth, `+${rest}`, false, undefined);
    }
  };

  const drawNextButton = (
    group: Selection<SVGGElement, unknown, null, undefined>,
    bounds: Bounds,
    shapeMarks: ElementMarks,
    name: string,
    box: Box | undefined,
  ): void => {
    const symbols = [...shapeMarks.options.keys()];
    // On a task box, the button appends the symbol of the box. Elsewhere one
    // next option appends directly, and more open a popover.
    const direct =
      box?.type === "task"
        ? box.symbol
        : symbols.length === 1 && shapeMarks.canEnd === undefined
          ? symbols[0]
          : undefined;
    const x = bounds.round
      ? bounds.x + bounds.width - buttonSize / 2
      : bounds.x + bounds.width - buttonSize / 2 - 2;
    const y = bounds.y - buttonSize / 2 - (bounds.round ? 0 : 2);
    const button = group
      .append("g")
      .attr("class", "next-button")
      .attr("role", "button")
      .attr("data-style", shapeMarks.outline ?? null)
      .attr(
        "aria-label",
        direct === undefined ? `Next options through ${name}` : `Append ${direct}`,
      )
      .attr("data-symbol", direct ?? null);
    button
      .append("title")
      .text(direct === undefined ? `Next options through ${name}` : `Append ${direct}`);
    button
      .append("rect")
      .attr("x", x)
      .attr("y", y)
      .attr("width", buttonSize)
      .attr("height", buttonSize)
      .attr("rx", 3);
    button
      .append("text")
      .attr("x", x + buttonSize / 2)
      .attr("y", y + buttonSize / 2)
      .text("▸");
    const activate = (anchor: Element): void => {
      if (direct !== undefined) {
        closePopover();
        trace?.choose(direct);
      } else {
        openPopover(anchor, name, shapeMarks);
      }
    };
    button.on("click", (event: MouseEvent) => {
      event.stopPropagation();
      activate(event.currentTarget as Element);
    });
    // The ▸ button is not a stop of the Tab key, so the Tab key still moves
    // through the boxes. Space on a focused box, or on a marked end event,
    // presses its button.
    if (box === undefined) {
      group.attr("tabindex", "0");
    }
    group.on("keydown.next", (event: KeyboardEvent) => {
      if (event.key === " ") {
        event.preventDefault();
        const node = button.node();
        if (node !== null) {
          activate(node);
        }
      }
    });
  };

  /** Lists the next options through an element, and "end" when the sequence can end. */
  const openPopover = (anchor: Element, name: string, shapeMarks: ElementMarks): void => {
    const popover = elements.popover;
    popover.replaceChildren();
    popover.setAttribute("aria-label", `Next options through ${name}`);
    const heading = document.createElement("p");
    heading.className = "popover-heading";
    heading.textContent = `Next options through ${name}`;
    const list = document.createElement("ul");
    list.className = "option-list";
    for (const [symbol, style] of shapeMarks.options) {
      const item = document.createElement("li");
      const button = document.createElement("button");
      button.type = "button";
      button.className = "popover-option";
      button.dataset.symbol = symbol;
      button.dataset.style = style;
      button.textContent = `▸ ${symbol}`;
      button.addEventListener("click", () => {
        closePopover();
        trace?.choose(symbol);
      });
      item.append(button);
      list.append(item);
    }
    if (shapeMarks.canEnd !== undefined) {
      const item = document.createElement("li");
      item.className = "popover-end";
      item.dataset.style = shapeMarks.canEnd;
      item.textContent = "end: the sequence can end here";
      list.append(item);
    }
    popover.append(heading, list);
    // The popover stands beside the button, and it stays inside the viewport,
    // so the options of a button near an edge stay visible.
    popover.hidden = false;
    const rect = anchor.getBoundingClientRect();
    const margin = 8;
    const maxLeft = window.innerWidth - popover.offsetWidth - margin;
    const maxTop = window.innerHeight - popover.offsetHeight - margin;
    popover.style.left = `${Math.round(Math.max(margin, Math.min(rect.right + 4, maxLeft)))}px`;
    popover.style.top = `${Math.round(Math.max(margin, Math.min(rect.top, maxTop)))}px`;
    popover.querySelector("button")?.focus();
  };

  const closePopover = (): void => {
    elements.popover.hidden = true;
    elements.popover.replaceChildren();
  };
  elements.popover.addEventListener("keydown", (event) => {
    if (event.key === "Escape") {
      closePopover();
    }
  });
  document.addEventListener("pointerdown", (event) => {
    const target = event.target;
    if (
      !elements.popover.hidden &&
      target instanceof Node &&
      !elements.popover.contains(target) &&
      !(target instanceof Element && target.closest(".next-button") !== null)
    ) {
      closePopover();
    }
  });

  /** The banner of a failure point, the readings outside the diagram and the unlisted options. */
  const renderTraceNotes = (): void => {
    const banner = trace?.banner;
    elements.banner.hidden = banner === undefined;
    elements.banner.textContent = banner ?? "";
    const outside = marks === undefined ? undefined : outsideText(marks.outside);
    elements.outside.hidden = outside === undefined;
    elements.outside.textContent = outside ?? "";
    const unlisted = marks?.unlisted ?? [];
    elements.unlisted.hidden = unlisted.length === 0;
    elements.unlisted.replaceChildren();
    if (unlisted.length > 0) {
      elements.unlisted.append("Next options that no listed reading allows: ");
      for (const symbol of unlisted) {
        const button = document.createElement("button");
        button.type = "button";
        button.className = "unlisted-option";
        button.dataset.symbol = symbol;
        button.textContent = `▸ ${symbol}`;
        button.addEventListener("click", () => trace?.choose(symbol));
        elements.unlisted.append(button, " ");
      }
    }
  };

  /** The marks of the open path for the trace on view, or none. */
  const computeMarks = (m: Model): MarksView | undefined => {
    if (trace === undefined || stale || !isOpenPath(m.graph, path)) {
      return undefined;
    }
    const outcome = trace.marks(path);
    if (outcome.type !== "marks") {
      return undefined;
    }
    return marksView(outcome.marks, trace.onView, trace.count, trace.order);
  };

  const zoomBy = (factor: number): void => {
    if (svgSelection !== undefined && zoomBehavior !== undefined) {
      svgSelection.call(zoomBehavior.scaleBy, factor);
    }
  };
  elements.zoomIn.addEventListener("click", () => zoomBy(1.25));
  elements.zoomOut.addEventListener("click", () => zoomBy(0.8));
  elements.fit.addEventListener("click", () => {
    if (svgSelection !== undefined && zoomBehavior !== undefined) {
      svgSelection.call(zoomBehavior.transform, fitTransform);
    }
  });

  const showNote = (text: string | undefined): void => {
    elements.note.hidden = text === undefined;
    elements.note.textContent = text ?? "";
  };

  const render = (): void => {
    elements.root.dataset.stale = String(stale);
    closePopover();
    const m = model;
    const rule = path[path.length - 1];
    const diagram = rule === undefined ? undefined : m?.diagrams.get(rule);
    const controls = [elements.zoomIn, elements.zoomOut, elements.fit];
    if (m === undefined || rule === undefined || diagram === undefined) {
      showNote("The grammar has errors, so it has no diagrams.");
      for (const part of [elements.index, elements.crumbs, elements.doc, elements.frame]) {
        part.replaceChildren();
      }
      elements.title.textContent = "No rule";
      elements.badge.hidden = true;
      for (const control of controls) {
        control.disabled = true;
      }
      svgSelection = undefined;
      zoomBehavior = undefined;
      marks = undefined;
      renderTraceNotes();
      return;
    }
    showNote(
      stale
        ? "The grammar text has errors. The diagrams show the last text without errors."
        : undefined,
    );
    for (const control of controls) {
      control.disabled = false;
    }
    marks = computeMarks(m);
    renderIndex(m);
    renderCrumbs();
    renderHeader(m, rule);
    renderTraceNotes();
    drawDiagram(diagram);
  };

  const update = (checked: CheckedGrammar, outcome: DiagramsOutcome): void => {
    // A new grammar text gets a new session, so the marks of the last one go.
    trace = undefined;
    if (outcome.type === "grammar-errors") {
      stale = model !== undefined;
      render();
      return;
    }
    const graph = ruleGraph(checked);
    model = {
      checked,
      diagrams: new Map(outcome.diagrams.diagrams.map((d) => [d.rule, d])),
      graph,
      badges: badgesOf(checked),
    };
    stale = false;
    // The open path stays when it is still an open path of the new grammar.
    // Otherwise its last rule opens as from the rule index, or else the start rule.
    if (!isOpenPath(graph, path)) {
      const last = path[path.length - 1];
      const rule = last !== undefined && graph.references.has(last) ? last : graph.start;
      path = rule === undefined ? [] : indexPath(graph, rule);
    }
    render();
  };

  const showTrace = (next: TraceMarks | undefined, open: OpenChange): void => {
    trace = next;
    if (open !== "keep" && next !== undefined && model !== undefined) {
      // When the state has no reading, the open path holds the start rule.
      const start = model.graph.start;
      const target =
        open !== "follow"
          ? open
          : next.onView > 0
            ? next.followPath
            : start === undefined
              ? []
              : [start];
      if (isOpenPath(model.graph, target)) {
        path = [...target];
      }
    }
    render();
  };

  return { update, openPath: () => path, showTrace };
}
