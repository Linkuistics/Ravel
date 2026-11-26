// The output view: the tree of one reading as an outline, with a header that
// counts the readings, the next options, the failure point and the steps that
// the tracer has not read. docs/specs/output-view.md decides each part.
//
// The page draws the outline with HTML elements and a D3 data join, as an
// ARIA tree. Each row exposes its kind, its label, its step range and its
// fold state as data attributes, so the end-to-end tests read structure.

import { select } from "d3";
import type { Attribute, CheckedGrammar, SessionState, TreeNode } from "./core.ts";
import {
  type FoldState,
  type GrammarFacts,
  type LabelPart,
  outlineRows,
  quoted,
  type Row,
  stepNumbers,
} from "./outline.ts";
import { countReferences, folderPath, referenceOf } from "./references.ts";
import { matchesElement, type Selection } from "./selection.ts";

export interface OutputViewElements {
  readonly count: HTMLElement;
  readonly status: HTMLElement;
  readonly notes: HTMLElement;
  readonly previous: HTMLButtonElement;
  readonly next: HTMLButtonElement;
  readonly back: HTMLButtonElement;
  readonly forward: HTMLButtonElement;
  readonly tree: HTMLElement;
}

/** What the output view shows. */
export interface OutputModel {
  readonly state: SessionState;
  readonly grammar: CheckedGrammar;
  readonly facts: GrammarFacts;
  /** The reading on view, from 1. It is 0 when the state has no reading. */
  readonly reading: number;
  readonly tree: TreeNode | undefined;
  readonly folds: FoldState;
  /** A note when the view shows no extension of the reading on view before the last step. */
  readonly ended: string | undefined;
  /** The selection: a row of the view or a grammar element. */
  readonly selection: Selection | undefined;
}

export interface OutputHandlers {
  /** Inserts a next option right after the state on view, and reads it. */
  readonly choose: (symbol: string) => void;
  readonly toggle: (row: ViewRow) => void;
  /** Selects a row, which becomes the row in focus of the details panel. */
  readonly select: (row: ViewRow) => void;
}

/** A next option as a chip. A solid chip is a next option of the reading on view. */
interface Chip {
  readonly symbol: string;
  readonly solid: boolean;
}

/** An attribute of a step as a chip on its row: a short form of its value. */
export interface AttributeChip {
  readonly key: string;
  /** The short form, such as `exercise-1.md` or `level=2`. A last chip "+2" counts the rest. */
  readonly text: string;
  /** The kind of reference, or "attribute" for a value that is not a reference. */
  readonly kind: "web" | "file" | "text" | "attribute" | "more";
  /** The whole attribute, for the tooltip. */
  readonly title: string;
}

/** A row of the view: a row of the tree, or one of the rows after it. */
export interface ViewRow {
  readonly key: string;
  readonly kinds: readonly string[];
  readonly level: number;
  /** The indent, in steps. The rows that belong to no node have none. */
  readonly indent: number;
  readonly parts: readonly LabelPart[];
  readonly right: string;
  readonly gutter: string;
  readonly fold: "open" | "folded" | undefined;
  readonly onPath: boolean;
  readonly last: boolean;
  /** The key of the fold choice of the row, as in `Row`. */
  readonly toggleKey: string;
  readonly chips: readonly Chip[];
  /** The nodes that the row holds, as in `Row`. The rows after the tree hold none. */
  readonly nodes: readonly TreeNode[];
  /** The passes that a fold row of the tree stands for. */
  readonly folded: readonly TreeNode[];
  /** The number of the step that the row shows, if it shows one. */
  readonly step: number | undefined;
  /** The attribute chips of that step. */
  readonly attributes: readonly AttributeChip[];
  /** The ancestors of the first node of the row, as in `Row`. */
  readonly ancestors: readonly TreeNode[];
  /**
   * Whether the row is selected: the selected row, or a row that matches the
   * selected grammar element.
   */
  readonly selected: boolean;
  /** For a folded row, the number of selected rows that it hides. */
  readonly inside: number;
}

const plural = (count: number, one: string, many: string): string =>
  `${count} ${count === 1 ? one : many}`;

