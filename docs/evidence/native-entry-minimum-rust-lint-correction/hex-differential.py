#!/usr/bin/env python3
"""Differential check for the `hex` and octal-prefix rewrites (k51).

Part 1 extracts `fn hex` from the base revision and from the working tree for
both files that define it, compiles them side by side with the named rustc, and
requires old and new to agree on every single byte value 0..=255, on the whole
0..=255 run, on the empty slice, and on a fixed pseudo-random set of lengths
1..=64.

Part 2 extracts, from `crates/ravel/tests/cli.rs`, the statements that compute
the length prefix and the `format!` that builds the fixture worker script, and
requires the old and new script text to be byte-identical for every `hello`
length 0..=1023 (all four prefix bytes exercised across their low byte) and for
the test's own `hello` text.

A deliberately mutated `new` (--mutate) must make both parts fail.

usage: hex-differential.py BASE_REV RUSTC [--mutate]   (run from the repo root)
"""
import re, subprocess, sys, tempfile, pathlib

base, rustc = sys.argv[1], sys.argv[2]
mutate = "--mutate" in sys.argv[3:]
FILES = ["crates/ravel/build.rs", "crates/ravel/src/context.rs"]


def fn_hex(text):
    m = re.search(r"^(?:pub )?fn hex\(bytes: &\[u8\]\) -> String \{\n.*?^\}\n", text, re.S | re.M)
    assert m, "fn hex not found"
    return re.sub(r"^pub ", "", m.group(0)).replace("fn hex(", "fn HEXNAME(")


def old_new(path):
    old = subprocess.run(["git", "show", f"{base}:{path}"], check=True, capture_output=True, text=True).stdout
    new = pathlib.Path(path).read_text()
    return fn_hex(old), fn_hex(new)


src = ["use std::fmt::Write as _;\n"]
for i, path in enumerate(FILES):
    o, n = old_new(path)
    if mutate and i == 1:
        n = n.replace("{byte:02x}", "{byte:02X}")
    src.append(o.replace("HEXNAME", f"old_{i}") + n.replace("HEXNAME", f"new_{i}"))
src.append("""
fn main() {
    let mut checked = 0u32;
    let mut fail = 0u32;
    let mut check = |bytes: &[u8]| {
        for (old, new) in [(old_0(bytes), new_0(bytes)), (old_1(bytes), new_1(bytes))] {
            checked += 1;
            if old != new { fail += 1; eprintln!("DIFF {:?}: {old:?} vs {new:?}", bytes); }
        }
    };
    for b in 0..=255u8 { check(&[b]); }
    let all: Vec<u8> = (0..=255u8).collect();
    check(&all);
    check(&[]);
    let mut x = 0x2545F4914F6CDD1Du64;
    for len in 1..=64usize {
        let v: Vec<u8> = (0..len).map(|_| { x ^= x << 13; x ^= x >> 7; x ^= x << 17; (x >> 24) as u8 }).collect();
        check(&v);
    }
    println!("checked {checked} comparisons, {fail} differ");
    std::process::exit(if fail == 0 { 0 } else { 1 });
}
""")
def cli_parts(text):
    hello = re.search(r'let hello = (r#".*?"#);', text, re.S).group(1)
    m = re.search(r"(    let length = .*?;\n    let prefix_bytes.*?;\n)    fs::write\(\n        &worker,\n        (format!\(.*?\)),\n    \)", text, re.S)
    assert m, "cli.rs fixture-script statements not found"
    return hello, m.group(1), m.group(2)


old_cli = subprocess.run(["git", "show", f"{base}:crates/ravel/tests/cli.rs"], check=True, capture_output=True, text=True).stdout
new_cli = pathlib.Path("crates/ravel/tests/cli.rs").read_text()
h_old, stmts_old, fmt_old = cli_parts(old_cli)
h_new, stmts_new, fmt_new = cli_parts(new_cli)
assert h_old == h_new, "the test's hello text changed"
if mutate:
    stmts_new = stmts_new.replace("{byte:03o}", "{byte:03x}")
src.append(f"""
fn script_old(hello: &str) -> String {{
{stmts_old}    {fmt_old}
}}
fn script_new(hello: &str) -> String {{
{stmts_new}    {fmt_new}
}}
fn cli_check() -> (u32, u32) {{
    let mut n = 0; let mut bad = 0;
    let mut one = |hello: &str| {{ n += 1; if script_old(hello) != script_new(hello) {{ bad += 1; }} }};
    one({h_old});
    for len in 0..=1023usize {{ one(&"x".repeat(len)); }}
    (n, bad)
}}
""")
src[-2] = src[-2].replace("    println!(\"checked", "    let (cn, cbad) = cli_check();\n    println!(\"cli.rs fixture script: {cn} comparisons, {cbad} differ\");\n    fail += cbad;\n    println!(\"checked")
with tempfile.TemporaryDirectory() as d:
    f = pathlib.Path(d) / "hexdiff.rs"
    f.write_text("".join(src))
    subprocess.run([rustc, "--edition=2021", "-o", str(pathlib.Path(d) / "hexdiff"), str(f)], check=True)
    r = subprocess.run([str(pathlib.Path(d) / "hexdiff")])
    sys.exit(r.returncode)
