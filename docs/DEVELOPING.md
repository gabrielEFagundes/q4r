# Development of the Quaoar compiler

This file holds every single rule contributors and agents must follow for developing the core for the Quaoar programming language.

## Internal Deps

Q4r's **[core](../quaoar-core/)** is **indendent**, meaning it should not have any dependencies of other core modules (such as any backends, or the cli module).

Other modules can depend on the core, but the core cannot depend on other modules.

## External Deps

Deps are highly dependant on what module we're talking about.

The Quaoar core itself has NO external deps, be it the compiler written in Rust or, in the future, in Quaoar itself. However, modules such as `quac` (Quaoar CLI), may have stable dependencies. `quac` itself uses the `getopts` deps to ease the manipulation of flags and arguments.

Dependencies on external projects that involve the compiler are allowed. E.g. Weywot, the future package manager for Q4r, will (most likely) have external deps.

## Git (commits, versioning)

The main repository for the Quaoar compiler is [q4r](https://github.com/gabrielEFagundes/q4r).

Commits on the repo must follow a very simple rule, to keep things organized and beautiful:

- Always follow the [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/#summary) rules
- Don't open a PR straight to main, never. Always open your PRs targeting the `dev` branch!
- Always make tests if you're implementing a new feature, tests are located at `tests/` at each module level and they all must pass before the maintainer accepts your PR 
    - You can write your tests following the existing ones, just make sure it passes both in a real life test and on the test itself.
- For overall patches, doc updates, etc, You can open the PR and the maintainer will merge it.