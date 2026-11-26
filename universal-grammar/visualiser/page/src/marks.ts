// The marks of a trace in the grammar view, by drawn element: the section
// "Marks of a trace" of docs/specs/rule-diagrams.md. The core computes the
// marks of the open diagram. This module only groups them by the element that
// draws them, and decides which outline is solid and which is dashed.

import type { MarkElement, Marks } from "./core.ts";

/**
 * The key of an element of the open diagram. A box, a back flow and a back
 * edge name the span of their grammar element, and a diagram has one start
 * event and one end event.
 */
export type ElementKey = string;

export function elementKey(element: MarkElement): ElementKey {
  switch (element.type) {
    case "start-event":
    case "end-event":
      return element.type;
    default:
      return `${element.type}:${element.span.start}-${element.span.end}`;
  }
}

/** The style of a next-option mark: solid for the reading on view, else dashed. */
export type OptionStyle = "solid" | "dashed";

/** The marks of one drawn element. */
export interface ElementMarks {
  /** The numbers of the readings whose markers sit on the element, in order. */
  readonly markers: readonly number[];
  /**
   * The next options that pass through the element, in the order of the next
   * options of the state. A symbol is solid when the reading on view allows
   * it through the element.
   */
  readonly options: ReadonlyMap<string, OptionStyle>;
  /** On the end event: whether the sequence can end, for the reading on view or only for others. */
  readonly canEnd: OptionStyle | undefined;
  /** The style of the outline, or none when no next option passes through. */
  readonly outline: OptionStyle | undefined;
}

/** The marks of the open diagram, grouped for the drawing. */
export interface MarksView {
  readonly elements: ReadonlyMap<ElementKey, ElementMarks>;
  /** The current pass of each loop that holds the reading on view, by the key of its back flow or back edge. */
  readonly flowPasses: ReadonlyMap<ElementKey, number>;
  /** The current pass, by the index of the breadcrumb of its pass rule. */
  readonly crumbPasses: ReadonlyMap<number, number>;
  /** The listed readings that do not stand in the open diagram, in order. */
  readonly outside: readonly number[];
  /** The next options that no listed reading allows. */
  readonly unlisted: readonly string[];
  /** The boxes of the data object of the step in focus, for the reading on view. */
  readonly dataObject: readonly ElementKey[];
}

interface Building {
  markers: number[];
  options: Map<string, OptionStyle>;
  canEnd: OptionStyle | undefined;
}

/** A solid style wins over a dashed one. */
const stronger = (a: OptionStyle | undefined, b: OptionStyle): OptionStyle =>
  a === "solid" || b === "solid" ? "solid" : "dashed";

/**
 * Groups the marks of the open diagram by element. `onView` is the number of
 * the reading on view, `count` the number of listed readings and `order` the
 * symbols of the next options of the state, in their order.
 */
export function marksView(
  marks: Marks,
  onView: number,
  count: number,
  order: readonly string[],
): MarksView {
  const building = new Map<ElementKey, Building>();
  const at = (element: MarkElement): Building => {
    const key = elementKey(element);
    let found = building.get(key);
    if (found === undefined) {
      found = { markers: [], options: new Map(), canEnd: undefined };
      building.set(key, found);
    }
    return found;
  };
  const addOption = (b: Building, symbol: string, style: OptionStyle): void => {
    b.options.set(symbol, stronger(b.options.get(symbol), style));
  };

  const standing = new Set<number>();
  const flowPasses = new Map<ElementKey, number>();
  const crumbPasses = new Map<number, number>();
  const dataObject: ElementKey[] = [];
  for (const reading of marks.readings) {
    const style: OptionStyle = reading.reading === onView ? "solid" : "dashed";
    for (const element of reading.marker) {
      at(element).markers.push(reading.reading);
    }
    if (reading.marker.length > 0) {
      standing.add(reading.reading);
    }
    for (const option of reading.options) {
      for (const element of option.elements) {
        addOption(at(element), option.symbol, style);
      }
    }
    if (reading.end !== undefined) {
      const end = at({ type: "end-event" });
      for (const symbol of reading.end.options) {
        addOption(end, symbol, style);
      }
      if (reading.end.canEnd) {
        end.canEnd = stronger(end.canEnd, style);
      }
    }
    // Only the reading on view shows its current passes and its data object.
    if (reading.reading === onView) {
      dataObject.push(...reading.dataObject.map(elementKey));
      for (const { place, pass } of reading.passes) {
        if (place.type === "breadcrumb") {
          crumbPasses.set(place.index, pass);
        } else {
          flowPasses.set(elementKey(place), pass);
        }
      }
    }
  }

  const rank = (symbol: string): number => {
    const i = order.indexOf(symbol);
    return i < 0 ? order.length : i;
  };
  const elements = new Map<ElementKey, ElementMarks>();
  for (const [key, b] of building) {
    const options = new Map(
      [...b.options].sort(([a], [c]) => rank(a) - rank(c) || (a < c ? -1 : a > c ? 1 : 0)),
    );
    let outline: OptionStyle | undefined = b.canEnd;
    for (const style of options.values()) {
      outline = stronger(outline, style);
    }
    elements.set(key, {
      markers: [...b.markers].sort((x, y) => x - y),
      options,
      canEnd: b.canEnd,
      outline,
    });
  }

  const outside: number[] = [];
  for (let reading = 1; reading <= count; reading++) {
    if (!standing.has(reading)) {
      outside.push(reading);
    }
  }
  return {
    elements,
    flowPasses,
    crumbPasses,
    outside,
    unlisted: marks.unlistedOptions,
    dataObject,
  };
}

/** A list of words for a label: "a", "a and b", "a, b and c". */
export function listWords(words: readonly string[]): string {
  if (words.length <= 1) {
    return words.join("");
  }
  return `${words.slice(0, -1).join(", ")} and ${words[words.length - 1]}`;
}

/**
 * The words that a label adds for the marks on an element, such as
 * "marker of reading 1, next option remind". It is empty without a mark.
 */
export function marksWords(marks: ElementMarks | undefined): string {
  if (marks === undefined) {
    return "";
  }
  const parts: string[] = [];
  if (marks.markers.length > 0) {
    const numbers = marks.markers.map(String);
    parts.push(
      numbers.length === 1
        ? `marker of reading ${numbers[0]}`
        : `markers of readings ${listWords(numbers)}`,
    );
  }
  const solid = [...marks.options].filter(([, s]) => s === "solid").map(([symbol]) => symbol);
  const dashed = [...marks.options].filter(([, s]) => s === "dashed").map(([symbol]) => symbol);
  if (marks.canEnd === "solid") {
    solid.push("end");
  } else if (marks.canEnd === "dashed") {
    dashed.push("end");
  }
  if (solid.length > 0) {
    parts.push(`${solid.length === 1 ? "next option" : "next options"} ${listWords(solid)}`);
  }
  if (dashed.length > 0) {
    parts.push(
      `${dashed.length === 1 ? "next option" : "next options"} of other readings ${listWords(dashed)}`,
    );
  }
  return parts.join(", ");
}

/** The sentence of the header for the readings outside the open diagram. */
export function outsideText(outside: readonly number[]): string | undefined {
  if (outside.length === 0) {
    return undefined;
  }
  return outside.length === 1
    ? `Reading ${outside[0]} stands outside this diagram.`
    : `Readings ${listWords(outside.map(String))} stand outside this diagram.`;
}
