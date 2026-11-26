// The rows of the output view: the tree of one reading as an outline, with
// folds, as the section "The outline" of docs/specs/output-view.md decides.
//
// This module has no DOM. It turns a tree of the core, the checked grammar
// and the fold state into rows, so the rules of the spec stay in one place.

import type { CheckedGrammar, Expr, Span, TreeNode } from "./core.ts";
import { TextIndex } from "./offsets.ts";

/** The kind of a node, or of a row that holds no node. */
export type RowKind = "rule" | "loop" | "pass" | "step" | "fold";

/** One part of a row label, styled by its kind. */
export interface LabelPart {
  readonly kind: "rule" | "loop" | "pass" | "step" | "note";
  readonly text: string;
}

/** One row of the outline. */
export interface Row {
  /** A key that stays the same while the tree grows: the path of child indexes. */
  readonly key: string;
  /** The kinds of the nodes that the row holds, from the outer node. A fold adds "fold". */
  readonly kinds: readonly RowKind[];
  /** The level in the ARIA tree, from 1. */
  readonly level: number;
  readonly parts: readonly LabelPart[];
  /** The right side, such as "pass 2 · 1 to 3 passes". */
  readonly right: string;
  /** The gutter: the number of a step, or the range of the steps of a fold, such as "5–12". */
  readonly gutter: string;
  /** Whether the row holds rows, and whether they show. */
  readonly fold: "open" | "folded" | undefined;
  /** Whether the row holds a node on the path from the start rule to the last step. */
  readonly onPath: boolean;
  /** Whether the row holds the step that the tracer read last. */
  readonly last: boolean;
  /**
   * The key of the fold choice of the row. It names the path of the row, with
   * the kind and the name of each node on it. So a choice about one node never
   * applies to another node at the same place, such as in another reading.
   */
  readonly toggleKey: string;
  /**
   * The nodes that the row holds, from the outer node. The fold row of the
   * completed passes of a loop holds the loop node, because it stands for
   * that loop.
   */
  readonly nodes: readonly TreeNode[];
  /** The passes that a fold row stands for. Other rows have none. */
  readonly folded: readonly TreeNode[];
  /** The ancestors of the first node of the row, from the root. */
  readonly ancestors: readonly TreeNode[];
  /** Whether the row holds a node that matches the selected grammar element. */
  readonly matched: boolean;
  /** For a folded row, the number of matching rows that it hides. */
  readonly inside: number;
}

/** The facts about a grammar that the rows need. They depend only on the grammar. */
export interface GrammarFacts {
  /** The rules for one step: each node of such a rule holds exactly one step and nothing else. */
  readonly oneStepRules: ReadonlySet<string>;
  /** Each repetition by the key of its span. */
  readonly repetitions: ReadonlyMap<string, Repetition>;
  /** The repetitions whose passes share a row with their node, by the key of their span. */
  readonly sharingRepetitions: ReadonlySet<string>;
  /** The text of a span of the grammar text, such as `show_example{1,3}`. */
  readonly written: (span: Span) => string;
}

export type Repetition = Extract<Expr, { type: "repetition" }>;

/** A set of toggle keys, as far as the rows read it. */
export interface KeySet {
  readonly has: (key: string) => boolean;
}

/** The rows that a person opened or closed, by the toggle key of each row. */
export interface FoldState {
  /** Fold rows that the person opened. Fold rows start folded. */
  readonly opened: KeySet;
  /** Other rows that the person folded. They start open. */
  readonly closed: KeySet;
}

/** The fold state that opens every row, so that each node has a row. */
export const allOpen: FoldState = { opened: { has: () => true }, closed: { has: () => false } };

export const spanKey = (span: Span): string => `${span.start}:${span.end}`;

/**
 * The bound of a repetition in words, such as "1 to 3 passes". It is empty
 * for `*` and `+`, which name no count.
 */
export function boundWords(r: Repetition): string {
  if (r.max === undefined) {
    return r.min > 1 ? `${r.min} or more passes` : "";
  }
  return r.min === r.max ? plural(r.max, "pass", "passes") : `${r.min} to ${r.max} passes`;
}

/** The toggle key of the fold of the completed passes of a loop. */
const foldKey = (loopKey: string): string => `${loopKey}/completed`;