/** The reading on view of a state, as the page remembers it for the next state. */
export interface Viewed {
  readonly grammar: string;
  /** The symbols of the steps read. */
  readonly read: readonly string[];
  readonly readings: SessionState["readings"];
  /** The reading on view, from 1, or 0 without a reading. */
  readonly reading: number;
}

const startsWith = (whole: readonly string[], start: readonly string[]): boolean =>
  start.every((symbol, i) => whole[i] === symbol);

/**
 * The reading to show for a new state, by the section "Readings" of the spec.
 * A step that extends the reading on view shows its first extension. A
 * removed last step shows the reading that the reading on view extended.
 */
export function chooseReading(
  before: Viewed | undefined,
  grammar: string,
  state: SessionState,
): { readonly reading: number; readonly ended: string | undefined } {
  const n = state.readings.length;
  if (n === 0) {
    return { reading: 0, ended: undefined };
  }
  if (before === undefined || before.grammar !== grammar || before.reading === 0) {
    return { reading: 1, ended: undefined };
  }
  const read = state.steps.slice(0, state.stepsRead).map((s) => s.symbol);
  const k = before.read.length;
  if (read.length === k + 1 && startsWith(read, before.read)) {
    const i = state.readings.findIndex((r) => r.parent === before.reading);
    if (i >= 0) {
      return { reading: i + 1, ended: undefined };
    }
    // The extensions of a reading keep the order of the readings that they
    // extend, and the cap keeps the start of that order. So a listed reading
    // that extends a later or unlisted reading proves that no extension exists.
    const ended =
      !state.capReached ||
      state.readings.some((r) => r.parent === undefined || r.parent > before.reading);
    return {
      reading: 1,
      ended: ended
        ? `Reading ${before.reading} ended at step ${k}, so the view shows reading 1.`
        : `The cap can leave out the extensions of reading ${before.reading}, so the view shows reading 1.`,
    };
  }
  if (read.length === k - 1 && startsWith(before.read, read)) {
    const parent = before.readings[before.reading - 1]?.parent;
    return { reading: parent !== undefined && parent <= n ? parent : 1, ended: undefined };
  }
  if (read.length === k && startsWith(read, before.read)) {
    return { reading: Math.min(before.reading, n), ended: undefined };
  }
  return { reading: 1, ended: undefined };
}

/** The chips that a row shows at most, before a last chip "+n" counts the rest. */
const maxAttributeChips = 3;

/** The text of an attribute value, as KDL writes it. */
export function valueText(attribute: Attribute): string {
  const value = attribute.value;
  switch (value.type) {
    case "string":
    case "integer":
    case "float":
      return value.value;
    case "bool":
      return value.value ? "#true" : "#false";
    case "null":
      return "#null";
  }
}

const shortened = (text: string, length: number): string =>
  text.length <= length ? text : `${text.slice(0, length - 1)}…`;

/** The whole attribute as one line, such as `exercise=(url-reference)"a.md"`. */
export function attributeLine(attribute: Attribute): string {
  const annotation = attribute.annotation === undefined ? "" : `(${attribute.annotation})`;
  const value = valueText(attribute);
  return `${attribute.key}=${annotation}${attribute.value.type === "string" ? quoted(value) : value}`;
}

/**
 * The chips of the attributes of a step: the references first, then the
 * other attributes, each group ordered by key. The section "Step attributes"
 * of the spec decides the order.
 */
export function attributeChips(attributes: readonly Attribute[]): AttributeChip[] {
  const chips = attributes.map((attribute): AttributeChip & { readonly first: boolean } => {
    const reference = referenceOf(attribute);
    const title = attributeLine(attribute);
    if (reference === undefined) {
      return {
        key: attribute.key,
        text: shortened(`${attribute.key}=${valueText(attribute)}`, 24),
        kind: "attribute",
        title,
        first: false,
      };
    }
    let text = valueText(attribute);
    if (reference.type === "web") {
      text = new URL(reference.href).host;
    } else if (reference.type === "file") {
      text = (folderPath(reference.path) ?? reference.path).split("/").at(-1) || reference.path;
    }
    return {
      key: attribute.key,
      text: shortened(text, 24),
      kind: reference.type,
      title,
      first: true,
    };
  });
  chips.sort((a, b) =>
    a.first === b.first ? (a.key < b.key ? -1 : a.key > b.key ? 1 : 0) : a.first ? -1 : 1,
  );
  const shown = chips.slice(0, maxAttributeChips).map(({ first: _, ...chip }) => chip);
  const rest = chips.length - shown.length;
  if (rest > 0) {
    shown.push({
      key: "",
      text: `+${rest}`,
      kind: "more",
      title: `${plural(rest, "more attribute", "more attributes")} in the details panel`,
    });
  }
  return shown;
}

