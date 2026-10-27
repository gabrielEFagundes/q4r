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

- [ ] Add error codes for future diagnosing;
- [ ] Add documentation comments
- [ ] Add copyright comments
  ```rust
  /*
    SPDX-License-Identifier: MPL-2.0
    Copyright (c) 2026 Gabriel Ehrat Fagundes and the Contributing Community
  */
  ```
- [ ] Implement colors on the error rendenrer (`ViewRenderer`)
- [ ] Enhance `quac-doctor`'s module code