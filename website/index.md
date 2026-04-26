---
title: Ravel
---

# Ravel

> A persistent actor daemon for AI-assisted development.
> Plans, phases, knowledge, and fresh-context reasoning, unified.

A [Linkuistics](https://github.com/Linkuistics) project.

---

AI coding assistants start every session from scratch. Even within a single session they accumulate context until it rots. Knowledge from yesterday vanishes by tomorrow. Cross-project learning is impossible. Long-running development — the kind that actually matters on real projects — drowns in the mismatch between the way developers think (continuous, hierarchical, specialist-consulting) and the way stateless LLM sessions work.

Ravel is a persistent orchestration daemon built around three convictions: fresh context is first-class (every reasoning task runs in a short, focused session with exactly the context it needs); plans and knowledge are actors (each ongoing work effort and each knowledge domain is a long-lived actor with its own state and mailbox); and the unit of orchestration is the actor (one daemon per machine hosts many actors, each backed by filesystem state that survives restarts).

The daemon is built on Elixir/BEAM using OTP primitives — actor model, supervision, message passing, and hot code reload are all BEAM-native. Clients attach over a local Unix socket: a Rust TUI ships in v1, with an Obsidian plugin and web UI planned for later versions. Ravel is currently in architectural design phase with all v1 sub-project brainstorms complete; implementation has begun on the daemon core.
