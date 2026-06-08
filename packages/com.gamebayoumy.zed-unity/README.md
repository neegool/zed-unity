# Zed Unity Package

Unity package that registers [Zed](https://zed.dev) as an External Script Editor and prepares Unity projects for a good Zed experience on Linux, macOS, and Windows.

## Features

- Registers Zed as a Unity External Script Editor
- Opens scripts at the requested line/column from Unity
- Detects common Zed installs across Linux, macOS, and Windows
- Creates/merges Unity-friendly `.zed/settings.json`
- Regenerates `.sln` / `.csproj` files through Unity's maintained Visual Studio project generator
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

After publishing to OpenUPM:

```sh
openupm add com.gamebayoumy.zed-unity
```

## Setup

1. Install the Unity package.
2. Install the Zed extension in Zed.
3. In Unity, open **Edit > Preferences > External Tools**.
4. Set **External Script Editor** to **Zed**.
5. Run **Tools > Zed > Setup / Health Check**.
6. Click **Fix Everything Possible**.

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

Project generation is delegated to `com.unity.ide.visualstudio` for better Unity compatibility.

## Supported platforms

| Platform | Common executables / installs |
| --- | --- |
| Linux | `zed`, `zeditor`, Flatpak, Snap, Nix, AppImage |
| macOS | app bundle CLI, Homebrew, user Applications folder |
| Windows | `zed.exe`, Scoop, Chocolatey, WinGet, portable installs |

## Release workflow

This package uses Changesets for changelog/version management:

```sh
npx changeset
npx changeset version
```

For OpenUPM, publish by creating a GitHub release/tag that matches the package version, for example `v0.2.0`.

## License

MIT License - see [LICENSE.md](LICENSE.md).
