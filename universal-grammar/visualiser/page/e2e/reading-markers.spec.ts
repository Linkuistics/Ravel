// The end-to-end tests of the marks of a trace in the grammar view: the
// second test seam of the root brief, for the section "Marks of a trace" of
// docs/specs/rule-diagrams.md. Each sequence comes from clicks on ▸ buttons
// alone. The tests check the markers, the next-option marks and the pass
// labels by structure, not by position.

import { readFile } from "node:fs/promises";
import { join } from "node:path";
import { expect, type Locator, type Page, test } from "@playwright/test";

const examplesDir = join(import.meta.dirname, "..", "..", "examples");
const marksCasesDir = join(import.meta.dirname, "..", "..", "crates", "core", "tests", "marks");

const diagram = (page: Page): Locator => page.locator("#diagram-frame svg.diagram");
const box = (page: Page, name: string): Locator =>
  diagram(page).locator(`.box[data-name="${name}"]`);
const endEvent = (page: Page): Locator => diagram(page).locator('[data-event="end-event"]');

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

/** The open path, by the rules of the breadcrumbs. */
async function openPath(page: Page): Promise<string[]> {
  return page
    .locator("#breadcrumbs li")
    .evaluateAll((els) => els.map((e) => (e as HTMLElement).dataset.rule ?? ""));
}

async function expectOpenPath(page: Page, path: readonly string[]): Promise<void> {
  await expect.poll(() => openPath(page)).toEqual(path);
  await expect(diagram(page)).toHaveAttribute("aria-label", path[path.length - 1] ?? "");
}

/** The first accepted sequence of an example, from its file of sequences. */
async function acceptedSequence(name: string): Promise<string[]> {
  const text = await readFile(join(examplesDir, `${name}.sequences.kdl`), "utf8");
  const line = text.split("\n").find((l) => l.startsWith("accepted "));
  if (line === undefined) {
    throw new Error(`The sequences of ${name} hold no accepted sequence.`);
  }
  const tokens = line.slice("accepted ".length).match(/"(?:[^"\\]|\\.)*"|[^\s"]+/g) ?? [];
  return tokens
    .filter((t) => t.startsWith('"') || !t.includes("="))
    .map((t) => (t.startsWith('"') ? (JSON.parse(t) as string) : t));
}

/**
 * Appends a symbol by a click on a ▸ button. The button is in the open
 * diagram, in a popover of the open diagram, or in the header. When the open
 * diagram has none, the test goes one level up the open path.
 */
async function appendByClick(page: Page, symbol: string): Promise<void> {
  const value = JSON.stringify(symbol);
  for (;;) {
    const direct = diagram(page).locator(`.next-button[data-symbol=${value}]`);
    if ((await direct.count()) > 0) {
      await direct.first().click();
      return;
    }
    const menus = diagram(page).locator(".next-button:not([data-symbol])");
    const count = await menus.count();
    for (let i = 0; i < count; i++) {
      await menus.nth(i).click();
      await expect(page.locator("#option-popover")).toBeVisible();
      const option = page.locator(`#option-popover .popover-option[data-symbol=${value}]`);
      if ((await option.count()) > 0) {
        await option.click();
        return;
      }
      await page.locator("#open-rule").click();
      await expect(page.locator("#option-popover")).toBeHidden();
    }
    const unlisted = page.locator(`#unlisted-options .unlisted-option[data-symbol=${value}]`);
    if ((await unlisted.count()) > 0) {
      await unlisted.click();
      return;
    }
    const up = page.locator("#breadcrumbs li button.crumb");
    if ((await up.count()) === 0) {
      throw new Error(`No ▸ button appends ${value}.`);
    }
    await up.last().click();
  }
}

/** Builds a sequence by clicks on ▸ buttons alone, from an empty sequence. */
async function buildByClicks(page: Page, symbols: readonly string[]): Promise<void> {
  await expect(page.locator("#sequence")).toHaveValue("");
  for (const [i, symbol] of symbols.entries()) {
    await appendByClick(page, symbol);
    await expect(page.locator("#sequence")).toHaveValue(symbols.slice(0, i + 1).join("\n"));
    // After each operation on the trace, the open diagram is the diagram of
    // the follow path, so the filled tab of the reading on view stands in it.
    await expect(diagram(page).locator('.marker-tab[data-filled="true"]')).not.toHaveCount(0);
  }
}

/** The marks of the open diagram, by structure: each marked element with its marks. */
async function marksOf(page: Page): Promise<string[]> {
  return diagram(page)
    .locator("[data-marker], [data-option], [data-pass]")
    .evaluateAll((els) =>
      els.map((e) => {
        const d = (e as SVGElement).dataset;
        const name = d.name ?? d.event ?? `${d.role} ${d.start}-${d.end}`;
        return `${name} marker=${d.marker ?? ""} option=${d.option ?? ""} pass=${d.pass ?? ""}`;
      }),
    );
}

