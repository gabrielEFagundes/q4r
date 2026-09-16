> [!WARNING]
> This is a BETA and PRE MANIFEST.md file, used to help on the actual development of v0.2. This is **not** final.

# Quaoar (Q4r) — v0.2 Manifest: AST + QBE Backend

Companion to `quaoar-v0.1-architecture.md`. v0.1 stays exactly as designed — single-pass, no AST, disposable, C-backend-only, kept purely for correctness/compatibility. v0.2 is a **second, separate compiler path**, not a rewrite of v0.1.

## Why v0.2 diverges from v0.1

v0.1's "no persistent tree" constraint was right for emitting plain C — C's mutable variables don't need SSA, don't need whole-function analysis. QBE's IL is SSA. SSA construction (Braun et al.) needs to know a block's predecessors to place `phi`s correctly — genuinely needs more than "process one thing and forget it." Decision: **give v0.2 a real AST.** Don't force QBE through the single-pass constraint that was only ever justified for C.

## AST

- A real, persistent tree this time — `Stmt`/`Expr` nodes, built during parsing, walked afterward for codegen.
- **Memory: arena allocation, not per-node `malloc`/`free`.** One arena per compilation unit (or per function), bump-allocated, the whole arena discarded in one shot once that unit's codegen finishes. Same reasoning as the original `malloc`/`brk` conversation — a compiler's own internal data structures are exactly the textbook case for this.
- `expdesc`-style transient descriptors (from v0.1) don't disappear — they still show up locally during codegen, just now operating over persistent tree nodes instead of live parser state.

## Scoping — two separate mechanisms, don't conflate them

**1. Ordinary name/type resolution** (declare/resolve, `Vec<HashMap<String, Type>>`) — same concept as v0.1. With a real tree, this can happen either while *building* the AST (catch errors early, same live-checking feel as v0.1) or during a separate walk afterward. Either is valid now — the tree persists, so timing no longer loses information the way single-pass would have.

**2. SSA-specific bookkeeping** (`def_var`/`use_var` — "which temp currently holds this variable's value *right now*") — only during the AST-walk/codegen phase, never during parsing. This is backend-specific state; it lives inside the QBE backend struct, not the shared parser/`Compiler` struct. C has no equivalent (a C name never needs a "current version").

## SSA construction

- Algorithm: Braun, Buchwald, Hack, Leißa, Mallon, Zwinkau (2013), *"Simple and Efficient Construction of Static Single Assignment Form."* Builds SSA directly from AST traversal — no separate CFG-building pass needed first. QBE's own author keeps this paper in his reference list.
- Real production precedent: Cranelift (Wasmtime/Wasmer's code generator) implements this exact algorithm in `cranelift_frontend/ssa.rs` — this is shipped, battle-tested infrastructure, not a risky technique.
- Key mechanic to implement: **block sealing.** A block can't have its `phi`s finalized until all its predecessors are known — this is exactly the loop case (a loop's back-edge isn't known until the loop body itself has been parsed). Braun's algorithm handles this via "incomplete" vs "sealed" blocks — look this up specifically when implementing loops.

## Backend architecture (shared with v0.1)

```rust
struct Compiler<B: CodegenBackend> {
    source: Vec<u8>,
    tokens: Vec<VoidstarToken>,
    scopes: Vec<HashMap<String, Type>>,   // ordinary resolution — shared, backend-agnostic
    signatures: SignatureTable,
    backend: B,
}

trait CodegenBackend {
    fn emit_var_decl(&mut self, name: &[u8], ty: &Type, init: &[u8]);
    fn emit_function(&mut self, /* ... */);
    // ...
}
```

- `CCompiler` (v0.1) and a future `QBECompiler` (v0.2) are both just `impl CodegenBackend for ...` — separate concrete structs, same trait, same generic `Compiler<B>` driving both.
- `QBECompiler` owns its *own* fields for `def_var`/`use_var` maps — not shared with `Compiler.scopes`. Ordinary resolution and SSA-temp tracking are genuinely different data, don't merge them.
- Neither backend reaches back into `Compiler`'s fields — everything a backend needs arrives as parameters to the trait method call, already resolved.

## QBE target specifics

- Emit QBE's textual IL — same deferred-conversion principle as C emission: identifiers/already-source-text copy straight from spans, literals go through the same `Literal` enum (needed for constant folding either way, not QBE-specific).
- **License: MIT.** Small (~8,000–15,000 LOC). Explicitly built to be read, extended, forked — not a black box. Real path to adding your own optimization passes later, not just consuming it as-is.
- **Windows target exists**: QBE 1.3 (June 2, 2026) added an x64 Windows backend via `-t amd64_win`. Externally contributed, output is AT&T-syntax assembly best assembled via mingw — newer than the Linux/macOS targets, verify yourself before relying on it in anything shipped.
- **Performance baseline**: targets ~70% of industrial optimizer (gcc/LLVM -O2) performance. QBE 1.2 (2024) actually sat closer to ~40%; 1.3's new passes (GVN/GCM, loop optimization, if-elimination, CFG simplification) closed most of that gap — recent, real progress, not a stale number.

## Optimization tiers

- **Tier 1 — QBE gets this for free, no extra work**: constant folding, dead-branch elimination, real register allocation (QBE ships this out of the box, unlike v0.1's C-delegated version).
- **Tier 2 — future, your own passes on top**: since QBE is MIT and small enough to actually read end to end, the plan is to study its existing passes first, then extend or replace specific ones once you know precisely where they're weak — not a rewrite from scratch, incremental improvement on real infrastructure.

## Self-hosting — unchanged principle from v0.1

Using QBE as an external backend doesn't break self-hosting, same reasoning as rustc+LLVM: self-hosted means the *frontend* (parsing, AST-building, SSA-construction logic) is eventually written in Quaoar itself. QBE remaining an external tool `quac` shells out to is no different from Voidstar-in-C shelling out to `nasm`/`ld` in the original plan, or C-backend piping to `zig cc`.

## Roadmap position

- **v0.1** — single-pass, no AST, C backend. Disposable. Correctness and compatibility only.
- **v0.2** — full AST, arena-allocated. QBE textual IL backend via the same `CodegenBackend` trait. Real register allocation, real optimization, Windows-capable.
- **Later** — Quaoar's own optimization passes layered onto or replacing parts of QBE, once it's been studied end to end.