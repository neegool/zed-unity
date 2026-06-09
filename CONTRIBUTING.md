# Contributing

Thanks for helping improve Zed Unity.

## Repository layout

| Path | Owns |
| --- | --- |
| `/` | Zed extension for Unity USS/TSS support |
| `packages/com.gamebayoumy.zed-unity` | Unity package for external editor setup, project generation, and `.zed/settings.json` creation |
| `crates/uss-language-server` | USS/TSS language server downloaded by the Zed extension |

C# language support is handled by Zed's official C# extension and Roslyn. This repo should not add another C# language server unless that direction is explicitly revisited.

## Local setup

Install JavaScript dependencies once:

```sh
npm install
```

Install the Zed extension WASI target when working on the extension:

```sh
rustup target add wasm32-wasip2
```

## Validation

Run the focused checks before opening a pull request:

```sh
cargo fmt --all -- --check
cargo check
cargo check --manifest-path crates/uss-language-server/Cargo.toml
npm run check:unity-package
```

Build the Zed extension WASM when touching extension code:

```sh
cargo build --release --target wasm32-wasip2
```

If your system Rust is not rustup-managed, run the build with a rustup toolchain so `wasm32-wasip2` can be found.

## Unity package changes

When changing files under `packages/com.gamebayoumy.zed-unity`:

- Keep compatibility with Unity 2021.3 or newer.
- Prefer Unity's maintained `com.unity.ide.visualstudio` project generator over custom `.sln` / `.csproj` generation.
- Do not overwrite user `.zed/settings.json` preferences. Merge conservatively.
- Keep OpenUPM metadata in `package.json` valid.

## Zed extension changes

When changing extension behavior:

- Keep this extension focused on Unity-specific USS/TSS support.
- Do not reintroduce the archived `csharp-language-server` dependency.
- Keep release asset names in sync with `docs/PUBLISHING.md` and `src/lib.rs`.

## Releases

Use `docs/PUBLISHING.md` as the single release guide. Releases use bare semver tags such as `0.3.0`, not `v0.3.0`.
