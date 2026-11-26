// The details panel under the output view: the row in focus, with every
// attribute of its step, the doc string of its rule or the passes of its
// loop. The sections "Step attributes" and "File references" of
// docs/specs/output-view.md decide its content.

import type { Attribute, CheckedGrammar, SessionState, TreeNode } from "./core.ts";
import { boundWords, type GrammarFacts, quoted, spanKey } from "./outline.ts";
import type { ViewRow } from "./output-view.ts";
import { valueText } from "./output-view.ts";
import { type OpenedFolder, referenceOf, safeBlob } from "./references.ts";

export interface DetailsElements {
  /** The entries of the row in focus. */
  readonly body: HTMLElement;
  /** The folder picker and the name of the open folder. */
  readonly folder: HTMLElement;
  readonly folderStatus: HTMLElement;
}

/** What the details panel shows. */
export interface DetailsModel {
  /** The row in focus, or undefined when no row is in focus. */
  readonly row: ViewRow | undefined;
  readonly state: SessionState;
  readonly grammar: CheckedGrammar;
  readonly facts: GrammarFacts;
  readonly folder: OpenedFolder | undefined;
}

/**
 * The object URLs of the files of the open folder. An URL lasts until the
 * person opens another folder, so a link of the panel keeps working.
 */
export class FileLinks {
  readonly #urls = new Map<File, string>();

  url(file: File): string {
    let url = this.#urls.get(file);
    if (url === undefined) {
      url = URL.createObjectURL(safeBlob(file));
      this.#urls.set(file, url);
    }
    return url;
  }

  clear(): void {
    for (const url of this.#urls.values()) {
      URL.revokeObjectURL(url);
    }
    this.#urls.clear();
  }
}

const plural = (count: number, one: string, many: string): string =>
  `${count} ${count === 1 ? one : many}`;

function make<K extends keyof HTMLElementTagNameMap>(
  tag: K,
  className: string,
  text?: string,
): HTMLElementTagNameMap[K] {
  const element = document.createElement(tag);
  element.className = className;
  if (text !== undefined) {
    element.textContent = text;
  }
  return element;
}

/** Shows the row in focus in the details panel. */
export function renderDetails(
  elements: DetailsElements,
  model: DetailsModel,
  links: FileLinks,
): void {
  const entries = model.row === undefined ? [] : rowEntries(model.row, model, links);
  if (entries.length === 0) {
    elements.body.replaceChildren(
      make(
        "p",
        "details-empty",
        model.row === undefined ? "No step is read yet." : "This row holds no node of the tree.",
      ),
    );
  } else {
    elements.body.replaceChildren(...entries);
  }
  // The panel offers the folder picker when a relative file reference needs
  // it, and it names the open folder.
  const needsFolder = elements.body.querySelector('[data-reference="file"]') !== null;
  elements.folder.hidden = !needsFolder && model.folder === undefined;
  elements.folderStatus.textContent =
    model.folder === undefined
      ? "No folder is open. Open the folder that holds the trace file to open its files."
      : `Folder ${model.folder.name}: ${plural(model.folder.size, "file", "files")}.`;
}

/** The entries of a row: one for each node that it holds, or one for its step. */
function rowEntries(row: ViewRow, model: DetailsModel, links: FileLinks): HTMLElement[] {
  if (row.nodes.length === 0) {
    return row.step === undefined ? [] : [stepEntry(row.step, model, links)];
  }
  return row.nodes.map((node) => nodeEntry(node, row, model, links));
}

function nodeEntry(
  node: TreeNode,
  row: ViewRow,
  model: DetailsModel,
  links: FileLinks,
): HTMLElement {
  switch (node.kind.type) {
    case "step":
      return stepEntry(node.kind.number, model, links);
    case "rule":
      return ruleEntry(node.kind.name, model.grammar);
    case "pass":
      return entry("pass", `pass ${node.kind.number}`, [
        make("p", "detail-note", plural(countSteps(node), "step", "steps")),
      ]);
    case "loop":
      return loopEntry(node, row, model.facts);
  }
}

function entry(kind: string, title: string, content: readonly HTMLElement[]): HTMLElement {
  const article = make("article", "detail");
  article.dataset.kind = kind;
  article.append(make("h4", "detail-title", title), ...content);
  return article;
}

function countSteps(node: TreeNode): number {
  return node.kind.type === "step"
    ? 1
    : node.children.reduce((sum, child) => sum + countSteps(child), 0);
}

