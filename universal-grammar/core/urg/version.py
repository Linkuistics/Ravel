"""The code version that every dependency record names.

A stage names the code version of its own module. That version covers the module and every
URG module that it imports, directly or through other URG modules. A change to a later stage
therefore leaves the research artifacts of earlier stages valid. A change to shared code, such
as the store, invalidates every stage that imports it.
"""

from __future__ import annotations

import ast
import hashlib
from functools import cache
from importlib.metadata import version
from pathlib import Path

PACKAGE_DIR = Path(__file__).resolve().parent
PACKAGE = "urg"


def _module_path(module: str) -> Path | None:
    parts = module.split(".")
    if parts[0] != PACKAGE:
        return None
    base = PACKAGE_DIR.joinpath(*parts[1:])
    for candidate in (base.with_suffix(".py"), base / "__init__.py"):
        if candidate.is_file():
            return candidate
    return None


def _imported_modules(path: Path) -> set[str]:
    """Return the URG modules that one source file imports, at any depth of the syntax tree."""
    found: set[str] = set()
    for node in ast.walk(ast.parse(path.read_bytes(), filename=str(path))):
        if isinstance(node, ast.Import):
            found.update(alias.name for alias in node.names)
        elif isinstance(node, ast.ImportFrom) and node.module and node.level == 0:
            found.add(node.module)
            found.update(f"{node.module}.{alias.name}" for alias in node.names)
    return {name for name in found if _module_path(name) is not None}


@cache
def module_closure(module: str) -> tuple[str, ...]:
    """Return the module and every URG module that it imports transitively, sorted by name."""
    seen: set[str] = set()
    pending = [module]
    while pending:
        name = pending.pop()
        path = _module_path(name)
        if name in seen or path is None:
            continue
        seen.add(name)
        pending.extend(_imported_modules(path))
    return tuple(sorted(seen))


@cache
def code_version(module: str) -> str:
    """Return the package version and the SHA-256 of the sources behind one stage module.

    The digest covers ``module`` and its transitive URG imports, including this module. The
    digest changes whenever that code changes, with or without a commit. A stage therefore never
    reuses a research artifact that older code produced.
    """
    if _module_path(module) is None:
        raise ValueError(f"{module} is not a module of the {PACKAGE} package")
    digest = hashlib.sha256()
    for name in (*module_closure(module), __name__):
        path = _module_path(name)
        assert path is not None
        digest.update(path.relative_to(PACKAGE_DIR).as_posix().encode())
        digest.update(b"\0")
        digest.update(path.read_bytes())
        digest.update(b"\0")
    return f"{PACKAGE} {version(PACKAGE)} source-sha256:{digest.hexdigest()}"
