# Release Guide

This is the canonical release guide for the `GameBayoumy/zed-unity` monorepo.

## What this repo releases

| Artifact | Source | Release destination |
| --- | --- | --- |
| Zed Unity extension archive | repository root | GitHub release asset |
| Unity package | `packages/com.gamebayoumy.zed-unity` | OpenUPM / Unity Git URL |
| USS language server binaries | `crates/uss-language-server` | GitHub release assets downloaded by the Zed extension |

C# support is intentionally not built or distributed from this repo. Users should install Zed's official C# extension, which uses Roslyn by default.

## Versioning

Use one bare semver tag for the whole monorepo, for example `0.3.0`.

The release version should match across:

- root `package.json`
- root `Cargo.toml`
- root `extension.toml`
- `packages/com.gamebayoumy.zed-unity/package.json`
- `crates/uss-language-server/package.json`
- `crates/uss-language-server/Cargo.toml`

To sync versions from the root `package.json` after changing it:

```sh
npm run sync-version
```

## Validation

Run from the repository root before tagging:

```sh
cargo fmt --all -- --check
cargo check
cargo check --manifest-path crates/uss-language-server/Cargo.toml
npm run check:unity-package
```

For the actual Zed extension WASM build:

```sh
rustup target add wasm32-wasip2
cargo build --release --target wasm32-wasip2
```

If your system `cargo`/`rustc` is not managed by rustup, use the rustup toolchain explicitly so the WASI target can be found.

## GitHub release assets

Create and push a bare semver tag:

```sh
git tag 0.3.0
git push origin 0.3.0
```

The release workflow is triggered by tags matching `x.y.z` and uploads:

- `zed-unity-extension.tar.gz`
- `uss-language-server-linux-x64.tar.gz`
- `uss-language-server-linux-arm64.tar.gz`
- `uss-language-server-darwin-x64.tar.gz`
- `uss-language-server-darwin-arm64.tar.gz`
- `uss-language-server-win-x64.zip`

Do not use `v0.3.0` tags for new releases.

## OpenUPM

Package name:

```text
com.gamebayoumy.zed-unity
```

Package path:

```text
packages/com.gamebayoumy.zed-unity
```

OpenUPM indexes GitHub tags. After the `0.3.0` GitHub release exists, OpenUPM should pick up the Unity package from the package path above.

## Unity Git URL installs

Latest from the default branch:

```text
https://github.com/GameBayoumy/zed-unity.git?path=/packages/com.gamebayoumy.zed-unity
```

Pinned to a release tag:

```text
https://github.com/GameBayoumy/zed-unity.git?path=/packages/com.gamebayoumy.zed-unity#0.3.0
```

## Release checklist

1. Update versions to the target release.
2. Run validation commands.
3. Push `main`.
4. Push the bare semver tag.
5. Confirm all GitHub release assets exist.
6. Confirm OpenUPM indexes the Unity package version.
