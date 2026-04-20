# Hierarchical Pivot Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make Ravel-Lite's phase loop stack-aware so a plan's work phase can pivot to a child plan by appending a frame to `<root>/stack.yaml`, run the child's full cycle, then pop back and resume. Wire the existing `ravel-orchestrator` plan as the first coordinator.

**Architecture:** Two repos touched.

- **Ravel-Lite** (`/Users/antony/Development/Ravel-Lite`): new `src/pivot.rs` (stack.yaml schema + read/write/validate), refactored `src/phase_loop.rs` (stack-aware driver with two pure decision functions + a `Vec<PlanContext>` held in `run_stack`), small additions in `src/types.rs`, `src/ui.rs`, `src/main.rs`.
- **Ravel** (this repo, `/Users/antony/Development/Ravel`): rewritten `LLM_STATE/ravel-orchestrator/prompt-work.md` (the routing prompt); seeded Gates section in `LLM_STATE/ravel-orchestrator/memory.md`.

**Tech Stack:** Rust + tokio + serde_yaml (already in `Cargo.toml`). Integration tests use `tempfile::TempDir` + `MockAgent` pattern already established in `tests/integration.rs`. TDD throughout.

**Design spec:** `docs/superpowers/specs/2026-04-20-hierarchical-pivot-design.md` — consult before any task.

---

## File layout (Ravel-Lite)

```
ravel-lite/
├── src/
│   ├── pivot.rs           # NEW: Frame, Stack, read/write/validate,
│   │                      #      decide_after_work, decide_after_cycle
│   ├── types.rs           # MODIFIED: pub use pivot::MAX_STACK_DEPTH
│   ├── phase_loop.rs      # MODIFIED: phase_loop → run_stack; stack-aware
│   ├── ui.rs              # MODIFIED: breadcrumb in phase header, push/pop logs
│   ├── main.rs            # MODIFIED: call run_stack; handle startup resume
│   └── lib.rs             # MODIFIED: pub mod pivot
└── tests/
    └── integration.rs     # MODIFIED: add stack-aware MockAgent tests
```

Responsibilities:
- `pivot.rs` — all stack mechanics: types, serde, validation, decision functions. No I/O beyond read/write of a single file. No tokio. Pure domain module with small I/O surface. Unit-testable in isolation.
- `phase_loop.rs` — the async driver. Holds the `Vec<PlanContext>` stack in memory, delegates decisions to `pivot.rs`, handles `tokio` concerns (agent calls, UI sends).
- `ui.rs` changes are small: a new optional parameter to the phase-header helper, two new log-line formats.
- `main.rs` change is one-liner swap: call `run_stack` instead of `phase_loop`.

---

## File layout (Ravel coordinator)

```
Ravel/
└── LLM_STATE/
    └── ravel-orchestrator/
        ├── prompt-work.md    # REWRITTEN: routing prompt
        └── memory.md         # MODIFIED: Gates section seeded
```

Existing `prompt-analyse-work.md`, `prompt-reflect.md`, `prompt-triage.md` are not authored — Ravel-Lite's defaults apply.

---

## Task 1: Stack types + serde

**Files:**
- Create: `/Users/antony/Development/Ravel-Lite/src/pivot.rs`
- Modify: `/Users/antony/Development/Ravel-Lite/src/lib.rs`

Establishes the wire format and the in-memory types. No I/O, no logic beyond (de)serialization. This is the foundation every later task depends on.

- [ ] **Step 1: Write the failing tests**

Append to `/Users/antony/Development/Ravel-Lite/tests/integration.rs` (at the end of the file):

```rust
// ===================== pivot: types + serde =====================

#[test]
fn pivot_stack_roundtrip_minimal_frame() {
    use ravel_lite::pivot::{Frame, Stack};
    use std::path::PathBuf;

    let s = Stack {
        frames: vec![Frame {
            path: PathBuf::from("LLM_STATE/ravel-orchestrator"),
            pushed_at: None,
            reason: None,
        }],
    };
    let yaml = serde_yaml::to_string(&s).unwrap();
    let back: Stack = serde_yaml::from_str(&yaml).unwrap();
    assert_eq!(s, back);
    // Minimal frames omit pushed_at/reason when serialized
    assert!(!yaml.contains("pushed_at"));
    assert!(!yaml.contains("reason"));
}

#[test]
fn pivot_stack_roundtrip_full_frame() {
    use ravel_lite::pivot::{Frame, Stack};
    use std::path::PathBuf;

    let s = Stack {
        frames: vec![
            Frame {
                path: PathBuf::from("LLM_STATE/ravel-orchestrator"),
                pushed_at: None,
                reason: None,
            },
            Frame {
                path: PathBuf::from("LLM_STATE/sub-F-hierarchy"),
                pushed_at: Some("2026-04-20T18:32:14Z".to_string()),
                reason: Some("sub-D/T2 landed".to_string()),
            },
        ],
    };
    let yaml = serde_yaml::to_string(&s).unwrap();
    let back: Stack = serde_yaml::from_str(&yaml).unwrap();
    assert_eq!(s, back);
}

#[test]
fn pivot_max_stack_depth_constant() {
    assert_eq!(ravel_lite::pivot::MAX_STACK_DEPTH, 5);
}
```

- [ ] **Step 2: Run tests to confirm they fail**

```bash
cd /Users/antony/Development/Ravel-Lite
cargo test pivot_ --lib --tests 2>&1 | head -30
```

Expected: compile error — `pivot` module does not exist.

- [ ] **Step 3: Create src/pivot.rs**

```rust
//! Stack-based pivot mechanism for nested plan execution.
//!
//! When a plan's work phase appends a frame to `<root>/stack.yaml`,
//! the driver pushes the new top onto an in-memory `Vec<PlanContext>`
//! and runs a full cycle of the target plan before popping back.
//!
//! This module owns:
//! - `Frame` / `Stack` — the on-disk schema.
//! - `read_stack` / `write_stack` — file I/O.
//! - `validate_push` — depth cap + cycle detection + target validity.
//! - `decide_after_work` / `decide_after_cycle` — pure state transitions.
//!
//! No tokio, no async. The async driver in `phase_loop.rs` orchestrates.
//! See docs/superpowers/specs/2026-04-20-hierarchical-pivot-design.md.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Hard compile-time cap on nesting depth. Prevents runaway recursion
/// from a buggy coordinator prompt.
pub const MAX_STACK_DEPTH: usize = 5;

/// One entry in `stack.yaml`. Path is the only required field; `pushed_at`
/// and `reason` are informational and appear in the TUI / session log.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Frame {
    pub path: PathBuf,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pushed_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// The on-disk stack. Ordered; `frames.last()` is the currently-executing plan.
/// A stack with `len <= 1` means "just the root"; the file is normally
/// deleted in that state.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Stack {
    #[serde(default)]
    pub frames: Vec<Frame>,
}
```

- [ ] **Step 4: Wire the module in lib.rs**

Open `/Users/antony/Development/Ravel-Lite/src/lib.rs`. Add `pub mod pivot;` alphabetically with the other `pub mod` lines. (Existing lib.rs has lines like `pub mod agent;`, `pub mod config;`, etc. Insert `pub mod pivot;` after `pub mod phase_loop;`.)

- [ ] **Step 5: Run tests to confirm they pass**

```bash
cd /Users/antony/Development/Ravel-Lite
cargo test pivot_ --lib --tests 2>&1 | tail -20
```

Expected: all three tests pass.

- [ ] **Step 6: Commit**

```bash
cd /Users/antony/Development/Ravel-Lite
git add src/pivot.rs src/lib.rs tests/integration.rs
git commit -m "pivot: Frame, Stack types with serde roundtrip + MAX_STACK_DEPTH"
```

---

## Task 2: stack.yaml read/write

**Files:**
- Modify: `/Users/antony/Development/Ravel-Lite/src/pivot.rs`
- Modify: `/Users/antony/Development/Ravel-Lite/tests/integration.rs`

Adds `read_stack` (returns `Option<Stack>` — `None` if file absent) and `write_stack` (serializes and writes). Deletion handled inline at call sites via `std::fs::remove_file`; no helper needed.

- [ ] **Step 1: Write the failing tests**

Append to `tests/integration.rs`:

```rust
#[test]
fn pivot_read_stack_missing_returns_none() {
    use ravel_lite::pivot::read_stack;
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("stack.yaml");
    assert!(read_stack(&path).unwrap().is_none());
}

#[test]
fn pivot_read_stack_present_returns_some() {
    use ravel_lite::pivot::{read_stack, write_stack, Frame, Stack};
    use std::path::PathBuf;

    let dir = TempDir::new().unwrap();
    let path = dir.path().join("stack.yaml");
    let s = Stack {
        frames: vec![Frame {
            path: PathBuf::from("LLM_STATE/x"),
            pushed_at: None,
            reason: None,
        }],
    };
    write_stack(&path, &s).unwrap();
    let back = read_stack(&path).unwrap().unwrap();
    assert_eq!(back, s);
}

#[test]
fn pivot_read_stack_corrupt_yaml_returns_error_with_context() {
    use ravel_lite::pivot::read_stack;
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("stack.yaml");
    fs::write(&path, "not: valid: yaml: at: all:\n  - [\n").unwrap();

    let err = read_stack(&path).unwrap_err();
    let msg = format!("{err:#}");
    assert!(msg.contains("stack.yaml"), "error should mention the file: {msg}");
}

#[test]
fn pivot_write_stack_creates_parent_if_needed() {
    use ravel_lite::pivot::{write_stack, Stack};
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("some/nested/stack.yaml");
    // Parent doesn't exist; write should succeed by creating parents.
    write_stack(&path, &Stack::default()).unwrap();
    assert!(path.exists());
}
```

