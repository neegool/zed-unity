# Zed Unity

Monorepo for Unity development support in [Zed](https://zed.dev).

## Packages

| Path | Purpose |
| --- | --- |
| `/` | Zed extension for Unity USS/TSS support |
| `packages/com.gamebayoumy.zed-unity` | Unity Package Manager package |
| `crates/uss-language-server` | USS language server downloaded by the Zed extension |

## Features

- USS / TSS language support for Unity UI Toolkit files
- Automatic USS language server download from GitHub releases
- Tree-sitter syntax highlighting for USS / TSS files
- Companion Unity package integration for external editor setup and Unity project file generation
- Roslyn-friendly Unity project settings for Zed's official C# extension

C# support is provided by Zed's official C# extension, which uses `roslyn-language-server` by default. This project focuses on the Unity-specific glue around that integration.

## Install

### Zed extensions

Install Zed's official C# extension for `.cs` / Roslyn support.

For local development of this extension:

```sh
rustup target add wasm32-wasip2
./build.sh
```

Then in Zed, install this folder as a dev extension.

### Unity package

Install the Unity package for External Script Editor registration and project setup:

```text
https://github.com/GameBayoumy/zed-unity.git?path=/packages/com.gamebayoumy.zed-unity
```

Or use OpenUPM with:

```sh
openupm add com.gamebayoumy.zed-unity
```

After installing, run:

```text
Tools > Zed > Setup / Health Check
```

## Configuration

The Unity package can create a Unity-friendly `.zed/settings.json` with Roslyn enabled for C#:

```json
{
  "languages": {
    "CSharp": {
      "language_servers": ["roslyn", "..."]
    }
  },
  "lsp": {
    "roslyn": {
      "settings": {
        "csharp|projects": {
          "dotnet_enable_automatic_restore": true
        },
        "csharp|background_analysis": {
          "dotnet_analyzer_diagnostics_scope": "openFiles",
          "dotnet_compiler_diagnostics_scope": "openFiles"
        }
      }
    }
  }
}
```

To override the Roslyn binary itself, configure Zed's official C# extension using the `roslyn` LSP settings documented at <https://zed.dev/docs/languages/csharp>.

## Supported files

| Extension | Support |
| --- | --- |
| `.cs` | C# via Zed's official C# extension and Roslyn |
| `.uss` | Unity Style Sheets via this extension |
| `.tss` | Unity Theme Style Sheets via this extension |

## Requirements

- Zed with Rust extension support
- Zed's official C# extension for C# / Roslyn support
- Unity 2021.3 or newer for the Unity package
- .NET SDK for C# language server functionality

## Release workflow

This repo uses Changesets for monorepo release notes/versioning metadata:

```sh
npm install
npm run changeset
npm run version-packages
```

`npm run version-packages` updates the Zed extension, Unity package, and USS language server versions together where applicable.

See `docs/PUBLISHING.md` for release and OpenUPM instructions.

## Contributing

See `CONTRIBUTING.md` for repository layout, validation commands, and contribution guidelines.

## Roadmap

- Unity debugger attach support
- ShaderLab / HLSL support
- UXML support
- Unity editor IPC commands

## License

MIT License - see [LICENSE.md](LICENSE.md).
