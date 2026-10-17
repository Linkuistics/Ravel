// Examine the book's text against the ASD-STE100 writing rules that a script
// can decide, and against the book's word decisions in docs/book/ste.md.
// scripts/ste.sh runs it: argv is the decisions file, the flag --strict when
// it is given, then the source files (Markdown pages and figure .layout.json
// records). Code blocks and code spans are removed first; a code span counts
// as one word. It reports each problem as FILE:LINE: RULE: TEXT, then a count
// for each file. Without --strict it exits 0 after the report; with --strict
// it exits 1 if it found a problem.
import { readFileSync } from "node:fs";

const [decisionsPath, ...rest] = process.argv.slice(2);
const strict = rest.includes("--strict");
const sources = rest.filter((a) => a !== "--strict");
if (!decisionsPath || sources.length === 0) {
  console.error("usage: ste.ts DECISIONS [--strict] SOURCE...");
  process.exit(2);
}

// ------------------------------------------------------------- the decisions

// Two tables of ste.md: "Words that the book does not use" (words in each
// first cell, with the words to use in their place) and "Words in -ing that
// the book keeps" (a word or a phrase in each first cell).
const avoid = new Map<string, string>();
const keep: string[][] = [];
{
  let table: "avoid" | "keep" | null = null;
  for (const line of readFileSync(decisionsPath, "utf8").split("\n")) {
    const h = line.match(/^##+\s+(.*)/);
    if (h) {
      const t = h[1].toLowerCase();
      table = t.startsWith("words that the book does not use") ? "avoid" : t.startsWith("words in -ing") ? "keep" : null;
      continue;
    }
    if (!table || !line.startsWith("|") || /^\|\s*-/.test(line)) continue;
    const cells = line.split("|").slice(1, -1).map((c) => c.trim());
    if (/^word( or phrase)?$/i.test(cells[0])) continue;
    for (const k of cells[0].split(/,\s*/).map((k) => k.toLowerCase())) {
      if (table === "avoid") avoid.set(k, cells[1] ?? "");
      else keep.push(k.split(/[\s-]+/));
    }
  }
  if (avoid.size === 0 || keep.length === 0) {
    console.error(`ste: ${decisionsPath} has no word tables: is it docs/book/ste.md?`);
    process.exit(2);
  }
}

// ---------------------------------------------------------------- the blocks

// A block is a unit of text with the line where it starts: a paragraph, a
// heading, a list item, a table cell or the text of one figure element.
// An _Avoid_ line of CONTEXT.md names the words to avoid, so the word
// decisions do not apply to it.
type Block = { line: number; kind: "prose" | "step" | "item" | "heading" | "cell" | "figure" | "avoid"; text: string };

function inline(text: string): string {
  return text
    .replace(/<!--[\s\S]*?-->/g, " ")
    .replace(/`[^`\n]*`/g, " CODE ") // a code span is one word
    .replace(/!?\[([^\]]*)\]\([^)]*\)/g, "$1") // keep link and image text
    .replace(/\[([^\]]*)\]\[[^\]]*\]/g, "$1")
    .replace(/<https?:[^>]*>/g, " URL ")
    .replace(/https?:\/\/\S+/g, " URL ")
    .replace(/<\/?[a-zA-Z][^>]*>/g, " ")
    .replace(/\{#[^}]*\}/g, " ")
    .replace(/\*\*|__/g, "");
}

function markdownBlocks(text: string): Block[] {
  const blocks: Block[] = [];
  let current: Block | null = null;
  let fence: string | null = null;
  const end = () => {
    if (current && current.text.trim()) blocks.push(current);
    current = null;
  };
  text.split("\n").forEach((raw, i) => {
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
    let line = raw.replace(/^\s*>\s?/, ""); // a blockquote is prose
    if (!line.trim()) return end();
    if (/^\s*\[[^\]]+\]:\s*\S+/.test(line)) return end(); // a reference definition
    if (/^#{1,6}\s/.test(line)) {
      end();
      blocks.push({ line: n, kind: "heading", text: inline(line.replace(/^#+\s*/, "")) });
      return;
    }
    if (/^\s*\|/.test(line)) {
      end();
      if (/^\s*\|[\s|:-]+\|\s*$/.test(line)) return; // a separator row
      for (const cell of line.trim().replace(/^\||\|$/g, "").split(/(?<!\\)\|/))
        if (cell.trim()) blocks.push({ line: n, kind: "cell", text: inline(cell) });
      return;
    }
    if (/^_Avoid_:/.test(line)) {
      end();
      blocks.push({ line: n, kind: "avoid", text: inline(line) });
      return;
    }
    const item = line.match(/^\s*([-*+]|\d+[.)])\s+(.*)$/);
    if (item) {
      end();
      current = { line: n, kind: /\d/.test(item[1]) ? "step" : "item", text: inline(item[2]) };
      return;
    }
    line = inline(line);
    if (current) current.text += " " + line;
    else current = { line: n, kind: "prose", text: line };
  });
  end();
  return blocks;
}

function figureBlocks(text: string): Block[] {
  const blocks: Block[] = [];
  const walk = (v: unknown) => {
    if (Array.isArray(v)) v.forEach(walk);
    else if (v && typeof v === "object")
      for (const [k, x] of Object.entries(v))
        if (k === "text" && typeof x === "string") blocks.push({ line: 1, kind: "figure", text: x.replace(/\s+/g, " ") });
        else walk(x);
  };
  walk(JSON.parse(text));
  return blocks;
}

// --------------------------------------------------------------- the checks

const WORD = /[\p{L}\p{N}][\p{L}\p{N}'’./_-]*[\p{L}\p{N}]|[\p{L}\p{N}]/gu;
const count = (s: string) => (s.match(WORD) ?? []).length;

// A sentence ends at . ! or ? before a space and a capital letter, a digit or
// an opening mark, or at the end of the block.
function sentences(text: string): string[] {
  return text
    .split(/(?<=[.!?][”"’)]*)\s+(?=[“"(\[]?[\p{Lu}\p{N}])/u)
    .map((s) => s.trim())
    .filter((s) => count(s) > 0);
}

const excerpt = (s: string) => (s.length > 90 ? s.slice(0, 87) + "..." : s);

// A kept word, or a word of a kept phrase that the text has at this place.
function kept(parts: string[], at: number): boolean {
  return keep.some((phrase) =>
    phrase.some((p, k) => p === parts[at] && phrase.every((q, j) => parts[at - k + j] === q)),
  );
}

type Problem = { file: string; line: number; rule: string; text: string };
const problems: Problem[] = [];

function check(file: string, blocks: Block[]) {
  for (const b of blocks) {
    const add = (rule: string, text: string) => problems.push({ file, line: b.line, rule, text });
    const ss = sentences(b.text);
    for (const s of ss) {
      const limit = b.kind === "step" ? 20 : 25;
      if (count(s) > limit) add(`${limit === 20 ? "Rule 5.1" : "Rule 6.3"} sentence of ${count(s)} words (limit ${limit})`, excerpt(s));
    }
    if (b.kind === "prose" && ss.length > 6) add(`Rule 6.6 paragraph of ${ss.length} sentences (limit 6)`, excerpt(b.text));
    const semi = b.text.indexOf(";");
    if (semi >= 0) add("Rule 8.1 semicolon", excerpt(b.text.slice(Math.max(0, semi - 40), semi + 30)));
    for (const m of b.text.matchAll(/\p{L}+(?:n['’]t|['’](?:re|ve|ll|d|m))(?!\p{L})/giu)) add("Rule 4.2 contraction", m[0]);
    for (const m of b.text.matchAll(/\b(MUST|SHOULD|MAY|SHALL|REQUIRED)\b/g)) add("normative keyword (contract)", m[0]);
    // A hyphenated word is examined in its parts: "pre-start", "operating-system".
    const parts = (b.text.match(WORD) ?? []).flatMap((w) => w.toLowerCase().replace(/['’]s$/, "").split("-"));
    parts.forEach((p, i) => {
      if (avoid.has(p) && b.kind !== "avoid") add("word that the book does not use (ste.md)", `${p}: use ${avoid.get(p)}`);
      if (p.length > 4 && p.endsWith("ing") && !kept(parts, i)) add("Rule 3.5 word in -ing (ste.md)", p);
    });
  }
}

for (const file of sources) {
  const text = readFileSync(file, "utf8");
  check(file, file.endsWith(".json") ? figureBlocks(text) : markdownBlocks(text));
}

for (const p of problems) console.log(`${p.file}:${p.line}: ${p.rule}: ${p.text}`);
const byFile = new Map<string, number>();
for (const p of problems) byFile.set(p.file, (byFile.get(p.file) ?? 0) + 1);
for (const file of sources) console.log(`ste: ${file}: ${byFile.get(file) ?? 0} problems`);
console.log(`ste: ${problems.length} problems in ${sources.length} files`);
if (strict && problems.length > 0) process.exit(1);
