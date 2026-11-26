// The end-to-end tests of the grammar view: the second test seam of the root
// brief. The expected structure comes from the files that the core wrote for
// each example: the checked grammar and the diagram KDL. The tests check
// structure, not positions, apart from the widths of the labels.

import { readdir, readFile } from "node:fs/promises";
import { join } from "node:path";
import { expect, type Locator, type Page, test } from "@playwright/test";

const examplesDir = join(import.meta.dirname, "..", "..", "examples");
const exampleNames = ["order-approval", "tutorial", "urg-moves"];
// The diagram cases of the core: a grammar and its expected diagram KDL.
const diagramCasesDir = join(
  import.meta.dirname,
  "..",
  "..",
  "crates",
  "core",
  "tests",
  "diagrams",
);

/** The width of one cell of the bundled font at 13 px: the spec rule-diagrams. */
const cellWidth = 7.8;

/** The structure of one diagram in the diagram KDL of the core. */
interface DiagramFacts {
  /** The names of the boxes for assistive technology, in the order of the text. */
  boxes: string[];
  /** The rules of the reference boxes, in the order of the text. */
  references: string[];
  /** The count of reference boxes with a drill-down marker. */
  drillDowns: number;
  /** The count of reference boxes that link a loop through rules. */
  loopLinks: number;
  gateways: number;
  /** The roles of the flows, in the order of the layout. */
  flows: string[];
  /**
   * For each flow, whether it has an arrowhead. A flow has none where it ends
   * at the first point of another flow: the section "The diagram KDL" of the spec.
   */
  arrowheads: boolean[];
}

/** A bare identifier or a quoted string of KDL 2.0, as the core writes them. */
const kdlValue = String.raw`("(?:[^"\\]|\\.)*"|[^\s"{}]+)`;

function kdlText(value: string): string {
  return value.startsWith('"') ? (JSON.parse(value) as string) : value;
}

/** Reads a diagram KDL file of the core, rule by rule. */
async function diagramFacts(file: string): Promise<Map<string, DiagramFacts>> {
  const text = await readFile(file, "utf8");
  const facts = new Map<string, DiagramFacts>();
  const points = new Map<DiagramFacts, string[][]>();
  let current: DiagramFacts | undefined;
  for (const line of text.split("\n")) {
    const diagram = line.match(new RegExp(`^ {4}diagram rule=${kdlValue} `));
    if (diagram !== null) {
      current = {
        boxes: [],
        references: [],
        drillDowns: 0,
        loopLinks: 0,
        gateways: 0,
        flows: [],
        arrowheads: [],
      };
      facts.set(kdlText(diagram[1] ?? ""), current);
      points.set(current, []);
      continue;
    }
    if (current === undefined) {
      continue;
    }
    const box = line.match(
      new RegExp(`^ {8}(task|reference) start=\\d+ end=\\d+ (?:symbol|rule)=${kdlValue} `),
    );
    if (box !== null) {
      const value = kdlText(box[2] ?? "");
      current.boxes.push(`${box[1]} ${value}`);
      if (box[1] === "reference") {
        current.references.push(value);
        current.drillDowns += line.includes(" one-step=#false") ? 1 : 0;
        current.loopLinks += line.includes(" loop-link=") ? 1 : 0;
      }
    } else if (/^ {8}gateway /.test(line)) {
      current.gateways++;
    } else {
      const flow = line.match(/^ {8}flow ((?:-?\d+ )+)role=(\S+)/);
      if (flow !== null) {
        current.flows.push(flow[2] ?? "");
        const xy = (flow[1] ?? "").trim().split(" ");
        points.get(current)?.push(xy);
      }
    }
  }
  for (const [fact, flows] of points) {
    const starts = new Set(flows.map((xy) => `${xy[0]},${xy[1]}`));
    fact.arrowheads = flows.map((xy) => !starts.has(`${xy[xy.length - 2]},${xy[xy.length - 1]}`));
  }
  return facts;
}