/** The counts of the references of some steps, such as "· 2 files · 1 link", or nothing. */
function referenceParts(state: SessionState, steps: readonly number[]): LabelPart[] {
  const { files, links } = countReferences(
    steps.flatMap((n) => state.steps[n - 1]?.attributes ?? []),
  );
  return [
    ...(files > 0 ? [{ kind: "note" as const, text: `· ${plural(files, "file", "files")}` }] : []),
    ...(links > 0 ? [{ kind: "note" as const, text: `· ${plural(links, "link", "links")}` }] : []),
  ];
}

/** The step that a row of the tree shows: the step of its innermost node. */
function rowStep(row: Row): number | undefined {
  const node = row.nodes.at(-1);
  return node?.kind.type === "step" && row.folded.length === 0 ? node.kind.number : undefined;
}

/** The part of a row that the view adds to a row of the tree. */
function fromRow(row: Row, model: OutputModel): ViewRow {
  const step = rowStep(row);
  const counts = referenceParts(model.state, row.folded.flatMap(stepNumbers));
  const selection = model.selection;
  const inside: LabelPart[] =
    row.inside > 0 ? [{ kind: "note", text: `· ${row.inside} selected` }] : [];
  return {
    ...row,
    parts: [...row.parts, ...counts, ...inside],
    indent: row.level - 1,
    chips: [],
    step,
    attributes:
      step === undefined ? [] : attributeChips(model.state.steps[step - 1]?.attributes ?? []),
    selected: selection?.type === "row" ? row.toggleKey === selection.toggleKey : row.matched,
  };
}

/** The fields of a row after the tree, which holds no node. */
function afterTree(
  model: OutputModel,
  key: string,
  step: number | undefined,
): Pick<
  ViewRow,
  | "key"
  | "toggleKey"
  | "nodes"
  | "folded"
  | "step"
  | "attributes"
  | "ancestors"
  | "selected"
  | "inside"
> {
  return {
    key,
    toggleKey: key,
    nodes: [],
    folded: [],
    step,
    attributes:
      step === undefined ? [] : attributeChips(model.state.steps[step - 1]?.attributes ?? []),
    ancestors: [],
    selected: model.selection?.type === "row" && key === model.selection.toggleKey,
    inside: 0,
  };
}

/** The key of the row of the steps that the tracer has not read. */
export const unreadKey = "unread";

/** The words "a, b and c" for a list of names. */
function listText(names: readonly string[]): string {
  if (names.length <= 1) {
    return names.join("");
  }
  return `${names.slice(0, -1).join(", ")} and ${names.at(-1)}`;
}

/** The reason that no reading accepts a symbol at the failure point. */
export function rejectionReason(grammar: CheckedGrammar, symbol: string): string {
  const found = grammar.symbols.find((s) => s.value === symbol);
  if (found === undefined) {
    return `The grammar has no symbol ${quoted(symbol)}.`;
  }
  const rules = [...new Set(found.uses.map((u) => u.rule))];
  return `The grammar uses ${quoted(symbol)} in ${listText(rules)}, but not at this point.`;
}

/** The text of the count of readings, such as "Reading 1 of 3" or "Reading 1 of 10+". */
export function countText(state: SessionState, reading: number): string {
  const n = state.readings.length;
  if (n === 0) {
    return "no reading";
  }
  return `Reading ${reading} of ${n}${state.capReached ? "+" : ""}`;
}

/** The status of the sequence: whether it can end, or where the tracer rejected it. */
export function statusText(state: SessionState): string {
  if (state.failurePoint !== undefined) {
    return `rejected at step ${state.failurePoint} · showing the state after step ${state.stepsRead}`;
  }
  const total = state.steps.length;
  const read =
    state.stepsRead < total
      ? `${state.stepsRead} of ${plural(total, "step", "steps")} read`
      : plural(total, "step", "steps");
  return `${read} · ${state.canEnd ? "can end here" : "cannot end yet"}`;
}

