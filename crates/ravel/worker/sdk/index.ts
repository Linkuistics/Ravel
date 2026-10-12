
import { readFileSync, statSync } from "node:fs";
import { join } from "node:path";

/** Any JSON value. A context carries data, never code. */
export type Json = null | boolean | number | string | readonly Json[] | { readonly [key: string]: Json };

/**
 * A piece of evidence a context attributes: its name, and its SHA-256, its
 * version or both, so that the context never claims evidence it cannot pin.
 * A host read returns one, ready to put in `sources`.
 */
export type SourceRecord = {
  /** Nonblank: a path, URL or other name for the evidence. */
  readonly name: string;
  /** Its length, when known. */
  readonly bytes?: number;
} & (
  | { readonly sha256: string; readonly version?: string }
  | { readonly sha256?: string; readonly version: string }
);

/** A judgment in a context, attributed to whoever made it. */
export interface Assessment {
  /** Nonblank: who assessed it, such as `owner` or `triage-model`. */
  readonly by: string;
  readonly value: Json;
}

/**
 * A version-1 context: what a caller supplies with `--context`, and what
 * `loadContext` returns. Every field but the version is optional, and absent
 * means unknown: an empty `facts` is a known empty set, not a missing one.
 * Unknown fields are refused, and so is any field named like an executable
 * one, such as `program` or `args`.
 */
export interface Context {
  readonly schemaVersion: 1;
  readonly summary?: string;
  readonly acceptanceCriteria?: readonly string[];
  /** The caller's facts, as data. */
  readonly facts?: { readonly [key: string]: Json };
  /** Judgments about the work, each attributed. */
  readonly assessments?: { readonly [name: string]: Assessment };
  /** The evidence the context draws on; at most 256 records. */
  readonly sources?: readonly SourceRecord[];
}

/** One source ravel measured for the context. */
export interface MeasuredSource {
  /** The canonical source-file path. */
  readonly name: string;
  /** The `--context` document, or the host operation that delivered it. */
  readonly via: "--context" | "readText" | "readJson";
  /**
   * The bytes actually read.
   */
  readonly bytes: number;
  /** The SHA-256 of those bytes, in lowercase hexadecimal. */
  readonly sha256: string;
}

/**
 * The context `select` receives: the context `loadContext` returned, or the
 * caller's without a loader, and every source ravel measured for
 * it, the `--context` document first and then each read in order. Its whole
 * JSON encoding, `measured` included, is within `limits.contextBytes`. It is
 * frozen.
 */
export interface DeliveredContext extends Context {
  readonly measured: readonly MeasuredSource[];
}

/** What a host read returns beside the content: a source record to attribute it. */
export interface ReadSource {
  readonly name: string;
  readonly bytes: number;
  readonly sha256: string;
}

/** The host `select` receives. */
export interface SelectHost {
  /** Write one line of policy diagnostic text on stderr. */
  diagnostic(text: string): void;
  /**
   * Aborted when ravel stops the selection, at its deadline, so
   * that work in flight, such as a `fetch`, can stop too.
   */
  readonly signal: AbortSignal;
}

/**
 * The host `loadContext` receives. Its reads are the only
 * measured sources: each read resolves a relative path against `request.cwd`,
 * reads a regular file once, and returns its content with the canonical name,
 * byte count and SHA-256 of the bytes read. A read over its limit, or a read
 * past the 256th source, refuses the selection, even if the error
 * it throws is caught. A missing or unreadable source throws an error that, if
 * `loadContext` fails because of it, names that source in the refusal.
 */
export interface ContextHost extends SelectHost {
  /**
   * Read a UTF-8 text file. `maxBytes` defaults to `limits.sourceBytes` and
   * may be set up to `limits.contextBytes`, never beyond.
   */
  readText(path: string, maxBytes?: number): { readonly text: string; readonly source: ReadSource };
  /** Read a JSON file, as `readText` does, and parse it. */
  readJson(path: string, maxBytes?: number): { readonly value: Json; readonly source: ReadSource };
}

