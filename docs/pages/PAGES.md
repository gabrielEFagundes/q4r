# Quaoar (Q4r) Programming Language

Quaoar is a compiled, systems-level programming language that aims for the speed of Rust, the simplicity of Go, and a touch of C. It compiles without LLVM, currently transpiling to C for v0.1, with a QBE-based native backend planned for v0.2.

- **Compiler CLI**: `quac`
- **File extension**: `.qo`
- **License**: MIT & Apache Licence, Version 2.0 (see [LICENSING-SOFTWARE](../LICENSING-SOFTWARE))
- **Written in**: Rust

---

<!-- page:getting-started -->

## Getting Started

### Hello, World!

Quaoar doesn't bundle its own standard I/O yet — instead, it provides full C ABI compatibility through the `extern` keyword, letting you call any C library function directly.

```quaoar
extern f printf(*char s, ...)

f main() int{
    printf("Hello, World!\n")
    return 0
}
```

### Compiling

```bash
quac hello.qo
./hello
```

`quac` transpiles your `.qo` source to C, pipes it to the host C compiler (gcc or zig cc), and produces a native binary. No intermediate files are left behind.

---

<!-- page:basics -->

## Language Basics

### No Semicolons

Quaoar uses **automatic semicolon insertion**, inspired by Go. You never write semicolons in source — the compiler's lexer inserts them for you based on the token before a newline. If the last token on a line could end a statement (an identifier, a literal, `)`, `]`, or `}`), a semicolon is synthesized automatically.

This means multi-line expressions work naturally when the prior line ends with an operator or an opening bracket:

```quaoar
int result = a
    + b  // no premature semicolon — previous line ended with `+`
```

### Comments

Quaoar supports both single-line and block comments:

```quaoar
// This is a single-line comment

/*
    This is a block comment.
    It can span multiple lines.
*/
```

---

<!-- page:types -->

## Types

### Primitive Types

| Type    | Description                    |
|---------|--------------------------------|
| `int`   | Signed integer                 |
| `uint`  | Unsigned integer               |
| `float` | 32-bit floating point          |
| `char`  | Single byte / ASCII character  |
| `bool`  | Boolean (`true` / `false`)     |
| `void`  | No value (return type only)    |

### Pointers

Unlike C, Quaoar places the asterisk **before** the type, not after it:

```quaoar
int value = 10
*int pointer = &value
```

This is a deliberate syntax choice — the pointer marker modifies the type, so it reads left to right as "pointer to int."

Dereferencing and address-of work identically to C:

```quaoar
*pointer = 5     // write through the pointer
int x = *pointer // read through the pointer
int y = &value   // take the address — y holds value's memory address
```

Pointers can be nested:

```quaoar
**int pp = &pointer  // pointer to pointer to int
```

### Strings

Quaoar does not currently have a dedicated `str` type. Strings are represented as `*char` (pointer to char), exactly as in C — null-terminated, pointing at static data for literals.

```quaoar
*char greeting = "Hello!"
```

A higher-level string type (likely a fat pointer or stdlib struct carrying both a pointer and a length) is planned for a future version, once struct support exists.

### String Literals

String literals use double quotes and support C-style escape sequences:

```quaoar
printf("Hello, World!\n")
printf("Tab:\there\n")
printf("Quote: \"inside\"\n")
```

The compiler passes escape sequences through to the C backend unchanged — the host C compiler interprets them.

### Character Literals

Character literals use single quotes:

```quaoar
char ch = 'a'
```

---

<!-- page:variables -->

## Variables

### Declaration

Variables are declared with a type followed by a name and an optional initializer:

```quaoar
int x = 10
float pi = 3.14
char letter = 'Q'
bool alive = true
```

### Assignment

Assignment to an existing variable uses `=`:

```quaoar
x = 20
```

Compound assignment operators are supported:

```quaoar
x += 5
x -= 1
x *= 2
x /= 4
```

