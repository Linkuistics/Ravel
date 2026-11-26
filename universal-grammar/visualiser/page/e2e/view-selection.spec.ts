// The end-to-end tests of the selection across the views and of the data
// object: the section "Selection across the views" of
// docs/specs/output-view.md, and the section "The selection and the data
// object" of docs/specs/rule-diagrams.md. They check structure, not
// positions: the data attributes of the rows and of the diagram elements.
//
// The trace file `examples/tutorial--files.trace.kdl` holds 16 steps of the
// tutorial grammar, with two passes of `lesson+`.

import { join } from "node:path";
import { expect, type Locator, type Page, test } from "@playwright/test";

const examplesDir = join(import.meta.dirname, "..", "..", "examples");
const filesTrace = join(examplesDir, "tutorial--files.trace.kdl");

const diagram = (page: Page): Locator => page.locator("#diagram-frame svg.diagram");
const box = (page: Page, name: string): Locator =>
  diagram(page).locator(`.box[data-name="${name}"]`);
const selectedInDiagram = (page: Page): Locator => diagram(page).locator("[data-selected]");

/** The row of a step in the outline. */
const stepRow = (page: Page, step: number): Locator =>
  page.locator(`#outline [role=treeitem][data-step="${step}"]`);
const selectedRows = (page: Page): Locator =>
  page.locator('#outline [role=treeitem][aria-selected="true"]');

async function openFilesTrace(page: Page): Promise<void> {
  await page.goto("/");
  await expect(page.locator("body")).toHaveAttribute("data-ready", "true");
  await page.locator("#example").selectOption("tutorial");
  await page.locator("#trace-file").setInputFiles(filesTrace);
  await expect(page.getByTestId("acceptance")).toHaveText(
    "Accepted. The sequence can end after step 16.",
  );
}

async function loadGrammarText(page: Page, name: string, text: string): Promise<void> {
  await page.locator("#grammar-file").setInputFiles({
    name,
    mimeType: "text/plain",
    buffer: Buffer.from(text, "utf8"),
  });
}