/** Collects the facts about a checked grammar without errors and its text. */
export function grammarFacts(grammar: CheckedGrammar, text: string): GrammarFacts {
  const repetitions = new Map<string, Repetition>();
  const sharingRepetitions = new Set<string>();
  const oneStepRules = new Set<string>();
  const empty = emptyRules(grammar);
  for (const rule of grammar.rules) {
    if (rule.body === undefined) {
      continue;
    }
    const inRule = repetitionsIn(rule.body);
    for (const r of inRule) {
      repetitions.set(spanKey(r.span), r);
    }
    // Two repetitions share one loop node only when equal trees join them,
    // and then both are in the body of one rule. A mixed repetition can make
    // one node in one pass and more nodes in another pass. Such a repetition
    // can join a repetition whose passes share, and a later step can remove
    // it again. So no repetition of its rule shares, and a row never merges
    // or splits when an origin goes.
    const mixed = inRule.some((r) => !makesOneNode(r.item) && canMakeOneNode(r.item, empty));
    if (!mixed) {
      for (const r of inRule.filter((r) => makesOneNode(r.item))) {
        sharingRepetitions.add(spanKey(r.span));
      }
    }
    if (holdsOneStep(rule.body)) {
      oneStepRules.add(rule.name);
    }
  }
  const index = new TextIndex(text);
  const written = (span: Span): string =>
    text.slice(index.utf16(span.start), index.utf16(span.end));
  return { oneStepRules, repetitions, sharingRepetitions, written };
}

/** The repetitions of a rule body, in the order of the text. */
function repetitionsIn(expr: Expr): Repetition[] {
  switch (expr.type) {
    case "sequence":
      return expr.items.flatMap(repetitionsIn);
    case "choice":
      return expr.alternatives.flatMap(repetitionsIn);
    case "optional":
      return repetitionsIn(expr.item);
    case "repetition":
      return [expr, ...repetitionsIn(expr.item)];
    case "symbol":
    case "reference":
      return [];
  }
}

/** The rules whose body can match the empty sequence. */
function emptyRules(grammar: CheckedGrammar): Set<string> {
  const found = new Set<string>();
  for (let grew = true; grew; ) {
    grew = false;
    for (const rule of grammar.rules) {
      if (rule.body !== undefined && !found.has(rule.name) && matchesEmpty(rule.body, found)) {
        found.add(rule.name);
        grew = true;
      }
    }
  }
  return found;
}

/** Whether an expression can match the empty sequence, and so make no node. */
function matchesEmpty(expr: Expr, emptyRules: ReadonlySet<string>): boolean {
  switch (expr.type) {
    case "symbol":
      return false;
    case "reference":
      return emptyRules.has(expr.name);
    case "optional":
      return true;
    case "repetition":
      return expr.min === 0 || matchesEmpty(expr.item, emptyRules);
    case "choice":
      return expr.alternatives.some((a) => matchesEmpty(a, emptyRules));
    case "sequence":
      return expr.items.every((item) => matchesEmpty(item, emptyRules));
  }
}

/**
 * Whether some match of an expression makes exactly one node. A reference
 * counts as one node, even to a rule that can match the empty sequence. So
 * the answer can be yes where no such match exists, and never the reverse.
 */
function canMakeOneNode(expr: Expr, emptyRules: ReadonlySet<string>): boolean {
  switch (expr.type) {
    case "symbol":
    case "reference":
    case "repetition":
      return true;
    case "optional":
      return canMakeOneNode(expr.item, emptyRules);
    case "choice":
      return expr.alternatives.some((a) => canMakeOneNode(a, emptyRules));
    case "sequence":
      return expr.items.some(
        (item, i) =>
          canMakeOneNode(item, emptyRules) &&
          expr.items.every((other, j) => j === i || matchesEmpty(other, emptyRules)),
      );
  }
}

/**
 * Whether each match of a rule body is exactly one step and nothing else. A
 * reference makes a node of its own, and an empty match makes no node, so an
 * optional symbol still holds one step in each node that it makes.
 */
function holdsOneStep(expr: Expr): boolean {
  switch (expr.type) {
    case "symbol":
      return true;
    case "choice":
      return expr.alternatives.every(holdsOneStep);
    case "optional":
      return holdsOneStep(expr.item);
    case "sequence":
    case "reference":
    case "repetition":
      return false;
  }
}

