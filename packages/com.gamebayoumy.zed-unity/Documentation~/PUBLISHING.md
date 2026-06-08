# Publishing the Unity package

Package name: `com.gamebayoumy.zed-unity`

## Release checklist

1. Make sure `package.json` has the target version.
2. Add a Changeset:

   ```sh
   npx changeset
   ```

3. Apply Changesets version/changelog updates:

   ```sh
   npx changeset version
   ```

4. Validate the package in Unity 2021.3 or newer:
   - Package imports without compile errors.
   - `Edit > Preferences > External Tools` lists Zed when installed.
   - `Tools > Zed > Setup / Health Check` opens.
   - `Fix Everything Possible` creates/updates `.zed/settings.json`.
   - `Regenerate Project Files` creates/updates `.sln` and `.csproj` files.
   - Double-clicking a script opens Zed at the requested line.

5. Commit the release changes.
6. Tag the release:

   ```sh
   git tag v0.2.0
   git push origin main --tags
   ```

7. Create a GitHub release from that tag.

## Publishing to OpenUPM

OpenUPM publishes from Git tags and Unity package metadata.

1. Ensure `package.json` is valid and has:
   - `name`
   - `version`
   - `displayName`
   - `unity`
   - `repository`
   - `dependencies`
2. Create a public GitHub release/tag like `v0.2.0`.
3. Go to <https://openupm.com/packages/add/>.
4. Submit the GitHub repository URL.
5. Set package path if the package is in a monorepo:

   ```text
   packages/com.gamebayoumy.zed-unity
   ```

6. After OpenUPM indexes the package, users can install it with:

   ```sh
   openupm add com.gamebayoumy.zed-unity
   ```

## Git URL installation

Monorepo package path:

```text
https://github.com/GameBayoumy/zed-unity.git?path=/packages/com.gamebayoumy.zed-unity
```

Version-pinned Git URL:

```text
https://github.com/GameBayoumy/zed-unity.git?path=/packages/com.gamebayoumy.zed-unity#v0.2.0
```
