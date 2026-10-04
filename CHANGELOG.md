## v0.1.0 - 2026-10-04

### Features
-  add AND & OR and patch bugs
-  full stress pipeline tested and functional, update docs and change quaoar-cli compiler pipeline
-  implement extern blocks and enhance code for more use cases, add examples
-  add string literals and a few optimizations
-  partial C compatibility working, fix ext installer
-  pointers implemented and enhancements made + extension installer
-  signature table mounter done
-  add initial voidstar extension pack + init lexer and transition to rust
-  Adding tokens, keywords and a hash lib (even though I didn't use it in the end)

### Fixes
-  patch statement and boolean algebra bugs
-  general fixes because of merge and autofix
-  small duplicate removal and documentation fixes
-  fix lexer entering infinite loops on specific branches
-  Deleting compiled test file

### Other changes
- [merge] merge `dev` into `main`
- workflow: limit autofix workflow to main branch
- [autofix.ci] apply automated fixes
- [merge] merge remote dev with local dev
- [autofix.ci] apply automated fixes
- doc: update current.md with new currents
- doc: update apache 2.0 license
- docs: update current and start backstory
- [merge] merge `voidstar-lang` with `dev`
- [autofix.ci] apply automated fixes
- [MERGE] Merge `fix/untangled-mess` into `voidstar-lang`
- [autofix.ci] apply automated fixes
- chore: clippy fixes and new clippy action
- chore: overhaul expdesc and start code adaption to new logic
- chore: temp patch for now
- chore: starting fixes
- chore: add error reporting with line and char position (not col)
- doc: add and organize documentations, add licenses
- chore: partial implementation of extern blocks, needs corrections
- chore: general fixes, support to unary operators on integers
- chore: continuing pointers (now with new notebook!)
- chore: optimize and organize code, initiating pointers support
- chore: finishing go-styled for loops and initializing pointers
- chore: happy path working, missing tweaks and pointers
- chore: more enhancements and preparations for release
- chore: if, functions and returnals are working as they should
- chore: c compiler almost done, backend needs changes
- chore: continuing c compilation
- chore: finish function header declaration
- doc: add changelog and version for quaoar release bot
- chore: initiating final c-compiler phase
- chore: add release bot and issue templates
- doc: fix readme formatting
- doc: rename everything and adapt to new name
- Merge branch 'voidstar-lang' of https://github.com/gabrielEFagundes/voidstar into voidstar-lang
- doc: change a bit of quaoar extension and add new icon
- chore: initial lexer working, storage of spans next
- chore: continuing lexer, need to fix eoc and char
- doc: add important info
- Merge pull request #3 from gabrielEFagundes/develop
- chore: Add .exe files to .gitignore
- Merge pull request #2 from gabrielEFagundes/feat/keywords-plus
- Merge pull request #1 from gabrielEFagundes/docs/new-snippet
- doc: Updating current state of development
- doc: Update code snippet to new syntax
- doc: Clarify development status and future plans
- doc: Aligning readme code snippet and titles
- chore: Enhancing readme and adding syntaxes
- doc: Adding first readme

