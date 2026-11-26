// File references: attribute values that the page can open. The section
// "File references" of docs/specs/output-view.md decides each rule.
//
// This module has no DOM. It classifies an attribute, and it resolves a
// relative file reference against the files of a folder that the person
// opened with the folder picker.

import type { Attribute } from "./core.ts";

/**
 * The type annotations of a reference. KDL 2.0 reserves them for RFC 3986
 * and RFC 3987 identifiers and references, as the section "File references"
 * of the spec states.
 */
const referenceAnnotations: ReadonlySet<string> = new Set([
  "url",
  "url-reference",
  "irl",
  "irl-reference",
]);

/** What a reference is: a web address, a relative file reference or text that the page never opens. */
export type Reference =
  | { readonly type: "web"; readonly href: string }
  | { readonly type: "file"; readonly path: string }
  | { readonly type: "text" };

/** The scheme of an RFC 3986 reference, such as `https` in `https://…`. */
const schemePattern = /^([A-Za-z][A-Za-z0-9+.-]*):/;

/**
 * The reference of an attribute, or undefined when the attribute is not a
 * reference. A plain string is never a reference, even when it looks like a
 * path.
 */
export function referenceOf(attribute: Attribute): Reference | undefined {
  if (attribute.annotation === undefined || !referenceAnnotations.has(attribute.annotation)) {
    return undefined;
  }
  if (attribute.value.type !== "string") {
    return { type: "text" };
  }
  const value = attribute.value.value;
  const scheme = schemePattern.exec(value)?.[1]?.toLowerCase();
  if (scheme !== undefined) {
    if (scheme !== "http" && scheme !== "https") {
      return { type: "text" };
    }
    // The URL parser checks the address, so the page opens only a valid
    // http or https address and never runs a value as a script.
    if (!URL.canParse(value)) {
      return { type: "text" };
    }
    return { type: "web", href: new URL(value).href };
  }
  // A path that starts with `/` or `//` is not relative to the folder of the trace file.
  if (value.startsWith("/") || value === "") {
    return { type: "text" };
  }
  return { type: "file", path: value };
}

/** The counts of the references of some steps, by kind: relative files and web addresses. */
export interface ReferenceCounts {
  readonly files: number;
  readonly links: number;
}

/** Counts the relative file references and the web addresses among some attributes. */
export function countReferences(attributes: Iterable<Attribute>): ReferenceCounts {
  let files = 0;
  let links = 0;
  for (const attribute of attributes) {
    const reference = referenceOf(attribute);
    if (reference?.type === "file") {
      files += 1;
    } else if (reference?.type === "web") {
      links += 1;
    }
  }
  return { files, links };
}

/** Where a relative file reference leads in an opened folder. */
export type Resolution =
  | { readonly type: "found"; readonly file: File; readonly path: string }
  | { readonly type: "not-found"; readonly path: string }
  | { readonly type: "outside" };

/**
 * The path of a relative file reference inside its folder: without a query
 * and a fragment, with percent escapes decoded and with `.` and `..`
 * resolved. It is undefined for a path that leaves the folder, and null for
 * a path with an escape that does not decode.
 */
export function folderPath(reference: string): string | undefined | null {
  const end = reference.search(/[?#]/);
  const path = end < 0 ? reference : reference.slice(0, end);
  const segments: string[] = [];
  for (const raw of path.split("/")) {
    let segment: string;
    try {
      segment = decodeURIComponent(raw);
    } catch {
      return null;
    }
    if (segment === "" || segment === ".") {
      continue;
    }
    if (segment === "..") {
      if (segments.pop() === undefined) {
        return undefined;
      }
      continue;
    }
    segments.push(segment);
  }
  return segments.join("/");
}

/**
 * The files of a folder that the person opened, by their paths inside that
 * folder. The folder lasts until the page reloads or the person opens
 * another folder.
 */
export class OpenedFolder {
  readonly name: string;
  readonly #files: ReadonlyMap<string, File>;

  /**
   * Takes the files of a `webkitdirectory` input. Each `webkitRelativePath`
   * starts with the name of the chosen folder
   * (https://developer.mozilla.org/en-US/docs/Web/API/File/webkitRelativePath),
   * so the path inside the folder drops that first segment.
   */
  constructor(files: Iterable<File>) {
    const byPath = new Map<string, File>();
    let name = "";
    for (const file of files) {
      const [first = "", ...rest] = file.webkitRelativePath.split("/");
      name ||= first;
      byPath.set(rest.length === 0 ? file.name : rest.join("/"), file);
    }
    this.name = name;
    this.#files = byPath;
  }

  get size(): number {
    return this.#files.size;
  }

  /** Where a relative file reference leads in this folder. */
  resolve(reference: string): Resolution {
    const path = folderPath(reference);
    if (path === undefined) {
      return { type: "outside" };
    }
    if (path === null) {
      return { type: "not-found", path: reference };
    }
    const file = this.#files.get(path);
    return file === undefined ? { type: "not-found", path } : { type: "found", file, path };
  }
}

/**
 * The media types that a new tab shows as they are. A page opens each other
 * file as plain text, because a file of the folder, such as an HTML or an SVG
 * file, could otherwise run a script with the origin of the page.
 */
const shownTypes =
  /^(image\/(png|jpeg|gif|webp|avif)|application\/pdf|text\/plain|audio\/.+|video\/.+)$/;

/** The blob of a file to open in a new tab: the file itself, or its text as plain text. */
export function safeBlob(file: File): Blob {
  return shownTypes.test(file.type) ? file : new Blob([file], { type: "text/plain;charset=utf-8" });
}
