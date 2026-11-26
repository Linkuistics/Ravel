// The core in WebAssembly, with typed results for the page.
//
// The page calls the core through typed objects, and it never parses KDL:
// docs/adr/typed-objects-at-the-webassembly-boundary.md. The core reports a
// domain failure as a tagged value. An exception from the core is a defect.

import init, {
  type CheckedGrammar,
  checkGrammar,
  type DiagramsOutcome,
  diagrams,
  type OpenOutcome,
  type OpenReport,
  openEditedTraceFile,
  openSequence,
  openSession,
  type TraceDiagnostic,
  type TraceSession,
} from "../generated/core/workbench_wasm.js";
// Vite documents the `?url` import for a WebAssembly file
// (https://vite.dev/guide/features#webassembly). The glue code of
// `wasm-bindgen` otherwise finds the module through `new URL(…, import.meta.url)`.
import wasmUrl from "../generated/core/workbench_wasm_bg.wasm?url";

export type {
  Attribute,
  CheckedGrammar,
  Diagnostic,
  Diagram,
  Diagrams,
  DiagramsOutcome,
  // The core names a marked element `Element`, which would hide the DOM type.
  Element as MarkElement,
  Expr,
  Flow,
  Marks,
  MarksOutcome,
  ReadingSummary,
  SessionState,
  Shape,
  Span,
  TraceDiagnostic,
  TraceSession,
  TreeNode,
} from "../generated/core/workbench_wasm.js";

/** Loads the WebAssembly module. Call it once, before any other function. */
export async function loadCore(): Promise<void> {
  await init({ module_or_path: wasmUrl });
}

/** Parses a grammar text and runs the rule checks. */
export function check(grammar: string): CheckedGrammar {
  return checkGrammar(grammar);
}

/**
 * Lays out the diagram of each rule: the operation Diagrams of
 * docs/specs/rule-diagrams.md. A grammar with errors has no diagrams.
 */
export function layOut(grammar: string): DiagramsOutcome {
  return diagrams(grammar);
}

/** A session that opened, or the report of the failure. */
export type Opened =
  | {
      readonly ok: true;
      readonly session: TraceSession;
      readonly warnings: readonly TraceDiagnostic[];
      /** The number of steps of a trace file whose attributes an edit dropped. */
      readonly attributesDropped: number;
    }
  | { readonly ok: false; readonly report: Exclude<OpenReport, { type: "opened" }> };

/** Opens a session on a grammar text and a sequence of symbols. */
export function openFromSymbols(grammar: string, symbols: readonly string[]): Opened {
  return settle(openSequence(grammar, [...symbols]));
}

/** Opens a session on a grammar text and the text of a trace file. */
export function openFromTraceFile(grammar: string, traceFile: string): Opened {
  return settle(openSession(grammar, traceFile));
}

/**
 * Opens a session on a grammar text, the text of a trace file and the symbols
 * of an edit of its sequence. The steps that the edit leaves unchanged at the
 * start and at the end keep their attributes.
 */
export function openFromEditedTraceFile(
  grammar: string,
  traceFile: string,
  symbols: readonly string[],
): Opened {
  return settle(openEditedTraceFile(grammar, traceFile, [...symbols]));
}

/**
 * The trace file of a session, as KDL text. A trace file holds no step on
 * view, and the core reads as many steps of a file as it can: the section
 * "Out of scope" of docs/specs/tracer.md. So the page saves the trace of the
 * whole sequence, read as far as the core can, also after a step back. The
 * core then gives the same text when it reads the file again.
 */
export function traceFileText(grammar: string, session: TraceSession): string {
  const text = session.save();
  const state = session.state();
  if (state.stepsRead === state.steps.length || state.failurePoint !== undefined) {
    return text;
  }
  const whole = openFromTraceFile(grammar, text);
  if (!whole.ok) {
    throw new CoreFault("The core could not read a trace file that it saved.");
  }
  const saved = whole.session.save();
  whole.session.free();
  return saved;
}

/** A result of the core that breaks its own contract. */
export class CoreFault extends Error {
  override name = "CoreFault";
}

function settle(outcome: OpenOutcome): Opened {
  const report = outcome.report();
  const session = outcome.takeSession();
  outcome.free();
  if (report.type === "opened") {
    if (session === undefined) {
      throw new CoreFault("The core opened a session but returned none.");
    }
    return {
      ok: true,
      session,
      warnings: report.warnings,
      attributesDropped: report.attributesDropped,
    };
  }
  session?.free();
  return { ok: false, report };
}
