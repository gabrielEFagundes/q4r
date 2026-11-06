# Contributing to Quaoar

First of all, thank you so much for taking your time to contribute to Quaoar! ❤️

Quaoar is and always will be 100% free and open-source.

## Folder structure

Quaoar follows the default crates structure:

`quac`: The entrypoint for Quaoar, the `quac` crate is where the compiler's cli lives and the whole pipeline is orchetrated.

`quac-core`: The core frontend of the language, where the source code is organized to be send to the backend.

`quac-doctor`: The diagnosis module, this crate is mostly used to render the error messages.

`quaoar-c`: The C backend for Quaoar, this crate receives the nodes from the frontend, emits C code and parses to the equivalent C compiler.

> This structure is subject to change as the project grows, every change must be documented here!

Beside the crates structure, there are a few other utility folders:

`.agents`: Agentic development instructions, **required** if agents are used.

`.github`: GitHub workflows and bots docs.

`assets`: README assets and etc.

`docs`: Overview of the Quaoar documentation.

`examples`: Quaoar basic examples.

`quaoar-ext`: Will move very soon to a separate repository, but it's the development extension for vscode/vscodium

The folder structure will most likely change as the project evolves, so don't expect it to be final.

## Overview

<!-- this will most likely change when I create the language's main site -->
For design decisions and structure of the project itself, check out [the internals file](/docs/INTERNALS.md)

- Always make tests if you're implementing a new feature;
    - You can write your tests following the existing ones, just make sure it passes both in a real life usage test and on the test itself.
- For overall patches, doc updates, etc, You can open the PR and the maintainer will merge it.

### How to run the demo

```bash
# runs the `_pipeline.qua` stress test, using the C backend
# output defaults to `.quaoar/_pipeline.exe` for windows, `.quaoar/_pipeline` for unix-based OS'
cargo run -- examples/stress_tests/_pipeline.qua --backend c

# runs the help command
cargo run -- -h

# to build the project on release mode
cargo build --release
# or...
cargo install --path .

# considering you're on powershell or bash
target/release/quac -h
```

### Git (commits, versioning)

The main repository for the Quaoar compiler is [q4r](https://github.com/gabrielEFagundes/q4r).

Commits on the repo must follow a very simple rule, to keep things organized and beautiful:

- Always follow the [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/#summary) rules;
- Don't open a PR straight to main, never. Always open your PRs targeting the `dev` branch;

### Issues and Pull Requests

For PR's, you can find a template [here](./.github/PULL_REQUEST_TEMPLATE.md).

For Issues, you can follow the templates that appear when you create one! Currently, the repo has only `bug` and `feat` templates, feel free to suggest other templates!

If your issue doesn't apply to either, you're free to create a blank issue and format it as you wish, just keep it organized and easy to understand, preferably following the `[TAG]: Issue Title` template.