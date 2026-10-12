import { definePolicy, readChoice } from "ravel/sdk";
import { selectRoute, routes } from "ravel/examples/static";

export const policy = definePolicy({
  schemaVersion: 2,
  version: "owner-choice-1",
  select(request) {
    const choice = readChoice(request.cwd, ["default", "careful"]);
    if (choice !== undefined && "status" in choice) return choice;
    const names = choice ?? ["default"];
    if (names.length !== 1) return { status: "refused", code: "one_choice", message: "choose one arrangement", remedy: "write default or careful in .ravel-choice" };
    const selected = selectRoute(routes, request);
    if (selected.status === "refused" || names[0] !== "careful") return selected;
    return { ...selected, effort: "high", args: ["--model", selected.model, "--effort", "high", request.prompt], reason: "careful choice" };
  },
});
