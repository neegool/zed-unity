# Zed Unity

Monorepo for Unity development support in [Zed](https://zed.dev).

## Packages

| Path | Purpose |
| --- | --- |
| `/` | Zed extension |
| `packages/com.gamebayoumy.zed-unity` | Unity Package Manager package |
| `crates/uss-language-server` | USS language server downloaded by the Zed extension |

## Features

- C# language server integration through [`csharp-language-server`](https://github.com/SofusA/csharp-language-server)
- USS / TSS language support for Unity UI Toolkit files
- Automatic USS language server download from GitHub releases
- Tree-sitter syntax highlighting for C# and USS
- Companion Unity package integration for external editor setup and project file generation

## Install

### Zed extension

Install from Zed's extension manager once published. For local development:

```sh
rustup target add wasm32-wasip2
./build.sh
```

Then in Zed, install this folder as a dev extension.

### Unity package

Install the companion Unity package for External Script Editor registration and project setup:

```text
https://github.com/GameBayoumy/zed-unity.git?path=/packages/com.gamebayoumy.zed-unity
```

After OpenUPM publishing, users can install with:

```sh
openupm add com.gamebayoumy.zed-unity
```

After installing, run:

```text
Tools > Zed > Setup / Health Check
```

## Configuration

To override the C# language server binary in Zed settings:

```json
{
  "lsp": {
    "csharp-language-server": {
      "binary": {
        "path": "/path/to/csharp-language-server",
        "arguments": []
      }
    }
  }
}
```

## Supported files

| Extension | Language |
| --- | --- |
| `.cs` | C# |
| `.uss` | Unity Style Sheets |
| `.tss` | Unity Theme Style Sheets |

## Requirements

- Zed with Rust extension support
- Unity 2021.3 or newer for the companion Unity package
- .NET SDK for C# language server functionality

## Release workflow

This repo uses Changesets for monorepo release notes/versioning metadata:

```sh
npm install
npm run changeset
npm run version-packages
```

`npm run version-packages` updates the Zed extension, Unity package, and USS language server versions together where applicable.

See `docs/PUBLISHING.md` for GitHub release and OpenUPM instructions.

## Roadmap

- Unity debugger attach support
- ShaderLab / HLSL support
- UXML support
- Unity editor IPC commands

## License

MIT License - see [LICENSE.md](LICENSE.md).
