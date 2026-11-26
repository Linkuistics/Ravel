# Language tooling for the grammar notation

Research report for Topic 6 (language tooling). Status: complete. Section 5 lists the searches that found nothing, and the text marks each fact that I did not check.

The question is which technology parses grammar files for both the browser tool and an editor extension. The fallback to beat is the proposal of the human: a tree-sitter grammar with a Rust language server.

During the survey, the human chose a Rust core that runs natively and in WebAssembly. Grammar inputs and derived artefacts must also have a serialised form that many languages can read. The human chose KDL 2.0 for this form. Architectures A and C are therefore the main candidates, and B, D and E stay as reference points. The five architectures are:

- **A.** A tree-sitter grammar, a Rust language server over the `tree-sitter` crate, and `web-tree-sitter` in the browser.
- **B.** A tree-sitter grammar and one TypeScript core over `web-tree-sitter`. The core serves the browser tool and a TypeScript language server.
- **C.** A Rust core, compiled to WebAssembly (WASM) for the browser and to native code for the language server.
- **D.** Langium, which generates a parser, a typed syntax tree, a language server and a VS Code extension from one grammar.
- **E.** A hand-written TypeScript parser, shared by the browser tool and a TypeScript language server.

## 1. Findings that change decisions

- **Q7.** A tree-sitter grammar does not give syntax colour in VS Code (section 3.1). VS Code has no contribution point for tree-sitter grammars from extensions. Issue 50140 asked for one in May 2018, and it is still open. VS Code uses tree-sitter only in its own experimental highlighting, which is off by default. An extension still needs a TextMate grammar or semantic tokens from a language server. GitHub takes only TextMate grammars from the public, and Neovim, Helix and Zed each need their own query files.
- **Q7.** Tree-sitter recovers from syntax errors, but it does not write error messages (section 3.3). The tree holds `ERROR` and `MISSING` nodes, without a message or a set of expected tokens. In 2022, the Solang compiler team concluded that tree-sitter is not usable for invalid input in its current form. The original request asks for line and column syntax errors, so our code must write every message.
- **Q7.** A hand-written recursive descent parser over `rowan` or `cstree` has the best evidence for editor error reports from a Rust core (section 3.5). `rust-analyzer` uses this design, and Aleksey Kladov's tutorial explains its recovery. The `rust-analyzer` guide also says that a parse from scratch is fast enough, so incremental parsing matters little (section 3.4). The parser libraries recover less well. LALRPOP recovers only at marked points, pest stops at the first error, and `chumsky` recovery can miss a valid route. Inference: this design costs more code than a grammar file, and the grammar exists only as code.
- **Q7.** The `tree-sitter` crate builds for `wasm32-unknown-unknown` only since September 2025 (section 3.5). The build needs a clang with the WebAssembly back end, and most reports say that Apple clang lacks it. A 2026 comment reports that `tree-sitter` with `tree-sitter-c` still fails to compile for this target at 0.27. Inference: architectures A and C need a pinned toolchain and a wasm32 build in continuous integration from the start.
- **Q7.** Architecture A adds the failure modes of `web-tree-sitter` (section 3.2). The runtime and each grammar WASM file must agree on the ABI version, and release 0.25.0 moved to ABI 15 in February 2025. Bundlers need extra configuration to find the runtime WASM file. Each object needed a manual `delete()` call until release 0.27.0 in August 2026. Inference: in architecture C, one WASM module holds the Rust core and its parser, so no ABI mismatch can occur at run time.
- **Q7.** VS Code runs a WASM language server only through a preview (section 3.5). `@vscode/wasm-wasi-lsp` has only pre-releases, and the last came in March 2026. Microsoft calls the engine experimental. Since release 1.61, VS Code installs platform-specific packages. `rust-analyzer` ships its native server in these packages. Inference: ship a native server for desktop VS Code, and use the WASI build only for the `web` target.
- **Q7.** The browser editor can talk to a language server in a web worker (section 3.7). CodeMirror 6 has an official LSP client since July 2025. A forum post shows this client with a TypeScript server in web workers. The examples of `monaco-languageclient` also run language servers in web workers. Inference: the Rust core in WASM can give the browser editor and VS Code the same diagnostics.
- **Q7.** The `kdl` crate is the only Rust library in the KDL implementation table with KDL 2.0 support (section 3.8). It has no derive of its own, but release 6.7.1 has serde modules. `knus` reads KDL 1 only, and `facet-kdl` 0.42 does not state its KDL version. Libraries for Python (`kdl-py`, `ckdl`) and JavaScript (`@bgotink/kdl`, `kdljs`) read KDL 2.0. The KDL Schema Language is a draft, and I found no checked tool that writes a KDL schema from Rust types. Inference: the missing schema costs little while only the Rust core decodes KDL.
- **Q7.** `tsify` 0.5.8 writes TypeScript declarations for Rust types that cross the WASM boundary (section 3.8). Its fork `tsify-next` is deprecated. The new `Ts<T>` wrapper returns a decode failure as an error value, not as an exception through the WASM frames.
- **Q6.** One snapshot seam serves the parser, the compiler and the tracer (section 2). Each test pairs an input file with an expected text dump, and an update mode rewrites the dump. Tree-sitter corpus tests, `rust-analyzer` parser tests and `expect-test` all use this seam. Inference: the seam does not depend on the parser technology, so a later change of parser keeps the tests.

## 2. Q6: Test seams