- [ ] **Step 2: Run tests to confirm they fail**

```bash
cd /Users/antony/Development/Ravel-Lite
cargo test pivot_read_stack pivot_write_stack --lib --tests 2>&1 | head -30
```

Expected: compile error — `read_stack` / `write_stack` undefined.

- [ ] **Step 3: Implement read_stack + write_stack**

Append to `src/pivot.rs`:

```rust
use std::fs;
use std::io;
use std::path::Path;

use anyhow::{Context, Result};

/// Read `stack.yaml` from `path`. Returns `Ok(None)` if the file is absent
/// (normal state at depth 1), `Ok(Some(stack))` on successful parse, or an
/// error that includes the path for diagnosability.
pub fn read_stack(path: &Path) -> Result<Option<Stack>> {
    match fs::read_to_string(path) {
        Ok(s) => {
            let stack: Stack = serde_yaml::from_str(&s)
                .with_context(|| format!("Failed to parse stack.yaml at {}", path.display()))?;
            Ok(Some(stack))
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(e) => {
            Err(e).with_context(|| format!("Failed to read stack.yaml at {}", path.display()))
        }
    }
}

/// Write `stack` to `path`, creating parent directories as needed.
pub fn write_stack(path: &Path, stack: &Stack) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create parent for {}", path.display()))?;
    }
    let s = serde_yaml::to_string(stack)
        .context("Failed to serialize Stack")?;
    fs::write(path, s)
        .with_context(|| format!("Failed to write stack.yaml at {}", path.display()))
}
```

- [ ] **Step 4: Run tests to confirm they pass**

```bash
cd /Users/antony/Development/Ravel-Lite
cargo test pivot_read_stack pivot_write_stack --lib --tests 2>&1 | tail -15
```

Expected: all four tests pass.

- [ ] **Step 5: Commit**

```bash
cd /Users/antony/Development/Ravel-Lite
git add src/pivot.rs tests/integration.rs
git commit -m "pivot: read_stack / write_stack with path-aware error context"
```

---

## Task 3: Push validation (depth cap, cycle, invalid target)

**Files:**
- Modify: `/Users/antony/Development/Ravel-Lite/src/pivot.rs`
- Modify: `/Users/antony/Development/Ravel-Lite/tests/integration.rs`

Pre-push check. The invalid-target check must see the real filesystem, so tests use `TempDir` fixtures.

- [ ] **Step 1: Write the failing tests**

Append to `tests/integration.rs`:

```rust
fn _make_plan_dir(dir: &std::path::Path, name: &str) -> std::path::PathBuf {
    let p = dir.join(name);
    fs::create_dir_all(&p).unwrap();
    fs::write(p.join("phase.md"), "work\n").unwrap();
    p
}

#[test]
fn pivot_validate_push_accepts_valid_target_below_cap() {
    use ravel_lite::pivot::{validate_push, Frame, Stack};
    let tmp = TempDir::new().unwrap();
    let root = _make_plan_dir(tmp.path(), "root");
    let child = _make_plan_dir(tmp.path(), "child");

    let stack = Stack {
        frames: vec![Frame {
            path: root.clone(),
            pushed_at: None,
            reason: None,
        }],
    };
    let new_frame = Frame {
        path: child,
        pushed_at: None,
        reason: None,
    };
    validate_push(&stack, &new_frame).unwrap();
}

#[test]
fn pivot_validate_push_rejects_depth_cap() {
    use ravel_lite::pivot::{validate_push, Frame, Stack, MAX_STACK_DEPTH};
    let tmp = TempDir::new().unwrap();

    let mut frames = vec![];
    for i in 0..MAX_STACK_DEPTH {
        frames.push(Frame {
            path: _make_plan_dir(tmp.path(), &format!("p{i}")),
            pushed_at: None,
            reason: None,
        });
    }
    let stack = Stack { frames };
    let new_frame = Frame {
        path: _make_plan_dir(tmp.path(), "overflow"),
        pushed_at: None,
        reason: None,
    };
    let err = validate_push(&stack, &new_frame).unwrap_err();
    assert!(format!("{err:#}").to_lowercase().contains("depth"));
}

#[test]
fn pivot_validate_push_rejects_cycle() {
    use ravel_lite::pivot::{validate_push, Frame, Stack};
    let tmp = TempDir::new().unwrap();
    let a = _make_plan_dir(tmp.path(), "a");
    let b = _make_plan_dir(tmp.path(), "b");

    let stack = Stack {
        frames: vec![
            Frame { path: a.clone(), pushed_at: None, reason: None },
            Frame { path: b, pushed_at: None, reason: None },
        ],
    };
    let dup = Frame { path: a, pushed_at: None, reason: None };
    let err = validate_push(&stack, &dup).unwrap_err();
    assert!(format!("{err:#}").to_lowercase().contains("cycle"));
}

#[test]
fn pivot_validate_push_rejects_nonexistent_path() {
    use ravel_lite::pivot::{validate_push, Frame, Stack};
    let tmp = TempDir::new().unwrap();
    let root = _make_plan_dir(tmp.path(), "root");

    let stack = Stack {
        frames: vec![Frame { path: root, pushed_at: None, reason: None }],
    };
    let bogus = Frame {
        path: tmp.path().join("does_not_exist"),
        pushed_at: None,
        reason: None,
    };
    let err = validate_push(&stack, &bogus).unwrap_err();
    assert!(format!("{err:#}").to_lowercase().contains("does not exist")
        || format!("{err:#}").to_lowercase().contains("invalid pivot"));
}

#[test]
fn pivot_validate_push_rejects_path_without_phase_md() {
    use ravel_lite::pivot::{validate_push, Frame, Stack};
    let tmp = TempDir::new().unwrap();
    let root = _make_plan_dir(tmp.path(), "root");

    // Create a directory that is NOT a plan (no phase.md).
    let fake = tmp.path().join("not_a_plan");
    fs::create_dir_all(&fake).unwrap();

    let stack = Stack {
        frames: vec![Frame { path: root, pushed_at: None, reason: None }],
    };
    let bogus = Frame { path: fake, pushed_at: None, reason: None };
    let err = validate_push(&stack, &bogus).unwrap_err();
    assert!(format!("{err:#}").to_lowercase().contains("phase.md")
        || format!("{err:#}").to_lowercase().contains("invalid pivot"));
}
```

- [ ] **Step 2: Run tests to confirm they fail**

```bash
cd /Users/antony/Development/Ravel-Lite
cargo test pivot_validate_push --lib --tests 2>&1 | head -20
```

Expected: compile error — `validate_push` undefined.

- [ ] **Step 3: Implement validate_push**

Append to `src/pivot.rs`:

```rust
use anyhow::bail;

/// Reject a push attempt that would:
/// - exceed `MAX_STACK_DEPTH` (runaway recursion guard)
/// - revisit an already-stacked plan (cycle)
/// - point at a path that isn't a valid plan directory
///
/// Path validation uses the real filesystem (existence + `phase.md` presence).
/// Returns `Err` with a one-line diagnostic on any violation.
pub fn validate_push(stack: &Stack, new_frame: &Frame) -> Result<()> {
    if stack.frames.len() >= MAX_STACK_DEPTH {
        bail!(
            "Max stack depth {} exceeded; attempted push target: {}",
            MAX_STACK_DEPTH,
            new_frame.path.display()
        );
    }
    for existing in &stack.frames {
        if existing.path == new_frame.path {
            bail!(
                "Cycle detected: {} already in stack",
                new_frame.path.display()
            );
        }
    }
    if !new_frame.path.is_dir() {
        bail!("Invalid pivot target: {} does not exist or is not a directory", new_frame.path.display());
    }
    let phase_file = new_frame.path.join("phase.md");
    if !phase_file.exists() {
        bail!(
            "Invalid pivot target: {} has no phase.md",
            new_frame.path.display()
        );
    }
    Ok(())
}
```

- [ ] **Step 4: Run tests to confirm they pass**

```bash
cd /Users/antony/Development/Ravel-Lite
cargo test pivot_validate_push --lib --tests 2>&1 | tail -20
```

Expected: all five validation tests pass.

- [ ] **Step 5: Commit**

```bash
cd /Users/antony/Development/Ravel-Lite
git add src/pivot.rs tests/integration.rs
git commit -m "pivot: validate_push — depth cap, cycle detection, target validity"
```

---

## Task 4: Post-work decision function

