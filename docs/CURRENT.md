## On the roadmap

`2026/11/23` 
  |- The QBE update will bring patches that will fix most of the compiler's pipeline, so files will change a **lot**.
  |-

  |- Instead of boxes, I use Arena Allocation for the AST, returning the expression ID (their index on the AST's vec), which eases the traversal for the parser and is more efficient.
  |-

> [!WARNING]
> For new contributors in the mean time of v0.1.2 to v0.2, the **whole** backend might change for organization purposes, meaning a **lot** will break, I'll leave this on the release notes, when v0.2 comes out.

Check [ROADMAP.md](./ROADMAP.md) for a longer vision of Q4r's roadmap.