const exampleFacts = (name: string): Promise<Map<string, DiagramFacts>> =>
  diagramFacts(join(examplesDir, `${name}.diagrams.kdl`));

/** Checks the structure of the open diagram against the diagram KDL of the core. */
async function expectDiagram(page: Page, fact: DiagramFacts): Promise<void> {
  const shown = diagram(page);
  await expect(shown).toHaveAttribute("role", "graphics-document");
  await expect(shown.locator('.box[role="graphics-symbol"]')).toHaveCount(fact.boxes.length);
  // The label of a box also names its marks, so the fixed name is in `data-name`.
  expect(
    await shown
      .locator(".box")
      .evaluateAll((els) => els.map((e) => (e as SVGElement).dataset.name)),
  ).toEqual(fact.boxes);
  for (const label of await shown
    .locator(".box")
    .evaluateAll((els) => els.map((e) => [e.ariaLabel, (e as SVGElement).dataset.name]))) {
    expect(label[0]?.startsWith(label[1] ?? "")).toBe(true);
  }
  await expect(shown.locator(".box.reference .drill-down")).toHaveCount(fact.drillDowns);
  await expect(shown.locator(".box.reference .loop-sign")).toHaveCount(fact.loopLinks);
  await expect(shown.locator(".gateway")).toHaveCount(fact.gateways);
  expect(await shown.locator(".flow").evaluateAll((els) => els.map((e) => e.dataset.role))).toEqual(
    fact.flows,
  );
  expect(
    await shown
      .locator(".flow .line")
      .evaluateAll((els) => els.map((e) => e.hasAttribute("marker-end"))),
  ).toEqual(fact.arrowheads);
}

/** The rule names of an example, in the order of the text, from the checked grammar. */
async function checkedRules(name: string): Promise<string[]> {
  const text = await readFile(join(examplesDir, `${name}.checked.kdl`), "utf8");
  return [...text.matchAll(/^ {4}rule name=(\S+)/gm)].map((m) => m[1] ?? "");
}

async function startRule(name: string): Promise<string> {
  const text = await readFile(join(examplesDir, `${name}.checked.kdl`), "utf8");
  return text.match(/^ {4}start-rule name=(\S+)/m)?.[1] ?? "";
}

