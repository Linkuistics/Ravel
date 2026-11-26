// The end-to-end tests of the output view: the section "Test seams" of
// docs/specs/output-view.md. They read the ARIA tree of the outline, and they
// check structure, not positions.

import { expect, type Page, test } from "@playwright/test";

async function open(page: Page, example: string): Promise<void> {
  await page.goto("/");
  await expect(page.locator("body")).toHaveAttribute("data-ready", "true");
  await page.locator("#example").selectOption(example);
}

async function loadGrammarText(page: Page, name: string, text: string): Promise<void> {
  await page.locator("#grammar-file").setInputFiles({
    name,
    mimeType: "text/plain",
    buffer: Buffer.from(text, "utf8"),
  });
}

/**
 * The rows of the outline as text: the indent of the ARIA level, the kinds,
 * the step range, the label, the right side and the fold state. The label is
 * the text that the row draws, not its data attribute.
 */
function rows(page: Page): Promise<string[]> {
  return page.locator("#outline [role=treeitem]").evaluateAll((items) =>
    items.map((item) => {
      const row = item as HTMLElement;
      const level = Number(row.getAttribute("aria-level"));
      const steps = row.dataset.steps === undefined ? "" : ` [${row.dataset.steps}]`;
      const label = [...row.querySelectorAll(".label .part")].map((p) => p.textContent).join(" ");
      const right = row.querySelector(".right")?.textContent ?? "";
      const fold = row.dataset.fold === undefined ? "" : ` (${row.dataset.fold})`;
      return `${"  ".repeat(level - 1)}${row.dataset.kind}${steps}: ${label}${right === "" ? "" : ` | ${right}`}${fold}`;
    }),
  );
}

/** The chips of the next options, each with whether the reading on view allows it. */
function chips(page: Page): Promise<string[]> {
  return page
    .locator("#outline .chip")
    .evaluateAll((items) =>
      items.map(
        (c) =>
          `${c.textContent}${(c as HTMLElement).dataset.allowed === "this-reading" ? "" : " (dashed)"}`,
      ),
    );
}

const row = (page: Page, label: string) =>
  page.locator(`#outline [role=treeitem][data-label="${label}"]`);

const chip = (page: Page, symbol: string) =>
  page.locator(`#outline .chip[data-symbol="${symbol}"]`);

test("the outline shows the tree of a reading, with shared rows and folds", async ({ page }) => {
  await open(page, "tutorial");
  // Before the first step, the start rule node is the root, and the next
  // options follow it.
  await expect.poll(() => rows(page)).toEqual(["rule: tutorial", "next [1]: next"]);
  expect(await chips(page)).toEqual(["overview"]);

  await page
    .locator("#sequence")
    .fill(
      "overview install install configure introduce_concept show_example show_example set_exercise give_hint give_hint show_solution check_understanding introduce_concept show_example"
        .split(" ")
        .join("\n"),
    );
  await expect(page.getByTestId("reading-count")).toHaveText("Reading 1 of 1");
  await expect(page.getByTestId("output-status")).toHaveText("14 steps · cannot end yet");
  await expect
    .poll(() => rows(page))
    .toEqual([
      "rule: tutorial (open)",
      '  rule step [1]: overview · "overview"',
      "  rule: setup (open)",
      "    loop fold [2–3]: install+ · 2 passes · 2 steps (folded)",
      '    rule step [4]: configure · "configure"',
      "  loop: lesson+ | pass 2 (open)",
      "    fold [5–12]: completed · 1 pass · 8 steps (folded)",
      "    pass rule: pass 2 · lesson (open)",
      '      rule step [13]: introduce_concept · "introduce_concept"',
      "      loop: show_example{1,3} | pass 1 · 1 to 3 passes (open)",
      '        pass rule step [14]: pass 1 · show_example · "show_example"',
      "next [15]: next",
    ]);
  await expect(page.locator("#outline [data-last]")).toHaveAttribute("data-steps", "14");

  // A click opens the fold of the completed pass. Its loops fold by the same rule.
  await row(page, "completed · 1 pass · 8 steps").locator(".twisty").click();
  const completedPass = [
    "    fold [5–12]: completed · 1 pass · 8 steps (open)",
    "      pass rule: pass 1 · lesson (open)",
    '        rule step [5]: introduce_concept · "introduce_concept"',
    "        loop fold [6–7]: show_example{1,3} · 2 passes · 2 steps (folded)",
    "        loop fold [8–11]: practice* · 1 pass · 4 steps (folded)",
    '        rule step [12]: check_understanding · "check_understanding"',
  ];
  await expect.poll(async () => (await rows(page)).slice(6, 12)).toEqual(completedPass);

  // A click on a next option appends that step. The open fold stays open,
  // and the new pass folds the pass before it into a fold row.
  await chip(page, "show_example").click();
  await expect(page.locator("#sequence")).toHaveValue(/\nshow_example\nshow_example$/);
  await expect(page.getByTestId("output-status")).toHaveText("15 steps · cannot end yet");
  await expect
    .poll(async () => (await rows(page)).slice(6))
    .toEqual([
      ...completedPass,
      "    pass rule: pass 2 · lesson (open)",
      '      rule step [13]: introduce_concept · "introduce_concept"',
      "      loop: show_example{1,3} | pass 2 · 1 to 3 passes (open)",
      "        fold [14]: completed · 1 pass · 1 step (folded)",
      '        pass rule step [15]: pass 2 · show_example · "show_example"',
      "next [16]: next",
    ]);
});

