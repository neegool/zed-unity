using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using UnityEditor;
using UnityEngine;

namespace Zed.Unity.Editor
{
    public static class ZedProjectSettings
    {
        private static readonly string[] RecommendedFileScanExclusions =
        {
            "**/.git",
            "**/.vs",
            "**/Library",
            "**/Temp",
            "**/Obj",
            "**/Build",
            "**/Builds",
            "**/Logs",
            "**/UserSettings",
            "**/*.csproj",
            "**/*.sln",
            "**/*.user",
            "**/*.userprefs",
            "**/*.pidb",
            "**/*.booproj",
            "**/*.unityproj",
            "**/*.dll",
            "**/*.exe"
        };

        public static string SettingsDirectoryPath => Path.Combine(ZedUtils.GetProjectPath(), ".zed");
        public static string SettingsFilePath => Path.Combine(SettingsDirectoryPath, "settings.json");

        public static bool SettingsFileExists => File.Exists(SettingsFilePath);

        public static void EnsureProjectSettings()
        {
            Directory.CreateDirectory(SettingsDirectoryPath);

            if (!File.Exists(SettingsFilePath))
            {
                File.WriteAllText(SettingsFilePath, BuildDefaultSettingsJson(), Encoding.UTF8);
                AssetDatabase.Refresh();
                Debug.Log($"[Zed Unity] Created {ZedUtils.FullPathToAssetPath(SettingsFilePath)}.");
                return;
            }

            MergeRecommendedFileScanExclusions();
        }

        private static void MergeRecommendedFileScanExclusions()
        {
            string existing = File.ReadAllText(SettingsFilePath, Encoding.UTF8);
            List<string> exclusions = ExtractStringArray(existing, "file_scan_exclusions");

            if (exclusions.Count == 0 && !existing.Contains("file_scan_exclusions"))
            {
                string merged = InsertFileScanExclusions(existing, RecommendedFileScanExclusions);
                File.WriteAllText(SettingsFilePath, merged, Encoding.UTF8);
                AssetDatabase.Refresh();
                Debug.Log($"[Zed Unity] Added Unity file scan exclusions to {ZedUtils.FullPathToAssetPath(SettingsFilePath)}.");
                return;
            }

            bool changed = false;
            foreach (string exclusion in RecommendedFileScanExclusions)
            {
                if (!exclusions.Contains(exclusion))
                {
                    exclusions.Add(exclusion);
                    changed = true;
                }
            }

            if (!changed)
                return;

            string replacement = BuildJsonArray("file_scan_exclusions", exclusions, 2);
            string updated = ReplaceStringArray(existing, "file_scan_exclusions", replacement);
            File.WriteAllText(SettingsFilePath, updated, Encoding.UTF8);
            AssetDatabase.Refresh();
            Debug.Log($"[Zed Unity] Updated Unity file scan exclusions in {ZedUtils.FullPathToAssetPath(SettingsFilePath)}.");
        }

        private static string BuildDefaultSettingsJson()
        {
            return "{\n" + BuildJsonArray("file_scan_exclusions", RecommendedFileScanExclusions, 2) + "\n}\n";
        }

        private static string InsertFileScanExclusions(string json, IEnumerable<string> exclusions)
        {
            string trimmed = json.Trim();
            string property = BuildJsonArray("file_scan_exclusions", exclusions, 2);

            if (string.IsNullOrEmpty(trimmed) || trimmed == "{}")
                return "{\n" + property + "\n}\n";

            int closingBrace = json.LastIndexOf('}');
            if (closingBrace < 0)
                return BuildDefaultSettingsJson();

            string before = json.Substring(0, closingBrace).TrimEnd();
            string after = json.Substring(closingBrace);
            string separator = before.EndsWith("{") ? "\n" : ",\n";

            return before + separator + property + "\n" + after.TrimStart() + (json.EndsWith("\n") ? string.Empty : "\n");
        }

        private static string BuildJsonArray(string propertyName, IEnumerable<string> values, int indent)
        {
            string propertyIndent = new string(' ', indent);
            string itemIndent = new string(' ', indent + 2);
            string[] escapedValues = values
                .Distinct()
                .Select(value => itemIndent + "\"" + EscapeJson(value) + "\"")
                .ToArray();

            return propertyIndent + "\"" + propertyName + "\": [\n" +
                   string.Join(",\n", escapedValues) +
                   "\n" + propertyIndent + "]";
        }

        private static List<string> ExtractStringArray(string json, string propertyName)
        {
            int propertyIndex = json.IndexOf("\"" + propertyName + "\"", StringComparison.Ordinal);
            if (propertyIndex < 0)
                return new List<string>();

            int arrayStart = json.IndexOf('[', propertyIndex);
            if (arrayStart < 0)
                return new List<string>();

            int arrayEnd = FindMatchingBracket(json, arrayStart);
            if (arrayEnd < 0)
                return new List<string>();

            string arrayContent = json.Substring(arrayStart + 1, arrayEnd - arrayStart - 1);
            return ParseJsonStringArray(arrayContent);
        }

        private static string ReplaceStringArray(string json, string propertyName, string replacementProperty)
        {
            int propertyStart = json.IndexOf("\"" + propertyName + "\"", StringComparison.Ordinal);
            if (propertyStart < 0)
                return InsertFileScanExclusions(json, RecommendedFileScanExclusions);

            int lineStart = json.LastIndexOf('\n', propertyStart);
            lineStart = lineStart < 0 ? 0 : lineStart + 1;

            int arrayStart = json.IndexOf('[', propertyStart);
            if (arrayStart < 0)
                return json;

            int arrayEnd = FindMatchingBracket(json, arrayStart);
            if (arrayEnd < 0)
                return json;

            int propertyEnd = arrayEnd + 1;
            return json.Substring(0, lineStart) + replacementProperty + json.Substring(propertyEnd);
        }

        private static int FindMatchingBracket(string text, int startIndex)
        {
            bool inString = false;
            bool escaped = false;
            int depth = 0;

            for (int i = startIndex; i < text.Length; i++)
            {
                char c = text[i];

                if (escaped)
                {
                    escaped = false;
                    continue;
                }

                if (c == '\\' && inString)
                {
                    escaped = true;
                    continue;
                }

                if (c == '"')
                {
                    inString = !inString;
                    continue;
                }

                if (inString)
                    continue;

                if (c == '[')
                    depth++;
                else if (c == ']')
                {
                    depth--;
                    if (depth == 0)
                        return i;
                }
            }

            return -1;
        }

        private static List<string> ParseJsonStringArray(string arrayContent)
        {
            var values = new List<string>();
            var current = new StringBuilder();
            bool inString = false;
            bool escaped = false;

            foreach (char c in arrayContent)
            {
                if (!inString)
                {
                    if (c == '"')
                    {
                        inString = true;
                        current.Clear();
                    }
                    continue;
                }

                if (escaped)
                {
                    current.Append(c);
                    escaped = false;
                    continue;
                }

                if (c == '\\')
                {
                    escaped = true;
                    continue;
                }

                if (c == '"')
                {
                    values.Add(current.ToString());
                    inString = false;
                    continue;
                }

                current.Append(c);
            }

            return values;
        }

        private static string EscapeJson(string value)
        {
            return value.Replace("\\", "\\\\").Replace("\"", "\\\"");
        }
    }
}
