// The selection across the views: the section "Selection across the views"
// of docs/specs/output-view.md. A selection is one row of the output view or
// one element of the grammar view.
//
// This module has no DOM. It maps a row to the grammar elements that the
// table of the spec gives, and to the open path of the diagram that holds
// them. It also tells whether a node matches a selected grammar element.
//
// A grammar element has the identity that the checked grammar gives it: the
// span of its construct. A task box, a reference box, a repetition and a back
// edge each name that span in the diagrams of the core. The origins of a node
// name the same spans. So one span links a row to its elements.

import type { CheckedGrammar, Span, TreeNode } from "./core.ts";
import { extendPath, indexPath, isOpenPath, type RuleGraph } from "./open-path.ts";
import { spanKey } from "./outline.ts";

/** The selection of the page. */
export type Selection =
  | {
      readonly type: "row";
      /** The toggle key of the row, as in `Row`. */
      readonly toggleKey: string;
      /** The number of the step that the row shows. A step keeps its number in every reading. */
      readonly step: number | undefined;
    }
  | {
      readonly type: "element";
      /** The span of the grammar element. */
      readonly span: Span;
    };

/** The part of a row that the selection reads. */
export interface SelectableRow {
  readonly nodes: readonly TreeNode[];
  readonly ancestors: readonly TreeNode[];
}

/** The grammar elements of a row, and the open path of the diagram that holds them. */
export interface RowElements {
  /** The span keys of the elements. */
  readonly spans: ReadonlySet<string>;
  readonly openPath: readonly string[];
}

/**
 * The node that a row stands for, with its ancestors from the root. A shared
 * row stands for its rule node, or else for its innermost node.
 */
export function standsFor(
  row: SelectableRow,
): { readonly node: TreeNode; readonly ancestors: readonly TreeNode[] } | undefined {
  const rule = row.nodes.findIndex((n) => n.kind.type === "rule");
  const at = rule >= 0 ? rule : row.nodes.length - 1;
  const node = row.nodes[at];
  return node === undefined
    ? undefined
    : { node, ancestors: [...row.ancestors, ...row.nodes.slice(0, at)] };
}

/**
 * The spans of the grammar elements of a node, by the table of the spec. The
 * first pass of a loop through rules and each pass of a repetition have no
 * origin, so they select the elements of their loop. The start rule node and
 * a loop entered at the start rule select the diagram of the start rule, so
 * they have no span.
 */
function nodeSpans(node: TreeNode, parent: TreeNode | undefined): readonly Span[] {
  if (node.origins.length > 0) {
    return node.origins;
  }
  if (node.kind.type === "pass" && parent !== undefined) {
    return parent.origins;
  }
  return [];
}

/** The rule whose body holds a span, if any. */
function ruleOfSpan(grammar: CheckedGrammar, span: Span): string | undefined {
  return grammar.rules.find(
    (r) => r.body !== undefined && r.body.span.start <= span.start && span.end <= r.body.span.end,
  )?.name;
}

/**
 * The grammar elements of a row and the open path of their diagram. The open
 * path follows the ancestors of the node in the tree: the rules of the rule
 * nodes above it, from the root. When the elements lie in another rule, the
 * path ends at the last occurrence of that rule, or else extends to it by the
 * first shortest chain of references. A row without a node has no elements.
 */
export function rowElements(
  row: SelectableRow,
  grammar: CheckedGrammar,
  graph: RuleGraph,
): RowElements | undefined {
  const stands = standsFor(row);
  if (stands === undefined) {
    return undefined;
  }
  const { node, ancestors } = stands;
  const spans = nodeSpans(node, ancestors.at(-1));
  const above = ancestors.flatMap((n) => (n.kind.type === "rule" ? [n.kind.name] : []));
  const start = graph.start === undefined ? [] : [graph.start];
  const keys = new Set(spans.map(spanKey));
  if (spans.length === 0) {
    return { spans: keys, openPath: start };
  }
  // The diagram of the nearest rule above the node, when it holds an
  // element. Otherwise the diagram of the first element.
  const rules = spans.flatMap((s) => {
    const rule = ruleOfSpan(grammar, s);
    return rule === undefined ? [] : [rule];
  });
  const nearest = above.at(-1);
  const target = nearest !== undefined && rules.includes(nearest) ? nearest : rules[0];
  if (target === undefined) {
    return { spans: keys, openPath: above.length > 0 ? above : start };
  }
  const last = above.lastIndexOf(target);
  const candidate = last >= 0 ? above.slice(0, last + 1) : (extendPath(graph, above, target) ?? []);
  return {
    spans: keys,
    openPath: isOpenPath(graph, candidate) ? candidate : indexPath(graph, target),
  };
}

/** Whether a node holds a grammar element among its origins. */
export function matchesElement(node: TreeNode, span: Span): boolean {
  return node.origins.some((o) => o.start === span.start && o.end === span.end);
}
