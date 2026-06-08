# Publishing the USS language server

## Release checklist

1. Add a Changeset:

   ```sh
   npm run changeset
   ```

2. Apply version updates:

   ```sh
   npm run version-packages
   ```

   This updates `package.json` and `Cargo.toml`.

3. Validate:

   ```sh
   cargo fmt --all -- --check
   cargo clippy -- -D warnings
   cargo test
   cargo build --release
   ```

4. Commit release changes.
5. Tag the release:

   ```sh
   git tag v0.1.1
   git push origin main --tags
   ```

6. The existing GitHub Actions release workflow builds assets for:
   - Linux x64
   - Linux ARM64
   - macOS x64
   - macOS ARM64
   - Windows x64

## Required asset names

The Zed extension expects these names:

```text
uss-language-server-linux-x64.tar.gz
uss-language-server-linux-arm64.tar.gz
uss-language-server-darwin-x64.tar.gz
uss-language-server-darwin-arm64.tar.gz
uss-language-server-win-x64.zip
```

Do not change these names without updating `zed-unity/src/lib.rs`.
