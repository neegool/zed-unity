# USS Language Server

[![Release](https://img.shields.io/github/v/release/GameBayoumy/uss-language-server)](https://github.com/GameBayoumy/uss-language-server/releases)
[![CI](https://github.com/GameBayoumy/uss-language-server/actions/workflows/ci.yml/badge.svg)](https://github.com/GameBayoumy/uss-language-server/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A Language Server Protocol (LSP) implementation for USS (Unity Style Sheets) - Unity's CSS-like styling language for UI Toolkit.

- **Auto-completion** for USS properties, values, selectors, and pseudo-classes
- **Hover documentation** for properties and values
- **Diagnostics** for syntax errors and unknown properties
- **Go to Definition** for USS variables
- **Document Formatting**
- **Color Preview** support

## Supported USS Features

### Properties
- All standard USS properties
- Unity-specific properties (`-unity-font-style`, `-unity-text-align`, etc.)
- Custom USS variables (`--my-variable`)

### Selectors
- Type selectors (`Button`, `Label`, `VisualElement`)
- Class selectors (`.my-class`)
- ID selectors (`#my-id`)
- Pseudo-classes (`:hover`, `:active`, `:focus`, `:disabled`, `:checked`, `:selected`)
- Descendant and child combinators

### Values
- Colors (hex, rgb, rgba, named colors)
- Units (px, %, em)
- Keywords (auto, none, flex, etc.)
- Functions (url(), var(), rgb(), rgba())

## Installation

### Pre-built Binaries

Download the latest release for your platform from the [Releases](https://github.com/GameBayoumy/uss-language-server/releases) page.

| Platform | Binary |
|----------|--------|
| Linux x64 | `uss-language-server-linux-x64.tar.gz` |
| Linux ARM64 | `uss-language-server-linux-arm64.tar.gz` |
| macOS x64 | `uss-language-server-darwin-x64.tar.gz` |
| macOS ARM64 | `uss-language-server-darwin-arm64.tar.gz` |
| Windows x64 | `uss-language-server-win-x64.zip` |

### From Source

```bash
git clone https://github.com/GameBayoumy/uss-language-server.git
cd uss-language-server
cargo build --release
```

The binary will be at `target/release/uss-language-server`.

## Usage

### Zed Editor

The [zed-unity](https://github.com/GameBayoumy/zed-unity) extension automatically downloads and uses this language server.

### Other Editors

Configure your editor's LSP client to run `uss-language-server` with stdio communication:

```bash
uss-language-server
```

The server communicates over stdin/stdout using the Language Server Protocol.

### VS Code (Manual)

Add to your `settings.json`:

```json
{
  "languageServerExample.serverPath": "/path/to/uss-language-server"
}
```

### Neovim (with nvim-lspconfig)

```lua
local lspconfig = require('lspconfig')
local configs = require('lspconfig.configs')

configs.uss = {
  default_config = {
    cmd = { 'uss-language-server' },
    filetypes = { 'uss' },
    root_dir = lspconfig.util.root_pattern('.git', '*.sln', '*.csproj'),
  },
}

lspconfig.uss.setup{}
```

## Environment Variables

- `RUST_LOG`: Set logging level (e.g., `RUST_LOG=debug uss-language-server`)

## Development

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Release Build

```bash
cargo build --release
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT License - see [LICENSE](LICENSE) for details.

## Related Projects

- [zed-unity](https://github.com/GameBayoumy/zed-unity) - Unity development extension for Zed
- [Unity UI Toolkit Documentation](https://docs.unity3d.com/Manual/UIE-USS.html)
