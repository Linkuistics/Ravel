
import { closeSync, constants, fstatSync, openSync, readSync, realpathSync, writeSync } from "node:fs";
import { createHash } from "node:crypto";
import { resolve } from "node:path";

/** The bounds the front sent, in the units their names say. */
export interface Bounds {
  readonly contextBytes: number;
  readonly sourceBytes: number;
  readonly sources: number;
  readonly messageBytes: number;
}

/** One measured source, as the front receives it. */
export interface Measured {
  readonly name: string;
  readonly via: "--context" | "readText" | "readJson";
  readonly bytes: number;
  readonly sha256: string;
}

/** A bound exceeded, as the failure frame reports it. */
export interface Breach {
  readonly bound: { readonly name: string; readonly actual?: number; readonly source?: string; readonly maxBytes?: number };
}

/** A read that failed: the source is missing, unreadable, not UTF-8 or not JSON. */
export class SourceUnreadable extends Error {
  override readonly name = "SourceUnreadable";
  constructor(
    readonly source: string,
    message: string,
  ) {
    super(`${source} ${message}`);
  }
}

/** A bound exceeded. The selection refuses for it whether or not this is caught. */
export class BoundExceeded extends Error {
  override readonly name = "BoundExceeded";
}

const utf8 = new TextDecoder("utf-8", { fatal: true, ignoreBOM: true });


/** `value`, with every object and array in it frozen. */
export function deepFreeze<T>(value: T): T {
  if (typeof value === "object" && value !== null && !Object.isFrozen(value)) {
    Object.freeze(value);
    for (const key of Object.keys(value)) deepFreeze((value as Record<string, unknown>)[key]);
  }
  return value;
}

/** The state of one evaluation: its bounds, what it measured, and any breach. */
export class Session {
  readonly ledger: Measured[];
  breach: Breach | undefined;
  /** Whether `loadContext` is running, the only time reads are open. */
  reading = false;
  private readonly stop = new AbortController();
  private listening = false;

  constructor(
    private readonly cwd: string,
    readonly bounds: Bounds,
    measured: readonly Measured[],
  ) {
    this.ledger = [...measured];
  }

  /** Record `breach` if none is recorded yet, and throw it at the policy too. */
  breached(breach: Breach, error: Error): never {
    this.breach ??= breach;
    throw error;
  }

  /** The host `loadContext` receives. */
  contextHost(): object {
    return this.host(true);
  }

  /** The host `select` receives: no reads. */
  selectHost(): object {
    return this.host(false);
  }

  private host(reads: boolean): object {
    const session = this;
    const closed = (operation: string) => () => {
      throw new Error(
        `host.${operation} is available in loadContext only: select receives the context ` +
          "loadContext returned, as it was measured, and reads nothing more through the host",
      );
    };
    return Object.freeze({
      readText: reads
        ? (path: unknown, maxBytes?: unknown) => {
            const { bytes, source } = session.read(path, maxBytes, "readText");
            const text = session.decode(bytes, source.name);
            session.ledger.push({ ...source, via: "readText" });
            return { text, source };
          }
        : closed("readText"),
      readJson: reads
        ? (path: unknown, maxBytes?: unknown) => {
            const { bytes, source } = session.read(path, maxBytes, "readJson");
            const text = session.decode(bytes, source.name);
            let value: unknown;
            try {
              value = JSON.parse(text);
            } catch (error) {
              throw new SourceUnreadable(source.name, `is not JSON: ${(error as Error).message}`);
            }
            session.ledger.push({ ...source, via: "readJson" });
            return { value, source };
          }
        : closed("readJson"),
      diagnostic(text: unknown): void {
        if (typeof text !== "string") throw new TypeError("host.diagnostic takes a string");
        const line = text.endsWith("\n") ? text : `${text}\n`;
        const bytes = Buffer.from(line, "utf8");
        let written = 0;
        while (written < bytes.length) written += writeSync(2, bytes, written, bytes.length - written);
      },
      get signal(): AbortSignal {
        session.listen();
        return session.stop.signal;
      },
    });
  }

