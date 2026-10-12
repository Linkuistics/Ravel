// The worker's half of the private protocol channel.
//
// The front process hands the worker one end of a socket pair as descriptor 3,
// and nothing else names it: no environment variable, path or argument carries
// the channel, so there is nothing ambient for a policy or a caller to redirect.
// Stdout and stderr stay the policy's diagnostic streams, which the front
// captures separately, so nothing a policy prints can reach a frame.
//
// A frame is a four-byte big-endian length followed by that many bytes of
// UTF-8 JSON. Reads and writes are synchronous: the worker does nothing else while it
// waits for the front, and a synchronous write cannot be reordered with a later
// `process.exit`.
//
// Each frame the worker sends has a bound, which the front applies again as it
// reads: the protocol message bound for a snapshot or a result, the context
// budget for a context. The caller checks a frame's encoded length against its
// bound before sending it, and reports an overflow in a small frame instead.

import { readSync, writeSync } from "node:fs";

const CHANNEL = 3;

/** The protocol version both halves speak; the hello announces it. */
export const PROTOCOL = 1;

/**
 * The largest frame the front sends: its evaluate message carries the caller's
 * context, up to the 8 MiB ceiling as the front re-encodes it, beside the
 * request, whose prompt and parameters the factor of two leaves room for. The
 * front's `FRONT_FRAME_BYTES` is the same.
 */
export const FRONT_FRAME_BYTES = 2 * 8 * 1024 * 1024 + 1024 * 1024;

/** A message's frame body: its UTF-8 JSON. */
export function encode(message: object): Buffer {
  return Buffer.from(JSON.stringify(message), "utf8");
}

/** Send a message whose frame is small by construction: a hello or a failure. */
export function send(message: object): void {
  sendEncoded(encode(message));
}

/** Send a frame body already encoded, and checked against its bound. */
export function sendEncoded(body: Buffer): void {
  const frame = Buffer.alloc(4 + body.length);
  frame.writeUInt32BE(body.length, 0);
  body.copy(frame, 4);
  let written = 0;
  while (written < frame.length) {
    written += writeSync(CHANNEL, frame, written, frame.length - written);
  }
}

/**
 * The front's next message, or `null` if the front closed the channel between
 * frames: it has nothing more to ask. A channel closed inside a frame throws.
 */
export function receive(): unknown {
  const header = readExact(4, true);
  if (header === null) return null;
  const length = header.readUInt32BE(0);
  if (length > FRONT_FRAME_BYTES) {
    throw new Error(`protocol frame of ${length} bytes exceeds ${FRONT_FRAME_BYTES}`);
  }
  return JSON.parse(readExact(length, false)!.toString("utf8"));
}

/** `length` bytes, or `null` when `mayEnd` and the channel ends before the first. */
function readExact(length: number, mayEnd: boolean): Buffer | null {
  const buffer = Buffer.alloc(length);
  let filled = 0;
  while (filled < length) {
    const read = readSync(CHANNEL, buffer, filled, length - filled, null);
    if (read === 0) {
      if (mayEnd && filled === 0) return null;
      throw new Error("the front process closed the protocol channel inside a frame");
    }
    filled += read;
  }
  return buffer;
}
