# Piper Language — VS Code Extension

Syntax highlighting, formatting, and run support for [Piper](https://github.com/GaliRithvik/piper) (`.piper` files).

## Features

- **Syntax highlighting** — keywords, strings, f-strings, operators (`|>`, `??`, `?.`), classes, functions
- **Auto-indent** — pressing Enter after a `:` line automatically indents
- **Bracket / quote matching** — `(`, `[`, `{`, `"`
- **Run file** — executes the current file in a terminal (`Ctrl+Shift+R` / `Cmd+Shift+R`)
- **Format file** — runs `piper fmt` to clean up the code (`Ctrl+Shift+I` / `Cmd+Shift+I`)
- **Format on save** — optional (disabled by default, enable in settings)

## Requirements

The `piper` binary must be installed and on your `PATH`:

```bash
# Build from source (requires Rust)
git clone https://github.com/GaliRithvik/piper
cd piper
cargo build --release
# Copy to PATH:
cp target/release/piper /usr/local/bin/piper
```

## Installation

### Option A — Development mode (no packaging needed)

1. Open the `piper-vscode` folder in VS Code
2. Press `F5` — this launches a new VS Code window with the extension loaded
3. Open any `.piper` file to see syntax highlighting

### Option B — Install as VSIX

```bash
# Install vsce (VS Code Extension CLI)
npm install -g @vscode/vsce

cd piper-vscode
vsce package          # produces piper-lang-0.9.0.vsix
code --install-extension piper-lang-0.9.0.vsix
```

## Settings

| Setting | Default | Description |
|---|---|---|
| `piper.executablePath` | `"piper"` | Path to the `piper` binary |
| `piper.formatOnSave` | `false` | Run `piper fmt` on every save |
| `piper.reuseTerminal` | `true` | Reuse the Piper terminal panel |

## Keyboard Shortcuts

| Shortcut | Action |
|---|---|
| `Ctrl+Shift+R` / `Cmd+Shift+R` | Run current `.piper` file |
| `Ctrl+Shift+I` / `Cmd+Shift+I` | Format current `.piper` file |
| `Shift+Alt+F` | VS Code Format Document (also triggers `piper fmt`) |

## Formatter

The formatter (`piper fmt file.piper`) applies these transformations:

- Strip trailing whitespace
- Normalize leading tabs → 4 spaces
- Cap consecutive blank lines at 2
- Insert a blank line before top-level `fn` / `class` declarations
- Ensure a space after `#` in comments (`#foo` → `# foo`)
- Add a space after `,` where missing

```bash
piper fmt myscript.piper       # formats in place
```
