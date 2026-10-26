// The TypeScript original of examples/harness-table.rhai: a policy with the
// shape of a working owner's, with invented names. The equivalence test of
// tests/rhai_examples.rs runs it beside its translation. The leaf that removes
// the TypeScript path deletes this file with the rest of that side.

import { readFileSync, statSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { definePolicy, type Refused, type SelectionResult } from "ravel/sdk";

/** A command line, and the labels it is recorded under. */
interface Harness {
  readonly provider: string;
  readonly model: string;
  readonly effort: string;
  readonly program: string;
  /** `grants` are the directories beyond the cwd the harness may write. */
  args(prompt: string, grants: readonly string[]): string[];
}

function reviewAgent(model: string, effort: string): Harness {
  return {
    provider: "provider-b",
    model,
    effort,
    program: "review-agent",
    args: (prompt, grants) => [
      "--model",
      model,
      "-c",
      `reasoning_effort=${effort}`,
      "-c",
      "permissions=full-access",
      "--ask-for-approval",
      "never",
      ...grants.flatMap((grant) => ["--add-dir", grant]),
      prompt,
    ],
  };
}

function leadAgent(model: string, effort: string): Harness {
  return {
    provider: "provider-a",
    model,
    effort,
    program: "lead-agent",
    args: (prompt, grants) => [
      ...grants.flatMap((grant) => ["--add-dir", grant]),
      "--model",
      model,
      "--effort",
      effort,
      prompt,
    ],
  };
}

function selectHarness(kind: string): Harness | undefined {
  const writer = (effort: string) => leadAgent("model-writer", effort);
  const lead = (effort: string) => leadAgent("model-lead", effort);
  const reviewer = (effort: string) => reviewAgent("model-review", effort);

  switch (kind) {
    case "scope": return leadAgent("model-deep", "max");
    case "scope-check": return reviewAgent("model-deep", "max");
    case "scope-fold": return leadAgent("model-deep", "max");

    case "design": return leadAgent("model-deep", "xhigh");
    case "design-check": return reviewAgent("model-deep", "xhigh");
    case "design-fold": return leadAgent("model-deep", "xhigh");

    case "build": return lead("high");
    case "build-check": return reviewer("high");
    case "build-fold": return lead("high");

    case "survey-a": return leadAgent("model-deep", "xhigh");
    case "survey-b": return reviewAgent("model-deep", "xhigh");

    case "draft": return writer("high");
    case "polish": return leadAgent("model-deep", "high");
    case "release-notes": return lead("medium");

    default: return undefined;
  }
}

function refused(code: string, message: string, remedy: string): Refused {
  return { status: "refused", code, message, remedy };
}

/**
 * The main repository a secondary jj workspace at `cwd` keeps its store in, or
 * nothing. A secondary workspace's `.jj/repo` is a file holding the store's
 * path, relative to `.jj/`, and the store is `<main>/.jj/repo`. The grant is
 * `<main>` rather than the store alone because a colocated repository's git
 * objects sit beside the store, in `<main>/.git`. A primary workspace's
 * `.jj/repo` is the store itself, a directory inside the cwd, and a directory
 * with no `.jj` is no workspace: neither needs a grant.
 */
function mainRepository(cwd: string): string[] | Refused {
  const file = join(cwd, ".jj", "repo");
  try {
    if (statSync(file, { throwIfNoEntry: false })?.isFile() !== true) return [];
    const store = readFileSync(file, "utf8").trim();
    if (store === "") throw new Error("it is empty");
    return [dirname(dirname(resolve(join(cwd, ".jj"), store)))];
  } catch (error) {
    return refused(
      "jj_store_unreadable",
      `${file} names no jj store: ${error instanceof Error ? error.message : String(error)}`,
      `run from a jj workspace whose ${file} names its store, or from a directory with no .jj`,
    );
  }
}

export const policy = definePolicy({
  schemaVersion: 2,
  version: "harness-table 2",

  select(request): SelectionResult {
    const harness = selectHarness(request.kind);
    if (harness === undefined) {
      return refused(
        "incomplete_mapping",
        `no mapping for kind ${JSON.stringify(request.kind)}`,
        "add a mapping for this kind in your policy",
      );
    }

    const grants = mainRepository(request.cwd);
    if ("status" in grants) return grants;

    return {
      status: "selected",
      program: harness.program,
      args: harness.args(request.prompt, grants),
      provider: harness.provider,
      model: harness.model,
      effort: harness.effort,
      reason: `routes ${JSON.stringify(request.kind)} to ${harness.program} with model ${harness.model} at effort ${harness.effort}`,
    };
  },
});
