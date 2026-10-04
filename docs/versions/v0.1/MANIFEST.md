# Quaoar (Q4r) — v0.1 Architecture Notes

## Pipeline

```
source.qo
    │
    ▼
Lexer (runs ONCE)
    │
    ▼
Token stream (spans into source, not owned strings)
    │
    ├──▶ Pass 1 (shallow walk) ──▶ SignatureTable
    │                                     │
    └──▶ Pass 2 (full parse + emit) ◀─────┘
                    │
                    ▼
            C source (Vec<u8>)
                    │
                    ▼
        piped to `zig cc -x c -o compiled -`
                    │
                    ▼
                 binary
```

## Lexer

- Tokens store **byte spans** `(start: u32, end: u32)` into `source`, not owned `String`s — "zero-copy lexing" / "lazy conversion." Text is only materialized (`&source[start..end]`) the moment something actually needs it (identifiers, literals). Fixed tokens (keywords, symbols) never need source bytes again after lexing.

- Keyword/symbol tables: **sorted once by hand at declaration**, not re-sorted at runtime (a `const` re-inlines a fresh copy on every use — sorting it in place is a no-op that silently discards itself). Looked up via `binary_search_by` on byte slices.

- Literal variants (`IntLiteral`, `FloatLiteral`, `CharLiteral`...) are **separate** from keyword variants (`Int`, `Float`...) — one names a type, one carries a value. They need different shapes (a literal needs a payload).

## Two-pass architecture (no AST)

Modeled on Lua/Wren's single-pass compilers: **no persistent whole-program tree** gets built and walked afterward. Parsing and code emission happen in the same walk.

This does **not** forbid small transient structs (Lua's own `expdesc` is exactly this — a per-expression descriptor, built, consumed by codegen, discarded). The test: does the struct get stored in something representing the *whole file*, walked again later? → that's an AST. Does it get built, handed straight to a codegen call, and dropped? → still single-pass.

- **Pass 1** (shallow): walks the token stream at brace-depth 0 only. Recognizes function headers and top-level variable declarations. Skips function bodies entirely (track `{`/`}` depth, jump to the matching close). Builds:
  ```rust
  enum Symbol { Function { params: Vec<Type>, return_type: Type }, Global { ty: Type } }
  type SignatureTable = HashMap<String, Symbol>;
  ```
- **Pass 2** (full parse + emit): walks the same token stream again, top to bottom, this time actually parsing every construct and emitting C text as it goes. Any call site or global reference can resolve against `SignatureTable` regardless of where it's defined in the file — solves the forward-reference problem without needing a real AST.

## Type system

- `Type` is a **separate, recursive** enum from `TokenType` — `TokenType` is flat (one variant per lexical category); `Type` needs to nest (`Type::Pointer(Box<Type>)`), which a flat enum structurally can't do.
- Base types (`Int`, `Char`, `Float`, `Bool`, `Void`) map straight from a keyword token — no payload needed.
- **Pointers are the one place real structure is required**, because Quaoar's declared syntax is prefix (`*char x`) while C's is postfix (`char *x`):
  ```rust
  fn parse_type(&mut self) -> Type {
      if self.current_is(Asterisk) {
          self.advance();
          let inner = self.parse_type(); // recurse — handles **int etc.
          Type::Pointer(Box::new(inner))
      } else {
          let ty = base_type_from_token(self.current_token_type());
          self.advance();
          ty
      }
  }

  fn emit_type(ty: &Type, out: &mut Vec<u8>) {
      match ty {
          Type::Pointer(inner) => { emit_type(inner, out); out.push(b'*'); } // star AFTER
          Type::Int => out.extend_from_slice(b"int"),
          // ...
      }
  }
  ```
  Parsing peels stars off the front recursively; emission appends the star *after* the recursive call — that reversal is what flips prefix into postfix automatically.

## Strings

- No `str` type yet — deferred until structs exist. Strings are `*char`, exactly like C (C has no string primitive either — a string literal is just a pointer to static null-terminated data). Revisit `str` (fat pointer or stdlib struct) once structs land.

## Scoping

- `Vec<HashMap<String, Type>>` — a **stack** of scopes, one map per nested block.
- `enter_scope()` on `{`, `exit_scope()` (pop) on `}` — reuses the same brace-depth tracking pass 1 already needed.
- `resolve()` walks the stack **innermost-first**, falling back to the pass-1 `SignatureTable` for globals/functions — this is what makes inner-block shadowing work correctly.
- **Only declarations touch this structure.** Assignments, calls, nested blocks all parse and emit straight through without going near the scope stack — it only ever answers "is this name declared, and what type?"
- No synthesized/mangled names needed for shadowing: real nested braces in the *emitted C* let gcc's own scoping handle it.

## `global` vs. file-scope

- A variable declared outside any function body is **automatically file-scope**, C-style — position is the marker, no keyword needed.
- The `global` keyword is reserved for a different concept entirely: visibility across **workspaces** (Quaoar's module system, akin to Rust modules / Java packages).

## Expression emission

- **Fully parenthesize every operation** on the way out: `((a) + ((b) * (c)))`. Sidesteps ever having to reason about C operator precedence at emission time — direct-to-buffer emission can't retroactively wrap something it already wrote past.
- Emit through a **shared backend trait** (`CodegenBackend` — `emit_var_decl`, `emit_function`, `emit_binop`, etc.). The C backend implements it now; v0.2's native ASM backend implements the same trait later. The frontend/parser never has to change when the backend does.

## v0.1 pipeline mechanics

- Build generated C source into a `Vec<u8>`, not a `String` — source-derived spans copy straight across with no UTF-8 validation step; synthesized text (e.g. temp variable names) uses `write!` via `std::io::Write`.
- Pipe to the host C compiler directly via `std::process::Command`, writing to the child's piped stdin — **no shell**, no temp file, no injection risk:
  ```rust
  let mut child = Command::new("zig")
      .args(["cc", "-x", "c", "-o", "compiled", "-"])
      .stdin(Stdio::piped())
      .spawn()?;
  child.stdin.take().unwrap().write_all(&c_source)?;
  child.wait()?;
  ```
- Must **drop/close** the child's stdin handle after writing (or let it go out of scope) — otherwise the C compiler blocks waiting for EOF forever.