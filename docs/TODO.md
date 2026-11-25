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

## Minor tasks that require more effort

The code (in v0.2) is pretty messy, there are currently 2 parsers, one for `expdesc` and another for the AST parser. Discussions and external opinions are greatly appreciated for deciding what I could do to solve that problem.

This is going onto the merge notes when I merge the parser to `dev`, before writing the QBE IR emitter.

- [ ] Centralize both backends to a single code generator

  Currently the QBE and C backends are separated by the Nodes, where QBE uses more structured and organized nodes for the AST, while C uses simple `expdesc`s to quickly mount a node and emit it as C on-the-fly.

  The problem is that solving this is going to take a little while, because of the structure of the nodes, meaning a lot of changes on the `core` and `quaoar-c` backend.

  Plus the fact that this would force the QBE backend to follow the single-pass C compiler rules, which might mean I'll have to keep the backends separated.

  I'd solve this myself in the process of developing v0.2, but I don't have much time and I'd like to reach the QBE milestone in time. So if it depends on me, it'll prob be releasing only on a v0.2 patch, more likely v0.2.1.