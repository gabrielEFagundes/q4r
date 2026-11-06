# Development of the Quaoar compiler

This file holds every single rule contributors and agents must follow for developing the core for the Quaoar programming language.

## Internal Deps

Q4r's **[core](../quaoar-core/)** is **indendent**, meaning it should not have any dependencies of other core modules (such as any backends, or the cli module).

Other modules can depend on the core, but the core cannot depend on other modules.

## External Deps

Deps are highly dependant on what module we're talking about.

The Quaoar core itself has NO external deps, be it the compiler written in Rust or, in the future, in Quaoar itself. However, modules such as `quac` (Quaoar CLI), may have stable dependencies. `quac` itself uses the `getopts` deps to ease the manipulation of flags and arguments.

Dependencies on external projects that involve the compiler are allowed. E.g. Weywot, the future package manager for Q4r, will (most likely) have external deps.

## Overall Design Structure

- NEVER call `panic!` on problematic Quaoar code, these errors must be added to their respective `Vec<QError>`, which you find everywhere they're necessary.
- You may call `panic!` on internal compiler errors, always with the `"rust: "` prefix.