/** Every row of the view, from the top. */
export function viewRows(model: OutputModel): ViewRow[] {
  const { state } = model;
  const rows: ViewRow[] = [];
  const hasReading = model.reading > 0 && model.tree !== undefined;
  if (hasReading && model.tree !== undefined) {
    // A selected grammar element selects every row that holds a node with
    // the element among its origins.
    const selection = model.selection;
    const match =
      selection?.type === "element"
        ? (node: TreeNode) => matchesElement(node, selection.span)
        : undefined;
    rows.push(
      ...outlineRows(model.tree, state.stepsRead, model.facts, model.folds, match).map((row) =>
        fromRow(row, model),
      ),
    );
  }

  const k = state.failurePoint;
  if (k !== undefined) {
    const symbol = state.steps[k - 1]?.symbol ?? "";
    rows.push({
      ...afterTree(model, "rejected", k),
      kinds: ["rejected"],
      level: 1,
      indent: 0,
      parts: [
        { kind: "step", text: quoted(symbol) },
        { kind: "note", text: rejectionReason(model.grammar, symbol) },
      ],
      right: "",
      gutter: `${k}`,
      fold: undefined,
      onPath: false,
      last: false,
      chips: [],
    });
  }

  if (hasReading) {
    const ofReading = new Set(
      (state.readings[model.reading - 1]?.nextOptions ?? []).map((o) => o.symbol),
    );
    const chips = state.nextOptions.map((o) => ({
      symbol: o.symbol,
      solid: ofReading.has(o.symbol),
    }));
    const end = state.canEnd ? (chips.length > 0 ? "or end here" : "end here") : "";
    const nextStep = state.stepsRead + 1;
    rows.push({
      ...afterTree(model, "next", undefined),
      kinds: ["next"],
      level: 1,
      indent: 0,
      parts: [
        { kind: "note", text: k === undefined ? "next" : `next at ${k}` },
        ...(chips.length === 0 && end === "" ? [{ kind: "note" as const, text: "no step" }] : []),
      ],
      right: end,
      gutter: `${nextStep}`,
      fold: undefined,
      onPath: false,
      last: false,
      chips,
    });
  }

  // The steps after the state on view that the tracer has not read.
  const firstUnread = (k ?? state.stepsRead) + 1;
  const unread = state.steps.slice(firstUnread - 1);
  if (unread.length > 0) {
    const open = model.folds.opened.has(unreadKey);
    const lastUnread = firstUnread + unread.length - 1;
    rows.push({
      ...afterTree(model, unreadKey, undefined),
      kinds: ["unread", "fold"],
      level: 1,
      indent: 0,
      parts: [
        {
          kind: "note",
          text: `${unread.length === 1 ? "1 later step" : `${unread.length} later steps`}, not read`,
        },
        ...referenceParts(
          state,
          unread.map((_, i) => firstUnread + i),
        ),
      ],
      right: "",
      gutter: unread.length === 1 ? `${firstUnread}` : `${firstUnread}–${lastUnread}`,
      fold: open ? "open" : "folded",
      onPath: false,
      last: false,
      chips: [],
    });
    if (open) {
      unread.forEach((step, i) => {
        rows.push({
          ...afterTree(model, `${unreadKey}.${i}`, firstUnread + i),
          kinds: ["unread", "step"],
          level: 2,
          indent: 0,
          parts: [{ kind: "step", text: quoted(step.symbol) }],
          right: "",
          gutter: `${firstUnread + i}`,
          fold: undefined,
          onPath: false,
          last: false,
          chips: [],
        });
      });
    }
  }
  return rows;
}

/** Whether a person can fold or open a row. The rows on the path to the last step stay open. */
export const canToggle = (row: ViewRow): boolean =>
  row.fold === "folded" || (row.fold === "open" && !row.onPath);

const labelText = (row: ViewRow): string => row.parts.map((p) => p.text).join(" ");

/** Whether the tree shows its end, so that it follows the last step. */
const following = new WeakMap<HTMLElement, boolean>();

