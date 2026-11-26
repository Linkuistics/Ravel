"""Write the table of label cells for the core: crates/core/src/cells_table.rs.

The spec `docs/specs/rule-diagrams.md`, section "Labels and the font", gives the
rule. A combining mark takes no cell, and a character with the East Asian Width
`W` or `F` takes two cells. Every other character takes one cell. A combining
mark is a character of the General Category `M`: `Mn`, `Mc` or `Me`
(https://www.unicode.org/glossary/#combining_character).

The data comes from the module `unicodedata` of Python. The file names its
version of Unicode. Run `task core:cells-table` to write the file again.
"""

import sys
import unicodedata
from pathlib import Path


def cells(cp):
    ch = chr(cp)
    if unicodedata.category(ch).startswith("M"):
        return 0
    if unicodedata.east_asian_width(ch) in ("W", "F"):
        return 2
    return 1


def ranges():
    out = []
    start, value = None, None
    for cp in range(0x110000):
        if 0xD800 <= cp <= 0xDFFF:
            c = 1
        else:
            c = cells(cp)
        if c != value:
            if value is not None and value != 1:
                out.append((start, cp - 1, value))
            start, value = cp, c
    if value != 1:
        out.append((start, 0x10FFFF, value))
    return out


def main():
    out = Path(sys.argv[1])
    rows = ranges()
    lines = [
        "//! The characters that do not take one cell of the label font. The script",
        "//! `crates/core/scripts/cells-table.py` writes this file. Do not edit it by hand.",
        "",
        "/// The version of Unicode of the table.",
        f'pub const UNICODE_VERSION: &str = "{unicodedata.unidata_version}";',
        "",
        "/// Ranges of code points, in order, that take 0 or 2 cells. Each other",
        "/// character takes 1 cell.",
        "#[rustfmt::skip]",
        "pub(crate) const RANGES: &[(u32, u32, u8)] = &[",
    ]
    for lo, hi, c in rows:
        lines.append(f"    (0x{lo:04X}, 0x{hi:04X}, {c}),")
    lines.append("];")
    out.write_text("\n".join(lines) + "\n")
    print(f"{len(rows)} ranges, Unicode {unicodedata.unidata_version}")


if __name__ == "__main__":
    main()
