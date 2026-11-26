// The list of diagnostics of the grammar text. The rule index of the grammar
// view lists the rules.

import { select } from "d3";
import type { Diagnostic } from "./core.ts";

/** Lists the diagnostics by line and column. A click selects the span. */
export function renderDiagnostics(
  list: HTMLElement,
  diagnostics: readonly Diagnostic[],
  onSelect: (diagnostic: Diagnostic) => void,
): void {
  const items = select(list)
    .selectAll<HTMLLIElement, Diagnostic>("li")
    .data(diagnostics)
    .join("li")
    .attr("data-code", (d) => d.code)
    .attr("data-severity", (d) => d.severity);
  items.selectAll("*").remove();
  const button = items
    .append("button")
    .attr("type", "button")
    .on("click", (_event, d) => onSelect(d));
  button
    .append("span")
    .attr("class", "place")
    .text((d) => `${d.line}:${d.column}`);
  button
    .append("span")
    .attr("class", "severity")
    .text((d) => d.severity);
  button
    .append("span")
    .attr("class", "code")
    .text((d) => d.code);
  button
    .append("span")
    .attr("class", "message")
    .text((d) => d.message);
}