test("a sequence of each example comes from clicks on ▸ buttons alone", async ({ page }) => {
  await open(page);
  for (const name of ["order-approval", "tutorial", "urg-moves"]) {
    await page.locator("#example").selectOption(name);
    await page.locator("#sequence").fill("");
    const symbols = await acceptedSequence(name);
    expect(symbols.length).toBeGreaterThan(3);
    await buildByClicks(page, symbols);
    await expect(page.locator("#output-status")).toContainText("can end here");
  }
});

test("before the first step, the marker sits on the start event", async ({ page }) => {
  await open(page);
  await page.locator("#example").selectOption("tutorial");
  await page.locator("#sequence").fill("");
  await expectOpenPath(page, ["tutorial"]);
  const start = diagram(page).locator('[data-event="start-event"]');
  await expect(start).toHaveAttribute("data-marker", "1");
  await expect(start.locator('.marker-tab[data-filled="true"]')).toHaveText("1");
  await expect(start).toHaveAttribute("aria-label", "start event, marker of reading 1");
  const overview = box(page, "reference overview");
  await expect(overview).toHaveAttribute("data-option", "solid");
  await expect(overview).toHaveAttribute("aria-label", "reference overview, next option overview");
  await expect(overview.locator(".next-button")).toHaveAttribute("data-symbol", "overview");
});

test("the tutorial marks each level of the open path after overview install", async ({ page }) => {
  await open(page);
  await page.locator("#example").selectOption("tutorial");
  await page.locator("#sequence").fill("");
  await buildByClicks(page, ["overview", "install"]);
  // The follow path leaves out the last rule, which is for one step.
  await expectOpenPath(page, ["tutorial", "setup"]);
  const install = box(page, "reference install");
  await expect(install).toHaveAttribute("data-marker", "1");
  await expect(install).toHaveAttribute("data-option", "solid");
  await expect(box(page, "reference configure")).toHaveAttribute("data-option", "solid");
  // Another install starts pass 2, so the back flow carries the mark, and its
  // label shows the current pass.
  const back = diagram(page).locator('.flow[data-role="back"]');
  await expect(back).toHaveAttribute("data-option", "solid");
  await expect(back).toHaveAttribute("data-pass", "1");
  await expect(back.locator(".pass-label")).toHaveText("pass 1");
  // The end event carries introduce_concept, which lies after setup.
  await expect(endEvent(page)).toHaveAttribute("data-option", "solid");
  await expect(endEvent(page).locator(".next-button")).toHaveAttribute(
    "data-symbol",
    "introduce_concept",
  );

  // One level up, the marker sits on the reference box that holds the reading.
  await page.locator('#breadcrumbs li[data-rule="tutorial"] button').click();
  await expectOpenPath(page, ["tutorial"]);
  const setup = box(page, "reference setup");
  await expect(setup).toHaveAttribute("data-marker", "1");
  await expect(setup).toHaveAttribute(
    "aria-label",
    "reference setup, marker of reading 1, next options install and configure",
  );
  // Two next options pass through setup, so its ▸ button opens a popover.
  await setup.locator(".next-button").click();
  const popover = page.locator("#option-popover");
  await expect(popover).toBeVisible();
  await expect(popover.locator(".popover-option")).toHaveText(["▸ install", "▸ configure"]);
  await page.keyboard.press("Escape");
  await expect(popover).toBeHidden();
  await expect(box(page, "reference lesson")).toHaveAttribute(
    "aria-label",
    "reference lesson, next option introduce_concept",
  );
  // A click inside the popover appends the symbol, and both views update.
  await setup.locator(".next-button").click();
  await popover.locator('.popover-option[data-symbol="configure"]').click();
  await expect(page.locator("#sequence")).toHaveValue("overview\ninstall\nconfigure");
  await expectOpenPath(page, ["tutorial", "setup"]);
  await expect(box(page, "reference configure")).toHaveAttribute("data-marker", "1");
  await expect(page.locator("#outline")).toContainText("configure");
});