**Files:**
- Modify: `/Users/antony/Development/Ravel-Lite/src/pivot.rs`
- Modify: `/Users/antony/Development/Ravel-Lite/tests/integration.rs`

A pure function that, given the observed state after the work phase ends, returns what the driver should do next. Four cases, covering the design spec's state-transition table.

- [ ] **Step 1: Write the failing tests**

Append to `tests/integration.rs`:

```rust
#[test]
fn pivot_decide_after_work_normal_cycle() {
    use ravel_lite::pivot::{decide_after_work, NextAfterWork};
    use ravel_lite::types::LlmPhase;

    let action = decide_after_work(LlmPhase::AnalyseWork, false, None);
    assert!(matches!(action, NextAfterWork::ContinueNormalCycle));
}

#[test]
fn pivot_decide_after_work_stateful_pivot() {
    use ravel_lite::pivot::{decide_after_work, Frame, NextAfterWork};
    use ravel_lite::types::LlmPhase;
    use std::path::PathBuf;

    let frame = Frame {
        path: PathBuf::from("/some/plan"),
        pushed_at: None,
        reason: None,
    };
    let action = decide_after_work(LlmPhase::AnalyseWork, true, Some(frame.clone()));
    match action {
        NextAfterWork::PushAfterCycle(f) => assert_eq!(f, frame),
        _ => panic!("expected PushAfterCycle"),
    }
}

#[test]
fn pivot_decide_after_work_stateless_short_circuit() {
    use ravel_lite::pivot::{decide_after_work, Frame, NextAfterWork};
    use ravel_lite::types::LlmPhase;
    use std::path::PathBuf;

    let frame = Frame {
        path: PathBuf::from("/some/plan"),
        pushed_at: None,
        reason: None,
    };
    let action = decide_after_work(LlmPhase::Work, true, Some(frame.clone()));
    match action {
        NextAfterWork::PushImmediately(f) => assert_eq!(f, frame),
        _ => panic!("expected PushImmediately"),
    }
}

#[test]
fn pivot_decide_after_work_no_advance_no_pivot_is_error() {
    use ravel_lite::pivot::{decide_after_work, NextAfterWork};
    use ravel_lite::types::LlmPhase;

    let action = decide_after_work(LlmPhase::Work, false, None);
    match action {
        NextAfterWork::Error(msg) => {
            assert!(msg.to_lowercase().contains("phase did not advance"));
        }
        _ => panic!("expected Error"),
    }
}
```

- [ ] **Step 2: Run tests to confirm they fail**

```bash
cd /Users/antony/Development/Ravel-Lite
cargo test pivot_decide_after_work --lib --tests 2>&1 | head -20
```

Expected: compile error — `decide_after_work` undefined.

- [ ] **Step 3: Implement decide_after_work**

Append to `src/pivot.rs`:

```rust
use crate::types::LlmPhase;

/// What the driver should do after a work phase finishes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NextAfterWork {
    /// Normal case: agent advanced phase.md and did not touch stack.yaml
    /// (or at least did not add a new top). Run analyse-work next.
    ContinueNormalCycle,
    /// Agent advanced phase.md AND added a new top. Run the full
    /// coordinator cycle; push happens at end of git-commit-triage.
    PushAfterCycle(Frame),
    /// Agent left phase.md at `work` AND added a new top. Push immediately
    /// and run the child's cycle; coordinator's remaining phases are
    /// skipped for this cycle.
    PushImmediately(Frame),
    /// Phase did not advance and no pivot was requested. Existing
    /// "phase did not advance" error case.
    Error(String),
}

/// Decide what to do after a work phase ends.
///
/// - `phase_after`: the LLM phase read from `phase.md` after work exits.
/// - `stack_grew`: whether `stack.yaml` gained a new top frame during work.
/// - `new_top`: the newly-added top frame, if `stack_grew` is true.
///
/// Caller is responsible for validating `new_top` via `validate_push`
/// before acting on `PushImmediately` / `PushAfterCycle`.
pub fn decide_after_work(
    phase_after: LlmPhase,
    stack_grew: bool,
    new_top: Option<Frame>,
) -> NextAfterWork {
    match (phase_after == LlmPhase::Work, stack_grew) {
        (true, true) => match new_top {
            Some(f) => NextAfterWork::PushImmediately(f),
            None => NextAfterWork::Error(
                "stack_grew=true but new_top=None — caller bug".into(),
            ),
        },
        (true, false) => NextAfterWork::Error(
            "phase did not advance and no pivot was requested".into(),
        ),
        (false, true) => match new_top {
            Some(f) => NextAfterWork::PushAfterCycle(f),
            None => NextAfterWork::Error(
                "stack_grew=true but new_top=None — caller bug".into(),
            ),
        },
        (false, false) => NextAfterWork::ContinueNormalCycle,
    }
}
```

- [ ] **Step 4: Run tests to confirm they pass**

```bash
cd /Users/antony/Development/Ravel-Lite
cargo test pivot_decide_after_work --lib --tests 2>&1 | tail -15
```

Expected: all four decision tests pass.

- [ ] **Step 5: Commit**

```bash
cd /Users/antony/Development/Ravel-Lite
git add src/pivot.rs tests/integration.rs
git commit -m "pivot: decide_after_work — four-case state table"
```

---

## Task 5: End-of-cycle decision function

**Files:**
- Modify: `/Users/antony/Development/Ravel-Lite/src/pivot.rs`
- Modify: `/Users/antony/Development/Ravel-Lite/tests/integration.rs`

The second pure decision. After `git-commit-triage` finishes for any plan, what next? Three cases: stateful-pivot push, pop, or continue root's loop.

- [ ] **Step 1: Write the failing tests**

Append to `tests/integration.rs`:

```rust
#[test]
fn pivot_decide_after_cycle_stateful_pivot_push() {
    use ravel_lite::pivot::{decide_after_cycle, Frame, NextAfterCycle};
    use std::path::PathBuf;

    let frame = Frame {
        path: PathBuf::from("/x"),
        pushed_at: None,
        reason: None,
    };
    let action = decide_after_cycle(1, true, Some(frame.clone()));
    match action {
        NextAfterCycle::Push(f) => assert_eq!(f, frame),
        _ => panic!("expected Push"),
    }
}

#[test]
fn pivot_decide_after_cycle_pop_when_nested() {
    use ravel_lite::pivot::{decide_after_cycle, NextAfterCycle};

    let action = decide_after_cycle(2, false, None);
    assert!(matches!(action, NextAfterCycle::Pop));
}

#[test]
fn pivot_decide_after_cycle_continue_at_root() {
    use ravel_lite::pivot::{decide_after_cycle, NextAfterCycle};

    let action = decide_after_cycle(1, false, None);
    assert!(matches!(action, NextAfterCycle::Continue));
}

#[test]
fn pivot_decide_after_cycle_push_takes_precedence_over_pop() {
    use ravel_lite::pivot::{decide_after_cycle, Frame, NextAfterCycle};
    use std::path::PathBuf;

    let frame = Frame {
        path: PathBuf::from("/deeper"),
        pushed_at: None,
        reason: None,
    };
    // A nested plan wrote a new top during its own cycle: go deeper,
    // don't pop. (A plan that both writes a new top AND wants to be
    // popped is ill-formed; push wins.)
    let action = decide_after_cycle(2, true, Some(frame.clone()));
    match action {
        NextAfterCycle::Push(f) => assert_eq!(f, frame),
        _ => panic!("expected Push to take precedence"),
    }
}
```

- [ ] **Step 2: Run tests to confirm they fail**

```bash
cd /Users/antony/Development/Ravel-Lite
cargo test pivot_decide_after_cycle --lib --tests 2>&1 | head -20
```

Expected: compile error — `decide_after_cycle` undefined.

- [ ] **Step 3: Implement decide_after_cycle**

Append to `src/pivot.rs`:

```rust
/// What the driver should do after a plan's full cycle ends
/// (post-git-commit-triage). Called at every cycle boundary; handles
/// the three terminal cases of a cycle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NextAfterCycle {
    /// Continue the current (root) plan's loop at its next work phase.
    /// Only valid at depth 1.
    Continue,
    /// A new top was added during this cycle — stateful pivot takes
    /// effect now. Push the frame and enter its plan's cycle.
    Push(Frame),
    /// Current plan is nested and had nothing new to push; pop to
    /// parent and resume its next phase.
    Pop,
}

/// Decide what to do after `git-commit-triage` completes for the
/// currently-executing plan.
///
/// - `current_depth`: stack depth *including* the currently-executing plan.
///   Depth 1 means "root only".
/// - `stack_grew`: whether a new top was added during this plan's cycle.
/// - `new_top`: the newly-added frame, if any.
pub fn decide_after_cycle(
    current_depth: usize,
    stack_grew: bool,
    new_top: Option<Frame>,
) -> NextAfterCycle {
    if stack_grew {
        if let Some(f) = new_top {
            return NextAfterCycle::Push(f);
        }
    }
    if current_depth > 1 {
        NextAfterCycle::Pop
    } else {
        NextAfterCycle::Continue
    }
}
```

- [ ] **Step 4: Run tests to confirm they pass**

