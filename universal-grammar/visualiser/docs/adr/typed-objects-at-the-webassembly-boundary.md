# Typed objects at the WebAssembly boundary

The workbench page calls the core through typed objects. The Rust types that cross the boundary derive the serde traits and `Tsify`, and `tsify` writes their TypeScript declarations (https://github.com/madonoharu/tsify). The objects cross through `serde-wasm-bindgen`, and each call returns an error as a value through the `Ts<T>` wrapper of `tsify` 0.5.8. KDL text crosses the boundary only as the content of a file that a person loads or saves. The core reads and writes that text, and the page never parses KDL.

## Trade-off

The Rust types gain a second serialisation beside the hand-written KDL mapping. The WebAssembly build also gains `wasm-bindgen`, serde, `serde-wasm-bindgen` and `tsify`. The derives sit behind a feature of the core, so native builds and tests need none of these crates. A field can now reach the page but not the KDL form. The round-trip tests of the mapping module and the end-to-end tests of the page catch such a gap.

In return, the TypeScript compiler checks the page against the types of the core. Each operation returns a small value, not a new KDL document of the whole trace. The page needs no KDL library, so no second KDL implementation can disagree with the `kdl` crate. Two implementations of one notation tend to drift apart: regular expressions behave differently in eight languages for 15% of real regexes (https://fservant.github.io/papers/Davis_Michael_Coghlan_Servant_Lee_ESECFSE19.pdf). The `Ts<T>` wrapper returns a bad value as an error, not as an exception through the WebAssembly frames.

## Rejected alternatives

KDL text in both directions. The page would parse each result with a TypeScript library for KDL 2.0, such as `@bgotink/kdl` or `kdljs`. It would keep its own types for each node by hand, so a change to the format would reach the page only at run time.

A JSON string, through the `json` feature of `tsify`. It adds a parse on each side, and it gives nothing that typed objects do not give.

Source: the language-tooling survey, the root brief of the slice, and the design of `tracer-design-k7` in the spec `tracer`.
