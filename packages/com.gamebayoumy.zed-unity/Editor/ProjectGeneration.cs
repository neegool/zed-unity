using System;
using System.Collections.Generic;
using System.Linq;
using System.Reflection;
using UnityEditor;
using UnityEngine;

namespace Zed.Unity.Editor
{
    /// <summary>
    /// Delegates C# solution/project generation to Unity's maintained Visual Studio package.
    /// This keeps Zed's C# LSP inputs aligned with Unity's own asmdef, package, analyzer,
    /// define-symbol, and player-project handling instead of maintaining a fragile fork.
    /// </summary>
    public class ProjectGeneration
    {
        private const string VisualStudioPackageName = "com.unity.ide.visualstudio";
        private const string GeneratorTypeName = "Microsoft.Unity.VisualStudio.Editor.SdkStyleProjectGeneration";

        private object _generator;

        public void GenerateAll()
        {
            if (!ZedConfig.GenerateCsprojFiles && !ZedConfig.GenerateSlnFile)
                return;

            object generator = GetGenerator();
            if (generator == null)
                return;

            InvokeGeneratorMethod(generator, "Sync");
        }

        public void SyncIfNeeded(
            string[] addedFiles,
            string[] deletedFiles,
            string[] movedFiles,
            string[] movedFromFiles,
            string[] importedFiles)
        {
            object generator = GetGenerator();
            if (generator == null)
                return;

            MethodInfo syncIfNeeded = generator.GetType().GetMethod("SyncIfNeeded");
            if (syncIfNeeded == null)
            {
                GenerateAll();
                return;
            }

            try
            {
                IEnumerable<string> affectedFiles = addedFiles
                    .Concat(deletedFiles)
                    .Concat(movedFiles)
                    .Concat(movedFromFiles);

                syncIfNeeded.Invoke(generator, new object[] { affectedFiles, importedFiles });
            }
            catch (Exception ex)
            {
                Debug.LogWarning($"[Zed Unity] Unity project generator SyncIfNeeded failed; regenerating all project files. {Unwrap(ex).Message}");
                GenerateAll();
            }
        }

        public void SyncFile(string filePath, FileChangeType changeType)
        {
            GenerateAll();
        }

        private object GetGenerator()
        {
            if (_generator != null)
                return _generator;

            Type generatorType = FindType(GeneratorTypeName);
            if (generatorType == null)
            {
                Debug.LogError(
                    "[Zed Unity] Could not find Unity's Visual Studio project generator. " +
                    $"Ensure the '{VisualStudioPackageName}' package is installed, then run Tools > Zed > Regenerate Project Files.");
                return null;
            }

            try
            {
                _generator = Activator.CreateInstance(generatorType);
                return _generator;
            }
            catch (Exception ex)
            {
                Debug.LogError($"[Zed Unity] Failed to create Unity project generator: {Unwrap(ex).Message}");
                return null;
            }
        }

        private static Type FindType(string fullName)
        {
            foreach (Assembly assembly in AppDomain.CurrentDomain.GetAssemblies())
            {
                Type type = assembly.GetType(fullName, false);
                if (type != null)
                    return type;
            }

            return null;
        }

        private static void InvokeGeneratorMethod(object generator, string methodName)
        {
            MethodInfo method = generator.GetType().GetMethod(methodName);
            if (method == null)
            {
                Debug.LogError($"[Zed Unity] Unity project generator does not expose {methodName}().");
                return;
            }

            try
            {
                method.Invoke(generator, null);

                if (ZedConfig.EnableLogging)
                {
                    Debug.Log("[Zed Unity] Unity project files synchronized.");
                }
            }
            catch (Exception ex)
            {
                Debug.LogError($"[Zed Unity] Failed to synchronize Unity project files: {Unwrap(ex).Message}");
            }
        }

        private static Exception Unwrap(Exception exception)
        {
            return exception is TargetInvocationException targetInvocationException && targetInvocationException.InnerException != null
                ? targetInvocationException.InnerException
                : exception;
        }
    }

    public enum FileChangeType
    {
        Created,
        Modified,
        Deleted,
        Renamed
    }
}