```bash
cd /Users/antony/Development/Ravel-Lite
cargo test pivot_decide_after_cycle --lib --tests 2>&1 | tail -15
```

Expected: all four tests pass.

- [ ] **Step 5: Commit**

```bash
cd /Users/antony/Development/Ravel-Lite
git add src/pivot.rs tests/integration.rs
git commit -m "pivot: decide_after_cycle — push/pop/continue state machine"
```

---

## Task 6: Frame → PlanContext reconstruction

**Files:**
- Modify: `/Users/antony/Development/Ravel-Lite/src/pivot.rs`
- Modify: `/Users/antony/Development/Ravel-Lite/tests/integration.rs`

When a frame is read from `stack.yaml` (either on startup-resume or at push time), the driver needs a `PlanContext` for it. Reuses Ravel-Lite's existing `find_project_root` logic so each pushed plan carries accurate context.

- [ ] **Step 1: Write the failing tests**

Append to `tests/integration.rs`:

```rust
#[test]
fn pivot_frame_to_context_resolves_project_dir_by_walkup() {
    use ravel_lite::pivot::{frame_to_context, Frame};
    use std::path::PathBuf;

    let tmp = TempDir::new().unwrap();
    let root = tmp.path();
    // Set up: root/.git, root/LLM_STATE/sub-F/
    fs::create_dir_all(root.join(".git")).unwrap();
    let plan = root.join("LLM_STATE").join("sub-F");
    fs::create_dir_all(&plan).unwrap();
    fs::write(plan.join("phase.md"), "work\n").unwrap();

    let frame = Frame {
        path: plan.clone(),
        pushed_at: None,
        reason: None,
    };
    let root_config_root = "/tmp/cfg".to_string();
    let ctx = frame_to_context(&frame, &root_config_root).unwrap();

    assert_eq!(PathBuf::from(&ctx.plan_dir), plan);
    assert_eq!(PathBuf::from(&ctx.project_dir), root);
    assert_eq!(ctx.config_root, root_config_root);
    assert!(PathBuf::from(&ctx.dev_root) == root.parent().unwrap());
}

#[test]
fn pivot_frame_to_context_errors_without_git_root() {
    use ravel_lite::pivot::{frame_to_context, Frame};

    let tmp = TempDir::new().unwrap();
    let plan = tmp.path().join("lonely_plan");
    fs::create_dir_all(&plan).unwrap();
    fs::write(plan.join("phase.md"), "work\n").unwrap();

    let frame = Frame { path: plan, pushed_at: None, reason: None };
    let res = frame_to_context(&frame, "/tmp/cfg");
    assert!(res.is_err());
}
```

- [ ] **Step 2: Run tests to confirm they fail**

```bash
cd /Users/antony/Development/Ravel-Lite
cargo test pivot_frame_to_context --lib --tests 2>&1 | head -20
```

Expected: compile error — `frame_to_context` undefined.

- [ ] **Step 3: Implement frame_to_context**

Append to `src/pivot.rs`:

```rust
use crate::types::PlanContext;

/// Walk upwards from `plan_dir` looking for a `.git` directory. Returns
/// the ancestor directory that contains `.git`, or an error if none found.
fn find_project_root(plan_dir: &Path) -> Result<PathBuf> {
    let mut cur = plan_dir.canonicalize()
        .with_context(|| format!("Failed to canonicalize {}", plan_dir.display()))?;
    loop {
        if cur.join(".git").exists() {
            return Ok(cur);
        }
        match cur.parent() {
            Some(p) => cur = p.to_path_buf(),
            None => bail!("No .git ancestor found for plan {}", plan_dir.display()),
        }
    }
}

/// Build a `PlanContext` for a pivoted plan, reusing the root's config_root.
/// Looks up the target's project_dir by walking up for a `.git` ancestor,
/// and derives dev_root as the project's parent directory.
///
/// `related_plans` is read from `<plan_dir>/related-plans.md` if present,
/// matching the behaviour of `main::run_phase_loop` at startup.
pub fn frame_to_context(frame: &Frame, config_root: &str) -> Result<PlanContext> {
    let plan_dir = frame.path.canonicalize()
        .with_context(|| format!("Failed to canonicalize frame path {}", frame.path.display()))?;

    let project_dir = find_project_root(&plan_dir)?;
    let dev_root = project_dir
        .parent()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();

    let related_plans = std::fs::read_to_string(plan_dir.join("related-plans.md"))
        .unwrap_or_default();

    Ok(PlanContext {
        plan_dir: plan_dir.to_string_lossy().to_string(),
        project_dir: project_dir.to_string_lossy().to_string(),
        dev_root,
        related_plans,
        config_root: config_root.to_string(),
    })
}
```

- [ ] **Step 4: Run tests to confirm they pass**

```bash
cd /Users/antony/Development/Ravel-Lite
cargo test pivot_frame_to_context --lib --tests 2>&1 | tail -15
```

Expected: both tests pass.

- [ ] **Step 5: Commit**

```bash
cd /Users/antony/Development/Ravel-Lite
git add src/pivot.rs tests/integration.rs
git commit -m "pivot: frame_to_context + find_project_root for pushed plans"
```

---

## Task 7: TUI breadcrumb helper

**Files:**
- Modify: `/Users/antony/Development/Ravel-Lite/src/phase_loop.rs`
- Modify: `/Users/antony/Development/Ravel-Lite/tests/integration.rs`

Extracts header rendering to take a stack slice so the breadcrumb can reflect nesting. A thin helper with unit tests — keeps the UX concern out of the main driver.

- [ ] **Step 1: Write the failing tests**

Append to `tests/integration.rs`:

```rust
#[test]
fn pivot_breadcrumb_single_plan() {
    use ravel_lite::phase_loop::format_breadcrumb;
    let s = format_breadcrumb(&[std::path::PathBuf::from("/repo/LLM_STATE/root-plan")]);
    assert_eq!(s, "root-plan");
}

#[test]
fn pivot_breadcrumb_nested_two_deep() {
    use ravel_lite::phase_loop::format_breadcrumb;
    let s = format_breadcrumb(&[
        std::path::PathBuf::from("/repo/LLM_STATE/coord"),
        std::path::PathBuf::from("/repo/LLM_STATE/sub-F-hierarchy"),
    ]);
    assert_eq!(s, "coord → sub-F-hierarchy");
}

#[test]
fn pivot_breadcrumb_handles_three_deep() {
    use ravel_lite::phase_loop::format_breadcrumb;
    let s = format_breadcrumb(&[
        std::path::PathBuf::from("/repo/coord"),
        std::path::PathBuf::from("/repo/sub-F"),
        std::path::PathBuf::from("/repo/sub-F-sub1"),
    ]);
    assert_eq!(s, "coord → sub-F → sub-F-sub1");
}
```

- [ ] **Step 2: Run tests to confirm they fail**

```bash
cd /Users/antony/Development/Ravel-Lite
cargo test pivot_breadcrumb --lib --tests 2>&1 | head -15
```

Expected: compile error — `format_breadcrumb` undefined.

- [ ] **Step 3: Implement format_breadcrumb and make it public**

Open `/Users/antony/Development/Ravel-Lite/src/phase_loop.rs`. Just after the existing `plan_name` helper (around line 40), add:

```rust
/// Render the stack path for phase-header display: basenames joined by ` → `.
///
/// Single-plan stacks render as just the plan's basename (unchanged from
/// pre-pivot Ravel-Lite behaviour). Nested stacks show every plan in the
/// stack — `coord → sub-F → sub-F-sub1`.
pub fn format_breadcrumb(stack_paths: &[std::path::PathBuf]) -> String {
    stack_paths
        .iter()
        .map(|p| plan_name(p))
        .collect::<Vec<_>>()
        .join(" → ")
}
```

- [ ] **Step 4: Run tests to confirm they pass**

```bash
cd /Users/antony/Development/Ravel-Lite
cargo test pivot_breadcrumb --lib --tests 2>&1 | tail -10
```

Expected: all three breadcrumb tests pass.

- [ ] **Step 5: Commit**

```bash
cd /Users/antony/Development/Ravel-Lite
git add src/phase_loop.rs tests/integration.rs
git commit -m "phase_loop: format_breadcrumb helper for stack-aware phase headers"
```

---

## Task 8: Stack-aware phase loop — single-plan regression safety

**Files:**
- Modify: `/Users/antony/Development/Ravel-Lite/src/phase_loop.rs`
- Modify: `/Users/antony/Development/Ravel-Lite/src/lib.rs`
- Modify: `/Users/antony/Development/Ravel-Lite/tests/integration.rs`

Introduces the new `run_stack` entry point. Initial scope: behave *exactly* like the current `phase_loop` for single-plan invocations — no pivot logic yet. This establishes the new function signature + in-memory stack + regression suite before we layer push/pop on top.

`phase_loop` is kept as a thin adapter for backwards compatibility inside tests and main.rs until Task 10 migrates those call sites.

- [ ] **Step 1: Write the regression tests for single-plan behaviour**

Append to `tests/integration.rs`:

```rust
#[tokio::test]
async fn pivot_run_stack_single_plan_completes_one_cycle() {
    use ravel_lite::phase_loop::run_stack;
    use ravel_lite::types::{LlmPhase, PlanContext, SharedConfig};
    use ravel_lite::ui::UI;
    use tokio::sync::mpsc;

    let tmp = TempDir::new().unwrap();
    let root = tmp.path();
    fs::create_dir_all(root.join(".git")).unwrap();
    Command::new("git").arg("init").current_dir(root).output().unwrap();
    Command::new("git").args(["config", "user.email", "t@t"]).current_dir(root).output().unwrap();
    Command::new("git").args(["config", "user.name", "t"]).current_dir(root).output().unwrap();

    let plan = root.join("LLM_STATE").join("solo");
    fs::create_dir_all(&plan).unwrap();
    fs::write(plan.join("phase.md"), "work\n").unwrap();
    Command::new("git").args(["add", "."]).current_dir(root).output().unwrap();
    Command::new("git").args(["commit", "-m", "init"]).current_dir(root).output().unwrap();

    // Mock agent that advances phase each time, ends after one triage cycle.
    let mut next_phase = HashMap::new();
    next_phase.insert(LlmPhase::Work, "analyse-work");
    next_phase.insert(LlmPhase::AnalyseWork, "git-commit-work");
    // Driver's handle_script_phase advances from git-commit-* to the next LLM phase.

    let agent = Arc::new(MockAgent {
        calls: Arc::new(Mutex::new(vec![])),
        next_phase_after: next_phase,
        confirm_handler: |_| false, // Decline the "proceed?" prompt to exit after one cycle
    });

    let ctx = PlanContext {
        plan_dir: plan.to_string_lossy().to_string(),
        project_dir: root.to_string_lossy().to_string(),
        dev_root: root.parent().unwrap().to_string_lossy().to_string(),
        related_plans: String::new(),
        config_root: "/tmp/cfg".to_string(),
    };
    let cfg = SharedConfig { agent: "mock".into(), headroom: 1500 };
    let (tx, mut rx) = mpsc::unbounded_channel();
    let ui = UI::new(tx);
    // Drain UI messages in background so sends don't block.
    let drainer = tokio::spawn(async move { while rx.recv().await.is_some() {} });

    // Should complete without attempting any pivot.
    let result = run_stack(agent.clone(), ctx, &cfg, &ui).await;
    ui.quit();
    let _ = drainer.await;
    assert!(result.is_ok(), "expected clean exit, got: {result:?}");

    // Verify the single plan's phase.md is at the next work phase.
    let phase = fs::read_to_string(plan.join("phase.md")).unwrap();
    assert_eq!(phase.trim(), "work");

    // No stack.yaml should have been created.
    assert!(!plan.join("stack.yaml").exists());
}
```

Note: this test reuses the `MockAgent` struct already defined earlier in `tests/integration.rs`. If `MockAgent` does not already have a `confirm_handler` field, it needs one that matches the existing adapter pattern — inspect `tests/integration.rs` for the current `MockAgent` definition and add a `confirm_handler: fn(&str) -> bool` field if missing, defaulting to `|_| true` in existing tests (and `|_| false` in this new one so the loop exits after one cycle).

- [ ] **Step 2: Adapt MockAgent if needed**

Find `struct MockAgent` in `tests/integration.rs` (around line 180). If it does not already have a `confirm_handler` field, inspect how other tests drive the cycle exit. If there's an existing mechanism for "decline the proceed prompt after N cycles", reuse it in the test above. If not, add the field:

```rust
struct MockAgent {
    calls: Arc<Mutex<Vec<LlmPhase>>>,
    next_phase_after: HashMap<LlmPhase, &'static str>,
    confirm_handler: fn(&str) -> bool,
}
```

...and update existing `MockAgent` instantiations to set `confirm_handler: |_| true`. Only modify the mock — do not add business logic to it.

- [ ] **Step 3: Add run_stack (single-plan passthrough)**

Open `src/phase_loop.rs`. At the very end of the file, add:

```rust
use crate::pivot;

/// Stack-aware phase loop entry point.
///
/// For single-plan invocations (the common case today), this behaves
/// identically to the pre-pivot `phase_loop` function.
///
/// When the running plan's work phase modifies `<root>/stack.yaml` and
/// leaves `phase.md` at `work`, the driver short-circuits into the child
/// plan's cycle. See docs/superpowers/specs/2026-04-20-hierarchical-pivot-design.md.
pub async fn run_stack(
    agent: Arc<dyn Agent>,
    root_ctx: PlanContext,
    config: &SharedConfig,
    ui: &UI,
) -> Result<()> {
    // Initial stack: either resume from <root>/stack.yaml or start at just the root.
    let root_plan_dir = std::path::PathBuf::from(&root_ctx.plan_dir);
    let stack_path = root_plan_dir.join("stack.yaml");

    let mut stack: Vec<PlanContext> = match pivot::read_stack(&stack_path)? {
        Some(s) if !s.frames.is_empty() => {
            let config_root = root_ctx.config_root.clone();
            s.frames
                .iter()
                .map(|f| pivot::frame_to_context(f, &config_root))
                .collect::<Result<Vec<_>>>()?
        }
        _ => vec![root_ctx.clone()],
    };

    // TODO in Task 9: add per-cycle stack-snapshot check + push/pop logic.
    // For now, just run the existing phase_loop for the top-of-stack plan
    // exactly once. This proves the single-plan regression suite passes.
    let top_ctx = stack.last().cloned().expect("stack has at least one frame");
    phase_loop(agent, &top_ctx, config, ui).await
}
```

- [ ] **Step 4: Run tests to confirm they pass**

```bash
cd /Users/antony/Development/Ravel-Lite
cargo test pivot_run_stack_single_plan --lib --tests 2>&1 | tail -20
```

Expected: the single-plan regression test passes. Existing phase_loop tests continue to pass.

- [ ] **Step 5: Commit**

```bash
cd /Users/antony/Development/Ravel-Lite
git add src/phase_loop.rs tests/integration.rs
git commit -m "phase_loop: run_stack entry point (single-plan passthrough; push/pop in next task)"
```

---

## Task 9: Stack-aware phase loop — push/pop integration

**Files:**
- Modify: `/Users/antony/Development/Ravel-Lite/src/phase_loop.rs`
- Modify: `/Users/antony/Development/Ravel-Lite/tests/integration.rs`

This is the biggest task: replaces the placeholder `phase_loop` call in `run_stack` with the real stack-aware driver. Extracts the inner per-phase execution into a reusable helper, adds snapshot-before-work + post-work decision, handles push/pop at cycle boundaries.

- [ ] **Step 1: Write the failing integration tests**

Append to `tests/integration.rs`:

