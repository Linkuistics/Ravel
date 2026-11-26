---
name: coding-style-python
description: Python coding standards — Ruff, pyright strict, type hints, asyncio, uv, pytest. Use when writing or refactoring Python code.
harnesses: [any]
paths: "**/*.py"
---

# Python Coding Style Guidelines

**The repo's own configuration wins.** `pyproject.toml` (`[tool.ruff]`, `[tool.pyright]`, `[tool.pytest]`), `setup.cfg`, `.editorconfig`, and the lint flags CI actually runs are the authority wherever they exist. What follows is the default for a repo that has not decided.

## Formatting & Linting
- **Formatter & Linter:** Ruff (replaces black, isort, flake8)
- **Type checker:** pyright in strict mode for new code; mypy is acceptable in existing projects
- **Line length:** 100 characters
- **Imports:** sorted by Ruff (`I` rules); absolute imports preferred

## Type Hints
- Required on all public function signatures and class attributes
- Use `from __future__ import annotations` in libraries targeting Python <3.12
- Prefer `pathlib.Path` over `str` for filesystem paths
- Use `typing.Protocol` over abstract base classes for structural typing
- Use `dataclasses` (or `attrs`) for plain data; use `pydantic` only when validation is needed

## Error Handling
- Catch the narrowest exception class that applies; never `except:` or bare `except Exception:`
- Always chain: `raise NewError("...") from e` to preserve traceback context
- Define domain exceptions as a small hierarchy rooted at one project base class
- Do not use exceptions for control flow; return values, sentinels, or `Optional` instead

## Async Code
- Use `asyncio` with `async/await`
- Mix with threads only via `asyncio.to_thread` or `loop.run_in_executor`
- Always set explicit timeouts on network I/O (`asyncio.timeout`)
- Use `asyncio.TaskGroup` (3.11+) over manual `gather` for structured concurrency

## Dependencies
- **uv** for new projects; poetry / pip-tools acceptable in existing
- Lock file (`uv.lock`, `poetry.lock`) must be committed
- Pin direct dependencies; let the lock file pin transitives
- Use `pyproject.toml` exclusively; no `setup.py` or `requirements.txt` in new projects

## Testing
- **Write the test first.** A new behaviour or a bugfix starts as a failing test; implement until it passes.
- **pytest**, never `unittest`
- Use fixtures (not setUp/tearDown); prefer narrow function-scoped fixtures
- One assertion concept per test; use `pytest.mark.parametrize` for variants
- Mock with `pytest-mock` (`mocker` fixture) over bare `unittest.mock`
