use std::fs;
use zed_extension_api::{
    self as zed, Command, DownloadedFileType, LanguageServerId, Os, Result, Worktree,
    settings::LspSettings,
};

/// Zed Unity Extension
/// Provides Unity-specific USS (Unity Style Sheets) language server support.
/// C# support is handled by Zed's official C# extension and Roslyn language server.
struct UnityExtension {
    cached_uss_binary_path: Option<String>,
}

impl UnityExtension {
    /// Get the appropriate USS language server release asset name for the current platform
    fn get_uss_server_asset_name(&self) -> Result<String, String> {
        let (os, arch) = zed::current_platform();

        let platform = match os {
            Os::Linux => match arch {
                zed::Architecture::Aarch64 => "linux-arm64",
                zed::Architecture::X8664 => "linux-x64",
                _ => return Err(format!("Unsupported Linux architecture: {:?}", arch)),
            },
            Os::Mac => match arch {
                zed::Architecture::Aarch64 => "darwin-arm64",
                zed::Architecture::X8664 => "darwin-x64",
                _ => return Err(format!("Unsupported macOS architecture: {:?}", arch)),
            },
            Os::Windows => match arch {
                zed::Architecture::X8664 => "win-x64",
                _ => return Err(format!("Unsupported Windows architecture: {:?}", arch)),
            },
        };

        let (os_type, _) = zed::current_platform();
        let ext = match os_type {
            Os::Windows => "zip",
            _ => "tar.gz",
        };

        Ok(format!("uss-language-server-{}.{}", platform, ext))
    }

    /// Download and install USS language server from GitHub releases
    fn install_uss_language_server(
        &self,
        language_server_id: &LanguageServerId,
    ) -> Result<String, String> {
        let asset_name = self.get_uss_server_asset_name()?;

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = zed::latest_github_release(
            "GameBayoumy/zed-unity",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )
        .map_err(|e| format!("Failed to fetch USS language server releases: {}", e))?;

        let asset = release
            .assets
            .iter()
            .find(|a| a.name == asset_name)
            .ok_or_else(|| {
                format!(
                    "No USS language server asset found for platform: {}",
                    asset_name
                )
            })?;

        let (os, _) = zed::current_platform();
        let binary_name = match os {
            Os::Windows => "uss-language-server.exe",
            _ => "uss-language-server",
        };

        let version_dir = format!("uss-language-server-{}", release.version);
        let binary_path = format!("{}/{}", version_dir, binary_name);

        if fs::metadata(&binary_path).is_ok() {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::None,
            );
            return Ok(binary_path);
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::Downloading,
        );

        let download_type = match os {
            Os::Windows => DownloadedFileType::Zip,
            _ => DownloadedFileType::GzipTar,
        };

        zed::download_file(&asset.download_url, &version_dir, download_type)
            .map_err(|e| format!("Failed to download uss-language-server: {}", e))?;

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::None,
        );

        Ok(binary_path)
    }

    /// Get command for USS language server (downloads from GitHub releases)
    fn uss_language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Command> {
        let (os, _) = zed::current_platform();

        let binary_name = match os {
            Os::Windows => "uss-language-server.exe",
            _ => "uss-language-server",
        };

        let settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;

        let binary_path = if let Some(binary) = settings.binary.as_ref() {
            binary
                .path
                .clone()
                .ok_or_else(|| "Binary path not specified in settings".to_string())?
        } else if let Some(path) = worktree.which(binary_name) {
            path
        } else if let Some(path) = &self.cached_uss_binary_path {
            path.clone()
        } else {
            let path = self.install_uss_language_server(language_server_id)?;
            self.cached_uss_binary_path = Some(path.clone());
            path
        };

        let args = settings
            .binary
            .as_ref()
            .and_then(|b| b.arguments.clone())
            .unwrap_or_default();

        Ok(Command {
            command: binary_path,
            args,
            env: Default::default(),
        })
    }
}

impl zed::Extension for UnityExtension {
    fn new() -> Self {
        Self {
            cached_uss_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Command> {
        match language_server_id.as_ref() {
            "uss-language-server" => self.uss_language_server_command(language_server_id, worktree),
            _ => Err(format!(
                "Unknown language server: {}",
                language_server_id.as_ref()
            )),
        }
    }
}

zed::register_extension!(UnityExtension);
