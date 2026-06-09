# USS Language Server

Language Server Protocol implementation for USS/TSS files used by Unity UI Toolkit.

This crate is maintained inside the [`GameBayoumy/zed-unity`](https://github.com/GameBayoumy/zed-unity) monorepo. The Zed Unity extension downloads release binaries from the monorepo GitHub releases.

## Features

- Completion for USS properties, values, selectors, and pseudo-classes
- Hover documentation for properties and values
- Diagnostics for syntax errors and unknown properties
- Go to definition for USS variables
- Document formatting
- Color preview support

## Release assets

The Zed Unity extension expects these asset names in monorepo GitHub releases:

| Platform | Asset |
| --- | --- |
| Linux x64 | `uss-language-server-linux-x64.tar.gz` |
| Linux ARM64 | `uss-language-server-linux-arm64.tar.gz` |
| macOS x64 | `uss-language-server-darwin-x64.tar.gz` |
| macOS ARM64 | `uss-language-server-darwin-arm64.tar.gz` |
| Windows x64 | `uss-language-server-win-x64.zip` |

Do not change these names without updating `src/lib.rs` in the Zed extension.

## Development

From the monorepo root:

```sh
cargo check --manifest-path crates/uss-language-server/Cargo.toml
cargo test --manifest-path crates/uss-language-server/Cargo.toml
cargo build --release --manifest-path crates/uss-language-server/Cargo.toml
```

The release binary is written under `target/release/uss-language-server` for native builds.

## Usage outside Zed

Configure your editor's LSP client to run the binary over stdio:

```sh
uss-language-server
```

Set `RUST_LOG=debug` when you need verbose server logs.

## License

MIT License - see the monorepo license.
