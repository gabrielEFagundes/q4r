# Roadmap

## v0.1 - Compiler to C

See [v0.1 MANIFEST.md](./versions/v0.1/MANIFEST.md)

Version 0.1 is the first milestone, it's where Q4r will simply compile its source code to C.

Using the **GNU Compiler Collection** through different C compilers, this is Q4r's most limited version, being capable, at best, of defining basic syntax, such as declarations, statements, memory management and compatibility with C's standard libraries.

The `quaoar-c` crate will be updated as the project evolves, but it will **not** be given as much attention as the Assembly compiler.

## v0.2 - Assembly Compiler ([QBE IL](https://c9x.me/compile/))

See [v0.2 MANIFEST.md](./versions/v0.2/MANIFEST.md)

Version 0.2 is where Q4r will be capable of compiling straight to ASM, that being ARM, ISA and most compatible architectures.

The QBE Intermediate Language will be utilized as Quaoar's own IL. In the future, I'll apply customizations and tweaks to improve the performance and speed of QBE.

## v0.3 - `stdlib` and more

See [v0.3 MANIFEST.md](./versions/v0.3/MANIFEST.md)

Version 0.3 will mark the addition of standard libraries for Q4r, those include `stdio`, `stdlib`, `string`, `os`, `http`, etc.

Differently from the support of C's standard libraries, those will be implemented in Rust and Quaoar itself, becoming almost completely independent from any other languages.

## v0.4 - Self-Hosting

See [v0.4 MANIFEST.md](./versions/v0.4/MANIFEST.md)

Version 0.4 will be the longest of the milestones because of its great complexity. 

Before self-hosting, a VSCode extension/owned editor will be written for Q4r, helping out on the actual development.

## v0.5 - Enhancements and fixes

See [v0.5 MANIFEST.md](./versions/v0.5/MANIFEST.md)

Version 0.5 is where Q4r will turn into an actual robust language.

Quaoar will have its own framework designed to help and abstract even more the creation of APIs, robust systems, hardware interaction, native applications, etc.

## v0.9 - Kernel

See [v0.9 MANIFEST.md](./versions/v0.9/MANIFEST.md)

Version 0.9 won't exactly change much of Q4r, it'll mostly adapt for the creation of a bootable Kernel from scratch.

You can consider this language close to stable once it reaches this version.

## v1.0 - Project Rewriting

This version is where Q4r will finally reach its stability, and prove itself as an actually usable language by rewriting my own projects.

## Future?

I plan on working in Quaoar for a long time, since I do have fun working on it and I'd absolutely love to see it being used in actual, real life projects.

If the language really ends up scaling, I **might** rewrite some modules (and when I say some, it could mean the entire thing) in LLVM itself. This is something I need to think a lot about though, because if Quaoar's QBE receives optimizations and updates, completely adopting LLVM would just be throwing all the work put into QBE away, which, if strips away my joy for working on this, I'll absolutely **not** do.