/** Shows a model in the output view, and returns the rows that it shows. */
export function renderOutputView(
  elements: OutputViewElements,
  model: OutputModel,
  handlers: OutputHandlers,
): ViewRow[] {
  const { state } = model;
  elements.count.textContent = countText(state, model.reading);
  elements.status.textContent = statusText(state);
  const notes = [
    ...(state.capReached
      ? [
          `The tracer stopped at the reading cap of ${state.readings.length} readings, so more readings exist.`,
        ]
      : []),
    ...(model.ended === undefined ? [] : [model.ended]),
  ];
  select(elements.notes)
    .selectAll<HTMLLIElement, string>("li")
    .data(notes)
    .join("li")
    .text((d) => d);
  elements.previous.disabled = model.reading <= 1;
  elements.next.disabled = model.reading === 0 || model.reading >= state.readings.length;
  elements.back.disabled = state.stepsRead === 0;
  elements.forward.disabled = state.stepsRead >= state.steps.length;
  const rows = viewRows(model);
  renderRows(elements.tree, rows, handlers);
  return rows;
}

/**
 * The row in focus of the details panel: the selected row, or else the row
 * of the last step that the tracer read. When the selection is a grammar
 * element, it is the last matching row, or the fold row that holds it.
 */
export function rowInFocus(rows: readonly ViewRow[]): ViewRow | undefined {
  return rows.findLast((r) => r.selected || r.inside > 0) ?? rows.find((r) => r.last);
}

/** Scrolls the outline so that a row shows. */
export function revealRow(tree: HTMLElement, key: string): void {
  tree.querySelector(`[role=treeitem][data-key="${CSS.escape(key)}"]`)?.scrollIntoView({
    block: "nearest",
  });
}

/** Shows that no session is open: an empty view. */
export function renderNoOutput(elements: OutputViewElements): void {
  elements.count.textContent = "no reading";
  elements.status.textContent = "";
  select(elements.notes).selectAll("li").remove();
  for (const button of [elements.previous, elements.next, elements.back, elements.forward]) {
    button.disabled = true;
  }
  select(elements.tree).selectAll(".row").remove();
}

