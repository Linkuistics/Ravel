# Rust core, built for WebAssembly

The parser, the checks, the tracer and the layout form one Rust library. The library builds as a native crate and as a WebAssembly module. The browser workbench calls the WebAssembly module from TypeScript and draws with D3. Other hosts, such as a later language server, can embed the same library.

## Trade-off

The project needs two toolchains and a typed boundary between them. `tsify` can generate TypeScript declarations for the types that cross that boundary (https://github.com/madonoharu/tsify). In return, one implementation serves the browser, native programs and a language server. VS Code can run a WebAssembly language server (https://code.visualstudio.com/blogs/2023/06/05/vscode-wasm-wasi).

## Rejected alternative

A TypeScript core, which the human first preferred. TypeScript does not compile to WebAssembly, so every native host would need a JavaScript runtime. The pasted request asked for a vanilla JavaScript web component with no dependencies. That component could not serve other languages.

Source: the human's decision of 2026-11-17 in `docs/milestones/grammar-workbench.md`, and the language-tooling survey.