test("a rule of two steps keeps the row of its first step, and a left loop folds", async ({
  page,
}) => {
  await open(page, "order-approval");
  await page.locator("#sequence").fill("submit_order\ncheck_order\nrequest_changes");
  const before = [
    "rule: order (open)",
    '  step [1]: "submit_order"',
    '  step [2]: "check_order"',
    "  loop: revision{0,3} | pass 1 · 0 to 3 passes (open)",
    "    pass rule: pass 1 · revision (open)",
    '      step [3]: "request_changes"',
  ];
  await expect.poll(() => rows(page)).toEqual([...before, "next [4]: next"]);

  // The second step of `revision` adds a row. The row of its first step stays.
  await chip(page, "revise_order").click();
  await expect
    .poll(() => rows(page))
    .toEqual([...before, '      step [4]: "revise_order"', "next [5]: next"]);

  // A step after the loop leaves it, so its loop row and its fold become one row.
  await chip(page, "remind").click();
  await expect
    .poll(() => rows(page))
    .toEqual([
      "rule: order (open)",
      '  step [1]: "submit_order"',
      '  step [2]: "check_order"',
      "  loop fold [3–4]: revision{0,3} · 1 pass · 2 steps (folded)",
      "  rule: outcome (open)",
      "    rule: approval (open)",
      '      loop: "remind"{0,2} | pass 1 · 0 to 2 passes (open)',
      '        pass step [5]: pass 1 · "remind"',
      "next [6]: next",
    ]);
});

test("a loop through rules counts one pass for each trip", async ({ page }) => {
  await open(page, "urg-moves");
  await page
    .locator("#sequence")
    .fill(
      "states the problem\nproposes a solution\nidentifies a gap\nrevises the solution\nidentifies a gap\nrevises the solution",
    );
  await expect(page.getByTestId("output-status")).toHaveText("6 steps · can end here");
  await expect
    .poll(() => rows(page))
    .toEqual([
      "rule: episode (open)",
      '  step [1]: "states the problem"',
      "  rule: development (open)",
      '    step [2]: "proposes a solution"',
      "    loop: refinement ↻ loop of tail references | pass 2 (open)",
      "      fold [3–4]: completed · 1 pass · 2 steps (folded)",
      "      pass rule: pass 2 · refinement (open)",
      '        step [5]: "identifies a gap"',
      '        step [6]: "revises the solution"',
      "next [7]: next | or end here",
    ]);
  expect(await chips(page)).toEqual(["identifies a gap", "gives an example"]);
});

test("the count of readings has a switch, and a step keeps the reading on view", async ({
  page,
}) => {
  await open(page, "order-approval");
  await page.locator("#sequence").fill("submit_order\ncheck_order\nremind");
  const count = page.getByTestId("reading-count");
  await expect(count).toHaveText("Reading 1 of 2");
  await expect(row(page, "approval")).toHaveCount(1);
  expect(await chips(page)).toEqual(["remind", "approve", "escalate", "reject (dashed)"]);
  await expect(page.locator("#reading-previous")).toBeDisabled();

  await page.locator("#reading-next").click();
  await expect(count).toHaveText("Reading 2 of 2");
  await expect(row(page, "rejection")).toHaveCount(1);
  await expect(row(page, "approval")).toHaveCount(0);
  expect(await chips(page)).toEqual(["remind", "approve (dashed)", "escalate", "reject"]);
  await expect(page.locator("#reading-next")).toBeDisabled();

  // A step that extends the reading on view keeps it on view.
  await chip(page, "reject").click();
  await expect(count).toHaveText("Reading 1 of 1");
  await expect(row(page, "rejection")).toHaveCount(1);
  await expect(page.locator("#output-notes li")).toHaveCount(0);

  // A step back shows the reading that the reading on view extended.
  await page.locator("#step-back").click();
  await expect(count).toHaveText("Reading 2 of 2");
  await expect(page.getByTestId("output-status")).toHaveText("3 of 4 steps read · cannot end yet");
  await expect(row(page, "1 later step, not read")).toHaveAttribute("data-steps", "4");

  // A step that the reading on view does not allow ends that reading.
  await chip(page, "approve").click();
  await expect(count).toHaveText("Reading 1 of 1");
  await expect(row(page, "approval")).toHaveCount(1);
  await expect(page.locator("#output-notes li")).toHaveText([
    "Reading 2 ended at step 3, so the view shows reading 1.",
  ]);
});