/** The badge of each member of a recursive group, from the checked grammar. */
async function checkedBadges(name: string): Promise<Map<string, string>> {
  const text = await readFile(join(examplesDir, `${name}.checked.kdl`), "utf8");
  const badges = new Map<string, string>();
  for (const group of text.matchAll(
    /^ {4}recursive-group badge=(\S+) \{\n((?: {8}member rule=\S+\n)+)/gm,
  )) {
    for (const member of (group[2] ?? "").matchAll(/rule=(\S+)/g)) {
      badges.set(member[1] ?? "", group[1] ?? "");
    }
  }
  return badges;
}

/**
 * The first shortest chain of references from the start rule, by the reference
 * boxes of the diagram KDL. A self tail call that draws as a back edge has no
 * box, but a chain through a rule to itself is never the shortest.
 */
function expectedPath(facts: Map<string, DiagramFacts>, start: string, rule: string): string[] {
  const parents = new Map<string, string | undefined>([[start, undefined]]);
  const queue = [start];
  for (let i = 0; i < queue.length; i++) {
    const at = queue[i] ?? "";
    for (const target of facts.get(at)?.references ?? []) {
      if (!parents.has(target)) {
        parents.set(target, at);
        queue.push(target);
      }
    }
  }
  if (!parents.has(rule)) {
    return [rule];
  }
  const path: string[] = [];
  for (let at: string | undefined = rule; at !== undefined; at = parents.get(at)) {
    path.unshift(at);
  }
  return path;
}

async function open(page: Page): Promise<void> {
  await page.goto("/");
  await expect(page.locator("body")).toHaveAttribute("data-ready", "true");
}

async function loadGrammarText(page: Page, name: string, text: string): Promise<void> {
  await page.locator("#grammar-file").setInputFiles({
    name,
    mimeType: "text/plain",
    buffer: Buffer.from(text, "utf8"),
  });
}

const diagram = (page: Page): Locator => page.locator("#diagram-frame svg.diagram");
const box = (page: Page, kind: "task" | "reference", name: string): Locator =>
  diagram(page).locator(`.box[data-name="${kind} ${name}"]`);

async function expectOpenPath(page: Page, path: readonly string[]): Promise<void> {
  await expect(page.locator("#breadcrumbs li .crumb-rule")).toHaveText([...path]);
  const rule = path[path.length - 1] ?? "";
  await expect(diagram(page)).toHaveAttribute("aria-label", rule);
  await expect(page.locator("#open-rule")).toHaveText(rule);
}

async function openFromIndex(page: Page, rule: string): Promise<void> {
  await page.locator(`#rule-index li[data-rule="${rule}"] button`).click();
}

test("the rule index opens the diagram of each rule of each example", async ({ page }) => {
  await open(page);
  let diagramsSeen = 0;
  for (const name of exampleNames) {
    await page.locator("#example").selectOption(name);
    const facts = await exampleFacts(name);
    const rules = await checkedRules(name);
    const start = await startRule(name);
    const badges = await checkedBadges(name);
    await expect(page.locator("#rule-index li .rule-name")).toHaveText(rules);
    await expectOpenPath(page, [start]);
    expect([...facts.keys()]).toEqual(rules);
    for (const rule of rules) {
      const fact = facts.get(rule);
      if (fact === undefined) {
        throw new Error(`The diagram KDL of ${name} has no diagram of ${rule}.`);
      }
      await openFromIndex(page, rule);
      await expectOpenPath(page, expectedPath(facts, start, rule));
      await expectDiagram(page, fact);
      const badge = badges.get(rule);
      if (badge === undefined) {
        await expect(page.locator("#open-rule-badge")).toBeHidden();
      } else {
        await expect(page.locator("#open-rule-badge")).toHaveAttribute("data-badge", badge);
      }
      diagramsSeen++;
    }
  }
  // The three examples hold 26 rules.
  expect(diagramsSeen).toBe(26);
});

test("the drawing of each construct follows the diagram KDL of the core", async ({ page }) => {
  await open(page);
  const names = (await readdir(diagramCasesDir))
    .filter((f) => f.endsWith(".ebnf"))
    .map((f) => f.replace(/\.ebnf$/, ""))
    .sort();
  let arrowless = 0;
  let diagrams = 0;
  for (const name of names) {
    const grammar = await readFile(join(diagramCasesDir, `${name}.ebnf`), "utf8");
    const facts = await diagramFacts(join(diagramCasesDir, `${name}.diagrams.kdl`));
    await loadGrammarText(page, `${name}.ebnf`, grammar);
    await expect(page.locator("#grammar-view"), name).toHaveAttribute("data-stale", "false");
    await expect(page.locator("#rule-index li .rule-name")).toHaveText([...facts.keys()]);
    for (const [rule, fact] of facts) {
      await openFromIndex(page, rule);
      await expect(diagram(page), name).toHaveAttribute("aria-label", rule);
      await expectDiagram(page, fact);
      arrowless += fact.arrowheads.filter((a) => !a).length;
      diagrams++;
    }
  }
  // The case of a join gateway with one flow has flows without an arrowhead.
  expect(names).toContain("join-gateway--one-flow");
  expect(arrowless).toBeGreaterThan(0);
  expect(diagrams).toBeGreaterThan(names.length);
});

test("a reference box opens its rule, and the breadcrumbs go back", async ({ page }) => {
  await open(page);
  await page.locator("#example").selectOption("tutorial");
  await expectOpenPath(page, ["tutorial"]);
  await expect(page.locator("#open-rule-doc")).toHaveText("The whole tutorial.");

  // A click on the drill-down marker.
  await box(page, "reference", "lesson").locator(".drill-down").click();
  await expectOpenPath(page, ["tutorial", "lesson"]);
  await expect(page.locator("#open-rule-doc")).toHaveText(
    "One lesson teaches one concept, with one to three examples.",
  );

  // A double click on the box.
  await box(page, "reference", "practice").dblclick();
  await expectOpenPath(page, ["tutorial", "lesson", "practice"]);

  // A breadcrumb opens its diagram, with the shorter open path.
  await page.locator('#breadcrumbs li[data-rule="tutorial"] button').click();
  await expectOpenPath(page, ["tutorial"]);

  // Enter on a focused box.
  await box(page, "reference", "lesson").focus();
  await page.keyboard.press("Enter");
  await expectOpenPath(page, ["tutorial", "lesson"]);

  // A reference to a rule for one step has no drill-down marker, and it opens
  // in the same two ways.
  const example = box(page, "reference", "show_example");
  await expect(example).toHaveAttribute("data-one-step", "true");
  await expect(example.locator(".drill-down")).toHaveCount(0);
  await example.dblclick();
  await expectOpenPath(page, ["tutorial", "lesson", "show_example"]);
  await page.locator('#breadcrumbs li[data-rule="lesson"] button').click();
  await box(page, "reference", "introduce_concept").focus();
  await page.keyboard.press("Enter");
  await expectOpenPath(page, ["tutorial", "lesson", "introduce_concept"]);

  // The doc string of several lines shows its first line, and a click shows the rest.
  await page.locator('#breadcrumbs li[data-rule="tutorial"] button').click();
  await openFromIndex(page, "configure");
  await expectOpenPath(page, ["tutorial", "setup", "configure"]);
  const summary = page.locator("#open-rule-doc summary");
  await expect(summary).toHaveText("Tell the reader how to configure the tools.");
  const rest = page.locator("#open-rule-doc details p");
  await expect(rest).toBeHidden();
  await summary.click();
  await expect(rest).toHaveText("Leave this step out when the defaults work.");
});

test("a rule can occur twice in the open path", async ({ page }) => {
  await open(page);
  await page.locator("#example").selectOption("order-approval");
  await openFromIndex(page, "approval");
  await expectOpenPath(page, ["order", "outcome", "approval"]);
  // The self-embedding reference links no loop through rules, so it has no loop sign.
  const inner = box(page, "reference", "approval");
  await expect(inner).not.toHaveAttribute("data-loop-link");
  await expect(inner.locator(".loop-sign")).toHaveCount(0);
  await inner.locator(".drill-down").click();
  await expectOpenPath(page, ["order", "outcome", "approval", "approval"]);
  await page.locator("#breadcrumbs li").nth(1).locator("button").click();
  await expectOpenPath(page, ["order", "outcome"]);
});

test("a self tail call draws as a back edge where the spec gives it a lane", async ({ page }) => {
  await open(page);
  await page.locator("#example").selectOption("urg-moves");
  await openFromIndex(page, "refinement");
  await expectOpenPath(page, ["episode", "development", "refinement"]);
  const backEdge = diagram(page).locator('.flow[data-role="back-edge"]');
  await expect(backEdge).toHaveCount(1);
  await expect(backEdge.locator(".flow-label")).toHaveText("↻ refinement");
  await expect(box(page, "reference", "refinement")).toHaveCount(0);
  await expect(diagram(page).locator('.gateway[data-role="back-edge-join"]')).toHaveCount(1);

  // Without a straight route to a lane, the self tail call is a reference box
  // with the loop sign: the case of the design review rule-diagram-design-k37.
  await loadGrammarText(
    page,
    "no-lane.ebnf",
    '%start r ;\nr = "very_long_symbol" | "b" r? | "c" ;\n',
  );
  await expectOpenPath(page, ["r"]);
  await expect(diagram(page).locator(".flow[data-role='back-edge']")).toHaveCount(0);
  const call = box(page, "reference", "r");
  await expect(call).toHaveAttribute("data-loop-link", "tail-references");
  await expect(call.locator(".loop-sign")).toHaveText("↻");
  await call.dblclick();
  await expectOpenPath(page, ["r", "r"]);
});

test("a loop through two rules shows as linked diagrams", async ({ page }) => {
  await open(page);
  await loadGrammarText(
    page,
    "question.ebnf",
    '%start question ;\nquestion = "asks" answer ;\nanswer = "answers" question? ;\n',
  );
  await expectOpenPath(page, ["question"]);
  const answer = box(page, "reference", "answer");
  await expect(answer).toHaveAttribute("data-loop-link", "tail-references");
  await expect(answer.locator(".loop-sign")).toHaveText("↻");
  await answer.locator(".drill-down").click();
  await expectOpenPath(page, ["question", "answer"]);
  const question = box(page, "reference", "question");
  await expect(question).toHaveAttribute("data-loop-link", "tail-references");
  await expect(question.locator(".loop-sign")).toHaveText("↻");
  await question.locator(".drill-down").click();
  await expectOpenPath(page, ["question", "answer", "question"]);
});

test("a rule that the start rule does not reach opens alone", async ({ page }) => {
  await open(page);
  await loadGrammarText(
    page,
    "unreached.ebnf",
    '%start s ;\ns = "x" ;\nu = "y" t ;\nt = "z" "w" ;\n',
  );
  await expectOpenPath(page, ["s"]);
  await openFromIndex(page, "u");
  await expectOpenPath(page, ["u"]);
  await box(page, "reference", "t").locator(".drill-down").click();
  await expectOpenPath(page, ["u", "t"]);
  await openFromIndex(page, "t");
  await expectOpenPath(page, ["t"]);
});

test("the Tab key moves through the boxes in the order of the text", async ({ page }) => {
  await open(page);
  await page.locator("#example").selectOption("urg-moves");
  const facts = await exampleFacts("urg-moves");
  for (const rule of ["episode", "fix", "generalisation"]) {
    await openFromIndex(page, rule);
    const expected = facts.get(rule)?.boxes ?? [];
    expect(expected.length).toBeGreaterThan(1);
    await diagram(page).locator(".box").first().focus();
    const focused = (): Promise<string | undefined> =>
      page.evaluate(() => (document.activeElement as SVGElement | null)?.dataset.name);
    const order = [await focused()];
    for (let i = 1; i < expected.length; i++) {
      await page.keyboard.press("Tab");
      order.push(await focused());
    }
    expect(order).toEqual(expected);
  }
});

test("after an error, the last diagrams stay, dimmed, under a note", async ({ page }) => {
  await open(page);
  await page.locator("#example").selectOption("tutorial");
  await openFromIndex(page, "lesson");
  await expectOpenPath(page, ["tutorial", "lesson"]);
  const view = page.locator("#grammar-view");
  const note = page.locator("#grammar-view-note");
  await expect(view).toHaveAttribute("data-stale", "false");
  await expect(note).toBeHidden();

  const text = await readFile(join(examplesDir, "tutorial.ebnf"), "utf8");
  await loadGrammarText(page, "broken.ebnf", `${text}\nbroken = ;\n`);
  await expect(page.locator('#diagnostics li[data-severity="error"]')).not.toHaveCount(0);
  await expect(view).toHaveAttribute("data-stale", "true");
  await expect(note).toHaveText(
    "The grammar text has errors. The diagrams show the last text without errors.",
  );
  await expect(page.locator("#diagram-frame")).toHaveCSS("opacity", "0.45");
  await expectOpenPath(page, ["tutorial", "lesson"]);
  await expect(box(page, "reference", "practice")).toHaveCount(1);

  await loadGrammarText(page, "tutorial.ebnf", text);
  await expect(view).toHaveAttribute("data-stale", "false");
  await expect(note).toBeHidden();
  await expectOpenPath(page, ["tutorial", "lesson"]);
});

test("the diagram first fits the width of the view, at a scale of 1 at most", async ({ page }) => {
  await open(page);
  const symbols = Array.from({ length: 24 }, (_, i) => `"step_${i}"`).join(" ");
  await loadGrammarText(page, "wide.ebnf", `%start s ;\ns = ${symbols} ;\n`);
  const scale = async (): Promise<number> =>
    diagram(page)
      .locator(".viewport")
      .evaluate((g) => (g as SVGGElement).transform.baseVal.consolidate()?.matrix.a ?? 1);
  const width = Number(await diagram(page).getAttribute("data-width"));
  const frame = await page.locator("#diagram-frame").evaluate((f) => f.clientWidth);
  expect(width).toBeGreaterThan(frame);
  expect(width * (await scale())).toBeLessThanOrEqual(frame + 1);

  await page.locator("#zoom-in").click();
  expect(await scale()).toBeGreaterThan(frame / width);
  await page.locator("#zoom-fit").click();
  expect(width * (await scale())).toBeLessThanOrEqual(frame + 1);

  // A narrow diagram keeps a scale of 1.
  await page.locator("#example").selectOption("tutorial");
  await openFromIndex(page, "install");
  expect(await scale()).toBe(1);
});

test("the page draws the labels in the bundled font, 7.8 px for each cell", async ({ page }) => {
  await open(page);
  expect(await page.evaluate(() => document.fonts.check('13px "IBM Plex Mono"'))).toBe(true);
  const cdp = await page.context().newCDPSession(page);
  await cdp.send("DOM.enable");
  await cdp.send("CSS.enable");

  /** Checks each label of the open diagram, and returns how many it checked. */
  const checkLabels = async (): Promise<number> => {
    const labels = await diagram(page)
      .locator("text.label, text.flow-label")
      .evaluateAll((els) =>
        els.map((e) => ({
          text: e.textContent ?? "",
          width: (e as SVGTextElement).getComputedTextLength(),
        })),
      );
    for (const label of labels) {
      // The labels of these grammars hold only characters of one cell each.
      expect(Math.abs(label.width - [...label.text].length * cellWidth), label.text).toBeLessThan(
        1,
      );
    }
    // Chromium names the font that drew each label.
    const { root } = await cdp.send("DOM.getDocument", { depth: -1 });
    const { nodeIds } = await cdp.send("DOM.querySelectorAll", {
      nodeId: root.nodeId,
      selector: "#diagram-frame svg.diagram text.label, #diagram-frame svg.diagram text.flow-label",
    });
    expect(nodeIds.length).toBe(labels.length);
    for (const nodeId of nodeIds) {
      const { fonts } = await cdp.send("CSS.getPlatformFontsForNode", { nodeId });
      expect(fonts.length).toBeGreaterThan(0);
      for (const font of fonts) {
        expect(font.familyName).toBe("IBM Plex Mono");
        expect(font.isCustomFont).toBe(true);
      }
    }
    return labels.length;
  };

  // The URG grammar has labels of two lines, a loop marker and a back edge.
  await page.locator("#example").selectOption("urg-moves");
  let checked = 0;
  for (const rule of await checkedRules("urg-moves")) {
    await openFromIndex(page, rule);
    checked += await checkLabels();
  }
  expect(checked).toBeGreaterThan(20);

  // Spaces count as cells, so symbols that differ only by spaces get labels of
  // different widths.
  await loadGrammarText(page, "spaces.ebnf", '%start s ;\ns = "long_symbol   " "long_symbol" ;\n');
  await expect(diagram(page).locator("text.label")).toHaveCount(2);
  expect(
    await diagram(page)
      .locator("text.label")
      .evaluateAll((els) => els.map((e) => e.textContent)),
  ).toEqual(["long_symbol   ", "long_symbol"]);
  expect(await checkLabels()).toBe(2);
});