/**
 * Assemble the context `select` will receive. It may be asynchronous, and it
 * runs within the selection bound. It returns a version-1 context, typically
 * built from `request.context` and host reads, and ravel attaches
 * the measured sources. A loader that throws, rejects or returns nothing
 * refuses the selection, and nothing is selected without its context.
 *
 * A loader that finds what it requires invalid, rather than unreadable, can
 * say why by returning a {@link Refused}, as `select` does. The selection then
 * refuses as the policy's own, `policy_refused` with its code, message and
 * remedy, and `select` is not called.
 */
export type LoadContext = (
  request: SelectionRequest,
  host: ContextHost,
) => Context | Refused | PromiseLike<Context | Refused>;

/**
 * A policy: the one named export, `policy`, of the selected entry.
 *
 * `select` may be synchronous or return a promise, and may compute anything,
 * within the whole-selection bound in `request.limits`. It receives the
 * request, the measured context and a host without reads. ravel
 * validates the shape of what it returns and nothing about its content: it
 * does not check that the prompt is among the arguments, that the program is
 * one you listed anywhere, or that a kind has a route. Those are your
 * function's to hold, with the type checker and whatever tables it keeps. A
 * result that adds a field, abstains with `undefined` or `null`, throws,
 * rejects or is left unsettled refuses, and nothing is run in its place.
 *
 * A policy that turns outside text into a command, such as an agent's answer
 * or a file's content, should choose among commands its own code wrote and
 * never place that text in `program` or `args`.
 *
 * `ravel/examples/static`
 * are editable starting points whose `select` consults an exact table by kind.
 */
export interface Policy {
  readonly schemaVersion: 2;
  /** Nonblank, owner-maintained policy version. */
  readonly version: string;
  /** Optional: assembles the context `select` receives. */
  readonly loadContext?: LoadContext;
  /**
   * `context` is what `loadContext` returned, or the caller's `--context`
   * without a loader, with its measured sources; `undefined` when there is
   * neither.
   */
  select(
    request: SelectionRequest,
    context: DeliveredContext | undefined,
    host: SelectHost,
  ): SelectionResult | PromiseLike<SelectionResult>;
}

/**
 * What `loadContext` and `select` are asked: everything the caller passed.
 * Optional fields are absent, not empty, when the caller did not supply them.
 * The request is frozen. Session completion authority is private to the managed child.
 */
export interface SelectionRequest {
  readonly schemaVersion: 2;
  /** The caller's kind, an open token. */
  readonly kind: string;
  /** The caller's prompt, unchanged. The caller supplies ending instructions. */
  readonly prompt: string;
  /** The caller's working directory, as data; the worker does not run in it. */
  readonly cwd: string;
  /**
   * Every `--param NAME=VALUE` the caller passed, by name; empty when it
   * passed none. ravel gives a parameter no meaning: only your
   * policy reads it, and a name the caller did not pass is absent.
   */
  readonly params: { readonly [name: string]: string };
  /** The caller's `--task-file`, absolute; not read on the policy's behalf. */
  readonly taskFile?: string;
  /** The caller's `--task-id`. */
  readonly taskId?: string;
  /** The caller's `--context` document, validated, as data. */
  readonly context?: Context;
  readonly limits: Limits;
}

/**
 * The bounds in effect for this invocation. They are for reading: the worker
 * enforces its own copies, and a policy cannot raise one.
 */
export interface Limits {
  /** The whole-selection bound, in milliseconds, from the worker's start. */
  readonly selectionMs: number;
  /** The delivered context's JSON encoding, `measured` included, in bytes. */
  readonly contextBytes: number;
  /** One host read's default limit, in bytes; `maxBytes` may raise it to `contextBytes`. */
  readonly sourceBytes: number;
  /** Measured sources, the `--context` document included. */
  readonly sources: number;
  /** A policy snapshot or selection result, as a protocol message, in bytes. */
  readonly messageBytes: number;
  /** Everything the policy prints on stdout and stderr together, in bytes. */
  readonly diagnosticsBytes: number;
}

/**
 * The command to run, and the owner's labels for what it runs.
 *
 * ravel executes `program` with `args`, each string one whole
 * argument, with no shell, no splitting and no second reading of any text. A
 * caller value inside an argument is ordinary string building. `provider`,
 * `model` and `effort` are your labels: they are never inferred from the
 * program or its arguments, they need not appear in `args`, and they reach
 * routing diagnostics as you wrote them. Every field is a nonblank
 * string except `args`, whose strings may be empty, and neither `program`
 * nor an argument may hold a NUL.
 */
