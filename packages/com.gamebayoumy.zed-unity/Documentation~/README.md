# Zed Unity Integration

This Unity package registers [Zed](https://zed.dev) as an external code editor and prepares Unity projects for a better Zed workflow on Linux, macOS, and Windows.

## Features

- Opens Unity scripts in Zed at the requested line/column
- Detects common Zed installations across Linux, macOS, and Windows
- Creates or merges Unity-friendly `.zed/settings.json` exclusions and Roslyn-friendly C# defaults
- Regenerates `.sln` and `.csproj` files through Unity's maintained Visual Studio project generator
- Refreshes Unity when files are changed externally
- Provides guided setup from `Tools > Zed > Setup / Health Check`

C# language support comes from Zed's official C# extension and Roslyn. This package generates the Unity project files and Zed settings that Roslyn needs.

## Installation

### Git URL

1. Open Unity Editor.
2. Go to **Window > Package Manager**.
3. Click **+** > **Add package from git URL**.
4. Enter:

```text
https://github.com/GameBayoumy/zed-unity.git?path=/packages/com.gamebayoumy.zed-unity
```

### OpenUPM

```sh
openupm add com.gamebayoumy.zed-unity
```

### Local installation

1. Clone the monorepo.
2. In Unity Package Manager, click **+** > **Add package from disk**.
3. Select `packages/com.gamebayoumy.zed-unity/package.json`.

## Setup

1. Install Zed's official C# extension for `.cs` / Roslyn support.
2. Install the Zed Unity extension for USS / TSS support.
3. Go to **Edit > Preferences > External Tools**.
4. Set **External Script Editor** to **Zed**.
5. Run **Tools > Zed > Setup / Health Check**.
6. Click **Fix Everything Possible**.

## Menu commands

- **Tools > Zed > Setup / Health Check**
- **Tools > Zed > Open Project in Zed**
- **Tools > Zed > Regenerate Project Files**
- **Tools > Zed > Create / Update Zed Settings**
- **Tools > Zed > Force Sync**
- **Tools > Zed > Open Preferences**

## Troubleshooting

### Zed not detected

1. Run **Tools > Zed > Setup / Health Check**.
2. Click **Auto-Detect Zed**.
3. Or manually browse to the Zed executable in **Edit > Preferences > External Tools**.

### Project files not generating

1. Check the Unity console for errors.
2. Ensure `com.unity.ide.visualstudio` is installed.
3. Run **Tools > Zed > Regenerate Project Files**.

### C# LSP not working in Zed

1. Ensure Zed's official C# extension is installed and Roslyn is enabled.
2. Run **Tools > Zed > Setup / Health Check** in Unity.
3. Ensure a `.sln` file exists in the Unity project root.
4. Confirm the Unity project has a `.zed/settings.json` with `CSharp` using `roslyn`.

### USS / TSS LSP not working in Zed

1. Ensure the Zed Unity extension is installed in Zed.
2. Open a `.uss` or `.tss` file.
3. If needed, configure `lsp.uss-language-server.binary.path` in Zed settings.

## License

MIT License - see `LICENSE.md`.
