## On the roadmap

`2026/11/23` 
  |- The QBE update will bring patches that will fix most of the compiler's pipeline, so files will change a **lot**.
  |-

  |- Instead of boxes, I use Arena Allocation for the AST, returning the expression ID (their index on the AST's vec), which eases the traversal for the parser and is more efficient.
  |-

Check [ROADMAP.md](./ROADMAP.md) for a longer vision of Q4r's roadmap.

## Quick tasks for current patch

- [ ] Add error codes for future diagnosing;
- [ ] Add documentation comments
- [ ] Add copyright comments on the header of each file
  ```rust
  /*
    SPDX-License-Identifier: MPL-2.0
    Copyright (c) 2026 Gabriel Ehrat Fagundes and the Contributing Community
  */
  ```
- [ ] Implement colors on the error renderer (`ViewRenderer`)
- [ ] Enhance `quac-doctor`'s module code focusing on performance