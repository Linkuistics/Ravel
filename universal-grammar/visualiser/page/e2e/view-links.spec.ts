// The end-to-end tests of the step attributes, the file references and the
// saved trace file: the sections "Step attributes" and "File references" of
// docs/specs/output-view.md. They check structure, not positions.
//
// The trace file `examples/tutorial--files.trace.kdl` holds a web address,
// relative file references and references that the page never opens. Its
// folder `examples/tutorial-files` holds some of the files.

import { readFile, writeFile } from "node:fs/promises";
import { join } from "node:path";
import { type BrowserContext, expect, type Page, test } from "@playwright/test";

const examplesDir = join(import.meta.dirname, "..", "..", "examples");
const filesTrace = join(examplesDir, "tutorial--files.trace.kdl");
// The trace file that the page saves after an insert. The core seam reads it
// as a trace fixture. Set UPDATE_EXPECT=1 to write it again from the page.
const pageSaved = join(examplesDir, "tutorial--files-page-saved.trace.kdl");

async function openFilesTrace(page: Page): Promise<void> {
  await page.goto("/");
  await expect(page.locator("body")).toHaveAttribute("data-ready", "true");
  await page.locator("#example").selectOption("tutorial");
  await page.locator("#trace-file").setInputFiles(filesTrace);
  await expect(page.getByTestId("acceptance")).toHaveText(
    "Accepted. The sequence can end after step 16.",
  );
}

/** The row of a step in the outline. */
const stepRow = (page: Page, step: number) =>
  page.locator(`#outline [role=treeitem][data-step="${step}"]`);

/** Opens every folded row of the outline, so that each step has a row. */
async function openAllFolds(page: Page): Promise<void> {
  const folded = page.locator('#outline [role=treeitem][data-fold="folded"] .twisty');
  while ((await folded.count()) > 0) {
    await folded.first().click();
  }
}

/** The attribute chips of a row, as their texts. */
const chipTexts = (page: Page, step: number) =>
  stepRow(page, step).locator(".attribute").allTextContents();

/** The entry of an attribute in the details panel. */
const detail = (page: Page, key: string) =>
  page.locator(`#details-body .attribute-item[data-key="${key}"]`);

/** Answers each request to example.org, so a test needs no network. */
async function serveExampleOrg(context: BrowserContext): Promise<void> {
  await context.route("https://example.org/**", (route) =>
    route.fulfill({ contentType: "text/plain", body: `served ${route.request().url()}` }),
  );
}

test("step attributes show as chips, as counts in folds and in the details panel", async ({
  page,
}) => {
  await openFilesTrace(page);

  // A finished loop folds into one row, which counts its relative file
  // references as files and its web addresses as links.
  const lessons = page.locator(
    '#outline [role=treeitem][data-kind="loop fold"][data-steps="5–14"]',
  );
  await expect(lessons).toHaveAttribute(
    "data-label",
    "lesson+ · 2 passes · 10 steps · 5 files · 1 link",
  );
  await expect(
    page.locator('#outline [role=treeitem][data-kind="loop fold"][data-steps="2–3"]'),
  ).toHaveAttribute("data-label", "install+ · 2 passes · 2 steps · 2 files");

  // The last step shows its web address as a chip with the host.
  expect(await chipTexts(page, 16)).toEqual(["example.org"]);
  await expect(stepRow(page, 16).locator(".attribute")).toHaveAttribute("data-reference", "web");

  // With no selection, the details panel shows the last step read.
  await expect(page.locator("#details-body article[data-step]")).toHaveAttribute("data-step", "16");
  await expect(detail(page, "more")).toHaveAttribute("data-state", "link");

  // References come first, ordered by key, then the other attributes. After
  // three chips, a last chip counts the rest.
  await openAllFolds(page);
  expect(await chipTexts(page, 8)).toEqual(["exercise.md", "solution.md", "missing.rs", "+1"]);
  expect(await chipTexts(page, 6)).toEqual(["example 1.rs", "level=2"]);
  expect(await chipTexts(page, 4)).toEqual([]);

  // A click selects a row, and the details panel shows every attribute of its step.
  await stepRow(page, 8).click();
  await expect(stepRow(page, 8)).toHaveAttribute("aria-selected", "true");
  await expect(page.locator("#details-body article[data-step]")).toHaveAttribute("data-step", "8");
  await expect(page.locator("#details-body .attribute-item dt")).toHaveText([
    "file",
    "starter",
    "solution",
    "minutes",
  ]);
  await expect(detail(page, "file").locator("dd")).toHaveText(
    "(url-reference) ./tutorial-files/exercise.mdopen its folder to open it",
  );
  await expect(detail(page, "minutes")).toHaveAttribute("data-reference", "attribute");
  await expect(detail(page, "minutes").locator("dd")).toHaveText("15");

  // A shared row also shows the doc string of its rule for one step.
  await expect(page.locator('#details-body article[data-rule="set_exercise"] .doc')).toHaveText(
    "Set an exercise.\nAttribute `file`: the relative path of the starter file.",
  );

  // A reference with another scheme, or a path from the root, stays text.
  await stepRow(page, 11).click();
  await expect(detail(page, "notes")).toHaveAttribute("data-state", "text");
  await expect(detail(page, "mail")).toHaveAttribute("data-state", "text");
  await expect(page.locator("#details-body a")).toHaveCount(0);

  // A loop row shows its repetition, its bound and its count of passes.
  await lessons.click();
  await expect(page.locator('#details-body article[data-kind="loop"] .repetition')).toHaveText(
    "lesson+",
  );
  await expect(page.locator('#details-body article[data-kind="loop"] .detail-note')).toHaveText(
    "2 passes",
  );
});

