## On the roadmap

`2026/10/30` 
  |- I finished an intial version of the diagnoser, I'll now focus on v0.2
  |-

  |- The QBE update will bring patches that will fix most of the compiler's pipeline, so files will change a **lot**!
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