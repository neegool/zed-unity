# Changelog

All notable changes to the USS language server are documented here.

This project follows [Semantic Versioning](https://semver.org/).

## [0.3.0] - 2026-06-09

### Changed

- Aligned the USS language server version with the `GameBayoumy/zed-unity` monorepo release.
- Release assets are now produced from the monorepo release workflow.

## [0.1.1] - 2026-06-08

### Changed

- Refreshed compatible Rust dependencies.
- Added Changesets-based release tooling.

## [0.1.0] - 2024-12-10

### Added

- Initial release.
- Auto-completion for USS properties and values.
- Support for Unity-specific properties (`-unity-*`).
- Selector completion for type, class, and ID selectors.
- Pseudo-class support (`:hover`, `:active`, `:focus`, `:disabled`, `:checked`, `:selected`).
- USS variable support (`--custom-var`).
- Hover documentation for properties.
- Basic diagnostics for syntax errors.
- Color value completion with Unity USS colors.
- Unit and keyword value completion.
