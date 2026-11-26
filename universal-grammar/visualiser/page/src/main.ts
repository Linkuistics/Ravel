// The workbench page: a grammar editor, its diagnostics and rules, and a
// sequence that the core traces through the grammar.

import "./style.css";
import {
  type CheckedGrammar,
  check,
  layOut,
  loadCore,
  type Opened,
  openFromEditedTraceFile,
  openFromSymbols,
  openFromTraceFile,
  type SessionState,
  type Span,
  type TraceSession,
  traceFileText,
} from "./core.ts";
import { FileLinks, renderDetails } from "./details-panel.ts";
import { createEditor } from "./editor.ts";
import { examples } from "./examples.ts";
import { diagramFontFamily, loadDiagramFont } from "./font.ts";
import { renderDiagnostics } from "./grammar-panel.ts";
import { createGrammarView, type OpenChange, type TraceMarks } from "./grammar-view.ts";
import { type RuleGraph, ruleGraph } from "./open-path.ts";
import { allOpen, type GrammarFacts, grammarFacts, spanKey } from "./outline.ts";
import {
  chooseReading,
  type OutputModel,
  rejectionReason,
  renderNoOutput,
  renderOutputView,
  revealRow,
  rowInFocus,
  type Viewed,
  type ViewRow,
  viewRows,
} from "./output-view.ts";
import { OpenedFolder } from "./references.ts";
import { rowElements, type Selection } from "./selection.ts";
import {
  attributesDroppedMessages,
  renderNoSession,
  renderState,
  type TraceMessage,
  traceFileMessages,
} from "./trace-panel.ts";

/**
 * Where the sequence comes from: the sequence input, or a loaded trace file.
 * Once a trace file opens, the sequence input shows its steps. An edit of the
 * input then edits that file, so the unchanged steps keep their attributes.
 */
type Source =
  | { readonly type: "typed" }
  | {
      readonly type: "trace-file";
      readonly name: string;
      /** The loaded file, or the trace of the session after a change through the view. */
      readonly text: string;
      readonly stage: "loaded" | "shown" | "edited";
    };

function element<T extends HTMLElement>(id: string, type: new () => T): T {
  const found = document.getElementById(id);
  if (!(found instanceof type)) {
    throw new Error(`The page has no ${type.name} with the id "${id}".`);
  }
  return found;
}

/**
 * The symbols of the sequence input: each line as typed, without empty lines.
 * A symbol can start or end with spaces, so the page keeps them.
 */
function typedSymbols(text: string): string[] {
  return text.split(/\r?\n/).filter((line) => line !== "");
}

function sourceText(source: Source): string {
  if (source.type === "typed") {
    return "The sequence comes from the input above.";
  }
  return source.stage === "edited"
    ? `The sequence comes from ${source.name}, with the edits in the input above.`
    : `The sequence comes from ${source.name}.`;
}

/**
 * The banner of the grammar view at a failure point: the rejected step, its
 * reason and the next options. The section "The failure point" of
 * docs/specs/output-view.md decides its content.
 */
function failureBanner(state: SessionState, grammar: CheckedGrammar): string | undefined {
  const k = state.failurePoint;
  const step = k === undefined ? undefined : state.steps[k - 1];
  if (k === undefined || step === undefined) {
    return undefined;
  }
  const next = state.nextOptions.map((o) => o.symbol);
  return [
    `Step ${k} ${JSON.stringify(step.symbol)} is rejected.`,
    rejectionReason(grammar, step.symbol),
    `Showing the state after step ${state.stepsRead}.`,
    next.length === 0 ? "No next option." : `Next: ${next.join(", ")}.`,
  ].join(" ");
}

function download(name: string, text: string): void {
  const url = URL.createObjectURL(new Blob([text], { type: "text/plain;charset=utf-8" }));
  const link = document.createElement("a");
  link.href = url;
  link.download = name;
  link.click();
  URL.revokeObjectURL(url);
}

