//! The tests at the core interface, the first test seam of the slice.
//!
//! Each case is a grammar file `NAME.ebnf` with the expected checked grammar in
//! `NAME.checked.kdl` beside it. The cases live in `tests/cases` and in the
//! `examples` directory of the repository. Set `UPDATE_EXPECT=1` to rewrite
//! the expected files from the output, as the tests of `rust-analyzer` do
//! (https://github.com/rust-analyzer/expect-test).
//!
//! Each case also checks two properties. The syntax tree keeps every byte of
//! the grammar text. The KDL output survives a round trip through the Rust
//! types.
//!
//! The cases in `tests/cases/syntax` and `tests/cases/checks` each hold one
//! fault. The name of such a case starts with the code of its diagnostic. The
//! test checks that the case gives exactly one diagnostic with that code.
//!
//! The cases in `tests/cases/no-check` each come close to one check without a
//! fault. The name of such a case starts with the code of that check. The
//! cases in `tests/cases/valid`, `tests/cases/badges` and `tests/cases/no-check`
//! give no diagnostic.

use std::{
    fs,
    path::{Path, PathBuf},
};

use workbench_core::{check_grammar, checked::Code, checked_grammar, from_kdl, parse, to_kdl};

fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn grammar_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let mut dirs = vec![dir.to_path_buf()];
    while let Some(dir) = dirs.pop() {
        for entry in fs::read_dir(&dir).unwrap_or_else(|e| panic!("{}: {e}", dir.display())) {
            let path = entry.expect("a directory entry").path();
            if path.is_dir() {
                dirs.push(path);
            } else if path.extension().is_some_and(|e| e == "ebnf") {
                files.push(path);
            }
        }
    }
    files.sort();
    files
}

fn expected_path(grammar: &Path) -> PathBuf {
    grammar.with_extension("checked.kdl")
}

fn update_mode() -> bool {
    std::env::var_os("UPDATE_EXPECT").is_some()
}

/// Runs every case in the directories and reports every failure at once.
fn run_cases(dirs: &[PathBuf]) {
    let mut failures = Vec::new();
    let mut count = 0;
    for dir in dirs {
        for grammar in grammar_files(dir) {
            count += 1;
            if let Err(failure) = run_case(&grammar) {
                failures.push(format!("{}:\n{failure}", grammar.display()));
            }
        }
    }
    assert!(count > 0, "no grammar files in {dirs:?}");
    assert!(failures.is_empty(), "{} of {count} cases failed:\n\n{}", failures.len(), failures.join("\n\n"));
}

fn run_case(grammar: &Path) -> Result<(), String> {
    let text = fs::read_to_string(grammar).map_err(|e| e.to_string())?;

    let tree = parse(&text).syntax().to_string();
    if tree != text {
        return Err("the syntax tree does not keep every byte of the text".into());
    }

    let output = check_grammar(&text);
    let types = from_kdl(&output).map_err(|e| format!("the output does not read back: {e}\n{output}"))?;
    if to_kdl(&types) != output {
        return Err("the output changes in a round trip through the Rust types".into());
    }

    let parent = grammar.parent().and_then(Path::file_name).and_then(|n| n.to_str()).unwrap_or_default();
    match parent {
        "syntax" | "checks" => check_one_fault(grammar, &text)?,
        "valid" | "badges" | "no-check" => check_no_fault(&text)?,
        _ => {}
    }

    let expected_path = expected_path(grammar);
    let expected = fs::read_to_string(&expected_path).unwrap_or_default();
    if expected == output {
        return Ok(());
    }
    if update_mode() {
        fs::write(&expected_path, &output).map_err(|e| e.to_string())?;
        return Ok(());
    }
    if expected.is_empty() {
        return Err(format!("{} is missing; run `task test:update`", expected_path.display()));
    }
    Err(format!("the output differs from {}:\n{}", expected_path.display(), diff(&expected, &output)))
}

/// Checks that a case named `CODE.ebnf` or `CODE--DETAIL.ebnf` gives exactly
/// one diagnostic, with the code `CODE`.
fn check_one_fault(grammar: &Path, text: &str) -> Result<(), String> {
    let stem = grammar.file_stem().and_then(|s| s.to_str()).unwrap_or_default();
    let code_name = stem.split("--").next().unwrap_or_default();
    let code = Code::parse(code_name).ok_or_else(|| format!("`{code_name}` is not a diagnostic code"))?;
    let found: Vec<Code> = checked_grammar(text).diagnostics.iter().map(|d| d.code).collect();
    if found != [code] {
        return Err(format!("expected exactly one `{code}` diagnostic, found {found:?}"));
    }
    Ok(())
}

