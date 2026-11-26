// The technical-communication layers for Pi: the instruction, the reminder, the
// document gate and the reply log. Pi has no hook files, so this extension is
// the harness entry (docs/specs/technical-communication.md, "Harness mapping").
//
// The extension holds no rule logic. Each handler sends one event to the hook
// core, `scripts/tc-hook pi EVENT`, which reads the off file, runs the checker
// and returns at most one JSON object. The off file therefore acts at the next
// event, as on the other harnesses.
//
// A broken extension stops Pi from starting, in every mode, so this file:
//   - imports only Node's built-in modules, and no value from Pi's packages;
//   - keeps its factory to registrations;
//   - catches every error inside every handler, and gives Pi nothing then.
// Event names are not checked by Pi (an unknown name is accepted and never
// fires), so the names below are those of Pi 0.84.2, from
// dist/core/extensions/types.d.ts.

import { spawn } from "node:child_process";
import { realpathSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

// The timeouts of the Codex entries, for the same events.
const TIMEOUT_MS = { instruction: 5000, prompt: 5000, start: 10000, reply: 10000, write: 15000 };
const OUTPUT_LIMIT = 100_000; // bytes of the core's output to keep

// The skill directory, through this file's own real path: the documented
// command links the file into ~/.pi/agent/extensions/, so the loaded path is
// the link.
function hookScript(): string {
  let own = "";
  try {
    own = fileURLToPath(import.meta.url);
  } catch {
    own = typeof __filename === "string" ? __filename : "";
  }
  const real = realpathSync(own);
  // harnesses/pi/technical-communication.ts -> the skill directory
  return join(dirname(dirname(dirname(real))), "scripts", "tc-hook");
}

// Run one core event and resolve to the text it adds, or null. It never rejects,
// and it kills the child at the timeout: Pi gives its handlers none, and
// pi.exec cannot feed standard input. spawnSync would block Pi's rendering.
function core(event: string, payload: object, timeoutMs: number): Promise<string | null> {
  return new Promise((resolve) => {
    try {
      const child = spawn("/bin/sh", [hookScript(), "pi", event], {
        stdio: ["pipe", "pipe", "ignore"],
      });
      let out = "";
      let done = false;
      const finish = (value: string | null) => {
        if (done) return;
        done = true;
        clearTimeout(timer);
        resolve(value);
      };
      const timer = setTimeout(() => {
        child.kill("SIGKILL");
        finish(null);
      }, timeoutMs);
      child.on("error", () => finish(null));
      child.on("close", () => {
        try {
          const text = JSON.parse(out).additionalContext;
          finish(typeof text === "string" && text ? text : null);
        } catch {
          finish(null); // no output is the normal answer: off, or nothing to add
        }
      });
      child.stdout.on("data", (chunk: Buffer) => {
        if (out.length < OUTPUT_LIMIT) out += chunk.toString("utf8");
      });
      child.stdout.on("error", () => {});
      child.stdin.on("error", () => {});
      child.stdin.end(JSON.stringify(payload));
    } catch {
      resolve(null);
    }
  });
}

function sessionId(ctx: any): string | undefined {
  try {
    return ctx.sessionManager.getSessionId();
  } catch {
    return undefined;
  }
}

// The text of the last assistant message of a run, or "" when that message is an
// error, an abort or has no text.
function lastReply(messages: any[]): { text: string; model: string | undefined } {
  for (let i = messages.length - 1; i >= 0; i--) {
    const message = messages[i];
    if (message?.role !== "assistant") continue;
    if (message.stopReason === "error" || message.stopReason === "aborted") break;
    const text = (Array.isArray(message.content) ? message.content : [])
      .filter((block: any) => block?.type === "text" && typeof block.text === "string")
      .map((block: any) => block.text)
      .join("\n");
    return { text, model: typeof message.model === "string" ? message.model : undefined };
  }
  return { text: "", model: undefined };
}

// What Pi's session_start reasons mean to the log's `source`. A reload keeps the
// same session and the instruction is already in place, so it writes no record.
const START_SOURCE: Record<string, string> = {
  startup: "startup",
  new: "startup",
  resume: "resume",
  fork: "fork",
};

export default function (pi: any) {
  // The start record. The instruction itself goes out in before_agent_start,
  // because Pi clears a system-prompt override after every run.
  pi.on("session_start", async (event: any, ctx: any) => {
    try {
      const source = START_SOURCE[event?.reason];
      if (!source) return;
      await core("session-start", { session_id: sessionId(ctx), source }, TIMEOUT_MS.start);
    } catch {}
  });

  // The instruction goes into the system prompt of this run, and Pi does not
  // save it. The reminder is a hidden message that Pi saves after the prompt, as
  // the other harnesses do. Both go out here, so one run asks the core twice, at
  // the same time. A saved message keeps the history the same from one request to
  // the next, so the provider's prompt cache keeps working. A reminder added to
  // the end of each request would move the cache breakpoint every time.
  pi.on("before_agent_start", async (event: any) => {
    try {
      const [instruction, reminder] = await Promise.all([
        core("instruction", {}, TIMEOUT_MS.instruction),
        core("prompt", {}, TIMEOUT_MS.prompt),
      ]);
      const result: any = {};
      if (instruction && typeof event?.systemPrompt === "string") {
        result.systemPrompt = `${event.systemPrompt}\n\n${instruction}`;
      }
      if (reminder) {
        result.message = { customType: "technical-communication", content: reminder, display: false };
      }
      return result.systemPrompt || result.message ? result : undefined;
    } catch {}
  });

  // The document gate. The findings join the tool result, which the model reads
  // before its next call. isError stays as it is.
  pi.on("tool_result", async (event: any, ctx: any) => {
    try {
      if ((event?.toolName !== "write" && event?.toolName !== "edit") || event.isError) return;
      const text = await core(
        "write",
        { session_id: sessionId(ctx), cwd: ctx?.cwd, tool_input: { path: event.input?.path } },
        TIMEOUT_MS.write,
      );
      if (!text || !Array.isArray(event.content)) return;
      return { content: [...event.content, { type: "text", text }] };
    } catch {}
  });

  // The reply log. agent_end fires once for each run, and a retry or a queued
  // follow-up is a run of its own.
  pi.on("agent_end", async (event: any, ctx: any) => {
    try {
      if (!Array.isArray(event?.messages)) return;
      const { text, model } = lastReply(event.messages);
      if (!text.trim()) return;
      await core(
        "reply",
        { session_id: sessionId(ctx), model, last_assistant_message: text },
        TIMEOUT_MS.reply,
      );
    } catch {}
  });
}