```rust
#[tokio::test]
async fn pivot_run_stack_short_circuit_pivot() {
    use ravel_lite::phase_loop::run_stack;
    use ravel_lite::pivot::{Frame, Stack};
    use ravel_lite::types::{LlmPhase, PlanContext, SharedConfig};
    use ravel_lite::ui::UI;
    use tokio::sync::mpsc;

    let tmp = TempDir::new().unwrap();
    let root = tmp.path();
    fs::create_dir_all(root.join(".git")).unwrap();
    Command::new("git").arg("init").current_dir(root).output().unwrap();
    Command::new("git").args(["config", "user.email", "t@t"]).current_dir(root).output().unwrap();
    Command::new("git").args(["config", "user.name", "t"]).current_dir(root).output().unwrap();

    let coord = root.join("LLM_STATE").join("coord");
    fs::create_dir_all(&coord).unwrap();
    fs::write(coord.join("phase.md"), "work\n").unwrap();

    let child = root.join("LLM_STATE").join("child");
    fs::create_dir_all(&child).unwrap();
    fs::write(child.join("phase.md"), "work\n").unwrap();

    Command::new("git").args(["add", "."]).current_dir(root).output().unwrap();
    Command::new("git").args(["commit", "-m", "init"]).current_dir(root).output().unwrap();

    // Coordinator's "work phase" writes stack.yaml with a child frame and
    // leaves phase.md at "work" — the short-circuit pivot signal.
    // Child's work phase advances normally and completes one cycle.

    let calls = Arc::new(Mutex::new(vec![]));
    let coord_clone = coord.clone();
    let child_clone = child.clone();

    struct PivotMockAgent {
        calls: Arc<Mutex<Vec<(std::path::PathBuf, LlmPhase)>>>,
        coord: std::path::PathBuf,
        child: std::path::PathBuf,
    }

    #[async_trait::async_trait]
    impl ravel_lite::agent::Agent for PivotMockAgent {
        async fn invoke_interactive(&self, _prompt: &str, ctx: &PlanContext) -> anyhow::Result<()> {
            let plan_dir = std::path::PathBuf::from(&ctx.plan_dir);
            self.calls.lock().unwrap().push((plan_dir.clone(), LlmPhase::Work));

            if plan_dir == self.coord {
                // Short-circuit pivot: write stack.yaml to coord/stack.yaml,
                // leave phase.md at "work".
                let stack = Stack {
                    frames: vec![
                        Frame { path: self.coord.clone(), pushed_at: None, reason: None },
                        Frame { path: self.child.clone(), pushed_at: None, reason: Some("test".into()) },
                    ],
                };
                let y = serde_yaml::to_string(&stack).unwrap();
                fs::write(self.coord.join("stack.yaml"), y).unwrap();
                // DON'T advance phase.md.
            } else {
                // Child: advance phase normally.
                fs::write(plan_dir.join("phase.md"), "analyse-work\n").unwrap();
            }
            Ok(())
        }

        async fn invoke_headless(
            &self,
            _prompt: &str,
            ctx: &PlanContext,
            phase: LlmPhase,
            _agent_id: &str,
            _tx: ravel_lite::ui::UISender,
        ) -> anyhow::Result<()> {
            let plan_dir = std::path::PathBuf::from(&ctx.plan_dir);
            self.calls.lock().unwrap().push((plan_dir.clone(), phase));

            // Advance the phase according to the normal cycle so
            // handle_script_phase does the rest.
            let next = match phase {
                LlmPhase::AnalyseWork => "git-commit-work",
                LlmPhase::Reflect => "git-commit-reflect",
                LlmPhase::Dream => "git-commit-dream",
                LlmPhase::Triage => "git-commit-triage",
                LlmPhase::Work => unreachable!("work is interactive"),
            };
            fs::write(plan_dir.join("phase.md"), format!("{next}\n")).unwrap();
            Ok(())
        }

        async fn dispatch_subagent(
            &self,
            _prompt: &str,
            _target_plan: &str,
            _tx: ravel_lite::ui::UISender,
        ) -> anyhow::Result<()> { Ok(()) }

        fn tokens(&self) -> HashMap<String, String> { HashMap::new() }
    }

    let agent = Arc::new(PivotMockAgent {
        calls: calls.clone(),
        coord: coord_clone.clone(),
        child: child_clone.clone(),
    });

    let ctx = PlanContext {
        plan_dir: coord.to_string_lossy().to_string(),
        project_dir: root.to_string_lossy().to_string(),
        dev_root: root.parent().unwrap().to_string_lossy().to_string(),
        related_plans: String::new(),
        config_root: "/tmp/cfg".to_string(),
    };
    let cfg = SharedConfig { agent: "mock".into(), headroom: 1500 };
    let (tx, mut rx) = mpsc::unbounded_channel();
    let ui = UI::new(tx);
    let drainer = tokio::spawn(async move { while rx.recv().await.is_some() {} });

    // Exit after child's cycle completes (user declines "proceed?" at
    // end of child's git-commit-triage — but MockAgent's UI confirm
    // handler sends the reply).
    let result = run_stack(agent.clone(), ctx, &cfg, &ui).await;
    ui.quit();
    let _ = drainer.await;
    assert!(result.is_ok(), "expected clean exit, got: {result:?}");

    let log = calls.lock().unwrap();

    // Expected call sequence:
    //   1. coord work (wrote stack.yaml with child frame; left phase.md at work)
    //   --> driver pushes child immediately (short-circuit)
    //   2. child work
    //   3. child analyse-work
    //   4. child reflect
    //   5. child triage (dream skipped since memory is small)
    //   --> driver pops after child git-commit-triage
    //   --> coord's phase.md is still at "work"; stack.yaml is back to just root
    //   6. coord work again (would start next cycle — but we need to cut it off)

    // Minimum assertion: both coord AND child were invoked, coord first.
    assert!(!log.is_empty());
    assert_eq!(log[0].0, coord, "first call should be coord.work");
    assert_eq!(log[0].1, LlmPhase::Work);
    assert!(log.iter().any(|(p, _)| p == &child), "child should have been invoked");

    // After pop, stack.yaml should be absent (trimmed to just root).
    assert!(!coord.join("stack.yaml").exists(),
        "stack.yaml should be deleted after pop to depth 1");
}
```

Note: this test exits via the existing `ui.confirm` mechanism in ravel-lite. The test's `UI` drainer accepts messages; if `handle_script_phase` sends a `Confirm` and nothing answers, the test hangs. Look for how existing async tests handle this — there may be a confirm-handler pattern in the UI or a timeout wrapper. Add a `tokio::time::timeout` of 5 seconds around the `run_stack` call if the existing tests don't establish a pattern, and `panic!("test hung")` on timeout.

- [ ] **Step 2: Run the test to confirm it fails**

```bash
cd /Users/antony/Development/Ravel-Lite
cargo test pivot_run_stack_short_circuit_pivot --lib --tests 2>&1 | tail -30
```

Expected: the test fails — `run_stack` still calls `phase_loop` once and never pushes.

- [ ] **Step 3: Implement the full stack-aware driver**

Replace the `run_stack` placeholder body in `src/phase_loop.rs` with the real driver. Remove the `// TODO in Task 9` line.

Design notes:
- Outer `loop`: peek `stack.last()` for the currently-executing plan.
- Inner: read `phase.md`, match on LLM or Script.
  - Script: call `handle_script_phase`. If it returns `Ok(false)` (user declined "proceed"), return.
  - If the script was `GitCommitTriage`: compute post-cycle action via `decide_after_cycle` using a stack.yaml snapshot taken at the start of the just-completed cycle. Act on Push / Pop / Continue.
  - LLM Work: snapshot stack.yaml before the call. Run `invoke_interactive` (suspend/resume UI). After the call: read phase.md + stack.yaml. Call `decide_after_work`. Act on PushImmediately / PushAfterCycle / ContinueNormalCycle / Error.
    - On `PushImmediately`: validate_push against pre-call stack, `frame_to_context`, push into in-memory `stack`, log breadcrumb, `continue` outer loop.
    - On `PushAfterCycle`: remember the frame; at end of this plan's git-commit-triage, treat as the `stack_grew` signal for `decide_after_cycle`.
    - On `ContinueNormalCycle`: continue outer loop (next phase read will find analyse-work).
    - On `Error(msg)`: log and return with error.
  - LLM other (AnalyseWork, Reflect, Dream, Triage): unchanged — call `invoke_headless` then continue.

Critical invariant: the in-memory `stack` and the on-disk `stack.yaml` must stay in sync. On push: update both. On pop: update both (file is deleted when trimmed to len 1).

Helpers you will add inside `phase_loop.rs`:

```rust
/// Write the in-memory stack to disk, or delete the file if the stack
/// is at depth 1 (root only).
fn sync_stack_to_disk(stack_path: &Path, stack: &[PlanContext]) -> Result<()> {
    if stack.len() <= 1 {
        if stack_path.exists() {
            std::fs::remove_file(stack_path)
                .with_context(|| format!("Failed to remove {}", stack_path.display()))?;
        }
        return Ok(());
    }
    let frames: Vec<pivot::Frame> = stack.iter().map(|ctx| pivot::Frame {
        path: std::path::PathBuf::from(&ctx.plan_dir),
        pushed_at: Some(chrono_like_timestamp()),
        reason: None,
    }).collect();
    pivot::write_stack(stack_path, &pivot::Stack { frames })
}

/// RFC3339-ish timestamp without a chrono dep. Acceptable: SystemTime since UNIX_EPOCH
/// formatted as seconds + "Z" — stack.yaml's timestamp is informational only.
fn chrono_like_timestamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    format!("@{secs}")
}
```

Keep `run_stack`'s body under 120 lines. If you find it growing larger, extract helpers (e.g., `handle_work_phase_result`, `handle_end_of_cycle`) — but each helper stays in `phase_loop.rs`. Do not move logic into `pivot.rs`; that module is deliberately I/O-light and decision-pure.

- [ ] **Step 4: Run the pivot test to confirm it passes**

```bash
cd /Users/antony/Development/Ravel-Lite
cargo test pivot_run_stack_short_circuit_pivot --lib --tests 2>&1 | tail -30
```

Expected: short-circuit pivot test passes. All existing tests continue to pass.

- [ ] **Step 5: Run the full test suite for regression safety**

```bash
cd /Users/antony/Development/Ravel-Lite
cargo test --lib --tests 2>&1 | tail -20
```

Expected: all tests pass (existing phase_loop / dream / config tests + all new pivot_* tests).

- [ ] **Step 6: Commit**

```bash
cd /Users/antony/Development/Ravel-Lite
git add src/phase_loop.rs tests/integration.rs
git commit -m "phase_loop: full stack-aware driver — short-circuit pivot, stateful pivot, pop"
```

---

## Task 10: main.rs migration + breadcrumb wiring

**Files:**
- Modify: `/Users/antony/Development/Ravel-Lite/src/main.rs`
- Modify: `/Users/antony/Development/Ravel-Lite/src/phase_loop.rs` (the `log_phase_header` call site)

Swap `phase_loop` → `run_stack` in main; thread the in-memory stack into phase-header logging so the breadcrumb renders.

- [ ] **Step 1: Swap the main.rs call**

Open `/Users/antony/Development/Ravel-Lite/src/main.rs`. At `main.rs:204`, change:

```rust
let result = phase_loop::phase_loop(agent, &ctx, &shared_config, &ui).await;
```

to:

```rust
let result = phase_loop::run_stack(agent, ctx, &shared_config, &ui).await;
```

Note: `run_stack` takes `PlanContext` by value (moved); `phase_loop` took it by reference. Remove the `&` and be prepared to clone `ctx` earlier if it's used after this call (inspect the surrounding function; I don't believe `ctx` is referenced after the `phase_loop` call, but verify).

- [ ] **Step 2: Thread breadcrumb into phase-header logging**

In `src/phase_loop.rs`, inside `run_stack`'s main loop, at the point where `log_phase_header` is called (inside the per-phase execution helper you extracted in Task 9), pass the in-memory `stack` slice. Example:

```rust
let stack_paths: Vec<std::path::PathBuf> = stack.iter()
    .map(|c| std::path::PathBuf::from(&c.plan_dir))
    .collect();
let breadcrumb = format_breadcrumb(&stack_paths);
log_phase_header_with_breadcrumb(ui, lp, &project, &breadcrumb);
```

Add the new header function in `src/phase_loop.rs`:

```rust
fn log_phase_header_with_breadcrumb(ui: &UI, phase: LlmPhase, project: &str, breadcrumb: &str) {
    let info = phase_info(phase);
    ui.log(&format!("\n{HR}"));
    ui.log(&format!("  ◆  {}  ·  {}", info.label, header_scope(project, breadcrumb)));
    ui.log(&format!("  {}", info.description));
    ui.log(HR);
}
```

The old `log_phase_header` stays in place for now, unused by `run_stack` — safe to keep during transition. If `cargo build` warns about dead code, add `#[allow(dead_code)]` until Task 11 (or delete the old function if no other test references it).

- [ ] **Step 3: Add push/pop log lines in run_stack**

At the point in `run_stack` where you perform a push (both `PushImmediately` and `PushAfterCycle` paths) and a pop, emit informational log lines:

```rust
// On push:
let reason = frame.reason.as_deref().unwrap_or("");
if reason.is_empty() {
    ui.log(&format!("\n▷▷ PUSH · {}", plan_name(&frame.path)));
} else {
    ui.log(&format!("\n▷▷ PUSH · {} · \"{}\"", plan_name(&frame.path), reason));
}

// On pop (use the popped plan's basename):
ui.log(&format!("\n◁◁ POP · {} → {}",
    plan_name(&popped.plan_dir.as_ref()),
    plan_name(&stack.last().unwrap().plan_dir.as_ref())));
```

(Adjust the exact types — `plan_name` takes `&Path`; if your helper variable is `String`, convert with `Path::new(&s)`.)

- [ ] **Step 4: Build and run the full test suite**

```bash
cd /Users/antony/Development/Ravel-Lite
cargo build 2>&1 | tail -15
cargo test --lib --tests 2>&1 | tail -15
```

Expected: clean build (no warnings in the paths you changed) and all tests pass.

- [ ] **Step 5: Commit**

```bash
cd /Users/antony/Development/Ravel-Lite
git add src/main.rs src/phase_loop.rs
git commit -m "main: swap phase_loop → run_stack; thread breadcrumb into phase headers"
```

---

## Task 11: .gitignore convention for stack.yaml

**Files:**
- Modify: `/Users/antony/Development/Ravel-Lite/defaults/phases/work.md` *(only if the phases reference `.gitignore` conventions; skip if not)*
- Modify: `/Users/antony/Development/Ravel/.gitignore` (add `LLM_STATE/*/stack.yaml`)

Ensures `stack.yaml` doesn't land in git history. Since the Ravel-Lite `init` command does not currently scaffold a `.gitignore` at plan dirs, the cleanest placement is a top-level project `.gitignore` entry.

- [ ] **Step 1: Check if there's an existing .gitignore in the Ravel project**

```bash
cat /Users/antony/Development/Ravel/.gitignore 2>/dev/null || echo "(no .gitignore)"
```

- [ ] **Step 2: Add or create the entry**

If `.gitignore` exists, append:

```
# Runtime state for hierarchical-pivot stack (see docs/superpowers/specs/2026-04-20-hierarchical-pivot-design.md)
LLM_STATE/*/stack.yaml
```

If it doesn't exist, create it with just those two lines.

- [ ] **Step 3: Commit (in the Ravel repo, not Ravel-Lite)**

```bash
cd /Users/antony/Development/Ravel
git add .gitignore
git commit -m "gitignore: exclude LLM_STATE/*/stack.yaml (hierarchical-pivot runtime state)"
```

---

## Task 12: Coordinator prompt-work.md

**Files:**
- Create: `/Users/antony/Development/Ravel/LLM_STATE/ravel-orchestrator/prompt-work.md`

The routing prompt. Authored once per coordinator; read every cycle. Contains the concrete list of children (sub-A through sub-R), the briefing format, and the stack.yaml append convention.

- [ ] **Step 1: Back up the existing file if any**

```bash
ls -la /Users/antony/Development/Ravel/LLM_STATE/ravel-orchestrator/prompt-work.md 2>/dev/null
```

If the file exists (it does — see the earlier `ls` output showing `prompt-work.md` in the orchestrator dir), inspect its contents first:

```bash
cat /Users/antony/Development/Ravel/LLM_STATE/ravel-orchestrator/prompt-work.md | head -80
```

Preserve any project-specific tokens or formatting conventions the existing file uses (e.g., `{{VAULT_CATALOG}}`, `{{PROJECT}}` placeholders). The new prompt overwrites the body but should match the token style of the surrounding Ravel-Lite prompt templates.

- [ ] **Step 2: Write the new prompt**

Use the Write tool to create `/Users/antony/Development/Ravel/LLM_STATE/ravel-orchestrator/prompt-work.md` with this content. Preserve any `{{TOKENS}}` your inspection in Step 1 revealed as load-bearing; otherwise the body below is the spec:

```markdown
# Work phase — Ravel coordinator

You are the routing agent for the Ravel v1 implementation. Your job this
cycle: decide which child plan should run next, and signal the decision
by writing `<root>/stack.yaml`. You do NOT implement tasks yourself — all
implementation work happens inside child plans.

Design reference: `docs/superpowers/specs/2026-04-20-hierarchical-pivot-design.md`.

## Your children

You know these plans as pivot candidates. Each one lives at
`LLM_STATE/<name>/`:

- sub-A-global-store
- sub-B-phase-cycle
- sub-C-adapters
- sub-D-coordination
- sub-E-ingestion
- sub-F-hierarchy
- sub-G-migration
- sub-H-skills
- sub-I-obsidian-coverage
- sub-M-observability
- sub-N-domain-experts
- sub-R-knowledge-ontology

If this list drifts from the real filesystem, update this prompt — the
list is authoritative for what you consider.

## Step 1 — Gather state

Read your own `memory.md` (focus on the *Stable architectural decisions*
and *Gates* sections) plus each child's `backlog.md`. For each child,
extract:

- Ready count: entries whose `- **Status:** not_started` AND whose
  `- **Dependencies:**` list is empty or references only tasks with
  `- **Status:** completed` / `- **Status:** done`.
- A one-line bottleneck note, if the child has a visible blocker the
  backlog prose mentions (e.g., "gated by Sub-D/T2").

Scope your reads. You do not need each child's full backlog — the file
header + the first 5 `not_started` entries per child is plenty. Use the
Read tool with appropriate offsets/limits.

## Step 2 — Brief the user

Present a compact grid:

```
   child                       ready   last touched   notes
   ──────────────────────────  ─────   ─────────────  ─────────────────────
   sub-A-global-store          3       today          —
   sub-F-hierarchy             0       3h ago         gated by sub-D/T2
   sub-N-domain-experts        5       1d ago         —
   ...
```

Include a *Bottleneck watch* section if one child's progress unblocks
multiple others (derivable from the Gates memory). Recommend one
target with a one-line rationale:

> Recommended: **sub-D-coordination**. sub-D/T2 blocks sub-F/T4, T6, T7;
> breaking it unblocks three downstream entries.

Invite the user to accept (empty/enter), pick a different child by
name, ask to *inspect <child>* (dispatch a read-only subagent to
summarise one child's state), or say *hold* / *done*.

## Step 3a — If the user picks a pivot target

Append a frame to `<root>/stack.yaml` using the Write tool. The root is
this plan's directory (the one you are running in). Typical content
after the append:

```yaml
frames:
  - path: LLM_STATE/ravel-orchestrator
  - path: LLM_STATE/sub-F-hierarchy
    reason: sub-D/T2 landed last cycle; sub-F/T4 now unblocked