async function main(): Promise<void> {
  // The page loads the font of the diagram labels before it draws the first diagram.
  const [, fontLoaded] = await Promise.all([loadCore(), loadDiagramFont()]);
  if (!fontLoaded) {
    element("status", HTMLParagraphElement).textContent =
      `The font ${diagramFontFamily} did not load, so the diagram labels can overflow their boxes.`;
  }

  const exampleSelect = element("example", HTMLSelectElement);
  const grammarFile = element("grammar-file", HTMLInputElement);
  const diagnosticsList = element("diagnostics", HTMLUListElement);
  const sequenceInput = element("sequence", HTMLTextAreaElement);
  const traceFile = element("trace-file", HTMLInputElement);
  const sourceLabel = element("source", HTMLParagraphElement);
  const saveButton = element("save", HTMLButtonElement);
  const panel = {
    acceptance: element("acceptance", HTMLParagraphElement),
    readings: element("readings", HTMLParagraphElement),
    nextOptions: element("next-options", HTMLParagraphElement),
    steps: element("steps", HTMLOListElement),
    messages: element("trace-messages", HTMLUListElement),
  };
  const output = {
    count: element("reading-count", HTMLSpanElement),
    status: element("output-status", HTMLSpanElement),
    notes: element("output-notes", HTMLUListElement),
    previous: element("reading-previous", HTMLButtonElement),
    next: element("reading-next", HTMLButtonElement),
    back: element("step-back", HTMLButtonElement),
    forward: element("step-forward", HTMLButtonElement),
    tree: element("outline", HTMLDivElement),
  };
  const details = {
    body: element("details-body", HTMLDivElement),
    folder: element("folder", HTMLDivElement),
    folderStatus: element("folder-status", HTMLSpanElement),
  };
  const folderOpen = element("folder-open", HTMLButtonElement);
  const folderInput = element("reference-folder", HTMLInputElement);

  const grammarView = createGrammarView({
    root: element("grammar-view", HTMLElement),
    note: element("grammar-view-note", HTMLParagraphElement),
    index: element("rule-index", HTMLUListElement),
    crumbs: element("breadcrumbs", HTMLOListElement),
    title: element("open-rule", HTMLHeadingElement),
    badge: element("open-rule-badge", HTMLSpanElement),
    doc: element("open-rule-doc", HTMLDivElement),
    frame: element("diagram-frame", HTMLDivElement),
    zoomIn: element("zoom-in", HTMLButtonElement),
    zoomOut: element("zoom-out", HTMLButtonElement),
    fit: element("zoom-fit", HTMLButtonElement),
    banner: element("grammar-view-banner", HTMLParagraphElement),
    outside: element("readings-outside", HTMLParagraphElement),
    unlisted: element("unlisted-options", HTMLParagraphElement),
    popover: element("option-popover", HTMLDivElement),
  });

  let grammarName = "grammar";
  let source: Source = { type: "typed" };
  let session: TraceSession | undefined;
  /** The checked grammar of the editor text, with the facts that the views need. */
  let grammar:
    | { text: string; checked: CheckedGrammar; facts: GrammarFacts; graph: RuleGraph }
    | undefined;
  /** The messages of the open session, such as the warnings of its trace file. */
  let messages: readonly TraceMessage[] = [];
  /** The reading on view, for the choice of the reading after the next change. */
  let viewed: Viewed | undefined;
  /** The model on view, so that a fold redraws without the core. */
  let model: OutputModel | undefined;
  // The rows that the person opened or folded. They last while the grammar stays.
  const opened = new Set<string>();
  const closed = new Set<string>();
  /** The selection: a row of the output view or an element of the grammar view. */
  let selection: Selection | undefined;
  /** The span keys of the grammar elements of the selection. */
  let selectedSpans: ReadonlySet<string> = new Set();
  /** The step in focus, with its count of attributes. */
  let focus: { step: number; attributes: number } | undefined;
  /** The folder that the person opened for the relative file references. */
  let folder: OpenedFolder | undefined;
  const links = new FileLinks();

  const handlers = {
    choose: (symbol: string): void => choose(symbol),
    toggle: (row: ViewRow): void => toggle(row),
    select: (row: ViewRow): void => selectRow(row),
  };

  /**
   * Finds the row of a selected row among every row of the view, with every
   * fold open. A selected row survives while the node that it stands for
   * exists. A step keeps its number in every reading, so a step row is found
   * by its step. A loop row becomes the fold row of the loop when the loop
   * finishes. Without such a row, the selection goes.
   */
  const resolveSelection = (): ViewRow | undefined => {
    const s = selection;
    if (model === undefined || s?.type !== "row") {
      return undefined;
    }
    const all = viewRows({ ...model, folds: allOpen, selection: undefined });
    const found =
      all.find((r) => r.toggleKey === s.toggleKey && r.step === s.step) ??
      (s.step === undefined
        ? all.find((r) => r.toggleKey === `${s.toggleKey}/completed`)
        : all.find((r) => r.step === s.step));
    selection =
      found === undefined
        ? undefined
        : { type: "row", toggleKey: found.toggleKey, step: found.step };
    model = { ...model, selection };
    return found;
  };

  /**
   * Shows the model in the output view, and its row in focus in the details
   * panel. It also finds the grammar elements of the selection and the step
   * in focus, for the grammar view. With `reveal`, the outline scrolls to the
   * row in focus.
   */
  const renderOutput = (reveal = false): void => {
    if (model === undefined || grammar === undefined) {
      return;
    }
    const selectedRow = resolveSelection();
    const rows = renderOutputView(output, model, handlers);
    const row = rowInFocus(rows);
    renderDetails(
      details,
      {
        row,
        state: model.state,
        grammar: grammar.checked,
        facts: grammar.facts,
        folder,
      },
      links,
    );
    if (reveal && row !== undefined) {
      revealRow(output.tree, row.key);
    }
    // The step in focus is the step of the row in focus. The core places a
    // data object only for a step that it read.
    const step = row?.step;
    focus =
      step !== undefined && step <= model.state.stepsRead
        ? { step, attributes: model.state.steps[step - 1]?.attributes.length ?? 0 }
        : undefined;
    const s = selection;
    selectedSpans =
      s?.type === "element"
        ? new Set([spanKey(s.span)])
        : selectedRow === undefined
          ? new Set()
          : (rowElements(selectedRow, grammar.checked, grammar.graph)?.spans ?? new Set());
  };

  /** Shows an empty output view and an empty details panel. */
  const noOutput = (): void => {
    renderNoOutput(output);
    details.body.replaceChildren();
    details.folder.hidden = folder === undefined;
  };

  /**
   * Marks the state on view, the selection and the data object of the step
   * in focus in the grammar view. After an operation on the trace, `open` is
   * "follow", and the open path follows the reading on view.
   */
  const markTrace = (open: OpenChange): void => {
    const current = session;
    if (current === undefined || grammar === undefined || model === undefined) {
      grammarView.showTrace(undefined, "keep");
      return;
    }
    const { state, reading } = model;
    const inFocus = focus;
    const marks: TraceMarks = {
      onView: reading,
      count: state.readings.length,
      followPath: state.readings[reading - 1]?.followPath ?? [],
      order: state.nextOptions.map((o) => o.symbol),
      marks: (openPath) => current.marks([...openPath], inFocus?.step),
      banner: failureBanner(state, grammar.checked),
      choose,
      focus: inFocus,
      selected: selectedSpans,
      selectElement,
      selectStep,
    };
    grammarView.showTrace(marks, open);
  };

  /** Shows a trace panel without a session, and a grammar view without marks. */
  const noSession = (text: string, notes: readonly TraceMessage[]): void => {
    renderNoSession(panel, text, notes);
    grammarView.showTrace(undefined, "keep");
  };

  /**
   * Shows the state on view of the session in the trace panel, the output view
   * and the grammar view. `follow` is true after an operation on the trace.
   */
  const show = (follow = true): void => {
    if (session === undefined || grammar === undefined) {
      return;
    }
    const state = session.state();
    const { reading, ended } = chooseReading(viewed, grammar.text, state);
    viewed = {
      grammar: grammar.text,
      read: state.steps.slice(0, state.stepsRead).map((s) => s.symbol),
      readings: state.readings,
      reading,
    };
    model = {
      state,
      grammar: grammar.checked,
      facts: grammar.facts,
      reading,
      tree: reading > 0 ? session.tree(reading) : undefined,
      folds: { opened, closed },
      ended,
      selection,
    };
    renderState(panel, state, messages);
    renderOutput();
    saveButton.disabled = false;
    markTrace(follow ? "follow" : "keep");
  };

  const showReading = (reading: number): void => {
    if (session === undefined || model === undefined || viewed === undefined) {
      return;
    }
    viewed = { ...viewed, reading };
    model = { ...model, reading, tree: session.tree(reading), ended: undefined };
    renderOutput();
    // A switch of reading is an operation on the trace.
    markTrace("follow");
  };

  const toggle = (row: ViewRow): void => {
    const set = row.kinds.includes("fold") ? opened : closed;
    if (!set.delete(row.toggleKey)) {
      set.add(row.toggleKey);
    }
    renderOutput();
    // A fold can change the row in focus, and so the step in focus.
    markTrace("keep");
  };

  /**
   * Selects a row of the output view. It becomes the row in focus of the
   * details panel. The grammar view selects the elements of its node, and it
   * opens the diagram that holds them.
   */
  const selectRow = (row: ViewRow): void => {
    if (model === undefined || grammar === undefined) {
      return;
    }
    if (selection?.type === "row" && selection.toggleKey === row.toggleKey) {
      return;
    }
    selection = { type: "row", toggleKey: row.toggleKey, step: row.step };
    model = { ...model, selection };
    renderOutput();
    markTrace(rowElements(row, grammar.checked, grammar.graph)?.openPath ?? "keep");
  };

  /**
   * Selects an element of the grammar view. The output view selects each
   * matching row of the reading on view, and it scrolls to the last one.
   */
  const selectElement = (span: Span): void => {
    if (model === undefined) {
      return;
    }
    selection = { type: "element", span };
    model = { ...model, selection };
    renderOutput(true);
    markTrace("keep");
  };

  /** Selects the row of a step: a click on the data object of the grammar view. */
  const selectStep = (step: number): void => {
    if (model === undefined) {
      return;
    }
    const row = viewRows({ ...model, folds: allOpen }).find((r) => r.step === step);
    if (row !== undefined) {
      selectRow(row);
    }
  };

  /**
   * Shows the steps of the session in the sequence input, after a change
   * through the view. A later edit of the input then edits the trace of the
   * session, not the loaded file. Only the session knows which step the view
   * inserted, so each step keeps its own attributes.
   */
  const syncSequence = (): void => {
    if (session === undefined) {
      return;
    }
    sequenceInput.value = session
      .state()
      .steps.map((s) => s.symbol)
      .join("\n");
    if (source.type === "trace-file") {
      source = { ...source, text: session.save(), stage: "edited" };
    }
    sourceLabel.textContent = sourceText(source);
  };

  /** Inserts a next option right after the state on view, and reads the inserted step. */
  const choose = (symbol: string): void => {
    if (session === undefined) {
      return;
    }
    const error = session.insert(symbol);
    if (error !== undefined) {
      messages = [{ severity: "error", code: error.code, text: error.message }];
    } else {
      session.read();
    }
    syncSequence();
    show();
  };

  /**
   * Opens a new session on the grammar text and the sequence. `follow` is
   * false after a change of the grammar text, which is not an operation on
   * the trace, so the open path stays.
   */
  const retrace = (grammar: string, hasErrors: boolean, follow: boolean): void => {
    session?.free();
    session = undefined;
    model = undefined;
    saveButton.disabled = true;
    noOutput();
    sourceLabel.textContent = sourceText(source);
    if (hasErrors) {
      noSession("The grammar has errors, so it has no traces.", []);
      return;
    }
    const symbols = typedSymbols(sequenceInput.value);
    const opened: Opened =
      source.type === "typed"
        ? openFromSymbols(grammar, symbols)
        : source.stage === "edited"
          ? openFromEditedTraceFile(grammar, source.text, symbols)
          : openFromTraceFile(grammar, source.text);
    if (!opened.ok) {
      const report = opened.report;
      switch (report.type) {
        case "grammar-errors":
          noSession("The grammar has errors, so it has no traces.", []);
          return;
        case "trace-file-errors": {
          const text = source.type === "trace-file" ? source.text : "";
          noSession("The trace file has errors.", traceFileMessages(text, report.diagnostics));
          return;
        }
        case "invalid-step": {
          const message: TraceMessage = {
            severity: "error",
            code: report.error.code,
            text: report.error.message,
          };
          noSession("A step of the sequence is not valid.", [message]);
          return;
        }
      }
    }
    session = opened.session;
    messages =
      source.type === "trace-file"
        ? [
            ...traceFileMessages(source.text, opened.warnings),
            ...attributesDroppedMessages(opened.attributesDropped),
          ]
        : [];
    show(follow);
    if (source.type === "trace-file" && source.stage === "loaded") {
      sequenceInput.value = session
        .state()
        .steps.map((s) => s.symbol)
        .join("\n");
      source = { ...source, stage: "shown" };
    }
  };

  /**
   * Checks and lays out the grammar text, and traces the sequence again.
   * `follow` is true when the trace changes too, as when a trace file opens.
   */
  const refresh = (text: string, follow = false): void => {
    const checked = check(text);
    if (grammar?.text !== text) {
      opened.clear();
      closed.clear();
      selection = undefined;
    }
    grammar = { text, checked, facts: grammarFacts(checked, text), graph: ruleGraph(checked) };
    editor.showDiagnostics(checked.diagnostics);
    renderDiagnostics(diagnosticsList, checked.diagnostics, (d) => editor.select(d));
    grammarView.update(checked, layOut(text));
    retrace(
      text,
      checked.diagnostics.some((d) => d.severity === "error"),
      follow,
    );
  };

  const editor = createEditor(element("editor", HTMLDivElement), refresh);

  const loadGrammar = (name: string, text: string): void => {
    grammarName = name;
    if (editor.text() === text) {
      refresh(text);
    } else {
      editor.setText(text);
    }
  };

  for (const example of examples) {
    exampleSelect.append(new Option(example.name, example.name));
  }
  exampleSelect.addEventListener("change", () => {
    const example = examples.find((e) => e.name === exampleSelect.value);
    if (example !== undefined) {
      loadGrammar(example.name, example.text);
    }
  });

  grammarFile.addEventListener("change", async () => {
    const file = grammarFile.files?.[0];
    if (file !== undefined) {
      exampleSelect.value = "";
      loadGrammar(file.name.replace(/\.ebnf$/, ""), await file.text());
    }
    grammarFile.value = "";
  });

  sequenceInput.addEventListener("input", () => {
    // The input shows the steps of a trace file only after the file opens.
    source =
      source.type === "trace-file" && source.stage !== "loaded"
        ? { ...source, stage: "edited" }
        : { type: "typed" };
    // An edit of the sequence changes the trace, so the open path follows it.
    retrace(
      editor.text(),
      check(editor.text()).diagnostics.some((d) => d.severity === "error"),
      true,
    );
  });

  traceFile.addEventListener("change", async () => {
    const file = traceFile.files?.[0];
    if (file !== undefined) {
      source = { type: "trace-file", name: file.name, text: await file.text(), stage: "loaded" };
      // The open of a trace file is an operation on the trace.
      refresh(editor.text(), true);
    }
    traceFile.value = "";
  });

  output.previous.addEventListener("click", () => {
    if (model !== undefined && model.reading > 1) {
      showReading(model.reading - 1);
    }
  });
  output.next.addEventListener("click", () => {
    if (model !== undefined && model.reading < model.state.readings.length) {
      showReading(model.reading + 1);
    }
  });
  output.back.addEventListener("click", () => {
    if (session?.back()) {
      show();
    }
  });
  output.forward.addEventListener("click", () => {
    if (session !== undefined) {
      session.read();
      show();
    }
  });

  folderOpen.addEventListener("click", () => folderInput.click());
  folderInput.addEventListener("change", () => {
    const files = folderInput.files;
    if (files !== null && files.length > 0) {
      // A new folder replaces the last one, and the links to its files go.
      links.clear();
      folder = new OpenedFolder(files);
      renderOutput();
    }
    folderInput.value = "";
  });

  saveButton.addEventListener("click", () => {
    if (session !== undefined && grammar !== undefined) {
      download(
        source.type === "trace-file" ? source.name : `${grammarName}.trace.kdl`,
        traceFileText(grammar.text, session),
      );
    }
  });

  const first = examples.find((e) => e.name === "tutorial") ?? examples[0];
  if (first !== undefined) {
    exampleSelect.value = first.name;
    loadGrammar(first.name, first.text);
  }
  document.body.dataset.ready = "true";
}

main().catch((error: unknown) => {
  const status = document.getElementById("status");
  if (status !== null) {
    status.textContent = `The workbench failed to start: ${String(error)}`;
  }
  throw error;
});
