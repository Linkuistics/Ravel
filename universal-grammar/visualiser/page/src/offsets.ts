// The core places each diagnostic by a span of UTF-8 byte offsets. The
// editor and the DOM count UTF-16 code units. This module converts one to the
// other for one text.

/** The UTF-16 index of each UTF-8 byte offset of one text. */
export class TextIndex {
  readonly #utf16At: Uint32Array;
  readonly #text: string;

  constructor(text: string) {
    this.#text = text;
    const at: number[] = [];
    let unit = 0;
    // A `for…of` loop visits code points. A lone surrogate becomes U+FFFD in
    // UTF-8, which also takes 3 bytes, so the offsets stay in step.
    for (const char of text) {
      const point = char.codePointAt(0) ?? 0;
      const width = point < 0x80 ? 1 : point < 0x800 ? 2 : point < 0x10000 ? 3 : 4;
      for (let i = 0; i < width; i++) {
        at.push(unit);
      }
      unit += char.length;
    }
    at.push(unit);
    this.#utf16At = Uint32Array.from(at);
  }

  /** The UTF-16 index of a byte offset. An offset past the end gives the end. */
  utf16(byte: number): number {
    const last = this.#utf16At.length - 1;
    return this.#utf16At[Math.min(Math.max(byte, 0), last)] ?? 0;
  }

  /** The line and the column of a byte offset, both from 1. A column counts code points. */
  lineAndColumn(byte: number): { readonly line: number; readonly column: number } {
    const before = this.#text.slice(0, this.utf16(byte));
    const lineStart = before.lastIndexOf("\n") + 1;
    const line = before.split("\n").length;
    const column = [...before.slice(lineStart)].length + 1;
    return { line, column };
  }
}