test("an ambiguous grammar shows one marker for each reading", async ({ page }) => {
  await open(page);
  await page.locator("#example").selectOption("order-approval");
  await page.locator("#sequence").fill("");
  await buildByClicks(page, ["submit_order", "check_order", "remind"]);
  await expect(page.locator("#reading-count")).toHaveText("Reading 1 of 2");
  await expectOpenPath(page, ["order", "outcome", "approval"]);
  await expect(page.locator("#readings-outside")).toHaveText(
    "Reading 2 stands outside this diagram.",
  );
  const remind = box(page, "task remind");
  await expect(remind).toHaveAttribute("data-marker", "1");
  await expect(remind.locator(".marker-tab")).toHaveCount(1);
  for (const name of ["task remind", "task approve", "task escalate"]) {
    await expect(box(page, name)).toHaveAttribute("data-option", "solid");
  }
  await expect(diagram(page).locator('.flow[data-role="back"] .pass-label')).toHaveText(
    "pass 1 · ↻ 0–2",
  );

  // One level up, each reading has its own marker. The tab of the reading on
  // view is filled, and the next options of the other reading are dashed.
  await page.locator('#breadcrumbs li[data-rule="outcome"] button').click();
  await expect(page.locator("#readings-outside")).toBeHidden();
  const approval = box(page, "reference approval");
  const rejection = box(page, "reference rejection");
  await expect(approval).toHaveAttribute("data-marker", "1");
  await expect(approval.locator(".marker-tab")).toHaveAttribute("data-filled", "true");
  await expect(approval).toHaveAttribute("data-option", "solid");
  await expect(rejection).toHaveAttribute("data-marker", "2");
  await expect(rejection.locator(".marker-tab")).toHaveAttribute("data-filled", "false");
  await expect(rejection).toHaveAttribute("data-option", "dashed");
  await expect(rejection.locator(".option-mark")).toHaveAttribute("data-style", "dashed");
  await expect(rejection).toHaveAttribute(
    "aria-label",
    "reference rejection, marker of reading 2, next options of other readings remind, escalate and reject",
  );

  // A switch of reading is an operation on the trace: the open path follows
  // reading 2, and its tab becomes the filled one.
  await page.locator("#reading-next").click();
  await expect(page.locator("#reading-count")).toHaveText("Reading 2 of 2");
  await expectOpenPath(page, ["order", "outcome", "rejection"]);
  await expect(box(page, "task remind")).toHaveAttribute("data-marker", "2");
  await expect(box(page, "task remind").locator('.marker-tab[data-reading="2"]')).toHaveAttribute(
    "data-filled",
    "true",
  );
  await expect(page.locator("#readings-outside")).toHaveText(
    "Reading 1 stands outside this diagram.",
  );
});

test("after three tabs, a tab +n counts the rest, and the header lists unlisted options", async ({
  page,
}) => {
  await open(page);
  // The grammar of the marks case "unlisted" of the core: after six steps a,
  // the cap lists ten readings, and only an unlisted reading allows b.
  const grammar = await readFile(join(marksCasesDir, "unlisted.ebnf"), "utf8");
  await loadGrammarText(page, "unlisted.ebnf", grammar);
  await buildByClicks(page, ["a", "a", "a", "a", "a", "a"]);
  await expect(page.locator("#reading-count")).toHaveText("Reading 1 of 10+");
  await expectOpenPath(page, ["s", "x"]);
  await page.locator('#breadcrumbs li[data-rule="s"] button').click();
  const x = box(page, "reference x");
  await expect(x).toHaveAttribute("data-marker", "1 2 3 4 5 6 7 8 9 10");
  await expect(x.locator(".marker-tab")).toHaveText(["1", "2", "3", "+7"]);
  await expect(x.locator('.marker-tab[data-filled="true"]')).toHaveText("1");
  await expect(x.locator(".marker-more")).toHaveText("+7");
  // The next option b belongs to no listed reading, so no element carries its
  // mark, and the header lists it with a ▸ button.
  const unlisted = page.locator("#unlisted-options .unlisted-option");
  await expect(unlisted).toHaveText(["▸ b"]);
  await unlisted.click();
  await expect(page.locator("#sequence")).toHaveValue("a\na\na\na\na\na\nb");
  await expect(page.locator("#output-status")).toContainText("can end here");
  await expectOpenPath(page, ["s", "y"]);
  await expect(page.locator("#unlisted-options")).toBeHidden();
});