/**
 * Whether each match of a repeated item makes exactly one node: a step, a
 * rule node or a loop node. The rule checks reject a repetition whose item can
 * match the empty sequence, so an optional item never repeats.
 */
function makesOneNode(expr: Expr): boolean {
  switch (expr.type) {
    case "symbol":
    case "reference":
    case "repetition":
      return true;
    case "choice":
      return expr.alternatives.every(makesOneNode);
    case "optional":
    case "sequence":
      return false;
  }
}

/** The first and the last step of a subtree, or undefined for a node without steps. */
function stepRange(node: TreeNode): readonly [number, number] | undefined {
  if (node.kind.type === "step") {
    return [node.kind.number, node.kind.number];
  }
  let range: [number, number] | undefined;
  for (const child of node.children) {
    const r = stepRange(child);
    if (r !== undefined) {
      range =
        range === undefined ? [r[0], r[1]] : [Math.min(range[0], r[0]), Math.max(range[1], r[1])];
    }
  }
  return range;
}

/** The numbers of the steps of a subtree, in order. */
export function stepNumbers(node: TreeNode): number[] {
  return node.kind.type === "step" ? [node.kind.number] : node.children.flatMap(stepNumbers);
}

function countSteps(node: TreeNode): number {
  return node.kind.type === "step"
    ? 1
    : node.children.reduce((sum, child) => sum + countSteps(child), 0);
}

const plural = (count: number, one: string, many: string): string =>
  `${count} ${count === 1 ? one : many}`;

const rangeText = (range: readonly [number, number] | undefined): string =>
  range === undefined ? "" : range[0] === range[1] ? `${range[0]}` : `${range[0]}–${range[1]}`;

/**
 * The labels of a node that never change while its tree grows: its kind, and
 * the rule, the kind of loop, the pass or the step. Origins can change, so
 * they are not among them.
 */
function nodeName(node: TreeNode): string {
  switch (node.kind.type) {
    case "rule":
      return `rule ${node.kind.name}`;
    case "loop":
      return `loop ${node.kind.kind} ${node.kind.passRule ?? ""}`;
    case "pass":
      return `pass ${node.kind.number}`;
    case "step":
      return `step ${node.kind.number}`;
  }
}

/** The toggle key of child i of the node with a toggle key. */
const childToggleKey = (parentKey: string, i: number, child: TreeNode): string =>
  `${parentKey}/${i}=${JSON.stringify(nodeName(child))}`;

/** The text of a symbol in a row: the symbol in quotes. */
export const quoted = (symbol: string): string => JSON.stringify(symbol);

/**
 * The rows of the tree of one reading. `lastStep` is the number of the last
 * step read, or 0 before the first step. `match` tells whether a node matches
 * the selected grammar element, when the selection is a grammar element.
 */
