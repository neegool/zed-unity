using System.IO;
using UnityEditor;
using UnityEngine;

namespace Zed.Unity.Editor
{
    public class ZedSetupWindow : EditorWindow
    {
        private string _testOutput;

        [MenuItem("Tools/Zed/Setup / Health Check")]
        public static void Open()
        {
            var window = GetWindow<ZedSetupWindow>("Zed Setup");
            window.minSize = new Vector2(520, 360);
            window.Show();
        }

        private void OnGUI()
        {
            EditorGUILayout.LabelField("Zed Unity Setup", EditorStyles.boldLabel);
            EditorGUILayout.Space(6);

            string zedPath = ResolveZedPath();
            bool hasZed = !string.IsNullOrEmpty(zedPath) && File.Exists(zedPath);
            bool hasSettings = ZedProjectSettings.SettingsFileExists;
            bool hasSolution = Directory.GetFiles(ZedUtils.GetProjectPath(), "*.sln", SearchOption.TopDirectoryOnly).Length > 0;

            DrawStatus("Zed executable", hasZed, hasZed ? zedPath : "Not found");
            DrawStatus("Project .zed/settings.json", hasSettings, hasSettings ? ZedProjectSettings.SettingsFilePath : "Missing");
            DrawStatus("Unity solution file", hasSolution, hasSolution ? "Found" : "Missing");
            DrawStatus("Unity Visual Studio package", IsVisualStudioGeneratorAvailable(), "Required for maintained .sln/.csproj generation");

            EditorGUILayout.Space(12);
            EditorGUILayout.LabelField("Actions", EditorStyles.boldLabel);

            using (new EditorGUILayout.HorizontalScope())
            {
                if (GUILayout.Button("Auto-Detect Zed"))
                {
                    string detected = ZedUtils.FindZedExecutable();
                    if (!string.IsNullOrEmpty(detected))
                    {
                        ZedConfig.ZedPath = detected;
                        _testOutput = "Found Zed: " + detected;
                    }
                    else
                    {
                        _testOutput = "Could not auto-detect Zed. Use Browse in External Tools preferences.";
                    }
                }

                if (GUILayout.Button("Create / Update Zed Settings"))
                {
                    ZedProjectSettings.EnsureProjectSettings();
                    _testOutput = "Updated .zed/settings.json.";
                }
            }

            using (new EditorGUILayout.HorizontalScope())
            {
                if (GUILayout.Button("Regenerate Project Files"))
                {
                    new ProjectGeneration().GenerateAll();
                    _testOutput = "Requested Unity project file regeneration.";
                }

                if (GUILayout.Button("Open Project in Zed"))
                {
                    ZedEditor.OpenProjectInZed();
                    _testOutput = "Requested project open in Zed.";
                }
            }

            if (GUILayout.Button("Fix Everything Possible"))
            {
                string detected = ZedUtils.FindZedExecutable();
                if (!string.IsNullOrEmpty(detected))
                    ZedConfig.ZedPath = detected;

                ZedProjectSettings.EnsureProjectSettings();
                new ProjectGeneration().GenerateAll();
                _testOutput = "Auto-detection, Zed settings, and project generation completed where possible.";
            }

            EditorGUILayout.Space(12);
            EditorGUILayout.HelpBox(
                "Install the Zed extension separately from Zed's extension manager. The Unity package handles editor registration, project files, and project settings; the Zed extension handles language support.",
                MessageType.Info);

            if (!string.IsNullOrEmpty(_testOutput))
            {
                EditorGUILayout.Space(8);
                EditorGUILayout.HelpBox(_testOutput, MessageType.None);
            }
        }

        private static void DrawStatus(string label, bool ok, string detail)
        {
            using (new EditorGUILayout.HorizontalScope())
            {
                GUILayout.Label(ok ? "✓" : "!", GUILayout.Width(20));
                GUILayout.Label(label, EditorStyles.boldLabel, GUILayout.Width(180));
                GUILayout.Label(detail);
            }
        }

        private static string ResolveZedPath()
        {
            if (!string.IsNullOrEmpty(ZedConfig.ZedPath) && File.Exists(ZedConfig.ZedPath))
                return ZedConfig.ZedPath;

            return ZedUtils.FindZedExecutable();
        }

        private static bool IsVisualStudioGeneratorAvailable()
        {
            foreach (System.Reflection.Assembly assembly in System.AppDomain.CurrentDomain.GetAssemblies())
            {
                if (assembly.GetType("Microsoft.Unity.VisualStudio.Editor.SdkStyleProjectGeneration", false) != null)
                    return true;
            }

            return false;
        }
    }
}
