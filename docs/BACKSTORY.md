## Quaoar's Backstory

> How hard would it be if I wrote my own programming language and used it to rewrite all the projects I have?

That's it, that's literally the whole purpose of Q4r. Coding my own language and rewriting my projects in it.

Turns out it's becoming a bit more than just that.

## The design

Quaoar is designed to have the performance of Rust, the simplicity of Golang (and compile speeds too, heh) and a touch of C.

This means the language can be both a low-level language and a highly abstracted one, which allows you to do whatever you want with it, be it an API, a game engine, or control a low-level machine.

At first, I was the one going to write the native assembly backend, because well, LLVM sucks to learn, especially as a single developer. My first thoughts were on making Q4r source code compile down to NASM, but turns out it doesn't support a few architectures (such as ARM, MIPS and RISC-V).

While I was doing my researches about ways to avoid the lack of compatibility, I found one of the best things that I could have ever found, **QBE**.

## QBE - The IL that matches LLVM