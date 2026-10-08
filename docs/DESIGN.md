# Design Decisions for Quaoar

Q4r keeps every single design decision organized in here to centralize and ease the access to everyone.

## Source Code Patterns

Q4r's **[core](../quaoar-core/)** is **indendent**, meaning it should not have any dependencies of other core modules (such as any backends, or the cli module).

Other modules can depend on the core, but the core cannot depend on other modules.