test("a web address opens directly in a new tab, without access to the page", async ({
  page,
  context,
}) => {
  await serveExampleOrg(context);
  await openFilesTrace(page);
  const link = detail(page, "more").locator("a");
  await expect(link).toHaveAttribute("href", "https://example.org/tutorial/next");
  await expect(link).toHaveAttribute("target", "_blank");
  await expect(link).toHaveAttribute("rel", "noopener noreferrer");

  const opening = context.waitForEvent("page");
  await link.click();
  const tab = await opening;
  await tab.waitForLoadState();
  expect(tab.url()).toBe("https://example.org/tutorial/next");
  expect(await tab.evaluate(() => window.opener)).toBeNull();
});

test("a relative file reference opens only after the person opens its folder", async ({
  page,
  context,
}) => {
  await openFilesTrace(page);
  await openAllFolds(page);
  await stepRow(page, 8).click();

  // Before any folder is open, the reference shows its path, and the panel
  // offers the folder picker.
  await expect(detail(page, "file")).toHaveAttribute("data-state", "no-folder");
  await expect(detail(page, "file").locator("a")).toHaveCount(0);
  await expect(page.locator("#folder")).toBeVisible();
  await expect(page.locator("#folder-status")).toHaveText(
    "No folder is open. Open the folder that holds the trace file to open its files.",
  );

  // The person opens the folder that holds the trace file.
  await page.locator("#reference-folder").setInputFiles(examplesDir);
  await expect(page.locator("#folder-status")).toHaveText(/^Folder examples: \d+ files\.$/);
  await expect(detail(page, "file")).toHaveAttribute("data-state", "found");
  await expect(detail(page, "starter")).toHaveAttribute("data-state", "not-found");
  await expect(detail(page, "starter").locator(".status")).toHaveText("not found in the folder");
  await expect(detail(page, "solution")).toHaveAttribute("data-state", "outside");
  await expect(detail(page, "solution").locator(".status")).toHaveText("outside the folder");

  // A click opens the file in a new tab, from the file that the picker gave.
  const opening = context.waitForEvent("page");
  await detail(page, "file").locator("a").click();
  const tab = await opening;
  await tab.waitForLoadState();
  expect(tab.url()).toMatch(/^blob:/);
  expect((await tab.locator("body").innerText()).trimEnd()).toBe(
    (await readFile(join(examplesDir, "tutorial-files", "exercise.md"), "utf8")).trimEnd(),
  );

  // The page decodes a percent escape, and it ignores a fragment.
  await stepRow(page, 6).click();
  await expect(detail(page, "code")).toHaveAttribute("data-state", "found");
  await stepRow(page, 3).click();
  await expect(detail(page, "guide")).toHaveAttribute("data-state", "found");
});

test("a trace file that the page saves after an insert keeps each attribute", async ({ page }) => {
  await openFilesTrace(page);
  // Back to the state after step 14, then a next option inserts a new step 15.
  await page.locator("#step-back").click();
  await page.locator("#step-back").click();
  await page.locator('#outline .chip[data-symbol="show_example"]').click();
  await expect(page.locator("#sequence")).toHaveValue(
    [
      "overview",
      "install",
      "install",
      "configure",
      "introduce_concept",
      "show_example",
      "show_example",
      "set_exercise",
      "give_hint",
      "give_hint",
      "show_solution",
      "check_understanding",
      "introduce_concept",
      "show_example",
      "show_example",
      "summarise",
      "suggest_next_steps",
    ].join("\n"),
  );

  const downloading = page.waitForEvent("download");
  await page.locator("#save").click();
  const download = await downloading;
  expect(download.suggestedFilename()).toBe("tutorial--files.trace.kdl");
  const saved = await readFile(await download.path(), "utf8");
  if (process.env.UPDATE_EXPECT === "1") {
    await writeFile(pageSaved, saved);
  }
  // The core seam reads this file as a trace fixture, and Save must give the
  // same text: `trace_fixtures` in crates/core/tests/trace_interface.rs.
  expect(saved).toBe(await readFile(pageSaved, "utf8"));
  expect(saved).toContain(
    '        step suggest_next_steps more=(url)"https://example.org/tutorial/next"\n',
  );
  expect(saved).toContain("        step show_example\n        step summarise\n");
});
