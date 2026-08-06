using System;
using System.IO;
using System.Text;
using Unity.CodeEditor;
using UnityEditor;
using UnityEngine;
using Debug = UnityEngine.Debug;

namespace Zed.Unity.Editor
{
    /// <summary>
    /// Zed Code Editor integration for Unity.
    /// Implements IExternalCodeEditor to register Zed as an external script editor.
    /// </summary>
    [InitializeOnLoad]
    public class ZedEditor : IExternalCodeEditor
    {
        private static readonly string[] SupportedExtensions = { ".cs", ".shader", ".compute", ".hlsl", ".cginc", ".uss", ".uxml", ".json", ".xml", ".txt", ".md", ".asmdef" };

        private readonly ProjectGeneration _projectGeneration;

        static ZedEditor()
        {
            // Register this editor with Unity's CodeEditor system
            CodeEditor.Register(new ZedEditor());
        }

        public ZedEditor()
        {
            _projectGeneration = new ProjectGeneration();
        }

        /// <summary>
        /// Display name shown in Unity's External Tools preferences.
        /// </summary>
        public CodeEditor.Installation[] Installations => GetInstallations();

        /// <summary>
        /// Called when the user opens a file from Unity (double-click, error log, etc.)
        /// </summary>
        public bool OpenProject(string filePath, int line, int column)
        {
            if (!string.IsNullOrEmpty(filePath) && !IsSupportedFile(filePath))
            {
                return false;
            }

            string zedPath = ZedConfig.ZedPath;

            if (string.IsNullOrEmpty(zedPath) || !File.Exists(zedPath))
            {
                zedPath = ZedUtils.FindZedExecutable();
                if (string.IsNullOrEmpty(zedPath))
                {
                    Debug.LogError("[Zed Unity] Could not find Zed executable. Please set the path in Preferences > External Tools.");
                    return false;
                }
                ZedConfig.ZedPath = zedPath;
            }

            if (!string.IsNullOrEmpty(filePath))
            {
                filePath = Path.GetFullPath(filePath);
            }

            return LaunchZed(zedPath, filePath, line, column);
        }

        /// <summary>
        /// Build command line arguments for Zed.
        /// </summary>
        private static string BuildArguments(string filePath, int line, int column)
        {
            string projectPath = ZedUtils.GetProjectPath();
            var args = new StringBuilder();

            if (ZedConfig.OpenInNewWindow)
            {
                args.Append("-n ");
            }

            args.Append(ZedUtils.QuoteArgument(projectPath));

            if (!string.IsNullOrEmpty(filePath))
            {
                args.Append(" -a ");
                args.Append(ZedUtils.QuoteArgument(AppendLocation(filePath, line, column)));
            }

            return args.ToString();
        }

        private static string AppendLocation(string filePath, int line, int column)
        {
            if (line <= 0)
                return filePath;

            return column > 0
                ? $"{filePath}:{line}:{column}"
                : $"{filePath}:{line}";
        }

        private static bool LaunchZed(string zedPath, string filePath = "", int line = -1, int column = -1)
        {
            string arguments = BuildArguments(filePath, line, column);

            try
            {
                bool opened = CodeEditor.OSOpenFile(zedPath, arguments);

                if (ZedConfig.EnableLogging)
                {
                    Debug.Log($"[Zed Unity] Opening: {zedPath} {arguments}");
                }

                if (!opened)
                {
                    Debug.LogError("[Zed Unity] Unity failed to open Zed. Verify the executable path in Preferences > External Tools.");
                }

                return opened;
            }
            catch (Exception ex)
            {
                Debug.LogError($"[Zed Unity] Failed to open Zed: {ex.Message}");
                return false;
            }
        }

        public static bool OpenProjectInZed()
        {
            string zedPath = ZedConfig.ZedPath;
            if (string.IsNullOrEmpty(zedPath) || !File.Exists(zedPath))
            {
                zedPath = ZedUtils.FindZedExecutable();
                if (!string.IsNullOrEmpty(zedPath))
                {
                    ZedConfig.ZedPath = zedPath;
                }
            }

            if (string.IsNullOrEmpty(zedPath) || !File.Exists(zedPath))
            {
                EditorUtility.DisplayDialog(
                    "Zed Not Found",
                    "Could not find Zed executable. Configure the path in Edit > Preferences > External Tools or run Tools > Zed > Setup / Health Check.",
                    "OK");
                return false;
            }

            return LaunchZed(zedPath);
        }

