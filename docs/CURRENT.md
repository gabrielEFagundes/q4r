# Current active work

Currently, I'm working on enhancing the code of v0.1, in the means of:
- Optimizing performance;
- Organizing the source code for the compiler itself;
- Covering more cases of uses.

For that last part, I'm stressing the whole pipeline with AI generated code for simple expressions (such as factorial, radian, etc) and also the import of external libraries with the `extern` keyword.

## On the roadmap

I still need to implement `extern` blocks, annotations (metaprogramming) support and `cross` tables.

After those implementations are finished, I'll enhance `quac` (or the `quaoar-cli` module) to add support for both backends and to make it a cooler command line program.

Check [ROADMAP.md](./ROADMAP.md) for a longer vision of Q4r's roadmap.

## Quick tasks for current patch

- [ ] Implement `extern` blocks
- [ ] Implement global error handler
- [ ] Fix bugs
    - [ ] Source code does not work properly when inline?
- [ ] Stress pipeline with functions that touch the whole compiler
- [ ] Enhance performance and optimize the source code
- [ ] Overhaul `quac` (Q4r's cli tool)
- [ ] Organize the source code.
- [ ] Organize documentation