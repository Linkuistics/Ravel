---
name: coding-style-rust
description: Rust coding standards — rustfmt settings, thiserror/anyhow error handling, Tokio async, test-first testing, workspace dependencies. Use when writing or refactoring Rust code.
harnesses: [any]
paths: "**/*.rs"
---

# Rust Coding Style Guidelines

**The repo's own configuration wins.** `rustfmt.toml`, `clippy.toml`, the workspace `Cargo.toml`, `.editorconfig`, and the lint flags CI actually runs are the authority wherever they exist. What follows is the default for a repo that has not decided.

## Formatting (rustfmt)
- **Edition:** 2021
- **Import Grouping:** Group imports as StdExternalCrate (stdlib, external crates, then local)
- **Import Granularity:** Crate level (merge imports from the same crate)
- **Field Init Shorthand:** Always use field init shorthand when variable name matches field name

### Function Parameters
- **Maximum arguments:** 20 parameters
- If approaching this limit, consider refactoring into a configuration struct

## Error Handling
- Use `thiserror` (`#[derive(Error)]`) for library error types; `anyhow` for application-level handling
- Return `Result` for fallible functions; avoid `unwrap`/`expect`/`panic!` in production code
- Propagate with the `?` operator
- Make error messages descriptive and actionable, carrying enough context to debug

## Async Code
- Use Tokio runtime
- Prefer bounded channels over unbounded
- Avoid blocking operations in async contexts

## Testing
- **Write the test first.** A new behaviour or a bugfix starts as a failing test; implement until it passes.
- Unit tests in a `#[cfg(test)] mod tests` beside the code; integration tests under `tests/`

## Dependencies
- Use workspace dependency versions to maintain consistency
- Justify any deviation from workspace versions
- Avoid unnecessary dependencies; prefer standard library and existing workspace dependencies when possible
