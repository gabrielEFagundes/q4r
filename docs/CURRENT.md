# Current active work

After the first pull request from `tree/voidstar-lang` to `tree/dev`, I've been working on enhancing the code itself:
- Solve simple bugs (see [quick tasks](#quick-tasks-for-current-patch))
- A few missing features, such as AND and OR operators (not bitwise, algebraic)
- Better error diagnosing
- `quac`!

## On the roadmap

I still need to implement annotations (metaprogramming) support and `cross` tables. But that's something I'll leave for v0.2, and will implement as I work with QBE, since quite a few things will change on the `expdesc.rs` file, making my life miserable to write all those features back onto the `CCompiler` backend.

After those implementations are finished, I'll enhance `quac` (or the `quaoar-cli` module) to add support for both backends and to make it a cooler command line program.

Check [ROADMAP.md](./ROADMAP.md) for a longer vision of Q4r's roadmap.

## Quick tasks for current patch

- [x] Implement `extern` blocks
- [ ] Enhance global error handler with custom messages for each error
- [ ] Add error diagnosis with position (kinda like rust)
    ```plaintext
    f main() integer{
             ^^^^^^^ unknown type found here
    ```
- [ ] Fix bugs
    - [ ] Source code does not work properly when inline?
    - [ ] v0.1: C keywords are not accepted properly (e.g. `double`), even though quaoar doesn't have these as keywords.
- [ ] Add `&&` (AND) and `||` (OR) operands
- [x] Stress pipeline with functions that touch the whole compiler
- [x] Enhance performance and optimize the source code
- [ ] Overhaul `quac` (Q4r's cli tool)
- [x] Organize the source code.
- [x] Organize documentation (added licenses and pages)