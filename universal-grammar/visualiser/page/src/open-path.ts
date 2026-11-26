// The open path of the grammar view: the chain of rules to the open rule
// diagram. The section "The grammar view" of docs/specs/rule-diagrams.md
// states its rules.

import type { CheckedGrammar, Expr } from "./core.ts";

/** The rules of a grammar and the references between them, in the order of the text. */
export interface RuleGraph {
  readonly start: string | undefined;
  /** The rule names, in the order of the text. */
  readonly rules: readonly string[];
  /** For each rule, the rules that it refers to, in the order of their first reference. */
  readonly references: ReadonlyMap<string, readonly string[]>;
  /** The rules that the start rule reaches, the start rule included. */
  readonly reachable: ReadonlySet<string>;
}

function referencesIn(expr: Expr | undefined, into: string[]): void {
  if (expr === undefined) {
    return;
  }
  switch (expr.type) {
    case "symbol":
      return;
    case "reference":
      if (!into.includes(expr.name)) {
        into.push(expr.name);
      }
      return;
    case "sequence":
      for (const item of expr.items) {
        referencesIn(item, into);
      }
      return;
    case "choice":
      for (const alternative of expr.alternatives) {
        referencesIn(alternative, into);
      }
      return;
    case "optional":
    case "repetition":
      referencesIn(expr.item, into);
      return;
  }
}

/**
 * The parents of a breadth-first walk from the start rule. Each rule keeps the
 * first parent that reaches it, and the walk takes the references of each rule
 * in the order of the text. So the chain of parents of a rule is the first
 * shortest chain of references from the start rule.
 */
function walk(
  start: string | undefined,
  references: ReadonlyMap<string, readonly string[]>,
): Map<string, string | undefined> {
  const parents = new Map<string, string | undefined>();
  if (start === undefined || !references.has(start)) {
    return parents;
  }
  parents.set(start, undefined);
  const queue = [start];
  for (let i = 0; i < queue.length; i++) {
    const rule = queue[i] as string;
    for (const target of references.get(rule) ?? []) {
      if (!parents.has(target) && references.has(target)) {
        parents.set(target, rule);
        queue.push(target);
      }
    }
  }
  return parents;
}

export function ruleGraph(grammar: CheckedGrammar): RuleGraph {
  const references = new Map<string, string[]>();
  for (const rule of grammar.rules) {
    const targets = references.get(rule.name) ?? [];
    referencesIn(rule.body, targets);
    references.set(rule.name, targets);
  }
  const start = grammar.startRules[0]?.name;
  return {
    start,
    rules: [...references.keys()],
    references,
    reachable: new Set(walk(start, references).keys()),
  };
}

/**
 * The open path of a rule that the rule index opens: the first shortest chain
 * of references from the start rule. A rule that the start rule does not reach
 * opens alone.
 */
export function indexPath(graph: RuleGraph, rule: string): string[] {
  const parents = walk(graph.start, graph.references);
  if (!parents.has(rule)) {
    return [rule];
  }
  const path: string[] = [];
  for (let at: string | undefined = rule; at !== undefined; at = parents.get(at)) {
    path.unshift(at);
  }
  return path;
}

/**
 * Whether a path is an open path: a chain of references that starts at the
 * start rule, or at a rule that the start rule does not reach.
 */
export function isOpenPath(graph: RuleGraph, path: readonly string[]): boolean {
  const first = path[0];
  if (first === undefined || !graph.references.has(first)) {
    return false;
  }
  if (first !== graph.start && graph.reachable.has(first)) {
    return false;
  }
  return path.every((rule, i) => {
    const next = path[i + 1];
    return next === undefined || (graph.references.get(rule)?.includes(next) ?? false);
  });
}

/**
 * A path that extends an open path to a rule by the first shortest chain of
 * references from its last rule. An empty path extends as from the rule index.
 * The result is undefined when the last rule does not reach the rule.
 */
export function extendPath(
  graph: RuleGraph,
  path: readonly string[],
  rule: string,
): string[] | undefined {
  const from = path.at(-1);
  if (from === undefined) {
    return indexPath(graph, rule);
  }
  const parents = walk(from, graph.references);
  if (!parents.has(rule)) {
    return undefined;
  }
  const chain: string[] = [];
  for (let at: string | undefined = rule; at !== undefined && at !== from; at = parents.get(at)) {
    chain.unshift(at);
  }
  return [...path, ...chain];
}
