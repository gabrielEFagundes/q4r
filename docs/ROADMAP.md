# Roadmap

## v0.1 - Compiler to C

See [v0.1 MANIFEST.md](./versions/v0.1/MANIFEST.md)

Version 0.1 is the first milestone, it's where Q4r will simply compile its source code to C.

Using the **GNU Compiler Collection** as the main backend, this is Q4r's most limited version, being capable, at best, of defining basic syntax, such as `struct`s, `enum`s and more.

Keep in mind that the `quaoar-c` backend will be updated as Q4r evolves, although those will only be fixes and seasonal chores, the `quaoar-asm` crate will be the default compiler, since it's pointless to compile to `C` when you already have an Assembly backend.

## v0.2 - Assembly Compiler

See [v0.2 MANIFEST.md](./versions/v0.2/MANIFEST.md)

Version 0.2 is where Q4r will be capable of compiling straight to ASM, that being ARM, ISA and most compatible architectures.

This backend is where Quaoar will differ from other languages by being incredibly fast in compile time and runtime, while still being small and simple.

QBE will be utilized as Quaoar's own IL, with my own tweaks and optimizations over it in the future.

## v0.3 - `stdlib` and more

See [v0.3 MANIFEST.md](./versions/v0.3/MANIFEST.md)

Version 0.3 will mark the addition of standard libraries for Q4r, those include `stdio`, `stdlib`, `string`, `os`, `http`, etc.

This is when Quaoar will finally start being usable for real projects.

## v0.4 - Self-Hosting

See [v0.4 MANIFEST.md](./versions/v0.4/MANIFEST.md)

Version 0.4 will be the longest of the milestones because of its extreme complexity. 

Before self-hosting, a VSCode extension/owned editor will be written, also in Q4r, not only to prove the potential of self-hosting, 
but also to help when actually writing something in Quaoar.

## v0.5 - Enhancements and fixes

See [v0.5 MANIFEST.md](./versions/v0.5/MANIFEST.md)

Version 0.5 is where Q4r will turn into an actual robust language.

Quaoar will have its own framework designed to help and abstract even more the creation of APIs, robust systems, etc.

## v0.9 - Kernel

See [v0.9 MANIFEST.md](./versions/v0.9/MANIFEST.md)

Version 0.9 won't exactly change much of Q4r, it'll mostly adapt for the creation of a bootable Kernel from scratch.

You can consider this language close to stable once it reaches this version.

## v1.0 - Project Rewriting

This version is where Q4r will finally reach its stability, and prove itself as an actually usable language by rewriting my own projects.