# Current active work

After the first pull request from `tree/voidstar-lang` to `tree/dev`, I've been working on enhancing the code itself:
- Solve simple bugs (see [quick tasks](#quick-tasks-for-current-patch))
- A few missing features, such as AND and OR operators (not bitwise, algebraic)
- Better error diagnosing
- `quac`!

## On the roadmap

As of v0.1.1, I'm enhancing the C compiler backend myself until I'm satisfied with the result.

After some patches for v0.1, I'll release it and dive into v0.2.

Check [ROADMAP.md](./ROADMAP.md) for a longer vision of Q4r's roadmap.

## Quick tasks for current patch

- [ ] Fix bugs
    - [ ] Source code does not work properly when inline?
    - [ ] v0.1: C keywords are not accepted properly (e.g. `double`), even though quaoar doesn't have these as keywords.
- [ ] Add error diagnosis with position (kinda like rust)
    ```plaintext
    f main() integer{
             ^^^^^^^ unknown type found here
    ```
- [ ] Add `&&` (AND) and `||` (OR) operands
- [ ] Overhaul `quac` (Q4r's cli tool)
- [x] Implement `extern` blocks
- [x] Stress pipeline with functions that touch the whole compiler
- [x] Enhance performance and optimize the source code
- [x] Organize the source code.
- [x] Organize documentation (added licenses and pages)