/** Opens every folded row of the outline, so that each step has a row. */
async function openAllFolds(page: Page): Promise<void> {
  const folded = page.locator('#outline [role=treeitem][data-fold="folded"] .twisty');
  while ((await folded.count()) > 0) {
    await folded.first().click();
  }
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

/** The steps of the selected rows, in order. */
async function selectedSteps(page: Page): Promise<string[]> {
  return selectedRows(page).evaluateAll((els) =>
    els.map((e) => (e as HTMLElement).dataset.step ?? ""),
  );
}

test("a row selects its grammar elements and opens the diagram that holds them", async ({
  page,
}) => {
  await openFilesTrace(page);
  await openAllFolds(page);

  // A shared row stands for its rule node. The reference box of that node
  // lies in the diagram of practice, and the open path follows the
  // ancestors of the node.
  await stepRow(page, 8).click();
  await expect(stepRow(page, 8)).toHaveAttribute("aria-selected", "true");
  await expectOpenPath(page, ["tutorial", "lesson", "practice"]);
  await expect(box(page, "reference set_exercise")).toHaveAttribute("data-selected", "");
  await expect(selectedInDiagram(page)).toHaveCount(1);
  await expect(box(page, "reference set_exercise")).toHaveAttribute("aria-label", /, selected$/);

  // A rule row selects the reference box among the origins of its node.
  // The pass of `practice*` shares its row with the rule node.
  const practice = page.locator(
    '#outline [role=treeitem][data-kind="pass rule"][data-label$="practice"]',
  );
  await practice.click();
  await expectOpenPath(page, ["tutorial", "lesson"]);
  await expect(box(page, "reference practice")).toHaveAttribute("data-selected", "");
  await expect(selectedInDiagram(page)).toHaveCount(1);

  // The loop row of a repetition selects the repetition: its gateways and
  // its back flow.
  await page.locator('#outline [role=treeitem][data-kind="loop fold"][data-steps="5–14"]').click();
  await expectOpenPath(page, ["tutorial"]);
  await expect(diagram(page).locator(".gateway[data-selected]")).toHaveCount(2);
  await expect(diagram(page).locator('.flow[data-role="back"][data-selected]')).toHaveCount(1);
  await expect(diagram(page).locator(".selection")).toHaveCount(2);
  await expect(diagram(page).locator(".selection-line")).toHaveCount(1);

  // The step in a pass of a repetition: the pass shares its row with the
  // rule node of the one-step rule, so the row selects the reference box.
  await stepRow(page, 3).click();
  await expectOpenPath(page, ["tutorial", "setup"]);
  await expect(box(page, "reference install")).toHaveAttribute("data-selected", "");
});

test("an element selects each matching row, and a fold counts the selected rows inside it", async ({
  page,
}) => {
  await openFilesTrace(page);

  // The task box of a rule for one step matches the shared rows of that
  // rule: three steps, all inside the folded loop of lessons.
  await page.locator('#rule-index li[data-rule="show_example"] button').click();
  await expectOpenPath(page, ["tutorial", "lesson", "show_example"]);
  await box(page, "task show_example").click();
  await expect(box(page, "task show_example")).toHaveAttribute("data-selected", "");
  const lessons = page.locator(
    '#outline [role=treeitem][data-kind="loop fold"][data-steps="5–14"]',
  );
  await expect(lessons).toHaveAttribute("data-fold", "folded");
  await expect(lessons).toHaveAttribute("data-selected-inside", "3");
  await expect(lessons).toHaveAttribute(
    "data-label",
    "lesson+ · 2 passes · 10 steps · 5 files · 1 link · 3 selected",
  );
  // The fold row that holds the last matching row is the row in focus.
  await expect(page.locator('#details-body article[data-kind="loop"]')).toHaveCount(1);
  // A click in the grammar view keeps the open path.
  await expectOpenPath(page, ["tutorial", "lesson", "show_example"]);

  // With the folds open, the rows themselves show the selection. The last
  // matching row is the row in focus, and the view scrolls to it.
  await openAllFolds(page);
  expect(await selectedSteps(page)).toEqual(["6", "7", "14"]);
  await box(page, "task show_example").click();
  await expect(page.locator("#details-body article[data-step]")).toHaveAttribute("data-step", "14");
  await expect(stepRow(page, 14)).toBeInViewport();

  // A reference box matches the rows of the nodes that it made. A repetition
  // matches its loop rows.
  await page.locator('#rule-index li[data-rule="lesson"] button').click();
  await box(page, "reference show_example").click();
  expect(await selectedSteps(page)).toEqual(["6", "7", "14"]);
  await diagram(page).locator('.gateway[data-role="loop-join"]').first().click();
  await expect(selectedRows(page)).toHaveCount(2);
  expect(
    await selectedRows(page).evaluateAll((els) =>
      els.map((e) => (e as HTMLElement).dataset.kind ?? ""),
    ),
  ).toEqual(["loop fold", "loop fold"]);

  // An optional part selects nothing.
  await page.locator('#rule-index li[data-rule="tutorial"] button').click();
  await diagram(page).locator('.gateway[data-role="option-split"]').click();
  await expect(selectedRows(page)).toHaveCount(2);
});

test("a selection survives a new step and a switch of reading", async ({ page }) => {
  await page.goto("/");
  await expect(page.locator("body")).toHaveAttribute("data-ready", "true");
  await loadGrammarText(
    page,
    "twins.ebnf",
    '%start s ;\ns = a | b ;\na = "x" "y" ;\nb = "x" "y" ;\n',
  );
  await page.locator("#sequence").fill("x");
  await expect(page.locator("#reading-count")).toHaveText("Reading 1 of 2");

  // A selected step row survives a switch of reading: a step keeps its number.
  await stepRow(page, 1).click();
  await expectOpenPath(page, ["s", "a"]);
  await expect(box(page, "task x")).toHaveAttribute("data-selected", "");
  await page.locator("#reading-next").click();
  await expect(page.locator("#reading-count")).toHaveText("Reading 2 of 2");
  await expect(stepRow(page, 1)).toHaveAttribute("aria-selected", "true");
  // The switch is an operation on the trace, so the open path follows the
  // reading. The step of the new reading has its own task box.
  await expectOpenPath(page, ["s", "b"]);
  await expect(box(page, "task x")).toHaveAttribute("data-selected", "");

  // A selected step row survives a new step.
  await diagram(page).locator('.next-button[data-symbol="y"]').click();
  await expect(page.locator("#output-status")).toContainText("2 steps");
  await expect(stepRow(page, 1)).toHaveAttribute("aria-selected", "true");

  // A selected grammar element survives a switch of reading. Its matching
  // rows follow the tree on view.
  await box(page, "task y").click();
  expect(await selectedSteps(page)).toEqual(["2"]);
  await page.locator("#reading-previous").click();
  await expect(page.locator("#reading-count")).toHaveText("Reading 1 of 2");
  await expect(selectedRows(page)).toHaveCount(0);
  await page.locator("#reading-next").click();
  expect(await selectedSteps(page)).toEqual(["2"]);
});

test("the data object of the step in focus shows its attributes, and a click selects the step", async ({
  page,
}) => {
  await openFilesTrace(page);

  // With no selection, the step in focus is the last step read. Its data
  // object sits above the box that stands for the step in the open diagram.
  await expectOpenPath(page, ["tutorial"]);
  const data = diagram(page).locator(".data-object");
  await expect(data).toHaveCount(1);
  await expect(data).toHaveAttribute("data-step", "16");
  await expect(data).toHaveAttribute("data-count", "1");
  await expect(box(page, "reference suggest_next_steps").locator(".data-object")).toHaveCount(1);
  await expect(data.locator(".association")).toHaveCount(1);

  // A click on the data object selects the step.
  await data.click();
  await expect(stepRow(page, 16)).toHaveAttribute("aria-selected", "true");
  await expect(box(page, "reference suggest_next_steps")).toHaveAttribute("data-selected", "");

  // The data object follows the step in focus. In a diagram above the step,
  // it sits on the reference box that holds the step.
  await openAllFolds(page);
  await stepRow(page, 8).click();
  await expectOpenPath(page, ["tutorial", "lesson", "practice"]);
  await expect(box(page, "reference set_exercise").locator(".data-object")).toHaveAttribute(
    "data-count",
    "4",
  );
  await page.locator('#breadcrumbs li[data-rule="lesson"] button').click();
  await expect(box(page, "reference practice").locator(".data-object")).toHaveAttribute(
    "data-step",
    "8",
  );

  // A row in focus without a step leaves no step in focus, and no data object.
  await page
    .locator('#outline [role=treeitem][data-kind="pass rule"][data-label$="practice"]')
    .click();
  await expect(diagram(page).locator(".data-object")).toHaveCount(0);
});
