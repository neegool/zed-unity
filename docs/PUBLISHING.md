# Publishing Zed Unity

This repository is a monorepo containing:

- Zed extension at the repository root
- Unity package at `packages/com.gamebayoumy.zed-unity`
- USS language server at `crates/uss-language-server`

## Versioning

Use Changesets from the repository root:

```sh
npm install
npm run changeset
npm run version-packages
```

`npm run version-packages` updates:

- root `package.json`
- root `Cargo.toml`
- root `extension.toml`
- `packages/com.gamebayoumy.zed-unity/package.json`
- `crates/uss-language-server/package.json`
- `crates/uss-language-server/Cargo.toml`

## Validation

From the repository root:

```sh
npm run check:unity-package
cargo check
cargo check --manifest-path crates/uss-language-server/Cargo.toml
```

CI additionally checks formatting, clippy, tests, and the Zed extension WASI target.

## GitHub release

Create one tag for the whole monorepo:

```sh
git tag v0.2.0
git push origin main --tags
```

The root release workflow publishes:

- `zed-unity-extension.tar.gz`
- `uss-language-server-linux-x64.tar.gz`
- `uss-language-server-linux-arm64.tar.gz`
- `uss-language-server-darwin-x64.tar.gz`
- `uss-language-server-darwin-arm64.tar.gz`
- `uss-language-server-win-x64.zip`

The Zed extension downloads USS language server assets from `GameBayoumy/zed-unity` releases.

## Publishing the Unity package to OpenUPM

Package name:

```text
com.gamebayoumy.zed-unity
```

Package path:

```text
packages/com.gamebayoumy.zed-unity
```

Steps:

1. Push the monorepo to GitHub.
2. Create a version tag/release, for example `v0.2.0`.
3. Go to <https://openupm.com/packages/add/>.
4. Submit:

   ```text
   https://github.com/GameBayoumy/zed-unity
   ```

5. If OpenUPM asks for a package path, enter:

   ```text
   packages/com.gamebayoumy.zed-unity
   ```

6. After indexing, install with:

   ```sh
   openupm add com.gamebayoumy.zed-unity
   ```

## Unity Git URL install

Use this URL in Unity Package Manager:

```text
https://github.com/GameBayoumy/zed-unity.git?path=/packages/com.gamebayoumy.zed-unity
```

Pinned to a tag:

```text
https://github.com/GameBayoumy/zed-unity.git?path=/packages/com.gamebayoumy.zed-unity#v0.2.0
```

## Zed extension publishing

Submit/update this repository in Zed's extension registry according to Zed's current extension publishing process. Keep `extension.toml` at the repository root.

Do not advertise debugger, ShaderLab, or UXML support as shipped until those paths are implemented and validated.
