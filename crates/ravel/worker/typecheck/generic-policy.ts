import { definePolicy, readChoice } from "ravel/sdk";
import { selectRoute, routes } from "ravel/examples/static";
export const policy = definePolicy({
  schemaVersion: 2, version: "generic-1",
  loadContext(request, host) {
    const read = host.readText(`${request.cwd}/input.txt`);
    return { schemaVersion: 1, facts: { input: read.text }, sources: [read.source] };
  },
  select(request, context) {
    const choice = readChoice(request.cwd, ["fast", "careful"]);
    if (choice !== undefined && "status" in choice) return choice;
    return selectRoute(routes, request);
  },
});
