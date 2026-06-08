# Changelog

All notable changes to this package are documented here.

This package follows [Semantic Versioning](https://semver.org/) and uses [Changesets](https://github.com/changesets/changesets) for release notes.

## [0.2.0] - 2026-06-08

### Added

- Added `Tools > Zed > Setup / Health Check` for guided setup and diagnostics.
- Added Unity-friendly `.zed/settings.json` creation and merge support.
- Added `Tools > Zed > Create / Update Zed Settings`.
- Added dependency on Unity's maintained `com.unity.ide.visualstudio` project generator.
- Added improved Zed executable detection for Linux, macOS, Windows, Flatpak, Snap, Nix, Scoop, Chocolatey, and WinGet installs.

### Changed

- Renamed package from `com.zed.unity` to `com.gamebayoumy.zed-unity` to avoid implying official Zed ownership.
- Replaced custom `.sln` / `.csproj` generation with Unity's maintained Visual Studio generator.
- Centralized Zed launching for Preferences, menu commands, and health-check actions.
- Improved cross-platform command-line argument quoting and file location opening.
- Replaced duplicate file-sync instances with a single static file-sync controller.

### Removed

- Removed obsolete package/analyzer project-generation toggles that no longer controlled anything with Unity's generator.
- Removed the fake `/usr/bin/zed` placeholder installation entry.

## [0.1.0] - 2024-12-10

### Added

- Initial release.
- `IExternalCodeEditor` implementation for Unity integration.
- Automatic Zed executable detection for Linux, macOS, and Windows.
- File opening with line/column navigation support.
- File synchronization using `FileSystemWatcher`.
- Unity preferences UI for configuration.
- Menu commands under `Tools > Zed`.
- Support for common Unity-related file types.