export function outlineRows(
  tree: TreeNode,
  lastStep: number,
  facts: GrammarFacts,
  folds: FoldState,
  match?: (node: TreeNode) => boolean,
): Row[] {
  const rows: Row[] = [];

  /**
   * The number of matching rows that a fold hides: the matching rows that
   * `emitInto` gives with every row open.
   */
  const hiddenMatches = (emitInto: (out: Row[]) => void): number => {
    if (match === undefined) {
      return 0;
    }
    const scratch: Row[] = [];
    emitInto(scratch);
    return scratch.filter((r) => r.matched).length;
  };

  const isOnPath = (node: TreeNode, isRoot: boolean): boolean => {
    if (isRoot) {
      return true;
    }
    const range = stepRange(node);
    return range !== undefined && range[0] <= lastStep && lastStep <= range[1];
  };

  /** The repetitions among the origins of a loop node, in the order of the text. */
  const repetitionsOf = (loop: TreeNode): Repetition[] =>
    loop.origins.flatMap((origin) => {
      const expr = facts.repetitions.get(spanKey(origin));
      return expr === undefined ? [] : [expr];
    });

  /** Whether each pass of a loop node holds exactly one node: section "Rows". */
  const passShares = (loop: TreeNode): boolean => {
    if (loop.kind.type !== "loop") {
      return false;
    }
    if (loop.kind.kind !== "repetition") {
      return true;
    }
    const found = repetitionsOf(loop);
    return found.length > 0 && found.every((r) => facts.sharingRepetitions.has(spanKey(r.span)));
  };

  /** Whether a node shares its row with its only child. */
  const shares = (node: TreeNode, parent: TreeNode | undefined): boolean => {
    const only = node.children.length === 1 ? node.children[0] : undefined;
    if (only === undefined) {
      return false;
    }
    switch (node.kind.type) {
      case "rule":
        return facts.oneStepRules.has(node.kind.name) && only.kind.type === "step";
      case "pass":
        return parent !== undefined && passShares(parent);
      case "loop":
      case "step":
        return false;
    }
  };

  const loopLabel = (loop: TreeNode): LabelPart[] => {
    if (loop.kind.type !== "loop") {
      return [];
    }
    if (loop.kind.kind === "repetition") {
      const forms = [...new Set(loop.origins.map(facts.written))];
      return [{ kind: "loop", text: forms.join(" | ") }];
    }
    const kindText =
      loop.kind.kind === "tail-references" ? "loop of tail references" : "loop of head references";
    return [
      { kind: "loop", text: `${loop.kind.passRule ?? ""} ↻`.trim() },
      { kind: "note", text: kindText },
    ];
  };

  /** The bound of a loop of repetitions, when every repetition has the same one. */
  const boundText = (loop: TreeNode): string => {
    const found = repetitionsOf(loop);
    const texts = new Set(found.map(boundWords));
    const [only] = texts;
    return found.length > 0 && texts.size === 1 && only !== undefined ? only : "";
  };

  const nodeLabel = (node: TreeNode): LabelPart[] => {
    switch (node.kind.type) {
      case "rule":
        return [{ kind: "rule", text: node.kind.name }];
      case "pass":
        return [{ kind: "pass", text: `pass ${node.kind.number}` }];
      case "step":
        return [{ kind: "step", text: quoted(node.kind.symbol) }];
      case "loop":
        return loopLabel(node);
    }
  };

  const countsText = (passes: readonly TreeNode[]): string =>
    `${plural(passes.length, "pass", "passes")} · ${plural(
      passes.reduce((sum, p) => sum + countSteps(p), 0),
      "step",
      "steps",
    )}`;

  const rangeOf = (nodes: readonly TreeNode[]): string => {
    const ranges = nodes.map(stepRange).filter((r) => r !== undefined);
    const first = ranges[0];
    const final = ranges.at(-1);
    return first === undefined || final === undefined ? "" : rangeText([first[0], final[1]]);
  };

  const emit = (
    node: TreeNode,
    parent: TreeNode | undefined,
    key: string,
    toggleKey: string,
    level: number,
    isRoot: boolean,
    ancestors: readonly TreeNode[],
    out: Row[],
    state: FoldState,
  ): void => {
    // The nodes of one row: a node and each only child that shares with it.
    const chain: {
      node: TreeNode;
      parent: TreeNode | undefined;
      key: string;
      toggleKey: string;
    }[] = [{ node, parent, key, toggleKey }];
    for (;;) {
      const tail = chain.at(-1);
      if (tail === undefined || !shares(tail.node, tail.parent)) {
        break;
      }
      const child = tail.node.children[0];
      if (child === undefined) {
        break;
      }
      chain.push({
        node: child,
        parent: tail.node,
        key: `${tail.key}.0`,
        toggleKey: childToggleKey(tail.toggleKey, 0, child),
      });
    }
    const tail = chain.at(-1) ?? chain[0];
    if (tail === undefined) {
      return;
    }
    const onPath = isOnPath(node, isRoot);
    const kinds = chain.map((c) => c.node.kind.type);
    const parts = chain.flatMap((c, i) => {
      const label = nodeLabel(c.node);
      return i === 0 ? label : [{ kind: "note" as const, text: "·" }, ...label];
    });
    const last = tail.node.kind.type === "step" && tail.node.kind.number === lastStep;
    const target = tail.node;
    const nodes = chain.map((c) => c.node);
    const matched = match !== undefined && nodes.some(match);
    // The ancestors of the rows below the row.
    const below = [...ancestors, ...nodes];
    const emitPasses = (
      passes: readonly TreeNode[],
      at: number,
      into: Row[],
      s: FoldState,
    ): void => {
      passes.forEach((pass, i) => {
        emit(
          pass,
          target,
          `${tail.key}.${i}`,
          childToggleKey(tail.toggleKey, i, pass),
          at,
          false,
          below,
          into,
          s,
        );
      });
    };

    if (target.kind.type === "loop") {
      const passes = target.children;
      if (!onPath) {
        // A finished loop: its loop row and its fold are one row.
        const toggleKey = foldKey(tail.toggleKey);
        const open = state.opened.has(toggleKey);
        out.push({
          key: tail.key,
          kinds: [...kinds, "fold"],
          level,
          parts: [...parts, { kind: "note", text: `· ${countsText(passes)}` }],
          right: "",
          gutter: rangeOf(passes),
          fold: open ? "open" : "folded",
          onPath,
          last,
          toggleKey,
          nodes,
          folded: passes,
          ancestors,
          matched,
          inside: open ? 0 : hiddenMatches((o) => emitPasses(passes, level + 1, o, allOpen)),
        });
        if (open) {
          emitPasses(passes, level + 1, out, state);
        }
        return;
      }
      const current = passes.at(-1);
      const passNumber = current?.kind.type === "pass" ? current.kind.number : passes.length;
      const bound = boundText(target);
      out.push({
        key: tail.key,
        kinds,
        level,
        parts,
        right: bound === "" ? `pass ${passNumber}` : `pass ${passNumber} · ${bound}`,
        gutter: "",
        fold: "open",
        onPath,
        last,
        toggleKey: tail.toggleKey,
        nodes,
        folded: [],
        ancestors,
        matched,
        inside: 0,
      });
      const completed = passes.slice(0, -1);
      if (completed.length > 0) {
        const toggleKey = foldKey(tail.toggleKey);
        const open = state.opened.has(toggleKey);
        // The fold row of the completed passes stands for the loop in the
        // details panel. It holds passes, not the loop, so it matches no
        // element of its own: the loop row above it matches.
        out.push({
          key: foldKey(tail.key),
          kinds: ["fold"],
          level: level + 1,
          parts: [
            { kind: "pass", text: "completed" },
            { kind: "note", text: `· ${countsText(completed)}` },
          ],
          right: "",
          gutter: rangeOf(completed),
          fold: open ? "open" : "folded",
          onPath: false,
          last: false,
          toggleKey,
          nodes: [target],
          folded: completed,
          ancestors: [...ancestors, ...nodes.slice(0, -1)],
          matched: false,
          inside: open ? 0 : hiddenMatches((o) => emitPasses(completed, level + 2, o, allOpen)),
        });
        if (open) {
          emitPasses(completed, level + 2, out, state);
        }
      }
      if (current !== undefined) {
        const i = passes.length - 1;
        emit(
          current,
          target,
          `${tail.key}.${i}`,
          childToggleKey(tail.toggleKey, i, current),
          level + 1,
          false,
          below,
          out,
          state,
        );
      }
      return;
    }

    const holdsRows = target.children.length > 0;
    // A person can fold a row that holds rows, but not a row on the path to
    // the last step: the spec keeps that path open.
    const open = !holdsRows || onPath || !state.closed.has(tail.toggleKey);
    const emitChildren = (into: Row[], s: FoldState): void => {
      target.children.forEach((child, i) => {
        emit(
          child,
          target,
          `${tail.key}.${i}`,
          childToggleKey(tail.toggleKey, i, child),
          level + 1,
          false,
          below,
          into,
          s,
        );
      });
    };
    out.push({
      key: tail.key,
      kinds,
      level,
      parts,
      right: "",
      gutter: target.kind.type === "step" ? `${target.kind.number}` : "",
      fold: holdsRows ? (open ? "open" : "folded") : undefined,
      onPath,
      last,
      toggleKey: tail.toggleKey,
      nodes,
      folded: [],
      ancestors,
      matched,
      inside: open ? 0 : hiddenMatches((o) => emitChildren(o, allOpen)),
    });
    if (holdsRows && open) {
      emitChildren(out, state);
    }
  };

  emit(tree, undefined, "0", childToggleKey("", 0, tree), 1, true, [], rows, folds);
  return rows;
}