        /// <summary>
        /// Synchronize (regenerate) all project files.
        /// </summary>
        public void SyncAll()
        {
            _projectGeneration.GenerateAll();

            if (ZedConfig.EnableLogging)
            {
                Debug.Log("[Zed Unity] Project files synchronized.");
            }
        }

        /// <summary>
        /// Synchronize project files if needed.
        /// </summary>
        public void SyncIfNeeded(string[] addedFiles, string[] deletedFiles, string[] movedFiles, string[] movedFromFiles, string[] importedFiles)
        {
            if (!HasSupportedFile(addedFiles) &&
                !HasSupportedFile(deletedFiles) &&
                !HasSupportedFile(movedFiles) &&
                !HasSupportedFile(movedFromFiles) &&
                !HasSupportedFile(importedFiles))
            {
                return;
            }

            _projectGeneration.SyncIfNeeded(addedFiles, deletedFiles, movedFiles, movedFromFiles, importedFiles);

            if (ZedConfig.EnableLogging)
            {
                Debug.Log("[Zed Unity] Project files synchronized due to file changes.");
            }
        }

        /// <summary>
        /// Initialize the code editor when it becomes the active editor.
        /// </summary>
        public void Initialize(string editorInstallationPath)
        {
            ZedConfig.ZedPath = editorInstallationPath;
            ZedProjectSettings.EnsureProjectSettings();
            FileSync.SetEnabled(ZedConfig.EnableFileSync);
        }

        /// <summary>
        /// Draw custom GUI in the External Tools preferences.
        /// </summary>
        public void OnGUI()
        {
            ZedPreferences.DrawPreferencesGUI();
        }

        /// <summary>
        /// Check if a given file path can be opened by this editor.
        /// </summary>
        public bool TryGetInstallationForPath(string editorPath, out CodeEditor.Installation installation)
        {
            if (ZedUtils.IsValidZedPath(editorPath))
            {
                installation = new CodeEditor.Installation
                {
                    Name = "Zed",
                    Path = editorPath
                };
                return true;
            }

            installation = default;
            return false;
        }

        /// <summary>
        /// Get all available Zed installations.
        /// </summary>
        private CodeEditor.Installation[] GetInstallations()
        {
            var installations = new System.Collections.Generic.List<CodeEditor.Installation>();

            // If a custom path is set and valid, add it first
            if (!string.IsNullOrEmpty(ZedConfig.ZedPath) && File.Exists(ZedConfig.ZedPath))
            {
                installations.Add(new CodeEditor.Installation
                {
                    Name = "Zed",
                    Path = ZedConfig.ZedPath
                });
            }

            AddDetectedInstallations(installations, ZedUtils.GetPossibleZedPaths());
            AddDetectedInstallations(installations, ZedUtils.GetDiscoveredZedPaths());

            return installations.ToArray();
        }

        /// <summary>
        /// Check if a file extension is supported.
        /// </summary>
        private static void AddDetectedInstallations(System.Collections.Generic.List<CodeEditor.Installation> installations, System.Collections.Generic.IEnumerable<string> paths)
        {
            foreach (string path in paths)
            {
                if (!File.Exists(path))
                    continue;

                bool alreadyAdded = false;
                foreach (var installation in installations)
                {
                    if (installation.Path == path)
                    {
                        alreadyAdded = true;
                        break;
                    }
                }

                if (!alreadyAdded)
                {
                    installations.Add(new CodeEditor.Installation
                    {
                        Name = "Zed",
                        Path = path
                    });
                }
            }
        }

        private static bool HasSupportedFile(string[] files)
        {
            if (files == null)
                return false;

            foreach (string file in files)
            {
                if (IsSupportedFile(file))
                    return true;
            }

            return false;
        }

        private static bool IsSupportedFile(string filePath)
        {
            string extension = Path.GetExtension(filePath)?.ToLowerInvariant();
            if (string.IsNullOrEmpty(extension)) return false;

            foreach (string supported in SupportedExtensions)
            {
                if (extension == supported) return true;
            }
            return false;
        }
    }
}