test("the count of readings notes the reading cap", async ({ page }) => {
  await open(page, "tutorial");
  const names = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten"];
  const grammar = [
    "%start choice ;",
    `choice = ${[...names, "eleven"].join(" | ")} ;`,
    ...[...names, "eleven"].map((name) => `${name} = "x" ;`),
  ].join("\n");
  await loadGrammarText(page, "cap.ebnf", grammar);
  await page.locator("#sequence").fill("x");
  await expect(page.getByTestId("reading-count")).toHaveText("Reading 1 of 10+");
  await expect(page.locator("#output-notes li")).toHaveText([
    "The tracer stopped at the reading cap of 10 readings, so more readings exist.",
  ]);
  await expect
    .poll(() => rows(page))
    .toEqual(["rule: choice (open)", '  rule step [1]: one · "x"', "next [2]: next | end here"]);
});

test("a rejected step shows the failure point, and the state before it", async ({ page }) => {
  await open(page, "tutorial");
  await page
    .locator("#sequence")
    .fill("overview\ninstall\nintroduce_concept\nset_exercise\nshow_solution\nsummarise");
  await expect(page.getByTestId("output-status")).toHaveText(
    "rejected at step 4 · showing the state after step 3",
  );
  await expect(page.getByTestId("reading-count")).toHaveText("Reading 1 of 1");
  const partialTree = [
    "rule: tutorial (open)",
    '  rule step [1]: overview · "overview"',
    "  rule: setup (open)",
    "    loop fold [2]: install+ · 1 pass · 1 step (folded)",
    "  loop: lesson+ | pass 1 (open)",
    "    pass rule: pass 1 · lesson (open)",
    '      rule step [3]: introduce_concept · "introduce_concept"',
  ];
  await expect
    .poll(() => rows(page))
    .toEqual([
      ...partialTree,
      'rejected [4]: "set_exercise" The grammar uses "set_exercise" in set_exercise, but not at this point.',
      "next [4]: next at 4",
      "unread fold [5–6]: 2 later steps, not read (folded)",
    ]);
  expect(await chips(page)).toEqual(["show_example"]);

  // A click opens the steps that the tracer has not read. They have no node.
  await row(page, "2 later steps, not read").locator(".twisty").click();
  await expect
    .poll(async () => (await rows(page)).slice(-2))
    .toEqual(['  unread step [5]: "show_solution"', '  unread step [6]: "summarise"']);

  // A click on a next option inserts it before the rejected step, and the
  // tracer reads only the inserted step.
  await chip(page, "show_example").click();
  await expect(page.locator("#sequence")).toHaveValue(
    "overview\ninstall\nintroduce_concept\nshow_example\nset_exercise\nshow_solution\nsummarise",
  );
  await expect(page.getByTestId("output-status")).toHaveText("4 of 7 steps read · cannot end yet");
  await expect(row(page, "3 later steps, not read")).toHaveAttribute("data-steps", "5–7");

  await page.locator("#step-forward").click();
  await expect(page.getByTestId("output-status")).toHaveText("5 of 7 steps read · cannot end yet");
  await expect(page.locator("#outline [data-last]")).toHaveAttribute(
    "data-label",
    'set_exercise · "set_exercise"',
  );

  // A symbol that the grammar never uses gets the other reason.
  await page.locator("#sequence").fill("overview\nbogus");
  await expect(page.locator('#outline [data-kind="rejected"]')).toHaveAttribute(
    "data-label",
    '"bogus" The grammar has no symbol "bogus".',
  );
});

