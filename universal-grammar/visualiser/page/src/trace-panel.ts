// The result of a trace as text: acceptance, the failure point, the count of
// readings and the next options. The output view of `output-view-k11` draws
// the tree of a reading beside it.

import { select } from "d3";
import type { SessionState, TraceDiagnostic } from "./core.ts";
import { TextIndex } from "./offsets.ts";

type Step = SessionState["steps"][number];

export interface TracePanel {
  readonly acceptance: HTMLElement;
  readonly readings: HTMLElement;
  readonly nextOptions: HTMLElement;
  readonly steps: HTMLElement;
  readonly messages: HTMLElement;
}

/** A message about the trace, such as a diagnostic of a trace file. */
export interface TraceMessage {
  readonly severity: "error" | "warning";
  readonly code: string;
  readonly text: string;
}

const quoted = (symbol: string): string => JSON.stringify(symbol);

/** The sentence that states whether the tracer accepts the sequence. */
export function acceptanceText(state: SessionState): string {
  const total = state.steps.length;
  if (state.failurePoint !== undefined) {
    const rejected = state.steps[state.failurePoint - 1];
    const symbol = rejected === undefined ? "" : ` ${quoted(rejected.symbol)}`;
    return (
      `Rejected at step ${state.failurePoint}: no reading accepts${symbol}. ` +
      `The state on view is the state after step ${state.stepsRead}.`
    );
  }
  if (state.stepsRead < total) {
    return `The tracer read ${state.stepsRead} of ${total} steps.`;
  }
  if (total === 0) {
    return state.canEnd
      ? "The empty sequence is complete."
      : "The sequence is empty, and it cannot end yet.";
  }
  return state.canEnd
    ? `Accepted. The sequence can end after step ${total}.`
    : `Accepted so far. The sequence cannot end after step ${total}.`;
}

function readingsText(state: SessionState): string {
  const count = state.readings.length;
  const noun = count === 1 ? "reading" : "readings";
  const cap = state.capReached
    ? " The tracer reached the reading cap, so more readings exist."
    : "";
  return `${count} ${noun}.${cap}`;
}

function nextOptionsText(state: SessionState): string {
  if (state.nextOptions.length === 0) {
    return "No step can come next.";
  }
  return `Next options: ${state.nextOptions.map((o) => quoted(o.symbol)).join(", ")}.`;
}

function stepState(state: SessionState, number: number): "read" | "rejected" | "unread" {
  if (number <= state.stepsRead) {
    return "read";
  }
  return number === state.failurePoint ? "rejected" : "unread";
}

/** Shows the state on view of a session. */
export function renderState(
  panel: TracePanel,
  state: SessionState,
  messages: readonly TraceMessage[],
): void {
  panel.acceptance.textContent = acceptanceText(state);
  panel.readings.textContent = readingsText(state);
  panel.nextOptions.textContent = nextOptionsText(state);
  const numbered = state.steps.map((step, i) => ({ step, number: i + 1 }));
  select(panel.steps)
    .selectAll<HTMLLIElement, { step: Step; number: number }>("li")
    .data(numbered)
    .join("li")
    .attr("data-state", (d) => stepState(state, d.number))
    .text((d) => d.step.symbol);
  renderMessages(panel, messages);
}

/** Shows that no session is open, with the reason and its messages. */
export function renderNoSession(
  panel: TracePanel,
  reason: string,
  messages: readonly TraceMessage[],
): void {
  panel.acceptance.textContent = reason;
  panel.readings.textContent = "";
  panel.nextOptions.textContent = "";
  select(panel.steps).selectAll("li").remove();
  renderMessages(panel, messages);
}

function renderMessages(panel: TracePanel, messages: readonly TraceMessage[]): void {
  select(panel.messages)
    .selectAll<HTMLLIElement, TraceMessage>("li")
    .data(messages)
    .join("li")
    .attr("data-code", (d) => d.code)
    .attr("data-severity", (d) => d.severity)
    .text((d) => `${d.severity} ${d.code}: ${d.text}`);
}

/** The message that an edit dropped the attributes of steps of a trace file. */
export function attributesDroppedMessages(count: number): TraceMessage[] {
  if (count === 0) {
    return [];
  }
  const steps = count === 1 ? "1 step" : `${count} steps`;
  return [
    {
      severity: "warning",
      code: "attributes-dropped",
      text: `The edits removed the attributes of ${steps} of the trace file.`,
    },
  ];
}

/** The messages of the diagnostics of a trace file, placed by line and column. */
export function traceFileMessages(
  text: string,
  diagnostics: readonly TraceDiagnostic[],
): TraceMessage[] {
  const index = new TextIndex(text);
  return diagnostics.map((d) => {
    const { line, column } = index.lineAndColumn(d.span.start);
    return { severity: d.severity, code: d.code, text: `${line}:${column} ${d.message}` };
  });
}
