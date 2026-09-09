# Q4r — VS Code syntax highlighting

## Try it without packaging (fastest, for test scripting)

1. Unzip this into a folder, e.g. `voidstar-ext/`
2. Open that folder in VS Code
3. Press `F5` — this launches an "Extension Development Host" window with the extension active
4. In that new window, open any `.qo`, `.qoa` or `.qua` file

Changes to `syntaxes/q4r.tmLanguage.json` apply after reloading the dev host
(`Cmd/Ctrl+R` in that window) — no repackaging needed.

## To install it permanently instead

Copy the whole folder into your VS Code extensions directory:

- macOS/Linux: `~/.vscode/extensions/voidstar-ext`
- Windows: `%USERPROFILE%\.vscode\extensions\voidstar-ext`

Then restart VS Code.

## Where to tweak things (ignore if you're not a contributor)

- `syntaxes/q4r.tmLanguage.json` → all coloring logic. Each `repository`
  entry is one token category; edit the `match` regex or add new entries as
  the syntax evolves.
- `language-configuration.json` → non-color editor behavior (auto-closing
  brackets, comment toggling with `Ctrl+/`).