/** A rule node: the doc string of its rule. */
function ruleEntry(rule: string, grammar: CheckedGrammar): HTMLElement {
  const doc = grammar.rules.find((r) => r.name === rule)?.doc?.text;
  const article = entry("rule", rule, [
    doc === undefined
      ? make("p", "detail-note", "The rule has no doc string.")
      : make("p", "doc", doc),
  ]);
  article.dataset.rule = rule;
  return article;
}

/** A loop node: each repetition among its origins with its bound, and the count of passes. */
function loopEntry(loop: TreeNode, row: ViewRow, facts: GrammarFacts): HTMLElement {
  if (loop.kind.type !== "loop") {
    return entry("loop", "loop", []);
  }
  const passes = loop.children.length;
  const completed = row.folded.length;
  const count = make(
    "p",
    "detail-note",
    completed > 0 && completed < passes
      ? `${plural(completed, "completed pass", "completed passes")} of ${passes}`
      : plural(passes, "pass", "passes"),
  );
  if (loop.kind.kind !== "repetition") {
    const kind =
      loop.kind.kind === "tail-references" ? "loop of tail references" : "loop of head references";
    return entry("loop", `${loop.kind.passRule ?? ""} ↻`.trim(), [
      make("p", "detail-note", kind),
      count,
    ]);
  }
  const list = make("ul", "repetitions");
  for (const origin of loop.origins) {
    const repetition = facts.repetitions.get(spanKey(origin));
    const bound = repetition === undefined ? "" : boundWords(repetition);
    const item = make("li", "repetition");
    item.append(make("code", "", facts.written(origin)));
    if (bound !== "") {
      item.append(` · ${bound}`);
    }
    list.append(item);
  }
  return entry("loop", "repetition", [list, count]);
}

/** A step: every attribute, with the key, the type annotation and the whole value. */
function stepEntry(number: number, model: DetailsModel, links: FileLinks): HTMLElement {
  const step = model.state.steps[number - 1];
  const article = entry("step", `step ${number} · ${quoted(step?.symbol ?? "")}`, []);
  article.dataset.step = String(number);
  const attributes = step?.attributes ?? [];
  if (attributes.length === 0) {
    article.append(make("p", "detail-note", "The step has no attributes."));
    return article;
  }
  const list = make("dl", "attributes");
  for (const attribute of attributes) {
    list.append(attributeItem(attribute, model.folder, links));
  }
  article.append(list);
  return article;
}

function attributeItem(
  attribute: Attribute,
  folder: OpenedFolder | undefined,
  links: FileLinks,
): HTMLElement {
  const item = make("div", "attribute-item");
  item.dataset.key = attribute.key;
  if (attribute.annotation !== undefined) {
    item.dataset.annotation = attribute.annotation;
  }
  const value = make("dd", "");
  if (attribute.annotation !== undefined) {
    value.append(make("span", "annotation", `(${attribute.annotation})`), " ");
  }
  const text = valueText(attribute);
  const reference = referenceOf(attribute);
  item.dataset.reference = reference?.type ?? "attribute";
  if (reference?.type === "web") {
    // A web address opens in a new tab, with no access to the page that opened it.
    value.append(link(reference.href, text));
    item.dataset.state = "link";
  } else if (reference?.type === "file") {
    const resolution = folder?.resolve(reference.path);
    if (resolution === undefined) {
      value.append(
        make("span", "value", text),
        make("span", "status", "open its folder to open it"),
      );
      item.dataset.state = "no-folder";
    } else if (resolution.type === "found") {
      value.append(link(links.url(resolution.file), text));
      item.dataset.state = "found";
    } else {
      const status =
        resolution.type === "outside" ? "outside the folder" : "not found in the folder";
      value.append(make("span", "value", text), make("span", "status", status));
      item.dataset.state = resolution.type;
    }
  } else {
    const shown =
      attribute.value.type === "string" && reference === undefined ? quoted(text) : text;
    value.append(make("span", "value", shown));
    if (reference?.type === "text") {
      value.append(make("span", "status", "the page does not open it"));
      item.dataset.state = "text";
    }
  }
  item.append(make("dt", "", attribute.key), value);
  return item;
}

function link(href: string, text: string): HTMLAnchorElement {
  const anchor = make("a", "value reference-link", text);
  anchor.href = href;
  anchor.target = "_blank";
  anchor.rel = "noopener noreferrer";
  return anchor;
}
