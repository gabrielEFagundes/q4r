## Syntaxes

> [!IMPORTANT]
> Most content here is notably inconsistent and WILL change
>
> Do not take this file as final.

We should always start out by **importing** external files.

```quaoar
workspace "main"
use static ("io", "math")
```

Importing works a bit like Golang, where you can import multiple files with a single import.

They're not necessary. but the `io` library **must** be imported to use basic functions, such as `writeLine()`.

---

The basic `main` function is where the code will startup, up until version v0.2, you need them defined somewhere, as without them, your code will not compile.

```quaoar
f main() int {
    writeLine("Hello, World!") // part of standart io lib
    return 0
}
```

---

Q4r encompasses a few types of metadata, such as the ones below.

```quaoar
int too = 11111110 // this is not -2, it's a literal
float codata = 5.67037442

str foo = "Hey!"
*char fee = "Hello." // memory management and pointers are here aswell
char boo = 'a'

bool isQuaoarFun = true
```

The `str` type is a null terminated array of characters, but with safe memory management. If you want full control over your memory on strings, use the `*char` pointer.

---

Functions are similar to C, the only change is the way you define their return type.

```quaoar
f sum(float a, float b) float{
    return a+b
}
```

---

There's also the other basics, like `if else` and `for`

`for` is used both as `while` and `for` iterators.

```quaoar
if 2 > 1{
    // if branch
}else if 2 < 1{
    // ...else if branches...
}else{
    // ...else branch
}
```

```quaoar
for i = 0; i < 10; +1{
    // i goes all the way up to 9
}
```

```quaoar
for true{
    from std::panic("sorry, can't allow infinite loops!")
}
```

`cross tables` are exhausting pattern matching statements, internally they mount a jump table for maximum performance.


```quaoar
bool hasSucceeded = true
cross hasSucceeded{
    true:  writeLine("Success!"),
    false: writeLine("Error!")
}
```