/// Checks that a grammar gives no diagnostic.
fn check_no_fault(text: &str) -> Result<(), String> {
    let found: Vec<Code> = checked_grammar(text).diagnostics.iter().map(|d| d.code).collect();
    if !found.is_empty() {
        return Err(format!("expected no diagnostic, found {found:?}"));
    }
    Ok(())
}

fn diff(expected: &str, actual: &str) -> String {
    let expected: Vec<&str> = expected.lines().collect();
    let actual: Vec<&str> = actual.lines().collect();
    let mut out = String::new();
    for i in 0..expected.len().max(actual.len()) {
        match (expected.get(i), actual.get(i)) {
            (Some(e), Some(a)) if e == a => {}
            (e, a) => {
                if let Some(e) = e {
                    out.push_str(&format!("{:>4} - {e}\n", i + 1));
                }
                if let Some(a) = a {
                    out.push_str(&format!("{:>4} + {a}\n", i + 1));
                }
            }
        }
    }
    out
}

#[test]
fn test_cases() {
    run_cases(&[manifest_dir().join("tests/cases")]);
}

#[test]
fn example_grammars() {
    let examples = manifest_dir().join("../../examples");
    run_cases(std::slice::from_ref(&examples));
    for grammar in grammar_files(&examples) {
        let text = fs::read_to_string(&grammar).expect("an example grammar");
        let checked = checked_grammar(&text);
        assert!(checked.diagnostics.is_empty(), "{}: {:?}", grammar.display(), checked.diagnostics);
    }
}

/// The codes that the names of the cases in one directory start with.
fn case_codes(dir: &str) -> Vec<String> {
    grammar_files(&manifest_dir().join("tests/cases").join(dir))
        .iter()
        .filter_map(|p| p.file_stem()?.to_str()?.split("--").next().map(str::to_string))
        .collect()
}

/// The codes in `codes` without a case in the directory.
fn codes_without_case(dir: &str, codes: impl Iterator<Item = Code>) -> Vec<&'static str> {
    let names = case_codes(dir);
    codes.map(Code::as_str).filter(|c| !names.iter().any(|n| n == c)).collect()
}

/// Each syntax error of the spec has at least one case of its own.
#[test]
fn every_syntax_error_has_a_case() {
    let missing = codes_without_case("syntax", Code::ALL.iter().copied().filter(|c| !c.is_check()));
    assert!(missing.is_empty(), "no case in tests/cases/syntax for {missing:?}");
}

/// Each check diagnostic of the spec has a case that triggers it, and a case
/// that comes close to it without a fault.
#[test]
fn every_check_has_cases() {
    let checks = || Code::ALL.iter().copied().filter(|c| c.is_check());
    let missing = codes_without_case("checks", checks());
    assert!(missing.is_empty(), "no case in tests/cases/checks for {missing:?}");
    let missing = codes_without_case("no-check", checks());
    assert!(missing.is_empty(), "no case in tests/cases/no-check for {missing:?}");
}

/// The badge cases of `rule-checks-k6`: right recursion, left recursion,
/// self-embedding and groups of several rules. No badge claims "not regular".
#[test]
fn badges() {
    let dir = manifest_dir().join("tests/cases/badges");
    let mut seen = Vec::new();
    for grammar in grammar_files(&dir) {
        let text = fs::read_to_string(&grammar).expect("a badge case");
        let stem = grammar.file_stem().and_then(|s| s.to_str()).unwrap_or_default().to_string();
        let expected = stem.split("--").next().unwrap_or_default().to_string();
        let groups = checked_grammar(&text).recursive_groups;
        assert!(!groups.is_empty(), "{stem}: no recursive group");
        for group in groups {
            assert_eq!(group.badge.as_str(), expected, "{stem}: {group:?}");
        }
        seen.push(stem);
    }
    for case in ["right-recursion", "left-recursion", "self-embedding", "several-rules"] {
        assert!(seen.iter().any(|s| s.contains(case)), "no badge case for {case}");
    }
}