The obvious candidate is the tree-sitter corpus test. A corpus file in `test/corpus` holds named cases, each with source text and the expected syntax tree as an S-expression. The `tree-sitter test` command runs these cases ([tree-sitter-java testing, secondary source](https://deepwiki.com/tree-sitter/tree-sitter-java/4-testing-framework)). Highlight tests in `test/highlight` are source files with comment assertions, based on the syntax tests of Sublime Text ([tree-sitter/tree-sitter#499](https://github.com/tree-sitter/tree-sitter/pull/499)). A caret (`^`) or an arrow (`<-`) in a comment points at a column, and a capture name follows. The same command tests tags queries.

These tests depend on the CLI version. The tree-sitter-gap README says that highlight priority came with version 0.22.2, so older CLI versions give wrong results on its tests ([gap-system/tree-sitter-gap](https://github.com/gap-system/tree-sitter-gap)). Release 0.27.1 changed the capture ranges in query tests as a breaking change (section 3.3).

`rust-analyzer` shows the same seam without tree-sitter. Each parser test is a pair of files: a `.rs` input and a text dump of the expected tree ([architecture](https://rust-analyzer.github.io/book/contributing/architecture.html)). Separate directories hold valid inputs and inputs with errors. A comment `// test name` in the grammar code becomes an inline case, which a task copies into `test_data/parser/inline`. The test run creates a missing expected file, and `UPDATE_EXPECT=1` rewrites changed files.

`rust-analyzer` uses the `expect-test` crate for snapshots ([rust-analyzer/expect-test](https://github.com/rust-analyzer/expect-test/blob/master/src/lib.rs)). The crate keeps the expected text inline in the test source, and an update mode rewrites it. Aleksey Kladov's essay "How to Test" explains the design ([matklad.github.io](https://matklad.github.io/2021/05/31/how-to-test.html)). A test states the expected result as data, and a special mode of the test run updates that data.

Langium gives helpers in `langium/test`. The `parseHelper` function parses a document, and the option `{ validation: true }` runs the validators ([Langium validations](https://langium.org/docs/learn/workflow/create_validations/)). A test then checks `parseResult.lexerErrors`, `parseResult.parserErrors` and `document.diagnostics` ([Langium artefacts](https://langium.org/docs/learn/workflow/generate_everything/)).

Inference: one snapshot seam fits the parser, the compiler and the tracer. Each stage writes a stable text dump: the syntax tree, the automaton and the trace. A test pairs an input file with the expected dump, and an update mode rewrites the dump. Error cases sit in their own directory, and their dumps include each diagnostic with line and column. This seam does not depend on the parser technology, so it stays valid if the project moves from tree-sitter to a hand-written parser.

## 3. Q7: Language tooling

### 3.1 Tree-sitter and VS Code

A tree-sitter grammar does not make the VS Code extension much easier. VS Code colours text with TextMate grammars and with semantic tokens from a language server. It does not load extension tree-sitter grammars. Issue 50140, opened on 19 May 2018, asks VS Code to accept tree-sitter grammars for highlighting ([microsoft/vscode#50140](https://github.com/microsoft/vscode/issues/50140)). The issue is open and sits in the Backlog milestone. In 2018 a maintainer wrote that grammar maintenance costs much effort, and "That will not be different with Tree-Sitter".

VS Code now uses tree-sitter for some of its own languages. Release 1.97 (January 2025) added an experimental tree-sitter highlighter for TypeScript behind `editor.experimental.preferTreeSitter`. The notes say that many of the TextMate grammars of VS Code are no longer maintained ([v1.97](https://code.visualstudio.com/updates/v1_97)). Release 1.99 (March 2025) added CSS and regular expressions inside TypeScript ([v1.99](https://code.visualstudio.com/updates/v1_99)). The test plan items for these languages are [microsoft/vscode#244444](https://github.com/microsoft/vscode/issues/244444) and [microsoft/vscode#244445](https://github.com/microsoft/vscode/issues/244445). I found no release note that opens this mechanism to extensions.

Third-party extensions put tree-sitter colours into VS Code through the semantic token interface. The `tree-sitter-vscode` extension registers a semantic token provider that runs a user-supplied tree-sitter parser ([marketplace](https://marketplace.visualstudio.com/items?itemName=AlecGhost.tree-sitter-vscode)). The "Syntax Highlighter" extension also uses the semantic token interface over TextMate colours ([marketplace](https://marketplace.visualstudio.com/items?itemName=evgeniypeshkov.syntax-highlighter)). The consequence for architectures A and B: the language server must send semantic tokens, or the extension must also carry a TextMate grammar. Inference: either way, colour rules exist in a second place beside the tree-sitter highlight queries.

#### Other editors and GitHub

Other editors load tree-sitter grammars directly, but each editor needs its own query files. Neovim, Helix and Zed highlight a grammar through its `highlights.scm` queries. The capture names in these queries follow the convention of each editor. Zed themes use older names such as `@text.title`, and nvim-treesitter uses newer names such as `@markup.heading` ([zed-industries/zed#40532](https://github.com/zed-industries/zed/issues/40532)). A Zed extension cannot override the query files of a grammar, but Helix users can override them in configuration.

Projects keep one copy of the queries for each editor. One project generates a highlight file for Helix and another for Zed from the same patterns ([elitekaycy/qkt#1584](https://github.com/elitekaycy/qkt/pull/1584)). The rule for overlapping captures also differs. The tree-sitter CLI applies the first matching capture, but nvim-treesitter applies all of them, so the last one wins ([alemuller/tree-sitter-vhdl#1](https://github.com/alemuller/tree-sitter-vhdl/issues/1)).

Emacs keeps its highlight queries in Emacs Lisp. A major mode passes them to `treesit-font-lock-rules` ([c3lang/c3-ts-mode](https://github.com/c3lang/c3-ts-mode)). Users compile each grammar with a local C compiler through `treesit-install-language-grammar` ([Mastering Emacs](https://www.masteringemacs.org/article/how-to-get-started-tree-sitter)).

GitHub does not take tree-sitter grammars from the public. Linguist detects languages on GitHub, and it accepts only TextMate grammars ([Linguist discussion #6073](https://github.com/github/linguist/discussions/6073)). In September 2022, a maintainer wrote that no public process exists to add a tree-sitter grammar. The current contributing guide asks for at least 2,000 files per extension from the last year, spread across many users and repositories ([CONTRIBUTING.md](https://github.com/github-linguist/linguist/blob/main/CONTRIBUTING.md)). Code navigation needs a mature tree-sitter parser with tags queries, at the discretion of GitHub ([github/code-navigation](https://github.com/github/code-navigation)). Inference: a new research notation gets no GitHub highlighting by either route.

### 3.2 Failure modes of `web-tree-sitter`

The runtime and the grammar must agree on the ABI version. Tree-sitter 0.25.0, released on 1 February 2025, raised the ABI to 15 ([v0.25.0 notes](https://newreleases.io/project/github/tree-sitter/tree-sitter/release/v0.25.0)). The same release rewrote `web-tree-sitter` in TypeScript and published both CommonJS and ES modules. It also renamed `pattern` to `patternIndex` in query matches and deprecated `Language.query`. A runtime at 0.24.7 accepts ABI 13 to 14 only. It refuses an ABI 15 grammar with "Incompatible language version 15. Compatibility range 13 through 14".

Projects report this break. A VS Code extension could not load a Dart grammar in its packaged build for this reason ([nedonatelli/sidecar#400](https://github.com/nedonatelli/sidecar/issues/400)). Zed users asked for ABI 15 support, and the workaround was `tree-sitter generate --abi=14` ([zed-industries/zed#24632](https://github.com/zed-industries/zed/issues/24632)). A Python grammar package had to raise its runtime floor to 0.25 ([intersystems/tree-sitter-objectscript#56](https://github.com/intersystems/tree-sitter-objectscript/issues/56)). Inference: architecture A pins two runtimes (the Rust crate and `web-tree-sitter`) to one ABI, and architecture B pins one.

A second failure looks the same but has another cause. Two concurrent calls to `Parser.init` can make a grammar report language version 0 ([speclynx/apidom#703](https://github.com/speclynx/apidom/issues/703), [code-yeongyu/senpi#4748](https://github.com/code-yeongyu/senpi/issues/4748)). I read only search summaries of these two reports.

Bundlers do not find the runtime WASM file without help. The README of `web-tree-sitter` says that the loader looks for the WASM file beside the JavaScript file by default ([README](https://raw.githubusercontent.com/tree-sitter/tree-sitter/master/lib/binding_web/README.md)). It tells users to pass a `locateFile` function to `Parser.init`. It also tells webpack users to set `resolve.fallback.fs` to `false`, and Vite users to copy the WASM file into `public`. The README warns that some prebuilt grammar WASM files use an older linking format that newer runtimes cannot load.

Bundler issues show the need for these steps. In Vite development mode, the WASM file is missing from the pre-bundled dependencies ([vitejs/vite#19962](https://github.com/vitejs/vite/issues/19962)). With esbuild, the WASM file could not be imported because the package did not export it ([tree-sitter/tree-sitter#4208](https://github.com/tree-sitter/tree-sitter/pull/4208)). A VS Code extension pinned the CommonJS build of `web-tree-sitter` and copied its WASM file beside the bundle ([fabric8-analytics-vscode-extension#952](https://github.com/fabric8-analytics/fabric8-analytics-vscode-extension/pull/952)).

Memory management was manual until 2026. Parser, tree and query objects live in WASM linear memory, which the JavaScript garbage collector does not free. Each object needs a `delete()` call.

Projects that forgot this call report leaks. One editor leaked 2.06 GB after 4,000 edits, because each reparse kept the old tree ([iamnbutler/multibuffer#781](https://github.com/iamnbutler/multibuffer/issues/781)). A language server project opened an issue to document its tree disposal design ([OpenModelica/modelica-language-server#82](https://github.com/OpenModelica/modelica-language-server/issues/82)). An open upstream issue reports unbounded memory growth from a global query cursor, which `query.delete()` does not free ([tree-sitter/tree-sitter#5547](https://github.com/tree-sitter/tree-sitter/issues/5547)).

Tree-sitter 0.27.0 (30 August 2026) adds automatic release through `FinalizationRegistry` ([tree-sitter/tree-sitter#5225](https://github.com/tree-sitter/tree-sitter/pull/5225)). The pull request keeps `delete()`, so the registry is a fallback. The garbage collector decides when the registry runs. Inference: a long-lived language server should still call `delete()` on each replaced tree.

### 3.3 Error recovery and syntax error messages

Tree-sitter recovers from syntax errors, but it gives little material for error messages. The tree holds `ERROR` nodes and zero-width `MISSING` nodes. It does not hold a message, an expected-token list or the start of the error. In 2020, a user asked how to get an error message from an `ERROR` node to show a VS Code diagnostic ([tree-sitter/tree-sitter#650](https://github.com/tree-sitter/tree-sitter/issues/650)). Another 2020 issue quotes a maintainer: "we haven't yet built out functionality for error messages" ([tree-sitter/tree-sitter#833](https://github.com/tree-sitter/tree-sitter/issues/833)). That issue asks for a stop-at-first-error mode, the error position and a list of missing tokens.

Compiler teams found this limit in practice. In 2022, the Solang Solidity compiler team could not build messages better than "parse error" ([tree-sitter/tree-sitter#1631](https://github.com/tree-sitter/tree-sitter/issues/1631)). They also needed the set of tokens that the parser accepts at the error. They concluded that tree-sitter "is not usable in its current form, unless you can assume that the input is already valid". A 2022 essay from the Sorbet author makes the same point ([blog.jez.io](https://blog.jez.io/tree-sitter-limitations/)). It says that a declarative grammar gives error recovery free, but "places a ceiling on possibilities for future improvement".

Recent releases still fix error-node behaviour. Release 0.26.13 (23 August 2026) fixed `MISSING` nodes in queries and wide error nodes on unparseable input ([releases](https://github.com/tree-sitter/tree-sitter/releases)). Release 0.27.2 (7 November 2026) fixed terminal `ERROR` nodes in `ts_node_has_error`. Release 0.27.1 changed query test capture ranges as a breaking change. Inference: the original request asks for line and column syntax errors in a live editor. With tree-sitter, our code must turn `ERROR` and `MISSING` nodes into these messages.

### 3.4 Incremental parsing

Inference from the measurements below: incremental parsing matters little for files of a few hundred lines. Tree-sitter reuses the old tree after an edit. In one editor, this reuse cut the reparse of an 18,000-line file from 43.8 ms to 5.4 ms ([anthonyamaro15/nevi#309](https://github.com/anthonyamaro15/nevi/pull/309)). Inference: if parse time grows linearly with length, a full parse of 300 lines takes about 0.7 ms in that editor.

`rust-analyzer` reparses incrementally, but its own guide doubts the value. The reparse tries to contain a change to a single `{}` block ([syntax guide](https://rust-analyzer.github.io/book/contributing/syntax.html)). For this, the parser keeps curly braces paired even in invalid code. If the attempt fails, the `reparse` method parses the whole file again ([syntax crate source](https://rust-lang.github.io/rust-analyzer/src/syntax/lib.rs.html)). The guide says: "incremental reparsing doesn't actually matter much for IDE use-cases, parsing from scratch seems to be fast enough". A third-party issue adds a fuzz test that compares each incremental result with a full parse ([NyalephTheCat/leekwars#284](https://github.com/NyalephTheCat/leekwars/issues/284)).

Larger claims do not state the file size. `sass-analyzer` claims that its incremental parse is 110 times faster than a full parse ([`mykhlv/sass-analyzer`](https://github.com/mykhlv/sass-analyzer)). Langium reports parse times over 1,000 ms for files over 1 MB (section 3.6). Inference: a parser for our notation can parse the whole file after each edit. This design saves a second code path and the fuzz test that guards it. Incremental work then belongs in the compiler and the renderer, which can cache their results.

### 3.5 Architecture costs

A Rust core in WASM changes architecture A. Inference: if the core uses the `tree-sitter` crate, the browser loads one WASM module and needs no `web-tree-sitter`. The grammar then compiles into the same module as the runtime, so the ABI mismatch of section 3.2 cannot occur. The bundler must still load the WASM module, as for any WASM build. The C toolchain for `wasm32-unknown-unknown` becomes the main cost, as the next subsection shows. Architectures A and C then differ only in the parser inside the core.

#### Rust core on `wasm32-unknown-unknown` (architecture C, and A if the core is shared)

The `tree-sitter` Rust crate got support for `wasm32-unknown-unknown` in September 2025. Before that, the C runtime had no libc on this target. Pull request 4820 adds a small hand-written libc shim and headers in the `tree-sitter-language` crate ([tree-sitter/tree-sitter#4820](https://github.com/tree-sitter/tree-sitter/pull/4820)). Grammar crates compile their C parser and scanner against the shim. The build still needs a C compiler that targets WASM.

Gaps remain on this target. A later pull request added missing libc definitions, and a JavaScript grammar issue for this target is open. Pull request 5851 (2026) adds support for multi-threaded browser applications on this target ([tree-sitter/tree-sitter#5851](https://github.com/tree-sitter/tree-sitter/pull/5851)). A comment there reports that `tree-sitter` with `tree-sitter-c` does not compile for the target, even at 0.27. Before 2025, users needed the pure-Rust port `tree-sitter-c2rust` or nightly flags ([discussion #1550](https://github.com/tree-sitter/tree-sitter/discussions/1550)).

The `wasm` feature of the crate is a different thing. It loads grammar WASM files through wasmtime, and it does not build for `wasm32` ([tree-sitter/tree-sitter#4336](https://github.com/tree-sitter/tree-sitter/issues/4336)).

The build needs a clang with the WebAssembly back end, because GCC cannot target WebAssembly. The documentation of the `org-element` crate, which wraps a tree-sitter grammar, states the macOS problem ([docs.rs/org-element](https://docs.rs/org-element/latest/org_element/)). Apple `/usr/bin/clang` has no WebAssembly back end, and the build stops with "No available targets". The fix is LLVM from Homebrew, set through `CC_wasm32_unknown_unknown` and `AR_wasm32_unknown_unknown` in `.cargo/config.toml`. The setting must cover the whole build graph, because the runtime crate also compiles C. The iroh maintainers report the same limit of Apple clang ([n0-computer/iroh#3200](https://github.com/n0-computer/iroh/discussions/3200)).

The sources disagree on newer Apple clang. One 2026 pull request says that Apple clang 21 targets `wasm32` ([alexsave/foolish#224](https://github.com/alexsave/foolish/pull/224)). This is the only source with this claim. Most Linux clang builds include the WebAssembly back end, so continuous integration on Linux needs no extra step. Inference: the toolchain for architectures A and C on `wasm32-unknown-unknown` is Rust, `wasm-bindgen` and a clang with the WebAssembly back end.

#### Parsers inside a Rust core

The Rust core can use one of five parser designs. The table compares their error recovery, their error messages and their latest release.

| Parser | Design | Error recovery | Error messages | Latest release |
|---|---|---|---|---|
| `tree-sitter` crate | GLR parser generated from a JavaScript grammar, with a C runtime | Automatic. The tree holds `ERROR` and `MISSING` nodes. | No message and no set of expected tokens (section 3.3) | 0.27.2, 7 November 2026 |
| Hand-written parser over `rowan` | Recursive descent with Pratt parsing, into a lossless tree | Written by hand with FIRST sets and recovery sets | Written by hand, so the project controls the text | `rowan` 0.17.0, 2 August 2026. Alphas of 0.18 followed in September and October 2026 ([docs.rs](https://docs.rs/crate/rowan/latest)). |
| `chumsky` | Parser combinators with ordered choice, as in PEG | Strategies such as `skip_then_retry_until`, `skip_until` and `nested_delimiters` give many errors and a partial tree | `Rich` errors with found and expected items. The `ariadne` crate renders them. | 0.14.1, 13 November 2026 ([docs.rs](https://docs.rs/crate/chumsky/latest)). The 1.0 alpha line stopped at alpha.8 in January 2025. |
| LALRPOP | LR(1) parser generator | Off by default. The grammar adds recovery points with the `!` token. | `UnrecognizedToken` and `UnrecognizedEof` carry `expected: Vec<String>` | `lalrpop-util` 0.23.2, 19 October 2026 |
| pest | PEG parser generator with a `.pest` grammar file | None. The parser stops at the first error. | Expected and unexpected rules, with more detail from `set_error_detail` | 2.9.6, 10 November 2026 ([docs.rs](https://docs.rs/crate/pest/latest)) |

The hand-written design has the strongest evidence for editor use. Aleksey Kladov's tutorial defines the goal: "Resilient parsing means recovering as much syntactic structure from erroneous code as possible" ([matklad tutorial](https://matklad.github.io/2023/05/21/resilient-ll-parsing-tutorial.html)). The parser never stops. Each loop consumes a token or gives control to an outer loop, and recovery sets decide which. The tutorial says that "LL style parsers are a good fit for resilient parsing". In its example, tree-sitter keeps small valid pieces but does not build the broken function as a function node.

The `rust-analyzer` guide shows how such a parser reports errors ([syntax guide](https://rust-analyzer.github.io/book/contributing/syntax.html)). The parser wraps extra input in an `ERROR` node and leaves a missing node out of the tree. It sends each error to a list, not into the tree. Where it can, it leaves errors to a separate validation step after the parse.

The `rowan` crate supplies the tree, not the parser. It stores a lossless tree in the red and green design that came from Roslyn ([docs.rs/rowan](https://docs.rs/rowan)). The `cstree` crate is a fork with persistent red nodes, thread-safe nodes and string interning ([docs.rs/cstree](https://docs.rs/cstree/0.10.0/cstree/)). One author found that every example of `rowan` uses a hand-written parser ([kiranshila blog](https://blog.kiranshila.com/post/easy_cst)).

The generators have weaker recovery. The `chumsky` documentation warns that recovery can make the parser miss a valid route through the grammar ([docs.rs/chumsky](https://docs.rs/chumsky/latest/chumsky/trait.Parser.html)). LALRPOP stops at the first error unless the grammar adds `!` recovery points ([LALRPOP tutorial](https://lalrpop.github.io/lalrpop/tutorial/008_error_recovery.html)). Its documentation warns that the expected-token strings "may not necessarily be suitable for presenting to the user" ([docs.rs/lalrpop-util](https://docs.rs/lalrpop-util/latest/lalrpop_util/enum.ParseError.html)). Pest has no recovery, and one author left pest for this reason ([ebastien blog](https://ebastien.name/posts/parsing-in-rust/)). The pest3 discussion lists the return of many errors as a wanted change ([pest-parser/pest#885](https://github.com/pest-parser/pest/discussions/885)).

The sources disagree on the maintenance of `chumsky`. Lib.rs labels the crate "minimal maintenance" ([lib.rs](https://lib.rs/crates/chumsky)). Docs.rs lists eight releases from 0.10.0 in March 2025 to 0.14.1 on 13 November 2026 ([docs.rs](https://docs.rs/crate/chumsky/latest)). All of these releases came after the last 1.0 alpha.

Inference: a hand-written parser over `rowan` or `cstree` gives the most control of messages for a small notation. It costs more code than a generator, and the grammar exists only as code. A tree-sitter grammar gives recovery and highlight queries without code, but our code must write every message.

#### Rust language server crates

The original `tower-lsp` crate is dormant. Its last release is 0.20.0 from August 2023, and its maintainer does not answer ([ebkalderon/tower-lsp#427](https://github.com/ebkalderon/tower-lsp/issues/427)). The community fork `tower-lsp-server` continues the code ([tower-lsp-community/tower-lsp-server](https://github.com/tower-lsp-community/tower-lsp-server)). The Supabase Postgres language server moved to fork release 0.23 ([supabase-community/postgres-language-server#877](https://github.com/supabase-community/postgres-language-server/pull/877)). Search summaries name Biome and oxc as other users, but I did not check this. The other choice is `lsp-server` from `rust-analyzer`, with a hand-written main loop that controls cancellation and request order.

#### A language server compiled to WASM in VS Code

VS Code can run a language server compiled to WASI, but the support is a preview. Microsoft introduced `@vscode/wasm-wasi-lsp` in June 2024 ([VS Code blog, part two](https://code.visualstudio.com/blogs/2024/06/07/wasm-part2)). The extension starts the server as a worker under the WebAssembly Core extension (`ms-vscode.wasm-wasi-core`), which supports WASI Preview 1. The example server in the post is Rust, built on the `lsp-server` crate. The 2023 post called the WASM execution engine an experimental preview, not ready for production extensions ([VS Code blog, 2023](https://code.visualstudio.com/blogs/2023/06/05/vscode-wasm-wasi)).

The package is still a pre-release after more than two years. Its `latest` tag points to `0.1.0-pre.1` from April 2024 ([npm](https://www.npmjs.com/package/@vscode/wasm-wasi-lsp)). Its `next` tag points to `0.1.0-pre.9`, published on 4 March 2026 ([npm registry](https://registry.npmjs.org/@vscode/wasm-wasi-lsp)). A 2026 pull request in `wgsl-analyzer` uses it with the `wasm32-wasip1-threads` target ([wgsl-analyzer#1596](https://github.com/wgsl-analyzer/wgsl-analyzer/pull/1596)). That pull request quotes Microsoft: most of this is "still in an experimental state".

A PHP extension met concrete problems with the package ([Tusk-PHP/vscode#3](https://github.com/Tusk-PHP/vscode/pull/3)). The pre-release expects a pre-release of `vscode-languageclient` 10, so `vsce package` needed `--no-dependencies`. The web version needs `SharedArrayBuffer`, so the page must be cross-origin isolated. The WASI engine ignored the memory limit passed to `createProcess`. Inference: a WASI server is a fit for vscode.dev, and desktop VS Code should run a native binary.

#### Native binaries in a VS Code extension

VS Code installs a platform-specific package since release 1.61.0 ([publishing guide](https://code.visualstudio.com/api/working-with-extensions/publishing-extension)). The publisher builds one VSIX for each target with `vsce package --target`. A package without a target is the fallback for every platform without its own package. The targets include `win32-x64`, `linux-arm64`, `alpine-x64`, `darwin-arm64` and `web` ([microsoft/vscode-vsce#1105](https://github.com/microsoft/vscode-vsce/issues/1105)).

`rust-analyzer` uses this design. Each platform VSIX holds the server binary, and `rust-analyzer-no-server.vsix` serves other platforms ([`rust-analyzer` book](https://rust-analyzer.github.io/book/vs_code.html)). A 2026 language server project builds eight platform packages and one universal package without a server ([opentidehq/language-server#176](https://github.com/opentidehq/language-server/issues/176)). It publishes through trusted publishing with `vsce publish --oidc`. Inference: the `web` target can carry the WASM build, so one Rust core serves desktop VS Code and vscode.dev.

### 3.6 Reference points: architectures B, D and E

**B. Tree-sitter grammar with a TypeScript core.** Two language servers show that this design works. Microsoft `vscode-anycode` runs `web-tree-sitter` in a web worker for vscode.dev and github.dev ([microsoft/vscode-anycode](https://github.com/microsoft/vscode-anycode)). Its README says that it "inaccurately" implements outline, symbol search and highlights from tree-sitter queries. It reports tree-sitter error nodes as diagnostics.

The `bash-language-server` loads `tree-sitter-bash.wasm` through `web-tree-sitter` in Node ([bash-lsp/bash-language-server](https://github.com/bash-lsp/bash-language-server)). An upgrade script rebuilds the grammar with `tree-sitter build --wasm`.

**D. Langium.** Langium generates the parser, typed syntax tree, scoping, language server and VS Code extension from one grammar ([features](https://langium.org/docs/features/)). Its parser uses Chevrotain, so the grammar is LL(k) and cannot use left recursion.

In December 2025, TypeFox, the company behind Langium, wrote about its limits ([TypeFox blog](https://www.typefox.io/blog/xtext-langium-what-next/)). The parser spends about 60% of its run time on building the concrete syntax tree. For files above 1 MB, responses take more than 1,000 ms, and a parse cannot stop when a newer edit arrives.

TypeFox announced a new high-performance toolkit in Go for 2026, as a complement to Langium. The same post says that "Langium remains our number one choice for most projects". Inference: our grammar files have a few hundred lines, far below these limits. Langium does not fit the Rust core, because it is a TypeScript framework.

**E. Hand-written TypeScript parser.** This design gives full control of error messages and needs no WASM. It does not fit the Rust core. Section 3.5 covers the same design in Rust, with `rowan`.

### 3.7 The browser editor and a language server in a web worker

The browser tool can use the same language server as the editor extension. The LSP libraries for VS Code have a browser implementation since LSP 3.16. In this form, the server runs in a web worker and talks through `postMessage` ([web extensions guide](https://code.visualstudio.com/api/extension-guides/web-extensions)). The guide warns that a missing Cross-Origin-Resource-Policy header can stop such a server on vscode.dev.

CodeMirror 6 has an official LSP client since July 2025. Marijn Haverbeke announced `@codemirror/lsp-client` on 2 July 2025 ([announcement](https://discuss.codemirror.net/t/codemirror-lsp-client/9309)). It supports completion, hover, signature help, go to definition, rename, formatting and references. Release 6.1.0 (23 August 2025) added basic diagnostics ([npm](https://www.npmjs.com/package/@codemirror/lsp-client)). A community demonstration runs a TypeScript server in a web worker behind this client ([demonstration](https://discuss.codemirror.net/t/show-and-tell-codemirror-lsp-client-with-patched-ts-language-server-running-in-web-workers/9337)).

Monaco connects to language servers through `monaco-languageclient` from TypeFox ([TypeFox/monaco-languageclient](https://github.com/TypeFox/monaco-languageclient)). The library talks to a server over a WebSocket or through a web worker. Version 10 presents the library as a toolbox for editor applications ([TypeFox blog](https://www.typefox.io/blog/monaco-languageclient-v10/)). Its examples run a Langium language server in a web worker, and clangd compiled to WASM in the browser.

One project shows the cost of a client written from scratch. Its own Monaco client misses cases that `vscode-languageclient` handles ([eclipse-emfcloud/hydranium#433](https://github.com/eclipse-emfcloud/hydranium/issues/433)). One case is a server that answers with a position encoding other than UTF-16. LSP 3.17 lets a client and a server agree on UTF-8 positions through `positionEncoding`, and UTF-16 stays the default ([LSP 3.17](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/)). Inference: the Rust core counts columns in UTF-8 bytes, so it must convert positions for clients that use UTF-16.

### 3.8 The serialised form: KDL 2.0 libraries

KDL 2.0.0 is final, and the specification repository expects no further changes ([kdl-org/kdl](https://github.com/kdl-org/kdl)). The specification gives 21 December 2024 as the release date ([kdl.dev/spec](https://kdl.dev/spec/)). The KDL Schema Language is a separate draft, loosely based on JSON Schema ([SCHEMA-SPEC.md](https://github.com/kdl-org/kdl/blob/main/SCHEMA-SPEC.md)). Its references are KDL Query strings, and the 2.0.0 release notes say that KDL Query is not final ([releases](https://github.com/kdl-org/kdl/releases)). The implementation table of the repository warns that a listed library does not always pass the full test suite.

#### Rust libraries for KDL 2.0

The table compares the Rust libraries on KDL 2.0 support, derive support, error messages and maintenance.

| Library | KDL 2.0 | Derive | Error messages | Maintenance |
|---|---|---|---|---|
| [`kdl`](https://lib.rs/crates/kdl) (kdl-rs) | Yes, by default. KDL 1.0 needs the `v1` or `v1-fallback` feature. The crate converts documents between versions ([README](https://github.com/kdl-org/kdl-rs)). | No derive of its own. Release 6.7.1 has serde modules `de` and `se` ([docs.rs/kdl](https://docs.rs/kdl/latest/kdl/)). A document API keeps formatting and comments, like `toml_edit`. | `KdlError` implements `miette::Diagnostic`, with spans, labels and help text. The 2.0 parser collects many errors in one pass ([zellij#3891](https://github.com/zellij-org/zellij/issues/3891)). | 6.7.1, 31 May 2026. 37 releases since December 2020, 262,678 downloads per month, Apache-2.0. |
| `knuffel` | No. Issue 35 only tracks KDL 2.0 ([knuffel#35](https://github.com/tailhook/knuffel/issues/35)). | `Decode` derive, the API that `knus` keeps | Not checked | Marked deprecated in the KDL implementation tracker ([kdl-org/kdl#372](https://github.com/kdl-org/kdl/issues/372)) |
| [`knus`](https://lib.rs/crates/knus) | No. The KDL table lists version 1 only. Issue 62 proposes a move to the `kdl` parser to gain version 2 ([knus#62](https://github.com/TheLostLambda/knus/issues/62)). | `#[derive(knus::Decode)]`. The KDL table calls it Serde-style, not Serde. | Reports through `miette`. The README example labels the invalid value. | 3.4.1, 2 November 2026. 21,606 downloads per month. The maintainer reviews pull requests but has little time for code. |
| [`facet-kdl`](https://docs.rs/facet-kdl/latest/facet_kdl/) | Not stated. Inference: version 2, because the crate requires `kdl` 6.5 or later. | `#[derive(Facet)]` with `kdl::argument`, `kdl::property`, `kdl::child` and `kdl::children` attributes | `KdlDeserializeError` carries source context. The crate requires `miette` 7. | 0.42.0, docs.rs date 4 November 2026. In the `facet` repository, MIT or Apache-2.0. |
| [`kdl-serde`](https://docs.rs/kdl-serde/latest/kdl_serde/), [`club-kdl`](https://crates.io/crates/club-kdl-compose) | Yes, by their descriptions | Serde derive (`kdl-serde`). Own derive (`club-kdl`). | Not checked | Not checked. I found them through search only. |

The `kdl` crate has KDL 2.0 support, 37 releases and 157 crates that depend on it directly. It has no derive of its own. Its serde modules map a document through the serde data model. The `kdl-serde` documentation groups this support with crates that treat a document like JSON ([docs.rs/kdl-serde](https://docs.rs/kdl-serde/latest/kdl_serde/)). Inference: this view hides the difference between arguments, properties and child nodes, which the attribute-based derives keep.

The `knus` README asks for co-maintainers and proposes a merge with `kdl-rs`: "we could bring the derive macro, they could bring the parser!" ([lib.rs/crates/knus](https://lib.rs/crates/knus)). The README also says that `facet` does not plan to support KDL. Inference: that sentence is out of date, because `facet-kdl` is in the `facet` repository. A November 2026 assessment in another project reports that the `knus` maintainer names `facet-kdl` as the successor ([developing-today/code#5753](https://github.com/developing-today/code/issues/5753)). I did not find this statement in the `knus` README.

Facet derives use runtime reflection. The facet FAQ says that facet is not faster than serde, and that one type can derive both ([facet FAQ](https://facet.rs/guide/faq/)). The older `facet-kdl` API moved to `facet-kdl-legacy` ([docs.rs/facet-kdl-legacy](https://docs.rs/facet-kdl-legacy/latest/facet_kdl_legacy/)). Inference: version 0.42 and a legacy crate mean that the API can change between releases.

#### Keeping the KDL mapping in step with the types

Three designs keep the mapping close to the Rust types. A derive puts the mapping on the type as field attributes, in `facet-kdl` or `knus`. A change to a field then changes the mapping in the same place. A hand-written mapping over the `kdl` document API keeps the formatting and comments of files that people edit. The serde modules of `kdl`, or the `kdl-serde` crate, reuse serde derives, which the Rust types can carry for other formats.

No design gives a checked schema for other languages (next subsection). Inference: round-trip tests hold the mapping in place. A test decodes each example file, encodes the value again and compares the text with a snapshot. The Python and TypeScript readers then use the same example files as test input.

#### What the missing schema costs

Inference: the missing schema costs little while the Rust core is the only decoder. The cost grows with each language that decodes KDL itself. Without a schema, an editor cannot check or complete a KDL file, as VS Code does for JSON with JSON Schema ([VS Code JSON](https://code.visualstudio.com/docs/languages/json)). Each consumer language must check the structure in its own decoder. A change to a Rust type reaches other languages only through failing tests.

Inference: two designs reduce these costs. Other languages can call the Rust core, in WASM or as a native library, and not decode KDL themselves. The language server of the Rust core can also report errors in KDL files in the editor.

The KDL Schema draft cannot fill this gap yet, because KDL Query is not final. An implementer of a schema validator reported open questions on the draft ([kdl-org/kdl#315](https://github.com/kdl-org/kdl/issues/315)). Two crates point at other routes, but I found them through search only. `kdl-config` has a `KdlSchema` derive and a `validation` module, and I did not check what the derive writes ([docs.rs/kdl-config](https://docs.rs/kdl-config/latest/kdl_config/)). `club-kdl-codegen` goes the other way: it generates Rust, TypeScript and Zod code from a KDL schema ([chronista-club/club-kdl](https://github.com/chronista-club/club-kdl)).

#### KDL in TypeScript and Python

The table lists the KDL libraries for JavaScript and Python. The KDL 2.0 column comes from the implementation table of the specification repository ([kdl-org/kdl](https://github.com/kdl-org/kdl)). The notes come from each project.

| Library | Language | KDL 2.0 | Notes |
|---|---|---|---|
| [`@bgotink/kdl`](https://github.com/bgotink/kdl) | JavaScript. Inference: written in TypeScript, because the repository has a `tsconfig.json`. | Yes. `parseCompat` in `@bgotink/kdl/v1-compat` also reads version 1. | Keeps formatting and comments, and runs in browsers. `@bgotink/kdl/dessert` decodes with typed accessors. Version 0.4.x, MIT. |
| [`kdljs`](https://github.com/kdl-org/kdljs) | JavaScript | Yes | Returns plain objects. I did not find the release that added version 2. |
| [`@virtualstate/kdl`](https://github.com/virtualstate/kdl) | JavaScript | No | Query only, based on JSX |
| [`kdl-py`](https://github.com/tabatkins/kdlpy) | Python 3.7 or later | Yes. The README says that it complies fully with 2.0.0. | Hand-written. Converts tagged values to Python types, and ships `kdlreformat`. |
| [`ckdl`](https://github.com/tjol/ckdl) | C, with Python and C++ bindings | Yes, and version 1 | Passes the upstream test suite. The README and [the ReadTheDocs page](https://ckdl.readthedocs.io/en/latest/) disagree on the default version. |
| [`kdlquery`](https://pypi.org/project/kdlquery/) | Python 3.10 or later | Yes, by its description | Pure Python with a CSS-style selector API. Version 1.4.1. |
| [`cuddle`](https://github.com/djmattyg007/python-cuddle) | Python | No | None |

The specification table has no TypeScript row. Inference: the browser tool needs no TypeScript KDL parser, because the Rust core can parse KDL in WASM and pass typed values to TypeScript.

#### TypeScript declarations at the WASM boundary

The `tsify` crate generates TypeScript declarations from Rust types for `wasm-bindgen`. A type derives `Tsify` beside the serde traits, and `wasm-bindgen` writes the declarations into its `.d.ts` file ([madonoharu/tsify](https://github.com/madonoharu/tsify)). The fork `tsify-next` is deprecated, because the original crate again has all its features ([siefkenj/tsify](https://github.com/siefkenj/tsify)). The latest release of `tsify` is 0.5.8. Its features choose the conversion: `json`, the default, uses `serde_json`, and `js` uses `serde-wasm-bindgen`.

Release 0.5.8 deprecates the attributes `into_wasm_abi` and `from_wasm_abi`. The README explains that the ABI boundary cannot report a failure. A serde error then becomes a JavaScript exception through `wasm_bindgen::throw_str`, and the exception unwinds past the WASM frames. The new wrapper `Ts<T>` converts inside the function, which returns `Result<_, JsError>`. Inference: with `Ts<T>`, a bad grammar or trace value reaches the browser tool as an error value. CosmWasm moved its TypeScript types from JSON Schema to `ts-rs`, which writes declaration files without `wasm-bindgen` ([CosmWasm/cosmwasm#2093](https://github.com/CosmWasm/cosmwasm/issues/2093)).

### 3.9 What the evidence supports

Inference: the evidence supports architecture C with a hand-written parser over `rowan` or `cstree`. The Rust core then holds the one source of truth for syntax and semantics. It serves the browser tool through WASM, desktop VS Code through a native server and vscode.dev through the `web` target. It writes its own error messages, which tree-sitter does not give. It needs no C toolchain for `wasm32-unknown-unknown` and no ABI match between two runtimes.

Inference: the proposal of the human stays the fallback in two cases. In the first case, the notation grows until a hand-written parser costs more than a grammar file. In the second case, users want Neovim, Helix or Zed early, because these editors load tree-sitter grammars. In both designs, VS Code colours text from a TextMate grammar or from semantic tokens (section 3.1). Semantic tokens from the language server keep the colour rules in the Rust core. I did not check whether `@codemirror/lsp-client` supports semantic tokens.

## 4. Systems examined

The walk-away column answers one question: when the tool is gone, what stays legible? A cell with "not checked" marks a gap.

| Name | URL | Purpose | Licence | Latest release | Maintenance | Walk-away result |
|---|---|---|---|---|---|---|
| tree-sitter (CLI, C runtime, Rust crate) | https://github.com/tree-sitter/tree-sitter | Parser generator and incremental parser runtime | Inference: MIT, as for `web-tree-sitter` in the same repository. | 0.27.2, 7 November 2026 ([docs.rs](https://docs.rs/crate/tree-sitter/latest)) | Active. Docs.rs lists twelve releases from 1 February to 7 November 2026. | `grammar.js`, corpus tests and queries stay legible. The generated `parser.c` is not for people. Queries need one copy for each editor. |
| `web-tree-sitter` | https://www.npmjs.com/package/web-tree-sitter | Tree-sitter in browsers and Node through WASM | MIT ([npm](https://registry.npmjs.org/web-tree-sitter/latest)) | 0.27.2, released with tree-sitter. 0.27.0 added `FinalizationRegistry`. | Active | Old grammar WASM files can stop loading after an ABI change. |
| VS Code highlighting | https://github.com/microsoft/vscode | TextMate grammars and semantic tokens | Not checked | Not checked | Active | A TextMate grammar also serves GitHub Linguist. |
| `tower-lsp`, `tower-lsp-server` | https://github.com/tower-lsp-community/tower-lsp-server | Rust language server framework | Not checked | `tower-lsp` 0.20.0, August 2023. Supabase uses fork release 0.23. | `tower-lsp` is dormant. The fork is active. | Inference: server logic over LSP types stays. Only the main loop changes. |
| `@vscode/wasm-wasi-lsp` | https://www.npmjs.com/package/@vscode/wasm-wasi-lsp | WASI language servers in VS Code | MIT | `latest` tag 0.1.0-pre.1, April 2024. `next` tag 0.1.0-pre.9, 4 March 2026. | Preview. Nine pre-releases and no stable release. | Inference: the server stays a standard WASI binary. |
| Langium | https://langium.org | Parser, syntax tree, language server and VS Code extension from one grammar | MIT | 4.4.2 ([npm](https://registry.npmjs.org/langium/latest)). Date not checked. | Active. The repository is now `eclipse-langium/langium`. TypeFox announced a toolkit in Go for 2026. | Inference: the `.langium` grammar stays legible, but the services depend on the framework. |
| `rowan` | https://github.com/rust-analyzer/rowan | Lossless syntax tree for hand-written parsers | MIT or Apache-2.0 | 0.17.0, 2 August 2026 | Active. Alphas of 0.18 in 2026. | Inference: the parser is plain Rust. A port replaces only the tree type. |
| `chumsky` | https://github.com/zesterer/chumsky | Parser combinators with error recovery | MIT | 0.14.1, 13 November 2026 | Sources disagree (section 3.5) | Inference: the grammar is combinator code, so a port means a rewrite. |
| LALRPOP | https://github.com/lalrpop/lalrpop | LR(1) parser generator | Not checked | 0.23.2, 19 October 2026 ([docs.rs](https://docs.rs/crate/lalrpop/latest)) | Active | Inference: the `.lalrpop` grammar is close to BNF and stays legible. |
| pest | https://github.com/pest-parser/pest | PEG parser generator | Not checked | 2.9.6, 10 November 2026 | Active. Three releases from October to November 2026. pest3 is under discussion. | Inference: the `.pest` grammar stays legible. |
| `@codemirror/lsp-client` | https://www.npmjs.com/package/@codemirror/lsp-client | LSP client for CodeMirror 6 | MIT | 6.3.1, 16 November 2026 ([npm](https://registry.npmjs.org/@codemirror/lsp-client)) | Active. First release on 2 July 2025. 6.1.0 added diagnostics. | The server speaks standard LSP, so only client code is lost. |
| `monaco-languageclient` | https://github.com/TypeFox/monaco-languageclient | LSP client for Monaco | MIT | 11.0.3 ([npm](https://registry.npmjs.org/monaco-languageclient/latest)). Date not checked. | Active | The server speaks standard LSP, so only client code is lost. |
| `kdl` (kdl-rs) | https://github.com/kdl-org/kdl-rs | KDL document parser that keeps formatting | Apache-2.0 | 6.7.1, 31 May 2026 | Active. 37 releases since 2020. | KDL files are plain text under a final specification, with parsers in many languages. |
| `knus` | https://github.com/TheLostLambda/knus | KDL 1 derive decoder, a fork of `knuffel` | MIT or Apache-2.0 | 3.4.1, 2 November 2026 | The maintainer asks for co-maintainers. `knuffel` is deprecated. | Data files stay plain KDL 1. |
| `facet-kdl` | https://github.com/facet-rs/facet | KDL mapping through `facet` reflection | MIT or Apache-2.0 | 0.42.0, November 2026 | Active. The older API moved to `facet-kdl-legacy`. | Data files stay plain KDL. |
| `@bgotink/kdl` | https://github.com/bgotink/kdl | KDL parser for JavaScript that keeps formatting | MIT | 0.4.x | Active | Data files stay plain KDL. |
| `kdl-py`, `ckdl` | https://github.com/tabatkins/kdlpy, https://github.com/tjol/ckdl | KDL parsers for Python | Not checked | Not checked | Not checked | Data files stay plain KDL. |
| `tsify` | https://github.com/madonoharu/tsify | TypeScript declarations for `wasm-bindgen` | Not checked | 0.5.8, 23 August 2026 | Active again. `tsify-next` is deprecated. | The generated `.d.ts` files are plain TypeScript. |
| `expect-test` | https://github.com/rust-analyzer/expect-test | Inline snapshot tests | Not checked | 1.5.1, 21 December 2024 ([docs.rs](https://docs.rs/crate/expect-test/latest)) | Used by `rust-analyzer`. No release since December 2024. | Snapshots are plain text in the test source. |

## 5. Silences

- I found no official VS Code release note from 2025 or 2026 that lets an extension contribute a tree-sitter grammar for highlighting. Queries: "VS Code release notes tree-sitter highlighting extensions contribute grammar", "VS Code tree-sitter syntax highlighting experimental preferTreeSitter TextMate 2025".
- I found no statement of the KDL version that `facet-kdl` reads and writes. Query: "facet-kdl crate facet reflection KDL 2.0 deserialize serialize". I also read the docs.rs page of the crate.
- I found no checked tool that writes a KDL schema from Rust types. Query: "generate KDL schema from Rust types derive "KDL schema" crate validation".
- I found no versioned release of the KDL Schema Language and no list of schema validators. Query: "KDL schema language KDL 2.0 validation schema spec status kdl-org".
- I found no plan for `@vscode/wasm-wasi-lsp` to leave pre-release. Query: "@vscode/wasm-wasi-lsp language server WebAssembly VS Code extension experimental".
- I found no parse-time measurement for files of a few hundred lines. Query: "rust-analyzer incremental reparsing block token "full reparse" fast enough syntax tree".
- I found no statement that `@bgotink/kdl` or `kdljs` ships TypeScript declarations. Query: "KDL 2.0 JavaScript TypeScript library @bgotink/kdl kdljs KDL v2 support". I also read the README of `@bgotink/kdl`.

## 6. Sources

- https://deepwiki.com/tree-sitter/tree-sitter-java/4-testing-framework
- https://github.com/tree-sitter/tree-sitter/pull/499
- https://github.com/gap-system/tree-sitter-gap
- https://rust-analyzer.github.io/book/contributing/architecture.html
- https://github.com/rust-analyzer/expect-test/blob/master/src/lib.rs
- https://matklad.github.io/2021/05/31/how-to-test.html
- https://langium.org/docs/learn/workflow/create_validations/
- https://langium.org/docs/learn/workflow/generate_everything/
- https://github.com/microsoft/vscode/issues/50140
- https://code.visualstudio.com/updates/v1_97
- https://code.visualstudio.com/updates/v1_99
- https://github.com/microsoft/vscode/issues/244444
- https://github.com/microsoft/vscode/issues/244445
- https://marketplace.visualstudio.com/items?itemName=AlecGhost.tree-sitter-vscode
- https://marketplace.visualstudio.com/items?itemName=evgeniypeshkov.syntax-highlighter
- https://github.com/zed-industries/zed/issues/40532
- https://github.com/elitekaycy/qkt/pull/1584
- https://github.com/alemuller/tree-sitter-vhdl/issues/1
- https://github.com/c3lang/c3-ts-mode
- https://www.masteringemacs.org/article/how-to-get-started-tree-sitter
- https://github.com/github/linguist/discussions/6073
- https://github.com/github-linguist/linguist/blob/main/CONTRIBUTING.md
- https://github.com/github/code-navigation
- https://newreleases.io/project/github/tree-sitter/tree-sitter/release/v0.25.0
- https://github.com/nedonatelli/sidecar/issues/400
- https://github.com/zed-industries/zed/issues/24632
- https://github.com/intersystems/tree-sitter-objectscript/issues/56
- https://github.com/speclynx/apidom/issues/703
- https://github.com/code-yeongyu/senpi/issues/4748
- https://raw.githubusercontent.com/tree-sitter/tree-sitter/master/lib/binding_web/README.md
- https://github.com/vitejs/vite/issues/19962
- https://github.com/tree-sitter/tree-sitter/pull/4208
- https://github.com/fabric8-analytics/fabric8-analytics-vscode-extension/pull/952
- https://github.com/iamnbutler/multibuffer/issues/781
- https://github.com/OpenModelica/modelica-language-server/issues/82
- https://github.com/tree-sitter/tree-sitter/issues/5547
- https://github.com/tree-sitter/tree-sitter/pull/5225
- https://github.com/tree-sitter/tree-sitter/issues/650
- https://github.com/tree-sitter/tree-sitter/issues/833
- https://github.com/tree-sitter/tree-sitter/issues/1631
- https://blog.jez.io/tree-sitter-limitations/
- https://github.com/tree-sitter/tree-sitter/releases
- https://github.com/anthonyamaro15/nevi/pull/309
- https://rust-analyzer.github.io/book/contributing/syntax.html
- https://rust-lang.github.io/rust-analyzer/src/syntax/lib.rs.html
- https://github.com/NyalephTheCat/leekwars/issues/284
- https://github.com/mykhlv/sass-analyzer
- https://github.com/tree-sitter/tree-sitter/pull/4820
- https://github.com/tree-sitter/tree-sitter/pull/5851
- https://github.com/tree-sitter/tree-sitter/discussions/1550
- https://github.com/tree-sitter/tree-sitter/issues/4336
- https://docs.rs/org-element/latest/org_element/
- https://github.com/n0-computer/iroh/discussions/3200
- https://github.com/alexsave/foolish/pull/224
- https://docs.rs/crate/rowan/latest
- https://docs.rs/crate/chumsky/latest
- https://docs.rs/crate/pest/latest
- https://matklad.github.io/2023/05/21/resilient-ll-parsing-tutorial.html
- https://docs.rs/rowan
- https://docs.rs/cstree/0.10.0/cstree/
- https://blog.kiranshila.com/post/easy_cst
- https://docs.rs/chumsky/latest/chumsky/trait.Parser.html
- https://lalrpop.github.io/lalrpop/tutorial/008_error_recovery.html
- https://docs.rs/lalrpop-util/latest/lalrpop_util/enum.ParseError.html
- https://ebastien.name/posts/parsing-in-rust/
- https://github.com/pest-parser/pest/discussions/885
- https://lib.rs/crates/chumsky
- https://github.com/ebkalderon/tower-lsp/issues/427
- https://github.com/tower-lsp-community/tower-lsp-server
- https://github.com/supabase-community/postgres-language-server/pull/877
- https://code.visualstudio.com/blogs/2024/06/07/wasm-part2
- https://code.visualstudio.com/blogs/2023/06/05/vscode-wasm-wasi
- https://www.npmjs.com/package/@vscode/wasm-wasi-lsp
- https://registry.npmjs.org/@vscode/wasm-wasi-lsp
- https://github.com/wgsl-analyzer/wgsl-analyzer/pull/1596
- https://github.com/Tusk-PHP/vscode/pull/3
- https://code.visualstudio.com/api/working-with-extensions/publishing-extension
- https://github.com/microsoft/vscode-vsce/issues/1105
- https://rust-analyzer.github.io/book/vs_code.html
- https://github.com/opentidehq/language-server/issues/176
- https://github.com/microsoft/vscode-anycode
- https://github.com/bash-lsp/bash-language-server
- https://langium.org/docs/features/
- https://www.typefox.io/blog/xtext-langium-what-next/
- https://code.visualstudio.com/api/extension-guides/web-extensions
- https://discuss.codemirror.net/t/codemirror-lsp-client/9309
- https://www.npmjs.com/package/@codemirror/lsp-client
- https://discuss.codemirror.net/t/show-and-tell-codemirror-lsp-client-with-patched-ts-language-server-running-in-web-workers/9337
- https://github.com/TypeFox/monaco-languageclient
- https://www.typefox.io/blog/monaco-languageclient-v10/
- https://github.com/eclipse-emfcloud/hydranium/issues/433
- https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/
- https://github.com/kdl-org/kdl
- https://kdl.dev/spec/
- https://github.com/kdl-org/kdl/blob/main/SCHEMA-SPEC.md
- https://github.com/kdl-org/kdl/releases
- https://lib.rs/crates/kdl
- https://github.com/kdl-org/kdl-rs
- https://docs.rs/kdl/latest/kdl/
- https://github.com/zellij-org/zellij/issues/3891
- https://github.com/tailhook/knuffel/issues/35
- https://github.com/kdl-org/kdl/issues/372
- https://lib.rs/crates/knus
- https://github.com/TheLostLambda/knus/issues/62
- https://docs.rs/facet-kdl/latest/facet_kdl/
- https://docs.rs/kdl-serde/latest/kdl_serde/
- https://crates.io/crates/club-kdl-compose
- https://github.com/developing-today/code/issues/5753
- https://facet.rs/guide/faq/
- https://docs.rs/facet-kdl-legacy/latest/facet_kdl_legacy/
- https://code.visualstudio.com/docs/languages/json
- https://github.com/kdl-org/kdl/issues/315
- https://docs.rs/kdl-config/latest/kdl_config/
- https://github.com/chronista-club/club-kdl
- https://github.com/bgotink/kdl
- https://github.com/kdl-org/kdljs
- https://github.com/virtualstate/kdl
- https://github.com/tabatkins/kdlpy
- https://github.com/tjol/ckdl
- https://ckdl.readthedocs.io/en/latest/
- https://pypi.org/project/kdlquery/
- https://github.com/djmattyg007/python-cuddle
- https://github.com/madonoharu/tsify
- https://github.com/siefkenj/tsify
- https://github.com/CosmWasm/cosmwasm/issues/2093
- https://github.com/tree-sitter/tree-sitter
- https://docs.rs/crate/tree-sitter/latest
- https://www.npmjs.com/package/web-tree-sitter
- https://registry.npmjs.org/web-tree-sitter/latest
- https://github.com/microsoft/vscode
- https://langium.org
- https://registry.npmjs.org/langium/latest
- https://github.com/rust-analyzer/rowan
- https://github.com/zesterer/chumsky
- https://github.com/lalrpop/lalrpop
- https://docs.rs/crate/lalrpop/latest
- https://github.com/pest-parser/pest
- https://registry.npmjs.org/@codemirror/lsp-client
- https://registry.npmjs.org/monaco-languageclient/latest
- https://github.com/TheLostLambda/knus
- https://github.com/facet-rs/facet
- https://github.com/rust-analyzer/expect-test
- https://docs.rs/crate/expect-test/latest

## 7. Decision records

These decision records cite this survey:

- `docs/adr/rust-core-with-webassembly.md`: one Rust core serves native hosts directly and the browser through WebAssembly.
- `docs/adr/hand-written-parser.md`: a hand-written parser over a lossless `rowan` tree replaces tree-sitter.
- `docs/adr/kdl-for-derived-artifacts.md`: one mapping module over the `kdl` crate keeps the KDL format in step with the Rust types.
- `docs/adr/typed-objects-at-the-webassembly-boundary.md`: the page calls the core through typed objects from `tsify`, and never parses KDL.