export interface Selected {
  readonly status: "selected";
  /**
   * An absolute path, a name looked up in the caller's PATH, or a relative
   * path containing a separator, resolved against the caller's cwd. It is also
   * the harness's argv[0].
   */
  readonly program: string;
  /** The command's arguments after `program`. */
  readonly args: readonly string[];
  /**
   * The model's origin as you declare it. A gateway change does not make a
   * different provider.
   */
  readonly provider: string;
  /** The model, as your harness understands it. */
  readonly model: string;
  /** The reasoning effort, as your harness understands it. */
  readonly effort: string;
  /** Routing diagnostics report it verbatim. */
  readonly reason: string;
}

/**
 * The policy's own refusal, from `select` or `loadContext`. It is reported as
 * `policy_refused`, exit 3, with `code` beside it as `policyCode`, and
 * launches nothing.
 */
export interface Refused {
  readonly status: "refused";
  /** Nonblank; the policy's own code, such as `no_reviewer`. */
  readonly code: string;
  /** Nonblank: what is wrong. */
  readonly message: string;
  /** Nonblank: what the caller or owner can do about it. */
  readonly remedy: string;
}

/** What `select` returns, or resolves to. */
export type SelectionResult = Selected | Refused;

/** The choice file's name, in the directory {@link readChoice} is given. */
export const CHOICE_FILE = ".ravel-choice";

/** The most a choice file may hold, in bytes. */
const CHOICE_BYTES = 4096;

/**
 * Read the choice file in `directory`, so that one checkout can select
 * differently from another: the names it holds, separated by whitespace, or
 * `undefined` when there is no such file. Pass the caller's directory,
 * `request.cwd`; a relative one resolves against `/`, where policy code runs.
 *
 * `offered` is every name your policy accepts. A name outside it is refused,
 * naming the file, the name and the names offered, and so is a file that is
 * not a regular UTF-8 file of at most 4 KiB. Return that refusal from
 * `select`:
 *
 *     const choice = readChoice(request.cwd, ["fast", "careful"]);
 *     if (choice !== undefined && "status" in choice) return choice;
 *     const names = choice ?? ["careful"];
 *
 * The file can therefore choose among what your policy already holds, and can
 * introduce no program, argument or label. What each name means, and how many
 * may be given, is your policy's to decide. This is an ordinary read, not a
 * measured source: name the choice you applied in your `reason`.
 */
export function readChoice(directory: string, offered: readonly string[]): readonly string[] | undefined | Refused {
  const file = join(directory, CHOICE_FILE);
  const refused = (code: string, message: string): Refused => ({
    status: "refused",
    code,
    message: `the choice file ${file} ${message}`,
    remedy: `write names from ${offered.join(", ")} in ${file}, separated by whitespace, or delete it`,
  });
  let text: string;
  try {
    // `throwIfNoEntry: false` answers a missing entry, a dangling link
    // included, with `undefined` in place of ENOENT:
    // https://nodejs.org/api/fs.html#fsstatsyncpath-options
    // tests/choice_file.rs holds the worker's pinned Bun to that.
    const stat = statSync(file, { throwIfNoEntry: false });
    if (stat === undefined) return undefined;
    if (!stat.isFile()) return refused("choice_unreadable", "is not a regular file");
    if (stat.size > CHOICE_BYTES) return refused("choice_unreadable", `holds more than ${CHOICE_BYTES} bytes`);
    text = new TextDecoder("utf-8", { fatal: true }).decode(readFileSync(file));
  } catch (error) {
    return refused("choice_unreadable", `cannot be read: ${error instanceof Error ? error.message : String(error)}`);
  }
  const names = text.split(/\s+/).filter((name) => name !== "");
  const unoffered = names.find((name) => !offered.includes(name));
  if (unoffered !== undefined) {
    return refused(
      "choice_unoffered",
      `names ${JSON.stringify(unoffered)}, which this policy does not offer; it offers ${offered.join(", ")}`,
    );
  }
  return names;
}

/**
 * Returns its argument unchanged. It exists so that
 * `export const policy = definePolicy({ ... })` type-checks the literal against
 * {@link Policy}, a field the contract does not have included, and types
 * `select`'s parameters without an annotation.
 */
export function definePolicy(policy: Policy): Policy {
  return policy;
}
