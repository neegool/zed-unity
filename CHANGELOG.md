# Changelog

All notable changes to the Zed Unity extension are documented here.

This project follows [Semantic Versioning](https://semver.org/) and uses [Changesets](https://github.com/changesets/changesets) for release notes.

## [0.2.1] - 2026-06-08

### Fixed

- Added corrected release metadata for the Unity package and monorepo release assets.

## [0.2.0] - 2026-06-08

### Added

- Added Changesets release tooling and version sync for `Cargo.toml` and `extension.toml`.
- Added Linux ARM64 and Windows ARM64 asset-name support for `csharp-language-server` downloads.

### Changed

- Updated extension metadata to describe only currently implemented features.
- Simplified `build.sh` to build only the Zed extension WASM with `wasm32-wasip2`.
- Refreshed compatible Rust dependencies in `Cargo.lock`.

### Removed

- Removed unused/incomplete `netcoredbg` debugger installer code until debugger support is implemented end-to-end.

## [0.1.0] - 2024-12-10

### Added

- Initial Zed extension with C# and USS language server integration.
- C# and USS language configuration files.
- USS language server download support.
