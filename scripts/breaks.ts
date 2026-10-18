// Report each run of prose in the book that is longer than the limit of
// docs/book/contract.md, section 2, "Breaks in prose". scripts/breaks.sh runs
// it: argv is the flag --strict when it is given, then the chapter files.
// A run is the text between two breaks. A fenced code block, a table row (a
// line that starts with `|`) and an image (a line that starts with `![`)
// break a run. A heading and a list do not. A heading, a blank line and a
// fence marker add no words. Paragraphs, list items and blockquotes add their
// words: the pieces of text that are separated by white space, after a link
// is reduced to its text and an HTML comment is removed; a heading's text, a
// blockquote's `>` and a list's marker are not words. For each run longer
// than the limit it reports FILE:FIRST-LAST: WORDS words, then the headings.
// It prints a count for each file. Without --strict it exits 0 after the
// report; with --strict it exits 1 if it found a long run.
import { readFileSync } from "node:fs";

const LIMIT = 450; // contract.md: "about 450 words", one screen at the 750-pixel column

const rest = process.argv.slice(2);
const strict = rest.includes("--strict");
const sources = rest.filter((a) => a !== "--strict");
if (sources.length === 0) {
  console.error("usage: breaks.ts [--strict] CHAPTER...");
  process.exit(2);
}

type Run = { first: number; last: number; words: number; headings: string[] };
type Long = Run & { file: string };

const NESTED_PARENS = "\\((?:[^()]|\\([^()]*\\))*\\)";

// A comment adds no words and keeps its line breaks, so that line numbers
// stay true. A link keeps only its text, and a reference definition goes.
const uncommented = (text: string) => text.replace(/<!--[\s\S]*?-->/g, (c) => c.replace(/[^\n]/g, ""));
const linkText = (line: string) =>
  line
    .replace(new RegExp(`\\[([^\\]]*)\\]${NESTED_PARENS}`, "g"), "$1")
    .replace(/\[([^\]]*)\]\[[^\]]*\]/g, "$1")
    .replace(/^\s*\[[^\]]+\]:\s*\S+.*$/, "");

function runs(text: string): Run[] {
  const found: Run[] = [];
  let run: Run | null = null;
  let heading = ""; // the heading that a run starts under
  let fence: string | null = null;
  const end = () => {
    if (run && run.words > 0) found.push(run);
    run = null;
  };
  uncommented(text)
    .split("\n")
    .forEach((raw, i) => {
      const n = i + 1;
      const f = raw.match(/^\s*(```|~~~)/);
      if (fence) {
        if (f && f[1] === fence) fence = null;
        return;
      }
      if (f) {
        end();
        fence = f[1];
        return;
      }
      if (/^\s*\|/.test(raw) || /^\s*!\[/.test(raw)) return end(); // a table row or an image
      if (!raw.trim()) return;
      const h = raw.match(/^#{1,6}\s+(.*?)\s*#*\s*$/);
      if (h) {
        heading = h[1];
        if (run) run.headings.push(h[1]);
        return;
      }
      const words = linkText(raw).replace(/^(?:\s*>)*\s*(?:(?:[-*+]|\d+[.)])\s+)?/, "").split(/\s+/).filter(Boolean).length;
      if (!run) run = { first: n, last: n, words: 0, headings: heading ? [heading] : [] };
      run.words += words;
      run.last = n;
    });
  end();
  return found;
}

const longRuns: Long[] = [];
const byFile = new Map<string, number>();
for (const file of sources) {
  const long = runs(readFileSync(file, "utf8")).filter((r) => r.words > LIMIT);
  byFile.set(file, long.length);
  for (const r of long) longRuns.push({ ...r, file });
}

for (const r of longRuns)
  console.log(`${r.file}:${r.first}-${r.last}: ${r.words} words (limit ${LIMIT}); headings: ${r.headings.map((h) => `"${h}"`).join(", ")}`);
for (const file of sources) console.log(`breaks: ${file}: ${byFile.get(file)} runs longer than ${LIMIT} words`);
console.log(`breaks: ${longRuns.length} runs longer than ${LIMIT} words in ${sources.length} files`);
if (strict && longRuns.length > 0) process.exit(1);