test("a rejected step leaves the marks of the state before it", async ({ page }) => {
  await open(page);
  await page.locator("#example").selectOption("order-approval");
  await page.locator("#sequence").fill("");
  await buildByClicks(page, ["submit_order", "check_order", "escalate", "approve"]);
  const path = await openPath(page);
  expect(path).toEqual(["order", "outcome", "approval", "approval"]);
  const before = await marksOf(page);
  expect(before).toContain("end-event marker= option=solid pass=");
  await expect(page.locator("#grammar-view-banner")).toBeHidden();

  // The step ship comes from the sequence input, because no ▸ button offers it.
  await page.locator("#sequence").fill("submit_order\ncheck_order\nescalate\napprove\nship");
  await expect(page.locator("#output-status")).toContainText("rejected at step 5");
  await expect(page.locator("#grammar-view-banner")).toHaveText(
    'Step 5 "ship" is rejected. The grammar uses "ship" in outcome, but not at this point. Showing the state after step 4. Next: hand_back.',
  );
  await expectOpenPath(page, path);
  expect(await marksOf(page)).toEqual(before);

  // One level up, the marker sits on the inner reference box, and hand_back
  // carries the next-option mark.
  await page.locator("#breadcrumbs li").nth(2).locator("button").click();
  await expect(box(page, "reference approval")).toHaveAttribute("data-marker", "1");
  await expect(box(page, "task hand_back")).toHaveAttribute("data-option", "solid");

  // The ▸ button inserts hand_back before the rejected step, and reads it.
  await box(page, "task hand_back").locator(".next-button").click();
  await expect(page.locator("#sequence")).toHaveValue(
    "submit_order\ncheck_order\nescalate\napprove\nhand_back\nship",
  );
  await expect(page.locator("#grammar-view-banner")).toBeHidden();
  await expectOpenPath(page, ["order", "outcome", "approval"]);
});

test("a back edge and a breadcrumb show the current pass", async ({ page }) => {
  await open(page);
  // The URG grammar: pass 2 of the self tail call of refinement.
  await page.locator("#example").selectOption("urg-moves");
  await page.locator("#sequence").fill("");
  await buildByClicks(page, [
    "states the problem",
    "proposes a solution",
    "identifies a gap",
    "revises the solution",
    "identifies a gap",
    "revises the solution",
  ]);
  await expectOpenPath(page, ["episode", "development", "refinement"]);
  const backEdge = diagram(page).locator('.flow[data-role="back-edge"]');
  await expect(backEdge).toHaveAttribute("data-pass", "2");
  await expect(backEdge.locator(".pass-label")).toHaveText("↻ refinement · pass 2");
  // identifies a gap would start pass 3, so its mark covers the back edge and the box.
  await expect(backEdge).toHaveAttribute("data-option", "solid");
  await expect(box(page, "task identifies a gap")).toHaveAttribute("data-option", "solid");
  await expect(box(page, "task revises the solution")).toHaveAttribute("data-marker", "1");

  // A loop with a back edge and a link in another rule: the case
  // "mixed-loop" of the core. The diagram of b shows the pass in the
  // breadcrumb of the pass rule a.
  const grammar = await readFile(join(marksCasesDir, "mixed-loop.ebnf"), "utf8");
  await loadGrammarText(page, "mixed-loop.ebnf", grammar);
  await page.locator("#sequence").fill("");
  await buildByClicks(page, ["x", "y"]);
  await expectOpenPath(page, ["a", "b"]);
  const crumb = page.locator('#breadcrumbs li[data-rule="a"]');
  await expect(crumb).toHaveAttribute("data-pass", "2");
  await expect(crumb.locator(".crumb-pass")).toHaveText(" ↻ 2");
  await expect(crumb).toHaveText("a ↻ 2");
  await crumb.locator("button").click();
  await expectOpenPath(page, ["a"]);
  await expect(page.locator('#breadcrumbs li[data-rule="a"]')).not.toHaveAttribute("data-pass");
  await expect(diagram(page).locator('.flow[data-role="back-edge"] .pass-label')).toHaveText(
    "↻ a · pass 2",
  );
});

test("a change of the grammar text keeps the open path, and an edit of the sequence follows the reading", async ({
  page,
}) => {
  await open(page);
  await page.locator("#example").selectOption("tutorial");
  await page.locator("#sequence").fill("");
  await page.locator('#rule-index li[data-rule="practice"] button').click();
  await expectOpenPath(page, ["tutorial", "lesson", "practice"]);
  // The tutorial again, with one more line: not an operation on the trace.
  const text = await readFile(join(examplesDir, "tutorial.ebnf"), "utf8");
  await loadGrammarText(page, "tutorial.ebnf", `${text}\n`);
  await expectOpenPath(page, ["tutorial", "lesson", "practice"]);
  // An edit of the sequence is an operation on the trace.
  await page.locator("#sequence").fill("overview\ninstall\nintroduce_concept");
  await expectOpenPath(page, ["tutorial", "lesson"]);
  await expect(box(page, "reference introduce_concept")).toHaveAttribute("data-marker", "1");
  // A step back is an operation on the trace too.
  await page.locator("#step-back").click();
  await expectOpenPath(page, ["tutorial", "setup"]);
});