function renderRows(tree: HTMLElement, rows: readonly ViewRow[], handlers: OutputHandlers): void {
  if (!following.has(tree)) {
    following.set(tree, true);
    tree.addEventListener("scroll", () => {
      following.set(tree, tree.scrollTop + tree.clientHeight >= tree.scrollHeight - 8);
    });
    tree.addEventListener("keydown", (event) => onKey(tree, event, handlers));
  }
  const focusedKey =
    document.activeElement instanceof HTMLElement && tree.contains(document.activeElement)
      ? document.activeElement.closest<HTMLElement>("[role=treeitem]")?.dataset.key
      : undefined;
  const follow = following.get(tree) ?? true;

  const items = select(tree)
    .selectAll<HTMLDivElement, ViewRow>("div.row")
    .data(rows, (d) => d.key)
    .join("div")
    .attr("class", "row")
    .attr("role", "treeitem")
    .attr("tabindex", -1)
    .attr("aria-level", (d) => d.level)
    .attr("aria-expanded", (d) => (d.fold === undefined ? null : String(d.fold === "open")))
    .attr("aria-selected", (d) => String(d.selected))
    .attr("aria-label", (d) =>
      [
        d.gutter,
        labelText(d),
        d.attributes.length === 0 ? "" : `attributes ${d.attributes.map((a) => a.text).join(", ")}`,
        d.right,
      ]
        .filter((t) => t !== "")
        .join(" "),
    )
    .attr("data-key", (d) => d.key)
    .attr("data-kind", (d) => d.kinds.join(" "))
    .attr("data-label", labelText)
    .attr("data-steps", (d) => (d.gutter === "" ? null : d.gutter))
    .attr("data-fold", (d) => d.fold ?? null)
    .attr("data-path", (d) => (d.onPath ? "" : null))
    .attr("data-last", (d) => (d.last ? "" : null))
    .attr("data-step", (d) => d.step ?? null)
    .attr("data-selected", (d) => (d.selected ? "" : null))
    .attr("data-selected-inside", (d) => (d.inside > 0 ? d.inside : null))
    .style("--indent", (d) => `${d.indent}`)
    .on("click", (_event, d) => handlers.select(d))
    .order();
  items.selectAll("*").remove();

  items
    .append("span")
    .attr("class", "gutter")
    .text((d) => d.gutter);
  items
    .append("button")
    .attr("type", "button")
    .attr("class", "twisty")
    .attr("tabindex", -1)
    .attr("aria-hidden", "true")
    .property("hidden", (d) => d.fold === undefined)
    .property("disabled", (d) => !canToggle(d))
    .text((d) => (d.fold === "open" ? "▾" : "▸"))
    .on("click", (event: MouseEvent, d) => {
      event.stopPropagation();
      handlers.toggle(d);
    });
  const label = items.append("span").attr("class", "label");
  label
    .selectAll<HTMLSpanElement, LabelPart>("span")
    .data((d) => d.parts)
    .join("span")
    .attr("class", (d) => `part ${d.kind}`)
    .text((d) => d.text);
  label
    .selectAll<HTMLButtonElement, Chip>("button")
    .data((d) => d.chips)
    .join("button")
    .attr("type", "button")
    .attr("class", "chip")
    .attr("data-symbol", (d) => d.symbol)
    .attr("data-allowed", (d) => (d.solid ? "this-reading" : "other-readings"))
    .text((d) => d.symbol)
    .on("click", (event: MouseEvent, d) => {
      event.stopPropagation();
      handlers.choose(d.symbol);
    });
  label
    .selectAll<HTMLSpanElement, AttributeChip>("span.attribute")
    .data((d) => d.attributes)
    .join("span")
    .attr("class", "attribute")
    .attr("data-key", (d) => (d.kind === "more" ? null : d.key))
    .attr("data-reference", (d) => d.kind)
    .attr("title", (d) => d.title)
    .text((d) => d.text);
  items
    .append("span")
    .attr("class", "right")
    .text((d) => d.right);

  // One row takes the focus of the keyboard: the focused row, or the first.
  const all = tree.querySelectorAll<HTMLElement>("[role=treeitem]");
  const focused =
    [...all].find((item) => item.dataset.key === focusedKey) ??
    (all.length > 0 ? all[0] : undefined);
  if (focused !== undefined) {
    focused.tabIndex = 0;
    if (focusedKey !== undefined) {
      focused.focus({ preventScroll: true });
    }
  }
  if (follow) {
    tree.scrollTop = tree.scrollHeight;
  }
}

/** The keyboard of an ARIA tree: arrows move and fold, Enter and Space fold. */
function onKey(tree: HTMLElement, event: KeyboardEvent, handlers: OutputHandlers): void {
  const item = (event.target as HTMLElement | null)?.closest<HTMLElement>("[role=treeitem]");
  if (item === null || item === undefined || event.target !== item) {
    return;
  }
  const items = [...tree.querySelectorAll<HTMLElement>("[role=treeitem]")];
  const at = items.indexOf(item);
  const row = select<HTMLElement, ViewRow>(item).datum();
  // The selection follows the focus of the keyboard, as in a tree with one selection.
  const move = (target: HTMLElement | undefined): void => {
    if (target !== undefined) {
      item.tabIndex = -1;
      target.tabIndex = 0;
      target.focus();
      handlers.select(select<HTMLElement, ViewRow>(target).datum());
    }
  };
  switch (event.key) {
    case "ArrowDown":
      move(items[at + 1]);
      break;
    case "ArrowUp":
      move(items[at - 1]);
      break;
    case "Home":
      move(items[0]);
      break;
    case "End":
      move(items.at(-1));
      break;
    case "ArrowRight": {
      // A folded row opens, an open row moves to its first child, and a row
      // without children keeps the focus.
      const child = items[at + 1];
      if (row.fold === "folded") {
        handlers.toggle(row);
      } else if (row.fold === "open" && Number(child?.getAttribute("aria-level")) > row.level) {
        move(child);
      }
      break;
    }
    case "ArrowLeft":
      if (row.fold === "open" && canToggle(row)) {
        handlers.toggle(row);
      } else {
        move(items.slice(0, at).findLast((i) => Number(i.getAttribute("aria-level")) < row.level));
      }
      break;
    case "Enter":
    case " ":
      if (canToggle(row)) {
        handlers.toggle(row);
      }
      break;
    default:
      return;
  }
  event.preventDefault();
}
