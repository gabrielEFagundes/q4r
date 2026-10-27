# AGENTS.md

Guidance for AI coding agents (and humans) contributing to **Quaoar** (repo: `q4r`, CLI: `quac`).

Quaoar is a compiled language aiming for the speed of Rust, the simplicity of Go, and a touch of C. The compiler is currently written in **Rust** and has no LLVM dependency.

## Project status

| Version | Backend | Status |
|---|---|---|
| v0.1 | C transpiler (single pass over tokens, **no AST**) | Working |
| v0.2+ | Native codegen via QBE (fork: "QBEAK"), **uses an AST** | In progress |

Long-term roadmap ends with a kernel written in Quaoar. Web/frontend targets are a very distant idea, so build cautiously toward them.

Reference docs: [v0.1 Manifest](../docs/versions/v0.1/MANIFEST.md), [v0.2 Manifest](../docs/versions/v0.2/MANIFEST.md), [Quaoar Docs](../docs/syntax/BASIC-SYNTAX.md). Read the relevant one before changing architecture.

> KEEP IN MIND that those files ARE NOT FINAL (besides the v0.1 MANIFEST), they are under risk of change until this warning is removed from here.

## Build and run

```sh
cargo build            # builds the `quac` binary
cargo test             # run before every PR
cargo fmt && cargo clippy
```

The v0.1 C backend shells out to `gcc` or `zig cc`. One of them must be on `PATH` to run compiled output.

<!-- TODO(maintainer): confirm test layout, example/ directory, and any extra commands. -->

## Architecture (v0.1)

Pipeline: **lexer → pass 1 (signature table) → pass 2 (codegen) → external C compiler**.

- **Lexer** synthesizes semicolons Go-style (see "Language rules"). Literal tokens store a **span**, not an owned `String`.
- **Pass 1** collects function signatures, including `extern` declarations, so forward references work.
- **Pass 2** emits code. It skips `extern` functions (bodyless, only a forward declaration is emitted).
- **Codegen abstraction:** trait `CodeGen` (`generate()`, `generate_headers()`), implemented by `CCompiler`. Shared state (source, tokens, cursor, signatures, scopes) lives in `Backend<B>`, which **owns** the implementer as `backend: B`. Keep this owning design; don't switch to parameter-passing.
- **Expression parser:** `parse_expr` (binary loop, the only public entry) → `parse_unary` (prefix `*` / `&`) → `parse_primary` (literals, identifiers, calls, parenthesized). Compound assignment (`+=`, `-=`, ...) is handled at **statement level**, not in the expression parser.
- `&` / `*` are disambiguated from binary `&` / `*` by **position** (prefix vs infix), not by token kind.

## Architecture (v0.2, planned)

- AST is a flat `Vec<StmtType>` at top level. Depth comes from nesting: expressions contain expressions, blocks contain `Vec<StmtType>`.
- Expressions use `Box<ExpType>` for now; migration to arena allocation is planned for v0.2.
- Backend is QBE. Semantic checking and real diagnostics belong here, not in v0.1.
- The C Backend will be kept as a secondary option for compilation, though it won't be fully supported, it should still be updated normally.

## Language rules (don't break these)

- **No explicit semicolons.** The lexer inserts a semicolon token after a newline only if the previous token can end a statement (identifier, literal, closing bracket, etc.). The parser then calls `expect(Semicolon)` as if it were explicit.
- **Memory management is optional.** Managed by default, manual when asked for:
  - `string word = "..."` is managed.
  - `*char word = malloc(10)` is manual raw pointer use.
  - An `unsafe {}`-style block for full manual control is planned.
- **String literals are `*char`.** There is no `str` type yet (v0.1).
- **C interop** uses `extern`, singly or as `extern { ... }` blocks. Variadic `...` is supported.
- Planned, not implemented: inline assembly blocks, `cross` (jump tables/switch statements), builtin HTTP and database libraries, and the `weywot` package manager. Don't half-implement these without opening an issue first.

## Conventions

- **v0.1 does not do rigorous type checking.** Type errors are intentionally left to gcc's diagnostics. Don't add a type checker to the v0.1 backend; propose it for v0.2.
- Keep `quac`'s own pipeline fast. The compile-time bottleneck is the external C compiler subprocess, not our code, so don't micro-optimize the pipeline for that.
- Accepted costs: `Literal::StringLiteral` uses `EcoString`
- Prefer small, focused PRs. One language feature or one fix per PR.
- Run `cargo fmt`, `cargo clippy`, and `cargo test` before submitting, even though the repo have workflows for this.
<!-- - New language features need: a parser change, codegen change, an example program, and an update to `quaoar-docs.md`. -->

## For AI agents specifically

- Read the architecture docs above before touching parser or codegen structure.
- Don't change the `CodeGen` / `Backend<B>` ownership design or the parser layering without a maintainer-approved issue.
- Don't introduce new dependencies (crates) without justification in the PR.
- Don't invent syntax. If a language feature isn't in [Quaoar Syntax Docs](../docs/syntax/BASIC-SYNTAX.md), ask rather than guess.
- Don't add LLVM. It is an explicit non-goal for now.
- Keep diffs minimal. No drive-by refactors or reformatting of unrelated code.

## Naming

Source extension: `.qo` Package manager: `weywot`. QBE-based backend fork: QBEak.