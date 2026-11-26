// The end-to-end tests of the workbench page: the second test seam of the
// root brief. They drive the built page with the real core in WebAssembly,
// and they check structure, not positions.

import { readFile } from "node:fs/promises";
import { join } from "node:path";
import { expect, type Page, test } from "@playwright/test";

const examplesDir = join(import.meta.dirname, "..", "..", "examples");
const exampleNames = ["order-approval", "tutorial", "urg-moves"];
// A test case of the core: a trace file whose steps have attributes of each type.
const attributesDir = join(
  import.meta.dirname,
  "..",
  "..",
  "crates",
  "core",
  "tests",
  "traces",
  "attributes",
);

/** The rule names of an example, from the checked grammar that the core wrote. */
async function checkedRules(name: string): Promise<string[]> {
  const text = await readFile(join(examplesDir, `${name}.checked.kdl`), "utf8");
  return [...text.matchAll(/^ {4}rule name=(\S+)/gm)].map((m) => m[1] ?? "");
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

test("each example grammar loads without diagnostics, with its rules and badges", async ({
  page,
}) => {
  await open(page);
  expect(await page.locator("#example option").allTextContents()).toEqual([
    "(none)",
    ...exampleNames,
  ]);
  // The tutorial grammar has no recursive group. The other two examples have
  // groups, so the parser of the expected badges must find some.
  let badgeCount = 0;
  for (const name of exampleNames) {
    await page.locator("#example").selectOption(name);
    await expect(page.locator("#rule-index li .rule-name")).toHaveText(await checkedRules(name));
    await expect(page.locator("#diagnostics li")).toHaveCount(0);
    await expect(page.locator(".cm-lintRange")).toHaveCount(0);
    const badges = await checkedBadges(name);
    badgeCount += badges.size;
    await expect(page.locator("#rule-index .badge")).toHaveCount(badges.size);
    for (const [rule, badge] of badges) {
      await expect(page.locator(`#rule-index li[data-rule="${rule}"] .badge`)).toHaveAttribute(
        "data-badge",
        badge,
      );
    }
  }
  expect(badgeCount).toBeGreaterThan(0);
});

test("a grammar with an undefined rule shows the diagnostic at its position", async ({ page }) => {
  await open(page);
  // The symbol before the reference holds characters of two and three bytes
  // in UTF-8, so a byte offset differs from the UTF-16 index of the editor.
  await loadGrammarText(page, "undefined.ebnf", '%start s ;\ns = "é日本" missing ;\n');
  const diagnostic = page.locator('#diagnostics li[data-code="undefined-rule"]');
  await expect(diagnostic).toHaveCount(1);
  await expect(diagnostic.locator(".place")).toHaveText("2:11");
  await expect(diagnostic).toHaveAttribute("data-severity", "error");
  await expect(page.locator(".cm-lintRange-error")).toHaveText("missing");
  await expect(page.getByTestId("acceptance")).toHaveText(
    "The grammar has errors, so it has no traces.",
  );
  await expect(page.locator("#save")).toBeDisabled();
});

test("a typed sequence shows acceptance, then the failure point", async ({ page }) => {
  await open(page);
  await page.locator("#example").selectOption("tutorial");
  const sequence = page.locator("#sequence");

  await sequence.fill("overview\ninstall\nintroduce_concept\n");
  await expect(page.getByTestId("acceptance")).toHaveText(
    "Accepted so far. The sequence cannot end after step 3.",
  );
  await expect(page.getByTestId("readings")).toHaveText("1 reading.");
  await expect(page.getByTestId("next-options")).toHaveText('Next options: "show_example".');
  await expect(page.locator("#steps li")).toHaveText(["overview", "install", "introduce_concept"]);
  await expect(page.locator('#steps li[data-state="read"]')).toHaveCount(3);

  await sequence.fill("overview\nbogus\ninstall\n");
  await expect(page.getByTestId("acceptance")).toHaveText(
    'Rejected at step 2: no reading accepts "bogus". The state on view is the state after step 1.',
  );
  await expect(page.locator('#steps li[data-state="read"]')).toHaveText(["overview"]);
  await expect(page.locator('#steps li[data-state="rejected"]')).toHaveText(["bogus"]);
  await expect(page.locator('#steps li[data-state="unread"]')).toHaveText(["install"]);
  await expect(page.getByTestId("next-options")).toHaveText('Next options: "install".');
});

test("a trace file of an example loads, shows its result, and saves the same text", async ({
  page,
}) => {
  await open(page);
  await page.locator("#example").selectOption("tutorial");

  const accepted = join(examplesDir, "tutorial--accepted-1.trace.kdl");
  await page.locator("#trace-file").setInputFiles(accepted);
  await expect(page.locator("#source")).toHaveText(
    "The sequence comes from tutorial--accepted-1.trace.kdl.",
  );
  await expect(page.getByTestId("acceptance")).toHaveText(
    /^Accepted\. The sequence can end after step \d+\.$/,
  );
  await expect(page.locator("#trace-messages li")).toHaveCount(0);

  const downloading = page.waitForEvent("download");
  await page.locator("#save").click();
  const download = await downloading;
  expect(download.suggestedFilename()).toBe("tutorial--accepted-1.trace.kdl");
  const saved = await readFile(await download.path(), "utf8");
  expect(saved).toBe(await readFile(accepted, "utf8"));

  await page
    .locator("#trace-file")
    .setInputFiles(join(examplesDir, "tutorial--rejected-1.trace.kdl"));
  await expect(page.getByTestId("acceptance")).toHaveText(
    'Rejected at step 2: no reading accepts "introduce_concept". The state on view is the state after step 1.',
  );
  await expect(page.locator("#sequence")).toHaveValue(
    "overview\nintroduce_concept\nshow_example\nsummarise",
  );
});

test("a trace file with an error shows its diagnostic", async ({ page }) => {
  await open(page);
  await page.locator("#example").selectOption("tutorial");
  await page.locator("#trace-file").setInputFiles({
    name: "broken.trace.kdl",
    mimeType: "text/plain",
    buffer: Buffer.from(
      "trace format-version=1 {\n    sequence {\n        step\n    }\n}\n",
      "utf8",
    ),
  });
  await expect(page.getByTestId("acceptance")).toHaveText("The trace file has errors.");
  await expect(page.locator('#trace-messages li[data-code="step-argument"]')).toHaveCount(1);
  await expect(page.locator("#save")).toBeDisabled();
});

test("a typed symbol keeps the spaces at its ends", async ({ page }) => {
  await open(page);
  await loadGrammarText(page, "spaces.ebnf", '%start s ;\ns = " x " " " ;\n');
  const sequence = page.locator("#sequence");

  await sequence.fill(" x \n \n");
  await expect(page.getByTestId("acceptance")).toHaveText(
    "Accepted. The sequence can end after step 2.",
  );
  expect(await page.locator("#steps li").allTextContents()).toEqual([" x ", " "]);

  await sequence.fill("x\n");
  await expect(page.getByTestId("acceptance")).toHaveText(
    'Rejected at step 1: no reading accepts "x". The state on view is the state after step 0.',
  );
});

test("an edit of a loaded trace file keeps the attributes of the unchanged steps", async ({
  page,
}) => {
  await open(page);
  const grammar = await readFile(join(attributesDir, "attributes.ebnf"), "utf8");
  await loadGrammarText(page, "attributes.ebnf", grammar);
  const traceFile = join(attributesDir, "attributes--each-type.trace.kdl");
  const original = await readFile(traceFile, "utf8");
  await page.locator("#trace-file").setInputFiles(traceFile);
  const sequence = page.locator("#sequence");
  await expect(sequence).toHaveValue("open\nset exercise\nclose");

  const save = async (): Promise<string> => {
    const downloading = page.waitForEvent("download");
    await page.locator("#save").click();
    const download = await downloading;
    expect(download.suggestedFilename()).toBe("attributes--each-type.trace.kdl");
    return readFile(await download.path(), "utf8");
  };
  const stepWithFile = 'step "set exercise" file=(url-reference)"starter/main.rs"';
  const stepOpen = "step open title=Recursion symbol=close";
  const stepClose = 'step close "unicode key é"=日本 tab="a\\tb"';

  // A line break changes no symbol, so Save writes the file unchanged.
  await sequence.fill("open\nset exercise\nclose\n");
  await expect(page.locator("#source")).toHaveText(
    "The sequence comes from attributes--each-type.trace.kdl, with the edits in the input above.",
  );
  await expect(page.locator("#trace-messages li")).toHaveCount(0);
  expect(await save()).toBe(original);

  // A new step between unchanged steps leaves their attributes in place.
  await sequence.fill("open\nset exercise\nset exercise\nclose\n");
  await expect(page.getByTestId("acceptance")).toHaveText(
    "Accepted. The sequence can end after step 4.",
  );
  await expect(page.locator("#trace-messages li")).toHaveCount(0);
  const inserted = await save();
  expect(inserted).toContain(stepOpen);
  expect(inserted).toContain(stepWithFile);
  expect(inserted).toContain('        step "set exercise"\n');
  expect(inserted).toContain(stepClose);

  // A removed step takes its attributes with it, and the page says so.
  await sequence.fill("open\nclose\n");
  await expect(page.getByTestId("acceptance")).toHaveText(
    "Accepted. The sequence can end after step 2.",
  );
  const dropped = page.locator('#trace-messages li[data-code="attributes-dropped"]');
  await expect(dropped).toHaveCount(1);
  await expect(dropped).toHaveAttribute("data-severity", "warning");
  await expect(dropped).toContainText("1 step");
  const removed = await save();
  expect(removed).toContain(stepOpen);
  expect(removed).not.toContain("starter/main.rs");
  expect(removed).toContain(stepClose);
});

test("an edit after a next option keeps each attribute on its own step", async ({ page }) => {
  await open(page);
  const grammar = await readFile(join(attributesDir, "attributes.ebnf"), "utf8");
  await loadGrammarText(page, "attributes.ebnf", grammar);
  await page
    .locator("#trace-file")
    .setInputFiles(join(attributesDir, "attributes--each-type.trace.kdl"));
  const sequence = page.locator("#sequence");
  await expect(sequence).toHaveValue("open\nset exercise\nclose");

  // A next option after step 1 inserts a new step 2 without attributes.
  await page.locator("#step-back").click();
  await page.locator("#step-back").click();
  await page.locator('#outline .chip[data-symbol="set exercise"]').click();
  await expect(sequence).toHaveValue("open\nset exercise\nset exercise\nclose");

  // A line break changes no symbol, so each step keeps its attributes.
  await sequence.fill("open\nset exercise\nset exercise\nclose\n");
  await expect(page.getByTestId("acceptance")).toHaveText(
    "Accepted. The sequence can end after step 4.",
  );
  const downloading = page.waitForEvent("download");
  await page.locator("#save").click();
  const saved = await readFile(await (await downloading).path(), "utf8");
  const steps = [...saved.matchAll(/^ {8}step .*$/gm)].map((m) => m[0].trim());
  expect(steps).toEqual([
    "step open title=Recursion symbol=close",
    'step "set exercise"',
    'step "set exercise" file=(url-reference)"starter/main.rs" link=(url)"https://example.org/a?b=1" count=16 weight=2.5 small=(u8)7 done=#false note=#null',
    'step close "unicode key é"=日本 tab="a\\tb"',
  ]);
});
