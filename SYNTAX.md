## Syntaxes

> [!IMPORTANT]
> Most content here is notably inconsistent and WILL change, not much, but it will.

We should always start out by **importing** external files.

```rust
workspace "main"
use static ("std.x", "math.x")
```

Importing works a bit like Golang, where you can import multiple files with a single import.

They're not necessary. but the `std` library **must** be imported to use basic functions, such as `writeLine()` and constants such as `MAX_VALUE`

---

The basic `main` function is where the code will startup, without it, you can't compile anything.

```rust
f main() void {
    writeLine("Hello, World!") // part of standart lib
    return 0
}
```

---

Voidstar encompasses a few types of metadata, such as the ones below.

```c
// i16, i32, i64, f32, f64, all those are artificial primitives, the real primitives are int, char, float and bool.
int bit16Integer = MAX_VALUE(i16.type)
int bit32Integer = MAX_VALUE(i32.type)
int bit64Integer = MAX_VALUE(i64.type)

float bit32Float = 5.67037442

str charArray = "Hey!"
*char alsoCharArray = "Hello."
char aChar = 'a'

bool isVoidstarNice = true
```

The `str` type is simply a simplified `char*`, but with safe memory management. If you want full control over your memory on strings, use the `*char` pointer.

---

Functions are similar to C, the only change is the way you define their return type.

```rust
f sum(f32 a, f32 b) f32 {
    return a+b
}
```

---

There's also the other basics, like `if else`, `switch`, `while` and `for`

```rust
// you can choose if you'll use parenthesis on if statements.
// that applies to else, while, for and other statements too!
if (1 < 2){
    writeLine("2 is greater than 1!")
}else{
    writeLine("this should be unreachable")
}
```

```rust
for (int i = 0; i < 10; i++){
    writeLine("{i}");
}
```

```rust
while true{
    writeLine("Infinite loop goes brrr!")
}
```

```rust
bool hasSucceeded = true
cross hasSucceeded{
    true    =      { writeLine("Success!") },
    false   =      { writeLine("Error!") }
}
```

For more info about these, check out the documentation, which is still in development.