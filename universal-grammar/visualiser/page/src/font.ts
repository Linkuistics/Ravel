// The font of the diagram labels. The core measures each label in cells of
// IBM Plex Mono at 13 px, so the page must draw the labels in that font:
// docs/adr/bundled-monospace-font-for-diagram-labels.md. The `@font-face` rule
// in style.css names the bundled file.

/** The family of the `@font-face` rule for the bundled font. */
export const diagramFontFamily = "IBM Plex Mono";

/**
 * Loads the bundled font. The page calls it before it draws the first
 * diagram. The promise holds false when no face of the family loaded.
 *
 * `document.fonts.load` loads each face that the font shorthand matches and
 * whose unicode range holds a character of the text
 * (https://drafts.csswg.org/css-font-loading/#font-face-set-load).
 */
export async function loadDiagramFont(): Promise<boolean> {
  const faces = await document.fonts.load(`13px "${diagramFontFamily}"`, "↻ a");
  return faces.some((face) => face.status === "loaded");
}
