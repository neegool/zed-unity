# Zed Unity Package

Unity package that registers [Zed](https://zed.dev) as an External Script Editor and prepares Unity projects for a good Zed experience on Linux, macOS, and Windows.

## Features

- Registers Zed as a Unity External Script Editor
- Opens scripts at the requested line/column from Unity
- Detects common Zed installs across Linux, macOS, and Windows
- Creates/merges Unity-friendly `.zed/settings.json` with Roslyn-friendly C# defaults
- Regenerates `.sln` / `.csproj` files through Unity's maintained Visual Studio project generator for Zed's official C# extension
- Refreshes Unity when files are changed externally
- Provides `Tools > Zed > Setup / Health Check`

## Installation

### Git URL

In Unity:

1. Open **Window > Package Manager**
2. Click **+** > **Add package from git URL**
3. Enter:

```text
https://github.com/GameBayoumy/zed-unity.git?path=/packages/com.gamebayoumy.zed-unity
```

### OpenUPM

With OpenUPM:

```sh
openupm add com.gamebayoumy.zed-unity
```

## Setup

1. Install the Unity package.
2. Install Zed's official C# extension for `.cs` / Roslyn support.
3. Install the Zed Unity extension for USS / TSS support.
4. In Unity, open **Edit > Preferences > External Tools**.
5. Set **External Script Editor** to **Zed**.
6. Run **Tools > Zed > Setup / Health Check**.
7. Click **Fix Everything Possible**.

## Menu commands

- **Tools > Zed > Setup / Health Check**
- **Tools > Zed > Open Project in Zed**
- **Tools > Zed > Regenerate Project Files**
- **Tools > Zed > Create / Update Zed Settings**
- **Tools > Zed > Force Sync**
- **Tools > Zed > Open Preferences**

## Settings

| Setting | Default | Description |
| --- | --- | --- |
| Zed Path | Auto-detected | Path to Zed executable |
| Open in New Window | Off | Open files in a new Zed window |
| Enable File Sync | On | Refresh Unity after external file changes |
| Sync Interval | 1.0s | File change processing interval |
| Generate .sln File | On | Enable Unity project generation |
| Generate .csproj Files | On | Enable Unity project generation |

Project generation is delegated to `com.unity.ide.visualstudio` so Roslyn can consume Unity-compatible `.sln` / `.csproj` inputs.

## Supported platforms

| Platform | Common executables / installs |
| --- | --- |
| Linux | `zed`, `zeditor`, Flatpak, Snap, Nix, AppImage |
| macOS | app bundle CLI, Homebrew, user Applications folder |
| Windows | `zed.exe`, Scoop, Chocolatey, WinGet, portable installs |

## Release workflow

This package is versioned from the monorepo root. See `../../docs/PUBLISHING.md` for the canonical release guide.

## License

MIT License - see [LICENSE.md](LICENSE.md).