test("a row keeps its place when a step removes an origin of its loop", async ({ page }) => {
  await open(page, "tutorial");
  // After `x x`, both repetitions give the same tree, so the loop node has
  // both of them as origins. The step `w` then removes the first one.
  await loadGrammarText(page, "origins.ebnf", '%start s ;\ns = ("x" "y"?)+ | ("x" | "w")+ ;');
  await page.locator("#sequence").fill("x\nx");
  await expect(page.getByTestId("output-status")).toHaveText("2 steps · can end here");
  await row(page, "completed · 1 pass · 1 step").locator(".twisty").click();
  const firstPass = ["      pass: pass 1 (open)", '        step [1]: "x"'];
  await expect.poll(async () => (await rows(page)).slice(3, 5)).toEqual(firstPass);

  await chip(page, "w").click();
  await expect(page.getByTestId("output-status")).toHaveText("3 steps · can end here");
  await expect
    .poll(() => rows(page))
    .toEqual([
      "rule: s (open)",
      '  loop: ("x" | "w")+ | pass 3 (open)',
      "    fold [1–2]: completed · 2 passes · 2 steps (open)",
      ...firstPass,
      "      pass: pass 2 (open)",
      '        step [2]: "x"',
      "    pass: pass 3 (open)",
      '      step [3]: "w"',
      "next [4]: next | or end here",
    ]);
});

test("a step that the cap leaves out does not end the reading on view", async ({ page }) => {
  await open(page, "tutorial");
  const others = Array.from({ length: 11 }, (_, i) => `c${i + 1}`);
  const grammar = [
    "%start s ;",
    "s = a | b ;",
    'a = "x" c ;',
    'b = "x" "y" ;',
    `c = ${others.join(" | ")} ;`,
    ...others.map((name) => `${name} = "y" ;`),
  ].join("\n");
  await loadGrammarText(page, "cap-extension.ebnf", grammar);
  await page.locator("#sequence").fill("x");
  const count = page.getByTestId("reading-count");
  await expect(count).toHaveText("Reading 1 of 2");
  await page.locator("#reading-next").click();
  await expect(count).toHaveText("Reading 2 of 2");
  await expect(row(page, "b")).toHaveCount(1);

  // Reading 2 allows `y`. After `y`, the eleven readings through `a` come
  // first, so the cap leaves out the reading through `b`.
  await chip(page, "y").click();
  await expect(count).toHaveText("Reading 1 of 10+");
  await expect(page.locator("#output-notes li")).toHaveText([
    "The tracer stopped at the reading cap of 10 readings, so more readings exist.",
    "The cap can leave out the extensions of reading 2, so the view shows reading 1.",
  ]);
});

test("a fold of a rule row stays with that rule in its reading", async ({ page }) => {
  await open(page, "tutorial");
  await loadGrammarText(
    page,
    "folds.ebnf",
    '%start s ;\ns = a "end" | b "end" ;\na = "x" "u" ;\nb = "x" "u" ;',
  );
  await page.locator("#sequence").fill("x\nu\nend");
  const count = page.getByTestId("reading-count");
  await expect(count).toHaveText("Reading 1 of 2");
  await row(page, "a").locator(".twisty").click();
  await expect(row(page, "a")).toHaveAttribute("data-fold", "folded");

  // The rule `b` of reading 2 has the same place, but the person never folded it.
  await page.locator("#reading-next").click();
  await expect(count).toHaveText("Reading 2 of 2");
  await expect(row(page, "b")).toHaveAttribute("data-fold", "open");
  await expect.poll(async () => (await rows(page)).length).toBe(6);

  await page.locator("#reading-previous").click();
  await expect(row(page, "a")).toHaveAttribute("data-fold", "folded");
});

test("the arrow keys move through the outline as an ARIA tree", async ({ page }) => {
  await open(page, "tutorial");
  await loadGrammarText(page, "keys.ebnf", '%start s ;\ns = "x" "y" ;');
  await page.locator("#sequence").fill("x\ny");
  await expect(page.getByTestId("output-status")).toHaveText("2 steps · can end here");

  const stepRow = (n: number) => page.locator(`#outline [role=treeitem][data-steps="${n}"]`);

  // Right arrow on an open parent goes to its first child.
  await row(page, "s").focus();
  await page.keyboard.press("ArrowRight");
  await expect(stepRow(1)).toBeFocused();

  // Right arrow on a leaf does nothing. Down arrow goes to the next row.
  await page.keyboard.press("ArrowRight");
  await expect(stepRow(1)).toBeFocused();
  await page.keyboard.press("ArrowDown");
  await expect(stepRow(2)).toBeFocused();

  // Left arrow on a leaf goes to its parent.
  await page.keyboard.press("ArrowLeft");
  await expect(row(page, "s")).toBeFocused();
});