  /**
   * Abort the signal when the front stops the worker with TERM, at the
   * selection deadline. A policy with no TERM listener of its own still ends
   * on TERM, as it would with none installed, once its abort listeners have
   * run; one that has its own owns its exit, within the front's grace.
   */
  private listen(): void {
    if (this.listening) return;
    this.listening = true;
    process.on("SIGTERM", () => {
      this.stop.abort(new Error("ravel stopped this selection"));
      if (process.listenerCount("SIGTERM") === 1) process.exit(143);
    });
  }

  private read(
    path: unknown,
    maxBytes: unknown,
    via: "readText" | "readJson",
  ): { bytes: Buffer; source: { name: string; bytes: number; sha256: string } } {
    if (!this.reading) {
      throw new Error(`host.${via} was called after loadContext returned its context`);
    }
    if (typeof path !== "string" || path === "") {
      throw new TypeError(`host.${via} takes a nonempty path string`);
    }
    const resolved = resolve(this.cwd, path);
    let limit = this.bounds.sourceBytes;
    if (maxBytes !== undefined) {
      if (typeof maxBytes !== "number" || !Number.isSafeInteger(maxBytes) || maxBytes < 1) {
        throw new TypeError(`host.${via}'s maxBytes is a positive whole number of bytes`);
      }
      if (maxBytes > this.bounds.contextBytes) {
        this.breached(
          { bound: { name: "source", source: resolved, maxBytes } },
          new BoundExceeded(
            `host.${via} of ${resolved} asked for maxBytes ${maxBytes}, over the context budget of ` +
              `${this.bounds.contextBytes} bytes`,
          ),
        );
      }
      limit = maxBytes;
    }
    if (this.ledger.length >= this.bounds.sources) {
      this.breached(
        { bound: { name: "sources", source: resolved, actual: this.ledger.length + 1 } },
        new BoundExceeded(`host.${via} of ${resolved} would be source ${this.ledger.length + 1}, over ${this.bounds.sources}`),
      );
    }

    let fd: number;
    try {
      // Nonblocking, so that a FIFO cannot hold the read open; a regular
      // file reads the same either way.
      fd = openSync(resolved, constants.O_RDONLY | constants.O_NONBLOCK);
    } catch (error) {
      throw new SourceUnreadable(resolved, `cannot be opened: ${(error as Error).message}`);
    }
    try {
      if (!fstatSync(fd).isFile()) throw new SourceUnreadable(resolved, "is not a regular file");
      let name: string;
      try {
        name = realpathSync(resolved);
      } catch (error) {
        throw new SourceUnreadable(resolved, `cannot be resolved: ${(error as Error).message}`);
      }
      const chunks: Buffer[] = [];
      const chunk = Buffer.alloc(Math.min(limit + 1, 64 * 1024));
      let total = 0;
      for (;;) {
        let read: number;
        try {
          read = readSync(fd, chunk, 0, chunk.length, null);
        } catch (error) {
          throw new SourceUnreadable(name, `cannot be read: ${(error as Error).message}`);
        }
        if (read === 0) break;
        total += read;
        if (total > limit) {
          this.breached(
            {
              bound: {
                name: "source",
                source: name,
                actual: total,
                ...(maxBytes === undefined ? {} : { maxBytes: maxBytes as number }),
              },
            },
            new BoundExceeded(`host.${via} of ${name} holds more than its limit of ${limit} bytes`),
          );
        }
        chunks.push(Buffer.from(chunk.subarray(0, read)));
      }
      const bytes = Buffer.concat(chunks, total);
      const sha256 = createHash("sha256").update(bytes).digest("hex");
      return { bytes, source: { name, bytes: total, sha256 } };
    } finally {
      closeSync(fd);
    }
  }

  private decode(bytes: Buffer, name: string): string {
    try {
      return utf8.decode(bytes);
    } catch {
      throw new SourceUnreadable(name, "is not valid UTF-8");
    }
  }
}

