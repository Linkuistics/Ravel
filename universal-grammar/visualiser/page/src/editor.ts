// The grammar editor: a CodeMirror 6 view that marks the diagnostics of the
// core at their spans.

import { defaultKeymap, history, historyKeymap } from "@codemirror/commands";
import { type Diagnostic as EditorDiagnostic, lintGutter, setDiagnostics } from "@codemirror/lint";
import {
  drawSelection,
  EditorView,
  highlightActiveLine,
  keymap,
  lineNumbers,
} from "@codemirror/view";
import type { Diagnostic } from "./core.ts";
import { TextIndex } from "./offsets.ts";

export interface GrammarEditor {
  /** The text of the editor. Its line breaks are LF. */
  readonly text: () => string;
  /** Replaces the whole text. The change listener runs. */
  readonly setText: (text: string) => void;
  /** Marks the diagnostics of the text that the editor holds now. */
  readonly showDiagnostics: (diagnostics: readonly Diagnostic[]) => void;
  /** Selects the span of a diagnostic and moves the focus to the editor. */
  readonly select: (diagnostic: Diagnostic) => void;
}

/**
 * Creates the editor in `parent`. The editor calls `onChange` after each
 * change of its text.
 *
 * CodeMirror keeps LF line breaks in its document, so the core checks the
 * text with LF line breaks. The spans of the core then index that text.
 */
export function createEditor(parent: HTMLElement, onChange: (text: string) => void): GrammarEditor {
  const view = new EditorView({
    parent,
    extensions: [
      lineNumbers(),
      history(),
      drawSelection(),
      highlightActiveLine(),
      lintGutter(),
      keymap.of([...defaultKeymap, ...historyKeymap]),
      EditorView.contentAttributes.of({ "aria-label": "Grammar text" }),
      EditorView.updateListener.of((update) => {
        if (update.docChanged) {
          onChange(update.state.doc.toString());
        }
      }),
    ],
  });

  const range = (diagnostic: Diagnostic): { from: number; to: number } => {
    const index = new TextIndex(view.state.doc.toString());
    return { from: index.utf16(diagnostic.span.start), to: index.utf16(diagnostic.span.end) };
  };

  return {
    text: () => view.state.doc.toString(),
    setText: (text) => {
      view.dispatch({ changes: { from: 0, to: view.state.doc.length, insert: text } });
    },
    showDiagnostics: (diagnostics) => {
      const index = new TextIndex(view.state.doc.toString());
      const marks: EditorDiagnostic[] = diagnostics.map((d) => ({
        from: index.utf16(d.span.start),
        to: index.utf16(d.span.end),
        severity: d.severity,
        message: d.message,
        source: d.code,
      }));
      view.dispatch(setDiagnostics(view.state, marks));
    },
    select: (diagnostic) => {
      const { from, to } = range(diagnostic);
      view.dispatch({ selection: { anchor: from, head: to }, scrollIntoView: true });
      view.focus();
    },
  };
}
