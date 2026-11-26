# KDL for every derived artifact

The core writes the checked grammar, the rule diagrams with their coordinates and each trace as KDL 2.0 (https://kdl.dev/spec/). Every derived artifact records a format version and a hash of its grammar text. The Rust types of the core define the format through one mapping module with round-trip tests, so the project keeps no separate schema. Trace files are a public format, because LLM tools and test fixtures read them. The mapping module reads and writes KDL with the `kdl` crate (https://github.com/kdl-org/kdl-rs).

## Trade-off

KDL has fewer libraries and tools than JSON. The grammar-notations survey found no study of how accurately LLMs write KDL, and no constrained decoder that supports it. In return, KDL suits LLM generation better, in the judgement of the human. Libraries exist for TypeScript, Python and C (https://github.com/kdl-org/kdljs, https://github.com/tabatkins/kdlpy, https://github.com/tjol/ckdl). The hash lets a reader detect an artifact that no longer matches its grammar. The railroad diagrams of SQLite drifted from their grammar without such a check (https://arxiv.org/html/2509.15834v2).

The `kdl` crate has no derive, so the project writes the mapping module by hand. In return, the crate is the only Rust library in the KDL implementation table that states support for KDL 2.0. Its errors carry spans through `miette`, and its direct dependencies are Rust crates, with no C library. The one mapping module keeps the crate replaceable.

## Rejected alternatives

JSON with a JSON Schema, the first proposal. The core adds a JSON encoding and a generated schema only if an outside consumer needs them. KDL as the grammar input was also rejected, because people and LLMs write grammars in the EBNF notation.

`knus` and `facet-kdl`, which derive the mapping from the Rust types. `knus` reads only KDL 1. `facet-kdl` does not state its KDL version. At version 0.42 it moved its older interface to a separate crate, so its interface can change between releases.

Source: the human's decisions of 2026-11-17 in `docs/milestones/grammar-workbench.md`, the grammar-notations, language-tooling and grammar-visualisers surveys, and the checks of `notation-k3` on 2026-11-18.
