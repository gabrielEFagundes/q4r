<h1>Quaoar - Q4r</h1>
<img align="left" src="assets/carbon.svg" width="350" />

Quaoar (Lowercase u. `/'kwa:wa:r/`, `/ˈkwɑː.oʊ.ɑːr/`) is a small, fast and independent set of instructions forming a programming language.

Quaoar is a middle ground language, combining the speed and performance of **Rust** with the simplicity of **Go** and a touch of **C**.

Because Quaoar is an assembled general purpose programming language, you can do all sorts of things, including backends, RESTful apis, desktop apps and even hardware-level integrations.

---

## Milestones

Keep track of the current state with the [milestones](https://github.com/gabrielEFagundes/q4r/milestones).

Each stage of development will be a beta release up until Q4r reaches the last milestone. 
(Keep in mind that the dates for the milestones are not final.)

## Quick Start

Currently, Quaoar has no install script besides its development extension for editors.

You'll need a C compiler (either GCC or Zig) and rust + cargo installed on your machine

First, clone this repo:

```bash
git clone https://github.com/gabrielEFagundes/q4r
cd q4r
```

Then simply create a `.qua` file and run the compiler using cargo!

```bash
cat << 'EOF' > main.qua
extern f printf(*char fmt, ...)

f main() int{
      printf("Hello, Quaoar!")
      return 0
}
EOF

cargo run -- ./main.qua --backend=c
```

## Contributing

Please feel free to open Pull Requests and Issues.

All contributions must follow a basic code of conduct, such as being polite, direct and, obviously, make sense.

Check out [CONTRIBUTING](/CONTRIBUTING.md) for more info.

## Contributors

Thank you so much for every contributor that helped on the development of Quaoar! ❤️

Every contributor is added here by hand.

<div>
      <img src="https://github.com/gabrielEFagundes.png" width="50rem">
      <img src="https://github.com/arifrhm.png" width="50rem">
</div>