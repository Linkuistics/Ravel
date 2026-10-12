
import { definePolicy, type Selected, type SelectionRequest, type SelectionResult } from "ravel/sdk";

/** What a route gives: a selected result without its status and reason. */
export type Command = Omit<Selected, "status" | "reason">;

/** A table entry: the command for a request. */
export type Route = (request: SelectionRequest) => Command;

/** One harness and model at `effort`, with the prompt as its last argument. */
export function agent(effort: string): Route {
  const model = "your-model";
  return (request) => ({
    program: "my-agent-wrapper",
    args: ["--model", model, "--effort", effort, request.prompt],
    provider: "your-provider",
    model,
    effort,
  });
}

/** Kind to command, exactly. A kind not listed refuses; nothing falls back. */
export const routes = {
  // Answering a question about existing code changes nothing, so it is wholly
  // reversible. The reader checks the answer at once, and a wrong one costs a
  // follow-up question rather than rework.
  question: agent("low"),
  // A bug fix is concrete and local. The failing test states the goal and
  // checks the result, and a bad fix is reverted in one step.
  bugfix: agent("medium"),
  // A feature adds behaviour across modules. Tests check what it does, but the
  // interfaces it chooses bind later work, so a poor choice is repaired
  // downstream.
  feature: agent("high"),
  // A migration changes data or published interfaces in place. It is hard to
  // reverse once it has run, its consequences reach every consumer, and
  // checks often show the damage only afterwards.
  migration: agent("xhigh"),
  // Architecture is the most abstract and most uncertain work here. Nothing
  // runs yet to check it, and every later task builds on it, so a mistake is
  // repaired by redoing that work.
  architecture: agent("xhigh"),
} as const satisfies Readonly<Record<string, Route>>;

/**
 * The command `routes` gives for the request's kind, with a reason naming the
 * entry applied, or the policy's own refusal for a kind the table does not
 * list.
 */
export function selectRoute(routes: Readonly<Record<string, Route>>, request: SelectionRequest): SelectionResult {
  const kind = JSON.stringify(request.kind);
  const route = Object.hasOwn(routes, request.kind) ? routes[request.kind] : undefined;
  if (route === undefined) {
    return {
      status: "refused",
      code: "incomplete_mapping",
      message: `the routes name no command for kind ${kind}`,
      remedy: "add a route for this kind in your copy of the policy",
    };
  }
  const command = route(request);
  return {
    status: "selected",
    ...command,
    reason: `routes[${kind}] gives ${command.program} with model ${command.model} at effort ${command.effort}`,
  };
}

/** The whole starter policy: {@link routes}, selected by kind. */
export const policy = definePolicy({
  schemaVersion: 2,
  version: "ravel/examples/static 2",
  select: (request) => selectRoute(routes, request),
});