### Global Variables

A variable declared **outside any function body** is automatically file-scope global — no keyword is needed, same as C. The position itself is the marker:

```quaoar
int global_counter = 0

f main() int{
    global_counter += 1
    return global_counter
}
```

> **Note:** The `global` keyword is **reserved** for a different purpose — cross-workspace visibility (see [Workspaces](#workspaces)).

---

<!-- page:functions -->

## Functions

### Declaration

Functions are declared with the `f` keyword, followed by the name, parameter list, and an optional return type. If no return type is specified, the function returns `void`:

```quaoar
f add(int a, int b) int{
    return a + b
}

f greet(){
    printf("Hello!\n")
}
```

### Calling

```quaoar
int result = add(3, 7)
greet()
```

### Forward References

Quaoar supports calling functions defined later in the same file. The compiler runs a **two-pass** system: pass 1 scans top-level declarations (function signatures and global variables) and records them in a signature table, skipping function bodies entirely. Pass 2 then walks the full source, emitting code with every signature already known regardless of file order.

```quaoar
f main() int{
    return square(5)  // works even though square is defined below
}

f square(int x) int{
    return x * x
}
```

---

<!-- page:control-flow -->

## Control Flow

### If / Else If / Else

Quaoar's conditional statements follow a C-like structure, without parentheses around the condition:

```quaoar
if 10 > 5{
    return true
} else if 10 < 5{
    return false
} else {
    return false
}
```

Conditions can use any comparison or boolean expression:

```quaoar
f cmp(char a, char b) bool{
    if a == b{
        return true
    }
    return false
}
```

### While Loops (Go-style `for`)

Quaoar uses the `for` keyword for while-style loops — a single condition, no initializer or incrementer. Identical to Go's `for condition { }`:

```quaoar
int i = 0
for i < 10{
    i += 1
}
```

A more complete example:

```quaoar
f whileLoop(int a, int b){
    for a >= b{
        a -= 1
    }
}
```

### For Loops (C-style)

Traditional C-style for loops use the `for` keyword with an initializer, condition, and increment step — separated by semicolons. The iterator variable is declared inline, and the increment is expressed as a signed offset:

```quaoar
for i = 0; i < 10; +1{
    // i starts at 0, increments by 1, runs while i < 10
}
```

The increment value can be negative for counting down:

```quaoar
f forLoop(int a){
    for i = 10; i <= a; -1{
        // this will form a countdown until a
    }
}
```

---

<!-- page:cross-tables -->

## Cross Tables

> **Status: Not yet implemented.** This documents the planned design.

Quaoar replaces the traditional `switch` statement with **cross tables** — pattern-matching jump tables using the `cross` keyword:

```quaoar
f crossTable(bool value) int{
    cross value{
        true: return 0,
        false: return 1
    }
}
```

Cross tables match against discrete values and execute the corresponding branch. Each arm is separated by a comma:

```quaoar
cross status_code{
    200: printf("OK\n"),
    404: printf("Not Found\n"),
    500: printf("Server Error\n")
}
```

---

<!-- page:extern -->

## C Interoperability

### The `extern` Keyword

Quaoar is **C ABI compatible**. You can call any C library function by declaring its signature with `extern` — no headers, no wrappers, no special types:

```quaoar
extern f printf(*char s, ...)
extern f malloc(uint size) *void
extern f free(*void ptr)
```

As long as the function is linked (via libc or another library passed to the linker), it will work.

### Extern Blocks

When declaring multiple external functions, use an `extern` block to group them:

```quaoar
extern {
    f printf(*char s, ...)
    f scanf(*char fmt, ...)
    f malloc(uint size) *void
    f free(*void ptr)
}
```

### Variadic Functions

External functions can be declared as variadic using `...` as the last parameter, matching C's `va_list`-based convention:

```quaoar
extern f printf(*char fmt, ...)
```

### Example: User Input

```quaoar
extern {
    f printf(*char s, ...)
    f scanf(*char fmt, ...)
}

f main() int{
    int var = 0

    printf("Hello, type a number in!\n> ")
    scanf("%d", &var)

    printf("You've typed %d!", var)
    return 0
}
```

---

<!-- page:operators -->

## Operators

### Arithmetic

| Operator | Description    |
|----------|----------------|
| `+`      | Addition       |
| `-`      | Subtraction    |
| `*`      | Multiplication |
| `/`      | Division       |

### Comparison

| Operator | Description              |
|----------|--------------------------|
| `==`     | Equal to                 |
| `!=`     | Not equal to             |
| `>`      | Greater than             |
| `<`      | Less than                |
| `>=`     | Greater than or equal to |
| `<=`     | Less than or equal to    |

### Assignment

| Operator | Description             |
|----------|-------------------------|
| `=`      | Assign                  |
| `+=`     | Add and assign          |
| `-=`     | Subtract and assign     |
| `*=`     | Multiply and assign     |
| `/=`     | Divide and assign       |

### Unary

| Operator | Description              |
|----------|--------------------------|
| `*`      | Dereference (prefix)     |
| `&`      | Address-of (prefix)      |
| `!`      | Logical NOT              |

> **Note:** `*` is disambiguated by position — prefix means dereference, infix means multiplication. No ambiguity exists in the grammar.

---

<!-- page:memory -->

## Memory Management

Quaoar's memory management is completely optional, differently from C's model. There is no garbage collector.

```quaoar
extern {
    f malloc(uint size) *void
    f free(*void ptr)
}

f main() int{
    *int data = malloc(8)
    *data = 42
    free(data)
    return 0
}
```

Pointers are **completely optional** — if your program doesn't need explicit memory management, you never have to touch them. Programs can use stack-allocated local variables and convenience mechanisms without any pointer or allocation code at all.

Those convenience mechanisms (RAII-style scope-based cleanup, or allocator-passing conventions, frameworks, etc.) will be introduced in the future, but these will layer on top of the manual model, never replace it.

---

<!-- page:workspaces -->

## Workspaces

> **Status: Planned, not yet implemented.**

Workspaces are Quaoar's module system, similar to Rust's modules or Java's packages. They provide namespace isolation and code organization across multiple files.

The `global` keyword (reserved, distinct from file-scope global variables) will mark a declaration as visible across all workspaces:

```quaoar
// Planned syntax — not yet available
global int SHARED_CONSTANT = 42
```

The `use` keyword will import declarations from other workspaces:

```quaoar
// Planned syntax — not yet available
use math
```

---

<!-- page:roadmap -->

## Roadmap

### v0.1 — C Pre-Compiler (Current)

The compiler transpiles Quaoar source to C, piped to the host C compiler via stdin. No intermediate files. Features implemented:

- Primitive types (`int`, `uint`, `float`, `char`, `bool`, `void`)
- Variable declarations and assignments (including compound operators)
- Functions with parameters and return types
- Forward references (two-pass signature resolution)
- If / else if / else statements
- While loops (`for condition{ }`)
- For loops (`for i = 0; i < n; +1{ }`)
- Pointers (declaration, address-of, dereference)
- String and character literals
- C interop via `extern` (single and block declarations, variadic support)
- Automatic semicolon insertion (Go-style, no explicit semicolons)
- Block and line comments

### v0.2 — QBE Native Backend

A second compiler backend targeting QBE's textual IL, producing native binaries without a C compiler in the loop. Key changes from v0.1:

- Full AST (arena-allocated, per-function lifetime)
- SSA construction via Braun et al. algorithm
- Real register allocation and optimization (provided by QBE, with custom passes planned)
- Structs and arrays
- The `str` type (fat pointer or stdlib struct)
- Cross tables (`cross` pattern matching)
- Scope-level type checking and diagnostics
- Windows support via QBE 1.3's `amd64_win` target

### Later Milestones

- Self-hosting (Quaoar compiler written in Quaoar)
- Custom optimization passes on top of QBE
- Workspace/module system (`use`, `global`)
- Built-in HTTP and database libraries
- Web/frontend targeting (distant future)
- A full kernel written in Quaoar

---

<!-- page:compiler-internals -->

## Compiler Internals

This section documents how `quac` works internally — useful for contributors or anyone curious about the architecture.

### Pipeline (v0.1)

```
source.qo
    │
    ▼
Lexer (runs once)
    │  - zero-copy: tokens store byte spans (start/end) into source
    │  - NUL sentinel appended to source buffer
    │  - Go-style automatic semicolon insertion
    ▼
Token stream
    │
    ├──▶ Pass 1 (shallow walk)
    │       - scans top-level function headers and global declarations
    │       - skips function bodies by tracking brace depth
    │       - builds SignatureTable: HashMap<String, Symbol>
    │
    └──▶ Pass 2 (full parse + emit)
            - walks the same token stream again
            - parses every construct, emits C text into Vec<u8>
            - resolves forward references against SignatureTable
            - pipes output to host C compiler via stdin
                │
                ▼
            Native binary
```

### Lexer

- Tokens carry byte spans `(start: u32, end: u32)` into the source buffer — no owned strings, no allocation during scanning.
- Source text is materialized from spans only at the point of use (identifier lookup, literal parsing).
- A NUL byte (`0x00`) is appended to the source as an EOF sentinel, eliminating bounds checks.
- Keywords and symbols are looked up via binary search over pre-sorted `const` arrays.

### Expression Parsing

Expressions are parsed via recursive-descent precedence climbing. All emitted C output is **fully parenthesized** (`((a) + ((b) * (c)))`) to sidestep operator precedence reasoning at emission time.

Unary operators (`*` for dereference, `&` for address-of) are disambiguated from binary operators by position: prefix position means unary, infix means binary.

### Type System

`Type` is a separate, recursive enum from `TokenType`:

```rust
enum Type {
    Int, UInt, Float, Char, Bool, Void,
    Pointer(Box<Type>),
}
```

Pointer types are parsed prefix-first (Quaoar's `*char`) and emitted postfix (C's `char*`) via recursive descent — the star is appended *after* the recursive call returns, automatically reversing the order.

### Backend Architecture

The compiler uses a trait-based backend system. A generic `Backend<B>` struct holds shared state (source, tokens, cursor, signature table) and owns a `B: CodeGen` implementer:

```rust
trait CodeGen {
    fn generate(&mut self, shared: &mut SharedState);
    fn generate_headers(&mut self, shared: &mut SharedState);
}
```

Currently only `CCompiler` implements `CodeGen`. The v0.2 QBE backend will implement the same trait, allowing the frontend and shared infrastructure to remain unchanged.

---

<!-- page:differences-from-c -->

## Key Differences from C

| Feature               | C                              | Quaoar                                        |
|-----------------------|--------------------------------|-----------------------------------------------|
| Semicolons            | Required                       | Automatic (Go-style insertion)                |
| Function keyword      | Return type before name        | `f` keyword, return type after params         |
| Pointer syntax        | `int *p` (postfix)             | `*int p` (prefix)                             |
| Header files          | Required for declarations      | Not needed — `extern` declares directly       |
| Switch statement      | `switch`/`case`/`break`        | `cross` table (planned)                       |
| While loop            | `while (cond) { }`            | `for cond{ }`                                 |
| For loop              | `for (init; cond; incr)`      | `for i = 0; i < n; +1{ }`                    |
| String type           | `char*`                        | `*char` (same ABI, reversed syntax)           |
| Memory management     | Manual (`malloc`/`free`)       | Manual, with optional pointer-free programming|
| Module system         | Header files + `#include`      | Workspaces (planned)                          |