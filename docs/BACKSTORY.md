## Quaoar's Backstory

> How hard would it be if I wrote my own programming language and used it to rewrite all the projects I have?

That's it, that's literally the whole purpose of Q4r. Coding my own language and rewriting my projects in it.

Turns out it's becoming a bit more than just that.

## The design

Quaoar is designed to have the performance of Rust, the simplicity of Golang (and compile speeds too, hahah) and a touch of C.

This means the language can be both a low-level language and a highly abstracted one, which allows you to do whatever you want with it, be it an API, a game engine, or control a low-level machine.

At first, I was the one going to write the native assembly backend, because well, LLVM sucks to learn, especially as a single developer. My first thoughts were on making Q4r source code compile down to NASM, but turns out it doesn't support a few architectures (such as ARM, MIPS and RISC-V).

While I was doing my researches about ways to avoid the lack of compatibility, I found one of the best things that I could have ever found, **QBE**.

## QBE - Quick BackEnd

That's when I found out about a very old project, called QBE, or Quick BackEnd. This project targets reaching 70% of LLVMs performance with only 10% of the code.

Up until version 1.2, QBE had no support at all for the Windows operating system, which would end up falling back to the same problem I was seeking to solve. Turns out that in version 1.3, launched only 3 months ago (from the time I'm writing this), it actually got Windows ABI compatibility, meaning now it could run on all the 3 main OS', MacOS, Linux and Windows.

Languages like Hare and Odin both use QBE, which is an amazing thing to know, because it proves that QBE can actually be used on real life programming languages.