```

If `stack.yaml` does not yet exist, create it with BOTH the root frame
(this plan) AND the target frame. If it exists but has only the root
frame (unusual — normally it's deleted at depth 1), rewrite with both
frames. If it already has multiple frames (resume from a prior session
that didn't fully unwind), append the new target as a new last entry.

**Do NOT advance `phase.md`.** Leave it at `work`. This is the
short-circuit pivot signal — the driver pushes immediately and runs the
child's full cycle.

## Step 3b — If the user says "hold" or "done" (or declines to pivot)

Do NOT touch `stack.yaml`. Advance `phase.md` to `analyse-work` as
normal. The coordinator's remaining phases will run (mostly no-op) and
the end-of-cycle "Proceed to next work phase?" confirmation will fire,
letting the user stop or loop.

## Step 3c — If the user asks to inspect a child

Use the Task tool to dispatch one read-only subagent into the named
child plan with a prompt like:

> "Summarise the current state of this plan in ≤20 lines: recent
> session-log entries, pending gates, top 3 ready backlog items with
> one-line descriptions, any blockers mentioned in memory. Read-only."

Return the summary to the user; then re-enter this work phase (do
nothing to stack.yaml or phase.md; just continue the conversation).
User may then accept, pick a different child, or ask to inspect another.

## Step 4 — Never leave both stack and phase unchanged

The driver will halt with the existing "phase did not advance" error if
you leave `phase.md` at `work` AND do not modify `stack.yaml`. Always do
one or the other before exiting this phase.

## Memory convention

The coordinator's `memory.md` tracks cross-plan state in a `Gates`
section. Entries are free-form prose, terse:

> **sub-F/T4 waits on sub-D/T2** (last checked cycle 5, sub-D/T2 still
> `not_started`; unblocking this releases sub-F/T6 and T7 too).

Reflect phase updates these. You read them in Step 1 to inform the
recommendation.
```

- [ ] **Step 3: Commit**

```bash
cd /Users/antony/Development/Ravel
git add LLM_STATE/ravel-orchestrator/prompt-work.md
git commit -m "coordinator(ravel-orchestrator): routing prompt for hierarchical pivot"
```

---

## Task 13: Seed Gates section in coordinator memory

**Files:**
- Modify: `/Users/antony/Development/Ravel/LLM_STATE/ravel-orchestrator/memory.md`

Translates the existing orchestrator's *Priority 1 — Implementation phase coordination* backlog entries (`LLM_STATE/ravel-orchestrator/backlog.md`) into the `Gates` memory section so the new routing prompt has concrete entries to read from day one.

- [ ] **Step 1: Inspect the current memory.md and backlog.md**

```bash
wc -l /Users/antony/Development/Ravel/LLM_STATE/ravel-orchestrator/memory.md
head -20 /Users/antony/Development/Ravel/LLM_STATE/ravel-orchestrator/memory.md
grep -n '^## ' /Users/antony/Development/Ravel/LLM_STATE/ravel-orchestrator/memory.md
```

Identify the bottom of the file; the Gates section gets appended below existing content.

Also inspect the backlog's Priority 1 section:

```bash
sed -n '/^## Priority 1/,/^## Priority 2/p' /Users/antony/Development/Ravel/LLM_STATE/ravel-orchestrator/backlog.md
```

Use the Priority 1 tasks as source material for Gates entries.

- [ ] **Step 2: Append the Gates section**

Append this content to `/Users/antony/Development/Ravel/LLM_STATE/ravel-orchestrator/memory.md`:

```markdown

## Gates

Cross-plan dependency tracking. Routing decisions in the work phase
consult this section. Entries are terse prose; reflect phase updates
them as children make progress.

### sub-C triage gate
**sub-C backlog has two dispatched gate tasks** (Elixir backlog rewrite,
model-configurability) that need formal promotion via a sub-C triage
cycle. Until that triage runs, Stream 1 of implementation is blocked
here. Running sub-C for one cycle should promote these tasks and clear
the gate.

### sub-B and sub-C → sub-F Task 0 readiness
**sub-F Task 0 is the readiness check for downstream sub-F work.** It
depends on sub-B's backlog rewrite gate AND sub-C's formal gate task
both being `completed` / `done`. Both are currently outstanding. sub-F
cannot meaningfully proceed beyond Task 0 without both.

### sub-D DaemonLock + FileIO → sub-F Tasks 1, 4, 6–12
**sub-D's `DaemonLock` and `FileIO` deliverables gate sub-F work.** Until
sub-D lands these primitives, sub-F's Tasks 1, 4, 6–12 remain blocked
even if their within-plan prerequisites are satisfied. Pivoting to sub-D
to advance these primitives unblocks a large sub-F branch.

### Independent streams (no cross-plan gates)
**sub-E Tasks 0+ and sub-N Tasks 1–15 are fully unblocked.** They have
no cross-plan dependencies and can be pivoted to whenever the user
wants to make parallel progress without touching the gated chains.

### sub-R literature survey (Tasks 1–3) is fully unblocked
**sub-R Tasks 1–3** (Datalog foundations, ontology/folksonomy literature,
LLM ontology interfaces) have no cross-project dependencies. These gate
sub-R's Datalog engine implementation (Tasks 4+), so kicking them off
early is valuable even if no immediate downstream consumer is waiting.
```

- [ ] **Step 3: Commit**

```bash
cd /Users/antony/Development/Ravel
git add LLM_STATE/ravel-orchestrator/memory.md
git commit -m "coordinator: seed Gates section from Priority 1 orchestrator backlog"
```

---

## Task 14: End-to-end pilot against the real repo

**Files:**
- None (operational task; may reveal prompt or driver bugs warranting amendments)

Validates the full path end-to-end by running one cycle of the coordinator against the real sub-plans.

- [ ] **Step 1: Verify both repos build cleanly**

```bash
cd /Users/antony/Development/Ravel-Lite && cargo build --release 2>&1 | tail -5
cd /Users/antony/Development/Ravel && ls LLM_STATE/ravel-orchestrator/prompt-work.md
```

Expected: clean release build of ravel-lite; prompt-work.md exists.

- [ ] **Step 2: Run one supervised cycle**

```bash
cd /Users/antony/Development/Ravel-Lite
cargo run --release -- run /Users/antony/Development/Ravel/LLM_STATE/ravel-orchestrator
```

The coordinator's work phase should present the children's status grid and recommend a target. Accept or override with a child name.

Expected end-to-end sequence:

1. Coordinator work phase runs; agent appends a frame to `<coord>/stack.yaml`; leaves phase.md at "work".
2. Driver logs `▷▷ PUSH · <child> · "<reason>"` in the TUI.
3. Child's work phase runs (interactive).
4. Child's analyse-work, git-commit-work, reflect, git-commit-reflect, [dream?], triage, git-commit-triage run.
5. Driver logs `◁◁ POP · <child> → ravel-orchestrator`.
6. Coordinator work phase runs again (next routing decision) OR user declines "Proceed to next work phase?" to stop.

- [ ] **Step 3: Verify on-disk state after exit**

- `LLM_STATE/ravel-orchestrator/stack.yaml` should NOT exist (trimmed when stack returned to depth 1).
- The child plan's git history should show its per-cycle commits (one or more `work`, one `reflect`, one `triage`).
- The coordinator's memory.md may have accumulated routing notes if the user exercised the reflect phase.

- [ ] **Step 4: If the pilot reveals issues, fix and re-commit**

Any prompt adjustments go as separate commits. Any Rust-side bug fixes land in Ravel-Lite's repo. Common issues to watch for:

- Breadcrumb format is off (spacing, separator). Adjust `format_breadcrumb`.
- Push/pop log lines are too noisy or too quiet. Tune the format strings.
- Coordinator prompt's children list is out-of-sync with filesystem. Update the prompt.
- Short-circuit detection misfires (driver treats something as a pivot that isn't). Compare `stack.yaml` before/after in the logs.

---

## Self-review checklist (informational — not executed)

**Spec coverage**
- §Architecture (two repos, three roles) → Tasks 1–10 (Ravel-Lite) + 11–13 (Ravel)
- §stack.yaml schema and location → Tasks 1, 2
- §Driver state transitions after a work phase → Tasks 4, 9
- §Driver state transitions at end of cycle → Tasks 5, 9
- §Startup resume → Task 8 (initial read in run_stack)
- §Safety rails (depth, invalid target, cycle, corrupt yaml) → Tasks 2, 3
- §Invariants preserved → enforced by the single-plan regression test in Task 8 + full-suite runs in Tasks 9, 10
- §Coordinator plan pattern → Tasks 12, 13
- §TUI (breadcrumb, push/pop lines) → Tasks 7, 10
- §Rollout → Tasks 14

**Placeholder scan**
- One TODO intentionally lands in Task 8 (the `run_stack` placeholder); Task 9 removes it. No TODOs linger past Task 9.
- No "add appropriate error handling" / "implement later" patterns.

**Type consistency**
- `Frame`, `Stack`, `NextAfterWork`, `NextAfterCycle`, `MAX_STACK_DEPTH` — defined once each in `pivot.rs`, used throughout.
- `PlanContext` — imported from `types.rs`, not redefined.
- `frame_to_context` signature `(&Frame, &str) -> Result<PlanContext>` — consistent across Tasks 6 and 9.

---

Plan complete and saved to `docs/superpowers/plans/2026-04-20-hierarchical-pivot.md`. Two execution options:

**1. Subagent-Driven (recommended)** — I dispatch a fresh subagent per task, review between tasks, fast iteration.

**2. Inline Execution** — Execute tasks in this session using executing-plans, batch execution with checkpoints.

Which approach?
