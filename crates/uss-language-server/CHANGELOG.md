# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1] - 2026-06-08

### Changed
- Refreshed compatible Rust dependencies.
- Added Changesets-based release tooling.

## [0.1.0] - 2024-12-10

### Added
- Initial release
- Auto-completion for USS properties and values
- Support for Unity-specific properties (`-unity-*`)
- Selector completion (type, class, ID selectors)
- Pseudo-class support (`:hover`, `:active`, `:focus`, `:disabled`, `:checked`, `:selected`)
- USS variable support (`--custom-var`)
- Hover documentation for properties
- Basic diagnostics for syntax errors
- Color value completion with Unity USS colors
- Unit completion (px, %, em)
- Keyword value completion

### Supported Platforms
- Linux x64
- Linux ARM64
- macOS x64 (Intel)
- macOS ARM64 (Apple Silicon)
- Windows x64

[Unreleased]: https://github.com/GameBayoumy/uss-language-server/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/GameBayoumy/uss-language-server/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/GameBayoumy/uss-language-server/releases/tag/v0.1.0
